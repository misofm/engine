# Research: default ramp lengths for live mute, fader and pan changes (cited, measured, listened)

Stream E of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-1, D15-13 E5).
Code anchors in the decision-15 sections verified on `main` at `6fb211594`.

Owner ruling (2026-09-28): the defaults for the session's `controlSmoothing` settings are chosen from evidence, not a rule of thumb. The proposed starting point is about 5 ms for mute/solo and about 20 ms for fader and pan moves; this research confirms or replaces it. AGENTS.md's evidence rules apply (primary citations, fixtures, objective tests, listening evidence).

## Product outcome

`docs/handoffs/control-smoothing-defaults/FINDINGS.md` gives a cited and measured default, with its
`controlSmoothing` key, for every live row decision 15 ramps: fader, mute, solo, pan, raw matrix,
input trim, polarity invert, send gain, send mute, send matrix, VCA offset, VCA mute, the effect
bypass crossfade and the detector link glide. *Session `controlSmoothing`: configurable ramp lengths
for live mute, fader and pan changes* (#1054) ships that table as its defaults. This issue closes on
that record. The blinded listening session is not part of it: *Run the blinded listening session
for the live ramp defaults* (#1388) runs it on the shipped defaults, and its result can change only
default values (decision 15, D15-1 recorded resolution).

## Questions

1. **What do others do?** From official documentation only: mute, solo and fader/pan smoothing or declick times in at least two production DAWs (for example Logic, Ableton Live, Pro Tools) and at least two digital consoles (for example DiGiCo, SSL, Yamaha), plus common plugin frameworks' parameter-smoothing guidance. Cite each; say where nothing is documented.
2. **Objective measurement.** On realistic material (a bass note, a kick, a full mix), measure the click energy of a mute at 0, 2, 5, 10, 20 and 50 ms, and the zipper of a fader drag with control updates at 30 and 60 Hz, for each ramp length. Use the engine's own fader, mute and matrix ramps.
3. **Listening packet.** Prepare a short blinded comparison packet the owner can run (mute click, drag smoothness, responsiveness), following the repository's listening-evidence conventions. Running it is #1388.

## Status (2026-10-05)

- Questions 1 and 2 and the listening packet of question 3 are delivered in
  `docs/handoffs/control-smoothing-defaults/` (`FINDINGS.md` sections 1-8, `data/`, `measure/`,
  `listening/`). The recommended table is mute (and solo) 10 ms, fader 20 ms, pan and raw matrix
  20 ms (`FINDINGS.md:11-17`).
- `FINDINGS.md` has no section 9 yet. What remains is the decision-15 addition below.
- No human has run the packet (`FINDINGS.md:257`). That is #1388's work, not a closing condition
  here.

## Decision-15 addition: every live row (D15-1) and the bypass switch (D15-13 E5)

Decision 15 makes every live value ramp over the session's `controlSmoothing` (#1054). The table
has three keys (`muteMs`, `faderMs`, `panMs`), and #1054 D3 maps every live row onto one of them.
This research backs that mapping with a default for each row. Use the same harness, materials,
measures and rates as questions 1 and 2.

4. **Which key and default each extra row takes.** For each row, name the kernel and its ramp law
   with a `path:line` anchor. If the law is the one already measured (a linear retarget of a gain
   or of a 2x2 coefficient), say so: the existing measurement transfers. Otherwise measure the row.
   - Input trim: `set_trim_signed` (`crates/builtins/src/lib.rs:1434-1478`), the D11 retarget of
     one signed coefficient. Expected key: `faderMs`.
   - Polarity invert: the same call, with the coefficient carried **through zero**
     (`lib.rs:1436-1439`). The switch is a step of twice the signal, and the ramp reaches silence
     at half its length. **Measure it** (mute-click method; bass, kick and mix; 0, 2, 5, 10, 20 and
     50 ms), and say whether `muteMs` is enough or whether polarity needs a longer ramp.
   - Send gain, mute and matrix: the live route ramp (`LiveRoute`, `crates/graph/src/runtime.rs:861`)
     uses the indexed law (`IndexedRamp`, `crates/lane/src/kernels.rs:1073-1168`), not D11. Show
     that its trajectory matches D11 to within rounding for the measured lengths, so the fader,
     mute and pan results transfer. Expected keys: `faderMs`, `muteMs`, `panMs`.
   - VCA offset and mute: they reach render as member fader and mute records
     (`crates/host-core/src/vca.rs:1-13`). The key is the member's `faderMs` and `muteMs`. Source
     check only.
5. **The bypass crossfade (decision 14 F7).** The shunt selects whole blocks of dry or wet
   (`crates/effect-contract/src/live.rs:841-843`), so a bypass toggle steps the output by
   `wet - dry`. *Crossfade the bypass switch over the session ramp* (#1341) replaces the step with a
   linear crossfade.
   - Measure the step's click (OOB, CTR, HF splatter) for each launch effect that has a shunt, at
     representative settings: compressor and limiter at 6 dB of gain reduction, EQ at +/-6 dB,
     saturator driven, transient shaper at 50 %, gate closing on the kick. Measure at 0 ms (today)
     and as a crossfade of 2, 5, 10 and 20 ms.
   - Emulate the crossfade in the harness with #1341's law: `out = dry + (wet - dry) * m(k)`, where
     `m(k)` follows the indexed law and both ends are exact copies.
   - Answer: is `muteMs` the right key, and its default the right length, for the crossfade?
6. **The detector link glide** (*Ramp a lane's detector link between modes*, #1370; *Carry the link
   record from the edit to the lane*, #1371). A link-mode change moves gain reduction, not the
   signal path. Expected key: `faderMs`.
   - Measure the compressor at 6 dB of gain reduction on the stereo mix (uncorrelated channels),
     switching `dual_mono` to `maximum` and back, at 0, 2, 5, 10, 20 and 50 ms.
   - Emulate with the same output crossfade as question 5, with the two modes' outputs in place of
     dry and wet. State that this emulates #1370's detector blend, and why its click bounds the
     real glide's: the effect's own ballistics smooth a detector blend further.
   - Answer: is `faderMs` right, or does the link need `muteMs`?

## Decisions frozen for this slice

- **D1. Output.** A section 9 in `FINDINGS.md` with one table: row, kernel anchor, ramp law, key,
  default in ms, and evidence (a CSV in `data/`, or a source argument that the law is one already
  measured). Every row of the product outcome appears. The new CSVs go in `data/`; harness changes
  go in `measure/`, which stays a standalone package outside the workspace.
- **D2. The table is the default record.** #1054 copies section 9's key column into its D3 table
  and its values into `CONTROL_SMOOTHING_DEFAULT`. If section 9 changes a key or a length from
  #1054's or #1341's text, the closing comment says so, and those issues follow it.
- **D3. Listening is separate.** Sections 1, 6 and 9 say that the defaults are the measured ones,
  that #1388 runs the preregistered session on them, and that a listening result changes values
  only. The preregistration (`listening/PREREGISTRATION.md`) is not edited here, except to add new
  contrasts for questions 4-6 if section 9 recommends them. Any such addition is made before any
  trial and recorded as an amendment with its date.
- **D4. No product code.** No file outside `docs/handoffs/control-smoothing-defaults/` changes.

## Deliverables

1. `FINDINGS.md` section 9 (D1), and the one-line pointers of D3 in sections 1 and 6.
2. The new CSVs in `data/` and the harness changes in `measure/` (bypass and link emulation, the
   polarity and route-law measurements).
3. The `measure/README.md` commands for the new outputs.

## Authorized paths

- `docs/handoffs/control-smoothing-defaults/` (all of it)

## Non-goals

- Running the listening session (#1388).
- Any product code, schema or default change (#1054).
- A new ramp shape (raised cosine or equal power). Section 5.5's what-if stays research.

## Objective gates

1. The harness tests pass from `measure/`:
   `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=<scratch>/target cargo test --release --offline`.
2. The new CSVs reproduce byte for byte over two runs.
3. `bash scripts/check-workspace-policy.sh` and `bash scripts/check-env-vocabulary.sh` pass.
4. Every row of section 9 has a kernel anchor and either a measurement or a source argument for why
   its law is one already measured. Every row has a key and a default in ms.
5. `python3 docs/handoffs/control-smoothing-defaults/listening/listening.py self-test` passes (the
   packet still prepares and validates if D3 added contrasts).

## Test value

- The harness unit tests that the new emulation adds turn red if the crossfade emulation uses an
  accumulated `m` instead of the indexed law, or if an endpoint is computed rather than copied. No
  test checks a crossfade emulation today. No product test is added.

## Dependencies

- None. *Session `controlSmoothing`: configurable ramp lengths for live mute, fader and pan changes*
  (#1054) and *Crossfade the bypass switch over the session ramp* (#1341) depend on this issue.
  *Run the blinded listening session for the live ramp defaults* (#1388) follows #1054.
