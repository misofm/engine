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
