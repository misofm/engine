# Share benchmark metadata command-output acquisition

Status: proposed numbered TOOL9 child of audit #349 and lane-B handoff #560, based on delivered main `735197b1bc94006eeb1c42ad447ab02bf61696d5` after #578 / PR #581 and successful post-main qualification `34163575770`. This continues one of the eight original partial findings; it does not start an original open finding. Sol HIGH coordinates and owns checkpoints, GitHub synchronization and delivery. Luna HIGH implements. Astra LOW performs every scope, source and exact-head/current-base verification assignment under the user's latest routing. No audio verification or artifact qualification is indicated.

## Smallest closable outcome

Make `bench-support::sysinfo` the single authority for executing an explicitly supplied metadata command and interpreting successful UTF-8 stdout. Replace the private duplicate acquisition bodies in the protocol and conformance benchmark modules with that authority while preserving their different subject policies and record projections.

The shared operation accepts a program and explicit argument slice and returns trimmed successful UTF-8 text while retaining successful empty output as a distinct value. Spawn failure, nonzero exit and invalid UTF-8 remain unavailable. Protocol continues to map unavailable or empty text to `"unknown"`. Conformance continues to map unavailable commit/status commands to `"unknown"`, successful empty `git status --porcelain` to `"false"`, and successful nonempty status to `"true"`.

This slice consolidates acquisition only. It does not merge metadata structs, schemas, record formatters or subject-specific fallback policy. TOOL9 remains partial afterward.

## Frozen semantics

- Run the requested program exactly once with the requested arguments and capture its output.
- A zero exit with valid UTF-8 returns trimmed text, including `Some("")` for empty or whitespace-only stdout.
- Spawn failure, nonzero exit and invalid UTF-8 return `None`; plausible stdout on a failing command does not become metadata.
- Stderr is neither metadata nor a fallback input.
- Protocol retains its existing empty/error `"unknown"` policy and exact metadata field spelling, escaping, numeric defaults and record projection.
- Conformance retains the semantic distinction between a clean empty workspace and command failure, plus its existing commit, dirty, incomplete-metadata and missing-field projections.
- Metadata collection remains control-plane benchmark scaffolding. No timed workload, benchmark result, performance claim or runtime behavior changes.

## Exact ownership

Allowed implementation and focused-test paths:

- `tools/bench-support/src/sysinfo.rs`
- `tools/bench/src/protocol.rs`
- `tools/bench/src/conformance.rs`
- this numbered spec and bounded evidence under `docs/audits/` or `artifacts/issue*`

Excluded paths include every crate or host runtime, DSP/effect/graph/bank/session/control implementation, other benchmark consumer, schema, timing loop, workload/corpus, manifest, lockfile, policy, workflow, SDK, browser, generated artifact and artifact pin.

Lane A #580 owns the frozen builtin endpoint/test/spec/evidence chain, cap preflight and bitwise PCM proof. This child must not edit or depend on those paths. #543/#555 precedes delivered #558/#552; #542's accepted work remains delivered through #567 without a fourth #542 attempt. Historical verdict provenance is unchanged.

## Objective gates

1. **Shared result law.** Deterministically exercise successful text with surrounding whitespace, successful empty output, whitespace-only success, nonzero exit with plausible stdout, spawn failure and invalid UTF-8. The shared result distinguishes successful empty from every failure class.
2. **Protocol projection.** Prove successful nonempty text is trimmed and retained, while successful empty, whitespace-only and every failure class map to exactly `"unknown"`. Preserve the existing synthetic protocol JSON record's field names, values, escaping and missing-data/numeric-default behavior.
3. **Conformance projection.** Prove clean successful empty status maps to `"false"`, successful nonempty status maps to `"true"`, and failed/unavailable status maps to `"unknown"`. Preserve commit acquisition and the existing synthetic conformance record, incomplete-metadata and missing-field projections exactly.
4. **Single acquisition authority.** Remove both private `Command::new(...).output()` acquisition bodies from protocol and conformance. Each consumer retains only its subject-specific conversion from the shared optional text. No second process wrapper or generic command framework is introduced.
5. **Proportional quality.** Existing and focused `bench-support` and `bench` tests pass in debug and release; affected strict Clippy/rustdoc, formatting, diff and applicable workspace/bench policies pass. Compile the supported native and Wasm targets only if the affected packages currently participate in those target gates. No benchmark or browser run receives credit.

