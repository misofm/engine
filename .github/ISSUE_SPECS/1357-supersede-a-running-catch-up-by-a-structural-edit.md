# Supersede a running catch-up by a structural edit

Stream C of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-8 step 5, D15-9, D15-17).
Slice of *Pre-roll a successor whose latency grows* (#1287), W5 second bullet. Code anchors verified
on `main` at `6fb211594`.

## Product outcome

A structural edit submitted while a warm successor is catching up is accepted at once. It never
gets `BACKPRESSURE` because a catch-up is pending. The catch-up restarts from a new block B with
the newer model, and the displaced revision completes as `superseded` inside the newer one. Audio
is never interrupted.

## Context

- *Supersede an unadopted candidate plan by compare-and-swap* (#1310) withdraws an unadopted
  candidate before preparing (D1). It prepares the newer one against the plan render runs, with the
  withdrawn one as a donor (D2), keeps the predecessor's model (D3), and republishes the old
  candidate on any refusal (D4).
- A catch-up's successor is in one of four places:
  - in its mailbox cell, `Returned` (#1311 D3, with #1354 D1's reasons): render copied into it, or
    refused the copy, or saw it late; it stays there until its publisher takes it with
    `withdraw()` (`Withdrawal::Returned`, #1311 D6);
  - control-owned: taken back and rendering forward (#1355);
  - published `ExactlyAt(S)` and unclaimed: withdrawable (#1311 D2);
  - adopted.
- Its predecessor's rings are armed (#1320 D4). Its held edits are in the committed model (#1356 D4).
- `SUPERSEDED` and `superseded_count` are #1314 D5's.
- Today a structural transaction is refused while a candidate is pending
  (`crates/capi/src/runtime/control.rs:959-961`). After #1309 this lives in `crates/control-plane`.

## Decisions frozen for this slice

- **D1. Take the catch-up back.**
  - If its successor is control-owned, the control plane holds it, which is #1310's *withdrawn*
    case.
  - If its cell is `Returned`, the control plane takes it with `withdraw()`
    (`Withdrawal::Returned { candidate, reason }`), which is also #1310's *withdrawn* case (#1311
    D6).
  - If render is copying into it right now (`Withdrawal::InFlight`, #1311 D6, #1354 D1), the
    structural submit never waits on render (decision 15, D15-17) and is never refused for it. It
    runs D2's preparation against P0 and the committed model as usual, and D3's commit. It
    records the in-flight successor as superseded, and holds the newer candidate control-side,
    unpublished: the mailbox has no free cell while the in-flight one is `Returned`. The steps of
    D3 that need the displaced successor in hand (2, 4 and 5) wait for it, as D1a says.
  - If it is published and unclaimed, the control plane withdraws it (#1310 D1).
  - If render took it, it is the base, as in #1310's *taken* case. The newer edit is then an
    ordinary rebuild, or a new catch-up if it grows latency again. The control call services
    first (*Add miso_engine_v1_service for bounded control work between edits*, #1348, D2), which
    reclaims the retired predecessor; its drop ends the peeks it holds (#1355 D6), so `take_peek`
    succeeds.
- **D1a. The service step completes the withdrawal.** The next `service` call (*Add
  miso_engine_v1_service for bounded control work between edits*, #1348) that finds the copy
  ended takes the displaced successor with `withdraw()` (`Withdrawal::Returned { reason: Copied |
  CopyRefused }`). A copy lasts at most the rest of one render callback (#1354 D1), so the first
  service call after that callback finds it ended; an earlier call gets `InFlight` again and leaves
  everything as it is. The service step then runs D3 steps 2, 4 and 5 for the newer candidate,
  which are infallible moves, drops the displaced plan, and publishes it. It never prepares
  anything again.
  - A further structural edit before that service call supersedes the held newer candidate, which
    is control-owned (the first case above); the in-flight successor stays recorded as superseded.
  - The displaced revision completes as `superseded` (D4).
- **D2. Prepare against P0.** In the withdrawn and in-flight cases, the newer candidate is
  prepared against the running plan P0 (#1310 D2). Its lead is `warm_lead` (#1354 D3) from P0,
  not from the displaced successor. If it grows latency it is a new warm successor; if no warm
  successor can be prepared (`WarmUnavailable`, #1354 D3, #1355 D7) it takes the transition;
  otherwise it is an ordinary candidate.
  - **Peeks pass by donation.** A warm newer candidate does not call `take_peek`: the displaced
    successor still owns the rings' peeks (#1320 D3). Preparation borrows them from the donor,
    untouched and still armed. On success they move to the newer candidate in D3. On refusal they
    stay with the displaced successor. In the in-flight case nothing is borrowed at preparation:
    the peeks and the displaced successor's new rings stay where they are until D1a's service
    step moves them, which is a move, never a check.
- **D3. Infallible order.** Only after every fallible check (#1310 D4):
  1. the protocol commit;
  2. `abandon` each borrowed peek (#1320 D8). A warm newer candidate keeps them: its claim's
     `arm_peek` completes the abandon and arms again at the new B (#1320 D4). An ordinary or
     transition candidate drops them, which ends them (#1320 D3);
  3. drop the hold (#1356 D4) and the displaced plan on the control thread;
  4. donate the displaced successor's new rings (#1310 D2);
  5. publish the newer candidate, `CopyAndReturn` when it is warm.

  On a refusal the displaced catch-up continues untouched, with its peeks still armed. In the
  in-flight case steps 1 and 3 run in the submit, except that the displaced plan is not yet in
  hand; D1a's service step drops it, and runs steps 2, 4 and 5.
- **D4. Completion.** The displaced revision completes with the newer one's adoption, flagged
  `SUPERSEDED` (#1314 D5).
- **D5. Acked-batch question.** The displaced revision's content is in the newer committed model,
  and a refusal leaves the catch-up running. In the in-flight case the ack follows the commit, and
  the newer candidate is held whole, control-side, until the service step publishes it; nothing is
  dropped while it waits. An ack can never precede a drop.

## Deliverables

1. D1-D4 (D1a included) in the control plane and `crates/host-core/src/catch_up.rs`.
2. Gates in the control-plane tests and `crates/host-core/tests/warm_successor.rs`.

## Authorized paths

- `crates/control-plane/src/`, `crates/host-core/src/catch_up.rs`, `crates/host-core/src/prepare.rs`
- `crates/host-core/tests/warm_successor.rs`, the control-plane tests

## Non-goals

- Deadline and fallbacks (#1358). Stop (#1359). C ABI service wiring (#1360).

## Objective gates

1. **No backpressure.** During a catch-up (#1355 gate 1 setup), a transaction that adds a second
   muted limiter track returns OK with path `rebuild`. In the control-owned case and in the
   published-unclaimed case, the swapped output from the newer adoption on equals a fresh plan with
   both tracks and the newer floors, fed the same PCM.
2. **Taken.** When render has already adopted the first successor, the newer edit's output equals
   the same reference.
3. **Refusal.** A newer edit refused by a cap leaves the first catch-up's adoption and output
   exactly as with no newer edit.
4. **Watermark.** The newer adoption's advance covers both revisions, with flags `EXACT | SUPERSEDED`
   and `superseded_count` up by 1.
5. **Rings released.** After D3, every predecessor ring's admission depth is back to configured
   before the new catch-up arms it again.
6. **Peeks move, never leak.** After a warm supersession, the newer catch-up reads every ring
   through the donated peek and reaches gate 1's equality. After an ordinary supersession, a later
   growth edit takes every peek with `take_peek`. After a refusal, the first catch-up still reads
   through its own peeks.
7. **In flight.** With a test-support hook that holds render inside the copy of #1354 D1 (the
   cell `Returned` with `Copying`), a structural submit on another thread returns OK with path
   `rebuild` while the hook still holds render: it does not wait. The newer candidate is not
   published. A `service` call while the hook holds gets `InFlight` and changes nothing. After the
   hook releases and the callback ends, the next `service` call publishes the newer candidate; from
   its adoption the output equals gate 1's reference, and the watermark shows gate 4's flags.
8. Commands: those of #1356.

## Test value

- Gate 1: a supersession that prepares the newer candidate against the displaced successor carries
  its stale state. Red.
- Gate 3: abandoning the peeks before the fallible checks leaves the first catch-up unable to
  finish. Red.
- Gate 4: completing the displaced revision as `EXACT`, or never, is wrong. Red.
- Gate 5: a displaced catch-up that keeps its gate armed starves the producer. Red.
- Gate 6: a newer warm candidate that calls `take_peek` while the displaced one owns the peeks
  gets `None` and falls back for no reason; a dropped successor whose peeks are never ended blocks
  every later growth. Red.
- Gate 7: a submit that spins on the state word until the copy ends does not return while the
  hook holds render; a service step that publishes before the copy ends, or never completes the
  withdrawal, fails the reference or the flags. Red.

## Dependencies

- *Catch up a returned successor and adopt it exactly at a scheduled sample* (#1355).
- *Hold live edits during a catch-up and apply them at the adoption sample* (#1356).
- *Supersede an unadopted candidate plan by compare-and-swap* (#1310).
- *Give the source ring a read-only peek cursor that gates release* (#1320).
- *Snapshot a running plan into a returned successor at a block* (#1354).
- *Add miso_engine_v1_service for bounded control work between edits* (#1348).
- *Publish an applied-revision watermark and complete edits asynchronously* (#1314).
