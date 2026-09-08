# Enable paired builtin dispatch inside the prepared batch endpoint

GitHub: #587 (https://github.com/misofm/engine/issues/587)

Bounded child of #444 and audit lane A #559 on delivered main
`defa979cbf0bf86b4ebba2f52b0647eb01b9ff29`. #576/#579/#580 delivered the
prepared Rust builtin batch endpoint through private separate owners, its lifecycle
and evidence corrections, exact cap preflight, and bitwise PCM comparison. Their
three-attempt histories are closed and are not reopened here. This issue owns the
next independently useful product slice: selecting the existing qualified
fader/matrix pair implementations for that endpoint's closed-batch render schedule.
The later automatic lifecycle-publication operation remains a separate #444 child.

Luna HIGH implements each attempt. Astra LOW performs every scope, source, and
exact-head verification. The Sol HIGH coordinator owns the brief, exact-path
checkpoints, GitHub synchronization, and delivery. Lane B owns AudioWorklet artifact
qualification and pinning.

## Smallest closable outcome

Prepare only `prepare_builtin_batch_endpoint` with the existing
`BuiltinControlDelivery::BetweenRenderCalls` lowering. The endpoint keeps every raw
track producer private, claims at most one complete FIFO batch at the render
boundary, injects it before graph execution, and performs no producer work while the
graph renders. That ownership and cutoff make the already implemented native bank
and scalar fader/matrix pair factories eligible without changing their arithmetic.

The public raw-console preparation functions and their `Concurrent` behavior remain
unchanged. The existing public `prepare_host_runtime_between_render_calls` contract
also remains unchanged. This child adds no pairing algorithm, wire or C ABI command,
session mutation, new queue or ledger, new lifecycle operation, plan swap, locate,
browser activation, artifact pin, benchmark, or performance claim.

## Frozen design

Add the smallest crate-private preparation route needed by the endpoint to select
the already existing between-render-calls policy. Production remains pinned to
`Backend::current()`. The endpoint's existing private test preparation may continue
to select the scalar backend and proof-only PostFader meters, but no backend selector,
meter selector, or raw producer becomes public.

Do not change delivery admission, application sample, late reporting, cancellation,
terminal collection, sticky-fault behavior, capacity arithmetic, resource reports,
allocator ownership, or thread-affinity. Pair eligibility and graph barriers remain
the authority of `builtins-compiler` and `graph`; the endpoint must not duplicate
their decisions. A placement or observation that the existing compiler declines must
still execute through separate processors with identical externally visible state
and PCM.

The independently prepared reference uses the existing separate `Concurrent`
lowering with identical compiled session, source samples, records, quantum, backend,
and selected observation. Comparisons use `f32::to_bits`, including signed zero.

## Exact ownership

Allowed implementation paths:

- `crates/host-core/src/prepare.rs`
- `crates/host-core/src/builtin_batch_endpoint.rs`
- `crates/host-core/tests/builtin_batch_endpoint.rs`
- this numbered spec
- focused implementation and review records under `docs/audits/`
- #444/#559/#560 handoff specs only for concise synchronized status

`crates/host-core/Cargo.toml` and `Cargo.lock` are frozen: #579 already enabled the
existing dev-only `builtins-compiler/test-support` witness surface. Do not edit
protocol, engine, graph, builtins, builtins-compiler, any other host-core module or
manifest, hosts, C ABI, browser, SDK, artifacts, policies, workflows, or lane B #585
path `tools/bench/src/graph.rs`. If selection requires a compiler/graph change or a
new public seam, stop and rebrief rather than widening this issue.

The only exception is direct red-mutation evidence: Luna may temporarily change one
named paired arithmetic operation in `crates/builtins-compiler/src/lib.rs`, run the
focused endpoint/reference discriminator, then restore it. That mutation is never
staged or committed and does not authorize production or test instrumentation there.

## Objective gates

1. Through the public prepared endpoint on native x86-64-v3, address an eligible
   bank lane with asymmetric fader, mute, crossfeed, and smoothing records. Directly
   read the existing test-support witness after preparation and render: a pair
   factory was selected, paired process/member counts are nonzero and internally
   consistent, the complete fader/matrix record set was drained once, and state
   advances through the selected pair. Across immediate, ramping, settled, mid-ramp
   retarget, mute, and unmute blocks, compare mapped PCM bits, applicable target/ramp
   state, and PostFader observation bits with an independently prepared separate
   `Concurrent` reference. These combined witnesses are the available dispatch
   accounting; do not invent a separate-bank call counter.
2. Through the private test-only scalar-backend endpoint, address a real scalar
   owner with nontrivial ramps and retargeting. Directly prove the existing scalar
   pair factory and paired processor were selected. Compare target/ramp state,
   PostFader observation, and output PCM bits with an independently prepared
   separate `Concurrent` reference across immediate, ramping, settled, mid-ramp
   retarget, mute, and unmute blocks.
3. Preserve selected PostFader observations while pairing where the existing split
   pair supports them. Separately exercise demonstrated existing eligibility-barrier
   fixtures for bank and scalar execution, such as a compiler-proven buffer alias or
   incompatible boundary. Prove the compiler declines each candidate, the records
   and state progress through separate processors, and endpoint/reference state plus
   PCM bits remain equal. Do not assume an observation declines pairing or weaken an
   observation to obtain a pair.
4. Preserve one-claim-per-block and after-claim publication behavior, FIFO/late
   reporting, atomic batch injection, cancellation exact-once/token/generation
   behavior, sticky-fault retention, cap preflight, resource accounting, allocator
   liveness, and zero render allocations/frees. Re-run the complete #576/#579/#580
   endpoint suite, including native-bank and forced-scalar cases.
