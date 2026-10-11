PASS

# #1407 attempt 2 -- adversarial verdict

Commit under review: `6af516317` (`codex/d15-stream-g`). Diff reviewed: `git diff ad96a6327 6af516317`.
I used only `git archive` and read-only git in the shared worktree. All builds and runs were in
exports under `/tmp/claude-1002/v1407a2/`. Evidence is in `/tmp/claude-1002/v1407a2/evidence/`:
the bound re-derivation (`bound.py`), the f32 simulation (`sim.py`, `sim2.py`), the probes, the
before/after bit dumps, the mutation script and logs, and the gate logs.

Summary: the attempt-1 BLOCKER is fixed. The invariant is correct and complete on the drain path.
Gate 6 is a real randomized differential, and it is red on attempt-1 semantics. The ramp-bound
proof is correct: I re-derived it independently, and an exact f32 simulation stays inside it. The
claimed bit moves reproduce exactly, and no pinned artifact should have moved. Every spec gate is
green. There is no BLOCKER and no MAJOR. One MINOR is worth folding: the owner-side leading
countdown that sets the stepped window in production has no test (mutants LF1 and LF2 survive
everything).

Root update received during the review: root accepted the four-stepped-word refinement in
`321f20024` (spec-only). I record 2(b) as root-accepted.

## BLOCKER

None.

## MAJOR

None.

## MINOR

### 1. No test reaches `InputStage::load_filter_leading`, the owner-side countdown that decides the stepped window in production.

Location: `crates/builtins/src/lib.rs:1328-1343`.

Gate 7 builds its own leading countdown (`filter_ramp_line.rs`, `remaining - floor`) and feeds the
kernel directly. Gate 3 checks frame 0 only. Gate 4's allowance is about 50 times the measured drift.
No pinned artifact runs a live filter ramp. Two plausible owner defects survive the whole builtins
crate (debug), `filter_liveness` (release) and gate 6:

- **LF1:** the floor is off by one, `64 - 4 + 1`. Production then steps three words, not four. The
  proven allowance becomes `P(1) = 1.563e-5`, so the stated `q_design + 1.419e-5` is no longer true.
- **LF2:** every lane takes lane 0's countdown. A bank lane whose ramp is out of phase with lane 0
  then gets the wrong window. In the worst case it computes every word from the target, which is
  option (iii) on every word. That gives `P(1) = 6.10e-5`, above the `5.21e-5` margin, so the
  stability proof no longer covers that lane.

The code is correct as committed. I checked it per lane, at every width, and in the mono path: the
mono body reads `[0][section]`, which is built from channel 0's countdown. But the property that root
accepted the refinement for depends on this function, and nothing tests it.

Fix: add a builtins-level test. It should render a `Simd4` and a `Simd8` bank, through both `process`
and `process_mono`, with lanes retargeted at staggered blocks and block sizes below and above 4. It
should then assert that every current word after every frame equals the oracle: four stepped words,
then `target - step*remaining`, then the hold. Same severity class as attempt 1's MINOR 3.

## NIT

2. **`P(q)` values.** The spec quotes `P(16) = 3.95e-6` and `P(63) = 1.16e-6`
   (`1407-...md:196` and `:535`). The closed form, which the test itself computes, gives `3.846e-6`
   and `1.182e-6`. The headline `P(q <= 4) = 1.419e-5` is right.
3. **"Undershoots" wording.** `1407-...md:187` says "the stored step undershoots the distance". This
   is not generally true: `fl(t - c)` can round up. The conclusion still holds, because no word uses
   more than `4/64` (stepped) or `59/64` (from the target) of `64 s`. So every word lies between its
   start and its target, and `|t - c| <= D` stands.
4. **Ruling formula.** The ruling (`builtins-input-liveness-d2.md:41-43`) writes the fixed point as
   "`E = max over j >= q of (64/j) rho_j` (and the interior-word maximum over it)". This merges
   `E_start` and `E`; the spec and `word_allowance` state them correctly. The ruling also rounds
   `16.3 u` to `9.715e-7`, where the spec and #1329 have `9.716e-7`.
5. **Lines over 100 columns** (rustfmt does not wrap comments):
   - `crates/builtins/src/lib.rs:976`, the doc comment of `collapsed`, 132 columns;
   - `crates/host-core/tests/successor_swap.rs:1958`, 145 columns;
   - spec lines 206-207.
6. **Gate 6 seeds at Simd8.** Gate 6 at `Simd8` is red on C1 for only one of its three committed
   seeds (seed 0). At `Simd4` it is red for all three. My 16-seed probe gives 10/16 red at `Simd8` and
   15/16 at `Simd4` (`evidence/gate6-reach.txt`), so the reach is adequate. Two more seeds would
   harden it cheaply.

