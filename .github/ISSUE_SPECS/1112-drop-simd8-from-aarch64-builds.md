# Drop the eight-lane kernels from the AArch64 builds

## Problem

AArch64 always runs four lanes: `lane::Backend::current()` is `Simd4` on `aarch64` (iOS, Android,
and Apple-silicon desktop). #1110 compiled the eight-lane backend off wasm32 with
`#[cfg(not(target_arch = "wasm32"))]`, which leaves it on AArch64, where it can never run. Sol's
#1110 verdict (L4) measured the iOS `capi` library: 27 eight-lane functions, about 114,520 B of its
2,469,368 B `__text` (4.6 %).

Owner, 2026-10-01: "there's no point in having code that can't be run in the iPhone build."

## Smallest closable slice

1. The eight-lane items #1110 gated (`lane::Simd8`, `Backend::Simd8`, `BankWidth::Eight`, the
   `match_bank_width!` arm, and the few local cfgs in builtins, rack and the corpora) compile only
   on `target_arch = "x86_64"`, the one target that selects eight lanes. Narrow the predicate at
   each of #1110's sites, or route it through one shared predicate if that is simple. Do not add
   cfg sites.
2. Tests and gates on AArch64 compare Scalar with Simd4. x86-64 keeps Scalar, Simd4 and Simd8. The
   wasm build is unchanged by this issue.
3. No behaviour changes anywhere. No target changes the width it selects.

Authorized paths: #1110's cfg sites and the crates that hold them, the tests and gates that name
eight lanes on AArch64 (including the AArch64 CI legs' test lists, `scripts/run-aarch64-tests.sh`
and `scripts/lib/aarch64-known-defects.py` if a count moves), and this spec.

## Objective gates

1. The AArch64 iOS and Android `capi` builds contain no eight-lane function. A committed check
   fails if one returns, for example in `check-cross-targets.sh` or the AArch64 release leg. Record
   the iOS and Android `__text`/`.text` size before and after.
2. x86-64 is untouched: its `capi` `.text`/`.rodata` are identical to `089ef456`'s, and every x86
   Simd8 digest is unchanged. The wasm module is unchanged (`cd49dc1c…` shipped).
3. Class A: all console-workload digests and the wasm-gate digests equal `089ef456`'s.
4. `check-cross-targets.sh` passes, and so do the AArch64 legs' `--no-run`/`--list` resolution,
   `judge-skips` and the silent-skip scan. This host has no AArch64 runner, so CI's AArch64 debug
   and release legs are the NEON gate.
5. fmt, clippy `-D warnings` (x86 and both AArch64 targets), rustdoc, both debug splits, the release
   digest gates, `run-wasm-gates.sh` and every lint-job step pass.

## Non-goals

- No change to which width any target runs.
- No other size work for the mobile library.

## Attempt 1 evidence

Terra, attempt 1, 2026-10-01, on `codex/1112-drop-simd8-aarch64` from `e68e58ba` (`089ef456` plus
this spec). Commits: `2d8c710b` (product), `cb30338a` and `fa890f13` (tests), `9b45d60f` (gates).

### The owner's design, as built

The owner extended the brief during the attempt. The lane width follows the target features a build
enables, never the architecture's name.
- **8-lane (AVX2).** One predicate, `target_feature = "avx2"`, gates every eight-lane item and arm:
  `lane::Simd8`, `Backend::Simd8`, the `f64x8`/`Widen` companions, `BankWidth::Eight` and its arms,
  `transpose_tile_8`, `match_bank_width!`'s eight arm, builtins' `per_width!` arms, rack's `Eight`
  views, graph's eight-lane planes, gate-expander's `Simd8` bank impl, and the corpora's width lists.
- **4-lane (NEON/simd128).** `Backend::current()` selects `Simd8` under `avx2`, and `Simd4` under
  `any(target_feature = "neon", target_feature = "simd128")`. Graph's `FrameLane` uses the same two
  predicates. `Simd4` itself stays on every build, because x86 tests run it.
- **Where `target_arch` stays.** Only where the architecture matters: the 64-bit guard, the x86 AVX2+FMA
  guard, `attest_host`, the MXCSR/FPCR code, `wide_impl`'s instruction lowerings, and the audit's
  x86 disassembly probes.
