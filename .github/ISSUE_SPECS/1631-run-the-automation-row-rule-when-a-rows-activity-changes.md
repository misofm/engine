# Run the automation row rule when a row's activity changes

Defect found by the stored-automation check (#1058 README, finding F21), decision 17
(`docs/rulings/one-control-abstraction-2026-10-13.md`, C1 and C12). Code anchors verified on
`2e6343a99`. Issue number: assigned at filing (written here as #1631). Feature sprint mode (owner,
2026-10-13): one implementer, normal tests, a batch review.

## Product outcome

A transaction that switches a strip's `matrix_mode` while a stored automation entry targets that
strip's pan or matrix rows never commits a model that its own next rebuild refuses. The switch is
refused with the row rule's `control.inactive` at the entry's path, on both hosts, unless the same
transaction also retargets or removes the entry. Found by reading, not reproduced.

## Context

- `matrix_mode` is live (C12). Its write swaps `MatrixOrPan`, which flips which of the strip's pan
  and matrix rows are active (`write_matrix_mode`, `crates/controls/src/strip.rs:200-227`).
- The row rule refuses an entry on an inactive row with `control.inactive`
  (`controls::automation_diagnostics`, `crates/controls/src/automation.rs:72`, the `active` check).
- The classifier runs the row rule only when the automation table changes: step 2,
  `current.automation != next.automation && !automation_diagnostics(next, ..).is_empty()`
  (`crates/host-core/src/live_delta.rs:371-376`). A switch with an unchanged table passes step 2,
  step 3 lowers it live, and the committed model holds a pan entry on a matrix strip (or the
  reverse). The next rebuild's preparation calls the row rule (`crates/host-core/src/prepare.rs`)
  and refuses that model with `control.inactive`. A document replacement that changes only the
  mode reaches the same path.
- *Make a send's follows_mute and a strip's matrix_mode non-automatable* (#1632) removes the
  automated switch. That narrows the fault class but does not remove it: a live `SET_CONTROL` or a
  document replacement still flips the rows' activity under a pan or matrix entry.

## Decisions frozen for this slice

- **D1. Run the row rule on any commit that changes a row's activity.** Step 2 runs
  `automation_diagnostics(next, ..)` when the automation table changes **or** when any row's
  `active` differs between `current` and `next`. `controls` gives the one predicate
  (`controls::activity_changed(current, next)`): today it is true exactly when a strip's
  `matrix_or_pan` variant differs; a later row whose activity depends on a value joins it there, so
  the classifier keeps no row list.
- **D2. The existing route.** A refused model gives `LiveRebuild::AutomationTarget`; the rebuild's
  preparation refuses it with the row's code at the entry's path, so nothing commits and no ack
  precedes the refusal (the acked-batch question: no). A transaction that switches the mode and
  retargets or removes the entry passes.
- **D3. Rejected: refuse the switch in the write.** `write_matrix_mode` sees one row, not the
  automation table; the row rule is the one author (C1).

## Objective gates

1. `crates/host-core/tests/live_delta.rs`: a strip with a pan entry takes `SET_CONTROL matrix_mode
   = matrix`; the classifier returns `Err(LiveRebuild::AutomationTarget)`. The same transaction on
   the C ABI (`crates/capi/src/runtime/live_tests.rs`) and in the browser
   (`hosts/host-web/tests/control_parity.rs`) is refused `control.inactive` at
   `$.automation[id=<id>].target`, and the committed revision is unchanged.
2. The switch together with `SetAutomationTarget` to the matching matrix row, in one transaction,
   commits.
3. A switch on a strip with no pan or matrix entry stays `live`, with no rebuild.
4. `cargo test --locked -p controls -p host-core --all-features -p capi -p host-web --features
   host-web/test-support`; clippy `-D warnings`; `cargo fmt --all -- --check`.

## Authorized paths

- `crates/host-core/src/live_delta.rs` (step 2), `crates/controls/src/{lib.rs,diff.rs}` (the
  predicate), `crates/host-core/tests/live_delta.rs`, `crates/capi/src/runtime/live_tests.rs`,
  `hosts/host-web/tests/control_parity.rs`, this spec's decision record.

## Dependencies

- None. Independent of the stored-automation slices and of #1632.

## Size

Half a day.

## Decision record: #1631 (batch 8)

Shipped. Anchors moved: step 2 sits at `crates/host-core/src/live_delta.rs` ~344 (the spec said
371-376); the rule is otherwise as filed.

- `controls::activity_changed(current, next)` (`crates/controls/src/diff.rs`, re-exported from
  `lib.rs`): true when the two models do not list the same strips in the same order, or when a strip's
  `matrix_or_pan` variant differs. A later value-dependent row joins it there.
- Classifier step 2 runs `automation_diagnostics(next, ..)` when the automation table changes or
  `activity_changed` holds. A refusal is `LiveRebuild::AutomationTarget`; the rebuild's preparation
  refuses it `control.inactive` at `$.automation[id=<id>].target` before any commit.
- Departure (test shape): the browser parity gate is a separate test, not a script row. A rebuild
  route refusal is a non-ok entry result with the diagnostic in the last-error buffer on both hosts
  (browser 10, C ABI 5: each host numbers its results), not a typed verdict frame, and the script
  harness asserts an ok result. `CAbi::send_status` and `CAbi::last_error` were added to
  `control_parity.rs` for it (the diagnostic is read before the reliable-lane drain, which clears it).

Gates and tests:
- Gate 1 and 3 (classifier): `host-core/tests/live_delta.rs::a_mode_switch_that_strands_an_automation_needs_a_rebuild`
  (pan entry then matrix switch, and the reverse, give `AutomationTarget`; another strip's entry and a
  strip with only a fader entry stay live).
- Gate 1 and 2 (C ABI): `capi/src/runtime/live_tests.rs::set_control::a_mode_switch_under_a_stored_pan_entry_is_refused_before_any_commit`
  (refused, diagnostic names `control.inactive` and the path, state unchanged; switch plus
  `SetAutomationTarget` commits; an entry-free strip switches live).
- Gate 1 and 2 (parity): `hosts/host-web/tests/control_parity.rs::a_mode_switch_under_a_stored_pan_entry_is_refused_alike_on_both_hosts`
  at all four rates (same diagnostic bytes; the same expected revision then commits the switch with
  the retarget, `model_only` because the switch keeps the effective 2x2).
- Red on revert: all three fail with step 2 restored to the table-only condition.
