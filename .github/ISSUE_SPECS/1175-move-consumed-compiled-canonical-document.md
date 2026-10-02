# Session: move the consumed compiled canonical document without copying

## Smallest authorized slice

The full crate housekeeping review found two session-validator callers that immediately discard CompiledSession after cloning its complete canonical_json into an owned result. Its only existing accessor borrows the String. Add one documented consuming accessor in crates/session/src/compile.rs that moves this already-validated canonical String out of the consumed control-plane artifact. The existing borrowed accessor and all compilation, model, wire and preparation contracts remain unchanged. This producer-only issue is paired with the session-validator crate issue #1151, which owns adopting the accessor after its complete review.

## Source evidence and boundaries

CompiledSession privately owns canonical_json: String and has no Drop implementation. validate_session_document finishes all five stages, clones compiled.canonical_json and then discards compiled; fold_mono_document does the same after the final transactional compilation before checking output length. Neither caller subsequently needs any compiled field. These are complete document copies, not render-plane PCM or persistent DSP state. No public field, serialization model, numeric rule, ABI symbol, schema, algorithm, queue admission, resource estimate or realtime behavior changes. Consumption retires the remaining model fields on the existing control thread.

## Objective gates and test value

- Implement only the consuming canonical accessor and exercise it through the existing canonical compile test after that test has checked the compiled fields. Preserve the independent expected canonical document and verify that the consuming accessor retains the original nonempty String backing allocation. This storage identity check catches cloning instead of moving, the specific ownership claim; do not add a duplicate test or a digest/resource-byte pin.
- The rewritten existing test must catch a wrong/empty owned snapshot or a copied backing allocation from the new accessor that the borrowed getter alone cannot exercise. The session-validator existing canonical-output/fixed-point/no-op/transactional/native-render tests qualify its callers in #1151.
- Focused locked session tests, fmt/diff and strict package lint. Reuse the full original #1118 crate audit and unchanged algorithm/RT evidence candidly. Supported simd128/iOS/Android compile checks are compile-only. No benchmark, corpus/harness expansion or additional API.
- First coherent focused-green exact-path checkpoint pauses for root commit/push. One root adversarial verdict per coherent attempt, maximum two attempts; do not broaden this issue if a public ownership or destructor constraint appears.

## Sol approval and delivery

Root Sol approves this bounded producer ownership slice on 2026-10-01 under the user's unnecessary-copy cleanup request. Worker A (GPT-6.1 Sol xhigh) may implement after #1152 is remotely closed and the issue boundary is synchronized; worker B reviews #1151 independently. No third agent is requested. The local numbered spec and matching GitHub issue are confirmed before implementation. Root owns checkpoints, PASS, evidence synchronization/remote closure and required qualification/main delivery.

## Attempt evidence

### Attempt 1 — GPT-6.1 Sol xhigh worker A

Read the complete producer implementation and canonical-schema test file, package manifest and this brief. Reused the full #1118 session audit and root PASS recorded at `6489967d`: its complete production/test-family review and independent canonical, numeric, parser/refusal, transactional, resource and allocation owners remain applicable. This is a bounded ownership successor, not another complete crate audit.

**Implementation and preservation.** Exact product checkpoint `fd01ac4b` is upstream through root `2cf148f5`. The two authorized Rust files add **13 lines**, with no deletions or new test functions. Documented `#[must_use] into_canonical_json(self) -> String` moves the private validated field. `CompiledSession` has no `Drop` implementation; remaining normalized-model/index fields are destroyed on the caller's control thread. The borrowed getter, every compiler stage/field, canonical bytes, numeric expressions, schema, resource/admission rules and dependencies are unchanged. Consumer adoption belongs to #1151.

**Test value.** The existing `signed_zero_and_double_rounding_values_survive_session_compilation` retains all original direct-canonical, reparsed-bit, normalized-bit, borrowed-snapshot and recanonicalization assertions. After those checks it saves the nonempty snapshot's backing pointer, consumes the artifact, then checks the existing expected canonical bytes and the same backing pointer. A wrong/empty owned snapshot or cloning/reserializing into another allocation turns this red; the borrowed getter checks cannot exercise this consuming ownership path. The pointer is only compared, never dereferenced. This directly witnesses the backing-allocation move, without an allocation-count, peak-memory or timing claim.