- **No new product cfg site.** These are #1110's 35 sites, narrowed in place. Rust has no cross-crate
  cfg alias without build scripts, so the one predicate is written identically at each site.
  `current()` went from three arch arms to two feature arms.
- **The rack policy.** `check-rack-policy.sh` forbade `target_feature` in rack. Its one exempt
  spelling is now the width predicate, alone on its line. Mutations cover a `neon` cfg, a `cfg!`
  expression and the complement, and each turns the self-test red. So does widening the exemption
  or dropping it.

### The owner's condition: no AArch64 path can select eight lanes

- On AArch64 `Backend::Simd8` and `BankWidth::Eight` do not exist, so no code there can name them.
- The C ABI, host-core and the shipped test-support closure all compile for iOS and Android without
  them.
- Every production caller (`host-core`'s `prepare.rs`) passes `Backend::current()`.
- The one AArch64 code that named `Backend::Simd8` was the dev binary `graph_fixture`, which compiled
  the corpus's 8-lane plan with eight-lane builtin banks. On a 4-lane build it now refuses every mode,
  and its two tests are `#[ignore]`d with the reason.

### Gate 1: no eight-lane function in the iOS and Android libraries

`capi` was built as a fat-LTO release staticlib with `cargo rustc --crate-type staticlib`. Base and
head were built from one path.

| | `089ef456` | head |
|---|---|---|
| iOS `capi` object `__text` | 2,469,368 B | 2,276,340 B (−193,028, −7.8%) |
| iOS archive `__text`, all members | 2,531,428 B | 2,338,400 B |
| iOS eight-lane function symbols | 27 | 0 |
| Android `capi` object `.text` | 2,377,700 B (2,447 sections) | 2,225,188 B (2,411; −152,512, −6.4%) |
| Android eight-lane function symbols | 28 | 0 |

- The size falls by more than the 114,520 B that #1110's L4 counted. Eight-lane bodies inlined into
  non-eight-lane-named callers went too.
- **The committed check.** `check-cross-targets.sh` already emits every product crate's iOS release
  assembly for the memset scan. It now fails on any line matching the browser rule `EIGHT_LANE`
  (`f32x8|f64x8|u32x8|i32x8|simd8|transpose_tile_8`). That covers labels, calls, and the
  line-table names of inlined functions.
  - **Red on `089ef456`.** The same loop over that commit's iOS assembly matches eight crates:
    builtins 451, multiband-compressor 255, gate-expander 173, transient-shaper 145, graph 110,
    parametric-eq 96, soft-clip 4, graph-compiler 2.
  - **Green on head.** All 25 crates have 0 matches.
- **Known-defect counts that moved (#1018).** Six iOS `memset_pattern16` counts fell, because the
  removed eight-lane instantiations' two-half splats made the same calls:
  - builtins 376 → 194, gate-expander 181 → 91, graph 20 → 10;
  - multiband-compressor 1,132 → 566, parametric-eq 146 → 132, transient-shaper 534 → 268.

  Their ceilings are lowered to those counts. The test rows did not move.

### Gate 2: x86-64 and wasm untouched

- **x86 `libcapi.so`.** Every allocated section is byte-identical to `089ef456`'s:
  - `.text` 3,386,963 B (`96c86fd6…`), `.rodata` (`f4e6ad9c…`), `.eh_frame`, `.eh_frame_hdr`;
  - also `.data.rel.ro` (`38c90f95…`), so no panic location moved.

  Only the build-id, `.debug_info` and `.debug_line` differ. Debug line information moved in
  comment-only regions; for example `lane/src/backend.rs`, which has no panic site, is one line
  shorter.
- **Wasm.** The shipped module is `cd49dc1c…d408` (2,670,821 B), the named twin `369d858a…` and the
  closure `02ca569e…`, all equal to main's.
- **Line numbers.** Every test edit in a shipped `src/` file sits in test or test-support code that
  runs to the end of its file. The identical `.data.rel.ro` and wasm module confirm it.

### Gate 3: class A

- `cargo test --release -p audit -p bench -p console-workload`: 114 passed.
- `run-wasm-gates.sh`, full:
  - native 358 comparisons at three widths, wasm 250 at Scalar and `Simd4`, 0 mismatches;
  - the residency pin scanned 7 limiter functions;
  - the `f64` and meter counts are 0.
- `g5_native_digests_match_pins` passes, with every `Simd8` pin.

### Gate 4: AArch64

- `check-cross-targets.sh` passes.
- **Clippy `-D warnings` on iOS and Android** passes for:
  - the debug leg's package set with its features;
  - `lane`, `math`, `console-workload`, `audit` and `wasm-gate-corpus` with `math/lane`, and with
    default features.

  `wasm-gates` does not build for either target here, as at base, because wasmtime's C build needs
  xcrun or the NDK.
- **The silent-skip scan** finds nothing (rg exit 1).
- **No arm64 runner, so a scratch emulation.** The legs' package sets were built on x86 from a
  scratch copy, not committed, with `avx2` off, lane's x86 guard removed, `neon` read as `sse2` and
  `RUSTFLAGS=-D warnings`. That is a 4-lane build on `f32x4`.
  - **What it caught.** Five tests that only an AArch64 leg would have failed: three padded-bank case
    counts and the EQ's `for lanes in [4, 8]`. `fa890f13` fixes them.
  - **Results.**
    - Debug leg: 231 binaries pass.
    - Release lane and math: 97 passed. Its rows are the arm64-only LANE-3 failures, so here they
      pass.
    - `console-workload`: 65 passed.
    - `wasm-gates` G5: 9 passed at two widths.
- **Listings.**

  | Leg | 8-lane (x86) | 4-lane |
  |---|---|---|
  | debug | 1,817 tests | 1,767 tests |
  | release | 114 tests | 110 tests |

  Every test in the difference names an eight-lane item. `judge-skips` passes for both modes, and
  `m2_exp2_lane_identity` and `m2_log2_lane_identity` each name one test.
- **The release rows still fail as written.** In `m2_lane_identity` the width-4 assertion still
  precedes width 8 at each input, and the scalar digest still follows the comparison. Neither row's
  reason depends on `Simd8`, so the counts stay. CI's arm64 legs are the NEON gate.

### Gate 5

- fmt passes.
- Clippy `--workspace --all-targets --all-features -D warnings` passes.
- `RUSTDOCFLAGS='-D warnings' cargo doc` passes.
- All 32 lint-job steps pass, including the three sub-v3 probes, and so do the routing check and its
  tests.
- **Debug split a:** 96 binaries, 1,150 passed.
- **Debug split b:** 151 binaries, 834 passed, plus the conformance fixtures.
- **Release:** 109 passed, plus the FMA cfg, loom, M1 and F1.
- **Every audit-native step passes.** `audit capi` has 0 violations and `pcm_digest`
  `ff6cdcb96cdcdad5`.

### Lost AArch64 coverage, and test choices for review

- **Coverage AArch64 loses** (all of it still runs on x86):
  - every `Simd8` run (two NEON halves);
  - the other-width refusal probes (conformance, `bank_mask`, compressor, soft-clip, shaper, EQ,
    limiter, rack);
  - eight-lane-only tests of generic mechanics: rack's seams and round trips, the EQ's elision
    refusals, the limiter's silence path, builtins-compiler's nine-track allocation audits;
  - the 8-lane graph corpus check.
- **Running at `Simd4` on AArch64 instead of `Simd8`:** G6 (FTZ inert under FPCR.FZ), the idle-ramped
  SVF identity, and graph's 64-track console gates (#916 gates 1 and 3).
- **New 4-lane pins.** These are the same renders as the 8-lane pins, minus the `Simd8` stream. They
  were taken on x86 and match in the emulation. Only CI's arm64 leg confirms them on NEON.
  - compressor `scenario_1006` `5d99e861…`;
  - builtins settled matrix `1c6d3055…`;
  - fused matrix `2703b0f2…` (184 fused blocks).
- **Test value.** No test is new. The cross-target scan's plausible defect is eight-lane code returning
  to the phone builds, through a reverted predicate or a new eight-lane instantiation. No other gate
  catches it on iOS. The rack-policy mutations turn red if the exemption admits any other
  target-feature spelling.

### Findings

1. **Rustdoc builds without `avx2`.** Rustdoc gets no `[target]` rustflags, so the x86 documentation
   pass is a no-`avx2` build. The rendered docs no longer list `Simd8`, `Backend::Simd8` or
   `BankWidth::Eight`. `-D warnings` passes.
2. **A second error in the sub-v3 probe.** The x86 build without `avx2` now also reports E0308 from
   `Backend::current()` having no arm, after the guard's message. The probe still passes.
3. **Stale prose outside the authorized paths.** `docs/TARGET_MATRIX.md`'s memset register and its
   wasm-guest width text predate #1110 and this issue.
4. **Many test cfg sites.** 438 target-feature cfgs were added in test code, one per eight-lane use.
   Product code gained none.

## Sol verdict, attempt 1

**PASS.** There are no H findings. M1–M3 should be fixed before merge; L1–L4 are notes.

Sol, 2026-10-01, on `7b13a205` (code `fa890f13`), base `089ef456`. One `target/`, built with
`CARGO_INCREMENTAL=0` and deleted afterwards. Base and head were built from one scratch path.

### Evidence

**Code generation (fat-LTO release)**
- **x86 `libcapi.so`.** These sections are byte-identical to base: `.text` (3,386,963 B,
  `96c86fd6…`), `.rodata` (`f4e6ad9c…`), `.data.rel.ro` (`38c90f95…`), `.eh_frame`, `.eh_frame_hdr`,
  `.data` and `.gcc_except_table`. The section tables are identical too.
- **iOS `capi` object `__text`.** 2,469,368 → 2,276,340 B. Across the whole archive,
  2,531,428 → 2,338,400 B.
- **Android `.text`.** 2,377,700 B (2,447 sections) → 2,225,188 B (2,411 sections).
- **Eight-lane function symbols** (`llvm-nm`, the `EIGHT_LANE` rule). iOS 27 → 0; Android 28 → 0.
- **Wasm.** The shipped module is `cd49dc1c…d408` at both base and head. In the artifact job the
  named twin is `369d858a…` and the closure is `02ca569e…`.
- **iOS release assembly, per product crate.**
  - Eight-lane lines on base: builtins 451, multiband-compressor 255, gate-expander 173,
    transient-shaper 145, graph 110, parametric-eq 96, soft-clip 4, graph-compiler 2. On head, all
    25 crates have 0.
  - `memset_pattern16`: each lowered row equals the count measured on head (194, 91, 10, 566, 132,
    268), and the other four rows did not move.

**Gates (all pass)**
- fmt, workspace clippy `-D warnings` and rustdoc.
- Clippy `-D warnings` for iOS and Android:
  - the product crates (cross-target);
  - lane, math, console-workload, audit, wasm-gate-corpus, dsp-reference, conformance, target-smoke
    and bench, with `math/lane` and with default features.
- All 32 lint-job steps.
- Tests:
  - debug-a: 96 binaries, 1,150 passed;
  - debug-b: 151 binaries, 834 passed, plus the conformance fixtures;
  - release: 109 passed, plus the FMA cfg, loom, M1 and F1.
- All audit-native steps. `audit capi` reports 0 violations and `pcm_digest` `ff6cdcb96cdcdad5`, and
  the release unit tests give 114 passed.
- `run-wasm-gates.sh` in full: native 358 comparisons and wasm 250, with 0 mismatches. The V8 spill
  check passes.
- `check-cross-targets.sh`, the artifact job, release-shape, the gate self-tests and the docs gates.

**AArch64 legs, emulated as four lanes on x86**

This host has no arm64 runner. The emulation is a scratch copy with `avx2` off, lane's x86 guard
disabled and `neon` read as `sse2`.
- **Debug.**
  - The listing has 1,767 tests, against 1,817 on x86. `judge-skips debug` passes.
  - 231 binaries: 1,730 passed, 0 failed, 38 ignored.
- **Release.**
  - The listing has 110 tests, against 114 on x86. `judge-skips release` passes.
  - The run passes; the LANE-3 rows fail only on arm64.
- **console-workload and G5** both pass.
- **The three new 4-lane pins** pass.
- **The silent-skip scan** finds nothing.

**Mutations**

These were made on a scratch copy; the worktree was never edited.
1. **Eight lanes back on AArch64.** Every `avx2` site was re-keyed to
   `any(target_feature = "avx2", target_arch = "aarch64")`, with the width selectors kept.
   `check-cross-targets.sh` goes red at the memset judge (builtins 376 > 194). With base's ceilings
   restored, it goes red at the eight-lane scan (builtins 451, gate-expander 173, graph 110,
   graph-compiler 2, …).
2. **The `neon` arm of `Backend::current()` dropped.** Cross-target goes red with E0308 on both
   AArch64 rows.
3. **`current()` returns `Simd4` under `avx2`.** target-smoke goes red (`lib.rs:49`).
4. **A `#[cfg(target_feature = "neon")]` item appended to rack.** `check-rack-policy.sh` goes red.
5. **Survivors.**
   - `current()` and `FrameLane` re-keyed on `target_arch` pass cross-target, the lane and workspace
     policies, and the tests (L3).
   - A wrong 4-lane matrix pin passes on x86 (M3).

**Test value**
- **The eight-lane scan.** It catches eight-lane code returning to the phone builds. It is red on
  base and under mutation 1. In graph-compiler and soft-clip nothing else catches that, because their
  memset counts do not move.
- **The three new pins.** They catch the Simd4 matrix or compressor bits moving on NEON. They re-pin
  existing claims; they are not new tests.

### Answers to the brief's questions

1. **438 test cfgs.** They are not the minimum, and consolidating them is warranted (M1).
2. **Feature keying.** Every width decision is feature-keyed:
   - in lane, effect-contract, graph's `FrameLane`, builtins and rack;
   - in conformance, the transient-shaper corpus and the wasm-gate corpus.

   The `target_arch` uses that remain are architectural:
   - the 64-bit and AVX2+FMA guards and `attest_host`;
   - `fpenv`/`softfma`;
   - `wide_impl`'s lowerings;
   - the MXCSR/FPCR tests.

   One width decision is still arch-keyed (L2). host-web's `selected_backend` reports an ABI code;
   it is not a width.
3. **Code-generation identity.** Confirmed as above.
4. **The 4-lane pins.**
   - They are compressor `kernel.rs:3454`, builtins `tests/matrix.rs:387` and `:547`. They are
     needed because each test folds every width into one digest.
   - They can be derived here: each is the `Simd4`-only fold. A wrong pin fails loudly on arm64,
     because it is a plain `assert_eq` with no expected-failure row (M3).
5. **The memset ceilings.** Each crate's ceiling is measured and exact. A rise of 1 fails, and
   eight-lane code coming back raises six of them.
6. **`graph_fixture`.** See L4.
7. **The new gate.** It scans iOS only (L1). It is red on base and green on head, and CI runs it in
   `cross-target` (`qualification.yml:911`).
8. **The gates.** All pass.
9. **Mutations.** See above.

### Findings

**M1. Consolidate the 438 per-site test cfgs before merge.**
- **Where they are.** 86 files. The largest counts:
  - `parametric-eq/src/lib.rs` 50;
  - `rack/src/lib.rs` 26 and `true-peak-limiter/src/lib.rs` 26;
  - `builtins/tests/meter.rs` 24 and `builtins-compiler/src/lib.rs` 24;
  - `graph/src/runtime.rs` 15;
  - `m2_lane_identity.rs` 13 and `effect-runtime/tests/lane_identity.rs` 13;
  - `builtins/src/tests.rs` 12;
  - `g2_kernel_identity.rs` 11;
  - `g1_op_identity.rs` 10 and `gate-expander/tests/state.rs` 10.

  The other 74 files have 1–9 each.
- **What they gate.**
  - 110 are `f::<Simd8>()` lines after `f::<f32>(); f::<Simd4>()`.
  - 57 gate a `use`.
  - About 100 gate an element of a width list.
  - 52 remove a whole test.
- **Repeated lists.** About 20 local re-declarations of the same width list: `WIDTHS` ×6,
  `BANK_WIDTHS` ×3, `VECTOR_BACKENDS`, `BACKENDS`, `LANE_COUNTS`, `FULL_BANKS`, a local `widths` ×5,
  and `type Widest` ×2.
- **The cost, shown by `fa890f13`.** Five tests would have failed only on an AArch64 leg, three of
  them literal `if cfg!(avx2) { 3 + 7 } else { 3 }` counts.
- **The fix: one helper set, each written once.**
  - `BankWidth::ALL` in effect-contract, and `Backend::VECTOR` in lane.
  - A lane test-support macro `each_lane!(|L| …)` over `f32`, `Simd4` and, with `avx2`, `Simd8`. A
    macro's `#[cfg]` is evaluated in the crate that expands it, exactly as `match_bank_width!`'s is.
  - A native-width alias, `lane::Native`, keyed by the same two predicates. Graph's `FrameLane` can
    be this alias.
- **Expected effect.** About 250–300 of the 438 cfgs go, and M2 is fixed with them.

**M2. Width-generic tests were compiled out of 4-lane builds instead of run at `Simd4`.**
- **The problem.** AArch64 loses its only coverage of claims the phones execute. Slice item 2 asks
  that AArch64 compare Scalar with `Simd4`. About 30 of the 52 removed tests check width-independent
  mechanics at W8 alone, and some removed their own scalar and W4 legs with them.
- **The limiter.** `true-peak-limiter/src/lib.rs`:
  - `:6908`, #182 S2's headline silence gate; its scalar and W4 legs went with it;
  - `:7004`, the −0.0 input case; its scalar and W4 legs went with it;
  - `:7146` and `:7395`.
- **The EQ's whole elision-refusal family.** `parametric-eq/src/lib.rs:5070`, 5102, 5133, 5170,
  5214, 5400, 5707, 5751 and 5803.
- **G4 lane-wise flush.** `lane/tests/g4_flush.rs:114`, the only mixed-lane flush check. On base it
  ran on NEON halves.
- **The banked allocation gates.** `builtins-compiler/tests/allocation_tracker.rs:302` and `:493`.
  The fixture already takes `backend` and `n`.
- **Other tests.**
  - `gate-expander/tests/state.rs:292`, 396, 605 and 721;
  - `rack/src/lib.rs:4991`, 5254, 5931, 5971, 6004 and 6228;
  - `builtins/src/tests.rs:254` and `:295`.
- **The fix.** Run these at M1's native width. Keep a cfg only on the true W8 twins and on the
  other-width probes.

**M3. The three new 4-lane pins can be checked here, but only arm64 checks them.**
- **Evidence.** The 4-lane emulation passes all three. Under mutation, a flipped digit at
  `builtins/tests/matrix.rs:390` stays green on x86.
- **The risk.** An intended bit change in future will be re-pinned on x86, and nobody can verify
  the 4-lane twin locally.
- **The fix.** On every build, hash the `Simd4` stream into its own sink and assert its pin, so that
  x86 checks both pins. The sites are `compressor/src/kernel.rs:3448–3461` and
  `builtins/tests/matrix.rs:387` and `:547`, including `FUSED_BLOCKS`.

**L1. The eight-lane scan reads iOS assembly only** (`scripts/check-cross-targets.sh:118–145`).
Gate 1 named Android too. Android's cfgs are the same as iOS's, and its library measured 0 here, so
only a `target_os`-keyed regression could slip through. Note this, or add an Android `llvm-nm` pass.

**L2. A width is still keyed on `target_arch`** in `tools/audit/src/vectorization.rs:32–43` and
`50–128`: `lane::Simd8` probes under `x86_64` and `Simd4` probes under `aarch64`. A 4-lane x86 build
cannot compile `audit` (E0433 in the emulation). The fix is to key the `Simd8` probes on
`all(target_arch = "x86_64", target_feature = "avx2")`.

**L3. Nothing enforces the owner's rule mechanically.** Mutation 5 survives every gate.
- The 4-lane predicate is written twice in product code: `crates/lane/src/backend.rs:54` and
  `crates/graph/src/runtime.rs:322`.
- It is written twice more in tests: target-smoke, and `soft-clip/tests/support/mod.rs:107`.

M1's alias would leave a single copy. A policy rule refusing `target_arch` next to
`Simd4|Simd8|BankWidth|Backend` would make the rule enforced.

**L4. `graph_fixture` is acceptable as is.**
- Refusing on 4-lane builds is right: the binary lives in `src/bin`, a dev tool that is not linked
  into `capi`.
- `#[ignore]` with a reason is the form `run-aarch64-tests.sh` sanctions.
- What arm64 loses is the byte-for-byte check of the plan compiler's corpus.
- `check_rejects_fixture_manifest_missing_and_unlisted_corruption` (`graph_fixture.rs:461`) does not
  depend on width, so it could generate its fixture at `Simd4`.

## Post-verdict fixes (root-directed)

Terra, 2026-10-01, on `codex/1112-drop-simd8-aarch64` after Sol's verdict (`62c51471`). This fixes M1,
M2, M3, L1 and L2. L3 is reduced; L4 is untouched, as accepted.
- **Commits.** `0b2f0662` (the helpers), `7c8971b4` (the tests and gates) and `e2eb2469` (a
  symbol-name fix).
- **Product code.** No product line moved. Graph's `FrameLane` is now an alias of `lane::Native`, and
  that is the same type.

### M1: one source of the widths

- **The helpers.** These are product-visible facts of the build. None is called from product code,
  so they emit no code. They are not behind `test-support`: `lane`'s own tests and `math`'s run in
  the AArch64 release leg without that feature.
  - **`lane::Native`.** The build's own lane type, keyed by the two width predicates. A compile-time
    assertion holds it to `Backend::current()`. Graph's `FrameLane` is this alias, so the 4-lane
    predicate now lives only in `lane`.
  - **`Backend::VECTOR` and `BankWidth::ALL`.** The vector backends and bank widths this build has.
  - **`BankWidth::backend()`.** The inverse of `for_backend`.
  - **`lane::each_lane!` and `each_vector_lane!`.** They run a body once per lane type. Like
    `match_bank_width!`, their `cfg` is expanded in the calling crate.
- **The rework.** Every test file was rebuilt from its `089ef456` text onto the helpers.
  - **Count.** Test-code `target_feature` lines added since the product commit fell from 443 to 35.
    The measure is `git diff 2d8c710b HEAD | grep '^+' | grep -c target_feature`, less the helpers'
    6 lines and audit's 14 re-keyed lines. (The `7c8971b4` message says 36; this count is the
    right one.)
  - **What the 35 are.**
    - 11 gate true eight-lane twins.
    - 10 gate other-width probes.
    - 6 select the 8-lane pins.
    - 4 are `graph_fixture`'s (L4).
    - 4 re-key lines that were `target_arch` at base: target-smoke's pin, and the existing ignores in
      gate-expander `identity` and host-core `symmetry_witness`.
- **Two side effects.**
  - The gate-expander support helpers `prepare_bank_w8` and `packed_w8` are now `prepare_bank_native`
    and `packed`.
  - builtins-compiler's test-support pair graph is one native bank plus a tail, with its track
    count exported as `TEST_ONLY_PAIR_GRAPH_TRACKS`: 9 at eight lanes, 5 at four.

### M2: no test silently leaves the AArch64 legs

- **What runs at the native width now.** Every test Sol listed runs at the build's own width
  (`lane::Native`) or at every width it has. On x86 they run at eight lanes exactly as before.
  - limiter: the silence gate and its −0.0, stale-history and automation cases;
  - the EQ's nine elision refusals;
  - G4's lane-wise flush;
  - the banked allocation audits;
  - gate-expander's state tests, which base ignored on arm64;
  - rack's seams, round trips and fold arming;
  - the G2 meter merge and post-ramp symmetry, the multiband resets, `m1_a_zero_seeded…`.
- **Other-width probes.** Where it can, a probe iterates the build's other widths, which is an empty
  loop on a 4-lane build.
- **Listings.** The base listing is `089ef456`'s x86 listing, which equals arm64's apart from
  `target_arch` tests, which this issue does not touch. The 4-lane listing comes from the
  emulation below.

  | Leg | `089ef456` | 4-lane | Removed |
  |---|---|---|---|
  | debug | 1,817 | 1,807 | 10 |
  | release (lane, math) | 114 | 113 | 1 |
  | console-workload | 70 | 68 | 2 |

  No test was added or renamed. The x86 listings equal base's exactly. `judge-skips` passes both
  legs.
- **Every removed test is a true eight-lane twin** whose four-lane twin still runs:
  - `randomized_differential_simd8` (twin `_simd4`);
  - `the_hot_console_renders_the_pre_990_words_at_simd8` (twin `_at_simd4`);
  - `mixed_elision_matches_frozen_bodies_w8` (twin `_w4`), in both legs;
  - the EQ's three `ramping_elision::…_simd8` tests (twins `_simd4`);
  - `detector_chunk_active_window_matches_old_shape_w8` (twin `_w4`);
  - `the_composition_is_partition_invariant_at_width_eight` (twin `_at_width_four`);
  - console-workload's `eq_only_gain_rides…_at_simd8` and `mono_console_both_rides…_at_simd8` (twins
    `_at_simd4`).
