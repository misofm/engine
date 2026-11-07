# Host core: an observation row can carry a window, or lanes, when it is not ready

Item HC-12 of the implicit-state-machine tracker #2258 (audit of 2026-11-06, agent-reported).
Re-verified here on `d441cd408`.

## Problem
`host_core::observation::ObservationRow` (`crates/host-core/src/observation.rs:102-120` on base)
stored five fields side by side:
- `status`;
- `state: Option<ObservationState>`;
- `window: Option<ObservationWindow>`;
- `left: Option<f32>`;
- `right: Option<f32>`.

The doc comments said the window is present exactly when the status is `Ready`, and the lanes only
when ready and asked for. The type did not enforce this. All of these compiled:
- a `Pending` or `Unarmed` row with a window;
- a `Ready` row with no window, or with no armed state;
- a lane that differs from its window's lane.

The base's own test built one: the refused-batch sentinel in
`crates/host-core/tests/effect_observation.rs` was `Ready` with a left lane and no window.

## Reachability (verified: not reachable)
**Construction and update sites.** `read_row` (`observation.rs:489-541` on base) is the only place
outside a test that builds a row. Nothing updates a row after that.
- `read_into` copies rows into a caller's table. Only tests call it.
- `read_with` passes each row by value to the host's `write`.
- The one production `write` is control-plane's `read_observation_rows`
  (`crates/control-plane/src/control.rs`). It converts the row at once with `wire_row` into a
  `protocol::ObservationReadRow`.
- The session's row table holds the wire type, not this one.

**`read_row` keeps the rule.** On base:
- `window` is `Some` only through `state.filter(armed && !disarmed)` and `window_since_arm`.
- `status` is `Ready` exactly when `window.is_some()`, after the two `Unarmed` arms. Both of those
  arms are ones where `window` is `None`.
- The lanes are set only in the `(Ready, Some(window), _)` arms, and are read from that window.

So every row it wrote held the rule.

**Every reader.**
- control-plane `wire_row`, which the C ABI and host-web both reach through
  `observation_read`/`read_observation_rows`;
- host-core's own tests.

capi and host-web use `protocol::ObservationReadRow` and never name the host-core type.

**The wire.** The wire does not depend on the in-memory layout. `protocol::ObservationReadRow` is a
separate type, and `observation_row_is_valid` (`crates/protocol/src/message_wire.rs`) enforces the
same rule in both directions:
- `write_observation_read_page` returns `InvalidField` for a row that breaks it;
- `decode_observation_read_page` refuses one.

The SDK's `decodeObservationRow` (`sdk/src/core/control-frame.ts`) also throws on "an observation
row that is not ready carries a window or a lane". So even a broken host-core row could not reach
a client. It would have turned into an internal fault at encode time.

**Threads.** Rows are written on the control thread. That is the session's command path: the
Worker, the C ABI caller's thread, or the non-isolated page's message handler outside `process()`.
Render only publishes the slot through `ObservationReader::publish_*`. No render code is touched,
so no rendered bit can move, and no state crosses threads that is not already crossing.

So this is hardening. There is no red-on-base defect test, because none is reachable.

## Contract
- `ObservationRow { reading: ObservationReading, tap }`.
- `ObservationReading` has three variants:
  - `Pending { state: Option<ObservationState> }`;
  - `Unarmed { state: Option<ObservationState> }`;
  - `Ready { arm_sample, arm_tag, window, channels }`.
- Only `Ready` holds a window. `Ready`'s armed state is implied, so it is not stored.
- `status()`, `window()`, `left()` and `right()` derive from the reading. The lanes are read from
  the window by the selection's channels, so a lane cannot disagree with its window.
- `read_row` builds the reading in one `match`. The ready guard is `window_since_arm`, as before.
- `wire_row` matches on the reading: `Ready` gives `armed: true` and its own arm sample and tag;
  `Pending` and `Unarmed` map their state as before.
- The wire type, its encoding and its validator are unchanged, and no wire byte moves.

