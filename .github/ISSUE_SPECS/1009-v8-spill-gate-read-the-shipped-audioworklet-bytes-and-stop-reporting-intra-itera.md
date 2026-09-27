# V8 spill gate: read the shipped AudioWorklet bytes, and stop reporting intra-iteration slot reuse

## Product outcome

#1000 added `scripts/check-web-audioworklet-v8-spill.py`, a wasm gate that fails when V8 keeps an EQ cascade loop's recurrence state in a stack slot. Its Sol verification (attempt 1, PASS) found that it compiles a copy of the shipped module from a repeated cargo line, so it can drift silently from `scripts/build-web-audioworklet.sh`, and that two of its rules can raise false reds. Before the batch reaches `main`, the gate must test the bytes that ship, and a benign future spill must not block merges.

## Smallest closable slice

The findings below, verbatim from the #1000 verdict (`.github/ISSUE_SPECS/1000-*.md`), are the scope. Findings 1 and 2 are required; 3 and 4 are required if they stay small; 5 is one line.

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


## Objective gates

1. One home for the cargo line: `run-wasm-gates.sh` obtains the module from `build-web-audioworklet.sh` (a module-only mode) and CI runs the gate on the downloaded, pin-verified artifact in `artifact-gates` (with `setup-node` 22.23.2), with no second fat-LTO build. A test changes a flag in the build script and shows the gate's module changes with it.
2. The #1000 red arms stay red (attempt-1 build, one-token edit) and the batch head stays green, ten identical runs each.
3. The synthetic intra-iteration reuse listing from the verdict is green; a synthetic back-edge carry is red.
4. With `--no-wasm-loop-unrolling`, the gate either still identifies the tail uniquely or fails closed with a clear message (never a vacuous green).
5. `scripts/run-wasm-gates.sh`, `scripts/check-env-vocabulary.sh` and the workspace policy scripts pass; `qualification.yml`'s verdict table stays consistent.

## Attempt 1 evidence

Implementer: Terra, attempt 1, 2026-09-27, branch `codex/1009-spill-gate-shipped-bytes` from the
batch head `b03edde4`. The change is `6f4c0379`. Host AMD EPYC 7313P, rustc 1.97.1, Node v22.23.2
(V8 12.4.254.21-node.56), `CARGO_INCREMENTAL=0`, host load average 12-24 during the runs. The arms
are the #1000 arms: batch head, #977 attempt 1 (`f1bf752c`), and the batch head plus the one-token
tail edit. Each was built with the shipped cargo line into its own target.

### The change

- **One home for the cargo line (finding 1).**
  - `build-web-audioworklet.sh --module-only DIR` runs the delivery build's own cargo line and
    writes `miso-engine-v1-audio-worklet.simd128.wasm` alone. It skips the pin check (a batch
    repins at its boundary) and prints the digest against the pin.
  - `run-wasm-gates.sh`'s `check_v8_spill` takes its module from that mode and builds nothing
    itself.
  - In CI, `artifact-gates` runs the gate, with `setup-node` 22.23.2, on the downloaded artifact
    after its pin check. Both steps come last, so the steps before them keep the runner's Node.
  - `wasm-guests` runs `run-wasm-gates.sh --without-v8-spill` and drops its `setup-node`. It does no
    second fat-LTO build.
  - `check-ci-path-routing.py` refuses `--without-v8-spill` unless `artifact-gates` runs the gate on
    the artifact after the pin step. `test-ci-path-routing.py` has two mutations for this: the step
    removed, and the step moved before the pin check. Both are red.
  - No job was added or removed, so the verdict table is unchanged.
- **Carries must cross the header (finding 2).** The recurrence rule's taint carries a bit, set
  when it flows along the back edge into the header. Only a crossed value stored to its own slot
  counts. The live-across rule is unchanged.
- **Tail match (finding 3).** A tail row must be reachable from its function's pair loop, since
  `interleave` runs the pairs and then the tail. The ramp path's per-section `svf_block` loop has
  the mono tail's exact arithmetic, but it is not reachable from the stationary passes. A held row
  must now match exactly one loop; none, or more than one, fails closed and lists the loops.
- **Node pin (finding 4).** `--check-toolchain` and `--self-test` moved into `check_v8_spill`, so
  G5's legs no longer need the pinned Node. The re-pin procedure is in the docstring and in
  `MUTATIONS.md`.
- **CPU model (finding 5).** Every verdict line, ok or fail, names it.
- **Found on the way.** Each indirect jump got only the first entry of its jump table, because the
  table's end was taken as the next entry. Now every entry is taken. No verdict on any arm changed.

### Gate 1: one home, shipped bytes, a flag follows

