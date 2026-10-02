# Deliver value-only VCA edits to the running C ABI plan

Slice V5 of *VCA groups*. It starts after *Enumerate VCA groups and drive them from the SDK* (the
VCA batch closed), *Let C ABI sends follow their source strip's mute live* and #1053 have closed. **Drafted; anchors re-verified at
filing:** every anchor below depends on those three landing.

## Product outcome

A fan's personal mix on a phone rides a VCA ("drums -3 dB") through the C ABI without a plan rebuild,
with exactly the browser's composition and bits. A member's own fader edit also becomes value-only,
because the VCA offset is now composed instead of being guarded off, and a VCA mute silences its
members' `follows_mute` sends in the same commit.

## Context (re-verify after the dependencies land)

- **After #1053, *Deliver value-only send and submix-strip edits to the running C ABI plan* and *Let
  C ABI sends follow their source strip's mute live*:**
  - capi classifies a committed-model delta that touches only strip faders and pans (tracks and
    submixes), and send gain, mute and matrix, as live, through host-core's `live_builtin_delta`
    (behind the `control-provider` feature, `crates/host-core/Cargo.toml:15`);
  - it domain-checks, room-checks every queue, pushes, and commits infallibly (#1053 A1.4,
    `.github/ISSUE_SPECS/1053-*.md:131-134`);
  - every live record uses the ramp length #1053 ruled (its A2 D3, `:155-161`);
  - a strip mute change also pushes its follow records through `LiveRouteMuteFollow::delta`, whose
    `effective_mute` the C ABI supplies from the committed model, kept in a capi-owned
    `LiveRouteState`.
- **After *Apply VCA offsets and mutes at preparation*:**
  - `SessionModel::effective_strip_faders` computes each strip's effective fader, mute and VCA mute
    term once, and both compilers consume it;
  - #1053's classifier treats a fader delta of a VCA member, or of a VCA, as **structural** (the P13
    guard of `DESIGN.md`). This slice removes that guard.
- **After *Ride VCA groups live in the browser*:** host-core's `LiveVcaState` computes the member
  fader records for a VCA change, feeds `vca_mute` into the mute state, emits only changes, and
  supports shadow, commit and rollback.
- **The C ABI has no solo**, so a strip's effective mute there is
  `committed member_mute || vca_mute`.
- **Opcodes** (*Declare VCA groups in the session*): `0700` upsert VCA (it can change membership),
  `0701` remove VCA, `0702` set VCA fader.
- **Resources.** `capi_resources` (`crates/capi/src/runtime/compile.rs:109-213` at `fe8ac679`) and
  the `resource_lifecycle` oracles (`crates/capi/tests/resource_lifecycle.rs:718`, `:1251`).

## Decisions frozen for this slice

- **D1. Classification.** A delta is live when, besides the earlier slices' live fields, it changes
  only VCA `fader` values (`0702`, or a `0700` that keeps the membership).
  - Any membership change, and any VCA added or removed, is structural.
  - A member's own fader delta is now live, with its effective value composed.
- **D2. Composition.** A live VCA delta, or a member fader delta, feeds a capi-owned `LiveVcaState`
  (seeded at preparation from the committed model). Its member `FaderDb` records, the mute-state
  deltas and the follow-mute records (`LiveRouteMuteFollow::delta`, with
  `effective_mute = member_mute || vca_mute`) are domain-checked and room-checked together, then
  pushed, then committed, in #1053's order. A full queue is typed `Backpressure`, with model, revision,
  replay and every mirror unchanged.
- **D3. Ramps.** Every record uses #1053's ruled ramp length; a VCA change's ramp applies to every
  member record it emits.
- **D4.** Remove the P13 guard for VCA members and VCA faders from `live_builtin_delta`.
- **D5. Resources.** The VCA mirror's bytes are charged in `capi_resources`, and the oracles change by
  exactly those rows.

## Deliverables

- D1-D5 in capi's live commit path and host-core's classifier.
- `docs/C_ABI_V1_QUALIFICATION.md`: VCA fader edits and member fader edits are value-only; membership
  edits are structural.

## Authorized paths

- `crates/capi/src/runtime/{control.rs,compile.rs}`, and any capi runtime module #1053 or slices 27
  and 28 added for the live commit path
