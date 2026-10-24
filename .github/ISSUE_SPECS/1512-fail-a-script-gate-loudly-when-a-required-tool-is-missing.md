# Fail a script gate loudly when a required tool is missing

## Product outcome

A CI or local gate whose command needs a tool that is absent (for example `rg`) fails with a clear
"requires <tool>" error and a nonzero exit, never with a misleading audit error and never by passing.

## Context

- 35 scripts call `rg`; 11 CI jobs install it inline (`apt-get install ripgrep`; there is no shared setup
  action). The nightly vectorization harness ran about 40 nights without `rg` while `continue-on-error`
  hid it, and the missing tool showed up as "green control produced no pass report", which reads like an
  audit regression.
- 11 call sites use the negated form `! rg …` (for example `scripts/check-parametric-eq-render-contract.sh:35-47`,
  `scripts/check-builtins-listening.sh:15`). If `rg` is missing, exit 127 is inverted and the
  forbidden-pattern gate passes silently. Every job that runs them installs `rg` today, so there is no
  live hole, but nothing enforces it.

## Decisions

- D1. One shared preflight helper (for example `scripts/lib/require-tools.sh`) that each script sources
  with the tools it needs; it exits 2 with "<script> requires <tool>" when one is missing.
- D2. Every script that calls `rg` (and any other non-coreutils tool) uses the helper before its first
  call; the negated `! rg` sites first.

## Gates

- G1. With a PATH shim directory without `rg`, each converted script exits 2 with the "requires rg"
  message before its first check (test value: a negated `! rg` gate that passes silently when `rg` is
  missing, which no current check catches).
- G2. With `rg` present, each converted script's existing result is unchanged.

## Authorized paths

`scripts/` (the helper and the converted scripts), this spec.

## Evidence and decision record

- Counts on `main` 82f987296 (2026-10-24): 33 shell files call `rg` (32 scripts plus `scripts/lib/gate.sh`);
  5 negated `! rg` sites survive (`check-parametric-eq-render-contract.sh:35,40,46,47`,
  `check-builtins-listening.sh:15`). Every CI job that runs one of these scripts installs ripgrep, so
  there is no live hole; the preflight is what makes a dropped install loud.
- D1 as built (root, 2026-10-24, option B): `require_tools` lives in `scripts/lib/gate.sh`, which calls
  `require_tools rg` at load. There is no separate `require-tools.sh`: several tests copy or link
  `gate.sh` alone into a scratch tree, so a sibling file would break them. `gate.sh` defines only
  functions plus that call (no `set`, trap or global).
- D2 scope (root): `rg` only. Other tools (jq, python3, cargo, node, wabt, llvm) are called positively,
  so a missing one already fails nonzero; listing them per script is a follow-up.
- Converted: the 20 scripts that called `rg` without `gate.sh`, plus `test-session-policy.sh`,
  `test-dsp-research.sh` and `test-lane-policy.sh`, which called `rg` (or a child gate) before sourcing
  `gate.sh`. Each has `source "$(dirname "${BASH_SOURCE[0]}")/lib/gate.sh"` right after `set -euo pipefail`.
  `check-unfused-seal.sh` links `gate.sh` beside its self-test mutant copy.
- G1: `scripts/test-gate-lib.sh` finds every `scripts/*.sh` with an `rg` call, runs it with no
  arguments under a PATH without `rg`, and requires exit 2 and exactly "<script> requires rg" (27
  scripts). Exclusions: itself (recursion) and `test-workspace-policy.sh`, which does not source `gate.sh` (it only copies it) and calls `rg`
  directly; converting it belongs to #1527, which owns that file this round (root note on #1527).
- Attempt 1 (2026-10-24): G1 and G2 pass locally (see the attempt report).
