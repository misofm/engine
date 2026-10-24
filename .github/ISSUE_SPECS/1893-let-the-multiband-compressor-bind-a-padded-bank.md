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

## Decision record (attempt 1, 2026-10-24)

- **Outcome corrected.** The planner never pads an insert group (`graph-compiler/src/banks.rs`, "An
  insert group is never padded"); it pads console groups only. The multiband was not on
  `effect_compiler::CONSOLE_ELIGIBLE_EFFECTS`, so no production path handed it a padded request,
  and a padded bind alone would be unreachable. The product outcome is: **the multiband becomes a
  console slot**, which decision 12 requires to accept padded requests. Decision 12 excluded it only
  until #1069 closed; #1069 closed on 2026-10-23, so root admitted it (root-level, pre-authorized).
- **Commit A (crate-local).** The `is_padded` decline is gone. `Instance` stores `active` (all lanes
  for a full bank and the scalar instance). D7 recovery zeroes and resets every failed lane, a pad
  lane too, but `Instance::charge` counts and reports active lanes only, and a block whose failures
  are all on pad lanes leaves the counters as they were. The bank skips a pad lane's automation, and
  `checked_track` refuses a pad lane's payload with `effect.state.track`, as the compressor does.
  Whole-bank decisions (quiet, silent fixed point, rest claim, channel agreement) read pad lanes,
  but a clone fed `+0.0` rests like any lane, so no active bit depends on it (coupling rule).
  `tests/product.rs`'s decline test is deleted (superseded).
- **Commit B (eligibility).** `CONSOLE_ELIGIBLE_EFFECTS` 6 -> 7 (appended, so no index moves),
  graph-compiler's `TEST_CONSOLE_ELIGIBLE` 7 -> 8, `native_session.rs`'s refused rows are the delay
  and the unknown identity (now at `post_insert`, so both sections keep a refusal row),
  `docs/SESSION_SCHEMA_V1.md`, and one amendment line in decision 12's Eligibility bullet.
- **#887.** Its tiled staging is PCM planes, independent of effect state size: nothing to check.

## Evidence (attempt 1)

- Tests (`src/padding_tests.rs`, both widths through the public `bind_homogeneous_bank`):
  `every_padded_bank_renders_its_members_per_node_bits` (random arm with the clone source drawn per
  seed, and the seven conformance fixtures) and
  `a_planted_nonfinite_state_in_a_padded_lane_recovers_uncharged`.
- Mutants (each red, then restored): charge mask removed -> the planted test red (pad lane charged
  one left block); pad-lane automation skip removed -> the per-node test red (`invalid_spans` on a
  pad lane); `checked_track` active check removed -> the per-node test red (pad payload served).
- Class A: no existing session has a multiband console slot, and a full bank's `active` is every
  lane, so no full-bank path changes. `tests/cross_target_digest.rs` passes unchanged.
