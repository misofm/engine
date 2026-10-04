# Deliver value-only fader, mute and pan transactions to the running C ABI plan through the live console lanes

**Umbrella.** Rewritten on 2026-10-04 from the single-issue spec of 2026-09-28. It closes when its
thirteen slices close (tables below). Every anchor was verified on `main` at `54b0a1bf8`.

The title keeps the original wording. The scope was widened on 2026-10-04, at the coordinator's
request, to every C ABI value that decision 14 classifies live (its follow-up F5: the input
section, effect parameters and effect bypass) and to model-only edits (its F8). The core slices
(#1253-#1258) deliver fader, mute and pan first; the others build on them.

- Research record: `docs/handoffs/live-control-2026-09-28/` (`FINDINGS.md`, `VERIFY.md`).
- The superseded single-issue text and its amendments A0-A5 are in git history
  (`git show 54b0a1bf8:.github/ISSUE_SPECS/1053-deliver-value-only-fader-mute-and-pan-transactions-to-the-running-c-abi-plan-thr.md`).
  "History" at the end says where each of them went.
- The rule: decision 14, *Live update versus plan rebuild* (#1259,
  `docs/rulings/live-update-versus-rebuild-2026-10-04.md`). Its live rows for a track's
  `fader_db`, `mute`, `pan`, `matrix_*`, `trim_db`, `polarity_invert`, `hpf_hz` and `lpf_hz`, for
  effect parameters with `automation_rate` `Block` and for effect bypass become live on the C ABI
  here. Its C1 ("one edit API; the engine decides") is this umbrella's product outcome.

## Product outcome

A phone, or any C ABI host, keeps one edit API: `miso_engine_v1_submit_command` with
`SESSION_TRANSACTION_APPLY`. The engine classifies each committed transaction itself:

- **Live update.** The committed session before and after the transaction differ only in live
  values of tracks: fader, mute, pan or matrix (smoothing included), input trim, polarity, HPF and
  LPF, live effect parameters and effect bypass. The engine queues records to the running plan. No
  plan is compiled; source rings, effect state and the render position continue; nothing acked is
  ever lost.
- **Model-only commit.** The two models differ only in fields no plan reads: the session ID, the
  profile IDs, and the stored automation table while nothing renders it. The engine commits with
  no record and no rebuild.
- **Plan rebuild.** Every other transaction, exactly as today.

Today every transaction is a plan rebuild. A fader move costs a full preparation, a source-ring
reset (`crates/capi/src/runtime/control.rs:46-53`, `:748-754`), a restart of every filter,
dynamics and delay state, silence until the host seeks and resubmits, and a hard step to the new
value.

## On `main` (`54b0a1bf8`)

- **The C ABI prepares no live lanes.** `prepare_runtime` calls `prepare_host_runtime(compiled,
  &caps)` (`crates/capi/src/runtime/compile.rs:421`), which requests no live controls
  (`crates/host-core/src/prepare.rs:563-574`). `ProviderEpoch` holds only the source producers
  (`control.rs:9-12`).
- **Every transaction takes the structural arm** of `SessionState::command` (`control.rs:694-860`):
  `plan_alive` (`:727`), the epoch-lag check (`:734`), the response-size check (`:737-742`),
  `prepare_runtime` (`:748-754`), `validate_replacement_peak` with `compiled_model_admission`
  (`:773-784`), the pending check (`:792`), the plan reservation, the protocol commit (`:840-842`)
  and the publication (`:843-848`).
- **The plan-swap race (old L0) is fixed** by #1042: `synchronize_plan_epochs` promotes the pending
  candidate when the reclaimed plan is the current one (`control.rs:625-675`).
- **The live lanes exist in host-core and are shared code.** One request field,
  `HostLiveControlRequest::control_queue_depth` (`prepare.rs:295-332`), attaches all of these
  together:
  - every strip's matrix, fader and input queues (`:985-994`);
  - one queue per effect instance (`:942-947`);
  - one queue per route into a submix (`:1127-1140`).

  The producers come back in `HostLiveControlHandles` (`:359-430`). The strips are the tracks
  first, then the submixes, each in canonical ID order. `prepare_host_runtime_with_live_controls`
  prepares with `Concurrent` delivery (`crates/builtins-compiler/src/lib.rs:3194-3210`). That is the
  same delivery the C ABI already prepares with, so the fader and the matrix keep their unfused
  bank processors (the fused pass needs `BetweenRenderCalls`, `:1043-1046`).
- **Two of the four builtin drains are unbounded.** `drain_fader_controls` (`:970-1004`) and
  `drain_matrix_controls` (`:1006-1026`) loop `while let Ok(record) = control.try_pop()`. The input
  drain (`:469-472`), the effect lane (`crates/effect-contract/src/live.rs:351-365`) and the route
  drain (`crates/graph/src/runtime.rs:897-900`) stop at `available_at_entry()`. The two test-only
  scalar drains (`:4200`, `:4266`) are unbounded too.
- **A live input lane makes the strip's builtin tail infinite.** Any strip with a control request
  gets `BuiltinTail::Infinite` (`builtins-compiler/src/lib.rs:3435-3439`), because a live filter
  target can enable a filter. On the C ABI that would flip `tail_kind` in every resource report
  (`compile.rs:430-433`).
- **Domains.** The session checks a fader's dB and a matrix's coefficients only for finiteness
  (`crates/session/src/validate.rs:454-472`). The `[-144, 24]` dB domain is builtins' private
  `checked_fader_gain` (`crates/builtins/src/lib.rs:4108-4113`). Preparation (`fader_lanes`,
  `:4134`) and the render-side setter (`BuiltinFaderBank::set_fader_db`, `:3695-3711`) both call
  it. The matrix domain is `Matrix2x2::checked` (`:88-103`) and `pan_matrix` (`:4179-4194`).
- **What a fader record does.** On a muted lane, `set_fader_gain` stores the gain and keeps the
  target at zero (`builtins/src/lib.rs:2613-2629`). `set_mute` retargets to zero or to the stored
  gain (`:2631-2652`). Every retarget restarts the ramp from the current value (`:2588-2607`).
- **Never emit a redundant record** (`crates/host-core/src/solo.rs:58-70`). When a settled lane
  re-enters the ramp kernel, an exact `+0.0` can turn into `-0.0`, and the digest shows it.
- **What a prepared plan reads from a strip's fader.** Only one consumer reads a strip's fader
  apart from its own fader stage: a `follows_mute` route reads its source strip's effective mute
  (`crates/graph-compiler/src/compile.rs:323-357`). VCAs compose into the prepared fader through
  `SessionModel::effective_strip_faders` (`crates/session/src/vca.rs:100`).

## Decision record

- **D1. Classification is by the committed-model delta.** Edit opcodes are never inspected.
  `host_core::classify_live_delta(current, next, ramps)` reads the two normalized models: that of the
  committed session and that of the token's prospective session. The steps run in this order:
  1. **The VCA guard (G3).** If either model declares a VCA, the delta is a rebuild.
  2. **The track set.** The two models must have the same track IDs in the same normalized order.
     Otherwise the delta is a rebuild.
  3. **The mask.** Clone `next`, set its revision to `current`'s, and copy `current`'s `fader` and
     `matrix_or_pan` into every track. Then compare the `session::canonical_session_json` bytes of
     that masked model with those of `current`. If they differ, the delta is a rebuild.
     - The comparison is on bytes, never on `PartialEq`. `PartialEq` equates `-0.0` and `+0.0`,
       so an edit that only flips the sign of a zero in a non-live field (for example `trim_db`)
       would read as live, although a rebuild could render it differently.
     - The canonical `f32` spelling keeps every bit, `-0.0` included
       (`crates/session/src/value.rs:22-37`).
     - The mask makes the submix guard (G1) structural by construction: only track fields are
       masked.
  4. **The domain.** A post-commit value that the setter's own checker refuses makes the delta a
     rebuild. The structural path then reports the same preparation diagnostic it reports today.
  5. **The follow guard (G2).** A track whose mute changes, and that is the source
     (`RouteSource::Track`) of a route with `follows_mute: true` in `next`, makes the delta a
     rebuild.

  6. **The records.** For each track, a record is emitted only when the value the render plane
     holds would change (D9):
     - **Fader.** A `FaderDb` record per lane whose `checked_fader_gain(db)` bits change; one
       `Both` record when both lanes change to the same `db` bits.
     - **Mute.** A `Mute` record per lane whose mute changes; one `Both` record when both lanes
       change to the same value.
     - **Matrix.** One matrix record when the lowered target's bits change. The target is
       `pan_matrix` for a pan and `Matrix2x2::checked` for a matrix, through one shared lowering
       function. The record carries the post-commit `smoothing_samples`.
       - A switch between `Pan` and `Matrix` that keeps the same target emits nothing.
       - A change of smoothing alone emits nothing. The window only applies to a later unsmoothed
         `set_target`, and no production code calls that.
     - **Order.** In a strip's fader queue, fader records come before mute records. Either order
       gives the same final ramp; the fixed order keeps the tests exact.
  The output, per changed track, is the track ID, at most four fader records and at most one
  matrix record. An empty output is valid: a transaction that rewrites identical values is live
  with zero records, and it still advances the revision and emits `SESSION_COMMITTED`.
- **D2. The timing contract.** An edit is admitted all or nothing.
  - It is applied no later than the first block whose render call begins after the submit returns.
  - A stage that has not drained yet in the current block may apply it one block earlier, so the
    records of one transaction can apply up to one quantum apart.
  - It is heard up to `latency_samples` later, because the compensation delays are downstream of
    the fader.
  - It is never lost.
  - #444's block-atomic batch claim is declined knowingly.
- **D3. The ramps.**
  - **Fader and mute records** take their lengths from one seam,
    `host_core::LiveRamps::for_session(model)`. It returns 0 (a step) until *Session
    `controlSmoothing`: configurable ramp lengths for live mute, fader and pan changes* (#1054)
    replaces its body with the session's `controlSmoothing`. The owner ruled (2026-09-28) that ramp
    lengths are session settings and are not hardcoded.
  - A step is bit-exact to the value today's rebuild bakes. It still removes the silent block, the
    ring reset and the DSP restart.
  - **Pan and matrix records** carry the post-commit model's own `smoothing_samples`.
- **D4. The queue depth.**
  - The depth is fixed at 16 records per lane per strip (a capi constant). There is no
    compile-limit field.
  - A net delta puts at most 4 fader records and 1 matrix record per strip into one transaction.
    So at least 4 worst-case transactions, and 16 single-value moves, fit between two render calls.
  - A full lane is typed `BACKPRESSURE` before anything changes.
  - A host that stops calling render (a paused transport) gets `BACKPRESSURE` after 16 single-value
    edits per strip. Today it gets one on its second edit, because a pending candidate refuses the
    next structural edit (`control.rs:792`).
- **D5. The lanes on the C ABI.** The core attaches only the fader and matrix lanes of every
  strip, through host-core's new `HostLiveLanes` selection (#1254). Each later lane arrives with the
  slice that pushes to it:
  - **The input lane** arrives with #1261. Before that it would make every C ABI plan report
    `TAIL_INFINITE` (Q4) and carry the largest ring (40-byte records) with nothing to push.
  - **The effect lanes** arrive with #1263.
  - **The route lanes** arrive with *Deliver value-only send and submix-strip edits to the running
    C ABI plan* (#1225).
  - Delivery stays `Concurrent`, which is what the C ABI renders with today, so rendering does not
    change.
- **D6. The commit order, and the acked-batch question.** In the structural arm, after the existing
  `plan_alive` and response-size checks, the steps run in this order:
  1. Classify (D1). A rebuild takes the existing path, unchanged.
  2. Run the live admission (D8). A refusal is `COMPILE_REJECTED`.
  3. Resolve each changed track's producer by ID in the newest epoch (D7). A missing ID is
     `INTERNAL`; it cannot happen.
  4. Check the room on every queue the delta touches (`Producer::available_capacity`). Too little
     room is `BACKPRESSURE`.
  5. Run `ProtocolController::check_prepared_structural`. This is new: it is the commit's own
     predicate, factored out, and the commit calls it first. A failure is `INTERNAL`.
  6. Push every record. A push cannot fail: the control thread is the only producer, and a
     consumer pop only grows the room (`crates/engine/src/realtime/spsc.rs:320-328`).
  7. Commit the token. The commit cannot fail after step 5, under the same `&mut` borrow.
  8. Write the committed response.

  A failure in steps 1-5 changes nothing: the model, the revision, the replay cache, the events and
  every queue stay as they were. Render applies every record it pops, because D1 refused every value
  the setter would refuse. **So no ack ever precedes a drop.**
- **D7. Which plan gets the records.** The newest epoch: the pending candidate if there is one,
  else the current provider. `replacement_base_report` uses the same rule (`control.rs:611-623`).
  - A candidate was prepared from the committed model before this edit, so it needs the records.
    The current plan is about to retire.
  - The live arm skips the epoch-lag check (`:734`). That check protects the report table, and the
    live arm adds no row. During the lag, the newest provider is the plan that is rendering
    (`synchronize_plan_epochs`, `:640-649`).
- **D8. The live admission.** The live arm prepares no plan. But the prospective compiled model
  lives beside the current one until the commit, and a value edit can grow the canonical JSON (for
  example `-6.0` becoming `-6.0123`). The edit is admitted only if all three of these hold:
  - **Graph.** The `graph_session_plus_plan_bytes` of the current plan and of any pending plan,
    plus `compiled_model_admission(current, prospective).retained_bytes`, is at most
    `maximum_graph_session_plus_plan_bytes`.
  - **capi.** The newest plan's `capi_retained_bytes`, plus the current provider's
    `epoch_retained` when a candidate is pending, plus the newest provider's
    `prepared_protocol_retained`, is at most `maximum_capi_retained_bytes`. The last term includes
    a candidate catalog the live arm never builds: a conservative overcount, kept for simplicity
    and documented.
  - **Largest allocation.** The maximum of both plans' `largest_named_allocation_bytes`, the newest
    provider's `CapiResources::largest` and the compiled models' largest allocation is at most
    `maximum_named_allocation_bytes`.

  Each provider epoch keeps its `CapiResources` for this check (#1257).
- **D9. The committed model is the authority, and the plan follows it.**
  - **The invariant.** The committed model's track values are the values the newest plan was
    prepared with, plus every record pushed into that plan since.
  - **A rebuild** prepares from the committed model, so it starts at every acked value. Records
    still queued in the retiring plan die with it; only their ramp is lost.
  - **`SessionSnapshotGet`** returns the committed model.
  - **A refused edit** changes neither the model nor the plan.
- **D10. What the host sees.**
  - The same response (`TransactionApplied`) and the same events as for a rebuild: one
    `SESSION_COMMITTED`, and one `AUTOMATION_CANCELED` per queued automation batch, because the
    revision changed (`commit_prepared_structural`, `crates/protocol/src/controller.rs:1944-2004`).
  - No new symbol, opcode, field, result code or event.
  - The host must drain the reliable lane after each edit (its capacity is 2,
    `compile.rs:98`), as it must after a rebuild.
  - A host detects a rebuild as it does today: its next source submission is refused until it
    seeks. After a live edit it just goes on submitting.
  - The header comment, `docs/C_ABI_V1_QUALIFICATION.md` and `docs/CONTROL_PROTOCOL_SEMANTICS.md`
    say all of this (#1257).
- **D11. Bounded drains.** Each of the four builtin fader and matrix drains pops at most the
  records present at block entry (#1253). The realtime policy gate refuses an unbounded `try_pop`
  loop in a marked region. **A forward hazard:** any future silence skip (the owner's standing
  "skip work on silence" rule) must still run every bounded drain on a skipped block, or live edits
  stall, and then backpressure, through a silent passage.
- **D12. Model-only edits** (#1260). The mask of D1 step 3 also covers `session_id`,
  `render_profile.id`, `output_profile.id` and `automation`. Preparation reads none of them. The
  automation mask holds only while no host renders stored automation (#1058); the first issue that
  renders it must take `automation` out of the mask.
- **D13. The input section** (#1261, #1262).
  - Trim and polarity ride the strip's input lane as `TrackInputRecord`s; HPF and LPF ride it as
    prepared filter targets, which host-core's `InputFilterPreparer` designs on the control thread
    from the pre-commit values.
  - Attaching the input lane makes every C ABI plan report `TAIL_INFINITE` (owner question Q4).
  - `delay_samples` stays a rebuild (decision 14, rule 1).
- **D14. Effects** (#1263-#1266).
  - Each prepared effect instance gets a live lane on the C ABI; capi keeps the producers and the
    EQ owners, and charges them.
  - A parameter is live when its descriptor's `automation_rate` is `Block`; values resolve through
    the one function preparation uses (`resolve_initial_values`, #1264). A prepared-only
    parameter is a rebuild.
  - The EQ's live parameters ride its owner transaction with targets that host-core's
    `EqTargetPreparer` designs (#1265).
  - Bypass rides the latency-preserving shunt, except for the delay and the multiband compressor,
    whose session bypass is prepared: a change of theirs is a rebuild (decision 14, F4).
  - The protocol's parameter readback follows every live value (`set_parameter_value`, #1264).
  - A transaction whose records for one effect exceed that effect's queue capacity takes the
    rebuild path, never an endless `BACKPRESSURE`.

## Guards (DESIGN P13)

`classify_live_delta` carries all three guards from #1255 on. Each guard keeps a delta structural
until a later slice composes what depends on it. The slice that lifts a guard marks its bullet here
as superseded.

- **G1. A submix strip's fields.** Structural by construction: D1 masks only track fields. Lifted
  for faders and pans by *Deliver value-only send and submix-strip edits to the running C ABI plan*
  (#1225), and for the input section and effects by *Apply value-only submix-strip input-section and
  effect edits to the running C ABI plan* (#1267).
- **G2. The mute of a follow source.** A change to `left_mute` or `right_mute` of a strip that, in
  the post-commit model, is the source of a route with `follows_mute: true` (D1 step 5). Lifted by
  *Let C ABI sends follow their source strip's mute live* (#1226).
- **G3. Any VCA.** Every delta while the pre- or post-commit model declares a VCA (D1 step 1). Lifted
  by *Deliver value-only VCA edits to the running C ABI plan* (#1247).
- **G4. EQ parameters** (inside this umbrella). #1264 classifies every parametric EQ parameter
  change as a rebuild; #1265 lifts it for the EQ's live parameters.
  **Superseded: lifted by #1265** (`b055a48d4`, Sol attempt 1 PASS) for every EQ parameter whose
  `automation_rate` is `Block`; a band's `enabled` and `kind` stay prepared (decision 14, F2).

## Slices, in dependency order

**Core: fader, mute and pan.**

| # | Slice | Crates | Depends on |
|---|---|---|---|
| #1253 | *Bound the builtin fader and matrix drains to the records present at block entry* | builtins-compiler, realtime policy | none |
| #1254 | *Let host-core attach a strip's fader and matrix lanes without its input, effect or route lanes* | builtins-compiler, host-core, host-web | none |
| #1255 | *Classify a committed session delta as a live track fader, mute and pan update or a rebuild* | host-core, builtins, builtins-compiler | none |
| #1256 | *Prepare C ABI plans with live track fader and matrix lanes* | capi, host-core | #1254 |
| #1257 | *Apply value-only track fader, mute and pan transactions to the running C ABI plan* | capi, protocol, docs | #1253, #1255, #1256 |
| #1258 | *Qualify live C ABI edits against a concurrently rendering plan* | capi tests, tools/audit | #1257 |

**Model-only edits.**

| # | Slice | Crates | Depends on |
|---|---|---|---|
| #1260 | *Commit model-only C ABI transactions without a plan rebuild* | host-core, capi tests | #1257, #1258 |

**The input section.**

| # | Slice | Crates | Depends on |
|---|---|---|---|
| #1261 | *Apply value-only input trim and polarity edits to the running C ABI plan* | host-core, builtins, capi | #1257, #1258, #1254, owner Q4 |
| #1262 | *Apply value-only input HPF and LPF edits to the running C ABI plan through prepared targets* | host-core, capi | #1261 |

**Effects.**

| # | Slice | Crates | Depends on |
|---|---|---|---|
| #1263 | *Prepare C ABI plans with live effect lanes* | capi | #1257, #1258 |
| #1264 | *Apply value-only effect parameter edits to the running C ABI plan* | effect-compiler, host-core, capi | #1263 |
| #1265 | *Apply value-only parametric EQ parameter edits to the running C ABI plan through prepared targets* | host-core, capi | #1264 |
| #1266 | *Apply value-only effect bypass edits to the running C ABI plan* | host-core, capi | #1264 |

**Parallel work.**

- #1253, #1254 and #1255 can be implemented in parallel worktrees. All three touch
  `crates/builtins-compiler/src/lib.rs` in different places, so merge them one at a time.
- After #1258, the model-only, input and effect strands can run in parallel. They all extend
  `classify_live_delta` and capi's `commit_live`, so merge them one at a time and rebase.
- #1225, #1226 and #1247 need only the core (#1257 and #1258), not this whole umbrella.

**Follow-ups filed with this umbrella, outside it.**

- *Apply value-only submix-strip input-section and effect edits to the running C ABI plan* (#1267):
  the submix half of decision 14's F5. It depends on #1225, #1262, #1265 and #1266.
- *Elide a builtin input filter section again after a live disable settles it to identity* (#1268):
  a performance gap that live filter disables have on both hosts (found for #1262).

## Closing the umbrella

- All thirteen slices are closed with a Sol PASS, and their evidence commits are upstream.
- #1225, #1226 and #1247 name #1257 and #1258 in their dependencies. #1054 names #1257 for its C
  ABI half.

## Out of scope

- The ramps' lengths are #1054's: D3 is the only seam.
- Submix strips, sends, follow-mute and VCAs on the C ABI: #1225, #1226 and #1247. A submix
  strip's input section and effects stay structural (G1) until #1267, after #1225.
- Stored automation rendering (#1058) and effect bypass crossfades (decision 14, F7).
- Solo on the C ABI. The app composes solo as mutes in one transaction, which this umbrella makes
  live.
- #444's block-atomic batch claim, and sample-accurate transient automation.
- The fused fader-and-matrix pass on the C ABI. It needs `BetweenRenderCalls`, which a concurrent
  C ABI producer cannot declare. The C ABI never had it.

## Owner questions

- **Q1. Steps first?** This umbrella ships live fader and mute changes as steps. #1054 then makes
  them ramps, at the session's `controlSmoothing` lengths, on both hosts. The alternative is to hold
  #1257 until #1054 lands. **Recommended: ship steps first.** A step is no worse than today's
  rebuild value and removes the gap.
- **Q2. A paused transport.** While a host makes no render calls, each strip takes 16 live edits.
  The 17th returns `BACKPRESSURE`, and the app keeps only its latest value and retries once
  rendering resumes. Is that acceptable? **Recommended: yes.** A deeper queue costs memory on every
  strip.
- **Q3. Tell the host which path ran?** Today the host learns of a rebuild when its next source
  submission is refused. A "rebuilt" flag in the response would need a protocol change. **Recommended:
  not now.**
- **Q4. An infinite tail for live input filters?** A live input lane can enable an HPF or LPF, so
  every C ABI plan that carries it must report `TAIL_INFINITE`, as every browser plan with live
  controls already does. Any session with an EQ or an enabled input filter already reports it
  today, so only sessions with neither change. #1261 waits for this answer. **Recommended: yes.**
  The tail is a report; no render path reads it.

## History of the 2026-09-28 spec

- **A0 (L0, the swap race).** Delivered by #1042.
- **Draft 02, its step 4 and A4.** Delivered by #1056, which deleted the unwired #140 stack.
- **A5.** Recorded in `docs/CONTROL_PROTOCOL_SEMANTICS.md:15`: #140 is descoped, stored automation
  is #1058, and IO-5 stays open.
- **Scope items 1-6 and A1.** Distributed over D4-D11 and the slices.
- **"L2" and "L3"**, the old out-of-scope successors (effect parameters and bypass; EQ targets,
  trim and polarity), are now #1261-#1266, with the input HPF and LPF that L3 left out.
  - A1.2, "full or builtins-only lanes", is decided by D5: fader and matrix only.
  - A1.4, "push, then an infallible commit", is kept by D6.
- **A2 D1-D4.** Became D1-D4 here.
- **A3's gates.** Became the gates of #1257 and #1258. Gate 7, the domain boundary, moved to #1255.
