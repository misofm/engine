# Catch a render kernel that the wasm build fully scalarises

Filed by root from #877's verification (backlog sprint, 2026-10-24).

## Product outcome

`scripts/check-web-audioworklet-callgraph.py` `kernel_arithmetic` skips every function with zero `f32x4` arithmetic as a helper. A non-roster render kernel (for example `InputStage::process`, `FaderRampStage::process_plane`, `route_mix_column_ramps_block`, the bypass crossfades) that a compiler or source change makes *fully* scalar therefore drops out of the count with no failure. Only rule 1 (the roster) catches this, and only for roster kernels; the K ratchet (26 counted against 13 today) would let 13 kernels vanish silently. A partly scalar kernel is already caught by rule 3.

## Smallest closable slice

Pin the set of render-kernel symbols that must stay present and vector-carrying: a reviewed table in the style of `INDIRECT_SITES` (symbol without the hash, one-line reason), checked on every artifact; a listed kernel that disappears or reaches vector == 0 fails; a stale entry fails. Seed it from the 26 kernels the 1.99.1 artifact counts today.

## Gates

The analyser's `--self-test` gains rows: a listed kernel made fully scalar fails; a removed listed kernel fails; a stale entry fails. `check-web-audioworklet.sh` passes on the shipped artifact.

## Paths

`scripts/check-web-audioworklet-callgraph.py`, its self-test fixtures, this spec.
