# Support Chrome and Safari only: remove Firefox from browser qualification

Filed by root under the owner ruling of 2026-10-25: "I don't think we need to benchmark for firefox as well. I think we should just support chrome and safari. Barely anyone uses firefox."

## Product outcome
The browser host's supported engines are Chromium (Chrome) and WebKit (Safari). Firefox is not a supported target: no CI qualification leg, no timing gate, no benchmark row and no release claim names it.

## Smallest slice
- Remove the Firefox leg from `.github/workflows/qualification.yml` and keep the verdict job's static expectation table exact (a required workflow must not leave a context pending).
- Remove Firefox from the qualification runner's browser list and matrix generator (`hosts/host-web/qualification/{run.mjs,generate-matrix.mjs,results.json}` and the cases that name it), the stem-store browser evals, and `scripts/ci-telemetry.py`.
- Restate `docs/TARGET_MATRIX.md`, `docs/RELEASE.md`, `docs/STEM_STORE_V1.md` and `docs/C_ABI_V1_QUALIFICATION.md` for two browser engines. Historical handoff and verdict records stay as they are.
- AGENTS.md's "three-browser matrix" wording changes only with the owner's approval (agents do not edit AGENTS.md).

## Gates
- `qualification.yml` verdict table matches the remaining jobs: the matrix job is one `browser` job in the table, and the router has no browser output, so neither changes. The PR's own `qualification` verdict is SUCCESS with two green `browser` legs.
- `npm run qualify -- --browser <b> --sdk-root sdk --check-matrix --self-test-mutations` passes for chromium and for webkit (no `sdk/dist`); `--browser firefox` is refused with the existing `unknown browser: firefox` error.
- `node hosts/host-web/qualification/generate-matrix.mjs --check`, `python3 scripts/ci-telemetry.py --self-test`, `python3 scripts/web-audioworklet-identity.py --self-test` and `node scripts/check-stem-store-v1.mjs` pass.
- The CI lint job's policy scripts pass with their self-tests (the old `lint-checks.sh --self-tests` line named no script in the repository; root ruling R4, 2026-10-25).
- No new test: the refusal is the existing `unknown browser:` throw.

## Decision record
- D1 (root R1): the Firefox row of `results.json` is deleted by hand and `BROWSER_DEPLOYMENT_MATRIX.md` is regenerated with `generate-matrix.mjs`; no `--record-matrix`, so `candidateCommit` and `wasmSha256` stay.
- D2 (root R2): code that served Firefox is feature-detected and engine-neutral (`detectSharedReadOnlyMode`, which WebKit still needs; the suspend-free offline renders; the pending `suspend()`), so it stays; only its comments change.
- D3 (root R3): the stem-store browser evals default to chromium and webkit; a firefox leg is refused as an unknown leg.
- D4: `docs/C_ABI_V1_QUALIFICATION.md` keeps its dated Firefox line (a record); `MUTATIONS.md` keeps its recorded Firefox rows and adds one note. Historical handoffs and verdicts stay.
- D5: AGENTS.md's "re-recorded three-browser matrix" sentence changes only with the owner's approval; root takes the wording to the owner.
- D6 (root, after review of attempt 1): `rust-toolchain.toml`'s live bump rule comment says "a re-recorded browser matrix (Chromium and WebKit)".

## Evidence (attempt 1, corrected after review)
- `npm run qualify -- --browser chromium|webkit --sdk-root sdk --check-matrix --self-test-mutations` on a fresh build, no `sdk/dist`: both "all qualification gates passed" (151.0.7922.34; 26.5). `--browser firefox`: `Error: unknown browser: firefox`, exit 1.
- The first record of `web-audioworklet-identity.py --self-test` was wrong: it read the exit code of a pipe, and the self-test failed (`release: 'records the three' missing`). The review found it; the release case now expects "records the two", and the self-test exits 0.
- `generate-matrix.mjs --check`, `ci-telemetry.py --self-test`, `check-stem-store-v1.mjs` and the lint job's policy scripts with self-tests: pass.
