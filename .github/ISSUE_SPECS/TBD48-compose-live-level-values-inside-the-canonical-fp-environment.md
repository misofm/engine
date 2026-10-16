# Compose live level values inside the canonical floating-point environment

Placeholder from the batch 17 U1 fold (controls coordinator, 2026-10-16). Root files it and gives
it a number.

## Product outcome

A live fader, VCA or trim move gives the same gain bits as a rebuild or a fresh preparation of the
same committed value, whatever floating-point environment the host's control thread runs in. Every
dB composition and dB-to-gain conversion that the control plane does for a live value runs inside
`lane::CanonicalFpEnv`, on both hosts, as preparation does since the U1 fold of #1646. The same
holds for every other live level record the control thread lowers: pan/matrix records
(`matrix_record`, `lower_matrix_or_pan`), send and route records (`route_record_between`, whose
`route_coefficients` converts the route gain) and input-filter records (`input_records`); and for
the whole preparation (route and send gains converted by graph-compiler's `compile_graph`, native
effect coefficients).

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
  - the other live level records the classifier lowers on the control thread: pan/matrix
    (`prepare.rs` `matrix_record` / `lower_matrix_or_pan`), sends and routes
    (`route_record_between` -> `route_controls::route_record` -> `route_coefficients`) and input
    filters (`input_records`);
  - host-core's preparation outside the inner #1646 guards: `compile_graph`'s route gains
    (`graph-compiler` `route_values` -> `math::db_to_gain_f32`) and native effect coefficient
    preparation (coordinator addition, U1 fold verification).
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

## Decision record: TBD48 (batch 17)

Root ruling R17-7: inside batch 17, closes in this PR.

- **D1, where the guard sits.** One RAII `lane::CanonicalFpEnv` at each core control entry, none in
  a host adapter:
  - `SessionState::command` (`crates/control-plane/src/control.rs`), for the whole call: the
    service step, the protocol preparation, the live classification (`classify_live_edit` /
    `classify_live_staged`, whose `EffectiveDiff` / `LocalDiff` compose the effective fader and VCA
    sum), the catalog's `stage_live` and every rebuild. Both hosts reach a transaction only
    through it: the C ABI's `miso_engine_v1_submit_command` and the browser's
    `control_command` (Worker and single-instance alike, `run_command` -> `command`).
  - `SessionState::declare_discontinuity_serviced`: the one core path of
    `declare_discontinuity` and of a plain `session_seek` on both hosts; its successor's carried
    level values are composed under it.
  - `compile_children` (`crates/control-plane/src/compile.rs`): the boot of both hosts (catalog
    build and boot preparation).
  - host-core `ControlCatalog::build` and `ControlCatalog::stage_live`
    (`crates/host-core/src/control_provider.rs`): the catalog's derived rows are composed in the
    canonical environment whatever the caller (`prepare_host_runtime*` builds the catalog outside
    the #1646 preparation guard).
  The dB-to-gain conversion of a live fader record is not on the control thread: the record
  carries dB and render converts it (`set_fader_db` in `drain_fader_controls`), under render's own
  pinned environment. So the control thread's only environment-dependent level arithmetic is the
  composition, and the guard covers it.
  Every return path restores the caller's word (RAII `Drop`, errors and unwinding included).
  Nesting is safe: an inner guard (the preparation's, the catalog's) saves the canonical word and
  restores it. None of these entries runs on the render thread: `submit_unserviced`, the one
  control-plane call made inside `process()`, is not guarded and composes nothing. On wasm the
  guard is a zero-sized value. `control-plane` gains a `lane` dependency (`Cargo.lock` updated).
- **Preparation entry (coordinator addition).** host-core's
  `prepare_host_runtime_with_live_controls_policy_and_spectrum` (`crates/host-core/src/prepare.rs`)
  holds one guard for its whole body, so `GraphCompiler::compile_with_builtins` -> `compile_graph`
  (route and send gains, `math::db_to_gain_f32`), native effect coefficient preparation and the
  successor joins (`join_strips`, `join_routes`) run canonically on every preparation entry; the
  inner #1646 guards stay and nest. Gate: the #1646 fixture of
  `canonical_preparation::a_hostile_caller_environment_prepares_the_canonical_faders` now gives
  every route its own non-0 dB gain (`-0.77 - 0.53 k`) and asserts at least one converts
  differently under the hostile word; red, checked once, with only this entry guard removed
  (plane 0 differs).
- **Live level paths covered.** Every live level record is lowered by `classify_live_edit` /
  `classify_live_staged` (`live_delta.rs` `Family::*`), which only `SessionState::commit_live`
  calls, inside `SessionState::command`'s guard: fader and VCA (`level_records`,
  `EffectiveDiff` / `LocalDiff`), pan/matrix (`matrix_record`, `lower_matrix_or_pan`), sends and
  routes (`route_records` -> `route_record_between` -> `route_record` -> `route_coefficients`,
  which converts the gain on the control thread) and input filters (`input_records`). A
  successor's carried retargets (`join_strips`, `join_routes`, `input_records` at
  `prepare.rs:2365`) run inside the preparation entry guard. `RouteControlProducer::set` has no
  production caller. The gate stays fader + VCA.
