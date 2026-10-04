# Deliver value-only send and submix-strip edits to the running C ABI plan

Slice 27 of *Submix strips and live aux sends* (#1196). It is in batch C1, with *Let C ABI sends follow their
source strip's mute live* (#1226). It starts only after the core of #1053 (*Apply value-only track
fader, mute and pan transactions to the running C ABI plan*, #1257, and *Qualify live C ABI edits
against a concurrently rendering plan*, #1258) has closed, and after batch K3 (*Let a send follow its
source strip's mute live in the browser*, #1224) is on `main`. **Re-verify every anchor after #1257
lands:** it adds the classifier and the live commit path this slice extends. (Amended 2026-10-04,
when #1053 became an umbrella of slices #1253-#1266; the references below follow its decision
record.)

The design record cited below (`DESIGN`, `VERIFY-1` to `VERIFY-3`, `REVISION-1`, `REVISION-2` and
`APPLIED-3`) is committed in `docs/handoffs/submix-sends-2026-10-02/`.

## Product outcome

A fan's phone, or any C ABI host, can change any of these without a plan rebuild:

- a send's level, on/off or 2x2 (a route whose destination is a submix);
- a bus's fader, mute, or pan or matrix;
- a track's fader, mute, or pan or matrix (as #1053 already delivers).

Today such a change costs, as the probe measured through the C ABI (`DESIGN.md` 3.2):

- one silent block;
- a source-ring reset (`crates/capi/src/runtime/control.rs:46-53`);
- `source.frame.noncontiguous` until the host seeks;
- a hard step to the new value.

After this slice the change lands on the same plan, with the ramp length #1053 gives (a step
until #1054), and it is never lost. A mute on a strip that a `follows_mute` send follows is still a plan
replacement here; *Let C ABI sends follow their source strip's mute live* makes it live.

## Context (verified on `fe8ac679`; re-verify after #1257 lands)

- **Today every C ABI transaction is structural.** `command()` (`crates/capi/src/runtime/control.rs:694`)
  runs its `Structural` arm (`:718-857`) in order:
  1. `plan_alive` (`:727`);
  2. epoch-lag backpressure;
  3. the response-size check;
  4. `prepare_runtime` (`:749`), which prepares with **no** live controls
     (`crates/capi/src/runtime/compile.rs:408-410`, `prepare_host_runtime(compiled, &caps)`);
  5. `compiled_model_admission` (`:777`);
  6. reserve, then commit.
- **#1053** (`.github/ISSUE_SPECS/1053-deliver-value-only-fader-mute-and-pan-transactions-to-the-running-c-abi-plan-thr.md`,
  an umbrella since 2026-10-04) is the machinery this slice extends. It must not be forked. Its
  decision record says:
  - **D1:** host-core's `classify_live_delta(current, next, ramps)` (#1255, behind the
    `control-provider` feature, `crates/host-core/Cargo.toml:15`; capi enables it,
    `crates/capi/Cargo.toml:19`) masks each track's `fader` and `matrix_or_pan`, compares the
    canonical JSON bytes, and returns a `LiveDelta` of per-strip records. Strips are addressed by
    ID, never by index, and every value passes the setter's own domain checker.
  - **D5:** the C ABI prepares through host-core's `HostLiveLanes` selection (#1254, #1256):
    the fader and matrix lanes of every strip, submixes included; no route lanes.
  - **D6:** `commit_live` runs admission, resolve, the room check on every queue, the protocol
    token check, then the pushes, then the commit, which cannot fail after the check.
  - **D3:** fader and mute records take their lengths from `host_core::LiveRamps::for_session`,
    0 (a step) until #1054; a pan or matrix record carries the model's `smoothing_samples`.
  - **D7:** records go to the newest epoch, the pending candidate if any.
  - **Gates:** #1257's live-PCM gate (1 and 10 tracks, the four launch rates, compared from
    `latency_samples` plus one quantum after the edit) and #1258's two-thread race in
    `crates/capi/tests/resource_lifecycle.rs`.
- **What the wire carries for ramps.** A route edit carries no smoothing: route keys are `id`,
  `source`, `destination`, `channel_matrix`, `gain_db` (`crates/session/src/visit.rs:106`), plus
  `mute` and `follows_mute` after slices 18b and 20. A route `channel_matrix` has no smoothing field
  (`crates/session/src/visit.rs:109`). A strip fader (`DualMonoFader`, `crates/session/src/model.rs:642-651`)
  carries none either; a strip `pan` or `matrix` carries `smoothing_samples` (`visit.rs:103`).
