# Detect a browser host that stops sharing its status read

Filed by root housekeeping (backlog sprint, 2026-10-25). Source: #1605 slice A (PR #1995), its
"M1 re-expressed" finding.

## Problem
The browser host shares one status read: concurrent `status()` calls join the in-flight read or one
follow-up read, so the host never posts more than one status request at a time. No qualification gate
checks this. #1605 slice A applied M1', which removes the sharing (`status()` always calls
`#sendStatus()`), and the browser qualification stayed green in Chromium, Firefox and WebKit. A host
that floods the port with status requests therefore passes.

## Evidence
- `hosts/host-web/web/miso-engine-v1-audio-worklet-host.js:1185-1199`: `status()` returns the shared
  `#statusRead` or the `#statusFollow` promise; `:1207-1215` `#sendStatus()` records the in-flight read.
- `.github/ISSUE_SPECS/1605-post-sprint-qualification-of-decision-15-heavy-gates.md:95,99-105`:
  M1' green in all three browsers.

## Smallest slice
One assertion in an existing browser qualification or host node test: N concurrent `status()` calls
with no request between them post exactly one `miso.status.v1` request, and a call made after another
request joins exactly one follow-up. Add the M1' mutant to the qualification's mutation list with the
gate that must catch it.

## Gates
- The assertion is red with M1' applied and green on the shipped host (both recorded in the PR).
- `npm run qualify -- --browser <b> --sdk-root sdk --check-matrix --self-test-mutations` passes in
  Chromium, Firefox and WebKit.

## Paths
The one host-web qualification or node test file that holds the assertion, its mutation list; this spec.
