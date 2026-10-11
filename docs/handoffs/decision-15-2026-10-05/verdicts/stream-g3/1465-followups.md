PASS

# #1465 attempt-1 follow-ups: adversarial verdict

Commits reviewed: `dc3cb439f` (verdict MINORs 1-4 and NITs, parent `55a92ec11`) and `583fd8607`
(root's ruling (c), batched calibration, parent `dc3cb439f`), as `git diff 55a92ec11 583fd8607`.
I built and tested an export of `583fd8607` (`/tmp/claude-1002/v1465fu/tree`,
`CARGO_TARGET_DIR=/tmp/claude-1002/v1465fu/target`) and an export of `55a92ec11` beside it for the
bit comparison. I did not touch the worktree.

Verdict: PASS. There is no BLOCKER and no MAJOR. The anchor change is sound as a proof and as
code. The new math test goes red on its named defect, and no other test catches that defect. No
builtin bound moved (2,112 designs, every bit of #1329's values, the composition, both halves of
the certificate and `input_section_bound`). The rewritten underflow step is correct. The batched
calibration implements ruling (c), and the record matches the raw outputs figure for figure.
Every gate is green. There are three MINORs: one class's batches are shorter than the ruling's
1 ms, one wrong middle factor in the `P^` bound, and one false statement (`t0 >= 256`) that came
from my own attempt-1 verdict. There are four NITs.

## BLOCKER

None.

## MAJOR

None.

## MINOR

1. **The one-section class does not meet the ruling's "about 1 ms or more", on most of its
   batches, not only on the shortest one** (`crates/builtins/examples/input_bound_budget.rs:433`,
   `BATCH = 48` for every class; spec `:863-867`). My probe timed the fixed-cost grid batches
   (`/tmp/claude-1002/v1465fu/ev/probe.out`, verifier-only, the committed `batches()` and
   `Batch::measure`, one run, load 2.1-2.8). At every rate, **4 of the 6 one-section batches have
   medians of 0.59-0.99 ms**. That is 16 of 24, and they are 4 of the 5 batches that go into that
   class's fit at each rate (the sixth, at about 56,000 frames a design, is above
   `CALIBRATION_FRAMES`). One two-section batch per rate is at 0.98-0.99 ms ("about 1 ms"). Every
   other batch is 1.14 ms or longer. The record states only the shortest sample (0.584 ms). It
   argues that the class does not bind the frame-equivalent (its need is about 11-12 ns against
   17.0). That is true, but the class also feeds the **section charge**: the charge is the largest
   per-section intercept over the classes. There the one-section class is 9-19 % below the binding
   two-section cost (median fits 7.60-8.46 us against 9.34 us in the record, and 7.55-8.45 us
   against 9.31 us in my run). In the record's run its per-round intercepts spread 8-17 % a rate
   (7.10-8.56 us in all), against 3-7 % for the two-section class. That spread is the short-sample
   noise that the ruling targets. (My run had a higher load, and both classes were noisier in it:
   6.66-8.76 us for one section.) In both runs the deviation moved
   no constant. Ruling (a) makes the batch verdict's run of this code the figure of record. So,
   before that run, make every batch meet the 1 ms floor. For example, give the one-section class
   a 96-step cutoff grid and batches of 96 (six batches a rate, each about 1.2 ms or more), or set
   each class's N from a 1 ms target. If not, get root's explicit acceptance of the deviation. The
   docs also say "the shortest designs (about 25-30 us a walk)" (`:34`, `:430`), but one-section
   designs take about 12-14 us. The charge doc in `crates/builtins/src/tail.rs:321` says 12-30 us,
   which is correct.
