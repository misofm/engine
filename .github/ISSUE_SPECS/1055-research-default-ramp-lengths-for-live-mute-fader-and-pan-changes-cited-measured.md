# Research: default ramp lengths for live mute, fader and pan changes (cited, measured, listened)

Stream E of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-1, D15-13 E5).
Code anchors in the decision-15 sections verified on `main` at `6fb211594`.

Owner ruling (2026-09-28): the defaults for the session's `controlSmoothing` settings are chosen from evidence, not a rule of thumb. The proposed starting point is about 5 ms for mute/solo and about 20 ms for fader and pan moves; this research confirms or replaces it. AGENTS.md's evidence rules apply (primary citations, fixtures, objective tests, listening evidence).

## Product outcome

`docs/handoffs/control-smoothing-defaults/FINDINGS.md` gives a cited and measured default, with its
`controlSmoothing` key, for every live row decision 15 ramps: fader, mute, solo, pan, raw matrix,
input trim, polarity invert, send gain, send mute, send matrix, the send's `follows_mute` toggle,
VCA offset, VCA mute, the effect bypass crossfade and the detector link glide. *Session `controlSmoothing`: configurable ramp lengths
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
   - The send's `follows_mute` toggle: turning it on while the source is muted, or off while it is
     muted, ramps the send between open and silent on the same live route ramp as a send mute.
     Expected key: `muteMs`. Source check only, if the law is the send mute's.
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

None.

