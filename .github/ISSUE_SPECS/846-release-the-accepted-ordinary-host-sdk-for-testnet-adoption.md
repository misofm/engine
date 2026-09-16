# Release the accepted ordinary-host SDK for testnet adoption

## Product outcome

Publish one immutable `@misofm/engine` release from the accepted current ordinary-host closure so downstream consumers can adopt the completed AudioWorklet boot-input ownership contract from #844 together with the coherent protected-observation ABI/native changes already merged and qualified on main. This release does not enable combined protected/ordinary observation preparation and makes no protected-EQ coexistence claim; #835 remains the policy boundary and #824 remains separate.

Registry SDK 0.3.0 predates #844 and the accepted observation closure. Current main has exact-main qualification and a source-derived worklet pin, but the npm workflow still names immutable 0.3.0 and its old Wasm digest. Shipping the host alone on the old Wasm/ABI is forbidden.

## Bounded release preparation

Freeze unused SDK 0.4.0 after live registry absence proof. One implementation tranche may change only this spec, `sdk/package.json`, `sdk/package-lock.json`, `.github/workflows/npm-publish.yml`, matching identity-only fixtures in `scripts/test-npm-publish-modes.py`, and concise `sdk/README.md` compatibility notes. Update all release version/job/subject guards and the expected Worklet digest to the accepted source pin `b47d05f053dca81687f0065306fb97159f892277e9543326cea7e21544186b61`.

No Rust, DSP, ABI, host/runtime, generated layout, generic publisher redesign, benchmark, or listening work. Preserve V1 wire identities, exact-main SHA enforcement, qualify→publish→verify archive ownership, OIDC-only publication, immutable-version refusal, and provenance verification.

## Objective gates

Install locked SDK dependencies, then run the existing publish-mode/trust, generated/deletion/type/headless/package, AudioWorklet, declaration mirror, and diff gates against the accepted seven-file CI artifact closure. Retain a candidate archive and prove the packed host with the existing `--boot-data-document` and `--boot-data-nested` selectors while its helper and ABI siblings remain colocated. Compare packed host/ABI/declaration/Wasm identities with the accepted closure. A fresh independent Astra MEDIUM review must PASS before delivery.

After required PR and exact-main qualification pass, hold the merged release SHA unchanged through npm qualify, publish, and any verify-only recovery. Qualify once, publish the exact immutable archive once, then independently verify registry integrity, public imports/types/CLI, packaged #844 host behavior, one trusted SLSA subject/source/workflow binding, and actual Wasm/host hashes. Do not republish after an ambiguous response. Record version, source SHA, archive SHA256/SHA512/integrity and workflow IDs. Only verified registry delivery closes this issue and advances the adapter release.

## Workflow

Astra XHIGH scope and fresh adversarial plan review approved the corrected three-repository order: engine release, adapter exact-dependency release, app adoption/testnet proof. A fresh Luna MAX owns this single release-preparation tranche; if it cannot satisfy the tranche within two rounds, escalate once to Sol HIGH, then Astra XHIGH. Root owns checkpoints, remote delivery, immutable publication and issue synchronization.

## Evidence

Starting source is clean synchronized main `85bb5997ba813d9b87007e801893d4a655138197`. Exact-main qualification run 35075507202 passed. Its retained seven-file AudioWorklet artifact contains Wasm SHA256 `b47d05f053dca81687f0065306fb97159f892277e9543326cea7e21544186b61` and host SHA256 `48a772b822b4e6b00ba9bad8d667490365361e9c5d59b7da24446eae10c54615`; the host is byte-identical to current source and contains #844 ownership preparation. Public SDK 0.3.0 is source `51e03cfdde61802fc8456872a6637a265fae5979`, uses the earlier Wasm, and its packaged host lacks #844. Live registry lookup returns parsed E404 for SDK 0.4.0. No implementation or publication has occurred.

Implementation preparation (Luna round 1): froze the SDK package and lockfile at 0.4.0; updated
the npm workflow's five release identity/subject guards and expected AudioWorklet digest to
`b47d05f053dca81687f0065306fb97159f892277e9543326cea7e21544186b61`; updated only matching
release fixtures and the SDK compatibility note. The workflow's historical normalized baseline,
mode matrix, exact archive ownership, OIDC-only publication, immutable-version refusal, and
provenance checks remain unchanged.

Luna round 1 gates (2026-09-16, uncommitted preparation worktree): `npm ci --ignore-scripts` in
`sdk/` PASS (5 packages added, 6 audited, 0 vulnerabilities); `python3 -B
scripts/test-npm-publish-modes.py` PASS (`npm publish mode reachability and trust gates: ok`);
`node scripts/test-parse-npm-trust-list.mjs` PASS (`npm trust list parser fixtures and mutations:
ok`); `bash scripts/check-sdk-generated.sh` PASS (assets, generated modules, and generated surface
are current); `python3 -B scripts/check-sdk-deletions.py` PASS (74 files carry none of the retired
spellings); `bash scripts/check-sdk-types.sh` PASS (including the shipped-host mirror pin); and
`git diff --check` PASS. Headless/package/AudioWorklet artifact gates were not started because the
retained Worklet artifact is outside this bounded preparation tranche; no source or generated
artifact was changed.