5. Run two direct red mutations against the exact claims and restore source: force
   the endpoint back to `Concurrent` so the selected-pair witness fails; then make a
   temporary, uncommitted change to one arithmetic operation in the selected bank or
   scalar pair processor in `crates/builtins-compiler/src/lib.rs` so the
   endpoint/reference bitwise equivalence gate fails. Record the named operation,
   failing assertion, restored diff, and clean status. A mutation that fails only a
   prose, source-text, or construction assertion is insufficient.
6. Pass focused host-core tests in debug and release, strict Clippy and rustdoc,
   formatting and diff checks, workspace/host policies, CI routing checks, native
   x86-64-v3 compilation, and Wasm scalar/simd128 compilation. Record the exact
   commands, statuses, mutation outputs, and restored-tree proof.

## Realtime and correctness invariants

Render remains allocation/free, lock, syscall, I/O, logging, and unbounded-loop free.
The endpoint continues to inject one complete claimed batch before graph arithmetic;
no raw producer can refill while the graph executes. Pairing only regroups the
existing fader and matrix owners and must preserve each lane's arithmetic, smoothing,
state, observations, fault behavior, latency, and ordering bit for bit. The acked-batch
question remains explicit: admission cannot acknowledge work that later drops, and
pair selection cannot create a second consumer or terminal authority.

## Workflow and completion

Attempt 1 starts only after Astra LOW approves this scope against exact delivered
main and the existing pair factories. Each coherent exact-path tranche is committed
before another implementation pass. Astra LOW adversarially reviews every source
checkpoint and the final integrated head. This successor has at most three attempts;
after three failures root records a hard stop and creates a newly bounded successor
instead of retrying.

On source PASS, root rebases or merges current main, obtains an exact-head Astra LOW
PASS, runs proportional local gates, pushes once for required pull-request
qualification, and asks lane B to qualify and pin an AudioWorklet artifact only if
the source changes its bytes. After merge and successful post-main qualification,
root synchronizes and closes #587 and updates #444/#559/#560. Closing #587 advances
RT4 but does not close #444 or RT4: one concrete lifecycle publication/cancellation
operation remains required.

## Attempt 1 review

Luna HIGH source checkpoint `0b84ecbce4a4bd31aefa9bfb2e12810027ca1cfc`
correctly selects `BetweenRenderCalls` only for the prepared endpoint while preserving
raw `Concurrent` preparation. Astra LOW returned **FAIL** with two attempts remaining.
The selected-pair fixture covers only one block, uses cumulative witness values, does
not compare paired/reference state or exact record/member counts, and uses the private
native constructor. Attempt 2 must extend that fixture across the frozen immediate,
ramping, settled, retarget, mute, and unmute schedule; capture owner-specific witness
snapshots/deltas; assert exact drains/members and applicable state; and exercise the
public native constructor. Existing PostFader tests already prove genuine bank/scalar
decline and need no duplicate fixture. Full verdict:
`docs/audits/587-attempt1-review.md`.

## Attempt 2 implementation record

Luna HIGH/XHIGH corrected only the selected-pair proof and root checkpointed the
coherent source as `751c6f3854c79e47e239e7b6912aa930b998b209`. Native execution now uses the public
endpoint constructor; forced scalar alone uses the private backend selector. Both run
against independent `Concurrent` references through immediate, ramping, mid-ramp
retarget, settled, mute, and unmute blocks. Per-owner witness resets isolate every
render, PCM bits match each block, record drains are exact, preparation and process
cohort/member counts match, and the stable `t2` scalar state words match without
trace overflow. The accepted PostFader decline and mutation evidence are unchanged.
Six endpoint unit tests, fourteen integration tests, the focused release test,
strict Clippy, rustdoc, formatting, and diff checks pass. Full record:
`docs/audits/587-builtin-endpoint-pairing-attempt2.md`.

## Attempt 2 review

Astra LOW returned **PASS** at exact clean upstream head
`a439cfa5c3425a2b1d6f84429f5b30ffb8f524da`. The public native and private forced-scalar
paths cover all six transitions with bitwise reference PCM, isolated exact dispatch
and drain accounting, and addressed scalar state. Existing PostFader declines and
both mutations remain valid; routing, cancellation, realtime, resource, public API,
and frozen-path behavior is preserved. Independent full host-core debug/release,
strict Clippy/rustdoc, formatting/diff, workspace/host policies, CI routing, native,
and Wasm checks pass. Two attempts were used. Full verdict:
`docs/audits/587-attempt2-review.md`.

## Artifact qualification and review

Integrated source head `ab3766caef34bcb035d7394224b0ccff1ea0be2d` produced AudioWorklet
candidate `39ebe7cd3f71f34ab11260f27fa1eaad281dd61642c50d9ed6210e703d95dd55`.
Lane B qualified and pinned that candidate at exact commit
`71c085ec18d7a8604c3e8d12ae11fe831737da5b`; the commit was cherry-picked onto this
delivery branch as `096e304b0585bfdcae1839aa3df1e878e4cc9e3d`. Pre/post six-file
identity, static/object/ABI/resource/native-PCM checks and 26 mutations, SDK 11/11,
Chromium 151, Firefox 153, WebKit 26.5, matrix generation, evidence leak, and
checksums pass. Astra LOW independently reviewed and reran the proportional gates and
returned **PASS**. Full evidence is under
`artifacts/issue587-artifact-qualification/`; review:
`docs/audits/587-artifact-review.md`.
