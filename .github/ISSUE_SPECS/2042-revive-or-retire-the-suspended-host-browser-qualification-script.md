# Revive or retire the suspended-host browser qualification script

Filed by root housekeeping (backlog sprint, 2026-10-25). Source: #1294b and the #1399 attempt-1 verdict
(`docs/handoffs/decision-15-2026-10-05/verdicts/stream-b3/1399-attempt1.md:69-74`).

## Problem
`hosts/host-web/qualification/suspended-host.mjs` qualifies a host on a suspended AudioContext, but it is
pinned to a historic build and no CI job or `npm run qualify` runs it. Its two exact status field lists
are older than the current status reply, so a revived run refuses every status: they lack
`blockingRebuilds`, `liveValuesSuperseded`, the watermark words (`appliedRevision`, `appliedSample`,
`appliedOutcome`), `singleModeControlAllocations` and `staleViewQuanta`. A list-only edit would disagree
with the script's own pin, so the fix is a decision, not a doc edit.

## Evidence (main `c1a218022`)
- `suspended-host.mjs:27-31`: `SOURCE_COMMIT = 35b0ff75...`, a wasm digest and a layout digest, checked against the matrix in `results.json` at
  `:136-157`.
- `suspended-host.mjs:37-40` (`STATUS_FIELDS`) and `:418-421` (`STATUS_FIELDS_IN_ORDER`), checked exactly at `:508`.
- `hosts/host-web/web/miso-engine-v1-audio-worklet-host.js:1068-1073`: the current status field order;
  `:1179-1197`: their validation (the new u64 fields are `bigint`; `plainStatus` at `suspended-host.mjs:502-506`
  converts only two).
- No reference to `suspended-host` in `.github/workflows/` or `scripts/`.

## Smallest slice
Decide one of two, and record why:
1. Retire: delete the script (git history keeps it), if another gate already
   covers the suspended-context behaviour (name it).
2. Revive: drop the historic pin (read the built artifacts like `run.mjs` does), take the status field list
   from the host (one source, not a copy), convert every `bigint` field in `plainStatus`, and wire it into
   `npm run qualify` or a CI job.

## Gates
- Retire: `lint-checks.sh --self-tests` and `npm run qualify` still pass; the named covering gate exists.
- Revive: `node suspended-host.mjs --artifacts <closure> --browser all` passes on the current build in three
  browsers, and a status with one field removed is refused.

## Paths
`hosts/host-web/qualification/suspended-host.mjs`,
`hosts/host-web/qualification/package.json` (revive only), this spec.

## Decision record
- 2026-10-25 (root, from the scope note): **retire**. Revive would copy the host's status field list
  (a second source) and add a fourth browser script for behaviour the main run already covers.
  `suspended-host.mjs` is deleted; git history keeps it. No package script, workflow or
  `scripts/` entry named it, so nothing else changed. The two handoff verdicts that mention it are
  history and stay.
- Covering gates (origin/main `4448e90b3`):
  1. `resume-clicks` (`run.mjs:441-451`, three mutations): every page boots the host on a context
     that is suspended and resumes it only through a trusted click.
  2. `observation-unsubscribe` (`run.mjs:433-437`): `runObservationUnsubscribeRun`
     (`qualification.js:1196-1317`) asserts the new context is suspended before boot
     (`:1199`, failure `context` at `:1405`), then reads `status("status (R1)")` (`:1314`) before the
     only `resumeThroughClick` (`:1317`). A status that is refused or times out while suspended
     throws `requestFailure`, and a missing `renderedQuanta` fails `render-between` (`:1412`).
     This is the deleted script's one distinct claim (a status answer before the first resume), so
     no new `run.mjs` assertion was needed.
  3. The status reply's field order and `bigint` validation live in the host
     (`miso-engine-v1-audio-worklet-host.js:1068-1073, :1179-1197`), checked by
     `scripts/test-web-audioworklet.mjs` on a suspended mock context (`:352, :1029, :1091`).
