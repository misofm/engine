# Deliver value-only send and submix-strip edits to the running C ABI plan

Slice 27 of *Submix strips and live aux sends* (#1196). It is in batch C1, with *Let C ABI sends follow their
source strip's mute live* (#1226). It starts only after #1053 has closed and after batch K3 (*Let a send
follow its source strip's mute live in the browser*, #1224) is on `main`. **Re-verify every anchor after
#1053 lands:** #1053 adds the classifier and the live commit path this slice extends.

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

After this slice the change lands on the same plan as a declicked ramp of the length #1053 ruled,
and it is never lost. A mute on a strip that a `follows_mute` send follows is still a plan
replacement here; *Let C ABI sends follow their source strip's mute live* makes it live.

## Context (verified on `fe8ac679`; re-verify after #1053 lands)

- **Today every C ABI transaction is structural.** `command()` (`crates/capi/src/runtime/control.rs:694`)
  runs its `Structural` arm (`:718-857`) in order:
  1. `plan_alive` (`:727`);
  2. epoch-lag backpressure;
  3. the response-size check;
  4. `prepare_runtime` (`:749`), which prepares with **no** live controls
     (`crates/capi/src/runtime/compile.rs:408-410`, `prepare_host_runtime(compiled, &caps)`);
  5. `compiled_model_admission` (`:777`);
  6. reserve, then commit.
- **#1053** (`.github/ISSUE_SPECS/1053-deliver-value-only-fader-mute-and-pan-transactions-to-the-running-c-abi-plan-thr.md`)
  is the machinery this slice extends. It must not be forked.
  - **Scope 2 and 3 (`:22-26`):** capi prepares with live controls, keeps `track_controls` in
    `ProviderEpoch`, and adds host-core's `live_builtin_delta` behind the `control-provider` feature
    (`crates/host-core/Cargo.toml:15`; capi enables it, `crates/capi/Cargo.toml:19`).
  - **D1 (`:38-40`):** a committed-model delta is live when `fader` and `matrix_or_pan` are the only
    fields that differ and every value passes the render-side setter's domain. Edit opcodes are not
    inspected.
  - **A1.2 (`:122-124`), open when this was written:** the same `control_queue_depth` also attaches
    every effect lane. #1053 either keeps those producers or adds a **builtins-only** live-control
    request to host-core. Read which one landed.
  - **A1.3 (`:126-130`):** address tracks by ID, never by index; call the same domain checkers that
    preparation and the render-side setters use.
  - **A1.4 (`:131-134`):** admission and the room check, then push, then an infallible commit.
  - **A2 D3 (`:155-161`):** a ramp, not a step. A step on a mute is a click (out-of-band energy
    -31 dB against -70 dB for a one-quantum ramp). Suggested: a fixed 5-10 ms derived from the session
    rate at preparation, at least one quantum. Read the length #1053 actually ruled and implemented.
  - **Gate 1 (`:51-55`, corrected by A3 `:167-170`):** four launch rates, 1 and 10 tracks, compared
    `latency_samples` plus one quantum after the edit.
  - **Gate 4 (A3 `:181-186`):** a two-thread barrier test in `crates/capi/tests/resource_lifecycle.rs`
    that races live and structural edits against render, counting allocations around each render
    call only.
- **What the wire carries for ramps.** A route edit carries no smoothing: route keys are `id`,
  `source`, `destination`, `channel_matrix`, `gain_db` (`crates/session/src/visit.rs:106`), plus
  `mute` and `follows_mute` after slices 18b and 20. A route `channel_matrix` has no smoothing field
  (`crates/session/src/visit.rs:109`). A strip fader (`DualMonoFader`, `crates/session/src/model.rs:537-546`)
  carries none either; a strip `pan` or `matrix` carries `smoothing_samples` (`visit.rs:103`).
- **The P13 guard** (`DESIGN.md` P13). Slice 00 annotated #1053's spec, and *Declare the submix
  strip in the session grammar and wire* (#1199, submix-strip fields) and *Let a route into a submix
  follow its source strip's mute in the session* (#1218, follow sources) implemented the guard in
  `live_builtin_delta` if #1053 had already landed; otherwise #1053 carries it. Either way a delta
  that changes `left_mute` or `right_mute` of a strip that, in the post-commit model, is the source of
  a `follows_mute` route is **structural**, and so is any delta to a **submix strip's** fields.
  This slice lifts the submix-strip part. The follow-source part stays until *Let C ABI sends follow
  their source strip's mute live*.
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
    source strip's mute live*).
- **D2. Ramp lengths.** Every live record, strip and route alike, uses the fixed ramp length #1053
  ruled for its fader and mute records (its A2 D3), derived at preparation. A per-change smoothing is
  used only where the wire carries one: a strip `pan` or `matrix` uses its own `smoothing_samples`,
  as #1053 does. Route records always use the ruled length. Every length is checked against
  `ROUTE_RAMP_LENGTH_MAXIMUM` by `RouteControlProducer::record`. If #1053 ruled a step (length 0),
  route records step too; say so in the PR.
- **D3. Route values.** A route record carries the post-commit model's `gain_db`, `channel_matrix`
  and `mute`, and `source_lane_muted = [false; 2]` for a route without `follows_mute`. For a route
  **with** `follows_mute`, `source_lane_muted` is its source strip's committed
  `[left_mute, right_mute]`. That value cannot change in a live delta here, because D1 keeps every
  follow-source mute change structural.
- **D4. Commit order** is #1053's, across every destination:
  1. every domain check (`RouteControlProducer::record`, the strip setters' shared checkers, the
     length bound), with nothing pushed;
  2. the room check on **every** queue (strip queues and route queues);
  3. the pushes;
  4. the infallible commit.

  A full queue is typed `Backpressure`, with the model, revision and replay unchanged and nothing
  pushed.
- **D5. Resources.** The route producer table, its `Box<str>` IDs and the route queues are charged
  in `capi_resources`, and the two `resource_lifecycle` oracles change by exactly those rows.
- **D6. Live controls in capi.** capi requests host-core live controls exactly as #1053 does; the
  route producers come with that request. If #1053 added a builtins-only live-control request to
  host-core (its A1.2), this slice extends that request so it also attaches route controls; effect
  lanes stay unattached. No new symbol, opcode or field is added.

## Deliverables

1. D1 in host-core's `live_builtin_delta` (or the classifier #1053 named), widened from tracks to
   strips and to routes into submixes.
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
- `crates/host-core/src/` (the `live_builtin_delta` classifier and the live-control request #1053
  added; nothing else) and `crates/host-core/tests/` (one new file, if the classifier is tested
  there)
- `docs/C_ABI_V1_QUALIFICATION.md`, `docs/CONTROL_PROTOCOL_SEMANTICS.md`
- this spec

## Non-goals

- No follow-mute composition: a follow-source mute stays structural (*Let C ABI sends follow their
  source strip's mute live*).
- No new C symbol, struct layout change or opcode.
- No live output routes (deferred item O9).
- No effect parameters through the C ABI (#1053's L2).
- No VCA (*Deliver value-only VCA edits to the running C ABI plan*).
- No `controlSmoothing` dependency beyond D2's rule.

## Hazards

- **#1053's L0 window.** A live edit during a plan swap must land in the candidate, never the
  retiring plan.
- **Replay.** An exact replay pushes nothing, and queue room is unchanged.
- **Delayed sends.** A send with a compensation delay keeps running while muted (`DESIGN.md` P4), so
  its settled comparison point is `latency_samples` plus the ramp plus its compensation delay.
  Compare from there, not earlier.
- **Addressing.** Strips are addressed by ID through `strips`, routes by ID through
  `route_controls`; never by index into the model's vectors (#1053 A1.3).
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
   block. After `latency_samples`, plus the ruled ramp, plus the edited route's compensation delay
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

   *Test value: it turns red if a route record is pushed before every queue's room is checked, or
   if the commit can still fail after a push.*
4. **Realtime.** Extend the two-thread barrier test (#1053's gate 4 shape, in
   `crates/capi/tests/resource_lifecycle.rs`) to race send and bus edits against render and a
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
- The ramp length #1053 ruled, and which live-control request (full or builtins-only) it landed.
- The `resource_lifecycle` row changes, each with its reason.

## Dependencies

- *Deliver value-only fader, mute and pan transactions to the running C ABI plan through the live
  console lanes* (#1053)
- *Let a send follow its source strip's mute live in the browser* (#1224, batch K3 closed and pushed)

## Standing rules for the implementer

- Work from this body and #1053's merged code. Extend its delta and commit path; do not fork them.
- The commit after the first push is infallible by construction. No ack precedes a drop.
- Never emit a redundant record: a delta whose values equal the committed ones pushes nothing.
- "Bit-identical" gates are hard stops. NaNs are folded (decision 10).
- A test that greps source or prose is refused. A superseded test is deleted in the same PR.
- Commit on the C1 batch branch; do not push until the root closes the batch.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).
