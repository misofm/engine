# Catch a render kernel that the wasm build fully scalarises

Filed by root from #877's verification (backlog sprint, 2026-10-24).

## Product outcome

`scripts/check-web-audioworklet-callgraph.py` `kernel_arithmetic` skips every function with zero `f32x4` arithmetic as a helper. A non-roster render kernel (for example `InputStage::process`, `FaderRampStage::process_plane`, `route_mix_column_ramps_block`, the bypass crossfades) that a compiler or source change makes *fully* scalar therefore drops out of the count with no failure. Only rule 1 (the roster) catches this, and only for roster kernels; the K ratchet (26 counted against 13 today) would let 13 kernels vanish silently. A partly scalar kernel is already caught by rule 3.

## Smallest closable slice

Pin the set of render-kernel symbols that must stay present and vector-carrying: a reviewed table in the style of `INDIRECT_SITES` (symbol without the hash, one-line reason), checked on every artifact; a listed kernel that disappears or reaches vector == 0 fails; a stale entry fails. Seed it from the 26 kernels the 1.99.1 artifact counts today.

## Gates

The analyser's `--self-test` gains rows: a listed kernel made fully scalar fails; a removed listed kernel fails; a stale entry fails. `check-web-audioworklet.sh` passes on the shipped artifact.

## Paths

`scripts/check-web-audioworklet-callgraph.py`, its self-test fixtures, this spec.

## Decision and evidence record

- **Decisions (root, from the scope note).** `RENDER_KERNELS: dict[str, str]` in the analyser (unhashed symbol, the stage it renders), `INDIRECT_SITES` style. `check_kernel_shape` gains `pinned=None` beside #877's `exempt`, and `main()` passes the table, so every `--kernel-shape` run covers it with no shell change (K stays 13). Each entry must resolve to exactly one function over the whole module (0 and more than 1 share one failure branch) with `KERNEL_VECTOR > 0`. Every counted kernel is pinned, roster and tiny ones included. Docstring rule 5. No fixture files: the self-test fixtures are inline.
- **Seed (measured).** The 1.99.1 named twin of shipped module `47e9c699` (main `6172dc4e6`) counts 30 functions matching `4wide6f32x4` with `f32x4` arithmetic. The table pins 28 of them: the 13 roster kernels, the collapsed limiter core, the two input-section bodies, two fader ramp planes, three `silence_skip_settle` instances, `route_mix_column_ramps_block`, two bypass crossfades, two `advance_clocks`, `parametric_eq::Channel::rest_lanes` and `compressor::kernel::settled_sidechain`. The spec's earlier "26" was a 1.97.1 build of another branch; main has since added the multiband, soft clip and transient shaper bodies.
- **Implementer decision.** One of the 30, `transient_shaper::Shaper::new` (4/0), is the shaper's constructor, called only from `prepare` and the bank factory, so it is not a render kernel. It joins `CONTROL_THREAD_EXEMPT` instead of the pinned table, which keeps the two tables disjoint (row e3) and makes the pinned set equal to the counted set: `kernels=28` with 2 exempt.
- **Gates.** Self-test rows e1 (a pinned kernel at vector 0 fails, at vector 1 passes), e2 (an entry that matches no function fails), e3 (the shipped table is hash-free and disjoint from `CONTROL_THREAD_EXEMPT`). One mutant per row turned only that row red: e1 with the `vector == 0` branch disabled, e2 with the count test made `> 1`, e3 with the exempt `decode_track` symbol and with a hashed symbol added to the table.
- **Attempt 2 (verifier MAJOR).** With `pinned` given, a counted kernel in neither `RENDER_KERNELS` nor `CONTROL_THREAD_EXEMPT` now fails, so the table cannot go stale when a kernel is added. Self-test row e4 (an unpinned counted kernel fails); its mutant (the new branch disabled) turned only e4 red. A PR that adds a counted kernel adds its row; #1922 and #1925 do so, and #1889 merges after them with their rows measured from a fresh twin.
- **Rebase onto #1922 and #1925 (measured).** On the named twin of shipped module `8e0e4e79`, the enforced rule reported the de-esser's five counted kernels unpinned: the two `run_block` inner closures (stereo 561/0, mono 283/0), `process_bank` (2/0), `process_bank_mono` (1/0) and `advance_clocks` (2/0). It also reported the three `PreparedParametricEq` rows vanished, because the impl index moved from `Xs9` to `Xsa` with identical counts (1352/1, 748/1, 2/0): a rename, so those rows were re-keyed. The table now pins 33, and the gate counts 33 kernels with 2 exempt.
