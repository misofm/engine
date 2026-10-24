# Keep V8's EQ armed depth-2 loops free of carried slots under Rust 1.99.1

Filed by root from #877 (backlog sprint, 2026-10-24). Blocks #877 (PR #1900, the toolchain move to 1.99.1).

## Finding

`scripts/check-web-audioworklet-v8-spill.py` (the #977/#1000/#1009 gate on the EQ cascade loops) passes on the shipped 1.97.1 build and fails on a 1.99.1 build of the same tree (base eb792c949, Node 22.23.2; CI run 40091997453, "AudioWorklet artifact gates"):

- mono armed depth-2 pair, select-free: V8 carries `[rbp-0x78]` across iterations (ceiling 0);
- mono armed depth-2 pair, masked: V8 carries `[rbp-0xe0]` across iterations (ceiling 0);
- dual armed depth-2 pair, select-free: two loops match where exactly one is expected (the gate fails closed).

This is the #977 mechanism: a loop value kept in a stack slot from one iteration to the next, which once made the browser EQ about 20 % slower. No bit moves.

## Smallest closable slice

Shape the EQ's armed depth-2 cascade loops so that both the 1.97.1 build (main's CI) and a 1.99.1 build pass the V8 spill gate with its current ceilings, and the dual armed pair matches exactly one loop. No rendered bit moves (class A: the console and corpus digests are unchanged). The gate's ceilings and its fail-closed rule do not change.

## Gates

- `scripts/check-web-audioworklet-v8-spill.py` passes on a 1.97.1 named-twin build and on a 1.99.1 named-twin build (record both outputs in this spec).
- `check-web-audioworklet.sh`, `check-browser-expected-resources.py` and the parametric-eq tests pass; the console and corpus digests equal base.
- `cargo test -p lane --release --test g2_kernel_identity` (skewed equals interleaved bit for bit).

## Order

Merge before #877. Then #877 re-pins the artifact and re-records the three browsers once.

## Paths

`crates/lane/src/kernels.rs` (`svf_cascade_skewed_form`, the `frames_left` doc, `threshold_at` if needed), this spec; the web roster rows only if a kernel name changes.

## Decision record

- D1 (scope note, 2026-10-24): the paths are corrected. The loops are not in `crates/parametric-eq`: the EQ's `interleave`/`interleave_mono` only call `svf_cascade_skewed`/`svf_cascade_skewed_with_dry_masks`, and the frame loop is `svf_cascade_skewed_form` in `crates/lane/src/kernels.rs`.
- D2 (root): the fix pins a toolchain-independent source loop structure; no `unsafe`, no `black_box`, no `cfg(version)`. Only the walk of the steady-state loop changes, never its arithmetic (class A). Shapes are tried in order: (a) `chunks_exact` with a ring of previous chunks; (b) one byte offset plus a countdown; (c) the `frames_left` split walk.
- D3 (root): the gate script, its ceilings and its fail-closed rule do not change. If every shape fails, stop and report.

## Evidence
- D4 (attempt 1, implementer; root to confirm): the dual armed pair's second loop was not a versioned steady state. It was the `frames < D` fallback in `svf_cascade_skewed_impl`: one whole-block call into the interleaved form, whose split walk LLVM 22 kept as a loop (the wasm of 1.99.1 shows it, with 4 loads and one exit test; 1.97.1 folds it). The fallback now calls the interleaved body once per exact one-frame slice, which leaves no loop. This is in `kernels.rs` but outside the skewed form itself; the per-frame calls run the same operations on the same frames in the same order (class A). It needs a forwarding `SvfOutput` impl for `&M`.
- D5 (attempt 1): shape (b) is chosen for the steady state: one word offset `base` (`while base < span`, `base += width`), which is the exit test and the index. The `(i - section) * width` products and the iteration count are gone; the bounds branches stay. Shape (a) (`chunks_exact` with a ring of the previous frames) failed on both toolchains: LLVM unrolled the unarmed mono pair by two (the #1454 failure) and V8 carried 4 slots in the 1.99.1 mono armed pair. Shape (c) was not tried. Neither change alone passes on 1.99.1: the offset alone leaves the fallback loop; the fallback alone leaves the mono armed pairs carrying `[rbp-0xf0]`/`[rbp-0xd0]`.

## Evidence (attempt 1)

V8 spill gate, Node v22.23.2, on named twins from `scripts/build-web-audioworklet.sh --module-only --named-twin` (instructions/blocks; carried slots, ceiling):

| row | 1.97.1 base | 1.97.1 branch | 1.99.1 base | 1.99.1 branch |
|---|---|---|---|---|
| dual depth-1 tail | 101/3; 0 | 101/3; 0 | 99/3; 0 | 99/3; 0 |
| dual depth-2 pair (info) | 185/5; 10 | 185/5; 10 | 183/5; 10 | 188/6; 10 |
| dual armed depth-1 tail (c 1) | 116/3; 0 | 116/3; 0 | 122/3; 1 | 122/3; 1 |
| dual armed depth-2 pair (c 11) | 223/5; 11 | 223/5; 11 | FAIL, 2 loops | 227/5; 11 |
| mono depth-2 pair | 82/5; 0 | 81/5; 0 | 81/5; 0 | 81/5; 0 |
| mono depth-1 tail | 43/3; 0 | 43/3; 0 | 43/3; 0 | 43/3; 0 |
| mono depth-2 pair, masked | 88/5; 0 | 88/5; 0 | 88/5; 0 | 87/5; 0 |
| mono armed depth-2 pair (c 0) | 96/6; 0 | 93/5; 0 | FAIL `[rbp-0x78]` | 90/4; 0 |
| mono armed depth-1 tail (c 0) | 51/3; 0 | 51/3; 0 | 51/3; 0 | 51/3; 0 |
| mono armed depth-2 pair, masked (c 0) | 103/6; 0 | 100/5; 0 | FAIL `[rbp-0xe0]` | 97/4; 0 |

Both branch builds: gate exit 0. Class A: the 51 `*output_sha256` fields (42 distinct) of one untimed `bench console` warmup are identical on base `82024d367` and the branch. `g2_kernel_identity` (10 tests), `parametric-eq --release`, `run-wasm-gates.sh`, `check-web-audioworklet.sh` (1.97.1) and `check-browser-expected-resources.py` pass. Mutants: section 1 reading the newest frame, and the short-block fallback skipped, both turn `g2_kernel_identity` red.
