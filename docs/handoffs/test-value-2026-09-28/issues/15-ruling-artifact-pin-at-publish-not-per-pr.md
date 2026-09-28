# Ruling: enforce the AudioWorklet artifact pin at publish, not on every PR

Draft, not a GitHub issue. From the 2026-09-28 test-value audit
([`../TEST-VALUE-AUDIT.md`](../TEST-VALUE-AUDIT.md) §4.3 and §12 R3). Base `a9414c0c`. Paths starting
`../` are relative to the audit's handoff folder. **Needs owner
ruling R3.** The app repository's provenance file consumes the pin, so its purpose is release
integrity, and that is the owner's call.

## Problem

Every PR's `artifact` job refuses to build when the shipped module's sha256 differs from
`hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256`
(`scripts/build-web-audioworklet.sh:95-99`). Any code change moves the digest, even a doc comment,
because panic line numbers are embedded: `52d59c7c` had to re-pin after a change with no rendered
difference.

Three more records follow the pin:
- `hosts/host-web/qualification/results.json`'s `wasmSha256` and `candidateCommit`, which
  `--check-matrix` requires to match (`hosts/host-web/qualification/run.mjs:123-124`);
- the generated `BROWSER_DEPLOYMENT_MATRIX.md`;
- a literal copy, `REAL_WASM_SHA256`, in `scripts/test-web-audioworklet.mjs:153`. It is read only
  under `--real-wasm-receiver`, which no workflow or script runs, yet it moved in 11 commits.

From 2026-09-01 to 09-28 (`../data/ci-red-jobs.tsv`):
- **41 red jobs**: 20 on the sha256 pin and 21 on browser lineage;
- **98 distinct pin values**;
- **50-125 pin-only commits since 2026-08-28**, depending on whether specs and docs count;
- **no pin red exposed an unintended change**;
- in the one real kernel defect of the window (#920), the pin had *already* been updated to the
  defective artifact. The vector-kernel ratchet caught it, not the pin.

What the pin *does* prove: the build is reproducible across checkouts, and `npm-publish.yml`
publishes exactly the reviewed bytes (`npm-publish.yml:69-76`, `:160`).

## Outcome (if R3 is "publish-time")

- **PR runs** build the artifact, record its digest in the job summary, and upload it. They do not
  compare it to a committed pin.
- **Reproducibility becomes a same-run check.** A second job builds the artifact again, in parallel,
  under a different `CARGO_HOME` and checkout path, and `artifact-gates` requires identical bytes.
  This proves checkout-independence on every PR with no commit. It does not prove agreement with a
  build made on another machine or toolchain image; the committed pin, checked at publish, still
  covers that.
- **`npm-publish.yml` keeps its pin assertions unchanged.** The committed pin file is updated in the
  release PR only.
- **The downstream "Verify the downloaded artifact against its source pin" steps**
  (`qualification.yml:162`, `:203`, `:258`) compare against the digest the `artifact` job publishes
  as a job output, instead of the committed file.
- **The browser jobs keep their digest-parity gates** (native digest = browser digest). They drop the
  `results.json` lineage check. `BROWSER_DEPLOYMENT_MATRIX.md` is regenerated when Playwright or the
  browsers are bumped, not per artifact.
- **`REAL_WASM_SHA256` and the `--real-wasm-receiver` mode are deleted,** or the mode is wired into
  `artifact-gates`, reading the pin file.

## Scope

Authorized paths:
- `scripts/build-web-audioworklet.sh`;
- `.github/workflows/qualification.yml` (`artifact`, `sdk`, `artifact-gates`, `browser`);
- `hosts/host-web/qualification/run.mjs` (the lineage gate);
- `scripts/test-web-audioworklet.mjs`;
- `docs/` for the release procedure;
- this issue's spec.

`npm-publish.yml` is unchanged.

## Gates

1. **Reproducibility is still proven, and better.** The two-build check fails when a scratch branch
   embeds the absolute checkout path into the module, for example via `file!()` in a panic message
   with path remapping disabled. The failure appears in the PR that introduces it, with no pin to
   update.
2. **Publish integrity is unchanged.** `npm-publish.yml`'s `EXPECTED_WORKLET_SHA256` assertion still
   fails a publish whose built bytes differ from the pin file. Dry-run on a scratch branch.
3. **The browser parity is unchanged.** A one-bit PCM change in the browser render still fails each
   browser leg's native-digest gate.
4. **Historical catches kept.** Re-apply #920's scalarised `route_reduce<f32x4>` shape (revert
   `f7524db7`'s fix, or apply `graph/tests/MUTATIONS.md` 926-7). `artifact-gates`' vector ratchet
   still goes red.
5. **Churn.** Over the following 30 days, pin-only commits drop to release PRs only.

## Saving and risk

- **Saving:** about 41 red jobs a month and 50-125 pin-only commits. CI time rises by one parallel
  artifact build of about 100 s of runner time, off the critical path if it runs as its own job.
- **Risk:** a non-reproducible build is found in the same run instead of by pin drift, which is an
  improvement. The app's provenance file still receives the pin at release.

## Amendments (Sol verification, 2026-09-28)

See [`../VERIFY-TEST-VALUE.md`](../VERIFY-TEST-VALUE.md). **These amendments supersede the body wherever they conflict.**

1. **Adopt R3 only with a replacement for what the per-PR pin enforces (finding F3).**
   - **What the pin enforces today.** At least 16 filed specs (#1017-#1041) carry a
     "Shipped artifact" gate, "unchanged" or "re-pin with the reason". The pin makes those gates
     mechanical.
   - **Its other role.** It is also the only link between the bytes `qualification.yml` gates and
     the bytes `npm-publish.yml` publishes. `npm-publish.yml` runs none of the callgraph ratchet, the
     V8 spill gate, the atomics check or the browser legs.
   - **Replacement 1.** Every PR prints `ARTIFACT CHANGED|UNCHANGED` against the merge base's digest
     in the `artifact` job summary. Issue gates cite that line.
   - **Replacement 2.** The committed-pin comparison still runs on any PR that edits the pin file,
     which is the release PR. So the published bytes are the bytes that passed the gates.
2. **Scope additions:**
   - `scripts/check-ci-path-routing.py` pins the step name "Verify the downloaded artifact against
     its source pin" and its order before the V8 spill gate (`ARTIFACT_PIN_STEP`, `:334-353`);
   - `scripts/test-ci-path-routing.py`;
   - `scripts/test-sdk-artifact-builder-output-contract.sh`;
   - the V8 benchmark's `module_matches_pin` provenance (`scripts/web-mixing-automation-benchmark.mjs`,
     `run-web-mixing-automation-benchmark.sh:92`). Footprint R9 keeps that benchmark as the
     benchmark.
3. **New gate.** In a scratch branch, a change that alters the shipped bytes while claiming
   "no artifact change" prints `CHANGED` in its run. A release-PR pin that differs from the built
   bytes fails that PR, not only `npm-publish`.
4. **Confirmed.** 98 distinct pin values and 97 pin commits on `main` since 2026-09-01.
