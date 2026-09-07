- **Smallest existing production seam: linked `host-web` Wasm.** Its boot path uses `host-core` → `launch_native_effect_registry()` → `GateExpanderFactory`, retaining actual `PreparedGate` processing. This avoids new callsite wiring. Existing build precedents are [qualification.yml:613](/home/bl/misofm/engine-gate-detector-access/.github/workflows/qualification.yml:613) and [build-web-audioworklet.sh:60](/home/bl/misofm/engine-gate-detector-access/scripts/build-web-audioworklet.sh:60).

  Minimal isolated capture commands, **not executed**:

  ```bash
  CARGO_TARGET_DIR=/tmp/issue534-web-scalar RUSTFLAGS='-C target-feature=-simd128' \
    cargo build --locked --release --target wasm32-unknown-unknown -p host-web
  CARGO_TARGET_DIR=/tmp/issue534-web-simd128 RUSTFLAGS='-C target-feature=+simd128' \
    cargo build --locked --release --target wasm32-unknown-unknown -p host-web
  ```

  For each resulting `wasm32-unknown-unknown/release/host_web.wasm`, use `wasm-objdump -x MODULE` and `wasm-objdump -d MODULE`. Preserve compiler/decoder status, toolchain, module hash and complete attributed function blocks as in issue475. These captures retain release fat LTO and debug information; they are not published artifact identities.

- **Symbol attribution at checkpoint `65b58a4c`:** locate `PreparedGate<f32,false/true>::process` and, for supported simd128 banking, `PreparedGate<Simd4,false>::process_bank`; follow emitted `run_block` or inlined segment bodies. Production `run_segment` classifies metadata/taps at `lib.rs:725`, then calls `gate_block_with_access`. Inspect conditional access there; standalone helper symbols are unnecessary. Scalar-Wasm bank admission and emitted W8 symbols must not be credited as supported bank execution.

- **Issue475’s guest workflow alone exposes fallback for this gate.** `wasm-gate-guest` → `digest_gate_expander` → `corpus::run_case` calls public `gate_block`, which hardcodes `DetectorAccess::LinkedUnequal`. Successful scalar/simd128 guest decoding therefore proves fallback lowering, not optimized `PreparedGate` routes. Direct effect `--emit=obj` is also not a reliable substitute: issue475 retained LLVM-bitcode bad-magic failures.

- **Remaining seam is decoded caller attribution, not source wiring.** Source establishes the existing host route; no candidate host module was built or decoded here. If its optimized callers cannot be mapped, report that precise limitation. The delivery wrapper additionally enforces an artifact pin and deletes temporary output, so it is unsuitable for retaining this inspection capture unchanged.

Read-only planning only; no transient fixture reliance or acceptance verdict.