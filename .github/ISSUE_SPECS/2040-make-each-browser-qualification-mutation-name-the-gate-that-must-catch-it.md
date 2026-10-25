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

## Decision record (attempt 1, 2026-10-25)
- `MUTATIONS` is a frozen `{ mutation, gate }` table (built with `caughtBy(gate, ...mutations)`);
  `mutationProofs` holds each thrown message to `` `${browserName}: ${gate}:` ``. Gates were read from
  one Chromium run on main `c42b37bf2` (81 mutations), then held green in Firefox and WebKit.
- Rule applied: a mutation is renamed when its name names a gate (exactly, or as the longest gate-name
  prefix) other than the one it reaches. Names that name no gate (`boot`, `stall-live-controls-load`,
  `structural-edit-skipped`, `structural-edit-adoption-shifted`, `sdk-track-response-*`) keep their name;
  the table gives their gate.
- Renamed to the gate reached: `sdk-spectrum-hop-wrong` -> `sdk-spectrum-continuous-hop-wrong`
  (`sdk-spectrum-continuous`; `sdk-spectrum-hop` keeps `sdk-spectrum-hop-missing`);
  `sdk-observation-window-reversed`/`-malformed` -> `sdk-observation-subscription-window-reversed`/`-malformed`
  (`sdk-observation-subscription`).
- Fixed instead of renamed: `sdk-protocol-refusal` reached `sdk-protocol-answers`, and the
  `sdk-protocol-refusal` gate had no other mutation. The mutation now sets `refusal.name = "Error"` on both
  engines, so the cross-engine answers gate holds and only the refusal gate's predicate fails. The refusal
  gate's own browser-versus-headless comparison is unreachable by any mutation (the answers gate compares
  the same object first); deleted under the root ruling below.
- Red check (PR evidence): `sdk-spectrum-continuous-hop-wrong` pointed at `sdk-spectrum-hop` fails Chromium with
  `chromium: sdk-spectrum-continuous-hop-wrong: red mutation escaped its gate sdk-spectrum-hop`.
- Root ruling (2026-10-25): the refusal gate's browser-versus-headless equality clause is deleted. The
  `sdk-protocol-answers` gate compares the same `answers` object (refusal included) first, so the clause
  held nothing the answers gate does not. The positive label `sdk-observation-window-width` is renamed
  `sdk-observation-subscription-window-width` to match the renamed rows.
