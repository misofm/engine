# Housekeeping: protocol

## Authorized scope and smallest closable slice

Review the complete `crates/protocol` Cargo package and its tests. Deliver practical, behavior-preserving cleanup in this package, or an evidenced no-change verdict when existing choices earn their cost. Cover all five user requests: repeated code/LoC; load-bearing test purposes; avoidable copies; applicable micro SIMD; and data structures that cap efficiency. The user requests two GPT-6.1 Sol xhigh agents concurrently; this overrides the default Terra implementation model for this workflow. Root Sol owns briefing, adversarial review, exact-path checkpoints, integration and remote synchronization.

## Frozen boundaries

Existing public APIs, feature behavior, canonical/wire identities, arithmetic order and per-lane rendered bits, latency/tail, link modes, smoothing and NaN/denormal rules remain contractual. Zero render allocations/frees, locks, I/O or syscalls; no runtime native ISA dispatch. Retain scalar tails, 4-lane Wasm/NEON and x86 AVX2/FMA. Preserve all feature and target configurations. Do not expand another open issue or change DSP algorithms. Read current related issue bodies before touching their subject; discoveries that require architecture or product decisions become bounded follow-ups and final owner questions.

## Implementation and test-value decisions

Prefer deleting or consolidating repetition to adding abstractions that increase total complexity. Assess every existing test or a clearly named homogeneous family by its plausible unique defect; remove trivial, redundant or obsolete cases only after identifying the surviving behavioral gate. Keep independent numeric/oracle, fault, allocation, queue, boundary and target tests. Add/rewrite tests only for a concrete uncovered defect, and state which plausible defect no existing test catches. No prose/source-grep tests, new bit-digest pins or exact resource-byte pins. Copies needed for ownership, snapshots or atomic admission stay unless the same semantics are proved with less work. Data structure changes must preserve deterministic order and bounded realtime work. Inspect applicable hot loops and generated code before claiming additional SIMD; recursive/stateful dependencies alone do not justify changing arithmetic.

## Objective gates and evidence

- Read all production and test files in this package; record concise findings for each of the five requests, concrete changed/deferred locations, and load-bearing test families with retained coverage for deletions.
- Run focused locked package tests and affected feature configurations; use existing downstream/RT/differential gates proportional to the changed contract. Check formatting and package clippy with warnings denied. Relevant Wasm and AArch64 compile checks are required for changed product code; record limitations candidly.
- Changes to DSP arithmetic or hot state need existing independent numeric and scalar/SIMD gates plus one-time base/head evidence when needed; no permanent comparison against the old implementation. Existing research remains the algorithm authority; no new algorithm or listening claim is authorized.
- Benchmarks are optional and descriptive. Any timed measurement freezes its workload/validator, passes zero-workload preflight, and runs exactly one invocation with one warmup/two measured rounds. No timing optimization loop, performance percentage or unsupported sound-quality claim.
- Root conducts one adversarial verdict per coherent attempt, at most five total attempts. Every new/rewritten test gets its unique-defect sentence in that verdict. No-change audits require the same five-axis review, not manufactured edits.
- Stop after a coherent compiling, focused-green tranche; root commits exact owned paths before further edits. Keep edits within `crates/protocol` and this numbered spec. Independent crates use separate worktrees. Root pushes promptly and updates this GitHub issue with evidence; close only after PASS and evidence upstream, verifying remote state. Merge through the required qualification check and remove clean completed worktrees.

## Sol brief approval and decision record

Approved 2026-10-01 by root Sol within the user's crate-by-crate housekeeping request. Attempt 1 is authorized with the frozen scope/gates above. Owner questions are collected for the final report; routine reversible cleanup proceeds autonomously. Scope is this package's housekeeping, not completion of pre-existing feature/bug issues.

After the full eighteen-file read, root approves the bounded diagnostic/UTF-8/revision-predicate cleanup and strict-subset/repeated-input test removals. The focused-green nine-path tranche is checkpointed as b2143372 before further edits. Preserve every golden wire case, malformed/limit/truncation input, maximum metadata case, queue/replay/admission rule, generator and schedule. Current IO6/IO16 and other open optimization/feature objectives stay separate.

