# Add the fused gain/pan console row

Scoping study: `docs/handoffs/builtins-less-removal-2026-09-27/SCOPE.md` (owner-directed removal, 2026-09-27; option (b) per the owner's second ruling below).

## Amendments (adversarial verification, 2026-09-27; override conflicting text)

This row is the browser's primary pure-path target, so it must be the browser boot's real shape:
between-render-calls delivery (fader and matrix fused), **driver-fed** through the production
source set, and **metered** as the default boot selects (`SAMPLE_PEAK` at `PostMatrix`, 12 x 128,
permanent observers). Add it to the wasm console arm too. Its digest equals the gain/pan ring
row's. Land after #956, one at a time with #955, #938 and #965 (shared record-count lines).

## Rulings (coordinator, 2026-09-27)

The owner directed the complete removal of the builtins-less path ("I don't think we should be
benchmarking something that never gets used in the real world"). Option (b) is taken, by the owner's
second ruling ("We should remove everything related to a builtins-less compile because a
builtins-less compile is never needed in production"): the builtins-less compile is removed
entirely, including any test-only entry point, builtins become mandatory in the graph compiler,
and every test that compiled without builtins is ported to `compile_with_builtins`. The optimisation batch lands unchanged and this work deletes #937's
code afterwards. The wasm console arm is re-indexed. `sixty_four_track_gain_pan_only` is the
pure-audio-path target; its fused twin follows. #938 is re-based onto the gain/pan session. The
phase-profile harness is retargeted to `gain_pan_only`. Ported tools compile at
`Backend::current()`.

## Smallest closable slice

- **S4. Add `sixty_four_track_gain_pan_fused`.** It is the web boot's between-render-calls
  no-effects plan, the gain/pan twin of #955, and the browser's pure-path target.

Follow #881's pattern and #941/#928's list of files a new row touches (floor pin, record counts, validators, self-test, preflight). Its digest equals `sixty_four_track_gain_pan_only`'s, and its chain-stage count is pinned (fader and matrix fused).

## Dependencies

#956; #881.

