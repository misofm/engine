# Replace the live-control queue depth with an attach flag in host-core

Stream B of decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-2).
Successor of *Hold effect parameter, bypass and EQ-target values in latest-target cells* (#1345),
from its attempt 1 open item 4 and verdict m1 (`/home/bl/misofm/submix-verdicts/1345-attempt1.md`,
"m1"). Code anchors verified on `codex/d15-stream-b3` at `3bcd99ebe` (#1345 attempt 1 is on that
branch; attempt 2 is pending). The anchors move after #1345 attempt 2: re-verify every anchor
before you start. In a later check, the `lib.rs` anchors of `hosts/host-web` had already moved by
1 to 35 lines (`live_control_request` was near `:7466`).

Root decisions 2026-10-12 after an adversarial review: the sibling issue *Make the browser
live-command boot word an on/off flag* uses layout option (A), a 0/1 flag at the same offset, with
every other value refused, and lands after #1345, before H #1332 and C #1406. This issue does not
depend on that choice.

**Why this is its own issue.** The whole removal of the live-control queue depth touches about 75
files: 30 Rust files for the host-core field and about 45 browser, SDK, harness and benchmark files
for the boot word. That does not fit half a day (AGENTS.md, "split it up front"). The two halves
close independently:

- This issue removes `control_queue_depth` in Rust. It is mechanical, changes no ABI and no
  behaviour, and is about two hours.
- The sibling issue *Make the browser live-command boot word an on/off flag* retires the browser
  boot word's number. It is an ABI-mirror change and is about half a day. It depends on this issue.

## Product outcome

A host that prepares a plan says only whether it wants live controls. It no longer passes a depth
that sizes nothing. `HostLiveControlRequest` has a `bool`, and the four depth constants that
existed only to fill the old field are deleted. Rendered bits, resource charges and every ABI stay
the same.

## Context

- **The field.** `HostLiveControlRequest::control_queue_depth: Option<NonZeroUsize>`
  (`crates/host-core/src/prepare.rs:321-331`; the default is `None` at `:362-372`). The field's own
  doc says the depth "sizes no queue ... so only `Some` matters". Every strip, route and effect lane
  is a latest-target cell with no depth (#1312, #1345, #1346, #1347). The effect observation queue
  has one slot per declared tap (#1345 D2).
- **Its five reads, all `is_some`/`is_none` or `Some(_)`.**
  - `:1340`: the strip input lane;
  - `:1467-1475`: the effect cells;
  - `:1486-1491`: the refusal `host.observation.live_controls`;
  - `:1514-1525`: the strip control requests;
  - `:1658-1665`: the route cells.
  Doc mentions: `:378` (`HostLiveLanes`) and `:806` (the lanes entry).
  `validate_options` (`hosts/host-web/src/lib.rs:2090`) runs before `live_control_request`.
- **The constants that only fill it.**
  - `STRIP_LIVE_QUEUE_DEPTH = 16` (`prepare.rs:304-311`), re-exported at
    `crates/host-core/src/lib.rs:140`.
  - `control_plane::LIVE_QUEUE_DEPTH` (`crates/control-plane/src/compile.rs:15-20`), re-exported at
    `crates/control-plane/src/lib.rs:73` and imported at `crates/capi/src/runtime/mod.rs:41`. The
    C ABI's preparation passes `Some(LIVE_QUEUE_DEPTH)` (`compile.rs:616-619`).
  - A test-local copy in `crates/capi/tests/resource_lifecycle.rs:612-613`.
  - `console_workload::CONTROL_QUEUE_DEPTH = 8` (`tools/console-workload/src/lib.rs:84-85`). No
    code reads it (`git grep CONTROL_QUEUE_DEPTH` finds only its definition).
- **The browser's one writer.** `live_control_request`
  (`hosts/host-web/src/lib.rs:7431-7463`) maps the boot word to a depth (`:7441-7444`). The fallback
  is `STRIP_LIVE_QUEUE_DEPTH` (import at `:38`), and the result is always `Some` (#1326 D1). The
  `WebBootOptions` field doc links `[`host_core::STRIP_LIVE_QUEUE_DEPTH`]` (`:1229-1231`); that link
  breaks when the constant goes.
- **The C ABI carries no depth.** The header has no queue-depth field
  (`grep -n -i queue crates/capi/include/miso_engine_v1.h`). capi passes the depth only in tests:
  `runtime/live_tests.rs:1731`, `runtime/tests.rs:2762`, `tests/resource_lifecycle.rs:648`, `:707`
  and `:1926`.
- **Test writers.** Each passes `Some(NonZeroUsize::new(n))` or `None`:
  - `crates/host-core/src/limiter_linked_session.rs:282`;
  - in `crates/host-core/tests/`: `collapse_arming.rs:81`, `effect_live_controls.rs:57`,
    `effect_observation.rs:83`, `:674`, `:777`, `:794`, `input_liveness_live_controls.rs:96`,
    `live_addressing.rs:84`, `live_delta.rs:794`, `:1191`, `live_lanes.rs:142`, `live_routes.rs:77`,
    `live_transaction.rs:174`, `prepare.rs:393`, `:489`, `:715`, `:839`, `randomized.rs:118`,
    `spectrum.rs:414`, `strip_controls.rs:136`, `strip_handles.rs:88`, `strip_meters.rs:652`,
    `successor_swap.rs:963`, `symmetry_witness.rs:97`, `track_delay.rs:67` and `vca_live.rs:1137`.
- **Not stored.** No retained structure holds a `HostLiveControlRequest`: it is passed by reference
  to the preparation entries only (`prepare.rs:755-1211`). A smaller field moves no resource count.

## Decisions frozen

- **D1.** `control_queue_depth: Option<NonZeroUsize>` becomes `attach_controls: bool`, with the
  default `false`. The doc says: whether to attach live controls; which lanes is the caller's
  `HostLiveLanes`. `Some(_)` becomes `true` and `None` becomes `false` at every read and writer.
  Nothing else in preparation changes.
- **D2.** Delete `STRIP_LIVE_QUEUE_DEPTH`, `control_plane::LIVE_QUEUE_DEPTH` and their
  re-exports, the test-local copy in `resource_lifecycle.rs`, and
  `console_workload::CONTROL_QUEUE_DEPTH`.
- **D3.** In host-web, `live_control_request` passes `attach_controls: true`, and the match at
  `:7441-7444` goes. The boot word keeps its meaning until the sibling issue *Make the browser
  live-command boot word an on/off flag* lands. The match's `None` branch (a word that does not fit
  `u32`) cannot be reached: boot refuses any word above `MAXIMUM_COMMAND_RECORDS` first
  (`lib.rs:6482`). So deleting the match loses no case.
- **D4.** The name is unversioned (AGENTS.md version-suffix rule). No rendered bit, no resource
  count and no C or browser ABI byte moves.

## Deliverables and authorized paths

- `crates/host-core/src/prepare.rs`: the field, `Default`, the five reads and the doc comments at
  `:305-311` (deleted with the constant), `:378`, `:806` and the route-cell comment at `:1660-1661`
  ("the request's depth sizes the effect queues only").
- `crates/host-core/src/lib.rs`: the re-export line only.
- `crates/host-core/src/limiter_linked_session.rs` and the host-core test files listed in Context:
  the field literal only, plus a `NonZeroUsize` import that becomes unused.
- `crates/control-plane/src/{compile.rs,lib.rs}`: the constant, its re-export and the request
  literal.
- `crates/capi/src/runtime/{mod.rs,live_tests.rs,tests.rs}` and
  `crates/capi/tests/resource_lifecycle.rs`: the import and the request literals.
- `hosts/host-web/src/lib.rs`: the import (`:38`), `live_control_request` only, and the
  `WebBootOptions` field doc's sentence that links `[`host_core::STRIP_LIVE_QUEUE_DEPTH`]`
  (`:1229-1231`) unless #1345 attempt 2 already removed it (named exception: H owns the file).
- `tools/console-workload/src/lib.rs`: the unused constant.
- This issue's own spec file.

## Non-goals

- The browser boot word, its mirrors and the SDK option. They belong to the sibling issue *Make the
  browser live-command boot word an on/off flag*.
- `meter_queue_depth` and the observation queue's depth. They are real queues.
- Any spec text, handoff or record that quotes the old names.

## Objective gates

1. **No new test.** The field becomes a `bool`, and the type system holds the change. Tests that
   already exercise both values keep their behaviour:
   - `effect_observation.rs:775`, `observation_without_a_control_channel_is_refused`
     (`false` → `host.observation.live_controls`);
   - `live_routes.rs`'s fresh plans with no live controls (`Live::new(.., None)`, route fold
     kept).
   - Mutation (PR evidence): make the observation refusal read `!lanes.effects` only. The
     `effect_observation` refusal test turns red.
2. **No moved bit or byte (PR evidence, not a committed test).**
   - Every resource corpus, ceiling and retained-bytes test passes unchanged, with no re-pin.
   - `git grep -n -E 'control_queue_depth|STRIP_LIVE_QUEUE_DEPTH|LIVE_QUEUE_DEPTH|CONTROL_QUEUE_DEPTH'`
     finds only specs, handoffs and verdicts.
3. **Commands.**
   - `cargo test --locked -p host-core --all-features`
   - `cargo test --locked -p control-plane --features test-support`
   - `cargo test --locked -p capi`
   - `cargo test --locked -p host-web --lib`
   - `cargo test --locked -p console-workload`
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
   - `bash scripts/check-workspace-policy.sh`
   - `bash scripts/check-realtime-policy.sh`, run once to show that no region moved.

## Test value

No new or rewritten test. Gate 1's tests already defend the `false` path. The edit to the 30 test
literals is a rename and changes no assertion.

## Dependencies

- #1345 (attempt 2 pending): the last lane moved to cells; its D2 sizes the observation queue per
  tap.
- #1346, #1347: the input and route lanes moved to cells. Both are on `codex/d15-stream-b3`.
- The sibling issue *Make the browser live-command boot word an on/off flag* depends on this one.

## Placement (root edits `STREAMS.md`)

Stream B, after #1345, before the sibling issue *Make the browser live-command boot word an on/off
flag*. Hot-file rows:

- `crates/host-core/src/prepare.rs`: this issue edits the field, the five reads and the docs. It is
  in either order with the A, C, D, F, G and H slices; the later slice rebases. #1346 and #1347 edit
  the same doc comments, so this issue comes after them.
- `crates/control-plane/src/*`: this issue (`compile.rs` constant and literal) comes after B #1312,
  in any order with the rest; the later slice rebases.
- `hosts/host-web/src/lib.rs` (H owns): this issue (`live_control_request`) lands before H #1332,
  like the other B slices in the row.
- `crates/capi/tests/resource_lifecycle.rs` and `crates/host-core/src/lib.rs` (J #1446 comment
  rows): in any order; the later slice rebases.

## Attempt record

- Attempt 1: not started.
