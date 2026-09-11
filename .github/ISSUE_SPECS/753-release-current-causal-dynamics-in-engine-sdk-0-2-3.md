# Release current causal dynamics in @misofm/engine 0.2.3

## Attempt 1 recoverable implementation checkpoint

Terra updated only the SDK package/lock version, existing trusted-publisher workflow identities and current worklet pin, and the packed public-consumer smoke. The new regression compares complete first quanta from empty-rack and bypassed-compressor sessions at48kHz/128 frames with asymmetric nonzero PCM; all four planes begin at sample zero and both outputs match without trimming or shifts. Packed compressor parameter IDs are exactly1 through7 with no removed lookahead parameter.

Static identity checks pass. One fresh Linux worklet build produced the current accepted SHA256 `0c633b091457c4ab9ec17f08327802f423874124182bc794a7988ed1c536fa73`. Existing sdk-package.sh check passes for the resulting0.2.3 archive. Evidence is retained under `/data/sparse-pcm-launch/tooling/engine-753/`, including `worklet-build.log`, `worklet-artifact-dir.txt` and `sdk-package-check.log`. This checkpoint is not independent acceptance or publication: remaining generated/deletion/types/headless/browser qualification, Sol adversarial verdict, required remote qualification and exact qualified-archive registry/provenance verification remain mandatory. No Rust DSP changed.

## Root note

App issue #187 proved that registry `@misofm/engine@0.2.2` delays a bypassed compressor by 20 ms (960 samples at 48 kHz; 882 at 44.1 kHz), even with lookahead set to zero. Engine issue #737 already fixed that defect and passed review; no new compressor Rust work belongs here. Publish one SDK built from a fresh synchronized current `main`, so the app can adopt the corrected public artifact in a separate bounded slice.

## Authority, baseline, and smallest closable slice

The user authorized the package work needed for sparse-stem/app delivery. This issue owns only the engine SDK 0.2.3 release: prepare the existing package and trusted-publisher workflow for 0.2.3, qualify the artifact built from the exact release commit, publish that same tarball once, verify registry identity/provenance and close the synchronized issue. Downstream adapter/app dependency updates and the remaining app #187 cancellation correction are separate issues.

Start implementation in an isolated worktree/branch from a newly fetched, clean, fast-forwarded `origin/main`. The read-only brief was taken from clean synchronized `main` `77c430a0046bd4fae72a6b7cc14284e3559abd76`; treat that SHA as evidence, not a release SHA. Main already contains accepted compressor source `004f379438f383df9195e97358c861e4ecafb9c1` through merge `8b1f0cb3108683ce67344bc61cec72adbcf5ffbd`, plus subsequently accepted causal gate, causal multiband, host, SDK, qualification, and evidence changes. The old installed/released worklet is `5695fbc4d72fae4a78b5acd1cf8970c489163703a11ac5351974ce05a90b1574`; #737's accepted intermediate artifact was `5d9e9ad045ee502eb34fe50fd55faec1aad582d650b6297d11c130170a69af6d`. Neither is eligible for 0.2.3 because neither represents the complete current source tree. At the brief baseline, the current checked-in artifact pin is `0c633b091457c4ab9ec17f08327802f423874124182bc794a7988ed1c536fa73`; rebuild and prove it from the final release source rather than assuming it remains current.

This issue may change only the numbered issue record, `sdk/package.json`, the root identities in `sdk/package-lock.json`, and `.github/workflows/npm-publish.yml`, unless a failing required gate demonstrates a minimal directly related generated/package-test correction. No new release workflow, publishing framework, Rust/DSP implementation, benchmark framework, website work, adapter/app dependency edit, private artist fixture, or full DSP requalification is authorized.

Maximum two coherent implementation attempts, each followed by one Sol adversarial verdict. A failed second attempt stops for rescope; gates may not be weakened. Terra implements attempt 1. Root owns the worktree, exact-path checkpoint commit, push/PR/merge, workflow dispatches, npm publication, GitHub evidence, closure, and clean worktree removal.

## Public delta that 0.2.3 must carry

Relative to the 0.2.2 preparation commit `cf7e695b29043abcae9f92d5172b719e33764f03`, current main changes shipped binary behavior and public generated metadata:

- `miso.compressor` is causal and reports zero prepared latency in engine contracts, including bypass. Parameter ID 8/name `lookahead` was removed and must remain absent; IDs 1–7 retain their meanings. This is the change that unblocks the app's unshifted bypass timing gate.
- `miso.gate-expander` is causal with its fixed delay removed. Parameter ID 8/name `lookahead` was removed; IDs 1–7 remain.
- `miso.multiband-compressor` retains crossover latency but removes the extra lookahead delay. Parameter ID 2/name `lookahead` was removed and is not reused; IDs 1 and 3–12 remain.
- `sdk/assets/miso-engine-v1-parameter-metadata.json` and `sdk/src/generated/catalog.ts` already encode those removals. The metadata has no generic effect-latency field, so release evidence must prove latency with real rendering and must not claim that the old or new catalog alone reports it.
- Current SDK browser behavior also rejects `engine.console()` with `MisoUsageError` when boot policy has no positive `console.commandQueueRecords`; this accepted current-main API behavior and its package smoke coverage travel with 0.2.3. Do not selectively combine #737's older binary with newer SDK sources.

Old documents or app maps that explicitly address the removed parameter IDs are incompatible and must receive the existing typed rejection. The later app adoption must regenerate its catalog-derived keys/defaults and remove compressor lookahead use; that downstream work is not performed here.

## Exact release identity edits

Set `sdk/package.json` version to `0.2.3`. Set both the top-level version and `packages[""]` version in `sdk/package-lock.json` to `0.2.3`; keep package name `@misofm/engine` and dependency versions unchanged.

