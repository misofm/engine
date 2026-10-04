# Export session replacement from the browser engine module

Slice B4 of *Swap a rebuilt plan without an audio gap* (#1269). Code anchors verified on
`54b0a1bf8` (unchanged at `24029badb`).

## Product outcome

The shipped AudioWorklet module exports what a worklet needs to replace the running session and to
start an added stem in time: stage a replacement document while the engine is live, run the
replacement, read its result and diagnostic, and seek a source at an anchored render sample. Every
published mirror of the export list (ABI layout, SDK, checkers) follows.

## Context

- `miso_engine_web_v1_document_ptr` refuses while a host is live (`hosts/host-web/src/ffi.rs:3500-3511`),
  so a replacement needs its own staging. Boot reports through `miso_engine_web_v1_boot_result` and
  `miso_engine_web_v1_boot_diagnostic_bytes` (`ffi.rs:3693`, `:3703`); a live host reports through its
  status and its diagnostic (`AudioWorkletEngineHost::diagnostic`, `lib.rs:2868`).
- `miso_engine_web_v1_source_seek` (`ffi.rs:3789`) takes the source ID through the staging buffer and
  its byte count.
- The export list is published and checked in several places: `scripts/check-web-audioworklet.sh`
  (`expected_exports`, `:203`), `tools/parameter-metadata/src/abi_layout.rs` (`EXPORTS`, `:135`),
  `scripts/check-abi-layout-v1.py` (`:191`, and its self-test fixture),
  `sdk/assets/miso-engine-v1-abi-layout.json`, `sdk/src/generated/abi.ts`, and
  `scripts/check-sdk-generated.sh`.
- `AudioWorkletEngineHost::replace_session` comes from *Replace the running browser session in the
  Rust host* (#1290) and *Keep browser live effect edits across a session replacement* (#1292);
  `SourceControlSet::seek_at` from *Hold an anchored source seek until its render sample* (#1274).

## Decisions frozen for this slice

- **D1. Staging.** `miso_engine_web_v1_replace_document_ptr(handle, len) -> u32` returns a pointer to a
  staging buffer for one replacement document of `len` bytes (bounded by `MAXIMUM_DOCUMENT_BYTES`),
  or 0 with a typed result. The buffer is allocated on that call, on the control path.
- **D2. Replace.** `miso_engine_web_v1_replace(handle, len) -> u32` runs `replace_session` on the
  staged bytes and frees the staging. On refusal the host's diagnostic holds the reason, read through
  the existing live-host diagnostic path; the sticky state is unchanged.
- **D3. Grown buffers.** If the new document needs larger bridge buffers (more source channels,
  longer IDs), they are reallocated. The worklet re-reads every pointer and view after every
  successful replacement (B5), so no status bit is added and `WebStatus` is unchanged.
- **D4. Anchored seek.** `miso_engine_web_v1_source_seek_at(handle, source_id_bytes, generation,
  source_frame, anchor_sample) -> u32`, the source ID through the existing staging, mirroring
  `miso_engine_web_v1_source_seek`.
- **D5. Mirrors.** Add the three exports to every list in the context, regenerate the SDK mirrors
  with the repository's generators, and keep every checker's self-test passing.

## Deliverables

1. D1-D4 in `hosts/host-web/src/ffi.rs` and `lib.rs`.
2. D5 in the checkers, the parameter-metadata generator and the generated SDK files.
3. Native tests of the three exports in `hosts/host-web/src/tests.rs`.

## Authorized paths

- `hosts/host-web/src/ffi.rs`, `lib.rs`, `tests.rs`
- `scripts/check-web-audioworklet.sh` (export list), `scripts/check-abi-layout-v1.py` and its fixture
- `tools/parameter-metadata/src/abi_layout.rs`
- `sdk/assets/miso-engine-v1-abi-layout.json`, `sdk/src/generated/abi.ts`, `scripts/check-sdk-generated.sh`

## Non-goals

- No worklet JavaScript (B5), no SDK API (B7-B8).

## Objective gates

1. **Exports work natively.** Through the `extern "C"` functions in a native test: stage a document,
   replace, render, and get gate 1 of B2's equality; a refused replacement leaves a readable
   diagnostic; `source_seek_at` aligns an added source as in slice 5's gate 1.
2. **Mirrors agree.** `python3 -B scripts/check-abi-layout-v1.py` and its self-test,
   `bash scripts/check-sdk-generated.sh`, and the parameter-metadata `--check` pass with the new
   exports, and each fails if one list omits one of them.
3. **Artifact.** The shipped module builds; `scripts/check-web-audioworklet.sh` passes (exports, no
   shared memory, no atomics, the render call graph reaches no allocator).
4. Commands:
   - `cargo test --locked -p host-web --features host-web/test-support`
   - `rm -rf target/ci/b4 && mkdir -p target/ci/b4/a target/ci/b4/n && bash scripts/build-web-audioworklet.sh --named-twin target/ci/b4/n target/ci/b4/a && bash scripts/check-web-audioworklet.sh target/ci/b4/a target/ci/b4/n/miso-engine-v1-audio-worklet.simd128.named.wasm`
   - `bash scripts/check-sdk-generated.sh` and `python3 -B scripts/check-abi-layout-v1.py`
   - the umbrella's inherited gates. The shipped artifact changes: report it.

## Test value

- Gate 1: a replacement staged through `document_ptr` (refused while live) or a refusal with no
  diagnostic turns it red.
- Gate 2: a mirror that misses one export would let the SDK call a name the module lacks; the
  checkers turn red.

## Dependencies

- *Keep browser live effect edits across a session replacement* (#1292).
- *Hold an anchored source seek until its render sample* (#1274).
