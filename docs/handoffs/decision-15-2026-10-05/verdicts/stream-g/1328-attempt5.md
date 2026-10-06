PASS

# #1328 attempt 5 (the last attempt): adversarial verdict

- **Reviewed:** `git diff 5cfc1fb6d c58c4e55f` on `codex/d15-stream-g`. I exported the commit and its
  parent with `git archive` to `/tmp/claude-1002/v1328a5/` (`tree`, `parent`, two mutation copies
  and scratch copies). I built and ran nothing in `/home/bl/misofm/wt-d15-g`.
- **Host:** AMD EPYC 7313P, rustc 1.97.1, Node v22.23.2 (V8 12.4.254.21-node.56). The load average
  was 4-20 because other builds were running. All timings below are descriptive only.
- **Why it passes:**
  - The A9 law is implemented as ruled, at every SVF user and on every path.
  - The CSV re-pin is the genuine A9 output, not the path variant. That variant does not exist; see
    item 1.
  - The new gates are red on A8's law and on attempt 3's law. Every gate I ran is green.
  - Of my 35 mutations, 32 are red. Two are test gaps (MI, MK), which are MINOR because the code
    under them is correct. One is equivalent (MS2).
  - The GitHub body is in sync.
- **For the root's ruling:** the cost. The record's baseline is mislabelled. Against a per-word
  build, the A9 machinery costs +0.8 % to +5.5 % p50. Two cheaper exact encodings are available
  (m2).
- **Evidence:** `/tmp/claude-1002/v1328a5/evidence/` (small files only; the build directories are
  deleted).

## BLOCKER

None.

## MAJOR

None.

## MINOR

**m1. The CSV re-pin is genuine, but the #1427 standing order was not acknowledged. Also, #1427's premise is false.**

The re-pin is the genuine A9 output (item 1 has the evidence):
- `fixtures/builtins/v1/reference/filter-response.csv` equals the A9 law's output exactly.
- `MANIFEST.tsv` and `ACCEPTED_MANIFEST_SHA256` follow from it.

The procedural defect:
- #1427's Hazards forbid any `--write` re-pin of this file until #1427 lands.
- The attempt record does not mention that hazard or ask root.

The finding root needs:
- The "two checkout-path variants" are #1328 laws, not checkout paths:
  - `ece29117` is the pre-#1328 per-word law.
  - `d577dab4` is #1328 attempt 1-3's law. It equals the independent twin under that law on all
    1,630 rows.
- The #1408 bisect's "culprit" was almost certainly a stale build of the branch code.
- Release builds of both the parent and the commit, at two checkout paths 41 characters apart,
  each write byte-identical fixture trees.
- So #1427 should be re-examined (or rescoped) before anyone spends a two-fat-LTO nightly job on it.

**Correct committed content:** what c58c4e55f commits. No change is needed.

**m2. Cost (for the root's ruling): the record's p50 baseline is mislabelled. Against per-word, A9 costs +0.8 % to +5.5 %, and cheaper exact encodings exist.**

*The baseline is mislabelled.* The record's "A8" numbers (150,075 / 153,372 / 157,348 / 234,255 /
146,198 / 337,250 ns) match a per-word-law build to within 1 %, not the parent A8. I measured
(descriptive, one warmup and two rounds, interleaved, taskset cpu 31):

| document | real A8 (`5cfc1fb6d`) → A9 | main (`6d28a80ec`) → A9 | per-word P0 (parent with `flush_pair` = two `flush`) → A9 |
|---|---|---|---|
| mono quiet | −3.3 % / −2.8 % | +8.4 % / +3.2 % (noisy round) | +3.0 % / +2.7 % |
| mono restated | −2.9 % / −3.4 % | +8.5 % / +3.4 % | +2.3 % / +2.8 % |
| mono automated | −2.8 % / −2.8 % | +7.7 % / +3.3 % | +1.8 % / +2.7 % |
| 64-track console | −3.6 % / −2.3 % | +3.6 % / +4.6 % | +3.5 % / +0.8 % |
| app shape | −4.7 % / −4.8 % | +5.3 % / +7.8 % | +5.5 % / +5.1 % |
| bus-and-send | −1.6 % / +0.1 % | +2.9 % / +4.2 % | +4.0 % / +2.1 % |

