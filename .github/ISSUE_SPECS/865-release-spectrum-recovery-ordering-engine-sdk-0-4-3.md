# Release spectrum recovery ordering in Engine SDK 0.4.3

## Product outcome

Publish one immutable `@misofm/engine` 0.4.3 release containing the managed-spectrum recovery publication correction delivered by #863 and merged in PR #864.

The accepted runtime source is clean synchronized `main` commit `c78e1fbed267b8e1382a59beaf266eac015335bc`. Exact-main qualification run 35178072805 passed the exact shipped AudioWorklet artifact, the SDK package/generated-surface route, and the final qualification verdict. Registry 0.4.2 is current; a live lookup for 0.4.3 returns parsed E404.

Issue #863 changed only its issue spec, `sdk/src/core/observation-subscriptions.ts`, and `sdk/test/spectrum-evals.mjs`. It did not change Rust, DSP, ABI, hosts, generated assets, Wasm, analysis parameters, session schema, or release machinery. The accepted AudioWorklet SHA-256 remains `e18acf9ca97af137a1917e52481c4bf962943d6d755369387969f84c3e381106`.

## Smallest closable release slice

Freeze SDK 0.4.3 without a new runtime change. The release-preparation tranche may edit only:

- this issue spec;
- `sdk/package.json`;
- `sdk/package-lock.json`;
- `sdk/README.md`, limited to install and release-record identity;
- `.github/workflows/npm-publish.yml`; and
- matching release-identity fixtures and diagnostics in `scripts/test-npm-publish-modes.py`.

Change package and root lockfile identities from 0.4.2 to 0.4.3 without dependency or integrity changes. Update the README install and release-record link to 0.4.3 and this issue. Update only the five workflow release/job/archive/provenance version literals. Keep the accepted Worklet pin unchanged. Update only current release identities and diagnostics in the publish fixture; preserve the historical normalized baseline, fixed workflow/qualify-step hashes, and accepted-pin literals.

The six authorized paths are this spec, `sdk/package.json`, `sdk/package-lock.json`, `sdk/README.md`, `.github/workflows/npm-publish.yml`, and `scripts/test-npm-publish-modes.py`. The exact metadata delta is one package version, two root lockfile versions, two README identities, and five workflow literals. The fixture changes eight current 0.4.2 release literals to 0.4.3, four #860 diagnostics to #865, and the invalid-version mutation predecessor from 0.4.1 to 0.4.2. Preserve the 0.2.4 normalization baseline, both fixed hashes, both accepted-pin literals, and every dependency/integrity entry byte-for-byte. Commit `c78e1fbed267b8e1382a59beaf266eac015335bc` must remain an ancestor of the candidate.

Do not edit runtime source/tests, Rust, DSP, ABI, hosts, generated files, policy defaults, session schema, generic publisher behavior, benchmarks, or V1 contract identities. Preserve exact-main SHA enforcement, qualify→publish→verify archive ownership, OIDC-only publication, immutable-version refusal, one real publication attempt, registry-convergence handling, and trusted SLSA provenance verification.

## Objective gates

Before review, prove package install; release-mode/trust/artifact/generated/deletion/type gates; one fresh seven-file AudioWorklet closure with the unchanged accepted Wasm pin; headless/package and AudioWorklet gates; retained candidate archive extraction and artifact identity; archive identities; exact-path/diff audit; and independent review before PR/exact-main qualification.

The H256 gate must import `dist/core/observation-subscriptions.js` from the freshly extracted candidate archive and reproduce #863's cold-start scheduler shape with `hopFrames = 256`: 60 consecutive `gap -> ready` cycles over 1.2 seconds, exactly 120 transport reads with no capture exceeding two, and exactly 60 callbacks that are all ready/available. Every callback's captured sample, end sample, and snapshot identity must match `readLatest()`. Every recovered callback must retain one native miss and one skipped/coalesced publication, and the final sequence/token must equal the final ready result. A source TypeScript test does not satisfy this release gate.

## Immutable publication sequence

After the reviewed preparation PR merges, freeze that exact `main` SHA through registry verification. Confirm 0.4.3 is still absent; dispatch `npm-publish.yml` once in qualify mode; retain its archive; dispatch publish once with the same SHA and qualification run ID; never retry an ambiguous publish; use verify-only after registry propagation if required. Independently verify registry version/latest, byte-identical archive, public imports/declarations, `enginectl --version`, repeated H256 recovery behavior, unchanged Wasm/host identities, signatures, and one trusted SLSA v1 subject/PURL/source/workflow binding.

Record all commits, PR and exact-main qualification runs, release workflow IDs, archive/provenance identities, and independent public evidence here. Close only after verified registry delivery and synchronized evidence. Adapter publication and app adoption are separate downstream issues.

