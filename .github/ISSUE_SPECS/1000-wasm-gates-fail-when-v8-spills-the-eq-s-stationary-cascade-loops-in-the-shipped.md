# Wasm gates: fail when V8 spills the EQ's stationary cascade loops in the shipped AudioWorklet artifact

## Product outcome

#977 attempt 1 made the standing one-band browser EQ +21 % slower (19.9 to 24.1 us per 64 tracks) because V8 kept one integrator of the depth-1 tail loop in a stack slot instead of a register. Every committed gate stayed green: the roster, the callgraph rules, the digests and the in-crate timings, which ran in a different wasm guest where the loop compiled cleanly. The #977 attempt-2 verification then showed that a one-token edit to the tail's surroundings brings the spill back, and that a rustc or V8 upgrade could too. Only a scratch V8-listing scan (`v8loops.py` in the #977 verifier's harness) sees it.

## Smallest closable slice

A committed, deterministic check in the wasm gate family (beside `scripts/check-web-audioworklet-callgraph.py`) that loads the shipped `host_web.wasm` in the pinned Node version, has V8 optimise the EQ's stationary cascade functions (depth-1 tail and depth-2 pair, dual and mono), and fails if any of those loops carries a stack slot across its back edge. It prints the offending listing. It must not time anything.

## Objective gates

1. Green on the batch head. Red on `codex/977-eq-elision-and-passes-attempt1` (the spilling build), and red under the one-token tail edit the #977 verifier recorded.
2. Deterministic: the same verdict on ten consecutive runs, with the Node version and V8 flags pinned in the script and checked at start (a different Node version is a clear error, not a pass).
3. It locates the loops by stable symbol names, not by listing offsets, and fails closed if it cannot find one.
4. Wired into `scripts/run-wasm-gates.sh`, with its runtime reported (target under 60 s).
5. `docs` note: what the check proves (no carried stack slot in those loops), and what it does not (timing).

## Attempt 1 evidence

Implementer: Terra, attempt 1, 2026-09-27, branch `codex/1000-v8-spill-gate` from
`codex/batch-plumbing-floor-2` (`fc43c97d`). Host AMD EPYC 7313P, rustc 1.97.1, Node v22.23.2 (V8
12.4.254.21-node.56). Every module was built with `build-web-audioworklet.sh`'s cargo line,
`CARGO_INCREMENTAL=0`, each arm in its own target.

### The change

- `scripts/check-web-audioworklet-v8-spill.py` (new).
  - **Pins, checked first.** The exact Node and V8 versions, linux/x64, and the host CPU features
    V8 12.4 selects instructions by.
  - **Compile.** It finds `PreparedParametricEq<f32x4, _>::process_bank` and `::process_bank_mono`
    in the name section, one match each. The child is `node --no-liftoff --no-wasm-lazy-compilation
    --print-wasm-code-function-index=N`, run without `NODE_OPTIONS`; it compiles the module and runs
    nothing. The listing must say TurboFan and carry protected instructions (trap-handler bounds
    checks).
  - **Loops.** It builds the listing's CFG and its natural loops (dominator back edges), leaving
    out call blocks (the out-of-line stack guard and trap stubs). Inside a function, loops are found
    by what they compute: `k` SVF steps (exactly `7k`/`9k`/`2k` `vmulps`/`vaddps`/`vsubps`), `s`
    streams (vector stores to non-stack memory), and no `vpor`/blend.
  - **Rule.** A loop fails when a stack slot written in it carries a value from one iteration to the
    next. Either the slot is live across the back edge (read on a path from the header before it
    is written), or it lies on a recurrence (a value loaded from it reaches, through registers,
    slots and the back edge, a store to it).
  - **Rows.** Held: dual depth-1 tail, mono depth-2 pair, mono depth-1 tail. A held row that matches
    no loop fails closed and prints the loops it found. Reported: the dual depth-2 pair (see the
    finding below). A failing loop is printed with its carried accesses marked.
  - **Modes.** `--self-test` runs 16 synthetic cases; `--check-toolchain` checks the pins alone.
