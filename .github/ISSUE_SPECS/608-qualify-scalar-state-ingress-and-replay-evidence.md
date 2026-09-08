# Qualify scalar StateGet ingress and replay evidence

Status: proposed qualification successor to stopped IO5 child #605 under audit #349 and lane-B handoff #560. It inherits #605's pushed implementation through stop record `b2c2d120` and exact accepted production correction `b5c1a3182f9aea176016d08797a31ed667f5e47c`; final fixture correction `c14cc8b20ecaf9acba02a9b3ae3d75bc9e38dfdd` remains preserved but failed review. #605 consumed three implementation attempts, is closed, and receives no fourth correction. This successor advances the same original partial finding and starts no original open finding.

Sol HIGH coordinates and owns checkpoints, GitHub synchronization, delivery, and any later AudioWorklet artifact qualification/pinning. Luna HIGH or XHIGH implements. Astra LOW performs every scope, source, artifact-applicability, and exact-head/current-base verification. Lane-A #607 is the only other active issue and owns disjoint RT5 preflight/evidence paths.

## Smallest closable outcome

Complete only the three finite evidence obligations left by Astra LOW's final #605 verdict:

1. Prove successful single-handle subsets and reversed two-handle order through each of typed, B1b and caller-buffer StateGet ingress against the same published page.
2. For every rejected publication, including nonfinite/invalid values in either record, immediately encode the accepted page again and compare its state-page payload bytes bit-for-bit with the pre-rejection accepted payload. Prove identical republication has the same encoded payload before any replacement.
3. After publication changes, prove retained StateGet replay succeeds into an output buffer of the exact cached-response length, a one-byte-short buffer refuses without consuming the cached response, and malformed/reused changed bytes preserve the delivered replay/error precedence.

The inherited production implementation, error surface, publication API, controller paths, host translation, resource accounting and render code are frozen. This successor adds no capability; it qualifies the already preserved fixed two-record explicit publication behavior.

## Exact ownership

Allowed implementation paths are:

- `crates/host-core/tests/scalar_point_endpoint.rs`
- this numbered spec and bounded issue evidence

Exclude all production Rust, every other test, `Cargo.toml`, `Cargo.lock`, protocol wire/schema/queue/delivery code, providers, other host endpoints, C ABI/browser/SDK/generated surfaces, graph/session/compiler/effects/banks/segments, policies/workflows, #607 paths/evidence, AudioWorklet artifacts/pins, and benchmarks. If a production defect prevents these tests, stop and amend or rebrief before editing source.

## Objective gates

1. **All ingress projections.** One published asymmetric page is queried through typed, B1b and caller-buffer paths. Each path proves Left-only, Right-only and reversed `[Right, Left]` requests with exact observed sample, record order, flags and value bits. Existing unknown/unpublished cases remain green. No second decoder, controller, provider, queue or ledger is introduced.
2. **Encoded preservation after refusal.** Capture the encoded state-page payload bytes, excluding request-specific frame identity fields through an explicit parser/offset already authoritative in the fixture. After each independently discriminated invalid publication—wrong/reversed/zero/duplicate handles, invalid flags, and nonfinite value in record one and record two—issue a fresh StateGet and require bit-identical payload bytes. Re-publish the identical valid snapshot and require the same payload bytes before testing a later valid replacement. Decoded equality alone receives no credit.
3. **Cached replay output contract.** Cache one successful StateGet response, change the live publication, and replay the original exact request. First use a one-byte-short caller buffer and require the existing output-reservation refusal without replay loss; then use the exact cached-response length and require the original response bytes. Exercise a changed-byte reuse of the same request ID and the existing malformed outer/correlatable payload precedence without changing the retained hit.
4. **No side effects.** Before and after every qualification group, assert outstanding and resident automation counts, queue report fields, reliable-event availability/sequence behavior, and terminal credit remain unchanged. State reads and rejected/idempotent publications may not acknowledge, drop, admit, hand off, collect or cancel automation.
5. **Regression and hygiene.** Protocol library and feature-enabled scalar endpoint tests pass in debug and release; strict affected Clippy, formatting, diff, workspace/protocol-control/host-core/realtime policy and supported scalar/SIMD Wasm checks pass proportionally. `Cargo.lock` remains byte-identical. No artifact, browser, listening or timed work receives credit before source PASS.

## Artifact and delivery workflow

Push and synchronize this numbered brief, #560, and GitHub before implementation. Astra LOW must pass exact clean scope/current-base/path ownership. Luna performs one coherent fixture pass and pauses for root checkpointing. Astra LOW reviews the exact pushed test source against all three remaining obligations. Up to three attempts remain available to this successor, but each attempt is one bounded fixture pass and one verdict; do not weaken gates.

Only after source PASS may root run the ordinary six-file AudioWorklet identity probe. Byte identity permits retained-artifact qualification with existing attribution; drift requires separately authorized scratch qualification before any pin or generated-consumer change. Recheck current main, #607 disjointness, exact reviewed head/base/merge-base, required PR qualification, guarded merge, post-main qualification, GitHub closure, and clean worktree removal. A post-delivery Astra LOW residual audit determines the next IO5 obligation or closure.

## Stop and split triggers

Stop before changing production code, introducing a fixture framework, editing another test, widening to automatic clock/lifecycle publication, activating a host, adding graph/bank/effect/parameter/segment support, altering wire/schema/replay semantics, touching #607, or running artifact/browser/timed work before source PASS. Preserve the inherited implementation and candid evidence rather than disguising another #605 attempt.

## Inherited evidence

Astra LOW's #605 final verdict at exact reviewed head `4a63daf087ce4c11374401036f4af4d23c697553` accepted second-zero validation, typed unpublished behavior, real sticky-fault refusal with retained state/time, cancellation record/time preservation, delivery ownership/event assertions, typed error documentation and live allocation counting. Protocol 161/161, scalar endpoint 10/10 and diff checks passed with no lock drift. It rejected only the all-ingress subset/order matrix, second-record invalid plus encoded-byte preservation, and exact cached-replay output/precedence evidence listed above. Those accepted controls remain regressions and must not be rewritten merely to restate them.
