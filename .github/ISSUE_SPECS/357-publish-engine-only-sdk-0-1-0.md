# Publish the Engine-only `@misofm/engine` 0.1.0 release

## Objective

Bootstrap the previously unused `@misofm/engine` namespace with one inert
`0.0.0` tombstone under the non-default `bootstrap` tag, configure npm trusted
publishing for GitHub Actions, then publish public `@misofm/engine@0.1.0` with
the `latest` dist-tag through OIDC only. The real release must come from one
GitHub-hosted Linux qualification archive containing only the Engine
AudioWorklet closure. Verify registry bytes, a fresh install, automatic SLSA
provenance, the immutable `sdk-v0.1.0` tag, and the matching GitHub release.

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
- `@misofm/engine@0.0.0` is also absent, the local interactive npm account is
  `misolabs` with `@misofm` publication authority and account-level 2FA, and no
  GitHub `NPM_TOKEN` secret exists.
- npm requires a package to exist before trusted publishing can be configured.
  The only non-OIDC registry mutation is therefore an explicitly confirmed,
  interactive local publish of the inert `0.0.0` bootstrap with 2FA. No npm
  credential is ever stored in GitHub.

## Smallest closable slice

1. Preserve the existing fail-closed `qualify` / `publish` / `verify` state
   machine, but make publication provably OIDC-only and qualify one Engine
   artifact directory and one newly packed Engine-only archive.
2. Repair the remaining #355 defect with a fixture-tested parser for npm
   11.19.0 trusted-publisher output: empty stdout means absent; one standalone
   JSON object must exactly name GitHub, `misofm/engine`, `npm-publish.yml`, and
   permissions `createPackage` and `createStagedPackage`, which npm returns for
   a direct-publish grant; malformed, multiple, or conflicting documents fail
   closed.
3. Dispatch `qualify` on the exact merged and required-check-green `main` SHA,
   then freeze `main` through publication. `qualification_run_id` is forbidden
   for `qualify` and required for both `publish` and `verify`; it must identify
   the exact successful run, job, SHA, workflow path, and single unexpired
   qualification artifact.
4. Build one temporary bootstrap tarball outside the repository containing only
   `package.json`, a tombstone README, and the repository LICENSE. It has no
   entry point, exports, bin, dependencies, lifecycle scripts, Wasm, SDK code,
   or Engine artifact. Review its exact inventory and digests, obtain explicit
   user confirmation, then publish `0.0.0` interactively with 2FA, public access,
   and the non-default `bootstrap` tag. Verify its bytes, ownership, access,
   `bootstrap == 0.0.0`, and absence of `latest` before continuing.
5. Configure and reread the exact trusted publisher for GitHub repository
   `misofm/engine`, workflow `npm-publish.yml`, with direct publish permission.
   The fixture-tested parser must return `present` during the interactive
   operator check and GitHub `NPM_TOKEN` must remain absent. OIDC authorizes
   `npm publish`, not `npm trust list`, so the workflow must not attempt an
   authenticated trust-list read; npm's OIDC exchange is the runtime identity
   enforcement.
6. Dispatch `publish` once against the successful qualification run. The job
   must reject npm token fallbacks, use GitHub-hosted OIDC, and publish the exact
   qualified archive. After any ambiguous response, use only `verify`.
7. Verify version, shasum, integrity, public access, `latest`, clean consumer
   imports, `enginectl --version`, and the cryptographically verified npm SLSA
   statement bound to the exact archive, repository, workflow, ref, and SHA.
8. Deprecate `0.0.0` as bootstrap-only and remove the `bootstrap` dist-tag.
   Create/verify the tag and GitHub release, then perform the two-phase evidence
   and issue closure below.

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
4. `qualification_run_id` is empty for `qualify` and digits-only for `publish`
   or `verify`. The selected run is exactly one successful manual invocation of
   `.github/workflows/npm-publish.yml` at the release SHA, with exactly one
   successful `qualify @misofm/engine 0.1.0` job and one unexpired
   `engine-sdk-qualify-$SHA` artifact.
5. The reviewed bootstrap archive has only the three allowed files and is
   published at most once under `bootstrap`; it never receives `latest`, never
   contains Engine code, and is deprecated with its dist-tag removed after the
   real release succeeds.
6. The workflow has `id-token: write`, runs on a GitHub-hosted runner with
   supported Node/npm versions, contains no `NODE_AUTH_TOKEN`/`NPM_TOKEN`
   publishing path, and fails if a token fallback is populated. GitHub has no
   npm credential secret.
7. The trust parser accepts zero-byte absence or one exact object with only
   `id`, `type`, `file`, `repository`, and `permissions`; the ID is a nonempty
   string and the remaining values are exactly `github`, `npm-publish.yml`,
   `misofm/engine`, and `[\"createPackage\", \"createStagedPackage\"]`.
   Fixtures reject whitespace-only absence, arrays/scalars, malformed or
   concatenated JSON, duplicate keys, missing/extra fields, and wrong or extra
   values.
8. OIDC `publish` consumes the exact successful qualification artifact, proves
   `0.1.0` absent, and invokes `npm publish` at most once with public access,
   `latest`, ignored lifecycle scripts, and provenance. Verify never publishes.
9. Registry convergence, clean install/import/CLI, and npm audit-signature
   checks bind the published bytes and SLSA statement to the exact release SHA.
10. Phase A rereads run/artifact/registry/provenance, deprecates the bootstrap,
    removes its tag, creates/verifies immutable tag and release, verifies trust
    and secret absence, appends candidate evidence to #354/#355/#357, pushes it
    upstream, synchronizes GitHub bodies, and leaves all three issues open.
11. A fresh Sol-high adversarial review returns PASS before qualification and
    again between Phase A and Phase B. Phase B rereads every external identity,
    evidence ancestry, and synchronized body before closing and rereading
    #354/#355/#357 as closed.

## Failure and rollback policy

Never retry an ambiguous bootstrap or real publish response: compare registry
integrity with the exact packed archive, and stop on any mismatch. Never
overwrite or force-move a tag or release. Never alter or revoke the user's local
npm login. If 0.1.0 is published but defective, deprecate it with an actionable
message and publish a reviewed 0.1.1; do not reuse the version or routinely
unpublish it.

## Brief evidence

Fresh Sol-high preflight on 2026-09-04 returned REJECT for the stale token-based
issue, and conditionally approved this tokenless architecture after the present
amendments are committed, pushed, and synchronized. The review requires an
inert `0.0.0` bootstrap rather than a duplicate unprovenanced SDK, exact
qualification-run/artifact identity, a fixture-tested fail-closed trust parser,
OIDC-only `0.1.0` publication, and two-phase closure. PR #361 makes
`qualification` the sole required context; the eventual release SHA must pass
that aggregate. Its environment-pin guard and action SHA updates must be
preserved. Historical token instructions in #354/#355 must not be executed.
