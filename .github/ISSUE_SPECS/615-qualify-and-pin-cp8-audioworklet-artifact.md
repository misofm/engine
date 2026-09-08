# Qualify and pin the CP8 AudioWorklet artifact

GitHub: https://github.com/misofm/engine/issues/615

Parent: #614. Coordination: #560. Passive dependent: #539.

The source-accepted CP8 branch at frozen checkpoint `0c715de9fbdf0b3873c707e10c52095fad750287` changes code compiled into `host-web`. Its one ordinary no-bypass build completed compilation and correctly stopped before publishing because observed simd128 Wasm SHA-256 `e338adae98454d0a365c0ef281aa6b3dcb24d5dc0a36427f917566682e0ff27b` differs from the delivered pin `39ebe7cd3f71f34ab11260f27fa1eaad281dd61642c50d9ed6210e703d95dd55`. The complete failed-probe record is preserved under `artifacts/issue614-artifact-probe/` at parent head `d47a8ededad4a6c4549b35a49ac3d134c97744a0`.

This issue is the bounded artifact-promotion successor required by #614's anticipated drift rule. #614 and this issue occupy the two active slots. #539 remains open as a passive dependent: its separate limiter candidate `7e242eb8f283bcba2ef5778cd430950fee7df92ef66b6ddaaf2a0a68e5bc7409` is not qualified here, and #539 must integrate the delivered CP8/main state and make its own later artifact decision.

Sol HIGH coordinates, owns artifact qualification/pinning decisions, checkpoints, GitHub synchronization and delivery. Astra LOW performs every scope, scratch-candidate, source/pin, exact-head and delivery verification. Luna HIGH or XHIGH performs any authorized repository edits. No benchmark or timing workload is allowed.

## Frozen source and scratch qualification

The artifact source is exactly `0c715de9fbdf0b3873c707e10c52095fad750287`; later issue/evidence commits do not change compiled inputs. Before qualification, verify the source commit is an ancestor of the clean pushed successor head, current main is still the recorded merge-base or document and review any integration, and these inputs match the failed probe:

- `scripts/build-web-audioworklet.sh`
- `Cargo.lock`, root `Cargo.toml`, `rust-toolchain.toml`, and `.cargo/config.toml`
- the four shipped web source/pin files
- the CP8 `effect-contract` and `effect-package` source identities

Create one isolated scratch checkout from the frozen source. The scratch checkout may differ only at `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256`, provisionally set to the observed candidate digest plus LF. Prove that one-file overlay and run the unchanged ordinary builder once to an empty external directory. It must emit exactly six files and the Wasm must hash to the frozen candidate. A different digest, extra output, build failure, or any other scratch source change is FAIL and stops this issue before repository edits.

Preserve exact argv, cwd, source/toolchain/config/input hashes, overlay diff, full streams/status, output census, six-file hashes and checksum manifest. Compare the five non-Wasm outputs byte-for-byte to the delivered #587/#608 manifest; record any mismatch and stop for rescope.

After that exact candidate and five-file identity are established, the scratch checkout may also
change only `hosts/host-web/qualification/results.json`'s `candidateCommit` and `wasmSha256` to
the frozen source and candidate, then regenerate only
`hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md` with the unchanged generator. Prove this second
scratch overlay separately and freeze every browser result row, version floor, gate and resource
value. These temporary lineage files allow `run.mjs --check-matrix` to validate the candidate
before browser launch; they are qualification inputs, not repository delivery edits.

## Qualification gates

On that exact six-file candidate and scratch source, run the repository's existing gates without changing them:

1. Shipped Wasm ABI/export/import/memory/realtime-callgraph/SIMD/static/metadata/vocabulary/resource checks with `scripts/check-web-audioworklet.sh`.
2. Hermetic host/worklet policy and mutation checks with an isolated `CARGO_TARGET_DIR` through `scripts/test-web-audioworklet.sh`.
3. Existing SDK package/generated-surface checks that apply to the six-file artifact.
4. Install only the locked browser qualification dependencies with `npm ci --ignore-scripts`.
5. Run Chromium, Firefox and WebKit qualification against the candidate with matrix checking and self-test mutations. Preserve actual browser versions, AudioWorklet boot/control/observation/stall, native-corpus PCM identity, lineage, resource and mutation results.
6. Verify the qualification output changes only the candidate source/digest lineage expected for this candidate. Browser outcome rows, version floors, gate vocabulary and resource limits must remain unchanged unless a real failure stops the issue.

