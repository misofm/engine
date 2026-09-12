# Proposed successor to #758: Satisfy Clippy for vectorization body accumulation

## Scope ruling and observed failure

APPROVED for root to file as a separate numbered CI-lint successor, in the existing PR #761 branch. This is not authorization for #758 attempt 3. #758 exhausted its two semantic implementation attempts and retains its final Astra medium semantic PASS at `bf0bb679d325024486bf3c7657896eebbab39043`. Its delivery is now explicitly CI-blocked.

Read-only review on 2026-09-12 confirmed clean worktree `/home/bl/misofm/engine-probe-body-attribution` at evidence head `37ae65d798dfb92350cc2dd5b83cb16c755dd0ac`, containing main `8bfbb5b62a08a9c1c3c46abbbbd5337b393d77c7`. The local captured log `/tmp/issue758-ci-clippy-failure.log` records PR #761 qualification run `34701531963`, merge ref `d09c42c84344c3180b6fc8215480cc37003af883`, failing with exit 101 because `tools/audit/src/vectorization.rs:233` nests two collapsible `if let` blocks. The diagnostic supplies the let-chain correction. `.github/workflows/qualification.yml:289` confirms the actual gate command below. No tests, builds, remote actions, or source edits were performed during this scope review.

## Smallest closable correction

In `symbol_bodies`, replace only:

```rust
if let Some((symbol, body_index)) = &active {
    if let Some(symbol_bodies) = bodies.get_mut(symbol) {
        // existing body
    }
}
```

with the equivalent short-circuit let chain:

```rust
if let Some((symbol, body_index)) = &active
    && let Some(symbol_bodies) = bodies.get_mut(symbol)
{
    // identical existing body
}
```

Keep the inner statements, pattern bindings, expression evaluation order, and all symbol identification/certification behavior unchanged. The second lookup still occurs only when `active` is `Some`; both expressions borrow existing values, and neither introduces an owned temporary with a destructor whose lifetime changes. No new tests are needed to mirror this syntax change: the existing 14 focused tests provide the relevant regression coverage.

Implementation edits are limited to that block in `tools/audit/src/vectorization.rs`, with formatting of the same block. Root owns the new matching `.github/ISSUE_SPECS/<number>-<slug>.md`, any required issue index entry, and a concise appended CI-block/successor decision record in `.github/ISSUE_SPECS/758-reject-ambiguous-or-misidentified-vectorization-probe-bodies.md`. These evidence edits must preserve both prior attempts and the distinction between semantic PASS and delivery CI status.

No lint suppression, changed lint configuration, weakened gates, workflow changes, new dependencies, manifest/lock changes, target changes, schema/hash/CLI changes, production kernel edits, parser redesign, new fixture framework, or unrelated cleanup. If another problem appears, record it and stop; this scope does not authorize repairing additional findings.

## Gates and evidence attribution

Run from the corrected worktree using the repository's pinned Rust 1.97.1 toolchain:

1. `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` — the actual CI command, not a narrowed substitute.
2. `cargo test --locked -p audit vectorization` — all 14 existing focused tests pass.
3. `cargo fmt --all -- --check` and `git diff --check` pass; inspect the exact diff for unchanged body statements and scope containment.
4. Astra medium reviews the single correction and command evidence once. Root records command outcomes and the exact tested implementation head, checkpoints before further work, and obtains green required CI for PR #761's updated head before merging/closing.

**No local release rebuild or repeated disassembler mutation run is required for this syntax-only successor.** The prior release evidence remains explicitly attached to the attempt-2 artifact SHA-256 `6c5632216ec11817d4a26da4c9a2417778f704293689bb3e652ea8944ad87492`; do not relabel that artifact or those runs as evidence executed at the new head. The preserved inner statements, evaluation-order review, 14 focused tests, full actual Clippy command, and required CI on the updated PR head provide proportional new-head validation. CI coverage must be described as the checks actually run, not as a new release-disassembly certification. Any semantic change would invalidate this no-rebuild rationale and requires stopping/rebriefing rather than growing the successor.

## Attempt and delivery bound

User-requested sequence: **Astra medium scope -> Luna max implementation -> Astra medium adversarial review**. Authorize exactly **one coherent implementation attempt and one Astra verdict** for this successor. On FAIL, preserve evidence and stop; no local retry or disguised continuation of #758's exhausted attempt budget is authorized.

Root must create/synchronize the matching numbered issue and stateless local spec in one checkpoint before implementation, confirm number/title, and openly record #758's CI block and successor relationship. Keep the existing PR #761 branch and history; no reset or force-push. After local gates and review PASS, root pushes the coherent checkpoint under the current delivery mode, observes required CI at the new PR head, and completes ordinary merge/delivery synchronization. Do not close #758 or the successor while required CI is failing or pending. The broader #377 remains open. This scope grants no additional implementation tranche beyond the one syntax correction.

## Sole implementation attempt evidence

Luna max changed only the specified nested if-let block into an equivalent short-circuit let chain, preserving the body statements. The actual pinned full-workspace Clippy command passes, all 14 existing focused vectorization tests pass, workspace formatting and diff checks pass. Logs are `/tmp/issue762-attempt1-*.log`. No local release rebuild or disassembly rerun was performed; prior semantic evidence keeps its original artifact attribution. Astra medium sole verdict is pending.
