Filed by root from the #2111 verification (2026-10-26).

## Problem
#2111 ships `miso-engine-v1-blake3.simd128.wasm` in the delivery closure. Two gaps remain:
1. Its Wasm path is tested only in node V8. No committed test runs it in a real browser (Chromium or WebKit), in a Worker. The engine's web tree resolves the default URL to a path that does not exist, so the browser qualification legs take the JS fallback.
2. The release fingerprints only the AudioWorklet module. The BLAKE3 module has no release pin.

## Smallest slice
- One qualification row in Chromium and WebKit that loads the module in a Worker, resolves `"simd128"`, and hashes the corpus with the expected identities.
- A release pin for the BLAKE3 module beside the worklet pin, checked by the identity report and by npm-publish's qualify step, with a test-npm-publish-modes mutation.


## Approved brief and decisions (2026-10-27)
Root approved the packaging/qualification slice above. The owner requested Sol 6.1 high implementation, replacing the standard Terra implementation stage; fresh Astra review follows the independent issue implementations. No runtime DSP, hash algorithm, worklet pin, or store ownership change is authorized.

- The raw host module lives at `/web/stem-store/incremental-blake3.js`, whose existing static default resolves to `/assets/miso-engine-v1-blake3.simd128.wasm`. The qualification server now serves that layout, which also matches the SDK's `dist/browser/stems` to `dist/assets` relationship. The row uses no URL override.
- One Worker row per browser runs in the normal qualification flow; `--blake3-only` runs this bounded packaging gate without rebuilding the SDK or exercising unrelated audio rows. JavaScript fallback is an immediate failure, before corpus hashes.
- The BLAKE3 release pin and `EXPECTED_BLAKE3_SHA256` have independent release lineage, approved by root so adding the BLAKE3 pin does not force an unrelated worklet repin. A package-version change checks both identities; a browser toolchain change requires both qualifications. `results.json.blake3Qualification` records its own candidate, built hash, and both browsers' SIMD completion. `--record-blake3 --blake3-only --browser all` updates only that lineage.
- The identity job reads BLAKE3 from the artifact job's uploaded closure. npm qualification verifies the source pin/environment agreement and the built BLAKE3 bytes; publish/verify retain the existing qualified-archive path. No release is dispatched or package published here.

## Attempt 1 implementation evidence (Sol)
Initial focused verification: identity-report self-test PASS; `scripts/test-npm-publish-modes.py` PASS, including executing the real workflow source-pin and build-digest checks against changed/empty BLAKE3 bytes, stale/missing pin, and stale environment fingerprint. Chromium 151.0.7922.34 and WebKit 26.5 each returned `simd128` and all six existing `fixtures/stem-identity/v1/VECTORS.tsv` identities.

Initial browser artifact provenance: `/home/bl/misofm/engine-work-evidence/easy-fixes-20261027/fresh-astra/browser-artifacts`, reviewed source `08fd9fa7277af1729506dab46762cfdd0904645e`; `git diff` from that source to this worktree's baseline is empty for `hosts/blake3-web`, `Cargo.lock`, `rust-toolchain.toml`, `.cargo/config.toml`, `scripts/build-web-audioworklet.sh`, and `scripts/strip-wasm-names.py`. This reused closure has an older worklet; these scoped rows exercise only its unchanged BLAKE3 module. Fresh Astra will build the integrated closure once and verify it. BLAKE3 SHA256: `ee373d2f549b0d33bdd0324a8833698158274f516e01a7b0c4018347991a18b1`.

Test value for review: the Worker parity row catches an absent/misrouted default asset or browser-specific load/instantiation failure that Node-only Wasm coverage and JavaScript-fallback browser ingest allow to pass. The extended npm workflow test catches a qualify step accepting altered BLAKE3 bytes or a stale/missing source fingerprint while the independently pinned worklet still matches. The extended identity-report self-test catches BLAKE3 release or toolchain checks accepting stale bytes, missing SIMD browser qualification, or malformed candidate lineage independently of the worklet.

Checkpoint status: local implementation and focused gates green; lineage recording and explicit browser red mutation still pending; fresh independent verdict and remote delivery remain root-owned. All verification uses the batch's shared `run-serial.sh`; no Rust build or full browser matrix was run in this worktree.

## Attempt 1 continuation evidence
Implementation checkpoint: `8890792627e9eb1abf49f8626403d9016fad8cc8` (verified as an existing commit). The recorded `blake3Qualification.candidateCommit` is that checkpoint; its two-browser record hashes the unchanged BLAKE3 bytes described above. The worklet's existing `candidateCommit`, `wasmSha256`, browser rows, matrix, and release pin remain unchanged.

