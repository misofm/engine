# Retire the scalar-Wasm CI legs together with the "every other target" code arms

Draft, not a GitHub issue. From the 2026-09-28 test-value audit
([`../TEST-VALUE-AUDIT.md`](../TEST-VALUE-AUDIT.md) §5 item 5 and §7). Base `a9414c0c`. **Needs owner
ruling R5.** It applies "modes production never needs should not be kept or tested" and "no
target-specific code" to the scalar Wasm build.

## Problem

Only the `simd128` AudioWorklet artifact ships:
- `scripts/build-web-audioworklet.sh:36-40` records owner decision W4-D1, one artifact, and its only
  build is `+simd128` (`:67-71`);
- the shipped host refuses a module whose backend row is not `simd128`
  (`hosts/host-web/web/miso-engine-v1-audio-worklet-host.js:12-16`, `:61-64`), so a scalar build
  cannot even boot.

CI still builds and gates a scalar (`-simd128`) Wasm tree. From PR #1016's logs and
`../data/script-gates-verify.md` §7:

| leg | where | cost |
|---|---|---:|
| 18-package `-simd128` release build; no later step reads it | `.github/workflows/qualification.yml:766-769` | 38 s |
| atomics check over a fresh scalar non-LTO build ("browser-local fallback artifact") | `qualification.yml:772-775`, `scripts/check-wasm-realtime-atomics.sh` | 35 s |
| scalar variant of the protocol golden parity | `scripts/check-protocol-wasm-parity.sh:201` | about 9 s |
| scalar G5 guest | `scripts/run-wasm-gates.sh:201` | about 35 s |
| scalar mode of the cross-target checks | `scripts/check-cross-targets.sh:65-114` | share of 43 s |

The shipped module is already checked for no atomics, no imports and no shared memory, on the real
artifact (`scripts/check-web-audioworklet.sh:309-341`, self-tested at `:82-109`).

The only code these legs cover is the arms that select `Scalar` as the *native* backend when a target
is neither x86-64-v3, AArch64 nor `wasm32+simd128`:
- `crates/lane/src/backend.rs:59-67`;
- `crates/lane/src/wide_impl.rs:290-297`, `:314-321`;
- `crates/graph/src/runtime.rs:328-335`;
- `crates/soft-clip/src/lib.rs:903-911`;
- `crates/target-smoke/src/lib.rs:74-83`;
- `hosts/host-web/src/lib.rs:7260-7265`.

AArch64 (mobile, NEON) is in the SIMD set and is **not** affected. The `Scalar` lane type stays: tests
use it as the oracle width everywhere.

## Outcome

- **The unsupported-target arms become compile errors.** A target outside {x86-64-v3, AArch64,
  `wasm32+simd128`} fails to compile with a clear `compile_error!`, the way `lane` already refuses a
  sub-v3 x86 build. The arms are removed.
- **The scalar legs are removed** from `qualification.yml`, `check-protocol-wasm-parity.sh`,
  `run-wasm-gates.sh` and `check-cross-targets.sh`. `check-wasm-realtime-atomics.sh` and its test
  are deleted.
- **The `simd128` legs and the shipped-artifact checks are unchanged.**

## Scope

Authorized paths:
- the six code arms above;
- `crates/lane/src/lib.rs` (the guard);
- `.github/workflows/qualification.yml`;
- `scripts/check-protocol-wasm-parity.sh`, `scripts/run-wasm-gates.sh`,
  `scripts/check-cross-targets.sh`;
- `scripts/check-wasm-realtime-atomics.sh` and `scripts/test-wasm-realtime-atomics.sh` (deleted);
- `docs/TARGET_MATRIX.md`;
- this issue's spec.

## Gates

1. **Unsupported targets are refused.**
   - `cargo check --target wasm32-unknown-unknown -p lane` with `-C target-feature=-simd128` fails
     with the new `compile_error!`;
   - `cargo check --target wasm32-unknown-unknown -p lane` with `+simd128` passes;
   - the x86-64-v3 build and the existing sub-v3 refusal are unchanged.
2. **The shipped-artifact checks still discriminate.** `check-web-audioworklet.sh`'s atomics
   self-test (`:82-109`) still fails on a seeded `i32.atomic.load`, and an injected import still
   fails the export/import gate.
3. **Class A is unchanged.** `run-wasm-gates.sh`'s native and `simd128` legs pass with the same pins,
   and `check-protocol-wasm-parity.sh`'s `simd128` variant passes.
4. **Historical bugs.** Unaffected; none involves the scalar Wasm build. Run `../tools/revert.py`
   for the record.
5. **Cost.** `wasm-guests` is at least 100 s shorter on a full-route PR.

## Saving and risk

- **Saving:** about 117 s of runner time per full PR, plus the code arms.
- **Risk:** a future non-SIMD target would need the arms back. The compile error makes that an
  explicit decision rather than a silent fallback.
