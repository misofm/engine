# Reduce general graph fan-in in one output traversal

Astra ready-to-number scope, inspected main `d8304664e8015e764285b55837c2970577abbc51` and the exact #349 RT-3 inventory. Queued behind accepted and integrated #419 source because graph arithmetic/tests overlap. Do not edit or broaden active #419. Root must number/synchronize this issue and freeze the integrated base before assignment. No implementation, Git, Cargo or timing was performed in this brief.

## Product outcome and premise

`crates/graph/src/runtime.rs::reduce_plane` still performs sum2 over the complete output and then one full sum_into pass for every additional input. This is the general reduction, independently useful outside RT-2 folded cohorts. For N>=2 contributors, its source-level per-word operand loads are 2N-2 and output stores N-1. The replacement still reads ALL N contributors but writes the output once per word. Do not claim zero contributor loads, literal machine instruction counts or a guaranteed timing win.

Smallest closable slice: change ONLY the >=2 general reduction to vectorize across frames while accumulating all input contributions in the original order in an owned register value, then store once. Retain 0-input zero-fill and 1-input copy/in-place-no-op arms exactly. Arbitrary graph fan-in remains supported within configured resources; no bank-width contributor limit, MAX_TRACKS, chunked subgroup subtotal, per-render allocation or prepare-time per-edge pointer cache.

## Complete safe ownership strategy — no new arena API

A general simultaneous read-many/output-borrow API is unnecessary. Existing `ArenaLease::read` returns a shared slice; load one frame vector into an OWNED `Lane` value and end that slice borrow. Sequentially load each subsequent contributor into an owned value and update the accumulator. Only after all contributor reads for that frame vector have ended, call `ArenaLease::write` and store the accumulator. Repeat for each independent frame vector and then scalar tail.

Conceptually, for one frame-vector offset:

```
acc = L::load(lease.read(plane, first)[offset..])
for input in remaining inputs in their existing order:
    acc = acc.add(L::load(lease.read(plane, input)[offset..]))
acc.store(lease.write(plane, out)[offset..])
```

This is sequential safe borrowing through the already-audited arena. There is never a live shared input slice when the mutable destination slice is formed. The accumulator owns values, not references. Repeated shared source IDs are legal and contribute repeatedly. Muted sources continue to resolve through the existing read API to silence. Do not resolve raw arena addresses, keep a reference across the store, cache muting state, bypass access policy, or add unsafe. No engine source/API or allocation shape change is approved or needed.

The existing private prepared-input invariants still establish valid plane/output access and legal reduction inputs. Preserve the legal one-input self-alias NO-OP explicitly, including the existing muted-self behavior; do not replace it with a read/copy that would change that behavior. General output/input alias combinations rejected by the old prepared contract are not newly admitted or used in tests of the old unsafe simultaneous-borrow methods. Inspect and cite the bind/program output/input invariant at assignment. The sequential strategy itself creates no overlapping references, but that does not authorize a new graph alias policy.

Repeated lease access/muting checks occur per frame vector and contributor rather than once per whole input block. This is the explicit simplicity/performance tradeoff. Record it as a possible residual cost, measure descriptively once, and do not add a pointer-access architecture to chase a timing number. If this strategy cannot satisfy existing invariants or a necessary access API change is discovered, stop before code expansion and amend the numbered issue with a concrete ownership proof. A new arbitrary-fan-in borrowed-view API is architectural scope, not an implicit retry permission. No Class B arithmetic change is authorized.

## Exact source scope

- `crates/graph/src/runtime.rs`: keep the public/private graph interfaces; introduce at most a private generic >=2 reduction helper parameterized by `Lane`, dispatched with existing `FrameLane`. Zero/one arms unchanged. Use existing Lane load/add/store vocabulary, not intrinsics or a new public kernel taxonomy.
- Existing inline graph tests and `tools/console-workload/tests/chain_shape.rs`: focused independent arithmetic/ownership/plumbing coverage only.
- Existing `crates/graph/tests/rt1_direct_bank_alloc.rs`: minimally extend the SAME isolated test's prepared workloads to exercise repeated general reduction with allocator liveness/thread-scoped zero counters. Retain direct/folded proof already present after #419; no second parallel test racing its global mode, new allocator, or ordinary-unit allocator attachment.
- Numbered evidence/spec, one fresh matching arm/usage in existing runner/operator preflight, and the same immutable-source worklet pin/publisher/current-ABI/generated browser evidence files required by #399/#419 when actual Rust changes regenerate the artifact.

No lane production change is needed: `lane::Lane` and frame vector types are already dependencies. Do not reuse #419's bounded 1..8-contributor cohort kernel by subtotaling arbitrary inputs; that changes D9 association. No engine change, new dependency, generic reducer framework, render scratch, fold eligibility, routing/meter/automation change or deferred AArch64 qualification.

## Frozen arithmetic and independent tests

For N>=2 each output sample computes `(((x0+x1)+x2)+...)` using existing Lane::add semantics, independent vectorization across frames and scalar tails. No zero seed, tree reduction, subgroup sums, fusion, sanitization, flush/canonicalization, route transform change or reordering. The first two operands must retain their order as well as the later additions. No tolerance relaxation.

