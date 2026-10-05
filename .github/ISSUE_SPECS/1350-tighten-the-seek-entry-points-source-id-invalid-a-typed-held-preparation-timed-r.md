# Tighten the seek entry points: source.id.invalid, a typed held preparation, timed reads only

Stream B of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-12).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

Four seek-path defects found in the #1275 review are closed:
- a malformed source ID gets its own diagnostic, `source.id.invalid`, instead of a stale one;
- an accepted anchored seek that a host prepares between blocks is reported as held, so no export
  can report it as `RESULT_INTERNAL` (the #1293 D4 trap);
- a debug build catches off-grid lateness, which would starve a source;
- only the render-clocked read paths remain in production builds.

## Context

- **Stale diagnostic.** `source_seek_entry` (`crates/capi/src/ffi.rs:579`) returns
  `RESULT_INVALID_ARGUMENT` for a null source ID (`:595-596`) and for an empty, non-UTF-8 or
  over-127-byte one (`:597-601`; `MAX_SOURCE_ID_BYTES`, `:90`) without touching `last_error`, so
  the host reads the previous call's diagnostic. `miso_engine_v1_source_submit_planar_f32` does the
  same (`:439-440`, `:460-465`). Existing test of the oversized case: `ffi.rs:2590` (code only).
- **#1293 D4 trap.** `PcmSourceConsumer::prepare_seek` (`crates/source/src/lib.rs:1049-1068`)
  returns `false` for an anchored seek it holds, the same value it returns for "not ready". The
  browser's plain seek export (`AudioWorkletEngineHost::seek_source`,
  `hosts/host-web/src/lib.rs:3175-3207`) maps `false` to `RESULT_INTERNAL`, and the #1293 spec's
  D4 says to mirror it for `seek_at`, so every accepted future anchor would report
  `RESULT_INTERNAL`. The graph driver forwards the bool (`prepare_source_seek`,
  `crates/source/src/lib.rs:1682-1694`) and, on `true`, flags a generation change for the next
  block (`pending_generation_change`), which would be wrong for a held seek that applies later.
- **Off-grid lateness.** A late anchored seek applies at `frame + (first_sample - anchor)`
  (`observe_seek_at_block_boundary`, `:1353-1363`). The producer's blocks start on `frame`'s
  quantum grid; if lateness is not a quantum multiple, no block ever matches `next_frame`
  (`acquire_current_block`, `:1389-1428`; `current_matches_next_frame`, `:1439-1443`) and the
  source starves. Anchors and render clocks are quantum multiples today (anchor check `:771-774`;
  the render clock starts at 0 and advances by quanta), so this is an invariant to assert.
- **Untimed reads.** `begin_block` (`:1103`) and `read_block` (`:1077`) use `SeekClock::Now`, which
  applies an anchored seek at once and ignores its anchor. Production renders through
  `begin_block_at` (`:1721`). The untimed paths are called only by the crate's own tests and by
  `crates/source/tests/randomized.rs:454` and `crates/source/tests/seek_schedule_model.rs:486`.

## Decisions frozen for this slice

- **D1. `source.id.invalid`.** Defined once in capi beside its other own diagnostics
  (`crates/capi/src/runtime/error.rs`, or its #1309 successor). Set on a null, empty, non-UTF-8 or
  over-127-byte source ID in `source_seek_entry`, in `miso_engine_v1_source_submit_planar_f32`, and
  in `miso_engine_v1_source_seek_report` (added by #1316, which lands before this slice). Result
  codes unchanged (`INVALID_ARGUMENT`). Null pointers for other arguments keep their current
  behaviour.
- **D2. Typed preparation.** `prepare_seek` returns `SeekPreparation { Applied, Held, NotReady }`:
  `Applied` when today's `true`; `Held` when the requested generation and frame are exactly the held
  anchored seek's; `NotReady` otherwise (including the playing generation while a newer seek is
  held, as the existing test at `:2347-2359` asserts). The driver's `prepare_source_seek` keeps its
  `bool` (the graph trait is stream A's) and returns `true` for `Applied` and `Held`; it sets
  `pending_generation_change` only for `Applied`. The held seek's block reports its generation
  change itself (`generation_changed`, as today).
- **D3. Assertion.** In `observe_seek_at_block_boundary`, `debug_assert!` that the lateness is a
  multiple of `quantum_frames`, with a message naming the starvation it would cause. No release
  behaviour change.
- **D4. Timed reads only.** `begin_block` and `SeekClock::Now` become `#[cfg(test)]`. `read_block`
  is replaced by `read_block_at(&mut self, source_read_sample: u64, output_planes)` (timed, the clock
  of #1316 D1); the two integration tests call it with their block's first sample. Plain-seek
  behaviour of those tests is unchanged; any anchored step they take now waits for its anchor.

## Deliverables

1. D1 in capi; D2-D4 in `crates/source`, with the integration tests moved to `read_block_at`.

## Authorized paths

- `crates/capi/src/ffi.rs`, `crates/capi/src/runtime/error.rs` (or the #1309 successor file)
- `crates/source/src/lib.rs`, `crates/source/tests/randomized.rs`,
  `crates/source/tests/seek_schedule_model.rs`

## Non-goals

- The browser's `seek_at` export (stream H, the rewritten #1293), which D2 makes safe to write.
  The graph trait's signature. Discarding behind-position blocks.

## Objective gates

1. **Diagnostic.** In `ffi.rs` tests: after a refused call that set another diagnostic, each of
   the four malformed-ID forms on `seek`, `seek_at`, `submit` and `source_seek_report` returns
   `INVALID_ARGUMENT` and `last_error` reads `source.id.invalid`.
2. **Held preparation.** Source unit test: anchor a seek in the future, `prepare_seek` with its
   generation and frame returns `Held`; through the driver, `prepare_source_seek` returns `true`
   and the next block's `source_generation_changed` is false until the anchor block, where it is
   true.
3. **Assertion fires.** A `#[should_panic]` debug-only unit test drives a consumer with a block
   clock off the quantum grid past a held anchor.
4. **Timed reads.** `cargo test --locked -p source` passes with `begin_block` and `read_block`
   unavailable outside `cfg(test)`; `cargo build --locked -p source` has no untimed public read.
5. Commands:
   - `cargo test --locked -p source` and `cargo test --locked -p capi`
   - `cargo test --locked -p host-core --features host-core/test-support`
   - `bash scripts/check-realtime-policy.sh`, `bash scripts/check-cross-targets.sh`
   - `bash scripts/check-workspace-policy.sh`, `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`

## Test value

- Gate 1: red if any malformed-ID branch returns before setting the diagnostic; the existing test
  checks the code only.
- Gate 2: red if a held seek reports `NotReady`/`false` (the D4 trap) or flags the generation
  change a block early.
- Gate 3: red if the assertion is removed or computed on the wrong clock.
- Gate 4 is a build property, not a new test; the moved integration tests keep their claims.

## Dependencies

- *Anchor every seek on the plan's source-read clock* (#1316): the `source_seek_report` entry point
  D1 covers, and the source-read clock that D4's `read_block_at` takes.