## Item 1: the BLOCKER 1 fix (collapsed-stage invariant)

### The flag and the redirect

- `InputStage::collapsed` (`lib.rs:982`):
  - set at the top of `process_mono` (`:1914`);
  - cleared in `desymmetrize` (`:2072`).
- `live_state_channel` (`:1320`) returns 0 while the flag is set.
- Sizes are measured by a unit probe at the parent and at the commit: `InputStage<f32/Simd4/Simd8>`
  is 688 / 2,064 / 3,904 bytes before and after, and `BuiltinInputBank` is 3,936 bytes before and
  after.

### Audit of channel-1 reads

I searched every `state.section` read in `lib.rs` and the drain in `builtins-compiler`
(`drain_controls`: `set_trim_db`, `set_polarity_invert`, `apply_prepared_filter`, `witness.admit`).

The only reads of channel-1 state reachable from the drain or record-application path while
collapsed are the two that were redirected:

- the rule-3 predicate (`:1484`);
- `refresh_filter_plan` (`:1296`).

Everything else is in one of four groups:

- **Records** (coef, target, step, countdown, trim ramp). `process_mono` mirrors these at the bottom
  of every collapsed block, after `settle_filter`, and only then refreshes the plan.
- **Test-support only:** `lane_state_words` and `set_lane_state_words`.
- **The carry path:** `export_lane`.
- **The M3 proof:** `channels_agree`.

Two further paths:

- `reset_with_kind` zeroes both channels and leaves the flag set. This is harmless: the channels are
  equal, and the next disengage clears the flag.
- No record writes integrators, so no per-channel record needs to desymmetrize first. The audit is
  complete.

### The C3 argument (`export_lane` and `channels_agree` are unreachable while collapsed)

I judge the argument correct, by reading the code and by experiment.

- **`export_lane`.** Its only production caller is `adopt_input_lane`, at the carry
  (`graph/src/runtime.rs:4174-4214`). That function calls `chain.disengage_for_carry()` on each
  predecessor chain before any drain or export. `disengage_for_carry` runs `disengage_collapse`,
  which runs `desymmetrize` on every prefix slot and so clears the flag. The successor stage is fresh
  (`collapsed: false`), and the carry runs before it renders. Both entry points say so:
  `adopt_predecessor_plan` and `plan_exchange.rs:418-421` ("before it renders").
- **`channels_agree`.** The chain asks for it only when `armed && witness &&
  !collapse_channels_agree` (`rack/src/lib.rs:2473`). A collapsed previous block required
  `collapse_channels_agree == true`. Between blocks only two things can clear it, and neither applies
  to a stage that rendered collapsed:
  - the maintenance step, which runs only on `!collapse` blocks, after the disengage;
  - `inherit_channel_agreement`, which applies only to a successor that has not rendered.

  `stage.collapsed` implies `chain.collapsed` for prefix slots.
- **Experiment.** I put hard `assert!(!self.collapsed)` in both functions and ran the whole
  `test-debug-a` command (1,428 passed, 0 failed) and `builtins` + `conformance` (151 passed). None
  fired.
- **C3 itself.** The redirect in `export_lane` survives gate 6 (`mut-C3-g6.log`), as the record says.

### The debug assertion

The new assertion is `lib.rs:1764-1767`, in `process`. Mutant CD, where `desymmetrize` does not clear
the flag, trips it on the first dual block after a disengage: `successor_swap` aborts in debug. No
existing test trips the assertion on the committed code.

### Gate 6 (`successor_swap.rs:2142-2215`)

The gate is a real randomized differential:

- three seeds, nine mono strips, quantum 16, 480 blocks, a swap every 40 blocks;
- `Both` enables, disables and re-sends at 5 %;
- each odd segment drives one strip apart with a `Left` or `Right` enable;
- the carry runs with non-zero integrators into a successor that collapses at once;
- M3 proofs are asserted;
- the output is compared bit for bit with `force_mono_collapse_off` on every plan, at `Simd4` and
  `Simd8`.

There is no scalar run, and that is acceptable: the `Scalar` backend binds no banks
(`graph-compiler/src/lib.rs:9765`), so it never collapses and the comparison would be vacuous.

My mutation results:

| Mutant | Gate 6 result | Other tests |
|---|---|---|
| C1, rule-3 predicate reads channel 1 | red at both widths, first diff block 61, debug and release | none red |
| C12, C1 plus the plan not redirected (attempt-1 semantics) | red at both widths, debug and release | none red |
| C2, plan only | debug abort on `process_mono`'s plan-symmetry assertion | release renders the same bits (the mono body never uses the channel-1 plan entries) |
| CD | red, debug assertion | - |
| C3 | survives (unreachable, see above) | - |

