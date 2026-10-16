# Compose live level values inside the canonical floating-point environment

Placeholder from the batch 17 U1 fold (controls coordinator, 2026-10-16). Root files it and gives
it a number.

## Product outcome

A live fader, VCA or trim move gives the same gain bits as a rebuild or a fresh preparation of the
same committed value, whatever floating-point environment the host's control thread runs in. Every
dB composition and dB-to-gain conversion that the control plane does for a live value runs inside
`lane::CanonicalFpEnv`, on both hosts, as preparation does since the U1 fold of #1646.

## Context

- The U1 fold of #1646 (batch 17) puts the builtins preparation body, both graph lowerings and
  host-core's static `controls::effective` call inside `CanonicalFpEnv`. A test prepares under a
  hostile caller word (FTZ, DAZ, round toward zero) and asserts canonical bits
  (`canonical_preparation::a_hostile_caller_environment_prepares_the_canonical_faders`).
- Live values do not have that guard yet. The fold agent found these places, which compose or
  convert on the control thread in the caller's environment:
  - host-core `live_delta.rs` level records (`LevelDiff::level_records`: `EffectiveDiff` in
    `prepare.rs`, `LocalDiff` in `live_delta.rs`), which compose the effective fader through
    `controls::effective` / `LocalEffective`;
  - the live setters that call `checked_fader_gain` when a record is written to a cell;
  - `crates/host-core/src/control_provider.rs:1199` and `:1573` (`controls::effective` for the
    catalog's derived rows; display only, but it must agree with render).
- README finding F11 of #1058 (preparation runs in the host's environment) is the same hazard.
- On a canonical host (every test host today) nothing differs, so no test is red today.

## Decisions to make at filing

- D1. Where the guard sits: once at the control-plane entry that classifies and commits a
  transaction (`SessionState` in `crates/control-plane`), so every live composition and conversion
  below it is covered on both hosts, or at each composition site. Prefer one entry guard (one core
  rule, thin hosts); the guard is cheap (two control-register writes).
- D2. Gate: a live fader move, a live VCA move (#1649's offsets-cell write) and a hand move on an
  automated fader (#1650b) made under a hostile caller word give the same gain bits as the same
  value prepared canonically, on the C ABI and the browser. Red if the guard is removed.

## Dependencies

- #1646 (U1 fold: the preparation guard and its test pattern), #1649, #1650b (their live writes
  join the gate).
