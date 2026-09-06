# Enforce complete Rust/jq console floor parity in existing CI tests

Ready-to-number bounded TOOL-8 brief; queued independent tooling only. Inspected delivered main59f35c62 source in engine-480-proof. No implementation, test/build, workload or timing execution. Root must number/synchronize before assignment; do not displace238 runtime priority.

## Problem and smallest outcome

The Rust floor_row function and jq floor_pins object independently encode the same console floor inventory. Existing Rust law tests and jq validator mutations do not compare the full maps. There is no current standalone untimed floor-table export to reuse: console emission invokes the workload, and operator preflight only hashes floor.rs. Do not run console or introduce a new benchmark/export CLI for this check.

Smallest closable implementation: one native unit test in the EXISTING floor.rs tests module constructs a JSON object directly from actual console_workload::WORKLOADS and actual floor_row values, invokes the existing jq library, and compares complete map equality. This uses private production values without source parsing, another literal Rust table, a parser dependency or public API. The existing native CI bench unit-test invocation already executes it.

## Exact paths and representation

Allowed code path: tools/bench/src/floor.rs, cfg(test) section only. Numbered spec/evidence are the other allowed changes. scripts/console-benchmark-record-lib.jq, console workload, record schema, Cargo/configuration, benchmark runner/preflight, historical records and floor arithmetic remain unchanged. No new shell harness, general helper or additional workflow call is necessary: qualification.yml's existing `cargo test --locked --release -p audit -p bench -p console-workload` includes this test. Its test cannot be ignored, conditionally skipped on missing jq, or hidden behind an unrequested feature.

Use WORKLOADS (currently16 session workload variants) as Rust's population. For every workload, key is workload.kind(). For Some(FloorRow), serialize its actual lane_ops, width_factor, control.kind() or "none", and basis. For None, preserve the existing record/validator convention `[null,1,"none","not_derived"]`; assert the known baseline's absence as existing tests already do. Reject duplicate emitted keys explicitly rather than letting JSON object overwrites hide one. Use existing bench_support::json::escape for strings and round-trip f64 formatting, not three-decimal record presentation. The test may access private fields through its module. Do not infer data by regex over Rust or jq source.

Resolve scripts directory from CARGO_MANIFEST_DIR, not process CWD. Execute jq directly with Command and checked stdin/output/status (no shell interpolation). Feed the generated object through stdin, use `-L` to the existing scripts directory and `include "console-benchmark-record-lib"; floor_pins` inside the comparison expression. Compare entire objects, including key sets, tuple lengths, nulls, all numeric components and exact strings. A tiny test-local function may return the checked boolean comparison result, with explicit error for spawn/write/wait/jq parse failure; require successful jq execution and exactly one boolean result. A missing tool/import or syntax failure must fail the test, never count as a mismatch proof.

This introduces a native test prerequisite on jq, already used by the repository's Linux qualification gates. State it in the numbered execution record; no Windows/Wasm matrix or new tool installation framework is required. floor module itself is native-only.

## Finite objective gates

1. New named test `floor::tests::rust_and_jq_floor_tables_have_exact_key_value_parity` compares the actual complete16-row Rust map against actual jq floor_pins and returns true. Verify nonempty actual test execution. Both key-set equality and tuple values are mandatory; iterating only Rust-known keys in jq is insufficient.
2. Through the SAME comparator, transform the actual jq-produced object in three bounded cases: delete one existing derived key; add one distinct extra key with an otherwise valid tuple; change one existing numeric inventory value. Each must complete jq successfully and return false at the intended equality assertion. Keep the immutable Rust object unchanged. Do not replace the comparator, use fabricated unrelated fixture maps, or accept arbitrary subprocess failure. These three divergences discriminate missing, extra and value drift without production-source mutations/rebuilds.
3. Existing null baseline, ragged16/9 width, control strings and basis strings all participate in full object comparison. No new numerical tolerance or repricing is allowed. Restore/unmodified comparison remains true after the negative cases. If current maps genuinely disagree, stop with exact key/field values for an owner ruling; do not make the test green by selecting an intersection, omitting columns or changing pins.
4. Focused execution: `cargo test --locked -p bench floor::tests::rust_and_jq_floor_tables_have_exact_key_value_parity` and the corresponding --release command; existing `cargo test --locked -p bench floor::tests` plus strict affected Clippy/fmt and existing console-validator suite. Unit-test execution is untimed validation, not a benchmark invocation. No `bench console`, runner, clock/cycle collection, full new fixture corpus or capture authority.

After source PASS, proportional parent-free tooling delivery and actual-head Astra/requiredCI precede closure. The existing required native bench-test job is the permanent gate; no duplicate CI build should be added. Historical LANE-13/#368 arithmetic correction stays delivered and unchanged. This issue closes continuous full-table parity only, not future floor derivation questions, Class-B decisions or historical capture deduplication.

Workflow: Astra scopes/reviews; fresh Luna1, Sol2/3 only after failure, one consolidated verdict each and hardstop/rescope after third failure. Root owns all checkpoints/upstream synchronization. No implementation is authorized by this unnumbered brief.

## Numbered implementation baseline

GitHub483 matches the title and this numbered spec. Base main024ad674789a96390bcc45a754931ef5119c8b59 contains merged PR482; the exact floor/validator/workload/Cargo inputs are unchanged from Astra-inspected59f35c62. Pending numbered Astra approval; no implementation assigned yet. Existing238 remains the active runtime feature; this test-only tooling is independent.

