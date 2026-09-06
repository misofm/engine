# 506: Use bench-support as the bench package's lowercase hex authority

Audit CP20 remains partial after delivered #500/PR505. This issue owns the three bench-package encoder bodies, not all repository encoders. Astra directly inspected the package before scoping this coherent consolidation.


the current tools/bench/src Rust search finds exactly three byte-to-hex encoder bodies, in builtins.rs1778, effect_interchange.rs77 and graph.rs565. `is_lower_hex` validates and `hex_bytes` decodes; neither is an encoder to replace. Consolidating the three encoder bodies is one bounded package-maintenance outcome, with no cross-crate production architecture change. This is not a re-count of the original repository-wide14 claim.

Recommended title: “Use bench-support as the bench package's lowercase hex authority”. Exact paths: tools/bench-support/src/digest.rs plus tools/bench/src/{builtins,effect_interchange,graph}.rs, numbered spec/evidence. Expose the existing private borrowed-byte `hex(&[u8]) -> String` in bench-support with ordinary documentation/must_use as a tooling helper; keep its algorithm unchanged. Its current Sha256Sink and sha256_hex callers continue using that same implementation. Do not invent a new encoder, sink trait, dependency or framework.

Builtins' finalized digest outputs must call this byte encoder directly, with the same already-finalized bytes; never hash a digest again. Its one-shot sha256 path may use existing bench_support::digest::sha256_hex. Effect-interchange's one-shot digest_hex and graph's sha256_hex may import that existing one-shot authority under their local names, deleting the redundant local bodies. Keep effect-interchange's raw `[u8;32]` digest helper and all its consumers unchanged, along with hex decoding/validation. Preserve hash boundaries, streaming update counts, fixture inputs, digest bit/byte ordering, lowercase zero-padding and record/schema bytes. String allocation strategy differences are not a measured performance claim.

Finite independent proof: one compact byte-encoding vector test in the existing digest module covers empty bytes, leading zeros and all high/low nibble values with a fixed literal expected string; retain published empty/abc SHA256 vectors. One small direct digest-identity assertion at each affected bench module confirms its actual imported/helper call emits the independently known abc digest; builtins additionally proves finalized digest encoding is not rehashed. Existing builtins untimed tests, graph fixture/percentile tests and effect_interchange::tests::exact_four_rate_migration_envelope_without_timing remain. Run affected module tests debug/release and bench-support digest tests, strict affected Clippy, fmt/diff and applicable existing static benchmark policy gates with individual nonempty records. No timed runner/preflight/capture invocation, broad workspace matrix or new mutation campaign.

Historical072/035/431/473/480 scripts, validators, records, source-seal snapshots and all fixture/output pins remain unchanged. These source bytes necessarily change future tool-source/binary identities; do not overwrite historical provenance or claim an old sealed preparation authorizes a new binary. Any future capture still requires its existing fresh preparation/identity checks and authority. If an existing static gate identifies a concrete protected source-contract conflict, stop for that finite ruling rather than repinning historical evidence. No current measurement authority is consumed by this untimed maintenance.

This consolidated slice can be numbered after root's duplicate/body boundary check and #500 delivery. It leaves CP20 PARTIAL because graph-compiler and other crates/tools/tests retain separately owned encoders; it does not authorize a repo-wide hex library migration. No implementation or tests were performed for this scope refinement.

## Workflow and current base

User override: Astra briefs and adversarially reviews, Luna attempt 1 implements, Sol gets attempts 2/3 after a consolidated FAIL. Root owns Git/GitHub, exact-path checkpoints and delivery. Implementation begins only after numbered current-base review. No benchmark or timing authority is granted.

## Astra numbered scope PASS and activation

# Astra #506 numbered current-base review — PASS

Reviewed clean exact head f775104c239ef4451272d3296c386cba1c106753 in engine-bench-hex-authority, based on delivered main71059eab. Only #500/#503 closure documents and numbered #506 differ from main. Relevant bench/bench-support/Cargo/config source is unchanged from inspected PR505 source3622d625. Independently verified live GitHub506 OPEN, matching title and exact local/remote body.

The numbered body faithfully adopts only the revised consolidated scope, without retaining the superseded one-graph-helper assignment. Approve fresh Luna1 for the four exact source paths: expose the existing bench-support borrowed-byte hex implementation unchanged and replace the three bench encoder bodies with the appropriate existing shared authority. Keep raw digest encoding distinct from hashing input bytes, preserve streaming update boundaries and the effect-interchange raw digest helper, and leave validators/decoders outside scope.

The compact independent nibble/leading-zero/empty vector, published SHA vectors and actual three-module call-site identity tests remain the finite proof. Builtins must specifically show finalized digest encoding without rehashing. Existing untimed module tests, strict affected Clippy/fmt/diff/static policies remain required; no new framework, mutation campaign, broader runtime/target matrix or measured speedup claim.

