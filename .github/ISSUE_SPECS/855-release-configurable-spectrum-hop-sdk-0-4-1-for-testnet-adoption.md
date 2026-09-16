# Release configurable spectrum-hop SDK 0.4.1 for testnet adoption

## Product outcome

Publish one immutable `@misofm/engine` 0.4.1 release so the web adapter and testnet app can adopt the configurable prepared spectrum hop delivered by #851 and #852 and merged in PR #854. The accepted runtime base is clean synchronized `main` commit `9fe7157e3485f016cd62963cb84189fddac9c2b3`; exact-main qualification run 35140015285 passed all required jobs.

The registry's latest SDK is immutable 0.4.0, which predates the spectrum-hop control. Live registry lookup returns parsed E404 for unused 0.4.1. The accepted AudioWorklet source pin is SHA256 `e18acf9ca97af137a1917e52481c4bf962943d6d755369387969f84c3e381106`.

## Smallest closable release slice

Freeze SDK 0.4.1 without changing runtime behavior. One implementation tranche may edit only:

- this numbered issue spec;
- `sdk/package.json`;
- `sdk/package-lock.json`;
- `sdk/README.md` release/install identity only;
- `.github/workflows/npm-publish.yml`; and
- matching release-identity and accepted-pin fixtures in `scripts/test-npm-publish-modes.py`.

Update the package and lock identities from 0.4.0 to 0.4.1. Update the README install command and release-record link to 0.4.1 and this issue. In the publish workflow, update exactly the five 0.4.0 release/job/packed-archive/provenance literals to 0.4.1 and replace the prior accepted Worklet digest with `e18acf9ca97af137a1917e52481c4bf962943d6d755369387969f84c3e381106`. Update only the corresponding version, PURL, pin, normalization, fixture CLI/package, mutation, issue-number, and diagnostic literals in `scripts/test-npm-publish-modes.py`; preserve its historical 0.2.4 normalized baseline and its fixed workflow/step hashes.

No Rust, DSP, ABI, host/runtime, generated asset/layout, policy-default, session-schema, generic publisher, benchmark, or listening work is allowed. Do not alter the compatible omitted-option default established by #852. Preserve V1 wire identities, exact-main SHA enforcement, qualify→publish→verify archive ownership, OIDC-only publication, immutable-version refusal, and SLSA provenance verification.

## Objective gates

Before review, prove:

1. `npm ci --ignore-scripts` in `sdk/`;
2. `python3 -B scripts/test-npm-publish-modes.py`;
3. `node scripts/test-parse-npm-trust-list.mjs`;
4. `bash scripts/test-sdk-artifact-builder-output-contract.sh`;
5. `bash scripts/check-sdk-generated.sh`;
6. `python3 -B scripts/check-sdk-deletions.py`;
7. `bash scripts/check-sdk-types.sh`;
8. headless, package, AudioWorklet, declaration-mirror, and packed-tarball smoke gates against one freshly built seven-file artifact closure whose Wasm SHA256 is the accepted pin; and
9. `git diff --check` plus an exact-path diff audit showing no change outside the allowed paths.

Retain the candidate archive evidence and compare its packaged host, ABI layout, declarations, and Wasm with the qualified closure. Exercise both the default omitted `spectrumHopFrames` path and an explicit supported hop so the published package proves legacy behavior and the new control. A fresh independent Astra MEDIUM review must return PASS before delivery. The required pull-request qualification must then pass on the bounded release-preparation commit.

## Immutable publication sequence

After the release-preparation PR merges, hold that exact merged `main` SHA unchanged through publication:

1. confirm `@misofm/engine@0.4.1` is still absent;
2. dispatch `npm-publish.yml` once in `qualify` mode with the exact merged SHA and an empty `qualification_run_id`;
3. after successful qualification, dispatch `publish` once with the same SHA and the successful qualification run ID;
4. never retry `publish` after an ambiguous response; after registry propagation use only `verify` with the same SHA and qualification run ID; and
5. independently verify public version/latest convergence, exact tarball SHA256/SHA512/integrity, public imports and types, `enginectl --version`, packaged default and explicit spectrum-hop behavior, one trusted SLSA v1 subject/PURL/source/workflow binding, and the actual Wasm/host hashes.

Record the release commit, PR, exact-main qualification run, npm qualification/publish/verify run IDs, archive identities, and independent registry evidence in this issue/spec. Close only after verified registry delivery; only then may the adapter exact-dependency release advance.

## Starting evidence

- Accepted runtime base: `9fe7157e3485f016cd62963cb84189fddac9c2b3`.
- Required qualification: run 35140015285, PASS.
- Accepted Worklet Wasm SHA256: `e18acf9ca97af137a1917e52481c4bf962943d6d755369387969f84c3e381106`.
- Registry latest before work: 0.4.0.
- Registry 0.4.1 before work: parsed E404 / unused.
- No release-preparation implementation or publication has occurred under this issue.

## Implementation attempt 1 evidence

