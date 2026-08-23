# Red-mutation log — issue #86 phase A (`miso-engine-builtins-compiler`)

Every gate this phase added or rewrote landed with the one-line mutation that makes it fail. Each
mutation below was applied to the committed code, run, and reverted; the failing test and the
first line of its output are recorded verbatim.

| # | mutation | file | gate it must break | observed |
| --- | --- | --- | --- | --- |
| A1 | drop the `has_sidechain_planes` guard in `gather_sidechain` | `rack/src/lib.rs` | E4 `main_only_scratch_has_two_planes_and_rejects_sidechain_use` | FAILED, `index out of bounds: the len is 0 but the index is 0` (lib.rs:262) |
| A2 | drop the `sidechain && !has_sidechain_planes` guard in `AoSoaScratch::process` | `rack/src/lib.rs` | E4, same test | FAILED at `RejectedBank::metadata`, "the sidechain-plane guard must reject before the bank is consulted" |
| A3 | reinstate `if group.len() == width.lanes()` in the planner | `builtins-compiler/src/lib.rs` | E1 `builtin_bank_layout_regroups_by_dependency_wave_and_scalar_falls_back` | FAILED, `left: [4, 4, 4]` / `right: [1, 4, 4, 1, 4, 3]` |
| A4 | charge four scratch planes per bank (`* 2` → `* 4`) | `builtins-compiler/src/lib.rs` (`builtin_bank_resource`) | E4 `builtin_bank_resource_charges_two_planes_and_actual_members` | FAILED, `left: 2048` / `right: 1024` |
| A5 | charge a full-width member array (`members.len()` → `lanes`) | `builtins-compiler/src/lib.rs` (`builtin_bank_resource`) | E4, same test | FAILED, `left: 2979` / `right: 2923` |
| A6 | revert the attach clause to `members.len() != width.lanes()` | `graph/src/lib.rs` (`with_builtin_banks`) | E5 `with_builtin_banks_accepts_padded_members_and_rejects_empty_or_oversized` | FAILED at the 1-of-4 attach |
| A7 | drop **only** the oversize clause, keeping the empty clause | `graph/src/lib.rs` (`with_builtin_banks`) | E5, same test | FAILED, `left: Err(IncompatibleMembers)` / `right: Err(InvalidMembers)` — proves the width clause is what rejects five distinct members, not the duplicate clause |
| A8 | `inputs.rotate_left(1)` in `build_input_bank`: lane `l` gets member `l+1`'s coefficients | `builtins-compiler/src/lib.rs` | E2 `banked_tracks_are_bit_identical_to_their_scalar_tails` **and** E3 `track_bits_do_not_depend_on_session_track_count` | both FAILED |
| A9 | restore the per-sample counter law (`.saturating_mul(4)` on `frames_processed`) | `builtins-compiler/src/lib.rs` (`BuiltinBankProcessor::process`) | E6, `frozen_issue_037_seeded_builtin_bank_layouts_have_exact_membership_and_counters` | FAILED, `left: 512` / `right: 128` |

## What A8 says about the padding direction

The executor gathers lane `l` from member `l` and scatters lane `l` back to member `l`
(`graph/src/lib.rs`, `render_builtin_bank`), and the bank fills lanes `members.len()..W` with
identity coefficients. Members must therefore occupy lanes `0..n`. Placing them at `W-n..W`
instead would be arithmetically equivalent *only* if the gather moved with them — nothing in the
bank is cross-lane (D9) — so it is not an observable-behaviour mutation of the bank but a
desynchronisation of the executor against it. A8 is exactly that desynchronisation expressed as a
one-line change, and it is red on both identity gates. The direction is stated as an invariant in
the `GraphPreparedBuiltinBank` doc comment rather than tested twice.

## Distinctness guards on E2/E3

Both identity tests would pass vacuously on a silent or degenerate harness. Each therefore asserts
first that every track carries signal (some sample is neither `+0.0` nor `-0.0`) and that no two
tracks render the same bit sequence, before comparing anything. This is the 83d lesson: the first
FMA corpus in that job was inert until a midpoint family was added.

## Structural pins that moved, and the evidence they did not hide a bit change

No PCM value changed in this phase. Four transcript constants moved, all of them folding bank and
tail counts:

| constant | site | why it moved |
| --- | --- | --- |
| seeded layout transcript | `graph-compiler` | folds `count.div_ceil(W)`, tail `0`, `calls * quantum`; all **100** per-layout `pcm_hash` values are byte-identical before and after |
| Issue-037 audit PCM hash | `graph-compiler` | **did not move** — re-pinned from `origin/main` @ `b60f9b8` measured before any edit; it was already stale there because that audit is release-and-env-gated |
| q128 preparation transcript + exact-100 aggregate | `scheduler-fixture` | fold the same counts; the crate's byte-identity PCM gates are untouched and green |
| primitive-owner mirror `447_864` | `capi` | re-derived row by row (2 banks, 9 member ids, no mask row, two-plane scratch) and independently equal to `2 x (207_310 - 205_915) + 445_074` from the production estimate |