- Every output digest is equal across the four modules.
- P0 equals main within noise, so #1407, #1408 and #1366 cost nothing here.
- The A9 machinery's own cost is the right-hand column. It is above the 2 % allowance on most
  documents. Against the real A8, A9 is faster.

*Where the cost comes from (codegen).*

| loop | per-word P0 | A8 | A9 |
|---|---:|---:|---:|
| V8 EQ held loops: mono tail / mono pair / masked pair / dual tail | 52 / 82 / 87 / 109 | 63 / 99 / 109 / 132 | 58 / 95 / 100 / 127 |
| V8 builtins dual stationary loop | 196 | 236 | 219 unarmed, 239 armed |
| V8 builtins mono loop | 95 | 113 | 94 unarmed, 113 armed |

- **EQ.**
  - The V8 EQ loops are 12 % to 17 % larger than per-word.
  - On live audio the EQ still pays `flush_pair`: 10 lane-ops against 6, plus a threshold load per
    stream-frame (one per section in the skewed pairs). The EQ has no unarmed form.
  - AVX2 `svf_block`: 33 instructions per frame (A8 37).
- **Builtins.**
  - The V8 dual loop is +23 instructions per frame (+12 %) over per-word, with the same arithmetic.
    The cause is the constants carried as words (`InputChainConstants`, the iOS memset ratchet) and
    16 carried stack slots.
  - The mono loop is unchanged.
  - AVX2: 149 instructions per frame unarmed, 188 armed (A8 183).
- **Counter.**
  - `silence_block` is 6 instructions per frame (`vpcmpgtd`+`vpand` for the select).
  - `silence_skip_block`'s live early-out is one `vcmpeqps` and one `vtestps` per block.
- **Multiband.** It runs the counter (5) and two joint flushes (+4 each) on every frame. It is not
  in the browser documents.

*A cost the record does not state: any silent or padding lane puts the whole bank on the armed path.*
- `silence_armable` is a `mask_any` over all lanes. A padding lane (fed `+0.0`) or any track silent
  for 85 ms makes every block armable for the whole bank, for as long as it stays silent:
  - the builtins run the armed body (82 lane-ops, not 69);
  - the EQ writes a rest plane per channel.
- Measured natively: an eight-lane builtin bank with one silent lane is **+15 % to +18 %** per block
  (`evidence/src/padcost.rs`: 1,950 → 2,280 ns per block).
- The floor-doc row "every block of live audio ... costs nothing per lane-sample" is true only for
  banks with no silent lane.

*Cheaper exact encodings for the root to consider. Neither changes a bit.*
- **(a) An EQ unarmed form, as the builtins have.** On a block where `silence_armable` is false,
  run `svf_step_when(false, …)` and read no plane. This returns the live EQ loops to per-word size
  and removes most of the measured cost.
- **(b) An armability refinement.** A block is armable only if some lane that can arm also holds a
  non-zero state word in that effect channel.
  - It is exact. A lane whose state is exactly `+0.0` at block start sees only zero input up to any
    armed frame, because `N_SILENCE` is larger than the block. Its state therefore stays `+0.0`
    through every armed frame, and the armed and unarmed bodies give the same bits for it. After a
    non-zero sample it cannot re-arm within the block.
  - It removes the permanent armed path of padding and silent lanes. The check costs a few
    compares per block.
