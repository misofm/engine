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
