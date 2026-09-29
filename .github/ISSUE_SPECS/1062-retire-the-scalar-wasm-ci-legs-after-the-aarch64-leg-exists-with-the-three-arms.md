# Retire the scalar-Wasm CI legs after the AArch64 leg exists, with the three arms only they compile

Source: `docs/handoffs/test-value-2026-09-28/TEST-VALUE-AUDIT.md` draft 05, verified in `VERIFY-TEST-VALUE.md`. **The Amendments section supersedes the body.** **Owner ruling (2026-09-28, decision 7):** retire the scalar (non-`simd128`) wasm builds and CI legs **once #1017's AArch64 CI job is running**; remove #1041's scalar-wasm exception in `lane` with them. **Blocked on #1017.** The scalar whole-plan renderer itself stays as a test-only reference (decision 3, #1059).

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

## Amendment (root, 2026-09-28): delete #1059's two transitional `target_arch` cfgs

#1059 (merged) keeps `Backend::Scalar` behind `lane`'s `test-support` feature, plus a scalar-wasm
exception so the scalar-Wasm CI legs still build. That exception carries two `target_arch` cfgs in
`lane` (on `Backend::Scalar` and on its `width()` arm) that repeat the product-target list of
`current()`'s complement arm. On every product target they reduce to `feature = "test-support"`.
When this issue retires the scalar-Wasm legs, delete both cfgs and the exception with them, so
`lane` names no target list outside `current()` (owner ruling: no target-specific code). See
`1059-*.md`, "Sol verdict, attempt 1", finding 1.

## Attempt 1 evidence

Terra, 2026-09-29. Branch `codex/1062-retire-scalar-wasm-legs` from `a8955ad4` (`main`). Commits
`b7fff4ff` (code, CI, scripts), `a6822f22` (docs) and this record.
Gate 1 holds: `aarch64-debug` and `aarch64-release` pass on `ubuntu-24.04-arm` in required CI
(runs `36507292157`, `36506233220`), and `aarch64-release` runs G5 (`cargo test --release -p
wasm-gates --features math/lane`) and `lane`'s G-gates in the shipping profile. #1037 has already
removed the effect-package, descriptor and interchange scripts, so their scalar legs were gone
before this attempt. #1038 has not landed, so the `miso_wasm_simd8` arms are untouched.

### What changed

- **`lane`.** The guard's scalar-wasm exception arm is deleted, so wasm32 without `simd128` gets
  the 64-bit-only `compile_error!`. Root's amendment: `Backend::Scalar` and its `width()` arm are
  now `#[cfg(feature = "test-support")]` alone, and `Backend::current()` has no `Scalar` arm. `lane`
  names a target list only in the guard and in `current()`. The guard's `not(doc)` note is
  corrected: a wasm32 doc pass needs `+simd128` in `RUSTDOCFLAGS` (checked both ways).
- **The other two arms** (spec: `graph/src/runtime.rs`, `target-smoke/src/lib.rs`): `FrameLane =
  f32` and the target-smoke `Scalar` assertion are deleted. `wide_impl.rs`'s portable max/min arm
  is kept as the complement (only its doc changes), because narrowing it to `aarch64` would add a
  target list. host-web's `BACKEND_SCALAR` branch is untouched.
- **CI (`qualification.yml`, `wasm-guests` only).** Deleted: the scalar 18-package build and its cfg
  assertion, the atomics step, and the evidence-crate check's scalar line. The protocol-parity step
  is renamed. `nightly.yml` is not touched.
- **Scripts.**
  - `run-wasm-gates.sh` loses the scalar guest. The flags (`--without-native`, `--without-v8-spill`)
    and #1048's pairing are unchanged.
  - `check-protocol-wasm-parity.sh` loses the scalar variant; its `--self-test` rebuilds at `+simd128`.
  - `check-cross-targets.sh`: the wasm rows run once at `+simd128`, in the same `.../simd` target
    dir, and a new refusal row fails if `lane` compiles for wasm32 without `simd128`.
  - `check-wasm-realtime-atomics.sh` and its test are deleted.
