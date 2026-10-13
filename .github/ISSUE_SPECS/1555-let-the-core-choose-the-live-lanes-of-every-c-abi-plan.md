# Let the core choose the live lanes of every C ABI plan

Slice S4c of decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, design point C6,
"Live lanes on every plan"). Split from the planner's S4b. Code anchors verified on `main`
`df585e503`. Feature sprint mode (owner, 2026-10-13): one implementer, normal tests, a batch
review; no per-attempt verifier or mutation runs.

## Product outcome

Which live lanes a C ABI plan carries follows from the session's live control rows, through one
core function, instead of a host constant. When a later slice makes a row live, the plans that need
its lane get it with no host change. Preparation no longer takes a queue depth that sizes nothing.

## Context

- **The C ABI's constant.** `C_ABI_LIVE_LANES` (`crates/control-plane/src/compile.rs:22-31`) adds
  the effect lanes to `HostLiveLanes::FADER_AND_MATRIX`; both preparation calls pass it
  (`:622-635`). `LIVE_QUEUE_DEPTH` (`:12-20`) only fills `Some(..)`. Both are re-exported
  (`crates/control-plane/src/lib.rs:73`) and imported by capi (`crates/capi/src/runtime/mod.rs:41`).
- **The lane set.** `HostLiveLanes { strip_input, effects, routes }`
  (`crates/host-core/src/prepare.rs:388-415`); a strip's fader/mute and matrix/pan cells are always
  attached when live controls are. `HostLiveControlRequest::control_queue_depth:
  Option<NonZeroUsize>` (`:327-337`, default `None` `:370-372`) only says "attach" (its doc,
  `:328-331`); its reads are `:1524`, `:1556`, `:1671`, `:1693`, `:1719`, `:1863`.
  `STRIP_LIVE_QUEUE_DEPTH` (`:310-317`) fills it.
- **#1523** (*Replace the live-control queue depth with an attach flag in host-core*, not started)
  lists every writer of the field: host-core's tests, `limiter_linked_session.rs`, capi's tests,
  `crates/capi/tests/resource_lifecycle.rs`, `tools/console-workload/src/lib.rs` and host-web's
  `live_control_request` (`hosts/host-web/src/lib.rs:7609`, which always passes
  `Some`). This slice deletes the field instead.
- **The browser** chooses its lanes from its boot word (`live_lanes`, `hosts/host-web/src/lib.rs:7648`)
  until #1565 deletes it (#1524). Its input and route lanes carry live edits today, while the core's
  input and route rows stay `RuledLiveNotBuilt` until #1556 and #1557, so the browser keeps its
  selection in this slice.
- **The rows.** `controls::list` (#1546, #1549) states each row's class and family.

## Decisions frozen for this slice

- **D1. One function.** `controls::live_lanes(&SessionModel) -> controls::LiveLanes
  { strip_input: bool, effects: bool, routes: bool }`: `strip_input` when some strip's input-family
  row is `Live`; `effects` when some effect instance has a `Live` row; `routes` when some route row
  is `Live`. Fader/mute and matrix/pan lanes are always attached and have no flag. host-core
  converts `LiveLanes` into `HostLiveLanes` with one `From` impl.
- **D2. The C ABI uses it.** `compile.rs` passes `controls::live_lanes(model).into()` for every
  preparation, the successor's included. `C_ABI_LIVE_LANES` and `LIVE_QUEUE_DEPTH` and their
  re-exports and imports are deleted. With today's rows the result equals `C_ABI_LIVE_LANES` for
  every session that has an effect, and drops the (empty) effect selection for one that has none.
- **D3. No depth.** `HostLiveControlRequest::control_queue_depth` is deleted. Every entry that
  takes a `HostLiveControlRequest` attaches live controls; a caller that wants none calls the plain
  entry (`prepare_host_runtime`). Each `Some(_)` read becomes "attached" and each `None` writer
  moves to the plain entry or is deleted with its case. `STRIP_LIVE_QUEUE_DEPTH` and
  `console_workload::CONTROL_QUEUE_DEPTH` are deleted. The observation refusal
  `host.observation.live_controls` stays for a request with taps and no effect lanes.
- **D4. The browser.** host-web keeps choosing `HostLiveLanes` from its boot word; only
  `live_control_request`'s depth line goes (named exception in stream H's file). #1565 deletes the
  boot word (#1524).
- **D5. Evidence, once (not a gate).** The PR records, for the console-workload 64-track session
  on the C ABI, the plan's retained bytes (`HostPrepareReport`) and the median per-block render time
  over 1,000 blocks after one warm-up run, with today's lanes and with `HostLiveLanes::ALL` (what
  every plan carries once #1556 and #1557 flip their rows). One invocation, one warm-up, two measured
  rounds; no tuning.
- **D6. No moved bit.** Every existing C ABI test, resource ceiling and `audit capi` digest passes
  unchanged.

## Deliverables

1. D1 in `crates/controls`; the `From` impl in host-core.
2. D2 in control-plane and capi; D3 across host-core, its tests and every writer; D4.
3. D5's evidence line in the PR and in this spec's record.

## Authorized paths

- `crates/controls/**`.
- `crates/host-core/src/{prepare.rs,lib.rs,limiter_linked_session.rs}`, `crates/host-core/tests/**`
  (the request literal only).
- `crates/control-plane/src/{compile.rs,lib.rs}`.
- `crates/capi/src/runtime/{mod.rs,tests.rs,live_tests.rs}`, `crates/capi/tests/resource_lifecycle.rs`.
- `tools/console-workload/src/lib.rs`.
- `hosts/host-web/src/lib.rs`: `live_control_request`'s depth line and its import only (named
  exception).
- `Cargo.lock`.

## Non-goals

- No new lane kind and no class flip (#1556, #1557).
- No browser lane change (#1562). No per-route lane selection (#1391 keeps its render work).

## Objective gates

Commands: `cargo test --locked -p controls`, `cargo test --locked -p host-core --all-features`,
`cargo test --locked -p control-plane --features test-support`, `cargo test --locked -p capi`,
`cargo test --locked -p host-web --lib`, `cargo test --locked -p console-workload`,
`cargo build --locked --release -p audit -p capi` then `./target/release/audit capi`.

1. **Lanes follow rows.** A `controls` test: a session with no effect gives no effect lane; one
   insert gives `effects`; with a test-only row table that makes a route row live, `routes` is set.
2. **Same plans.** A capi test prepares a session with a console slot and an insert and asserts the
   attached lanes (effect producers present, no input cells, no route producers), as before.
3. **Nothing moved.** The existing suites and `audit capi` pass with no re-pin; `git grep -n -E
   'control_queue_depth|STRIP_LIVE_QUEUE_DEPTH|LIVE_QUEUE_DEPTH|CONTROL_QUEUE_DEPTH|C_ABI_LIVE_LANES'`
   finds only specs, handoffs and verdicts.

## Test value

- Gate 1 turns red if a live row's lane is not attached, so a live edit would find no cell.
- Gate 2 turns red if the switch changes the C ABI's lane set before a row flips.
- Gate 3 turns red if a depth or a host lane constant survives.

## Dependencies

- #1554 (the rows drive the classifier the lanes serve). #1546, #1549.

## Supersedes and shrinks

- Supersedes *Replace the live-control queue depth with an attach flag in host-core* (#1523).
- Shrinks *Give every route whose tap precedes its strip's fader a live lane on every plan* (#1391):
  its D2 ("attach whenever `control_queue_depth` is `Some`") becomes "attach when the route's rows
  are live"; its render work stays (see `amendments-a.md`; #1557 has the route half).
