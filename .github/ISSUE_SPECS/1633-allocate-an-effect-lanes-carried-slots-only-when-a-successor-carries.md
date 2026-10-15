# Allocate an effect lane's carried slots only when a successor carries

Successor of #1280a, decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`). Code
anchors verified on the batch 10 branch `codex/controls-batch-10`. Issue number: assigned at filing
(written here as #1633). Feature sprint mode (owner, 2026-10-13): one implementer, normal tests, a
batch review. Root, 2026-10-14: the first lane-K slice of batch 11, before #1282.

## Product outcome

An effect lane holds no carry storage until a plan swap carries it, so a first boot pays nothing for
the carry: the browser's boot growth regains its margin under the 20 MiB pin, and the C ABI retains
less.

## Context

- #1280a gave every effect lane preallocated carried slots (`carried_targets:
  Box<[CarriedCell<PREPARED_EFFECT_TARGET_WORDS>]>` and `carried_parameters`, built in
  `effect_control_lane`, `crates/effect-contract/src/live.rs`). On the 1 MiB worst browser document
  that is 672 B x 768 EQ lanes = 516 KB, plus lane-struct growth; boot growth went from 20,447,232
  to 20,971,520 bytes (exactly at the pin after batch 10's #1281 fix).
- The slots serve only a successor that carries: `EffectControlLane::carry_from` fills them and
  `stage` resolves them in the successor's first block.
- `graph::install_effect_carry` already allocates the carry program's storage (`carry_scratch`) on
  the control thread for the lanes in `moves`, charged through `effect_carry_retained_bytes`.

## Decisions frozen for this slice

- **D1.** The carried slots are allocated in `install_effect_carry`, on the control thread, for the
  successor lanes in `moves` only, and charged through `effect_carry_retained_bytes`. A lane that no
  successor carries holds none.
- **D2.** `EffectControlLane::carry_from` reads a missing predecessor slot as empty, and
  `can_carry_from` refuses a successor lane whose slots were not installed, so a carry can never
  write into storage that does not exist.
- **D3.** The swap block stays allocation-, lock- and syscall-free; render reads no new state.

## Objective gates

1. Boot budget: `node scripts/check-web-boot-budget.mjs` growth measured below the 20 MiB pin, with
   the margin recorded in the decision record (batch 10: 20,971,520, zero margin). The multiplier
   does not change.
2. `capi_retained_bytes` is tightened to the new measurement (measured + 10 %, rounded up to 64), at
   both lane widths from the CI print.
3. Every carry gate of #1280a (and #1280b/c, #1281) passes unchanged, including zero allocations and
   frees on the swap block; the charge-equals-freed resource tests hold.
4. `cargo test --locked -p effect-contract -p graph -p graph-compiler -p rack -p host-core
   --all-features -p capi -p host-web --features host-web/test-support`; clippy; realtime-policy.

## Authorized paths

- `crates/effect-contract/src/live.rs`, `crates/graph/src/{lib.rs,runtime.rs}`,
  `crates/graph-compiler/src/estimate.rs`, their tests, `crates/capi/tests/resource_lifecycle.rs`
  (the ceilings), `hosts/host-web/tests/browser-v1/expected.json` (measured memory rows only), this
  spec's decision record.

## Dependencies

- #1280a. Before #1282.

## Size

Half a day to a day.

## Decision record: carried slots on demand (batch 11)

Shipped D1-D3 as frozen.

- `effect_control_lane` builds no carried slots. `EffectControlLane::install_carry_slots` (control
  thread, idempotent, none for a lane without cells) allocates them; `carry_slot_bytes` is the one
  spelling of their size; `carry_slots_installed` reports them.
- `graph::install_effect_carry` installs the slots of the successor sites in `moves` only (bank
  lanes of a live stage, and per-node instances), after every move validated. `effect_carry_slot_bytes`
  (new) reads the charge from the bound plan before the install; `effect_carry_retained_bytes` takes
  it as a third argument; `carry_program_retained_bytes` sums the same function over the installed
  successor sites, so admission and the retained row cannot disagree (the existing
  `debug_assert_eq!` in `prepare.rs` ties them).
- D2: `can_carry_from` is the old shape check plus "this lane's slots are installed";
  `carry_from` reads a predecessor that holds no slots as empty (`.get(i)` with an empty default).
  `Reads::read` takes `Option<&mut CarriedCell>`, so a lane without slots reads none in `stage`.
  The shape check alone is now `same_cell_shape`.
- D3: the swap block allocates and frees nothing and reads no new state beyond the slot slices'
  lengths; `stage` does one bounds-checked `get_mut` per cell, which `Box<[T]>` indexing did already.

Departure (mechanical, outside the authorized list): `rack::LiveControlEffectBankStage::can_carry_instance`
was used in both directions of a bank-lane/per-node move, but the new rule needs the *successor's*
slots installed, so the into-instance direction got its own `can_carry_lane_into_instance` (the
instance is then the successor); `carry_slot_bytes(lane)` and `install_carry_slots(lane)` were added
on the stage; `crates/graph/src/runtime.rs` calls them; `crates/host-core/src/prepare.rs` passes the
slot charge into the admission. No edit under `crates/engine/src/realtime/`.

Gates.

1. Boot budget (`target/ci/b11k-artifacts`, scripts/check-web-boot-budget.mjs with the growth
   printed): growth 20,971,520 -> 20,447,232 bytes; the pin is 20,971,520, so the margin is
   524,288 bytes (eight Wasm pages), the same growth as before #1280a. The multiplier is unchanged.
   `check-browser-expected-resources.py` passes with no re-pin: no expected.json memory row moved
   (a one-document first boot carries nothing).
2. `capi_retained_bytes` is 165,166 before and after (it charges capi's own allocations, not the
   plan's lanes). Its eight-lane ceiling is tightened 216,832 -> 181,696 (165,166 + 10 %, rounded up
   to 64). The four-lane value stays 216,832 until the next `aarch64-debug` print. The plan rows
   the slots were in fell at eight lanes: `graph_session_plus_plan_bytes` 321,846 -> 315,798,
   `graph_incremental_plan_bytes` 321,774 -> 315,726, `graph_metadata_bytes` 113,601 -> 107,553
   (-6,048 each); their ceilings (346,432, 346,368, 117,376 and the four-lane ones) still bound
   them and are left for the same print.
3. All carry gates pass unchanged: `cargo test --locked -p host-core --all-features` (the whole
   `successor_swap` suite, including zero allocations on the swap block and the charge tests),
   `-p effect-contract -p graph -p graph-compiler -p rack -p capi`, `-p host-web --features
   host-web/test-support`.
4. New tests, both in `crates/effect-contract/tests/live_control.rs`:
   `carried_slots_are_installed_for_a_successor_only_and_charged_exactly` (a built lane holds no
   slots and refuses a carry; the install grows `retained_resources` by exactly `carry_slot_bytes`;
   a second install changes nothing) and
   `a_predecessor_without_slots_hands_its_unread_write_to_an_installed_successor` (D2's
   missing-predecessor-slot read).
   Clippy `-D warnings`, fmt, rustdoc `-D warnings`, `check-builtins-fixtures.sh`,
   `check-cross-targets.sh`, `check-web-audioworklet.sh` with the artifacts, `realtime-policy` and
   `check-bench-policy.sh` pass.
