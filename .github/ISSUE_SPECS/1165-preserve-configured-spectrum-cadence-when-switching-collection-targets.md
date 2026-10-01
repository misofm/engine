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
