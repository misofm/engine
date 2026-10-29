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
- D4 (attempt 2, replacing attempt 1's epoch rule): the session layout serves a strip only while
  the newest list has the session's strips (the rule of `withSession()` and the headless engine)
  and that strip's console slots and inserts, each rack in the same order with the same effects
  (`stripSections`). A strip the rebuild changed gets no layout: an index on it refuses and its IDs
  resolve from the control list, so an index never addresses a moved insert; the other strips keep
  index addressing. An order whose sections the SDK did not read is held to the layout's epoch.
  The structure epoch also moves on a control-descriptor change (an automation upsert, an insert
  on another strip), so attempt 1's rule (drop the layout on any epoch move) broke index
  addressing on untouched strips (verify of `d41af1727`, MAJOR).
- D5: the headless engine already returns new live controls, with a fresh order, on each
  `liveControls()` call; its held objects are not changed by this issue.

## Gates (one test per claim)

`sdk/test/browser-evals.mjs`, `#2189: live controls an app holds follow a rebuild that adds or
removes a submix`: live controls taken before an apply that adds `bus` accept
`edit.strip("bus")` after it (and the core admits a fader edit on it once the plan is adopted), and
refuse it after an apply that removes it.

`sdk/test/browser-evals.mjs`, `#2189: a rebuild that moves no strip keeps index addressing on the
strips it left` (attempt 2): after a rebuild that puts a compressor before t1's EQ, both the held
and a fresh live controls resolve `t2.insert(0, "miso.parametric-eq")`, refuse
`t1.insert(0, ...)` (index 0 would name `insert-1`, now second) and resolve `t1.insert("glue")`.

## Evidence (2026-10-29, base `0f97cc7ef`)

- Red with the static order (the browser passing `order` in place of the function): `the compiled
  session has no track or submix 'bus'; expected one of t1, t2, t3, t4`.
- Red with the post-rebuild order read removed (D3): the same message.
- Green: the SDK headless evals (704 of 704) on the artifact built from this tree.
- Attempt 2, the second gate: red with attempt 1's epoch rule (`insert(0): this engine was not
  booted from a session the SDK built, ...` on t2) and red with the layout always kept (`Missing
  expected exception (MisoUsageError)` on t1). A first attempt-2 cut compared the console and
  insert sections in one row and failed four headless console evals: the list puts a strip's
  sections in signal order (pre-insert slots, inserts, post-insert slots), so each rack is compared
  on its own.
- Attempt 2 green: the SDK headless evals (705 of 705); `check-sdk-types.sh`,
  `check-sdk-generated.sh`, `check-sdk-deletions.py`, `sdk-package.sh check`,
  `check-workspace-policy.sh`; the Chromium browser qualification, both legs, with the SDK rows.
- Recorded residual (attempt 2 verify, MINOR, no code): the live controls read the newest order
  the page holds, so they are stale in two windows: between a rebuild's commit and the order read
  before `apply` or `replaceSession` resolves, and after a strip change committed on another page
  until this page next reads the order. An edit built in either window is held by the core (an
  unknown strip is refused `control.address.unknown`, a changed effect `control.effect.changed`).
  The README documents both.
- README: "Edit a playing session" states the behaviour; the control-list section says that a
  headless caller calls `liveControls()` again after a rebuild.
