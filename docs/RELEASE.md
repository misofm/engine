# Releasing `@misofm/engine`

Owner decision 5 (`docs/rulings/engine-footprint-2026-09-28.md`, issue #1061): the shipped
AudioWorklet module's fingerprint is checked at release, not on every change. This is what each
kind of change must do from now on.

## What runs when

| change | what CI does with the shipped module |
| --- | --- |
| any PR on the `sdk` or `full` route, and every `main` push on them | `artifact` builds the module once (`scripts/build-web-audioworklet.sh`), with its named twin (below); `sdk`, `artifact-gates` and the two browser legs (Chromium and WebKit) download it and check it against that job's digest, so every artifact gate reads exactly those bytes. `artifact-identity` builds the same commit again from another checkout path and `CARGO_HOME` and fails if the bytes differ, then compares the module with **the digest the base commit's own CI run recorded**, and prints `ARTIFACT CHANGED`, `ARTIFACT UNCHANGED` or `ARTIFACT CANNOT TELL` (with the reason) in its job summary. Nothing is compared with the committed pin and nothing is re-pinned. |
| a `main` push on those routes | also records the module it built: `artifact-record`, a job that runs only on a push to `main`, checks out nothing, and is the workflow's only job with a write permission (so no pull request can forge a record), posts the commit status `audioworklet-sha256` (`<sha256> rustc <toolchain>`, linking the run; the toolchain is the browser artifact's dated nightly channel, below) on the pushed commit. That record is what later changes compare against. |
| a PR on the `evidence` route (documentation only) | no module is built; the verdict's summary says `ARTIFACT UNCHANGED`, because no build reads those paths. |
| a **release change**: a PR, or the `main` push that merges it, that edits the pin file or `npm-publish.yml`'s `PACKAGE_VERSION` or `EXPECTED_WORKLET_SHA256` | all of the above, and `artifact-identity` also requires the pin to be the built digest, `EXPECTED_WORKLET_SHA256` to be the pin, `npm-publish.yml`'s `BROWSER_ARTIFACT_TOOLCHAIN` to be the toolchain the `artifact` job built with, and `hosts/host-web/qualification/results.json` to record a two-browser qualification of the built digest (`wasmSha256`) with a canonical `candidateCommit`. |
| a **browser artifact toolchain change**: a PR, or the `main` push that merges it, that moves `rust-toolchain.toml`'s `[browser-artifact] channel` | all of the first row, and `artifact-identity` also requires `hosts/host-web/qualification/results.json` to record a two-browser qualification of the built digest with a canonical `candidateCommit` (its `TOOLCHAIN:` line). A `rust-toolchain.toml` change routes `full`, so the AArch64 and native parity gates run too. It is not a release: the pin and `npm-publish.yml`'s release identity stay. |
| `npm-publish.yml` (unchanged by #1061) | refuses to start unless `EXPECTED_WORKLET_SHA256` is the pin file; `qualify` rebuilds the module and refuses bytes other than `EXPECTED_WORKLET_SHA256`; `publish` and `verify` send or recheck only the archive `qualify` produced. |

## The browser artifact toolchain

Decision 15, D15-10 (issue #1334): the shipped module alone builds on a pinned, dated nightly with
`-Zbuild-std`, named once, in `rust-toolchain.toml`'s `[browser-artifact] channel` (rustup ignores
that table; every other build stays on the stable `[toolchain]` channel).
`bash scripts/build-web-audioworklet.sh --print-toolchain` prints it, and every job that runs the
builder installs exactly that, with `rust-src`. Bump it only together with a re-recorded
two-browser matrix of the module it builds (step 3 below, in the same PR) and green AArch64 and
native parity gates; `artifact-identity` refuses the bump without the record. A release that also
moves it sets `npm-publish.yml`'s `BROWSER_ARTIFACT_TOOLCHAIN` to the new channel (step 4).

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

## The named twin: symbolicating a production stack trace

Issue #1109: the shipped module carries no wasm `name` section, so a browser prints its frames as
`wasm-function[N]` rather than `host_web::...`. Every digest, pin and record above is the shipped
module's. The same build also produces the **named twin**,
`miso-engine-v1-audio-worklet.simd128.named.wasm`: rustc's output, which is the shipped module plus
that one section and nothing else. It is never published. Keep it for debugging:

- **In CI.** Every `artifact` job uploads it as the workflow artifact `audioworklet-named-<commit>`,
  beside `audioworklet-<commit>`. `artifact-gates` checks it against the job's digest and against
  the shipped module, and the gates that find functions by name read it. A release's merge push to
  `main` uploads the twin of the released bytes. Workflow artifacts expire (90 days by default).
- **At any later time.** The build is reproducible (`artifact-identity` proves it on every change),
  so rebuild the twin from the release commit with the toolchain its `qualification.yml` pins:

  ```sh
  shipped=$(mktemp -d) named=$(mktemp -d)
  bash scripts/build-web-audioworklet.sh --module-only --named-twin "$named" "$shipped"
  # prints the shipped module's digest: it must be the release's pin
  python3 -B scripts/strip-wasm-names.py check \
    "$named/miso-engine-v1-audio-worklet.simd128.named.wasm" \
    "$shipped/miso-engine-v1-audio-worklet.simd128.wasm"
  ```

The check passes only when the shipped module is the twin minus its `name` section, every other
section byte-identical. So function indices are the same in both, and so is every code offset,
because rustc writes the `name` section after the code and data. Frame `wasm-function[N]` is the
twin's function `N`: `wasm-objdump -x -j name <twin> | grep 'func\[N\]'` names it, and a
devtools session that loads the twin instead of the shipped module prints the names itself.

## Between releases

Nothing re-pins the pin (`hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256`),
`results.json` or `hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md` between releases. They describe
the last release, 0.10.3 (#2215): the pin is `npm-publish.yml`'s `EXPECTED_WORKLET_SHA256`, and
`results.json` is that module's two-browser record, one row per browser and leg (`single` and
`worker`).

The browser legs still check every browser row, the Playwright version and the generated matrix
against `results.json` on every PR (`--check-matrix`). Re-record the matrix when Playwright or a
browser floor changes (step 3 below, without the re-pin); that record names the module it ran.

## BLAKE3 release identity

The shipped `miso-engine-v1-blake3.simd128.wasm` has its own release pin,
`hosts/host-web/web/miso-engine-v1-blake3-artifact.sha256`, mirrored by
`npm-publish.yml`'s `EXPECTED_BLAKE3_SHA256`. The identity report always displays its built digest
and pin. Editing either BLAKE3 fingerprint, or `PACKAGE_VERSION`, requires agreement between the
built bytes, pin, npm fingerprint/toolchain, and `results.json.blake3Qualification`. That record
names the candidate commit, SHA256 and successful `simd128` Worker corpus row in both Chromium
and WebKit. A browser toolchain change also requires that BLAKE3 qualification. Ordinary code
changes may differ from the last release's pin, as the worklet may. npm `qualify` checks the
BLAKE3 source pin and built digest before packing; `publish` and `verify` keep using the exact
qualified archive.

The full `--record-matrix` command below also records BLAKE3. To introduce or update only its pin
without changing the worklet release lineage, build the closure, read the BLAKE3 file's SHA256,
update its pin and `EXPECTED_BLAKE3_SHA256`, then run from `hosts/host-web/qualification`:

```sh
npm run qualify -- --artifacts "$artifacts" --browser all --blake3-only \
  --record-blake3 --candidate-commit "$(git rev-parse HEAD)"
```

This updates only `results.json.blake3Qualification`; it leaves the worklet hash, candidate and
matrix intact. The scoped row needs no audio device. It serves the raw host module at
`/web/stem-store/incremental-blake3.js`, uses its default `../../assets/` URL, and refuses
JavaScript fallback before hashing the existing canonical PCM corpus. The server's `/assets/`
route exposes the same shipped closure as `/artifacts/`. No second digest corpus is introduced.

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

3. **Record a fresh two-browser qualification of those bytes.** The supported browsers are
   Chromium and WebKit (Chrome and Safari); Firefox is not qualified (owner ruling 2026-10-25,
   #2092). On Linux a real `AudioContext`
   needs an output device. Start the null PulseAudio sink exactly as `qualification.yml`'s browser
   job does before running, with its socket directory on a short path such as
   `mktemp -d /tmp/qa.XXXXXX`: a UNIX socket path must stay under 108 bytes. From a deep scratch
   path the sink never starts, and a real `AudioContext` stays suspended.

   ```sh
   (cd sdk && npm ci)
   cd hosts/host-web/qualification
   npm ci && npx playwright install chromium webkit
   npm run qualify -- --artifacts "$artifacts" --sdk-root ../../../sdk --browser all --leg all \
     --record-matrix --candidate-commit "$(git rev-parse HEAD)" --self-test-mutations
   ```

   This rewrites `results.json` and `BROWSER_DEPLOYMENT_MATRIX.md`. `--record-matrix` requires
   `--browser all --leg all` and records four rows: each browser on the `single` leg (a page
   without cross-origin isolation) and on the `worker` leg (an isolated page, where the control
   Worker runs the control plane; #1332 S9). `--candidate-commit` names the commit whose tree you
   built.

4. **Set the release identity.**
   - `.github/workflows/npm-publish.yml`: `PACKAGE_VERSION`, the version in the job name and in the
     three other literals that carry it (five in all), `EXPECTED_WORKLET_SHA256: "<digest>"`,
     and `EXPECTED_BLAKE3_SHA256` matching the BLAKE3 pin.
     Its `BROWSER_ARTIFACT_TOOLCHAIN` must be the toolchain `qualification.yml`'s `artifact` job
     builds with (`rust-toolchain.toml`'s `[browser-artifact] channel`); `qualify` refuses any
     other, and the release report checks it. `RUSTUP_TOOLCHAIN` is the stable release the SDK
     gates' other cargo runs use. Any other difference between the two workflows' build
     environments cannot publish other bytes: `qualify` rebuilds and refuses anything but
     `EXPECTED_WORKLET_SHA256`. So it fails there, safely, and the release PR must first align
     the workflows.
   - `scripts/test-npm-publish-modes.py`: `VERSION`, the version literals it normalizes, and the
     accepted `release_pin` and its "wrong release pin" mutation (both `<digest>`). Run it.
   - `sdk/package.json`, `sdk/package-lock.json`, the install line and release-record link in
     `sdk/README.md`, and the release's issue spec.

5. **Open the PR.** Its `artifact-identity` summary must show a `RELEASE:` line with every check
   passing, and both browser legs must pass on the built bytes. The merge push to `main` runs
   the same release checks on the merged tree; if another merge changed the module in between, they
   fail there, and the release needs a new release PR.

6. **Publish** from `main` with `npm-publish.yml`: `qualify` with `expected_sha` set to the merge
   commit, then `publish` with that run's ID as `qualification_run_id` (`verify` only to recheck
   an ambiguous publish). The app's provenance file takes the pin from this release.
