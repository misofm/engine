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

Add a manually dispatched, SHA-pinned npm publication workflow with three
modes. `qualify` alone installs Rust 1.97.1/Wasm, builds the AudioWorklet and
FLAC sidecar exactly once into empty directories, proves their pinned digests,
runs the generated/deletion/type/headless/package gates, packs and smokes one
final archive, records checksums/integrity, and uploads immutable evidence.

`publish` consumes that exact qualification archive, refuses an occupied
version, and sends the archive once with public access, `latest`, ignored
lifecycle scripts, and provenance. `verify` consumes the same artifact after
an ambiguous publish response and never sends a package. Both `publish` and
`verify` poll npm until version, shasum, integrity, access, and dist-tag match,
then fresh-install the exact version, import all public entries, run
`enginectl --version`, and verify the npm provenance attestation.

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
4. `qualify` packs, smokes, checksums, and uploads one final `.tgz`; `publish`
   uses that unchanged archive once with `--provenance --access public --tag
   latest --ignore-scripts`; `verify` never invokes publication.
5. Before publish, npm reports `@misofm/engine@0.1.0` absent. Ambiguous outcomes
   query and compare registry integrity before any retry.
6. After publish or verify recovery, registry version/shasum/integrity/public
   access/`latest` and the cryptographically verified SLSA provenance match
   local evidence; a clean registry install imports `.`, `./headless`,
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

## Attempt 1 implementation evidence (Terra, pending Sol review)

- Added dispatch-only Ubuntu workflow `.github/workflows/npm-publish.yml` with
  explicit `qualify` and `publish` modes. It refuses non-`main` refs, checks out
  only a required full expected SHA, requires that SHA in `origin/main`, validates
  both package manifests at `@misofm/engine@0.1.0`, proves PR #353's merged
  commit is ancestral, and refuses every named rejected #352 ancestor.
- It installs Node 22/npm plus Rust 1.97.1 and `wasm32-unknown-unknown`, builds
  both shipped Wasm closures once into empty directories, checks both locked Linux
  digests, then runs generated, deletion, type, headless, builder-contract, and
  package-preparation gates against those directories.
- Qualification packs exactly one archive, smokes that archive, performs `npm
  publish --dry-run` on it, records npm SHA-1/SHA-512 integrity plus SHA-256, and
  uploads the archive, listing, checksums, and pack metadata as evidence. Publish
  requires the ID of a successful earlier `qualify` job for the same SHA, downloads
  that immutable evidence, and re-smokes/rechecks its exact archive; it never
  rebuilds or publishes a presumed-identical replacement.
- Publish mode first accepts only a definite npm 404 as absence. It publishes the
  already-smoked archive with provenance/public/latest/ignored scripts and maps
  the `NPM_TOKEN` secret to `NODE_AUTH_TOKEN` only in that one step. A failed
  publish response is ambiguous rather than retried: registry version, shasum,
  integrity, access, and tag must match the recorded archive before the workflow
  can succeed. It then fresh-installs from npm, imports all four public entry
  points, and runs `enginectl --version`.

## Attempt 2 correction record (Terra, pending Sol review)

- The stateless state machine is now explicit: `qualify` alone installs the
  pinned toolchains, builds the two Wasm closures once, runs all qualification
  gates, packs and smokes one archive, and uploads immutable evidence. `publish`
  consumes that exact prior qualification artifact and sends it at most once.
  `verify` is the only recovery path after an ambiguous publish response: it
  downloads and re-smokes the same archive, polls the registry to convergence,
  and cannot invoke `npm publish`.
- The workflow has `actions: read` for the run/job and cross-run artifact APIs.
  `qualify` and `publish` seal the manual-dispatch main ref by requiring both
  `GITHUB_SHA` and checked-out `HEAD` to equal the requested SHA. `verify` may
  run after `main` advances, so it requires checked-out `HEAD` to equal that SHA
  and the SHA to remain ancestral to `origin/main`. Public access is queried as
  JSON through `npm access get status` and parsed as either the package-keyed
  object or a status object, not compared as incidental CLI text. Version,
  shasum, integrity, public access, and `latest` are polled as one convergence
  predicate.

## Attempt 3 correction record (Terra, pending final Sol review)

- npm is explicitly upgraded and asserted at `11.12.1` before packing, publishing,
  or verification. `publish` makes one `npm publish` request only; a nonzero
  response says it is proceeding to registry verification without retry. `verify`
  is a credential-free recovery mode which cannot reach that request.
- After convergence, both publish and verify create a fresh exact-version consumer
  lockfile, execute `npm audit signatures --json --include-attestations`, preserve
  its JSON as evidence, and fail closed unless one cryptographically verified
  `@misofm/engine@0.1.0` SLSA v1 DSSE attestation binds the tarball SHA-512,
  repository, workflow path/ref, and resolved git commit to the requested SHA.

## Root-owned post-publication closure (reserved)

This workflow intentionally ends at npm verification; it has no authority to
create repository tags, releases, or issue state. After a successful `publish`
or `verify` run, root must use a clean checkout and perform this fail-closed
closure before recording PASS/closing #354. Substitute the successful run's
`SHA`, `RUN_ID`, and `MODE` (`publish` or `verify`) exactly:

