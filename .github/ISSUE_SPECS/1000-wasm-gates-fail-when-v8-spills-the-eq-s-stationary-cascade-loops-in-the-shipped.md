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
`codex/batch-plumbing-floor-2` (`fc43c97d`); the change is `53958a6b`. Host AMD EPYC 7313P, rustc
1.97.1, Node v22.23.2 (V8 12.4.254.21-node.56). Every module was built with
`build-web-audioworklet.sh`'s cargo line, `CARGO_INCREMENTAL=0`, each arm in its own target.

### The change

- `scripts/check-web-audioworklet-v8-spill.py` (new). It checks the exact Node and V8 versions,
  linux/x64, and the host CPU features V8 12.4 selects instructions by. It finds
  `PreparedParametricEq<f32x4, _>::process_bank` and `::process_bank_mono` in the name section, one
  match each. The child is `node --no-liftoff --no-wasm-lazy-compilation
  --print-wasm-code-function-index=N`, run without `NODE_OPTIONS`; it compiles the module and runs
  nothing. The listing must say TurboFan and carry protected instructions (trap-handler bounds
  checks). The gate builds the listing's CFG and its natural loops (dominator back edges), leaving
  out call blocks (the out-of-line stack guard and trap stubs). Inside a function, loops are
  found by what they compute: `k` SVF steps (exactly `7k`/`9k`/`2k` `vmulps`/`vaddps`/`vsubps`),
  `s` streams (vector stores to non-stack memory), and no `vpor`/blend. A slot is carried when it
  is live into the header (read on a path from the header before it is written) and written in the
  loop. Each row must match at least one loop, or it fails closed and prints the loops found. A
  failing loop is printed with its carried accesses marked. `--self-test` runs 14 synthetic cases;
  `--check-toolchain` checks the pins alone.
- `scripts/run-wasm-gates.sh`: `--check-toolchain` and `--self-test` run before anything is built.
  `check_v8_spill` runs last: it builds `host-web` with `build-web-audioworklet.sh`'s flags into
  `target/ci/wasm-gates-web`, runs the gate, and prints its runtime.
- `.github/workflows/qualification.yml`: the `wasm-guests` job (which runs `run-wasm-gates.sh`) gets
  `actions/setup-node` at `22.23.2` (available in `actions/node-versions`).
- Docs: a section in `tools/wasm-gates/MUTATIONS.md` (what it proves and what it does not, the red
  mutations, the allowance), and one paragraph in `docs/rulings/effect-floor-accounting.md`.

### Finding: the select-free dual pair already carries two integrators at the batch head

In the batch head's module, the select-free dual depth-2 pair (181 instructions) keeps `ic2`
(`[rbp-0xc0]`) and `ic1` (`[rbp-0x2a0]`) of one (stream, section) chain in stack slots across the
back edge. `[rbp-0xc0]` is loaded at `+0x6e35` and feeds `vsubps xmm5,xmm9,xmm4` (`v3 = x - ic2`). It
is reloaded at `+0x6f9f` for `v2` and for `ic2 + 2*d2`, flushed, and stored at `+0x70b6`, and again by
the back edge's gap moves at `+0x6cd5`. The loop is entered at `+0x6ce9`, in its middle. #977's
`carried.py` read the body in a straight line from the back edge's target `+0x6cc0`, saw the gap
move's store first, and reported the loop clean. So did the #977 attempt-2 evidence ("the dual pairs
have none either") and its verdict. Attempt 1's pair carries one slot, and the one-token arm's pair
carries two. The spec requires all four loops clean and the gate green on the head, and both cannot
hold. The dual pair row therefore has an **allowance of 2** (the other rows: 0). The gate prints
"lower the allowance" when a loop carries fewer slots than its row allows. Whether the two slots
cost time was not measured. The 181-instruction body is probably throughput-bound, where the
83-instruction tail was latency-bound, but that is a hypothesis.

### Gate 1: green on the head, red on attempt 1 and under the one-token edit

| arm | source | `host_web.wasm` sha256 | verdict |
|---|---|---|---|
| head | `fc43c97d` | `eee596e87210cfbb…` | **ok**, exit 0 |
| attempt 1 | `codex/977-eq-elision-and-passes-attempt1` (`f1bf752c`) | `0db9b2f52a810ee2…` | **FAIL**, exit 1: dual tail carries `[rbp-0xa0]` (84 insns) |
| one-token edit | head + `if !admitted && (L::mask_any(…) \|\| L::mask_any(…))` in `interleave`'s tail | `59fda8ba7322cc45…` | **FAIL**, exit 1: dual tail carries `[rbp-0xc0]` (84 insns) |

