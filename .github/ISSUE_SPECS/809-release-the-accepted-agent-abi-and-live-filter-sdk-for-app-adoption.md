# Release the accepted agent ABI and live filter SDK for app adoption

Delivery child of #804. Astra XHIGH scoped this release in
`/tmp/804-app-release-spec.md`; root approves the bounded engine portion.
Dependencies: accepted and integrated #147, #805, #807 and #808, including
disabled builtin filter elision during trim ramps. No release can substitute
prepared-only filters for the required live browser/headless capability.

## Product outcome and compatibility

Publish one new immutable `@misofm/engine` registry version containing structured
parameter edits, authoritative units, dedicated live EQ cuts and live builtin
filter pairs. Browser and headless use the same accepted Rust coefficient
authority and matching generated metadata/Wasm. The app will adopt this exact
version through the engine-web-adapter; this issue owns the engine release.

Freeze an unused minor version (0.3.0 if available at freeze), verifying registry
availability then. Final EQ state is 464 bytes per channel (116 words), plus
the existing common header; old 304-byte and intermediate 456-byte channel
payloads are rejected rather than reinterpreted. Session defaults and original
four-band IDs remain compatible. Never overwrite 0.2.6 or claim patch-level
binary-state compatibility.

## Bounded implementation and independent verification

One Luna XHIGH tranche updates the exact SDK version/lock, current Wasm pin,
release workflow version/digest/registry identity guards, narrowly affected
release-mode fixtures and compatibility documentation. Reuse existing build,
qualification, publishing and verification machinery; no release framework
redesign. Root commits the focused-green tranche before a fresh Astra MEDIUM
adversarial review of its concrete source, package and provenance evidence.
Maximum five coherent attempts, each with one verdict.

Use the existing `.github/workflows/npm-publish.yml` qualify -> publish -> verify
mechanism from #794. Preserve OIDC-only/token refusal, exact SHA/ref/ancestry,
archive, source/workflow provenance and recovery gates. Only update version and
artifact identity where the accepted new release requires it; never weaken the
gates or rewrite historical release evidence. Required main qualification must
pass for the exact merged source before release qualification is dispatched.

## Release execution and objective completion

1. Freeze one accepted merged-main SHA after the feature and package gates.
   Build the new DSP/Wasm and complete SDK closure with the existing pinned
   toolchain. Regenerate the browser matrix from actual qualification, not by
   manually substituting its artifact identity. Qualify one immutable archive.
2. Publish that exact qualified archive through the existing workflow without
   rebuilding/repacking. If publication is ambiguous, run verify-only recovery
   against the same qualification; do not republish or reserve another version
   merely to evade uncertainty.
3. Verify the actual registry archive/integrity, public imports/types/CLI and
   trusted SLSA source/workflow binding in a fresh registry consumer. Record
   source SHA, version, Wasm SHA256, archive SHA256/SHA512 and workflow run IDs.
   Prior 0.2.6 evidence cannot qualify changed DSP bytes.
4. Hand the verified exact package identity to the separately tracked adapter
   adoption/release, then app adoption and testnet deployment. No source links,
   committed dependency overrides or copied Wasm count as final adoption.
5. Synchronize this local spec and GitHub evidence, close only after publication
   and independent verification, and preserve evidence before removing completed
   clean worktrees. Parent #804 remains open until the requested app outcome.

Proportional gates include the existing release-mode guard tests, generated SDK
and types, actual packed package/fresh consumer, real browser/headless filter
proofs, required CI and immutable qualify/publish/verify workflows. Reuse the
accepted feature evidence rather than inventing another benchmark or fixture
campaign. Root owns dispatch, GitHub synchronization and release coordination.

## Decision/evidence record

Astra XHIGH scope approved; implementation/version freeze and fresh Astra MEDIUM
verdict pending. No registry version is reserved or published by this brief.

### Release preparation brief, 2026-09-14