Historical scripts/validators/records/source-seal snapshots and all fixture/output pins remain frozen. No benchmark/preflight/capture/timing invocation is authorized. Future source/binary identities change naturally and cannot inherit an old sealed preparation by assertion. Any concrete protected-contract failure requires a bounded ruling, not a historical repin.

CP20 remains partial after this slice because other separately owned encoders survive. Root retains checkpoint/GitHub ownership and normal Luna1/Sol2/3 escalation, eventual exact-head PR review and required CI. Independent #496 runtime correction is unaffected. No source/spec/Git/GitHub mutations or tests/builds/timing performed for this review; only /tmp review/readback files written.

Root activates fresh Luna attempt1, independent of the #496 runtime test correction. Git/GitHub/checkpoints remain root-owned; no benchmark/timing authority is granted.

## Luna attempt 1 source and evidence

Final sourcee88a2bb2 shares the existing byte/one-shot digest authority across all three bench modules. Literal all-nibble and abc/finalized-byte consumer tests pass. Separate final captures run bench-support digest5 and bench35 in debug/release plus strict Clippy, fmt and applicable static policies, all exit0. Raw/source-identity records are retained in artifacts/issue506-bench-hex-authority; initial one-command provenance and omitted process-wide environment are candidly documented. Historical seals, pins and measurement authority are untouched. Consolidated Astra review pending.

## Astra Luna attempt 1 PASS

# Astra #506 Luna attempt1 consolidated review — PASS

Reviewed clean exact head deaa13438d460eaf54d3add802468d3a51aac2ff in engine-bench-hex-authority, source e88a2bb2. The four allowed source paths alone change; no implementation delta follows final source. The complete numbered package-consolidation contract is satisfied.

bench-support exposes its existing borrowed-byte encoder with documentation/must_use and no algorithm change. Its SHA256 sink and one-shot helper continue using that authority. All three bench local encoder bodies are removed. Builtins' three already-finalized digest sites call hex directly, preserving input bytes and update/finalization boundaries; its one-shot wrapper delegates to sha256_hex. Effect-interchange and graph import the existing one-shot helper at unchanged consumer names. Effect-interchange decoding, lowercase validation, raw digest byte function and fixture/workload behavior remain. Graph's removed fmt::Write import belonged solely to its deleted encoder; no remaining formatting code or record emission changed.

The retained raw effect digest function is now used only by tests because the shared one-shot authority replaced its former text-wrapper caller. Its exact body/signature remain; the local dead_code annotation is a narrow consequence of the frozen retention contract, not a global lint exemption or new functionality. No Cargo, shared hashing algorithm, scripts/validators, fixture/output pins, historical source seals, record schema or measurement authority changes appear.

The fixed empty/00-through-ff repeated-nibble vector covers each high and low nibble value and leading zero. That vector alone would not detect nibble swapping, but the independent published empty/abc SHA literals and three actual consumer abc assertions contain asymmetric nibbles and do discriminate ordering. Builtins explicitly checks finalized abc bytes equal the known digest and their rehash differs. Existing sink update/vector tests, builtins untimed population, graph fixture/percentile and exact four-rate effect-interchange migration fixture execute and remain unchanged. This is sufficient finite proof without another corpus or mutation campaign.

Authoritative final captures record bench-support digest5 and bench35 tests in each profile, strict affected bench-support/bench Clippy, fmt, bench policy and effect-interchange qualification policy, all numeric exit0. The records identify the preceding HEAD with the final source bytes uncommitted; independently verified every final per-command source-SHA256 list against reviewed files. This is honest byte-attributed execution, not a false clean-HEAD claim. The initial retained debug-support capture is credited only for its actual command; no full-process environment dump is required in the package. Independently ran only a read-only source/spec diff check, exit0; no tests/builds executed for review.

Independently verified every one of74 manifest payload hashes/sizes and exact tracked75-file coverage. Historical reports and capture limitations remain candid. No benchmark runner/preflight/timing was invoked or credited, and no render-speedup claim follows. CP20 remains partial beyond these three package-local encoders.

Root may proceed to delivery preparation with accepted source identity preserved. This tooling-only slice does not require unconditional current-artifact regeneration; classify any integration/build-input drift explicitly and retain historical provenance. Actual-PR exact-head Astra review and required qualification success remain mandatory before merge/closure. No source/spec/Git/GitHub mutations performed.

## Delivered and remotely closed

PR507 merged at2026-09-06T06:44:45Z as0fc4e9592453171f9900aececa156576203d8a55. Astra exact-head PASS applies to4b29296d5cf307227c76e12db537e1824e517ea0; required qualification34016958560 completed SUCCESS on that same head before merge. GitHub #506 is verified CLOSED. All three bench-package local encoders now use the existing shared authority, preserving finalized bytes and hash boundaries. CP20 remains PARTIAL across other owned encoders. No timing authority was consumed. Post-main CI is monitored separately.

Post-main qualification34017298536 at0fc4e9592453171f9900aececa156576203d8a55 completed SUCCESS. Both required pre-merge and post-main CI are green.