- Chromium and WebKit route-revert proof: temporarily removing only the `/assets/` server route makes each scoped row fail with `expected simd128, got javascript`; restoring the route makes both pass the six identities. The reversible proof restored the server before later checks. External logs: `2117/chromium-default-asset-revert-red.log`, `2117/webkit-default-asset-revert-red.log`, `2117/browser-green.log`.
- `scripts/web-audioworklet-identity.py --self-test`: PASS, including independent BLAKE3 pin/environment, stale qualification, JavaScript completion, malformed candidate, and npm toolchain mutations; a BLAKE3-only pin update passes while the worklet pin remains unchanged. Log: `2117/identity-self-test.log`.
- `scripts/test-npm-publish-modes.py`: PASS, including the behavioral BLAKE3 source pin and built-byte mutations. Log: `2117/npm-mode-tests.log`.
- The existing CI routing policy seals the identity job's shape; adding the artifact download therefore required synchronizing its existing step contract and checkout-mutation anchor. The downloaded closure is verified against the artifact job's digest before the report reads BLAKE3. `scripts/check-ci-path-routing.py` and `scripts/test-ci-path-routing.py`: PASS. Logs: `2117/ci-routing-check.log`, `2117/ci-routing-tests.log`.
- Test value of the rewritten checkout mutation: losing the identity job's full-history checkout still turns its owning routing test red after the new download step is inserted; without this check, a shallow checkout could make every report unable to find its base.

Evidence directory for all paths above: `/home/bl/misofm/engine-work-evidence/easy2-20261027/`. `2117/blake3-lineage-record.log` records the successful lineage run. No full audio matrix, Rust build, release dispatch, npm publication, or upstream push was performed in this worktree. Fresh independent Astra review and integration verification remain required, followed by root-owned GitHub synchronization and delivery. Local gates are green; this implementation author does not claim the independent verdict.


## Independent review — attempt 1 PASS (2026-10-27)

Fresh Astra independently reviewed the implementation and its release-policy boundary: a
BLAKE3-only pin update leaves worklet release checks intact, while a worklet pin/environment
or package-version change still invokes the existing worklet requirements. The identity job
checks the downloaded closure before reading its BLAKE3 module; npm qualification executes
both the source-pin and built-byte checks.

The final integrated closure was built once successfully. Its BLAKE3 module remains
`ee373d2f549b0d33bdd0324a8833698158274f516e01a7b0c4018347991a18b1`, matching the committed
pin. Fresh Chromium 151.0.7922.34 and WebKit 26.5 scoped runs each used the raw host module's
default URL, reported `simd128`, and matched all six existing canonical PCM identities.
Independent identity self-tests, npm behavioral mutations, CI routing, workspace/builtins
policies, unfused seal, realtime dependency-leak gate, formatting and workspace Clippy passed;
Clippy also passed after integration of the newer main. The first artifact command was refused
before compilation because the output directory did not exist; that setup rejection is preserved,
the directory was created, and the subsequent actual closure build passed. A separately queued
build was canceled before execution during main synchronization.

Test value: the Worker row catches a missing default asset or browser-specific Wasm failure
that Node and browser JavaScript fallback permit; npm mutations catch altered BLAKE3 bytes or
missing/stale pin/environment while the worklet still matches; identity self-tests catch stale
BLAKE3 qualification, non-SIMD completion, malformed candidate or toolchain disagreement; the
updated routing mutation continues to reject a shallow identity checkout after insertion of the
download step. These reuse the existing corpus and workflow owners.

Evidence is retained under `engine-work-evidence/easy2-20261027/fresh-astra/`:
`artifact-build.log`, `artifact-directory-preflight-rejection.log`, `artifact-sha256.txt`,
`blake3-both.log`, `integrated-policies.log`, and `final-clippy.log`. No release or npm publication
was performed.

All local verification used the shared serial runner (two Cargo jobs, one Rust test thread).
The final combined production source is `24000af983207d5f18aecb2aece5d1a3466741c1`.
Full required PR `qualification`, including both unmutated AArch64 legs and both standard
browser matrices with their mutations, remains mandatory before delivery and issue closure.
Those full browser rows were deliberately deferred to required CI, not reported as local PASS.

## Root co-review and delivery status (2026-10-27)

