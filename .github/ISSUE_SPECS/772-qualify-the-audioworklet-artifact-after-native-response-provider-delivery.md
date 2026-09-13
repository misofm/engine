# Qualify the AudioWorklet artifact after native response-provider delivery

Source dependency: #770. Parent: #763. Reuse the accepted procedure and pinned build settings in `.github/ISSUE_SPECS/768-qualify-the-audioworklet-artifact-after-native-input-filter-response-delivery.md`, including its #766 reference. This is a half-day artifact delivery slice with no new framework, timing workload, publication, or browser response API.

## Start and allowed scope

#770 changes native owner descriptors/factory traits and adapters. A shipped Wasm digest change is **not yet observed**. Start only after Astra medium SOURCE PASS and root's pushed source/test checkpoint. Record that exact 40-hex commit as `PROVIDER_ARTIFACT_SOURCE`; freeze all source/tests thereafter. Later evidence/pin/result commits do not replace the accepted source identity.

Delivered reference source is `5d8fe1401983da9bd1c731522b9a201f74244261`. Qualified Wasm SHA-256 is `0e6008d94c4a3a235feed551401983321f1782d73906144e227483803769d941`. #768 preserves the complete six-hash list and successful qualification lineage, with artifact `/tmp/issue768-attempt1-artifact` and logs `/tmp/issue768-attempt1-logs`. Retain the historical browser candidate identity accurately; the later delivered merge is the comparison base, not a new browser run.

Astra xhigh approves scope; Luna max qualifies and Astra medium supplies one verdict per coherent attempt, maximum five. Root numbers/synchronizes before any mutation and audits/checkpoints/pushes each coherent tranche. #770 is a passive accepted-source dependency during this qualification. All execution is pending; this brief records no build or result.

Allowed tracked edits only: `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256` if the digest changes; `hosts/host-web/qualification/results.json` and generated `hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md` after a successful required browser run; this numbered spec and concise #770/#763 records. No source/test, ABI/metadata asset, dependency/lock/toolchain/build flag, script/workflow, browser floor/version, resource expectation or PCM fixture changes. Unexpected drift stops for scope review.

## One discovery and one ordinary artifact

Use #768's unchanged builder, prerequisites and logging convention. Record accepted source/current head/main, allowed source-to-head diff, Rust/Cargo, Node/npm and wasm-objdump versions, actual argv/cwd/relevant settings/stdout/stderr/exits. If matching prerequisites are absent only:

```sh
npm --prefix sdk ci --ignore-scripts
npm --prefix hosts/host-web/qualification ci --ignore-scripts
hosts/host-web/qualification/node_modules/.bin/playwright install chromium firefox webkit
```

Bind task-specific `PROVIDER_ARTIFACT_PROBE` and `PROVIDER_ARTIFACT_DIR` to fresh preexisting empty nonsymlink external directories. Discover once:

```sh
MISO_ENGINE_WEB_AUDIOWORKLET_REPIN=1 bash scripts/build-web-audioworklet.sh "$PROVIDER_ARTIFACT_PROBE"
```

Require exit 0, one lowercase 64-hex digest plus LF on stdout, and an empty probe directory. If unchanged, retain the pin; if changed, write exactly the observed digest plus LF as provisional and record it as unqualified. Root checkpoints/pushes the discovery/evidence tranche (plus changed pin if needed) before continuing.

```sh
env -u MISO_ENGINE_WEB_AUDIOWORKLET_REPIN bash scripts/build-web-audioworklet.sh "$PROVIDER_ARTIFACT_DIR"
```

Require exit 0, candidate/pin/output digest agreement, and exactly the six regular nonsymlink files named by the existing checker. Record all six basename-normalized hashes. A second differing digest stops; do not guess another pin. Compare five outputs with their authorities and verify those authorities unchanged from delivered reference:

```sh
cmp hosts/host-web/web/miso-engine-v1-audio-worklet.js "$PROVIDER_ARTIFACT_DIR/miso-engine-v1-audio-worklet.js"
cmp hosts/host-web/web/miso-engine-v1-audio-worklet-host.js "$PROVIDER_ARTIFACT_DIR/miso-engine-v1-audio-worklet-host.js"
cmp hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts "$PROVIDER_ARTIFACT_DIR/miso-engine-v1-audio-worklet-host.d.ts"
cmp sdk/assets/miso-engine-v1-parameter-metadata.json "$PROVIDER_ARTIFACT_DIR/miso-engine-v1-parameter-metadata.json"
cmp sdk/assets/miso-engine-v1-abi-layout.json "$PROVIDER_ARTIFACT_DIR/miso-engine-v1-abi-layout.json"
git diff --exit-code 5d8fe1401983da9bd1c731522b9a201f74244261 -- hosts/host-web/web/miso-engine-v1-audio-worklet.js hosts/host-web/web/miso-engine-v1-audio-worklet-host.js hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts sdk/assets/miso-engine-v1-parameter-metadata.json sdk/assets/miso-engine-v1-abi-layout.json
```

## Seven or eight existing gates

Run these first six gates once against the same ordinary artifact:

```sh
bash scripts/check-web-audioworklet.sh "$PROVIDER_ARTIFACT_DIR"
python3 -B scripts/check-browser-expected-resources.py --artifacts "$PROVIDER_ARTIFACT_DIR"
bash scripts/test-web-audioworklet.sh
bash scripts/check-sdk-types.sh
bash scripts/check-sdk-headless.sh "$PROVIDER_ARTIFACT_DIR"
bash scripts/sdk-package.sh check "$PROVIDER_ARTIFACT_DIR"
```

Do not omit artifact arguments and trigger additional builds. Existing resource/native-witness negative controls remain required; package check already checks generated SDK surfaces.

