# Let the multiband compressor bind a padded bank

Filed by root (backlog sprint, 2026-10-24).

## Product outcome

`crates/multiband-compressor/src/lib.rs` (bank factory, near line 2023) still declines every padded bank request: "Issue #1088: the multiband is not padded until #1069 closes". #1069 closed on 2026-10-23, so the stated reason is gone. A partial group of multiband inserts now renders per node, while the other banked effects pad their partial groups with inactive lanes (decision 12, `docs/rulings/engine-footprint-2026-09-29.md`; issue #887 tiles the gather and scatter of padded banks).

## Smallest closable slice

1. Remove the padded decline. Bind a padded request as the other padded effects do: the pad lanes are inactive clones that never count, never report and never change an active lane's bits.
2. Join the padded-bank tests the other effects use (the `padding_tests` module shape): each member renders its per-node bits; a planted fault recovers its own lane alone; active lanes do not depend on the clone source.
3. Class A: no bit of an active lane moves; record one console and corpus digest comparison as PR evidence.

## Order

After #1871 (same file). Check #887's tiled staging covers the multiband's state size.

## Paths

`crates/multiband-compressor/src/lib.rs`, its tests, this spec; a ceiling row only if one crosses.
