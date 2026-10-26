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