2. **The middle factor of the `P^` bound is wrong; the final bound is correct**
   (`docs/derivations/1379-graph-tail-composition.md:27`, `:256-258`; spec `:50`). The
   derivation says that `ceil_mB` adds "less than one millibel plus `|x| 2^-30 + 2^-30`", so
   `sigma < 10^(1/2000) (1 + 2^-29) F a`. Here `x` is the level in millibels
   (`ceil_millibels`, `crates/builtins/src/tail.rs:81-86`). The factor that the margin gives is
   `10^((|x| + 1) 2^-30 / 2000)`. That is at most `1 + 2^-29` only while `|x| <= 1,736` mB. The
   stalls are at -33,000 to -39,000 mB (F3 rows), and `|x|` can reach about 76,000 mB (a
   `2^-126` stall). This gives `1 + 4e-8` to `1 + 8e-8`, which is more than 20 times `2^-29`.
   The end bound still holds: with `|x| < 650,000` mB (the whole `f64` range) the factor is below
   `1 + 2^-20`, and `2 * 10^(1/2000) (1 + 2^-20) = 2.0023059 < 2.0024`. Replace `(1 + 2^-29)` with
   `(1 + 2^-20)` and state the reason (`|x| < 650,000` mB). I re-measured F3 (release
   `tail_contract`): `P^ >= 2 P*` on 16 of 112 rows, maximum 2.0016, and `P^ >= P*` on every row.
   These agree with the record.
3. **"so `t0 >= 256`" is false** (`docs/derivations/1379-graph-tail-composition.md:129`). The
   argument (threshold at most about 0.5, `O >= ||h_1||_1 >= 1`) gives `t0 >= 1`, not `t0 >= 256`.
   A design whose output majorant is concentrated in its first frames crosses inside block 0. On
   my grid of 2,112 builtin designs (4 rates, 11 x 11 cutoffs, trims -144, -143.99, -100, -61.3,
   -6, 0, 3.3, +24 dB), **346 rows have `t0 < 256`, and the least is `t0 = 2`**. Examples:
   44.1 kHz, LPF 3,170 Hz, -144 dB: `t0 = 5`; LPF 1,003 Hz, 0 dB: `t0 = 174`. The conclusion that
   no builtin design reaches `t0 = 0` holds. The error is mine: my attempt-1 verdict (MINOR 2)
   wrote "so `first_block >= 1` and `t0 >= 256` always", and the follow-up copied it. Write
   "so `t0 >= 1` (the least on a 2,112-design grid is 2)".

## NIT

- **The underflow step does not state the multiplicity of a loss** (derivation `:205-213`). A
  loss in `G^(2^s)` reaches the result through every later use of that power: it is squared
  into each higher power, so it appears about `m / 2^s` times, each time at a different left
  exponent `a`. The bound per loss site still holds, because the left factors of those copies sum
  to at most `sum_a G^a <= R` (1.07 R for the computed powers). One sentence would close this.
  The count "`26 * 16 * 4` products" also leaves out the `SLACK` product of each entry and of
  each `dot`. With them there are about 2,620 sites, which is still below `2^12`. With these two
  additions, I checked every other step and found it correct: the gap
  `1 - G_pp > 3 2^-30 - 2^-52 > 2^-29` from the code's
  `lambda - G_pp SLACK^3 > 0`, `R < 2^116 * 125 < 2^123` (`M`: `< 2.85 2^58`), the inflation
  `SLACK^(2^26) < 1.07`, `2^12 2^-1075 (2^249 max z + 2^124)`, the state bounds `w < 2^93` and
  `E_2 < 2^93`, and the conclusion `< 2^-719`, which `tau = 2^-600` covers.
- **The new test's power rests on 7 tie gains of one single-section design.** A probe that logs
  `first_block >= 1 && t0 == 0` on the unmutated code found 7 distinct gains, all on one
  single-section design (`/tmp/claude-1002/v1465fu/ev/mut-anchor.txt`). Nothing in the test asserts
  that a tie is present. If the order of a sum changes, the ties can go away, and the test then
  stays green without the power to catch the defect. The probe in the record also found a tie for
  LOW_LPF into TOP_LPF, and the test does not include that design. Adding it would give the test a
  second design with a tie.
- **"Every run's value" lists calibrations only** (spec `:812-819`). Gate-2 runs are runs too:
  fold-in 2's gate 2 (need 19.0 ns, 18.523), my attempt-1 addendum's descriptive gate 2 on
  `55a92ec11` (19.0 ns, 18.887, charge 470), and my two descriptive runs below (calibrate
  17.5 ns / 540; gate 2 16.5 ns, 16.367).
