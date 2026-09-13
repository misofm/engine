# Qualify the AudioWorklet artifact after native input-filter response delivery

Source dependency: #767. Parent outcome: #763. This bounded half-day delivery slice reuses the accepted #766 workflow in `.github/ISSUE_SPECS/766-qualify-and-pin-the-audioworklet-artifact-after-native-eq-response-delivery.md`; it adds no qualification framework or browser analysis endpoint.

## Scope and start condition

#767 adds an owner-native HPF/LPF response module. Its effect on shipped Wasm bytes is **not yet observed**. Start only after Astra medium records #767 SOURCE PASS and root checkpoints/pushes the exact accepted source/test commit. Record that 40-hex identity as `FILTER_ARTIFACT_SOURCE`; later pin/results/spec commits do not replace it. Freeze source/tests throughout this qualification.

Delivered reference source is `7dcb127518d0666f6cbe809053732b215defeb9f` (PR #765). Its qualified Wasm digest is `b3422caa59e95b8e5a9e20e591bf5e7341ba7216b155a6790fdd72352cb0df69`. #766 records all six artifact hashes, its successful eight gates, and browser/source lineage. Its retained artifact is `/tmp/issue766-attempt1-artifact` and authentic logs are `/tmp/issue766-attempt1-logs`; the checked-in #766 evidence remains the reference if those external copies are unavailable.

Astra xhigh approves this scope; Luna max performs qualification and Astra medium reviews one coherent verdict per attempt, maximum five attempts. Root numbers/synchronizes this brief before any artifact edit. #767 becomes a passive source dependency while this slice owns the implementation slot. All discovery, builds, and gates are pending the start condition; this brief records no execution or observed mismatch.

## Allowed tracked paths

- `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256`, only if the discovered digest changes;
- `hosts/host-web/qualification/results.json` and generated `hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md`, only if new bytes require a successful recorded browser run;
- this numbered issue spec and concise #767/#763 delivery records.

Do not change accepted source/tests, ABI/catalog/generated SDK assets, dependencies/locks/toolchain/build flags, scripts/workflows, browser versions/floors, resource expectations, or PCM fixtures/digests. A discrepancy requiring such changes stops for a bounded scope ruling. Do not change build settings merely to recover the old hash. No timing workload, publication, or extra matrix is included.

## One discovery and ordinary artifact

Record accepted-source/current-head/current-main identities and the source-to-head allowed-path audit. Preserve actual invocation, working directory, relevant build settings, stdout/stderr, and exit per command in ordinary external logs, as in #766. Record Rust/Cargo, Node/npm, and wasm-objdump versions; use the unchanged pinned build and locked prerequisites. If needed only:

```sh
npm --prefix sdk ci --ignore-scripts
npm --prefix hosts/host-web/qualification ci --ignore-scripts
hosts/host-web/qualification/node_modules/.bin/playwright install chromium firefox webkit
```

Use fresh preexisting empty nonsymlink external directories bound to task-specific variables `FILTER_ARTIFACT_PROBE` and `FILTER_ARTIFACT_DIR`. Discover exactly once:

```sh
MISO_ENGINE_WEB_AUDIOWORKLET_REPIN=1 bash scripts/build-web-audioworklet.sh "$FILTER_ARTIFACT_PROBE"
```

Require exit 0, exactly one lowercase 64-hex digest plus LF on stdout, and an empty probe directory. This is an unqualified candidate. If unchanged, retain the pin. If changed, write exactly the discovered digest plus LF to the allowed pin and record its provisional status. Root checkpoints/pushes this coherent discovery/pin tranche and audits status/upstream before further work; an unchanged candidate needs an evidence-only checkpoint, not an artificial pin change.

Produce one ordinary artifact, explicitly unsetting report mode:

```sh
env -u MISO_ENGINE_WEB_AUDIOWORKLET_REPIN bash scripts/build-web-audioworklet.sh "$FILTER_ARTIFACT_DIR"
```

Require exit 0, exactly the six regular nonsymlink files named by the existing checker, and Wasm candidate/pin/output digest agreement. A different ordinary-build digest stops for attribution, not a second speculative repin. Record the six basename-normalized SHA-256 values.

Prove the five non-Wasm outputs equal their authorities, and those authorities are unchanged from delivered `7dcb1275`:

```sh
cmp hosts/host-web/web/miso-engine-v1-audio-worklet.js "$FILTER_ARTIFACT_DIR/miso-engine-v1-audio-worklet.js"
cmp hosts/host-web/web/miso-engine-v1-audio-worklet-host.js "$FILTER_ARTIFACT_DIR/miso-engine-v1-audio-worklet-host.js"
cmp hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts "$FILTER_ARTIFACT_DIR/miso-engine-v1-audio-worklet-host.d.ts"
cmp sdk/assets/miso-engine-v1-parameter-metadata.json "$FILTER_ARTIFACT_DIR/miso-engine-v1-parameter-metadata.json"
cmp sdk/assets/miso-engine-v1-abi-layout.json "$FILTER_ARTIFACT_DIR/miso-engine-v1-abi-layout.json"
git diff --exit-code 7dcb127518d0666f6cbe809053732b215defeb9f -- hosts/host-web/web/miso-engine-v1-audio-worklet.js hosts/host-web/web/miso-engine-v1-audio-worklet-host.js hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts sdk/assets/miso-engine-v1-parameter-metadata.json sdk/assets/miso-engine-v1-abi-layout.json
```

## Existing gates and two digest outcomes

Use the same ordinary artifact for all consumers; omitting its argument can trigger unwanted additional builds. Run the existing static/source-consumer checks once in either outcome:

```sh
bash scripts/check-web-audioworklet.sh "$FILTER_ARTIFACT_DIR"
python3 -B scripts/check-browser-expected-resources.py --artifacts "$FILTER_ARTIFACT_DIR"
bash scripts/test-web-audioworklet.sh
bash scripts/check-sdk-types.sh
bash scripts/check-sdk-headless.sh "$FILTER_ARTIFACT_DIR"
bash scripts/sdk-package.sh check "$FILTER_ARTIFACT_DIR"
```

These are #766 gates 1–6, including current native/resource-witness parity and existing negative controls. Package check already checks generated SDK surfaces. Do not duplicate it or add native response fixtures here; those belong to #767's accepted source gates.

**Unchanged digest:** additionally require all six actual hashes equal #766's accepted six-hash list. Retain the existing pin, results JSON, matrix, and their historical `candidateCommit` unchanged. This accurately reuses browser qualification of identical six-file payloads; do not relabel an old browser run with #767's commit. Record the new build's source identity and byte-equivalence proof in this issue. No browser rerun or new matrix recording is needed when those payloads, browser/gate inputs, and pinned tooling remain unchanged. An intervening relevant change requires attribution and scope review before this reuse decision.

**Changed digest:** run #766's existing browser gate once for all three browsers, against this exact output, with accepted source lineage and existing mutation checks:

```sh
npm --prefix hosts/host-web/qualification run qualify -- --artifacts "$FILTER_ARTIFACT_DIR" --browser all --record-matrix --candidate-commit "$FILTER_ARTIFACT_SOURCE" --self-test-mutations
```

Require Chromium, Firefox, and WebKit success before accepting generated results/matrix. Compare with the delivered record: expected differences are `candidateCommit`, `wasmSha256`, and corresponding matrix lineage only. Explain any other actual discrepancy and stop for scope review rather than editing rows/expectations to pass.

In either outcome finish with #766's existing eighth gate and the tracked-diff check:

```sh
node hosts/host-web/qualification/generate-matrix.mjs --check
git diff --check
```

Preserve any failure and its actual exit; no silent retry. Successful expensive gates are not rerun for evidence-only commits. There is no requirement to prove that a hash change is layout-only: optional inspection of an already available baseline may describe differences, but section sizes or unused public API claims do not prove universal PCM equivalence. The acceptance claim is existing representative production consumer qualification, not browser execution of the new native response API.

## Review and delivery

Root checkpoints/pushes the allowed result/evidence tranche. Astra medium verifies frozen source identity, report/pin/ordinary-build agreement, six-file census/hashes, five authority comparisons, authentic exits, correct conditional browser evidence, truthful result lineage, unchanged expectations, and allowed tracked diff. No source fixes are layered onto qualification.

Deliver together with #767 through its feature PR, without a separate PR merely to trigger CI. Require successful `qualification` for the exact delivered PR head and successful main qualification after merge. Synchronize GitHub bodies/evidence/states, close the source and delivery children after integrated delivery, and retain #763 open for its remaining capabilities. Clean completed worktrees only after all checkpoints are upstream and external evidence preserved under AGENTS.md. This slice does not discharge the later native provider, SDK analysis streams, engine signal capture/FFT, sample-time correlation, or integration obligations of #763.

## Accepted source freeze

#767 earned Astra medium SOURCE PASS on attempt 2 at `dc070bcda506896fe05b4dfefb931841b6d6caa3`, with Luna max implementation. It is pushed and frozen. Prior full builtin/Clippy/policy/scalar-SIMD Wasm gates passed; final test-only correction passed focused8/Clippy/fmt/diff gates. This exact source identity is `FILTER_ARTIFACT_SOURCE`; later evidence commits do not relabel it. #768 now owns active artifact qualification while #767 is a passive source dependency. Candidate discovery has not run.

## Attempt 1 candidate discovery checkpoint — Luna

The accepted frozen source identity is `dc070bcda506896fe05b4dfefb931841b6d6caa3`. The current
worktree head is `d4ad8bf730c0fae7c6eca4c2a4da090adac1cf00`, and `origin/main` is
`7dcb127518d0666f6cbe809053732b215defeb9f`. The source-to-head audit lists only the #767 issue
spec update and this #768 issue spec; `git diff --exit-code` over `crates/builtins`, `Cargo.lock`,
`hosts`, `sdk` and `scripts` passed. Rust/Cargo are `1.97.1`, Node is `v22.23.2`, npm is
`10.9.8`, and `/usr/bin/wasm-objdump` is `1.0.34`. The identity command's authentic streams and
exit are `/tmp/issue768-attempt1-logs/identity.{stdout,stderr,exit}`, with its exact invocation in
`/tmp/issue768-attempt1-logs/identity.invocation`.

The locked prerequisites were absent in this worktree, so the unchanged allowed setup commands
ran successfully: `npm --prefix sdk ci --ignore-scripts`,
`npm --prefix hosts/host-web/qualification ci --ignore-scripts`, and
`hosts/host-web/qualification/node_modules/.bin/playwright install chromium firefox webkit`.
Each exited `0`; authentic streams, exit files and invocations are preserved as
`/tmp/issue768-attempt1-logs/{sdk-setup,qualification-setup,playwright-setup}.{stdout,stderr,exit,invocation}`.

Fresh empty nonsymlink directories were created at `/tmp/issue768-attempt1-probe` and
`/tmp/issue768-attempt1-artifact`. The official report-mode builder was invoked exactly once from
`/tmp/miso-engine-767`:

```
FILTER_ARTIFACT_SOURCE=dc070bcda506896fe05b4dfefb931841b6d6caa3
FILTER_ARTIFACT_PROBE=/tmp/issue768-attempt1-probe
MISO_ENGINE_WEB_AUDIOWORKLET_REPIN=1 bash scripts/build-web-audioworklet.sh "$FILTER_ARTIFACT_PROBE"
```

It exited `0` and emitted exactly one lowercase 64-hex digest plus LF:
`0e6008d94c4a3a235feed551401983321f1782d73906144e227483803769d941`. Canonical-output and empty
probe checks also exited `0`. The authentic discovery invocation, streams and exits are in
`/tmp/issue768-attempt1-logs/candidate-discovery.{invocation,stdout,stderr,exit}` and
`/tmp/issue768-attempt1-logs/candidate-discovery.verification.{invocation,stdout,stderr,exit}`.

The discovered candidate differs from the delivered pin
`b3422caa59e95b8e5a9e20e591bf5e7341ba7216b155a6790fdd72352cb0df69`, so the allowed pin file now
contains the candidate digest plus LF as a **provisional** pin. The update exited `0`, with the
old/new values and exact invocation preserved at
`/tmp/issue768-attempt1-logs/provisional-pin.{invocation,stdout,stderr,exit}`. No ordinary
artifact was built, no six-file output or result/matrix update was produced, and no qualification
gate ran. This discovery/pin tranche is paused for root's exact-path checkpoint and push audit.

## Attempt 1 ordinary artifact and qualification evidence — Luna

After the provisional pin checkpoint, the unchanged builder ran exactly once with report mode
unset into `/tmp/issue768-attempt1-artifact`:

```
env -u MISO_ENGINE_WEB_AUDIOWORKLET_REPIN bash scripts/build-web-audioworklet.sh /tmp/issue768-attempt1-artifact
```

It exited `0`. The ordinary output contains exactly the six expected regular nonsymlink files, and
the candidate, provisional pin and ordinary Wasm all agree at
`0e6008d94c4a3a235feed551401983321f1782d73906144e227483803769d941`. The exact six normalized
hashes are:

```
40f6fe2e23e1b47500011c14871750a75922ab194136add8b387a4b40eb56919  miso-engine-v1-abi-layout.json
c42c997bc8bdcb21245f54251fff9f95020a0f5305bc24d22fe54386f043f18e  miso-engine-v1-audio-worklet-host.d.ts
59549964170b87f331a7ecb90fa4272734ad09184f891f6e5b46ab71aa0fb425  miso-engine-v1-audio-worklet-host.js
225bc06043ed6e2c62a38d63f1c2015b40480d673e3a53109c938eba481556cb  miso-engine-v1-audio-worklet.js
0e6008d94c4a3a235feed551401983321f1782d73906144e227483803769d941  miso-engine-v1-audio-worklet.simd128.wasm
2b0e1195bbad7e2672de62a4e4f331d21f01f5ce042338ae8e400a4a9afa5585  miso-engine-v1-parameter-metadata.json
```

All five authority comparisons passed, and the delivered `7dcb127518d0666f6cbe809053732b215defeb9f`
diff over those five authorities was empty. Authentic ordinary-build and verification invocation,
streams and exits are `/tmp/issue768-attempt1-logs/ordinary-build.{invocation,stdout,stderr,exit}`
and `/tmp/issue768-attempt1-logs/ordinary-build.verification.{invocation,stdout,stderr,exit}`.

Because the digest changed, every original qualification gate ran once against this same ordinary
artifact and exited `0`:

```
bash scripts/check-web-audioworklet.sh /tmp/issue768-attempt1-artifact
python3 -B scripts/check-browser-expected-resources.py --artifacts /tmp/issue768-attempt1-artifact
bash scripts/test-web-audioworklet.sh
bash scripts/check-sdk-types.sh
bash scripts/check-sdk-headless.sh /tmp/issue768-attempt1-artifact
bash scripts/sdk-package.sh check /tmp/issue768-attempt1-artifact
npm --prefix hosts/host-web/qualification run qualify -- --artifacts /tmp/issue768-attempt1-artifact --browser all --record-matrix --candidate-commit dc070bcda506896fe05b4dfefb931841b6d6caa3 --self-test-mutations
node hosts/host-web/qualification/generate-matrix.mjs --check
git diff --check
```

The per-gate authentic invocation, stdout, stderr and exit files are preserved under
`/tmp/issue768-attempt1-logs/gate-{web-static,browser-resources,web-hermetic,sdk-types,sdk-headless,sdk-package,browser-qualify,matrix-check,diff-check}.{invocation,stdout,stderr,exit}`.
The browser run reported all gates passed for Chromium `151.0.7922.34`, Firefox `153.0` and
WebKit `26.5`, then recorded `results.json` and `BROWSER_DEPLOYMENT_MATRIX.md`. Their only
tracked changes are the expected `candidateCommit`/`wasmSha256` values and corresponding generated
matrix lineage; browser versions, native corpus/resource rows, expectations and gate vocabulary
are unchanged. No source, test, dependency, build-input or qualification-input path changed.

This successful results tranche is paused for root's exact-path checkpoint and push audit. No
successful gate is to be rerun merely to record this evidence.