Root freezes unused SDK minor **0.3.0** after a live registry check (latest0.2.6;
0.3.0 absent). Accepted #808 runtime/source and attempt3 test correction are at
ea8efbc0; PR#812 required CI/merge remain. To avoid idle release preparation,
root permits the bounded version/guard/documentation tranche in isolated
`codex/809-live-filter-sdk` from that accepted checkpoint while #808 CI runs.
Merge the accepted #808 main lineage into this branch before its delivery.
This sequencing does not relax the release prerequisites: no qualification
release dispatch or publication until #808 is merged and the exact final main
source has passed required CI. Frozen Wasm is
`7e925d939234b67d14be41a647b4cc6de23099501a763d27e8a56c77524999e7`.
The numbered-issue audit found all414 local specs represented among518 remote
issues; #809 number/title match and #804 remains open. Root owns all publication.

Luna XHIGH attempt1 checkpoint: SDK/package-lock0.3.0, the existing workflow's
five release identity guards and Wasm pin, matching mode/mock/negative fixtures,
and concise binary-state compatibility documentation are updated. No source DSP,
ABI, helper or published artifact has changed, and no new release machinery was
introduced. PASS: existing npm publish-mode/trust tests, generated SDK check,
SDK types and publishable-package check using the accepted #808 artifact; diff
check clean. Fresh Astra MEDIUM source/package review and main integration remain.
No package is published yet.

### Attempt1 source/package verdict

Fresh Astra MEDIUM **PASS** for ea8efbc0..275484b6: existing release mode,
trust and trust-list parser checks pass; the packed fresh consumer passes public
imports, declarations, embedded runtime and integrity checks. All seven artifact
manifest hashes match the accepted runtime. Candidate source is
`275484b65ca37a7a7ec50ef2015e38bd437062ea`; local preparation archive
`/tmp/809-candidate-pack/misofm-engine-0.3.0.tgz` SHA256 is
`0da5f34d6d5e021f543acf21053b6176931548f3bbc584301c42c2f278f5b804`.
This verdict establishes source/package readiness only. Final main integration,
required CI and immutable qualification/publication/registry verification remain;
no registry release is claimed. App #222 may use these exact accepted candidate
bytes for its already authorized temporary preparation, with tracked registry
pins unchanged until the actual release.

### Main integration

#808 PR#812 passed required qualification34878694424 and merged as
`7c4238000da90358db1bec7ffe9a2459962590b4`. This release branch merged that
main lineage without conflict. The only release delta remains the reviewed
version/guard/documentation changes; no runtime bytes changed. Required release
PR/main CI and immutable workflow dispatches follow.

### Published and independently verified delivery

PR#813 required qualification34879497461 PASS; merged source
`51e03cfdde61802fc8456872a6637a265fae5979` passed exact-main34880226665.
Immutable qualification34880929687 produced SDK **0.3.0**,98 files, archive SHA256
`0da5f34d6d5e021f543acf21053b6176931548f3bbc584301c42c2f278f5b804`, SHA512
`e7a5f0bbb2bf93b81148618ba4dcfb568d564b069a3f6176b68e8085bfe2319336d464c5b20936e195d54d5a5d56c1b543c7e67bf55b10f363b66e74b694452d`, integrity
`sha512-56Xwu7K/k7gRSGGLpNz7Vo1WSwaaP2F2to6Ahb/iMZM21GTFsgk24ZXVTVpdVsG1Q8fme/VbEPNjtm50tpRFLQ==`.

Publication34881586727 submitted that exact archive once through OIDC. npm
accepted it and signed provenance, but public propagation exceeded the workflow
window. After availability, **verify-only34881963287 PASS** against the same
qualification: fresh registry public imports/CLI and cryptographically verified
SLSA package/archive/source/workflow binding. No republish, rebuild or version
change was used. Root independently downloaded the actual registry tarball and
matched its SHA256/integrity, and inspected the verified DSSE subject, main
workflow identity and exact51e03cfd source. Wasm remains
`7e925d939234b67d14be41a647b4cc6de23099501a763d27e8a56c77524999e7`.

Evidence archives are the qualify/publish/verify workflow artifacts; local
copies are /tmp/809-qualified-51e03cfd, /tmp/809-publish-51e03cfd and
/tmp/809-verified-51e03cfd, plus /tmp/809-registry-archive. The registry archive
also exactly matches the accepted temporary app candidate. #809 completion
requires this evidence checkpoint upstream and GitHub closure; downstream
adapter#111 and app#222 remain tracked under open parent#804.
