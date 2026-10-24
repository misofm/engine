# Hold document parse, the protocol session store, capi prepare and host-web boot above 65,535 tracks

Successor B of #1045 (Sol verdict, attempts 3-5). Engine rule: arbitrary track counts, never a compiled `MAX_TRACKS`. Nothing parses a session document of this size today (#1046 deleted `session/tests/descriptive_scale.rs`, which was ignored and never scheduled). A 65,537-track document is 88.8 MB and takes 19.3 s to parse in debug, so this is nightly work.

## Smallest closable slice

In a release-mode nightly gate, parse one 65,537-track document once and drive each entry point from that parse: the session parser, the protocol session store edits, capi prepare and host-web boot. Assert each accepts it (or refuses with the documented configured-resource diagnostic, never a hard cap).

## Gates

1. A planted track cap at each of the four entry points turns the nightly gate red; green when reverted.
2. The job's time and peak memory are stated and fit the nightly runner; routing and actionlint pass.

## Decision record (2026-10-24, batch 13)

- R1 (root): no host-web leg. `MAXIMUM_DOCUMENT_BYTES = 1 << 20` (`hosts/host-web/src/lib.rs`) refuses an 88.8 MB document for its size before any track is seen; that ceiling is a documented resource, not a track cap, so a host-web row would test the wrong claim. Gate 1's four entry points are three.
- R2 (root): the capi row lives in host-core. capi prepare is the live-attached runtime entry `host_core::prepare_host_runtime_with_live_lanes_and_spectrum`, called by `CapiPreparer::prepare` (`crates/capi/src/runtime/mod.rs`) behind FFI. It is not `prepare_host_session`, which attaches no live controls. capi adds no track arithmetic of its own.
- Each row builds its 65,537-track model in memory from a checked-in fixture (no 88.8 MB file in the tree) and runs release-only, `#[ignore]`, with release overflow checks on:
  - `session/tests/scale_transaction.rs::parses_a_65_537_track_document_back_to_its_canonical_model`: the canonical writer's document parses back to 65,537 tracks and re-canonicalises to itself.
  - `protocol/tests/scale.rs::a_fader_edit_on_the_strip_at_index_65536_commits_and_snapshots`: a `SET_CONTROL` fader edit on the store's strip at index 65,536 commits and only that strip moves in the snapshot. The compiled model orders tracks by ID, so that strip is `track-9999`, not `track-65536`: an edit on `track-65536` (index 3 in ID order) would not reach a narrowed index.
  - `host-core/tests/scale.rs::prepares_65_537_live_attached_tracks_as_capi_does`: `compile_host_session`, then `prepare_host_runtime_with_live_lanes_and_spectrum` with `CapiPreparer`'s request (observation slots for `launch_maximum_observation_taps()`, the control plane's `controls::live_lanes` with the effect lanes on, no spectrum, no successor), every count and byte cap at `u64::MAX`. It asserts `Ok`: no fixed limit is documented on this path, and the session is accepted (run of 2026-10-24, attempt 2, 17.7 s). It also asserts 65,537 strip producers, `strip_position` finds `track-9999` at index 65,536, and that strip's console EQ producer (`LiveEffectAddress::console(0)`) resolves. Attempt 1 called `prepare_host_session`, which skips the live half; the verifier's mutant below stayed green on it.
- Red first (PR evidence, mutants reverted): a parser list index narrowed to `u16` (`parse_list`) turns the session row red (`WrongType`); a strip lookup bounded to the first 65,536 tracks (`controls::strip::strip_mut`) turns the protocol row red (`AddressUnknown`); host-core's track count narrowed to `u16` (`prepare.rs`) turns the host-core row red (`host.count`); the verifier's `.take(65_536)` on the `live_control_strips` track iterator (`prepare.rs`) turns it red (`host.meter.order`). A `u16` narrowing of the binary search in `controls::registry::strip_index` stays green: the scan fallback finds the strip, so it is no defect.
- Gate 2, measured on the 32-core box (release, test time only; peak RSS of the test process): session 3.1 s, 1.1 GB; protocol 3.3 s, 1.1 GB; host-core (live-attached) 17.7 s, 2.6 GB. The document is 88.8 MB.
- Scheduling: the three nightly lines go in `nightly.yml`'s #1045 release-budgets block, and `scripts/check-ci-path-routing.py` pins that block's exact command list (`NIGHTLY_BUDGET_COMMANDS`), so the same three lines go there too. Root authorized both paths (`.github/workflows/nightly.yml`, `scripts/check-ci-path-routing.py`); `check-ci-path-routing.py` passes with them.
- Overlap with #1947 (a kitchen-sink nightly session at 65,537 tracks): #1947 owns one session that exercises every feature at that size; these rows hold one entry point each on a plain strip. A #1947 session that parses, edits and prepares through these entry points may supersede them; then this issue's rows are deleted in #1947's PR.
