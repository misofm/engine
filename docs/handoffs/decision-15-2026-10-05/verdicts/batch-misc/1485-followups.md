PASS

# #1485 attempt 1 follow-ups: adversarial verdict

Commit under review: `a90fa33ec` (parent `870a61436`). Worktree `/home/bl/misofm/wt-d15-misc`, not
touched. Exported with `git archive` into `/tmp/claude-1002/v1485fu/tree`,
`CARGO_TARGET_DIR=/tmp/claude-1002/v1485fu/target`, `CARGO_INCREMENTAL=0`. Verifier: opus-xhigh,
2026-10-08. Evidence logs: `/tmp/claude-1002/v1485fu/evidence/`.

No BLOCKER and no MAJOR. One MINOR and one NIT. The follow-ups do what root's ruling 2 and the
attempt-1 verdict ask: gate 1 has the worst-sign history at every rate, `F_p = 23.94` is root's
formula applied to gate 1's measured `r_p,max`, gate 2 is still red on MP, both corrected
test-value claims are true, no library code moved, and the derivation's Gates section is accurate.
But the worst-sign history is not the largest real-kernel peak. A three-event history beats it by
5.3 % at every rate, so `g_meas`, `r_p`, `F_p` and #1487's figures are again too high (MINOR 1).

## Checks

### Gate 1 has the worst-sign history at every rate

`live_peak_gain_bounds_the_real_kernel_on_its_worst_histories` runs `worst_sign_history` inside the
`for &rate in rates()` loop, after the five Nyquist-drive histories, and folds its peak into
`largest` (`crates/builtins/tests/tail_contract.rs:3154-3174`). In release it runs at all four
launch rates. In debug, `rates()` is 48 kHz only, as for every other history. I read the
construction:

- `sign_history_frames` records the kernel's words one frame per block with zero input, from the
  worst-case pair, events applied before their frames. `sign_row` is the adjoint recursion of
  #1467's oracle (`h(n, m) = c_n A_(n-1) ... A_(m+1) b_m`, `h(n, n) = d_n`). The index arithmetic is
  correct.
- `peak_run` drives the real kernel with `sign h(n, m)` (blocks of 64, then blocks of 1 from 128
  frames before the lead). The assertion `peak >= exact (1 - 10^-3)` proves that the recorded words
  and the drive agree. The measured shortfall is 2.5e-4 to 2.6e-4.

Release figures (`evidence/tc-release.log`), the same as the record to the last printed digit:

| rate | exact x trim / kernel peak | `g_meas` | `r_p` raw / stated |
|---|---|---|---|
| 44.1 kHz | 5.678090e4 / 5.676644e4 | 5.676644e4 | 22.7968 / 22.7986 |
| 48 kHz | 5.645027e4 / 5.643538e4 | 5.643538e4 | 22.6871 / 22.6960 |
| 88.2 kHz | 5.678101e4 / 5.676656e4 | 5.676656e4 | 22.7966 / 22.7986 |
| 96 kHz | 5.645033e4 / 5.643544e4 | 5.643544e4 | 22.6870 / 22.6959 |

`F_p`: `r_p,max` (stated) is 22.7986. `22.7986 x 1.05 = 23.9385`, rounded up `23.94`. This is
root's formula, and the value is below 39.44.

### `g_meas` is not the largest measured peak (see MINOR 1)

I added a search test to my export copy (not committed, file restored by SHA-256). It uses the
same oracle and the same kernel drive as `worst_sign_history`. It ranks histories by the exact row
`l1` x trim, then confirms the best history on the real `f32` kernel.

- At 44.1 kHz: a grid of 500 two-event histories (HPF to 10-15 Hz, LPF to 10-15 Hz, 8-26 frames
  apart, both orders), 96 rule-4 chains, 171 other lead pairs, and 240 random histories with 2 to
  4 events. Nothing beats 5.678090e4. Other lead pairs are much lower (HPF at 0.995 max gives
  2.35e4).
- At 48, 88.2 and 96 kHz: the random search (seed 7) found
  `[(0, HPF -> ~0.8 max), (22, LPF -> 10 Hz), (24, HPF -> ~600 Hz)]`, which beats `g_meas` at those
  three rates.
- A focused search of this family (`evidence/search-fam.log`, `search-refine.log`) climbs to the
  history *HPF to 0.9 max at +0, LPF to 10 Hz at +36, HPF to 10 Hz at +39*. Its row is at +42.
  The search was still rising at the edge of its grid.

