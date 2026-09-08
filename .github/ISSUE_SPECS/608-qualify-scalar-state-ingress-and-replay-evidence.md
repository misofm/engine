# Qualify scalar StateGet ingress and replay evidence

Status: active qualification successor to stopped IO5 child #605 under audit #349 and lane-B handoff #560. It inherits #605's pushed implementation through stop record `b2c2d120` and exact accepted production correction `b5c1a3182f9aea176016d08797a31ed667f5e47c`; final fixture correction `c14cc8b20ecaf9acba02a9b3ae3d75bc9e38dfdd` remains preserved but failed review. #605 consumed three implementation attempts, is closed, and receives no fourth correction. This successor advances the same original partial finding and starts no original open finding.

Sol HIGH coordinates and owns checkpoints, GitHub synchronization, delivery, and any later AudioWorklet artifact qualification/pinning. Luna HIGH or XHIGH implements. Astra LOW performs every scope, source, artifact-applicability, and exact-head/current-base verification. Lane-A #611 is the only other active issue and owns disjoint RT5 vocabulary mutation-test/evidence paths.

## Smallest closable outcome

Complete only the three finite evidence obligations left by Astra LOW's final #605 verdict:

1. Prove successful Left-only and Right-only subsets through typed, B1b and caller-buffer StateGet ingress against the same published page. Prove reversed two-handle order succeeds and is preserved through typed ingress. At each encoded ingress boundary, mutate an otherwise valid request to reversed handle bytes and prove the frozen canonical wire decoder rejects it as malformed while preserving that boundary's existing replay behavior and all publication/automation/resource/event/credit state.
2. For every rejected publication, including nonfinite/invalid values in either record, immediately encode the accepted page again and compare its state-page payload bytes bit-for-bit with the pre-rejection accepted payload. Prove identical republication has the same encoded payload before any replacement.
3. After publication changes, prove retained StateGet replay succeeds into an output buffer of the exact cached-response length, a one-byte-short buffer refuses without consuming the cached response, and malformed/reused changed bytes preserve the delivered replay/error precedence.

The inherited production implementation, error surface, publication API, controller paths, host translation, resource accounting and render code are frozen. This successor adds no capability; it qualifies the already preserved fixed two-record explicit publication behavior.

## Exact ownership

Allowed implementation paths are:

- `crates/host-core/tests/scalar_point_endpoint.rs`
- this numbered spec and bounded issue evidence

Exclude all production Rust, every other test, `Cargo.toml`, `Cargo.lock`, protocol wire/schema/queue/delivery code, providers, other host endpoints, C ABI/browser/SDK/generated surfaces, graph/session/compiler/effects/banks/segments, policies/workflows, #611 test/evidence paths, AudioWorklet artifacts/pins, and benchmarks. If a production defect prevents these tests, stop and amend or rebrief before editing source.

## Objective gates

1. **Ingress projections at their frozen boundaries.** One published asymmetric page is queried through typed, B1b and caller-buffer paths. Every path proves Left-only and Right-only requests with exact observed sample, record order, flags and value bits. Typed ingress also proves successful reversed `[Right, Left]` order. B1b and caller-buffer each receive a byte-mutated reversed request derived from a valid encoded frame and must return the existing malformed-frame result. B1b proves decode failure before replay admission. Caller-buffer uses a new correlatable request with sufficient output capacity, proves that the malformed non-OK response is cached and returned byte-identically on exact replay, proves changed-byte reuse precedence, and proves unrelated retained hits remain byte-identical. Neither boundary may change the published page or automation/resource/event/credit state. This preserves the wire contract's bounded, sorted, unique, nonzero handle rule and each ingress path's frozen replay semantics. Existing unknown/unpublished cases remain green. No second decoder, controller, provider, queue or ledger is introduced.
2. **Encoded preservation after refusal.** Capture the encoded state-page payload bytes, excluding request-specific frame identity fields through an explicit parser/offset already authoritative in the fixture. After each independently discriminated invalid publication—wrong/reversed/zero/duplicate handles, invalid flags, and nonfinite value in record one and record two—issue a fresh StateGet and require bit-identical payload bytes. Re-publish the identical valid snapshot and require the same payload bytes before testing a later valid replacement. Decoded equality alone receives no credit.
3. **Cached replay output contract.** Cache one successful StateGet response, change the live publication, and replay the original exact request. First use a one-byte-short caller buffer and require the existing output-reservation refusal without replay loss; then use the exact cached-response length and require the original response bytes. Exercise a changed-byte reuse of the same request ID and the existing malformed outer/correlatable payload precedence without changing the retained hit.
4. **No side effects.** Before and after every qualification group, assert outstanding and resident automation counts, queue report fields, reliable-event availability/sequence behavior, and terminal credit remain unchanged. State reads and rejected/idempotent publications may not acknowledge, drop, admit, hand off, collect or cancel automation.
5. **Regression and hygiene.** Protocol library and feature-enabled scalar endpoint tests pass in debug and release; strict affected Clippy, formatting, diff, workspace/protocol-control/host-core/realtime policy and supported scalar/SIMD Wasm checks pass proportionally. `Cargo.lock` remains byte-identical. No artifact, browser, listening or timed work receives credit before source PASS.