Astra LOW must return candidate-qualification PASS over the complete retained evidence before any repository pin or consumer-lineage edit.

## Conditional repository edits and delivery

After candidate PASS, Luna HIGH/XHIGH may change only:

- `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256` to the approved candidate digest plus LF;
- `hosts/host-web/qualification/results.json` only for `candidateCommit` and `wasmSha256`;
- `hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md` only through the unchanged generator, yielding matching lineage text;
- this numbered spec/evidence as root-owned records.

No Rust, JS/TS, ABI, metadata, browser result row, version floor, resource expectation, dependency, lockfile, toolchain/config, script, policy, workflow, corpus, fixture, DSP, session, SDK surface or #539 file may change.

Root checkpoints the exact edit before further work. Run one ordinary no-bypass post-pin build from the clean repository head and require exact six-file identity with the qualified scratch candidate. Re-run the static/resource/hermetic/SDK gates and proportional formatting/diff/policy checks. Astra LOW then reviews exact pushed head/current main, source ancestry, evidence checksums, three-file edit, generated lineage, pin and post-pin identity.

Open one PR only after that PASS. Require the repository's `qualification` check, verify live main immediately before exact-head merge, verify merge parents and post-main qualification, then synchronize and close this issue and #614. Update #560 and #559 so #539 can resume, and remove clean delivered worktrees while retaining branches/history.

One scratch qualification and one Luna repository-edit attempt are authorized. A candidate mismatch or substantive gate failure stops for a reviewed rescope; do not retry browser or build workloads to obtain a green result. The repository three-attempt limit remains binding.

## Initial Astra LOW scope review — FAIL

Astra LOW returned **FAIL** at clean pushed head
`30efe9ebbe02f88825da3d3eb91b7ba2f5f2fcff`. The one-pin scratch rule could not satisfy
`run.mjs --check-matrix`, which validates retained old lineage before browser launch, and four
probe evidence files had one extra EOF blank line. The probe's 13 checksums, status 1, empty output,
observed digest, frozen ancestry and unchanged compiled inputs otherwise passed. Root has added only
the scratch lineage-overlay rule above, removed the four extra blank lines and refreshed the
evidence checksum manifest. Raw build stdout/stderr/status and all substantive probe content remain
unchanged. Scratch qualification remains unauthorized until Astra LOW passes the corrected head.

## Corrected Astra LOW scope review — PASS

Astra LOW returned **PASS** at exact clean pushed head
`c5dc394d7df23db40e099fe828ee6fc70e249bb0`, current main
`9e113be98cf31c1eaf4297b0a031518244b71c33`, and synchronized tracker
`3113cfe9ba95155082e68a3e9a1745806af855e0`. Only the intended corrections changed. All 13 probe
evidence checksums pass, raw build streams/status remain unchanged, diff hygiene passes, and local/
remote #615/#560 match. The scratch lineage overlay is correctly conditional on exact candidate and
five-file identity and limited to candidate/digest plus generated matrix, with all browser rows and
resources frozen. One specified scratch qualification is authorized. Repository pin/lineage edits,
retries and #539's distinct candidate remain unauthorized.

## Astra LOW scratch candidate qualification — PASS

Astra LOW returned **PASS** for the single authorized qualification at frozen source
`0c715de9fbdf0b3873c707e10c52095fad750287`; no command was retried. The one scratch-overlay
builder invocation emitted exactly six files. Candidate Wasm SHA-256 is
`e338adae98454d0a365c0ef281aa6b3dcb24d5dc0a36427f917566682e0ff27b`; the other five files match
the delivered #587/#608 manifest exactly: ABI `40f6fe2e…`, declaration `445254e7…`, host JavaScript
`21c8947d…`, worklet JavaScript `225bc060…`, and metadata `6eac2cb3…`.

