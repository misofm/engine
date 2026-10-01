**Purpose**: a standing check-in issue. The engine ships the **unfused** multiply-add contract everywhere (#163 phase 2, owner-approved 2026-08-26) because wasm has no deterministic hardware FMA — the fused semantics cost ~54 emulated instructions per multiply-add and made the browser 10.65× native at session level (measured: 64-track console 969.6 µs/block fused-emulated → ~175 µs unfused+interleaved). The day wasm gets deterministic hardware FMA, fused becomes optimal on **every** platform and should be re-adopted across the stack.

## What to check (routinely — quarterly is fine)

- [ ] **Relaxed SIMD deterministic profile exposed to web content?** `f32x4.relaxed_madd` is real hardware FMA; the spec's *deterministic profile* pins it to the fused single-rounding result. Watch for Chrome AND Safari exposing that profile to web pages. Check: [WebAssembly/proposals phase table](https://github.com/WebAssembly/proposals), Chrome Platform Status, WebKit feature status.
- [ ] **Any new spec'd deterministic FMA instruction** entering the proposals list at phase 3+.
- [ ] **Flexible vectors (>128-bit wasm SIMD) at phase 3+** — changes the width story (wasm Simd8) and the `Lane::SVF_CASCADE_DEPTH` tuning, independent of FMA semantics.

## What re-adoption takes (deliberately kept cheap)

All multiply-add dispatch lives at **one point**: `crates/miso-engine-lane/src/wide_impl.rs` (+ the scalar impl). The per-site audit (`docs/rulings/unfused-multiply-add-audit.md`, 19 sites / 6 families, all ruled incidental with the pole-invariance proof) means no site needs individual re-analysis — flipping back is:
1. one dispatch-point change (fused on every backend, including the wasm deterministic op),
2. one §8 re-pin cycle with byte accounting (the same machinery phase 2 exercised; ~21 target groups),
3. cross-backend `to_bits` identity re-verified (wasm gates all legs), listening/audit suites at threshold,
4. paired measurement on both console arms — expected: native recovers its ~4.5% fused advantage, wasm keeps hardware speed.

**Do not** adopt `relaxed_madd` outside a deterministic profile — implementation-defined fusion breaks cross-backend bit identity (TARGET_MATRIX.md / spec 024's standing ban).
