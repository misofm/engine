# Skip settled filter words in the post-ramp symmetry refresh

## Product outcome

After any ramping block the input bank refreshes its mono-collapse symmetry witness over 39 word pairs, spilling about 80 vectors and running hundreds of scalar compares, even when only trim or polarity ramped and the 24 filter word pairs cannot have changed. Skip the filter words when no filter ramp ran, and compare vectorially. Class A (control words only).

## Root evidence

- `crates/builtins/src/lib.rs:1216` `refresh_filter_channel_symmetry_post_ramp` (39 pairs of `lane_read`), `:1351` `settle_filter`, `settle`/`load_countdown` near `:1570-1618`, called from the ramp dispatch near `:1693-1748`. The filter word pairs were added by #808 after RT-5 (#238/#496/#611) had trimmed this path.

## Smallest closable slice

Authorized paths: `crates/builtins/src/lib.rs` (symmetry refresh only), its tests, and this spec.

Track `filter_ramping` for the block; when false, refresh only the trim/polarity words. Compare each word pair with `L::eq` and a single `store_bits` instead of a per-lane scalar loop.

## Non-goals

No change to the ramp kernels, to what "symmetric" means, or to the mono-collapse decision itself.

## Objective gates

1. New test: for random ramp sequences (trim only, filter only, both), the symmetry witness bits after each block are identical to the current implementation's.
2. Existing mono-collapse tests and `scripts/check-builtins-fixtures.sh` pass; `scripts/check-builtins-policy.sh` passes.

## Dependencies

None. Can be batched with "Fuse the ramping fader, mute and matrix pass".

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A means the change moves no rendered bit: same arithmetic in the same order, fewer passes, loads, stores, copies or branches. Every gate below that says "bit-identical" is a hard stop, not a tolerance.
- Render paths stay allocation-free, lock-free and syscall-free (`scripts/check-realtime-policy.sh` is mandatory). Only `crates/lane` may name `wide` or intrinsics (`scripts/check-lane-policy.sh`).
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, and the focused tests named below before every checkpoint. Commit on a `codex/<issue>-<slug>` branch from synchronized `main`; do not touch paths outside the authorized list.
- Do not quote a projected saving. If a benchmark row is listed, run it exactly once, one warmup and two measured rounds, and attach the record as descriptive evidence.
- Source of these findings: `docs/audits/render-path-cost-audit-2026-09-24.md` (PR #879) and tracker #349.
