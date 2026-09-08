# Qualify and pin the limiter detector AudioWorklet artifact

GitHub: https://github.com/misofm/engine/issues/627

Parent: #621 (lane A, FX2). Coordination: #559/#560. Predecessor: delivered #625/PR #626.

This is the separately numbered lane-B artifact successor required by #621. The accepted limiter access change is frozen at `crates/true-peak-limiter/src/lib.rs` SHA-256 `32ab4abf975b32d47c85a748e617e74c9547b22e1b585f0d36713be439a62908`. #621 integrated delivered main `30680709c58f0be99e09d006d8d661c1ce96324d` and passed Astra LOW review at merge checkpoint `77e9c7d7536916f045a8aa9c66aea86b5c2fe0c2`; its synchronized record head is `dc14ca856e10cb5ad7f6fdacb0ea9322342a9251`. The delivered AudioWorklet Wasm pin is `ac71c64033b0cfc637cf14edcacaa6ed1b3bbf7093a5caa641ef84adea7e88e3`, attributed to #623 source `ca5a8b492a41ba85b3e90d8dedb2b49b787f2f00`.

#621 remains the one launch-critical implementation issue. This successor occupies the second issue slot and exclusively owns the artifact applicability decision, qualification, pin, lineage, and evidence. Sol HIGH coordinates and owns checkpoints, artifact qualification/pinning decisions, GitHub synchronization, and delivery. Astra LOW performs every scope, probe, candidate, promotion, post-pin, exact-head, and delivery review. Luna HIGH or XHIGH executes each probe, qualification, and repository-promotion stage only after the preceding Astra LOW verdict separately authorizes that stage. No compiler capture, benchmark, timing workload, DSP edit, or new `.ll` artifact is allowed.

## Smallest closable outcome

At a clean isolated checkout of exact frozen source `dc14ca856e10cb5ad7f6fdacb0ea9322342a9251`, run one ordinary no-bypass AudioWorklet build attempt into an empty external directory and preserve its complete status, streams, source/toolchain/config/input identities, output census, and hashes. The build must use the unchanged `scripts/build-web-audioworklet.sh`; do not set the repin bypass.

If the existing pin accepts the build, require exactly six emitted files and byte-for-byte identity with the six delivered #623 files. That proves artifact non-applicability: record Astra LOW PASS, make no pin or lineage change, and release this successor for closure.

If the builder rejects only because the observed Wasm digest differs from the delivered pin, preserve the failed probe with zero published output and the observed digest. Astra LOW must independently verify the probe, compiled inputs, frozen source, and mismatch before authorizing one scratch qualification. Any build failure with another cause, missing digest, extra output, dirty source, or changed build input is FAIL and stops for reviewed rescope. Do not retry the probe.

## Conditional scratch qualification

After Astra LOW mismatch PASS, create one detached scratch checkout from exact frozen source. It may first differ only by setting `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256` to the verified candidate digest plus LF. Prove that overlay, run the ordinary builder exactly once into a new empty external directory, require exactly six files, require the candidate Wasm digest, and compare all five non-Wasm files byte-for-byte with the delivered #623 artifact. Any difference outside Wasm stops for rescope.

The scratch checkout may then change only `hosts/host-web/qualification/results.json` to set `candidateCommit` exactly to `dc14ca856e10cb5ad7f6fdacb0ea9322342a9251` and `wasmSha256` exactly to the verified candidate digest, then regenerate only `hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md` with the unchanged generator. Browser rows, browser version floors, gate vocabulary, resource values, ABI, SDK surface, and all other result fields remain frozen.

On that exact six-file candidate and scratch source, run each unchanged gate once:

1. shipped Wasm ABI/export/import/memory/realtime-callgraph/SIMD/static/metadata/vocabulary/resource checks;
2. the separate expected-resource/native-witness gate and its full existing red-mutation set;
3. hermetic host/worklet policy and mutation checks with an isolated Cargo target directory;
4. applicable SDK package and generated-surface checks using locked installs only;
5. locked qualification dependency installation;
6. exactly one Chromium, Firefox, and WebKit qualification, including matrix self-tests, AudioWorklet boot/control/observation/stall, native-corpus PCM identity, resources, and lineage;
7. proof that qualification changes only the authorized candidate source/digest lineage while the frozen browser outcomes, versions, gates, and resources remain identical.

