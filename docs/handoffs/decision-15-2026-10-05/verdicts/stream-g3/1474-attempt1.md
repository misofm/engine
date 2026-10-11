PASS

# #1474 attempt 1: verdict (decision-15 stream G, batch 3)

Verifier: opus-xhigh, 2026-10-08. Commits reviewed: `3c1f4ee23` (parent `1e78d7820`) and `04fc3cc8f`
(parent `7c6cdb1b4`, #1473's commit; it does not touch `math`, `builtins` or the example). Built from
`git archive 04fc3cc8f` in `/tmp/claude-1002/v1474/tree`, `CARGO_TARGET_DIR=/tmp/claude-1002/v1474/target`.
The base was exported from `1e78d7820` and built in a separate target dir (a shared target dir
reused the final tree's artifacts for the base because `git archive` mtimes are older; I caught it
by binary hash and rebuilt). The worktree was only read.

No BLOCKER and no MAJOR. Three MINORs and four NITs below.

## The soundness of the flush, per quantity

I read `Majorants::step`, `Majorants::flush_first`, `Majorants::remainders`, `Deviation::floor`,
`fixed_cascade_walk` and the derivation's new paragraphs (`docs/derivations/1329-input-section-tail-and-rest.md:264-320`).
I found no path where a propagated quantity can fall below what it bounds.

- **Non-negative quantities** (`first_error`, `later`, `out.input[1..]`, the deviation's `error`
  and `reference`, the walked value): each is `max(fl(step), TAU)`. A larger non-negative value is
  still a majorant under the non-negative recursions. Correct.
- **The signed first state.** A word with `0 < |w| < TAU` is set to zero and `TAU` per flushed word
  is added to `first_error` as `(e + n TAU) * STEP_UP`. `v_norm(x) = ||R x||_2` with
  `R = [[1, r], [0, r]]`, whose columns have 2-norm one (`tail.rs:210-215`), so the flushed vector's
  `V`-norm is at most `|w0| + |w1| < n TAU`. Both summands are normal, so
  `(1 - u)^2 (1 + 4u) >= 1` holds. The flush runs after the state's step and after `first_error`'s
  step, which used the pre-step (already flushed) state, so the step error `nu (|s1| + |s2|)`
  bounds the rounding of the operations on the stored values. Correct.
- **Underflow term.** `fl(x*y) = xy(1+d) + e`, `|e| <= 2^-1075`, sums exact on underflow: correct.
  If the bounded value `T <= tau` the floor covers it; else `E >= T > tau` and the margin
  `(u - 9u^2) tau >= 2^-654` exceeds `2 r 2^-1075` for `r < 2^420`. Correct. In practice no product
  of a stored state word (zero or `>= 2^-600`) with an `A` entry (from `f32` words) underflows.
- **The `|s1| + |s2|` remainder guard** (`tail.rs:1888-1892`, called from `remainders`): `|s1| + |s2| >= ||R s||_2` by the
  triangle inequality. When one word is `>= 2^-500`, one component of `R s` is at least
  `~s_big / 2.6`, so `||R s||^2 >= 2^-1003` and a lost square costs at most `2^-74` relative,
  far inside `SLACK`. Correct.
- **Closing terms.** Every absolute loss is at the `2^-1075` scale against `SLACK` margins on values
  at the threshold scale (`>= 2^-34`). The deviation's closed form: with `x_n = max(L x_{n-1}, tau)
  <= x_{n-1}`, `L x_n <= L x_{n-1} <= x_n`, so the powers from the floored state stay
  non-increasing and bound the true deviation. Correct.
- **Termination.** The derivation's constant ranges hold: over a 0.1 % cutoff grid at the four
  rates, max `beta` 1.99995, `gamma` 1.41389, `|d|` 0.99995, `1/(1-q)` 19,181. A failed termination
  gives `TailBoundError::Horizon`, never a wrong bound.

Counterexample search: none found (state flush placement, frame 0, zero words, one word flushed, both
flushed, replay from checkpoints, `at_end` floor versus the unfloored `at_end` of the rest bound).

## Gate 1: bounds identical (reproduced)

Verifier-only dumper (not committed): 16,228 `fixed_cascade_within` cascades, 4 rates x 4 gains
(0, +12, +24, -24 dB), HPF-only, LPF-only and HPF->LPF over a log grid, the 0.70-0.95 near-top band
in 0.5 % steps, the top f32 steps and gate 2's cheap pairs; 4,337 walks above 400,000 frames; every
`CascadeBound` field printed with its `f64` bits plus frames walked. Base `1e78d7820` against final:
**identical**, sha256 `8783acb9...` both ways. The base took 68.8 s CPU, the final 47.3 s.
`tail_contract` (release, the CI command): 13 passed.

## Gate 2 (diagnosis): reproduced on the base walk

Instrumented test in my base export, `taskset -c 7`, release, load about 5: HPF at 0.778 of the
48 kHz maximum into the top LPF, +24 dB: first state subnormal on 75 frames, exactly zero on 449,297,
error radius subnormal on 449,424; 23.97 ns a frame normal against 13.50 ns under FTZ+DAZ, bounds
identical. HPF at 0.80: state subnormal on 449,217 frames, radius on 449,274; 19.96 against 13.51 ns.
This agrees with the record's table (my counts include the replay and deviation frames).

## Gate 5: mutation runs (redone)

`cargo test -p math --lib tail::` per mutation, file restored after each:

| mutation | red |
|---|---|
| M1 radius set to 0 when `<= TAU` | `a_non_negative...`, `the_band_designs...` |
| M2 later majorant set to 0 below `TAU` | `a_non_negative...` |
| M3 floor removed on `out.input[i+1]` | `a_non_negative...` |
| M3b floor removed on `out.input[1]` | `a_non_negative...`, `the_band_designs...` |
| M4 deviation floor sets to 0 | `a_non_negative...` |
| M5 flush without the `TAU` addition | `the_first_state...` |
| M6 state rounded up to `TAU` | `the_first_state...` |
| M7 words below `4 TAU` flushed | `the_first_state...` |
| M8 flush before the state's step | `the_first_state...`, `the_band_designs...` |
| M9 flush removed | `the_first_state...`, `the_band_designs...` |
| M10 remainder always `v_norm` | `a_tiny_first_state...` |
| M11 `flushed = TAU` (one `TAU` for two words) | **none** (MINOR-1) |
| M12 no `STEP_UP` on the addition | none (one-ulp rounding; not demanded) |

No existing test catches them: `tail_contract` (release) stays green under M1, M5, M6, M9, M10 and
M11, and the two older `math::tail` tests stay green under every mutation.

### Test value

- `a_non_negative_majorant_below_tau_propagates_as_tau`: red when any non-negative quantity's floor
  rounds down, keeps the subnormal or sets it to zero (radius, later majorant, input and output
  majorants, deviation components), which no other test sees.
- `the_first_state_below_tau_is_flushed_to_zero_and_its_magnitude_enters_the_radius`: red when the
  flush drops the radius addition, rounds the signed state up, flushes words at or above `tau`, or
  runs before the step; not when it adds only one `tau` for two words (MINOR-1).
- `a_tiny_first_state_still_enters_the_remainder`: red when the remainder squares words whose
  squares underflow and the state drops out of the sum.
- `the_band_designs_walk_with_every_propagated_quantity_at_least_tau`: red when a band design leaves
  the radius, an output majorant or a state word below `tau` at the end of a frame (floor removed,
  flush removed or misplaced): the premise of the underflow term.

## Gates 3 and 4: timing and gate 8

- **17.0 ns is justified by the record.** The final calibration (`g3/cal-final.log`, load
  1.52 -> 2.18) gives the slowest median 16.54 ns (16.39-17.10), typical, 80 Hz into 18 kHz, 0 dB,
  44.1 kHz; 17.0 is that median rounded up, as the spec and the ruling require. Every family of the
  final gate 2 (`g3/gate2-final.log`, load 1.96 -> 2.13) is at most 16.85 ns per consumed
  frame-equivalent, except the 21.35 ns sample below. 7.03 us / 17.0 ns = 413.5, so 420. Every
  figure in the record matches the logs.
- **The 28.44 ms sample: I rule it tainted.** It is the no-cache preparation of 64 typical designs,
  88.2 kHz, round 2. The walk is deterministic (1,278,528 frames and 1,332,288 charged in every
  sample). In the same round the first preparation, which does the same walks plus the cache
  insertion, took 19.43 ms, and the other two no-cache samples at that rate took 19.39 and 19.52 ms.
  A 46 % jump on one of three identical computations is not a frame class. My own descriptive run
  (below) gives 19.60-19.66 ms for the same rows. Under root's standing ruling, the batch verdict's
  single gate-2 invocation is therefore the figure of record.
- **25.44 ms inside the budget.** True for the recorded run: 25.444 ms against 25.67 ms
  (1,510,000 x 17.0 ns). See MINOR-3 for the margin.
- **The record states the runs plainly:** three calibrations (before, after, final) and two gate-2
  invocations (one on uncommitted 14.0 ns / 490 constants, one on the final constants), each with
  its load; nothing was retried.
- **Verifier's descriptive gate 2** (one invocation, `taskset -c 7`, load 1.78 -> 1.89; it does
  not overrule the record): worst design work 24.34 ms (16.12 ns per consumed frame-equivalent),
  4,096 cheap two-section 1 / 1.28 kHz, 88.2 kHz; 64 typical at 88.2 kHz 19.60-19.66 ms.
  `/tmp/claude-1002/v1474/gate2-verifier.log`.
- **Gate 8** (Amendment 3's command): passed; the charges and margins equal the record exactly
  (96 kHz charged 1,361,536, margin 148,464; 88.2 kHz 244,720; mono 96 kHz 704,576). Frames walked
  are the same as before the flush (96 kHz 1,254,016), so the charge change is the constant only.
- **The other specs.** #1468 `:152` and #1471 `:21` change only the figures. #1470 `:28-33`
  changes only the figures and their source. #1470 `:48` is still stale (MINOR-2).

## Findings

### MINOR-1: the flush test does not check the full magnitude of two flushed words

`crates/math/src/tail.rs:2525-2562`. The test's flushed state has a `V`-norm of about 0.5 `tau`,
so a flush that adds one `tau` for two flushed words (`flushed = TAU` instead of
`flushed += TAU`, M11) stays green. That defect is unsound: words `(0.9 tau, 0.9 tau)` have a
`V`-norm of 1.66 `tau`. A verifier probe (not committed) that sets `first = [0.9 TAU, 0.9 TAU]`,
`first_error = 4 TAU`, calls `flush_first` and asserts
`first_error >= 4 TAU + v_norm([0.9, 0.9]) TAU` is green on the committed code and red under M11.
Add such a case to the existing test (no new file).

### MINOR-2: one stale figure left in #1470

`.github/ISSUE_SPECS/1470-cache-design-bounds-across-browser-preparations-in-the-control-worker.md:48`
still says `INPUT_BOUND_SECTION_CHARGE` (290 frame-equivalents). Root authorized correcting the
stale figures in #1470, and the record lists only `:28-31`. It should say 420.

### MINOR-3: the margin is thinner than the run-to-run spread; say so for the batch

The record gives every figure, but not the margin they imply. The worst family is at 99.1 % of the
budget (25.44 of 25.67 ms; 16.85 of 17.0 ns per consumed frame-equivalent). The same family at
the same rate spread 24.28-25.44 ms (4.6 %) in the recorded run. The calibration's fit slopes for
two cascades of two sections reach 16.90 ns per frame (44.1 kHz, both measured rounds 16.80-16.90),
0.6 % under 17.0, and the slowest class's own spread reaches 17.10. This follows the ruling's
median statistic and fails no gate. But because the 28.44 ms sample is tainted, the batch verdict's
one gate-2 invocation becomes the figure of record, and it can cross 25.67 ms with no code change
(my run: 24.34 ms). Root should know this before the batch run. The record should state the margin
in one line.

### NIT-1: 25.7 ms against 25.67 ms

With a 0.23 ms margin, the rounding matters. The record (`1474...md:330,360`), #1470 (`:29,31`)
and `crates/builtins/src/tail.rs:295` say 25.7 ms; the tool prints 25.67 ms.

### NIT-2: `TAU` visibility

`crates/math/src/tail.rs:128`: `pub(crate)`, but only `tail.rs` and its own test module use it; a
private `const` is enough.

### NIT-3: the termination paragraph omits the suffix-sum test

`docs/derivations/1329-input-section-tail-and-rest.md:305-320` covers the remainder, the falling
test, the `eps / 4` test and `p_star`, but not the suffix-sum search for `t0`. There the floors add
at most `HORIZON_LIMIT tau = 2^-574` against a threshold above `2^-34`. "Falls as it did" for the
falling test is informal; the Horizon fallback covers termination, so this is wording only.

### NIT-4: the calibration binary carried the old constants

`g3/cal-final.log` ends "frame-equivalent 23.5 ns (committed)". The measurement does not depend on
the constants (`one_design` uses a budget of `u64::MAX`), so it is valid for 17.0 / 420. The ruling
says "on the final constants"; one sentence in the record would explain this.

## Gates run (export of `04fc3cc8f`)

- `cargo fmt --all -- --check`: clean.
- `cargo clippy --locked -p math -p builtins -p builtins-compiler --all-targets --features
  builtins/test-support -- -D warnings` and `-p math --features lane --all-targets`: clean.
- `cargo test --locked -p math`: all pass (lib 7, integration 3 and 8 with 2 ignored, doctests 0).
- `cargo test --locked -p builtins --features test-support --lib`: 15 passed;
  `-p builtins-compiler --features test-support --lib`: 57 passed.
- `cargo test --locked --release -p builtins --features builtins/test-support --test
  tail_contract`: 13 passed.
- Gate 8 (`every_sixty_four_track_console_document_is_bounded_exactly_at_every_launch_rate`):
  passed, figures as above.
- `scripts/check-workspace-policy.sh`: ok. `scripts/check-realtime-policy.sh`: ok.
- `scripts/check-cross-targets.sh`: exit 0, PASS (only the expected #1018 rows).
- Gate 1 dump (16,228 cascades): identical. Gate 2 diagnosis probe: reproduced. Gate 5: 13
  mutations as above. One descriptive gate-2 invocation (load recorded).
- Paths: only authorized files changed. No render change and no queue: the acked-batch question
  does not apply. New names (`TAU`, `SQUARE_SAFE`, the tests) are unversioned.

Note for root: the worktree has uncommitted changes to `scripts/check-cross-targets.sh` and
`scripts/lib/aarch64-known-defects.py`, written at 06:56-06:57 while I reviewed. They are not part
of these commits. They look like #1472's in-progress work.
