# Make each browser qualification mutation name the gate that must catch it

Filed by root housekeeping (backlog sprint, 2026-10-25). Source: #1522 (closed).

## Problem
`mutationProofs` in `hosts/host-web/qualification/run.mjs` proves that each red mutation makes `validate`
throw, but for most mutations it checks only the `"<browser>:"` prefix of the error message. It does not
prove which gate caught the mutation. Example: `sdk-spectrum-hop-wrong` sets
`spectrum.continuous.first.hopFrames = 2048`, and the `sdk-spectrum-continuous` gate catches it, not the
`sdk-spectrum-hop` gate. A gate that stops firing stays green while another gate still throws.

## Evidence (main `c1a218022`)
- `hosts/host-web/qualification/run.mjs:1358-1374`: the prefix chain; only `resume-clicks-*`,
  `sdk-observation-wait-*`, `sdk-spectrum-hop-wait-*` and `page-errors*` name a gate, every other mutation
  falls through to `` `${browserName}:` ``.
- `run.mjs:29-87`: the `MUTATIONS` list; `run.mjs:1121`: `mutate`; `run.mjs:1176-1178`: `sdk-spectrum-hop-wrong`.
- `run.mjs:124-126`: `gate()` throws `` `${browserName}: ${name}: ${detail}` ``, so the gate name is in every message.
- `run.mjs:959-962` (`sdk-spectrum-continuous`) and `run.mjs:1093` (`sdk-spectrum-hop`).

## Smallest slice
Turn `MUTATIONS` into a table of `{ mutation, gate }` and assert that the error message starts with
`` `${browserName}: ${gate}:` ``. Fill each expected gate from a run of the current proofs (record the
gate each mutation reaches today). Where a mutation reaches a gate other than the one its name targets
(for example `sdk-spectrum-hop-wrong`), either fix the mutation so it reaches its own gate or rename it;
record each choice. No gate's own logic changes.

## Gates
- `npm run qualify -- --artifacts <closure> --browser chromium` (and firefox, webkit) prints
  `all qualification gates passed` with the mutation proofs on.
- A red check, PR evidence: for one mutation, change its expected gate to another gate name; the proof fails
  with the `red mutation escaped its gate` message.
- `lint-checks.sh`.

## Paths
`hosts/host-web/qualification/run.mjs`, this spec.
