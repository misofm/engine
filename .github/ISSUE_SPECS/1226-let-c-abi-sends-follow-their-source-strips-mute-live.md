# Let C ABI sends follow their source strip's mute live

Slice 28 of *Submix strips and live aux sends* (#1196). Batch C1, after *Deliver value-only send and
submix-strip edits to the running C ABI plan* (#1225); the root pushes C1 once after this slice's verdict.
**Re-verify every anchor after #1053 and slice 27 land.**

The design record cited below (`DESIGN`, `VERIFY-1` to `VERIFY-3`, `REVISION-1`, `REVISION-2` and
`APPLIED-3`) is committed in `docs/handoffs/submix-sends-2026-10-02/`.

## Product outcome

On a fan's phone, or any C ABI host, muting or unmuting a track or a bus is value-only even when a
send follows that strip's mute. The strip's mute and every `follows_mute` send from it move together,
through the same declicked ramp, in one all-or-nothing commit, with no plan rebuild.

Until this slice such a mute is a plan replacement (the P13 guard of `DESIGN.md`): a silent block, a
source-ring reset and a re-seek, because #1053's live path would otherwise push the strip's mute and
leave the follow-muted send's prepared coefficients behind, rendering something other than the
committed model.

## Context (re-verify after #1053 and slice 27 land)

- **After *Deliver value-only send and submix-strip edits to the running C ABI plan*:**
  - capi classifies a committed-model delta that touches only strip faders and pans (tracks and
    submixes) and the gain, mute and matrix of routes into submixes as live, through host-core's
    `live_builtin_delta` (#1053, behind the `control-provider` feature, `crates/host-core/Cargo.toml:15`);
  - it checks every domain, then room in every queue, then pushes, then commits infallibly (#1053
    A1.4, `.github/ISSUE_SPECS/1053-*.md:131-134`);
  - every live record uses the ramp length #1053 ruled (its A2 D3, `:155-161`);
  - the **P13 follow guard** is still in `live_builtin_delta`: a delta that changes `left_mute` or
    `right_mute` of a strip that, in the post-commit model, is the source of a route with
    `follows_mute: true` is structural.
- **The follow composition already exists in host-core** (*Let a send follow its source strip's mute
  live in the browser*, #1224):

  ```rust
  impl LiveRouteMuteFollow {
      pub fn delta<'a>(routes: &'a LiveRouteState,
                       effective_mute: &'a dyn Fn(usize /* strip */, usize /* lane */) -> bool)
          -> impl Iterator<Item = (usize, [bool; 2])> + 'a;
  }
  ```

  It yields `(live-route index, new source_lane_muted)` only for a `follows_mute` route whose
  `source_lane_muted` changes, never a redundant one. It takes the effective mute as a function so
  the C ABI can supply the committed model's mutes without host-web's solo state.
- **`LiveRouteState`** (*Admit live send commands in the browser*, #1222) mirrors each live route's
  `{gain_db, matrix, mute, follows_mute, source_lane_muted}` and its source strip index, with shadow,
  commit and rollback.
- **The C ABI has no solo.** A strip's effective mute on the C ABI is its committed
  `[left_mute, right_mute]`.
- **Follow mute is a send's property.** A route into the output never follows: `follows_mute: true`
  on it is refused at validation (*Let a route into a submix follow its source strip's mute in the
  session*, #1218), so every follow record targets a live route.
- **The "never a redundant record" rule** (`crates/host-core/src/solo.rs:32-45`): re-entering the ramp
  kernel on a settled lane can turn an exact `+0.0` into `-0.0`; it is digest-visible.
- **#1053's spec carries the P13 note** that slice 00 appended to its A2 D1 "Add:" bullet
  (`.github/ISSUE_SPECS/1053-*.md:145-147`), if the spec is still in `.github/ISSUE_SPECS/`.

## Decisions frozen for this slice

- **D1. Follow records.** A live delta that changes a strip's `left_mute` or `right_mute` also
  composes its follow records: capi keeps one `LiveRouteState` per live plan (seeded from the
  committed model at preparation), updates its shadow with the post-commit `gain_db`, `matrix` and
  `mute` of every live route, and calls `LiveRouteMuteFollow::delta` with
  `effective_mute(strip, lane) = ` the post-commit model's mute of that strip lane. For each yielded
  `(route, source_lane_muted)` it builds the record with `RouteControlProducer::record` (the route's
  current `gain_db`, `matrix` and `mute`, the new `source_lane_muted`, the ruled ramp length).
- **D2. One commit.** Strip mute records, route value records and follow records are domain-checked
  together, room-checked together across every queue, pushed, then committed infallibly, in #1053's
  order. A full queue is typed `Backpressure`: model, revision, replay, queues and the route mirror
  unchanged.
- **D3. Ramps.** A follow record uses the same ruled ramp length as the strip mute record it follows.
- **D4. The guard goes.** Remove the P13 follow guard from `live_builtin_delta`: a mute on a follow
  source is now live. The submix-strip and VCA parts of P13 are not this slice's (slice 27 lifted the
  first; the VCA umbrella owns the second).
- **D5. #1053's spec note.** Mark the follow-source bullet of the P13 note in #1053's spec as
  superseded by this slice, if the spec is still in `.github/ISSUE_SPECS/`. The VCA bullet stays
  until *Deliver value-only VCA edits to the running C ABI plan* (#1247), and this slice keeps the
  VCA guard in `live_builtin_delta` (amended at filing of *VCA groups*, #1239).
- **D6. Resources.** The `LiveRouteState` mirror and its shadow are charged in `capi_resources`
  (`crates/capi/src/runtime/compile.rs:109-213`), and the `resource_lifecycle` oracles change by
  exactly those rows.

## Deliverables

- D1-D3 in capi's live commit path.
- D4 in host-core's `live_builtin_delta`.
- D5 and D6.
- `docs/C_ABI_V1_QUALIFICATION.md`: a mute on a strip that sends follow is value-only, and its sends
  follow in the same commit.

## Authorized paths

- `crates/capi/src/runtime/{control.rs,compile.rs}`, and any capi runtime module #1053 or slice 27
  added for the live commit path
- `crates/capi/tests/resource_lifecycle.rs` and the capi test file slice 27 added
- `crates/host-core/src/` (`live_builtin_delta` only; `LiveRouteMuteFollow` and `LiveRouteState` are
  reused, not changed) and `crates/host-core/tests/`
- `docs/C_ABI_V1_QUALIFICATION.md`
- `.github/ISSUE_SPECS/1053-*.md` (the P13 note only), if the spec is still in the directory
- this spec

## Non-goals

- No solo on the C ABI.
- No change to `LiveRouteMuteFollow` or to the browser path.
- No live output routes, and no follow on them (they never follow).
- No VCA mute composition (*Deliver value-only VCA edits to the running C ABI plan*).
- No new symbol, opcode, field or struct layout change.

## Hazards

- **A redundant record moves bits.** A strip mute that changes no follow route's
  `source_lane_muted`, or a strip that is the source of no `follows_mute` route, pushes no route
  record.
- **Ordering inside one transaction.** A transaction that both mutes a source and edits one of its
  sends must compose from the post-commit values of both, so the route record carries the edited gain
  and the new `source_lane_muted` together.
- **Delayed sends.** A follow-muted delayed send stays active and mixes zeros (`DESIGN.md` P4); its
  settled comparison point includes its compensation delay.
- **#1053's L0 window.** A live edit during a plan swap lands in the candidate, never the retiring
  plan; the candidate's `LiveRouteState` is the one updated.

## Objective gates

1. **PCM through the C ABI.** In the capi test file slice 27 added: a session where track `t` sends
   `pre_fader` with `follows_mute: true` into bus `b`, and `b` routes to the output. Through the
   exported entry points, `020f` mutes `t`'s left lane, then both lanes, then unmutes it.
   - Each edit changes the **same** plan: no new epoch, no re-seek, no silent block.
   - After `latency_samples` plus the ruled ramp (plus the send's compensation delay where it has
     one), the output is bit-identical to a plan compiled from the committed model and fed the same
     sources from sample 0.
   - `t` and a second track feed distinct, non-constant signals on each lane, and the send's matrix
     is asymmetric, so a wrong column or lane differs.
   - Run with 1 and 10 tracks at each of the four launch rates (44.1, 48, 88.2 and 96 kHz).

   *Test value: it turns red if the follow records are not pushed with the strip mute, zero the wrong
   column, or carry a stale gain, or if the follow-source mute is still classified structural.*
2. **No redundant record.**
   - Muting a track that is the source of no `follows_mute` route pushes no route record (each
     route queue's `free()` is unchanged).
   - Re-muting an already muted source pushes none.
   - A mute that flips only the right lane pushes one record, with only the right column zeroed.

   *Test value: it turns red if the composition re-emits unchanged targets, which re-enters the ramp
   kernel and moves a settled lane's zero sign.*
3. **All or nothing.** A transaction whose follow record would overfill its route queue returns
   `Backpressure`; no strip record and no route record is pushed, and the model, revision, replay
   and route mirror are unchanged.
   *Test value: it turns red if the strip mute is pushed or committed while its follow record is
   refused.*
4. **Realtime.** The two-thread barrier test in `crates/capi/tests/resource_lifecycle.rs` (#1053's
   gate 4 shape) races follow-source mutes against render and a structural swap over 20 runs:
   `allocations == 0` and `frees == 0` around every render call after warm-up; zero `INTERNAL`
   results; the final block bit-identical to a fresh plan of the final committed model.
   *Test value: it turns red if the follow composition allocates on the render thread or a racing
   mute lands in the retiring plan.*
5. **Unchanged behaviour.**
   - Every existing capi test passes, including slice 27's, except its gate-2 case that expected a
     follow-source mute to be structural, which this slice inverts in the same PR.
   - `bash scripts/check-capi-abi.sh` and `bash scripts/check-capi-abi.sh --self-test` pass.
   - `cargo build --locked --release -p audit -p bench -p capi -p session-validator`, then
     `./target/release/audit capi`, reports zero allocations, locks and syscalls.
   - The `resource_lifecycle` oracles change only by the D6 rows.
6. **4-lane.** `bash scripts/run-aarch64-tests.sh debug` on an arm64 host, or CI's `aarch64-debug`
   job at the C1 push.
7. **Workspace and policy.**
   - the workspace test command (`DESIGN.md` section 7)
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
   - `bash scripts/check-realtime-policy.sh` and `bash scripts/test-realtime-policy.sh`
   - `bash scripts/check-host-core-policy.sh` and `bash scripts/test-host-core-policy.sh`

## Evidence

- The output of every gate command above, from the PR's head commit.
- Each new test's name with its one-sentence test-value answer.
- The `resource_lifecycle` row changes, each with its reason.
- The diff of #1053's spec note, if the spec was still in the directory.

## Dependencies

- *Deliver value-only send and submix-strip edits to the running C ABI plan* (#1225)

## Standing rules for the implementer

- Work from this body and the merged code of #1053 and slice 27. Extend their paths; do not fork
  them.
- The commit after the first push is infallible by construction. No ack precedes a drop.
- Never emit a redundant record.
- "Bit-identical" gates are hard stops. NaNs are folded (decision 10).
- A test that greps source or prose is refused. A superseded test is deleted, or inverted, in the
  same PR.
- Commit on the C1 batch branch. The root pushes C1 once after this slice's verdict.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).
