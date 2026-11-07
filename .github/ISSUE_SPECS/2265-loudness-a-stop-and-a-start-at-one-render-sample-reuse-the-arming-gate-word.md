# Loudness: a stop and a start at one render sample reuse the arming's gate word

Found by the implicit-state-machine audit of 2026-11-06 (tracker #2258, item PR-10).

## Mission
Make a `LoudnessMonitor` arming's identity a value that no other arming can share, so the
module's rule holds by construction: the control half ingests only hops observed under its
current arming, and a block in flight across a stop and a start never feeds the new arming.

## Verified defect (base `de0878e23`)
`crates/control-plane/src/loudness.rs`:
- The gate word, and with it every hop's `ArmedHop::arm` tag, is the start's `render_sample`
  (`apply`, `Start`, lines 204 and 208). The monitor ingests a hop when
  `self.armed && hop.arm == self.arm` (`drain`, line 175). `arm: u64` plus `armed: bool`
  (lines 71-72) encode three states (never armed, armed, stopped) in two fields.
- `render_sample` is the next block's first sample as render last published it
  (`crates/control-plane/src/control.rs` 4694, published by `PlanState::publish_render_observation`,
  `crates/control-plane/src/plan.rs` 461-466, after the tap observed the block). A stop and a
  start with no published block between them read the same `render_sample`, so the second arming
  gets the first arming's word.
- The race is cross-thread wherever control and render run on different threads (the browser's
  isolated mode, Worker and AudioWorklet on one shared memory; the C ABI's control thread and the
  host's audio thread). A block that loaded the first arming's gate before the stop pushes its hops
  after the second start; they carry the old word, which equals the new one, and are ingested.
  When the block completes more than one hop, the stop's or the start's drain takes the first
  hops and the new arming takes the rest, so the new arming begins mid-block.
- The existing test (`a_record_of_an_earlier_arming_is_discarded`) re-arms at a distinct sample
  (0 then 960), so it cannot see this.

**Severity, re-assessed.** L. The ingested hops are always audio at or after the new arming's
origin, on a hop grid the tap restarted at or after it, and `integrating_since` reports the first
hop the analysis took, so no reading is numerically wrong. What fails is the stated guarantee:
the arming that a hop belongs to depends on a sample coincidence, and the new arming can open on
a partial block.

## Decision / model
- The gate word is `INACTIVE` (`0`) or an **arming generation**: a monotone counter the monitor
  advances at every start (`armings`), never a sample. Hops are tagged with it as before.
- The monitor's `arm: u64` + `armed: bool` become `arm: Option<u64>`: `Some(generation)` while
  armed, `None` otherwise. `armed` in the reply is `arm.is_some()`.
- The tap restarts its observer at the first block that loads a new generation, at that block's
  first sample (what it already did). The tap's `first_sample < gate` refusal goes: with a
  generation there is no origin to compare, and it was unreachable. A start stores the gate after
  it read `render_sample`, which render publishes after the tap's gate load for the block before
  (program order, `Release`/`Acquire`), so any block that can load a new generation starts at or
  after the `render_sample` the start read.
- In the race above, the in-flight block's hops now carry the old generation and are discarded
  whole; the new arming restarts at the next block, the same outcome as a start that races a
  block today.
- `gate` and `dropped` are loom's `AtomicU64` under `--cfg loom` (the `ModelledWord` pattern of
  `crates/host-core/src/spectrum.rs`), so a loom model runs the real tap and monitor.

## Non-goals
No change to `crates/loudness`, the protocol reply, the queue depth, or reset semantics. No
rendered bit moves; the meter only observes.

## Gates
- The rewritten unit test (a re-arming at the same sample) is red on base, green at head.
- `loudness_arming_loom`: render observes one two-hop block while control stops and re-starts at
  the same `render_sample`; in every interleaving the new arming ingests the block whole or not
  at all, and nothing when render ran under the earlier arming. A CI leg in
  `.github/workflows/qualification.yml`.
- No loudness reply moves: a one-time comparison of every reply in
  `crates/control-plane/tests/loudness_monitor.rs` at base and head, as evidence.
- `cargo test --locked -p control-plane`; clippy `-D warnings`; `cargo fmt --check`;
  `bash scripts/check-workspace-policy.sh`; `cargo run --release -p realtime-policy`.

## Evidence (attempt 1)

**Strictly required beyond the loudness files.** `crates/control-plane/Cargo.toml` names
`loom = "=0.7.2"` as a dev-dependency (one `Cargo.lock` line) for the model, and the same pin
under `[target.'cfg(loom)'.dependencies]`, because its library's `ModelledWord` names `loom` and a
dependent built under `--cfg loom` (capi, host-web) would otherwise fail with E0433 in
control-plane (reproduced with `cargo check -p capi` under the leg's flags; it builds after the
fix). `crates/host-core/Cargo.toml` names the same pin under `[target.'cfg(loom)'.dependencies]`:
the model builds host-core as a *dependency* under `--cfg loom`, where its spectrum
`ModelledWord` names `loom`, and a dev-dependency does not reach a dependency's build (the reason
`engine` has the same section for #1609). Nothing reaches either without `--cfg loom`.
`.github/workflows/qualification.yml` gains the loom leg, on the host-core leg's target dir and
flags, and the `test-release` job's timeout is re-derived for it (23 to 28 minutes: about 44 s
warm on 32 cores, estimated at 3 min cold on a 4-core runner; to be re-derived from the first
`main` run that includes the leg).