The builtins crate is green on C1, so no existing builtins test catches the defect.

### Attempt 1's BLOCKER repro

I appended `evidence/zz_blocker1_repro.rs` to `successor_swap.rs` in the mutation tree. It is green on
`6af516317` at both widths and red on C1 and C12 at both widths.

## Item 2: MAJOR 2 (ramp arithmetic)

### (a) The proof is correct

I re-derived it independently (`evidence/bound.py`).

**Stepped words.** For a stepped word, `w_j = c + j s + sum eps_i`, with `s = fl(t - c)/64`. This
gives `rho_j = j h + (j/64)|t - c| u`.

**Words from the target.** For a word computed from the target,
`fl(t - fl(s r)) = m_j - (r/64)(t - c)[(1 + d)(1 + d1) - 1] + eps`. This gives
`rho_j = (1 - j/64)|t - c|(2u + u^2) + h`.

**Ranges and half-ulps.** From the designer (`lib.rs:723-745`):
- `c1 = t1/(1 + t1)` and `a3 = g^2/(1 + g(g + k))` lie in `[0, 1]`, so `h = 2^-25`;
- `a2 <= 1/(2 + sqrt 2) = 0.2929 < 0.3`, so `h = 2^-26`.

**Contraction.** At a restart, the hull point moves to `(1 - j/64) h_c + (j/64) t`, and the error
contracts to `(1 - j/64) e_c`. A frozen word or a held word keeps its error. A rule-3 jump or the
snap writes a design exactly.

**Fixed point.** The restart error has the fixed point `E_start = max over j >= q of (64/j) rho_j`.
Any word then has `E = max over j of (1 - j/64) E_start + rho_j`. For `q <= 4` this is exactly
`64 h + u D` = `(33, 16.3, 33) u` = `(1.967e-6, 9.716e-7, 1.967e-6)`.

**Norm step.** `A(h + e) - A(h) = -2 [[e1, e2], [-e2, e3]]`. The V-norm is convex, so its maximum
over the box is at a vertex. I checked this against 3,000 random interior points.

**Results.** This gives `P(1..4) = 1.4188e-5`, `P(5) = 1.175e-5`, `P(16) = 3.846e-6` and
`P(63) = 1.182e-6`. The stability margin follows:

| Rates | `q_top - 1 + P(1)` |
|---|---|
| 44.1 / 88.2 kHz | `-3.7945e-5` |
| 48 / 96 kHz | `-3.8223e-5` |

So the bound `<= 1 - 3.79e-5` holds at every rate and block size. I evaluated the operator norm two
ways, by sampling the unit circle and by the closed-form singular value, and they agree.

**Checks against the implementation.**
- The test's `word_allowance` and `norm_allowance` match my numbers at the quanta the test prints
  (for example `7.416e-6` at `q = 8`).
- Option (iii) on every word gives `P(1) = 6.0955e-5`.
- The accumulated law gives `1.4188e-5` at every `q`.
- Windows of 1 / 2 / 3 stepped words give `3.01e-5` / `1.98e-5` / `1.56e-5`.

So four is the smallest window that reaches the floor, as claimed.

**Empirical check of the componentwise claim.** `sim.py` and `sim2.py` simulate the kernel's f32
arithmetic exactly, with random, extreme and close restart chains. They track the exact hull point
and report `max |w - h| / E`:

| Law | Worst ratio | Where |
|---|---|---|
| Hybrid | `<= 0.979` | at `q = 63`, so the bound is nearly tight there and never exceeded |
| Accumulated | `<= 0.975` | - |
| (iii) | `<= 0.58` | - |

One limitation, of no consequence: the proof ignores underflow of `fl(t - c)/64`. Design words are at
least about `1e-7` apart, so no reachable step underflows.

### (b) The hybrid

Root accepted it (`321f20024`). It is a sound reading, and the decision is recorded.

For root's information (no action asked):
- At `q <= 4` the proven worst case is identical to the old accumulated law's, which is `1.419e-5`
  at every `q`. The spec says so itself.
- The gain is the proven bound at `q >= 5` (`3.85e-6` at 16, against `1.42e-5`).
- The measured drift is 10 times lower (`2.6e-7` against `2.57e-6`).
- The cost is that design-to-design retargets now move bits.

### (c) The per-lane leading countdown

