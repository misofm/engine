# Ruling: enforce the AudioWorklet artifact pin at publish, not on every PR

Draft, not a GitHub issue. From the 2026-09-28 test-value audit
([`../TEST-VALUE-AUDIT.md`](../TEST-VALUE-AUDIT.md) §4.3 and §12 R3). Base `a9414c0c`. **Needs owner
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
  `--check-matrix` requires to match (`hosts/host-web/qualification/run.mjs:121-124`);
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
- **Reproducibility becomes a same-run check.** The `artifact` job builds twice, under two different
  `CARGO_HOME`s and checkout paths, and requires identical bytes. This is a stronger claim than the
  pin, and it needs no commit.
- **`npm-publish.yml` keeps its pin assertions unchanged.** The committed pin file is updated in the
  release PR only.
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

- **Saving:** ≈ 0 s of CI. It removes about 41 red jobs a month and 50-125 pin-only commits.
- **Risk:** a non-reproducible build is found in the same run instead of by pin drift, which is an
  improvement. The app's provenance file still receives the pin at release.