- **(c) The builtins' constants-as-words.** They cost about 12 % of the V8 dual loop. The
  implementer measured splatted constants 1 to 4 points cheaper. This trades against the iOS
  memset ratchet (#1018) and is root's trade.

**m3. Two plausible defects are unguarded (the code is correct).**
- **MK: the skewed cascade reads frame `i`'s threshold instead of frame `i − k`'s**
  (`crates/lane/src/kernels.rs:511-520`). No test catches it; every test is green.
  - The rewritten `g2_skewed_cascade_equals_the_interleaved_cascade` (doc at
    `crates/lane/tests/g2_kernel_identity.rs:1062-1064`) claims to catch exactly this. It does not:
    its hostile signals never put a section-2 state in the joint band at an arming frame.
  - It does not even catch "the skewed cascade never arms" (MK2). Only the EQ fixed-point test does.
  - My scratch differential (`evidence/src/skewcheck.rs`) proves the mutant is a real defect.
    Seeding both sections at (`5e-15`, `−4e-15`) and arming mid-block gives skewed ≠ interleaved at
    4 of 5 offsets, and equal on the real code.
  - The EQ runs every depth-2 pair through this kernel, dual and mono.
  - Fix: add that seeded case to the skew gate.
- **MI: the builtins dual bodies decide armability from channel 0's counter only**
  (`crates/lane/src/kernels/builtins.rs:638-640`, `:796-798`, `:1021-1023`). It stays green.
  - All rest and carry tests drive L = R.
  - Fix: add a case where only the right channel is silent and its state is in the band.

**m4. G5 no longer reaches the joint flush on wasm.**
- The six `svf_block` cases are back to their pre-#1328 pins. This is correct for their 1,024-frame
  fresh-counter blocks, but no G5 case (lane or delegated) now arms the rule.
- So the wasm leg executes `max_u32`, the armed compare and the counter's armed path in no
  discriminating way. With an unarmed threshold the joint test is false whatever `max_u32` returns.
  Under A3 and A8, G5 covered the joint flush.
- g1 and g4 hold these ops at the native widths only.
- Add one G5 case with a pre-armed counter whose impulse tail crosses the joint band.

**m5. Statements that are not accurate.**
- **Decision 15 #1328 entry and `docs/BUILTINS_AND_METERING_V1.md`:** "a block of live audio pays
  nothing for the rule in the builtin chain and the EQ" is false for the EQ, which pays 4 lane-ops
  per section on live audio (m2). It is also false for any bank with a silent lane.
- **`dsp-research/filters.md:19`:** "ignoring `f32` rounding, which moves no state above
  `FLUSH_EPS`-scale" is false.
  - After a flushed silence, later loud audio can differ by about one ulp of the signal. I found
    `9.3e-10` at a relative `1.6e-7` with the final window, and `3.7e-9` at 2,048 frames.
  - The record's "up to `4.66e-10`" for rounding-scale changes should therefore be stated as
    relative (about 2^-23 of the signal), not absolute.
- **The 1,024-frame residual table** (spec, `filters.md`) is a search maximum. My independent f32
  simulation found more at 88.2 and 96 kHz: `1.53e-9` and `1.68e-9` (record: `1.31e-9`, `1.33e-9`).
  The conclusion holds either way.

**m6. The EQ's three heap rest planes are undeclared scratch.**
- The EQ allocates `3·quantum·W·4` bytes per prepared EQ (1,536 B scalar, 12,288 B at W8, quantum
  128) at preparation (`crates/parametric-eq/src/lib.rs:3446-3450`).
- The descriptor still declares `scratch_fixed_bytes: 0` and `scratch_bytes_per_frame: 0` (`:667`).
  The effect-contract definition calls an effect that uses more than its ceiling non-conforming.
- `declared_effect_bytes` counts only the +8-byte payload growth.
- The fix is a descriptor change (declare it) or a stated ruling that these are not scratch.

**m7. The section-input term (for the root).**
- Dropping it is justified as recorded. My simulation found the same worst change, at the same
  position, with and without it, at 1,024 frames and at the final window, at every rate.
- But it loosens the provable one-tail chain bound from `3.74e-10` (each section flushes once) to
  `3.1e-7` (−130 dBFS, above a 24-bit LSB), for 2 lane-ops per section.
- The ruling allowed either choice.

## NIT

- **n1.** The multiband refuses a malformed counter word with `effect.state.filter`
  (`crates/multiband-compressor/src/lib.rs:1451`); a distinct code would be clearer.
- **n2.** `lane::silence_frames` (a `u64` `div_ceil`) is evaluated once per block in the EQ and
  multiband render (`silence_window`, `process_block`). Precompute it at preparation.
- **n3.** `select(eq, run + 1, 0)` lowers to `vpcmpgtd` + `vpand` on AVX2. A plain `and` with the
  compare mask would save one op per frame.
- **n4.** The V8 held-loop counts in my run were 95 and 100, against the record's 97 and 101. This
  is descriptive only.
- **n5.** MS2 (the EQ discontinuity reset keeps the counter) survives, but it is equivalent for
  rendering: the integrators are `+0.0` at the reset, so no armed frame can move a bit before the
  next non-zero sample. The multiband twin is red through conformance.

## Judgments on the points asked

1. **The CSV is a genuine A9 bit move, not the path variant.**
   - I wrote a separate program (`evidence/src/lawdft.rs`) that runs the scalar twin
     (`ReferenceRetainedTptF32`) through each of the 1,630 rows' one-second impulse under each law.
     It takes the same DFT.
   - The committed new CSV equals twin-A9 on 1,630/1,630 rows. The parent's pin (`ece29117`) equals
     twin-per-word on 1,630/1,630. The parent's own build writes twin-A8 (1,630/1,630).
   - The reported maximum, `3.209e-5` dB at `response-low_pass-96000-*-1-5`, is per-word → A9:
     `−186.77327288007166` → `−186.77324078548963`. A9 ends that tail at sample 27,377; per-word
     rings to 39,957.
   - Release `audit` of c58c4e55f, built at `/tmp/…/pa/src` and at
     `/tmp/…/path-b-a-much-longer-checkout-directory-name/src`, writes the identical whole fixture
     tree. That tree equals the committed one, `MANIFEST.tsv` (`a10fdfef…`) included.
   - The parent at both paths also writes identical trees. Runtime environment and output path do
     not change it.
   - So the MANIFEST and `ACCEPTED_MANIFEST_SHA256` re-pins are exact consequences.
   - `resources.jsonl` (+24 bytes per input stage: +48 per two-track row, `maximum_single_allocation`
     1,072 → 1,096) is a size change, regenerated identically.
   - `direct-route.resources.json` (+224) passes `graph_fixture --check`.
   - The `track_delay` digest's +72 (9 EQs × 8 bytes of payload) follows from `STATE_LANE_WORDS` + 1
     per channel, and the test is green.
   - G5: `lane_digests.in` is byte-identical to `1975fc44f`.
2. **The law against A9.**
   - **Counters.** There is one counter word per effect input and channel: builtins
     `InputChainState::silence[2]`, EQ `Channel::silence`, multiband `Side::silence`. Every SVF user
     (builtins, EQ, LR4) reads its own effect input's counter. The builtins count the raw sample
     before sanitising and trim; NaN, infinity and subnormal samples reset the counter.
   - **Saturation.** `2^24 + 1` rounds to `2^24`; g4 shows this and MZ (`silence_advance` without
     the clamp) is red.
   - **The section-input term:** see m7.
   - **The #1407 collapsed-stage invariant.** The mono bodies touch `silence[0]` only. The disengage
     copy and `channels_agree` carry it (MO red). A carry disengages first.
   - **Swap, restore and reset.** These go through the payloads and the carry (MQ, MR, MT red). The
     reset clears the counter (MS2 is equivalent; MT2 is red).
   - **Bypass and D7.** Bypass freezes the counter with the state, which is consistent. D7 recovery
     leaves it, as the counter counts the input.
   - **Elision and skipping cannot desynchronise it:**
     - the EQ plane is written before any section, whatever the plan elides;
     - the silent fast path uses an exact `silence_advance` (MN red);
     - the all-identity builtin bodies use `silence_skip_block` (MG, MH, MH2 red);
     - rack slots with no active lanes are a static prepared mask.
3. **N_SILENCE.** My independent f32 simulation (`evidence/src/sim1328.rs`, `sim-window2.txt`)
   searched four and one shelves, 10 and 100 Hz, every rate, continuous and sparse inputs including
   periods N, N+1, N+2, N+17, 2N and 4N.
   - At 1,024 frames, four 10 Hz shelves leave `6.2e-10`, `7.0e-10`, `1.53e-9` and `1.68e-9` at the
     four rates. All exceed A8's tail level of about `3.4e-10`, so A9's rule requires a time.
   - At 85⅓ ms they leave `3.05e-10`, `3.26e-10`, `3.40e-10` and `3.26e-10` (four 100 Hz:
     ≤ `3.10e-10`; one shelf: ≤ `1.90e-13`). That is one tail.
   - The choice satisfies the rule.
4. **Gates.**
   - The sparse gate is red on the A8 law ("misses … by 1.0375549e3 … peak 1.102915e3") and on the
     A3 law. The stale-plane mutant (ML) is also red.
   - The cancellation gate is red on A8 (`6.996817e-5` against a `7.8e-6` tolerance).
   - The multiband gates are red on MT, MU and MV. Gates 1 and 2 are green.
   - **m2 bounds re-derived** (`evidence/bounds.py`, f64 from the cast words):
     - one section: `1.9527e-13` at S 1 (10 and 100 Hz) and `3.37e-13` at S 0.1, every rate;
     - first-section share: `3.479e-10`;
     - chain: `3.104e-7` and `3.136e-8` at 96 kHz, `1.428e-7` and `1.459e-8` at 44.1 kHz;
     - any input: `4.997e-6`.
     All match the record.
5. **Subnormals.** The argument holds. The per-word law runs on every word every sample, so every
   state word is `+0.0` or at least `1e-20`, and the smallest product (about `1e-7 · 1e-20`) is
   normal.
6. **`max_u32`.**
   - It is NaN-exact by construction.
   - g1 tests every ordered pair of the edge pool plus the NaN, infinity and zero patterns, lane by
     lane, at every native width. MY, MY2 and MX are red.
   - `flush_pair` is 10 operations; the AVX2 listing shows exactly 10.
   - See m4 for wasm.
7. **Spill gate: a correction, not a weakening.**
   - The held rows' "no carried slot" rule is unchanged. Only the anchor search now accepts a pair of
     the right shape whether it is select-free or masked, and both pair kernels are stationary
     passes.
   - Held rows are clean (127 / 95 / 58 / 100). The dual pair reads masked (reported, not held).
   - The self-test mutation is red: anchoring on select-free pairs only gives
     `[1,0,1,0,0,1,1,1,0,1]`.
8. **Cost:** m2.
9. **Floors.**
   - `flush_pair` 10, `svf_step` 23, section 28 / 29, EQ 31.
   - Builtins: 7 + 2·24 + 14 = 69 live; 7 + 2·28 + 5 + 14 = 82 armable.
   - Strip: 69 + 31 + 81.5 + 129.5 = 311.
   - `floor.rs`, the jq file and the test script agree.
   - n3 is documented accurately.
10. **Gates:** all green (table below).
11. **GitHub:** the #1328 body equals the spec at c58c4e55f (99,974 bytes; jq adds only a trailing
    newline). The issue is OPEN.