- **D2, gate.** A live fader move on a VCA member and a live VCA offset move over it, each sent
  while the caller's thread holds FTZ, DAZ and round toward zero, commit live (same plan, no
  candidate) and, from the end of their ramp, render the bits of the committed session prepared
  on a canonical thread. The pair (member `-0.01384` dB, offset `1/3` dB, #1646's) composes to
  another `f32` and another gain under that word (asserted), so the gate is not vacuous.
  - C ABI: `runtime::live_tests::set_control::canonical_live::a_live_fader_and_vca_move_under_a_hostile_caller_word_render_the_canonical_bits`
    (`crates/capi/src/runtime/live_tests.rs`); it also asserts the catalog's derived
    `effective_fader_db` row holds the canonical composition.
  - Browser: `tests::canonical_live::a_live_fader_and_vca_move_under_a_hostile_caller_word_render_the_canonical_bits`
    (`hosts/host-web/src/tests.rs`, native lib test; the parity rig was not needed). Its reference
    is a canonical boot of the committed model on the stateless `flat_fader_base` fixture.
  - Both x86-64 and AArch64 cfgs; nothing on wasm.
  - Red, checked once, with the guard in `SessionState::command` removed: both tests fail at the
    render-bits assertion (C ABI at the fader case; browser at the fader case, and at the VCA case
    run alone). With that guard removed the catalog row still passed, because `stage_live` holds
    its own guard.
- **Where #1649 and #1650b plug in.** Their live S writes (the offsets-cell write of a VCA move on
  an automated member, and a hand move on an automated fader) run inside `SessionState::command`,
  so they are already under the guard. Their gate rows join the two `canonical_live` modules: add
  a `live_move_under_hostile_word` case per write (an automated member under a VCA for #1649, a
  fader hand move over an automated lane for #1650b), reusing the `MEMBER_DB` / `OFFSET_DB` pair.
- **Hostile-word helper (departure, realtime policy).** `realtime-policy` refuses `unsafe` outside
  its audited owner files, and the #1646 U1 fold's `stored_fader_automation.rs` already broke it
  (two `write_fp_control_word` blocks; the policy failed at the batch head). The one write now
  lives in the audited `crates/lane/src/fpenv.rs`: `hostile_fp_control_word` and
  `under_hostile_fp_env` (behind lane's `test-support` feature, x86-64 and AArch64), a safe RAII
  helper that installs the hostile word over the caller's and restores it on every return,
  panics included. The three gate tests call it with no `unsafe` of their own; capi, host-core and
  host-web name `lane` with `test-support` as a dev-dependency. The policy's allowlist is
  unchanged and passes. All three red checks were run again with the helper.
