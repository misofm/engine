# Retire the scalar-Wasm CI legs after the AArch64 leg exists, with the three arms only they compile

Draft, not a GitHub issue. From the 2026-09-28 test-value audit
([`../TEST-VALUE-AUDIT.md`](../TEST-VALUE-AUDIT.md) §5 item 5 and §7). Base `a9414c0c`. Paths
starting `../` are relative to the audit's handoff folder.

**Needs owner ruling R5, and depends on issue 12.** R5 applies "modes production never needs should
not be kept or tested" and "no target-specific code" to the scalar Wasm build. The dependency on
issue 12 is explained under the problem.

## Problem

Only the `simd128` AudioWorklet artifact ships:
- `scripts/build-web-audioworklet.sh:36-40` records owner decision W4-D1, one artifact, and its only
  build is `+simd128` (`:67-71`);
- the shipped host refuses a module whose backend row is not `simd128`
  (`hosts/host-web/web/miso-engine-v1-audio-worklet-host.js:61-64`).

CI still builds and gates a scalar (`-simd128`) Wasm tree. From PR #1016's logs and
`../data/script-gates-verify.md` §7:

| leg | where | cost |
|---|---|---:|
| 18-package `-simd128` release build, whose target dir the evidence-crate check's scalar line at `:784` reuses | `.github/workflows/qualification.yml:766-769`, `:782-785` | 38 s |
| atomics check over a fresh scalar non-LTO build ("browser-local fallback artifact") | `qualification.yml:772-775`, `scripts/check-wasm-realtime-atomics.sh` | 35 s |
| scalar variant of the protocol golden parity | `scripts/check-protocol-wasm-parity.sh:201` | about 9 s |
| scalar G5 guest | `scripts/run-wasm-gates.sh:201` | about 35 s |
| scalar mode of the cross-target checks, the effect package and descriptor corpora | `scripts/check-cross-targets.sh:65-114`, `scripts/check-effect-package-v1.sh:36`, `scripts/check-effect-descriptor-v1.sh:28`; `scripts/check-effect-interchange-qualification.sh:207` greps `check-cross-targets.sh` for `feature=-simd128` | share of 43 s + 34 s |

The shipped module is already checked for no atomics, no imports and no shared memory, on the real
artifact (`scripts/check-web-audioworklet.sh:309-341`, self-tested at `:82-109`).

**Which code only the scalar legs compile.** Three arms select `Scalar` as the native backend on a
target that is neither x86, AArch64 nor `wasm32+simd128`:
- `crates/lane/src/backend.rs:60-68`;
- `crates/graph/src/runtime.rs:328-334`;
- `crates/target-smoke/src/lib.rs:75-85`.

**Not** such arms:
- `crates/lane/src/wide_impl.rs:291-298` and `:315-322` are the portable `max`/`min`, taken by
  every target except x86 and `wasm32+simd128`, **including AArch64 (NEON)**. The comment at `:289-290`
  says NEON has no instruction with the D8 rule. **Today the scalar G5 guest is the only CI
  execution of the source that phones run for `max`/`min`**, the LANE-3 site. That is why this issue
  waits for issue 12.
- `hosts/host-web/src/lib.rs:7260-7265` (`BACKEND_SCALAR`) is the branch every native x86 build
  takes. `scripts/check-browser-expected-resources.py:274-278` relies on it.
- `crates/soft-clip/src/lib.rs:903-911` is a width predicate.

## Outcome (after issue 12's AArch64 job runs the lane G-gates and the G5 corpus)

- **The three arms become a `compile_error!`** for unsupported targets, the way `lane` already
  refuses a sub-v3 x86 build. `wide_impl.rs`'s portable arm stays; its `cfg` may be narrowed to
  AArch64. host-web's native branch stays.
