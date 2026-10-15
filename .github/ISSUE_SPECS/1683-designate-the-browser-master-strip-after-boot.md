# Designate the browser master strip after boot

Successor of #1682 (an empty session boots), decision 17. Code anchors verified on the batch 13a
branch `codex/controls-batch-13aw`. Issue number: assigned at filing (written here as #1683).
Feature sprint mode (owner, 2026-10-13): one implementer, normal tests, a batch review.

## Product outcome

An app that opens an empty mixer can still have a designated master strip: the designation is a
control the app sets (or changes) after boot, through the one core path, instead of a boot option
that must name a strip that exists at boot.

## Context

- The browser boot option `liveControlMasterTrackPlusOne` (with observation taps) designates a
  master strip at boot; over an empty session the boot is refused `host.observation.master_track`,
  because the strip does not exist yet (#1682's decision record).
- Since #1682 an empty session boots on both hosts and grows by structural transactions.

## Decisions to take (the implementer proposes, the coordinator rules)

- Where the designation lives: a session-scoped control row (a setting) the core owns and both
  hosts read, or a host option that may name a strip added later (resolved at each plan's
  preparation).
- What a designation naming a strip that does not exist yet does (refused before commit, or held
  until the strip exists).

## Objective gates

1. Boot empty, add a track, designate it master after boot; the master's observation behaves as a
   boot-time designation does today, on the browser (and on the C ABI if the row is core-owned,
   with a parity row).
2. host-web and control-plane tests; the browser qualification.

## Dependencies

- #1682.

## Size

Half a day.

## Decision record: #1683 (batch 14a)

**Decisions taken (implementer's proposal, for the coordinator's ruling).**

- **Where it lives: a core-owned monitor row.** Every strip (track and submix) has an own row
  `master` (control ID 4, toggle, `scope: monitor`, family `setting`, fixed motion, not
  automatable). Like `solo` it is host-local: held in the control plane's monitor overlay
  (`controls::MonitorOverlay::master`), never saved, never in a snapshot, and a write is a
  monitor-only `SET_CONTROL` that spends no revision and replies path `monitor`. It is held by
  strip ID, so a strip added or removed before the master never moves it. At most one strip is
  the master: designating one ends the other's designation; clearing a strip that is not the
  master changes nothing. Both hosts reach it through the same `SESSION_TRANSACTION_APPLY`; the
  SDK's generated builders gain `strip.master(enabled)`.
- **A strip that does not exist yet: refused before commit.** The write is checked by
  `controls::check_monitor` like any address; an unknown strip is `AddressUnknown`. A
  structural transaction that removes the master strip ends the designation
  (`MonitorOverlay::retain_strips_of`), as a removed strip keeps no solo.

**What shipped.**

- `controls`: `MonitorOverlay` gains `master`, `is_master`, `set_master`, `same_solo`;
  `MonitorWrite` gains `row: MonitorRow { Solo, Master }` and its `soloed` field is now `on`;
  `MonitorOverlay::after` applies each write to its row, and only solo writes choose the ramp.
- `protocol`: a later monitor write replaces an earlier one of the same strip *and row*.
- `host-core`: the classifiers compare solos only for the effective values (a master change
  moves none) and mark the owners whose master row changed, so the catalog refreshes them. The
  preparation resolves the master once, before allocating: a first plan from
  `HostLiveControlRequest::master_track` (the browser's boot word, by index, refused
  `host.observation.master_track` when it names no strip, as before); a successor from
  `SuccessorBase::monitor.next` by ID. A successor request that names an index is refused with
  the same code. The boot catalog lists the boot word's designation.
- `control-plane`: the boot plan's `master_track` seeds the session's overlay
  (`PreparedRuntime::master_track`); `SessionState::master_strip()` reads it.
- `host-web`: the boot word designates the first plan only (successor requests carry `None`);
  after every successful control command `ReadyOwnership::sync_master` resolves the core's
  master by ID in the rendering plan's tables and in each replacement's that waits for adoption,
  and writes `PlanShape::master_track` and the meter header's `master_track_plus_one`. The meter
  fold is unchanged.

**Departures.** None from frozen text. The boot word `liveControlMasterTrackPlusOne` stays: it
seeds the core's overlay, so a boot designation and a later `SET_CONTROL` are one state. It also
fixes a latent defect: a successor plan no longer reuses the boot index (after a strip was added
before the master, it named another strip, or refused the rebuild).

**Gates.**

1. Browser: `structural_apply::a_master_designated_after_an_empty_boot_reads_as_a_boot_designation`
   (`hosts/host-web/src/tests.rs`): empty boot refuses the boot word; a rebuild adds `t0`
   (compressor); `SET_CONTROL master` on `t0` applies on path `monitor` at the same revision;
   the header names strip 1 and the master word is `t0`'s own, bit for bit; a later rebuild adds
   `a0` before it and the master follows `t0` to strip 2; an unknown strip is refused; clearing
   reads absent. C ABI parity row:
   `both_hosts_designate_the_master_strip_after_an_empty_boot_alike`
   (`hosts/host-web/tests/control_parity.rs`): same verdicts, control list and state pages and
   snapshots on both hosts for designate, move, refuse and clear.
   Core: `the_master_designation_is_one_monitor_row_held_by_strip_id`
   (`crates/controls/tests/monitor.rs`), `a_successor_takes_its_master_from_the_overlay_by_strip_id`
   (`crates/host-core/tests/strip_meters.rs`); `list_yields_every_expected_address_once_in_ruled_order`
   updated for the new row.
2. Suites (all green): `cargo test -p controls`, `-p protocol -p control-plane --all-features`,
   `-p host-core --all-features`, `-p host-web --all-features`, `-p capi --all-features`,
   `-p conformance`; `check-sdk-generated.sh`, `check-sdk-types.sh`; realtime policy ok (149
   regions); clippy `-D warnings` and fmt on the touched crates. The real-browser qualification
   was not run here (no browser matrix in this worktree); the boot word it uses is unchanged.

**Moved pins (measured).**

- `hosts/host-web/tests/browser-v1/expected.json`, simd128 `initialStatus` and
  `beforeDisposeStatus` `memoryBytes`: 2,818,048 -> 2,883,584 (one 64 KiB page), reason #1683
  (one more own row per strip in the catalog and the overlay's master). Measured with
  `check-browser-expected-resources.py` on this tree; the base `616106095` measures 2,818,048.
- `hosts/host-web/tests/fixtures/control-list.json`: blessed (`MISO_ENGINE_BLESS_FIXTURES=1`), four
  `master` rows added, one per strip of the parity session.
- SDK regenerated: `sdk/assets/miso-engine-v1-parameter-metadata.json`,
  `sdk/src/generated/controls.ts`, `sdk/src/generated/control-builders.ts`. No wire change, so no
  schema hash moved.
