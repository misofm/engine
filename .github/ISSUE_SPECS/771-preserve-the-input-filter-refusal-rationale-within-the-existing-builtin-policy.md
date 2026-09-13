# Preserve the input-filter refusal rationale within the existing builtin policy

Source/delivery dependencies: #767 and #768; parent #763. This is a bounded CI correction for PR #769, not another DSP or artifact qualification attempt.

## Observed failure and approved correction

PR #769 run `34729987518`, job `103650939704`, exits 1 in unchanged `scripts/check-builtins-policy.sh`. The authentic log `/tmp/issue767-ci-policy-failure.log` identifies `crates/builtins/tests/filter_response.rs:404`: the comment contains `so no unsafe fixture is`. The checker performs a lexical token scan across the builtin sources/tests, with its existing single allocation-tracker allowance. The diagnostic is a comment match; no executable code defect or changed artifact has been demonstrated.

Astra xhigh approves one bounded implementation attempt. Luna max rephrases only the last two lines of that rationale to:

```rust
    // reaching its overflow arm would require an invalid synthetic slice. Constructing such a
    // slice would violate Rust validity requirements, so that fixture is inappropriate here.
```

Preserve the preceding explanation that valid `&[f32]` byte lengths are bounded by `isize::MAX`, making the two-section multiplication overflow unreachable through valid slices, while production retains checked arithmetic. Keep all test cases, assertions, function names and executable tokens unchanged. The replacement removes the incidental scanned word without changing the reason for omitting an invalid-slice fixture.

## Allowed paths and validation

Only this comment in `crates/builtins/tests/filter_response.rs`, the numbered successor spec, and concise #767/#768/#763 evidence records may change. No production source, test logic, policy/checker/allowlist/mutation fixture, build input, pin, browser results/matrix, dependency, workflow or expectation changes. Root numbers/synchronizes the issue before Luna edits and checkpoints the coherent correction and authentic command evidence before review.

Run exactly these proportional local gates, preserving actual command/exit evidence:

```sh
cargo test --locked -p builtins --test filter_response
cargo fmt --all -- --check
bash scripts/check-builtins-policy.sh
bash scripts/test-builtins-policy.sh
bash scripts/check-realtime-policy.sh
git diff --check
```

All must exit 0. The policy mutation suite must continue detecting the existing forbidden-token and error-path fixtures; do not bypass the production scan. A failure is preserved and returned for rebrief rather than widening this one-attempt correction.

## Review and delivery

Astra medium supplies one verdict on the exact corrected checkpoint, verifying the comment-only source diff, retained valid-slice rationale, unchanged executable test/production inputs, genuine gate exits and unchanged policy enforcement. #767's attempt-2 source PASS and #768's attempt-1 artifact PASS remain the accepted substantive evidence; this successor records the delivery correction separately.

No artifact rebuild, repin or browser qualification rerun is needed: test comments and issue records are not inputs to the shipped six-file build. Verify the allowed diff and retain #768's honest source/artifact/result lineage, without relabeling its completed browser run. Any other changed build input invalidates this shortcut and requires scope review.

Root integrates the correction into PR #769, requires successful qualification for its exact final head and main after merge, synchronizes the related GitHub issues/evidence, then closes this successor and the delivered children. Parent #763 remains open. #770 implementation resumes after this isolated correction is checkpointed and its shared source basis is synchronized.
