# Astra #430/#459 immutable qualification commands

Frozen candidate `7951736605fa64870bc1d91342d00d5fdb6417c5`, `/home/bl/misofm/engine-live-pair-proof`. Source PASS remains e6f54b5f/2a152bf8; integration adds delivered scanner/evidence inputs without runtime/build-input drift. This is an operational command brief, not qualification evidence. No commands below were executed by this review. Root owns execution, retained statuses, pin/evidence edits and final PR/CI.

## Execution discipline

Use the repository root as cwd and `PATH=/home/bl/.cargo/bin:$PATH`. Save each command's combined output and actual exit status separately under `/tmp/engine-430-QUALIFIER.log` and `.status`; do not infer status from the last tee. Preserve failed outputs. Keep tracked tree immutable until current full-workspace session29302 terminates. Its target `/tmp/engine-430-workspace-qualified` is exclusively reserved and must not be shared with any other Cargo command, especially feature/profile changes to the un-hashed capi rlib.

Independent Cargo targets below may run alongside workspace; execution within each named target must be sequential. Limit simultaneous expensive builds by root judgment. No timing, benchmark wrapper, second corpus, AArch64 build or new harness is authorized. Existing non-timed browser qualification is required. Do not run wasm-console or its benchmark wrapper as a substitute for target correctness.

## Supported scalar/SIMD Wasm and protocol

These reproduce the existing #435/#442 supported-target scope on current source. Each target is task-specific; ensure it is not owned by another active command.

```sh
CARGO_TARGET_DIR=/tmp/engine-430-wasm-scalar RUSTFLAGS='-C target-feature=-simd128' cargo build --locked --release --target wasm32-unknown-unknown -p engine -p session -p effect-contract -p effect-compiler -p protocol -p target-smoke -p host-core -p host-web -p lane -p math -p builtins -p builtins-compiler -p effect-runtime -p gate-expander -p delay -p multiband-compressor -p parametric-eq -p soft-clip
rustc --print cfg --target wasm32-unknown-unknown -C target-feature=-simd128
CARGO_TARGET_DIR=/tmp/engine-430-wasm-simd RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --target wasm32-unknown-unknown -p target-smoke -p protocol
rustc --print cfg --target wasm32-unknown-unknown -C target-feature=+simd128
bash scripts/check-protocol-wasm-parity.sh
bash scripts/check-wasm-realtime-atomics.sh /tmp/engine-430-wasm-inspection
```

For both cfg commands capture successful rustc production first. On captured files, exact `target_feature="simd128"` must have rg status1 for scalar,0 for SIMD; pointer atomic support must be present and atomics target feature absent for the scalar inspection. Do not hide failed cfg production inside an inverted pipeline.

Protocol parity's current script takes NO target argument. It owns `target/ci/issue005-wasm-scalar` and `target/ci/issue005-wasm-simd` in this worktree, independently of CARGO_TARGET_DIR. Reserve those two paths; do not run two parity invocations concurrently. It builds both protocol_wasm_golden guests and actually invokes `main` with two i32:0 arguments, requiring the exact returned `main(i32:0, i32:0) => i32:0` text. Empty successful run-all-exports output proves nothing. No repeat mutation campaign is needed for an unchanged parity checker.

IMPORTANT change from historical #435/#442: delivered #427 now supplies the trustworthy atomics gate directly. Do not reconstruct the former /tmp Python supplement. The current script owns a fresh `.wasm-inspection.*` child below the supplied absolute parent; builds engine/source/target-smoke in scalar release with LTO=false; checks cfg, every discovery/sort/archive/extraction/reconciliation/decoder/search status, exactly one archive per family and every object member. It checks observation matches across objects, invoking source ObservationSlot fallback only if no object matches. Preserve its actual printed object count, not an assumed3; successful children are removed and failed children retained according to the script's own cleanup. An optional `bash -x` invocation instead of the plain invocation above can retain per-operation provenance in that SAME execution, without a second build.

This is inspectable **NON-LTO three-family object evidence**, not proof about fat-LTO bitcode or the shipped AudioWorklet. The ordinary scalar18 build and the fresh shipped simd128 artifact are separate claims. Scalar host-web is a compile-supported path, not a second shipped browser artifact. No new scalar/Simd128 matrix corpus is required: #463 owns that separate queued evidence change.

## Native and retained resource/CAPI gates

The live workspace command already tests the native x86-64-v3 default configuration and includes doctests; preserve its terminal result and actual per-suite count. The accepted final source has focused debug/release compiler28, allocation4, graph53 and host62/1ignored with test-support, plus five policies. Those source tests need not be repeated merely because integration changed only scanner/evidence inputs.

Run the current independent resource lifecycle suite in release, using its own native target:

```sh
CARGO_TARGET_DIR=/tmp/engine-430-native-qualified cargo test --locked --release -p capi --test resource_lifecycle
CARGO_TARGET_DIR=/tmp/engine-430-native-qualified cargo build --locked --release -p capi
MISO_ENGINE_CAPI_SKIP_BUILD=1 MISO_ENGINE_CAPI_LIBRARY=/tmp/engine-430-native-qualified/release/libcapi.so MISO_ENGINE_CAPI_STATIC_LIBRARY=/tmp/engine-430-native-qualified/release/libcapi.a bash scripts/check-capi-abi.sh
```

This host is Linux; the explicit shared/static paths avoid the ABI script's default `target/release` lookup. Do not use its --self-test with a redirected Cargo target: that branch has its own hardcoded target lookup, and unrelated ABI mutation expansion is unnecessary here.