## Numbered scope approval and Luna attempt 1

# Astra #483 numbered scope review — PASS

Exact planning headba64710039bd865f3d43265f8b9e5ccd10bddf7d, engine-483-floor-parity, delivered base024ad674789a96390bcc45a754931ef5119c8b59. Local numbered spec contains the approved draft verbatim plus the correct numbered baseline. Live GitHub483 is OPEN with matching title “Enforce complete Rust/jq console floor parity in existing CI tests”. Root reports synchronized body; this review independently checked title/number/state.

Relevant floor, jq validator, workload and Cargo inputs have no delta from inspected59f35c62. Planning delta is only the numbered spec. Approve the smallest cfg(test)-only floor.rs comparison of actual production values against the existing jq object, with full keys/tuples and the same comparator's missing/extra/value divergences. Existing native bench CI execution supplies the gate; no new CLI, workflow, parser/framework, repricing, historical pin change or actual workload/timing is allowed. Missing jq/import/process errors must fail independently, not count as successful negative controls.

The historical wording “unnumbered brief” is superseded by the numbered baseline and this approval; it does not forbid root's next assignment. Root may assign fresh Luna1 after its checkpoint audit. #238 remains the sole active runtime feature; this test-only tooling is independent. Existing Astra review/Sol retry/three-failure hard stop and actual-PR/requiredCI/remote delivery rules apply. No code, tests, builds or timing performed.

Root adopts PASS and assigns fresh Luna attempt1. Pause at a coherent compiling/focused-green tranche before further implementation so root can checkpoint exact paths. All test commands are untimed; no benchmark runner/capture is authorized.

## Luna attempt 1 source checkpoint

The cfg(test)-only floor.rs test builds the actual complete Rust table and compares the existing jq floor_pins object, including successful missing/extra/value divergences and restored equality. Native jq is a required test prerequisite. Raw /tmp/483-luna1-{debug,release,floor-suite}.log show the named test passing in both profiles and all9 floor tests passing. Existing validator log reports zero real runner/workload/timing invocations. Bench-only strict Clippy --tests --no-deps passes.

The full strict Clippy log fails at three inherited builtins-compiler question_mark sites introduced before this branch; that source is unchanged here. This is preserved as a failure, not a full lint PASS. The initial cargo PATH failure and empty successful-format log are retained; these logs lack standalone numeric status records, so root will capture explicit command/status evidence for delivery. Astra must independently rule scope and gates; no source acceptance is claimed by this checkpoint.

## Astra attempt 1 source acceptance

# Astra #483 Luna1 source verdict — PASS

Exact d9a3413911e6f95f98c2134a5fa17d85c4b337f2, engine-483-floor-parity. Read full numbered scope, cfg(test)-only floor delta, Luna logs and root's independently captured six command/status sets. No builds/tests/timing or repository/Git/GitHub edits performed.

The test serializes actual WORKLOADS/floor_row private production values with duplicate-key detection, finite numeric fields and shared JSON escaping. It does not parse source or copy the floor table. jq imports the actual existing floor_pins object; entire-object equality checks key set, tuple components and exact strings/null conventions. Missing, extra and numeric-value transformations apply to that same actual jq map, compare against the SAME unchanged Rust input through the same helper, require successful subprocess completion and false result, then restored actual equality passes. A tool/import/parse failure is an Err and cannot satisfy a negative comparison. The scripts path comes from CARGO_MANIFEST_DIR and no shell/workload dispatcher is involved.

Root evidence has numeric0 for fmt/debug/release/floor-suite/affected-Clippy/validator. Named parity test executes once in debug/release; existing floor suite executes9. Existing native bench CI test command runs this test without a new workflow call. No runtime/pin/validator arithmetic changes or real benchmark invocation occurred.

The full strict dependency Clippy failure is real but inherited: three unchanged builtins-compiler normal-feature question_mark sites. For this cfg(test)-only bench slice, strict `cargo clippy --locked -p bench --tests --no-deps -- -D warnings` is the proportional affected lint gate and passes. Do not call the broader command passing or modify dependency runtime code in483. A separately numbered minimal repair should preserve its error-time witnesses; draft supplied separately. Required actual PR/CI still governs delivery and cannot be waived if that CI reports a blocking failure.

PASS permits proportional packaging/actual-head Astra review and required CI. Root must synchronize483 only on delivery; TOOL8 parity is the precise closure, not floor repricing or a broader audit completion. Historical source logs without separate exit files are distinguished from the root recapture, not retroactively assigned numeric provenance.

Raw evidence is retained in artifacts/issue483-console-floor-parity with a hash/size manifest. Root also captured environment vocabulary and workspace policy checks separately. Actual PR-head review and required qualification remain pending; issue483 stays OPEN until merged evidence is upstream.

## Delivered and closed

PR484 merged2026-09-06T01:44:58Z as4587bfae673adbb848cda66f473beecefeae8deb. Exact-head Astra PASS covers de57f7bffe123e42650109db6869c94adfe96bd4; required qualification34004423637 completed SUCCESS before merge. GitHub483 is verified CLOSED. TOOL8 continuous complete Rust/jq parity is delivered; this makes33 merged audit PRs and108 audit entries retaining work. Post-main qualification is tracked separately and no timing improvement is claimed.