## Starting evidence

- Accepted source: `c78e1fbed267b8e1382a59beaf266eac015335bc` from #863 / PR #864.
- Exact-main qualification run 35178072805: PASS.
- Accepted unchanged Worklet SHA-256: `e18acf9ca97af137a1917e52481c4bf962943d6d755369387969f84c3e381106`.
- Registry latest before work: 0.4.2.
- Registry 0.4.3 before work: parsed E404 / unused.
- No 0.4.3 preparation, workflow dispatch, publication, or registry mutation has occurred.

## Attempt 1 preparation evidence

Luna completed the metadata-only preparation pass in the release worktree based at
`ccc6a64d5b66968089b4aa12a849948d69edba23`; `c78e1fbed267b8e1382a59beaf266eac015335bc`
remains an ancestor. `npm ci --ignore-scripts` in `sdk` passed (5 packages added, 6 audited,
0 vulnerabilities). The hermetic publish-mode fixture passed, as did the trust parser,
artifact-builder output contract, generated-surface, deletion, and type gates. The exact
authorized-path/literal audit passed for the specified package, lockfile, README, workflow, and
fixture deltas; `git diff --check` passed. No closure build, archive pack, workflow dispatch,
commit, push, PR, registry mutation, or publication was attempted in this tranche.

## One-round candidate qualification evidence

Sol qualified clean source checkpoint `172964aed7db43fc080191d41e94a7a5e02d8294`
with accepted `c78e1fbed267b8e1382a59beaf266eac015335bc` ancestry. Against the fresh
seven-file closure at `/tmp/issue865-worklet`, whose Wasm SHA-256 is the unchanged
`e18acf9ca97af137a1917e52481c4bf962943d6d755369387969f84c3e381106`, the scripted SDK
headless gate passed 284/284 tests in 31 suites, the package gate passed, and the Worklet
static/object, explicit real-Wasm receiver, and full hermetic gates passed. The real-Wasm gate
also passed its corrupt-Wasm and acknowledged-disposal mutation controls.

npm 11.19.0 packed exactly one retained 98-file candidate at
`/tmp/issue865-qualification/misofm-engine-0.4.3.tgz`. It is 1,331,353 bytes packed and
5,460,599 bytes unpacked, with npm shasum
`8a48c97c8f12f37b17436216552f4b259c2bbf65`, SHA-256
`0b81dc9cec57d89703e42da5da592f5cb450ffec71a980ea0c4a00517cd294a0`, SHA-512
`73b4226365f67288123599def4cf42d49f5bd279e0fc97cce901a1ce1bc7b5217492e9dc1b27189ff138a6d7fb2b3a94d104ef8c9c6d91b4163ab4f4f4c08a16`,
and integrity
`sha512-c7QiY2X2cogSNZne9M9C1J9b0nng/JfM6QGhzhvHtSF0kuncGycYn/E4ptf7KzqU0QTvjJxtkbQWOrT09MCKFg==`.
A fresh extraction passed the independent archive smoke, strict declaration consumer, all four
package-name imports, and `enginectl --version` = `0.4.3`; all seven packaged Engine artifacts
are byte-identical to the qualified closure.

The release-only H256 proof imported the fresh extraction's compiled
`dist/core/observation-subscriptions.js`. It passed 60 consecutive `gap -> ready` cycles over
1,200 ms with exactly 120 reads and exactly two reads per capture, 60/60 callbacks all
`ready`/available, and callback captured sample, end sample, and snapshot token equal to
`readLatest()` on every cycle. Every callback retained exactly one native miss and one
skipped/coalesced publication. The final ready identity was sequence `119`, snapshot token
`1119`, captured sample `30464`, and end sample `30720`. Qualification records and the sole
archive are retained under `/tmp/issue865-qualification`. No source/runtime/test change, push,
PR, workflow dispatch, registry mutation, or publication occurred. Sol's one-round local
candidate-qualification verdict is **PASS**.

## Fresh Astra medium review

PASS at clean checkpoint `85bef8f7`. The reviewer independently confirmed the exact six-path
metadata-only diff and literal counts, accepted-source ancestry, unchanged dependencies,
integrities, pins, historical hashes, and publisher semantics. The retained archive hashes and
all 98 extracted members matched; all seven closure artifacts were byte-identical. Independent
publisher, trust, type, generated/deletion, artifact-builder, spectrum, fresh-extraction smoke,
and strict-declaration checks passed. The reviewer reran the extracted compiled H256 proof and
observed 60/60 ready/available callbacks, exactly 120 reads and two per capture, matching
publication identities, and one native miss plus one skipped publication per callback. No blocker
remains before PR qualification and immutable registry delivery.