## Artifact and delivery workflow

Push and synchronize this numbered brief, #560, and GitHub before implementation. Astra LOW must pass exact clean scope/current-base/path ownership. Luna performs one coherent fixture pass and pauses for root checkpointing. Astra LOW reviews the exact pushed test source against all three remaining obligations. Up to three attempts remain available to this successor, but each attempt is one bounded fixture pass and one verdict; do not weaken gates.

Only after source PASS may root run the ordinary six-file AudioWorklet identity probe. Byte identity permits retained-artifact qualification with existing attribution; drift requires separately authorized scratch qualification before any pin or generated-consumer change. Recheck current main, #611 disjointness, exact reviewed head/base/merge-base, required PR qualification, guarded merge, post-main qualification, GitHub closure, and clean worktree removal. A post-delivery Astra LOW residual audit determines the next IO5 obligation or closure.

## Stop and split triggers

Stop before changing production code, introducing a fixture framework, editing another test, widening to automatic clock/lifecycle publication, activating a host, adding graph/bank/effect/parameter/segment support, altering wire/schema/replay semantics, touching #611, or running artifact/browser/timed work before source PASS. Preserve the inherited implementation and candid evidence rather than disguising another #605 attempt.

## Inherited evidence

Astra LOW's #605 final verdict at exact reviewed head `4a63daf087ce4c11374401036f4af4d23c697553` accepted second-zero validation, typed unpublished behavior, real sticky-fault refusal with retained state/time, cancellation record/time preservation, delivery ownership/event assertions, typed error documentation and live allocation counting. Protocol 161/161, scalar endpoint 10/10 and diff checks passed with no lock drift. It rejected only the all-ingress subset/order matrix, second-record invalid plus encoded-byte preservation, and exact cached-replay output/precedence evidence listed above. Those accepted controls remain regressions and must not be rewritten merely to restate them.

## Astra LOW scope review

Astra LOW returned **PASS** at exact clean head/upstream
`6d395f054ef78a4c84ce004a727b3642c947d701`, with main/merge-base
`6fe8676e1537bc2c952ac87ee2fe31c545438474`, tracker `6bff8180c5a7c923f6e836942e635f42318aef09`
and current #607 coordination head `20a9265cd705fe33d76657a37ce44c43fc1e7997`. The reviewer confirmed
that this qualification-only successor preserves #605's three failed attempts, freezes production
and artifacts, owns only the scalar endpoint fixture plus spec/evidence, and discriminates exactly
the three residual obligations without a new framework. Luna HIGH/XHIGH attempt 1 is authorized
within that exact scope. A production defect requires a stop and rebrief; artifact work remains
deferred until source PASS.

## Pre-implementation scope correction

Luna stopped before editing at clean head `8589cbb895ada9c86416d7e2b5bf8bfc6c865d45` after finding that
the initial wording required an impossible successful encoded reverse order. The authoritative
`message_wire::check_handles` rejects zero, duplicate and non-increasing handles during both encode
and decode; `ParameterStateRequest` documents the same sorted contract. No implementation attempt
was consumed, no test or source changed, and production remains frozen.

The corrected first gate above preserves successful reverse order only for typed ingress. B1b and
caller-buffer must instead prove that a byte-mutated reversed request is rejected at the canonical
wire boundary, while both successful single-handle subsets remain required through all three
ingress paths. The prior Astra LOW PASS is retained as historical evidence but does not authorize
implementation against this amendment. A fresh exact-head Astra LOW scope PASS is required.

## Astra LOW corrected-scope review 1

Astra LOW returned **FAIL** at exact clean head/upstream
`d574d878481609208b782d8757fc16d6d26d8b65`, tracker
`0413f9b7070bd658133ee667d27a9e74065c0b6c`, and main/merge-base `6fe8676e`. The ordering correction
was accepted, and the reviewer acknowledged that the earlier all-ingress reversed-success demand
was wrong. One boundary distinction remained: malformed B1b decoding returns before replay
admission, while a sufficiently provisioned caller-buffer request caches its correlatable malformed
non-OK response through `replay.complete`.

The first gate now freezes those distinct delivered behaviors: unchanged B1b replay state;
caller-buffer cached malformed response, exact replay, changed-byte reuse precedence, and unrelated
retained-hit preservation. Publication and automation/resource/event/credit state stay unchanged
in both cases. No implementation attempt was consumed. Implementation remains unauthorized pending
exact-head Astra LOW confirmation of this bounded wording correction.


## Current lane-A coordination

#607 stopped after PR #609 failed the required environment/marker vocabulary gate; its accepted
capture is preserved and will not be rerun. Stopped #610 preserved those 20 rows. Active replacement #611 owns only two stale count expectations in
`scripts/test-env-vocabulary.sh` plus its spec/evidence. #608/#611 are the two active issue slots and
their paths are disjoint. Historical #607 scope statements above remain accurate for their recorded
checkpoints.