```sh
set -euo pipefail
SHA='SHA'
RUN_ID='RUN_ID'
MODE='MODE'
test "$MODE" = publish || test "$MODE" = verify
test "$SHA" = "$(printf '%s' "$SHA" | grep -E '^[0-9a-f]{40}$')"
test -z "$(git status --porcelain)"
git fetch origin main --tags
git merge-base --is-ancestor "$SHA" origin/main
evidence=$(mktemp -d)
trap 'rm -rf -- "$evidence"' EXIT
gh run download "$RUN_ID" --name "engine-sdk-$MODE-$SHA" --dir "$evidence"
npm view @misofm/engine@0.1.0 --json > "$evidence/registry-version.json"
npm access get status @misofm/engine --json > "$evidence/registry-access.json"
npm view @misofm/engine dist-tags.latest --json > "$evidence/registry-latest.json"
node - "$evidence" <<'NODE'
const fs = require('node:fs');
const root = process.argv[2];
const local = JSON.parse(fs.readFileSync(`${root}/tarball-evidence.json`, 'utf8'));
const registry = JSON.parse(fs.readFileSync(`${root}/registry-version.json`, 'utf8'));
const accessReply = JSON.parse(fs.readFileSync(`${root}/registry-access.json`, 'utf8'));
const latest = JSON.parse(fs.readFileSync(`${root}/registry-latest.json`, 'utf8'));
const access = typeof accessReply === 'string' ? accessReply : accessReply?.[local.package] ?? accessReply?.status;
if (registry.version !== local.version || registry.dist?.shasum !== local.shasum || registry.dist?.integrity !== local.integrity || access !== 'public' || latest !== local.version) throw new Error('npm registry does not match uploaded tarball evidence');
const audit = JSON.parse(fs.readFileSync(`${root}/npm-audit-signatures.json`, 'utf8'));
const verified = Array.isArray(audit.verified) ? audit.verified.filter((entry) => entry?.name === local.package && entry?.version === local.version) : [];
if (verified.length !== 1 || verified[0].integrity !== local.integrity) throw new Error('uploaded cryptographic provenance evidence does not uniquely bind the tarball');
NODE
if git rev-parse -q --verify refs/tags/sdk-v0.1.0 >/dev/null; then
  test "$(git rev-list -n1 sdk-v0.1.0)" = "$SHA"
else
  git tag -a sdk-v0.1.0 "$SHA" -m 'SDK 0.1.0'
  git push origin refs/tags/sdk-v0.1.0
fi
git fetch origin --tags
test "$(git rev-list -n1 refs/tags/sdk-v0.1.0)" = "$SHA"
test "$(git ls-remote origin 'refs/tags/sdk-v0.1.0^{}' | awk '{print $1}')" = "$SHA"
if gh release view sdk-v0.1.0 --json targetCommitish >/tmp/engine-sdk-0.1.0-release.json 2>/dev/null; then
  test "$(gh release view sdk-v0.1.0 --json targetCommitish --jq .targetCommitish)" = "$SHA"
else
  gh release create sdk-v0.1.0 --target "$SHA" --title '@misofm/engine 0.1.0' --generate-notes
fi
test "$(gh release view sdk-v0.1.0 --json targetCommitish --jq .targetCommitish)" = "$SHA"
printf '\n## Final PASS evidence\n\nPublished @misofm/engine@0.1.0 from %s; successful %s run %s supplied the verified archive and provenance evidence.\n' "$SHA" "$MODE" "$RUN_ID" >> .github/ISSUE_SPECS/354-publish-engine-sdk-0-1-0.md
git add .github/ISSUE_SPECS/354-publish-engine-sdk-0-1-0.md
git commit -m 'docs(#354): record npm publication evidence'
git push origin main
gh issue comment 354 --body "Published @misofm/engine@0.1.0 from $SHA; $MODE run $RUN_ID passed archive, registry, and provenance verification."
gh issue close 354
test "$(gh issue view 354 --json state --jq .state)" = CLOSED
```

This procedure downloads and mechanically compares the successful run's
tarball, registry, and provenance evidence before tag or release creation. It
stops on any pre-existing tag/release that targets a different commit, never
force-moves references, and commits/pushes final PASS evidence before the issue
comment and closure.

## Final Sol verdict: FAIL; split required

Attempt 3 stopped before commit or publication. The workflow is safe in that it
cannot reach a false PASS, but it is not executable to completion:

- the provenance parser reads `EXPECTED_SHA` without the step exporting that
  input, so its resolved-commit match always fails;
- the root closure expects a top-level `integrity` property that npm 11.12.1
  does not emit in `verified[]`; and
- the root closure does not prove the downloaded run succeeded or prove its
  evidence commit reached `origin/main` before closing the issue.

No npm bytes were published and no tag or GitHub release was created. Under the
three-attempt stop rule, implementation ends here. A fresh bounded successor
must repair and requalify the release workflow without weakening any gate.
## Successor handoff

Issue #355 owns the repaired closure procedure. This issue remains the evidence record for the stopped #354 attempt; no publication, tag, release, or issue closure occurred under #354.
