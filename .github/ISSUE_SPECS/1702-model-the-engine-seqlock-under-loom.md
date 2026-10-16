# Model the engine seqlock under loom

Root ruling of 2026-10-16, inside batch 15c. Follows #1316 (B15-R1). Feature sprint mode (owner,
2026-10-13): one implementer, normal tests, a batch review. Issue number: assigned at filing
(written as #1702).

## Product outcome

The engine's one single-writer seqlock is proved under a weak memory model, not only stressed on
x86. Its two contracts, the applied-revision watermark and the per-ring seek report, are read by
hosts on iOS and Android (AArch64), where a missing fence or a missing sequence re-check tears a
record that x86's strong ordering hides. A loom model of the real code runs in CI on every
qualifying change.

## Context

- `crates/engine/src/realtime/seqlock.rs` (new in #1316, B15-R1): `PublishedWords<const N>`, its
  writer's `publish` (the one `REALTIME_POLICY_BEGIN`/`END` region) and its reader's bounded
  `read`. Before this issue it took `AtomicU64`, `Ordering`, `fence` and `Arc` from `core`/`std`
  directly, so loom could not model it; its only concurrency test was the x86 stress test
  `a_read_is_one_whole_publication_or_busy`.
- `crates/engine/src/realtime/watermark.rs` (applied-revision watermark, #1314) and
  `crates/source/src/lib.rs` (seek report, #1316) are its two instances.
- `crates/engine/src/realtime/spsc.rs` owns the engine's `sync` shim: `core`/`std` in every
  shipped build, `loom` under `--cfg loom`. The `spsc_loom_*` models run on it.
- CI: `.github/workflows/qualification.yml`, step "Realtime race model (engine SPSC loom leg)",
  `CARGO_TARGET_DIR=target/ci/loom RUSTFLAGS='--cfg loom --check-cfg=cfg(loom)'`.

## Decisions frozen

- **D1. One shim.** The seqlock takes `Arc`, `AtomicU64`, `Ordering` and `fence` from the existing
  `spsc.rs` `sync` shim; no second shim. A non-loom build resolves to the same `core`/`std` items
  as before, so shipped code is unchanged.
- **D2. Orderings unchanged.** The protocol's orderings stay as #1316 shipped them unless loom
  finds a defect; a defect is reported before it is fixed.
- **D3. The real code is the model.** The loom test calls `published_words`, `publish` and `read`;
  no hand-written copy of the protocol.
- **D4. A loom-only read bound.** Under `--cfg loom` one read takes 2 attempts instead of
  `MAXIMUM_READ_ATTEMPTS` (64): the no-tear argument holds per attempt, and 64 spinning attempts
  are beyond loom's reach. Shipped builds keep 64.
- **D5. Own CI step.** The model runs in its own step next to the SPSC leg, same target dir and
  flags, filtered to `seqlock_loom`, so neither filter can drop the other's models.

## Authorized paths

- `crates/engine/src/realtime/seqlock.rs`
- `crates/engine/src/realtime/spsc.rs` (the `sync` shim's visibility and its `fence` item only)
- `.github/workflows/qualification.yml` (the loom step only)
- this spec

## Objective gates

1. `seqlock_loom_a_read_is_one_whole_record_or_busy`: one writer publishes two records (`[1, 1]`,
   `[2, 2]` over the initial `[0, 0]`) while one reader reads; every `Ok` is one whole record, and
   a read after the writer is joined is `[2, 2]`. `Busy` is allowed. Passes under loom's default
   bounds.
2. CI runs it: `CARGO_TARGET_DIR=target/ci/loom RUSTFLAGS='--cfg loom --check-cfg=cfg(loom)'
   cargo test --locked --release -p engine --lib seqlock_loom`, and the `spsc_loom` leg still
   passes.
3. Commands: `cargo fmt --all`; `cargo clippy --locked -q -p engine --all-targets --all-features
   -- -D warnings`; `cargo test --locked -q -p engine --features engine/realtime-audit --lib`;
   `cargo run --locked --release -q -p realtime-policy` (region count unchanged);
   `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`;
   `python3 -B scripts/check-ci-path-routing.py`, `check-test-support-ci.py`,
   `check-script-reachability.py`.

## Test value

Gate 1 turns red if the reader's `Acquire` fence, the reader's second sequence compare, or the
writer's `Release` fence is removed or weakened: each lets a reader on a weakly ordered target
(AArch64) return words of two publications as one record, which the x86 stress test cannot see.

## Dependencies

- #1316 (the seqlock module, B15-R1).

## Decision record: #1702 (batch 15c)

**Shipped.**

- `crates/engine/src/realtime/spsc.rs`: the `sync` module is `pub(super)`; `Arc`, `AtomicU64`,
  `Ordering` and the new `fence` item are `pub(in crate::realtime)`, in both the `core`/`std` and
  the `loom` variants. Everything else stays `pub(super)`.
- `crates/engine/src/realtime/seqlock.rs`: imports `Arc`, `AtomicU64`, `Ordering`, `fence` from the
  shim (`core::sync::atomic::AtomicUsize` stays direct: it only sizes the retained-bytes mirror).
  New private `READ_ATTEMPTS` (2 under loom, `MAXIMUM_READ_ATTEMPTS` otherwise, D4) bounds `read`.
  New `loom_tests::seqlock_loom_a_read_is_one_whole_record_or_busy`. No ordering changed (D2): loom
  found no defect. The publish region is still one marked region; `realtime-policy` reports 151
  regions in 33 files, as before.
- `.github/workflows/qualification.yml`: new step "Realtime race model (engine seqlock loom leg)"
  after the SPSC leg, same `CARGO_TARGET_DIR` and `RUSTFLAGS`, filter `seqlock_loom` (D5).

**Model size.** A first model with two reads per run and the full 64-attempt bound did not finish
in 9 minutes; two reads with the 2-attempt bound took 302 s. The shipped model (one read, 2
attempts) explores in 4.3 s (about 28 s wall with the incremental build on the shared loom target
dir). The SPSC leg is unchanged: 15 models, 55 s.

**Red evidence** (scratch copy of the workspace outside the tracked tree, same loom command):

| Mutation | Result |
| --- | --- |
| Reader's `fence(Ordering::Acquire)` removed | FAILED: `a read returned a torn record [1, 2]` |
| Reader's second sequence compare removed (return after the word loads) | FAILED: `a read returned a torn record [0, 1]` |
| Writer's `fence(Ordering::Release)` removed | FAILED: `a read returned a torn record [1, 2]` |

**Checks.** fmt, clippy (`-D warnings`), the engine lib tests with `realtime-audit` (91 passed),
both loom legs, `realtime-policy` (ok, 151 regions in 33 files), `cargo doc` with `-D warnings`,
and the three CI scripts all pass.

**Known limit.** `cargo test` with a filter that matches no test passes; a rename of the model
that drops the `seqlock_loom` prefix would leave the CI step green and empty. The name carries the
prefix for that reason.
