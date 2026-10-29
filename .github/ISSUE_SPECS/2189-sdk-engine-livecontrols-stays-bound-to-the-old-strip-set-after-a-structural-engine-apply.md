# SDK: engine.liveControls() stays bound to the old strip set after a structural engine.apply

GitHub #2189. Found by the studio-v2 console's adjustable-buses spike on `@misofm/engine` 0.9.0
(2026-10-28). The console works around it with `createBrowserLiveControls(engine.host, base)`,
which reads the core's strip order again.

## Root cause

`engine.liveControls()` is cached, and only `replaceSession` cleared the cache
(`sdk/src/browser/engine.ts`). The live controls held the strip order they were made with
(`LiveControlEdits`, `sdk/src/core/live-controls.ts`), so after a structural
`engine.apply(transaction(base)...)` that adds or removes a submix, `edit.strip(<new bus>)` was
refused and an edit for a removed bus was not refused early.

## Decision record (attempt 1)

- D1: the live controls follow the order; the cache is not invalidated. Invalidation on a committed
  rebuild would leave a live-controls object an app already holds silently stale.
  `LiveControlTransport.order` may be a function that returns the transport's newest order, and
  `LiveControlEdits` reads it at each edit: the strip set `strip()` checks, the effect of a named
  console slot or insert, and the strip check of `withSession()`.
- D2: the browser passes the host's one strip-order reader's newest order
  (`BrowserStripOrders.latest`, the order `current` returned last). Every read of it (meter frames,
  feeds, the queue's effect-row check) moves the strips the live controls check.
- D3: before a committed `apply` or `replaceSession` on the `rebuild` path resolves, the browser
  engine reads the strip order again. The call is committed, so a failed read does not reject it;
  the next read catches up.
- D4: once the structure epoch moved from the one the session layout was checked against, the
  layout is dropped: index addressing refuses and IDs resolve from the control list alone, as
  `withSession()` already does for a session whose strips moved.
- D5: the headless engine already returns new live controls, with a fresh order, on each
  `liveControls()` call; its held objects are not changed by this issue.

## Gate (one test per claim)

`sdk/test/browser-evals.mjs`, `#2189: live controls an app holds follow a rebuild that adds or
removes a submix`: live controls taken before an apply that adds `bus` accept
`edit.strip("bus")` after it (and the core admits a fader edit on it once the plan is adopted), and
refuse it after an apply that removes it.

## Evidence (2026-10-29, base `0f97cc7ef`)

- Red with the static order (the browser passing `order` in place of the function): `the compiled
  session has no track or submix 'bus'; expected one of t1, t2, t3, t4`.
- Red with the post-rebuild order read removed (D3): the same message.
- Green: the SDK headless evals (704 of 704) on the artifact built from this tree.
- README: "Edit a playing session" states the behaviour; the control-list section says that a
  headless caller calls `liveControls()` again after a rebuild.
