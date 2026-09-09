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
