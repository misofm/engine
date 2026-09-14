# #807 assignment 1 evidence

Worktree: `/tmp/miso-engine-807` (`codex/807-live-eq`), baseline `80f2918b` with the #805
delivery checkpoint `9d5535d7`. This assignment is a local, uncommitted foundation checkpoint;
no commit or push was made.

## Changed paths

- `crates/effect-contract/src/prepared_target.rs`
  - Added the fixed `PREPARED_EFFECT_TARGET_WORDS` constant, 56-byte `Copy`
    `PreparedEffectTarget`, borrowed `Copy` `EffectTargetRequest`, five-variant `Copy`
    `EffectTargetError`, and `NativeEffectTargetPreparation`.
  - Added target size, enclosing `EffectControlRecord <= 64`, `Copy`, and object-safety tests.
- `crates/effect-contract/src/lib.rs`
  - Re-exported the new contract types.
  - Added the default `NativeEffectFactory::target_preparation()` capability.
  - Added default scalar and bank application hooks returning `Unsupported` without mutation.
- `crates/engine/src/realtime/spsc.rs`
  - Added `Consumer::available_at_entry()`, using one acquire producer-cursor snapshot and the
    consumer-local cursor for bounded modular occupancy.
  - Added empty/full/wrapped and later-publication freeze tests.

`EffectControlRecord::PreparedTarget` was deliberately not added. No effect implementation was
changed or opted in; target staging, owner shadows, queue enlargement, and production admission
remain later assignments.

## Type and realtime facts

`PreparedEffectTarget` contains only `u32`, `ParameterChannel` (`repr(u32)`), and
`[u32; 12]`: `size_of == 56`, it is `Copy`, has no heap ownership or destructor, and satisfies the
current enclosing-record `size_of <= 64` assertion. `EffectTargetRequest` only borrows the final
configuration and touched mask. `validate_targets` is documented as bounded, allocation-free,
non-designing validation suitable for admission; `prepare_targets` is control/main/worker-plane
design and promises unchanged output storage on every error.

`available_at_entry()` takes `&self`, performs no cursor/cache/counter mutation, and does not loop
or chase the producer. The existing SPSC release/acquire wrappers, including loom's synchronization
shim, are retained.

## Focused gates

All commands ran from `/tmp/miso-engine-807` and passed:

```text
cargo fmt --all -- --check
pass (no output)

git diff --check
pass (no output)

cargo test --locked -p effect-contract
16 unit tests + 12 lattice + 7 live-control + 9 observation + 2 response-analysis tests passed;
0 doc-tests; no failures.

cargo test --locked -p engine --lib realtime::spsc
6 tests passed; no failures.

CARGO_TARGET_DIR=target/ci/loom RUSTFLAGS='--cfg loom --check-cfg=cfg(loom)' \\
  cargo test --locked --release -p engine --lib spsc_loom
1 loom test passed; no failures.

cargo check --locked -p effect-contract --target wasm32-unknown-unknown
Finished successfully.

cargo check --locked -p engine --target wasm32-unknown-unknown
Finished successfully.
```

## Remaining gates

The assignment is intentionally incomplete as a product feature. The deferred
`PreparedTarget` queue record, bounded target staging/consumption, EQ implementation and factory
opt-in, retained producer owner/shadow, host/browser companion preparation and admission, queue
resource accounting, and end-to-end scalar/bank/live-EQ evidence remain pending in later
assignments. No live-cut or issue-level completion claim follows from these foundation gates.
