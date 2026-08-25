# Bulk source submit (B4, issue #148): null — closed without building

**Candidate.** An additive bulk-submit export — one call, N sources' staged
quanta — replacing N per-source `submitSource` calls on the render thread at
64–128 sources. Projected in the boundary-convergence plan at ~2–4 µs per call
(130–500 µs/block at 64–128 sources). Decision input: the misofm/app#26
shared-memory feed's measurement (landed as misofm/app#45).

**Measurement.** Three-arm isolation at 64 sources, 1500 blocks, real engine
wasm through the app's realm harness (single-threaded, so the arms bound the
render-side share from above):

| arm | µs/block |
|---|---|
| full shared feed (pump + drain + 64 submits) | 2 600 |
| rings attached, pump halted (empty-ring scan) | 2 178 |
| no rings at all | 2 191 |

* The empty-ring scan at 64 sources costs **nothing measurable** (−13 µs,
  inside noise).
* The full-feed delta (423 µs/block) is dominated by costs a bulk export
  cannot remove: the pump's plane slicing and window handling (main-thread in
  the field), and the two copies (ring → staging → wasm) whose bytes must move
  under any call shape. Per the frozen contract, bulk submit validates **per
  entry**, so validation cost is call-shape-invariant too.
* The bulk-addressable share is exactly the JS→wasm boundary crossings:
  64/block at 64 sources. Modern engines cross at ~50–200 ns, bounding the
  addressable saving at **≈3–13 µs/block at 64 sources (≤26 µs at 128)** —
  under 0.5% of the 2 666 µs block budget, and under 6% of the engine's own
  222 µs render cost. The plan's 2–4 µs/call projection came from the
  `postMessage` era, where each submit was a structured-clone message; the SAB
  drain already deleted that cost (28 600 → 0 messages/s at 64 sources).

**Field corroboration.** The feed sustains 64 and 128 sources with zero
underruns in the fixtures, and the live-browser lifecycle probe on
misofm/app#45 showed zero source messages and clean seeks with the per-call
path in place.

**Ruling.** Null. The per-call share is noise once the feed is shared memory;
the contract evolution (multi-quantum submit, #101) stays available if a
future measurement disagrees. Closed without building, per the
owner-approved decision procedure on #148.
