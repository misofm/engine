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
- The decision-15 addition below is delivered as `FINDINGS.md` section 9 (root, 2026-10-05, from
  #1055): commits `d541cc11c` (attempt 1), `21a1d5af4` (attempt 2), `17f0bf18c` (the attempt-2
  verdict's follow-ups) and `8eacefa34` (listening Amendment 1). The verdict is PASS on attempt 2
  (`docs/handoffs/decision-15-2026-10-05/verdicts/stream-e/1055-attempt2.md`). Root's ruling
  confirms the polarity row at twice `muteMs`, a derived rule with no fourth key (9.2, 9.8).
- Amendment 1 to `listening/PREREGISTRATION.md` adds block P (36 polarity trials, made before any
  trial); the session is now 104 trials (root, 2026-10-05, from #1055).
- No human has run the packet (`FINDINGS.md:260`). That is *Run the blinded listening session for
  the live ramp defaults* (#1388), owner-pending, not a closing condition here (root, 2026-10-05,
  from #1055).

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

### Follow-up: #1388 polarity trials (root, 2026-10-05)

**Why.** Root's ruling on #1055 (2026-10-05): the polarity row's derived rule (`2 x muteMs`, no
fourth key) is confirmed, and #1388's blinded packet gains polarity-invert trials before anyone
runs it, to answer the open question of `FINDINGS.md` 9.7: at the shipped rule, is the flip heard
as a click or as a level dip? A listening result changes default values only. D3 allows a contrast
added before any trial as a dated amendment; no packet, trial, response or reveal existed.

**Done.** Evidence commit `8eacefa34` on `codex/d15-stream-e`; only
`docs/handoffs/control-smoothing-defaults/` and this record changed (D4). No listening response,
answer or result was written or simulated outside the self-test's temporary directory.

- `listening/PREREGISTRATION.md`: "Amendment 1, 2026-10-05, before any trial" (reason, source,
  question, counts and power, statistics, decision rule, stimulus identity, training) before the
  sign-off, and one pointer line under "Identity and status". The diff adds lines only; status stays
  `preregistered`.
- **Block P, 36 trials** (session 104): positive controls P-bass-0-oob (a hard flip's out-of-band
  change on the unflipped note: a click) and P-bass-200-inband (a 200 ms flip's in-band change: a
  dip), 2 trials each; primaries P-bass-20 (the shipped 20 ms flip), P-bass-20-inband (its in-band
  change only: the dip) and P-bass-20-oob (its out-of-band change only: the click), 8 trials each;
  P-kick-20 and P-mix-20, 4 each, descriptive. The reference of every contrast is the same note
  through the same strip with no record. The flip's change `d = y - x` is split at the measures'
  own edge (bass partials stop at 1 kHz, section 1's out-of-band measures read from 1.5 kHz) by a
  zero-phase Kaiser FIR, so each primary carries one cue; a length contrast cannot, because a
  longer flip has both a longer dip and a weaker click. Counts: 8 is the smallest count that allows
  one lapse at p <= 0.05 (7/8, p = 0.0352); power 0.94 / 0.81 / 0.50 for a listener who answers
  correctly on 95 / 90 / 80 % of trials (corrected 2026-10-05 from "hit rates"; see the verdict
  fold below).
- **Rule (block P, changes no value).** Each cue at the shipped flip is heard (detected), not
  heard (not detected, both P controls 2/2) or inconclusive. Outcome: not heard (all three not
  heard: answers 9.7 for that listener, chain and level), heard (as a dip, a click, both, or with
  the cue not separated), or inconclusive. A heard outcome is a finding for root; every outcome
  leaves the defaults unchanged, because any value change for polarity's sake moves `muteMs` off
  what blocks M and R decide on the mute itself, and the trade (a shorter flip clicks more, a longer
  one dips longer) is not measured. The result is carried to `2 x` the decided `muteMs` only where
  length decides it (40 ms: a heard dip stays heard, an unheard click stays unheard; 10 ms: the
  reverse); otherwise "not assessed". P controls gate only block P; the M, F and R controls gate
  only M, F and R (unchanged rules).
- Harness: `measure/src/split.rs` (new: the band split), `measure/src/stimuli.rs` (14 block-P
  conditions through `InputBuiltins::set_polarity_invert`; `render` split into render and edge
  fade with the same arithmetic; a comment corrected: the kick transitions fall 200 ms, not 150 ms,
  into a body), `measure/src/analysis.rs` (`Fft` crate-visible for a test), `measure/src/main.rs`,
  `measure/README.md`. The 37 earlier stimulus WAVs are byte-identical to `17f0bf18c`'s; the
  manifest gains 14 rows at its end. Levels: RMS differences 0.00-0.06 dB for every block-P contrast
  but the 200 ms dip control (0.63 dB); peaks at or below -1.894 dBFS.
- `listening/listening.py`: block P in the design, training (three files), `run --block P`,
  validate, scoring and `reveal` (`decisions.polarity`); `listening/README.md` for the new block.
- `FINDINGS.md`: section 6's and 9.6's "no contrast" statements now point to Amendment 1; 9.7's
  dip bullet and "Awaits #1388" name block P.

**New tests and their value.**

- `stimuli::tests::the_shipped_polarity_stimulus_flips_over_twice_the_mute_length` turns red if
  block P's stimulus is built from a mute record, at the mute's own length, or with a halved
  window; no test checks a stimulus builder today.
- `stimuli::tests::the_split_stimuli_hold_the_dip_and_the_click_apart` turns red if the split is
  wired to the wrong signal (the out-of-band stimulus without the note's band, a sign error on
  `d`), so a "dip only" stimulus would carry the click or a "click only" one the dip.
- `split::tests::the_split_passes_the_bass_band_and_stops_the_out_of_band_region` turns red on a
  wrong cutoff or a short kernel (passband within 2e-5 dB to 1 kHz, stopband below -115 dB from
  1.5 kHz; measured 1.12e-5 dB and -119.4 dB).
- `split::tests::the_low_part_is_centred_on_its_input` turns red if the split is applied causally,
  which would misalign the in-band change with the note.
- The listening self-test's block-P section turns red if P controls gate M, F and R (or the
  reverse), if block P is missing from prepare, validate or reveal, if the dip and click contrasts
  are swapped, if the carry to the decided flip runs the wrong way, if a non-detection is read
  without the P controls, or if a P result moves a value.

**Mutation evidence** (each applied, the tests run, then the file restored and checked by
SHA-256; harness `cargo test --release --offline -- polarity split low_part`, listening
`self-test`):

- Harness, red each time and only in the named test(s): H1 the stimulus built from a mute record
  (`Control::Mute`) and H2 the shipped flip at the mute's own length (480 samples) and H3 the
  records' window halved: `the_shipped_polarity_stimulus_flips_over_twice_the_mute_length`; H4 the
  split applied causally: `the_low_part_is_centred_on_its_input` and
  `the_split_stimuli_hold_the_dip_and_the_click_apart`; H5 the cutoff at the passband edge:
  `the_split_passes_the_bass_band_and_stops_the_out_of_band_region` and the split-stimuli test; H6
  the out-of-band stimulus as `y - L y` (no note band) and H7 a sign error on `d`: the split-stimuli
  test. Restored: 21 passed.
- Listening self-test, red each time: L1 P controls also gating M, F and R; L2 block P missing from
  `BLOCKS` (prepare refuses: "balanced schedule: P-bass-0-oob"); L3 the dip and click contrasts
  swapped; L4 the carry to a longer flip reversed; L5 a non-detection read as "not heard" without
  the P controls; L6 `reveal` dropping `decisions.polarity` (the record check); L7 a heard dip moving
  `muteMs`. Restored: passed.

**Gates** (on the final tree):

1. `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/tmp/claude-1002/r1388/target cargo test --release
   --offline` from `measure/`: 21 passed (17 earlier, 4 new); `cargo clippy --release --offline
   --all-targets` clean.
2. All 11 CSVs regenerated twice (`measure` and `live`, all four rates, into two scratch
   directories) with the final build: 22 of 22 files `cmp` identical to the committed `data/`.
3. `stimuli` run three times: 52 files identical across runs; the 37 earlier WAVs identical to
   those `17f0bf18c` renders (SHA-256), the manifest's first 38 lines unchanged.
4. `prepare --commit 8eacefa34` from those stimuli into a private scratch directory: 104 trials,
   nine training files; `validate`: packet valid (the private key was not opened).
5. `TMPDIR=/tmp/claude-1002/r1388/tmp python3 docs/handoffs/control-smoothing-defaults/listening/listening.py self-test`:
   passed (104 trials).
6. `bash scripts/check-workspace-policy.sh`: ok. `bash scripts/check-env-vocabulary.sh`: ok
   (59 names). `bash scripts/check-dsp-research.sh`: ok.

**For #1388's spec** (not edited here; root or the next worker applies it): the packet is now
104 trials in blocks M, F, R and P; block P changes no value and records a finding; its controls
gate only block P. See this worker's report for the exact text.
*Note, 2026-10-05:* this is no longer pending. `fa089b8cb` and `7d67810a6` applied it to #1388's
spec (Context, D1-D5, Deliverable 3, gates 2-3).

### Follow-up verdict fold (root rulings, 2026-10-05)

Verdict: `/home/bl/misofm/submix-verdicts/1055-followup-attempt1.md` (PASS; 4 MINOR, 8 NIT). Every
finding is folded in text only, in this entry's commit on `codex/d15-stream-e`; no harness code,
CSV or stimulus changed, and no listening response or result was written.

- MINOR-1: #1388 D5 says what each miss leaves standing. After a missed P control alone, the M,
  F and R decisions stand and D4 applies them; after a missed M, F or R control the defaults stay
  unless a rerun decides otherwise. The one fresh packet is allowed, not required; a rerun repeats
  all four blocks and the issue closes on the second record (root's ruling).
- MINOR-2: #1339 gate 8 and #1340 gate 7 gain a pass criterion quoted from `FINDINGS.md` 9.4
  (OOB is the mute reference's plus the step within -1.1 to +1.8 dB; no measure exceeds the mute
  reference's), a representative setting (delay: mix 0.5, feedback 0.5; multiband: 6.0 dB peak gain
  reduction per band), byte-identical existing rows of `bypass_crossfade.csv`, and a stop for root
  on failure.
- MINOR-3: Amendment 1's power sentence names the correct-answer rate, with a dated note in the
  amendment; the same wording is corrected above in this record.
- MINOR-4 and NIT-6: the successor draft (link glide) names per effect the material and setting on
  which the link change moves the gain, reports a row with too small a gain difference as
  vacuous, requires a latency-matched reference for the limiter, and fixes the `:365` anchor, gate
  2's naming and the redundant test-value clause. The draft is outside the repository (root files it).
- NIT-1 and NIT-5: Amendment 1's control line ("every M, F and R non-detection") and peak line
  (-1.894 dBFS) are exact.
- NIT-2: #1054 D3's rationale bullet is cut to the table note's values and a pointer to 9.2.
- NIT-3: `FINDINGS.md` 9.1 points to root's matrix ruling (#1054 D3, #1388 D4); 9.7 says 21 tests.
- NIT-4: #1388's new lines are rewrapped to 100 characters; D2's marker sits after its parenthesis.
- NIT-7: the stale "not edited here" line above carries a dated note naming the two commits.
- NIT-8: noted only. `split::tests::the_low_part_is_centred_on_its_input` overlaps
  `stimuli::tests::the_split_stimuli_hold_the_dip_and_the_click_apart` (every centring defect
  the verdict tried turns both red); it stays, because it points at the defect directly.
