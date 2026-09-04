# Publish `@misofm/engine` 0.1.0 from one provenance-attested tarball

## Objective

Publish the existing reviewed `sdk/` package as public
`@misofm/engine@0.1.0` with npm dist-tag `latest`, using exactly one
Linux-built, fully qualified tarball with GitHub/npm provenance. Verify the
registry bytes and a fresh install before creating tag `sdk-v0.1.0` and the
matching GitHub release.

## Preconditions and baseline

- Merge PR #353 first so accepted/closed issue #351 is present in the release
  ancestry. Its PCM pump file is not in the npm tarball, but the repository tag
  must not strand an accepted launch fix.
- Rejected issue #352 commits `3ab49a3d`, `bf1a6672`, and `6a08315c` must not be
  ancestors of the release.
- `sdk/package.json` and `sdk/package-lock.json` already declare version
  `0.1.0`; no SDK source or manifest change is needed.
- The app's pinned 71-file tarball from reviewed commit `5c3f8abe` passes the
  extracted package smoke test and `npm publish --dry-run` with SHA-1
  `50bb5d3f5e9f5b2d6a2fc535408b4af094be88c5`.
- A clean Darwin rebuild is not the release artifact: its FLAC Wasm digest
  differs from the Linux-qualified pin. Build and publish on one GitHub-hosted
  Ubuntu runner.

## Smallest closable slice

Add a manually dispatched, SHA-pinned npm publication workflow with separate
`qualify` and `publish` modes. Both modes must build the AudioWorklet and FLAC
sidecar exactly once into empty directories, prove their pinned digests, run
the generated/deletion/type/headless/package gates, pack exactly one final
archive, smoke that exact archive, record checksums/integrity, and upload it as
workflow evidence.

`publish` additionally refuses an occupied version, publishes that same `.tgz`
with public access, `latest`, ignored lifecycle scripts, and provenance, then
queries npm until version, shasum, integrity, access, and dist-tag match. A
fresh registry consumer must import all four public entries and run
`enginectl --version`.

The workflow accepts the exact expected commit SHA and refuses unless the run
is dispatched on `main`, `HEAD` equals that SHA, both package manifests name
`@misofm/engine@0.1.0`, PR #353's accepted commit is an ancestor, and rejected
#352 commits are absent. It never publishes from a directory; only the already
smoked archive is the publish operand.

### Allowed paths

- `.github/ISSUE_SPECS/354-publish-engine-sdk-0-1-0.md`
- `.github/workflows/npm-publish.yml`

No SDK, Rust, host, sidecar, generated, lockfile, or artifact pin may change.

### Forbidden scope

- repinning either Wasm artifact;
- publishing from macOS, an unreviewed worktree, a package directory, or a
  different archive than the one smoked;
- including any rejected #352 change;
- changing the package API/version/content or moving the enginectl FLAC
  sidecar in this release issue;
- exposing npm credentials in logs/artifacts; or
- routine unpublish as rollback.

## Objective gates

1. PR #353 and all required checks pass and merge to `main`; the release commit
   descends from it and excludes the three rejected #352 commits.
2. GitHub-hosted Ubuntu with Rust 1.97.1 reproduces AudioWorklet digest
   `6dcd9ced2daeb886843a764bcc6abc0b4f1b2c7a50af1ed91151a5ab366461e5`
   and FLAC digest
   `a9fc3301cb6f290909e165fd5d21d7ded5fb3535d8c41472c93beed66173b65e`.
3. Generated, deletion, type, headless, enginectl, package, and tarball mutation
   gates pass against those exact artifact directories.
4. One final `.tgz` is packed, smoked, checksummed, uploaded, and used unchanged
   by `npm publish --provenance --access public --tag latest --ignore-scripts`.
5. Before publish, npm reports `@misofm/engine@0.1.0` absent. Ambiguous outcomes
   query and compare registry integrity before any retry.
6. After publish, registry version/shasum/integrity/public access/`latest` match
   local evidence, and a clean registry install imports `.`, `./headless`,
   `./browser`, `./assets` and runs `enginectl --version`.
7. Workflow shell syntax, action pinning/policy, workspace policy, formatting,
   and `git diff --check` pass. A qualification dispatch passes before publish.
8. Tag `sdk-v0.1.0` and GitHub release `@misofm/engine 0.1.0` point to the exact
   publishing SHA. Final evidence is synchronized to this issue before closure.

## Authentication and rollback

The first publish cannot use npm trusted publishing because the package does
not yet exist. Use a short-lived granular npm token with package write and
bypass-2FA stored only as GitHub secret `NPM_TOKEN`; the workflow has
`id-token: write` for provenance. After first publish, configure
`npm-publish.yml` as the trusted publisher and revoke/remove the temporary
token.

If publication is defective, deprecate 0.1.0 with an actionable message and
publish a reviewed 0.1.1. Do not routinely unpublish, and never reuse a version.

## Brief evidence and decision record

Sol reviewed current main and returned NO-GO for tagging `4797a544` directly
because accepted issue #351 had not been merged. It approved release after PR
#353 plus this publication-only checkpoint. The npm package does not contain
the PCM pump changed by #351; ordering is required for coherent repository
history, not tarball byte changes. The existing FLAC artifact remains an
external sidecar staged only for `enginectl --stems`; browser source plumbing
is still caller-owned. Splitting that CLI capability is outside this release.
