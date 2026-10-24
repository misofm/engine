# Change the spectrum taps at run time with SPECTRUM_CONFIGURE

Slice S30b of decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, design points C1,
C9 and C13). Code anchors verified on `main` `df585e503`. Feature sprint mode (owner, 2026-10-13):
one implementer, normal tests, a batch review; no per-attempt verifier or mutation runs.

Split from the planner's S30: #1593 makes the spectrum configuration ride every plan; this slice
adds the message that changes it.

## Product outcome

A C ABI host sets, changes or removes the spectrum taps of a running session with one command,
`SPECTRUM_CONFIGURE`, through `miso_engine_v1_submit_command`. A change of the taps prepares a
replacement plan at the unchanged committed revision; taps that stay keep their open window and
history. The reply says which configuration is set and which plan carries it, and every spectrum
read says which configuration it reads. No revision, no saved state and no new C symbol.

## Context

- **Message registry.** `MessageId` (`crates/protocol/src/wire.rs:98`), `from_raw` (`:143`),
  `permits_kind` (`:168`); the allocated set (`allocated_id`,
  `crates/protocol/src/message_wire.rs:3534-3540`); the capability registry
  (`crates/protocol/src/controller.rs:3219`). #1551 takes `0x000c` and `0x000d`; the frozen C13
  codes give this message `0x0010`.
- **Dispatch.** `prepare_command_frame` (`controller.rs:1566`) returns
  `PreparedCommandFrame::Immediate` or `::Structural` (`:1072`); `SessionState::command`
  (`crates/control-plane/src/control.rs:1082`) runs `service()` first (`:1059`), then executes.
- **Every gate that refuses an unchanged revision.** A rebuild today always commits a new
  revision. For one at the unchanged revision `r`:
  - the structural arm refuses a revision outside `1..=GATE_REVISION_MAX` (`control.rs:1140-1143`);
  - preparation refuses a successor base revision of 0 with `host.successor.revision`
    (`crates/host-core/src/prepare.rs:1409`, `:2680`);
  - a gate's `stamp` refuses a revision at or below the last published one
    (`crates/engine/src/realtime/latest_cell.rs:302-303`), and `publish` asserts a revision above
    it (`:254-266`); a fresh candidate gate starts at 0;
  - the watermark ignores an advance to a revision at or below its own (`watermark.rs:186-189`),
    and a claim keeps its outcome and superseded count for the next advance (`:171-176`). Every
    production candidate is `CandidateOutcome::Exact` today (`watermark.rs:52-56`).
  - Preparation stamps its carry retargets only when there are some (`prepare.rs:2299-2318`); a
    successor of an equal model has none.
  - The first plan publishes its revision on its gate only when it is above 0
    (`crates/engine/src/realtime/plan_exchange.rs:387-391`).
- **#1593**: `SpectrumMonitor`, `SpectrumConfiguration`, `retain_present`, the readers paired at
  promotion and `configured_epoch` and `adopted_epoch`.

## Decisions frozen for this slice

- **D1. ID.** `MessageId::SpectrumConfigure = 0x0010`, command and response. The allocated
  command set and `from_raw` include it. Contract identity stays V1; Rust names are unversioned.
