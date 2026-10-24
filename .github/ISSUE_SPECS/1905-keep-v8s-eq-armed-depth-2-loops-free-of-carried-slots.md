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