The code is correct:
- `remaining.min(64) - 60` is computed per lane, from the same integer countdown that
  `load_filter_countdown` reloads.
- The countdown is decremented with the ramp, and a frame steps while the countdown is `>= 0`, which
  is exactly frames 1-4.
- It is partition-invariant, because it is rebuilt from the integer countdown every block.
- Settled and padding lanes take `done`.
- The mono body reads the channel-0 words.

I confirmed in the new code, frame by frame, that frame 5 and later are `target - step*remaining`
(`evidence/zz_words.rs`). The owner side has no test (MINOR 1).

### (d) Gate 4 is a real proof-consistency gate

- The allowance `P(q)` is computed from the closed form in the test, never pinned.
- The stability condition `q_top + P(1) < 1` is asserted per rate, from the real design words.
- Per-history resolution is restored. The 16 log-uniform histories draw no endpoints. There are 2
  endpoint histories and 4 close-retarget histories. Patterns 0-2 are attempt 1's chains near the
  maximum cutoff, and pattern 3 adds a disable and re-enable.
- It asserts that every recursion word is normal or zero.

Mutations: K1 (accumulated) is red in release (`q = 35`, `2.047e-6 > 1.907e-6`) and in the debug CI
subset (`q = 63`, `2.522e-6 > 1.182e-6`). K3, M2 and M3 are also red.

I also extended attempt 1's own probe (`evidence/zz_verifier_stall2.rs`) with random close cutoffs and
a maximum/half-maximum alternation. Over every rate, section and quantum 1-63, the worst excess is
`2.622e-7`, and the worst `excess / P(q)` is `0.043`.

### (e) Measured excesses reproduce

The release gate-4 table reproduces exactly:
- the worst excess is `2.622e-7` (44.1 / 88.2 kHz) and `1.964e-7` (48 / 96 kHz), against
  `P = 1.419e-5`;
- the log-uniform and endpoint excesses are 0;
- norm minus spectral radius is `1.97e-8` / `3.94e-8`.

### #1329 D5

The restatement is consistent:
- `E`;
- `q_design + 1.419e-5`;
- `1 - 3.794e-5` and `1 - 3.822e-5`;
- `q_ramp + 8.63e-7 < 1` with `3.70e-5` to spare (`3.794e-5 - 8.63e-7 = 3.708e-5`).

## Item 3: bits moved

Method: before is `ad96a6327`, after is `6af516317`, each in its own target directory. The probe is
attempt 1's `zz_verifier_bits.rs`, at all four launch rates, with the target applied at A = 256.

A warning on method: a shared `CARGO_TARGET_DIR` across `git archive` exports silently reuses the
other tree's build. The archive mtimes are the commit time, so cargo sees the new sources as older
than the build. My first "after" run did exactly this.

Results, which match the attempt record exactly:

| Change | Frames that move | First moved frame | Max \|diff\| per rate (44.1 / 48 / 88.2 / 96 kHz) |
|---|---|---|---|
| HPF 1 kHz to 2 kHz | 82 / 83 / 110 / 78 | A+6 (A+9 at 96 kHz) | `9.239e-7` / `4.768e-7` / `4.172e-7` / `2.980e-7` |
| LPF 15 Hz to 30 Hz | 56-227 | A+20 or later | `6.98e-10` to `4.66e-9` |
| HPF disable | 53-57, inside A+5..A+63 | A+5 | up to `4.172e-7` |
| HPF enable | 35-40, inside A+5..A+63 | A+6 or later | up to `3.576e-7` |

- LPF disable and LPF enable are bit-identical, because their mix words are exact in both laws.
- The crossfade now tracks the f64 blend more closely:
  - disable: `5.0e-8` to `6.2e-8` after, against `3.5e-7` to `3.9e-7` before;
  - enable: `4.1e-8` to `6.4e-8` after, against `2.0e-7` to `3.2e-7` before.

Pinned artifacts: none moved, and none should have.
- The builtins fixtures are prepared-only.
- The wasm G5 cross-target corpus (`builtins/src/corpus.rs`) runs no live filter retarget.
- The browser `expected.json` digests agree with the built module.
- The host-web live-filter tests check admission, not samples.

(Coverage note: no cross-target digest covers the new mul/sub/select ramp arithmetic. Gate 7 is its
per-width check.)

## Item 4: MINORs and NITs from attempt 1

- **M2c:** red only on gate 2b (debug and release).
- **"Slowest design":** gone from the spec's DSP evidence and from gate 4's doc.
- **Spectral radius:** stated to `1.97e-8` / `3.94e-8`, and reproduced.
- **Ruling:** the #808 paragraphs are held to 100 columns, and the old tail now comes before the
  rules.

