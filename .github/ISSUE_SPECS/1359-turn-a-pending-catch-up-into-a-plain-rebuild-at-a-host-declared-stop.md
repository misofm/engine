# Turn a pending catch-up into a plain rebuild at a host-declared stop

Stream C of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-8, D15-17).
Slice of *Pre-roll a successor whose latency grows* (#1287), W8 last sentence. Code anchors verified
on `main` at `6fb211594`.

## Product outcome

When a host stops (or is about to seek every source) and says so, any pending latency-growth
catch-up ends at once.
- The edit is applied at the next render as a plain rebuild with no floors and no read-ahead.
- Nothing is ducked or pre-rolled, because no continuity is owed after a stop.
- The revision completes when that plan renders.

## Context

- *Reset latency floors at a host-declared discontinuity* (#1323) adds
  `miso_engine_v1_declare_discontinuity` (D1). If any floor is raised, it prepares the committed
  model with no floors, carrying only the sources (D2-D3). It supersedes a pending candidate
  through #1310 (D2).
- *Give a plan a source-read clock that leads its render clock* (#1396, split from #1355)
  sets the discontinuity successor's source-read offset to 0 (its D2), so `ΣP` resets there, and
  defines a stop without a seek (its D3): no frame is repeated or skipped.
- A catch-up's successor is `Returned`, control-owned, published-unclaimed, or adopted (#1357
  Context), or render is copying into it (`InFlight`, #1357 D1). It
  holds armed peeks on persisting sources (#1320) and held edits in its own cells (#1356).

## Decisions frozen for this slice

- **D1. A declaration ends the catch-up.** When #1323's declaration runs with a catch-up pending:
  1. It takes the successor back as #1357 D1 does.
  2. #1323's preparation takes the displaced successor as donor (#1310 D2, *Prepare a successor
     across a withdrawn candidate plan*, #1344 D3): the rings of sources the edit added are reused,
     never dropped or allocated again. They pass #1344 D5's unconsumed check although the
     catch-up rendered blocks, because a warm successor's added entries stay closed until adoption
     and never begin their consumers (#1355 D2). If render is
     copying into the successor (`InFlight`), the check runs against its `DonorRecord` and the
     donation is split exactly as #1357 D1 and D1a do. The peeks stay untouched with the displaced
     successor while preparation can fail.
  3. After every fallible check of #1323 D4, it drops the displaced successor on the control
     thread, which ends its peeks on persisting sources (#1320 D3) and drops the held edits in its
     cells (#1356 D4).
  4. It publishes #1323's discontinuity successor for the next block.

  In the `InFlight` case, dropping the displaced plan, the consumer half of the donation and the
  publication wait for the service step that completes the withdrawal (#1357 D1a); the declaration
  itself returns at once.

  This happens even when the predecessor has no raised floor, because the successor's lead counts
  as raised.
- **D2. Completion.** The abandoned revision is in the committed model the discontinuity successor
  is built from. It completes at that successor's adoption with `EXACT`: no continuity is owed
  after a declared stop, so nothing fell back.
- **D3. Refusal.** If #1323's preparation fails, the declaration returns its error and the catch-up
  continues untouched.
- **D4. Acked-batch question.** The revision's content is carried by the committed model, and a
  refused declaration changes nothing. An ack can never precede a drop.

## Deliverables

1. D1-D3 in the control plane and `crates/host-core/src/catch_up.rs`.
2. Gates in the control-plane tests.

## Authorized paths

- `crates/control-plane/src/`, `crates/host-core/src/catch_up.rs`
- The control-plane tests, `crates/host-core/tests/warm_successor.rs`

## Non-goals

- The declaration's ABI and its floor reset (#1323). The service wiring (#1360).

## Objective gates

1. **Stop during a catch-up.** In #1355 gate 1's setup, declare a discontinuity before
   publication.
   - The next rendered block is the discontinuity successor's: the added track is present, there
     are no floors, and the offset is 0.
   - The advance reports the revision with `EXACT`.
   - Every predecessor ring's admission depth is back to configured.
2. **Published but unclaimed.** The same after `ExactlyAt(S)` is published and before `S`.
3. **Refused.** A declaration whose preparation fails leaves the catch-up adopting at `S` as without
   it.
4. **Rings and peeks.** With an edit that also adds a source, the discontinuity successor holds the
   ring the displaced successor prepared. After the stop, a new growth edit takes every peek with
   `take_peek`.
5. Commands: those of #1356, plus `cargo test --locked -p capi`.

## Test value

- Gate 1: a declaration that ignores the pending catch-up adopts the warm successor after the stop,
  with read-ahead and floors that #1323 was meant to remove. Red.
- Gate 2: a withdraw that misses the published state leaves two candidates pending. Red.
- Gate 3: abandoning before the fallible checks loses the catch-up on a refusal. Red.
- Gate 4: a declaration that prepares a new ring for an added source loses the chunks already
  submitted; one that never ends the displaced peeks blocks every later growth. Red.

## Dependencies

- *Reset latency floors at a host-declared discontinuity* (#1323).
- *Supersede a running catch-up by a structural edit* (#1357).
- *Catch up a returned successor and adopt it exactly at a scheduled sample* (#1355).
- *Give a plan a source-read clock that leads its render clock* (#1396).
- *Supersede an unadopted candidate plan by compare-and-swap* (#1310), D2.
- *Prepare a successor across a withdrawn candidate plan* (#1344), D3 and D5.