## Test value (each new or rewritten test)

- **`g1_max_u32_is_the_unsigned_maximum_of_the_bits`:** a `max_u32` built from the float `max` or
  `select(gt)` (drops a NaN, misorders −0), or one that broadcasts a lane, is red (MY, MY2).
- **`g4_silence_counter_law_at_every_width`:** a counter that does not reset on a live, subnormal,
  NaN or infinity sample, that arms one frame early or late, or that does not saturate, is red (MC,
  MD, ME).
- **`g4_silence_armable_is_exact`:** a block armability test one frame late skips the arming frame
  (MF).
- **`g4_silence_advance…`:** an advance that does not saturate at `2^24` (MZ).
- **`g4_silence_frames…`:** a wrong rounding or a frame count instead of a time.
- **g4 pair law (rewritten):** a joint test built from `Lane::max`, or OR instead of AND (MX).
- **`g2_svf_step_yields_both_taps_of_one_state` (rewritten):** a rule armed on a zero run shorter
  than the window, or on any zero (A8) or any input (A3) (MA, MB, MD).
- **`g2_skewed_cascade…` (rewritten):** catches skew/interleave schedule drift as before. Its A9
  claim is false (m3).
- **EQ sparse gate:** an armed rule that erases a sparse signal's state (A8, A3), or an unarmed
  block that reads a stale armed plane (ML).