Model chosen: the tracker's `enum Row { Ready{window, left, right}, .. }`, with two changes:
- the lanes are derived from the window and the selection's channels, not stored;
- the slot state moves into the variants, so a `Ready` row cannot lack its armed state.

## Non-goals
- The wire layout, `protocol::ObservationReadRow` and its validator.
- `tap: Option<..>`, which is `None` only in a default row. A default row exists only for
  `read_into`'s caller-owned table, and only tests call `read_into`.
- The arm-tag-zero case. A slot armed with tag `0` while no record is issued would read `Ready`
  with `arm_tag == 0`, which the wire validator refuses. The review found it unreachable: every
  arm tag comes from `next_arm_tag`, which starts at 1, and the wire maps an arm to 0, meaning
  "issue fresh".
- `crates/host-core/src/spectrum.rs`, `source.rs`, `meters.rs` and `warm.rs`. Other agents own
  them; `window_since_arm` in `meters.rs` is called, not changed.

## Gates
- `cargo test --locked` for host-core, control-plane, capi and host-web with the CI features.
- `cargo clippy --locked -p host-core -p control-plane --all-targets -- -D warnings`, with the
  test-support features and with none.
- `cargo fmt --check`.
- `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps -p host-core -p control-plane`.
- `bash scripts/check-workspace-policy.sh` and `bash scripts/check-host-core-policy.sh`.
- A one-time comparison against the base rule, recorded below and not committed.
- Mutations of the lane, readiness and wire-armed rules.

## Evidence (attempt 1)

**Fix.**
- `crates/host-core/src/observation.rs`: `ObservationRow` becomes `{ reading, tap }`, with the new
  `ObservationReading` enum and the four accessors.
- `read_row` builds the reading in one `match`.
- `crates/control-plane/src/control.rs`: `wire_row` matches the reading.
- `crates/host-core/tests/effect_observation.rs`:
  - the refused-batch sentinel is now a legal `Ready` row with no tap, which no read writes;
  - the batch test reads through the accessors;
  - its index construction moves into an `observation_index` helper that the new test shares.

**Red on base.** None is possible: no reachable row breaks the rule (see Reachability). The type
change removes the illegal combinations.

**New test.** `a_re_armed_tap_reads_pending_until_a_window_opens_after_the_re_arm`
(`crates/host-core/tests/effect_observation.rs`). It re-arms a tap whose slot still holds the
previous arm's window, then checks three things:
- the slot really holds that window at the new arm sample and tag;
- the row is `Pending`, with no window and no lane;
- after a full window the row is `Ready`, with a window that opened at or after the re-arm.

Test value: it goes red if the ready rule stops comparing the window with the arm sample. A
re-armed tap would then report the previous arm's window as its own, and at the C ABI that row
breaks the wire's presence rule, an internal fault. No existing test catches this, because every
other read test arms a slot that holds no window yet. Mutation M2 below survived the whole
observation suite before this test was added. The test is green on base, because base is correct.
It defends the guard this change rewrote.

**Mutations (each reverted).**
- M1, `left()` returns `window.right`: `capi`
  `runtime::live_tests::an_observation_read_follows_the_arm_through_the_c_abi` goes red.
- M2, ready guard `window_since_arm(window, 0)`, which ignores the arm sample:
  - before the new test, it survived host-core `effect_observation`, every capi and host-web lib
    test matching `observation`, and host-web `protocol_cross_host` and `worker_handoff`;
  - with the new test, `a_re_armed_tap_reads_pending_until_a_window_opens_after_the_re_arm` goes
    red (`left: Ready, right: Pending`).
- M3, `wire_row`'s `Ready` maps `armed: false`. These go red, because the encoder refuses the
  row:
  - capi `an_observation_read_follows_the_arm_through_the_c_abi`;
  - host-web lib `selected_observation_reads_keep_same_track_owner_windows_and_sequences` and
    `selected_observation_reads_are_bounded_stable_and_non_consuming`;
  - `protocol_cross_host` `both_hosts_read_one_observation_tap_alike` and
    `the_browser_and_the_c_abi_give_the_same_bytes_for_one_script`;
  - `worker_handoff` `the_worker_reads_an_observation_tap_while_the_worklet_renders`.