Preserve compact checksum-verified evidence. Full compiler IR/assembly captures and archives whose purpose is retaining them are forbidden. Astra LOW must return scratch-candidate PASS before any repository pin or lineage edit.

## Conditional promotion and delivery

After scratch-candidate PASS, Luna HIGH/XHIGH may change only:

- `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256` to the approved digest plus LF;
- `hosts/host-web/qualification/results.json` only at `candidateCommit` and `wasmSha256` with the exact approved values;
- `hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md` only through the unchanged generator, yielding matching lineage text;
- this numbered spec and bounded evidence under this issue.

No Rust, JS/TS, ABI, metadata, browser result row, version floor, resource expectation, dependency, lockfile, toolchain/config, build/check script, policy, workflow, corpus, fixture, DSP, session, SDK surface, #621 source/test/evidence, or historical artifact evidence may change.

Root checkpoints the exact promotion before further work. Run one ordinary no-bypass post-pin build from the clean pushed repository head and require exact six-file identity with the qualified scratch candidate. Run the proportional static gate, expected-resource/native-witness mutation gate, hermetic/SDK checks, matrix generation/check, formatting/diff, workspace and effect-runtime policies once; do not repeat successful browser qualification. Astra LOW then reviews the exact pushed head against current main, frozen-source ancestry, evidence checksums, exact promotion scope, unchanged rows/resources, pin/lineage, and post-pin identity.

Open one PR only after Astra LOW exact-head/current-main PASS. Require repository `qualification`, verify live main immediately before a guarded exact-head merge, verify exact merge parents and post-main qualification, synchronize and close this successor and #621, update #559/#560, and remove clean delivered and detached worktrees while retaining branches/history/evidence.

One applicability probe and, conditionally, one scratch qualification plus one Luna promotion attempt are initially authorized only after their preceding Astra LOW PASS. A candidate mismatch beyond the expected Wasm digest, substantive gate failure, or tooling defect stops for reviewed rescope; do not rerun builds or browsers to obtain a green result. The repository three-attempt rule remains binding.

## Astra LOW initial scope review — PASS

Astra LOW passed exact clean pushed brief
`8961928817faf566084850285c8d4438cfda2695` against live main and merge-base
`30680709c58f0be99e09d006d8d661c1ce96324d`, with synchronized tracker
`454e621f832affa4239ea89508b97adc088864a3`. Local/GitHub #621/#627 and #559/#560
records match. Frozen source ancestry, limiter SHA-256, delivered #623 pin and
lineage, the two-slot ownership split, one-shot failure stops, conditional
static/resource/SDK/three-browser gates, exact three-file promotion boundary,
post-pin proof, compact evidence and delivery controls all pass. Diff hygiene
passes, and only this spec was added after frozen parent `dc14ca85`.

Exactly one ordinary no-bypass applicability build is authorized from frozen
`dc14ca856e10cb5ad7f6fdacb0ea9322342a9251`, with complete evidence and no
retry. A pin mismatch requires a separate Astra LOW review before scratch
qualification. Promotion, PR, and merge remain unauthorized.

This initial review accepted root's stated artifact-qualification ownership and
authorized root to launch that exact one-shot operation. It is distinct from
the later routing review below.

## Astra LOW later routing review — FAIL

A separate Astra LOW review then returned **FAIL** at the same exact brief
`8961928817faf566084850285c8d4438cfda2695`, live main `30680709`, parent #621
head `dc14ca85`, and tracker `454e621f`. Scope substance passed, but the brief
allowed Luna HIGH/XHIGH only to perform conditional repository promotion rather
than all later execution stages. Commit `da1f64ac3f05badce2a249b226fb520ba2ffcd4e`
corrected the routing sentence. No operation was launched on the authority of
this failed review.

## Astra LOW corrected routing review — PASS

Astra LOW returned **PASS** at exact clean pushed head
`da1f64ac3f05badce2a249b226fb520ba2ffcd4e`, live main
`30680709c58f0be99e09d006d8d661c1ce96324d`, and unchanged tracker
`454e621f832affa4239ea89508b97adc088864a3`. The corrected routing sentence
assigns probe, qualification, and promotion execution to Luna HIGH/XHIGH after
each preceding Astra LOW authorization. It does not erase the distinct initial
PASS or authorize a second probe.

