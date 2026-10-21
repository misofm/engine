# Read every capi test once and cut the host twins (TA read, capi)

Owner decision 2026-10-20, relayed by root: read every remaining test once, crate by crate, and
delete each test that a named remaining test already defends (AGENTS.md "Test value"). The owner's
"host twins, then stop" rule applies to this host crate: a claim is tested once, in the crate that
owns its logic, and each host keeps at most one short parity row per host behaviour. This spec
covers the `capi` package. Branch `codex/batch-test-read-capi`, from `main` at `64fe78c46`. No new
scope: findings that need work are listed under "Gaps and successors".

## Method

- Two readers read all 215 `capi` tests in 14 files: c1 read `src/runtime/live_tests.rs`, c2 read
  the 13 other files (`src/` test modules and `tests/`). Per test they answered "which plausible
  defect turns this red that no remaining test catches?" and wrote KEEP, DELETE or MERGE.
- A capi test whose logic lives in the host crate (ABI layout, pointers, lengths and null refusals,
  error-code mapping, the C ABI resource ceilings and boot path) is an adapter test and stays.
- A twin of a claim whose logic lives in another crate (host-core, control-plane, protocol,
  session) is deleted when a named test in the owning crate asserts the same claim. The defender
  had to exist after the other read cuts: host-core defenders were checked at
  `codex/batch-test-read-host-core` (`8c842d3f8`), host-web `tests/control_parity.rs` defenders in
  the host-web read worktree, others at `main`.
- Never-delete classes were kept: realtime and allocation gates, counts and ceilings, ABI and wire
  pins, regression reproducers, seeded differentials, sole red tests of a MUTATIONS.md row, the
  acked-batch question, and tests a workflow or script runs by name.
- The integrator checked every DELETE defender (all 29 exist after every read cut, and none is
  deleted or merged away), read each pair's claims, grepped every removed name across the
  repository, built, removed what clippy reported unused, ran a mutation spot-check, and ran the
  full lint-job gate list.

## Result per file

Tests and test lines as the readers counted them (lines of the test function bodies).

| file | tests before -> after | kept | deleted (lines) | merged (lines) -> new tests |
|---|---|---|---|---|
| `src/runtime/live_tests.rs` | 100 -> 81 | 81 | 19 (1,184) | 0 |
| `src/runtime/tests.rs` | 55 -> 49 | 49 | 6 (309) | 0 |
| `src/runtime/mute_automation_tests.rs` | 4 -> 2 | 2 | 2 (52) | 0 |
| `src/ffi.rs` (tests) | 18 -> 17 | 17 | 1 (65) | 0 |
| `tests/response_get.rs` | 6 -> 5 | 5 | 1 (38) | 0 |
| `tests/resource_lifecycle.rs` | 11 -> 10 | 9 | 0 | 2 (72) -> 1 (45) |
| 8 other files | unchanged | 21 | 0 | 0 |
| **total** | **215 -> 185** | **184** | **29 (1,648)** | **2 (72) -> 1** |

Eight helpers that only the deleted tests used went with them (clippy `-D warnings` reported each
unused): `Rig::current_follow_route_cells`, `live_bypass_pcm_shape`, `builtin_automation_edit`,
`builtin_handle`, `fresh_list`, `live_twin`, `solo_twin` and `mutes_but_eq1`, all in
`live_tests.rs`. The mute module's `run`, `Expect` and `Static` and `response_get.rs`'s `Endpoint`
codec stay used. `mute_automation_tests.rs` keeps
`solo_vca_and_hand_mutes_compose_with_the_curve` as the C ABI parity row for the composed stored
mute (the host-web read cut deleted its twin and relies on it), and its header now names what the
file tests and the host-core owner tests.

Diff against `64fe78c46`: 10 files, +75/-2,057 before this spec.

## The deletes and their defenders

host-core paths are `crates/host-core/...`; control_parity is
`hosts/host-web/tests/control_parity.rs`.

- `live_tests.rs` (19): host-core `tests/live_delta.rs` (10:
  `records_carry_the_session_ramps_and_the_models_own_pan_window`,
  `a_prepared_bypass_change_needs_a_rebuild`,
  `a_static_filter_edit_that_crosses_an_automated_curve_needs_a_rebuild`,
  `filter_records_are_the_preparers_targets_for_the_changed_values`,
  `output_route_values_stay_rebuilds`, `a_rebuild_only_row_is_a_rebuild_with_its_reason`,
  `vca_and_follow_records_take_the_ramp_of_the_edit_that_moved_them`,
  `a_change_of_the_monitor_overlay_alone_is_the_other_strips_mutes_and_the_followers_gate`,
  `a_fader_automation_edit_is_a_rebuild`), `src/control_provider.rs` (5:
  `a_staged_live_commit_gives_the_list_of_a_full_build` twice,
  `a_commit_refreshes_values_and_only_a_changed_address_sequence_advances_the_epoch` three times),
  `tests/live_transaction.rs::one_transaction_is_applied_whole_in_the_block_its_revision_is_published_for`,
  `tests/strip_handles.rs::handles_list_tracks_then_submixes_and_file_bus_effects`,
  `tests/live_addressing.rs::a_live_bypass_reaches_the_addressed_lane`,
  `tests/route_mute.rs::removing_a_source_mute_curve_of_a_following_send_lands_on_the_fresh_plan`;
  and protocol `tests/edit_ramps.rs::set_control_records_the_row_its_address_names`.
