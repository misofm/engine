# Issue #966 handoff

- `ROOT-CAUSE.md`: the root-cause study.
- `VERIFY-966.md`: the adversarial verification of the brief.
- `prototype.patch`: the study's prototype fix. The fix landed from it unchanged.
- `evidence-harness.patch`: the study's scratch harness. Never apply its `crates/graph/src/lib.rs`,
  `Cargo.toml` or `Cargo.lock` hunks to a tree you commit from.
- `wasm-pins-harness.patch`: reproduces the four-lane pins (see below).

The #970 verification's reduced reproducer, `reduced-nobus-from-970-verify.json`, now lives at
`crates/graph-compiler/tests/data/`, where `bank_levels.rs` reads it.

## Reproducing the `Simd4` pins

`crates/graph-compiler/tests/bank_levels.rs` pins each reproducer's effect-bank count at
`Backend::current()`. An `x86-64-v3` build is `Simd8`, and its factories decline most four-lane
banks (D4), so the `Simd4` pins never run on x86 and no CI job builds the file for a four-lane
target. `wasm-pins-harness.patch` checks them in a `wasm32-unknown-unknown` + `simd128` guest,
where `Backend::current()` is `Simd4`, under the pinned wasmtime 47.0.3.

The patch adds:

- a guest crate, `tools/wasm-pins-966-guest`, that compiles `bank_levels.rs` by `#[path]`;
- a host test, `tools/wasm-gates/tests/wasm_pins_966.rs`;
- the guest as a workspace member.

It needs a new workspace member, so it is kept as a patch and never merged. Apply it only in a
scratch worktree:

```sh
git worktree add --detach ../engine-966-pins HEAD
cd ../engine-966-pins
git apply docs/handoffs/bug-966-2026-09-27/wasm-pins-harness.patch
CARGO_TARGET_DIR=target/wasm-pins-966 RUSTFLAGS='-C target-feature=+simd128' \
  cargo build --release --target wasm32-unknown-unknown -p wasm-pins-966-guest
WASM_PINS_966_GUEST=$PWD/target/wasm-pins-966/wasm32-unknown-unknown/release/wasm_pins_966_guest.wasm \
  WASM_PINS_966_COUNT=1000 cargo test --release -p wasm-gates --test wasm_pins_966 -- --nocapture
```

Notes on these commands:

- They run without `--locked`, because the new member adds a `Cargo.lock` entry.
- The build uses its own target directory, because a `wasm32` build and the host's release build
  can collide in a shared one.

The host test checks every reproducer:

- it asserts the pinned `Simd4` effect-bank count and the misaligned planned slots;
- it asserts no refusal and no cross-level bank;
- over 16 blocks, it asserts that the unarmed render and the render armed as `prepare_host_session`
  arms it both equal the `Scalar` render bit for bit.

The probe checks `WASM_PINS_966_COUNT` generator seeds from `WASM_PINS_966_START`:

- each seed must bind at `Simd4`;
- each misaligned seed must render the `Scalar` bits, unarmed and armed;
- `WASM_PINS_966_RENDER_ALL=1` renders every seed.

Remove the worktree afterwards with `git worktree remove --force ../engine-966-pins`.
