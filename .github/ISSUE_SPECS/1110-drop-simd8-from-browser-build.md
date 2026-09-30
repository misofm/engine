# Drop the eight-lane kernels from the browser build

## Problem

The browser always runs four lanes: `lane::Backend::current()` is `Simd4` on wasm `simd128`
(`crates/lane/src/backend.rs`; #183 measured eight lanes on wasm as a null, and #1038 removed the
cfg that selected them). Yet the shipped module still carries the eight-lane (`Simd8`, `f32x8`)
instantiations of the kernels and banks, because the effect crates' `match backend` arms
instantiate every width on every target. That was about 135 KB of the module when measured on
2026-09-29, and it is code the browser can never execute.

The wasm gate guest also exercises all three widths (Scalar, Simd4 and Simd8) on wasm. The owner
asked (2026-09-29) why eight is checked on a target that never runs it. On wasm it should check
Simd4 against Scalar. Native x86 keeps its 1/4/8 checks.

Owner, 2026-09-30: do this "if it doesn't actually help run any of the browser code".

## Smallest closable slice

1. The eight-lane backend and every kernel, bank or dispatch arm that instantiates it compile only
   where eight lanes run: `x86_64`, plus `test-support` builds on native hosts if tests need them.
   On wasm32, `Backend::Simd8` and its arms are absent at compile time, not merely unreachable.
   This is the one allowed difference between targets: the lane width. The implementation shape
   stays one generic kernel per effect (see the owner's "no target-specific code" rule). Record
   the mechanism, for example `#[cfg]` on the variant and a single cfg'd helper rather than a
   `cfg` at every match site.
2. The wasm gate guest (`scripts/run-wasm-gates.sh` and its guest crate) checks Scalar against
   Simd4 on wasm. Native gates keep Scalar, Simd4 and Simd8.
3. No behavioural change anywhere: the browser already selects Simd4, and native already selects
   Simd8 on x86-64 and Simd4 on AArch64.

Authorized paths: `crates/lane` and each crate whose backend match must change, the wasm gate
guest and its script, their tests, and this spec. Keep the edit mechanical; if it reaches more than
the lane crate plus one cfg'd dispatch point per crate, say why.

## Objective gates

1. The named browser module contains no eight-lane instantiation. The check greps its name section
   for the eight-lane symbols (`f32x8`, `Simd8`, or the kernel names the audit uses) and finds
   none. Record the module's before and after size.
2. Class A: every console-workload digest, every wasm-gate digest (Scalar against Simd4), the V8
   harness digests and the browser qualification render digests equal `7d030945`'s.
3. Native x86-64 is untouched: the workspace tests, both debug splits, the release digest gates
   and the audits pass, and the Simd8 digests are unchanged.
4. The AArch64 iOS and Android compile checks and `check-cross-targets.sh` pass. The AArch64 CI legs
   gate NEON.
5. `scripts/run-wasm-gates.sh` passes on the narrowed guest. The browser qualification passes in
   three browsers, and so do the SDK checks, fmt, clippy `-D warnings`, rustdoc and every lint-job
   step.

## Non-goals

- No change to which width any target runs.
- No change to native x86 or AArch64 code generation.
- Stripping the name section is its own issue.

## Attempt 1 evidence

Implementer: attempt 1, 2026-09-30, on `codex/1110-drop-simd8-wasm` (base `381202ec`, which is
`7d030945` plus the specs). Code commits `006cbfa9` and `a0647084`.

### The owner's condition

The browser can never select or execute `Simd8`. `lane::Backend::current()` is `Simd4` on
`wasm32` with `simd128` (`crates/lane/src/backend.rs`; `target-smoke` pins it per target). Every
shipped path takes its width from `Backend::current()` through `BankWidth::for_backend`
(`host-core`'s `prepare.rs`, `graph-compiler`). The `backend: Backend` parameters in `host-core`
are test hooks. The SDK and the worklet JavaScript only report `"scalar"` or `"simd128"`. The
compile proves it: with `Simd8` and `BankWidth::Eight` absent on `wasm32`, the whole `host-web`
closure still compiles for the browser, so no code in the shipped module names either one.

### Mechanism

The predicate is `#[cfg(not(target_arch = "wasm32"))]`, the same wherever it is written. AArch64
keeps `Simd8`, so its code generation does not change (see "No native code moved").

- **`lane`.** The predicate is on `Backend::Simd8` and its `width` arm, on `mod simd8`, on the
  `pub use wide::f32x8 as Simd8`, and on the `f64x8` `LaneF64` impl, the `Widen for f32x8` impl
  and their width assertion.
- **`effect-contract`.** The predicate is on `BankWidth::Eight` and on that type's own arms
  (`lanes`, `for_backend`, `matches_backend`, `full_mask`), on `transpose_tile_8`, and on the
  eight-lane arm of the new `match_bank_width!`. That macro is the one dispatch from a width to a
  lane type:
  - `|L| body` binds the lane type, `|L, N| body` also binds the lane count, and
    `|L, N, tile| body` also binds the tile transpose.
  - It expands to a `match` in the caller, so `?` and `return` still act on the caller.
  - `BankWidth::for_lanes` turns a lane type's `WIDTH` into a width the macro can take.
- **Crates with no `cfg` at all.** `compressor`, `multiband-compressor`, `parametric-eq`
  (library and corpus), `soft-clip`, `true-peak-limiter` and `transient-shaper`'s library each
  bind their bank through `match_bank_width!`. `soft-clip`'s `lane_width`/`width_is_native` pair
  became `executes(L::WIDTH)`, the rule `gate-expander` already used.
- **Crates with more than one dispatch point, and why.**
  - `builtins` (3 variants plus a 4-arm macro). Its three kernel enums hold the width-typed stage,
    so their `Simd8` variants carry the predicate. The crate's 27 matches over them go through one
    local macro, `per_width!`, whose eight-lane arms are the crate's only other `cfg`.
  - `rack` (4). Its own per-width plane-view enum loses the `Eight` variant, `from_eight`, and the
    two arms that match it. Its three tiled gathers and scatters use `match_bank_width!`.
  - `graph` (1). `distinct_planes_mut`'s eight-lane arm is the only caller of `from_eight`.
    `fold_resident` and the two meter passes use the macro.
  - `gate-expander` (1). Its bank trait impl is written per concrete lane type by
    `bank_impl!(…)`, so the `Simd8` invocation is an item, not an arm.
  - `transient-shaper`'s corpus (3). Its `WIDTHS` has two definitions, and `run_case` keeps its
    per-width match with one arm under the predicate. The arm is kept rather than using the macro
    so that the AArch64 code stays byte-identical (see "No native code moved").
  - `conformance` (1). The `WIDTHS` element.
  - The gate corpus (2). `WIDTHS` and the eight-lane arm of its local `at_width!`.
- **The wasm gates.**
  - The guest now digests at Scalar and Simd4. `wasm_gate_corpus::WIDTHS` is 2 on `wasm32` and
    3 natively.
  - The host no longer requires the guest's width count to equal its own. It pins the count to
    the expected backend instead: `ExpectedBackend::widths()` is 2 for `simd4`. So a guest that
    stopped digesting Simd4 fails the leg rather than passing on the scalar run alone.
  - Natively the conformance probe for "another width's backend" gets the same pair as before. On
    `wasm32`, where that probe is only compiled, it has no other width and is skipped.

### Gate 1: the browser module

The module is built by `build-web-audioworklet.sh` and still carries its name section. It was
demangled and searched for `f32x8`, `f64x8`, `u32x8`, `i32x8`, `Simd8` and `transpose_tile_8`.

| | Base `381202ec` | `a0647084` |
|---|---|---|
| Module bytes | 3,289,705 | 3,044,007 (−245,698, −7.5%) |
| Code section bytes | 2,801,606 | 2,563,514 (−238,092) |
| Functions | 2,502 | 2,456 |
| Name section bytes | 380,140 | 373,182 |
| Eight-lane names | 29 | 0 |

- The 29 base names were 18 `multiband_compressor` bank and instance functions, 9 `builtins`
  stage functions, `lane::…::input_chain_plan::<f32x8>`, and the graph's
  `fold_resident_tiles<8, f32x8, transpose_tile_8>` closure.
- Base SHA-256: `ac3a9353e8107fa409398c16fb27de7cc9846b3c5b0cecf8f864535ad302efc9`.
- This branch's SHA-256: `369d858a0dcb1a26c46ebf73cfa2f342f3f932fdaf56c1d1585f64488aacfcde`.
- The closure digest of the delivery directory is
  `2b804ace4ae4ed5923e32772b4d0c688b4f43c5e38c13841a981fec0650d809b`.
- The release pin (`6c952a2c…`) is not re-pinned.
- The wasm gate guest shrank from 4,697,156 to 2,956,864 bytes.

### Gate 2: class A

Every pin below is unchanged by this issue, so passing it means the digest equals the base's.

- **Console workload.** The digests are pinned in `console-workload`'s release unit tests
  (`cargo test --release -p audit -p bench -p console-workload`): 114 passed.
- **Wasm gates.** 142 cases with no mismatches. The native leg made 358 comparisons at three
  widths. The wasm leg made 250 at Scalar and Simd4, where the base's three-width guest made 358.
  The 108 comparisons dropped are the width-dependent cases' `Simd8` runs. The `max`/`min`,
  `f64` lane and meter-block counts are 0 on both legs.
- **V8 harness.** `check-browser-expected-resources.py --artifacts` reports that the `expected.json`
  digests and exact rows agree with the built module.
- **Browsers.** The qualification passed in Chromium 151.0.7922.34, Firefox 153.0 and WebKit 26.5,
  with `--check-matrix --self-test-mutations` and the SDK bundled from source, as CI runs it. That
  includes the native-digest gate.

### Gate 3: native x86-64

- **Debug split a:** 1,150 passed.
- **Debug split b:** 834 passed, plus the conformance fixture check.
- **Release** (`-p lane -p math -p wasm-gates`): 109 passed, including
  `g5_native_digests_match_pins`, which holds every `Simd8` digest. The M3 FMA cfg and the loom leg
  pass too.
- **Every `audit-native` step passes:**
  - the C ABI audit (0 violations, `pcm_digest` `ff6cdcb96cdcdad5`);
  - the delay, compressor, EQ, gate, builtins, graph, protocol and realtime audits, and the probe
    mutation tests;
  - the one-million-call effect audit;
  - the C ABI linkage and self-test;
  - the scalar-oracle-absent check on `libcapi.so`;
  - graph determinism.

### Gate 4: AArch64

`check-cross-targets.sh` passes. It checks and lints the iOS and Android product crates, and every
iOS `memset_pattern16` count equals its recorded ceiling.

### Gate 5 and the rest

- `run-wasm-gates.sh` passes all legs: native, wasm `simd128`, detector residency, both `f64`
  censuses and the V8 EQ loops.
- fmt, clippy `-D warnings` (native, and for the wasm packages) and rustdoc `-D warnings` pass.
- All 32 steps of the lint job pass.
- The SDK checks pass: generated surface, deletions, types, headless, and the package check.
- The artifact gates pass: scalar-oracle-absent, the hermetic worklet tests and the V8 spill gate.
- The wasm `simd128` checks pass, including the `--all-targets` rows for `effect-compiler` and
  `conformance`.

### No native code moved

This is checked by building the base and this branch from the same path.

- **x86-64.** The release `libcapi.so` has identical `.text` (3,386,963 bytes, SHA-256
  `96c86fd6…`), `.rodata` and `.eh_frame`. The only difference is in `.data.rel.ro`: 239
  panic-location line numbers.
- **AArch64.** All 25 product crates' `aarch64-apple-ios` release assembly is identical to the
  base's, once DWARF, debug labels and panic-location line and column bytes are masked.
- **Two first-checkpoint edits that did move code, now reverted.**
  - A `matches_backend` rewrite changed every factory's `bind_homogeneous_bank` (it is inlined
    through `validate_shape`) by a few bytes.
  - A `for_lanes` rewrite of the shaper corpus changed `run_case`.

  `a0647084` restores both shapes, and the shaper corpus's panic text is the original one.

### Findings for review

1. **Blocking, outside this issue's paths.** `check-web-audioworklet.sh` fails on one row. The
   `KERNEL_ROSTER` in `scripts/check-web-audioworklet-callgraph.py` requires exactly one
   `multiband-compressor f32x8` kernel, and the browser module no longer has one. The brief puts
   that script with #1109, so this attempt does not edit it.
   - With that one row deleted in a transient, uncommitted edit, the whole gate passes on this
     module: 12 kernels against `--kernel-min 11`, and every other roster row within its ceiling.
   - The row's derivation notes are also stale: `multiband f32x8 2560/20`, in that script and in
     `check-web-audioworklet.sh`.
   - Whoever owns those files must delete the row before this merges.
2. **The detector-residency pin has no live witness on the guest.** Deleting its `HotChannel` and
   `History` exclusion used to turn the `simd128` guest red on `HotChannel<f32x8>::load`, a
   384-byte `memory.copy`. That was re-measured red on the base guest.
   - This guest has no `Simd8`, and no limiter function in it makes a history-sized copy.
   - `run-wasm-gates.sh` now says so. It also drops the wasm-`Simd8` sizes 352 and 384 from
     `HISTORY_SHIFT_SIZES`.
   - The pin still fails the first history-sized copy that appears.
3. **Test value.** No test function is added or rewritten. Test modules only import `Simd4` and
   `Simd8` from `lane` directly now.
   - The new host check, `ExpectedBackend::widths()`, turns red on a guest that stopped digesting
     its own width. No existing gate caught that before, because the old check compared the
     guest's width count with the host's own.
