# Restore the truthful dense-invalid maximum-document Web boot gate below one second

## Objective

Restore a truthful bounded-diagnostic qualification of the production Web boot refusal path for an
exactly 1 MiB Session V1 JSON document containing more than 10,000 structurally valid but
semantically invalid automation segments, and keep that complete refusal below the existing strict
one-second launch ceiling. This is a bounded successor to issue #338 after its hard three-attempt
stop; it is not a fourth #338 revision, a benchmark retry, or permission to weaken the ceiling.

## Baseline, failed run and proven contradiction

The exact baseline is corrective head `0256b854` on PR #339, based on `origin/main` `51468d5d`.
The remote engine CI invocation reached the existing host-web test
`maximum_document_dense_invalid_boot_is_typed_and_finishes_under_one_second` and measured
`1.133843116s` against the strict `<1s` assertion at `hosts/host-web/src/tests.rs:501`. The other
57 host-web tests and every other completed engine, browser, SDK and fuzz leg passed. Preserve this
result as the failed baseline: an unchanged rerun or a later green retry cannot replace it as #338
evidence.

The Session JSON conversion also made the timing fixture contradict its stated claim. Its repeated
records have `start_sample == end_sample`, but the JSON footer appends eleven empty `{}` segment
objects. Those tail records produce the 64 reported `schema.missing_field` parser diagnostics.
Because `parse_session_json` calls `validate_session` only when parser diagnostics are empty, none
of the repeated invalid ranges reaches the semantic validator. The test comment still says that
every segment reaches that validator, and `hosts/host-web/MUTATIONS.md` still attributes the test's
bound to the semantic validator's 64-diagnostic guard. The assertion, comment and mutation record
therefore no longer prove the same behavior.

The remote result is a real miss of a named launch ceiling, not a preflight integration omission.
Its phase cause is not yet established: the timed region includes strict JSON parsing, typed model
construction, semantic validation, source-span recovery, diagnostic encoding and the complete
production `AudioWorkletEngineHost::boot` refusal. The current schema-invalid footer prevents the
run from measuring its claimed semantic path. Do not label the miss an algorithm regression until
the truthful path is restored, and do not label it harmless debug-runner variance: the established
gate explicitly calls this a fixed CI wall and the required workflow deliberately runs the debug
workspace test command.

## Smallest closable slice

Repair the existing in-test JSON construction so the exact 1,048,576-byte document contains more
than 10,000 structurally valid automation segments and no schema-invalid sentinel records. Each
segment remains semantically invalid because `start_sample == end_sample`; insignificant trailing
JSON whitespace may fill the exact byte ceiling. Restore the semantic `automation.invalid_range`
refusal, its bounded 64-line diagnostic output and the unchanged complete-boot timing assertion.

First use code inspection and deterministic focused assertions to establish that the corrected
fixture exercises the intended phase. If the truthful production path cannot meet the existing
wall, make only the smallest evidenced control-plane parser/semantic-diagnostic optimization needed
for that same fixture and result. Do not introduce a new parser, general performance framework,
second large corpus or alternate host path.

### Always allowed paths

- `.github/ISSUE_SPECS/342-restore-the-truthful-dense-invalid-maximum-document-web-boot-gate-below-one-second.md`
- `hosts/host-web/src/tests.rs`
- `hosts/host-web/MUTATIONS.md`

### Conditionally allowed paths

The following paths may change only if the corrected fixture and phase-specific evidence show that
production parser or semantic-diagnostic work is required to satisfy the unchanged gate:

- `crates/session/src/parse.rs`
- `crates/session/src/validate.rs`
- one existing focused Session parser/validation test or mutation record needed to pin the exact
  optimization independently of elapsed wall time

Record the phase evidence and why each conditional path is necessary before changing it. No other
tracked path may change.

### Forbidden scope

- increasing the one-second threshold, changing strict `<` to `<=`, adding a tolerance, retry,
  averaging, warmup, sleep, environment multiplier, conditional skip or ignored-test behavior;
- moving the assertion to release-only qualification, changing Cargo profiles, serializing or
  otherwise reshaping CI solely to obtain a green time, changing runner labels, or editing any
  workflow;
- rerunning the unchanged failed test as acceptance evidence, selecting the fastest result, or
  treating a green retry as cancellation of the `1.133843116s` baseline;
- shrinking the exact 1 MiB document, reducing the population below 10,000, moving invalid records
  out of the timed production path, retaining empty/schema-invalid sentinels, or changing the
  semantic invalid-range condition;
- changing diagnostic codes, paths, ordering, the 64-diagnostic cap, source-span semantics,
  Session V1 schema, strict JSON grammar, depth/duplicate/numeric/Unicode handling, canonical bytes,
  parser public APIs or sole-format policy;
- changing the 1 MiB preparse refusal, the 17x parse projection, host caps, ABI, protocol, SDK,
  package, generated/browser/Wasm artifacts, realtime/render code, dependencies or lockfiles;
- rerunning issue #338's descriptive benchmark, manufacturing a replacement pre-migration timing,
  rerunning live browser qualification, or making a browser-performance claim; and
- broad optimization, speculative tuning, a new benchmark harness or unrelated cleanup.

## Objective gates

1. The constructed document is exactly `1_048_576` bytes, remains valid strict JSON at the typed
   schema layer, contains more than 10,000 automation segments, contains no empty sentinel segment,
   and every repeated segment has `start_sample == end_sample`.
2. The unchanged production `AudioWorkletEngineHost::boot` entry point returns
   `RESULT_REFUSED_DOCUMENT`. Its first diagnostic is exactly
   `automation.invalid_range\t$.automation[0].segments[0].end_sample`, the output contains exactly
   `MAXIMUM_PREPARE_DIAGNOSTIC_LINES` (64) newline-terminated diagnostics, and the final retained
   invalid-range path is segment index 63. The comment and mutation record describe this actual
   semantic-validation path.
