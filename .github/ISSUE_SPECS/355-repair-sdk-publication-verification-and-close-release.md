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

- `.github/ISSUE_SPECS/354-publish-engine-sdk-0-1-0.md` (successor and final
  closure evidence only)
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

Sol found this a legitimate bounded successor and required the following frozen
contract before approval. npm 11.12.1 provenance verification must require:

- no target package in `invalid` or `missing` and exactly one `verified` entry
  named `@misofm/engine@0.1.0`;
- exactly one `verified[0].attestationBundles` entry whose predicate type is
  `https://slsa.dev/provenance/v1`;
- DSSE payload type `application/vnd.in-toto+json`, decoded statement type
  `https://in-toto.io/Statement/v1`, exact subject PURL
  `pkg:npm/%40misofm/engine@0.1.0`, and subject SHA-512 equal to the qualified
  tarball's hexadecimal SHA-512; and
- exact repository `https://github.com/misofm/engine`, workflow path
  `.github/workflows/npm-publish.yml`, ref `refs/heads/main`, and exactly one
  matching resolved dependency whose `gitCommit` is the explicitly exported
  `expected_sha`.

Successful `npm audit signatures` supplies cryptographic envelope verification;
the parser fail-closes on identity and statement contents.

Post-publication closure must prove the selected run is a successful
`workflow_dispatch` of this exact workflow and expected publish/verify job. A
publish run's `head_sha` must equal the release SHA. The exact named, unexpired
artifact must belong to that run. Closure rechecks its tarball digests, registry
integrity/access/latest, and provenance before creating references.

Repository closure begins on clean local `main` synchronized with `origin/main`.
It refuses conflicting references, verifies the remote peeled tag and the
release's tag/title/non-draft/non-prerelease/target, appends PASS evidence to
#355 plus a successor-closure note to #354, pushes and proves that exact evidence
commit is in `origin/main`, refreshes both GitHub issue bodies from the local
specs, then closes and rereads both issues. Configure npm trusted publishing for
this repository/workflow and remove/revoke the temporary token before final
issue closure.

## Attempt 1 implementation evidence (Terra, pending Sol review)

- Exported the requested `expected_sha` directly into the npm provenance parser.
  The parser now follows npm 11.12.1's frozen verified-array shape: exactly one
  target `verified` entry, no target `invalid`/`missing`/`unverified` entries,
  exactly one SLSA v1 `attestationBundles` item, its required DSSE payload type,
  the exact PURL and hexadecimal tarball SHA-512, trusted workflow identity, and
  one resolved `gitCommit` equal to that exported SHA.
- Replaced the #354 closure's nonexistent `verified[].integrity` assumption with
  the same DSSE statement checks. The concrete closure now proves its selected
  publish/verify workflow run and job passed, the named evidence artifact is
  unexpired and belongs to that run, and only then downloads it.
- The closure now starts from clean synchronized local `main`, rechecks remote
  tag/release identity, commits both #355 PASS evidence and #354 successor
  closure evidence, proves that exact evidence commit is `origin/main`, refreshes
  both GitHub issue bodies, and closes/rereads both issues. Package bytes, SDK
  gates, sidecars, and artifact pins remain untouched.

## Attempt 2 correction record and root closure (Terra, pending Sol review)

The frozen npm 11.12.1 JSON shape places an attestation envelope at
`verified[0].attestationBundles[*].bundle.dsseEnvelope`; it is not a direct
property of an attestation-bundle entry. Both workflow and closure now require
that shape and the signed statement's `predicateType` as well as the bundle's.

The workflow ends after npm verification. Root owns this explicit, fail-closed
account and repository closure. Substitute exact non-secret values for `SHA`,
`RUN_ID`, `MODE`, and the npm token identifier `TEMP_NPM_TOKEN_ID`. The trust
and token commands can open an npm browser-authentication/2FA flow; that pause
is expected. Resume only after the same account completes authentication; never
put the token value in a command, file, log, artifact, or issue.

