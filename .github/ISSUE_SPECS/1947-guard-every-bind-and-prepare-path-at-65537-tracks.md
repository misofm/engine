# Guard every bind and prepare path at 65,537 tracks with one kitchen-sink nightly session

Filed by root as #967's R1 successor (backlog sprint, 2026-10-24).

## Finding

#967 linearised `scatter_redirects` and reshaped the nightly 65,537-track test (a compiled routes+console phase and a hand-built banks-plus-route-per-track phase). Other per-track structures still have no guard at that scale: sources, observers and meters, sidechains, sends and submixes, and stored automation. A quadratic scan in any of them would pass every committed test.

## Smallest closable slice

One nightly (ignored, release) session at 65,537 tracks that carries sources, a meter per strip, a sidechained insert on some tracks, sends into submixes, and stored automation on faders, with a bound chosen as in #967 (several times the measured linear time, well under a quadratic time measured once on the pre-fix code or estimated by doubling). Assert the exact counts of each structure. Split into two nightly sessions if one exceeds 60 s (#967 R2). Fix any quadratic path the session exposes in its own follow-up.

## Paths

`crates/graph-compiler/tests/scale.rs` or `crates/host-core/tests` (whichever owns prepare), `.github/workflows/nightly.yml` only if a new test name must be listed, this spec. After #967 merges.

## Decision record (2026-10-25, batch 13, attempt 1)

- R1 (root): the row lives in `crates/host-core/tests/scale.rs` and replaces #1079's host-core row `prepares_65_537_live_attached_tracks_as_capi_does` (same live-attached entry `CapiPreparer::prepare` calls: `compile_host_session`, then `prepare_host_runtime_with_live_lanes_and_spectrum` with an observation slot for every declared tap, the control plane's lanes with the effect lanes on, no spectrum, no successor, every cap at `u64::MAX`). It keeps #1079's assertions: 65,537 tracks, `strip_position` finds `track-9999` at index 65,536, its console EQ producer resolves. With 64 submixes the strip producer count is now 65,601 (tracks plus submixes), not 65,537. #1079's session and protocol rows stay: they own other claims in other crates. The renamed test is one line in `nightly.yml`'s release-budgets block and one in `scripts/check-ci-path-routing.py`'s `NIGHTLY_BUDGET_COMMANDS` (root authorized both).
- R2 (root): the session, built in memory from `parametric-eq-nine-track.json`: 65,537 tracks, each on its own source `src-{i}`, with the console EQ, a two-segment fader automation and a post-fader send into `bus-{i % 64}`; 64 unity submixes routed to the output; a `miso.compressor` insert keyed (`SidechainDeclaration::Routed`) by track `i - 1`'s post-fader tap on every track with `i % 8 == 7` (8,192; the phase puts one on `track-9999`); one VCA (-3 dB) whose members are every track, in ID order.
- Name: `prepares_a_65_537_track_kitchen_sink_session_live_attached`. It prepares the session, then asserts exact counts on the prepared result: sources 65,537 (`PreparedHost::sources` and the inventory), meters 65,601 (handles and inventory), live sends 65,537 (`route_controls` and `live_route_count`), automated faders 65,537 (strip producers that hold offsets cells), effect producers 73,793 (65,601 console EQs plus 8,192 compressors), compressor observations 8,192. The prepared result exposes no count of VCA reach or sidechain edges, so the row also renders 8 blocks with only `src-9998` and `src-9999` fed, and the output must equal, bit for bit, the same session built from those two tracks only: the strip at index 65,536 must get its own source, its automation, its VCA offset and its sidechain key. Each of those three last inputs changes the pair's bits (checked once, PR evidence).
- Bound: 100 s on the whole row (document build, both prepares, both renders), about 3.7 times its measured time. Under 60 s, so no split (#967 R2).
- No feature was refused or capped at 65,537 tracks.

## Evidence (2026-10-25, 32-core box, release with overflow checks)

Doubling (one invocation, descriptive; document build plus prepare plus 8 render blocks):

| Tracks | Build | Prepare | Render | Total | Ratio to the row above |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 8,192 | 0.09 s | 2.49 s | not run (no `src-9999`) | 2.58 s | |
| 16,384 | 0.18 s | 4.64 s | 0.42 s | 5.23 s | 2.03 |
| 32,768 | 0.35 s | 10.36 s | 0.85 s | 11.57 s | 2.21 |
| 65,537 | 0.72 s | 22.01 s | 1.71 s | 24.44 s | 2.11 |

Every doubling is about 2x: no path in this session is quadratic. The row itself (one prepare of each session and both renders), timed alone under the exclusive lock: 26.7 s, peak RSS 4.19 GB (`/usr/bin/time -v`); the GitHub runner has 16 GB. The 100 s bound is 3.7 times that time; a quadratic path would take about 4 times as long at each doubling and fail it.

Red first (mutants applied in a scratch worktree and reverted; each run is the row at 65,537 tracks):

| Mutant | Path | Result |
| --- | --- | --- |
| track source index narrowed to `u16` (`prepare.rs`, the track-to-source mapping) | sources | red: render differs |
| `.take(65_536)` on the automation table walk (`controls::AutomationIndex::of`) | automation | red: 65,536 automated faders |
| `rows.truncate(65_536)` on the live send rows (`graph` `live_route_rows`) | sends | red: 65,536 live sends |
| strip index narrowed to `u16` in the sidechain edge loop (`graph-compiler` `compile.rs`) | sidechains | red: prepare panics (`validated prepared effect`) |
| `.take(65_536)` on each VCA's members (`SessionModel::vca_reach`) | VCA | red: render differs |
| `.take(65_536)` on `meter_selection` | meters | red: 65,536 meters |
| live send's route index narrowed to `u16` (`graph` `bind_live_routes`) | sends | green: the index only selects the send's bit in the route-activity table, and no count or rendered bit of this session reads that bit for the aliased send |

Test value: a per-track path that narrows an index or count to 16 bits, or goes quadratic in track count, in source binding, stored-automation attachment, live sends, sidechain edges, VCA reach or meter selection turns this row red, which no committed test catches (the plain #1079 row shared one source and had no sends, sidechains, VCA or automation).