Confirmed on the real kernel with a 700,000-frame lead (`evidence/beat-gmeas.log`):

| rate | exact x trim | kernel peak | over `g_meas` | `r_p` stated |
|---|---|---|---|---|
| 44.1 kHz | 5.976576e4 | 5.974981e4 | x1.0526 (+0.44 dB) | 21.660 |
| 48 kHz | 5.941791e4 | 5.940206e4 | x1.0526 (+0.45 dB) | 21.562 |
| 88.2 kHz | 5.976595e4 | 5.974994e4 | x1.0526 (+0.44 dB) | 21.660 |
| 96 kHz | 5.941805e4 | 5.940219e4 | x1.0526 (+0.45 dB) | 21.562 |

Soundness is not affected. `g_p` is still at least 21.5 times this peak.

### Gate 2 is still red on MP

MP (`g_p = (relative.outputs[0] + delta^2 g SLACK) SLACK`, #1466's `W_0` bound, in
`LiveBound::composition` only): red on gate 2 (`44100 Hz gate 2: the stated g_p
1.6292960326397214e7 exceeds F_p g_meas = 1.3589885137500002e6 (ratio 287.02)`), on the table test
and on L3. The other 19 tests are green, and all six gate 1 histories pass at 44.1 kHz before
gate 2 fires (`evidence/MP.log`). This is the same as the record.

### The two corrected test-value claims are true, and both are unique catches

Each mutant was run on the whole release `tail_contract`. Each file was restored and checked by
SHA-256, and a green full run followed.