- **Or it is an other-width probe**, which needs a second width:
  - transient-shaper's `bank_resources_and_validation_precede_legal_unavailable_fallback`;
  - rack's `wrong_width_provider_is_dropped_before_complete_staged_fallback`, whose provider always
    hands four lanes to an eight-lane chain.
- **Attempt 1 removed 50 debug tests; 10 are removed now.**
- **Ignored on 4-lane builds, each with its reason.** These stay in the listing.
  - compressor `bank_fallback_never_hides_malformed_or_incompatible_requests`, an other-width probe;
  - console-workload `a_seam_side_only_chain_reads_as_vacuous_rather_than_eligible`, which attempt 1
    had compiled out;
  - `graph_fixture`'s two tests;
  - host-core `the_scalar_live_control_effect_arm_maintains_its_own_live_terms`, unchanged since
    #1017.

### M3: the 4-lane pins fail locally

- `scenario_1006` and both builtins matrix scenarios hash the four-lane stream into a sink of its
  own on every build, and assert its pin, with the fused-block count 184. Avx2 builds also assert
  the existing all-width pins and 368.
- On x86, a flipped digit in each of the three 4-lane pins turns its test red: settled matrix, fused
  matrix and `scenario_1006`. `FOUR_FUSED_BLOCKS = 183` is red too. All are reverted.