- **D2. Request** (`spectrum_configure_request`, TLV per `docs/CONTROL_BTLV_V1.md`):
  1 `form` `U8` (1 single, 2 collection); 2 `entry` `Message` repeated (`spectrum_entry`:
  1 `target_kind` `U8` (1 track post-input, 2 track post-pan, 3 output), 2 `target_id` `Utf8`
  (1 to 127 bytes), 3 `channels` `U8` (1 left, 2 right, 3 both)); 3 `maximum_capture_bytes` `U64`;
  4 `hop_frames` `U32` optional (256, 512, 1024 or 2048; absent: the default cadence). A value
  outside a table is `INVALID_FIELD`. The form rules are #1591 D2's constructors: a single form
  holds exactly one entry; a collection holds zero or more; an empty collection with budget 0
  removes every tap. A `SpectrumRequestError` is `INVALID_FIELD` with its code (as in #1595 D6).
- **D3. Reply** (`spectrum_configured`): 1 `revision` `U64` (the committed revision, unchanged),
  2 `configured_epoch` `U64`, 3 `adopted_epoch` `U64`, 4 `rebuild` `Bool`, 5 `plan_epoch` `U64`
  optional (present exactly when `rebuild` is true).
- **D4. Every check before any write.** In order: the codec; D2's rules; each target names a strip
  (track or submix) or an output of the committed model (`NOT_FOUND`, `spectrum.target.unknown`);
  no duplicate entry (`VALIDATION_FAILED`, `spectrum.duplicate`); no pending candidate and no
  lagging epoch (`BACKPRESSURE`, as the structural arm); the preparation and resource admission
  (`LIMIT_EXCEEDED` with the `host.spectrum.*` or resource code). A refusal changes nothing.
- **D5. What rebuilds.**
  - An equal configuration changes nothing: `rebuild` false, both epochs unchanged.
  - A change of only the hop or the budget publishes no plan: the configuration is stored and both
    epochs advance by 1 together. The hop applies at the next stream start; a running stream keeps
    its cadence. A budget below what the prepared captures retain is `LIMIT_EXCEEDED`
    (`spectrum.budget.retained`).
  - Any change of the form or of the entries is a monitor rebuild (D6). A form change carries no
    capture (#1395's pairing is per form).
- **D6. The monitor rebuild at the unchanged revision `r`.**
  - The structural arm's steps after the protocol prepare (prepare, admit, reserve, push the
    candidate provider and its report row, commit the reservation) move into one function,
    `publish_successor`. The structural arm calls it and keeps its stamp and its protocol commit;
    the monitor rebuild calls it with neither.
  - `SuccessorBase` gains `unchanged_revision: bool` (false everywhere else). With it,
    preparation accepts `0..=GATE_REVISION_MAX` (the check at `prepare.rs:1409` reads the flag),
    takes `committed` equal to the prospective model and empty `EditRamps`, and refuses a non-empty
    retarget set with `host.successor.monitor` (it cannot happen for equal models).
  - No stamp is taken. The candidate's gate publishes `r` only when `r > 0`, as the first plan
    does. Render reports `r` at adoption, so the watermark does not move.
  - The candidate's outcome is `CandidateOutcome::Exact` with a superseded count of 0, so its claim
    adds nothing to the next revision's advance. A monitor rebuild is never a transition fallback.
  - No `SESSION_COMMITTED` event, no revision, no snapshot change.
  - The configuration is stored and `configured_epoch` advances after the reservation commits.
    `adopted_epoch` advances at promotion (#1593 D5).
- **D7. How adoption is reported.** The watermark cannot report a monitor rebuild: no revision
  advances. The reply's `plan_epoch` names the plan. Every spectrum reply (#1595) carries
  `adopted_epoch`; the configuration is in effect when it equals the reply's `configured_epoch`.
  Any command runs `service()`, which promotes an adopted candidate, so polling needs no new call.
- **D8. Replay and revision.** The reply is recorded like every response: an exact replay returns
  the same bytes and prepares nothing. The request has no expected revision: it changes no model
  value.
- **D9. One dispatch form for commands the session executes.** The controller decodes and checks
  the frame and returns a new `PreparedCommandFrame::Session(PreparedSessionCommand)` (request ID,
  replay plan, decoded payload). `SessionState::command` executes it and calls
  `ProtocolController::complete_session_command(prepared, reply)` (encode, record the replay
  entry, write the response) or `complete_session_command_refused(prepared, status, code)`. #1595
  uses the same variant. If a decision-17 slice that serves another session command (#1573, #1578,
  #1586) lands a variant like this first, use it: one variant for every session-executed command.
- **D10. Capability.** Capability flag bit 16 (frozen for spectrum by the C13 writers) means
  "serves `0x0010`, `0x0011` and `0x0012`, with `hop_frames`"; it replaces the browser's
  `miso_engine_web_v1_spectrum_hop_capability` export. `CapabilityFlags::KNOWN` gains it, and the
  decode invariants (`crates/protocol/src/message_wire.rs:3505-3530`) gain two rules: bit 16 is set
  exactly when all three commands are listed, and the three are listed together or not at all.
  `ProviderFeatures` gains `spectrum`. This slice serves `0x0010` and keeps the three IDs unlisted
  and bit 16 clear; #1595 lists them and sets the bit on the C ABI session.
- **D11. Docs.** `docs/CONTROL_PROTOCOL_REGISTRY.md`, `docs/CONTROL_BTLV_V1.md`,
  `docs/CONTROL_PROTOCOL_SEMANTICS.md` (monitor class, D5-D7), `docs/C_ABI_V1_QUALIFICATION.md`,
  and the prose of `crates/capi/include/miso_engine_v1.h`. No C symbol or struct changes.

## Deliverables

1. D1-D3, D9's protocol half and D10 in `crates/protocol`; corpus frames in
   `crates/conformance/src/protocol_corpus.rs` and the re-pinned schema hash.
2. D4-D9 in `crates/control-plane/src/{control.rs,spectrum.rs,compile.rs}`; D6's flag in
   `crates/host-core/src/prepare.rs`.
3. D11.
4. The C ABI tests (`crates/capi/src/runtime/spectrum_tests.rs`).

## Authorized paths

- `crates/protocol/src/**`, `crates/conformance/src/protocol_corpus.rs`,
  `crates/conformance/tests/conformance_corpus.rs`, `fuzz/corpus/complete-schema-manifest.md`.
- `crates/control-plane/src/**`, `crates/host-core/src/prepare.rs` (D6's flag only).
- `crates/capi/src/runtime/spectrum_tests.rs`, `crates/capi/include/miso_engine_v1.h` (prose),
  `tools/audit/src/protocol.rs` (if its registry list needs the new ID).
- `docs/CONTROL_PROTOCOL_{REGISTRY,SEMANTICS}.md`, `docs/CONTROL_BTLV_V1.md`,
  `docs/C_ABI_V1_QUALIFICATION.md`.

## Non-goals

- No read message (#1595). No browser change (#1596).
- No new watermark field: adoption is reported by D7.

## Objective gates

Commands: `cargo test --locked -p protocol --all-features`, `cargo test --locked -p conformance`,
`cargo test --locked -p control-plane --features test-support`,
`cargo test --locked -p host-core --features host-core/test-support`,
`cargo test --locked -p capi`, `cargo build --locked --release -p audit -p capi &&
target/release/audit capi`, `bash scripts/check-capi-abi.sh`,
`cargo run --locked --release -q -p realtime-policy`.

Fixture as #1593's. The tests send frames through `miso_engine_v1_submit_command` and read spectrum
through #1593's `test_spectrum_mut()`.

1. **Codec.** Request and reply round-trip; each table value outside D2, a 128-byte ID, a hop of
   300 and a single form with two entries are refused with the D2 status, on encode and decode. A
   capability record with bit 16 and only `0x0010` listed, or with the three IDs and bit 16 clear,
   is refused on decode.
2. **Revision 0.** On a freshly compiled session at revision 0, configure single
   `(t1 post-pan, Stereo)`: the reply has `revision` 0, `rebuild` true and a `plan_epoch`. After
   render blocks and one `miso_engine_v1_service`, `adopted_epoch == configured_epoch`, and an armed
   window equals the reference window of #1593 gate 1.
3. **The watermark is exact.** On a session at revision 5, configure (a rebuild): after adoption,
   `miso_engine_v1_plan_watermark` reports revision 5 with flags and counters unchanged. A
   following live fader `SET_CONTROL` advances it to 6 with `OUTCOME_EXACT`, `exact` +1,
   `transition_fallback` 0 and `superseded` 0.
4. **No-op and hop.** The same configuration again: `rebuild` false, epochs and plan epoch
   unchanged. Only `hop_frames` 1024: `rebuild` false, epochs +1, a running stream keeps its hop,
   and a stream started after it publishes windows 1024 samples apart.
5. **Keep, add, remove.** From collection `[(t1, Stereo)]` with a running stream: configure
   `[(t1, Stereo), (t2 post-input, Left)]`; `t1`'s stream carries (no `Failed`, `Gap` or
   `Warming`; same `capture_epoch`). Configure `[(t2, Left)]`: the stream stops and reads
   `Inactive`. Configure an empty collection with budget 0: every read is `Unprepared`, and the
   next plan has no spectrum observer.
6. **Refusals change nothing.** An unknown target, a duplicate entry, a budget below one capture
   and a configure while a structural candidate is pending: each is refused with its D4 status and
   code, and the configuration, both epochs, the plan epoch and the watermark are unchanged.
7. **Replay.** An exact replay of a configure returns the same bytes and publishes no plan.
8. **Realtime.** While the render thread renders, the control thread alternates two
   configurations 200 times: the render thread's `bench_support::alloc` counters read
   `allocations == 0 && frees == 0` around every render call, and no call answers `INTERNAL`.

## Test value

- Gate 1 turns red if the codec accepts a malformed or contradictory request, or a capability
  record whose bit 16 disagrees with its command list.
- Gate 2 turns red if any gate in Context still refuses the unchanged revision 0.
- Gate 3 turns red if a monitor rebuild moves the watermark or leaks its claim into the next
  revision's outcome.
- Gate 4 turns red if a no-op or a hop change publishes a plan, or a hop change restarts a stream.
- Gate 5 turns red if a kept tap restarts, or a removed tap stays in the plan.
- Gate 6 turns red if a refused configure writes anything.
- Gate 7 turns red if a replayed configure prepares a plan again.
- Gate 8 turns red if the monitor rebuild allocates on render or races into an internal error.

## Dependencies

- #1593 (the monitor state and the pairing), #1551 (the registry pattern and the allocated set).
- `crates/capi` is stream B's: rebase after stream B's open edits.

## Supersedes and shrinks

- None.

## Decision and evidence record (attempt 1)

Root decisions of 2026-10-24 (scope note `1594.md`; they amend the frozen decisions above):

- **R1 (amends D6).** No `publish_successor` extraction: the monitor rebuild is a third
  `Rebuild::Monitor` kind run by the existing `rebuild()`. It carries like a transaction, commits
  like a discontinuity, takes no stamp and publishes its revision only when it is above 0.
  `SuccessorBase.unchanged_revision` relaxes host-core's successor revision check.
- **R2 (amends D4).** A configure over a pending candidate supersedes it through `rebuild()`
  (D4's `BACKPRESSURE` is stale since #1310). The command acks nothing new; a refusal republishes
  the withdrawn candidate whole.
- **R3 (moves D10).** D10 (capability bit 16, `ProviderFeatures::spectrum`, `KNOWN`, the decode
  invariants) moves whole to #1595. For one batch `0x0010` is served and not advertised on both
  hosts: it is in the allocated command set, so a capabilities record may list it, but no
  registry does.
- **R4.** Tests: the control-plane gates in `crates/control-plane/tests/spectrum_monitor.rs`, the
  replay in `crates/protocol/src/controller/tests.rs`, one capi parity row.
- **R5, R6.** `host-core/src/{spectrum_service.rs,spectrum.rs}` are authorized; the service and
  analysis bytes of a promotion are charged to capi's ledger with one constant.
- **Corpus paths (root, 2026-10-24).** The re-pin also edits `crates/conformance/src/main.rs`,
  `crates/conformance/tests/mutation_million.rs`, `scripts/check-protocol-wasm-parity.sh` and
  `docs/CONTROL_PROTOCOL_CONFORMANCE.md`, as #1386 did.

Checkpoint 1 (protocol):

- D1-D3: `MessageId::SpectrumConfigure = 0x0010`; `SpectrumConfigureRequest`, `SpectrumEntry`,
  `SpectrumForm`, `SpectrumTargetKind`, `SpectrumConfigured` (`message_wire/spectrum.rs`). The codec
  checks the tables, the identity length (1-127 bytes), the hop table and the single form's one
  entry, both ways; a reply's `rebuild` must agree with its plan epoch. The budget rules stay the
  core's (`spectrum.budget`).
- D9: `PreparedCommandFrame::Session(Box<PreparedSessionCommand>)` with `SessionCommand` and
  `SessionCommandReply`. The token is affine like the structural one (it holds the odd controller
  generation, so every other entry point refuses while it is outstanding, and a dropped token
  leaves the replay cache as it was). `complete_session_command` and
  `complete_session_command_refused` encode the reply, then apply the replay plan and complete the
  entry in the same call. A replay, reuse, expiry or replay backpressure takes the one-call path
  and never reaches the session. The one-call path (`process*`) answers `UNAVAILABLE`.
- Corpus: `command.spectrum_configure` and `response.spectrum_configured` are added (50 frames).
  `COMPLETE_SCHEMA_HASH` moves from `c85b2a0dff5d2a92` to `26579fcd7f959310`. The FNV hash of the
  48 earlier frames alone, in order, is `c85b2a0dff5d2a92`, the old pin: they are byte-equal.
- `tools/audit/src/protocol.rs` is untouched: it enumerates no message IDs.
- Checkpoint 1 leaves the control plane answering `UNAVAILABLE` (`spectrum.unavailable`) for the
  new frame; checkpoint 2 replaces that arm with the executor.

Checkpoint 2 (control plane, host-core, capi):

- `SessionState::command` completes `PreparedCommandFrame::Session` through
  `session_command` / `spectrum_configure`: the D4 checks in order against the committed model,
  then D5. A typed refusal is a recorded non-OK reply; a retryable or internal error (plan gone,
  backpressure) drops the token, so nothing is recorded or acknowledged. A preparation or
  admission refusal of the monitor rebuild is `LIMIT_EXCEEDED` with the preparation's first code.
  Duplicates are exact `(kind, id, channels)` repeats, as preparation's `host.spectrum.duplicate`.
- D5 hop or budget only: `SpectrumMonitor::retune` stores the configuration, advances both epochs
  (the adopted one when the running captures are the replaced configuration's) and sets the
  service's hop with `SpectrumService::set_hop`; a pending candidate of the replaced epoch takes the
  new epoch and hop. The budget check uses `SpectrumPreparation::retained_bytes`.
- R1 as built: `Rebuild::Monitor { model, configuration }` in `rebuild()`: no edit ramps, no
  monitor write, the request's configuration instead of `successor_configuration`, no stamp,
  `publish(r)` only when `r > 0`, outcome `Exact`, commit as a discontinuity. Deviation found while
  building: `SuccessorBase.unchanged_revision` is set only when the base is the running plan's (or
  `r = 0`). Across a withdrawn candidate at a newer revision the base is the older plan, whose
  model differs by the candidate's edits, so preparation stamps those retargets with `r` exactly as
  for a transaction; the flag's retarget refusal (`host.successor.monitor`) would otherwise refuse
  every supersession. `fold_send_move` (ramp 0) and `replace_session_catalog` (same structure) are
  no-ops for the equal model, as R1 asked to verify.
- `SpectrumMonitor::adopt` now sets the kept service's hop from the promoted candidate, so a hop
  that changed while a rebuild was pending reaches the service; a promotion that installs captures
  after every tap was removed makes a new service (the case #1593 T3 named).
- R6: `SpectrumService::RETAINED_BYTES` (the stream window, the analyzer and the smoothing history)
  is added to a plan's capi `active_retained` when the plan has spectrum captures.
- hosts/host-web is untouched: its preparer returns no captures until #1596, so a browser
  configure makes candidates without captures and `promote_spectrum`'s `debug_assert` (captures
  after a removed service) cannot fire; #1596 must create the service there.
- The #1595 spec records R3 (D8a).

Gates (attempt 1): the codec and replay tests in `crates/protocol`; six tests in
`crates/control-plane/tests/spectrum_monitor.rs` (revision 0, watermark, no-op and hop, keep/add/
remove/re-add, refusals and supersession, 40 alternations against a rendering thread); one capi
row. Mutants, each red on its gate and restored: revision 0 refused in preparation (revision-0
test), a monitor rebuild that publishes `r + 1` (watermark and supersession tests), a retune that
leaves the service's hop (hop test).

Attempt 2 (after the attempt-1 verdict, FAIL):

- A monitor rebuild that supersedes a withdrawn candidate takes a transaction's outcome terms
  (`follow_into_six_word`, the send fallback and the rebuild send ramp): it carries the withdrawn
  edits against the same older base. Red first: `a_configure_that_supersedes_a_fallback_edit_
  reports_the_fallback` (removing `eq0`'s last mute entry under a following send) was red with the
  old gate (`transition_fallback` 0) and is green with the fix.
- The supersession test's pending edit now also moves `eq1`'s fader (a carry retarget), so the
  `unchanged_revision` restriction is exercised: the mutant `unchanged_revision = monitor` turns it
  and the fallback test red.
- The hop test configures a hop-only change while a structural candidate is pending, then checks
  adopted == configured and the 512-frame spacing; the mutant that skips moving the candidate turns
  it red.
- R6: `CapiResources::spectrum_service` (part of `active_retained`) is added to the replacement
  peak beyond the newest plan's, as `edit_ramps` is; a unit row in `compile.rs` (accepted at the
  peak, refused one byte below, nothing for a kept service) and a session row (the running plan's
  control bytes grow by the service plus at most 1 KiB of its spectrum rows, and fall back) turn red
  when either charge is removed. Root's attempt-2 note authorized the field in
  `crates/capi/src/runtime/live_peak_tests.rs`.
- A non-superseding monitor rebuild two blocks into a 4,000-sample send move on a following route
  renders bit-equal to a twin that never configures (`a_monitor_rebuild_keeps_a_live_send_move_on_a_
  following_route`).