## Astra LOW final scope confirmation

Astra LOW returned **PASS** at exact clean feature head/upstream
`afd897596d8cc1e361bca126f6aebb2281dc1861`, main/merge-base
`6fe8676e1537bc2c952ac87ee2fe31c545438474`, and synchronized tracker
`8f8bd132381b577347febac38795613ea0e031f7`. GitHub #608 matched; #607 was closed and #610 open
with disjoint documentation ownership. The second corrected ingress/replay gate and the other two
residual gates are approved. No implementation attempt was consumed before this verdict.

Tracker head `f835f685fb1aafbd94cdbd3f1c3b81582a8c3479` subsequently recorded only #610's Astra LOW scope
authorization and preserves the same disjoint ownership. Luna HIGH/XHIGH attempt 1 may edit only
`crates/host-core/tests/scalar_point_endpoint.rs`; production, other tests, #611 paths and artifacts
remain frozen. Artifact probing remains deferred until source PASS.

## Luna attempt 1 checkpoint

Luna HIGH attempt 1 is pushed at source `0820c8a7ba785e5834694f1a5663dd7b2b835d46` from authorization
head `4e33fcee`. The only implementation change is
`crates/host-core/tests/scalar_point_endpoint.rs`. It adds both successful subsets across all three
ingress paths, typed reverse ordering, the distinct B1b/caller-buffer malformed replay behavior,
encoded state-page payload preservation after every refused publication including record two,
idempotent republication, and cached-response output/replay precedence after replacement.

Focused and full scalar endpoint debug tests pass; scalar endpoint release passes 10/10; protocol
passes 161/161; strict affected Clippy, formatting and diff checks pass. `Cargo.lock` is
byte-identical. Production, #611 and artifacts remain untouched. Astra LOW source review is
pending; artifact probing remains unauthorized.


#610 stopped after its correct vocabulary rows exposed two stale hard-coded count expectations outside
its ownership. Active #611 owns only those two `114` to `134` test payload updates plus evidence.
#608/#611 remain the two disjoint active slots; the accepted RT5 capture cannot be rerun.

## Astra LOW attempt 1 source review

Astra LOW returned **PASS** for implementation source
`0820c8a7ba785e5834694f1a5663dd7b2b835d46` at requested evidence head
`1034164addf30197b7651e899cf986b82f2f87dd`, against main/merge-base `6fe8676e`. Coordination-only
head `b3537844b0813aa6714db899056778fbd5d88777` changes only this spec to name active disjoint #611;
the source remains byte-identical and the tree is clean.

The reviewer accepted the complete corrected ingress matrix, boundary-specific malformed replay,
encoded payload preservation, second-record invalid rejection, idempotent republication, and exact
cached-response buffer/replay precedence controls. Header/payload offsets match the authoritative
framing layout. Independent scalar endpoint debug/release passed 10/10, protocol passed 161/161,
and strict affected Clippy, formatting, diff, host-core policy and protocol-control policy passed
with no lock drift. Production remains frozen.

After this verdict is recorded upstream, root may run exactly one ordinary six-file artifact
identity probe. The verdict authorizes no pin change, browser qualification or PR.

## Root-owned AudioWorklet identity probe

After the source verdict and tracker records were pushed, root ran the ordinary no-bypass builder
exactly once at clean head `6366c304627f28ae8e27d3b9e5f7acce3860225a`. It exited zero and emitted
exactly six files. Every SHA-256 is byte-identical to the canonical delivered #587 manifest: ABI
layout `40f6fe2e…`, host declaration `445254e7…`, host JavaScript `21c8947d…`, AudioWorklet
JavaScript `225bc060…`, simd128 Wasm `39ebe7cd…`, and parameter metadata `6eac2cb3…`. The Wasm
matches the repository pin, and the copied web sources match their repository inputs.

The command, status, complete compiler stderr, file census, hashes and exact manifest comparison
are preserved in `artifacts/issue608-attempt1/`. No repin/bypass environment, retry, second build,
browser run, pin change or generated-consumer change occurred. Astra LOW artifact review is pending;
the probe alone does not authorize PR or delivery.

## Astra LOW artifact identity review

Astra LOW returned **PASS** at exact clean evidence head/upstream
`ed1090d02d5d2fc2fec594cb93ed374074f5c57e`, main/merge-base `6fe8676e`, and tracker
`97088f2c285db1aa110acebee137c318484ec2f3`. The reviewer verified all ten evidence checksums, both
zero status records, complete compiler stderr, the six-file census and hashes, empty comparison
diff, unchanged repository source copies, and unchanged pin. The evidence records exactly one
ordinary no-bypass builder invocation at `6366c304`; no repeat or repin-mode invocation occurred.

All six files match delivered #587 byte-for-byte. Its static/resource/SDK/browser qualification
applies with original candidate `ab3766caef34bcb035d7394224b0ccff1ea0be2d` and browser attribution
unchanged; this issue claims no new browser run. No pin or generated-consumer change is authorized.
Root may proceed to final exact-head/current-base PR-readiness review.
