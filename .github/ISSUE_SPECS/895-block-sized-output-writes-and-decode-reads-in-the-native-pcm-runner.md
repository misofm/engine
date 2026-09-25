# Block-sized output writes and decode reads in the native PCM runner

## Product outcome

The reference runner writes its output one sample per `write_all` and hashes four bytes at a time, and decodes its sources with a scratch sized to one quantum so each block issues one small `read`. At a 128-frame quantum that is hundreds of syscalls per block, the throughput ceiling of every runner-based measurement. Tooling only; output bytes unchanged.

## Root evidence

- `tools/native-pcm-runner/src/lib.rs` near `:1095` (`.write_all(&bytes)` per sample inside the render loop, with a per-sample `digest.update`); the file is an unbuffered `File` opened near `:725-735`.
- Source decoders are prepared with a per-quantum scratch (the decode preparation near `:525-540` passes the quantum as the maximum frames per decode); `crates/source/src/native_wave.rs` near `:416` then issues one `read_exact` per decode; the worker's shape helper `worker_decode_buffer_shape` (`crates/source/src/native_source.rs:1459`) targets 64 KiB.
- `tools/native-pcm-runner/src/lib.rs:586` `source.planar.fill(0.0)` precedes a full overwrite.

## Smallest closable slice

Authorized paths: `tools/native-pcm-runner/src/lib.rs`, its tests, `scripts/check-native-pcm-runner.sh` only if a gate needs a new fixture, and this spec.

Serialise the whole block to a preallocated `Vec<u8>` (little-endian `f32`), one `write_all`, one `digest.update`, one `written_bytes` add. Prepare decoders with the worker's buffer shape. Delete the dead fill.

## Non-goals

No engine change; no change to the output format or digest definition.

## Objective gates

1. Output file bytes and reported digests are identical to before on every fixture `scripts/check-native-pcm-runner.sh` runs.
2. `scripts/test-native-pcm-runner-v1-policy.sh` and `scripts/test-native-pcm-runner-portability-v1-policy.sh` pass.
3. Optional descriptive: `strace -c` syscall count for one fixture before/after, attached.

## Dependencies

None.

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A means the change moves no rendered bit: same arithmetic in the same order, fewer passes, loads, stores, copies or branches. Every gate below that says "bit-identical" is a hard stop, not a tolerance.
- Render paths stay allocation-free, lock-free and syscall-free (`scripts/check-realtime-policy.sh` is mandatory). Only `crates/lane` may name `wide` or intrinsics (`scripts/check-lane-policy.sh`).
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, and the focused tests named below before every checkpoint. Commit on a `codex/<issue>-<slug>` branch from synchronized `main`; do not touch paths outside the authorized list.
- Do not quote a projected saving. If a benchmark row is listed, run it exactly once, one warmup and two measured rounds, and attach the record as descriptive evidence.
- Source of these findings: `docs/audits/render-path-cost-audit-2026-09-24.md` (PR #879) and tracker #349.