## Item 5: the decision-15 sentence

As instructed, `6af516317` puts no replacement sentence in the spec or the ruling. The attempt record
defers it to the coordinator. The sentence the coordinator added in `321f20024` matches the proven
bound: "64 half-ulps plus u·D per word", `<= q_design + 1.419e-5 <= 1 - 3.79e-5`, and collapsed
equals dual. The #808 ruling now states the design/mixture claim as a proven bound: a design, the
identity at rest with `+0.0` integrators, or within `E` of the hull.

## Test value

- **Gate 2b** (`a_disable_in_the_enables_drain_still_completes_and_elides`): a rule 2 that restarts
  the countdown only when a mix word differs (M2c) leaves an enable and disable drained together on
  the design recursion under the identity mix, never cleared and never elided. Only this test is red.
- **Gate 3** (frame-0 expectations rewritten through `after_one`): a kernel that computes every word
  from the target (K2) misrounds the first mix word. It is red here and in gate 7.
- **Gate 4** (rewritten): judged by what its generator reaches. A law that drifts off the hull by more
  than the proven allowance at its block size is red: the accumulated law (K1), a dropped zero-step
  hold (K3), and rules 2 or 3 removed (M2, M3). This covers close-retarget chains at the maximum
  cutoff, every rate, and quanta 1-63 (release).
- **Gate 6** (`a_collapsed_chain_renders_the_forced_dual_bits_at_{four,eight}_lanes`): a collapsed
  stage that decides over channel 1's frozen integrators (C1, C12) renders bits the forced-dual run
  does not. A split plan (C2) or an uncleared flag (CD) trips the debug assertions. No builtins test
  catches C1.
- **Gate 7** (`every_filter_ramp_word_lies_on_the_line_to_its_target`): a kernel ramp body that
  accumulates every word (K1), computes every word from the target (K2), drops the hold (K3), holds at
  completion (K4), uses a 3-word window (KW), or reverts only the mono body (ML3) is red at rounding
  level, at every width. K4, KW and ML3 are red only here. (K4 is invisible at system level, because
  `settle_filter` snaps settled lanes, but it breaks the kernel contract.) The test does not reach
  the owner's leading countdown (MINOR 1).

## Gates run (export of `6af516317`, x86-64-v3)

| Gate | Result |
|---|---|
| `cargo test --locked --all-targets -p lane -p builtins -p dsp-reference --features builtins/test-support,lane/test-support` | pass, 235 passed |
| `cargo test --locked --release -p builtins --features builtins/test-support --test filter_liveness` | pass, 14 passed, table reproduced |
| `cargo test --locked -p host-core --features test-support --test successor_swap` | pass, 34 passed, gate 6 at both widths |
| `cargo test --locked -p lane --features test-support --test filter_ramp_line` | pass |
| `test-debug-a` workspace command | pass, 1,428 passed, 0 failed, 10 ignored |
| `test-debug-b` DSP command | pass, 834 passed |
| `test-release` (`-p lane -p math -p wasm-gates --release`, including `g5_native_digests_match_pins`) | pass, 110 passed |
| `cargo build --release -p audit && check-builtins-fixtures.sh` | `ok (50 files)` |
| Worklet chain: build `--named-twin`, `check-web-audioworklet.sh --without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts`, `test-web-audioworklet.sh` | all pass; shipped module sha256 `8d3920eeed352468a4fe2dda1446311a933d919d086a0ff31236a0b6b4f9c95f`, the same as the attempt record |
| `run-wasm-gates.sh` | ok; V8 spill gate's four held rows all `ok`, no carried stack slot |
| `check-cross-targets.sh` | PASS; `builtins` iOS `memset_pattern16` is 186, inside the #1018 expectation |
| `check-builtins-policy`, `check-workspace-policy`, `check-lane-policy`, `check-realtime-policy` | ok |
| `clippy -D warnings`, workspace, plain and with `builtins/test-support,lane/test-support,host-core/test-support` | clean |
| `cargo fmt --check` | clean |

Mutation runs (logs in `evidence/mutations/`): P, C1, C12, C2, CD, C3, M2, M2c, M3, K1, K2, K3, K4, KW,
ML3, LF1, LF2, as tabled above. LF1 and LF2 survive (MINOR 1), and C3 survives (unreachable).

## Not verified

- AArch64 and wasm execution of gates 4, 6 and 7 (CI-only here).
- The exact `f44cf54bf` `lib.rs` under gate 6. I used C12, which has the same collapse semantics
  with the new kernel, and attempt 1's own repro.
- Listening: the spec requires none.
