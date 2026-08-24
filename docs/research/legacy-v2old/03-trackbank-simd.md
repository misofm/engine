<!--
Provenance: copied from misofm/engine-v2-old docs/research/03-trackbank-simd.md on 2026-08-24 for issue #144 item 8.
Legacy research archive only; current Engine V2 contracts and rulings remain authoritative.
-->

# TrackBank SIMD

The logical lane type is pinned to `wide = "=1.6.1"` / `wide::f64x4`, crates.io SHA-256 `de2aaf408e58689c2096682331b1f42bb2d9f2ed6b11560407d023cd0a6c634e`, tag `v1.6.1` commit `7eadea9f76195a5b22bd3469dcf9c8b3a4e49d79`. Cargo.lock checksum, vendored source, and source-shape audit are acceptance evidence. The native release is a separately built pinned AVX2 artifact, using Rust target-feature rules ([Rust codegen attributes](https://doc.rust-lang.org/stable/reference/attributes/codegen.html)); it attests ISA/build at startup and fails fast on an unsupported machine. There is no runtime dispatch or in-binary scalar production fallback. A scalar implementation is an oracle/test build or separately named portable artifact. Browser WASM SIMD represents the logical operation as two physical f64x2 registers. No contract, session JSON, or plugin descriptor exposes that geometry.

TrackBank partitions tracks into mono, stereo, and dual-mono execution cohorts. After input built-ins each cohort presents left/right planes: mono duplicates or routes according to the documented input policy; stereo preserves paired channels; dual-mono preserves independent controls. Padding is private and masked; only logical tracks may contribute to output.

Storage uses flat two-plane SoA/AoSoA blocks addressed as `(cohort, plane, bank, frame)`. A bank has four logical tracks. Parameter slabs and recursive state use the same bank ordering so a rack walks contiguous samples and controls. The mounted plan supplies dense ordinal → cohort/bank/lane mapping; public immutable IDs are resolved before mount and never traversed in RT.

Avoid contributor-wide horizontal reductions. Bus summation walks stable source ordinals and applies one scalar/lane add at a time. This preserves deterministic fold order while retaining across-track SIMD. The [wide project](https://github.com/Lokathor/wide) is a pinned implementation dependency, not an API commitment.
