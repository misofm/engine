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

## Sol verdict, attempt 1

**PASS**, with one M and four L findings for follow-up. Sol, 2026-10-01, on `bbf55bd7` (`006cbfa9`
and `a0647084` are the change) and on the combined batch `codex/batch-wasm-size-1` at `6725f0e0`
(#1109 plus this issue plus root's roster-row deletion). The batch's Rust, JS and SDK sources equal
this branch's (`git diff bbf55bd7 6725f0e0 -- crates tools hosts sdk Cargo.* .cargo` is empty), so
every code-level result below holds for both. Every claim was re-run; none was taken from the
attempt's evidence.

**The owner's condition holds.**
- `Backend::current()` is `Simd4` on `wasm32` with `simd128` and is a compile-time constant; `lane`
  refuses `wasm32` without `simd128`. `Backend::Simd8`, `lane::Simd8` and `BankWidth::Eight` do not
  exist on `wasm32`, so no shipped path can select or name eight lanes: re-enabling them in `lane`
  and `effect-contract` alone does not compile for the browser (mutation M1 below).
- Every production `host-core` caller passes `Backend::current()`; the explicit-backend seams are
  `#[cfg(test)]` or are called only with it. The SDK and the worklet name a backend only as
  `"scalar" | "simd128"` and never a width.
- No test-support path ships: `cargo tree -p host-web --target wasm32-unknown-unknown -e features`
  enables no `test-support` feature anywhere, and `check-scalar-oracle-absent.py --wasm` passes on the
  named twin.

**Gate 1, the browser module.** I rebuilt base `7d030945` and demangled both name sections.

| | `7d030945` | batch named twin |
|---|---|---|
| Module bytes | 3,289,705 (`ac3a9353…`) | 3,044,007 (`369d858a…`) |
| Code section bytes | 2,801,606 | 2,563,514 |
| Functions | 2,502 | 2,456 |
| Eight-lane names | 29 | 0 |

- The eight-lane names were `f32x8`, `f64x8`, `u32x8`, `i32x8`, `Simd8`, `transpose_tile_8`, any
  `x8`, or a const-generic `8`.
- The shipped (stripped) batch module is 2,670,821 B (`cd49dc1c…`), and the closure digest is
  `02ca569e…`.

**Mechanism: one kernel shape, with the width set as the only target difference.**
- The change adds 35 code `cfg` sites, all on the same architecture predicate. 29 are product code:
  - `lane` (7) and `effect-contract` (9) define the width and its one dispatch, `match_bank_width!`;
  - `builtins` (7), `rack` (4), `graph` (1) and `gate-expander` (1).

  Each of the last group is forced by a per-width enum variant, a per-type trait impl or a
  lane-count arm. A width-to-type macro cannot express any of them.
- The other 6 are test and corpus code: `conformance` (1), the shaper corpus (3) and the gate corpus
  (2).
- Every kernel body stays one generic body, so no target-specific shape crept in.

**Native code did not move.** Base `381202ec` and `bbf55bd7` were built from one path.
- **x86 `libcapi.so`.** `.text` (3,386,963 B, `96c86fd6…`), `.rodata`, `.eh_frame` and
  `.eh_frame_hdr` are identical. In `.data.rel.ro`, 239 `u32` words differ, all panic line numbers.
- **iOS staticlib.** I built `capi` as a staticlib with `cargo rustc --crate-type staticlib`, fat
  LTO. `__text` (2,469,368 B) is identical, as are every other `__TEXT` section, `__compact_unwind`,
  `__data`, the relocations and the 373 other archive members. `__DATA,__const` differs in 236
  line-number words.
- **Android staticlib.** All 2,445 `.text.*` sections (2,377,700 B) are identical. The 236 differing
  sections are 24-byte panic `Location`s that differ only in line and column.
- **The arm64 test legs** cannot run on this x86 host. Their code is the base's.

**Gates on the batch `6725f0e0`.** All green.
- **Lint job, 32 of 32 steps.** This includes fmt, clippy `--workspace --all-targets
  --all-features -D warnings` and rustdoc `-D warnings`.
- **Debug splits.** Split a: 1,150 passed. Split b: 834 passed, plus the conformance fixture check.
- **Release** (`-p lane -p math -p wasm-gates`). 109 passed, including
  `g5_native_digests_match_pins` with every `Simd8` pin. The M3 FMA cfg and loom pass.
- **Every `audit-native` step.**
  - Console-workload: 114 passed.
  - C ABI audit: 0 violations, `pcm_digest` `ff6cdcb96cdcdad5`.
  - The delay, compressor, EQ, gate, builtins, graph, protocol and realtime audits, and the probe
    mutation tests.
  - The one-million-call effect audit.
  - C ABI linkage and self-test.
  - The native scalar-oracle check.
  - Graph determinism, the fixtures and the effect contract.
- **Wasm.** The `simd128` probe, the evidence crates, `host-web` for wasm and the protocol parity
  pass.
- **`run-wasm-gates.sh`, full.**
  - Native: 358 comparisons at three widths. Wasm: 250 at Scalar and `Simd4`. 0 mismatches.
  - The `max`/`min`, `f64` and meter counts are 0.
  - The residency pin, the `f64` censuses and the V8 EQ loops pass.
- **`check-cross-targets.sh`.**
- **Artifact gates.**
  - The #1109 named/shipped equivalence (`strip-wasm-names.py` self-test and `check`).
  - `check-web-audioworklet.sh`: 12 kernels against `--kernel-min 11`, and all ten roster rows ok.
  - `check-browser-expected-resources.py --artifacts`.
  - `check-scalar-oracle-absent --wasm`.
  - `test-web-audioworklet.sh`.
  - The V8 spill gate.
  - The call-graph self-test.
- **SDK.** generated, deletions and its self-test, types, headless, package.
- **Browsers.** Chromium 151.0.7922.34, Firefox 153.0 and WebKit 26.5, with `--check-matrix
  --self-test-mutations` and the SDK from source.
- **Routing.** `test-ci-path-routing.py` and `check-ci-path-routing.py`.

**Class A.** No pinned digest changed between `7d030945` and `6725f0e0`; the only hex edits are in
specs and `docs/`. So every digest gate above passing means its digests equal `7d030945`'s:
- the console-workload digests;
- the wasm gates at Scalar against `Simd4`;
- the V8 harness's `expected.json`;
- the three browsers' native-digest gate.

**The roster deletion is the correct consequence.**
- `6725f0e0` removes exactly the `multiband-compressor f32x8` row and its derivation line. The
  other nine rows and the limiter forwarding rule are intact, and the self-test passes.
- The branch's own roster fails this module on that row, as the attempt disclosed.
- The 12 ratchet kernels are the ten roster kernels plus `compressor::settled_sidechain<f32x4>` and
  `FaderRampStage<f32x4>::process_plane`. Neither of those two was in the roster before this change.

**Lost coverage.**
- **The wasm `Simd8` leg.** It digested `wide::f32x8` lowered to two `v128` halves, which no
  browser executes. Generic eight-lane bugs stay caught natively at `Simd8`, on x86 AVX2 and on
  arm64 as two NEON halves.
- **What it also was.** It was the wasm leg's second vector instantiation. Since this change,
  nothing checks that the guest's width index 1 really runs `Simd4` (finding M1).
- **The detector-residency pin.**
  - **What it defended.** The limiter's twelve-tap history stays in wasm locals and is not
    block-moved through linear memory every frame. That move is a per-tap store-to-load round trip
    on a latency-bound kernel, and `History<L>`'s twelve named fields are the structural fix.
  - **Native coverage.** No test defends the property on x86 `Simd8`, or on any native target.
    That is consistent with its nature: native SROA promotes the array, and x86 is not a live
    shipped target (ruling R2).
  - **The shipped module.** The named twin's 55 in-scope limiter functions carry no 44-, 48-, 176-
    or 192-byte copy.
  - **What is needed** is finding L1.

**Test value.**
- No test function is new. The new host pin, `ExpectedBackend::widths()`, is the only new check.
  The plausible defect only it turns red: a wasm guest that digests fewer widths than Scalar plus
  its own. Mutation M3 shows it. With the host's old equality check relaxed, nothing else would
  catch that.
- I planted five mutations in a scratch worktree at `6725f0e0`, then reverted all of them:

| Mutation | Result |
|---|---|
| M1: remove the predicate in `lane` and `effect-contract` | Red. `host-web` does not compile for wasm32: three non-exhaustive `BankWidth::Eight` matches in `builtins`, and a missing `gate-expander` bank impl. |
| M2: `match_bank_width!`'s four-lane arm binds an identity instead of `transpose_tile_4` | Red natively: 8 `graph` runtime tests fail. The V8 harness stays green, because its sessions never reach the full four-lane tiled gather, scatter or fold. That gap predates this issue; the arm64 legs run this path at `Simd4`. |
| M3: wasm `WIDTHS = 1` | Red: "guest digests at 1 widths but a simd4 guest has 2". |
| M4: `at_width!` index 1 maps to `f32` | Green: 250 comparisons, 0 mismatches. See M1. |
| M5: the residency pin also forbids 92 B | Red: 6 hits in `true_peak_limiter::corpus::run_case`. The scanner is wired to real limiter functions. |

**Findings.**
- **M1** (`tools/wasm-gate-corpus/src/lib.rs:83`, `tools/wasm-gates/src/lib.rs:81` and `:398`).
  - **The problem.** The wasm leg's only vector witness has no witness of its own. The new host
    check pins the count of widths, not which lane type each one runs. M4 shows that a guest whose
    index 1 silently runs `f32` passes with 0 mismatches.
  - **What changed.** Before this issue, the `Simd8` leg was a second vector run on wasm. The
    attempt's claim that "a guest that stopped digesting Simd4 fails the leg" holds only when the
    count drops.
  - **Fix.** Have the guest export each width index's `L::WIDTH`, and have the host require
    `[1, 4]`.
  - Non-blocking follow-up.
- **L1** (`scripts/run-wasm-gates.sh:102`).
  - **The problem.** The residency pin has no proof that it scans anything. A guest without a
    `name` section, or a renamed crate, would pass vacuously.
  - **Fix.** Assert that at least one in-scope `true_peak_limiter` function was scanned. Add a
    hermetic self-test that feeds a synthetic disassembly with a 192-byte `memory.copy` in a limiter
    function and expects red.
- **L2** (`scripts/check-web-audioworklet.sh:463` on the batch).
  - **The problem.** The owner's condition has no committed guard. The batch's
    `--callgraph`/`--kernel-shape` gate passes the base module, which has 29 eight-lane functions
    and 15 kernels, with exit 0. So eight lanes returning to wasm32, by a full revert of the
    predicate, turns nothing red. Only the predicate itself guards it, and M1 shows that a partial
    revert fails to compile.
  - **Fix.** Turn the deleted presence row into an absence rule: no `f32x8` name in the named twin.
    Also narrow the ratchet pattern `4wide6f32x[48]` to `4wide6f32x4`, so that eight-lane kernels
    cannot pad the count.
- **L3** (`6725f0e0` left stale prose in `scripts/check-web-audioworklet.sh`; line numbers are
  the batch's).
  - `:443` still lists `multiband f32x8 2560/20`.
  - `:454` says "The artifact carries thirteen … two-kernel slack". It now carries 12, which leaves
    a slack of 1.
  - `scripts/check-web-audioworklet-callgraph.py:35` and `:116` say "eight" roster kernels. That
    was already stale before this change.
- **L4.** Out of scope here (a non-goal), recorded for the owner.
  - **The fact.** arm64 also never selects eight lanes, because `Backend::current()` is `Simd4`
    there. The iOS library still carries 27 eight-lane functions: about 114,520 B of its
    2,469,368 B `__text`, or 4.6%.
  - **What it means.** This is the same question this issue answered for the browser. It would be
    a successor issue if wanted.
  - **Related cosmetic point.** The shaper corpus's local `cfg`s
    (`crates/transient-shaper/src/corpus.rs:65` and `:178`) could use `for_lanes` with
    `match_bank_width!`, as the EQ corpus does. The per-crate rlib identity they preserve is not
    product code, because fat LTO drops the corpus from the linked library.

## Post-verdict fixes (root-directed)

Terra, 2026-10-01, on `codex/batch-wasm-size-1` from `50ad88df`. This closes Sol's M1, L1, L2 and
L3. L4 is out of scope and untouched. Only the gates and the gate guest changed, so the shipped
module is still `cd49dc1c…` and its named twin `369d858a…`.

- **M1** (`d327b705`, `089a566e`).
  - The guest exports `miso_gate_lane_width(index)`: the `Lane::WIDTH` of the type `at_width!`
    binds at that index (`wasm_gate_corpus::lane_width`).
  - The host's `ExpectedBackend::widths()` became `lane_widths()`, which is `[1, 4]` for `simd4`.
    The guest's per-index lane counts must equal it exactly.
  - Mutation "`at_width!` index 1 runs `f32`" is now red: "guest digests at lane widths [1, 1] but
    a simd4 guest has [1, 4]". It was green before, with 250 comparisons and 0 mismatches.
- **L1** (`5b3cf97a`).
  - The residency pin counts the in-scope `true_peak_limiter` functions it scans, and fails on zero.
    The `simd128` guest has 7.
  - A hermetic self-test runs before the guest build, on synthetic disassembly:
    - a 192-byte `memory.copy` in a limiter function is red;
    - a disassembly with no limiter function is red;
    - the same function copying 64 bytes is green.
  - Both scanner mutations turn the self-test red: removing the zero check, and breaking the size
    match.
  - The guest with its `name` section stripped is red. The old pin passed it.
- **L2** (`45f5b488`).
  - `--kernel-shape` also runs a fourth rule, `check_no_eight_lanes`: no function name may match
    `EIGHT_LANE`, which covers `f32x8`, `f64x8`, `u32x8`, `i32x8`, `simd8` and `transpose_tile_8`.
  - The ratchet pattern is now `4wide6f32x4`.
  - `7d030945`'s named module (`ac3a9353…`) is red, with 29 eight-lane functions named. The old
    gate passed it, counting 15 kernels.
  - The batch's named twin is green: 12 kernels, and no eight-lane name among its 2,456 functions.
  - Self-test case (g) shows each spelling red. A crate hash that merely contains `x8` is not red;
    the base module had three such hashes.
  - A const-generic `8` (`Kj8_`) is left out of the rule. All 19 of those in the base module also
    name `f32x8`, and an eight-element array is not a lane width.
- **L3** (`45f5b488`).
  - In `check-web-audioworklet.sh`, the multiband `f32x8` row is now a historical note, and
    "thirteen kernels, two-kernel slack" now reads twelve kernels with a slack of one.
  - In the analyser, `KERNEL_ROSTER` has nine rows. The collapsed limiter's kernel, held by the
    forwarding rule, makes ten. Seven of the ten carry zero scalar arithmetic; the old prose said
    four of eight.

**Test value.** Each new check, and the plausible defect only it turns red:
- **M1's host pin:** a guest whose width index 1 runs a lane type other than `Simd4`. Such a guest
  digests the scalar oracle twice and matches every pin.
- **L1's zero-scan assertion and its self-test:** a guest without names, or a renamed limiter crate,
  which the pin used to pass unread; and a residency scanner whose size match broke.
- **L2's rule and case (g):** eight lanes coming back to the browser build in full, through a
  reverted predicate. Every other gate passes the base module.

**Gates.** All green.
- `bash scripts/check-web-audioworklet.sh`, in its self-building no-argument form. It reports
  `cd49dc1c…`, 12 kernels, all ten roster rows ok, and no eight-lane name.
- The call-graph `--self-test`, inside `test-web-audioworklet.sh`.
- `run-wasm-gates.sh`, full.
  - Native: 358 comparisons. Wasm: 250. 0 mismatches.
  - Residency: 7 limiter functions scanned.
  - The `f64` censuses and the V8 EQ loops pass.
- `cargo test --release -p wasm-gates -p wasm-gate-corpus`: 9 passed.
- fmt.
- Clippy `-D warnings` on the three touched crates: natively with `--all-targets --all-features`,
  and for `wasm32` with `simd128`. rustdoc `-D warnings` on them too.
- `test-ci-path-routing.py` and `check-ci-path-routing.py`.
- The lint job's hermetic steps: 25 of 25.

## Batch boundary evidence

**PASS, nothing fixed.** Terra checked the merged batch once, as `qualification.yml` would. The
batch is `codex/batch-wasm-size-1` at `0eabf578` on `origin/main` `7d030945`. It merges #1109 (the
shipped module drops its `name` section; the gates read the named twin) and #1110 (no eight-lane
code in the browser build; the wasm guest is Scalar against `Simd4`), with root's post-verdict gate
hardening (M1, L1-L3), root's roster-row deletion and the removal of three closed specs.
- The router gives `route=full`, `math_closure=true`, `release_inputs=true` and all five
  `self_tests` (`env-vocabulary`, `conformance-boundaries`, `console-benchmark`, `sdk-deletions`,
  `dsp-research`), for `--event push` and `--event pull_request` alike, base `7d030945`.
