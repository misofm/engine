# Publish the Engine-only `@misofm/engine` 0.1.0 release

## Objective

Publish public `@misofm/engine@0.1.0` with npm dist-tag `latest` from one
GitHub-hosted Linux qualification archive containing only the Engine
AudioWorklet closure. Verify registry bytes, a fresh install, and SLSA
provenance before creating `sdk-v0.1.0`, the matching GitHub release, and the
trusted-publisher configuration.

This is the bounded successor to exhausted issues #354 and #355. Issue #356
removed the unreleased in-repository FLAC delivery stack, so every earlier
two-Wasm tarball, digest, `enginectl --stems` assertion, and qualification
record is obsolete and must not authorize this release.

## Preconditions

- Issue #356 has a final Sol PASS, is merged to `main`, and is closed with its
  GitHub body synchronized.
- The release commit descends from merged PR #353 and excludes rejected #352
  commits `3ab49a3d`, `bf1a6672`, and `6a08315c`.
- npm still reports `@misofm/engine@0.1.0` absent and no `sdk-v0.1.0` tag or
  GitHub release exists.
- A short-lived granular npm token with `@misofm` package write and bypass-2FA
  is stored only as GitHub `NPM_TOKEN`. Its value never enters logs, files,
  artifacts, issues, or commands.

## Smallest closable slice

1. Preserve the existing fail-closed `qualify` / `publish` / `verify` state
   machine, but qualify one Engine artifact directory and one newly packed
   Engine-only archive.
2. Repair the only remaining #355 defect with a fixture-tested parser for npm
   11.19.0 trusted-publisher output: empty stdout means absent; one standalone
   JSON object must exactly name GitHub, `misofm/engine`, `npm-publish.yml`, and
   permission `createPackage`; malformed, multiple, or conflicting documents
   fail closed.
3. Dispatch `qualify` on the exact merged `main` SHA. Dispatch `publish` once
   against that successful qualification run; after any ambiguous publish
   response, use only `verify`.
4. Verify version, shasum, integrity, public access, `latest`, clean consumer
   imports, `enginectl --version`, and the cryptographically verified npm SLSA
   statement bound to the exact archive, repository, workflow, ref, and SHA.
5. Create/verify the tag and GitHub release, configure the exact trusted
   publisher, delete the GitHub secret, revoke the temporary token, synchronize
   #354/#355/#357 evidence to `origin/main` and GitHub, then close all three.

## Allowed paths

- `.github/ISSUE_SPECS/354-publish-engine-sdk-0-1-0.md` (supersession/final
  closure note only)
- `.github/ISSUE_SPECS/355-repair-sdk-publication-verification-and-close-release.md`
  (supersession/final closure note only)
- `.github/ISSUE_SPECS/357-publish-engine-only-sdk-0-1-0.md`
- `.github/workflows/npm-publish.yml`
- `scripts/parse-npm-trust-list.mjs` (add)
- `scripts/test-parse-npm-trust-list.mjs` (add)

No SDK, Rust, host, sidecar, generated, manifest, lockfile, Engine artifact, or
artifact pin may change.

## Objective gates

1. Workflow YAML/shell/policy checks and the trust-parser fixture/mutations
   pass. The parser distinguishes empty, exact object, malformed JSON, arrays,
   concatenated documents, missing/extra fields, wrong provider/repository/file,
   and wrong/extra permissions.
2. GitHub Ubuntu reproduces only the sealed Engine AudioWorklet SHA-256
   `6dcd9ced2daeb886843a764bcc6abc0b4f1b2c7a50af1ed91151a5ab366461e5`.
   No FLAC decoder is built, staged, named, or present in the archive manifest.
3. Qualification runs all generated/type/headless/CLI/package/archive mutation
   gates and uploads exactly one newly packed archive with SHA-1, SHA-256,
   SHA-512, and npm integrity evidence.
4. Publish consumes that exact successful qualification artifact, proves the
   version absent, and invokes `npm publish` at most once with public access,
   `latest`, ignored lifecycle scripts, and provenance. Verify never publishes.
5. Registry convergence, clean install/import/CLI, and npm audit-signature
   checks bind the published bytes and SLSA statement to the exact release SHA.
6. The tag, GitHub release, trusted publisher, retired temporary secret/token,
   evidence commit, synchronized GitHub issue bodies, and closed issue states
   are all reread and verified before reporting completion.
7. Fresh Sol-high adversarial review returns PASS before qualification is
   dispatched and again before final closure claims PASS.

## Failure and rollback policy

Never retry an ambiguous publish response. Never overwrite or force-move a tag
or release. If registry bytes differ, stop. If 0.1.0 is published but defective,
deprecate it with an actionable message and publish a reviewed 0.1.1; do not
reuse the version or routinely unpublish it.

## Brief evidence

Pending fresh Sol brief. The starting workflow contains the accepted archive,
registry, provenance, run-identity, and ambiguity-recovery corrections from
#354/#355 plus #356's Engine-only packaging change. The one known remaining
defect is #355's trust-list parser shape; no npm bytes or release references
have been created.
