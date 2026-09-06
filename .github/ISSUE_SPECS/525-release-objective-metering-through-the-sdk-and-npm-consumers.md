# Release objective metering through the SDK and npm consumers

## Authorized outcome

Merge reviewed metering PR #521 while preserving newer main-branch prepared-slot activity (#523). Build and qualify the combined Linux/amd64 WASM, publish @misofm/engine 0.2.1 using the existing exact-artifact trusted-publisher workflow, then publish adapter 0.3.1 with its exact engine dependency updated to 0.2.1. Only after registry verification, update misofm/app and misofm/website to these npm versions.

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
