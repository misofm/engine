# Hoist arena slice derivation out of the master reduction loop

## Product outcome

`reduce_many` re-derives each input's arena slice (bounds check, offset multiply, raw-parts) inside the vector loop because `lease.write` takes `&mut self`, and indexes with `&source[index..]`. Derive every slice once and iterate with `chunks_exact`. Class A.

## Root evidence

- `crates/graph/src/runtime.rs:291` `reduce_many`; `ArenaLease::write_read` (`crates/engine/src/realtime/disjoint.rs:309`) and `read` near `:198-213`; the pattern to copy is `ordered_accumulate_block` (`crates/lane/src/kernels.rs:644`), which takes slices once.

## Smallest closable slice

Authorized paths: `crates/engine/src/realtime/disjoint.rs` (a `write_read_many::<N>` returning one output and up to eight input slices with the same disjointness proof as `write_read`), `crates/graph/src/runtime.rs` (`reduce_many`), their tests, and this spec.

## Non-goals

No change to summation order or to single-input reduction.

## Objective gates

1. Existing reduction bit-identity tests and `scripts/check-graph-determinism.sh` pass; new test: random N-input reductions bit-identical before/after.
2. `scripts/check-realtime-policy.sh`; `cargo test -p engine -p graph` green.

## Dependencies