**Red on base.** `loudness::tests::a_record_of_an_earlier_arming_is_discarded` is rewritten, not
added: it now re-arms at the same sample (960, stop, 960) and tags the queued hops with the gate
word as render would have loaded it before the stop. It supersedes the old distinct-sample form,
which a generation makes the same case. On base `de0878e23`:
```
assertion `left == right` failed: no stale hop was delivered
  left: 1440
 right: 960
```
At head it passes.

**Loom.** `loudness::tests::loudness_arming_loom::loudness_arming_loom` passes at head (every
interleaving explored). Under `cargo test --release -p control-plane --lib loudness_arming_loom`
with the leg's flags.

**Mutations (each reverted).**
- The generation not advanced (`self.armings = 1` in place of `+= 1`, so both armings share a
  word, as base's sample did): the unit test is red (`left: 1440, right: 960`), and loom is red
  (`a block of the earlier arming fed the new one`).
- The same mutation with the model's per-word branch disabled, leaving only the
  model-independent assertion: loom finds base's interleaving and is red with
  `the new arming took part of a block: Some(480) to 960` (the stop's drain took the first hop,
  the new arming the second).

**Test value.**
- Unit test: red if an arming's identity is anything two armings can share, such as the start's
  render sample, so a hop of the earlier arming passes the current arming's check. No existing
  test re-arms at one sample.
- Loom model: red if, in any interleaving of a block with a stop and a start at one
  `render_sample`, the new arming ingests part of a block or a block render observed under the
  earlier arming, or loses a block render observed under the new one. Only a two-thread schedule
  reaches it; the session tests are single-thread.

**No reply moved.** A scratch `eprintln!` of every successful `LOUDNESS` reply in
`crates/control-plane/tests/loudness_monitor.rs` (12 replies over both tests, including a stop
and an immediate start on and off the hop grid), run at base `de0878e23` and at head and never
committed: all 12 replies are identical (`diff` shows only the timing line). No rendered bit can
move: the tap only observes the master planes.

**Gates (head).**
- `cargo test --locked -p control-plane --features test-support`: all 12 binaries pass.
- `cargo clippy --locked -p control-plane --all-targets --features test-support -- -D warnings`:
  clean (CI runs `--all-features`; without the feature the lib-test build has a pre-existing
  `compile_children_warm` dead-code error from `crates/control-plane/src/compile.rs`, untouched
  here). Clippy of the lib test under `--cfg loom`: clean.
- `cargo fmt --check -p control-plane`: clean.
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
- `cargo run --locked --release -p realtime-policy`: `realtime policy: ok (292 marked regions in
  46 files)`.
- The host-core spectrum loom leg still passes with host-core's new `cfg(loom)` dependency
  (3 passed).

## Verdict (attempt 1)
**PASS**, with two minor findings folded on the branch. The reviewer confirmed the argument for
removing `first_sample < gate`, the no-bit-moved comparison (12 of 12 replies identical), and
that the loom model catches the gate-before-drain mutation, which the unit test misses.
- *loom reached control-plane only as a dev-dependency*, so `cargo check -p capi` under
  `--cfg loom` failed with E0433 in control-plane. Folded: the `cfg(loom)` dependency above;
  that check now builds.
- *The `test-release` timeout was not re-derived* for the new leg. Folded: 23 to 28 minutes,
  derived in the comment, to be re-derived from the first `main` run.