Root resource-test ruling: retire `replay_layout_stays_within_the_capi_resource_oracle` entirely; its four private Rust sizes and repin history are not ABI or a distinct product gate. Current allocator-observed C API accounting/admission owners remain. In `replay_resource_projection_is_bounded_and_overflow_checked`, replace the historical exact 1,248-byte assertion with a 2-KiB ceiling for this four-entry/1-KiB configured fixture; retain the exact configured 1-KiB largest-allocation and maximum-entry overflow checks. This bounds accidental extra retained charges without fixing a private layout. No product padding, resource formula, new test/harness or external budget changes. Record the surviving rewritten test's unique ceiling/overflow purpose and final cumulative metrics.

## Attempt evidence

### Attempt 1 — worker A completion, root verdict recorded below

Read all eighteen package files (sixteen Rust files, manifest and mutation notes), both feature configurations, and relevant current #560 IO2/IO3/IO6/IO11/IO16, #338, #763 and #1057 boundaries. No dependency, lock, public API, schema/registry identity, corpus word, generator, external schedule, admission rule, resource formula or render expression changed. Root checkpointed the product before each authorized continuation: b2143372, 4fba0e41, 54befe60 and the resulting unused-import correction a707bd58. Relative to 5d0cc1bb, nine package paths have 37 Rust lines added and 317 removed: **−280 Rust; five active functions retired**. Manifest/prose outside Rust is unchanged; this record is separate.

#### Five review axes

- **Repetition/LoC:** encoding now uses the existing MessageId exact-revision predicate already used by decoding. Removed repeated identical encodes and five subset/private-layout tests. Corrected future-codec prose and exponential endpoint docs to describe the unchanged finite, nonzero, same-sign law. Kept schema-specific public wrappers and separately enumerated registry tables: the complete IO2/IO3 consolidation is outside this slice.
- **Test value:** every retained test is assigned to the behavioral families below. Exact wire goldens remain contract evidence; the unchanged conformance corpus remains the sole cross-target digest owner. Resource layout pins are gone. No new test, reference, generator, harness or permanent base/output comparison was added.
- **Copies:** reliable diagnostic encoding borrows the retained value without cloning its strings/path; ownership is released by the same successful-egress path after encoding. Bounded error diagnostics accumulate in one reserved response vector rather than cloning each growing prefix; the first failing candidate is popped, preserving prefix order, count/byte limits, first-failure break, omitted count and encoding-error treatment. Stable-ID decoding validates borrowed UTF-8 before the one required StableId allocation. The maximum metadata test moves its descriptor vector into its sole owner. Transaction candidates, retained replay bytes, snapshots, queue batches and caller-output copies retain their ownership/atomicity purpose.
- **Micro SIMD/codegen:** inspected the current AVX2/FMA release test artifact `protocol-49d8b6ed8e0906ea`, including compiled production `session_wire::stable_id` and `SliceSink::raw`. The former calls borrowed `core::str::converts::from_utf8` then `StableId::parse`, with vector moves for result storage; the latter keeps bounds checks and a `memcpy` call. Compiler-generated `vmovups` also moves iterator/state words. Variable-length structural validation and ordered wire stores have no newly identified numerical lane kernel to justify manual SIMD. No ISA dispatch, scalar/tail change, instruction-count comparison or timing claim.
- **Data structures:** retained fixed-capacity queue slots, bounded telemetry staging with deterministic first-key/flush order, the replay byte arena and ordered prefix eviction, and the fixed-slot borrowed BTLV schema view. Removing diagnostic-prefix copies preserves the same bounded serialization scans. IO6 admission scans and IO16 response staging remain separate optimizations; no new index, hash table or capacity change.

#### Retained behavioral-family purposes

Counts describe the final **126 unit + 9 integration = 135 active tests, zero ignored**. Family names identify the existing module/prefix owners; the following covers every retained function.

