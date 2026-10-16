# Reset latency floors at a host-declared discontinuity

Stream A of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-8, D15-17; D15-8 (round-5 amendment)).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

Latency does not ratchet up for the life of a session. *Keep every node's latency from dropping
during playback* (#1285) keeps a removed limiter's latency while audio plays. When the host
declares a discontinuity (it has stopped, or it is about to seek every source), the engine drops
every latency floor. The next render runs the session at its natural latency, and the plan
resource report says so. A pending warm successor is replaced by that plain rebuild. A declaration
with nothing to reset changes nothing.

## Context

- Floors are recorded in the predecessor's inventory and passed on to every successor (#1285 D2).
  Without a reset they never come down.
- No C ABI call lets a host say "continuity is not needed". The render clock must stay contiguous
  ("render.time.discontinuity", `crates/capi/include/miso_engine_v1.h:72-75`). Seeks are per
  source (`miso_engine_v1_source_seek`, `miso_engine_v1_source_seek_at`, `:293-303`). On
  `6fb211594` the last feature bit is `MISO_ENGINE_V1_FEATURE_SOURCE_SEEK_AT` (`:136-142`;
  `crates/capi/src/abi.rs:64-71`). Other decision-15 entry points may take bits before this slice
  merges.
- The successor entry points take a `SuccessorBase` (`crates/host-core/src/prepare.rs:641-647`).
- D15-8 (round-5 amendment): a pending warm successor is a candidate published `Primed {
  not_before, lead_blocks }` (*Adopt a successor plan no earlier than a scheduled sample*, #1311)
  whose carried nodes are floored at `a(n) + P`. At a host-declared stop it is superseded by a
  plain rebuild through #1310, with no continuity constraint, and its revision completes `exact`
  or `superseded`. This slice owns that rule; it absorbs the retired #1359.
- The accumulated read-ahead `ΣP` of the warm successor resets at the same declaration (D15-8).
  *Give a plan a source-read clock that leads its render clock* (#1396), a slice of *Grow latency
  during playback by adopting a primed warm successor* (#1287), introduces that read-ahead and
  resets it at the declaration defined here.

## Decisions frozen for this slice

- **D1. The declaration.**
  - A new C ABI entry point, `uint32_t miso_engine_v1_declare_discontinuity(miso_engine_v1_session
    *session)`, guarded by `MISO_ENGINE_V1_FEATURE_DECLARE_DISCONTINUITY`. Its value is the next
    free bit when this slice merges, and the mask grows to match. This spec fixes no bit number;
    code, tests and docs name the bit by its symbol (decision 15, D15-12: a feature bit per
    addition).
  - It is a control-thread call. Its meaning: the next block the host renders need not continue the
    previous output, because the host has stopped or is about to seek every source.
  - The header documents the duty: a host calls it at a stop, and before a seek of every source.
  - The ABI growth rule of *Document the seek contract and the C ABI growth rule in the header*
    (#1317) applies: one feature bit, the mask never compared with `==`.
- **D2. What it does.**
  - The declaration rebuilds if the newest plan (the pending candidate, or else the active plan)
    was compiled with any floor above its node's natural arrival, or reads its sources ahead of
    render (a source-read offset above 0). The control plane then prepares the committed model as a
    successor at a discontinuity. That successor has no floors and carries only the source
    consumers (ring, generation and read position). Every other owner starts at rest: there is no
    continuity constraint. It is published for adoption at the next block.
  - The offset term matters because a plan can lead with no raised floor: after a warm growth, a
    later edit can make a node's natural arrival equal its floor. The offset is recorded in the
    inventory by *Give a plan a source-read clock that leads its render clock* (#1396) D2, which
    lands after this slice and adds the term to this test. Before #1396 no plan has an offset, so
    the floor term alone is the whole test.
  - The call lives in `crates/control-plane/src/`; the capi entry point only forwards to it.
  - A pending candidate is superseded by *Supersede an unadopted candidate plan by
    compare-and-swap* (#1310): withdrawn, used as the donor of the rings it created (#1310 D2), its
    persisting producers returned to the running epoch (#1310 D5 step 5), then dropped.
  - **A pending warm successor.** A `Primed` candidate always has a raised floor (its carried nodes
    are floored at `a(n) + P`, and `P > 0`), so the declaration always rebuilds while one is
    pending, even when the active plan has no raised floor. It is superseded as above. The
    discontinuity successor is published `Next`, never with the withdrawn candidate's `Primed`
    kind: it has no lead, no readiness check, no deadline and no transition. The same holds before
    a seek of every source. The control plane's `PrimedCandidate` record of the withdrawn candidate
    is dropped with it, and the discontinuity successor leaves none (*Classify a latency-growth
    edit and publish its warm successor from the control plane*, #1403 D4, which adds the record
    and this drop).
  - If neither term holds, the call returns OK and publishes nothing.
- **D3. Host-core.** `SuccessorBase` gains `discontinuity: bool`. When it is true, the successor
  is compiled with an empty floor map, and the join carries sources only. Its source-read offset
  is 0 (#1396 D2, which lands after this slice and reads this flag). The declaration may or may not
  be followed by a seek: a stop without a seek follows #1396 D3. Each carried consumer keeps its
  read position, so no frame is repeated or skipped, and the source-read clock steps back to the
  render clock at the adoption block. The inventory records
  natural arrivals beside floored ones, so D2's test is a comparison of two numbers.
  - **The constructor.** `SuccessorBase::new(inventory, committed, revision)` exists from
    *Carry fader, mute and pan ramps across a plan swap* (#1277 D10), which added `revision`
    first; `revision` has no neutral value, so `new` takes it as an argument. This slice adds
    `discontinuity` to `new`'s output: `new` sets every other field to its neutral value,
    `discontinuity: false` here. Each later slice that adds a field extends `new` with that
    field's neutral value (*Prepare a warm successor whose carried nodes lead the predecessor by
    P*, #1354 D4: `warm: None`; *Duck-swap the strips a latency growth restarts, and fall back to
    the transition when a warm successor cannot adopt*, #1397 D2: `forced_restart` empty).
  - Every literal construction site that exists when this slice lands moves to `new` in this
    change:
    - `crates/host-core/tests/successor_swap.rs:988` and
      `crates/host-core/tests/support/successor.rs:101`;
    - `crates/capi/tests/resource_lifecycle.rs:1591`;
    - the control plane's sites in `crates/control-plane/src/`: the `prepare_runtime` call
      (`crates/control-plane/src/control.rs:1046`, moved there by #1309 D2) and #1310 D2's
      base across a withdrawn candidate;
    - `crates/host-core/tests/withdrawn_successor.rs`, the gates of *Prepare a successor across a
      withdrawn candidate plan* (#1344), which lands before this slice through #1310.

    A site added by a slice that is not ordered against this one is in the list if that slice
    lands first. The discontinuity successor is
    `SuccessorBase { discontinuity: true, ..SuccessorBase::new(inventory, committed, revision) }`. Every
    later site is built through `new`, and a later field changes `new` and no construction site.
- **D4. Acked-batch rule.** Preparation can fail (budgets, ceilings). On failure the call returns
  the preparation error and nothing changes. On success the plan is published with its retirement
  credit reserved, as a structural transaction is. The model and the revision do not change. No
  acknowledged edit is lost: the committed model is the source of the successor.
  - **Completion.** The discontinuity successor carries the committed revision, so a withdrawn
    candidate's revision completes when that successor is adopted. #1310 D6 stores its
    `superseded` word with that same revision as B's: the advance is `EXACT`, with `SUPERSEDED`
    when the withdrawn candidate had itself folded in earlier revisions. It is never
    `TRANSITION_FALLBACK`: no continuity is owed after a declared stop, so nothing fell back.
- **D5. Both hosts.** The call is implemented in the control plane, in the crate of *Extract the C
  ABI control plane into a portable crate both hosts call* (#1309). The C ABI exposes it here. The
  browser reaches the same function when *Run the browser control plane in a Worker and keep the
  AudioWorklet render-only* (#1332) lands, and its SDK surface belongs to *Apply session
  transactions from the browser SDK* (#1296).

## Deliverables

1. D1: the entry point in `crates/capi/src/ffi.rs`, the feature bit and mask in
   `crates/capi/src/abi.rs`, the header prototype and text, and `docs/C_ABI_V1_QUALIFICATION.md`.
2. D2 and D4 in `crates/control-plane/src/` (package `control-plane`, lib `control_plane`, created
   by #1309).
3. D3 in `crates/host-core/src/prepare.rs`, with the move of every literal `SuccessorBase` site
   to `SuccessorBase::new`.
4. Tests in `crates/host-core/tests/successor_swap.rs`, `crates/capi/src/runtime/tests.rs` and the
   control-plane crate's unit tests (gate 5).

## Authorized paths

- `crates/host-core/src/prepare.rs`, `crates/host-core/tests/successor_swap.rs`
- The `SuccessorBase` construction sites only (D3): `crates/host-core/tests/support/successor.rs`,
  `crates/host-core/tests/withdrawn_successor.rs` (#1344's) and
  `crates/capi/tests/resource_lifecycle.rs`
- `crates/capi/src/ffi.rs`, `crates/capi/src/abi.rs`, `crates/capi/src/runtime/tests.rs`,
  `crates/capi/include/miso_engine_v1.h`, `docs/C_ABI_V1_QUALIFICATION.md`, and
  `crates/control-plane/src/` (#1309). These are stream B's files; root orders the merge after
  #1309 and #1310.
- `scripts/check-capi-abi.sh` (only if its symbol list must learn the new entry point).

## Non-goals

- No read-ahead reset: #1396 sets the discontinuity successor's source-read offset to 0 (its D2)
  and defines the stop without a seek (its D3).
- No browser export (#1332, #1296).
- No change to the render clock's contiguity rule.

## Objective gates

1. **Floors reset, host-core.** A has three tracks, the first with a true-peak limiter insert. B
   removes that track and is swapped in after block 6 with A's floors (#1285). Then prepare B again
   with `discontinuity: true` and swap it in after block 10. From block 11 on, its output equals a
   fresh B, compiled without floors, whose sources were fed the same PCM from the frames the carried
   consumers stood at. `latency_samples` is B's natural latency.
2. **Floors reset, C ABI.** The same edit as a `RemoveTrack` transaction, then
   `miso_engine_v1_declare_discontinuity`. After one render, `miso_engine_v1_plan_resources`
   reports the natural `latency_samples`. Before the declaration it reported A's.
3. **Nothing to reset.** On a session with no raised floor, the declaration returns OK, publishes
   no plan (the swap counter is unchanged), and leaves the output bit-identical to a run without
   the call.
4. **Failure changes nothing.** With a resource cap set so that the floorless preparation fails,
   the call returns the preparation error, and the active plan, its floors and the committed model
   are unchanged.
5. **A declared stop supersedes a pending warm successor (new control-plane test,
   `test-support`).** Gate 1's A runs with no raised floor. Commit its `RemoveTrack` edit, and
   publish the resulting B (floored at A's arrivals by #1285, so above its natural ones) through a
   `test-support` publication hook as `Primed { not_before, lead_blocks: 1 }`, with `not_before`
   64 blocks ahead. Render 4 blocks (B stays pending), then declare a discontinuity and render 8
   blocks.
   - The block after the declaration is the discontinuity successor's, and from it the output
     equals a fresh B compiled without floors, fed the same PCM from the frames the carried
     consumers stood at; `latency_samples` is B's natural latency.
   - The withdrawal reported `Withdrawn`, and B was never adopted.
   - The watermark advances to the committed revision at that block with `EXACT` and without
     `TRANSITION_FALLBACK`. Repeated with a structural edit committed and superseded before B (so
     B folds in one revision): `EXACT | SUPERSEDED`, `superseded_count` grows by 1.
   - Owners and retirement credits balance, and no producer was dropped on the control thread.
6. **ABI surface.** `query_capabilities` reports `MISO_ENGINE_V1_FEATURE_DECLARE_DISCONTINUITY`,
   and the mask test checks that the mask is the OR of every feature symbol, the new one
   included (by symbol, never by a literal number).
7. Commands:
   - `cargo test --locked -p host-core -p control-plane -p capi --features
     host-core/test-support,control-plane/test-support`
   - `bash scripts/check-capi-abi.sh` and `bash scripts/check-capi-abi.sh --self-test`
   - `cargo build --locked --release -p audit && ./target/release/audit capi`
   - `bash scripts/check-workspace-policy.sh`, `cargo fmt --all -- --check`,
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`

## Test value

- Gate 1: a reset that keeps the floors, or one that carries DSP state, diverges from the fresh
  floorless reference. It turns red.
- Gate 2: a declaration that clears floors in the inventory but never publishes a plan keeps
  reporting A's latency. It turns red.
- Gate 3: a declaration that always rebuilds resets every effect tail on every stop. It turns red.
- Gate 4: a declaration that drops floors before its preparation succeeds leaves a plan whose next
  successor loses latency mid-playback. It turns red.
- Gate 5: a declaration that publishes its successor with the pending candidate's `Primed` kind,
  or leaves a warm candidate to its own adoption, plays the floored plan past the stop (the
  successor waits for `not_before` and readiness), so the floors survive it; one that reports the
  revision as `TRANSITION_FALLBACK` miscounts a stop as a fallback. Gates 1-4 have no `Primed`
  candidate. It turns red.

## Dependencies

- *Keep every node's latency from dropping during playback* (#1285).
- *Extract the C ABI control plane into a portable crate both hosts call* (#1309).
- *Supersede an unadopted candidate plan by compare-and-swap* (#1310).
- *Adopt a successor plan no earlier than a scheduled sample* (#1311): the `Primed` kind gate 5
  publishes.
- *Publish an applied-revision watermark and complete edits asynchronously* (#1314): gate 5's
  outcome flags.

## Amendment (root, 2026-10-11): `SuccessorBase::new` takes `revision`

Root amended this spec per the *Carry fader, mute and pan ramps across a plan swap* (#1277) ruling.
#1277 D10 adds `SuccessorBase::revision`, which has no neutral value. So D3's constructor is
`SuccessorBase::new(inventory, committed, revision)`, and the sentence "the first slice that adds a
field" is removed. Nothing else changes.

### Open question for root (from the #1277 ruling review, 2026-10-11)

The discontinuity successor publishes the *committed* revision on a fresh gate, and a session at
revision 0 would be refused by `stamp(0)` (and by host-core's entry check, `host.successor.revision`,
which requires `1 <= revision`). Root decides how this slice handles a session at revision 0 when
#1323 is next.

## Amendment (controls coordinator, batch 15 check, 2026-10-15)

Checked against `228fca26c`.

**Moved anchors.**
- "render.time.discontinuity" is a render diagnostic, now at
  `crates/capi/include/miso_engine_v1.h:182`. The seek prototypes are `:420-430`.
- The last feature bit is now `MISO_ENGINE_V1_FEATURE_SERVICE` (`1 << 7`,
  `crates/capi/src/abi.rs:70`, header `:251`). The mask is `abi.rs:72-79` and header `:252`. #1316
  and #1642 also add bits. D1's "next free bit when this slice merges" still governs.
- The mask test at `crates/capi/src/abi.rs:554` asserts `FEATURE_MASK == 0xff`, a literal. Gate 6
  replaces that assertion in this slice.
- `SuccessorBase` is `crates/host-core/src/prepare.rs:1070-1097`.
- The control plane's `prepare_runtime` call is `crates/control-plane/src/control.rs:1662`, with
  its `SuccessorBase` literal at `:1670`.

**`SuccessorBase::new` does not exist.** #1277 closed without it, so this slice adds it.
- `SuccessorBase` now has six fields: `inventory`, `committed`, `revision`, `edit_ramps` (#1394),
  `monitor` (#1559) and `observation` (#1584).
- Each new field has a neutral value: `EditRamps::new()` is `const`
  (`crates/session/src/edit_ramp.rs:102`), `MonitorBase::NONE` (`prepare.rs:1125`) and
  `NO_OBSERVATION` (`prepare.rs:1100`). `new` sets those and `discontinuity: false`.
  `edit_ramps` is a reference, so `new` needs a static neutral `EditRamps` beside
  `NO_OBSERVATION`.

**Construction sites at `228fca26c`.** These replace D3's list:
- `crates/control-plane/src/control.rs:1670`;
- `crates/host-core/tests/support/successor.rs:98`;
- `crates/host-core/tests/successor_swap.rs:1043`, `:2663`, `:3926`, `:3950`, `:3980`, `:7159`,
  `:9331`;
- `crates/host-core/tests/browser_successor.rs:284`;
- `crates/host-core/tests/effect_observation.rs:544`;
- `crates/capi/tests/resource_lifecycle.rs:1805`;
- `hosts/host-web/src/tests.rs:5938`.

`crates/host-core/tests/withdrawn_successor.rs` does not exist (#1344 is open). Add to the
authorized paths, for the construction sites only: `crates/host-core/tests/browser_successor.rs`,
`crates/host-core/tests/effect_observation.rs` and `hosts/host-web/src/tests.rs`.

**Dependencies.**
- #1285, #1317 and #1396 are in the batch.
- #1309 is closed: the control plane is `crates/control-plane/src/`.
- #1311 is closed: `PlanAdoption::Primed` is `crates/engine/src/realtime/spsc.rs:532`.
- #1314 is closed: the outcome flags are header `:255-256`.
- #1310 is open and not in batch 15/16. D2's supersede, D4's completion and gate 5 need it. The
  engine's `withdraw` exists (`crates/engine/src/realtime/plan_exchange.rs:552`, #1343), but #1310's
  compare-and-swap and its D2, D5 and D6 do not. #1310 also waits on #1344 and #1398, both open.
- #1403 (open) adds `PrimedCandidate` later. It is not a dependency.
- Batch order (root, 2026-10-15): batch 15a is the controls lane, #1344, #1398 and #1310.
  #1324 left batch 15a (root ruling, 2026-10-15); it lands in a later batch after #1288, #1391,
  #1363 and #1325.
  The rest of batch 15 follows. This slice is in the rest (B15-R5).

**B15-R5, root decision under owner delegation (2026-10-15): order #1310, #1344 and #1398 before #1323. No placeholder
refusal.** #1344, #1398 and #1310 are in batch 15a; #1323 follows in the rest of batch 15.
The finding follows. D2 supersedes a pending candidate
through #1310, which is not planned for this batch. A declaration while a candidate is pending has
no rule without it. Proposed: order #1310, #1344 and #1398 before #1323. Do not ship a placeholder,
such as a typed backpressure refusal while a candidate is pending.

**B15-R6, root decision under owner delegation (2026-10-15): no revision-0 special case. At revision 0 the call returns OK and
publishes nothing.** D2 adds that sentence. Gate 3 adds a revision-0 case: a declaration
on a host at revision 0 returns OK, publishes no candidate, and leaves the next render and the
reported latency unchanged. The finding follows. Proposed answer: no special
case. Floors and source-read offsets come only from a successor. A successor needs
`1 <= revision` (`host.successor.revision`, `prepare.rs:1915`, `:3200`), and the committed revision
only grows. So when D2's test holds, the committed revision is at least 1. At revision 0 the call
returns OK and publishes nothing. Add that sentence to D2 and a revision-0 case to gate 3.

**B15-R7, root decision under owner delegation (2026-10-15): this slice also adds `miso_engine_web_v1_declare_discontinuity(handle)
-> u32`.** It forwards to the same control-plane function. It adds the export-list mirrors
of #1293 D7 and a native host-web gate equal to gate 2 (in `hosts/host-web/src/tests.rs`).
#1643 calls it before its seek. Add `hosts/host-web/src/ffi.rs`, `hosts/host-web/src/lib.rs` and
the mirror files of #1293 D7 to Authorized paths. The finding follows. D5 leaves the browser to #1332 and
#1296. Since #1381a the browser runs `control_plane::SessionState` in its one instance
(`hosts/host-web/src/lib.rs:6-16`), so the function is reachable there now. Only a browser entry
is missing. Decision 17 C1 says a behaviour on one host only is a defect. #1642 makes a C ABI host
declare before a session seek, but #1643 (the browser session seek) does not, so browser floors
would never reset. Proposed: this slice also adds a control-half export
`miso_engine_web_v1_declare_discontinuity(handle) -> u32` that forwards to the same control-plane
function. It adds the export-list mirrors of #1293 D7 and a host-web native gate equal to gate 2.
#1643 then calls it before its seek. Alternative: carry the declaration as a protocol frame
through `submit_command` and `control_command`, C13's form with no new export. But D1, #1317 and
#1642 already name the C export.

**Other notes.**
- `TRANSPORT_SET` now carries the state only (#1635, code landed; issue still open). A transport
  stop is not a declaration. The host calls D1 itself.
- #1641 (open) gives every plan a timeline that "carries like a source". D2's successor "carries
  only the source consumers". If #1641 lands first, the timeline consumer carries too, or the
  playhead and stored automation (#1647) restart at the declaration. Whichever lands second adds
  it to the discontinuity join.
- Decisions D1-D4 otherwise hold against the code and decision 17. The declaration is not a
  control.

**Gates and commands.**
- The named files exist, except `withdrawn_successor.rs` (#1344, new there).
- `tools/audit/src/capi.rs` exists, and `scripts/check-capi-abi.sh` has `--self-test`.
- Gate 7's commands are valid. Its features exist.

## Amendment (controls coordinator, batch 15a fold, 2026-10-16)

- **Batch placement.** "#1310 is open and not in batch 15/16" (Dependencies) is void. #1310, #1344
  and #1398 are batch 15a, which lands before this slice (B15-R5).

## Decision record: #1323 (batch 15b)

**What shipped.**
- D1: `miso_engine_v1_declare_discontinuity(session)` in `crates/capi/src/ffi.rs`, a forwarder to
  `control_plane::SessionState::declare_discontinuity`. `FEATURE_DECLARE_DISCONTINUITY = 1 << 8`
  (the next free bit), and the mask ORs it. Header: the define, the prototype, the session thread
  list, and a paragraph "Declared discontinuity (issue 1323)" with the host's duty. The #1285
  paragraph now points to it. `scripts/check-capi-abi.sh` lists the symbol (18 frozen).
- B15-R7: `miso_engine_web_v1_declare_discontinuity(handle) -> u32` in `hosts/host-web/src/ffi.rs`,
  through `AudioWorkletEngineHost::declare_discontinuity`, which runs the same control-plane
  function. The host's `run_command` became `run_control`, so the plan a declaration publishes
  keeps its tables as a rebuild's do. The #1293 D7 mirrors are updated and the SDK is regenerated.
- D2/D4 in `crates/control-plane/src/control.rs`. `rebuild` takes `Rebuild::Transaction(token)`
  or `Rebuild::Discontinuity(Box<CompiledSession>)`. The discontinuity arm has no protocol commit
  and returns no response. Its copy of the committed model is the kept base of the epoch it
  displaces (the direct case), or it is dropped when the arm supersedes a candidate, as a commit's
  displaced model is. It is always published `Next`, and the test adoption hook is not taken.
  Supersession, the donation, the acked-PCM guard, observation forwarding, the `superseded` word
  (`r - 1 - last_running`) and republish-on-refusal are the transaction path's own code.
- D3 in `crates/host-core/src/prepare.rs`. `SuccessorBase::discontinuity`, `SuccessorBase::new`
  (with a private static neutral `EditRamps`), and every literal site moved to `new`. At
  preparation, a discontinuity base swaps its inventory for `at_discontinuity()`: identity, rate,
  quantum and source rows only, with no floors. Every join reads the base's inventory, so that one
  substitution is the whole rule. The inventory records `raised_arrivals` (each raised node's
  natural arrival, beside its floored one in `input_floors`), and
  `runs_behind_natural_timing()` is D2's test.

**Rule for the supersession case (the spec is silent).** A transaction E that supersedes a
discontinuity successor D before render adopts it is prepared at the same discontinuity (sticky
`ProviderEpoch::discontinuity`). The combined twin P0 -> E is "declare, then edit" as one
transaction against the running plan, and render never played D. So E has no floors and carries
only sources. A declaration while D waits finds nothing to reset (D has no raised floor) and
publishes nothing. D over a withdrawn A carries A's revision. A's revisions complete at D's
adoption, `EXACT`, with `SUPERSEDED` for those A folded in.

**Departures.**
- `crates/graph-compiler/src/{pdc.rs,compile.rs,lib.rs}` (not authorized): D3 needs natural
  arrivals in the inventory, and only PDC knows them. `GraphCompileReport::raised_arrivals` is a
  report field only. Program, canonical bytes and fixtures are unchanged, and the unit test is
  extended.
- Gate 5 and the supersession gate are in `crates/capi/src/runtime/tests.rs`, not in control-plane
  unit tests. Those tests drive `control_plane::SessionState` directly. The control-plane crate has
  no render or session-fixture harness, and its `tests/` directory is not authorized.
- Gate 4's error is the replacement-peak cap's `CompileRejected` (set with
  `set_replacement_peak_limit`). The C ABI's limits give no per-preparation cap that fails only
  the floorless successor.
- Mechanical edits outside the paths: `crates/capi/tests/c/{abi_smoke.c,header_smoke.cpp}` (the
  bit inside the mask and the signature, replacing `MASK == 255`), `crates/host-core/tests/strip_meters.rs`
  (a `SuccessorBase` site added after the batch check), `ProviderEpoch::test_inventory`, and the capi
  test wrappers `test_declare_discontinuity` and `test_plan_latency`.
- `hosts/host-web/tests/control_parity.rs`: a parity row, at the coordinator's request.
- Header: the #1310 paragraph is re-wrapped to 100 columns (one line was 135).
- No browser latency report exists yet (#1688). So the browser gate reads the plan latency through
  the host's `queries`, and the parity row compares bits and the C ABI latency.

**Gates.**
- Gate 1: `host-core` `successor_swap::a_declared_discontinuity_resets_the_floors_at_every_rate_and_width`.
  Red when the floors are kept (447 against 0 at 44.1 kHz) and when all state is carried (block 0
  differs).
- Gate 2: `capi` `runtime::tests::a_declared_discontinuity_reports_the_natural_latency_on_the_c_abi`.
  Red when nothing is published. Browser:
  `host-web` `tests::structural_apply::a_declared_discontinuity_resets_the_floors_on_the_browser`,
  red when the host does not forward. Parity:
  `control_parity::a_declared_discontinuity_resets_the_floors_alike_on_both_hosts` (48 and
  96 kHz), red at block 8 when the browser does not forward.
- Gate 3: `...::a_declaration_with_nothing_to_reset_publishes_nothing` (revision 0 and after an
  added track). Red when the declaration always rebuilds.
- Gate 4: `...::a_refused_declaration_changes_nothing`. Red when the error is swallowed. The retry
  under the lifted cap resets the floors.
- Gate 5: `...::a_declared_stop_supersedes_a_pending_warm_successor_alone_and_folded`. Red when
  the successor is published `Primed`, and when the pending candidate is left alone.
- Supersession: `...::an_edit_that_supersedes_a_discontinuity_successor_is_prepared_at_it`. Red
  without the sticky rule.
- Gate 6: `abi::tests::masks_and_result_codes_are_frozen` (the OR of the symbols) and
  `ffi::tests::version_and_capabilities_are_exact`.
- Gate 7 command: all suites pass. `host-web` (test-support) and `graph-compiler` suites pass.
  `check-capi-abi.sh` and `--self-test` pass. `check-workspace-policy.sh` passes. `fmt` passes.
  Clippy passes on the touched crates. Rustdoc passes. `realtime-policy` passes.
  `check-sdk-generated.sh` passes. `check-abi-layout-v1.py` and `--self-test` pass.
  `check-web-audioworklet.sh` passes.

**Measured.** `audit capi`: allocations 0, deallocations 0, syscalls 0, pcm_digest
`4f52089e4a8cf83c` (unchanged; `tools/audit` not touched). C ABI gate 2: latency 486 -> 0 at
48 kHz. No resource ceiling was re-pinned.