3. Document construction remains outside the timer. The timer continues to cover the complete
   production boot/refusal call and its diagnostic materialization, and the assertion remains
   strictly `elapsed < Duration::from_secs(1)`.
4. After the implementation is otherwise complete, run exactly one local post-change
   pinned-CI-equivalent timing invocation:

   ```text
   cargo +1.97.1 test --locked --workspace --all-targets
   ```

   The named test must pass on that invocation. Do not run a focused timed copy before or after it,
   do not retry a failed invocation, and do not report a minimum/average. A miss returns this issue
   to HOLD and requires a fresh scope ruling; it is never retry-as-evidence. The required PR run is
   independent remote delivery confirmation, not authority to rerun or select local evidence.
5. Deterministic focused tests prove the fixture reaches semantic validation and that the
   diagnostic cap, order and paths remain exact without using elapsed time as their only oracle. If
   production code changes, a focused mutation or structural assertion makes bypass of the bounded
   behavior red.
6. The existing maximum-document preparse refusal, depth and diagnostic caps, strict duplicate,
   numeric and Unicode behavior, source-span behavior, 17x parse projection and sole JSON policy
   remain green. No accepted-session, canonicalization, resource, protocol, SDK, package, ABI,
   realtime or render behavior changes.
7. `cargo +1.97.1 clippy --locked --workspace --all-targets --all-features -- -D warnings`,
   `RUSTDOCFLAGS='-D warnings' cargo +1.97.1 doc --locked --workspace --no-deps`,
   `bash scripts/check-session-policy.sh`, `bash scripts/check-workspace-policy.sh` and
   `git diff --check` pass. The exact-path diff and worktree status contain only the approved
   tranche.
8. Issue #338's descriptive benchmark and live browser qualification are not rerun. Existing
   qualified Wasm/browser lineage remains unchanged, and no new timing claim is made beyond this
   named native CI wall.

## Review and delivery

This issue gets exactly one Sol-medium implementation attempt and one fresh Sol-high adversarial
review. HOLD rather than retrying, weakening the wall, changing the profile or expanding into a
general parser/performance effort if the one coherent attempt does not satisfy every gate. A
progress-only turn does not create another attempt, but it cannot be used to conceal a second
implementation pass.

Keep the work on `codex/batch-338-canonical-json` and deliver it in PR #339 as a distinct
`fix(#342)` checkpoint. A separate issue and commit make this a newly bounded gate-restoration
workflow rather than a disguised fourth #338 attempt; the already-red PR cannot deliver #338 while
the required engine check remains failed. In CI-conscious mode, commit the exact allowed-path
tranche locally, run the proportional non-timing gates, perform the single final local timing
invocation, then update PR #339 once. Do not force-push, manufacture a CI commit or request a rerun
to seek a favorable wall time.

Before implementation, create the matching GitHub issue with this exact title, verify it receives
number 342, synchronize its body with this file, and commit the brief checkpoint. After local
Sol-high PASS, push the coherent PR candidate once. Remote required CI must pass the unchanged test
on that candidate; if it misses the wall, HOLD without rerun. Synchronize and close #342 only after
the accepted evidence commit is upstream and remote CI is green. Issue #338 may cite #342 as the
resolution of its terminal timing blocker, but it cannot claim a fourth attempt or PASS until that
delivery evidence exists.

## Brief evidence and decision record

Sol-high briefing inspected corrective head `0256b854`, the #338 stateless contract, the full timed
region, the JSON fixture constructor, parser-to-validator control flow, semantic diagnostic cap,
mutation record and pinned CI command. It ruled that the failed wall cannot be retried away, that
the schema-invalid footer is a proven harness contradiction, and that elapsed phase attribution is
not possible from the failed run. Restoring the truthful fixture and, conditionally, the minimum
production optimization needed for the same unchanged wall is one independently closable outcome;
splitting the evidence correction from the performance qualification would leave each half unable
to prove the product claim.

## Implementation and Sol-high review evidence

Checkpoint `0a500d6a` changes only `hosts/host-web/src/tests.rs` and
`hosts/host-web/MUTATIONS.md`. The fixture now joins complete segments with commas, removes every
empty-object sentinel, and fills its exact 1,048,576-byte size only with legal trailing whitespace.
A separate untimed oracle proves the population remains above 10,000, every repeated segment has
equal sample bounds, typed parsing reaches semantic validation, and the result is exactly 64
ordered `AutomationInvalidRange` diagnostics at segment paths 0 through 63. This deterministic
phase evidence made a conditional production parser or validator change unnecessary.

Focused host-web tests excluding the timed case passed, as did the complete Session tests,
warning-denied workspace Clippy and rustdoc, formatting, session/workspace policies and diff
checks. The one authorized post-change timing invocation was
`cargo +1.97.1 test --locked --workspace --all-targets`; it passed on its first and only run. The
named production boot test passed its unchanged strict `<1s` assertion, and the host-web 59-test
binary completed in 0.17 seconds. No focused timed run or retry occurred, and no benchmark or live
browser qualification was run locally.

Fresh Sol-high adversarial review returned PASS for `0a500d6a`. It independently inspected the
exact path scope, strict JSON construction, exact size/population/equal-bound oracles, all 64
semantic paths, timer placement, unchanged wall and truthful mutation record, then audited the sole
timing transcript without rerunning it. Remote closure still requires the unchanged required CI to
pass on the pushed candidate; until then this is local accepted evidence only.