The source already charges the two original bank owners/consumer arrays plus the new conservative two-box-pointer outer owner. The independent FaderMatrixBankProcessorMirror is included in graph ownership accounting, not silently attributed to builtin ownership or removed merely because a Concurrent path declines pairing. Retain exact/one-below compile and replacement-cap refusal, maximum single allocation and double-live resource proofs. `external_primitive_double_live_oracle_drives_exact_and_one_below_c_caps` and the remaining lifecycle cases exercise these actual calculations; do not blanket-update resource pins from the DUT to make a gate green.

No native sub-v3 scalar fallback should be enabled: native remains compile-pinned AVX2/FMA. Unchanged lane/math/native attestation gates remain enforced by CI; the already-executed focused W4/W8 real graph identity tests are the representative native product evidence. The scalar18 Wasm leg supplies the supported scalar target build.

## Fresh shipped artifact and its current consumers

After immutable workspace terminal success, root may obtain the digest from the standard builder and update current consumers only. Create two distinct empty output directories (do not overwrite historical evidence): `/tmp/engine-430-worklet-digest` and `/tmp/engine-430-worklet-current`.

```sh
MISO_ENGINE_WEB_AUDIOWORKLET_REPIN=1 CARGO_TARGET_DIR=/tmp/engine-430-artifact-host bash scripts/build-web-audioworklet.sh /tmp/engine-430-worklet-digest
```

REPIN is a digest-producing build, not a populated six-file artifact directory and not normal verification. The builder independently owns a fresh temp SIMD compilation target. Record immutable source79517366 and observed digest. Root updates `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256`, `.github/workflows/npm-publish.yml` EXPECTED_WORKLET_SHA256 and the current artifact statement in `docs/C_ABI_V1_QUALIFICATION.md` to the actual digest where necessary. Preserve historical matrices/records at their original source; do not rewrite historical qualification. Do not publish npm.

Then run the normal builder independently:

```sh
CARGO_TARGET_DIR=/tmp/engine-430-artifact-host bash scripts/build-web-audioworklet.sh /tmp/engine-430-worklet-current
CARGO_TARGET_DIR=/tmp/engine-430-artifact-host bash scripts/check-web-audioworklet.sh /tmp/engine-430-worklet-current
CARGO_TARGET_DIR=/tmp/engine-430-artifact-host python3 -B scripts/check-browser-expected-resources.py --artifacts /tmp/engine-430-worklet-current
CARGO_TARGET_DIR=/tmp/engine-430-artifact-host bash scripts/test-web-audioworklet.sh
```

These host-side metadata/example builds honor the specified target and must be serialized with each other. The builder's separate SIMD temp build is not that host target. The static gate inspects actual shipped bytes/metadata; expected-resources uses those bytes plus the independent native browser_fixture_resources example and automatically runs its26 red comparator controls. Hermetic worklet tests use existing fake WebAssembly/metadata machinery and do not replace real browser execution.

If expected.json resource rows have genuinely changed because of the approved conservative outer allowance, derive the actual Wasm report with the EXISTING instrument:

```sh
MISO_ENGINE_WEB_ORACLE_PRINT=1 node hosts/host-web/tests/browser-v1/direct-oracle.mjs /tmp/engine-430-worklet-current hosts/host-web/tests/browser-v1/expected.json
```

Reconcile each changed resource row with the independent native/target-sensitive accounting and existing CAPI mirror before root edits those current expectations. Existing PCM identity pins are not permission to repin arbitrary audio changes. Retain initial failure and rerun the affected resource/static gate only after a justified correction; a new unrelated discrepancy requires an explicit ruling, not invented tolerance.

## Browser deployment record and checks

Use `hosts/host-web/qualification` as cwd. Existing Node>=20, locked npm dependencies and Playwright Chromium/Firefox/WebKit must be available; if absent, `npm ci` and the existing Playwright install commands are setup, separately recorded. Do not mutate lockfiles to accommodate a missing installation.

```sh
CARGO_TARGET_DIR=/tmp/engine-430-browser-host npm run qualify -- --artifacts /tmp/engine-430-worklet-current --browser all --record-matrix --candidate-commit 7951736605fa64870bc1d91342d00d5fdb6417c5 --self-test-mutations
CARGO_TARGET_DIR=/tmp/engine-430-browser-host npm run qualify -- --artifacts /tmp/engine-430-worklet-current --browser chromium --check-matrix --self-test-mutations
CARGO_TARGET_DIR=/tmp/engine-430-browser-host npm run qualify -- --artifacts /tmp/engine-430-worklet-current --browser firefox --check-matrix --self-test-mutations
CARGO_TARGET_DIR=/tmp/engine-430-browser-host npm run qualify -- --artifacts /tmp/engine-430-worklet-current --browser webkit --check-matrix --self-test-mutations
npm run matrix
```

Execute these sequentially to avoid browsers/resource contention and shared native identity example builds. The all-browser record writes current `qualification/results.json` and deployment matrix; `matrix` regenerates documentation from those actual rows. All executed rows must identify immutable source79517366 and the actual rebuilt digest; evidence-only packaging heads can differ under the accepted immutable-source-candidate convention. Per-browser checks must validate the newly recorded rows, not a stale previous matrix. No generic correctness seal/ChromeDriver workflow is needed in addition to this current Playwright matrix.

Finish with exact digest equality across normal artifact/pin/publisher/current docs/results, retained logs/status/provenance manifest with tracked hash/size coverage, and proportional diff/fmt/changed policy checks after packaging. Final actual PR Astra review and required qualification SUCCESS remain mandatory. No browser success or artifact digest has yet been observed by this review. #431 owns any descriptive timing; this work issues no benchmark authority.
