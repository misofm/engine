# Remove the in-repository FLAC delivery stack before SDK publication

## Objective

Remove the unreleased FLAC decoder, publisher, catalog migration utility,
browser qualification, and `enginectl --stems` convenience path before
publishing `@misofm/engine@0.1.0`.

The engine package exposes generic session, canonical-PCM, render, browser-host,
and PCM-ingress primitives only. Delivery codecs, transport policy, and Miso
platform publishing/migration tools belong in external packages or `misofm/cli`.
This issue is a prerequisite of #354; all earlier tarballs and qualification
evidence are obsolete after this change.

## Decision and smallest closable slice

Deleting only `sidecars/flac-decoder` would leave broken workspace dependencies,
stale SDK asset types, an unusable `enginectl --stems`, orphaned browser jobs,
and a release workflow that still requires the decoder. Remove the complete live
FLAC product surface:

- delete `sidecars/flac-decoder`, `tools/stem-publisher`,
  `tools/catalog-migrate`, and the FLAC delivery fixture corpus;
- remove `enginectl --stems`, `--session-id`, and `--quantum-frames` while
  preserving byte-compatible `enginectl session build --request` behavior;
- remove decoder assets/types from the npm package and simplify its builder to
  accept only the Engine AudioWorklet artifact directory;
- remove FLAC-specific build, qualification, browser, CI, release, and current
  design/runbook documentation; and
- retain historical issue specs, derivations, rulings, and mutation records
  unchanged, with a current boundary ruling that identifies them as history.

Retain canonical PCM identity and `stem-hasher`, canonical sessions and explicit
sources, browser PCM rings/pump/OPFS/resolver seams, native WAV/RF64 support, and
all Engine Wasm/AudioWorklet/metadata/ABI/generic asset-manifest surfaces.
Keep `sidecars/` with a short README stating that delivery codecs do not ship
from this repository.

## Migration contract

`@misofm/engine@0.1.0` has not been published, so no compatibility alias or
deprecation period is required. This remains supported:

```text
enginectl session build --request request.json --output session.json
```

`--stems` becomes an unknown flag and exits 2 without creating output or loading
an engine asset. Existing canonical session JSON remains valid. Callers resolve,
download, decode, verify, and store transport bytes externally, then submit
decoded PCM through the existing generic ingress APIs. Removed code stays
recoverable from Git history for future extraction into an external package.

## Allowed paths

- `.github/ISSUE_SPECS/356-remove-in-repository-flac-delivery-stack.md`
- `.github/ISSUE_SPECS/354-publish-engine-sdk-0-1-0.md`
- `.github/workflows/browser-qualification.yml`
- `.github/workflows/ci.yml`
- `.github/workflows/npm-publish.yml`
- `AGENTS.md`
- `Cargo.toml`
- `Cargo.lock`
- `sidecars/flac-decoder/**` (delete)
- `sidecars/README.md` (add)
- `tools/stem-publisher/**` (delete)
- `tools/catalog-migrate/**` (delete)
- `fixtures/flac-delivery/**` (delete)
- `hosts/host-web/qualification/package.json`
- `hosts/host-web/qualification/flac-decoder-server.mjs` (delete)
- `hosts/host-web/qualification/flac-decoder-worker.js` (delete)
- `hosts/host-web/qualification/flac-decoder-throughput-worker.js` (delete)
- `hosts/host-web/qualification/run-flac-decoder.mjs` (delete)
- `hosts/host-web/qualification/run-flac-decoder-throughput.mjs` (delete)
- `hosts/host-web/tests/stem-store-hash-v1.mjs`
- `scripts/build-flac-decoder.sh` (delete)
- `scripts/check-flac-decoder.mjs` (delete)
- `scripts/check-flac-decoder.sh` (delete)
- `scripts/test-flac-decoder.sh` (delete)
- `scripts/sdk-package.sh`
- `scripts/test-sdk-artifact-builder-output-contract.sh`
- `scripts/build-web-audioworklet.sh`
- `scripts/test-lane-policy.sh`
- `scripts/test-workspace-policy.sh`
- `sdk/README.md`
- `sdk/codegen/stage-package.mjs`
- `sdk/src/assets.ts`
- `sdk/src/enginectl.ts`
- `sdk/src/cli/stems.ts` (delete)
- `sdk/test/enginectl-cli.mjs`
- `sdk/test/package-tarball-smoke.mjs`
- `docs/FLAC_DELIVERY_V1.md` (delete)
- `docs/FLAC_CATALOG_REHASH_RUNBOOK.md` (delete)
- `docs/DELIVERY_CODEC_BOUNDARY.md` (add)
- `docs/README.md`
- `docs/ENGINE_ENV_VOCABULARY.md`
- `docs/STEM_IDENTITY_V1.md`

No DSP, session-schema, PCM-ingress, OPFS, resolver, or app change is authorized.

## Historical evidence retained unchanged

Closed issue specs (including #268, #328, #333, #335, #338, and #345),
`docs/derivations/243-sdk-boot.md`,
`docs/derivations/281-qualification-harness-boot.md`,
`docs/rulings/prefix-strip-inventory.md`, and
`hosts/host-web/MUTATIONS.md` remain historical evidence. Their mentions do not
describe live product behavior; this issue and `docs/DELIVERY_CODEC_BOUNDARY.md`
are the current ruling.

## Objective gates

1. Removed code and fixture directories no longer exist.
2. Cargo metadata contains no `flac-decoder`, `stem-publisher`,
   `catalog-migrate`, `flacenc`, or `symphonia`; `Cargo.lock` is regenerated.
3. Workspace policy/mutations and proportional workspace tests pass;
   `stem-hasher` canonical PCM vectors/mutations remain green.
4. Request-mode `enginectl` preserves help, canonical JSON, receipt, failure,
   and output publication behavior; `--stems` exits 2 and publishes nothing.
5. SDK assets and the extracted archive contain no decoder, decoder digest,
   `cli/stems`, or delivery-codec artifact; all public entries still import,
   headless boot/render succeeds, and mutated Engine Wasm still fails.
6. The package builder accepts one Engine artifact directory and its contract
   test rejects a second codec artifact requirement.
7. Browser qualification retains all Engine/AudioWorklet/attestation/digest/
   stall/deployment gates with no decoder job; CI invokes no deleted script.
8. #354 builds and pins only the Engine AudioWorklet closure and publishes only
   a newly qualified exact archive.
9. Fresh Sol-high adversarial review returns PASS before #354 resumes.

## Required mutations

- Reintroducing a decoder asset or manifest entry makes package smoke fail.
- Re-enabling `--stems` makes CLI compatibility fail.
- Requiring a second codec artifact directory makes the builder-contract test fail.
- Reintroducing a removed Cargo member/dependency makes inventory fail.
- Mutating retained Engine Wasm continues failing before compilation.

## Risks and rollback

Consumers of unreleased FLAC asset keys or `enginectl --stems` must migrate; this
is intentional before npm publication. The app's current vendored tarball still
contains the decoder and must be replaced by the clean candidate. Historical
tarball hashes/sizes/dry-runs are invalidated. Restore functionality only through
a new reviewed external codec or platform-CLI issue, not by reverting this
engine ownership boundary.

## Brief evidence

Sol xhigh: APPROVE. Terra inventory independently confirmed the current npm
tarball embeds the decoder and that deletion must cover its workspace, SDK,
enginectl, browser, CI, fixture, documentation, and release dependencies.
