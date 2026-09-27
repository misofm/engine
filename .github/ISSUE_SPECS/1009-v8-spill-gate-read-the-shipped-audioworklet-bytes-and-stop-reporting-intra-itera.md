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
