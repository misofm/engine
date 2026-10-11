# C ABI V1 native qualification

#1033 (owner ruling R2, 2026-09-28) removed the native PCM reference runner, its fixtures and its
checks; the C ABI stays as the mobile interface. The runner rows below are historical.

Issue 114 qualifies the joined, accepted Issue-116 native PCM runner and Issue-121 C ABI product.
It does not change or reseal either product. The accepted C header, CAPI/protocol implementation,
runner, runner contract, portability checks, Cargo lock, session fixture, and native runner corpus
were pinned by `fixtures/capi-qualification/v1/AUTHORITIES.sha256`. #1029 deleted that ledger,
which no code read and which named checkers #319 had already deleted; the files named below are at
<https://github.com/misofm/engine/tree/5379e46ca3b349b9d277d642c008bb7a9643fb76/fixtures/capi-qualification/v1>.
The C ABI's live gates are `scripts/check-capi-abi.sh`, the `capi` tests and `audit capi`.

The source authority is clean-main commit `feb039765271ca62b0c905004689b88ad92df65b`, tree
`e3e11c343c6f6a5b5b380abe03c0431c6fe81579`. Issue 116 is bound to commit/tree
`45f8f5af8bdd578b5ccb27fdb787f7a663c39818` /
`7e0a7b7d48362c9b9eaa15b1cfce7180c935c5b5`; Issue 121 is bound to commit/tree
`a9a975d8f679707701cc60ad102c817eb54c3082` /
`16728c5ea434dde1a75bdd4500568db8c283a2ca`. The joined header SHA-256 is
`83880c2fd7b5bc835425a5a64cae19c8a0bba17f49b4802b4033a8e7dfeac37c`, and the joined lock
SHA-256 is `c89b195f0d31ad21852d0a931023c70e1eb4a0caa534bfd6e1692c1e1178fd52`.

## Frozen preflight

Tool availability was frozen before the qualification build. The Linux x86_64 host has Rust and
Cargo 1.97.1 with LLVM 22.1.6, GCC/G++ 13.3.0, GNU binutils 2.42, Python 3.12.3, Bash 5.2.21,
strace 6.8, and jq 1.7. Exact paths and installed Rust targets are in `TOOLCHAINS.tsv`.

Cross-target outcomes are candid `UNAVAILABLE`, not compile failures:

- Windows GNU has an installed Rust standard library but no MinGW C/C++ compiler, linker, or
  object inspector.
- Windows MSVC has neither its Rust target nor `cl`, MSVC `link`/`lib`, or `dumpbin`.
  `/usr/bin/link` is GNU coreutils and was explicitly rejected as an MSVC tool.
- macOS x86_64/AArch64 and iOS AArch64 device Rust targets are installed, but `xcrun`, Apple SDK
  linkers, `otool`, and `lipo` are absent.
- The iOS AArch64 simulator Rust target and Apple SDK tools are absent.
- Android AArch64 has its Rust standard library but no Android NDK Clang/linker or LLVM object
  inspectors.

No cross row was executed or relabeled after the preflight. Only the Linux runtime was run.

## Linux artifact and consumer boundary

Fresh `target/capi-qualification/v1` staging was required to be absent. One locked release Cargo
command produced the static and shared libraries there. The accepted header and both libraries
were copied into qualification-owned `installed/` staging and hashed before any consumer linked
them. Existing artifacts elsewhere under `target/` were never inputs.

The same warning-denied source compiled as strict C11 and C++17. Each language linked and ran once
against each frozen library form. The consumer verifies version/layout constants, reserved-zero
rejection, engine/session/plan construction, source generation 1 submission, generation 2 seek and
submission, two render blocks, resource rows, malformed event lane, one-short command canary,
exact command replay bytes, empty reliable egress, and both plan/session destruction orders.
The accepted Rust exported-C regressions supply the complete 11-command, six-event, transactional
replacement, retirement/reclaim, source-preserving/source-changing, failure, replay, and lifecycle
matrix without copying protocol semantics into the qualification consumer.