- `tests.rs` (6): control_parity
  `re_authoring_keeps_the_hand_offset_and_removal_ends_it_alike_on_both_hosts` (the C ABI's stale
  epoch refusal) and `a_plain_session_seek_declares_and_an_anchored_one_does_not_alike_on_both_hosts`
  (twice); host-core `tests/prepare.rs::prepare_reports_the_session_shape_and_feeds_sources_independently`,
  `tests/successor_swap.rs::removing_a_limited_track_keeps_the_output_timing_at_every_rate_and_width`,
  `src/source.rs::tests::a_refusal_pushes_nothing_and_publishes_nothing`.
- `mute_automation_tests.rs` (2): host-core `tests/stored_mute_automation.rs`
  `a_mute_step_mutes_the_lane_at_its_exact_sample_over_the_mute_length` (exact samples, mute length,
  quanta 128/100/37, every bank form) and
  `a_seek_that_leaves_the_mute_unchanged_lets_the_unmute_ramp_finish` (its seek-into-muted-span
  partner: muted exactly from the seek). The curve's next step after the seek, on the new
  timeline, is defended by the kept host-web copy
  `hosts/host-web/tests/stored_mute_automation.rs::a_seek_into_a_muted_span_mutes_exactly_from_the_seek`,
  and control_parity `a_composed_stored_mute_renders_alike_on_both_hosts` runs that seek on the C
  ABI. The owner, host-core, lacks this test: see the host-web successor note.
- `ffi.rs` (1): `empty_object_document_is_diagnosed_not_aborted`, a #387 reproducer whose logic is the
  session preflight. Host-twin exception: the owner reproducer
  `crates/session/tests/json_grammar.rs::empty_object_refuses_at_regression_placements_instead_of_corrupting_the_code_map`
  stays, and the C ABI's compile-rejection path stays reached by `resource_lifecycle.rs`'s
  `assert_count_refusal` rows.
- `response_get.rs` (1): protocol
  `src/controller/tests.rs::response_get_dispatches_refuses_and_replays_through_the_controller`.
- MERGE: `maximum_submixes_bounds_submixes_and_zero_defers_to_maximum_tracks` and
  `maximum_vcas_bounds_vcas_and_zero_defers_to_maximum_tracks` became
  `maximum_submixes_and_vcas_bound_their_kind_and_zero_defers_to_maximum_tracks`, one row per word;
  every case of both stays (ceiling claim).

Integrator notes on composed defences (each read, none restored):
`a_vca_added_live_advances_the_epoch_and_lists_its_rows_live` is the epoch rule
(`control_provider.rs::tests::a_commit_refreshes_values_and_only_a_changed_address_sequence_advances_the_epoch`)
plus the VCA rows' live class (`crates/controls/tests/list.rs::classes_match_decision_14`, line 173);
`a_live_track_edit_beside_a_submix_strip_reaches_its_track` is the strip table order
(`strip_handles.rs`) plus records addressed by strip ID (`live_delta.rs::a_submix_strip_fader_mute_and_pan_are_live`);
`a_live_commit_gives_the_control_list_of_a_fresh_session`'s followed-mute case is
`control_provider.rs::tests::a_followed_mute_refreshes_the_send_gate_of_an_unchanged_route`, and its
insert-bypass case is the same owner-run stage that every strip edit runs over the strip's insert
section; the successor half of
`a_fader_entry_added_after_an_unwritten_smoothing_change_reads_the_new_lengths` is the kept capi test
`a_successor_prepared_with_a_control_smoothing_edit_holds_the_new_lengths`.

## Restores

None. No delete failed the defender cross-check, and no spot-check mutant needed a deleted test.

## Never-delete keeps by class

Readers' class labels on KEEP rows (one class per test): allocation claims 25; ceiling claims 15
(16 with the merged test); count claims 15; ABI gates 18 and wire-format gates 2; the acked-batch
question 11; realtime gates 5; regression reproducers 6; MUTATIONS.md sole red tests 4 (rows in
`crates/host-core/tests/MUTATIONS.md` and `hosts/host-web/MUTATIONS.md`); seeded differentials 1.
Parity rows 23 and adapter tests 14. The other 45 KEEP rows name a unique claim without a class;
most are control-plane claims that only the C ABI tests (see the successor below).

## Mutation spot-check

