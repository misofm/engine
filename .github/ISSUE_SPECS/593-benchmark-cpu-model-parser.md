# Share exact-prefix benchmark CPU-model parsing

Status: proposed numbered TOOL9 child of audit #349 and lane-B handoff #560, based on delivered main `566810a9f249f26d96edc4d65b22bd9b7c649976` after #590 / PR #591 and successful post-main qualification `34171818628`. This advances an original partial finding, not an original open finding. Sol HIGH coordinates and owns checkpoints, GitHub synchronization and delivery. Luna HIGH implements. Astra LOW performs every scope, source and exact-head/current-base verification. No audio, browser or artifact qualification is indicated.

## Smallest closable outcome

Move the identical pure `/proc/cpuinfo` model-line parser used by conformance and protocol benchmarks into `bench_support::sysinfo`. Migrate only those two production callers. Preserve the exact prefix `"model name\t: "`, first-match selection and suffix bytes, including an empty suffix, trailing whitespace and Unicode. Unavailable input or no exact-prefix match remains exactly `"unknown"` in each caller.

Preserve acquisition: protocol continues to read only its current CPU source and must not call `HostToolchainFacts::gather()` or acquire unrelated facts. Session's trimmed colon-based parser and graph's shell extraction are intentionally distinct and excluded. This removes one concrete parsing-law duplicate; TOOL9 still requires a closure audit afterward.

## Exact ownership

Allowed implementation paths are:

- `tools/bench-support/src/sysinfo.rs`
- `tools/bench/src/conformance.rs`
- `tools/bench/src/protocol.rs`
- this numbered spec and bounded issue evidence

Exclude acquisition changes, every other consumer/parser/metadata policy, schemas, JSON, statistics, numeric parsing, timing/workloads/corpora, manifests/lockfiles, policies/workflows, runtime/host/DSP/session/control code, SDK/browser code, generated artifacts and pins. Active lane-A #587 / PR #592 owns its documented host-core source and AudioWorklet qualification paths; there is no overlap.

## Objective gates

1. The shared pure parser covers unavailable and empty input; unmatched or differently spaced prefixes; an exact match; first of multiple matches; empty suffix; and retained trailing whitespace and Unicode.
2. Conformance and protocol production callers both use the shared parser and their duplicate parser bodies disappear. Protocol acquisition remains local and unchanged; conformance retains its existing `HostToolchainFacts` acquisition.
3. Existing conformance and protocol record oracles pass unchanged, proving no schema, sentinel or field drift.
4. Focused bench-support/conformance/protocol tests pass in debug and release; complete affected bench debug and proportional release tests pass. Strict affected Clippy/rustdoc, formatting/diff, workspace policy, bench policy and its mutation suite pass. Preserve the unrelated release allocator disposition. No timed benchmark, browser or artifact run receives credit.

## Stop and split triggers

Stop before normalizing broader CPU formats, changing acquisition, migrating another consumer, changing a schema/sentinel, introducing a generic parser framework, or touching a manifest/lock, policy/workflow, workload/timing, runtime/audio/browser/artifact path. Preserve the checkpoint and brief a separate successor if the exact duplicate cannot be removed within the three source files.

One Luna HIGH attempt receives one Astra LOW adversarial verdict and pauses at the first coherent focused-green tranche for root checkpointing. After three failed attempts, preserve evidence and rebrief without weakening gates. Historical #543/#555 → #558/#552 and #542 → #567 delivery order and verdict provenance remain unchanged.

## Preliminary residual audit

Astra LOW reviewed current main `566810a9f249f26d96edc4d65b22bd9b7c649976`. Conformance and protocol both select the first line beginning with exact prefix `"model name\t: "`, preserve its suffix verbatim, and project unavailable/unmatched input to `"unknown"`. Percentile algorithms, six-field summaries, command acquisition, environment fallback and JSON string-array assembly already use delivered shared authorities. Rack ASCII validation, builtins control-character rejection, interchange failure policy and schema-specific field assembly remain intentional differences.

TOOL11 and IO5 require broader contracts. #587 / PR #592 and its source/artifact worktrees are active but disjoint. Astra LOW is sufficient for all review. Activation requires exact local/GitHub numbered identity, a pushed clean brief, current-base and ownership checks, and Astra LOW scope PASS.