- The F3 maximum in the record is "(88.2 kHz, 1 kHz HPF at 0 dB, and 1 kHz LPF at +24 dB)". At the
  printed four digits, four more rows round to the same 2.0016: 88.2 kHz LPF 1 kHz at 0 dB, and
  three 48 kHz rows.

## 1. The anchor change

- **As a proof.** `pass_end_sums` is `Majorants::remainders()` at the pass's last frame. That is
  `remainder_sums(constants, m(H))`, the same function the `t0 > 0` branch applies to the record
  at `t0`. So it is `w(H) >= sum_{t >= H} m(t)`, with `H >= 256 >= 1`. Summing
  `m(t + 1) <= M m(t)` gives `w(t + 1) <= M w(t)` for `t >= 1`. When `H <= T`, the carry
  `M^(T - H) w(H)` bounds `w(T)`. When `H > T`, `o = H - T`, and every crossing `o + ...` is at
  or after `H`, where `w(H)` bounds the suffix. Neither case uses `t0`, so `H` is sound for every
  `t0`. In the code, `t0 > 0` implies `first_block >= 1`, so `(first_block - 1) * BLOCK` cannot
  underflow. The record index `t0 - (first_block - 1) * BLOCK` is the frame `t0` (the record is
  pushed before each step, plus once after the last step).
- **As code.** `crates/math/src/tail.rs:2649` tests `t0 == 0`. Before the change, the old test
  read frame 0's record (all zero, before the impulse) when `first_block = 1` and `t0 = 0`. Its
  sums were 0, so the half became `tau lambda^i`.
- **Mutation, redone.** I restored `first_block == 0`. The new test went red:
  `gain 2.854376257044066e-8: the walked suffix 1.10524550631239e0 from frame 0 (k = 1) is not
  below 1.1052455073417926e-1`. This is the record's message. The other 7 math lib tests stayed
  green, and the release `tail_contract` stayed green (17 passed) under the mutant. Restored: green.
- **Test value.** "An anchor that reads frame 0's pre-impulse record when the crossing block is
  block 1 but `t0 = 0` turns it red, and nothing else reaches `t0 = 0`" holds. The math lib and
  `tail_contract` are green under the mutant, and builtin designs have `t0 >= 2`. The test also
  checks the `H` branch on every gain below the boundary where `t0 = 0` and there is no tie, and
  the builtins never reach that branch.
- **No builtin bound moved.** I put a harness in both exports (`zz_v1465fu_bits.rs`, verifier only,
  not shipped) and ran it on 2,112 designs (the grid above). For each design it printed: `tail`,
  `tail_every_peak`, both rests, `rest_at_flush_floor`, `tail_reference`, the bits of
  `flush_floor`, the frames walked, `D`, the bits of each half's `lambda`, `offset`, `transient`
  and `decade`, the bits of `O`, `dev_loud`, `peak_gain` and `stall`, and the
  `input_section_bound` `NodeTailBound`. `55a92ec11` and `583fd8607` are **byte-identical**
  (`cmp`), and the 550 charge left every single design inside the budget.

## 2. The rewritten underflow step and the `P^` bound

- The underflow step is correct (see the first NIT for the one sentence it omits and the site
  count). The resolvent now comes from the verified gap and not from `2^53`. A loss now enters as
  `G^a E G^b z` with both factors and `z`, as attempt 1 asked.
- The `P^` bound: `P* <= P^` holds. `P* >= 2 F a / eps` holds. The middle factor is MINOR 2. The
  end bound `< 2.0024 P*` holds.
- The other NIT fixes of `dc3cb439f` are correct: `D` (not `T(k)`) is below `HORIZON_LIMIT`; the
  `k = 1` case is right for `a >= 0`, and `a < 0` is stated; the contract's `Unstated` cases;
  M2/M2r; the M12 and M13 texts; the F1(a) and F2(c) figures.
