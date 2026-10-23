# License the engine under FSL-1.1-ALv2

Owner request (2026-10-23): "Please change the engine to FSL license." Owner answers: the variant is FSL-1.1-ALv2 (Apache 2.0 future license), and the licensor is Miso Labs, Inc.

## Problem

Issue #314 licensed the original project work under Apache-2.0. The workspace policy (`scripts/check-workspace-policy.sh`) pins the LICENSE text by digest, and checks that the Cargo workspace, `fuzz/`, every `package.json` and every `package-lock.json` root package say `Apache-2.0`. The owner now licenses the engine under the Functional Source License, Version 1.1, ALv2 Future License.

## Decisions

- D1. LICENSE is the official template `FSL-1.1-ALv2.template.md` from https://github.com/getsentry/fsl.software (main), copied byte for byte, with only the notice line filled in: `Copyright 2026 Miso Labs, Inc.` (the first commit is 2026-08-20).
- D2. The SPDX identifier is `FSL-1.1-ALv2` in every Cargo and npm manifest that names the project's own license.
- D3. Material that is not original project work keeps its own license: the vendored libm material (`crates/math/LICENSE-libm.txt`) and every external dependency. Their notices do not change.
- D4. Copies that were published under Apache-2.0 before this change stay available under Apache-2.0 to those who received them. This change licenses the work from this commit on. NOTICE keeps its lineage lines for the adapted misofm sources (misofm/engine-web-adapter, misofm/app). Root assumes that Miso Labs, Inc. owns them; the owner must confirm.
- D5. The policy keeps its shape: it pins the new LICENSE digest and checks the new identifier. No new check.

## Smallest closable slice

- Replace `LICENSE` (D1). Add `Copyright 2026 Miso Labs, Inc.` to NOTICE, and change NOTICE's and `THIRD_PARTY_LICENSES.md`'s statement of the project license to FSL-1.1-ALv2 (D3, D4).
- Set `license = "FSL-1.1-ALv2"` in `Cargo.toml` `[workspace.package]` and in `fuzz/Cargo.toml`; every crate keeps `license.workspace = true`.
- Set `"license": "FSL-1.1-ALv2"` in `sdk/package.json`, `hosts/host-web/qualification/package.json` and the root package entry of both `package-lock.json` files. Do not change dependency entries.
- Update the README license line in `sdk/README.md`.
- Update `scripts/check-workspace-policy.sh` (digest, identifier, messages), `scripts/test-workspace-policy.sh` (fixtures and mutants) and `scripts/check-release-shape.py` (its valid manifest fixture).

## Authorized paths

`LICENSE`, `NOTICE`, `THIRD_PARTY_LICENSES.md`, `Cargo.toml`, `fuzz/Cargo.toml`, `sdk/package.json`, `sdk/package-lock.json`, `sdk/README.md`, `hosts/host-web/qualification/package.json`, `hosts/host-web/qualification/package-lock.json`, `scripts/check-workspace-policy.sh`, `scripts/test-workspace-policy.sh`, `scripts/check-release-shape.py`, this spec.

## Objective gates

1. `diff` between LICENSE and the official template with the notice line filled in shows no difference.
2. `bash scripts/check-workspace-policy.sh` and `bash scripts/test-workspace-policy.sh` pass; the old Apache text now fails the digest check.
3. `rg -n 'Apache-2\.0' --glob '!crates/math/**' --glob '!**/node_modules/**' --glob '!target/**'` lists only dependency licenses, rulings, specs, research and history, never the project's own license.
4. `cargo metadata --format-version 1 --no-deps` reports `FSL-1.1-ALv2` for every workspace package.
5. `python3 scripts/check-release-shape.py`'s self-test passes, if it has one.

## Test value

No new test. The existing policy mutants (`mutate_workspace_license`, `mutate_npm_license` and others) move to the new identifier, and each still turns red on an Apache or MIT manifest.

## Dependencies

None. #314's spec is history.
