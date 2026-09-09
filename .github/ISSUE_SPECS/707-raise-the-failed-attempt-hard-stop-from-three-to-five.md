# Raise the failed-attempt hard stop from three to five

## Problem

The repository workflow currently stops after three failed implementation attempts. The active #705 qualification chain exhausted that cap on procedural preflight failures before its product probe ran, leaving two useful bounded recovery attempts unavailable.

## Objective

Raise the repository-wide hard stop to five total attempts while preserving every attempt's required adversarial review, evidence, checkpoint, and delivery accounting.

## Scope

- Amend the prospective workflow language in `AGENTS.md` from three total attempts to five.
- Keep completed and historical attempt records unchanged.
- After this policy is delivered, amend active coordination records so #705 retains Attempts 1–3 as failures and may use Attempts 4–5.
- Do not weaken objective gates, permit retries inside an attempt, or grant delivery credit to failed work.

## Acceptance gates

- The policy states a maximum of five total attempts and one adversarial verdict per attempt.
- Attempt five remains a mandatory hard stop followed by preservation and rescope/rebrief.
- Historical records remain intact.
- Local spec and GitHub issue body match exactly; proportional repository checks pass.
- The policy is merged before #705 is reopened or Attempt 4 begins.

## Status

Open for Astra XHIGH scope review.

## Astra XHIGH scope review — PASS

Scope review passes at clean pushed scope commit `67ff8d84ae0562f662af003d36d3b97a4816bf43`. Only these prospective paths change: `AGENTS.md`, `docs/IMPLEMENTATION_PLAN.md`, and `.github/ISSUE_SPECS/707-raise-the-failed-attempt-hard-stop-from-three-to-five.md`. History is preserved. The policy must merge before #705 is reopened.

## Luna HIGH attempt 1

- Updated prospective five-attempt workflow language and the hard-stop paragraph in `AGENTS.md`.
- Updated the general Review cadence sentence in `docs/IMPLEMENTATION_PLAN.md`, preserving issue-specific historical sentences.
- Appended this scope review and attempt record to the issue specification.

Review pending.

## Astra XHIGH attempt 1 review — PASS

Astra XHIGH returned **PASS** at exact clean pushed head `628f596477a262adaf39074bab3ef546d1f9f692`. Exactly the three approved paths changed. The policy now permits five total attempts with one adversarial verdict per attempt, preserves binding smaller issue budgets, stops and rescopes after a fifth failure, and forbids a disguised sixth retry. `docs/IMPLEMENTATION_PLAN.md` changes only the prospective general cadence sentence; its historical issue-specific budgets and outcomes remain unchanged. `git diff --check` and `bash scripts/check-workspace-policy.sh` passed. GitHub #707 had exact title/body parity at review time. Required qualification and merge remain pending; #705 stays closed until this policy is delivered.