- **F1(b) at -144 dB (evidence) reproduces exactly.** I added -144 to the F1 test's trims and
  skipped the 1.5 assertion there, in a verifier-only change that I then reverted. All 56 rows pass
  (a), (c) and (d). The largest ratio is **1.358** (44.1 and 88.2 kHz: `T` 59,596, `D` 70,794,
  `D_mean` 52,147.1, `D_floor` 45,036; 48 and 96 kHz: `D` 70,414, `D_mean` 51,863.4). Next come
  the 10 Hz HPF into 1 kHz rows, at 1.331-1.338.

## 3. The batched calibration against ruling (c)

- **Different designs, no cache hits, the same frames.** `preparation()` asserts
  `fixed_input_bounds_computed()` (a thread-local count of designs computed, not served from the
  cache) equals the batch size, with a fresh cache and no budget. `Batch::measure` asserts that
  the frames and sections of each round are deterministic, and that the sum of the single walks'
  frames equals the batch's frames. The first incomplete run (`cal-crashed.out`, 47 of 48) shows
  that the assertion catches designs with the same words. The batch is a real
  `input_section_bounds_within` preparation, so it walks its strips in preparation order. The
  cache is built outside the timed closure in both the batch and the single walks.
- **The joint search uses per-design medians.** `fixed_costs` fits `median(ns) / count` against
  `frames / count`. `frame_equivalent` compares each batch's whole median with its whole charge,
  which is the same as the per-design ratio. The charge is the largest median-fit intercept per
  section, and the two values are found together on the 0.5 ns grid. Near-top points are batches
  of one (5-7 ms). Gate 2 is unchanged except for its constants.
- **Batch against single walks: 0.9842-1.0367, median 1.0000 (the record).** My run gave
  0.9842-1.0498, median 1.0000, 108 of 960 below 1. The batch is not systematically cheaper. A
  single walk also carries one extra preparation's overhead, which would bias the ratio below 1,
  and the ratio is centred on 1 all the same. The single-walk medians need 16.933 ns at charge
  550 (17.071 at 540 in my run), so the same grid value follows. I accept this as the measurement
  the ruling asks for.
- **The record matches `cal.out` and `g2.out`.** I checked: 960 batches and 27,144 designs
  (6/70/24/8 batches a rate; 288/3,384/1,152/420 designs), 9.34 us, 549.7, 550, 9.35 us, the
  binding batch (48 kHz, two sections, batch 34, 44,723 ns, 1,537.0 + 1,100 = 2,637.0, 16.960 ns),
  the per-design terms -1.93 to -1.44 us, the median fits' largest 8.37-9.34 us, the per-round
  values 8.35-9.68 us, 0.584 ms, the raw maximum 19.104 ns and 1.468 x, the batch/single values,
  the per-class ranges, the 16.65 ns slowest class, and the loads and times (93 s, 15 s). Gate 2:
  25.67 ms, need 16.5 (16.289), worst 24.597 ms (95.8 %, 807 of 4,096 exact), raw 24.831 ms
  (96.7 %, 1.010 x), every per-family median range, ratios 1.000-1.054, and 65,537 strips at
  61.20-62.64 ms (warmup excluded). The two incomplete runs printed no frame-equivalent, as the
  record says. The calibration binary was built with charge 470. This does not matter: the
  section count is `fixed / charge`, and there is no budget.

## 4. The restated figures

They are consistent. `INPUT_BOUND_SECTION_CHARGE = 550` and its doc. `FRAME_EQUIVALENT_NS = 17.0`.
25.67 ms (1,510,000 x 17.0 ns). The cache cap: 257 + 550 = 807, and 1,510,000 / 807 = 1,871. #1468
(550 of 17.0 ns). #1470 (17.0 ns, 25.67 ms, 24.60 ms worst at 95.8 %, cheap 17.80-24.60, typical
11.08-22.88, near-top 20.64-21.22, band 20.20-20.45, top designs 3.71-7.33, charge 550). #1471
(17.0 ns, 25.67 ms). #1474's note (17.0, 550, 25.67, 807, 1,871, earlier values superseded).
Gate 8's figures in #1465. A search for 21.5 ns, 470, 32.47/32.465, 727, 2,077 and 27.97 outside
the #1465 record finds only #1474's "superseded" sentence (and numbers that are not related, such
as line references).