**Unchanged digest:** require all six actual hashes equal #768's accepted manifest and unchanged browser/gate inputs/pinned tooling. Retain the pin, results and matrix byte-for-byte, including historical `candidateCommit`. Record this source's new build identity and identical-payload proof in the spec. Reuse qualification of identical bytes; no browser rerun or relabeling of the old run. Relevant intervening input drift stops for attribution before reuse.

**Changed digest:** run the existing browser gate exactly once for Chromium, Firefox and WebKit:

```sh
npm --prefix hosts/host-web/qualification run qualify -- --artifacts "$PROVIDER_ARTIFACT_DIR" --browser all --record-matrix --candidate-commit "$PROVIDER_ARTIFACT_SOURCE" --self-test-mutations
```

Require all browsers/mutations to pass before accepting generated results. Expected record differences are `candidateCommit`, `wasmSha256` and corresponding matrix lineage only; preserve and obtain scope review for any other discrepancy. Never hand-edit browser rows or expectations to pass.

In either outcome finish with the seventh unconditional gate, then whitespace validation:

```sh
node hosts/host-web/qualification/generate-matrix.mjs --check
git diff --check
```

Every actual exit must be zero. Preserve failures; no silent retry. No successful expensive gate reruns for evidence-only commits. No layout-only or universal PCM-equivalence proof is required or implied; acceptance covers existing representative production consumers, not browser execution of #770's native API.

## Review and delivery

Root checkpoints/pushes the permitted result/evidence tranche. Astra medium reviews frozen source identity, single discovery/ordinary-output agreement, six-file census/hashes, five unchanged authorities, authentic gate exits, correct conditional browser evidence and truthful lineage, unchanged expectations and allowed diff. No feature fixes are layered into qualification.

Deliver with #770 through its feature PR. Require `qualification` success on the exact final PR head and main after merge; synchronize GitHub records/states and close delivered children with evidence upstream. Preserve external evidence before removing completed clean worktrees. #763 stays open for session binding/composition, generated SDK preview, live analysis and remaining signal/spectrum/time-correlation requirements.

## Scope review

Astra xhigh, 2026-09-13: **APPROVED without workflow changes.** The brief preserves #768's accepted builder, six-file authority checks, conditional reuse versus three-browser qualification, exact accepted-source lineage, existing gates and bounded tracked paths. All execution remains pending #770 SOURCE PASS and root's pushed accepted-source checkpoint. This scope review ran no build, timing, qualification or source mutation and makes no finding about active #770 implementation.

## Accepted source freeze

#770 earned Astra medium SOURCE PASS on attempt 2 at `a5db34217ff7f656765121463c7acd8247f65e1b`, with Luna max implementation. That exact source/test checkpoint is upstream and is `PROVIDER_ARTIFACT_SOURCE`. Focused/Clippy/policy/full-contract/conformance/workspace/scalar-SIMD Wasm gates passed. Later evidence commits do not change source attribution. #770 is now a passive source dependency and #772 owns qualification. Candidate discovery has not yet run.

## Attempt 1 discovery checkpoint

The accepted source identity is `a5db34217ff7f656765121463c7acd8247f65e1b`. At discovery time the
current branch head was `9a1a4a74`, with only the expected docs ancestry after the accepted source;
the source-to-head identity and status audit is preserved at
`/tmp/issue772-attempt1-source-identity.log` with exit
`/tmp/issue772-attempt1-source-identity.exit` (`0`). Tool identities, Rust target details, and
relevant environment settings are at `/tmp/issue772-attempt1-tool-versions.log` with exit
`/tmp/issue772-attempt1-tool-versions.exit` (`0`).

The authorized missing prerequisites were installed successfully: SDK npm CI
(`/tmp/issue772-attempt1-prereq-sdk.log` and `.exit`), qualification npm CI
(`/tmp/issue772-attempt1-prereq-qualification.log` and `.exit`), and Chromium/Firefox/WebKit
Playwright installation (`/tmp/issue772-attempt1-prereq-playwright.log` and `.exit`). Fresh empty
nonsymlink directories were `/tmp/issue772-attempt1-probe` and
`/tmp/issue772-attempt1-artifact`.

Exactly one official report-mode discovery ran with the accepted source and exited `0`:

```text
MISO_ENGINE_WEB_AUDIOWORKLET_REPIN=1 bash scripts/build-web-audioworklet.sh /tmp/issue772-attempt1-probe
```

Its stdout, stderr and exit are `/tmp/issue772-attempt1-discovery.stdout`,
`/tmp/issue772-attempt1-discovery.stderr` and `/tmp/issue772-attempt1-discovery.exit`. The stdout
was exactly one lowercase 64-hex digest plus LF, and the probe remained empty; validation is
preserved at `/tmp/issue772-attempt1-discovery-validation.log` with exit
`/tmp/issue772-attempt1-discovery-validation.exit` (`0`). The observed digest
`68040d1e0089705b18fc43db51a81e36366c9da275178d5993902a708b458bee` differs from the prior pin,
so the pin file now contains exactly that observed digest plus LF as an **unqualified provisional
pin**. No ordinary artifact build or qualification gate has run. This tranche stops for the root
checkpoint before the ordinary build.

## Discovery validation correction before ordinary build

Root found the original diagnostic log printed `digest_record_shape=False` while exiting0, so that log did not establish the claimed shape validation. It is preserved unchanged. Root independently read the original builder stdout and enforced all conditions: exactly65bytes matching lowercase64hex+LF, byte-identical provisional pin, empty probe and actual builder exit0. All checks passed with an enforced zero status; authentic output/exit are `/tmp/issue772-root-discovery-verification.log` and `.exit`. No candidate discovery or build was repeated, and the pin is unchanged. This explicit verification supersedes the original diagnostic's claim; subsequent validation must fail on false predicates.
