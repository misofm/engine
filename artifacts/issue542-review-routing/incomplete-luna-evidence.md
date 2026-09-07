# Issue #542 misrouted read-only review evidence

This is preserved evidence from an interrupted **Luna XHIGH** process. It is not a completed review and does not satisfy the required Sol XHIGH verification.

## Provenance

- Implementation candidate: `33eed6752f3a4324f37146e65bb9feff26244481`
- Immutable comparison base: `9cce4350b18fc8b924fb68111fd7a888da780a3c`
- Launcher: `/tmp/issue539/luna-fresh.py`
- Recorded argv: `/tmp/issue542/sol-xhigh-review-run.command.json`
- Actual model argument: `-m gpt-5.6-luna`
- Reasoning argument: `-c model_reasoning_effort="xhigh"`
- Prompt: `/tmp/issue542/sol-xhigh-review.prompt`
- Captured stderr: `/tmp/issue542/sol-xhigh-review-run.stderr`
- No launcher status or final output exists because root requested interruption after the routing error was discovered.
- Source remained untouched; the feature worktree was clean when the process was stopped.

## Preserved gate matrix

Every listed log records `head: 33eed6752...` and `status: 0`, except the deliberately failing bypass described below.

| Gate | Log | Result |
|---|---|---|
| Focused frozen corpus | `sol-review/focused-corpus.log` | PASS: 3/3 |
| Focused exact 39-edit fixture | `sol-review/focused-39-edit.log` | PASS |
| Million mutations debug | `sol-review/mutation-debug.log` | PASS: 1/1 |
| Million mutations release | `sol-review/mutation-release.log` | PASS: 1/1 |
| Protocol default features | `sol-review/protocol-default.log` | PASS |
| Protocol test-support | `sol-review/protocol-test-support.log` | PASS |
| Conformance debug | `sol-review/conformance-debug.log` | PASS |
| Conformance release | `sol-review/conformance-release.log` | PASS |
| Conformance boundary ordinary run | `sol-review/boundary.log` | PASS |
| Protocol control policy | `sol-review/control-policy.log` | PASS |
| Workspace policy | `sol-review/workspace-policy.log` | PASS |
| Normal protocol dependency check | `sol-review/normal-protocol-check.log` | PASS |
| Normal dependency tree assertion | `sol-review/normal-tree-assertion.log` | PASS: no normal conformance node |
| Cargo normal tree | `sol-review/cargo-tree-normal.log` | PASS |
| Cargo metadata | `sol-review/cargo-metadata.log` | PASS |
| Bench consumer compile | `sol-review/bench-consumer.log` | PASS |
| Strict Clippy | `sol-review/strict-clippy.log` | PASS |
| Strict rustdoc | `sol-review/strict-rustdoc.log` | PASS |
| rustfmt | `sol-review/fmt-check.log` | PASS |
| Diff check | `sol-review/diff-check.log` | PASS |
| Shell syntax | `sol-review/shell-syntax.log` | PASS |
| Scalar/SIMD Wasm parity self-test | `sol-review/parity-self-test.log` | PASS |
| Production export counterexample and byte-exact restore | `sol-review/boundary-counterexample-independent.log` | PASS: mutation rejected with checker 1; restored checker 0 |
| Source/ownership audit | `sol-review/source-audit.log` | PASS |
| Packed/raw evidence and checkpoint identities | `sol-review/evidence-integrity.log` | PASS: 28 entries, 24 identities, 0 errors |

## Boundary-gate bypass for Sol assessment

The candidate script globally removes every scan hit whose path ends in `tests.rs`:

```sh
filtered_uses="$(gate_filter_exclude "${production} test-only harness filter" '/tests\.rs:' "$filtered_uses")"
```

That pathname rule does not prove the file is compiled only under `#[cfg(test)]`.

The Luna reviewer made a scratch copy of candidate `33eed675`, removed the `#[cfg(test)]` line immediately before `mod tests;` in `crates/protocol/src/controller.rs`, and appended `use conformance::ConformanceDecoder;` to `crates/protocol/src/controller/tests.rs`. It then ran:

```text
CARGO_TARGET_DIR=/tmp/issue542-review-target/filter-bypass bash scripts/check-conformance-boundaries.sh .
```

The checker printed `conformance boundaries: ok` and exited 0. The evidence log deliberately records overall `status: 1 (checker accepted an unguarded tests.rs harness use)` at `/tmp/issue542/sol-review/boundary-test-only-filter-bypass.log`.

Smallest correction for actual Sol review to assess: replace the blanket `/tests\.rs:` exemption with a gate that ties each allowed test-child path to a literal test-only module declaration, or otherwise proves that every exempt file is unreachable from a normal build. Preserve the existing ordinary boundary and production-export counterexample behavior.
