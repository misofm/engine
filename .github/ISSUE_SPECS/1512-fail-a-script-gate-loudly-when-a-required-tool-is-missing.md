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
