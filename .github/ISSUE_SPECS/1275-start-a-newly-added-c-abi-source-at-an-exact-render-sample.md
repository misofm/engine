# Start a newly added C ABI source at an exact render sample

Slice 6 of *Swap a rebuilt plan without an audio gap* (#1269). Code anchors verified on
`54b0a1bf8` (unchanged at `24029badb`).

## Product outcome

A phone app that adds a new stem to a playing session through the C ABI can start that stem in
exact time with the stems already playing: one new call, `miso_engine_v1_source_seek_at`, exports
the anchored seek of *Hold an anchored source seek until its render sample* (#1274).

## Context

- `miso_engine_v1_source_seek` (`crates/capi/src/ffi.rs:522`; header
  `crates/capi/include/miso_engine_v1.h:237`) calls `SessionState::seek`
  (`crates/capi/src/runtime/control.rs:973`), which *Keep sources playing across a C ABI structural
  transaction* (#1273) routes to the newest committed session, so an added source can be seeked
  before the swap.
- Capability bits: `MISO_ENGINE_V1_FEATURE_SOURCE_SEEK` is 4 and `MISO_ENGINE_V1_FEATURE_MASK` is 31
  (header); the report is built in `crates/capi/src/ffi.rs` (`miso_engine_v1_query_capabilities`,
  `:202`) from `crates/capi/src/abi.rs`.
- `scripts/check-capi-abi.sh` pins the exported symbol set and has a self-test;
  `docs/C_ABI_V1_QUALIFICATION.md` records the symbol count. `crates/capi/tests/c/` is the C
  consumer.
- Earlier in-place V1 amendments (the `maximum_submixes` and `maximum_vcas` words) changed no
  `ABI_VERSION` (`docs/C_ABI_V1_QUALIFICATION.md`).

## Decisions frozen for this slice

- **D1. Export.** `uint32_t miso_engine_v1_source_seek_at(miso_engine_v1_session *session, const
  uint8_t *source_id, uint64_t source_id_bytes, uint64_t generation, uint64_t source_frame, uint64_t
  anchor_sample)`: same thread rule, argument checks and result codes as
  `miso_engine_v1_source_seek`, plus `source.seek.anchor_unaligned`.
- **D2. Capability.** `MISO_ENGINE_V1_FEATURE_SOURCE_SEEK_AT = 32`; the mask becomes 63. A host
  checks the bit before calling the symbol.
- **D3. Amendment.** In-place V1 amendment: no struct size and no `ABI_VERSION` change. Record it in
  `docs/C_ABI_V1_QUALIFICATION.md` like the earlier amendments, with the new symbol count.
- **D4. Guidance.** The header explains how to start an added stem: after the transaction returns,
  pick `A` a few quanta past the last rendered block, call `seek_at(id, g, F, A)` with `F` the frame
  the playing stems read at `A`, then submit generation `g` from `F`.

## Deliverables

1. D1-D4 in `crates/capi/src/ffi.rs`, `abi.rs`, `lib.rs`, `runtime/control.rs`, the header and the
   docs.
2. `scripts/check-capi-abi.sh`: the frozen symbol set gains the symbol; its self-test still rejects a
   definition replaced by an undefined reference.
3. The C consumer in `crates/capi/tests/c/` checks the bit and calls the symbol once.
4. Tests (below).

## Authorized paths

- `crates/capi/src/ffi.rs`, `abi.rs`, `lib.rs`, `runtime/control.rs`, `runtime/tests.rs`
- `crates/capi/include/miso_engine_v1.h`, `crates/capi/tests/c/`
- `scripts/check-capi-abi.sh`
- `docs/C_ABI_V1_QUALIFICATION.md`

## Non-goals

- No change to source semantics (slice 5 owns them). No browser export (B4).

## Objective gates

1. **Gap-free acceptance, through the exported entry points.** Session A: one track on `s1`, no
   stateful DSP. Render 6 blocks feeding `s1` from frame 0. Apply `UpsertSource s2`, `UpsertTrack t2`
   (on `s2`) and its route. Call `miso_engine_v1_source_seek_at(s2, 2, A, A)` with `A` two blocks past
   the next render, submit `s2` from frame `A` at generation 2, render 10 more. Reference: the
   post-edit session compiled fresh, `s1` from frame 0, `s2` zeros for frames `< A` and the same PCM
   from `A`. All 16 blocks bit-identical.
2. **Capabilities.** `miso_engine_v1_query_capabilities` reports bit 32 and mask 63.
3. **Refusals.** Unaligned anchor, stale generation and an unknown source each return their result
   and diagnostic.
4. Commands:
   - `cargo test --locked -p capi`
   - `cargo build --locked --release -p audit -p capi && ./target/release/audit capi`
   - `bash scripts/check-capi-abi.sh` and `bash scripts/check-capi-abi.sh --self-test`
   - the umbrella's inherited gates.

## Test value

- Gate 1: an export that forwards to the plain seek (observation timing) starts the stem on the wrong
  block; it turns red.
- Gate 2: a symbol without its capability bit leaves hosts unable to detect it; it turns red.

## Dependencies

- *Keep sources playing across a C ABI structural transaction* (#1273).
- *Hold an anchored source seek until its render sample* (#1274).
