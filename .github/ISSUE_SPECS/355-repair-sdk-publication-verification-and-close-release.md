# Repair SDK publication verification and close the 0.1.0 release

## Objective

Repair the fail-closed but non-completing publication workflow preserved by
issue #354, qualify and publish the unchanged `@misofm/engine@0.1.0` tarball,
then synchronize its tag, GitHub release, evidence, and issue state.

## Preconditions

- PR #353 must merge with all required checks passing before this issue lands.
- The release commit must descend from accepted issue #351 and exclude rejected
  issue #352 commits `3ab49a3d`, `bf1a6672`, and `6a08315c`.
- Issue #354 attempt 3 is the starting checkpoint. It published no npm bytes and
  created no tag or GitHub release.
- Package name, version, API, generated assets, artifact pins, and tarball
  contents remain unchanged.

## Smallest closable slice

Correct only the deterministic blockers from #354's final Sol verdict:

1. Export the exact `expected_sha` input to the provenance-verification step so
   the verified SLSA resolved dependency can bind the release commit.
2. Make root closure validate the npm 11.12.1 DSSE payload rather than relying
   on a nonexistent top-level `verified[].integrity` field.
3. Before consuming an always-uploaded run artifact, prove the selected run and
   expected publish/verify job completed successfully for the release SHA.
4. Require closure from a clean local `main`, push the exact evidence commit,
   prove `origin/main` contains that commit, and verify the remote tag, release
   target/title, and closed issue.

Then run the existing qualification and one-shot publication state machine.
Do not weaken its digest, package, exact-archive, registry, provenance, ancestry,
or ambiguity-recovery gates.

## Allowed paths

- `.github/ISSUE_SPECS/355-repair-sdk-publication-verification-and-close-release.md`
- `.github/workflows/npm-publish.yml`

The post-publication evidence checkpoint may append PASS evidence to this spec.
No SDK, Rust, host, sidecar, generated, manifest, lockfile, or artifact-pin path
may change.

## Objective gates

1. Workflow YAML, extracted shell syntax, action pinning/policy, formatting,
   workspace policy, and `git diff --check` pass.
2. A GitHub-hosted Ubuntu `qualify` run reproduces both pinned Wasm digests,
   passes all existing SDK/package gates, and uploads exactly one smoked tarball
   with SHA-1, SHA-256, and SHA-512 evidence.
3. `publish` consumes that exact qualification artifact, first proves version
   absence, and invokes `npm publish` at most once with public/latest/provenance.
4. `publish` or recovery-only `verify` proves registry byte identity, public
   access, latest tag, fresh imports, enginectl, and a cryptographically verified
   npm SLSA provenance statement bound to `misofm/engine`, this workflow on
   `refs/heads/main`, and the expected release commit.
5. `sdk-v0.1.0` and GitHub release `@misofm/engine 0.1.0` resolve to that commit;
   PASS evidence reaches `origin/main` before issues #354 and #355 are closed and
   their remote closed state is verified.

## Authentication and failure policy

The first publish uses a short-lived granular npm token with package write and
bypass-2FA stored only as GitHub secret `NPM_TOKEN`; OIDC supplies provenance.
Never print or artifact the token. After publication, configure trusted
publishing for `.github/workflows/npm-publish.yml`, remove the secret, and revoke
the temporary token.

If the publish response is ambiguous, never publish again: use only `verify`.
If registry bytes differ, stop. If a published package is defective, deprecate
0.1.0 and ship a reviewed 0.1.1; do not routinely unpublish or reuse a version.

## Brief and evidence record

Pending Sol brief.