- M4, `right()` only for `Both`: host-core `a_read_checks_the_whole_batch_before_it_writes_a_row`
  and host-web `selected_observation_reads_are_bounded_stable_and_non_consuming` go red.

**No wire field moved (one-time, not committed).** A temporary probe in `read_row` recomputed the
base rule verbatim next to the new reading. For every row it asserted that these are equal to the
base values:
- `status()`;
- `window()`, all six fields bit for bit;
- `left()` and `right()`, by `to_bits`;
- the wire's `armed`, `arm_sample` and `arm_tag`.

Runs:
- `cargo test -p host-core -p control-plane` (test-support and control-provider): all pass;
  2 rows compared.
- `cargo test -p capi -p host-web` (test-support features), without printing: 430 tests pass, so
  every assertion held. That includes the C ABI read test, whose 1-row and 256-row reads are
  counted for allocations.
- The same suite with per-binary counts, skipping that one allocation-counted test, because the
  probe's printing allocates: 429 pass; 3,170 rows compared.

Total over the counted runs: 3,172 rows, made of 33 `Pending`, 3 `Unarmed` and 3,136 `Ready`.
The probe was then removed (the source equals the pre-probe copy).

**Gates (head).**
- `cargo test --locked --no-fail-fast -p host-core -p control-plane -p capi -p host-web
  --all-targets`, with
  `host-core/test-support,host-core/control-provider,control-plane/test-support,protocol/test-support,engine/test-support,session/test-support,graph/test-support,host-web/test-support,host-web/render-lock-probe,engine/realtime-audit`:
  100 binaries and 1,145 tests, all pass.
- Doctests for `-p host-core -p control-plane` (with test-support): pass.
- `cargo clippy --locked -p host-core -p control-plane --all-targets -- -D warnings`: clean in
  both runs:
  - with `host-core/test-support,host-core/control-provider,control-plane/test-support`;
  - `-p host-core` with no features.
- `cargo fmt --check`: clean.
- `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps -p host-core -p control-plane`: clean.
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
- `bash scripts/check-host-core-policy.sh`: `host-core policy: ok`.
- Not run:
  - the realtime policy tool, because no render-path code changed;
  - `scripts/test-web-audioworklet.sh`, because no web or JS file and no wire byte changed.

## Follow-ups (not in this issue; the coordinator tracks them in #2258)
- Remove `read_into`. It has no production caller and only tests use it. It is the only reason
  `ObservationRow` needs `Default` and `tap: Option`, so both go with it.
- Optionally type the engine's `ObservationState` so that `armed` and `arm_tag` cannot disagree,
  for example an armed state that carries its tag.

## Verdict (attempt 1)
**PASS** (Sol's adversarial review). The reviewer confirmed three points:
- the wire mapping is identical for all three readings;
- the new test is legitimate, because it is the only test that catches M2;
- tag 0 is not reachable (see Non-goals).

Two minors and the follow-up list are folded in on the same branch, after a merge of
`origin/main`:
1. `read_row` builds `Ready` from the window `window_since_arm` returns. It used to test that
   result and then store the slot's own window, which depended on the function returning its
   input.
2. The doc of `Unarmed`'s `state` said "as for `Pending`", which implied the state is armed. It
   now says the state is the published state at or above the issued tag, and that it either reads
   unarmed or is superseded by an issued disarm.
3. The follow-ups above.

Re-run after the fold:
- `cargo test --locked` for host-core and control-plane, with test-support and control-provider;
- clippy `-D warnings`;
- `cargo fmt --check`;
- rustdoc `-D warnings`.

All pass. The tests ran 66 binaries and 715 tests. `scripts/check-workspace-policy.sh` is also ok.