## Stop and split triggers

Stop before widening this child if it needs a generic process abstraction, stderr capture, environment/current-directory control, timeout or streaming support, a schema/record change, another consumer migration, a manifest/lock change, a policy/workflow change, benchmark timing, a fixture corpus, host/runtime code or any audio/artifact work. Preserve the checkpoint and brief the independent outcome instead.

The slice is one small shared acquisition operation plus two existing consumer conversions. A coherent passing implementation tranche pauses for the root exact-path checkpoint before more work. Each attempt receives one Luna HIGH implementation pass and one Astra LOW adversarial verdict. After three failed attempts, preserve evidence and rescope without weakening gates.

## Preliminary residual audit

Astra LOW reviewed current main `735197b1bc94006eeb1c42ad447ab02bf61696d5` after #578. Protocol and conformance still each spawn a metadata command, require success and valid UTF-8, and trim stdout; `bench-support::sysinfo` already owns the delivered #557 command acquisition law. The only intentional consumer difference is empty-output policy, especially clean `git status --porcelain` in conformance.

This TOOL9 slice is smaller than IO5 live-state publication, host activation, lifecycle/clock, graph/bank/effect/parameter rollout or segment execution, and smaller than TOOL11's remaining declarative-rule contract. It is disjoint from active lane-A #580. Astra LOW returned conditional PASS for this product shape and is sufficient for all review. Activation still requires matching numbered local/GitHub identity, an exact pushed brief, current-base and ownership checks, and an Astra LOW scope verdict. No implementation is authorized by this preliminary audit alone.

## Numbered current-base scope review

Astra LOW returned **PASS** for exact pushed brief `873a7b5caa19d3d2672a8271f272ebbfebe7f6ea` on base and merge-base `735197b1bc94006eeb1c42ad447ab02bf61696d5`. GitHub #583 is open with matching number, title and exact body; the worktree is clean and pushed, the sole base-to-head delta is this spec, diff checks pass, and base post-main qualification `34163575770` succeeded.

The frozen law distinguishes successful empty stdout from spawn, exit-status and UTF-8 failures while leaving protocol and conformance projection policy local. The allowed paths and gates are sufficient for one half-day acquisition consolidation. Manifests, locks, policies, timing loops and artifacts remain excluded; conditional target wording does not claim unsupported Wasm execution of host commands. Ownership is disjoint from #580 at clean pushed head `517bbf486f84fdfd6d59c42f7683348b55cea781`. #580 and #583 occupy the two active slots, and the historical dependency/verdict order remains unchanged.

No corrections are required. Astra LOW is sufficient for this bounded non-audio verification. Luna HIGH attempt 1 may begin only from this reviewed brief/base and must pause at the first coherent focused-green tranche for root checkpointing. TOOL9 remains partial after this child.

## Attempt 1 implementation checkpoint

Luna HIGH delivered the first coherent implementation checkpoint at `e9deb0a84b1feec5520fab96bc75d9317572291b`. `bench-support::sysinfo::command_output` is now the one `Command::output` authority across the three owned files. It executes the explicit program/argument pair once and returns trimmed successful UTF-8, retaining empty or whitespace-only success as `Some("")`; spawn failure, nonzero exit with plausible stdout and invalid UTF-8 return `None`. Existing `HostToolchainFacts` and physical-core collection also consume that shared law.

Protocol removes its private process acquisition and retains the local nonempty filter plus `"unknown"` fallback. Conformance removes its private process acquisition and retains `command_allow_empty`, so clean empty status remains distinct from failure before mapping to `"false"`, `"true"` or `"unknown"`. Unix deterministic fixtures exercise text/whitespace, empty success, nonzero stdout, spawn failure and invalid UTF-8 in the authority and both consumer projections. Existing synthetic JSON record fixtures remain unchanged.