- **Builder contract.** `test-sdk-artifact-builder-output-contract.sh` (mock cargo, runs in CI via
  `sdk-package.sh check`) now checks that `--module-only`:
  - writes only the module;
  - produces the same bytes as the delivery build, from the identical single cargo line
    (`RUSTFLAGS` included);
  - accepts an unpinned module, which the delivery build refuses;
  - refuses a non-empty directory without building;
  - reflects a flag changed in the script (`-C opt-level=s`) in the module.

  It also checks that `run-wasm-gates.sh` builds `host-web` only through `--module-only`. Three
  mutations were each red on it:
  - `--module-only` given its own strip flag;
  - `--module-only` falling through to the pin;
  - `run-wasm-gates.sh` with its own `cargo build -p host-web`.
- **Real build.** I changed the script's strip default from `debuginfo` to `none`.
  `run-wasm-gates.sh` then checked module `4c335472…` instead of head's `9ac37ae7…` and stayed green,
  as the code is unchanged.
- **CI step.** Run locally on the head bytes (`9ac37ae7…`, which equals Sol's merged head), the
  step's two commands give "19 cases ok" and "ok".

### Gate 2: the #1000 arms, ten runs each

| arm | module | verdict | ten runs |
|---|---|---|---|
| batch head | `9ac37ae7…` | ok, exit 0 | 10 identical outputs |
| #977 attempt 1 | `0db9b2f5…` | FAIL dual tail `[rbp-0xa0]`, exit 1 | 10 identical |
| one-token edit | `36015854…` | FAIL dual tail `[rbp-0xc0]`, exit 1 | 10 identical |

End to end, `run-wasm-gates.sh` exits 0 on head (258 s, gate 1.9 s). With the tail edit in the tree
it exits 1 (193 s). The dual pair is still reported with 10 slots on its recurrences under the new
rule, the same count as before.

### Gate 3: reuse green, carry red

- **Self-test.** It has 19 cases. "slot reuse" is the verdict's shape: fresh A spilled to
  `[rbp-0x40]`, reloaded, and B = f(A) spilled to the same slot, inside an otherwise clean two-step,
  two-stream tail. It is green. "carried" (#977's shape), "rotated" and "back-edge reload" are red.
- **Rule mutations.** Dropping the crossing bit (#1000's rule) turns "slot reuse" red with
  `[rbp-0x40]`, reproducing the finding. Never setting the bit turns "back-edge reload" green.
- **All mutations.** 15 mutations of the analysis were each red on the self-test (the list is in
  the scratch log). Besides the two above they include:
  - no `after` filter;
  - holding the first of several matches;
  - #977's linear loop scan;
  - dropping either rule;
  - the zero idiom;
  - call blocks;
  - compares and `lea`;
  - selects;
  - a `vmulps`-only shape.

### Gate 4: `--no-wasm-loop-unrolling`

The gate identifies every tail uniquely. The mono tail is the 42-instruction loop; the
35-instruction per-section loop is not reachable from the pair. Head is green and the two red arms
are red. With the `after` filter removed, the mono tail row fails closed: "2 innermost loops … where
exactly one was expected", with both loops listed. `--no-turbo-loop-rotation` gives the same
verdicts. `--turbo-instruction-scheduling` turns the one-token arm green (V8 then allocates the
tail without the slot); that is a different V8 configuration, not the pinned one.

### Gate 5 and other checks

These all pass:

- `bash scripts/run-wasm-gates.sh`, as above;
- `bash scripts/check-env-vocabulary.sh` (134 names);
- `bash scripts/check-workspace-policy.sh` and `bash scripts/test-workspace-policy.sh`;
- `python3 -B scripts/check-ci-path-routing.py` and `scripts/test-ci-path-routing.py`;
- `bash scripts/test-sdk-artifact-builder-output-contract.sh`.

Findings 4 and 5, with a shim `node` reporting v22.22.0 first on PATH:

- `run-wasm-gates.sh --without-v8-spill` exits 0 (9 s, warm).
- Without the flag, it exits 2 at the toolchain check, after G5's legs and before the module build.
  Nothing was compiled.

### Notes for the verifier

1. **Local cost.** `run-wasm-gates.sh` now pays the delivery build's cold `host-web` build
   (`mktemp` target) on every local run. That is about 3 min at load 20, and about 30 s idle. It no
   longer runs in CI.
2. **Test fixtures.** The mock cargo's fixture now carries its `RUSTFLAGS`. The existing delivery
   check reads its first line. The mock's `MOCK_UNPINNED` is not a `MISO_ENGINE_` name.
3. **Not run here.** CI itself, and `check-web-audioworklet.sh` on a full artifact directory: the
   batch pin is stale until the batch boundary, so the delivery build refuses.

## Sol attempt 1 verdict: PASS

Verifier: Sol, 2026-09-27, on `613a8610` (change `6f4c0379`). Host EPYC 7313P, rustc 1.97.1, Node
v22.23.2, `CARGO_INCREMENTAL=0`, every module built from `git archive` into its own target. No
crate changed between #1000's merge (`3c920f73`) and this head.

**The #1000 findings are closed.**

1. **One home for the cargo line.**
   - **Byte identity.** From the same tree, `build-web-audioworklet.sh --module-only` wrote
     `9ac37ae7…`. So did the full delivery build, run in a scratch copy whose pin I set to that
     digest; its `miso-engine-v1-audio-worklet.simd128.wasm` is byte-identical (`cmp`). The same
     digest comes from #1000's copied cargo line on the merged batch head.
   - **CI.** `artifact-gates` runs the self-test and then the gate on the downloaded file. Both come
     after its pin check, behind `setup-node` 22.23.2. `wasm-guests` passes `--without-v8-spill` and
     no longer builds the module or installs Node.
   - **Tests.** `test-sdk-artifact-builder-output-contract.sh` and the two new routing mutations
     are green.
2. **Intra-iteration reuse no longer fails.** Synthetic listings through `check_function`:
   - my #1000 reuse listing, verbatim, is green;
   - an invariant reload is green;
   - a back-edge carry (#977's shape) is red;
   - a register-carried recurrence round-tripped through a slot mid-iteration is red;
   - a carry through two slots is red;
   - reuse whose second value is read in the next iteration is red.
3. **The tail is unique without unrolling.** Under `--no-wasm-loop-unrolling`, and under
   `--no-turbo-loop-rotation`, every held row matches one loop. Head is green; attempt 1 and the
   one-token edit are red.
4. **The Node pin no longer blocks G5.** With a shim reporting v22.22.0,
   `run-wasm-gates.sh --without-v8-spill` exits 0. Without the flag it exits 2 after G5's legs and
   before the module build.
5. **The CPU model is named.** Every verdict line prints it.

**Real carries are still caught.** I compared #1000's rule with this one on all 60 SVF loops of both
functions, in head, attempt 1 and the one-token module. Every loop's live-across and recurrence sets
are identical, and the loops found are the same, so the jump-table fix moved none. In particular:

- #977's tail is still caught by both rules (`[rbp-0xa0]`, and `[rbp-0xc0]` under the edit);
- the masked mono tail's back-edge reload is still caught, by the recurrence rule alone;
- head's dual pair still has its 10 recurrence slots, 2 of them live across.

**Reproduced.** Ten runs per arm gave one distinct output each: head `9ac37ae7…` exits 0, attempt 1
`0db9b2f5…` and the one-token edit `36015854…` exit 1. `run-wasm-gates.sh` exits 0 cold in 297 s,
with the gate taking 1.5 s. These are also green:

- `check-ci-path-routing.py` and `test-ci-path-routing.py`;
- `check-workspace-policy.sh` and `test-workspace-policy.sh`;
- `check-env-vocabulary.sh`;
- `test-sdk-artifact-builder-output-contract.sh`;
- `check-artifact-evidence-leak.sh` and `test-artifact-evidence-leak.sh`;
- `test-npm-publish-modes.py`;
- the gate's self-test (19 cases).

**CI.** No job was added or removed, so the verdict table still expects `artifact-gates` and
`wasm-guests` to succeed on `full`. Both jobs run on `full` only, as before.
`build-web-audioworklet.sh`, the gate and `run-wasm-gates.sh` all route `full`. On the `sdk` route
nothing that shapes the module can change.

- **What `wasm-guests` lost.** Only the V8 leg (toolchain check, self-test, gate), and all of it now
  runs in `artifact-gates`.
- **Loud failures.** A stale pin fails `artifact`, which skips `artifact-gates` and fails the
  verdict. A missing or floating Node exits 2.

**Stale pin.** Nothing here waits on the repin. Locally, `--module-only` ignores the pin. In CI the
gate reads only an artifact that passed its pin check, and the batch's single push needs a fresh pin
anyway. The one duty at the boundary is to run `run-wasm-gates.sh` on the repinned tree before
pushing.

Findings (none blocking):

1. **LOW: the routing guard catches deletion and reordering, not disabling.**
   `check-ci-path-routing.py` checks the gate line by substring. I tried four mutations of the
   artifact-gates step, and all four passed the guard:
   - the line commented out inside `run:`;
   - `|| true` appended;
   - `if: false` on the step;
   - `node-version` floated to `22`. This one is still loud in CI: 22.23.3 exits 2.

   This matches the file's other presence guards. Tighten it only if disabling the step becomes a
   real risk.
2. **INFO: the held dual tail is anchored on the reported dual pair's shape.** #1010's candidate
   stores section 0's frame to the block, which makes that pair four streams. The dual tail row
   would then fail closed with "0 innermost loops … reachable from the depth-2 pair" until #1010
   updates the pair row. It fails closed, not vacuously, but #1010's brief should expect it.
3. **INFO: every local `run-wasm-gates.sh` pays a cold fat-LTO build.** `--module-only` builds in a
   fresh `mktemp` target. CI no longer does this build.