On 2026-09-16, the bounded release-preparation tranche was applied in worktree
`/tmp/miso-engine-0-4-1` on branch `codex/release-engine-0-4-1`, from brief commit
`dc42960ac2cd0951e82c59a79d204f6f006dc7d6` whose parent is the accepted runtime base
`9fe7157e3485f016cd62963cb84189fddac9c2b3`. Only the five allowed metadata paths changed in
the implementation tranche. The package and lockfile identities are both `0.4.1`; the workflow
contains exactly five `0.4.1` release literals and exactly one accepted Worklet pin
`e18acf9ca97af137a1917e52481c4bf962943d6d755369387969f84c3e381106`; no `0.4.0` remains in the
workflow. The release fixture uses `0.4.1` for package, CLI, smoke, PURL, mutation, and
diagnostic identities while retaining the historical `0.2.4` normalization and fixed workflow
and qualify-step hashes. The README install and release-record links now identify 0.4.1/#855.

The focused gates all pass:

- `npm ci --ignore-scripts` in `sdk/`: PASS (5 packages added, 6 audited, 0 vulnerabilities).
- `python3 -B scripts/test-npm-publish-modes.py`: PASS (`npm publish mode reachability and
  trust gates: ok`).
- `node scripts/test-parse-npm-trust-list.mjs`: PASS (`npm trust list parser fixtures and
  mutations: ok`).
- `bash scripts/test-sdk-artifact-builder-output-contract.sh`: PASS.
- `bash scripts/check-sdk-generated.sh`: PASS for assets, generated modules, and generated
  surface.
- `python3 -B scripts/check-sdk-deletions.py`: PASS (74 files carry none of the retired
  spellings).
- `bash scripts/check-sdk-types.sh`: PASS, including the shipped-host declaration mirror pin.
- `bash scripts/build-web-audioworklet.sh <fresh-empty-directory>`: PASS; the exact seven-file
  closure was built and its Wasm SHA-256 is
  `e18acf9ca97af137a1917e52481c4bf962943d6d755369387969f84c3e381106`.
- `bash scripts/check-sdk-headless.sh <that-closure>`: PASS (278 tests, 31 suites).
- `bash scripts/sdk-package.sh check <that-closure>`: PASS (artifact-builder contract, 11/11
  `enginectl` checks, and the 98-file publishable tarball gate).
- `bash scripts/check-web-audioworklet.sh <that-closure>`: PASS for static/object, callgraph,
  metadata, ABI, vocabulary, session-map, and boot-budget checks.
- `node scripts/test-web-audioworklet.mjs --real-wasm-receiver --artifacts <that-closure>`:
  PASS for exact seven-file membership/digest, ordinary lifecycle, corrupted-Wasm refusal, and
  disposal mutation controls.
- `bash scripts/test-web-audioworklet.sh`: PASS for the full hermetic Worklet suite, safe-integer
  boundary, opcode policy, qualification mutations, browser response, policy, metadata,
  vocabulary, and session-map controls.
- A retained fresh archive smoke passed against a fresh extraction, and all seven packaged
  closure files are byte-identical to the fresh closure. The archive is
  `/tmp/issue855-sdk-candidate.N0hX1r/misofm-engine-0.4.1.tgz`, 1,331,109 bytes and 98 files,
  with SHA-256
  `251ba94b46cfc648ff867a1191e3d28e1cf18b910fc651c0c87de3d18111a7bc`, SHA-512
  `85345d6f548ec919e06e7b5edb54306923223d9d58f741605e47a14831b61e4dfb841650daab602192f41f6b3536da6067ef2977802c9a174b9de0638fa2203b`, npm shasum
  `dedf9cf506205b628e1966b0fa08b8cd7c387523`, and npm integrity
  `sha512-hTRdb1SOyRngbnte21QwaSMiPZ1Y90FgXkehSDG2Hk37hBZQ2qtgIZL0H2s1NtpgZ+8pd4AsmhdLneBjj6IgOw==`.
- `git diff --check`: PASS. The exact implementation diff path set is exactly
  `.github/workflows/npm-publish.yml`, `scripts/test-npm-publish-modes.py`, `sdk/README.md`,
  `sdk/package-lock.json`, and `sdk/package.json`; no runtime, generated, ABI, or historical
  normalization fixture was changed.

The candidate is locally qualified for fresh review. No commit, push, pull request, workflow
dispatch, npm publication, or registry mutation occurred in this attempt.

## Fresh Astra MEDIUM adversarial review

PASS on frozen implementation commit `f6328d836c9ed9861b2def49c5ea3ddef857c822`. The reviewer confirmed the diff is confined to the issue spec and five authorized release-metadata/fixture files; runtime, DSP, ABI, defaults, and generated assets are unchanged. The workflow retains exactly five `0.4.1` literals, one accepted Wasm pin, exact-SHA qualification, immutable publication, prior-qualified archive reuse, OIDC, and provenance guards. Publish-mode/trust tests, fresh archive smoke, recorded archive SHA-256, and all seven closure byte comparisons pass. A direct packaged managed-spectrum probe observed omitted-hop end samples `2048, 4096` and explicit H256 end samples `2048, 2304`. Review approval covers release preparation; required PR qualification and exact-main registry delivery remain.
