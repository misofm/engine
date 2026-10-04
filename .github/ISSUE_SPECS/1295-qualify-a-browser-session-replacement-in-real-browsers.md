# Qualify a browser session replacement in real browsers

Slice B6 of *Swap a rebuilt plan without an audio gap* (#1269). Code anchors verified on
`54b0a1bf8` (unchanged at `24029badb`). Qualification only.

## Product outcome

Evidence, in the three real browsers the qualification matrix runs, that a session replacement
during playback renders the same PCM as the native engine, with no worklet error and no stall
beyond the one the replacement itself costs.

## Context

- The browser qualification harness (`hosts/host-web/qualification`, run by
  `npm run qualify -- --artifacts <dir> --sdk-root <sdk> --browser <name> --check-matrix
  --self-test-mutations`, `.github/workflows/qualification.yml`) runs a real AudioContext and compares
  worklet output with a native digest of the same feed. Its matrix is recorded; adding a case
  re-records it for every browser.
- The native digest is read from `expected.directOracle` (`hosts/host-web/qualification/qualification.js:702-703`),
  produced by `hosts/host-web/tests/browser-v1/direct-oracle.mjs` and stored in
  `hosts/host-web/tests/browser-v1/expected.json`.
- The replacement message comes from *Send a replacement session to the AudioWorklet* (#1294).

## Decisions frozen for this slice

- **D1. Case.** One matrix case: boot A, render, post `miso.replace.v1` with B (A plus a muted track),
  render on; compare the captured output, after the replacement and before it, with a native digest
  of the same swap. `direct-oracle.mjs` produces that digest by running the shipped module under Node
  through the same exports (boot, render, replace, render) with the same feed, and records it in
  `expected.json` beside the existing one.
- **D2. Stall.** Record the handler duration and any deadline misses around the replacement in the
  case's evidence; they are descriptive, not a gate.
- **D3. Matrix.** Re-record the matrix for every browser in one run and state it in the PR.

## Deliverables

1. D1-D3 in `hosts/host-web/qualification/`.
2. The re-recorded matrix.

## Authorized paths

- `hosts/host-web/qualification/`
- `hosts/host-web/tests/browser-v1/direct-oracle.mjs`, `hosts/host-web/tests/browser-v1/expected.json`,
  and a replacement document beside `session.json`

## Non-goals

- No product change. No timing gate.

## Objective gates

1. The new case passes in Chromium, Firefox and WebKit: output digests equal the native swap's.
2. The harness's mutation self-test turns the case red when the replacement is skipped.
3. Commands: the browser qualification run for each browser, as in `qualification.yml`, and the
   umbrella's inherited gates.

## Test value

- Gate 1: a browser path that refreshes a view too late (detached after growth) or misroutes a
  source after the replacement diverges from the native digest; it turns red. No hermetic test sees
  real rendered bits.

## Dependencies

- *Send a replacement session to the AudioWorklet* (#1294).