Root launched the sole applicability probe under the initial PASS before seeing
the concurrent routing FAIL/correction. It ran once from frozen `dc14ca85` and
was not retried. Every subsequent qualification or promotion execution belongs
to Luna HIGH/XHIGH. Scratch qualification and repository promotion remain
unauthorized pending separate Astra LOW reviews.

## Astra LOW mismatch-evidence review — documentation FAIL

Astra LOW reviewed clean pushed evidence head
`e440d62b1aa781c43c3b7e5f83a92e405720d77b` against main `30680709` and frozen
source `dc14ca85`. All 16 manifest entries and input hashes verify. The one
no-bypass invocation completed compilation, exited 1 solely on the pin mismatch,
published zero files, and left source clean. Delivered digest is `ac71c640…` and
candidate digest is `63ef81c105d50aed41164aa3c6c6f8853a314b99d642e209e7cc3aefe3bdbca1`.

The evidence is technically valid and root execution does not invalidate its
bytes. Review failed only because this spec and the evidence README had not
recorded the distinct review chronology and executor attribution. This
documentation-only correction changes no raw probe byte. Astra LOW confirmation
is required before one Luna scratch qualification can be authorized; no probe
rerun is permitted.

## Astra LOW mismatch-evidence confirmation — PASS

Astra LOW accepted the candidate mismatch at exact clean pushed head
`573d2420f4140cb181931a9bc5330a9f2afa6491`, live main `30680709`, and
synchronized tracker `7343ed73cbc834149f07d316a1e802ad65fc6076`.
Only the spec, evidence README, and its manifest hash changed after `e440d62b`;
all raw probe bytes remain unchanged. All 16 checksums, issue bodies, executor
chronology, and diff hygiene pass.

Candidate Wasm SHA-256 is frozen as
`63ef81c105d50aed41164aa3c6c6f8853a314b99d642e209e7cc3aefe3bdbca1`.
Exactly one Luna HIGH scratch qualification is authorized from frozen source
`dc14ca856e10cb5ad7f6fdacb0ea9322342a9251` under the overlay sequence and
stop-without-retry gates above. The applicability probe must not be repeated.
Repository promotion remains unauthorized until Astra LOW returns candidate
PASS.

## Astra LOW corrected mismatch-evidence review — PASS

Astra LOW returned **PASS** at exact clean pushed head
`573d2420f4140cb181931a9bc5330a9f2afa6491`, frozen source `dc14ca85`, tree
`5817dc4710f089a869d6849a85e7478ac71051f4`, and live main `30680709`. All 16
manifest entries and frozen input hashes verify. The recorded invocation
explicitly unsets the repin bypass; compilation succeeded before exit 1 solely
for delivered `ac71c640…` versus candidate `63ef81c105d50aed41164aa3c6c6f8853a314b99d642e209e7cc3aefe3bdbca1`.
The detached source remains clean and the output remains empty. Scope, diff
hygiene, GitHub synchronization, and corrected chronology pass.

The record proves one invocation without a retry but is not independent process-
count telemetry. Its cwd and external paths are context rather than durable
artifacts, and the candidate digest is builder-reported because no rejected Wasm
was published. Exactly one Luna HIGH/XHIGH detached scratch qualification is now
authorized to reproduce and independently hash the candidate bytes, prove all
five non-Wasm outputs, and run the brief's remaining gates. Repository promotion
remains unauthorized pending Astra LOW candidate PASS.

## Scratch candidate attempt 1 — execution complete; procedural review required

The first detached scratch sequence ran from frozen `dc14ca85`. Its builder
emitted exactly six files; independent hashing confirms candidate Wasm
`63ef81c105d50aed41164aa3c6c6f8853a314b99d642e209e7cc3aefe3bdbca1` and
byte-identical #623 hashes for all five non-Wasm outputs. Matrix generation, the
static gate, separate expected-resource/native-witness mutations, hermetic gate,
locked SDK and browser installs, SDK checks, one Chromium/Firefox/WebKit
qualification, and final matrix check each exited zero. The scratch diff is
exactly the pin, two lineage fields, and generated matrix lineage; rows, versions,
gates, and resources remain frozen.

