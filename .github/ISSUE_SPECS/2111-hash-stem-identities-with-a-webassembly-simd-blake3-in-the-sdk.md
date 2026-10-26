# Hash stem identities with a WebAssembly SIMD BLAKE3 in the SDK

Filed by root from the console sprint (2026-10-26). The console opens a folder of 16 stems and waits 15-20 s before it can play: every stem's `blake3:` content identity is hashed in pure JavaScript.

## Measured (node, this box, 64 MiB buffer)
- `hosts/host-web/web/stem-store/incremental-blake3.js` (published by #2102 as `IncrementalBlake3`/`blake3Stream`): about 29 MB/s.
- `@noble/hashes` blake3: about 49 MB/s.
- Sixteen 3-minute 24-bit stereo stems are about 830 MB.

## Product outcome
`blake3Stream` and `IncrementalBlake3` in the SDK hash at memory-bandwidth speed: a WebAssembly SIMD (simd128) BLAKE3 at roughly 1 GB/s or more per core, with the same API and the same digests. They work in a Worker, so an app can hash one stem per worker in parallel. The stem store's verify-on-open path also gets faster.

## Smallest slice
- A small Wasm module built from the `blake3` crate (or an equivalent audited implementation) with simd128. It is separate from the engine module, so the engine artifact pin does not change. It exposes an incremental update and finalize over a caller-owned buffer.
- The JS wrapper keeps the exact API of `IncrementalBlake3`/`blake3Stream`. If the module cannot load, it falls back to the existing JS implementation.
- Staged into the SDK package beside the other stem files, with provenance.

## Gates
- The official BLAKE3 test vectors and the stem corpus identities (`VECTORS.tsv`): equal digests in Wasm and JS, at hostile chunk boundaries.
- A descriptive throughput figure (one run, two rounds) against the JS implementation, recorded in the spec.
- The tarball smoke covers the new module.
- AGENTS.md: browser Wasm builds on the pinned browser toolchain; no target-specific code.

## Decision record (attempt 1)
- D1 Crate `hosts/blake3-web` (cdylib, lib `blake3_web`), `blake3 = "=1.8.7"`, default features off, `pure` + `wasm32_simd`. Built by one more cargo line in `scripts/build-web-audioworklet.sh` on the `[browser-artifact]` nightly with `-Zbuild-std` and `+simd128`, in delivery and `--check-pin` modes only; shipped in the closure as `miso-engine-v1-blake3.simd128.wasm` (44,861 B, `name` section stripped). The worklet module's bytes are unchanged (cmp against a base build of `e4421343c`).
- D2 Exports (contract identity, V1): `miso_engine_blake3_v1_{new, input_capacity, input, update, finalize, free}`. Deviation from the scope note: `update(h, len)` hashes the first `len` bytes of the hasher's own 256 KiB staging buffer, and `finalize(h)` writes the digest to the start of that buffer, so no export takes a data pointer and a bad length is refused (returns 0), not a stray read.
- D3 JS: `IncrementalBlake3` and `blake3Stream` keep their API; `prepareBlake3({ url? }) -> Promise<"simd128" | "javascript">` is the one addition. One load per URL per realm; a hasher takes the module when one has loaded, else the JS class; a load failure resolves `"javascript"` and never rejects. `blake3Stream` and `OpfsStemStore.open()` prepare. Undigested hashers are freed by a `FinalizationRegistry`.
- D4 The JS fallback allocates nothing per block or per chunk (coordinator addition): module scratch arrays, a precomputed round schedule, number counters, a fixed tree stack.
- D5 Provenance: `incremental-blake3.provenance.json` keeps its shape; its `implementation`/`reason` name the module. The crate version, features and checksum are in Cargo.toml/Cargo.lock, the toolchain in the module's `producers` section, so no sibling record duplicates them. A release fingerprint for the module is a successor issue (root).

## Evidence (attempt 1)
Descriptive, one invocation, one warmup, two rounds, 64 MiB per worker in 1 MiB updates; equal digests across every engine and round.

| harness | engine | 1 worker (MB/s) | 8 workers, per worker / aggregate (MB/s) |
|---|---|---|---|
| Chromium 151 (headless) | JS before | 24.4, 24.3 | 6.5 / 51.0, 6.5 / 51.5 |
| Chromium 151 | JS after D4 | 66.2, 65.6 | 66.1 / 518, 66.1 / 525 |
| Chromium 151 | Wasm simd128 | 1,488, 1,462 | 1,557 / 11,957, 1,529 / 11,571 |
| node 22 | JS before | 25.1, 25.2 | 25.7, 24.6 per worker |
| node 22 | JS after D4 | 56.5, 60.0 | 60.5, 60.9 per worker |
| node 22 | Wasm simd128 | 1,471, 1,432 | 1,393, 1,529 per worker |

Chromium reproduces the console's 8-worker collapse of the old JS class (allocator contention); node does not. Aggregate is total bytes over the slowest worker's hash time. Harness: scratch `b3bench/` (not committed).
- D6 (root ruling) The package ships the module once: the closure's digested copy in `dist/assets`. `prepareBlake3`'s default URL is `../../assets/miso-engine-v1-blake3.simd128.wasm` from `dist/browser/stems/incremental-blake3.js`; the tarball smoke asserts the module is packed once and that the default URL fetches exactly that file.
- D7 (root ruling) Paths added beyond the scope note: `scripts/check-web-audioworklet.sh`, `hosts/host-web/qualification/server.mjs`, `sdk/test/boot-evals.mjs`, `hosts/host-web/DEPLOYMENT.md` (the closure is seven files), `docs/REALTIME_DEPENDENCY_POLICY.md` (unsafe owner), `scripts/check-sdk-headless.sh` (runs `stem-store-hash-v1.mjs` with the artifacts in its one `node --test`, so CI runs the Wasm half) with `sdk/test/headless-path-evals.mjs` (its fake node pins that argv), and the source redirect `sdk/src/browser/stems/incremental-blake3.js`.
