# Releasing `@misofm/engine`

## What runs when

| when | what runs |
| --- | --- |
| a pull request, a push to `main`, a schedule | nothing. No status check is required to merge. |
| a release | `npm-publish.yml`, about 10 minutes from dispatch to published (below) |
| by hand, when a change needs it | `qualification.yml` (the full gate set, about 25 minutes), `nightly.yml` (deep suites), `fuzz.yml` (fuzz smoke): `gh workflow run <file> --ref <branch>` |

Run `qualification.yml` on a branch when a change reaches beyond what a release checks: AArch64,
the C ABI, realtime audits, the DSP crates' own suites, fmt and clippy. Its jobs and the scripts it
calls are unchanged, so a manual run is the same evidence an old per-PR run was. A run has no base,
so `artifact-identity` reports `ARTIFACT CANNOT TELL` and checks no release fingerprint.

## Release

```sh
gh workflow run npm-publish.yml -R misofm/engine -f version=patch   # or minor, major, X.Y.Z
gh workflow run npm-publish.yml -R misofm/engine -f version=patch -f dry_run=true
```

Dispatch on `main`; a dry run may use any branch (`--ref <branch>`). Nothing is committed and no
release PR is needed. One run:

1. **prepare** resolves the version against npm: `patch`, `minor` or `major` bumps npm's `latest`;
   an exact `X.Y.Z` must be above it. A version already on npm is refused.
2. **artifact** builds the shipped closure (`scripts/build-web-audioworklet.sh`) and its named twin
   once, and checks the modules name the browser artifact toolchain's rustc. Every later job
   downloads these bytes and checks them against this job's digests.
3. In parallel, against those bytes:
   - **package**: stamps the version into `sdk/package.json` and its lock, runs the generated
     surface, deletion, type and headless gates, builds the package, packs exactly one tarball,
     checks it carries the built modules, smokes it and dry-run publishes it;
   - **artifact-gates**: the frozen export set and single import, the allocation-free and
     trap-free render closure, native/simd128 digest parity over the expected resources, no
     scalar oracle, the hermetic host and worklet tests, and the V8 spill gate;
   - **browser**: Chromium and WebKit, `single` (no cross-origin isolation) and `worker`
     (isolated, control Worker) legs, rendering the module in a real `AudioContext`, with
     `--check-matrix` against `hosts/host-web/qualification/results.json`;
   - **core-tests**: `host-web`, `session` and `protocol` tests: the crate that is the module, and
     the session JSON and control protocol the SDK speaks. Other crates' output reaches the parity
     and expected-resource digests above.
4. **publish** sends the tarball **package** tested (its sha256 is checked) with
   `npm publish --provenance` through npm trusted publishing (OIDC, no token), or
   `npm publish --dry-run` on a dry run. It then waits until the registry serves that tarball as
   `latest` and public, installs it fresh, imports every entry, runs `enginectl --version`, and
   checks the SLSA provenance names this workflow on `main` and the run's commit. A non-zero
   `npm publish` is checked against the registry, never retried: npm versions are immutable.
5. **record** (not on a dry run) creates the GitHub Release `v<version>` on the source commit:
   the module, BLAKE3 module, named twin and tarball digests, the toolchain, and the npm integrity.
   It attaches the tarball and the named twin.

A failed check job can be rerun alone, `gh run rerun <run> --failed`: it reads the same artifact,
and `publish` runs once every check is green. The browser legs' suspend-resume stall gate has
stalled once in WebKit's `worker` leg and passed on rerun.

`sdk/package.json` carries `0.0.0-development`; a version exists only in a release. To rebuild
one: `git checkout v<version>`, then `npm version <version> --no-git-tag-version` in `sdk/` and the
same build. Write upgrade notes (`sdk/README.md`, "Upgrading from ...") in the change that needs
them.

### Security

- npm's trusted publisher for the package is `misofm/engine`, workflow `npm-publish.yml`, with no
  environment. Renaming the file breaks publishing until the trusted publisher is updated.
- Only the `publish` job holds `id-token: write`, and it publishes only on `main`. Only the
  `record` job holds `contents: write`; it checks out and runs no repository code. Everything
  else reads.

## The browser artifact toolchain

Decision 15, D15-10 (issue #1334): the shipped module alone builds on a pinned, dated nightly with
`-Zbuild-std`, named once, in `rust-toolchain.toml`'s `[browser-artifact] channel` (rustup ignores
that table; every other build stays on the stable `[toolchain]` channel).
`bash scripts/build-web-audioworklet.sh --print-toolchain` prints it, and every job that runs the
builder installs exactly that, with `rust-src`. Bump it only with a re-recorded two-browser matrix
of the module it builds (below) and a green manual `qualification.yml` run (AArch64 and native
parity).

## Recording the browser matrix

`results.json` and `hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md` record each browser's floor,
outcome and the Playwright version; the browser legs check them with `--check-matrix` on every
release. Re-record them when Playwright, a browser floor or the browser artifact toolchain
changes. On Linux a real `AudioContext` needs an output device: start the null PulseAudio sink as
the browser job does, with its socket directory on a short path such as `mktemp -d /tmp/qa.XXXXXX`
(a UNIX socket path stays under 108 bytes).

```sh
artifacts=$(mktemp -d)
bash scripts/build-web-audioworklet.sh "$artifacts"
(cd sdk && npm ci)
cd hosts/host-web/qualification
npm ci && npx playwright install chromium webkit
npm run qualify -- --artifacts "$artifacts" --sdk-root ../../../sdk --browser all --leg all \
  --record-matrix --candidate-commit "$(git rev-parse HEAD)" --self-test-mutations
```

`--blake3-only --record-blake3 --candidate-commit <sha>` re-records only
`results.json.blake3Qualification`.

The committed pins (`hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256` and
`...-blake3-artifact.sha256`) hold the modules of the last release made by the retired release-PR
flow. Releases no longer re-pin them; the GitHub Release is the record.
`bash scripts/build-web-audioworklet.sh --check-pin <dir>` still compares a build with them.

## The named twin: symbolicating a production stack trace

Issue #1109: the shipped module carries no wasm `name` section, so a browser prints its frames as
`wasm-function[N]`. The same build produces the **named twin**,
`miso-engine-v1-audio-worklet.simd128.named.wasm`: the shipped module plus that one section and
nothing else. It is never published to npm; each GitHub Release attaches it. To rebuild it from a
release's tag:

```sh
shipped=$(mktemp -d) named=$(mktemp -d)
bash scripts/build-web-audioworklet.sh --module-only --named-twin "$named" "$shipped"
python3 -B scripts/strip-wasm-names.py check \
  "$named/miso-engine-v1-audio-worklet.simd128.named.wasm" \
  "$shipped/miso-engine-v1-audio-worklet.simd128.wasm"
```

The check passes only when the shipped module is the twin minus its `name` section, every other
section byte-identical, so function indices and code offsets agree. Frame `wasm-function[N]` is
the twin's function `N`: `wasm-objdump -x -j name <twin> | grep 'func\[N\]'` names it.
