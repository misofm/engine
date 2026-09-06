# Release objective metering through the SDK and npm consumers

## Authorized outcome

Merge reviewed metering PR #521 while preserving newer main-branch prepared-slot activity (#523). Build and qualify the combined Linux/amd64 WASM, publish @misofm/engine 0.2.1 using the existing exact-artifact trusted-publisher workflow, then publish adapter with its exact engine dependency updated to 0.2.1 (final consumer target 0.3.2 after the provenance correction below). Only after registry verification, update misofm/app and misofm/website to these npm versions.

## Bounded implementation

- Resolve generated artifact pin/browser evidence merge conflicts by qualifying a fresh combined artifact; preserve both production changes. Retain dirty original checkouts and work in isolated branches/worktrees.
- Update SDK package/lock version and the existing npm workflow's release identity, qualified job identity and WASM digest. Preserve all ancestry, provenance, immutable-version, qualified-run and no-retry checks. No new release framework.
- Merge the checked PR, qualify on its exact main SHA, then publish exactly that qualifying archive and verify registry version, integrity and embedded WASM identity. Do not rebuild a different archive for publication.
- Adapter release changes only its package/lock dependency and existing workflow identity unless a concrete compatibility failure requires a minimal fix.
- Downstream changes use registry packages, remove app's live vendored dependency overrides/check assumptions as needed, and preserve audio/session behavior. No meter decay tuning, new UI, progressive playback, stem generation or unrelated cleanup.

## Acceptance

Dedicated Astra review of merge/release changes; combined engine focused tests and unchanged artifact/browser/resource checks, followed by green PR CI. SDK package/headless/typed/generated checks and trusted release verification must pass before npm consumer edits. Adapter existing package/type/test gates must pass and its registry artifact must depend on engine 0.2.1. App runs its required lint/typecheck/test/build checks; website production build and mixer asset integration checks pass. Verify consumers bundle the published engine artifact and keep one SDK dependency resolution. Preserve and report baseline-only failures separately. Push reviewable changes and synchronize GitHub evidence; merge/dependency delivery is authorized by the user's request.

## Limits

The metering timing fixture repair remains #522 and is not part of this release. No measured CPU speedup is claimed. Display ballistics remain downstream follow-up scope, not part of dependency adoption.

## Combined engine release checkpoint

Sol medium approved the bounded release brief. Merge checkpoint `5d1f4907` preserves main `70ce3d7b` prepared-slot activity and all reviewed metering source; only generated artifact records conflicted. SDK/workflow identities are updated to 0.2.1. Candidate `fb801b8d2b71a1b20833e2d8102345fd76e84c85` qualifies canonical Linux WASM `54dcf7dd5f6199cf3ceeab77afefe09067e18b730c9e0a6ef9df73fbfd3afc69`, matching the source pin and trusted publishing workflow.

Astra medium independently reviewed the source preservation, release identity/guards and actual artifact. Local qualification PASS: unchanged static/object/budget gates, Chromium/Firefox/WebKit with mutation checks, resource oracle with 26 red mutations, SDK0.2.1 tarball smoke, 167 headless tests passed/one skipped/zero failed, generated surface and TypeScript mirror checks. No production source changed after the candidate. Review: `docs/evidence/metering-525/review.md`. Combined PR CI passed (run 34034182930) and PR #521 merged as `be781895decc72328f727dcd816b8b40a2ab6051`; no timing speedup is claimed.

## Published releases

