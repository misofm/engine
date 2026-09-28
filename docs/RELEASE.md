# Releasing `@misofm/engine`

Owner decision 5 (`docs/rulings/engine-footprint-2026-09-28.md`, issue #1061): the shipped
AudioWorklet module's fingerprint is checked at release, not on every change. This is what each
kind of change must do from now on.

## What runs when

| change | what CI does with the shipped module |
| --- | --- |
| any PR on the `sdk` or `full` route, and every `main` push on them | `artifact` builds the module once (`scripts/build-web-audioworklet.sh`); `sdk`, `artifact-gates` and the three browser legs download it and check it against that job's digest, so every artifact gate reads exactly those bytes. `artifact-identity` builds the same commit again from another checkout path and `CARGO_HOME` and fails if the bytes differ, then compares the module with **the digest the base commit's own CI run recorded**, and prints `ARTIFACT CHANGED`, `ARTIFACT UNCHANGED` or `ARTIFACT CANNOT TELL` (with the reason) in its job summary. Nothing is compared with the committed pin and nothing is re-pinned. |
| a `main` push on those routes | also records the module it built: `artifact` posts the commit status `audioworklet-sha256` (`<sha256> rustc <release>`, linking the run) on the pushed commit. That record is what later changes compare against. |
| a PR on the `evidence` route (documentation only) | no module is built; the verdict's summary says `ARTIFACT UNCHANGED`, because no build reads those paths. |
| a **release change**: a PR, or the `main` push that merges it, that edits the pin file or `npm-publish.yml`'s `PACKAGE_VERSION` or `EXPECTED_WORKLET_SHA256` | all of the above, and `artifact-identity` also requires the pin to be the built digest, `EXPECTED_WORKLET_SHA256` to be the pin, `npm-publish.yml`'s `RUSTUP_TOOLCHAIN` to be the rustc release the `artifact` job built with, and `hosts/host-web/qualification/results.json` to record a three-browser qualification of the built digest (`wasmSha256`) with a canonical `candidateCommit`. |
| `npm-publish.yml` (unchanged by #1061) | refuses to start unless `EXPECTED_WORKLET_SHA256` is the pin file; `qualify` rebuilds the module and refuses bytes other than `EXPECTED_WORKLET_SHA256`; `publish` and `verify` send or recheck only the archive `qualify` produced. |

An issue gate that claims "shipped artifact unchanged" cites its PR's `ARTIFACT UNCHANGED` line;
one that expects a change cites the `ARTIFACT CHANGED` line and says why the bytes moved.

"Base" is the commit the tested tree sits on: for a PR, the first parent of GitHub's merge commit
(the tip of `main` it merges onto), so the line reports only what the PR does to the module; for a
`main` push, the previous tip.

The base's module is never rebuilt in the change's own run. That run's workflow is the change's,
so a rebuilt base inherits whatever build environment the change sets (a toolchain, a cargo
variable, a `PATH` entry) and can read UNCHANGED when the module moved (#1061 attempts 1 and 2).
The comparison is with the digest the base's own run built and recorded.

`ARTIFACT CANNOT TELL` does not fail the job. It appears when:
- the run has no base (a `workflow_dispatch` run, which also checks no release);
- the base's run has not reached its `artifact` job yet. Rerun `artifact-identity` once it has;
- the base's run never built the module, for example a documentation-only push. Such a base uses
  its nearest recorded first-parent ancestor, up to 20 back, when everything between them is
  documentation the router's evidence route admits;
- the base predates #1061 and has no record.

A status lookup the API refuses fails the job instead, since that is a misconfiguration, not a
missing record. `scripts/web-audioworklet-identity.py` has the rules and their self-test.

## Between releases

Nothing re-pins the pin (`hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256`),
`results.json` or `hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md` between releases. From the first
release PR under #1061 on, they describe the last release. **Until then they do not:**

- They hold the last per-change re-pin, made at a batch boundary before #1061, and its
  three-browser record: `git log -1 -- <pin file>` names it.
- That is not the module release 0.4.3 shipped, whose digest is `npm-publish.yml`'s
  `EXPECTED_WORKLET_SHA256`.
- So while the two differ, `npm-publish.yml`'s first step ("Assert the worklet sha256 pin matches
  its source-of-truth file") refuses on `main`. That predates #1061, and the next release PR clears
  it.

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
   `npm-publish.yml`'s `EXPECTED_WORKLET_SHA256` (step 4). The PR's `artifact-identity` report is
   the check that CI built these bytes: its `RELEASE:` lines compare the pin with the module the
   `artifact` job built.

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
     Its `RUSTUP_TOOLCHAIN` must be the rustc release `qualification.yml`'s `artifact` job builds
     with. The release report checks that. Any other difference between the two workflows' build
     environments cannot publish other bytes: `qualify` rebuilds and refuses anything but
     `EXPECTED_WORKLET_SHA256`. So it fails there, safely, and the release PR must first align
     the workflows.
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