- **The scalar legs are removed:**
  - the two workflow steps;
  - the scalar variants in `check-protocol-wasm-parity.sh`, `run-wasm-gates.sh`,
    `check-cross-targets.sh`, `check-effect-package-v1.sh` and `check-effect-descriptor-v1.sh`;
  - the `feature=-simd128` expectation in `check-effect-interchange-qualification.sh`;
  - `check-wasm-realtime-atomics.sh` and its test.
- **The evidence-crate check at `qualification.yml:782-785`** keeps only its existing `+simd128`
  line.

## Scope

Authorized paths:
- the three arms and `crates/lane/src/lib.rs` (the guard);
- `.github/workflows/qualification.yml`;
- the six scripts named above, and `scripts/check-wasm-realtime-atomics.sh` plus
  `scripts/test-wasm-realtime-atomics.sh` (deleted);
- `docs/TARGET_MATRIX.md`;
- this issue's spec.

## Gates

1. **The AArch64 leg is live first.** Issue 12's job runs `run-wasm-gates.sh --native` (or the G5
   Rust test) on `ubuntu-24.04-arm` and records a result.
2. **Unsupported targets are refused; supported ones build.**
   - `cargo check --target wasm32-unknown-unknown -p lane` with `-C target-feature=-simd128` fails
     with the new `compile_error!`;
   - the same check with `+simd128` passes;
   - the AArch64 build on issue 12's job passes;
   - x86-64-v3 and the sub-v3 refusal are unchanged.
3. **The shipped-artifact checks still discriminate.** `check-web-audioworklet.sh`'s atomics
   self-test (`:82-109`) still fails on a seeded `i32.atomic.load`, and an injected import still
   fails the export/import gate.
4. **Class A is unchanged.** `run-wasm-gates.sh`'s native and `simd128` legs pass with the same pins.
   So do `check-protocol-wasm-parity.sh`'s `simd128` variant and the effect package and descriptor
   corpora.
5. **Cost.** `wasm-guests` and `cross-target` together are at least 100 s shorter on a full-route
   PR.

## Saving and risk

- **Saving:** about 117 s of runner time per full PR, plus three code arms.
- **Risk:** done before issue 12, the portable `max`/`min` arm that AArch64 uses would lose its only
  CI execution. That is why gate 1 comes first.

## Amendments (Sol verification, 2026-09-28)

See [`../VERIFY-TEST-VALUE.md`](../VERIFY-TEST-VALUE.md). **These amendments supersede the body wherever they conflict.**

1. **Re-scope: this is the pending decision #1041 left open.** #1041's implementation
   (`4ef7c986`, branch `codex/1041-refuse-32-bit-targets`) adds the unsupported-target
   `compile_error!` this draft proposed. It keeps wasm32 *without* `simd128` as a named
   "scalar-wasm CI exception, a separate, pending decision", and lists the legs in
   `docs/TARGET_MATRIX.md`. So this draft no longer adds a guard. It:
   - deletes that marked exception arm and the now-unreachable `Backend::Scalar` selection arm in
     `lane/src/backend.rs`;
   - retires the legs.
2. **The legs shrink before this lands.** #1037 deletes `check-effect-package-v1.sh`,
   `check-effect-descriptor-v1.sh` and the interchange qualification, so those scalar legs go with
   it. #1038 edits the neighbouring `miso_wasm_simd8` arm in `backend.rs`. Land after #1037 and
   #1038.
3. **The AArch64 dependency is now #1017.** Gate 1 reads "#1017's job runs the lane G-gates and G5
   on AArch64 in the shipping profile". Issue 12 is merged into #1017.
4. **Relation to footprint R8.** R8 covers the whole-plan `Backend::Scalar` path *and* the scalar wasm
   CI build. This draft removes only the wasm build. `Backend::Scalar` and the per-node oracle stay,
   so R8's oracle question is untouched. Ask the owner to confirm that the two are separable.
5. **Measured saving.** The scalar build, the atomics check and the scalar G5 guest take a median
   88 s across 8 full-route runs (108 s on PR #1016's run). The cross-target scalar share cannot be
   split from its step, so gate 5's "at least 100 s" becomes "at least 80 s".
