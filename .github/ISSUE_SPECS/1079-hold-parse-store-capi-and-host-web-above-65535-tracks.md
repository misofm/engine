# Hold document parse, the protocol session store, capi prepare and host-web boot above 65,535 tracks

Successor B of #1045 (Sol verdict, attempts 3-5). Engine rule: arbitrary track counts, never a compiled `MAX_TRACKS`. Nothing parses a session document of this size today (#1046 deleted `session/tests/descriptive_scale.rs`, which was ignored and never scheduled). A 65,537-track document is 93.7 MB and takes 19.3 s to parse in debug, so this is nightly work.

## Smallest closable slice

In a release-mode nightly gate, parse one 65,537-track document once and drive each entry point from that parse: the session parser, the protocol session store edits, capi prepare and host-web boot. Assert each accepts it (or refuses with the documented configured-resource diagnostic, never a hard cap).

## Gates

1. A planted track cap at each of the four entry points turns the nightly gate red; green when reverted.
2. The job's time and peak memory are stated and fit the nightly runner; routing and actionlint pass.

## Decision record (2026-10-24, batch 13)

- R1 (root): no host-web leg. `MAXIMUM_DOCUMENT_BYTES = 1 << 20` (`hosts/host-web/src/lib.rs`) refuses an 89 MB document for its size before any track is seen; that ceiling is a documented resource, not a track cap, so a host-web row would test the wrong claim. Gate 1's four entry points are three.
- R2 (root): the capi row lives in host-core. capi prepare is `host_core::prepare_host_session` behind FFI and adds no track arithmetic.
- Each row builds its 65,537-track model in memory from a checked-in fixture (no 89 MB file in the tree) and runs release-only, `#[ignore]`, with release overflow checks on:
  - `session/tests/scale_transaction.rs::parses_a_65_537_track_document_back_to_its_canonical_model`: the canonical writer's document parses back to 65,537 tracks and re-canonicalises to itself.
  - `protocol/tests/scale.rs::a_fader_edit_on_the_strip_at_index_65536_commits_and_snapshots`: a `SET_CONTROL` fader edit on the store's strip at index 65,536 commits and only that strip moves in the snapshot. The compiled model orders tracks by ID, so that strip is `track-9999`, not `track-65536`: an edit on `track-65536` (index 3 in ID order) would not reach a narrowed index.
  - `host-core/tests/scale.rs::prepares_65_537_tracks_or_refuses_only_a_resource`: `prepare_host_session` with every count and byte cap at `u64::MAX` accepts (it does today) or refuses only `PrepareRejection::Resource`.
- Red first (PR evidence, mutants reverted): a parser list index narrowed to `u16` (`parse_list`) turns the session row red (`WrongType`); a strip lookup bounded to the first 65,536 tracks (`controls::strip::strip_mut`) turns the protocol row red (`AddressUnknown`); host-core's track count narrowed to `u16` (`prepare.rs`) turns the host-core row red (`Platform`, `host.count`). A `u16` narrowing of the binary search in `controls::registry::strip_index` stays green: the scan fallback finds the strip, so it is no defect.
- Gate 2, measured on the 32-core box (release, test time only; peak RSS of the test process): session 3.1 s, 1.1 GB; protocol 3.3 s, 1.1 GB; host-core 16.9 s, 2.1 GB. The document is 88.8 MB.
- Scheduling: the three nightly lines go in `nightly.yml`'s #1045 release-budgets block, and `scripts/check-ci-path-routing.py` pins that block's exact command list (`NIGHTLY_BUDGET_COMMANDS`), so the same three lines go there too. Root authorized both paths (`.github/workflows/nightly.yml`, `scripts/check-ci-path-routing.py`); `check-ci-path-routing.py` passes with them.
- Overlap with #1947 (a kitchen-sink nightly session at 65,537 tracks): #1947 owns one session that exercises every feature at that size; these rows hold one entry point each on a plain strip. A #1947 session that parses, edits and prepares through these entry points may supersede them; then this issue's rows are deleted in #1947's PR.