## 5. Descriptive runs (one invocation each, `taskset -c 7`; recorded, not the figure of record)

- **calibrate** (load before 1.95 2.48 2.00 after a 30 s wait for the 1-minute load to fall
  below 2, which was not a retry; after the run 2.23 2.44 2.03; 94 s; exit 0): **17.5 ns, charge
  540**. Binding: 96 kHz, one cascade of two sections, batch 29, 45,285 ns a design,
  2,638.3 charged frame-equivalents, **17.164 ns** (record: 16.960). The largest median-fit
  fixed cost per section is 9.31 us. The one-section median fits are 7.55-8.45 us. The shortest
  batch sample is 0.581 ms. The largest raw sample is 2.189 x its batch's median. The batched
  statistic is much better conditioned than before. The binding ratio moved 1.2 % (it was about
  9 % with single walks: 21.5 against 23.5 ns). But 16.960 was only 0.24 % below 17.0, so a 1.2 %
  move crosses the grid step. The batch verdict's run will decide.
- **gate 2** (load before 1.94 2.36 2.01, after 2.03 2.36 2.01; 15 s; exit 0): need **16.5 ns**
  (16.367). Worst median **24.714 ms, 96.3 %** of 25.67 ms (cheap 1 kHz into 1.28 kHz, +24 dB,
  88.2 kHz). Largest raw 24.868 ms (96.9 %, 1.006 x). Ratios 1.001-1.032. No family was over the
  budget, so 17.0 ns holds against gate 2, with a thin margin.

## Test value

- `the_reference_certificate_is_sound_where_the_crossing_is_at_frame_zero` (new, `math`): an
  anchor that reads frame 0's pre-impulse record when the replay's crossing block is block 1 but
  `t0 = 0` (`first_block == 0` restored) turns it red at the tie gains (walked suffix 1.105 from
  frame 0 against 0.1105 at `k = 1`). No other test catches it: the other math tests and the
  release `tail_contract` stay green under the mutant, and builtin designs have `t0 >= 2`.

## Gates run (export of `583fd8607`)

- `cargo test --locked --release -p builtins --features builtins/test-support --test
  tail_contract -- --nocapture --test-threads=1`: 17 passed (F3: 112 rows, `P^ >= P*` on all).
- `cargo test --locked -p math`: lib 8 passed (the new test included), integration tests pass.
- `cargo test --locked --no-fail-fast --all-targets -p lane -p math -p builtins -p dsp-reference
  --features math/lane,builtins/test-support,lane/test-support`: 45 test binaries, all pass.
- `cargo test --locked -p builtins --features test-support --lib`: 15 passed.
- `cargo test --locked -p builtins-compiler --features test-support --lib`: 57 passed. Gate 8 is
  exact at every rate, and its charges equal the record: stereo 751,232 / 799,616 / 1,298,560 /
  1,394,816; mono 386,880 / 412,224 / 671,552 / 721,216; least margin **115,184** (96 kHz stereo,
  1,254,016 frames walked).
- `cargo fmt --all -- --check`: clean. `cargo clippy --locked -p math -p builtins -p
  builtins-compiler --all-targets --all-features -- -D warnings`: clean (the example included).
- `scripts/check-workspace-policy.sh`: ok. `scripts/check-realtime-policy.sh`: ok (89 regions).
- Not run: `scripts/check-cross-targets.sh`. Free disk was 29 GB while another build ran, and the
  script checks the whole workspace for several targets. The only change to engine code is one
  boolean condition and a `#[cfg(test)]` test in `math`. The implementer recorded PASS.

Evidence is in `/tmp/claude-1002/v1465fu/ev/`: `bits-base.txt`, `bits-new.txt`, the harness
`zz_v1465fu_bits.rs`, `mut-anchor.txt`, `mut-anchor-tail_contract.log`,
`tail_contract_release.log`, `f1b-144.log`, `cal.out`/`cal.load`, `g2.out`/`g2.load`,
`probe.out` with its source `zz_probe.rs`, `bc-lib.log`, `all-targets.log`. The target and
the trees are deleted.
