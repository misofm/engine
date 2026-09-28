# Releasing `@misofm/engine`

Owner decision 5 (`docs/rulings/engine-footprint-2026-09-28.md`, issue #1061): the shipped
AudioWorklet module's fingerprint is checked at release, not on every change. This is what each
kind of change must do from now on.

## What runs when

| change | what CI does with the shipped module |
| --- | --- |
| any PR on the `sdk` or `full` route, and every `main` push on them | `artifact` builds the module once (`scripts/build-web-audioworklet.sh`); `sdk`, `artifact-gates` and the three browser legs download it and check it against that job's digest, so every artifact gate reads exactly those bytes. `artifact-identity` builds the same commit again from another checkout path and `CARGO_HOME` and fails if the bytes differ, builds the base's module with the base's own build script and pinned toolchain (so a toolchain bump with no source change reports `ARTIFACT CHANGED`), and prints `ARTIFACT CHANGED` or `ARTIFACT UNCHANGED` in its job summary. Nothing is compared with the committed pin and nothing is re-pinned. |
| a PR on the `evidence` route (documentation only) | no module is built; the verdict's summary says `ARTIFACT UNCHANGED`, because no build reads those paths. |
| a **release change**: a PR, or the `main` push that merges it, that edits the pin file or `npm-publish.yml`'s `PACKAGE_VERSION` or `EXPECTED_WORKLET_SHA256` | all of the above, and `artifact-identity` also requires the pin to be the built digest, `EXPECTED_WORKLET_SHA256` to be the pin, and `hosts/host-web/qualification/results.json` to record a three-browser qualification of the built digest (`wasmSha256`) with a canonical `candidateCommit`. |
| `npm-publish.yml` (unchanged by #1061) | refuses to start unless `EXPECTED_WORKLET_SHA256` is the pin file; `qualify` rebuilds the module and refuses bytes other than `EXPECTED_WORKLET_SHA256`; `publish` and `verify` send or recheck only the archive `qualify` produced. |

An issue gate that claims "shipped artifact unchanged" cites its PR's `ARTIFACT UNCHANGED` line;
one that expects a change cites the `ARTIFACT CHANGED` line and says why the bytes moved.

"Base" is the commit the tested tree sits on: for a PR, the first parent of GitHub's merge commit
(the tip of `main` it merges onto), so the line reports only what the PR does to the module; for a
`main` push, the previous tip. A `workflow_dispatch` run has no base: it says `ARTIFACT BASE
UNAVAILABLE` and checks no release. `scripts/web-audioworklet-identity.py` has the rules and their
self-test.

## Between releases

Nothing re-pins the pin (`hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256`),
`results.json` or `hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md` between releases. From the first
release PR under #1061 on, they describe the last release. **Until then they do not:**

- The pin is `f7bd75ca...`, the last per-change re-pin, from the slim-1 batch (`570a79f0`), and
  `results.json` and the matrix record the three-browser qualification of that module.
- Release 0.4.3 shipped `e18acf9c...`, which is `npm-publish.yml`'s `EXPECTED_WORKLET_SHA256`.
- So `npm-publish.yml`'s first step ("Assert the worklet sha256 pin matches its source-of-truth
  file") refuses on `main` today. That predates #1061, and the next release PR clears it.

The browser legs still check every browser row, the Playwright version and the generated matrix
against `results.json` on every PR (`--check-matrix`). Re-record the matrix when Playwright or a
browser floor changes (step 3 below, without the re-pin); that record names the module it ran.

## The release PR

One PR, on the tip of `main`, that changes no Rust source, so the bytes it pins are the bytes `main`
builds once it merges.

1. **Build and read the digest.**

   ```sh
   artifacts=$(mktemp -d)
   bash scripts/build-web-audioworklet.sh "$artifacts"   # prints: AudioWorklet module <digest>
   ```

2. **Re-pin.** Set the pin to the digest step 1 printed: the module of the release commit's own
   tree, built with the toolchain its `qualification.yml` pins, not a module from an earlier
   batch or from a release. Write `<digest>` and a newline to
   `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256`, then confirm with
   `bash scripts/build-web-audioworklet.sh --check-pin "$(mktemp -d)"`. The same digest goes into
   `npm-publish.yml`'s `EXPECTED_WORKLET_SHA256` (step 4).

3. **Record a fresh three-browser qualification of those bytes.** On Linux a real `AudioContext`
   needs an output device. Start the null PulseAudio sink exactly as `qualification.yml`'s browser
   job does before running, with its socket directory on a short path such as
   `mktemp -d /tmp/qa.XXXXXX`: a UNIX socket path must stay under 108 bytes. From a deep scratch
   path the sink never starts, and firefox fails with its `AudioContext` still suspended.

   ```sh
   (cd sdk && npm ci)
   cd hosts/host-web/qualification
   npm ci && npx playwright install chromium firefox webkit
   npm run qualify -- --artifacts "$artifacts" --sdk-root ../../../sdk --browser all \
     --record-matrix --candidate-commit "$(git rev-parse HEAD)" --self-test-mutations
   ```

   This rewrites `results.json` and `BROWSER_DEPLOYMENT_MATRIX.md`. `--candidate-commit` names the
   commit whose tree you built.

4. **Set the release identity.**
   - `.github/workflows/npm-publish.yml`: `PACKAGE_VERSION`, the version in the job name and in the
     three other literals that carry it (five in all), and `EXPECTED_WORKLET_SHA256: "<digest>"`.
   - `scripts/test-npm-publish-modes.py`: `VERSION`, the version literals it normalizes, and the
     accepted `release_pin` and its "wrong release pin" mutation (both `<digest>`). Run it.
   - `sdk/package.json`, `sdk/package-lock.json`, the install line and release-record link in
     `sdk/README.md`, and the release's issue spec.

5. **Open the PR.** Its `artifact-identity` summary must show a `RELEASE:` line with every check
   passing, and all three browser legs must pass on the built bytes. The merge push to `main` runs
   the same release checks on the merged tree; if another merge changed the module in between, they
   fail there, and the release needs a new release PR.

6. **Publish** from `main` with `npm-publish.yml`: `qualify` with `expected_sha` set to the merge
   commit, then `publish` with that run's ID as `qualification_run_id` (`verify` only to recheck
   an ambiguous publish). The app's provenance file takes the pin from this release.