The other four canonical-schema owners remain: exact minimal/representative fixture bytes; longest numeric spellings fitting the estimate; populated tagged-surface round trips; and empty console/insert round trips. All other #1118 retained families remain unchanged, including independent grammar/Unicode/span and schema checks, directed/generated finite-f32 round trips, typed diagnostic parity, deterministic mutation smoke, large-track/resource-first/transactional refusal, visitor/token tables and allocation ceilings. No test, fixture, generator, schedule or gate was retired.

**Five-axis boundary.** One short accessor removes the need for complete document copies at consuming call sites; no redundant helper or general abstraction is introduced. The existing independent test owns both canonical identity and the new move claim. Other snapshot/model/index ownership remains as justified by #1118. Control-plane String ownership has no arithmetic loop needing micro SIMD; lane/DSP implementations and target configuration are unchanged. Existing ordered indexes and resource structures are untouched.

**Actual checks.** Cargo used `CARGO_TARGET_DIR=/home/bl/misofm/engine/target/housekeeping-a` throughout.

- Focused locked canonical compilation test: **1 passed**, four other cases filtered, before the exact-path checkpoint.
- `cargo test --locked -p session --all-features` and the same command with `--release`: each **61 passed, 2 existing ignored, 0 doctests**, exit 0, no warnings. The ignores are the #391 json-syntax empty-object dependency sentinel and nightly exhaustive 2^32 f32 sweep; neither was launched. The package declares only an empty default feature set.
- `cargo clippy --locked -p session --all-targets --all-features -- -D warnings`, `cargo fmt --all --check`, `git diff --check`, `scripts/check-session-policy.sh` and `bash scripts/check-workspace-policy.sh`: exit 0.
- `cargo check --locked -p session --lib --all-features --target <target>`: exit 0 without warnings for `wasm32-unknown-unknown` with `RUSTFLAGS='-C target-feature=+simd128'`, `aarch64-apple-ios` and `aarch64-linux-android`. These establish library compilation, not browser/device execution.
- Root's final paired product snapshot `63cd5ee0`: `cargo test --locked -p session -p session-validator` independently passed **79 active tests**, with the same two existing session ignores and no warnings. This root integration result was inspected and reused, not repeated locally.

Logs: `/tmp/housekeeping-a-producer-{focused,session,release,clippy,fmt,wasm,ios,android,session-policy,workspace-policy}.log`; paired integration `/tmp/engine-housekeeping-session-validator-integration.log`. Prior complete audit/check evidence remains in #1118's recorded commit and `/tmp/issue1118-session-*.log`. No timed workload, new serializer/model/corpus/harness, DSP change, render allocation claim or new owner question. Product edits are frozen; root verdict and evidence synchronization are pending.

## Root adversarial verdict: PASS — attempt 1

Root Sol independently inspected the complete accessor/test diff, the artifact's owned fields and lack of Drop, the preserved borrowed and normalized-model assertions, and the actual focused/full debug/release/lint/compile evidence. Moving the private String preserves the validated bytes and destroys only control-plane fields on the calling thread; neither producer nor consumer is render work. Root also ran the paired source snapshot at 63cd5ee0: 79 active tests passed, no warnings, with only the two documented existing session ignores. No scalar/SIMD arithmetic, queue, schema, resource estimate or ABI identity changes.

Test value: the rewritten existing signed-zero/double-rounding canonical test rejects returning the wrong or empty document and cloning/rebuilding the consuming result into a different backing allocation, a path the existing borrowed accessor checks cannot exercise; its previous independent numeric and canonical assertions remain. The nonempty allocation pointer is compared while the returned String remains alive and is never dereferenced after consumption. No new test function or permanent baseline digest is needed. All bounded gates pass within their stated execution/compile limits.

The producer is complete; #1151 owns its already-checkpointed consumer adoption. Root will synchronize this evidence upstream and verify remote issue closure, then deliver the final tool/validator slice through required qualification. No unresolved owner decision belongs to this producer.