*Session `controlSmoothing`: configurable ramp lengths for live mute, fader and pan changes*
(#1054) and *Crossfade the bypass switch over the session ramp* (#1341) depend on this issue.
*Run the blinded listening session for the live ramp defaults* (#1388) follows #1054.

## Attempt record

### Attempt 1 (2026-10-05)

**Done.** The decision-15 addition, questions 4-6 and D1-D4, in
`docs/handoffs/control-smoothing-defaults/`: `FINDINGS.md` section 9 (one table, every row of the
product outcome, then 9.2-9.8), the one-line D3 pointers in sections 1 and 6, the header updated in
place, [REISS-COMP] added to section 8; four new CSVs in `data/` (`polarity_click.csv`,
`law_transfer.csv`, `bypass_crossfade.csv`, `link_glide.csv`); `measure/` gains the input section
in `src/strip.rs`, a `live` subcommand (`src/live_rows.rs`), the six shunted effects through their
factories and the engine's `BypassShunt` (`src/effects.rs`), #1341's crossfade law
(`src/crossfade.rs`), path dependencies only in `measure/Cargo.toml`, and the new tables in
`summarise.py`; `measure/README.md` has the commands and columns. No file outside the authorized
path changed except this record (D4). Evidence commit `d541cc11c` on `codex/d15-stream-e`.

**Results.** Keys and defaults: fader, input trim, send gain, VCA offset, detector link glide
`faderMs` 20 ms; mute, solo, send mute, send `follows_mute` toggle, VCA mute, bypass crossfade
`muteMs` 10 ms; polarity invert `muteMs` doubled, 20 ms; pan, raw matrix, send matrix `panMs`
20 ms. One length differs from #1054 D3 (and so from #1261 D2, #1364, #1394): polarity invert is
`2 * mute_samples`, because a flip over `N` clicks 6 dB more than a mute over `N` and a flip over
`2N` matches the mute over `N` within 0.80 dB on OOB, worst OOB, CTR and HF splatter at every rate;
on total splatter it does not (at 20 ms against 10 ms: bass +2.11 to +2.20 dB, kick -0.94 to
-0.99 dB, mix +0.30 to +0.94 dB) (`FINDINGS.md` 9.2, 9.8). [Corrected in attempt 2: this sentence
said "within 0.8 dB on every measure and rate", which `polarity_click.csv` and `mute_click.csv`
contradict on total splatter.]
#1341's `mute_ms` and #1370/#1371's `fader_ms` are confirmed. No listening contrast was added
(9.6); `listening/` is unchanged.

**Anchors.** No file named by this spec's anchors changed between `6fb211594` and `8be19c86e`; each
anchor still reads as quoted (`crates/builtins/src/lib.rs:1434-1478`, `:1436-1439`,
`crates/graph/src/runtime.rs:861`, `crates/lane/src/kernels.rs:1073-1168`,
`crates/host-core/src/vca.rs:1-13`, `crates/effect-contract/src/live.rs:841-843`). `FINDINGS.md`
lines 11-17 and 19-30, which #1054 cites, did not move; the "No human has listened" line this spec
cites as `:257` is now `:260`.

**Gates** (all run on the final tree):

1. `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/tmp/claude-1002/r1055/target cargo test --release
   --offline` from `measure/`: 16 passed (9 earlier, 7 new); `cargo clippy --release --offline
   --all-targets` clean.
2. `control_smoothing_measure live` run twice, all four rates, into two scratch directories: `cmp`
   identical. SHA-256: `bypass_crossfade.csv` a5fe06ddb447c6e4825716466cfeca88f65fb5976948bcf4d62361653778e6f5,
   `law_transfer.csv` 89eb4b73b6c9e8a6700da2240d4ec8b98e4d43c8ac5ed03e26a804014b14a52b,
   `link_glide.csv` 98d30ca0e1299a1e870ec8f2776ec4d7da1f86b867f9c9e2bcb545662fdf6afc,
   `polarity_click.csv` e22ff0d64baacb243e2478369972b2735de0266ef40188fb9878d839d513e2b6. Two
   single-rate (48 kHz) runs, one before and one after the last source edit, match the committed
   48 kHz rows. The seven earlier CSVs reproduce byte for byte both with the harness at `8be19c86e`
   and with the extended harness (`measure`, all four rates).
3. `bash scripts/check-workspace-policy.sh`: ok. `bash scripts/check-env-vocabulary.sh`: ok.
   `bash scripts/check-dsp-research.sh`: ok.
4. A script over `FINDINGS.md` 9.1: 15 rows, the product outcome's 15 in order, each with a
   `path:line` anchor, a key, a default in ms and a CSV or source argument.
5. `TMPDIR=/tmp/claude-1002/r1055/tmp python3 docs/handoffs/control-smoothing-defaults/listening/listening.py self-test`:
   passed (68 trials; the packet is unchanged).

**Mutation evidence** (each applied to `measure/src/crossfade.rs`, then reverted to identical bytes
with `cmp`; `cargo test --release --offline crossfade`):

- Accumulated mix (`m += step` per frame in place of `IndexedRamp::coefficients_at(k)`): red,
  `the_mix_follows_the_indexed_law_from_the_first_frame_after_the_record` ("length 441, Dry, frame 1
  (k = 2)") and `a_ramping_frame_is_dry_plus_the_scaled_difference`.
- Settled end after the ramp computed (`dry + (wet - dry) * target` in place of the copy): red,
  `both_ends_are_exact_copies_of_the_settled_planes` (the "after" assertion).
- Settled end before the record computed (`dry + (wet - dry) * m_start` in place of the copy): red,
  the same test (the "before" assertion).
- Reverted: 3 passed.

**Open items.**

- #1054, #1261, #1364 and #1394 follow 9.8 (polarity `2 * mute_samples`). Root carries this in the
  closing comment (D2).
- Not verified (`FINDINGS.md` 9.7): the real #1341 crossfade and #1370 glide (emulated here; their
  own gates test the kernels); the link bound below about 1 ms of compressor attack; whether a
  polarity flip's brief level dip at its midpoint is audible as a dip (a question for any
  polarity-kernel successor, not for #1388); effect settings beyond the representative ones; a
  transient shaper switched during a kick's attack; banked, AArch64 and wasm renders.
- [REISS-COMP] could not be opened in this session (the author link redirects, the AES page refused
  the fetch); 9.5 rests on the kernel anchors and an inline derivation and cites it for context only.
- #1388 remains owner-pending: no listening response or result exists or was simulated.
- This spec's Status section ("`FINDINGS.md` has no section 9 yet") is stale after this attempt;
  only this record was appended (D4).

### Attempt 2 (2026-10-05)

**Why.** Attempt 1's adversarial verdict (kept outside the repository) failed it on one MAJOR
finding, record text only: the polarity parity "within 0.8 dB on every measure and rate" is
false on total splatter. It also listed MINOR-1 to MINOR-4 and NIT-1 to NIT-7. The harness, data
and gates were confirmed sound.

**Done.** Evidence commit `21a1d5af4` on `codex/d15-stream-e`; only
`docs/handoffs/control-smoothing-defaults/` and this record changed (D4).

- MAJOR-1: `FINDINGS.md` 9.2 states the parity only on OOB, worst OOB, CTR and HF splatter, and
  reports total splatter per material, rate and length. The reason: a flip over `2N` is two mutes
  over `N` back to back, so its gain spectrum is the `N`-mute's times `|2 cos(pi f T)|`. Far from the
  source (where the four click measures read) the band energies match; near it (where total
  splatter starts, 56-82 Hz from the bass's low partials) the factor runs between 0 and 2, so total
  splatter changes sign with length (bass: +4.5 to +4.8, -3.1 to -3.2, +2.1 to +2.2 and -0.8 to
  -1.1 dB at 2/1, 10/5, 20/10 and 40/20 ms). A one-partial model (`flip_model_db`,
  `measure/summarise.py`) gives ranges that hold the measured bass values at 10/5, 20/10 and 40/20
  and miss them by 0.2 dB at 2/1. The factor of two stands under section 1's
  rule, which decides by OOB, CTR and HF splatter; 9.6 is re-justified on that basis (no contrast,
  no amendment: the factor is a rule, a listening result moves values only, and no factor
  equalises total splatter). The sentence of attempt 1's record above is corrected in place.
- The flip is now also measured at 40 ms (`POLARITY_RAMPS_MS`), twice the largest `muteMs` the
  preregistered rules can choose, so the parity is measured for every rule outcome: within 0.80 dB
  at 20/10 on the four click measures; at 10/5 and 40/20 within 1.19 and 1.00 dB on OOB and HF
  splatter, and within 2.40 dB on the per-event peaks (worst OOB, CTR).
- MINOR-1: the mute-reference rows reproduce `mute_click.csv` within 0.20 dB on OOB (bypass against
  mute and unbypass against unmute), 0.09 dB on worst OOB, 0.12 dB on CTR and 0.01 dB on splatter
  and HF splatter (9.4, 9.7). The verdict's 0.13 dB compares with the mute rows only; against the
  unmute rows the OOB gap reaches 0.20 dB.
- MINOR-2: 9.5 and 9.7 state that only the compressor's link glide was assessed, why the bound
  does not carry to the gate-expander as it stands, and that no #1370 gate measures a click.
- MINOR-3: 9.1 adds the matrix coefficient through zero (raw matrix in [-1, 1], `B:88-102` and
  `B:3019`; send matrix any finite value, `crates/graph-compiler/src/ids.rs:288-304`, `:341`) and
  points to 9.2.
- MINOR-4: 9.2 weighs the four choices (the mute's own length, `faderMs`, a fourth key
  `polarityMs`, the derived `2 x muteMs`) and the cost of the longer dip (at 20 ms: 10 ms below
  -6 dB, 2 ms below -20 dB); 9.8 lists each dependant's wording change and the one choice root
  confirms (the derived rule inside the three-key schema).
- NIT-1: the largest excess per measure is reported; on worst OOB it is -0.01 dB (the gate on the
  kick at 44.1 kHz, 2 and 10 ms, and at 48 kHz, 0 and 10 ms; the verdict's "48 kHz 2 ms" is
  -0.02 dB). NIT-2: the reductions are rate-qualified (36.8-41.2 and 26.2-40.8 dB over all rates)
  and the transient shaper on the kick is reported apart (10.2-11.2 dB). NIT-3: 16.8 dB. NIT-4:
  [REISS-COMP] removed from section 8 (it could not be opened in this session either). NIT-5: the
  coefficient anchor is `rate_coefficient`, `crates/compressor/src/design.rs:129-143` (reason
  `:18-30`); `envelope.rs:163-176` stays as the recurrence `ballistic` calls. NIT-6: the new test
  below. NIT-7: 9.7 lists the delay and multiband compressor crossfades (#1339, #1340) as not
  measured.
- `summarise.py` prints every new range of 9.2-9.5; `measure/README.md` names the 40 ms rows.

**New test and its value.** `live_rows::tests::a_retarget_restarts_the_index_from_where_the_route_is`
turns red if the route probe keeps its position when a record lands (mid-ramp or after the ramp
has settled), which `the_route_probe_mixes_by_the_indexed_law_from_the_applying_block` (one record
from a settled state) cannot see. Mutation (the `position = 0;` line of `route_probe` deleted):
red, only this test, "ll, frame 384" (the first frame of the second record's block); 16 others
pass. Restored with `cp` and checked identical with `cmp`: 17 passed.

**Gates** (all run on the final tree):

1. `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/tmp/claude-1002/r1055/target cargo test --release
   --offline` from `measure/`: 17 passed (9 earlier, 8 new); `cargo clippy --release --offline
   --all-targets` clean.
2. `control_smoothing_measure live` run twice, all four rates, into two scratch directories: `cmp`
   identical. `bypass_crossfade.csv`, `law_transfer.csv` and `link_glide.csv` are byte-identical to
   the committed files (SHA-256 unchanged from attempt 1). `polarity_click.csv` changes on purpose:
   it gains the 24 rows at 40 ms, and with them removed it is attempt 1's file byte for byte; new
   SHA-256 801274e7742bb812aba3c91e41be2e634f0c4a223eb577e47eed5cbb7c215e51.
   `control_smoothing_measure measure` run twice, all four rates: the seven earlier CSVs are `cmp`
   identical to each other and to the committed files.
3. `bash scripts/check-workspace-policy.sh`: ok. `bash scripts/check-env-vocabulary.sh`: ok
   (59 names). `bash scripts/check-dsp-research.sh`: ok.
4. A script over `FINDINGS.md` 9.1: 15 rows, the product outcome's 15 in order, each with a
   `path:line` anchor, a key, a default in ms and a CSV or source argument.
5. `TMPDIR=/tmp/claude-1002/r1055/tmp python3 docs/handoffs/control-smoothing-defaults/listening/listening.py self-test`:
   passed (68 trials; `listening/` is unchanged).

**Open items.**

- Root carries 9.8 in the closing comment (D2) and confirms the derived rule `2 * mute_samples`
  inside the three-key schema, over #1054's present text and over a fourth key.
- Not verified (`FINDINGS.md` 9.7): the real #1341 crossfade and #1370 glide; the link bound below
  about 1 ms of attack; the link glide of the gate-expander, transient shaper and true-peak
  limiter; the delay and multiband compressor crossfades (no shunt yet); whether the polarity
  flip's dip through zero is heard as a dip (a kernel question for a successor root may open; #1388
  cannot act on it); settings beyond the representative ones; banked, AArch64 and wasm renders.
- #1388 remains owner-pending: no listening response or result exists or was simulated.

### Follow-ups (2026-10-05)

Attempt 2 received PASS (`docs/handoffs/decision-15-2026-10-05/verdicts/stream-e/1055-attempt2.md`;
attempt 1's verdict is beside it). Its four findings are folded into `FINDINGS.md`, text only, in
the commit that adds this entry; no CSV, harness or `listening/` file changed.

- MINOR-1: 9.8 adds the three missed places that pin polarity to the plain mute length: #1261 gate 1
  (`1261-*.md:146-147`), #1364 gate 1 (`1364-*.md:131-132`, polarity resolves to 960 at 48 kHz)
  and D2 (`:62`), and #1394's background sentence (`1394-*.md:26-27`), its only change. The specs
  themselves are not edited; root carries the list.
- NIT-1: 9.2 fact 1 gives HF splatter as +6.00 to +6.03 dB, apart from total splatter (+6.02 to
  +6.03 dB), checked against `polarity_click.csv` and `mute_click.csv`.
- NIT-2: total splatter "is not a measure section 1 decides by".
- NIT-3: the matrix-through-zero paragraph notes that the #1388 outcome `muteMs` 20 ms with
  `panMs` 20 or 35 ms puts `panMs` below twice `muteMs` at the defaults, with a pointer to 9.2 and
  9.8 and no new rule.
