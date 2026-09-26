# Build the console benchmark without graph test-support and align its frozen buffers

**Ruled** (coordinator, 2026-09-26): claims (d) and (e) and the harness half of change 6 of
`docs/handoffs/plumbing-floor-2026-09-26/DIAGNOSIS-2.md`, as confirmed by its adversarial
verification (`DIAGNOSIS-2-VERIFY.md`). Benchmark tooling only; no engine source changes.

## Product outcome

Two harness effects make the console benchmark measure something other than the shipped engine.

1. `tools/bench/Cargo.toml:31` builds `graph` with `features = ["test-support"]`, although nothing
   under `tools/bench/src` uses a test-only API. The benchmarked render path therefore carries test
   hooks (a probe test per unit, a thread-local counter per in-place Output input). The verification
   measured +40 to +110 cycles per block on the ring row.
2. The harness's frozen per-claim blocks (`FrozenGraphSource`, `tools/console-workload/src/lib.rs:1657`,
   two `[f32; QUANTUM]` arrays) and the session output planes (`lib.rs:1097`,
   `vec![0.0; QUANTUM * 2]`) land on whatever 16-byte boundary the allocator returns. At 16 or 48 mod 64
   half the 32-byte loads split cache lines. The verification measured about ±0.085 us of movement on
   the ring row from this alone.

Build the benchmark binary without `test-support` and give both buffers 64-byte alignment, so every
record measures the shipped engine at a fixed placement.

## Smallest closable slice

Authorized paths: `tools/bench/Cargo.toml`, `tools/console-workload/src/lib.rs`, their tests, and this
spec. `Cargo.lock` only if the feature change requires it.

1. In `tools/bench/Cargo.toml`, depend on `graph` without `test-support` (`graph.workspace = true`).
   If a bench **test** needs a test-only API, move that feature to `[dev-dependencies]` rather than
   keeping it on the binary.
2. Give `FrozenGraphSource` `#[repr(C, align(64))]`. Its two 512-byte arrays then both start on a
   64-byte boundary wherever the struct is boxed, including `FrozenSourceDriver::claims`
   (`lib.rs:1811`) and the bound row's `Box::new(frozen_track_source(..))` (`lib.rs:1786`).
3. Replace the output `Vec` (`lib.rs:1097`) with a 64-byte-aligned boxed buffer of the same length,
   for example a `#[repr(C, align(64))]` wrapper around `[f32; QUANTUM * 2]`. Every reader keeps
   seeing the same `&[f32]` / `&mut [f32]` slices.
4. No `unsafe`.

## Non-goals

No change to the engine's own allocations (the graph arena and `TransferBlock` alignment are deferred
until a live-producer row exists), to any workload's content, to the runner's arms, or to the record
schema.

## Objective gates

1. `cargo tree -p bench -e features -i graph` (with the target the runner builds) shows no
   `test-support`. `cargo build --release -p bench` and `cargo test -p bench` pass.
2. A console-workload test asserts `as_ptr() as usize % 64 == 0` for every frozen claim's left and
   right plane on the ring row, for the bound row's frozen blocks, and for both output planes.
3. Every console workload's 64-block digest is unchanged, including `BASE_DIGEST` in
   `tools/console-workload/tests/chain_shape.rs`.
4. The wasm console arm still builds: `tools/wasm-console` and `tools/wasm-console-guest` link
   `console-workload`. Run their build and tests as the repository's scripts do, and record whether
   any wasm console digest pin moved (a layout change may move the guest's bytes; a moved pin is
   repinned with the reason, never silently).
5. `scripts/operator/preflight-console-benchmark.sh` passes (zero-launch preflight). Do not run the
   timed runner; the coordinator runs it once after the batch.
6. fmt, clippy with `-D warnings`, and `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace
   --no-deps`.

## Console benchmark rows

Every row may move by a small amount in either direction; no digest may move. The coordinator's next
paired run records the new baseline.

## Dependencies

None. Land it first in the batch so the engine changes that follow are measured against the corrected
harness.

## Standing rules for the implementer

- Work only from this body. Do not survey the workspace.
- Commit on `codex/<issue>-<slug>` from synchronized `main`.
- Benchmarks are descriptive. Do not tune or retry a timing.