None.

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A means the change moves no rendered bit: same arithmetic in the same order, fewer passes, loads, stores, copies or branches. Every gate below that says "bit-identical" is a hard stop, not a tolerance.
- Render paths stay allocation-free, lock-free and syscall-free (`scripts/check-realtime-policy.sh` is mandatory). Only `crates/lane` may name `wide` or intrinsics (`scripts/check-lane-policy.sh`).
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, and the focused tests named below before every checkpoint. Commit on a `codex/<issue>-<slug>` branch from synchronized `main`; do not touch paths outside the authorized list.
- Do not quote a projected saving. If a benchmark row is listed, run it exactly once, one warmup and two measured rounds, and attach the record as descriptive evidence.
- Source of these findings: `docs/audits/render-path-cost-audit-2026-09-24.md` (PR #879) and tracker #349.

## Attempt 1 evidence

Implementer: Terra (attempt 1). Branch `codex/898-reduce-many-slices-once`; implementation checkpoint `633ab4aa`.

### Design

`ArenaLease::write_read_many::<N>(plane, out, &[u32; N]) -> Option<(&mut [f32], [&[f32]; N])>` (`N` in `1..=8`, enforced at compile time) forms one output slice and `N` input slices in one call. It uses `write_read`'s disjointness argument (I1 for the output, I2/E1 for the reads, distinct buffer indices within one plane). Its premises are checked in release, once per call and before any reference is formed: plane in range, output in the write set, every read reserved, no read equal to the output. A failed premise returns `None`. `reduce_many` now takes the whole edge list and splits it into consecutive groups of up to eight. Each group gets one `write_read_many` call, then one pass that walks the output with `chunks_exact_mut` and every input with its own `chunks_exact` iterator. The first group stores `in0 + in1 + ...`. Each later group reloads that running sum from the output and keeps adding. A store/reload moves `f32` bits unchanged, and every add still takes (running sum, next input) in edge order, so each frame is still `((in0 + in1) + in2) + ...` for any fan-in. The scalar tail runs the same body at `L = f32`. The fan-in 0 fill and the fan-in 1 copy/in-place arms of `reduce_plane` are unchanged.

### Tests added

- `engine` `realtime::disjoint::tests::write_read_many_forms_the_slices_the_single_borrows_form`: at `N` = 1, 2, 3, 8 and 8-with-repeats-and-silence, in both planes, every slice has the same address, length and words as the single-buffer `read`/`write` borrow. A write through the output lands only in `out`'s plane.
- `engine` `realtime::disjoint::tests::write_read_many_refuses_every_unsound_borrow`: `None` for read == output, output outside the write set (foreign, silence, unreserved, `u32::MAX`), unreserved read, and out-of-range plane.
- `graph` `runtime::tests::random_fan_in_reductions_are_bit_identical_before_and_after_slice_hoisting` (the gate-1 test): 150 cases. Fan-in is random in `2..=64`, plus fixed at 2, 7, 8, 9, 15, 16, 17, 63, 64 and 65. Frames are random in `1..=131`, plus 1, 13 and 128. The corpus uses hostile words: magnitudes over `2^-24..2^25`, signed zeros, subnormals, one-signed infinities and one NaN payload. Edge lists include repeats and silence, and both planes differ. `reduce_many::<f32 | Simd4 | Simd8>` must be bit-identical to `frozen_per_vector_reduce_many` (the pre-#898 kernel, copied verbatim into the test module) and to the scalar left-to-right `reduce`. The production `reduce_plane` must also match the scalar reference, and every input buffer must come back unchanged. The test also asserts that its own corpus distinguishes a fresh-subtotal-per-group sum and a reversed-group-order sum from the reference.
- `graph` `runtime::tests::negative_zero_survives_every_reduction_group_edge`: all-`-0.0` inputs at fan-in 2, 8, 9, 16, 17, 64, 65 stay `-0.0`.
- Updated caller: `assert_width_matches_old` (9 inputs, so it now crosses a group edge) calls the new `reduce_many` signature.

### Gates

| gate | command | result |
|---|---|---|
| 1 | `cargo test -p graph --lib -- reduc hoisting negative_zero every_lane_width folded_epilogue` | 13 passed (debug); the same 13 passed with `--release` |
| 1 | `bash scripts/check-graph-determinism.sh` | `graph fresh-process determinism: PASS (100/100)`; the evidence JSON is byte-identical to the one recorded before the change |
| 2 | `bash scripts/check-realtime-policy.sh` | `realtime policy: ok (50 marked regions in 14 files)` |
| 2 | `cargo test -p engine -p graph` | engine 40 + 4 + 1 doc; graph 83 + 1 + 1; 0 failed |
| std | `cargo fmt --all --check` | clean |
| std | `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | exit 0 |
| extra | `cargo test -p graph-compiler --lib -- frozen_issue_037_seeded_builtin_bank_layouts_have_exact_membership_and_counters` (M1c production-corpus reduction order) | 1 passed |

The two scripts are committed without the executable bit, so they were run through `bash`.

Red mutations. Each was applied to `633ab4aa`, run against the named test, and reverted:

| mutation | test | result |
|---|---|---|
| every group stores (the running sum is dropped) | `random_fan_in_...` | RED |
| groups visited in reverse | `random_fan_in_...` | RED |
| a continuation group seeds from `0.0` and adds the running sum last | `negative_zero_survives_...` and `random_fan_in_...` | RED, RED |
| scalar tail skipped | `random_fan_in_...` | RED |
| reads offset into the other plane | `write_read_many_forms_...` | RED |
| reads off by one buffer | `write_read_many_forms_...` | RED |
| read-is-output check removed | `write_read_many_refuses_...` | RED |
| reserved-read check removed | `write_read_many_refuses_...` | RED |
| write-set check removed | `write_read_many_refuses_...` | RED |

Disclosed as not run: removing the plane check. The failing case would perform out-of-bounds pointer arithmetic, which is undefined behavior. Disclosed equivalent: swapping the operands of one add. With a single NaN payload it is equivalent, because IEEE addition is commutative on every other value.

Codegen (descriptive only, no timing). In `cargo rustc --release -p graph --lib -- --emit asm` (x86-64-v3), `reduce_plane` is emitted with the group kernels inlined. The accumulate-form group-of-eight loop is: one `vmovups` reload of the running sum, eight `vaddps` in edge order from base pointers held in registers, one `vmovups` store, and the loop counter. No lease access, offset multiply or bounds check remains in it. `reduce_plane`'s only four `panic_bounds_check` sites belong to the unchanged fan-in-0 fill (`write`) and fan-in-1 copy (`write_read`) arms.

### Deviations and disclosures

- **Output traffic above fan-in 8.** The spec's one-to-eight accessor means a fan-in above eight carries the running sum through the output buffer. That is `ceil(N / 8)` passes over the output, and each later group adds one load and one store of the output per vector. For the 64-track master that is 8 stores and 7 reloads per vector where the old fused loop did 1 store. In exchange, the per-input, per-vector slice derivation is gone: write-set table bounds check, offset multiply, cell bounds check and `&source[index..]` check. `fold_cohort` uses the same carry through `ordered_accumulate_block`. The arithmetic and its order are unchanged, and no saving is claimed.
- **Accessor shape.** `write_read_many` returns `Option` and checks its premises in release, where `write_read` relies on a `debug_assert`. It derives pointers with `UnsafeCell::raw_get` from the whole-allocation `cells.as_ptr()`, not through `cells[start].get()`, which avoids per-slice bounds checks after the up-front checks.
- **Output aliasing an input.** The old `reduce_many` tolerated an output that was also one of its inputs, reading the pre-store value. The new one refuses: it debug-asserts and returns in release. A lowered program cannot produce that shape, because `program::lower` retires a slot one op after its last reader. `program::tests::sixty_four_plain_tracks_keep_one_ordered_non_aliasing_master_reduction` passes.
- **Local kernel.** `lane::kernels::ordered_accumulate_block` was copied as a pattern, not called. It indexes `&input[index..]`, and `crates/lane` is outside the authorized paths. `reduce_many` is private; its signature changed from `(first, second, rest)` to `(inputs)`.
- **Mutation ledger.** `crates/graph/tests/MUTATIONS.md` is outside the authorized paths, so the red mutations are recorded here only.
- **Pre-existing clippy failure.** `cargo clippy -p engine -p graph --all-targets -- -D warnings` without `--all-features` fails on dead code inside the untouched `crates/graph/tests/rt9_resident_bank_input_alloc.rs`, which needs `test-support`. The same command with `--all-features`, and the workspace gate, are clean.
- **Spec anchors.** All matched the base tree: `runtime.rs:291`, `disjoint.rs:309`, `read` at `:198`, `kernels.rs:644`.

## Sol attempt 1 verdict: PASS

Adversarial review (Fable 5.1, high effort) against `633ab4aa` and `ee5547bd` on base `6b150fba`.
No blocking findings. Independently verified: the summation is `((in0 + in1) + in2) + ...` in
edge order at every fan-in, across group boundaries, in the scalar tail, and for repeated inputs,
the silence buffer and all-`-0.0` inputs, against a verbatim copy of the old kernel and a scalar
reference at `f32`, `Simd4` and `Simd8`; the corpus discriminates a fresh-subtotal-per-group and a
reversed-group order; `write_read_many` has `write_read`'s disjointness proof, checks every premise
in release before forming a reference, and keeps the workspace's `unsafe` inside `disjoint.rs`;
the refused-borrow return in `reduce_many` is unreachable for lowered programs (`in_place` requires
`single`, slots retire one op late, and the 64-input master test asserts no input aliases the
output); three mutations re-applied and reverted, all red on the random fan-in test.

**Ruling on the group carry (the decision this record exists for).** For fan-in above eight the
running sum travels through the output buffer: `ceil(N / 8)` passes, one extra L1-resident reload
and one extra store per vector per later group. Against the 64-input, 128-frame `Simd8` master
that is 112 extra loads and 112 extra stores to a 512-byte output per plane, in exchange for
removing about nine thousand per-vector slice-derivation instructions and their branches. Read from
the release binary, the eight-input group loop is one reload, eight in-order `vaddps` with memory
operands, and one store. The arithmetic and its order are unchanged, so the class-A criterion that
matters is met; the "fewer stores" clause is not met above fan-in eight and this ruling accepts
that explicitly, because the register-accumulator alternative needs `N` simultaneously live read
slices (heap is forbidden on the render path, a compiled bound would be a `MAX_TRACKS`, and raw
pointers in `graph` would move `unsafe` out of `disjoint.rs`), a vector-outermost re-borrow per
group costs more instructions than it saves, and a larger group constant already spills base
pointers. `fold_cohort` ships the same carry through `ordered_accumulate_block`. No attempt 2.
The spec lists no benchmark row and none is owed; a later measurement would need the 64-track
console's master-reduction share not to rise.

Reviewer-run gates: `cargo test -p engine -p graph`, the focused reduction tests in `--release`,
`check-graph-determinism.sh` (100/100), `check-realtime-policy.sh`, `cargo fmt --all --check`, and
`cargo clippy --locked -p engine -p graph --all-targets --all-features -- -D warnings`, all green.
Not re-verified by the reviewer: the byte-identity of the determinism JSON against a base rebuild
(the script compares fresh processes), and the full-workspace clippy the implementer reports.
