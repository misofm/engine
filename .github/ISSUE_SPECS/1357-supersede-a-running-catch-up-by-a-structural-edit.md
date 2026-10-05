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
- A catch-up's successor is in one of three places:
  - control-owned: returned (#1354) and rendering forward (#1355);
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
  - If it is published and unclaimed, the control plane withdraws it (#1310 D1).
  - If render took it, it is the base, as in #1310's *taken* case. The newer edit is then an
    ordinary rebuild, or a new catch-up if it grows latency again.
- **D2. Prepare against P0.** In the withdrawn cases, the newer candidate is prepared against the
  running plan P0 (#1310 D2). Its lead is computed from P0, not from the displaced successor. If it
  grows latency it is a new warm successor; otherwise it is an ordinary candidate.
- **D3. Infallible order.** Only after every fallible check (#1310 D4):
  1. the protocol commit;
  2. abandon the displaced catch-up's peeks (#1320), and drop its hold (#1356 D4) and its plan on
     the control thread;
  3. donate the displaced successor's new rings (#1310 D2);
  4. publish the newer candidate, `CopyAndReturn` when it is warm.

  On a refusal the displaced catch-up continues untouched, with its peeks still armed.
- **D4. Completion.** The displaced revision completes with the newer one's adoption, flagged
  `SUPERSEDED` (#1314 D5).
- **D5. Acked-batch question.** The displaced revision's content is in the newer committed model,
  and a refusal leaves the catch-up running. An ack can never precede a drop.

## Deliverables

1. D1-D4 in the control plane and `crates/host-core/src/catch_up.rs`.
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
6. Commands: those of #1356.

## Test value

- Gate 1: a supersession that prepares the newer candidate against the displaced successor carries
  its stale state. Red.
- Gate 3: abandoning the peeks before the fallible checks leaves the first catch-up unable to
  finish. Red.
- Gate 4: completing the displaced revision as `EXACT`, or never, is wrong. Red.
- Gate 5: a displaced catch-up that keeps its gate armed starves the producer. Red.

## Dependencies

- *Catch up a returned successor and adopt it exactly at a scheduled sample* (#1355).
- *Hold live edits during a catch-up and apply them at the adoption sample* (#1356).
- *Supersede an unadopted candidate plan by compare-and-swap* (#1310).
- *Publish an applied-revision watermark and complete edits asynchronously* (#1314).