Seed `20261020`; `random.Random(20261020).sample` of 15 from the 59 current `capi` mutants and 15
from the 1,477 current `host-core` mutants (`cargo mutants --list --json --diff`, cargo-mutants
27.1.0) whose function the 29 deleted tests covered (coverage map `cov.tsv`/`fns.tsv` at
`81a3074fc`, joined by file and function name). A capi mutant takes about 3 minutes, so to stay near
one hour of machine time 18 of the 30 drawn ran: capi 0-7 and host-core 15-24 of the draw. Each was
applied with `git apply`; a host-core mutant first ran the 20 named host-core defenders, then
`cargo test --locked --all-features -p capi --no-fail-fast` ran; the file was reverted
(signal-safe; the tree was clean at the end). Survivors then ran the full
`cargo test --locked --all-features -p host-core` on the merged tree (after the host-core cut), and
the 29 deleted tests at `64fe78c46` with the mutant in place.

- 2 unviable (do not compile).
- 13 of 16 viable caught by a remaining test: 11 by a host-core defender or a capi test (mutant
  24, `check_response_axis` `<=` to `>`, by a timeout of the host-core defenders at 1,800 s), and 2
  (`follow_route_jumps` to `Default`, `resolve_spectrum_carry` `!=` to `==`) by other remaining
  host-core tests (`route_mute.rs`, `spectrum_carry.rs`, `prepare::tests`). For
  `follow_route_jumps` the deleted
  `removing_a_source_mute_curve_of_a_following_send_lands_on_the_fresh_plan_on_the_c_abi` was red
  too, but four remaining host-core tests are red, so it is not restored.
- 3 not caught by any remaining test that ran, and none of the deleted tests is red on them either
  at `64fe78c46`, so the cut lost no catcher; they are older gaps (host-web was not run):

| # | function (file:line) | mutant |
|---|---|---|
| 5 | `clear_engine_error` (`crates/capi/src/ffi.rs:45`) | body replaced with `()` (a success leaves the last error text) |
| 21 | `join_strips` (`crates/host-core/src/prepare.rs:5933`) | `<<` to `>>` on the right fader lane's continued bit |
| 22 | `classify_rows` (`crates/host-core/src/live_delta.rs:608`) | `\|\|` to `&&` in the strip/route count guard |

## References fixed

- `.github/ISSUE_SPECS/1336-...`, `1340-...` and `1654-...`: citations of deleted capi tests now
  name their host-core defender, or drop the deleted name from a list.
- `docs/C_ABI_V1_QUALIFICATION.md` (#1285 evidence): now names host-core
  `successor_swap.rs::removing_a_limited_track_keeps_the_output_timing_at_every_rate_and_width` and
  the control_parity row `removing_a_limited_track_keeps_the_plan_latency_alike_on_both_hosts`.
- `capi` has no MUTATIONS.md, and no MUTATIONS.md row names a deleted capi test. `scripts/`,
  `.github/workflows/` and `tools/` name none. Dated records under `docs/handoffs/` and
  `docs/audits/` stay as written.

## Gaps and successors

- Logic-owner tests missing (successor, not this PR):
  `live_tests.rs::set_control::automated_hand_move::a_zero_offset_renders_the_static_fader` is a
  host-core claim (a `+0.0` or `-0.0` hand offset renders the static fader bits) that only capi
  tests; it uses only host-core plans. Add the host-core test, then delete the capi one in the same
  PR. Readers also marked about 15 `live_tests.rs` KEEPs as control-plane claims with no
  control-plane test (successor base from the committed model, edit ramps into the successor,
  replay writes nothing, routing a write to the pending producer, drain-then-carry, commit events,
  the EQ owner push and lookup, the submix ramp carry, the smoothing seed, the fader ride carry
  cases). The successor writes each claim once in control-plane and deletes the capi twin.
- Solo and VCA terms on a stored mute curve: only capi
  `mute_automation_tests.rs::solo_vca_and_hand_mutes_compose_with_the_curve` checks them composing
  with a stored curve against a static-plan oracle; host-core covers them only in pieces.
  Successor: one host-core test of the full composition; this capi test then stays as the parity row.
- Spot-check survivors (older gaps, not caused by this cut). Successor: one test per claim in
  the owning crate: capi, a successful call after a failed one reads an empty last error; host-core,
  a strip join whose right fader lane continues keeps that lane's bit; host-core, a row-only
  transaction whose strip, route or submix count differs leaves the row fast path (or record the
  guard as a pure fast-path guard if the fallback gives the same delta).

## Gates

On the merged tree (branch at the merge of `main` `d85e38442`):

- All 31 `run:` steps of the qualification `lint` job (fmt, workspace clippy `-D warnings`,
  `cargo doc -D warnings`, every hermetic policy gate and the three x86 lane-guard probes), from
  `policy-job.sh` pointed at this worktree: PASS.
- `cargo test --locked --all-features -p capi`: PASS, 185 tests (capi is the only package touched).
- `scripts/check-capi-abi.sh` and its `--self-test` (the C ABI link, symbol and consumer gate the
  lint job does not run): PASS.
- After the last header edit in `mute_automation_tests.rs` (comments only): fmt, `cargo clippy -p
  capi --all-targets --all-features -D warnings` and `cargo doc -p capi -D warnings`: PASS.

The per-test verdict list (215 rows) is one part, posted as an issue comment.