A concurrency race launched a later builder-only invocation in a second detached
checkout while the first sequence was already running. It emitted the same six
hashes and was stopped before any downstream matrix, static, resource, hermetic,
SDK, install, or browser command. This was not a retry made after observing a
result, but it exceeds the brief's exactly-one scratch-builder rule. Both records
are preserved together and distinguished in the evidence README; no action was
hidden or rerun. Candidate PASS is not claimed. Astra LOW must rule on both the
technical evidence and this procedural breach. Repository promotion and every
further workload remain unauthorized.

## Astra LOW scratch attempt 1 review — FAIL

Astra LOW returned **FAIL** at exact clean pushed head
`bd7f39e98398c3d3ae3d66ec9ad73a35ed307551`. Repository promotion is not
authorized. The decisive breach is the later second scratch-builder invocation;
identical output and concurrency do not satisfy the explicit exactly-one-build
rule. Branch-wide diff hygiene also exited 2 on trailing spaces inside two raw
overlay diff captures.

The technical evidence remains useful: all 82 scratch and 16 probe checksums
verify; numbered stages `00` through `10` succeeded; the six candidate hashes
independently verify, including Wasm `63ef81c1…`; source/tree and the three-file
scratch overlay match; results change only the two lineage fields; the resource
gate reports all 26 red mutations; and Chromium 151.0.7922.34, Firefox 153.0,
and WebKit 26.5 passed. No builder, gate, install, or browser rerun is authorized
or needed to repair the procedural breach.

Root losslessly packaged only `second-overlay.diff` and `final-overlay.diff` as
deterministic gzip files. Their exact original byte counts and SHA-256 identities
are recorded, decompression reproduces the reviewed bytes, and the evidence
manifest is refreshed. This changes no qualification result or raw content.

## Attempt 2 disposition scope — review required

Attempt 1 remains failed and must never be relabeled. Attempt 2 is a bounded,
no-execution disposition of the existing evidence. It may determine whether the
first complete scratch sequence independently supplies acceptable product
qualification despite the separately recorded unauthorized concurrent builder.
The review must prove the first sequence began independently, owns the numbered
`00`–`10` record and qualified output, and was not altered or selected based on
the later builder. It must separately preserve and identify the duplicate's
build-only record and identical hashes. No new or repeated build, gate, install,
browser, output generation, pin edit, lineage edit, or promotion may occur.

Astra LOW must PASS this corrected packaging and attempt-2 scope before Luna may
perform any documentation-only evidence revision. A later Astra LOW attempt-2
evidence PASS is separately required before the existing candidate can authorize
the original exact three-file repository promotion. This rescope preserves the
failed exactly-one gate as history; it does not claim attempt 1 passed or hide the
concurrent invocation.

## Astra LOW attempt 2 disposition-scope review — PASS

Astra LOW passed the no-execution disposition scope at exact clean pushed head
`2b9d3abed51b77f40202bc719ecc2c0659c2948a`. Attempt 1 remains failed and
promotion remains blocked. Luna HIGH may revise only documentation/evidence to
establish the first sequence's independently recorded chronology and output
ownership and segregate the later duplicate builder as non-credit. Every raw
record must remain, and the revision must distinguish recorded facts from
chronology that the retained evidence cannot prove.

No build, gate, install, browser, output generation, pin, lineage, or promotion
execution is authorized. A separate Astra LOW attempt-2 evidence-disposition
PASS remains required before promotion can be considered.

## Astra LOW attempt 2 evidence-disposition review — PASS

Astra LOW returned **PASS** at exact clean pushed head
`40ecd3db63d8b0e6f728217a41a5502287359029`. All 91 manifest entries verify.
The eleven numbered records consistently bind frozen `dc14ca85`, the first
scratch checkout and `/tmp/issue627-qualified-output`; all statuses are zero and
none refers to the duplicate checkout or output. Normalized comparison proves
the five non-Wasm files unchanged and only the expected Wasm digest changed.
GitHub/spec synchronization passes.

The first sequence independently supports candidate qualification. The retained
records cannot prove duplicate chronology or independent process count, but no
such claim supplies technical credit: the duplicate remains separately preserved
and receives none. Attempt 1 remains **FAIL**. Luna HIGH/XHIGH may now make
exactly the original three-file repository promotion for approved candidate
`63ef81c105d50aed41164aa3c6c6f8853a314b99d642e209e7cc3aefe3bdbca1`. No
build, gate, install, or browser execution is authorized until root checkpoints
that promotion.