Implementer gates passed: `bench-support` debug 40 tests, complete `bench` debug 39 tests, focused release `sysinfo` 12 tests, protocol 5 tests and conformance 3 tests, strict affected Clippy/rustdoc, formatting and diff checks. The complete release `bench-support` suite reports only the unchanged `alloc::tests::current_thread_counts_every_allocator_operation` compiler-optimization failure: candidate 39 passed/1 failed and exact base `735197b1` 38 passed/1 failed with the same observed versus expected counters. Neither #583 source nor ownership includes `alloc.rs`; generated `Cargo.lock` ordering from both reproductions was restored and the detached base worktree removed cleanly.

The implementation remains limited to the three authorized source/test files. No manifest, lock, policy, timing, workload, runtime, browser, artifact or #580 path changed, and no benchmark ran. Complete proportional evidence and Astra LOW adversarial attempt-1 review remain pending before any PR or delivery claim.

## Attempt 1 source verdict

Astra LOW returned **FAIL** at exact pushed head `d4f9120ec0ea0b9d0d9c7b445accdc78809fd1e4`, frozen source `e9deb0a84b1feec5520fab96bc75d9317572291b`, base and merge-base `735197b1bc94006eeb1c42ad447ab02bf61696d5`. The shared acquisition implementation satisfies the frozen law, Unix fixtures discriminate its success/failure cases, both duplicate consumer acquisition bodies are removed, and no product defect, scope drift, manifest/lock change or #580 overlap was found.

Attempt 1 fails because conformance tests the shared optional-text result through `command_allow_empty` but does not directly exercise production's final `workspace_dirty` conversion. Attempt 2 must factor only that existing conversion into a narrow helper used by `Metadata::gather` and assert successful empty or whitespace maps to `"false"`, successful nonempty maps to `"true"`, and unavailable maps to `"unknown"`. The existing synthetic record oracle stays unchanged. No other product or path change is authorized.

Astra LOW independently passed the full debug suites (40/39), focused release suites (12/5/3), strict affected Clippy/rustdoc, format/diff, workspace and bench policy/mutation checks. Complete release `bench-support` fails identically on candidate and exact base with observed counters `(2,1,1,48)` against expected `(3,2,1,64)`; this pre-existing allocator failure remains outside #583. Delivery review is blocked until the bounded attempt-2 correction receives source PASS.

## Attempt 2 correction checkpoint

Luna HIGH completed the sole attempt-1 correction at source `9ed73fa5ebe030007a45ae41fc5c417e49c039c7`. Only `tools/bench/src/conformance.rs` changed. `Metadata::gather` now passes the existing `command_allow_empty` result through a private `workspace_dirty` conversion. The same conversion is directly tested for empty and whitespace-only success → `"false"`, nonempty success → `"true"`, and unavailable → `"unknown"`. Command acquisition, the synthetic record oracle and all other source remain unchanged.

Focused conformance debug and release each pass 4 tests, complete `bench` debug passes 40 tests, and strict bench Clippy, formatting and diff checks pass. No benchmark ran and no manifest, lock, policy, evidence, artifact or #580 path changed. Astra LOW attempt-2 adversarial review remains pending before delivery.

## Attempt 2 source verdict

Astra LOW returned **PASS** at exact pushed head `215ca939d5e23b74372b65b286d81a0617fe70d2`, correction source `9ed73fa5ebe030007a45ae41fc5c417e49c039c7`, base and merge-base `735197b1bc94006eeb1c42ad447ab02bf61696d5`. Production directly uses the tested clean/dirty/unavailable conversion; the synthetic record oracle and all other attempt-1 source remain unchanged. One command-output acquisition authority and both consumer policies are preserved with no manifest, lock, policy or #580 overlap.

Independent conformance debug/release 4/4, complete bench debug 40, strict affected Clippy/rustdoc, format/diff, workspace and bench policy/mutation gates pass. The exact-base release allocator failure remains unrelated and unrepaired. GitHub/spec identity, clean exact upstream and #580 head `517bbf486f84fdfd6d59c42f7683348b55cea781` pass review.

Source is accepted on attempt 2. No audio artifact probe, qualification, pin change, browser run or benchmark is required. Final documentation-only exact-head/current-base review, PR CI, merge, post-main qualification, GitHub closure and clean-worktree removal remain pending. TOOL9 remains partial.