| Owner/family | Count | Plausible load-bearing defect |
| --- | ---: | --- |
| `btlv::tests` | 6 | Constructor count/frame/reserved/depth limits diverge; nested errors fail to restore depth or parent field counts; under/over emission escapes; bodies run twice or nested lengths/optional flags are wrong. |
| `wire::tests` | 9 | Frozen command/response/event headers, correlation and revision rules drift; public peek confuses structural and selected-schema errors; truncation/padding/short-output ownership fails; PCM or cross-kind carriers become reachable. |
| `schema::tests` | 4 | Field metadata loses sorted uniqueness, numeric wire codes or session-parameter enum mappings; required/optional/repeated count rules disagree with the schema. |
| `typed_frame::tests` | 5 | Sizing miscounts optional/repeated fields; any registered command/success/event or non-OK status dispatches to the wrong payload; mutation revision/response-kind/backpressure exclusivity or poisoned short-buffer behavior fails. |
| `model::tests` | 9 | Revisions/snapshots cease to be authoritative; edit or final compilation refusal partially commits; launch-rate validation/order, compound parameter keys or Any/max-revision rules drift; current/retired opcode identities are reallocated. |
| `queue::tests` | 11 | Projection overflows; manual LE/reserved-byte codec drifts; full/empty/wrap/generation drops ownership; 10,000 records are not forty atomic batches; past/order/overlap/aggregate density/empty refusal is partial; reliable events coalesce, reserved capacity leaks, or lossy counters fail to saturate. |
| `message_wire` common-error family | 5 | Canonical diagnostic/path/mandatory flags, unknown optional/required/variant handling, truncation and count/string/depth limits diverge. |
| `message_wire` capability + B1b success/golden families | 3 | Borrowed native/LE ID views accept inconsistent registries; success fields, snapshot UTF-8 chunks or fixed wire identities fail round-trip/truncation contracts. |
| `message_wire` metadata/state/descriptor family | 3 | Domain/default/enum/mapping invariants, sorted handles, state flags, mandatory fields or maximum 256-descriptor pages drift. |
| `message_wire` automation, transport, telemetry/counters, diagnostics-page and event families | 5 | Count/stride/reserved/range/overlap or typed transport/origin rules fail; selector coupling/order, diagnostic cursor/severity/order/257 refusal, meter/counter/diagnostic/cancellation event fields, NaN/reserved words or PCM refusal drift. |
| `session_wire` canonical framing/count/ownership family | 6 | Golden session ID, outer repeat counts, all 64 current edits, nested repeated IDs, exact/nonempty mutation rule or caller poison/frame/count overflow checks fail. |
| `session_wire` depth/string descendants | 2 | Three envelope levels consume logical depth incorrectly, deep input escapes limits, or descendant text loses the configured string bound. |
| `session_wire` live source/render/effect/console/tap/route-automation variants | 6 | Current nested values, link/quality/rack/channel/unit/shape codes, section/order or any live send tap fail canonical typed round-trip; render mode accepts a retired code. |
| `session_wire` retired/malformed/optional/truncation family | 7 | Retired opcode/track IDs revive; duplicate/type/order/tagged-pan mismatches pass; optional versus required semantics, nested corruption or any byte truncation escapes refusal. |
| External console integration | 5 | Wire edits reach an insert/other track/declaration; fixed console structure is mutable; skipped-track slot-set changes or per-track add/drop/reorder partially commit. Handwritten JSON expectations remain independent. |
| External builtins-rack + response API | 4 | Builtins edits resolve an actual/phantom rack, real insert or strip edits stop working, or the response's full canonical frame ceases to be public to external callers. Mutation notes remain valid. |

Controller's 45 retained functions have these separate families:

| Controller owner/family | Count | Plausible load-bearing defect |
| --- | ---: | --- |
| Resource projection | 1 | The four-entry/1-KiB fixture exceeds its 2-KiB ceiling, largest allocation no longer equals the configured 1 KiB, or maximum-entry arithmetic fails to refuse overflow. No other protocol test owns this preallocation projection boundary. |
| Replay prefix/arena/reservation/bounds, deterministic replay, compatibility response and admission preflight | 7 | Eviction/compaction invalidates a surviving hit, foreign/stale hits read another cache, exact request binding or reservation bounds fail, replay executes again, backing/header decode loses canonical ownership, or provider work precedes retention admission. |
| Public B1b reader-pass, full-frame ingress and provider feature matrix | 5 | Dispatch reintroduces a second structural walk, misroutes one registered command, changes exact replay/reuse, mutates short caller output or correlates malformed outer input, or capabilities disagree with feature refusal. |
| Immediate and one-call preparation-cost owners | 2 | Immediate commands allocate a prepared vector; transaction success/replay/reuse/expiry/conflict/validation/capacity paths unnecessarily clone the prospective replay cache or build staging vectors. These operation counters complement the allocation tool. |
| Structural prepare/token, revision refusal and commit-event owners | 5 | Preparation becomes visible, cancellation/owner/generation/serial tokens lose affine rules, Any/conflict mutation commits, or model replacement precedes reliable event reservation. |
| Diagnostic path/omission/edit refusal, retired/console conformance and all non-OK statuses | 5 | Field/index/stable-ID paths, first retained diagnostic/omitted count, operation attribution, refusal code/status mapping or canonical error bytes drift. |
| Slot-set nonack + decoded session/track/route/automation and rate rollback | 6 | Ack precedes rejected partial state, decoded edits diverge from the typed model, or final validation/rate errors alter revision/model/snapshot. |
| Public automation replay, cached pressure, endpoint time and B2b ingress | 4 | Exact replay enqueues twice; pressure retry becomes a silent drop; client data replaces the endpoint clock; domains/header identity/queue admission diverge. |
| Transport and locate/order epoch | 2 | Idempotence/state-event origin fails, state-only edits cancel automation, or locate fails to cancel/reset ordering. |
| Telemetry/counters and diagnostic pages | 2 | Echo/selective configuration, unknown IDs, nondestructive reads, severity/cursor expiry or pagination loses bounded typed semantics. |
| Snapshot pagination and canonical JSON UTF-8 splits | 2 | Exact-revision/EOF boundaries or byte-chunk splits corrupt the authoritative snapshot before/after a commit. |
| Six event families, counter splitting, short-retry storage, disabled/full ownership | 4 | Header/report/event words drift, nonascending counters stall, short output consumes reliable/diagnostic/lossy state, disabled streams emit, or full admission loses the original event. |

#### Deletions and surviving defects

| Retired function | Stronger surviving owner |
| --- | --- |
| `complete_frame_encoders_preserve_prepared_typed_values` | The four full-frame registry/status/event families compare actual typed payloads/canonical framing and poisoned short buffers. Repeating four basic immutable inputs 32 times checked only lengths and added no allocation/state witness. |
| `direct_common_encoder_is_byte_stable_in_caller_storage` | `common_error_payload_is_canonical_and_caller_owned` and common truncation/limit families catch wrong diagnostic bytes, flags, paths and output ownership. Sixteen identical encode/decode calls added no distinct input. |
| `frozen_deep_transaction_reaches_public_b1b_process_path` | `public_b1b_uses_exactly_the_typed_reader_passes_and_replays_identical_bytes` reaches the same deep frame and also checks decoder-pass/replay behavior; current envelope-depth and conformance dispatch owners defend nesting refusal. |
| `replay_layout_stays_within_the_capi_resource_oracle` | The local ceiling/overflow survivor plus C API `capi_retained_bytes_charge_every_byte_the_compile_retains`, reference budget, exact/one-below admission and tiny-frame accounting owners defend actual costs. Four private size pins/repin history were neither ABI nor allocator evidence. |
| `five_session_edit_encoders_are_canonical_and_ordered` | `transaction_outer_header_uses_sizing_sink_repeated_count` and the 64-edit full-schema equality owner catch wrong top-level counts or omitted variants. The retired body checked only count five. |

Four identical-input tails (transport, telemetry/counters, diagnostic pages and events) were deleted after their earlier canonical/truncation/malformed assertions already reached the same values. Maximum metadata still encodes/decodes all 256 unchanged descriptors once; the 64-edit test still decodes all edits and performs one cleared-buffer canonical reencode. No boundary case, random input or meaningful partition/admission schedule was deleted.

#### Actual gates, timing and limits