Freeze a TEST-ONLY reference implementing the old algorithm explicitly with existing `sum2_block` followed by `sum_into_block`, plus original 0/1 behavior. Do not let the #419 folded-oracle fixture call the newly changed reduce_plane and thereby compare two changed implementations as if independent. Redirect its reference through the frozen old primitive sequence or keep the existing explicit independent oracle if #419 already supplies one. This oracle refactor is required evidence within graph tests, not permission to change #419 production code.

Required representative gates:

1. Fan-ins 0,1,2,3,8,9,64 and at least one larger-than-64 count; repeated source IDs; silence ID and muted inputs; unmuted/muted legal one-input self-alias; independently asymmetric L/R planes. Poison destination and preserve unrelated buffers. No invalid-lease UB reproduction.
2. Frame lengths 1, below vector width, exact width, width+1, several vectors plus tail and128; instantiate scalar, Simd4 and Simd8 against the same old-width oracle. Existing hostile corpus includes signed zeros/subnormals/infinities/NaN payloads, with bitwise comparisons under the repository's actual FP environment and matching execution-arm behavior. Do not invent cross-platform NaN guarantees beyond existing primitives or weaken an observed identity mismatch.
3. Strong finite ordering witness `[16777216,1,-16777216]` gives0 with the old left association and1 when the small term is accumulated after cancellation. Add many-input sequences whose 8-contributor subgroup subtotal changes the result; explicitly compute both wrong and old outcomes in the test and require them to differ before asserting the implementation matches the old one. The oracle is the old primitive sequence, not the new helper against itself.
4. A private graph/prepared-program fixture establishes the actual nonfolded multi-input path, stable input order and no output alias under normal preparation. Keep prior graph redirect/PDC/observation/route-fold decline tests and #419 first/continuation cohort witnesses. General reduction must not invalidate the independent folded oracle.
5. Repeated real prepared graph renders through the general fan-in path perform zero audited allocations/frees after preparation; positive allocator liveness is required. Source/mechanism evidence shows output write only after the inner contributor loop, once per vector/sample. A callback/access counter in a small test seam is optional if it improves discrimination, but no new permanent telemetry or instruction-count byte gate is required.

The complete source slice should remain half-day bounded using the existing API and fixture corpus. Do not attach extra graph features or a new large qualification corpus to it.

## Actual existing workload and one descriptive measurement

The standing `SixtyFourTrackPlumbingOnly` console workload is the appropriate named row, not an assumption that every console row reaches reduce_plane. `tools/console-workload/src/lib.rs` explicitly binds NO builtin bank in this arm; the existing chain-shape test proves zero bank slots/round-trips, so routes cannot fold into a bank. Confirm after #419 integration that its prepared master reduction still has the intended64 ordered inputs. Preserve its output digest and frozen workload bytes. Strengthen only the existing test assertion if needed; no new timing workload or schema is required.

Register one fresh issue-owned runner/preflight namespace; preserve consumed #399/#415 namespaces. Freeze candidate, existing46-record workload/fixtures/floor/validators and actual profile/binary. Root completes non-timed committed-head preflight with zero launches, builds the exact runner profile before readiness, lets other work settle, and permits exactly one controlled invocation: one warmup and two measured rounds. Unchanged load ceiling0.50, cooldown60seconds, affinity/sibling checks; no uncontrolled override. Inspect the two plumbing p50 rows and output/structural identities explicitly. Other folded rows are contextual and cannot independently prove the general reduction mechanism. Compare historical timing only with stated comparability limits; no causal speedup from uncontrolled/different-profile evidence and no fabricated cycles.

Run unchanged complete record and aggregate validators, preserve raw/stderr/disposition/identities. Prelaunch refusal consumes the invocation and is preserved; no automatic retry or successor chain. A post-workload tooling failure preserves raw evidence and moves repair to an explicitly scoped tooling issue. Descriptive timing is not permission for performance retries or arithmetic changes.

## Delivery gates and workflow

Freeze main baseline/candidate workspace counts, focused graph/lane-applicable and console chain/identity suites in debug/release, realtime42/12 policy/mutations (or the accepted current marker count), lane/graph/workspace/audit policies, fmt/diff/clippy and supported Wasm/artifact/static/resource/browser gates. Keep native AArch64 deferred. Use isolated targets and no concurrent Cargo in one target. Actual artifact consumers follow the existing immutable-source-candidate convention; no unrelated publication.

Root synchronizes numbered issue/spec before implementation and owns checkpoint/pushes. Astra scopes/reviews; Luna attempt1, Sol only after FAIL, maxthree attempts then hard stop/rescope. No overlap with active #419; wait for its accepted integrated source and preserve its independent oracle before changing general reduction. After semantic PASS, finish frozen qualification, actual PR Astra review and required CI before merge/closure. Broader #349 remains open for its other findings.

## Numbered queue

Issue #420 owns RT-3 in #349. Astra supplied this approved scope; root synchronized it before implementation. It remains queued until accepted and integrated #419 source. The actual implementation base and unchanged workload/invariant checks will be frozen at assignment. No implementation is authorized to overlap active #419.