### L1, L2 and the remaining notes

- **L1.** `check-cross-targets.sh` also emits the Android `capi` release staticlib's assembly. With
  fat LTO it is one module, every function the app ships after inlining, and needs no NDK. The
  `EIGHT_LANE` rule is applied to it.
  - On `089ef456` it matches 832 lines and 28 function labels, and the row's own block fails.
  - On head it matches 0. Locally the whole script takes about 3 minutes.
- **L2.** Audit's vectorization probes key the lane width on `avx2` (the `Simd8` probes,
  `x86_64-avx2`) and on `neon` (the `Simd4` probes, `aarch64-neon`).
  - On x86 the audit record is unchanged: backend `x86_64-avx2`, and the same disassembly and
    allowlist digests.
  - A 4-lane build compiles `audit`, which Sol's E0433 showed it could not.
- **L3, reduced.** The 4-lane predicate is written once in `lane::Native` and once in
  `Backend::current()`, and the assertion holds the two together. soft-clip's support copy is gone,
  and target-smoke keeps its literal pin. No policy rule was added.
- **A named-twin side effect, fixed by `e2eb2469`.** The first placement of `BankWidth::ALL`, a new
  impl at the crate root, renumbered the root's derived impls.
  - Four function names in the browser module's named twin changed (`b05b0acc`). The shipped module
    was `cd49dc1c` regardless.
  - The impl now sits in a submodule, and the named twin is `369d858a` again.

