# Share session and conformance host/toolchain fact acquisition

One-line summary: Make `bench-support::sysinfo` the single acquisition point for the host, Rust toolchain, and common runner-environment facts duplicated by the session and conformance benchmark subjects, while preserving both local metadata records, CPU projections, unavailable fallbacks, timestamps, and emitted schemas exactly.

## Parent finding and current baseline

This is the next independently closable slice of audit #349 TOOL9 after delivered #554 / PR #556. Baseline is clean synchronized `main` `30f658ee1c0c7d86002f5f2fea075a5dfa8a7c2c`; required run `34111293519` succeeded. Delivered #554 made the shared six-field percentile summary authoritative, but explicitly left TOOL9's metadata residual partial.

The original audit count of seven local `Metadata` structs was a discovery clue, not an acceptance oracle. Current source behavior is the scope oracle. The live metadata census is:

| Surface | Current acquisition/projection | Decision here |
| --- | --- | --- |
| `tools/bench/src/session.rs:285-414` | Repeats CPU-file, logical/physical-core, kernel, governor, `rustc -V/-Vv`, toolchain-field, and eight common runner-variable acquisitions; projects them into the session schema, adds `runtime_or_browser`, and uses session-specific CPU parsing and `rustc_version`/missing names. | Consolidate only the common acquisition. Keep the local record and projection. |
| `tools/bench/src/conformance.rs:178-317` | Repeats the same acquisition tail; projects it into the conformance schema, adds git commit/dirty state, uses a stricter CPU line parser and the field name `compiler`. | Consolidate only the common acquisition. Keep git and the local record/projection. |
| `tools/bench-support/src/sysinfo.rs` | Already owns the one physical-core `lscpu -p=CORE,SOCKET` probe and parser shared by these two subjects. | Extend this existing authority; do not add a module or crate. |
| `tools/bench/src/graph.rs` and `protocol.rs` | Overlap at low-level sources but retain different placeholder, zero, OS/kernel, and missing-list behavior. | Out of scope; later reviewed decision. |
| `tools/bench/src/rack.rs` and `builtins.rs` | Share a 16-variable runner inventory but have different Unicode, numeric, placeholder, null, and missing-list laws. | Out of scope; separately mapped next slice. |
| `tools/bench/src/effect_interchange.rs` | Uses a different required/optional environment vocabulary and lowercased missing names. | Out of scope. |
| `tools/bench/src/effect_contract.rs` | Rewrites quotes before embedding metadata. | Out of scope correctness/escaping decision. |
| `tools/audit/src/record.rs`, `tools/bench/src/console.rs`, `tools/wasm-console/src/main.rs` | Consume the delivered shared environment snapshot/record projection. | Delivered, not residual scope. |

This issue does not remove either local `Metadata` type or claim TOOL9 completion. It removes the one proven duplicate acquisition family and leaves every non-equivalent projection visible for later disposition.

## Frozen shared acquisition contract

Extend `tools/bench-support/src/sysinfo.rs` with one public, unversioned `HostToolchainFacts` value and `HostToolchainFacts::gather()`. The value carries only the common intersection needed by both subjects:

- raw readable `/proc/cpuinfo` text as `Option<String>` so each subject retains its own parser;
- physical-core count;
- logical-core count;
- kernel release;
- power source;
- governor or power mode;
- `rustc -V` text;
- LLVM version and host target parsed from `rustc -Vv`;
- opt level, LTO, codegen units, target CPU, compile target features, and background-load note.

Use the existing memoized `bench_support::metadata::Metadata::gather()` snapshot for environment values. Do not add another environment snapshot or read live process variables directly.

The collector remains control-plane benchmark scaffolding. It may allocate, spawn bounded commands, and read the two current host files. It is never called from a timed closure or product/realtime code.

### Exact value/fallback law

Preserve the current observable acquisition behavior:

1. `rustc -V`, `rustc -Vv`, `uname -r`, and `lscpu -p=CORE,SOCKET` are bounded one-shot commands. Spawn failure, unsuccessful status, non-UTF-8 stdout, and empty trimmed stdout become `"unknown"`, except that the existing `lscpu` parser continues to map empty/header-only/unparsable output to `"unknown"` and otherwise returns the distinct core/socket-pair count.
2. A readable UTF-8 `/proc/cpuinfo` is retained raw. Missing/unreadable/non-UTF-8 input is `None`. Do not choose one CPU model parser in the shared layer.
3. Logical core acquisition remains `std::thread::available_parallelism()` rendered as a decimal positive count, with `"unknown"` on error.
4. A successful governor-file read is trimmed and retained even when the trimmed string is empty. Only a file read failure falls back to `MISO_ENGINE_BENCH_GOVERNOR_OR_POWER_MODE`; an absent, non-Unicode, or empty fallback variable becomes `"unknown"`.
5. Each other common runner variable returns its exact nonempty Unicode snapshot value. Absent, non-Unicode, or empty values become `"unknown"`.
6. `LLVM version: ` and `host: ` retain the first exact-prefix remainder from normalized `rustc -Vv`; missing fields become `"unknown"`.

Implement the source boundary so bench-support unit tests can inject command results, file results, parallelism, and environment values without modifying the process environment or PATH. Keep that test seam private to `sysinfo.rs`; do not create a generic command/filesystem framework or expose a public mock trait.

## Frozen local projections

`session.rs` and `conformance.rs` consume one gathered `HostToolchainFacts` each, then build their existing local `Metadata` records.

Preserve these intentional differences:

- Session CPU parsing accepts a line whose name trimmed around `:` equals `model name`, and trims the value. Conformance accepts only the exact `model name\t: ` prefix and retains the remainder as it currently does. Missing or unparseable raw CPU text becomes `"unknown"` in either local projection.
- Session emits the shared Rust version under `rustc_version`; conformance emits it under `compiler`.
- Session alone reads `MISO_ENGINE_BENCH_RUNTIME_OR_BROWSER` through the existing shared environment snapshot/fallback law.
- Conformance alone gathers `git rev-parse HEAD` and `git status --porcelain`. Preserve its exact command failure/non-UTF-8 behavior and `workspace_dirty` mapping: unavailable is `"unknown"`, empty successful output is `"false"`, and nonempty successful output is `"true"`.
- Each timestamp remains acquired after its subject's metadata inputs and uses its existing `SystemTime` error message. Do not move wall-clock acquisition into the shared collector.
- Each local field list remains authoritative for `missing_metadata`. Preserve exact field names and current declared order. Only the literal value `"unknown"` is missing; an empty governor value from a successful file read is retained and is not added.
- Preserve every JSON key, key order, spelling, string escaping, scalar representation, runtime literal, schema version, fixture identity, percentile, and `metadata_incomplete` result.

Delete the duplicated common `variable` and host/toolchain command/field helpers only where their last common use disappears. Conformance may retain the narrow command helper needed for its git-only fields. Do not route git through the shared collector.

## Allowed paths

Implementation and tests may change only:

- `tools/bench-support/src/sysinfo.rs`;
- `tools/bench/src/session.rs`;
- `tools/bench/src/conformance.rs`;
- `scripts/check-bench-policy.sh`;
- `scripts/test-bench-policy.sh`;
- `.github/ISSUE_SPECS/557-shared-session-conformance-host-facts.md` and issue evidence paths under root ownership.

No manifest, lockfile, other benchmark subject, validator, runner, fixture, artifact, record schema, environment vocabulary document, product crate, or host crate may change.

## Structural policy boundary

Extend the existing benchmark policy narrowly for these two subjects:

1. Both `session.rs` and `conformance.rs` must call `HostToolchainFacts::gather()`.
2. Neither may directly retain the common acquisition spellings: `/proc/cpuinfo`, the scaling-governor path, `lscpu`/`CORE,SOCKET`, `available_parallelism`, common `rustc`/`uname` probes, the shared verbose-field prefixes, or the seven common non-governor runner-variable names.
3. The session-only runtime variable and conformance-only git commands remain allowed.
4. Add one mutation that restores a direct common probe in one subject and fails for that exact reason, plus one mutation that removes a required delegation and fails. Keep the existing checker structure and all current mutations; do not add a framework or campaign.