- **Not in the spec's list, but needed.** `check-artifact-evidence-leak.sh` took its only wasm32
  artifact invocation from the workflow's scalar 18-package line. Without that line the gate fails
  ("found no cross-target artifact invocation"). It now also scans the invocation that ships,
  `build-web-audioworklet.sh`'s `-p host-web` build. Its test's cases 1 and 7 mutated the deleted
  line, so they now mutate the delivery script and the evidence-crate line. A new case 1b removes
  the delivery invocation, and the gate must fail on that.
- **Dead tooling of the retired leg.** `wasm-gates`' `ExpectedBackend::Scalar` (`--expect-backend
  scalar`) is removed: no guest can report backend 0 any more.
- **Stale text corrected:** `docs/TARGET_MATRIX.md` (the exception becomes the retirement record,
  with one row per retired leg naming its successor check); `REALTIME_MEMORY.md`,
  `REALTIME_DEPENDENCY_POLICY.md` and `CONTROL_PROTOCOL_CONFORMANCE.md` (atomics and protocol
  claims); the corpus docs of four effect crates, `wasm-gate-corpus`, `wasm-gate-guest`, `bench`,
  the conformance guest, host-web's `BACKEND_SCALAR` doc line (now "what a native build
  reports"), `direct-oracle.mjs`, `build-web-audioworklet.sh` and `check-web-audioworklet.sh`.
  The host-web and effect-crate doc edits keep their line counts: rebuilding after the host-web edit
  gives the same module digest. MUTATIONS.md files and rulings are records and are left alone.

Diff `a8955ad4..` before this record: 32 files, +218/-514 (net -296). The two atomics scripts
account for 290 of the deleted lines, the scalar arms and cfgs in `lane`, `graph` and
`target-smoke` for 54, and the workflow for 14.

### Nothing lost coverage

`docs/TARGET_MATRIX.md`, "Scalar wasm is refused too", has the per-leg table. The measured facts
behind it:

| Retired leg | Its claim | Survivor, and how it was checked here |
|---|---|---|
| scalar 18-package build | the crates build for wasm32 without `simd128` | Subject retired. The refusal row goes red in both mutations: base's tree compiles, and a guard arm without a `current()` arm fails without the message. `cargo tree` shows all 18 crates still built at `simd128`: 16 in `host-web`'s closure (`artifact`), and `target-smoke` and `protocol` in the probe. |
| atomics inspection | no `atomic.` opcode in `engine`, `source`, `target-smoke` | `check-web-audioworklet.sh` on the shipped module, which links `engine` and `source`. It passes on the head module. Gate 3: `--self-test-opcodes` still catches the seeded `i32.atomic.load` and `memory.atomic.notify`. A copy of the module with an injected `env.injected` function import fails with "Wasm imports are forbidden" (exit 1). |
| scalar evidence-crate check | `dsp-reference`, `conformance` build for wasm32 | The `simd128` line (run here). |
| scalar protocol parity | `COMPLETE_SCHEMA_HASH` under wasm32 | The `simd128` variant passes. `--self-test` passes too, now rebuilt at `simd128`: the control is green, and the inert invocation and all 3 red rebuilds are refused. Natively, `conformance_corpus` runs on x86 and in `aarch64-debug`. |
| scalar G5 guest | the corpus at three widths, the three counts, detector residency | The `simd128` guest digests all 142 cases at all three widths (358 comparisons, 0 mismatches, every count 0). `wasm-objdump` shows it holds the limiter at `f32`, `f32x4` and `f32x8`, and detector residency passes on all three. Deleting the exclusion turns it red on the `Simd8` `HotChannel::load` (384 bytes), so the pin is still wired to something. The portable max/min arm (LANE-3) runs natively on arm64 in `g5_native_digests_match_pins` and in `lane`'s G1 (`aarch64-release`). `wide`'s scalar-array fallback is no longer compiled for any accepted target. |
| scalar cross-target rows | 5 crates build for wasm32 without `simd128` | The same rows at `simd128` (run here). |

**G5 and the wasm-gates corpora are unchanged.** No pin file is touched. G5 is still the single
owner (#1048): `check-ci-path-routing.py` and `test-ci-path-routing.py` pass unchanged, and CI
keeps `--without-native --without-v8-spill` in `wasm-guests` with the native leg in `test-release`
and `aarch64-release`.

**The shipped module's code is unchanged.** A local build of the base reproduces its CI-recorded
digest, `a4a2383f…`. The head builds `fd6b034f…`. The code sections disassemble identically. The
129 differing bytes all fall in the data section, at the 16-byte stride of `core::panic::Location`
records: the deleted lines move later line numbers. `artifact-identity` will therefore report
ARTIFACT CHANGED, which is expected.

### Gates

All run on the final tree, x86-64 Linux, `CARGO_INCREMENTAL=0`, `nice`:

- **Gate 2, refusal.** `cargo check --target wasm32-unknown-unknown -p lane` with `-simd128`
  fails with lane's 64-bit-only `compile_error!`, and with `+simd128` it passes. rustc then also
  reports E0308 on `current()`, which has no arm there. `armv7-linux-androideabi` is still refused
  with the same message. The three x86 probes are unchanged: `-avx2,-fma` and `+avx2,-fma` fail
  with "requires x86-64-v3", and `+avx2,+fma` compiles. `cargo doc --target wasm32-unknown-unknown
  -p lane` passes with `RUSTDOCFLAGS=-C target-feature=+simd128` and is refused without it.
- **Gate 2, AArch64.** `check-cross-targets.sh` passes in 357 s: both AArch64 rows checked and
  clippy-clean, the 10 iOS memset rows unchanged, armv7 and scalar wasm refused, and the wasm rows
  at `simd128`. CI's arm64 legs were green on `main` at the base (gate 1). On x86,
  `run-aarch64-tests.sh`'s sets resolve and build: debug is 25 product crates plus
  `dsp-reference`, `conformance` and `target-smoke`, with its nine features, `--all-targets
  --no-run`; release is `-p lane -p math --features math/lane --no-run`, `-p console-workload`
  (run) and `-p audit` (built).
- **Gate 3.** See the atomics row above.
- **Gate 4, class A unchanged.**
  - `run-wasm-gates.sh` default form passes: `ok (native + wasm simd128 + V8 EQ loops)`. The native
    and wasm legs each compare 142 cases in 358 comparisons with 0 mismatches and all three counts
    at 0; the `f64`-lane and meter-block censuses are unchanged; the V8 EQ loops pass (109, 78 and
    53 instructions, no carried slot).
  - The CI form (`--without-v8-spill --without-native`) passes: `ok (wasm simd128; …)`. The script
    was byte-compared before and after both runs.
  - `cargo test --release -p wasm-gates --features math/lane`: `g5_native_digests_match_pins` ok.
  - `check-protocol-wasm-parity.sh` and `--self-test` pass. The effect-package and descriptor
    corpora no longer exist (#1037).
  - Console digests: `cargo test --release -p console-workload` gives 62 passed and 2 ignored.
  - `check-browser-expected-resources.py --artifacts`: rows and digests agree, and the self-test
    catches 26 mutations.
  - No pin, digest or expected-resource file is touched.
- **Artifact gates on the head module:** `check-web-audioworklet.sh`, `test-web-audioworklet.sh`,
  `check-web-audioworklet-v8-spill.py`, and `check-scalar-oracle-absent.py --wasm` (2649 symbols,
  no scalar-path identifier, 4 banked controls) all pass. `--native` passes on `libcapi.so` built
  alone (3612 symbols).
- **Workspace.**
  - `cargo check --workspace --all-targets`, with `--all-features` and with default features,
    passes.
  - `cargo clippy --workspace --all-targets --all-features -- -D warnings` passes.
  - `cargo fmt --all -- --check` and `RUSTDOCFLAGS=-D warnings cargo doc --workspace --no-deps`
    pass.
  - The wasm `simd128` probes pass: `target-smoke` and `protocol`, `dsp-reference` and
    `conformance`, and `lane`, `graph` and `target-smoke` with `--all-targets`.
- **Script gates.**
  - Every `check-*-policy.sh` and `test-*-policy.sh` (22) passes.
  - So do `check-artifact-evidence-leak.sh` and its test (6 mutations, each red for its own reason),
    and the other lint-job shell gates (env vocabulary, unfused seal, conformance boundaries, DSP
    research, effect-runtime and builtins fixtures, parametric-EQ render contract, bench
    preconditions, console fixture, builtins listening, realtime-audit leak, trace validator, and
    the console and protocol benchmark tests).
  - Python, each with `python3 -B`: `check-ci-path-routing.py` and `test-ci-path-routing.py`
    (#1048's pairings unchanged), `check-script-reachability.py` (126 reached) and its test (18
    cases), `check-release-shape.py` with `--self-test`, `check-sdk-deletions.py` with
    `--self-test`, `check-step-vocabulary.py` with `--self-test`, `check-test-support-ci.py` and its
    test, `test-npm-publish-modes.py`, and the `--self-test`s of `check-scalar-oracle-absent.py`,
    `web-audioworklet-identity.py` and `check-web-audioworklet-v8-spill.py`.
  - `check-sdk-generated.sh` passes. actionlint 1.7.12 is clean on every workflow.

### Cost

Measured from the `wasm-guests` job logs of the last four full-route runs (`36484715483`,
`36486469924`, `36506233220`, `36507292157`):

| retired | seconds per run |
|---|---|
| scalar 18-package build | 30.3 to 35.4 |
| atomics step | 29.3 to 31.6 |
| scalar evidence-crate line | 1.5 to 1.8 |
| scalar protocol-parity variant (build and interp) | 7.8 to 9.4 |
| scalar G5 guest (build and wasmtime run) | 30.1 to 35.7 |
| **`wasm-guests` total** | **101.3 to 112.9 (median 107.7)** |

`cross-target`'s scalar rows cannot be split out of its quiet step. Rerun cold at `-j4` from the
base tree on this loaded host, they take 41.2 s, against 44.1 s for the `simd128` rows. So the
estimate is about 40 s. The new refusal row costs 6.5 s cold locally. The net saving is about
140 s, roughly 2.3 runner-minutes per full-route PR. Gate 5 (at least 80 s) is met by
`wasm-guests` alone. Neither job is on the critical path: in `36507292157`, `wasm-guests` took
295 s, `cross-target` 243 s and `aarch64-debug` 703 s. So wall time does not move.

### `cargo test -- --list`

`cargo test --all-features -- --list --format terse` for every package whose source changed
(`lane`, `graph`, `target-smoke`, `bench`, `wasm-gates`, `wasm-gate-guest`, `wasm-gate-corpus`,
`delay`, `multiband-compressor`, `compressor`, `soft-clip`), plus `lane` at default features, at
the base and at the head, each tree in its own target dir: 100 binary listings and 474 test
entries on both sides. With the binary hashes stripped, **the diff is empty**. No other package's list can
move: none of these crates exports a macro, and every deleted arm is compiled out on x86-64. The
`wasm32` and AArch64 test sets gain or lose nothing either: the deleted `target-smoke` assertion
compiled only on scalar wasm, where no test ever ran.

### Follow-up (not done here)

- `scripts/run-protocol-benchmark.sh` wants a scalar wasm `bench.wasm`. No script in the tree
  builds it, and `lane` now refuses that build. Retiring its `wasm_scalar_bytes` record field
  touches the validator, its fixture, `tools/bench/src/protocol.rs` and the env vocabulary
  (`MISO_ENGINE_BENCH_WASM_SCALAR_BYTES`), so it belongs in its own issue.
- `builtins-compiler`'s named panic for a scalar dispatch without `test-support` (#1059) can no
  longer be reached on any target. Removing it is #1059's layout follow-up.
