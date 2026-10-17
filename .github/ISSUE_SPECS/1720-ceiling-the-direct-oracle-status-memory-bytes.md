# Ceiling the direct oracle's status memory bytes

Proposed by the controls coordinator from batch 20 (#1669 review, NIT/placeholder; 2026-10-17).
Filed by root. Not phase-2 automation work: test hygiene for the browser identity
gate. Code anchors at `589403170`.

## Product outcome

The direct oracle's printed `beforeDisposeStatus.memoryBytes` (and any other status field that
is a resource size, not a contract fact) is a ceiling, not an exact pin. A layout change that grows
the Wasm linear memory by one page inside its budget does not turn the gate red; growth past the
budget does.

## Context

- `scripts/check-browser-expected-resources.py` splits resource rows into `EXACT_ROWS` and
  `CEILING_ROWS` (#1060, owner decision 4: budgets, not exact counts). Everything else in the
  printed document must equal `expected.json` exactly (the docstring, `:33-36`).
- The status snapshots carry `memoryBytes`, the Wasm memory size. It falls in "everything else",
  so it is pinned to the byte: 2,883,584 in the identity, SET_CONTROL and observation legs
  (`hosts/host-web/tests/browser-v1/expected.json:39`, `:49`, `:117`) and 2,949,120 in the batch-20
  automation leg (`:146`).
- AGENTS.md (test value): an exact resource byte count is used only for a wire, ABI or on-disk
  format. `memoryBytes` is none of these. The batch-20 reviewer flagged it; the older legs had the
  same pin before batch 20.

## Decisions to make at filing

- D1. Which status fields are resource sizes (at least `memoryBytes`) and which are contract facts.
- D2. The ceiling rule: reuse the `CEILING_ROWS` rule (value plus 10 %, rounded up to a 64 KiB page
  for memory), or a separate status-ceiling table.

## Deliverables

- The script classifies the status resource fields as ceilings, with a `--self-test` mutation for
  one page over the ceiling.
- `expected.json` carries their ceilings; the oracle prints the measured values.

## Authorized paths

- `scripts/check-browser-expected-resources.py`, `hosts/host-web/tests/browser-v1/expected.json`,
  and, if the oracle compares the whole document, `hosts/host-web/tests/browser-v1/direct-oracle.mjs`.

## Objective gates

1. `python3 -B scripts/check-browser-expected-resources.py --self-test` catches one page over a
   status ceiling.
2. `python3 -B scripts/check-browser-expected-resources.py --artifacts target/ci/qualification-artifacts`
   passes on the shipped artifact.

## Test value

The new self-test mutation turns red if a status resource field stops being ceilinged or grows past
its budget, which the exact pin catches today only as noise on every layout change.

## Dependencies

None. #1669 (batch 20) adds the fourth leg it covers.