- Engine 0.2.1: qualification run [34034567543](https://github.com/misofm/engine/actions/runs/34034567543) and trusted publish/registry verification run [34034773546](https://github.com/misofm/engine/actions/runs/34034773546) passed. Independently downloaded registry archive SHA256 `d0bbe8b7a7aa4981706975217aea930b75052ce26fe2fae85f08f232ff7c56ea`; embedded WASM matches `54dcf7dd5f6199cf3ceeab77afefe09067e18b730c9e0a6ef9df73fbfd3afc69`.
- Adapter 0.3.1: [PR #41](https://github.com/misofm/engine-web-adapter/pull/41) merged as `37284ed1cc2a9da034fad32abb0ccaf1dc5b8264`. Exact engine dependency 0.2.1; runtime unchanged. Two synthetic meter fixtures were updated to include the new required metadata. All 158 tests and full package checks passed; Astra medium review PASS. Trusted publish [34035276892](https://github.com/misofm/engine-web-adapter/actions/runs/34035276892) passed registry, fresh consumer imports and provenance verification.
- Only after both releases passed registry verification did app and website npm migration begin. Consumer implementation, checks, review and remote merges are recorded below.

## Producer provenance correction

App migration found that adapter 0.3.1 exported stale `ADAPTER_PROVENANCE.engine` metadata despite its correct package dependency. The earlier package tests and Astra review missed this mismatch. [Adapter PR #42](https://github.com/misofm/engine-web-adapter/pull/42), merged as `20ed3808756cb152e67ab2385aab5b5a5961077e`, corrects the exported package/source/archive metadata and adds source and built-package checks against the declared exact dependency. Full checks passed (158 tests) and Astra medium independently verified the correction. Adapter 0.3.2 is the final consumer target; trusted publish [34035710691](https://github.com/misofm/engine-web-adapter/actions/runs/34035710691) passed registry/fresh-consumer/provenance verification. Registry gitHead matches the merge SHA and the exact dependency is engine 0.2.1. Audio runtime code is unchanged.

## Website adoption

[Website PR #3](https://github.com/misofm/website/pull/3) merged as `0df6dd5ecaffd1b6ef08e19b5aaf8309252e703d`. Only `package.json` and `bun.lock` change, to exact npm engine 0.2.1 / adapter 0.3.2. Production build, single-SDK resolution and corrected exported adapter provenance passed. Real Chromium production-preview smoke independently hashed the loaded WASM to `54dcf7dd5f6199cf3ceeab77afefe09067e18b730c9e0a6ef9df73fbfd3afc69`, observed nonzero track/master meter messages and visible meters, exercised play/pause/restart and fader interaction, and confirmed pause clears visible meters with zero page errors. This is integration evidence, not a fader-DSP numerical or performance claim. Dedicated Astra medium review PASS. App adoption is recorded below.

## App adoption and completion

[App PR #111](https://github.com/misofm/app/pull/111) merged as `2661183542cff147d068a7d07000cc60790262cf`. Exact npm engine 0.2.1 / adapter 0.3.2 replace the vendored tarballs and file override. Registry identities/SHA-512 lock integrity, exported adapter provenance, a single SDK resolution, SDK asset-manifest hashes, generated catalog consistency and deployed runtime asset attestation remain verified. The existing launcher's enginectl lookup changes only to follow the installed package after archive removal. Audio and display behavior are unchanged; fake meter fixtures now supply required metadata.

Sol medium implemented; dedicated Astra medium final review PASS for checkpoint `4d818025`. Lint, formatting, typecheck, asset/export checks, focused attestation/catalog tests and production build (PWA and bundle budgets) passed. Local full unit run had 986 passes and four sandbox loopback-bind failures; both affected files passed all five tests with authorized loopback access. The remote [Checks run 34036453506](https://github.com/misofm/app/actions/runs/34036453506) passed the full required suite; React Doctor CI passed, and its local unchanged-code warning was not treated as a regression.

Real Chromium app integration passed: clean runtime attestation; a deliberately corrupted decoder produces `stale-artifact` identifying exactly the decoder; nonzero master meters arrive during playback; pause returns to ready; no page errors. Website evidence above separately verifies the exact published WASM response hash. Neither integration introduces meter ballistics or performance claims. All requested repositories are merged and consumers use verified npm packages. Original dirty checkouts remain preserved.
