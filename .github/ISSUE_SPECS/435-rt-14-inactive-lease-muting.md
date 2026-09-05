# Remove inactive lease muting from sequential render reads

**QUEUED behind merged #420 and Astra confirmation of the actual integrated base; no implementation assigned.** Read current main1af76181490a623675960c244a6c677c06aae745, Sol's inventory, local specs and remote RT-14 issue search. No dedicated implementation owner was found: #349 is the open finding; returned #371/#83/#406 records are not this repair. Recheck after #420 merges because its accepted tests deliberately exercise muting. No legacy source, builds, tests or timing inspected/executed.

## Premise and smallest outcome

The premise remains real. ArenaLease stores `wave` and a per-buffer access byte with WRITE and MUTED bits. Public wave/set_muted/is_muted have no current production callers; observed lease muting calls are engine tests (plus the accepted #420 graph tests on its pending branch), unrelated builtin fader is_muted calls are not lease users. Every read/write_read/write_read2/copy input resolves through effective(), whose access-byte load selects silence when MUTED. Production graph builds one lease at wave0 over its sequential arena.

Smallest issue title: **Remove inactive lease muting from sequential render reads.** Delete runtime lease wave storage/accessor and scheduler-only mute state/accessors/selection, retain the write-access table and builder's actual ownership/order validation. This is one engine module plus directly affected graph tests/docs: independently useful and half-day bounded. It does not depend on #429/#430 arithmetic integration, but overlaps the arena behavior exercised by #420 and must wait for its merged source. Root chooses later feature sequencing; do not run it alongside active #429.

## Explicit numbered ruling required before coding

These are public Rust lease methods, not ABI wire symbols. No repository production use is not proof no external caller exists. The numbered issue must expressly authorize removing scheduler-era `ArenaLease::wave`, `set_muted`, `is_muted` and their semantics as obsolete under the approved single-render-thread architecture. Do not silently make muting a no-op, hide it behind cfg(test), keep a separate graph-only compatibility mechanism, or claim the old public behavior is preserved. Root's user authorization to resolve blockers permits preparing this concrete ruling; actual synchronized issue must state it before assignment.

**Do NOT remove all occurrences of wave.** PendingLease::wave and ArenaLeaseSetBuilder::lease(wave, writes, reads) currently prove that a foreign read belongs to a strictly earlier producer, rejecting same/later-wave reads. That is a real construction invariant and shared API shape, even though production currently uses one lease. Retain builder argument, field, validation/error variants and their tests. Removing the returned lease's redundant runtime wave copy is sufficient. No builder redesign, worker resurrection or compiled track limit.

## Safety boundary that must not be missed

The old effective() does more than mute: `self.access[index]` supplies a RELEASE bounds check before an unsafe arena slice is formed. Its debug_assert alone is insufficient. Replacing effective(buffer) with `buffer as usize` plus debug_assert would remove a real guard and potentially expose out-of-range safe calls to unchecked storage. Preserve that existing release check explicitly, for example with a bounds-checking access-table index whose byte value is not used, returning the unchanged index. The compiler may eliminate an unused byte LOAD but must retain any unproven bounds check. No unchecked index, new unsafe, panic-policy exception, silent malformed-ID-to-silence fallback or alternate plane validation contract.

Retain WRITE bits/table, writes(), checked_write semantics, silence ID0, buffer allocation/numbering, all read/write/stereo/copy/sum alias checks and output non-alias contract. No simplification of write_read2's repeated shared-input legality. Existing disjoint-arena unsafe storage remains justified by current ownership/ordering rules; only input index selection changes. Do not broaden this into repairing unrelated plane/lease construction API contracts. Never execute an invalid old lease or pointer as a test oracle.

## Exact allowed scope

- `crates/engine/src/realtime/disjoint.rs`: runtime mute/wave removal, bounds-preserving read-index helper and directly corresponding docs/unit tests. Keep builder proofs and enum/API unaffected beyond explicitly retired methods.
- `crates/graph/src/runtime.rs`: ONLY the #420 test cases referring to removed lease muting. Keep silence ID, repeated inputs, unmuted legal self-alias, asymmetric planes, poisons/sentinels, old-width oracle and ordering tests. No production reducer change. Remove the obsolete mute assertions explicitly; do not rewrite historical evidence to pretend they never existed.
- `docs/REALTIME_DEPENDENCY_POLICY.md`: concise current scope/invariant clarification if needed, plus numbered evidence. Update disjoint module's obsolete scheduler/deadline description and obsolete references from safety comments carefully: production execution is sequential; any retained general lease-set use still owes its existing ordering/nonconcurrency conditions. Do not assert a removed scheduler currently discharges them or weaken the safety proof.
- Existing isolated graph allocation fixture may be rerun without modification. No builtins/compiler/rack changes, access cache, allocation representation optimization, public borrowed-view API, new dependency or telemetry.

## Objective gates

1. Before editing, checked complete workspace source discovery and exact receiver/type resolution establish no production lease wave/mute caller. Capture statuses, distinguish tests/builtins methods, include post-#420 graph test references. Any new actual production caller requires rebrief, not automatic deletion.
2. Existing builder tests still reject overlap, silence writes, unknown IDs, same/later producer reads and preserve permitted earlier/self reads. Existing multi-lease concurrency tests remain evidence for their retained disjoint ownership conditions, not an enabled render scheduler.
3. Existing arena primitive tests plus a small valid-lease representative set preserve silence reads, repeated shared inputs, exact signed-zero and finite outputs, copy/no-op behavior and unrelated sentinels. Retain a RELEASE malformed buffer-ID refusal check at the old protected read-index boundary using safe catch_unwind around a valid lease. Verify no unsafe slice is formed before the check; do not add an invalid-plane test or a test whose old behavior was UB. Existing write-side checks retain their prior scope.
4. Actual graph debug/release tests and #420 old-width/association/64-input proofs still pass, with only the expressly retired muting-specific legs removed. Repeated prepared graph render retains positive allocator liveness and zero allocations/frees in the existing isolated integration binary. Preserve PDC, fold/redirect/observation tests and PCM identities.
5. Mechanism evidence: production input resolution no longer reads/tests MUTED or conditionally substitutes silence, and returned lease no longer stores wave. Existing bounds and write-access protection remain. A small source/codegen observation may substantiate this exact change, but do not create a brittle general disassembly gate or claim guaranteed instruction count/performance. Reintroducing old mute selection must contradict the recorded mechanism; timings alone cannot prove it.
6. Relevant engine/graph focused tests, realtime/graph/lane/workspace policies, fmt/diff/clippy and required immutable workspace/target/artifact delivery follow the existing workflow. Public Rust API retirement is documented; ABI identities unchanged. Supported-target and generated artifact consumers use the established immutable-candidate convention; native AArch64 remains deferred.

## Qualification, closure and roles

Keep descriptive measurement separate from proof of the product mechanism. An existing console plumbing workload reaches these arena reads; use the established fresh console namespace/preflight contract only after source acceptance and exact workload/validators/profile/binary freeze. One controlled invocation,1 warmup/2 measured rounds, unchanged readiness and no retry; preserve refusal/raw failures. If runner work exceeds simple registration or remains faulty after one bounded correction, move it to an explicitly numbered tooling successor rather than inflate this engine slice. No timing is authorized by this scope draft.

The numbered decision must say that historical #420 muted-input behavior was intentionally retired by this successor; its original tests/evidence remain historically true at that source. No retroactive rewriting of sealed artifacts or earlier PASS claims. Once the runtime field/select removal, preserved safety/identity gates and required qualification are delivered, this issue can close RT-14; other #349 findings and RT-4 children stay open.

Astra approves numbered scope/base, Luna one coherent attempt, Sol only after FAIL up to two retries, hard stop after third failure. Root owns issue sync, isolated base, checkpoints/pushes and actual-head PR/required-CI merge. No new issue was created and no implementation started in this read-only scoping pass.

## Explicit public Rust API retirement ruling

Under the user-authorized #349 audit repair and the approved single-render-thread architecture, this issue authorizes removal of the obsolete public Rust `ArenaLease::wave`, `set_muted` and `is_muted` methods and their runtime semantics. This is an intentional public Rust API retirement, not behavior-preserving compatibility. Wire and C ABI identities do not change. Historical #420 muting-specific behavior is intentionally retired by this successor; its original evidence remains valid for its original source. The builder wave argument, ownership/order validation, write-access policy and release-mode read-ID bounds checks remain mandatory. If complete post-#420 discovery finds a production caller, stop and rebrief before implementation.

This ruling makes the proposal concrete under the existing task authorization; Astra must approve the numbered scope and root must freeze the merged prerequisite base before Luna assignment. #429/#430/#431 remain independently queued, and only one launch-critical feature may be implemented at a time.

## Astra numbered scope approval

# Astra #435 numbered scope approval

**PASS for planning checkpoint `af187c3ef322fff7378c6efdfd996c176ae19ceb`.** Remote #435 is OPEN with the matching title “Remove inactive lease muting from sequential render reads”. The numbered text retains the approved RT-14 scope; changes are the numbered title/queue and explicit retirement ruling.

The ruling authorizes the exact public Rust runtime API/behavior retirement openly, without implying compatibility or changing wire/C ABI identity. It expressly preserves builder wave arguments and ownership/order validation, write-access policy and release-mode read-ID bounds checks. The distinction between dead returned-lease wave/mute state and still-operative builder proofs is intact. Historical #420 mute-specific behavior is intentionally superseded only at the successor source; original evidence is not rewritten.

No scope expansion or material correction found. Freeze the post-#420 merged base and repeat complete production-call discovery before assignment; any live caller requires rebrief. Do not overlap #429 or another launch-critical feature. No implementation, benchmark or new architecture is authorized merely by this planning approval. Existing source/test/qualification and exact-head PR/required-CI gates remain.

Read-only local diff/spec and remote identity inspection. No tests, Cargo, timing or repository/GitHub mutation.

## Post-prerequisite base readiness

Root integrated delivered main `4b352b36ba33334ea2e0c6847c0e3ecf6e8ab33a` after #420 and #429 closed with exact-head Astra PASS and required CI. This is a scope-readiness checkpoint only: Astra must repeat complete production-call discovery and confirm the frozen API retirement/release bounds proof before Luna assignment. #430 ownership/admission design proceeds separately; no implementation overlap is authorized.