Every recorded command exited zero: matrix generation/check, shipped static/realtime checks,
expected resources/native witness and 26 red mutations, hermetic worklet policies/mutations, both
locked npm installs, SDK package gate with 11 tests, and exactly one all-browser qualification with
matrix and self-test mutations. Chromium 151.0.7922.34, Firefox 153.0 and WebKit 26.5 passed. The
final scratch diff is exactly the provisional pin, two results lineage fields and generated matrix
lineage paragraph; browser rows, versions, gates and resource values are unchanged.

Root copied the bounded raw command/context/stream/status, overlay, identity, six-hash and
orchestration evidence to `artifacts/issue615-scratch-qualification/`. Generated six-file outputs
and Cargo build caches remain outside Git; their exact hashes are retained. The repository manifest
covers all 55 retained files and verifies. Astra's PASS authorizes root to preserve this evidence
and Luna HIGH/XHIGH to make the conditional three-file repository promotion. It does not itself
authorize merge or #539's different candidate.

## Luna HIGH repository promotion checkpoint

Luna HIGH changed exactly the three conditionally authorized files, and root checkpointed/pushed
the tranche as `09a86c3e3dc3cad67afb16cef874c6ef9740d985`. The pin is the approved
`e338adae98454d0a365c0ef281aa6b3dcb24d5dc0a36427f917566682e0ff27b` plus LF;
`results.json` changes only `candidateCommit` to frozen source `0c715de9…` and `wasmSha256`; the
unchanged generator changes only the matrix lineage paragraph. All three files match the qualified
scratch overlay byte-for-byte. Browser rows, versions, gates, resources and every other result are
unchanged. Matrix `--check`, exact pin spelling, formatting and diff checks pass; `Cargo.lock` is
unchanged. No build, browser, install, benchmark or timing command ran in this tranche. Root must
now run the single ordinary post-pin build and exact six-file comparison before Astra review.

## Ordinary post-pin build — PASS

Root ran exactly one ordinary no-bypass builder invocation at clean, pushed promotion evidence head
`5bf40ad03b4ba1c422eb721529ceb88eef30b7c8`. The process exceeded its initial output yield but
continued under the same PID; root polled it and did not launch another builder. It exited zero and
published exactly six files. Every file matches the qualified scratch candidate: ABI `40f6fe2e…`,
declaration `445254e7…`, host JavaScript `21c8947d…`, worklet JavaScript `225bc060…`, metadata
`6eac2cb3…`, and Wasm `e338adae…`. Full streams/status, exact argv/context, input hashes, both
six-file manifests and their empty comparison are preserved under `artifacts/issue615-postpin/`.
No generated output or build target is committed. Astra LOW must run the bounded post-pin gates and
review exact pushed head/current main before PR delivery.

## Initial post-pin exact-head review — FAIL

Astra LOW returned **FAIL** at exact clean pushed head
`5cf6f9f690924115e4d5be7a9b93b553841382c7` only for evidence packaging. Static/object/ABI,
resources/native witness and 26 mutations, hermetic checks, locked SDK install/package with 11
tests, matrix check and formatting each passed once. The reviewer stopped before the remaining
workspace/effect-runtime policy gates because outer `git diff --check origin/main...HEAD` found
single-space context lines inside the two raw captured overlay diffs. Frozen ancestry, exact
three-file promotion, unchanged results/resources, six post-pin hashes, 13/55/11-entry evidence
manifests, lock/source invariance and absence of committed binaries/caches all passed.

Root losslessly compressed only `final-overlay.diff` and `second-overlay.diff` with deterministic
gzip and refreshed the scratch-evidence manifest. Original capture bytes decompress unchanged; no
qualification command was rerun. Attempted source/artifact behavior remains accepted. Astra LOW
must verify the corrected packaging, run only the policy gates that stopped before execution, and
confirm exact head/current main before PR creation.
