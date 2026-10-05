# Qualify a structural browser edit in real browsers

Stream H(c) of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-10, D15-11, D15-17).
Formerly slice B6 of *Swap a rebuilt plan without an audio gap* (#1269). Qualification only.
Code anchors verified on `main` at `6fb211594`.

## Product outcome

Evidence, in the three real browsers the qualification matrix runs, that a structural edit applied
during playback (a transaction committed by the control plane, adopted by the render worklet)
renders exactly what the same module renders for the same edit at the same adoption sample, with
no worklet error, no source underrun and no render-path allocation. It is shown for a
cross-origin-isolated page (the Worker control plane) and for a non-isolated page (the single
instance, whose structural edit is the blocking rebuild, reported and counted, D15-10).

## Context

- The browser qualification harness (`hosts/host-web/qualification`, run by
  `npm run qualify -- --artifacts <dir> --sdk-root <sdk> --browser <name> --check-matrix
  --self-test-mutations`, `.github/workflows/qualification.yml:430`) runs a real AudioContext.
  Today it compares worklet output with committed digests read from `expected.directOracle`
  (`hosts/host-web/qualification/qualification.js:702-703`), produced by
  `hosts/host-web/tests/browser-v1/direct-oracle.mjs` (its `boot` helper at `:90`) and stored in
  `hosts/host-web/tests/browser-v1/expected.json`. Its mutation self-test list is `MUTATIONS`
  (`run.mjs:23`), applied by `mutate` (`qualification.js:616`). The recorded matrix is
  `hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md`; adding a case re-records it for every browser.
- The qualification server (`server.mjs`) sends no COOP/COEP headers today. *Run the browser
  control plane in a Worker and keep the AudioWorklet render-only* (#1332) adds an isolated leg and
  a non-isolated leg to `npm run qualify` and reports `instanceMode` (`worker` or `single`) on each
  (its deliverable 4 and gate 5). The host status's saturating `blockingRebuilds` counter (#1332 D1; incremented by *Replace the
  running browser session in the Rust host*, #1290, D6) counts each `single`-mode structural apply.
- `miso.replace.v1` reaches the `replace` export of *Diff a replacement document against the
  committed model and export replace from the browser engine module* (#1386).
- The edit messages, the watermark message and the meter switch at adoption come from *Send a session transaction
  to the browser control plane* (#1294). The render-path allocation counter comes from
  *Gate AudioWorklet render against allocation statically and at runtime* (#1333).
- Native equality of a swap with a fresh boot of the new session is gated by *Replace the running
  browser session in the Rust host* (#1290). This slice proves the browser engines execute the same
  module path bit for bit.

## Decisions frozen for this slice

- **D1. Case.** One matrix case, `structural-edit`: boot A (the browser fixture's session), play,
  then send `miso.replace.v1` with document B = A plus a muted track on an existing source with an
  EQ insert (no latency change), a fixture beside `session.json`. `replace` diffs B into one
  transaction and applies it, so this case runs the same apply path as `miso.apply.v1` and needs no
  transaction encoder. Capture the output until the watermark covers the transaction's
  revision R plus 32 quanta. Record R's path, the watermark's `(R, S, outcome)` and the capture.
- **D2. Reference at run time, not a committed digest.** `run.mjs` runs the shipped module under
  Node through the same exports, single instance: boot A, render the blocks before S with the same
  feed, call `replace` with B, render from S. The browser capture must equal this render
  bit for bit. No new digest is committed to `expected.json`: the reference is recomputed on every
  run.
- **D3. Determinism.** The case requires path `rebuild`, outcome `exact`, and zero source
  underruns over the capture. A run that reports another outcome fails with that outcome named.
- **D4. Both legs.** The case runs on #1332's isolated leg (`instanceMode === "worker"`,
  `blockingRebuilds` 0) and on its non-isolated leg (`instanceMode === "single"`,
  `blockingRebuilds` exactly 1).
- **D5. Allocation.** After the case, #1333's render-locked allocation counter reads exactly 0 on
  both pages.
- **D6. Matrix.** Re-record the matrix for every browser in one run and state it in the PR.
- **Superseded.** The old D1 (`miso.replace.v1` handled inside the worklet, compared with a
  committed native digest) is superseded by decision 15 D15-10 and D15-11.

## Deliverables

1. D1-D5 in `hosts/host-web/qualification/` (case, reference run, mutations).
2. The re-recorded `hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md` and `results.json`.

## Authorized paths

- `hosts/host-web/qualification/`
- `hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md`
- `hosts/host-web/tests/browser-v1/` (the document B fixture beside `session.json` only)

## Non-goals

- No product change. No timing gate: the commit-to-adoption delay is recorded, not gated.
- No committed PCM digest for this case.

## Objective gates

1. The case passes in Chromium, Firefox and WebKit on both pages: capture equals the run-time
   reference, path `rebuild`, outcome `exact`, zero underruns, allocation counter 0.
2. Three new mutations in `MUTATIONS` turn the run red: `structural-edit-skipped` (the replacement
   is never sent), `structural-edit-adoption-shifted` (the reference applies one block late), and
   `structural-edit-blocking-uncounted` (the `single` leg's `blockingRebuilds` reads 0).
3. Commands, per browser, from `hosts/host-web/qualification` after `npm ci`, with the artifact
   built as in `qualification.yml`'s `artifact` job:
   - `npm run qualify -- --artifacts <dir> --sdk-root ../../../sdk --browser chromium --check-matrix --self-test-mutations`
     and the same for `firefox` and `webkit`
   - `bash scripts/check-web-audioworklet.sh <dir> <named-twin>` and
     `python3 -B scripts/check-browser-expected-resources.py --artifacts <dir>` from the repo root

## Test value

- Gate 1: a browser path that adopts at a different block than the watermark reports, re-cuts a
  view too late, or misroutes a source after the swap diverges from the reference; it turns red.
  No hermetic test sees real rendered bits, and no native test runs a browser engine's Wasm.
- Gate 1 (non-isolated): a single-instance page whose structural edit silently fails or is not
  counted turns it red.
- Gate 1 (allocation): a swap path that allocates in the worklet after boot turns the counter
  non-zero.
- Gate 2: each mutation proves the comparison can fail for the defect it names.

## Dependencies

- *Send a session transaction to the browser control plane* (#1294).
- *Run the browser control plane in a Worker and keep the AudioWorklet render-only* (#1332).
- *Gate AudioWorklet render against allocation statically and at runtime* (#1333).
- *Replace the running browser session in the Rust host* (#1290).
- *Diff a replacement document against the committed model and export replace from the browser
  engine module* (#1386).