- `scripts/run-wasm-gates.sh`: `--check-toolchain` and `--self-test` run before anything is built.
  `check_v8_spill` runs last: it builds `host-web` with `build-web-audioworklet.sh`'s flags into
  `target/ci/wasm-gates-web`, runs the gate, and prints its runtime.
- `.github/workflows/qualification.yml`: the `wasm-guests` job (which runs `run-wasm-gates.sh`) gets
  `actions/setup-node` at `22.23.2` (present in `actions/node-versions`).
- Docs: a section in `tools/wasm-gates/MUTATIONS.md` (what it proves and what it does not, the red
  mutations, the reported pair), and one paragraph in `docs/rulings/effect-floor-accounting.md`.

### Finding: the dual pair already carries integrators through memory at the batch head

The select-free dual depth-2 pair (181 instructions) at the batch head keeps `ic2` (`[rbp-0xc0]`)
and `ic1` (`[rbp-0x2a0]`) of one (stream, section) chain in stack slots across the back edge:

- `[rbp-0xc0]` is loaded at `+0x6e35` into `vsubps xmm5,xmm9,xmm4` (`v3 = x - ic2`), reloaded at
  `+0x6f9f` for `v2` and `ic2 + 2*d2`, flushed, and stored at `+0x70b6`. The back edge's gap moves
  store it again at `+0x6cd5`.
- The loop is entered at `+0x6ce9`, in its middle. #977's `carried.py` read the body in a straight
  line from the back edge's target `+0x6cc0`, saw the gap move's store first, and reported the loop
  clean. The #977 attempt-2 evidence says the same ("the dual pairs have none either").

With the recurrence rule the pair has ten slots on its recurrences (spilled `d` terms and
integrators) at the head, at attempt 1 and under the one-token edit. It carries ten values across
its back edge beside 24 loop-invariant coefficients in sixteen vector registers.

The spec requires both "fails if any of those loops carries a stack slot" and "green on the batch
head". For this loop they cannot both hold. The gate therefore **reports the dual pair and does not
hold it**. I first tried an allowance of 2, the count under the live-across rule, and dropped it
once the recurrence rule showed ten. In a loop that starved, a slot count moves with any
allocation change and says nothing about time, so it would be a byte pin by another name. Whether
these slots cost time is not measured. The pair is probably throughput-bound, where the
83-instruction tail was latency-bound, but that is a hypothesis.

The recurrence rule also sees what the live-across rule misses. The masked mono tail spills the new
`ic2` mid-iteration and reloads it on the back-edge path (`+0x4337` store, `+0x42cf` load), so
nothing is live across its header in memory. It is out of scope (masked), and #977's scan flagged
it too.

### Gate 1: green on the head, red on attempt 1 and under the one-token edit

| arm | source | `host_web.wasm` sha256 | verdict |
|---|---|---|---|
| head | `fc43c97d` | `eee596e87210cfbb…` | **ok**, exit 0 |
| attempt 1 | `codex/977-eq-elision-and-passes-attempt1` (`f1bf752c`) | `0db9b2f52a810ee2…` | **FAIL**, exit 1: dual tail carries `[rbp-0xa0]` (84 insns) |
| one-token edit | head + `if !admitted && (L::mask_any(…) \|\| L::mask_any(…))` in `interleave`'s tail | `59fda8ba7322cc45…` | **FAIL**, exit 1: dual tail carries `[rbp-0xc0]` (84 insns) |

- **Head.** Dual tail 83 insns, mono pair 79, mono tail 42, none carried. The dual pair is reported
  with 181 insns and 10 slots.
- **Red arms.** The attempt-1 and one-token failures match the #977 verdicts: 84 instructions, the
  slot reloaded at the top and stored mid-iteration.
- **Pin.** The committed batch pin (`8934cdd9…`) is stale for all three; it was last repinned at
  #925-#928. `run-wasm-gates.sh`'s own build of the head is `eee596e8…`, byte-identical to the
  scratch build from another path, so the remap flags make it the shipped bytes.
- **End to end.** With the one-token edit applied in the worktree, `run-wasm-gates.sh` exits 1 at
  the V8 gate (module `59fda8ba…`, 3m21s including the rebuilds). With the edit reverted it exits
  0 (3m07s), and `git diff crates/` is empty.