In `.github/workflows/npm-publish.yml`, update every release identity consistently:

- `PACKAGE_VERSION`, job display name, successful qualification-job selector, packed-package identity assertion, and SLSA PURL subject must all name `0.2.3`.
- `EXPECTED_WORKLET_SHA256` must equal the exact lowercase digest in `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256` for the final release commit. The qualify job must rebuild the worklet and compare its bytes to that digest.
- Preserve all existing exact-SHA, main-only, accepted ancestry/rejected-commit, immutable-version refusal, qualification-run binding, one-archive, checksum, no-publish-retry, public/latest convergence, fresh registry consumer, OIDC-only trusted publisher, and SLSA provenance checks. Do not loosen or delete a guard to make the release pass.

Before implementation and again before publication, query the public registry and prove `0.2.3` is absent while `0.2.2` remains the latest known release. An ambiguous registry error is a stop, not absence evidence. npm versions are immutable.

## Objective gates before merge

1. Static identity proof finds no release-semantic `0.2.2` or old worklet digest in the three owned package/workflow files, while historical issue evidence remains untouched. Parse package and lock files and assert the exact names/versions above. Parse or exercise the workflow assertions so job-name and PURL literals cannot drift from `PACKAGE_VERSION` unnoticed.
2. From the frozen candidate source, run the existing supported Linux worklet build once and prove its SHA-256 equals both the source-of-truth pin and `EXPECTED_WORKLET_SHA256`. Run `scripts/check-sdk-generated.sh`, SDK deletion/type/headless gates, and `scripts/sdk-package.sh check` (or the equivalent exact sequence already used by `npm-publish.yml`). Record nonzero test counts.
3. Add or use a minimal public headless regression on the packed 0.2.3 consumer: a nonzero unequal L/R source, empty-rack baseline, and a compressor-only variant with bypass true and no lookahead parameter at 48 kHz/128 frames. Both first nonzero indices must be sample 0 and PCM must compare unshifted; no trimming, delay compensation, shifted-array comparison, or elapsed render counter may satisfy this gate. Also prove the packed catalog has compressor IDs 1–7 and no ID/name `8`/`lookahead`. Prefer extending the existing packed consumer smoke only if current tests do not already prove these facts.
4. Run the existing opt-in genuine packed Vite/Chromium consumer gate using the final packed tarball and available pinned browser tools. It must import public entries, load the bundled worklet whose response bytes match the candidate pin, boot and render without page/request failures. Run the existing supported browser/worklet qualification required for a changed artifact; do not create another browser harness or expand the browser matrix.
5. Sol reviews one frozen candidate commit against this brief, particularly current-source/artifact coherence, removed metadata, unshifted compressor timing, exact version guards, and unchanged publication protections. Required PR qualification passes before merge. Record exact commands, counts, tarball digest, embedded worklet digest, review verdict, and CI run in the issue.

These gates qualify packaging and the causal regression needed by the downstream app. They do not repeat #737's full native DSP proof, create benchmarks, claim performance or sound-quality improvements, or qualify private sparse-stem audio.

## Exact-artifact publication and concurrency procedure

The existing workflow deliberately requires `GITHUB_REF == refs/heads/main`, `GITHUB_SHA == expected_sha`, and checks out that exact SHA. Because a later commit advancing `main` makes a dispatch for an older release SHA fail before checkout, root must reserve a short main release window after the reviewed version commit merges:

1. Fetch and verify the merge is the current `origin/main` tip. Record its full SHA as `RELEASE_SHA`; dispatch `mode=qualify`, `expected_sha=RELEASE_SHA`, and an empty qualification run ID while main remains at that SHA.
2. After qualification succeeds, inspect the named `engine-sdk-qualify-RELEASE_SHA` artifact and run metadata. Without allowing main to advance, dispatch `mode=publish` with the same SHA and the successful qualification run ID. The workflow publishes the downloaded qualified tarball, never a rebuild.
3. If publication returns ambiguously, do not retry publish. Dispatch only `mode=verify` with the same SHA/run ID after propagation. Confirm version, tarball shasum/integrity, public access, `latest`, fresh imports/`enginectl`, signature audit, SLSA subject `pkg:npm/%40misofm/engine@0.2.3`, workflow identity, and resolved dependency equal to `RELEASE_SHA`.
4. If main advances at any point before successful publish/verify, stop the release window. Do not weaken the SHA checks and do not reuse the old qualification. Rebase/recreate the tiny identity change on the new synchronized tip, rebuild and qualify a fresh artifact, receive Sol review for the changed candidate, and restart with its SHA. Coordinate the main hold rather than adding a second pipeline.

Only after the evidence commit is upstream should root update/close the matching GitHub issue and verify remote closure. Report 0.2.3 as released only after registry and provenance verification. Then remove the clean completed worktree under the repository policy.

## Risks and explicit exclusions

- The largest correctness risk is source/binary skew: publishing #737's accepted intermediate worklet with later gate, multiband, SDK, or host sources. Exact current-tip rebuilding and hash equality prevent it.
- Removed lookahead parameters are a deliberate prelaunch contract change. Consumers with copied keys/defaults can fail until their separately reviewed dependency-adoption slice regenerates them.
- The workflow's exact-head guard creates an operational serialization requirement. Concurrent main movement invalidates the candidate and qualification; it never justifies relaxing provenance.
- A green package build alone does not prove the app blocker fixed. The unshifted public headless timing discriminator is required, while the full app #187 gate runs only after registry adoption.
- This issue publishes no adapter, app, website, codec, private fixture, or new Rust behavior and makes no CPU, listening, or broad zero-latency-suite claim.