- 110 steps ran on an x86-64-v3 host (AMD EPYC 7313P) with rustc 1.97.1, `CARGO_INCREMENTAL=0` and
  the worktree's `target/`. Every step exited 0.
- No merge-interaction defect was found, and nothing needed a commit beyond this record.
- No timed benchmark ran.

| Job | Gate (command) | Result |
|---|---|---|
| route | `check-ci-path-routing.py`, `test-ci-path-routing.py` (the batch changes CI steps) | ok |
| docs-gates | `check-dsp-research.sh`, `check-builtins-listening.sh` | ok |
| lint | `cargo fmt --all -- --check` | ok |
| lint | `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | ok |
| lint | `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | ok |
| lint | the job's other 25 steps, the same list as C4's, including script reachability (`strip-wasm-names.py` is reached) | all ok |
| lint | the three sub-v3 probes: scalar and AVX2-without-FMA are refused with "requires x86-64-v3", and AVX2+FMA compiles | ok |
| gate-self-tests | the five routed suites: `test-env-vocabulary.sh`, `test-conformance-boundaries.sh`, `test-console-benchmark.sh`, `check-sdk-deletions.py --self-test`, `test-dsp-research.sh` | all ok |
| new self-tests | `strip-wasm-names.py --self-test` (every mutation caught); `check-web-audioworklet-callgraph.py --self-test` (case (g), eight lanes); `check-web-audioworklet-v8-spill.py --self-test` (19 cases); `web-audioworklet-identity.py --self-test`; the residency self-test inside `run-wasm-gates.sh` | all ok |
| test-debug-a | `cargo test --locked --workspace --all-targets --exclude …` with the job's features | 96 binaries, 1150 passed, 0 failed |
| test-debug-b | `cargo test --locked --all-targets -p lane … -p conformance` with the job's features; `conformance_fixtures -- --check` | 151 binaries, 834 passed, 0 failed; fixtures ok |
| test-release | `cargo test --locked --release -p lane -p math -p wasm-gates --features math/lane` | 109 passed; `g5_native_digests_match_pins` ok, every `Simd8` pin included |
| test-release | the M3 FMA cfg check; loom `spsc_loom`; M1 `m1_exhaustive` and F1 `f1_fast_db_bounds` `--ignored` (math closure) | ok |
| audit-native | release build; `cargo test --release -p audit -p bench -p console-workload` | 114 passed, 0 failed |
| audit-native | `audit capi` and its record validator (`pcm_digest` `ff6cdcb96cdcdad5`, unchanged); `audit delay`, `compressor` and `parametric-eq` at 100,000 blocks; `audit gate-expander` (bank width 8, `bank_available`) | every counter 0, `total_violations` 0 |
| audit-native | the builtins, builtins-graph, graph, realtime and effect-contract traces at 1,000,000 blocks; the protocol allocation audit; the realtime, builtins and builtins-graph probe mutations | ok |
| audit-native | `check-capi-abi.sh` and `--self-test`; scalar oracle absent from `libcapi.so`; graph determinism 100/100; builtins fixtures (50 files); console fixtures; `check-effect-contract.sh` (8 factories, 0 failed gates) | ok |
| wasm-guests | runner build; simd128 probe; evidence crates; `check-protocol-wasm-parity.sh`; `run-wasm-gates.sh --without-v8-spill --without-native` | ok: wasm 142 cases, 250 comparisons at Scalar and `Simd4`, 0 mismatches; residency scanned 7 limiter functions |
| wasm gates, full | `run-wasm-gates.sh` with no flags | ok: native 358 comparisons at three widths, wasm 250, 0 mismatches; the `f64` censuses and the V8 EQ loops pass |
| cross-target | `scripts/check-cross-targets.sh` | PASS. The iOS `memset_pattern16` rows are the #1018 expected failures (parametric-eq 146). |
| release-shape | `check-release-shape.py`; `CARGO_PROFILE_RELEASE_PANIC=unwind cargo check --locked --release --workspace --all-targets` | ok |
| artifact | `build-web-audioworklet.sh --named-twin target/ci/qualification-named-twin target/ci/qualification-artifacts` | shipped `cd49dc1c…d408`, named twin `369d858a…cfcde`, closure `02ca569e…b259` |
| artifact-identity | a twin `--module-only` build from another path and `CARGO_HOME`; `web-audioworklet-identity.py --self-test` and `report --event push --before 7d030945` | reproducible; **ARTIFACT CHANGED** |
| artifact-gates | the named twin against the shipped module: `strip-wasm-names.py --self-test` and `check` | ok: 9 non-custom and 2 other custom sections byte-identical; the `name` section is 373,186 B |
| artifact-gates | `check-web-audioworklet.sh --without-metadata-regeneration` on the shipped closure and the named twin; `check-browser-expected-resources.py --artifacts` | ok: 12 kernels on `4wide6f32x4` against `--kernel-min 11`, all ten roster rows ok, "eight lanes: none among 2456 functions"; `expected.json` digests and rows agree |
| artifact-gates | `check-scalar-oracle-absent.py --wasm` on the named twin; `test-web-audioworklet.sh`; the V8 spill self-test and gate on the named twin (Node 22.23.2) | ok |
| #1109 gate 3 | the call graph, scalar-oracle and V8 spill gates fed the shipped module | each refuses it with its typed "no `name` section … give it the named twin" message (exit 1, 1 and 2) |
| sdk | from a clean `npm ci`: `check-sdk-generated.sh`, `check-sdk-deletions.py`, `check-sdk-types.sh`, `check-sdk-headless.sh` (329 passed) and `sdk-package.sh check` | ok |
| browser | `npm run qualify -- … --check-matrix --self-test-mutations` on the shipped module, for Chromium 151.0.7922.34, Firefox 153.0 and WebKit 26.5, each in CI's source-bundle mode | all qualification gates passed; the exact 7-file shipped set is pinned |
| console benchmark | `scripts/test-console-benchmark.sh`; `operator/preflight-console-benchmark.sh --step terra-wasm-boundary-scratch` | PASS; the preflight PASS has 0 workload launches and wrote no step directory |
| V8 harness | `run-web-mixing-automation-benchmark.sh prepare` and then `preflight`, into empty scratch directories, at the head and at `7d030945` | PASS |
| aarch64-debug/-release | see below | resolved; not run (no arm64 host) |

