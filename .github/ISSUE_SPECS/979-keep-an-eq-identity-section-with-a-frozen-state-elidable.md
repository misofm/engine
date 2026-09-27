# Keep an EQ identity section with a frozen state elidable


EQ optimisation, slice 4 (research 2026-09-27, base `6ca203f8`). Evidence:
`docs/handoffs/effects-2026-09-27/EQ-DIAGNOSIS.md`, section 5 (EQ-4); the measurement is
`docs/handoffs/effects-2026-09-27/eq-disable-cliff-test.patch`.

## Product outcome

Elision leg (b) requires every elided identity section to hold exactly `+0.0` in both integrators
(`section_state_is_positive_zero`, `crates/parametric-eq/src/lib.rs:975`, used at `:1715-1716`
and in `cascade_sections_mono` at `:1530`). An identity section never moves its state: `c1 = a2 = a3 = +0.0`
make every increment a signed zero, so whatever state the section held when it became the identity
stays there. That happens every time a dedicated cut is switched on and then off through a prepared
target (#807): the ramp ends on the identity words with a non-zero state. From then on leg (b)
refuses the **whole bank** on every block, and the bank renders all six sections until a reset.

Measured with the patch above (`Simd8`, a band switched on at block 2 and off at block 5): the
shipped rule elides on **0 of the 9** blocks after the disable; the relaxed rule below elides on
**9 of 9**, and its output words and integrator words equal the full cascade's. An affected bank
costs about 14,600 more cycles per block natively (six sections instead of one; +3.9 us per 8
tracks) and about 18,200 more in the browser (+4.9 us per 4 tracks). A restored payload with a
non-zero state in a disabled section has the same effect.

## Why the relaxed leg is exact

Take an identity section (exact identity words, no ramp) whose every integrator word is `+0.0` or
finite with magnitude at least `FLUSH_EPS`, on an admitted block (leg (a): finite, within
`BLOCK_LIMIT`, no `-0.0`).

* `v3 = v0 - ic2` is finite (`|v0| <= 1e30` cannot push a finite `ic2` past `f32::MAX`).
* `d1 = (-0.0 * ic1) + (+0.0 * v3)` and `d2 = (+0.0 * v3) + (+0.0 * ic1)` are zeros of some sign.
* `ic1' = flush(ic1 + (d1 + d1))`: a non-zero `ic1` absorbs the zero and `flush` keeps it
  (`|ic1| >= FLUSH_EPS`); `ic1 = +0.0` gives `+0.0`. Likewise `ic2`. **The state is unchanged.**
* `y = (+0.0 * v2) + ((+0.0 * v1) + 1.0 * v0)`: `v0` for every finite `v0 != 0`, and `+0.0` for
  `v0 = +0.0` whatever the signs of the zero terms. **The output is the input.**
* A dedicated cut additionally selects `v0` through its dry mask, which gives the same bits.

So executing the section and eliding it leave the same output and the same state, and the section
passes no `-0.0` on, which is what legs (a) and (c) rely on downstream. A word with magnitude below
`FLUSH_EPS`, a `-0.0`, or a non-finite word is still refused: the executed recurrence would flush
or propagate it, and the elided one would not.

## Lessons carried from #944

Dev and release; NaN words compared as "both NaN, or equal bits"; scenario test pinned on base
first; mutations recorded red in `crates/parametric-eq/tests/MUTATIONS.md`.

## Invariants

- **Class A.** Every rendered word and every integrator word is unchanged on every block.
- Legs (a) and (c) are unchanged. Only leg (b)'s predicate widens.
- `a_tiny_restored_disabled_cut_state_refuses_elision_but_preserves_old_bands` (`lib.rs:3663`) stays
  green unchanged: a restored word below `FLUSH_EPS` still refuses.
- Allocation-free, no new field.

## Interface contract

1. Replace `section_state_is_positive_zero` with `section_state_is_inert<L: Lane>(section) -> bool`:
   true when every lane of `ic1` and `ic2` has bits `0`, or magnitude bits in
   `[FLUSH_EPS.to_bits(), 0x7f80_0000)`. Read the bits with `store_bits`, as the current helpers do.
2. `cascade_sections` and `cascade_sections_mono` call it for dead sections.
3. Amend the proof in `cascade_sections`' doc comment (`:1599-1682`): the induction starts from an
   inert state, not from `+0.0`, with the argument above.

## Smallest closable slice

Authorized paths: `crates/parametric-eq/src/lib.rs` (the helper, the two call sites, the doc, the
`elision` test module); `crates/parametric-eq/tests/bank.rs` (gate 1);
`crates/parametric-eq/tests/MUTATIONS.md`; this spec.

## Non-goals

Clearing a disabled section's state (that moves bits for `-0.0` input), caching the legs per block
(EQ-7), any kernel change.

## Objective gates

1. **Scenario (public API, pinned on base).** `a_cut_switched_off_keeps_the_bank_eliding` in
   `tests/bank.rs`: `native_bank()` and scalar; enable the HPF on every lane with a prepared target,
   render 4 blocks, disable it (a target with `enabled = 0`), render 12 more blocks of hostile input
   (subnormals, `+0.0`, `2^-24..2^25`; two blocks with `-0.0`). SHA-256 of outputs and snapshots,
   pinned on base.
2. **Engagement (in-crate).** Port the patch's `diag_disable_cliff` as a test: a general band and,
   separately, the HPF switched on and off on every lane of both channels; assert the stationary
   blocks after the ramp all elide (`kept < EQ_SECTION_COUNT`) and that the rendered words and
   integrators equal the per-section path's, at `f32`, `Simd4` and `Simd8`.
3. **Refusals (in-crate).** A dead section holding a `-0.0` word, a word of `1e-30`, or (through
   `restore_track`) the largest finite word: `-0.0` and `1e-30` refuse; the largest finite word is
   admitted and stays bit-exact against the per-section path.
4. `cargo test -p parametric-eq`, dev and release; `chain_shape` and every `WORKLOADS` digest
   unchanged (the standing fixtures never disable a section mid-run).
5. **Mutations**, each alone, red:
   - M1: back to the exact `+0.0` test: gate 2's engagement assertion.
   - M2: accept magnitudes below `FLUSH_EPS`: gate 3 (`1e-30`: integrators differ).
   - M3: accept `-0.0`: gate 3 (integrators differ: the executed path flushes it to `+0.0`).
6. Toolchain and policy scripts as EQ-1 gate 7.

## Console benchmark rows

No standing row switches a cut mid-run: expect no move. A row that toggles a cut is a separate
tooling issue.

## Dependencies

None. EQ-7 (caching the legs) lands after this one.

## Standing rules for the implementer

As EQ-1.

## What the implementer will hit

- **The identity flag is per section across all lanes of both channels.** The cliff needs the
  section dead on every lane; a cut left live on one lane keeps the section live and never reached
  leg (b). The gates switch it off on every lane.
- **The ramp ends inside `process_section`**, which snaps the words and refreshes `identity`; the
  first stationary block after it is where leg (b) is asked.
- **The old predicate has a test caller** (`lib.rs:3396`, which asserts a state is exactly `+0.0`).
  Keep that assertion's meaning: move the exact-zero helper into the test module rather than
  pointing the test at the widened predicate.


## Attempt 1 evidence

Implementer: attempt 1, 2026-09-27, branch `codex/977-eq-elision-and-passes`, on #978 (`100469fa`;
#976, #980 and #977 below it). The verification amendment on the GitHub issue is applied: the
**capped rule** (magnitude bits in `[FLUSH_EPS.to_bits(), ELISION_MAGNITUDE_CEILING]`), gate 3
refuses the largest finite word, mutation M4 (accept above the cap) and the restored
`ic2 = -f32::MAX` scenario are added, and `a_non_zero_state_in_a_dead_section_refuses_elision` is
listed as changed. Host: AMD EPYC 7313P (Zen 3), `rustc 1.97.1`, `x86-64-v3`, every build
`CARGO_INCREMENTAL=0`.

### The change

- `section_state_is_positive_zero` is replaced by `section_state_is_inert<L>(section)`, through
  `lane_is_inert`: every lane of `ic1` and `ic2` read with `store_bits` is `0`, or has magnitude bits
  in `[INERT_MAGNITUDE_FLOOR, ELISION_MAGNITUDE_CEILING]` (`INERT_MAGNITUDE_FLOOR =
  lane::FLUSH_EPS.to_bits()`, `0x1e3ce508`; the ceiling is `BLOCK_LIMIT`'s, `0x7149f2ca`). `-0.0`
  (magnitude 0), every magnitude below `FLUSH_EPS`, every magnitude above `1e30`, and every infinity
  and NaN refuse.
- `cascade_sections` and `cascade_sections_mono` call it for dead sections (leg (b)); legs (a) and
  (c) are unchanged.
- `cascade_sections`' proof now starts from an inert state: `v3 = v0 - ic2` stays finite for every
  finite `v0` because `|ic2| <= 1e30 < 2^103`; `d1`, `d2` are zeros; `v1`, `v2` finite; `flush` keeps
  `ic1`, `ic2` exactly; `y = v0` except `-0.0`. A closing paragraph says why each bound is
  load-bearing (the floor and `-0.0`: the executed section flushes them; the cap: VERIFY-EQ finding
  1's `ic2 = -f32::MAX` behind a +24 dB bell).
- `lane_is_positive_zero` is no longer imported by the library; the exact-`+0.0` helper moved into
  the `interleave_identity` test module, where its one caller keeps its meaning.

### Gate 1: `a_cut_switched_off_keeps_the_bank_eliding` (`tests/bank.rs`)

Eight tracks. Two shapes: the HPF alone, and the HPF beside a live bell; the HPF switched on on
every lane of both channels by prepared target at block 0, four blocks, a target with `enabled = 0`,
twelve more blocks of hostile input (`+0.0`, subnormals of either sign, normals `2^-24..2^26`;
blocks 7 and 12 carry one `-0.0`). Then VERIFY-EQ finding 1's scenario: a +24 dB bell at 12 kHz, band 2
disabled with `ic2 = -f32::MAX` restored (scalar: track 0; bank: lane 0 of every bank), a 10 Hz LPF,
eight blocks of a `9e29` sine at 12 kHz. Scalar, bank and bank-mono legs; every output word, report
and state payload after every block. Pinned on the unmodified base (`100469fa`), identical in dev
and release: scalar `a34ce0342d321d08dae9a99fa1adc175a2511041035a27cd5b744cff89b1b5f4`, bank
`2e0845c6619db72d76cfd1d96b6ab89414a923fed6dabbb65dab4ccadcad1214`, bank-mono
`26a755c16fc0efd143ff5ac79115aa835149a0336a6cc94c01cea91947e30cd2`; the overflow leg faults its
first block once per leg (asserted). After the change: the same three digests and the same fault,
dev and release, with and without `test-support`. The rule as briefed (M4) moves all three and
renders that block as audio.

### Gate 2: engagement, `elision::a_band_switched_off_keeps_the_bank_eliding`

The diagnosis patch's `diag_disable_cliff`, ported: beside one live general band, a general band
(section 3) and, separately, the HPF (section 0) switched on at step 2 and off at step 5 through
`start_ramp` on every lane of both channels, sixteen 512-frame blocks, at `f32`, `Simd4` and
`Simd8`. It asserts the switched-off section is the identity again and holds a frozen non-zero
state, that every stationary block after the ramp elides, and that every block's words and the
final integrators equal the per-section path's. Blocks eliding after the disable (steps 7-15):

| rule | Scalar s3 | Scalar HPF | Simd4 s3 | Simd4 HPF | Simd8 s3 | Simd8 HPF |
|---|---:|---:|---:|---:|---:|---:|
| shipped (`+0.0` only), dev and release | 0 of 9 | 0 of 9 | 0 of 9 | 0 of 9 | 0 of 9 | 0 of 9 |
| capped inert rule, dev and release | 9 of 9 | 9 of 9 | 9 of 9 | 9 of 9 | 9 of 9 | 9 of 9 |

(and the first stationary block after the ramp, step 6, elides as well). The shipped-rule row was
taken on the base with the test's count printed instead of asserted.

### Gate 3: refusals, `elision::a_non_inert_state_in_a_dead_section_refuses_elision`

Formerly `a_non_zero_state_in_a_dead_section_refuses_elision` (which refused `1.0` as well; it is
the test the verification listed as changing). Words restored through `Channel::restore_track`
into lane 5 of dead section 3, in either integrator. Refused (`kept == 6`): `-0.0`, `+-1e-30`, the
word under the floor, the word over the ceiling at either sign, and `+-f32::MAX` (the largest finite
word refuses, per the amendment). Admitted (`kept == 1`): `+-1.0`, `+-FLUSH_EPS`, `+-1e30`. Every
case also renders the per-section path's words and integrators, checked before the refusal
assertion. `a_tiny_restored_disabled_cut_state_refuses_elision_but_preserves_old_bands` is
unchanged and green.

### Gate 4: suite and rows

`cargo test -p parametric-eq`: 109 passed, 3 ignored, dev and release, with and without
`test-support` (every earlier pin of #976, #977 and #978 included). `chain_shape` (release): 23
passed. Scratch digests (as in #980's record): 90 native lines and 30 wasm guest digests identical
to the base, each row's native and wasm digests equal. The standing fixtures never switch a section
mid-run.

### Gate 5: mutations

Recorded in `crates/parametric-eq/tests/MUTATIONS.md` ("Issue #979"), release,
`--features test-support --lib --test bank`: M1 (exact `+0.0`: gate 2, 0 of 9), M2 (below
`FLUSH_EPS`: gate 3, `1e-30` integrators differ), M3 (`-0.0`: gate 3, integrators differ), M4 (above
the cap: gate 3 admits `1.0000001e30`; gate 1 scalar `c060ba98…`, bank `b0063058…`, bank-mono
`7698bb8b…`, no fault reported) -- all red.

### Gate 6: toolchain

fmt, clippy (`--workspace --all-targets --all-features -D warnings`), doc (`-D warnings`), `-p lane`
dev and release (69), `-p effect-runtime` (86), `-p console-workload` (39), `-p builtins-compiler
--features test-support` (79), `-p wasm-gates` (9), `-p bench floor` (9), lane policy, realtime
policy (57 regions), EQ render contract, console benchmark validators: green. AudioWorklet artifact
(build script's cargo line, not repinned) `0db9b2f5…`: all four callgraph checks give the lines #978
gave (render closure=8 traps=5; kernels=14; EQ dual 672 / collapsed 336, scalar 0).

### Descriptive A/B (not a gate)

No standing or scratch row switches a cut mid-run, so this change is not expected to move one; the
cliff it removes is shown by gate 2 (the brief's replica: about 14,600 cycles per affected bank-block
natively). One hold of #978 against this change (load 8-9) moved the isolates by 0.1-0.5 us either
way, which is noise.

**Combined A/B of #980, #977, #978 and #979** (`1d8c4851` against this commit's tree): clean scratch
builds, each verified by its artifact (base: no `i32x4.min_u` in the EQ `process_bank`, 312 EQ
ops; final: 672 and the inert-floor constants); two holds of `flock … timing.lock`, `taskset -c 31`,
arms in both orders, load average 5-6; the native harness alternated three times (six rounds of
1,500 blocks per row at `Simd8`), then both guests in one Node process (ten interleaved rounds of
1,000 blocks). Isolates (row minus `builtins_only`, or minus the mono rack-free row), median of the
per-round p50s, us, hold 1 / hold 2:

| row | native base | native final | wasm base | wasm final |
|---|---:|---:|---:|---:|
| `eq_only` (standing, one band) | 9.34 / 9.55 | 9.41 / 9.58 | 20.34 / 20.92 | 20.80 / 20.42 |
| two-band `eq_only` (scratch) | 15.44 / 15.01 | 14.15 / 13.74 | 43.65 / 43.73 | 30.94 / 31.29 |
| EQ-only mono, one band (scratch) | 7.55 / 7.51 | 7.48 / 7.52 | 15.84 / 15.98 | 16.23 / 16.26 |
| EQ-only mono, two bands (scratch) | 9.02 / 9.10 | 8.83 / 8.84 | 23.34 / 24.05 | 19.76 / 19.65 |

The standing one-band rows do not move beyond noise (their pass was already select-free and depth
1 after #976; #980's gate saving is below this noise). A session with two live bands per track gains
about 1.3 us natively and 12.5 us (29 %) in V8 per 64-track block; its EQ-only mono form about 0.2
us natively and 3.6-4.4 us in V8. No projected saving is claimed; the paired console benchmark is
the batch boundary's.

### Deviations and notes for the verifier

1. `a_non_zero_state_in_a_dead_section_refuses_elision` is renamed
   `a_non_inert_state_in_a_dead_section_refuses_elision` and rewritten to the capped rule (gate 3);
   its old `1.0` case is now admitted and asserted bit-exact.
2. Gate 3 restores through `Channel::restore_track` (in-crate), as the brief says; the public
   restore path is gate 1's overflow scenario.
3. The combined A/B's scratch rows (two bands, EQ-only mono) are not committed.

## Sol attempt 1 verdict: PASS

Verifier: Sol, 2026-09-27, on `04e439db`. Host, toolchain and artifact checks are as in #977's
verdict. This commit's AudioWorklet artifact is `0db9b2f5…`, as recorded.

**The capped rule is exact (class A).** Take an identity section whose integrators are `+0.0` or
have magnitude bits in `[FLUSH_EPS, 1e30]`, and any finite `v0`, whichever section feeds it:

- **`v3` stays finite.** `|v0 - ic2| <= f32::MAX + 1e30 < f32::MAX + 2^103`, so it cannot round to
  an infinity.
- **The state does not move.** Both products in `d1` and `d2` are zeros, so `v1 = ic1` and
  `v2 = ic2`. `flush` keeps every `|x| >= FLUSH_EPS` (`crates/lane/src/lib.rs:133`), so both
  integrators come back unchanged.
- **The output is the input.** The section returns `v0`, except for `-0.0`, which legs (a) and (c)
  exclude.
- **Non-finite input.** A non-finite `v0` makes both arms non-finite on the same lane, so the §4.4
  mask, the zeroing and the reset agree.

**The differential agrees.** As #977's verdict describes: 0 differing runs, with 1.33M admitted
blocks carrying a non-zero frozen or restored dead-section state. Two harness mutations go red: no
cap moves 934 runs (the restored `-MAX` behind +24 dB), and no floor moves 6,813.

**The questions asked:**

- **Gate 3 refuses `f32::MAX`.** Yes: `+-MAX` and the ceiling's successor, at either sign, refuse. M4
  re-run in a scratch copy is red on gate 3 (`lib.rs:4633`) and on gate 1 (`tests/bank.rs:2342`).
- **The cliff is fixed.** Gate 2 prints 9 of 9 at every width and section here. Under M1, the shipped
  `+0.0` rule, it prints 0 of 9 (re-run).
- **The renamed test is stronger.** Every old refusal case (`+-1e-30`, `-0.0`) still refuses, now in
  either integrator and on one lane, which tests the any-lane reduction. The floor's and ceiling's
  neighbours and `+-MAX` are added. `1.0` moves to the admitted list, as the rule requires. Every
  case now also asserts the per-section path's bits and integrators.
- **Rows and the protected test.** The 90 native and 30 wasm digests are identical.
  `a_tiny_restored_disabled_cut_state_refuses_elision_but_preserves_old_bands` is unchanged and
  green.

Findings:

1. **LOW (performance) `crates/parametric-eq/src/lib.rs:1057`.** `lane_is_inert` is a short-circuiting
   per-word `all()`, where the helper it replaced was a branch-free OR reduction. Natively it costs
   about 35-40 ns per eight-track bank block on every stationary block. Measured #978 to this
   commit, four alternations, `Simd8`: one-band dual 1,120 to 1,157 ns, two-band 1,611 to 1,647 ns.
   That is about +0.3 us per 64 tracks; the console `eq_only` isolate moved +0.46 us in two holds.
   V8 moves about +1 %. A branch-free reduction would remove the cost. Not blocking.
2. **LOW `docs/rulings/effect-floor-accounting.md:242`.** The refusal list still says "a non-`+0.0`
   state in a dead section". Since this issue it is a non-inert state. The file is outside the
   brief's paths; fix it at the batch boundary.
3. **NIT `crates/parametric-eq/src/lib.rs:4232`.** The comment "a non-`+0.0` state in a dead section
   is a refusal leg" is stale.
4. **Stacked on #977, which fails attempt 1.** Re-run gates 1-3 after #977 is revised.

## Attempt 2 re-verification (rebased onto #977 attempt 2)

2026-09-27. This commit was cherry-picked onto the revised #977 and #978 (`04e439db` -> `f2952432`).
Two conflicts, both mechanical, were resolved: the `elision` test module's import list (#977 renamed
its counter to `masked_pair_pass_count`) and the end of `MUTATIONS.md`. Two changes answer Sol's
findings:

- **LOW 1:** `lane_is_inert` is one branch-free reduction. It folds the lanes with non-short-circuiting
  `&`/`|`, and the range test is a single unsigned compare,
  `(word & MAGNITUDE_MASK).wrapping_sub(FLOOR) <= CEILING - FLOOR`. A scratch program
  (`scratchpad/work-977-evidence/inert_eq.rs`) checks it against the range form on all `2^32`
  words: they agree, and 2,786,728,839 words are inert, which is `1 + 2 * (CEILING - FLOOR + 1)`.
- **NIT 3:** the stale "non-`+0.0` state in a dead section" comment in the `elision` module now reads
  "non-inert".

Sol's LOW 2, the ruling's refusal list, is outside this issue's paths and is left for the batch
boundary, as the verdict says.

| gate | result |
|---|---|
| 1 `a_cut_switched_off_keeps_the_bank_eliding` | its base pins (`a34ce034…`, `2e0845c6…`, `26a755c1…`), and the overflow leg faults once per leg; dev and release, ±`test-support` |
| 2 engagement | 9 of 9 later blocks elide at every width and section, dev and release (first stationary block after the ramp too) |
| 3 refusals | green (every refused and admitted word, bits and integrators) |
| 4 suite and rows | `-p parametric-eq` 109 passed, 3 ignored, dev and release, ±`test-support`; `chain_shape` 23; 90 native lines and 30 wasm guest digests identical to base |
| 5 mutations | M1-M4 red on the new body with the same messages and digests (patterns rewritten for it; `MUTATIONS.md` notes the re-run) |
| 6 toolchain | as in #977's attempt-2 record, all green |
| 7 shipped artifact `1d46945a…` | callgraph and roster lines identical to #978's; the dual select-free depth-one tail is 83 instructions with no carried stack slot; one-band isolate 20.51 us against base 20.69 (range 20.15-21.43 over 6 runs), two bands 80.39 against 92.94 us, builtins within 0.3 %: PASS |

**Combined, the stack's tip against the batch base** (console harness and guest, two holds, EQ
isolates, us): native `Simd8` one band 9.82 / 9.56 -> 9.09 / 9.17, two bands 15.40 / 15.49 ->
13.11 / 13.32. V8 guest one band 20.94 / 21.17 -> 20.01 / 20.25, two bands 43.74 / 43.95 -> 30.29 /
30.84. The shipped artifact is in the gate 7 row above.

## Sol attempt 2 verdict: PASS

Verifier: Sol, 2026-09-27, on `f2952432`, merged onto the batch head `f12d1466`.

**The new `lane_is_inert` is exact.** For `m >= FLOOR`, `(m - FLOOR) <= CEILING - FLOOR` is
`m <= CEILING`. For `m < FLOOR`, the subtraction wraps to at least `2^32 - FLOOR = 0xe1c31af8`,
which is above `CEILING - FLOOR = 0x530d0dc2`. `+0.0` is admitted through `word == 0`.

**The rest.** The differential shows zero differing runs, over 1.33M admitted blocks with a non-zero
dead-section state. Gates 1-3 and the renamed test are green on the merged tree. Attempt 1's NIT 3
(the stale comment) is fixed.

Findings:

1. **LOW (performance, not blocking): the standing one-band browser EQ gives back part of #980's
   gain.**
   - **Size.** Against the batch head, the shipped artifact's one-band isolate at this commit is
     +0.47 us on average (+2.4 %) over 15 runs. It was higher in all three holds, by +0.23 to
     +0.72 us. That is about +0.7 % of the `eq_only` row.
   - **Where it lands.** Mostly at this commit: +0.15 to +0.22 us over #978. The likely cause is the
     dead-section check that runs on every stationary block, 20 calls per four-lane bank. Native is
     flat (-0.1 us).
   - **Why the evidence missed it.** The attempt-2 gate 7 "PASS, no greater than base" compared with
     `1d8c4851`, which lacks #980's -0.6 us.
   - **Remedy.** Two bands still gain 12 us, so this is a weekly-optimisation item, not a blocker.
     EQ-7, caching the legs per block, is the planned remedy.
2. **LOW (carried).** `docs/rulings/effect-floor-accounting.md:244` still says "non-`+0.0` state in a
   dead section". Fix it at the batch boundary.