```sh
set -euo pipefail
REPOSITORY=misofm/engine
SHA='SHA'
RUN_ID='RUN_ID'
MODE='MODE'
TEMP_NPM_TOKEN_ID='TEMP_NPM_TOKEN_ID'
test "$REPOSITORY" = misofm/engine
test "$MODE" = publish || test "$MODE" = verify
test "$SHA" = "$(printf '%s' "$SHA" | grep -E '^[0-9a-f]{40}$')"
test "$TEMP_NPM_TOKEN_ID" = "$(printf '%s' "$TEMP_NPM_TOKEN_ID" | grep -E '^[A-Za-z0-9_-]{1,128}$')"
test -z "$(git status --porcelain)"
test "$(git branch --show-current)" = main
git fetch origin main --tags
test "$(git rev-parse HEAD)" = "$(git rev-parse origin/main)"
git merge-base --is-ancestor "$SHA" origin/main
evidence=$(mktemp -d)
trap 'rm -rf -- "$evidence"' EXIT
gh api "repos/$REPOSITORY/actions/runs/$RUN_ID" > "$evidence/run.json"
gh api "repos/$REPOSITORY/actions/runs/$RUN_ID/jobs?per_page=100" > "$evidence/jobs.json"
gh api "repos/$REPOSITORY/actions/runs/$RUN_ID/artifacts?per_page=100" > "$evidence/artifacts.json"
node - "$evidence/run.json" "$evidence/jobs.json" "$evidence/artifacts.json" "$SHA" "$MODE" <<'NODE'
const fs = require('node:fs');
const [runPath, jobsPath, artifactsPath, sha, mode] = process.argv.slice(2);
const run = JSON.parse(fs.readFileSync(runPath, 'utf8'));
const jobs = JSON.parse(fs.readFileSync(jobsPath, 'utf8'));
const artifacts = JSON.parse(fs.readFileSync(artifactsPath, 'utf8')).artifacts;
if (run.conclusion !== 'success' || run.event !== 'workflow_dispatch' || run.name !== 'Publish @misofm/engine' || !String(run.path ?? '').startsWith('.github/workflows/npm-publish.yml@')) throw new Error('selected run is not a successful manual npm-publish workflow');
if (mode === 'publish' && run.head_sha !== sha) throw new Error('publish run head SHA differs from release SHA');
if (!jobs.jobs?.some((job) => job.name === `${mode} @misofm/engine 0.1.0` && job.conclusion === 'success')) throw new Error('selected run did not successfully complete the expected job');
const named = Array.isArray(artifacts) ? artifacts.filter((artifact) => artifact?.name === `engine-sdk-${mode}-${sha}` && artifact?.expired === false) : [];
if (named.length !== 1) throw new Error('selected run lacks exactly one unexpired named evidence artifact');
NODE
gh run download "$RUN_ID" --repo "$REPOSITORY" --name "engine-sdk-$MODE-$SHA" --dir "$evidence"
shopt -s nullglob
archives=("$evidence"/*.tgz)
test ${#archives[@]} = 1
node - "$evidence/tarball-evidence.json" "${archives[0]}" <<'NODE'
const fs = require('node:fs');
const crypto = require('node:crypto');
const local = JSON.parse(fs.readFileSync(process.argv[2], 'utf8'));
const bytes = fs.readFileSync(process.argv[3]);
const actual = {
  shasum: crypto.createHash('sha1').update(bytes).digest('hex'),
  sha256: crypto.createHash('sha256').update(bytes).digest('hex'),
  sha512: crypto.createHash('sha512').update(bytes).digest('hex'),
  integrity: `sha512-${crypto.createHash('sha512').update(bytes).digest('base64')}`,
};
for (const [key, value] of Object.entries(actual)) if (local[key] !== value) throw new Error(`downloaded tarball ${key} differs from evidence`);
NODE
npm install --global npm@11.12.1
test "$(npm --version)" = 11.12.1
npm view @misofm/engine@0.1.0 --json > "$evidence/registry-version.json"
npm access get status @misofm/engine --json > "$evidence/registry-access.json"
npm view @misofm/engine dist-tags.latest --json > "$evidence/registry-latest.json"
node - "$evidence" "$SHA" <<'NODE'
const fs = require('node:fs');
const root = process.argv[2];
const expectedSha = process.argv[3];
const local = JSON.parse(fs.readFileSync(`${root}/tarball-evidence.json`, 'utf8'));
const registry = JSON.parse(fs.readFileSync(`${root}/registry-version.json`, 'utf8'));
const accessReply = JSON.parse(fs.readFileSync(`${root}/registry-access.json`, 'utf8'));
const latest = JSON.parse(fs.readFileSync(`${root}/registry-latest.json`, 'utf8'));
const access = typeof accessReply === 'string' ? accessReply : accessReply?.[local.package] ?? accessReply?.status;
if (registry.version !== local.version || registry.dist?.shasum !== local.shasum || registry.dist?.integrity !== local.integrity || access !== 'public' || latest !== local.version) throw new Error('npm registry does not match tarball evidence');
const audit = JSON.parse(fs.readFileSync(`${root}/npm-audit-signatures.json`, 'utf8'));
const target = (entry) => entry?.name === local.package && entry?.version === local.version;
const verified = Array.isArray(audit.verified) ? audit.verified.filter(target) : [];
const rejected = ['invalid', 'missing', 'unverified'].flatMap((key) => Array.isArray(audit[key]) ? audit[key].filter(target) : []);
if (verified.length !== 1 || rejected.length !== 0) throw new Error('audit did not verify exactly one target package');
const bundles = Array.isArray(verified[0].attestationBundles) ? verified[0].attestationBundles.filter((entry) => entry?.predicateType === 'https://slsa.dev/provenance/v1') : [];
const envelope = bundles.length === 1 ? bundles[0].bundle?.dsseEnvelope : undefined;
if (!envelope || envelope.payloadType !== 'application/vnd.in-toto+json' || typeof envelope.payload !== 'string') throw new Error('audit lacks one required SLSA v1 DSSE envelope');
const statement = JSON.parse(Buffer.from(envelope.payload, 'base64').toString('utf8'));
if (statement?._type !== 'https://in-toto.io/Statement/v1' || statement?.predicateType !== 'https://slsa.dev/provenance/v1') throw new Error('signed statement is not SLSA provenance v1');
const subjects = Array.isArray(statement.subject) ? statement.subject.filter((subject) => subject?.name === 'pkg:npm/%40misofm/engine@0.1.0' && subject?.digest?.sha512 === local.sha512) : [];
const workflow = statement?.predicate?.buildDefinition?.externalParameters?.workflow;
const dependencies = Array.isArray(statement?.predicate?.buildDefinition?.resolvedDependencies) ? statement.predicate.buildDefinition.resolvedDependencies.filter((dependency) => dependency?.digest?.gitCommit === expectedSha) : [];
if (subjects.length !== 1 || workflow?.repository !== 'https://github.com/misofm/engine' || workflow?.path !== '.github/workflows/npm-publish.yml' || workflow?.ref !== 'refs/heads/main' || dependencies.length !== 1) throw new Error('signed statement does not bind the tarball, publisher, and release SHA');
NODE
if git rev-parse -q --verify refs/tags/sdk-v0.1.0 >/dev/null; then test "$(git rev-list -n1 sdk-v0.1.0)" = "$SHA"; else git tag -a sdk-v0.1.0 "$SHA" -m 'SDK 0.1.0'; git push origin refs/tags/sdk-v0.1.0; fi
git fetch origin --tags
test "$(git rev-list -n1 refs/tags/sdk-v0.1.0)" = "$SHA"
test "$(git ls-remote origin 'refs/tags/sdk-v0.1.0^{}' | awk '{print $1}')" = "$SHA"
if ! gh release view sdk-v0.1.0 --repo "$REPOSITORY" --json targetCommitish,name,isDraft,isPrerelease > "$evidence/release.json" 2>/dev/null; then gh release create sdk-v0.1.0 --repo "$REPOSITORY" --target "$SHA" --title '@misofm/engine 0.1.0' --generate-notes; fi
gh release view sdk-v0.1.0 --repo "$REPOSITORY" --json targetCommitish,name,isDraft,isPrerelease > "$evidence/release.json"
node - "$evidence/release.json" "$SHA" <<'NODE'
const fs = require('node:fs'); const release = JSON.parse(fs.readFileSync(process.argv[2], 'utf8'));
if (release.targetCommitish !== process.argv[3] || release.name !== '@misofm/engine 0.1.0' || release.isDraft || release.isPrerelease) throw new Error('remote release identity is wrong');
NODE
# The following account commands may pause for browser authentication/2FA. Do not bypass that pause.
npm install --global npm@11.19.0
test "$(npm --version)" = 11.19.0
npm trust list @misofm/engine --json > "$evidence/npm-trust-before.json"
trust_state=$(node - "$evidence/npm-trust-before.json" <<'NODE'
const fs = require('node:fs'); const entries = JSON.parse(fs.readFileSync(process.argv[2], 'utf8'));
if (!Array.isArray(entries)) throw new Error('npm trust list did not return an array');
const exact = (entry) => entry?.type === 'github' && entry?.file === 'npm-publish.yml' && entry?.repository === 'misofm/engine' && Array.isArray(entry?.permissions) && entry.permissions.length === 1 && entry.permissions[0] === 'createPackage';
if (entries.length === 0) process.stdout.write('absent');
else if (entries.length === 1 && exact(entries[0])) process.stdout.write('present');
else throw new Error('existing trusted-publisher configuration conflicts with the exact release trust');
NODE
)
case "$trust_state" in
  absent) npm trust github @misofm/engine --file npm-publish.yml --repository "$REPOSITORY" --allow-publish --yes ;;
  present) ;;
  *) echo "invalid trusted-publisher inspection state" >&2; exit 1 ;;
esac
npm trust list @misofm/engine --json > "$evidence/npm-trust.json"
node - "$evidence/npm-trust.json" <<'NODE'
const fs = require('node:fs'); const entries = JSON.parse(fs.readFileSync(process.argv[2], 'utf8'));
if (!Array.isArray(entries) || entries.length !== 1) throw new Error('npm trust list does not contain exactly one publisher');
const entry = entries[0];
if (entry?.type !== 'github' || entry?.file !== 'npm-publish.yml' || entry?.repository !== 'misofm/engine' || !Array.isArray(entry?.permissions) || entry.permissions.length !== 1 || entry.permissions[0] !== 'createPackage') throw new Error('npm trusted publisher differs from the exact GitHub publish trust');
NODE
gh secret list --repo "$REPOSITORY" --json name > "$evidence/github-secrets-before.json"
secret_state=$(node - "$evidence/github-secrets-before.json" <<'NODE'
const fs = require('node:fs'); const secrets = JSON.parse(fs.readFileSync(process.argv[2], 'utf8'));
if (!Array.isArray(secrets)) throw new Error('GitHub secret listing is not an array');
const matches = secrets.filter((secret) => secret?.name === 'NPM_TOKEN');
if (matches.length > 1) throw new Error('GitHub secret listing has ambiguous NPM_TOKEN entries');
process.stdout.write(matches.length === 1 ? 'present' : 'absent');
NODE
)
case "$secret_state" in present) gh secret delete NPM_TOKEN --repo "$REPOSITORY" ;; absent) ;; *) exit 1 ;; esac
gh secret list --repo "$REPOSITORY" --json name > "$evidence/github-secrets.json"
node - "$evidence/github-secrets.json" <<'NODE'
const fs = require('node:fs'); const secrets = JSON.parse(fs.readFileSync(process.argv[2], 'utf8'));
if (!Array.isArray(secrets) || secrets.some((secret) => secret?.name === 'NPM_TOKEN')) throw new Error('NPM_TOKEN remains configured');
NODE
npm token list --json > "$evidence/npm-tokens-before.json"
token_state=$(node - "$evidence/npm-tokens-before.json" "$TEMP_NPM_TOKEN_ID" <<'NODE'
const fs = require('node:fs'); const [path, id] = process.argv.slice(2); const tokens = JSON.parse(fs.readFileSync(path, 'utf8'));
if (!Array.isArray(tokens)) throw new Error('npm token list did not return an array');
const matches = tokens.filter((token) => token?.key === id || token?.id === id);
if (matches.length > 1) throw new Error('temporary token identifier is ambiguous');
process.stdout.write(matches.length === 1 ? 'present' : 'absent');
NODE
)
case "$token_state" in present) npm token revoke "$TEMP_NPM_TOKEN_ID" ;; absent) ;; *) exit 1 ;; esac
npm token list --json > "$evidence/npm-tokens.json"
node - "$evidence/npm-tokens.json" "$TEMP_NPM_TOKEN_ID" <<'NODE'
const fs = require('node:fs'); const [path, id] = process.argv.slice(2); const tokens = JSON.parse(fs.readFileSync(path, 'utf8'));
if (!Array.isArray(tokens) || tokens.some((token) => token?.key === id || token?.id === id)) throw new Error('temporary granular npm token remains listed');
NODE
printf '\n## Successor closure evidence\n\nIssue #355 closure verified release %s from %s run %s.\n' "$SHA" "$MODE" "$RUN_ID" >> .github/ISSUE_SPECS/354-publish-engine-sdk-0-1-0.md
printf '\n## Final PASS evidence\n\nPublished @misofm/engine@0.1.0 from %s; successful %s run %s passed archive, registry, provenance, trust, and credential-retirement checks.\n' "$SHA" "$MODE" "$RUN_ID" >> .github/ISSUE_SPECS/355-repair-sdk-publication-verification-and-close-release.md
git add .github/ISSUE_SPECS/354-publish-engine-sdk-0-1-0.md .github/ISSUE_SPECS/355-repair-sdk-publication-verification-and-close-release.md
git commit -m 'docs(#355): record npm publication evidence'
evidence_commit=$(git rev-parse HEAD)
git push origin main
git fetch origin main
test "$(git rev-parse origin/main)" = "$evidence_commit"
gh issue edit 354 --repo "$REPOSITORY" --body-file .github/ISSUE_SPECS/354-publish-engine-sdk-0-1-0.md
gh issue edit 355 --repo "$REPOSITORY" --body-file .github/ISSUE_SPECS/355-repair-sdk-publication-verification-and-close-release.md
gh issue close 354 --repo "$REPOSITORY"
gh issue close 355 --repo "$REPOSITORY"
test "$(gh issue view 354 --repo "$REPOSITORY" --json state --jq .state)" = CLOSED
test "$(gh issue view 355 --repo "$REPOSITORY" --json state --jq .state)" = CLOSED
```

## Attempt 3 correction record (Terra, pending final Sol review)

- Archive and provenance verification remain on frozen npm `11.12.1`. The
  separately account-scoped trusted-publisher transition upgrades and asserts npm
  `11.19.0`, whose `npm trust list @misofm/engine --json` shape is an array of
  `{ id, type, file, repository, permissions }` entries. It resumes an existing
  one-exact-entry configuration, creates only an absent configuration, and fails
  on multiple or conflicting entries.
- Secret and token retirement are now idempotent: each is inspected first,
  removed only when its exact identifier is uniquely present, re-listed, and
  required absent. Partial token-key matches are never accepted. The browser
  authentication/2FA pause for npm account operations remains explicit.

## Final Sol verdict: FAIL; trust parser successor required

Attempt 3 stopped before commit or publication. npm 11.19.0 emits an empty body
for an absent trusted publisher and one standalone JSON object for one publisher;
the closure incorrectly requires a JSON array in both cases. All preceding
workflow, archive, provenance, run/reference, credential-retirement, and
synchronization gates passed review. No npm bytes, tag, release, trust, secret,
token, or issue state changed. A bounded successor must correct and fixture-test
only this trust-list parser before release execution.
