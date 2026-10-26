# Default a strip with no pan to the identity matrix

Filed by root from the console sprint (2026-10-26). Source: the scope note `scratchpad/opt/scope/sdk-console-gaps.md`, items 7 and 8, and root ruling Q1 (2026-10-26).

## Defect
A track or submix built without `pan` takes the catalog default pan 0 on each lane (sdk/src/core/session.ts ~1374-1380), which collapses a stereo track to mono: L = R = 0.707(xL + xR). For a mono source the same default gives +3.01 dB per side (both lanes at pan 0 add two copies). The transparent submix already defaults to the identity matrix (session.ts ~1958).

## Smallest slice
- In `normalizeMatrixOrPan` (session.ts ~1353-1383), an absent `pan` becomes `matrix` with the catalog `matrix_*` defaults (ll = rr = 1, lr = rl = 0), for tracks and submixes alike.
- Behaviour change for documents built without `pan`. This is prelaunch and acceptable, and it is not an API break.
- A strip declared with `matrix` must still accept a live `pan(l, r)` edit. Check this.
- README: the mono model. A mono source reads channel 0 on both lanes and is processed as one plane up to the fader/matrix seam. With the identity matrix it plays at exactly its stereo dual-mono twin's level.
- Root ruling Q1: the console's placement law is a balance (0 dB at centre, the far side cos(p*pi/2)), not a -3 dB equal-power mono pan. A matrix coefficient is bounded by 1, so a constant-power law with 0 dB at centre is impossible. This keeps a correct mono export at its stereo twin's level.

## Gates
- One eval (sdk/test/builder-evals.mjs): a track and a submix built without `pan` serialize `matrix` {1, 0, 0, 1}. This catches the mono-collapsing default coming back.

## Paths
sdk/src/core/session.ts, sdk/README.md, sdk/test/builder-evals.mjs, this spec.

## Decision record (attempt 1, 2026-10-26)
- Root ruling Q1 (2026-10-26) is binding: a mono stem plays at exactly its stereo dual-mono twin's level; the placement law is a balance (0 dB at centre, far side cos(p*pi/2)). Recorded in sdk/README.md "Placement and mono sources".
- `normalizeMatrixOrPan`: an absent `pan` returns `matrix` from `builtinDefaultNumber("matrix_ll" | "matrix_lr" | "matrix_rl" | "matrix_rr")`, smoothing 0. No SDK-invented number. The spec-less transparent submix keeps its explicit identity.
- Rust side: no default to match. `crates/session/src/parse.rs` `parse_matrix_or_pan` refuses a strip with neither `pan` nor `matrix` (`MissingField`), and `SubmixStrip`'s transparent default (model.rs) is already the identity matrix. Not a class-B question; no rendered bit of any document that declares its pan moves.
- Live pan on a matrix strip: `pan(l, r)` alone is refused `control.inactive` (crates/controls `builtin_rows::write`). `matrixMode("pan")` and `pan(l, r)` in one transaction is accepted and commits pan {l, r}. The eval checks both. This is a behaviour change for an app that built a strip without `pan` and then sent `pan(...)` alone: it must now switch the mode in the same transaction (README says so). An SDK `pan()` that adds the switch itself would be a generated-builder change, outside this slice.
- Fixtures and digests: none moved. `fixtures/session/v1/builtins-automation.json` and the canonical-writer corpus declare their pan; sdk/test/fixtures are unchanged; the qualification live-bypass session (hosts/host-web/qualification/sdk-response-entry.ts) builds a track without pan, so its render changes, but its digests are compared live against authored, never pinned.
- Two existing assertions moved. builder-evals' "matrix-lane" row (matrix_ll on a pan strip) became "pan-lane" (pan on the now-matrix strip, still `control.inactive`). sdk/test/console-evals.mjs:1180 asserts the bus session's `verb` submix ends with `pan`; it now ends with `matrix`. The fix (give `verb` an explicit `pan: { left: 0, right: 0 }`, which keeps the pan arm in the round trip) is outside this spec's paths and waits for root.
