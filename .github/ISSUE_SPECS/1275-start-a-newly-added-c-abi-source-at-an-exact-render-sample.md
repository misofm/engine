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

## Attempt record

### Attempt 1 (implementer)

**What changed.**

- D1: `miso_engine_v1_source_seek_at` in `crates/capi/src/ffi.rs`. The two seek exports share one
  private body (`source_seek_entry`), so the argument checks, result codes, thread rule and
  `last_error` handling are the plain seek's. `SessionState::seek_at`
  (`runtime/control.rs`) mirrors `SessionState::seek`: it synchronizes epochs and forwards to
  `SourceControlSet::seek_at` on the newest committed providers.
- D2: `FEATURE_SOURCE_SEEK_AT = 1 << 5` in `abi.rs`, and the mask is now `0x3f`. The header
  defines `MISO_ENGINE_V1_FEATURE_SOURCE_SEEK_AT` as 32 and the mask as 63.
- D3: `docs/C_ABI_V1_QUALIFICATION.md` records the in-place amendment and the new symbol count of
  15. No struct, size or `ABI_VERSION` changed.
- D4: the header explains how to start an added stem: pick `A`, call `seek_at(id, g, F, A)`,
  then submit `g` from `F`.
- `lib.rs` needed no change, because it already re-exports `ffi::*` and `abi::*`.
- `scripts/check-capi-abi.sh`: the frozen set gains the symbol. A new self-test case,
  `seek-at-undefined-reference`, lists the real definitions without the new symbol (as a
  defined-only listing would show it if the symbol were only an undefined reference), and the
  checker rejects it.
- `crates/capi/tests/c/abi_smoke.c`: static asserts for bit 32 and mask 63, a signature pointer,
  and one call to the new symbol after checking the bit. The call uses a null session and expects
  `MISO_ENGINE_V1_INVALID_ARGUMENT`.
- The #1273 attempt-1 MINORs are closed here too. See that spec's "Follow-ups after attempt 1
  PASS".

**Tests, with test value.**

- Gate 1, `runtime::tests::an_added_c_abi_source_starts_at_its_anchored_render_sample`, at 48 and
  96 kHz, entirely through the exported entries.
  - Steps: 6 blocks on `s1`, then the `UpsertSource`/`UpsertTrack`/`UpsertRoute` transaction.
    Then `seek_at(s2, 2, 1024, 1024)` (anchor block 8, two blocks past the next render, which is
    also the swap block). `s2` generation 2 is submitted from frame 1024, and 10 more blocks
    render.
  - All 16 blocks are bit-identical to the committed snapshot compiled fresh, fed `s1` from
    frame 0 and `s2` zeros below 1024, then the same PCM.
  - Test value: it goes red if the export forwards to the plain seek (M1), or if `seek_at`
    addresses the running plan's set instead of the newest one (M5).
- Gate 2, `ffi::tests::version_and_capabilities_are_exact`: the mask is now `0x3f` and bit 32 is
  set. `abi::tests::masks_and_result_codes_are_frozen` pins bit 32 and mask `0x3f`.
  - Test value: it goes red if the bit is left out of the mask (M2).
- Gate 3, `ffi::tests::anchored_seek_refusals_reach_the_c_host_as_their_own_diagnostic`.
  - Results checked: a null session gives `INVALID_ARGUMENT` and an engine handle gives
    `WRONG_HANDLE`.
  - Each refusal comes back as `INVALID_ARGUMENT` with its own `last_error` string: anchor 129
    gives `source.seek.anchor_unaligned`, an absent source gives `source.id.unknown`, and a second
    generation-2 seek gives `source.generation.stale`. An accepted seek clears the diagnostic.
  - Test value: it goes red if the export drops its anchor (M1: the unaligned anchor is accepted).

**Mutations.** Each was applied, run, and then restored from a backup.

| Mutation | Result |
|---|---|
| M1: `seek_at` export calls the plain `seek` | gate 1 red at "added stem block 8"; gate 3 red (`left: 0`, `right: 1`) |
| M2: `FEATURE_SOURCE_SEEK_AT` left out of `FEATURE_MASK` | `version_and_capabilities_are_exact` red (31 vs 63); `masks_and_result_codes_are_frozen` red |
| M5: `SessionState::seek_at` uses `self.providers` | gate 1 red: the seek returns 1 (`source.id.unknown`) before the swap |
| V10 (#1273 MINOR 1): `committed` = prospective model | `structural_command_keeps_…` red: `NonContiguous { expected: 256, actual: 0 }` |
| V4 (#1273 MINOR 2): `seek` uses `self.providers` | `removing_a_track_…` red; `structural_command_keeps_…` green, so the new assertion is the only catch |

**Gates.** All passed locally on x86-64.

| Gate | Result |
|---|---|
| `cargo fmt --all -- --check` | exit 0 |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | exit 0 |
| `RUSTDOCFLAGS=-Dwarnings cargo doc --locked --workspace --no-deps` | exit 0 |
| workspace policy check and its mutation test | ok / ok |
| realtime policy check and its mutation test | ok (58 regions in 16 files) / ok |
| `cargo test --locked -p capi` | lib 40, `plan_swap_race` 2, `resource_lifecycle` 9: all pass |
| `cargo test --locked -p audit` | 33 pass |
| `cargo build --locked --release -p audit -p capi && ./target/release/audit capi` | 100,000 calls, every violation counter 0 |
| `check-capi-abi.sh` | ok (shared and static) |
| `check-capi-abi.sh --self-test` | ok, including `seek-at-undefined-reference` |
| `check-cross-targets.sh` | PASS; only the expected #1018 iOS `memset_pattern16` rows fail |

- `audit capi`'s `pcm_digest` is `c60671f6593fa603`. The audit makes no seek, and no live gate
  pins the digest.
- No crate compiled into the browser Wasm module changed (capi only), so the worklet chain was
  not run and the artifact is unchanged.
- `Simd4` runs only in CI's aarch64 legs. Like #1273, the slice adds no bank code.
