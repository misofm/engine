# EQ: decide a dead section's elision state leg when its state can change, not on every block

## Product outcome

Since #979, every stationary EQ block checks each dead section's integrators with `section_state_is_inert` (`crates/parametric-eq/src/lib.rs`), about 20 calls per four-lane bank per block. The #979 verification measured the standing one-band browser EQ (the shipped `host_web.wasm` render export) +0.47 us (+2.4 %) against the batch head, which gives back most of #980's gain; native is flat. A dead (identity) section's state never moves: its recurrence leaves every inert word exactly where it is. So its leg (b) answer can only change where the state or the section's identity changes: a restore, a reset, a ramp that ends (a section becoming or leaving the identity), or a prepared-target switch. EQ-7 in `docs/handoffs/effects-2026-09-27/EQ-DIAGNOSIS.md` measured the saving of caching it at about -0.85 us in wasm and -0.17/-0.33 us natively (`Simd8`/`Simd4`).

## Smallest closable slice

Cache leg (b) per section (and per channel, if the state is per channel) beside the section's `identity` flag, recomputed at every site that writes that section's state or changes whether it is the identity, and read on stationary blocks instead of scanning. Class A: the admitted/refused verdict on every block must equal today's.

**Out of scope, and a known hazard.** Do not cache leg (c) (`section_state_is_finite_without_negative_zero`, live sections). A live section's state changes every sample. The kernel never writes `-0.0`, but since #977's amendment 1 leg (c) also requires finite integrators, and a live section restored with a very large finite word may overflow to an infinity on a later block. Caching leg (c) would need a proof that no reachable live state turns non-finite, which this issue does not attempt. If the implementer finds that leg (b) and leg (c) share a scan that cannot be split without cost, say so and stop.

## Objective gates

1. **Equivalence.** A test that drives every cache-writing site (restore with inert, non-inert and `-0.0` dead states, reset, a band switched on then off, a prepared-target switch, a ramp ending on the identity and one leaving it) and asserts after every block that the cached leg equals a fresh `section_state_is_inert` scan, at `f32`, `Simd4` and `Simd8`, in dev and release.
2. **Bit-identity.** Rendered words and integrators equal the batch head's on the #979 differential (restored and frozen dead states), and every `WORKLOADS` digest is unchanged.
3. **Mutations** (each alone, red): the cache is not refreshed on restore; not refreshed when a ramp ends on the identity; refreshed from the wrong channel.
4. **Timing, no regression** (descriptive, under the timing lock, built first outside it): the shipped `host_web.wasm` one-band, two-band and builtins isolates through its render export (harness as in #977's attempt-2 evidence), and native `Simd8`/`Simd4` `eq_only` isolates. The one-band browser isolate must not be slower than the batch head.
5. `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --check`, `scripts/check-web-audioworklet-callgraph.py` and the wasm gates pass. Render stays allocation-free.

## Held (root, 2026-09-28): not merged, owner to decide

Attempt 2 passed its gates (verdict `b02ceeaa` on `codex/998-eq-dead-leg-cache`), but Sol
recommends not merging it and root agrees: the gain is about 0.5 us on the browser one-band EQ
isolate (under 0.2 % of the console), while every future write site must keep the per-channel cache
fresh, and a miss is silent in release. The branch is kept. If the owner wants it, the merge must
also mark the cache stale in #1005's ramping arm (now merged).

## Owner ruling (2026-09-28): dropped

"I don't think a 0.5us improvement is worth it for a flimsy cache." Not merged. The branch
`codex/998-eq-dead-leg-cache` is kept for the record only.
