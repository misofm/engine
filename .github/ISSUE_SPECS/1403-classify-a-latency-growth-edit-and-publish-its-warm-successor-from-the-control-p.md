# Classify a latency-growth edit and publish its warm successor from the control plane

Stream C of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-8 (round-5 amendment), D15-17).
Slice of *Grow latency during playback by adopting a primed warm successor* (#1287): the control
plane's half of the warm path. Split from *Adopt a warm successor with a raw-frame prime at the
first ready block* (#1355) by the round-5 review (M4, M6). Code anchors verified on `main` at
`6fb211594`.

## Product outcome

The control plane turns a latency-growing structural edit into a published warm successor, and
keeps exactly one record of it until it is adopted, superseded or withdrawn.
- A session that holds a `WarmConfig` classifies every rebuild with `warm_lead` (#1354).
- A growth that ducks no strip (it removes none and its successor restarts none) is prepared warm
  and published `Primed { not_before, lead_blocks }` (#1311), with each ring's `prime_required`
  flag (#1320 D4) set first.
- The control plane keeps one `PrimedCandidate` record for that candidate, keyed by its epoch. The
  record never outlives its candidate, so a later step never acts on a candidate it does not name.
- Live edits, a superseding structural edit and a declared stop act on the pending candidate in
  the ordinary way, and its revision completes as each of those paths says.

## Context

- **The rebuild path today.** The control plane prepares a successor against the newest epoch
  (`prepare_runtime` with a `SuccessorBase`, `crates/capi/src/runtime/control.rs:907-914`), runs the
  resource admission (`validate_replacement_peak`, `:939`), reserves the publication
  (`reserve_replacement`, `:962`), commits (`:1006-1008`), moves the persisting producers
  (`adopt_persisting`, `:1018-1019`) and publishes (`reservation.commit()`, `:1025`). *Extract the C
  ABI control plane into a portable crate both hosts call* (#1309) moves this path into
  `crates/control-plane`.
- **Epoch synchronization.** `synchronize_plan_epochs` (`control.rs:783-824`) loads the active
  epoch (`:784`) and promotes the pending provider render adopted (`:785-787`).
- **Producers.** Each source's `HostChunkProvider` lives in its epoch's `SourceControlSet`
  (`crates/host-core/src/source.rs:25`); `adopt_persisting` (`:286`) moves them between sets.
- Inputs from earlier slices:
  - *Prepare a warm successor whose carried nodes lead the predecessor by P* (#1354): `WarmConfig`,
    `warm_lead`, `WarmDecision`, `WarmLead`, warm preparation through `SuccessorBase::warm` (D4),
    `restarted_strips()`;
  - *Adopt a successor plan no earlier than a scheduled sample* (#1311): `PlanAdoption::Primed`
    and `reserve_replacement(plan, adoption)` (D1, D6);
  - *Let a source consumer check and replay its next blocks for a prime* (#1320): the per-ring
    `prime_required` flag (D4), which render reads only for a `Full` `Primed` cell;
  - #1355: render's readiness check, adoption and prime, and the warm successor's source-read
    offset (D1);
  - *Give a plan a source-read clock that leads its render clock* (#1396) D4:
    `PlanPublisher::render_clock()`;
  - *Let the control thread withdraw an unadopted candidate plan* (#1343) D3 and D5: publication
    into the `Empty` cell, and `withdraw() -> Withdrawal::{Withdrawn, Taken, Nothing}`;
  - *Supersede an unadopted candidate plan by compare-and-swap* (#1310) D1-D6;
  - *Reset latency floors at a host-declared discontinuity* (#1323) D2 and D4;
  - *Size the C ABI's plan capacities and resource admission for a superseding candidate* (#1398)
    D4: `AdmissionPeak::WithReprepare`.

## Decisions frozen for this slice

- **D1. The configuration is injected.** `SessionState` holds `warm: Option<WarmConfig>`, set when
  the session is created.
  - With `None`, no rebuild is classified, and every rebuild takes today's path. That is every
    product session until *Check the warm-successor deadline in miso_engine_v1_service and report
    its outcome* (#1360) builds the C ABI's config and *Check the warm-successor deadline in the
    browser Worker's service loop and report its outcome* (#1361) builds the browser's.
  - This slice adds a `test-support` constructor argument that sets it. Gates set both bounds
    directly, as #1354 D1 says.
- **D2. Classification.** With `Some(config)`, the rebuild path calls `host_core::warm_lead`
  (#1354 D2) before it prepares the successor (`control.rs:907`). The result routes the edit:
  - `Ordinary`: today's path, published `Next`.
  - `Warm(lead)` with `lead_samples > 0` and an empty duck set: *Duck-swap a strip whose state
    cannot continue across a plan swap* (#1324) D4's set, the strips the edit removes plus the
    warm successor's `restarted_strips()`. It is prepared warm (`SuccessorBase::warm =
    Some(&lead)`, #1354 D4), admitted with `AdmissionPeak::WithReprepare` (#1398 D4) so that a
    later transition re-preparation fits the caps, then published by D3.
  - `Warm` with a non-empty duck set, `Warm` with `lead_samples == 0`, and `Unavailable`: these are
    *Duck-swap the strips a latency growth restarts, and fall back to the transition when a warm
    successor cannot adopt* (#1397) D1 and D2, which extend this match. This slice routes them to
    today's path. No product session reaches them before #1397, because no product session holds
    a `WarmConfig` before #1360 and #1361, which depend on #1397.

  The transaction response is `rebuild` in every case (*Report each transaction's edit path in its
  response*, #1313).
- **D3. Publication (moved from #1355 D3).** New `crates/host-core/src/warm.rs`:
  `publish_primed(publisher, reservation_plan, running_sources, candidate_sources,
  withdrawn_sources: Option<&SourceControlSet>, not_before) -> PrimedCandidate`.
  1. It stores `prime_required` on every ring the running plan renders: `true` for each ring W
     carries, `false` for every other. The rings are reached through the providers in W's source
     control set and in the running epoch's set.
  2. It stores `false` on every ring in `withdrawn_sources` that W does not carry. Those are the
     rings only a withdrawn candidate held: between #1310 D5's publication (step 4) and its step 5,
     the producer of a source the running plan renders and W removed is still in the withdrawn
     candidate's set. Without this step, a `true` written for that candidate stays on a ring W never
     primes, and render's check waits on a source the host may have stopped feeding.
  3. It then reserves and publishes W with `Primed { not_before, lead_blocks: P / q }` (#1311 D6).

  `not_before` is the `render_clock()` read at publication in D2's arm, whose duck set is empty; in
  #1397 D1's arm it is #1325 D3's `S`. Control publishes only into an `Empty` cell (#1343 D3), so no
  `Primed` candidate is `Full` while the flags are written. Render reads them only for a `Full`
  `Primed` cell, so a stale
  value outside that window is never read. Every `Primed` publication rewrites every flag it can
  reach.
- **D4. The record and its lifecycle.** `publish_primed` returns
  `PrimedCandidate { epoch, not_before, lead_blocks }`. `SessionState` keeps at most one, in
  `primed: Option<PrimedCandidate>`, because at most one candidate is pending. Every step that
  reads it compares its `epoch` with the candidate it acts on.
  - **Set** when `publish_primed` returns, replacing any earlier record.
  - **Dropped when render adopts it:** `synchronize_plan_epochs` drops the record once the active
    epoch is at or past the record's epoch. Every control call services first (*Add
    miso_engine_v1_service for bounded control work between edits*, #1348 D2), so no later step
    sees a record whose candidate render already runs.
  - **Dropped on a withdrawal that does not return its candidate:** a `withdraw()` that returns
    `Taken` or `Nothing` drops the record. One that returns `Withdrawn(c)` with `c`'s epoch equal to
    the record's hands both to the path that withdrew it; with another epoch, the record is stale
    and is dropped.
  - **#1310 supersession (D5):** the record of the withdrawn candidate is dropped at step 6, when
    the candidate is dropped. If B is itself published `Primed`, B's `publish_primed` sets the new
    record, passing the withdrawn candidate's set as `withdrawn_sources`.
  - **#1323's declared stop (D2):** the withdrawn candidate's record is dropped with it. The
    discontinuity successor is published `Next`, so it leaves no record.
  - **A refused supersession or stop keeps the record.** When #1310 D4 refuses B, it republishes
    W unchanged; when #1323 D4 refuses the discontinuity successor, nothing changes. Either way the
    record stays as it was, naming W's epoch, so W stays a pending candidate with its record and
    #1358's deadline step still acts on it. Only the drops above remove a record.
  - **#1358's deadline step** (*Fall back to the transition when a warm successor is not ready by
    its deadline*, #1358 D3): it drops the record on `Taken`, `Nothing` or a successful
    transition. When the transition's re-preparation is refused, it republishes the withdrawn
    candidate as #1310 D4 republishes and sets the record again, unchanged, so the record and its
    candidate stay together and every path above handles it as a pending candidate.
- **D5. Edits while W is pending** (#1355 D6). A live edit goes to the newest candidate's cells
  and applies at S (#1053 D7). A structural edit supersedes W by #1310, including D5 step 5. A
  declared stop supersedes W by #1323 D2. This slice gates all three through the control plane.
- **D6. Acked-batch question.** Classification runs before the commit and changes no queue. A
  `Primed` publication drops nothing: render claims W only when ready (#1311 D2), and W stays
  withdrawable until then. Clearing a flag or a record discards no PCM and no command. Every
  withdrawal path keeps the withdrawn candidate until its replacement is prepared from the
  committed model (#1310 D4, #1323 D4). An ack can never precede a drop.

## Deliverables

1. D1, D2, D4 and D5's wiring in `crates/control-plane/src/`, with the `test-support` constructor
   argument and a `test-support` accessor that returns the record.
2. D3 in `crates/host-core/src/warm.rs` (new), the flag handles of the source control set in
   `crates/host-core/src/source.rs`, and the re-exports in `crates/host-core/src/lib.rs`.
3. Gates 1-4 in the control-plane crate's unit tests.

## Authorized paths

- `crates/control-plane/src/`
- `crates/host-core/src/warm.rs` (new), `crates/host-core/src/source.rs` (the flag handles only),
  `crates/host-core/src/lib.rs` (re-exports only)

## Non-goals

- Warm preparation and its refusals (#1354). Render's readiness check, adoption and prime
  (#1355).
- The duck of restarted strips, the transition and the outcome word (#1397). The deadline step (#1358)
  and the ring headroom (#1406).
- The production `WarmConfig`, the C ABI and the browser (#1360, #1361).

## Objective gates

All gates are control-plane unit tests with `test-support`, at 48 kHz and quantum 128. A is the
two-track fixture of `crates/host-core/tests/successor_swap.rs`. Every source ring is set
explicitly to 7,296 frames: `stall_ring_frames(48 kHz, 128)` (#1354 D1, 5,120), plus
*Record the swap block's cost on the 64-track console* (#1286) D3's `p_max_samples(48 kHz, 128)`
(2,048), plus one quantum. So #1354 D2's headroom check passes whether or not *Grow the default
source ring by the warm-prime headroom* (#1406) has changed the default ring. Every carried source
is kept queued at least `P + q` frames ahead unless a gate withholds frames. "Equals A continued"
means bit-identical, every block, to A rendered on with
no swap and the same host commands, timed on the source-read clock.

1. **Classification and publication.** The edit adds a muted track with a true-peak limiter insert
   and silent input (#1355 gate 1's shape).
   - With a `WarmConfig` of `p_max = 2,048` and an unbounded `prime_bytes_max`, the candidate is
     published `Primed { not_before, lead_blocks: 4 }`, with `not_before` equal to `render_clock()`
     at publication. The record equals `{ epoch, not_before, 4 }`. Every ring A renders reads
     `prime_required() == true`. The response path is `rebuild`.
   - The admission is `WithReprepare`: with one D4 cap of #1398 set to `R + 2C - 1` (#1398
     gate 3's rows), the same submit is refused before commit with that cap's diagnostic; at
     `R + 2C` it passes.
   - With no `WarmConfig`, the same edit is published `Next`, no record exists, and every flag is
     `false`.
   - With the config, an edit that grows nothing is published `Next`, and no record exists.
2. **Routing on the duck set.** The flags of a withdrawn candidate's rings (D3 step 2) are gated
   by *Duck-swap the strips a latency growth restarts, and fall back to the transition when a warm
   successor cannot adopt* (#1397) gate 5, since an edit that removes a source's track has a
   non-empty duck set. Here: gate 1's growth plus the removal of track 2 has a non-empty duck set,
   so D3's arm does not take it. Until #1397 it takes today's path, a two-phase removal
   (*Remove a strip in two phases: ramp out, then a scheduled swap*, #1325) published
   `NoEarlierThan(S)`; no record exists, and no block before `S` differs from A continued with
   track 2 live-muted by the same ramped mute at the same block. #1397 rewrites this test when
   its D1 lands (its deliverable 5): the edit is then published `Primed` with a record.
3. **Edits while pending (moved from #1355 gate 5).** W is gate 1's growth, published with frames
   withheld.
   - A live fader edit on track 2 applies at S. The output equals A continued with the same value
     written to A's cell just before block S.
   - A structural edit that removes carried source `c` supersedes W. Its adoption reports
     `EXACT | SUPERSEDED`. Until then, `c`'s producer is back in the running epoch, its submits
     are accepted, and A plays them without underrun.
   - A declared stop supersedes W, which folded in no revision. The revision completes `EXACT`,
     without `SUPERSEDED` or `TRANSITION_FALLBACK` (#1323 D4).
4. **Record lifecycle.**
   - W is adopted by render. The next control call leaves no record. A structural edit B then
     publishes `Next`: still no record. With B a warm growth instead, the record names B's epoch.
   - W is superseded by a `Next` candidate B: no record. W is superseded by a warm B: the record
     names B's epoch, never W's.
   - A declared stop over W: no record.
   - A structural edit B whose supersession is refused (one D4 cap of #1398 set one below the
     value B's admission needs, as in gate 1's cap case): W is back in the mailbox `Full`, and the
     record still equals `{ W's epoch, not_before, lead_blocks }`. #1358 gate 4 shows that W's
     deadline then still fires.
5. Commands:
   - `cargo test --locked -p control-plane --features control-plane/test-support`
   - `cargo test --locked -p host-core --features host-core/test-support`
   - `bash scripts/check-realtime-policy.sh`, `bash scripts/check-workspace-policy.sh`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`

## Test value

- Gate 1: a classifier that publishes the warm candidate as `Next`, or a `not_before` read from
  the source-read clock, adopts it before render is ready or at the wrong block; a session without
  a config that classifies changes today's path; an admission at `Single` would let a later
  transition exceed the caps. Red.
- Gate 2: a classifier that routes on `restarted_strips()` alone publishes the edit `Primed` at the
  render clock, and track 2 stops dead at the adoption block (#1269 P4). Red.
- Gate 3: a supersession that drops W's producers with W leaves `c` with no producer, so its
  submits are refused and A underruns; a stop reported `SUPERSEDED` for a candidate that folded in
  nothing miscounts it. Red.
- Gate 4: a record kept after W's adoption or supersession names W's epoch; #1358's deadline step
  would then withdraw B, report it `TRANSITION_FALLBACK` and lose its `S`. A record dropped on a
  refused supersession leaves W pending with no deadline step, so the edit never completes. Red.

## Dependencies

- *Prepare a warm successor whose carried nodes lead the predecessor by P* (#1354).
- *Adopt a warm successor with a raw-frame prime at the first ready block* (#1355).
- *Supersede an unadopted candidate plan by compare-and-swap* (#1310).
- *Adopt a successor plan no earlier than a scheduled sample* (#1311).
- *Reset latency floors at a host-declared discontinuity* (#1323).
- *Let a source consumer check and replay its next blocks for a prime* (#1320): `prime_required`.
- *Give a plan a source-read clock that leads its render clock* (#1396): `render_clock()`.
- *Let the control thread withdraw an unadopted candidate plan* (#1343).
- *Size the C ABI's plan capacities and resource admission for a superseding candidate* (#1398):
  `AdmissionPeak::WithReprepare`.
- *Add miso_engine_v1_service for bounded control work between edits* (#1348): every call services
  first.
- *Report each transaction's edit path in its response* (#1313).
- *Duck-swap a strip whose state cannot continue across a plan swap* (#1324): D4's duck set.
- *Remove a strip in two phases: ramp out, then a scheduled swap* (#1325): gate 2's path.
- *Publish an applied-revision watermark and complete edits asynchronously* (#1314).