### Gate 2: determinism and pins

- **Ten runs per arm.** Output was byte-identical (absolute code addresses are stripped from printed
  listings): head 10/10 exit 0, attempt 1 10/10 exit 1, one-token 10/10 exit 1. Normalised V8
  listings (address and byte columns dropped) are identical across runs without
  `--single-threaded`.
- **Node pin.** With a shim `node` reporting `v22.22.0 12.4.254.21-node.33 linux x64`, the gate
  exits 2: "Node v22.23.2 (V8 12.4.254.21-node.56) on linux/x64 is required, found … Another V8
  allocates registers differently".
- **Flags and CPU.** Each of `--no-turbo-loop-rotation`, `--no-wasm-loop-unrolling` and
  `--no-enable-avx` changes the listing, so the flags are pinned in `V8_FLAGS` and the CPU features
  are checked from `/proc/cpuinfo`.
- **`NODE_OPTIONS`.** Node refuses V8 flags there, and the child clears it anyway.

### Gate 3: symbols, not offsets; fail closed

Functions come from name-section symbols and loops from their computed shape. The script pins no
offset, register or instruction count.

Thirteen mutations of the analysis were run against `--self-test`. Twelve are red:

- call blocks kept;
- compares counted as writes;
- `lea` counted as a read;
- selects ignored;
- the reported row held;
- linear loops (#977's scan, back edge = any backward jump): red on the rotated-loop case;
- no kill within a block;
- `vmulps` count alone;
- no fail-closed;
- the recurrence rule dropped: red on the back-edge-reload case;
- the live-across rule dropped: red on the #977 case;
- the zero idiom ignored.

One stays green: dropping the read half of a read-modify-write from the live-across rule. The
recurrence rule catches the same case, so the two rules overlap there. A held row with no matching
loop fails (self-test).

### Gate 4: wiring and runtime

- **The gate itself.** 1.35-2.42 s per run over 30 runs, at host load average 12-24. That is two
  TurboFan compiles of the whole module (about 0.64 s each) and about 0.5 s of analysis.
  `run-wasm-gates.sh` prints `wasm gates: V8 spill gate ran in … s (build excluded)`.
- **The whole script.** Cold, first run: 4m14s including the new `host-web` build. Warm, final
  script: 10.9 s, exit 0, with the gate at 1.3 s. A cold scratch `host-web` build is about 26 s at `-j 10` here.
- **CI.** The `wasm-guests` job gains one fat-LTO `host-web` build. The `artifact` job's
  equivalent takes about 2 min there.

### Gate 5: docs

`tools/wasm-gates/MUTATIONS.md`, "The EQ's stationary cascade loops under V8 (issue #1000)", says
what it proves and what it does not:

- **Proves:** no value of a held loop goes through a stack slot from one iteration to the next, in
  the reference V8.
- **Does not prove:** anything timed or anything else about speed. Node's V8 and eager TurboFan are
  proxies for a browser.

The same statement is in the script's docstring, and the ruling points to it.

### Other gates

`bash scripts/check-env-vocabulary.sh` ok (134 names; no new name). `bash
scripts/check-workspace-policy.sh` ok, and `bash scripts/test-workspace-policy.sh` ok.
`python3 -B scripts/check-ci-path-routing.py` and `scripts/test-ci-path-routing.py` ok.
`bash scripts/run-wasm-gates.sh` ok, as above.

### Deviations and notes for the verifier

1. **The dual pair is reported, not held** (the finding above). This narrows the spec's "any of those
   loops": three of the four are held. Owner question: open an EQ issue to take the pair's
   recurrences out of memory, or to show that they cost nothing, if the pair is to be held.
2. **The rule is wider than "live across the back edge".** The recurrence half is needed so that a
   reload placed on the back-edge path cannot hide the #977 mechanism.
3. **`run-wasm-gates.sh` builds the module itself** with `build-web-audioworklet.sh`'s cargo line,
   rather than calling that script. That script refuses a module that does not match its pin, and
   the batch pin is stale until the batch boundary. The flags are duplicated, and drift between the
   two copies is not gated.
4. **The CI runner's CPU was not observed.** Equality with this host's code is argued from V8
   12.4's feature probe: every feature it selects by is required and present on x86-64-v3. The
   first CI run is the real check.
5. **Proxy limits.** Eager TurboFan without Liftoff feedback, and Node 22's V8 12.4, not Chrome's.
   Wasm inlining is off in 12.4, and a newer V8 may inline.
6. **The register semantics are approximate.** The recurrence rule models x86 register semantics for
   the shapes these loops use: VEX three-operand, moves, two-operand arithmetic, compares and the
   zero idiom. An unknown shape reads all its operands, which can only add a path.

## Sol attempt 1 verdict: PASS

Verifier: Sol, 2026-09-27, on `1364cf2d` and on its merge with the current batch head `4cf2a104`
(which now carries #971, #997 and #1001; `git merge-tree` is clean). Host EPYC 7313P, rustc 1.97.1,
Node v22.23.2. Every module was built from `git archive` into its own target with the shipped cargo
line and `CARGO_INCREMENTAL=0`.

**Gate 1 reproduces.** I got the recorded bytes for head (`eee596e8…`), attempt 1 (`0db9b2f5…`)
and the one-token tail edit (`59fda8ba…`):

- head and the merged batch head (`9ac37ae7…`) are green;
- attempt 1 is red on the dual tail with `[rbp-0xa0]`;
- the one-token edit is red with `[rbp-0xc0]`.

`build-web-audioworklet.sh` in repin mode prints `eee596e8…`, so today the gate's copied cargo line
builds the bytes that ship. `run-wasm-gates.sh` passed end to end (cold, 250 s; the gate took 1.5 s).

**Gate 2 reproduces.** Ten runs per arm (head, merged, attempt 1, one-token) gave one distinct output
and one exit code per arm. The local `node` binary is byte-identical to nodejs.org's
`node-v22.23.2-linux-x64` (`3517c2df…`), which is what `setup-node` installs.

**It cannot pass vacuously on the paths I tried.**

- **Symbol renamed.** Patching `12process_bank` to `12process_bonk` in the name section fails with
  "0 functions match", exit 1.
- **Tiering.** Dropping `--no-liftoff` exits 2 (the listing says Liftoff). Dropping
  `--no-wasm-lazy-compilation`, or passing no flags, exits 2 (nothing printed). `--no-enable-avx`
  fails all three held rows closed.
- **Other V8 flags.** Under `--no-wasm-loop-unrolling`, `--no-turbo-loop-rotation` or
  `--turbo-instruction-scheduling`, the gate still finds every row, and the loops stay clean.
- **Tier-up.** I ran the eq1 and eq2 fixtures through the render export with default flags (Liftoff,
  then dynamic tier-up). V8 12.4 then prints a TurboFan `process_bank` identical to the eager
  listing, all 8,574 lines. So for this V8, eager compilation is exact, not a proxy; only Chrome's V8
  version remains a gap.
- **My own mutations.** Swapping the tail's `||` operands changes the bytes (`eb723b82…`) and stays
  green. The one-token edit applied to the *mono* tail stays green, so V8 did not respill it. Hoisting
  `nc1` per step builds identical bytes.

**CI: the runner will run the gate, and it should pass.** `wasm-guests` is expected `success` on the
full route. Changes to `crates/parametric-eq`, `crates/lane` and the gate script all route `full`.
The script runs under `set -euo pipefail`, with no skip path.

- **Node.** `setup-node` fetches 22.23.2, which is present in `actions/node-versions`. 22.23.3 is the
  latest release there now, and the other jobs float on `22`.
- **CPU.** V8 12.4 probes exactly the twelve features the script checks, plus `is_atom`. Its
  flag list has no AVX-512 or VNNI entries.
  - Disabling BMI1, BMI2, LZCNT, POPCNT, SAHF or FMA3 one at a time leaves both functions' code
    identical. So do `--mcpu=atom` and `--intel-jcc-erratum-mitigation`.
  - Disabling AVX2 changes 127 lines, but the held loops stay clean.
  - Disabling SSE3 through SSE4.2 turns AVX off and fails closed.
  - So on any x86-64-v3 host the verdict is the reference one. The same job's native leg already
    needs AVX2 and FMA (the workspace's pinned flags, and `lane`'s host attestation), so the CPU
    check adds no new way for a hosted runner to fail.
  - If a runner ever lacks a feature, exit 2 blocks merges. For a required gate that is the right
    failure, since the alternative is a skip.
- **Time budget.** The job gains one fat-LTO build of about 2 min on the `artifact` job's evidence.
  It runs about 5 min today, against a 15-minute limit.

**The deviation is justified.** At both heads the dual pair routes ten slots through its
recurrences. `[rbp-0xc0]` (`ic2`, `v3 = x - ic2`) and `[rbp-0x2a0]` (`ic1`) of stream 1, section 0
are stored at the end of the iteration and by the back edge's gap moves, and reloaded in the next
iteration. The loop is 181 instructions, and 27 of them (13 loads, 14 stores) are that traffic. A
further 25 reload invariants. It carries ten values and 24 invariants, against the 15 XMM registers
V8 can allocate (`xmm15` is its scratch).

Every variant I could build cheaply is slower. Two holds under `timing.lock` with `taskset -c 31`,
arms rotated in both orders, load average 5.8-7.9. The table gives the two-band isolate through the
render export, us per 64-track block, mean of 6 runs:

| arm | pair loop | slots on recurrences | hold 1 | hold 2 |
|---|---|---:|---:|---:|
| head (skewed, S=2, D=2) | 181 insns | 10 | 30.48 | 30.61 |
| split: two S=1 skewed passes (bit-identical) | 83 + 80 | 0 | 35.31 | 35.37 |
| non-skewed S=2, D=2, #977's shape (bit-identical) | 159 | 7 | | 33.69 |
| coefficients re-read from memory per step (bit-identical) | 192 | 11 | 32.13 | |
| one coefficient set for all four chains (timing only, not exact) | 178 | 13 | | 31.38 |

- **Checks.** The one-band isolate is 20.0-21.2 us in every arm. The exact variants hash
  identically to head over 397 blocks of eq1, eq2 and builtins. A variant that skips the right
  channel changes eq2 only, so eq2 does run the admitted dual pair.
- **What it shows.** Even with 6 invariants, V8 spills the loop's recurrences first. The only
  spill-free form loses 16 %. Holding the pair today would force a slower EQ.

**Ceremony.** The gate discriminates a claim: no held loop routes a value through a stack slot from
one iteration to the next. It pins no offset, register or count. The dual pair's count is printed
and not held. The version pin is needed because register allocation belongs to one V8.

Findings:

1. **MEDIUM (follow-up): the gate reads a copy of the shipped bytes, not the bytes, and the copy can
   drift silently.** `check_v8_spill` repeats `build-web-audioworklet.sh`'s RUSTFLAGS and cargo line.
   - **How it fails.** Suppose that script changes its build, for example a new `-C` flag, a
     `--features` on `host-web`, or a different `MISO_ENGINE_WEB_STRIP` default. The gate keeps
     compiling the old module. It can stay green while the shipped tail respills, or go red on bytes
     that never ship. Nothing notices.
   - **CI cost.** It also contradicts `qualification.yml`'s "Built exactly once here; every consumer
     downloads it and re-hashes", and pays for a second fat-LTO build.
   - **The single source of truth.**
     - Give `build-web-audioworklet.sh` a module-only mode that writes the unpinned `.wasm`, and have
       `run-wasm-gates.sh` call it, so the cargo line has one home.
     - In CI, run the gate in `artifact-gates` on the downloaded, pin-verified
       `miso-engine-v1-audio-worklet.simd128.wasm`. That job would need `setup-node` 22.23.2. It
       has the exact shipped bytes and needs no rebuild; its 10-minute limit has room for the 1.5 s.
2. **LOW: the recurrence rule reports intra-iteration slot reuse as a carried value.**
   - **Why it matters.** V8 merges non-overlapping spill ranges into one slot. A value A that is
     fresh each iteration can be spilled, reloaded, and used to compute B, which is spilled to the
     same slot. Nothing crosses the back edge.
   - **Proof.** A synthetic listing of that shape, with an otherwise clean two-step, two-stream tail,
     fails `check_function` with "V8 carries [rbp-0x40] from one iteration to the next".
   - **Impact.** A future benign spill would block merges with a false diagnosis.
   - **Fix.** Flag only a load-to-store path that crosses the header. That still catches #977's
     shape, the masked tail's back-edge reload, and head's mid-iteration spills of the `d` terms.
3. **LOW: the shape key relies on V8 unrolling the per-section `svf_block` loop by three.**
   - **The collision.** Under `--no-wasm-loop-unrolling`, `process_bank_mono` has two select-free
     loops with one stream and one step: the 35-instruction per-section loop and the 42-instruction
     tail. Both would be held.
   - **Consequences.** Today that is only over-holding, so a false red. A vacuous green needs two
     changes together: the real tail reshapes, and another loop takes its shape.
   - **Fix.** Disambiguate by the depth-1 tail's instruction mix, or report a row that matches more
     than one loop.
4. **LOW: `run-wasm-gates.sh` checks the Node pin before the native leg.** Without exactly Node
   22.23.2, G5's cross-target legs cannot run locally at all. Checking the pin in `check_v8_spill`
   would keep that coupling to the V8 gate. MUTATIONS.md should also say how to re-pin Node: re-run
   the red arms on the new V8, and keep the rule whatever they show.
5. **INFO.** Nothing has shown the mono rows going red on a real module. My mono one-token edit
   stayed clean, so the mono rows rest on the shared analysis and the self-test. The first CI run is
   still the only observation of the runner's codegen, so the `ok` line should print the CPU model.

### Draft follow-up issue: the dual depth-2 pair routes its integrators through V8 stack slots

**Problem.** In the shipped `host_web.wasm`, V8 12.4's TurboFan compiles the admitted, select-free
dual depth-2 pair loop of `PreparedParametricEq<f32x4, _>::process_bank` (`svf_cascade_skewed`
with S=2, D=2) to 181 instructions. Ten stack slots lie on its recurrences, among them `ic1` and
`ic2` of stream 1, section 0, which go through memory from one iteration to the next. 27
instructions are that traffic.

- **Why V8 spills.** The loop carries 8 integrators and 2 skew carries beside 24 invariants, against
  15 allocatable XMM registers. V8 spills the loop phis rather than the invariants.
- **What does not help.** Splitting it into spill-free single-stream passes costs 16 % on the
  two-band isolate (30.5 to 35.3 us per 64 tracks). Re-reading the coefficients, sharing them, or
  dropping the skew does not remove the spills either.
- **What is at stake.** Store-to-load forwarding on the recurrence and the extra dispatch. An upper
  bound from the instruction count is about 15 % of the pair loop, at most about 3 us of the 30.5 us
  isolate. Nothing measured has realised any of it.

**Candidate.** Take the two skew carries out of registers by routing section 0's output through the
block in place: section 0 writes frame `i`, and section 1 reads it back one iteration later. An
`f32` store and load is the identity, and a carry is not a recurrence, so memory costs it no
recurrence latency. Then check whether V8 keeps the eight integrators in registers.

**Gates.**

1. The render is bit-identical: the G2 kernel identity, the parametric-eq suite, and the 90 native
   and 30 wasm console digests.
2. #1000's gate, with finding 2 fixed, reports no slot on an integrator recurrence in the dual pair.
   The pair is then moved from reported to held in the same change.
3. The two-band isolate through the render export (`web.mjs`) improves by at least 1.0 us. This is
   the mean of 6 runs per arm in both orders under `timing.lock` and `taskset -c 31`. The one-band
   isolate and the builtins row must stay within noise.
4. Stop after one prototype if gate 2 or gate 3 fails. Record the listing and the numbers, and name
   the reason for the gap: V8 x64's 15 XMM registers at S=2, D=2 is class B. Do not chase it further.
   This is weekly-optimisation work, not launch-critical.
