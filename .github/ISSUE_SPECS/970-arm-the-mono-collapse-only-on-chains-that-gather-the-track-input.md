# Arm the mono collapse only on chains that gather the track input


Filed from the dual-mono research (`docs/handoffs/dual-mono-2026-09-27/DUAL-MONO.md`). Class A correctness fix. Base `6ca203f8`.

## Problem

A mono-mapped track (`left_source_channel == right_source_channel`) can render **wrong audio**: its
right channel is replaced by its left one. It happens when the track's strip is split into more
than one bank chain and a stage *before* the later chain makes L and R differ.

The collapse's structural term `SOURCE` means "the track input carries identical planes". It is
decided per track from the session (`session_structural_symmetry`,
`crates/builtins-compiler/src/lib.rs:3936`) and joined onto chains by
`Runtime::arm_mono_collapse` (`crates/graph/src/runtime.rs:2304-2316`), which arms **every** chain
whose lanes' tracks have it. That premise is true only for the chain that gathers the track input.
A later chain reads planes produced by an earlier chain or a per-node op of the same track, which
are in neither its own witness (`BankChain::lane_symmetry_bank`, `crates/rack/src/lib.rs:2003`)
nor the join. `crates/rack/src/lib.rs:2342-2350` assumes `SOURCE` "cannot be an episode", which is
only true of the first chain.

A strip splits when a stage meter or a send taps an internal boundary (`PostInputBuiltins`,
`PostSimd1`, `PostDynamic`), or when a per-node op (delay, sidechained effect, effect in a partial
cohort) sits between banked racks.

Reproduced by two tests (file `crates/host-core/tests/dualmono_probe.rs` in
`docs/handoffs/dual-mono-2026-09-27/dual-mono-prototypes.patch`; eight tracks of
`fixtures/session/v1/parametric-eq-bank-console.json`, all mono-mapped; collapse armed versus
`force_mono_collapse_off(true)`, output bits compared):

* asymmetric EQ in `simd1` (band gain +6 dB left, -6 dB right), symmetric EQ in `simd2`, console
  meter at `MeterTap::PostSimd1`: right channel differs from sample 0;
* asymmetric per-node `miso.delay` in `dynamic` (5 ms / 7 ms) between two banked EQs: right channel
  differs from sample 240.

## Outcome

Only a chain whose every lane's **first slot is that track's input-builtins stage**
(`GraphNodeId::TrackStage { stage: TrackStage::PostInputBuiltins, .. }`) may be armed on the
structural witness. Every later chain stays unarmed, so it renders dual: correct, and slower only
on split strips. No standing row changes shape, bits or collapse counts (all standing strips are
one chain per cohort).

## Read first

* `crates/graph/src/runtime.rs:5661-5710`: where each unit's `UnitIdentity` is built from its ops;
  `node_of(ops[lane])` is the first slot's node for lane `lane`.
* `crates/graph/src/runtime.rs:1961-1980, 2047-2062`: `UnitIdentity` and the compile-time
  assertion that its flags fit the padding; on wasm32 there is room for exactly four flag bytes,
  and all four are used.
* `crates/graph/src/runtime.rs:2304-2316`: `arm_mono_collapse`.

## Authorized paths

`crates/graph/src/runtime.rs`, `crates/host-core/tests/` (one new test file), and this issue's spec.

## Design constraint

Do not grow `UnitIdentity` (the wasm32 assertion must hold). Carry the "reads the track input" fact
some other way, for example a per-unit `Box<[bool]>` beside `identity` computed in the same loop,
or by deciding the arming there and storing only the chain's existing `collapse_source`. Bind-time
only; nothing on the render path changes.

## Gates

* The two probe tests above, ported as a new `crates/host-core/tests/` file: **red on the base,
  green after**. Keep the two controls (symmetric per-node delay: bit-identical and still
  collapsing its first chain; asymmetric EQ in one chain: bit-identical, no collapse).
* A count assertion: in the per-node-delay probe, `bank_collapse_counters()[1]` is `1` (the first
  chain only), not `2`.
* Unchanged: `cargo test -p console-workload --test chain_shape` (every collapse counter, e.g. 8 of
  8 cohorts on `_mono`, 4 of 8 on `half_mono`), `-p host-core --test symmetry_witness --test
  track_delay --test input_liveness_console`, `-p rack --test mono_reengage`, `-p graph --test
  rt9_resident_bank_input_alloc`.
* `cargo check -p graph --target wasm32-unknown-unknown` (the identity-size assertion).
* `cargo fmt --check`, `cargo clippy -p graph -p host-core --all-targets -- -D warnings`.

## Non-goals

Letting a later chain collapse when its predecessor collapsed in the same block (a separate
design issue); anything about pooling or detection.