**Artifact.** ARTIFACT CHANGED. The committed pin (`6c952a2c…`) was not touched; under #1061 it is
checked only at release.

| | main `7d030945` | batch named twin (not shipped) | batch shipped module |
|---|---|---|---|
| Digest | `ac3a9353e8107fa409398c16fb27de7cc9846b3c5b0cecf8f864535ad302efc9` | `369d858a0dcb1a26c46ebf73cfa2f342f3f932fdaf56c1d1585f64488aacfcde` | `cd49dc1c4926fd98b9d5240b9b97dd9dd807f2df0e237cf1918c57abe21fd408` |
| Size | 3,289,705 B | 3,044,007 B (−245,698 B, −7.47 %: #1110) | 2,670,821 B (−618,884 B, −18.81 %: #1110 and #1109) |

- The main digest is the `audioworklet-sha256` status that main's own run recorded. The V8
  harness's `prepare` on a detached clone of `7d030945` reproduces it byte for byte.
- The digests and sizes are the ones Sol's #1110 verdict and the post-verdict fixes report.

**Class A.** The head and a detached clone of `7d030945` each got their own release build, with
the base's in `target/ci/base-7d030945`.
- **Console digests.** `cargo test --release -p console-workload --test gain_pan_profile digests
  -- --ignored`: **22 of 22 rows are identical**, including `sixty_four_track_app_shape`
  `c740fa2dd904` and the nine-, ten- and thirteen-track ragged strips.
- **`bench console --preflight`.** The output is identical (16 lines).
- **V8 preflight.** Apart from `module_sha256`, the output is identical to the base's. The 7 arm
  digests also equal S0's record (`preflight_output_sha256`): quiet and restated `014e5f5b…`,
  automated `e7025b5c…`, EQ `2540aff4…`, compressor `c29d12a7…`, limiter `8db18991…`. Both
  documents are identical: console `d913ad961d2d` and app `3dd8b2fff4b9`. `controls.json` is
  identical (`a32cb879…`).
- **S0's baselines, untimed.** As in C4: a `git archive` of the head, built into its own target
  directory, with `bench_support::timing::timed` returning 0 without reading the clock and nothing
  else changed. One run of its `bench console` subject (round marker `1`, output in scratch) gives
  all 30 of S0's round-1 records in `artifacts/steps/console-strip-base/`.
  - **All 50 render-digest fields are equal**, every row's `output_sha256` among them.
  - Besides timing, only the app row's `strip_layout` differs (`inserts:` -> `pre_insert:`), as in
    C3 and C4.
- The wasm gates (Scalar against `Simd4`), the G5 native corpus (every `Simd8` pin), the browsers'
  native-digest gate and `expected.json` all pass on pins this batch does not edit.

**AArch64 legs.** They cannot run on this host: there is no arm64 machine, cross linker or
`qemu-aarch64`. Resolved:
- **Debug leg.** The 25 product crates (`capi`'s closure, from `scripts/lib/product-crates.sh`)
  plus `dsp-reference`, `conformance` and `target-smoke`. It has no expected-failure rows.
- **Release leg.** `lane` and `math` with `math/lane`, then `console-workload`, then the capi,
  delay, compressor, parametric-eq and gate-expander audits. It has two #1019 rows:
  `m2_exp2_lane_identity` and `m2_log2_lane_identity`, both in `math`'s `m2_lane_identity`.
  `aarch64-release`'s G5 step is unfiltered.
- **Checks run here.**
  - The no-silent-skip scan, exactly as the script writes it (rg exit 1, no match).
  - The known-defect self-test.
  - `judge-skips` over each leg's `--list`: debug 1817 tests, release 114 tests. The listings come
    from x86 builds of the same packages and features. #1110's predicate is
    `not(target_arch = "wasm32")`, so it adds or removes no AArch64 test. Every row names exactly
    one test.

**Local deviations, none a gate change.**
- The twin build's second checkout is a `git archive` of `HEAD`, with a fresh `CARGO_HOME`, both
  in scratch.
- The identity report ran as the main push will: `--event push --before 7d030945`.
- The release-shape unwind check used its own target directory, `target/ci/unwind`.
- The wasm guest builds in `target/ci/wasm-gates-*` were cargo-fresh from an earlier build of this
  head.
- The browser legs' pulse sockets and `TMPDIR` used short `/tmp` paths, as in C2 to C4.
- The `sdk/dist` that `sdk-package.sh check` leaves was removed before the browser legs, so all
  three ran in CI's source-bundle mode.
