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
- `qualification.yml` verdict table matches the remaining jobs (its self-test or a dry router run).
- `npm run qualify -- --check-matrix --self-test-mutations` passes for chromium and webkit; a request for firefox is refused with a clear error.
- `lint-checks.sh --self-tests` passes.