Head rows: dual pair 181 insns, carries `[rbp-0x2a0]` and `[rbp-0xc0]`, within its allowance of 2.
Dual tail: 83 insns, none. Mono pair: 79, none. Mono tail: 42, none. The attempt-1 and one-token
failures match the #977 verdicts (84 instructions, the slot reloaded at the top and stored
mid-iteration). The committed batch pin (`8934cdd9…`) is stale for all three: it was last repinned
at #925-#928. `run-wasm-gates.sh`'s own build of the head is `eee596e8…`, byte-identical to the
scratch build from another path, so the remap flags make it the shipped bytes. End to end, with the
one-token edit applied in the worktree, `run-wasm-gates.sh` exits 1 at the V8 gate (module
`59fda8ba…`). With the edit reverted it exits 0.

### Gate 2: determinism and pins

Ten consecutive runs per arm gave byte-identical output (absolute code addresses are stripped from
printed listings): head 10/10 exit 0 (`fcb73755…`), attempt 1 10/10 exit 1 (`e00649a4…`),
one-token 10/10 exit 1 (`f7db0bed…`). Normalised V8 listings (the address and byte columns
dropped) are identical across runs without `--single-threaded`. With a shim `node` reporting
`v22.22.0 12.4.254.21-node.33 linux x64`, the gate exits 2: "Node v22.23.2 (V8
12.4.254.21-node.56) on linux/x64 is required, found … Another V8 allocates registers differently".
Each of `--no-turbo-loop-rotation`, `--no-wasm-loop-unrolling` and `--no-enable-avx` changes the
listing, so the flags are pinned in `V8_FLAGS`, and the CPU features are checked from
`/proc/cpuinfo`. Node refuses V8 flags in `NODE_OPTIONS`, and the child clears it anyway.

### Gate 3: symbols, not offsets; fail closed

Functions come from name-section symbols; loops from their computed shape. The script pins no
offset, register or instruction count. Ten mutations of the analysis were each red on
`--self-test`:

- call blocks kept;
- no read-modify-write;
- compares counted as writes;
- `lea` counted as a read;
- selects ignored;
- allowance off by one;
- linear loops (back edge = any backward jump, #977's scan): red on the rotated-loop case;
- no kill within a block;
- `vmulps` count alone;
- no fail-closed.

A row with no matching loop fails (self-test case; and with the natural-loop rule replaced, all
four real rows fail closed).

### Gate 4: wiring and runtime

The gate itself takes 1.09-1.30 s per run (30 runs), two TurboFan compiles of the whole module at
about 0.5 s each. `run-wasm-gates.sh` prints `wasm gates: V8 spill gate ran in 1.1 s (build
excluded)`. The whole script took 4m14s cold, including the new `host-web` build, and 10.7 s warm,
exit 0. A cold scratch `host-web` build is about 26 s at `-j 10` here. On CI it adds one
`host-web` fat-LTO build to the `wasm-guests` job (the `artifact` job's equivalent is about 2 min).

### Gate 5: docs

`tools/wasm-gates/MUTATIONS.md`, "The EQ's stationary cascade loops under V8 (issue #1000)": what it
proves (no carried stack slot beyond the allowance in the reference V8) and what it does not (no
timing, no claim about speed, Node's V8 and eager TurboFan are proxies for a browser). The same
statement is in the script's docstring. The ruling points to it.

### Other gates

`bash scripts/check-env-vocabulary.sh` ok (134 names; no new name). `bash
scripts/check-workspace-policy.sh` ok, and `bash scripts/test-workspace-policy.sh` ok.
`python3 -B scripts/check-ci-path-routing.py` and `scripts/test-ci-path-routing.py` ok.
`bash scripts/run-wasm-gates.sh` ok, as above.

### Deviations and notes for the verifier

1. **The dual pair's allowance of 2** (the finding above). This narrows the spec's "any of those
   loops" for one row. The alternative was a gate that is red on the head. Owner question: keep
   the allowance and open an EQ issue to take the pair's integrators out of memory (or show that
   they cost nothing), or leave the pair unguarded.
2. **`run-wasm-gates.sh` builds the module itself** with `build-web-audioworklet.sh`'s cargo line,
   so it does not call that script. That script refuses a module that does not match its pin, and
   the batch pin is stale until the batch boundary. The flags are duplicated, and drift between the
   two copies is not gated.
3. **The CI runner's CPU was not observed.** Equality with this host's code is argued from V8
   12.4's feature probe: every feature it selects by is required and present on x86-64-v3. The
   first CI run is the real check.
4. **Proxy limits.** Eager TurboFan without Liftoff feedback, and Node 22's V8 12.4, not Chrome's.
   Wasm inlining is off in 12.4, and a newer V8 may inline.
5. **Scope.** Masked kernels are out of scope. The masked dual and mono tails carry a slot at the
   head, as they did before #977.