The policy must distinguish a clean no-match from grep execution failure and preserve the existing checked-operation diagnostics style.

## Objective finite gates

All gates are untimed. Do not invoke `bench session`, `bench conformance`, a benchmark runner, or any workload/timing command.

1. `cargo test --locked -p bench-support sysinfo::tests` passes. Add deterministic injected-source coverage for:
   - a fully populated source;
   - spawn failure, unsuccessful command status, non-UTF-8 and empty command output;
   - missing/non-UTF-8 CPU and governor files;
   - unavailable logical-core count;
   - absent, non-Unicode, and empty environment values;
   - successful whitespace-only governor content remaining empty rather than becoming `"unknown"`;
   - existing physical-core distinct-pair, duplicate, empty, whitespace-only, and header-only cases.
2. Add and run exact pure tests named `session::tests::shared_host_toolchain_facts_preserve_session_projection` and `conformance::tests::shared_host_toolchain_facts_preserve_conformance_projection`. Feed the same synthetic shared facts containing:
   - CPU lines that deliberately make the two parsers produce their distinct current values;
   - at least one `"unknown"`, one empty successful-governor value, and one Unicode environment value;
   - fixed Rust/LLVM/target strings.

   Each test pins its local field names/values, exact `missing_metadata` names and order, `metadata_incomplete`, and an exact ordered JSON metadata substring. Session additionally pins `runtime_or_browser`; conformance pins fixed injected git/dirty projections. Do not use the implementation to generate expected strings.
3. `cargo test --locked -p bench --bin bench session::tests::shared_host_toolchain_facts_preserve_session_projection -- --exact` passes.
4. `cargo test --locked -p bench --bin bench conformance::tests::shared_host_toolchain_facts_preserve_conformance_projection -- --exact` passes.
5. `cargo test --locked -p bench --bin bench` passes. These are unit tests only.
6. `bash scripts/check-bench-policy.sh` and `bash scripts/test-bench-policy.sh` pass, including the two new directed mutations and every existing mutation.
7. Static census over `tools/bench/src/{session,conformance}.rs` confirms both delegate and contain none of the common acquisition spellings. Inspect the remaining `Metadata`, command, CPU parser, runtime, git, timestamp, and missing-list code and record every retained local owner. A workspace-wide census must still list graph/protocol/rack/builtins/effect-interchange/effect-contract as unresolved or intentionally deferred TOOL9 rows; do not claim they were consolidated.
8. `cargo clippy --locked -p bench-support -p bench --all-targets -- -D warnings`, `cargo fmt --all -- --check`, `bash scripts/check-workspace-policy.sh`, and working-tree/full committed `git diff --check` pass.
9. Evidence records exact base and implementation heads, changed paths, raw command/status outputs, relevant source hashes, and an explicit diff audit showing no record-format string, key order, schema, fixture/hash, percentile, workload, timer, or environment-name contract changed.

Required remote `qualification` and exact-head Sol XHIGH review remain delivery gates after root checkpoints the coherent candidate.

## Attempt and review workflow

Actual Luna HIGH implements attempt 1 under Sol HIGH coordination. The worker edits source and runs the focused gates, then pauses with one coherent uncommitted tranche. Root alone audits, commits, pushes, synchronizes the issue, and launches review. Actual Sol XHIGH reviews the frozen whole attempt against this brief, concentrating on injected unavailable behavior, the deliberately different CPU parsers and local fields, timestamp/git/runtime ownership, missing-list order, exact record substrings, policy mutation sensitivity, and the absence of timed work. Up to three attempts total; no gate or fallback may be weakened.

## Completion

Completion means session and conformance acquire their common host/toolchain facts through the one shared authority, preserve both local projections and exact synthetic record bytes, pass every finite gate, receive Sol XHIGH PASS, pass required CI, merge, synchronize/close the matching GitHub issue, and clean the completed worktree.

TOOL9 remains `PARTIAL` after this slice. Rack/builtins raw inventory, graph/protocol acquisition differences, effect-interchange vocabulary, effect-contract escaping, remaining inline numeric summaries, and schema-specific record writers retain their separately mapped decisions. This issue closes none of those residuals.
