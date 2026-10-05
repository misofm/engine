# Research: default ramp lengths for live mute, fader and pan changes (cited, measured, listened)

Stream E of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-1, D15-13 E5).
Code anchors in the decision-15 sections verified on `main` at `6fb211594`.

Owner ruling (2026-09-28): the defaults for the session's `controlSmoothing` settings are chosen from evidence, not a rule of thumb. The proposed starting point is about 5 ms for mute/solo and about 20 ms for fader and pan moves; this research confirms or replaces it. AGENTS.md's evidence rules apply (primary citations, fixtures, objective tests, listening evidence).

## Questions

1. **What do others do?** From official documentation only: mute, solo and fader/pan smoothing or declick times in at least two production DAWs (for example Logic, Ableton Live, Pro Tools) and at least two digital consoles (for example DiGiCo, SSL, Yamaha), plus common plugin frameworks' parameter-smoothing guidance. Cite each; say where nothing is documented.
2. **Objective measurement.** On realistic material (a bass note, a kick, a full mix), measure the click energy of a mute at 0, 2, 5, 10, 20 and 50 ms, and the zipper of a fader drag with control updates at 30 and 60 Hz, for each ramp length. Use the engine's own fader, mute and matrix ramps.
3. **Listening.** Prepare a short blinded comparison packet the owner can run (mute click, drag smoothness, responsiveness), following the repository's listening-evidence conventions.

## Output

`docs/handoffs/control-smoothing-defaults/` with the citations, the measurements (committed script and data) and the listening packet, and a recommended default table for the `controlSmoothing` issue. No product code change.

## Status (2026-10-05)

- Questions 1 and 2 and the listening packet of question 3 are delivered in
  `docs/handoffs/control-smoothing-defaults/` (`FINDINGS.md`, `data/`, `measure/`, `listening/`).
  The recommended table is mute (and solo) 10 ms, fader 20 ms, pan and raw matrix 20 ms
  (`FINDINGS.md:11-17`). It is provisional on the preregistered listening rules
  (`FINDINGS.md:47-51`, `listening/PREREGISTRATION.md`).
- No human has run the packet yet (`FINDINGS.md:257`). The issue closes with the listening result
  and the decision-15 addition below.

## Decision-15 addition: every live row (D15-1) and the bypass switch (D15-13 E5)

Decision 15 makes every live value ramp over the session's `controlSmoothing`
(*Session `controlSmoothing`: configurable ramp lengths for live mute, fader and pan changes*,
#1054). The table has three keys, and #1054 D3 maps every live row onto one of them. This research
must back that mapping before #1054 starts. Use the same harness, materials, measures and rates as
questions 1 and 2.

4. **Which key each extra row takes.** For each row, name the kernel and its ramp law with a
   `path:line` anchor. If the law is the one already measured (a linear retarget of a gain or of a
   2x2 coefficient), say so, and the existing measurement transfers. Otherwise measure the row.
   - Input trim: `set_trim_signed` (`crates/builtins/src/lib.rs:1434-1478`), the D11 retarget of
     one signed coefficient. Expected key: `faderMs`.
   - Polarity invert: the same call, with the coefficient carried **through zero**
     (`lib.rs:1436-1439`). The switch is a step of twice the signal, and the ramp reaches silence
     at half its length. **Measure it** (mute-click method, bass, kick and mix, 0, 2, 5, 10, 20 and
     50 ms), and say whether `muteMs` is enough or whether polarity needs a longer ramp.
   - Send gain, mute and matrix: the live route ramp (`LiveRoute`, `crates/graph/src/runtime.rs:861`)
     uses the indexed law (`IndexedRamp`, `crates/lane/src/kernels.rs:1073-1097`), not D11. Show
     that its trajectory matches D11 to within rounding for the measured lengths, so the fader,
     mute and pan results transfer. Expected keys: `faderMs`, `muteMs`, `panMs`.
   - VCA offset and mute: they reach render as member fader and mute records
     (`crates/host-core/src/vca.rs:1-13`). The key is the member's `faderMs` and `muteMs`. Source
     check only.
5. **The bypass switch (decision 14 F7).** The shunt selects whole blocks of dry or wet
   (`crates/effect-contract/src/live.rs:835-845`), so a bypass toggle steps the output by
   `wet - dry`. Decision 15 E5 replaces the step with a linear crossfade over a session ramp
   (*Crossfade the bypass switch over the session ramp*, #1341).
   - Measure the step's click (OOB, CTR, HF splatter) for each launch effect that has a shunt.
     Use representative settings: compressor and limiter at 6 dB of gain reduction, EQ at
     +/-6 dB, saturator driven, transient shaper at 50 %, gate closing on the kick. Measure at
     0 ms (today) and as a crossfade of 2, 5, 10 and 20 ms.
   - Emulate the crossfade in the harness with #1341's law: `out = dry + (wet - dry) * m(k)`,
     where `m(k)` follows the indexed law and both ends are exact copies.
   - Answer: is `muteMs` the right key and length for the crossfade?

Output for 4 and 5:
- a section 9 in `FINDINGS.md` with the row-to-key table and its evidence;
- the new CSVs in `data/`;
- the harness changes in `measure/`, still standalone and not a workspace member.

No product code changes. If the answer changes a key or a length, #1054 D3 and #1341 follow the
closing record.

## Gates for the addition

- The harness tests pass from `measure/`:
  `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=<scratch>/target cargo test --release --offline`.
- The new CSVs reproduce byte for byte over two runs.
- `bash scripts/check-workspace-policy.sh` and `bash scripts/check-env-vocabulary.sh` pass.
- Every row in the table has an anchor or a measurement. "Not measured" is allowed only with a
  source argument for why the law is identical.

## Dependencies

- None. #1054 and #1341 depend on this issue.
