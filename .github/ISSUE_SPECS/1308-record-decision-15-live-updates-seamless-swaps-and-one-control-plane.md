# Record decision 15: live updates, seamless swaps and one control plane

Stream S0 of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`,
D15-0 to D15-17). Read against `main` at `6fb211594`.

## Product outcome

The repository records decision 15 and the rules it changes, so that every coordinator and
implementer works from one written source: the ruling, the amended `AGENTS.md`, the rewritten and
new issue specs, and a stream index.

## Context

- On 2026-10-05 the owner set a no-shortcuts principle and delegated the decisions on the open items
  (#1053 Q1-Q5, #1269 Q1-Q6, decision 14's findings F1-F9) to root and a fresh adversarial agent.
  The ruling quotes the owner verbatim and records D15-0 to D15-17 as root decisions made under that
  delegation, not as the owner's words.
- The evidence is the root draft, two adversary rounds and the agreed plan, copied into
  `docs/handoffs/decision-15-2026-10-05/`.
- `AGENTS.md` limits active implementation WIP to one launch-critical feature issue ("Recovery and
  throughput"); D15-0 replaces that rule.

## Decisions frozen for this slice

- **D1.** The ruling uses decision 13's authority kinds; D15-n are "owner-delegated decisions (root
  and adversary)".
- **D2.** `AGENTS.md` changes: the no-shortcuts principle (product principles); decision 15's
  additions to decision 14's rules and the non-blocking submit (D15-17); the browser Worker control
  plane, render-only worklet and nightly browser toolchain (D15-10); the supersession sentence after
  "never silently lost" (D15-2); the parallel-coordinator rule replacing the one-WIP rule (D15-0).
- **D3.** Decision 14's ruling gains one pointer paragraph to decision 15; its 2026-10-04 text is
  kept.
- **D4.** #1291 and #1292 close as not planned; #1020 closes as answered. Each gets a comment that
  points at decision 15 and the replacing issue, and its spec leaves `.github/ISSUE_SPECS/`.

## Deliverables

1. `docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`.
2. The `AGENTS.md` amendment (D2) and the decision-14 pointer (D3).
3. Every spec rewrite and new issue of the plan's section 3, created or edited on GitHub with a body
   equal to its local spec.
4. `docs/handoffs/decision-15-2026-10-05/STREAMS.md` and the copied plan files.

## Authorized paths

- `docs/rulings/`, `AGENTS.md`, `.github/ISSUE_SPECS/`, `docs/handoffs/decision-15-2026-10-05/`

## Non-goals

- No code, script, workflow or test change.
- No push, pull request or issue closure for implementation work; root merges.

## Objective gates

1. `bash scripts/check-workspace-policy.sh` and `bash scripts/test-workspace-policy.sh` pass.
2. `bash scripts/check-dsp-research.sh` and `bash scripts/check-builtins-listening.sh` (the
   `docs-gates` job of `.github/workflows/qualification.yml`) pass.
3. The set of numbers in `gh issue list --state open --limit 500` equals the set of numeric prefixes
   in `.github/ISSUE_SPECS/*.md`, and every new or rewritten issue's GitHub body equals its local
   file byte for byte (`gh issue view <n> --json body`).
4. A fresh adversarial verifier checks the authority statement, every decision's presence and
   consistency, each spec's anchors and gates, acyclic dependencies and GitHub equality; every
   BLOCKER and MAJOR is folded in.

## Test value

No test is added; the gates are policy scripts and a set comparison.

## Dependencies

- none