A structural transaction keeps every unchanged source playing (#1273, slice 4 of #1269). The
replacement plan is prepared from the newest plan's state inventory and committed session model,
with no ring for a source whose declaration and ring configuration are unchanged; at its swap block
it takes that source's ring from the plan it displaces, with the queued PCM, the generation and the
read position. The source's producer moves to the replacement's source set right after the protocol
commit succeeds, which is the last fallible step, so a refused transaction moves nothing. From the
commit on, `miso_engine_v1_source_submit_planar_f32` and `miso_engine_v1_source_seek` address the
newest committed session: a host keeps feeding an unchanged source contiguously and must not reseek
it. A removed source is refused as `source.id.unknown`, and PCM already accepted for it is discarded
with its plan. An added source starts at generation 1, frame 0, until the host seeks it. A source
whose declaration the transaction changed restarts the same way, in a new ring at generation 1,
frame 0; PCM accepted for it before the commit is discarded with its plan, so the host restarts
its feed. For a session whose unchanged paths hold no DSP state, the output across the swap is
bit-identical to the post-edit session compiled fresh and fed the same PCM from frame 0
(`runtime::tests`' `a_c_abi_structural_transaction_keeps_the_source_playing`). The replacement's
resource report counts the rings it carries and its carry program, as the plan will own them once
active; the double-live admission counts a carried ring once, with the plan it displaces; and
capi's epoch row charges the state inventory each epoch keeps.

Submix strips are editable through the same `SESSION_TRANSACTION_APPLY` transactions as track
strips: opcodes `0203`-`0211` take a submix ID as their strip ID (#1204). Every such transaction is
structural, compiling and swapping a replacement plan, until *Deliver value-only send and
submix-strip edits to the running C ABI plan* (#1225) lands.

A caller bounds submix strips through `miso_engine_v1_compile_limits.maximum_submixes` (#1206), an
in-place V1 amendment: the word is the former `reserved[0]`, at offset 176, followed by
`maximum_vcas` at 184 (#1243, below) and two reserved words at 192..207, so the struct stays 208
bytes and no symbol, size or `ABI_VERSION` changes. Zero means "use `maximum_tracks`" -- not "no
submixes" and not "unbounded" -- which is the value every caller written before the word was named
already passes. A nonzero value is the bound itself; a session over it refuses with
`host.resource.count`. The two remaining reserved words must still be zero. For a caller written
before #1206, zero newly bounds submixes by `maximum_tracks`: between K1 (#1199-#1205) and #1206
they were uncounted, so such a caller now gets that typed refusal for a session with more submixes
than `maximum_tracks`. A caller cannot probe whether a library honours the word; a library older
than #1206 refuses a nonzero word with `RESULT_INVALID_ARGUMENT`, the safe direction.

A caller bounds VCA groups through `miso_engine_v1_compile_limits.maximum_vcas` (#1243), the same
kind of in-place V1 amendment: the word is the next former reserved word, at offset 184, and
`reserved` shrinks to two words at 192..207, so the struct is still 208 bytes and no symbol, size
or `ABI_VERSION` changes. Zero means "use `maximum_tracks`" -- not "no VCAs" and not "unbounded" --
which every caller written before the word was named already passes. A nonzero value is the bound
itself, independent of `maximum_tracks` and `maximum_submixes`; a session over it refuses with
`host.resource.count`. No released library accepted a VCA before this word existed (VCA groups
arrive in the same batch, #1240), so zero changes no previously accepted session; a library older
than #1243 refuses a nonzero word with `RESULT_INVALID_ARGUMENT`.

A host starts a stem a structural transaction added in exact time through
`miso_engine_v1_source_seek_at` (#1275, slice 6 of #1269), an in-place V1 amendment: one new
exported symbol, announced by `MISO_ENGINE_V1_FEATURE_SOURCE_SEEK_AT` (32) in the capability
report, whose feature mask becomes 63; no struct, size or `ABI_VERSION` changes. It is
`miso_engine_v1_source_seek` with an anchor: the seek's source frame enters the graph in the
block that starts at absolute render sample `anchor_sample`, which must be a multiple of the
quantum or the call returns `RESULT_INVALID_ARGUMENT` with `source.seek.anchor_unaligned`; its
thread rule, argument checks and every other result are the plain seek's. The host checks the bit
before calling the symbol, since a library older than #1275 does not export it. After the
transaction returns, the host picks `A` a few quanta past the last rendered block, calls
`seek_at(id, g, F, A)` with `F` the frame the playing stems read at `A`, then submits generation
`g` from `F`; the stem renders silence until `A` and plays `F` from `A` on, bit-identical to the
post-edit session compiled fresh and fed that PCM from frame 0 (`runtime::tests`'
`an_added_c_abi_source_starts_at_its_anchored_render_sample`). The frozen exported set is now 15
`miso_engine_v1_*` definitions (`scripts/check-capi-abi.sh`, whose self-test refuses a library
where the new symbol is only an undefined reference: its `nm` wrapper drops the symbol from a
defined-only listing and shows it as `U` otherwise, so the case also fails if the checker stops
asking for defined symbols alone).

A host learns which committed revision is audible through `miso_engine_v1_plan_watermark`
(#1314, decision 15 D15-17), an in-place V1 amendment: one new exported symbol and one new
96-byte struct, `miso_engine_v1_watermark` (`MISO_ENGINE_V1_WATERMARK_SIZE`), announced by
`MISO_ENGINE_V1_FEATURE_PLAN_WATERMARK` in the capability report, whose feature mask grows to
include it; no other struct, size or `ABI_VERSION` changes. Like `miso_engine_v1_plan_resources`
it runs on any thread, concurrently with render, and is pure: it writes nothing through the plan
handle, its diagnostic word included. It copies `(revision, first_sample, outcome_flags)` and
saturating `exact`, `transition_fallback` and `superseded` counts: the highest committed revision
in effect together with every revision before it, the render sample of the block that reported it (never early: it is the start of the first block that begins after the commit's last write, so at most one block after the submit returns (while a replacement plan is pending: the block that adopts it); a live value of that revision can apply earlier, in any block that ran between the submit's first push and its revision store, and one transaction's values can spread across those blocks until #1502, #1503, #1504, #1312 and #1345 have all landed, which together make it exact), and the OR
of `MISO_ENGINE_V1_OUTCOME_*` over the revisions the last advance covered. Submit stays
synchronous only for what can fail (validate, classify, prepare a rebuild and reserve its
credits, commit); every committed revision is pending until the watermark covers it, and a paused
host's revisions stay pending until render resumes. Each revision travels with the plan that
carries it (a revision word in each publication-mailbox cell), the control plane routes each
commit's revision to the newest pending candidate as the commit's last write, and render reads
only the word of the plan it runs, after the adoption decision and before any drain. The
watermark is a level, not a queue: render overwrites it, never waits and drops no command, so an
acknowledged edit cannot be dropped behind it. A wrong `struct_size` or a nonzero reserved word is
`RESULT_INVALID_ARGUMENT`; a copy that keeps landing inside render's publication (bounded at 64
attempts) is `RESULT_BACKPRESSURE` with `out` untouched, and the host retries. In this slice every
revision completes as `exact`; the `superseded` and `transition_fallback` producers are #1310,
#1397 and #1358. The frozen exported set is now 16 `miso_engine_v1_*` definitions;
`abi_smoke.c` checks the bit against the queried mask and queries a compiled plan with a valid
and a wrong-size struct, `header_smoke.cpp` pins the size and every field offset, and
`./target/release/audit capi` stays at 0 allocations, 0 deallocations, 0 syscalls and
`"total_violations":0` with watermark publication on (its live edits and its carrying swap
advance the watermark during the audited calls).

The engine owns no thread, so the host drives the control work between edits through
`miso_engine_v1_service(session)` (#1348, decision 15 D15-17), an in-place V1 amendment: one new
exported symbol, announced by `MISO_ENGINE_V1_FEATURE_SERVICE` (the next free bit, 128) in the
capability report, whose feature mask grows to include it; no struct, size or `ABI_VERSION`
changes. It is a session call, serialized with the others. One call runs one bounded step: it
reclaims the plans render has retired and brings the provider epochs up to render, refreshes the
provider's counters (the controller's telemetry counters today), stages the render observation,
and refreshes the counters again so a record that staging coalesced or dropped is counted in the
same call. Every other session control call (`source_submit_planar_f32`, `source_seek`,
`source_seek_at`, `submit_command`, `dequeue_event`) that passes its argument checks runs the same
step first, in place of the parts each ran before; a call refused for its arguments (a null
`source_id`, a bad struct size, an invalid lane, an empty request) returns before the step. Before
#1348 only `submit_command` refreshed the counters and the source
calls staged no telemetry. The step reclaims at most the retirement queue's capacity of plans,
because each retired plan holds one of that many retirement credits and only a publication
reserves one; it stages at most one render observation, never waits for render and never loops on
its progress, so its cost does not grow with the time since the last call. It commits and
acknowledges nothing, adds no queue, and never takes or drops a pending candidate: a scheduled
candidate stays in its mailbox cell until render adopts it, and its provider is promoted only
after. So an ack cannot precede a drop through it. Results: `OK`; a null session
`INVALID_ARGUMENT` and another handle `WRONG_HANDLE`, as every session call; an epoch
synchronization failure `INTERNAL` with the session diagnostic `capi.source.epoch`, the one the
source calls already set for it. The host's duty, in the header: a pending edit progresses only
while the host makes session calls, the same duty as draining events. A host that renders but
never calls keeps retired plans allocated, and its next structural edit waits for that reclaim; a
warm successor that misses its deadline falls back only at the host's next session call (#1358,
#1360); the watermark never covers an edit before it is in effect. While edits are pending, the host calls
`service` (or another session call) at least once per render-buffer period. The frozen exported
set is now 17 `miso_engine_v1_*` definitions; `abi_smoke.c` checks the bit against the queried
mask and calls `service(NULL)` for `INVALID_ARGUMENT`, `header_smoke.cpp` pins the bit inside the
mask and the signature, and the capi runtime tests cover reclaim by `service` alone, the step
inside a source submit, the counter refresh, a scheduled candidate kept across 1,000 calls, and
idle calls that change nothing.

The first C11-static launch found one qualification-fixture error: it attempted generation-1 seek
before the initial generation-1 submission and exited 13. No product byte or staged library was
changed or rebuilt. The new consumer was corrected to submit generation 1 first, then seek and
submit generation 2; all four consumer rows passed against the same once-built libraries. This is
recorded as one consumer-fixture correction in `QUALIFICATION.tsv`.

GNU `nm` found exactly the 14 frozen `miso_engine_v1_*` definitions (15 since #1275) in both
library forms. The object parser classifies undefined references separately; a synthetic mutation
replacing a definition with an identically named undefined reference is rejected.

## Runner and realtime evidence

The frozen runner package test command was invoked exactly once. Its 18 tests passed, including
the single test that executes all four RIFF launch rates and representative RF64, compares every
8,192-byte output SHA-256 to the independent manifest, and covers atomic success/failure cleanup
and no-clobber behavior. The runner, its fixtures, and its exclusive-output-directory contract were
not modified, retried, or described as a new Issue-116 seal.

The exported C render audit completed 100,000 calls with stable caller storage and zero allocation,
deallocation, lock, feature-detection, log, file/network I/O, syscall, unwind, or render errors.
Since #1273 the audit applies one structural transaction (a muted track and its route) after its
first call, outside the render scopes, so its second audited call is a swap block that carries the
source's ring; the tool fails unless that transaction's plan is the only one the render thread
swapped in (so every periodic live edit was applied live, #1258 D3) and its swap carried. A
separate functional one-million-block render/swap audit observed two accepted swaps, one retirement
deferral, zero forbidden-operation counters, and zero syscalls between the explicit realtime trace
markers. Neither audit selected a benchmark mode or recorded durations.

The exact matrix was `fixtures/capi-qualification/v1/MATRIX.tsv`. `ARTIFACTS.tsv`, `SYMBOLS.tsv`,
`AUDITS.jsonl`, `QUALIFICATION.tsv`, `CONSUMER_RESULTS.tsv`, `RAW_EVIDENCE.tsv`, `GATES.tsv`, and
`TOOLCHAINS.tsv` contained its independent evidence, and `EVIDENCE.sha256` bound those files. The
semantic checker independently pins every artifact size/hash, symbol set, audit field, result
counter, consumer exit/binary/library binding, raw-log hash, and strict gate; updating the checksum
manifest cannot bless correlated fabricated data. Preserved-stage mode additionally checks the raw
manifest, logs, audit JSON, binaries, libraries, symbols, and armed syscall trace in place. The
checker mutations cover each evidence family as well as authority drift, target omissions,
undefined-reference false positives, fabricated tool availability, timing surfaces, stale staging,
and generated source-tree artifacts.

Final prohibited counters are: benchmark 0, timing 0, playback 0, listening 0, browser 0, and
device 0.

## Issue 369 control-provider refresh

IO-4 replaces the C ABI's conformance-only `MockProvider` with the opt-in
`host_core::SessionControlProvider`. The production parameter catalog is snapshotted directly from
each accepted `EffectPreparedEntry`'s `metadata.descriptor` and
`bank_preparation.initial_values` before graph lowering consumes those entries. Parameter state
therefore uses the exact prepared values; automation domain
checks therefore address real revision-scoped handles. Current/effective sample reads the active
plan's existing release/acquire next-sample publication, while transport remains endpoint-local.
Protocol telemetry counters and the existing bounded CAPI render-diagnostic slots feed the
provider's counter and diagnostic pages. The three provider-owned counters retain an independent
three-slot minimum even when the frame-derived telemetry configuration capacity is smaller.
Candidate catalogs allocate before structural commit and are included in double-live resource
admission. Host-core's default feature graph remains protocol-free; only capi enables the optional
`control-provider` edge.

`resource_lifecycle` checks these charges against the allocator (#1060). Its counting allocator
observes a C ABI compile and a replay of its host-core half owner by owner, and nothing is taken
from the accounting it checks. What capi allocates itself, plus the observed effect live-control
producers (#1263), strip live-control producers (#1256), source producers, parameter catalog and
plan state inventory (#1273), must equal `capi_retained_bytes` to the byte.
The launch native-effect registry is built once per process (#1469) and its bytes are
process-level: no plan or engine is charged for them. A plan is charged only what it retains, so a
factory `Arc` the registry owns is charged to no plan (the plan's clone allocates nothing); on the
nine-track EQ reference session this removes the EQ factory's 16-byte allocation from
`capi_retained_bytes`.
The session store must fit its compiled-model estimate, and the prepared plan its engine rows (a
bound; see the test). The canonical JSON is charged once, with the compiled model in the graph cap:
capi's epoch row no longer charges it a second time. The double-live admission is derived from the two live reports
and the owning crates' resource reports rather than from a hand-maintained layout mirror.
The C response vectors now pin session-derived metadata/state and registered telemetry
counter rows. `MockProvider` and `MockProviderConfig` are absent from a normal protocol library
build and available only to unit tests or consumers explicitly selecting `protocol/test-support`.
The exact AudioWorklet rebuild remains protocol-free but changes crate identity because host-core's
declared feature surface changed. The subsequent #371 marker-only integration was rebuilt
and reproducibly qualified as `a89c9606bfa72d69ced42b606cc4b7000d1b53f2b419b12ec63649a385b3eaf1`.
The RT-1 (#399) artifact from source candidate
`e46bc0d1a7917de8c65204cdee931877aea671d8` has SHA-256
`60c23ee23e7f16c1f71c503baa07a462a8ce94c5287bec4580060e27a4651503`; its reproducibility and browser evidence are recorded in #399.
The RT-2 (#419/#422) artifact from source candidate
`0a0e39e42e4ae2585d5f5ee507a4cb9aaf7b741a` has SHA-256
`518b5aa864c0a825cd324112b24270a7e0714fc63db6bd1029779f21066ea9de`.
The independent rebuild, static/resource checks and three-browser matrix passed;
retained workspace and descriptive measurement delivery are recorded in #419.
The RT-3 (#420) artifact from source candidate
`51e2aed211b30523076e0e8dd07973b13b57dc11` has SHA-256
`24f81af304e541ba0e734de5c7a3dc5221e71fa4de73f2545edea3c2960761fe`.
Independent builds, static/resource/mutation checks and all three browser engines passed;
workspace evidence belongs to #420; its uninvoked descriptive measurement is tracked by #436.
The RT-4 public full-chain (#429) artifact from source candidate
`e4bcaa2feae13c9f016bb7b2e1eaff8bd7314547` has SHA-256
`10b0581f72d921b520e4066b82dc32cb7bea90b757c20ccca3dfc52cf7b9e098`.
Independent builds, static/resource/mutation checks and all three browser engines passed;
workspace and supported-Wasm evidence were retained in `artifacts/issue429-qualification`,
which #1030 removed in `df8cebb3`; git keeps it
(`git show df8cebb3^:artifacts/issue429-qualification/<file>`).
Live integration and descriptive full-chain measurement remain in #430 and #431.
The current RT-14 lease cleanup (#435) artifact from source candidate
`69fd0bfb0504075db4d302df08ff480faab4102e` has SHA-256
`766848a4688b2ec34c96e81c243286216a7d7e647b6b42f842c0f85a654fc326`.
Independent builds, static/resource/hermetic checks and all three browser engines passed;
workspace and supported-Wasm evidence were retained in `artifacts/issue435-qualification`,
which #1030 removed in `df8cebb3`; git keeps it
(`git show df8cebb3^:artifacts/issue435-qualification/<file>`).
This is an intentional public Rust lease API retirement; wire/C ABI identities are unchanged.

Refresh gates: `cargo test -p capi`; `cargo test -p capi --test resource_lifecycle`;
`scripts/check-capi-abi.sh`; `scripts/check-abi-layout-v1.py`; and `cargo test --workspace` against
both the issue worktree and `origin/main`. Exact outcomes and the worktree comparison are attached
to issue #369's implementation pull request.

## Live fader/matrix qualification candidate (#430/#459)

The fresh SIMD AudioWorklet digest build from immutable source candidate
`7951736605fa64870bc1d91342d00d5fdb6417c5` produced SHA-256
`a08a868cf1b62bb466a8fa5b826b214fa708265669fc730398706c869c9e43bd`. Independent rebuild, static/resource gates, hermetic worklet tests, and
Chromium/Firefox/WebKit qualification with matrix checks passed. The resource
expectations include one additional eight-byte graph owner on Wasm; PCM digests are unchanged. The initial
builder invocation refused a missing output directory before compilation (exit 2);
the corrected invocation with an existing empty directory completed successfully.
Historical artifact and qualification records above retain their original identities.

## Live fader and matrix lanes on every plan (#1256)

Every C ABI plan, at compile and at every structural replacement, carries two live lanes per
strip, tracks and submixes alike: a 16-record fader/mute ring and a 16-record matrix/pan ring
(#1053 D4). The rings are charged in `builtin_retained_payload_bytes`. capi keeps the strips'
producers with the plan's provider epoch, and that producer table (one producer per strip plus
its strip ID bytes) is charged in `capi_retained_bytes`. On the nine-track EQ reference session the
two rows move 17,451 -> 28,521 and 256,812 -> 258,135 bytes. A caller whose
`maximum_builtin_retained_bytes` and `maximum_capi_retained_bytes` were exact before this change
must raise both. No input, effect or route lane is attached, so rendering, `latency_samples`,
`tail_kind` and `tail_samples` do not change. #1257 delivers fader, mute and pan edits through
them (next section).

## Value-only track edits on the running plan (#1257)

`miso_engine_v1_submit_command` classifies each `SESSION_TRANSACTION_APPLY` by the committed
model's delta (`host_core::classify_live_delta`, #1053 D1), never by its opcodes.

- **Live.** The committed models before and after differ only in track `fader` (`left_db`,
  `right_db`, `left_mute`, `right_mute`) and `pan`/`matrix` values. The engine pushes the records
  that change a rendered value to the newest plan -- the pending replacement if there is one, else
  the current plan -- and commits. No plan is prepared; source rings, effect state and the render
  position continue, and the host keeps submitting with no seek. Fader and mute changes are steps
  until #1054 gives them ramps; a pan or matrix change carries the session's own
  `smoothing_samples`.
- **Model-only (#1260).** A delta that changes only fields no prepared plan reads -- the session
  ID, the render profile's `id`, the output profile's `id` and the stored `automation` table --
  is live with no records: it commits through the same admission and protocol predicates, emits
  the same response and `SESSION_COMMITTED`, and the running plan, its source rings and its effect
  state continue untouched. It may ride with fader, mute and pan edits. The automation term holds
  only while no host renders stored automation (#1058); every other profile field stays
  structural.
- **Live effect parameters (#1264, #1265).** A track's console-slot entry or insert whose
  `params` change only in live parameters, parametric EQ values included, is live too; see
  "Value-only effect parameter edits" and "Value-only parametric EQ edits" below. So is a
  bypass change on one, except the delay's and the multiband compressor's; see "Value-only effect
  bypass edits" below.
- **Rebuild.** Everything else replaces the plan exactly as before: any submix strip value, any
  edit while either model declares a VCA, a mute change on a track that a `follows_mute` route
  reads, a fader dB outside `[-144, 24]` or a pan/matrix the setter refuses (reported as
  `COMPILE_REJECTED` with the preparation diagnostic a rebuild gives), and every other field.
- **Timing (#1053 D2).** A live edit applies no later than the first render call that begins after
  the submit returns, and a stage that has not drained yet may apply it one block earlier, so one
  transaction's records can land up to one quantum apart. It is heard up to `latency_samples`
  later. There is no block-atomicity claim.
- **Backpressure.** Each strip's fader/mute and matrix/pan lanes hold 16 records each. A
  transaction that does not fit every lane it touches returns `BACKPRESSURE` with last error
  `control.live.backpressure` (a full plan queue stays `control.plan.backpressure`). The model,
  the revision, the replay cache, the events and every lane are unchanged; the host retries with a
  new request ID after a render call. A host that is not rendering takes 16 single-value edits
  per lane before the first refusal.
- **No ack before a drop (#1053 D6).** Classification, the live admission (#1053 D8), the
  producer lookup, the room check on every lane and the protocol's commit predicate all run before
  the first push; nothing changes until the last of them passes. The push and the commit then
  cannot fail, and render applies every record it pops, because the classifier refuses every value
  the setters would refuse.
- **Same response and events.** A live edit returns `TransactionApplied` and emits one
  `SESSION_COMMITTED` (plus `AUTOMATION_CANCELED` per queued batch), exactly as a replacement does.
  The reliable event lane holds two events, so a host drains it after each edit either way.
- **No rebuild signal.** Since #1273 a host needs none. After a live edit or a replacement alike
  it goes on submitting every source the transaction left unchanged, with no seek; after a
  replacement it acts only on a source the transaction added, changed or removed (see the #1273
  paragraph under "Linux artifact and consumer boundary", and `miso_engine_v1_source_seek_at`
  (#1275) to start an added one in time). The ABI gives no way to tell the two apart: the response and the events are
  the same, and `miso_engine_v1_plan_resources` describes whichever plan is active, which a
  replacement may leave unchanged. No new symbol, opcode, field, result code or event is added.
- **Live admission.** The prospective compiled model lives beside the current one until the
  commit, so a live edit is admitted only if both plans' graph bytes plus both compiled models fit
  `maximum_graph_session_plus_plan_bytes`; the newest plan's `capi_retained_bytes`, plus the
  current epoch's rows while a candidate is pending, plus the prepared-protocol rows, fit
  `maximum_capi_retained_bytes` (the last term counts a catalog the live arm never builds: a
  documented overcount); and the largest allocation fits `maximum_named_allocation_bytes`.
- **Resource movement.** Each provider epoch now keeps its own capi resource figures (32 bytes)
  for the live admission, so every session's `capi_retained_bytes` grows by 96 bytes: the inline
  current epoch in the session plus its two reserved epoch slots. On the nine-track EQ reference
  session the row moves 258,135 -> 258,231 bytes. A caller whose `maximum_capi_retained_bytes` was
  exact must raise it.

## Live effect lanes on every plan (#1263)

Every C ABI plan, at compile and at every structural replacement, also carries one live control
lane per prepared effect instance, console slots and inserts alike, on tracks and submixes. Each
lane's ring holds `min(16, the effect's automation capacity)` records, and a parametric EQ's lane
also carries its prepared-target staging. capi keeps the producers with the plan's provider epoch;
each EQ's producer owns its prepared-target owner. #1264 pushes live effect parameter edits
through them, and #1265 publishes live EQ targets through the owners (below). No input or route
lane is attached.

- **Rendering, latency and tail do not change.** A lane is seeded from the session's bypass, so
  the plan renders bit-identically to a lanes-free plan, and `latency_samples`, `tail_kind` and
  `tail_samples` equal those of the same plan without effect lanes. Both are checked on the
  nine-track EQ reference session and on the one- and ten-track parity sessions (the ten-track
  one has a bypassed limiter insert) at the four launch rates.
- **Charges.** The rings, the target staging and the banked live-control owner the lanes make the
  plan build are graph rows, charged in `graph_session_plus_plan_bytes`,
  `graph_incremental_plan_bytes` and `graph_metadata_bytes`. The producer table, its strip and
  effect IDs and the EQ owners are reported once, in `capi_retained_bytes`. Both enter the
  replacement and live admissions through those rows. The producer payload is admitted twice, a
  conservative double admission: at preparation host-core also admits it against
  `maximum_graph_session_plus_plan_bytes`, so the initial compile needs the graph row plus the
  compiled model plus the payload `capi_retained_bytes` charges for the producers (15,361 bytes on
  the reference session below). A replacement's two-plan peak and the live admission dominate it.
- **Resource movement on the nine-track EQ reference session** at the reference `limits()`
  (`maximum_automation_spans_per_block` S = 128, a 128-frame quantum; x86-64, eight lanes). These
  are that session's figures, not general ones:

  | row | before | after |
  |---|---|---|
  | `graph_session_plus_plan_bytes`, `graph_incremental_plan_bytes` | 253,934 | 382,918 |
  | `graph_metadata_bytes` | 56,137 | 185,121 |
  | `capi_retained_bytes` | 258,231 | 273,640 |
  | `largest_named_allocation_bytes` | 90,720 | 90,720 |

- **The general formula.** The graph rows grow by a per-member term plus a per-bank term:

  - each parametric EQ member 2,104 bytes, each other effect member 1,208 (no 896-byte target
    staging);
  - at a 128-frame quantum, each eight-lane effect bank 8,944 + 360 * S bytes, plus 64 * L for a
    prepared latency of L samples, and each four-lane bank 4,560 + 200 * S. S is the caller's
    `maximum_automation_spans_per_block`, because each lane's packed window equals the automation
    capacity, so the bank term scales with it: at S = 4,096 a four-lane bank is about 824 KB.

  On the reference session that is nine EQ members and two eight-lane banks at S = 128 and L = 0:
  9 * 2,104 + 2 * 55,024 = +128,984. `capi_retained_bytes` grows by the producer table and its
  payload (15,361 bytes there: nine 104-byte producers, their IDs and the nine EQ owners) and 16
  bytes in each of the three provider-epoch slots. Every other row is unchanged. A session without
  effects moves only `capi_retained_bytes`, by 48 bytes. A caller whose
  `maximum_graph_session_plus_plan_bytes` or `maximum_capi_retained_bytes` was exact before this
  change must raise it; a structural replacement charges both plans' graph rows, so its peak grows
  by twice the graph move.

## Value-only effect parameter edits on the running plan (#1264)

A transaction that changes, adds or removes `params` of a track's native effect -- a console
slot's entry or an insert -- is live when every value it changes is live. The classifier resolves
the pre- and post-commit `params` through `effect_compiler::resolve_initial_values`, the one
function preparation uses (unit, channel policy, defaults, `-0.0` to `+0.0`, domain, unknown IDs),
and compares the resolved values bit by bit.

- **Live.** A changed value of a parameter whose descriptor is automatable with
  `automation_rate` `Block`. The engine pushes one parameter record per changed
  `(parameter, lane)` -- `Both` for a shared parameter, `Left` and `Right` apart for a per-lane one
  -- to that instance's lane in the newest plan, and the effect ramps it with the descriptor's own
  smoothing. A removed entry returns to the default. Rewriting a value in another representation
  (a `both` value as equal `left` and `right` values) pushes nothing. It may ride with fader, mute,
  pan and model-only edits in one transaction.
- **Prepared, so a rebuild.** A changed value of a parameter that is not automatable or whose
  `automation_rate` is `None` (for example the gate/expander's attack, hold and release, and a
  parametric EQ band's `enabled` and `kind`). The parametric EQ's live parameters ride prepared
  targets; see "Value-only parametric EQ edits" below.
- **Rebuild for everything else about an effect**: identity, quality, link mode, a prepared bypass
  (#1266, below), sidechain, the insert order and the console slot set, any effect on a submix
  strip, and `params` that preparation refuses (reported as `COMPILE_REJECTED` with the
  preparation diagnostic).
- **A transaction too large for a lane.** An effect lane holds `min(16, automation capacity)`
  records. A transaction whose records for one instance outnumber that capacity could never fit,
  so it takes the rebuild path instead of an endless `BACKPRESSURE`. A transaction that fits the
  capacity but not the lane's current room returns `BACKPRESSURE` with
  `control.live.backpressure`, exactly as a full fader lane does, and changes nothing.
- **No ack before a drop.** Every record passes its producer's preflight, and every readback row
  is looked up, before the first push, beside the room check of #1257.
- **The readback follows.** After the commit, the protocol's parameter state
  (`PARAMETER_STATE_GET`) reports each live value, exactly as a rebuild's catalog would. The
  catalog's handles and metadata do not change.
- **Effects keep their state.** A live parameter edit is not bit-identical to a fresh plan of the
  edited session, because the effect's history differs; it is bit-identical to the browser's live
  lane given the same records at the same block, at the four launch rates on one and ten tracks.
- **No resource movement.** No row moves: the records ride the lanes #1263 attached.

## Value-only parametric EQ edits on the running plan (#1265)

A parametric EQ instance (a console slot's entry or an insert of a track) takes its live values as
prepared targets through its owner, never as bare parameter records.

- **Live.** Each band's frequency, gain, Q and shelf slope, and the HPF's and the LPF's `enabled`,
  frequency and Q (every EQ parameter whose `automation_rate` is `Block`).
- **Prepared, so a rebuild.** A band's `enabled` and `kind` (decision 14, F2), and any EQ value
  preparation refuses (`COMPILE_REJECTED`, as for any effect).
- **Targets are designed on the control thread.** The classifier seeds host-core's
  `EqTargetPreparer` with the instance's pre-commit values, resolved by the one function
  preparation uses, passes the changed `(parameter, lane)` values as edits, and designs the
  targets at the session's sample rate. Render only applies them, through the EQ's prepared-target
  staging.
- **All or nothing.** For each EQ instance, in the newest plan: `begin_owner` at the owner's
  committed revision, `edit_owner` for each edit, and `preflight_candidate_targets` (revision,
  validation and the whole target prefix's queue room), before the room checks and preflights of
  #1264 and the protocol predicate. A lack of room is `BACKPRESSURE` with
  `control.live.backpressure`; any other refusal is `INTERNAL`. A refusal at any of these steps
  discards every owner begun and changes nothing. Then the records are pushed, each owner's targets
  published, the protocol committed and each owner committed; none of them can fail.
- **A transaction too large for a lane.** An EQ instance whose designed targets outnumber its
  lane's whole capacity (at most 12 targets against `min(16, automation capacity)`) takes the
  rebuild path, as #1264 does for records.
- **The readback follows** every changed EQ value, as for #1264.
- **Effects keep their state.** A live EQ edit renders bit-identically to the browser's lane given
  targets that `EqTargetPreparer` designs from the owner's own committed values, published through
  the owner at the same block, on the nine-track EQ fixture at the four launch rates.
- **No resource movement.** The targets ride the EQ lanes and owners #1263 attached.

## Value-only effect bypass edits on the running plan (#1266)

A transaction that switches the bypass of a track's console-slot entry or insert is live through
the rack's latency-preserving bypass shunt (decision 14).

- **Live.** One bypass record carrying the new bypass goes to the instance's lane in the newest
  plan. It rides with the instance's parameter records or, for a parametric EQ, its targets, under
  the one-quantum allowance of #1053 D2; no order between them is promised.
  The shunt selects the latency-matched dry signal for the lane while the wet path keeps running,
  so the effect's state continues either way and prepared latency does not change. The switch is a
  step: there is no bypass crossfade (decision 14, F7). It may ride with every other live edit.
- **Prepared, so a rebuild.** A bypass change, in either direction, on the delay or the multiband
  compressor: their session bypass is prepared into the effect, not lowered to the lane
  (`effect_compiler::lowers_session_bypass`), so a live record would be acked and never heard
  (decision 14, F4). A submix strip's bypass stays a rebuild too, as every submix value does.
- **No ack before a drop.** The bypass record passes its producer's preflight and the room check
  of #1264 before the first push; for an EQ the room must hold the record and the whole target
  prefix together, or the transaction is `BACKPRESSURE` with `control.live.backpressure` and
  changes nothing.
- **Equal to a rebuild, on the gate fixture.** The wet path keeps running, so the switched
  instance's own state is the same either way. An effect downstream of it keeps the live history,
  as after any live edit, so the from-sample-0 oracle holds only while that downstream memory fits
  the window. On the gate fixture (a compressor insert, then a soft-clip console slot), from
  E + ceil(latency / quantum) + 1 on, a live bypass or lift renders bit-identically to a plan
  compiled from the committed snapshot and fed the same source from sample 0, on one and ten
  tracks at the four launch rates. A second stateful effect after the switched one (a compressor
  after a compressor) does not converge within that window; its difference decays.
- **No resource movement.** The record rides the lanes #1263 attached; no readback row reads it.