PASS for attempt1 after independent Astra checkpoint `a666b967fa5e8ae5b816542d6d48c194c59cd62b`. The shipped default Worker URL, SIMD-mode guard and frozen corpus are exercised in both browsers, including a route-revert failure proof. Cold-built BLAKE3 bytes match the new independent pin; release identity, npm behavior and router checks defend the actual publication gates. No release publication is performed.

Implementation and local/native proof are complete. The full required batch qualification remains mandatory before merged delivery and verified GitHub closure. This record does not claim that pending gate has passed.

## Attempt 1 adversarial verdict: required CI FAIL; attempt 2 correction
Required CI for PR #2163, run `40454965624`, lint job `122026314555`, rejected attempt 1 in `validateSourceProvenance`: the continuation's two documentation-line changes to `hosts/host-web/web/stem-store/incremental-blake3.js` changed its full-file SHA256, while its existing provenance owner correctly still pinned `051c39c28ef475a6f44076eca9f6aaa8bfa6797dcde56114dd76ab6755a5b4fe`. Fresh Astra confirmed that only those comments differed from the integrated base. Attempt 1's conditional review approval is superseded by this required-CI failure; its focused packaging checks did not cover that provenance owner.

Attempt 2 (Sol, root-authorized bounded correction) restores the loader's exact bytes from `e8414e96e9b8efc159b9a464a0bf54de79ca716a`. The provenance record, gate, runtime behavior, server routes, Worker proof, release pin and release documentation remain unchanged. No new or rewritten test is needed: the existing stem-store gate owns the source-provenance contract.

Validation: `.github/workflows/qualification.yml`'s exact blocking command, `node scripts/check-stem-store-v1.mjs`, ran through `/home/bl/misofm/engine-work-evidence/easy2-20261027/run-serial.sh` after external engine workloads became quiet and passed (operator path, source provenance, hash/store/pump gates; three opt-in wall-clock budget assertions skipped by the gate's normal lint mode). Log: `/home/bl/misofm/engine-work-evidence/easy2-20261027/2117/attempt2-stem-store-gate.log`. `git diff --quiet` against that base confirms the restored loader is identical. This checkpoint contains only the restoration and this issue record; fresh Astra re-verification and root-owned corrective delivery remain pending.

## Independent review — attempt 2 bounded PASS (2026-10-27)

Fresh Astra reviewed author commit `9d5db062015b3cdf5f639fb02c9cd86ac8a3644a`, integrated at
`5efcef1c9fa24ef8975777e7eb0e8a593c860aff`. The complete source diff from the first PR head is
only restoration of the two original comment lines. Independent SHA256 calculation gives
`051c39c28ef475a6f44076eca9f6aaa8bfa6797dcde56114dd76ab6755a5b4fe`, exactly the unchanged
provenance record; `git diff --exit-code` against `e8414e96e9b8efc159b9a464a0bf54de79ca716a`
also confirms byte-for-byte restoration. No executable behavior or Rust/Wasm compiler input changed.
The preserved author log confirms the exact previously failing CI command passes its operator,
source-provenance, store and pump gates in the workflow's normal lint mode. No gate was weakened
and no pin was changed. This correction introduces no new or rewritten test; the existing
provenance owner discriminated the defect and now passes.

The first PR run also passed both full browser qualifications with `--check-matrix` and
`--self-test-mutations`, the artifact identity/reproducibility/release-pin job, artifact gates,
and SDK qualification. Fresh review retained the Chromium and WebKit job logs, each showing
the SIMD Worker corpus and all qualification gates passing. These results supplement the local
and native Arm evidence; they do not turn the failed first workflow into a PASS. The corrective
PR head must still pass the complete required qualification before merged delivery or closure.

## Final main integration (2026-10-27)

Required qualification run [40456242475](https://github.com/misofm/engine/actions/runs/40456242475) passed all jobs for corrected head `f67d212b57f7da344c8699111292e8569ee96c11`. Before delivery, independently qualified main `d45e728f160b0d5b86757602c72905da7fb5b0d3` was integrated at `5e9d1271cf1f7bca174d9ab0fbb106039c2513f7`; the closed #2138 spec was retired in the following checkpoint.

Fresh Astra bounded static integration review PASS at `aac808e22f8c6fa60a36265329c27784c8a9e1b4`: automation, builtins-compiler, host-web Rust sources, browser fixtures, Arm runner and known-defect inputs remain byte-identical to native proof source `24000af983207d5f18aecb2aece5d1a3466741c1`. Graph and rack compiler sources match main exactly, with no merge adaptation. Native mutation evidence remains applicable. Complete required qualification on this final combined tree is still mandatory before merge and verified closure. No additional local build or test ran concurrently.