- Baseline all-feature debug: 140 PASS. First checkpoint all-feature debug: 137 PASS. Resource survivor: 1 selected PASS. Full release all-features before the last count-only deletion: **136 PASS**, zero ignored. After that deletion, sizing and full-schema owners each passed; root's final locked combined all-feature debug gate then passed **protocol 135 + C API 44 = 179**, zero ignored. Its unused SampleFormat import warning was fixed in the final import-only checkpoint; no test body changed afterward.
- Default production `cargo check --locked -p protocol --no-default-features` PASS. All-feature checks for Wasm `+simd128`, `aarch64-apple-ios` and `aarch64-linux-android` PASS. These ran on the final production code before the subsequent test-only deletion/import cleanup; they are compilation evidence, not mobile device execution. No bare-Wasm build or expanded matrix.
- Native release `conformance_corpus`: 3 PASS; existing `check-protocol-wasm-parity.sh`: simd128 PASS. The unchanged 46-frame cross-target owner/pin survives; no new digest or fixture.
- Rebuilt current release audit binary and existing `run-protocol-allocation-audit.sh`: PASS for command/success/non-OK/event encoding, reliable/meter/counter egress, 64 edits and 10,000 records in forty batches. Its measured claim is zero allocations in those prepared operations; diagnostic egress is excluded typed control-plane storage. It does not newly prove zero frees/syscalls or all controller preparation paths.
- Final import-cleanup `cargo clippy --locked -p protocol --all-features --all-targets -- -D warnings` exits 0; no package-code/unused-import warning. It emits the existing unresolved disallowed-method configuration warnings for the real `math::fast_db::fast_level_db` and `fast_gain_from_db` functions because they are gated by `math`'s `lane` feature, absent in this dependency graph. Workspace configuration is unchanged; these are not removed functions.
- `cargo fmt --all --check`, `git diff --check`, workspace/realtime/typed-control policies and the existing typed-control policy mutation suite PASS. No timed benchmark, new corpus, listening claim, fuzz sweep or target matrix expansion.

Raw logs: `/tmp/housekeeping-a-protocol-{baseline,focused,resource-focused,release,default-check,wasm-check,ios-check,android-check,native-corpus,wasm-parity,audit-build,allocation,final-import-clippy,workspace-policy,rt-policy,control-policy,policy-mutations,sizing-survivor,schema-survivor}.log`; codegen `/tmp/housekeeping-a-protocol-native-disassembly.txt`. Root's final integration log is `/tmp/engine-housekeeping-protocol-capi-final-test.log`. All actual gate statuses were checked, including cargo exit codes. No unresolved product/safety decision arose; broader queue/response/registry optimization stays with existing issues. Worker product work is paused for the single root attempt-1 verdict.


## Root adversarial verdict: PASS — attempt 1

Root Sol inspected all nine changed package diffs and each retirement's surviving owner, the resource ruling and complete family inventory. Borrowed diagnostics retain their successful-egress lifetime; the single diagnostic-prefix vector keeps the original omission-zero sizing probe and first-failure/prefix/limit rules. Borrowed UTF-8 retains invalid-UTF8-before-invalid-TLV refusal order. The shared revision predicate maps the same four mutation IDs. Exact wire values, actual allocator/admission owners, all64 edit variants and every malformed/boundary input survive. The final combined179-test log, actual caller-buffer allocation gate, native/Wasm corpus parity, supported-target builds and precise clippy limitations were verified. No product issue was found in this slice.

Each rewritten test retains this concrete purpose:

- `replay_resource_projection_is_bounded_and_overflow_checked` catches overflowing preallocation arithmetic or a replay fixture exceeding its resource ceiling, while preserving the configured largest-allocation claim.
- `b2a_goldens_truncations_malformed_matrix_and_encoder_audit` catches metadata domain/ordering or256-descriptor boundary encoding errors with the same maximum payload moved into its final owner.
- `b3a_transport_goldens_truncation_and_direct_codec_are_strict` catches transport-origin, state or malformed/truncated wire acceptance, retaining every distinct probe after identical repetitions were removed.
- `b3b1_telemetry_and_counters_are_typed_canonical_and_bounded` catches selector coupling, unknown-ID/order or counter-payload refusal defects that other message families cannot reach.
- `b3b2_diagnostics_pages_are_canonical_bounded_and_strict` catches diagnostic cursor/severity/order or page-limit drift, with all original refusal and canonical inputs retained.
- `b4_event_payloads_are_typed_canonical_and_truncation_safe` catches event-family routing/field/reserved-word and short-output defects across all existing payload families.
- `direct_full_schema_encoder_is_byte_identical_in_caller_storage` catches an omitted/misencoded current session edit or caller-buffer canonicalization error across all64 distinct edits; its full decode/equality and cleared-buffer reencode remain.

No new test was introduced. All five user axes and each retained family are covered; five strict subsets/private-layout functions were retired with stronger named owners. Existing broader IO2/IO3/IO6/IO16 work remains bounded separately. No owner question or measured timing claim is added.