- `crates/capi/tests/` (`resource_lifecycle.rs` and one new test file)
- `crates/host-core/src/` (the classifier only; `LiveVcaState` and `LiveRouteMuteFollow` are reused,
  not changed) and its tests
- `docs/C_ABI_V1_QUALIFICATION.md`
- this spec

## Non-goals

- No new symbol, opcode or field.
- No VCA solo, no send trim and no automation.
- No change to the browser path.

## Hazards

- **#1053's L0 window.** A live VCA edit during a plan swap lands in the candidate.
- **Redundant records.** A VCA move that changes no effective value pushes nothing.
- **Reach drift.** The capi mirror's reach sets come from preparation; a membership change must stay
  structural, or the pushed records target a reach the plan no longer has.

## Objective gates

1. **PCM through the C ABI.** New capi test file. `0702` on a nested VCA (with a `follows_mute`
   pre-fader send from one member), a `0702` that mutes that VCA, and a member's own `020f` each render
   bit-identically to a plan compiled from the committed model after `latency_samples` plus the ramp
   (plus a send's compensation delay where it has one). No new epoch, no re-seek; the downstream is
   stateless. Run with 1 and 10 tracks at each of the four launch rates (44.1, 48, 88.2 and 96 kHz).
   *Test value: it turns red if a VCA or member fader edit stays structural, if capi's composition
   diverges from preparation's, or if a VCA mute leaves a member's follow send open.*
2. **Membership is structural.** `0700` changing members, and `0701`, each produce a new epoch.
   *Test value: it turns red if a membership change is pushed as values to a plan whose reach sets no
   longer match.*
3. **No ack before a drop.**
   - A VCA move that would overfill a member queue or a follow route queue is typed `Backpressure`,
     with model, revision, replay and every mirror unchanged.
   - An exact replay pushes nothing.
   - A VCA move that changes no effective value (every member clamped) pushes nothing.

   *Test value: it turns red if any record is pushed before every queue's room is checked, or if
   composition re-emits unchanged targets.*
4. **Realtime.** The two-thread barrier test (in `crates/capi/tests/resource_lifecycle.rs`, #1053's
   gate 4 shape) races VCA edits against render and a structural swap over 20 runs:
   `allocations == 0` and `frees == 0` around each render call after warm-up, zero `INTERNAL`
   results, and a final block bit-identical to a fresh plan of the final committed model.
   *Test value: it turns red if VCA composition allocates on the render thread or a racing VCA edit
   lands in the retiring plan.*
5. **Unchanged behaviour.**
   - Every existing capi test passes, except the V2 guard case (a member fader edit was structural),
     which this slice inverts in the same PR.
   - `bash scripts/check-capi-abi.sh` and `bash scripts/check-capi-abi.sh --self-test` pass.
   - `cargo build --locked --release -p audit -p bench -p capi -p session-validator`, then
     `./target/release/audit capi`, reports zero allocations, locks and syscalls.
   - The `resource_lifecycle` oracles change only by the D5 rows.
6. **4-lane.** `bash scripts/run-aarch64-tests.sh debug` on an arm64 host, or CI's `aarch64-debug` at
   the push.
7. **Workspace and policy.**
   - the workspace test command (`DESIGN.md` section 7)
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
   - `bash scripts/check-realtime-policy.sh`, `bash scripts/test-realtime-policy.sh`,
     `bash scripts/check-host-core-policy.sh`, `bash scripts/test-host-core-policy.sh`

## Evidence

- The output of every gate command above, from the PR's head commit.
- Each new test's name with its one-sentence test-value answer.
- The `resource_lifecycle` row changes, with reasons.

## Dependencies

- *Enumerate VCA groups and drive them from the SDK* (the VCA batch closed; it follows *Ride VCA
  groups live in the browser*, whose `LiveVcaState` this slice reuses)
- *Let C ABI sends follow their source strip's mute live*
- *Deliver value-only fader, mute and pan transactions to the running C ABI plan through the live
  console lanes* (#1053)

## Standing rules for the implementer

- Extend #1053's and slices 27-28's paths; do not fork them.
- No ack precedes a drop. The commit after the first push is infallible.
- Never emit a redundant record.
- "Bit-identical" gates are hard stops. NaNs are folded (decision 10).
- A test that greps source or prose is refused. A superseded test is deleted, or inverted, in the
  same PR.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).