| mutant | defect, as the spec states it | red | green |
|---|---|---|---|
| G1Z | `peak_gain = self.tail_gain(gain)` in `LiveBound::composition`, and `peak_gain: tail_gain` in L3's `LiveOracle` | gate 1 (`44100 Hz L1, HPF to 10 Hz: the real kernel's peak 3.4455813e4 exceeds g_p + sigma_p = 8.157735689623506e2`), the table test | the other 20, L3, L2 and L5' included (`evidence/G1Z.log`) |
| G1T | `self.tail_gain(1.0)` in `composition`, and `gain *` removed from L3's `tail_window` and `tail_settled` | L2 at every rate (largest supremum against `g_t`: 55.93 / 51.52, 55.59 / 51.58, 53.45 / 51.82, 53.12 / 51.88), the table test | the other 20, L3' and L5' included (`evidence/G1T.log`) |

Both claims in the spec's "Test value" section are true as written. The table pin is the only other
red test, and the spec names that exception. The other test binaries that read the computed bound
(`builtins-compiler` `mod tests` near `:12044`, which compares it with the strip's table-read
bound, and `host-core/tests/live_lanes.rs`, which reads only `.tail`) are table pins of the same
kind, or they cannot see a gain.

### #1487's local figures

The changed figures agree with the record: `r_p` 22.80 / 22.70 / 22.80 / 22.70 (+27.2 dB; 20
log10 22.7986 = 27.16), `g_meas` 5.6766e4, gate 1's "six histories", `F_p = 23.94`, and "the
measured 3.58e3" per unit of the trim word (5.676644e4 / 15.848932 = 3.582e3). No old figure is
left in the spec. But MINOR 1 makes all four too high.

### No library code moved (V-D4)

`git diff 870a61436 a90fa33ec -- crates/*/src hosts tools` is empty. The commit touches only the
#1485 spec, the #1487 spec (root's ruling 2 orders this), `tail_contract.rs` and the derivation.
SHA-256 of `crates/math/src/tail.rs` and `crates/builtins/src/tail.rs` in the export is the same
before and after every mutant.

### The derivation's Gates section

`docs/derivations/1379-graph-tail-composition.md:763-788` now names gate 1's six histories (five
after a Nyquist drive, one driven by the worst-sign input), gate 2 with both halves (`g_meas <=
g_p`, stated `G_p <= F_p g_meas`, `g_meas` the largest measured peak) and gate 2t with both halves.
This agrees with the code (`tail_contract.rs:3189-3200` and `l2_at_rate`). The three over-long
lines from attempt 1 (570, 726 and 766 at the parent) are wrapped. The six lines over 100
characters that remain were there before. The "Looseness" paragraph (+95.08 dB, 27.1 to 27.2 dB,
22.7 to 22.8 times) agrees with the record, but MINOR 1 applies to it. Line 472 ("gate 1's L1
history reaches +90.75 dB") is still true of L1.

## Findings

**MINOR 1. The worst-sign history is not the largest real-kernel peak. A three-event history beats
`g_meas` by 5.26 % at every launch rate, so `r_p`, `F_p`, the record's looseness and #1487's
figures are too high.**
`crates/builtins/tests/tail_contract.rs:3099-3104` and `:3154-3158`; the spec's follow-ups
"*Which histories.*" paragraph; #1487's Context.
- The record says: "At every rate the largest row is the verifier's history ... So one history
  is enough at every rate." The test's doc says: "It is the largest peak found at every launch
  rate." The record's 120-history grid only moved the HPF to 10 or 100 Hz first. It never moved
  the HPF to a high cutoff first, and it never tried three events.
- HPF to 0.9 max at +0, LPF to 10 Hz at +36, then HPF to 10 Hz at +39 (row +42) reaches
  5.974981e4 at 44.1 kHz on the real kernel (5.940206e4 / 5.974994e4 / 5.940219e4 at the other
  rates). This is a lower bound: the search was still rising at its grid edge.
- Results: `r_p,max <= 21.660` (not 22.7986). Root's formula would give `F_p <= 22.75` (not 23.94),
  so the committed `F_p` has at least 10.5 % margin, not 5 %. The remaining looseness is at most
  26.7 dB (not 27.2 dB). #1487's figures become at most 21.66 / 21.56 / 21.66 / 21.56, at least
  5.975e4 and at least 3.77e3 per unit of the trim word.
- Soundness and gate 2's catch of MP are not affected (MP's ratio is still about 270).
- Fix (root's choice): `worst_sign_history` already takes these events, and `SIGN_SEARCH = 16`
  covers row +42 (the last event + 3). So the history is one more call in gate 1. Then restate
  `g_meas`, `r_p`, `F_p` (by root's formula) and #1487's Context, and correct the record's "one
  history is enough" claim. Or record the history and the restatement as #1487's starting point,
  because #1487's gate 1 reads "widened with any history the diagnosis finds worse". In both cases,
  the record must stop saying that the two-event history is the largest one at every rate.

**NIT 1 (observation, no change needed).** G1Z's and G1T's unique catches belong to the test
functions, not to the per-history assertions alone. In the same function, gate 2's lower half
(`g_meas <= g_p`) asserts more strictly than gate 1's `peak <= g_p + sigma_p`, and gate 2t's
`s_scan <= g_t` asserts more strictly than L2's per-history check. The spec defines these lower
halves, so the claims are true at the test level, and the tests carry the catch.

## Test value

- Gate 1 (G1Z): a `G_p` derivation that drops the pre-retarget state, coherent in the module and in
  L3, is red only in `live_peak_gain_bounds_the_real_kernel_on_its_worst_histories` (besides the
  table pin).
- Gate 1t / L2 (G1T): a `G_t` derivation that drops the trim word, coherent in the module and in
  L3', is red only in `live_tail_gain_covers_the_exact_supremum_of_a_late_input_over_the_scanned_ramps`
  (besides the table pin), at every rate.
- The worst-sign history (new code in gate 1): no unique catch, as the spec says. It sets gate 2's
  reference. Without it, gate 2's upper half reads `r_p` 37.56 and is red against `F_p = 23.94`.
  Root's ruling 2 ordered it.
- Gate 2 (MP): still red only on gate 2 among the gates (plus the table pin and L3). This is the
  same as attempt 1.

## Gates run (export of `a90fa33ec`)

- `cargo test --locked --release -p builtins --features builtins/test-support --test tail_contract`:
  22 passed, 11.40 s (`evidence/tc-release.log`).
- `cargo test --locked -p builtins --features builtins/test-support --test tail_contract` (debug):
  20 passed, 2 ignored (release scale: #1329 gate 2 and #1467 L2), 63.46 s on this 32-core host
  (`evidence/tc-debug.log`). The worst-sign history runs at 48 kHz with the same figures as
  release. The required debug jobs (`test-debug-b`, 15 min; `aarch64-debug`, native
  `ubuntu-24.04-arm`, 30 min) keep their headroom.
- Mutants G1Z, G1T and MP, as above.
- `cargo clippy --locked -p builtins -p math --all-targets --features
  builtins/test-support,math/lane -- -D warnings`: clean. `cargo fmt --all -- --check`: clean.
  `scripts/check-workspace-policy.sh`: ok.
- Not rerun: `audit capi`, the fixtures, the wasm gates and the cross-target check. No library,
  render or CI file changed (V-D4 above).
- The acked-batch question does not apply: no queue is changed.
