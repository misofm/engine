You are Luna HIGH implementing attempt 1 for engine issue #544 in:
/home/bl/misofm/engine-audit-subject-disposition

Read AGENTS.md, .github/ISSUE_SPECS/544-runtime-audit-callers.md, artifacts/issue544-brief/sol-high-census.md, tools/audit/src/capi.rs, tools/audit/src/source_duration.rs, and the audit-native job in .github/workflows/qualification.yml. Do not perform broad historical rereads.

Scope and ownership:
- Edit only .github/workflows/qualification.yml.
- Do not edit audit source, dispatcher, Cargo files/lock, fixtures, scripts, DSP, specs/evidence, or any other repository path.
- Do not run git commit/push or GitHub mutations; root owns all Git/GitHub state.
- You may create evidence/capture helpers only below /tmp/issue544.
- This is exactly one coherent attempt/tranche. Stop after focused green results and report to Sol; do not begin another tranche.

Implement exactly two new invocations in the existing audit-native job, reusing its existing release audit build:
1. ./target/release/audit capi
2. ./target/release/audit source-duration

Each invocation must capture its original single JSON stdout record to a target/ file and validate it in the same workflow step (or two clearly bounded steps). Preserve stderr visibility/failure semantics. Use a strict Python JSON validator or jq. Validate exact current schemas and assertions without timing/RSS thresholds and without pin changes:

capi required exact key set:
schema_version, kind, calls, sample_rate_hz, quantum_frames, stable_output_address, pcm_digest, render_errors, allocations, deallocations, locks, feature_detection, logs, file_io, network_io, syscalls, panic_unwinds, total_violations.
Values: schema_version=1; kind=issue022_capi_render_audit; calls=100000; sample_rate_hz=48000; quantum_frames=128; stable_output_address=true; render_errors=0; all nine counters allocations/deallocations/locks/feature_detection/logs/file_io/network_io/syscalls/panic_unwinds=0; total_violations=0. Validate pcm_digest as exactly 16 lowercase hex characters, but do not pin its value.

source-duration required exact key set:
schema_version, kind, minute_frames, multi_hour_frames, minute_file_bytes, multi_hour_file_bytes, layout_entries, layout_total_bytes, layout_equal, source_report_equal, graph_report_equal, minute_rss_bytes, multi_hour_rss_bytes, os, arch, rust, timed_benchmark_invocations.
Values: schema_version=1; kind=issue041_source_duration_layout; minute_frames=2880000; multi_hour_frames=518400000; minute_file_bytes=11520044; multi_hour_file_bytes=2073600044; layout_entries=17; layout_total_bytes=6416; all three equality booleans=true; os=linux; timed_benchmark_invocations=0. Validate minute_rss_bytes and multi_hour_rss_bytes as nonnegative integers only, with no relationship/threshold. Validate arch and rust as nonempty strings only. Do not introduce any benchmark invocation.

Single-record validation must reject empty output, multiple JSON lines/records, non-object JSON, missing/extra keys, and bool-as-int confusion for integer fields. Preserve workflow triggers, routing, expectation table, leaf job identity, and the single required qualification context. Do not add dependencies or generic reachability machinery.

Execution/evidence protocol:
- Before editing, record current HEAD, exact cwd, and SHA-256 hashes of tools/audit/src/capi.rs and tools/audit/src/source_duration.rs (and qualification.yml before/after) in a new uniquely named manifest below /tmp/issue544. Include exact argv arrays for every build/test/audit/validator command you run.
- Set PATH=/home/bl/.cargo/bin:$PATH and CARGO_TARGET_DIR=/tmp/issue544-target for all Cargo builds/tests and real audit executions.
- Build/test existing audit in release locked: cargo build --locked --release -p audit, then cargo test --locked --release -p audit.
- Run each real audit subject exactly once. For each, preserve raw stdout, raw stderr, and numeric exit status in unique /tmp/issue544 files even on failure. Do not pipe execution through a validator in a way that loses the audit exit status.
- If either audit subject itself fails, stop immediately, preserve evidence, make no edits outside qualification.yml, and report the bounded failure. Do not rewrite audit source.
- Validate the captured raw stdout records using the same logical assertions added to the workflow, without rerunning either audit.
- Run cargo fmt --all -- --check. Validate YAML/workflow syntax using an already available local mechanism; do not edit/install/pin dependencies. If actionlint is absent, use a safe installed parser/checker and report exactly what was used.
- Inspect git diff/status and prove the repository edit is only .github/workflows/qualification.yml.

Final report must state: terminal/live status; exact repository path changed; exact commands and outcomes; captured evidence paths; audit numeric statuses; decisive observed JSON fields; syntax validation mechanism/result; git diff scope; any failure. Do not overwrite this runner's report output path yourself.