- **EQ cancellation gate:** a rule armed by a section's own exact-zero input (A8).
- **EQ fixed-point test:** a silent fast path earned before arming (MM), a skipped block that does
  not advance the counter (MN), or a skewed pass that never arms (MK2).
- **EQ carry restore / malformed word:** a payload that drops or mis-validates the counter (MR, MZ2).
- **EQ mono collapse:** a disengage copy without the counter (MS).
- **Builtins carry / disengage:** an import or disengage copy that drops the counter (MQ, MO).
- **Multiband: carried and validated / own input / crossover arms:** a dropped counter (MT), a
  channel counting the other's input (MU), a second stage never armed (MV), or arming late or early
  (MD, ME).
- **`input_chain_elision` (rewritten):** a body that skips the counter, or a skip path that is
  inverted, does not reset or reads the wrong frame (MC, MG, MH, MH2).
- **Builtins `stage` twin:** a twin always armed (MW).
- **Spill self-test case:** an anchor tied to the select-free classification.
- **Audit `unfused-fma` silence pass (n1):** a model armed always, armed on `v0 == 0` or never
  armed is red (exit 134 ×3).

## Gates run (export at c58c4e55f)

| gate | result |
|---|---|
| gate-4 debug `cargo test --all-targets`, 14 packages, spec features | 853 passed, 0 failed |
| other crates (engine, session, source, effect-contract, effect-compiler, rack, rack-compiler, builtins-compiler, graph, graph-compiler, host-core, protocol, capi, host-web, bench-support, parameter-metadata, session-validator, wasm-gate-corpus) | 1,414 passed, 0 failed |
| release `-p lane -p math -p wasm-gates`; release `-p audit -p bench -p console-workload` tests | ok |
| `run-wasm-gates.sh` | exit 0; held rows clean; shipped module `0e02a3c8…` |
| worklet chain: build `--named-twin`, `check-web-audioworklet.sh --without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts`, `check-scalar-oracle-absent.py --wasm`, `test-web-audioworklet.sh`, spill self-test (28) | all ok |
| `check-cross-targets.sh` | PASS; builtins 71, parametric-eq 88, multiband 566 |
| `conformance_fixtures --check`, `check-builtins-fixtures.sh` (50 files), `graph_fixture --check`, `check-graph-determinism.sh` (100/100) | ok |
| `audit unfused-fma conformance` | unfused, 0 mismatches, silence pass included |
| `audit parametric-eq` / `compressor` (100,000 blocks), builtins and builtins-graph traces, effect-contract trace (1M) and conformance | 0 allocations, 0 syscalls, ok |
| `cargo test -p bench floor`, `test-console-benchmark.sh` | ok |
| `check-lane-policy.sh`, `check-dsp-research.sh`, `check-workspace-policy.sh`, `cargo fmt --check`, `cargo clippy --workspace --all-targets -D warnings` | ok |

**Mutations.** Each was applied in an export copy, run against the five DSP crates' tests and then
restored. Full list: `evidence/mut-summary.txt`.
- **Red:** MA (A8 law), MB (A3 law), MC, MD, ME, MF, MG, MH, MH2, MJ (conformance), MK2, ML, MM, MN,
  MO, MQ, MR, MS, MT, MT2, MU, MV, MW, MX, MY, MY2, MZ, MZ2, the three audit-model mutants and the
  spill anchor.
- **Green:** MI and MK (m3), and MS2 (equivalent, n5).
