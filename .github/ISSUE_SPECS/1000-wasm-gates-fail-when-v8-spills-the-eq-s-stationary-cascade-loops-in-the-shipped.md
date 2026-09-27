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