- **The P13 guard** (`DESIGN.md` P13). #1053 carries it as guards G1-G3 in `classify_live_delta`
  (#1255). A delta that changes `left_mute` or `right_mute` of a strip that, in the post-commit
  model, is the source of a `follows_mute` route is **structural** (G2), and so is any delta to a
  **submix strip's** fields (G1: only track fields are masked). This slice lifts G1 and marks it
  superseded in #1053's spec. G2 stays until *Let C ABI sends follow their source strip's mute
  live*.
- **After batch K3:**
  - host-core attaches one `RouteControlProducer` per route into a submix, in canonical route-ID
    order, whenever live controls are requested (`HostLiveControlHandles.route_controls`, *Produce
    live send records from host-core*, #1221). Routes into the output have none.
  - `RouteControlProducer::{free, record, push, set}` exist. `record(gain_db, matrix, mute,
    source_lane_muted, length)` is pure and refuses `Domain` or `Length`; `push` refuses `Full` with
    nothing pushed. The target comes from `graph_compiler::route_coefficients`, the single domain and
    coefficient authority.
  - Strip control producers exist for every strip, submixes included, in `strip_controls`, parallel
    to `HostLiveControlHandles.strips` (tracks first) (*Give every strip one mute owner and
    live-control producers in host-core*, #1211).
  - A route into the output never follows (`follows_mute: true` there is refused at validation,
    *Let a route into a submix follow its source strip's mute in the session*).
- **Resources.** `capi_resources` (`crates/capi/src/runtime/compile.rs:109-213`) computes the retained
  bytes; `capi_retained_bytes` is a `PlanResourceReport` field (`crates/capi/src/abi.rs:286`, set at
  `compile.rs:451`). The oracles are `capi_retained_bytes_charge_every_byte_the_compile_retains`
  (`crates/capi/tests/resource_lifecycle.rs:718`) and
  `double_live_oracle_drives_exact_and_one_below_c_caps` (`:1251`).

## Decisions frozen for this slice

- **D1. Classification** (extends #1053 D1). A committed-model delta is live when it touches only:
  - `fader` and `pan`/`matrix` of any **strip** (track or submix), addressed by ID through
    `strips`;
  - `gain_db`, `mute` and `channel_matrix` of a route whose destination is a **submix**, addressed
    by route ID through `route_controls`.

  Everything else is structural:
  - a route's `follows_mute`, source, tap or destination;
  - the route set;
  - **any** change to a route into the output;
  - a change to `left_mute` or `right_mute` of a strip that, in the post-commit model, is the
    source of a `follows_mute` route (the P13 follow guard, kept until *Let C ABI sends follow their
    source strip's mute live*);
  - **any** delta while the pre- or post-commit model declares at least one VCA (the P13 VCA guard,
    kept until *Deliver value-only VCA edits to the running C ABI plan*, #1247; added at filing of
    *VCA groups*, #1239): D3's `source_lane_muted` reads raw committed mutes, which would reopen a
    VCA-muted member's following send.
- **D2. Ramp lengths.** Every live record, strip and route alike, uses the ramp length #1053 gives
  its fader and mute records (its D3, `LiveRamps::for_session`). A per-change smoothing is
  used only where the wire carries one: a strip `pan` or `matrix` uses its own `smoothing_samples`,
  as #1053 does. A route gain or matrix record uses `LiveRamps::for_session(next).fader_samples`;
  a route mute record uses `mute_samples`. Every length is checked against
  `ROUTE_RAMP_LENGTH_MAXIMUM` by `RouteControlProducer::record`. While `LiveRamps::for_session`
  returns 0 (until #1054), route records step too; say so in the PR.
- **D3. Route values.** A route record carries the post-commit model's `gain_db`, `channel_matrix`
  and `mute`, and `source_lane_muted = [false; 2]` for a route without `follows_mute`. For a route
  **with** `follows_mute`, `source_lane_muted` is its source strip's committed
  `[left_mute, right_mute]`. That value cannot change in a live delta here, because D1 keeps every
  follow-source mute change structural.
- **D4. Commit order** is #1053's D6, across every destination:
  1. every domain check (`RouteControlProducer::record`, the strip setters' shared checkers, the
     length bound), with nothing pushed;
  2. the live admission (`validate_live_peak`, #1053 D8);
  3. the room check on **every** queue (strip queues and route queues);
  4. `check_prepared_structural` on the protocol token;
  5. the pushes;
  6. the commit, which cannot fail after step 4.

  A full queue is typed `Backpressure`, with the model, revision and replay unchanged and nothing
  pushed.
- **D5. Resources.** The route producer table, its `Box<str>` IDs and the route queues are charged
  in `capi_resources`, and the two `resource_lifecycle` oracles change by exactly those rows.
- **D6. Live controls in capi.** capi prepares through #1053's `HostLiveLanes` selection (#1254,
  #1256), and this slice sets `routes: true` in it. The input and effect lanes stay as #1053's
  later slices left them (#1261, #1263). No new symbol, opcode or field is added.

## Deliverables

1. D1 in host-core's `classify_live_delta`, widened from tracks to strips and to routes into
   submixes.
2. D2-D4 in capi's live commit path: strip records through `strip_controls`, route records through
   `route_controls`.
3. D5 and D6.
4. Docs:
   - `docs/C_ABI_V1_QUALIFICATION.md`: which edits are value-only; their timing (#1053 D2, plus the
     route's compensation delay); that a change to a route into the output, to `follows_mute`, or to
     the mute of a follow source is structural.
   - `docs/CONTROL_PROTOCOL_SEMANTICS.md`: the delivery-status paragraph (`:15`).

## Authorized paths

- `crates/capi/src/runtime/{control.rs,compile.rs}`, and any capi runtime module #1053 added for its
  live commit path
- `crates/capi/tests/resource_lifecycle.rs` and one new capi test file
- `crates/host-core/src/` (the `classify_live_delta` classifier and the `HostLiveLanes` selection
  #1053 added; nothing else) and `crates/host-core/tests/` (one new file, if the classifier is tested
  there)
- `docs/C_ABI_V1_QUALIFICATION.md`, `docs/CONTROL_PROTOCOL_SEMANTICS.md`
- `.github/ISSUE_SPECS/1053-*.md` (guard G1's bullet only, marked superseded), if the spec is still in
  the directory
- this spec

## Non-goals

- No follow-mute composition: a follow-source mute stays structural (*Let C ABI sends follow their
  source strip's mute live*).
- No new C symbol, struct layout change or opcode.
- No live output routes (deferred item O9).
- No effect parameters or input section on a submix strip; on tracks those are #1053's later
  slices (#1261-#1266).
- No VCA (*Deliver value-only VCA edits to the running C ABI plan*).
- No `controlSmoothing` dependency beyond D2's rule.

## Hazards

- **#1053's L0 window.** A live edit during a plan swap must land in the candidate, never the
  retiring plan.
- **Acked records at a plan swap** (#1221 verdict, "The acked-batch question across a plan's
  life"). A route producer belongs to the plan it was prepared with, and a record still queued when
  that plan retires is discarded with it. That is correct only because the committed model already
  holds the acked value, so the replacement prepares it as a constant: the edit survives as a step
  at the swap boundary, its ramp lost. The producers must be swapped with the plan atomically at
  the L0 boundary.
- **No cross-route fence.** Records to different routes (or a strip and a route) in one transaction
  sit in different queues, and each drains at its own consumer's op, so they can apply up to one
  block apart. This is the same skew #1053 accepts for strip records; accept at most one quantum
  and say so in the docs, or add a fence.
- **Replay.** An exact replay pushes nothing, and queue room is unchanged.
- **Delayed sends.** A send with a compensation delay keeps running while muted (`DESIGN.md` P4), so
  its settled comparison point is `latency_samples` plus the ramp plus its compensation delay.
  Compare from there, not earlier.
- **Addressing.** Strips are addressed by ID through `strips`, routes by ID through
  `route_controls`; never by index into the model's vectors (#1053 D1).
- **A divergent model.** A delta classified live that some prepared value also depends on renders
  something other than the committed model. The follow guard (D1) is exactly that case; do not drop
  it here.

## Objective gates

1. **PCM through the C ABI.** New capi test file. Through the exported entry points
   (`miso_engine_v1_submit_command` with `SESSION_TRANSACTION_APPLY`), apply each of these to a
   session with one bus `b`, a send into `b` and a route into the output:
   - `0505` (route gain), `0506` (route mute) and `0504` (route matrix) on the send;
   - `020f` (strip fader, `SetTrackFader`) and `0210` (strip pan) on the submix `b`.

   Each changes the **same** plan: no new epoch, no pending provider, no re-seek and no silent
   block. After `latency_samples`, plus the ramp `LiveRamps::for_session` gives, plus the edited route's compensation delay
   where it has one, the output is bit-identical to a plan compiled from the committed model and fed
   the same sources from sample 0. The destination strip has no stateful insert, an identity input
   section and an empty console, so the comparison is exact. Run with 1 and 10 tracks, at each of the
   four launch rates (44.1, 48, 88.2 and 96 kHz), and once with a send that carries a 486-sample
   compensation delay (a true-peak limiter insert on a sibling contributor at 48 kHz).
   *Test value: it turns red if a send or bus edit is still classified structural, if a live route
   target differs from the prepared one, or if a record lands on the wrong strip or route.*
2. **The boundary of liveness.** Each of these produces a new epoch (structural):
   - `0505` on a route into the **output**;
   - `0507` (`follows_mute`) on a send;
   - `020f` muting a track that is the source of a `follows_mute` send (the P13 follow guard);
     after the boundary the output is bit-identical to a plan compiled from the committed model.

   *Test value: it turns red if the classifier treats an output route, a structural flag or a
   follow-source mute as live, which would write to a producer that does not exist or render other
   than the committed model.*
3. **No ack before a drop.**
   - A transaction that would overfill one route queue returns `Backpressure`; the model, revision
     and replay are unchanged, and no strip record was pushed (the strip queue's `free()` is
     unchanged).
   - An exact replay pushes nothing.
   - A live edit while a candidate is pending lands in the candidate.

   - An edit acked, then a structural swap before the render thread drains it: the new plan renders
     the edited value from the swap boundary (a step), and no later block renders the pre-edit
     value (#1221 verdict).

   *Test value: it turns red if a route record is pushed before every queue's room is checked, if
   the commit can still fail after a push, or if an acked edit is lost at a plan swap.*
4. **Realtime.** Extend the two-thread race of *Qualify live C ABI edits against a concurrently
   rendering plan* (#1258, in `crates/capi/tests/resource_lifecycle.rs`) to race send and bus edits against render and a
   structural swap, over 20 runs: `allocations == 0` and `frees == 0` around every render call,
   measured with the file's allocator counters after warm-up; zero `INTERNAL` results; and the final
   block bit-identical to a fresh plan of the final committed model.
   *Test value: it turns red if the route drain or the live commit allocates or frees on the render
   thread, or if a send edit racing a swap reaches the retiring plan.*
5. **Unchanged behaviour.**
   - Every existing capi test passes.
   - `bash scripts/check-capi-abi.sh` and `bash scripts/check-capi-abi.sh --self-test` pass.
   - `cargo build --locked --release -p audit -p bench -p capi -p session-validator`, then
     `./target/release/audit capi`, reports zero allocations, locks and syscalls.
   - The `resource_lifecycle` oracles change only by the D5 rows.
6. **4-lane.** `bash scripts/run-aarch64-tests.sh debug` on an arm64 host (capi and host-core are in
   its crate list), or CI's `aarch64-debug` job at the C1 push.
7. **Workspace and policy.**
   - the workspace test command (`DESIGN.md` section 7)
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
   - `bash scripts/check-realtime-policy.sh` and `bash scripts/test-realtime-policy.sh`
   - `bash scripts/check-host-core-policy.sh` and `bash scripts/test-host-core-policy.sh`

## Evidence

- The output of every gate command above, from the PR's head commit.
- Each new test's name with its one-sentence test-value answer.
- The ramp length `LiveRamps::for_session` gives at the head commit.
- The `resource_lifecycle` row changes, each with its reason.

## Dependencies

- *Apply value-only track fader, mute and pan transactions to the running C ABI plan* (#1257), the
  core of the umbrella *Deliver value-only fader, mute and pan transactions to the running C ABI
  plan through the live console lanes* (#1053)
- *Qualify live C ABI edits against a concurrently rendering plan* (#1258)
- *Let a send follow its source strip's mute live in the browser* (#1224, batch K3 closed and pushed)

## Standing rules for the implementer

- Work from this body and the merged code of #1053's slices. Extend its classifier and commit path;
  do not fork them.
- The commit after the first push is infallible by construction. No ack precedes a drop.
- Never emit a redundant record: a delta whose values equal the committed ones pushes nothing.
- "Bit-identical" gates are hard stops. NaNs are folded (decision 10).
- A test that greps source or prose is refused. A superseded test is deleted in the same PR.
- Commit on the C1 batch branch; do not push until the root closes the batch.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).
