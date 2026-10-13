# Preserve configured spectrum cadence when switching collection targets

## Finding and evidence

During the complete host-core housekeeping audit (#1137), worker B found a source-level mismatch in `SpectrumCaptureCollection::select` (`crates/host-core/src/spectrum.rs`). Its public contract says that an active continuous capture restarts at the same cadence on the new prepared entry. It retrieves the old `SpectrumCadence`, then starts the new capture with `start_continuous(sample_rate_hz, quantum_frames)`. That call reconstructs the legacy default cadence and discards an explicitly configured `SpectrumHop` (256, 512 or 1024 frames when different from that default).

Root independently read the selection and both start paths. Existing `explicit_collection_start_uses_the_checked_hop` exercises a single-entry collection, so it never switches to another target. Existing collection switching coverage uses one-shot capture. This is a source-level finding; no failing behavioral reproducer or runtime fix is claimed yet. It is independent of the intermittent Firefox composite-gate failure recorded in PR #1164; no common cause is established.

## Smallest closable product slice

Preserve the active checked cadence when switching between two already-prepared collection entries, using the existing checked-hop/begin-continuous machinery. Keep all fallible admission checks before cancelling or changing the old active entry. Same-entry selection, one-shot selection, default cadence, selection/stream epochs, channel masks, queued-result invalidation and typed refusal behavior remain contractual. No FFT, capture-copy, queue or spectrum smoothing algorithm change.

Authorized future paths: `crates/host-core/src/spectrum.rs`, the existing spectrum test owner if needed, and this spec. Parent #763 continues to own the broader spectrum lifecycle. #896/#897 capture-copy/publication optimization and #1106 browser probe timing remain separate.

## Objective gates

1. A focused two-entry regression starts an explicit supported hop, switches targets during continuous capture, and observes the same cadence and actual successive publication sample spans on the replacement. Cover non-default hops without duplicating existing one-entry and one-shot families.
2. Record the regression red when the fix is reverted. Its unique defect is loss of an explicit checked hop on successful target replacement, which the existing one-entry start test cannot reach.
3. Existing atomic unknown-entry/channel-mask/busy/epoch refusal and queued-result/capture-selection checks pass; refused replacements retain the current selection and active cadence.
4. Existing allocation-free render and spectrum numeric owners pass, along with proportional feature/target/lint/policy checks. No new harness, benchmark or browser matrix is necessary for the bounded native state transition unless implementation expands a host boundary.

## Decision record

Recorded by root Sol on 2026-10-01 from worker B's full review and root's independent source read. Deferred from behavior-preserving housekeeping #1137. No implementation attempt is authorized or complete by this record; freeze the exact regression and review limits before a future attempt. This is a specific existing-contract bug, not an unresolved product-design decision.

## Amendment (decision 17, root 2026-10-13)

**Frozen for the next attempt.** Code read at `4900b52d3`: the defect is still present.
`SpectrumCaptureCollection::select` (`crates/host-core/src/spectrum.rs:979`) reads the old
entry's `SpectrumCadence` (`:998`) and restarts the new entry with
`start_continuous(cadence.sample_rate_hz(), cadence.quantum_frames())` (`:1001`), which builds
the default cadence (`SpectrumCadence::new`, `:1182`) and drops an explicit hop.

- **D1. Restart with the same cadence value.** `select` restarts the new entry with the old
  entry's `SpectrumCadence` value unchanged (sample rate, quantum and hop), through the capture's
  `begin_continuous(cadence, false)` (`:635`, same module). It builds no cadence: the default hop
  is not a `SpectrumHop` (`SpectrumHop::SUPPORTED` is 256, 512, 1024 and 2048), so it cannot be
  re-checked through `start_continuous_with_hop`.
- **D2. Nothing else changes.** The error mapping (`EpochOverflow`, otherwise `Busy`), the order
  (every fallible step before `cancel_except`), the selection and stream epochs, same-entry
  selection, one-shot selection and the default cadence stay as they are. No new public API.
- **D3. Paths.** `crates/host-core/src/spectrum.rs` (the fix and its unit test) and this spec.
- **D4. Order.** It lands before #1592, which moves this code (STREAMS root answer).

**Named test (gate 1).** `collection_select_keeps_the_cadence_on_the_new_entry`, in
`spectrum.rs`'s `mod tests`, beside `explicit_collection_start_uses_the_checked_hop` (`:3687`).
For each hop in `SpectrumHop::SUPPORTED`, and once with the default cadence: a two-entry
collection (two prepared targets, as in `collection_cancel_except_drains_every_capture_but_the_kept_one`,
`:3364`) selects entry 0, starts continuous capture, selects entry 1, and asserts
`collection.cadence()` equals the started cadence; then it renders a ramp through entry 1's
observer (as `collect_planar_ramp_windows`, `:2960`) and asserts that successive published windows
start `hop` frames apart. Gate 2: record it red with D1 reverted (every explicit hop fails; the
default case passes).