### Gates, all on the final tree

- **Code generation.**
  - x86 `libcapi.so`: all 24 allocated sections and the NOBITS headers are byte-identical to
    `089ef456`'s, built from a `git archive` of it.
  - Wasm: shipped `cd49dc1c…`, named twin `369d858a…`, closure `02ca569e…`.
  - iOS `capi` `__text` is 2,276,340 B and Android `.text` 2,225,188 B, as in attempt 1. Both have 0
    eight-lane functions.
- **AArch64, emulated 4-lane** (scratch copy, as before; `audit` included):
  - debug leg: 231 binaries, 1,769 passed, 0 failed, 39 ignored;
  - release lane and math: 99 passed;
  - console-workload: 65 passed;
  - G5: 9 passed at two widths;
  - `audit capi`: 0 violations, `pcm_digest` `ff6cdcb96cdcdad5`, the x86 digest;
  - the delay, compressor, EQ and gate audits: 0 violations, `bank_available` true.
- **x86.**
  - debug-a: 96 binaries, 1,150 passed;
  - debug-b: 151 binaries, 834 passed, plus the conformance fixtures;
  - release: 109 passed, including `g5_native_digests_match_pins`, plus FMA, loom, M1 and F1;
  - every audit-native step, with `audit capi` `pcm_digest` `ff6cdcb96cdcdad5`.
- **Also green.**
  - `run-wasm-gates.sh`: native 358, wasm 250, 0 mismatches, 7 limiter functions scanned.
  - `check-cross-targets.sh`, with both eight-lane rows and the unchanged memset ceilings.
  - fmt; workspace clippy `-D warnings`; and clippy `-D warnings` on iOS and Android for the
    product crates (`--all-features`), the debug leg's set, the release leg's set (`math/lane`) and
    default features.
  - rustdoc `-D warnings`.
  - All 32 lint-job steps and the three sub-v3 probes.
  - The routing check and its tests.
  - The silent-skip scan (rg exit 1) and the known-defect self-test.
