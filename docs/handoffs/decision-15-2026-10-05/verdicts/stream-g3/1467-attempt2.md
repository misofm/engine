PASS

# #1467 attempt 2 (slice B2): derive the live input section's tail gain and state its composition

Commits under review: `8d994fa55` (the derivation, first) and `12cebe828` (the code and the spec),
parent `a1a742d36`, branch `codex/d15-stream-g3`. Attempt 1 was `ae054b277` (FAIL, MAJOR 1: L2's
scan coverage). Verifier: opus-xhigh, 2026-10-08. Exported with `git archive 12cebe828` into
`/tmp/claude-1002/v1467a2/tree`, `CARGO_TARGET_DIR=/tmp/claude-1002/v1467a2/target`. The worktree
was not touched. I judged the whole slice. The proof passed in attempt 1, so I re-checked only the
parts that changed.

Attempt 1's MAJOR is fixed. L2 now reads the real kernel's words frame by frame. Its stated
coverage is true, and I re-counted it. Each recorded mutant gives the recorded result. Every gate
passes. There is one MINOR: a parenthetical gives the wrong owner for the rounding deviation. It
does not overclaim coverage. There are four NITs.

## MINOR 1: "the rounding deviation (B1's part)" gives the deviation to the wrong owner

- Where: spec `:507` ("... the trim ramp, or the rounding deviation (B1's part); it claims no
  more") and `crates/builtins/tests/tail_contract.rs:3504` ("not the rounding deviation, which is
  B1's part"). Attempt 1 said only "the rounding deviation". Attempt 2 added the owner.
- Why it is wrong: the derivation's (P3) splits each relative rounding perturbation in proportion
  to the parts. The late part keeps its own share, `||e^late|| <= mu ||s^late|| + mu_x |v^late|`, and
  `g_t >= sup |y_late| / eps` bounds it. In G_t's terms, `mu` is inside `rho_k`, `r_k` and
  `rho_ramp`, `mu_x` is inside `beta_k` and `iota`, and `omega_x` is inside `delta`. The contract
  (spec `:37-40`) says the same: G_t bounds "the reference's response to that part, plus the
  rounding deviation it drives". Only the flush part (`F`, sigma_t) and the early part belong to
  B1.
- What is true: L2 does not cover the rounding deviation, because its oracle is exact
  arithmetic. The kernel check samples that deviation on six histories (ratio within 2e-5).
- Why MINOR, not MAJOR: no value is wrong, and the record does not claim coverage that it does not
  have. But #1485 tightens G_t next. If it reads "B1's part", it can remove G_t's own rounding
  terms.
- Fix: one phrase in each place, for example: "the rounding deviation (G_t's derivation bounds the
  late input's relative rounding; the flush part is B1's sigma_t)".

## NITs

1. **The stated maximizers are grid points.** Derivation `:653-656` and record `:478-481`. My
   search used 601 log-spaced points plus golden-section refinement, with f64 impulse responses of
   the designed f32 words over 400k frames. The settled `l1` of an HPF into the top LPF has a flat
   maximum at about `f = 0.00342 fs`:

   | rate | HPF at the maximum | `l1` |
   |---|---|---|
   | 44.1 kHz | 150.7 Hz | 3.79801 |
   | 48 kHz | 164.1 Hz | 3.79766 |
   | 88.2 kHz | 301.4 Hz | 3.79801 |
   | 96 kHz | 328.1 Hz | 3.79766 |

   The record's frequencies (147.98 / 173.54 / 286.93 / 341.86 Hz) are 61-point grid points
   within 1.1e-4 of these maxima. At 286.93 Hz (88.2 kHz) the value is 3.79792, so it rounds to
   3.7979, not the stated 3.7980. "About 3.798, +35.59 dB with the trim" is true, and no bound
   depends on the frequencies.
2. **K3's reason needs one more premise.** Spec `:457` and `:531`, and `tail_contract.rs:3051-3054`.
   - The reason given: an identity section's state does not move and does not reach the output.
     This is true, but it also needs the cleared section to stay the identity to the end of the
     history.
   - In this scan it does. Every event comes before `N`, and every disable completes at `N + 1` or
     later, so no event comes after a clear.
   - In a scan with an enable after a completed disable, the clear would matter, because rule 3
     requires zero integrators.
   - Fix: state the premise.
3. **Wording of (c).** Record `:493-494` says "`a != b`, `c = b` a re-send (rule 1)". This can read
   as "`c = b` only". The code (`tail_contract.rs:3440-3446`) and the count (800 = 2 x 5 x 4 x 5 x
   4) take `c` over all five cutoffs. "`c` any of the five (`c = b` a re-send, rule 1)" is exact.
4. **`nan_max` (verdict NIT 2).** `crates/math/src/tail.rs:1384-1392, :1676, :1683, :1691`.
   - Attempt 1's verdict said no change was needed. The change is correct anyway: a NaN in `phi`
     reaches `state_output`, then `first`, then `cap`, which sets `capped`.
   - It is value-neutral: the table test, L3' and L5' do not change.
   - No test reaches the NaN branch, and the record says so. Production reads
     `input_section_live_bound_table`, so the cost is not on any production path.
   - No action needed.

## MAJOR 1 of attempt 1: resolved

**(a) Rules 2 and 3 are now the kernel's.** `record_history` (`tail_contract.rs:3055-3094`) does
this for each frame:
- It builds a real `InputBuiltins` at the start pair.
- It applies each event with `apply_prepared_filter` before the event's frame.
- It reads `input_section_words` before the frame is processed, with zero input throughout.

So the oracle gets #1407's law as `crates/builtins/src/lib.rs:1426-1560` implements it. I
confirmed the frame timing with a probe at 48 kHz:

| history | words recorded from `N` |
|---|---|
| rule 4, 10 to 100 Hz, before `N - 1` | `c1` weight 1/64 at `N`, target at `N + 63`, `fixed_from` 63 |
| rule 4, before `N - 63` | weight 63/64 at `N`, target at `N + 1` |
| rule 2, 100 Hz disabled before `N - 1` | `c1` frozen at the design from `N` to `N + 62`, `m1` weights 1/64 to 63/64, identity and clear at `N + 63` |
| rule 3, 100 Hz enabled before `N - 1` | `c1` at the design at `N`, `m1` weight 1/64 |

The kernel's own comment agrees (`crates/lane/src/kernels/builtins.rs:1144-1145`): current, then
advance; the clear comes after the last old-word sample.

**(b) The event at `N - 1` is now scanned with the kernel's timing.** `apply_retargets` fires
when `frame + before == HISTORY_LEAD`, so the event comes before frame `N - before`, and
`frames[0]` is frame `N`. `s = 1` is the latest event that the contract admits ("no control event
at or after `N`").

**Why the scan covers what it claims:** the words are the kernel's, so no model of #1407 can be
wrong. The kernel check (below) confirms that the per-frame recording equals the words the kernel
runs inside a block.

## The stated coverage: true, and no more than what it covers

I re-counted the record from the code (`l2_scan`, `tail_contract.rs:3376-3479`), with 9 cutoffs
and the HPF below the LPF when both are on:

| part | count | how it is made |
|---|---|---|
| (a) | 1,792 | 256 valid ordered pairs (128 per section) x 7 offsets |
| (b) | 45 | 9 + 8 + C(8,2) settled pairs |
| (c) | 800 | 2 sections x 5 x 4 x 5 x 4 |
| (d) | 576 | 12 x 12 x 4 |
| all | 3,213 | |

Counted by hand per rate:
- 622 histories have a disable that completes after `N`: 210 in (a), 160 in (c) and 252 in (d).
- 622 have an enable from rest, with the same split.
- 3,168 have a retarget in flight at `N` (all minus the 45 settled pairs).

The test prints these same figures at every rate.

Rules covered:
- Rule 1 is only in (c), with `c = b`. The second event is always 62 or fewer frames after the
  first, so the first ramp is still in flight.
- Rule 2 and its completion and clear are in (a), (c) and (d).
- Rule 3 is in every history whose section starts at the identity. The section starts settled,
  with zero integrators.
- Rule 4 starts from settled words in (a) and (d). It also starts from in-flight words in (c),
  including after a frozen disable (`b = 0`, `c != 0`) and after a rule-3 enable.

The "does not cover" list is exact for what it names. "Those histories only" bounds the rest, for
example three events on one section, mono collapse, bank lanes, or a restored non-zero identity.

Results (my run; the same as the record):

| rate | largest `trim x sup` | margin to `g_t` | largest remainder |
|---|---|---|---|
| 44.1 kHz | 60.1642 (+35.59 dB) | 56.83 dB | 1.1e-12 |
| 48 kHz | 60.1721 (+35.59 dB) | 56.83 dB | 1.1e-12 |
| 88.2 kHz | 60.1204 (+35.58 dB) | 56.84 dB | 1.1e-12 |
| 96 kHz | 60.1203 (+35.58 dB) | 56.84 dB | 1.2e-12 |

`g_t` is 4.178304e4 (9,242 mB) at every rate. The labels of the worst histories are also the same
as the record.

**The oracle.** I re-read `exact_row_supremum` (`:3243-3302`):
- Inside the changing frames, the backward rows are `h(n, m) = c_n A_(n-1) .. A_(m+1) b_m`.
- After them, the columns are the states at `N + fixed_from`, then `c A^s v_m`, plus the settled
  prefix `l1` (`prefix[0] = |d|`).
- The window's sup remainder and the settled sum's remainder come from `free_response_bounds`. I
  re-derived both bounds; they are correct for a cascade.
- The value after the computed frames is `total + remainders`.
- `history_frames` folds a clear into frame `j - 1`, on the cleared section's rows of `A` and `b`.
  That is the correct linear model.

## K3, the clear folding (equivalent): the reason is correct for the scan

Identity words give `A = I` and `b = 0` for that section's state, an output row of 0, and `d = 1`.
The state is frozen and the output cannot see it. A clear happens only at a disable's completion,
which is at `N + 1` or later. No event comes after `N`, so a cleared section stays the identity to
the end of the history (see NIT 2 for the missing premise).

I re-ran K3: the clears were removed from both `history_frames` and `forward_response`. The test is
green, and every printed L2 line is the same to its printed precision.

## The new kernel check: correct and discriminating

- The oracle's row at `N + 40` and `N + 200` equals the forward impulse sum to 1e-12 on all six
  histories at every rate. The forward sum runs the kernel's equations per impulse and shares no
  code with the adjoint.
- The real f32 kernel uses trim +24 dB and `2^-12 sign h(n, m)` from `N`, in one block. Its
  `|y(N+n)|` divided by `trim 2^-12 row` gives ratios from 0.999997 to 1.000020 at every rate, the
  same as the record. The 1e-3 tolerance is 50 times the measured deviation.
- `drive_to_n` applies the events through the same `apply_retargets` as the recording, so the
  check also confirms that one-frame blocks and in-block frames load the same words.

## Mutation runs (re-done in the export; release `tail_contract` with `test-support`; files restored)

| mutant | red | green |
|---|---|---|
| M2 trim and window dropped, mirrored in L3' (all tests but the table test) | L2 only: `g_t` 15.867; first failure 55.93 (44.1 kHz) | the other 20 |
| O1 the oracle's window part zeroed after the changing frames | L2 at the forward-sum check, `N + 200`: 2.1796 against 2.1934 (44.1 kHz) | - |
| K1 a single retarget modelled as the all-six f64 mixture with the kernel's timing | L2 kernel check, HPF disable at `N + 40`: kernel 4.750e-3, oracle 4.520e-3 (44.1 kHz); the rule-4 check passes | the soundness scan, which runs before the check and completes |
| K1b only an enable from rest modelled as the mixture | L2 kernel check, HPF enable at `N + 40`: 5.439e-3 against 4.448e-3 | the soundness scan |
| K2 words read after the frame is processed | L2 kernel check, retarget at `N + 40`: 4.953e-3 against 4.989e-3 | the soundness scan |
| K3 clears not folded (both places) | none: equivalent | all; the L2 lines are the same |
| M1 (attempt 1's) window part dropped, trim kept, not mirrored | L3' (2.5126e2 against 4.1745e4) and the table test | L2 (+48.0 dB is more than +35.6 dB) and the rest |

Every result matches the record to the printed digits. K1 is red at the disable, and its rule-4
history passes. So a disable-only mixture would fail at the same check, which supports the
test-value text "a disable or an enable modelled as the all-six mixture".

**The test-value text is true** (spec `:159-165`). M2 is still L2's unique catch. No rule-2, rule-3
or timing modelling mutant makes the soundness assertion red, because the margin is 56.8 dB. Only
the kernel check catches them, and the spec says exactly that. M1 still shows that L3' is the only
value gate for a dropped window term, as the amendment requires.

## Test value (rewritten test)

- **L2** `live_tail_gain_covers_the_exact_supremum_of_a_late_input_over_the_scanned_ramps`: red for
  a derivation error that puts `G_t` below the trim times the kernel's exact late-input supremum
  and is mirrored into L3' (M2). No other test sees this. Through its self-checks, L2 is also red
  for an oracle whose words or timing are not the kernel's (K1, K1b, K2) and for a wrong window
  part (O1).

## Independent comparison of the largest `trim x sup` (about 60.17)

- **Attempt 1's verifier oracle** (`/tmp/claude-1002/v1467/verifier_1467.rs`, log
  `verifier-scan.log`) was written separately: forward impulse columns over kernel-recorded words,
  about 3,960 histories per rate, with other endpoints and random restarts. It found 60.164 /
  60.172 / 60.120 / 60.120. Attempt 2's adjoint gives 60.1642 / 60.1721 / 60.1204 / 60.1203, the
  same to 5 significant digits.
- **My settled fine grid** (above) gives a maximum of `trim x l1` = 60.194 (+35.591 dB). That is
  0.003 dB above the scan, as the record says ("L2's cutoffs step over the fine grid's settled
  maximum ... 0.003 dB"), and 56.83 dB below `g_t`. The 10 Hz pair gives 3.52867 / 3.50766 /
  3.37272 / 3.35158, the same as the record.

## Attempt 1's MINOR and NITs

| item | status |
|---|---|
| MINOR 1 | Fixed. Spec `:381-384` corrects the sentence in place, with a note. The claim is true: `input_bound_budget.rs:1177-1182` refuses any argument other than `calibrate` and exits with code 2. |
| NIT 1 | Fixed. The derivation is in `8d994fa55`, committed before `12cebe828`. |
| NIT 2 | Done, although it was optional (NIT 4 above). |
| NIT 3 | Fixed. The comment is at `crates/builtins/src/tail.rs:527`. |
| NIT 4 | Fixed: the derivation now names the largest settled pair (about 3.798, +35.59 dB). NIT 1 above records the precision of the stated frequencies. |

## Other checks

- **Paths.** The two commits touch only authorized paths: the spec, `math/src/tail.rs`,
  `builtins/src/tail.rs`, `tail_contract.rs` and the derivation.
- **Spec and code.** The `mixture` function and its false doc comment are deleted, and nothing
  uses them. The spec's L2 gate text and test value are updated. Root's amendment text is left
  unchanged.
- **AGENTS.md.**
  - No new name has a version suffix: `KernelHistory`, `record_history`, `SettledSum`, `nan_max`.
  - No test greps source or prose. No byte pin is added.
  - There is no queue, so the acked-batch question does not apply.
  - No render code changes. `nan_max` is control-plane only.
- **No rendered bit moves.** `audit capi` gives `pcm_digest cb10fbface44a3a4`, the same as
  attempt 1.
- **Shipped wasm module.** Its hash changes (8750dbf0... against attempt 1's ba197906...) because
  the math code changed. It has the same size, 3,128,767 B. The browser-correctness `expected.json`
  digests and rows agree, and the budgets do not change.

## Gates run (export of `12cebe828`)

| gate | result |
|---|---|
| `cargo test --locked --release -p builtins --features builtins/test-support --test tail_contract` | 22 passed (10.0 s); the L2 figures are as in the record |
| the same in debug | 20 passed, 2 ignored (release scale) |
| `cargo test --locked -p math` | all passed |
| `cargo test --locked -p builtins --features builtins/test-support --lib` | 16 passed |
| `cargo fmt --all -- --check` | clean |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | clean (232 crates compiled or checked) |
| `check-workspace-policy.sh` | ok |
| `check-realtime-policy.sh` | ok (89 regions, 25 files) |
| `check-cross-targets.sh` | PASS: x86-64-v3; aarch64 iOS and Android checked and linted; the #1018 expected failures; wasm simd128 |
| worklet chain: `build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh --without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts`, `test-web-audioworklet.sh` (private TMPDIR, left empty) | all exit 0; source budget 3,358 of 3,648 |
| `audit capi` | 100,000 calls; 0 allocations, deallocations, locks and syscalls; `pcm_digest cb10fbface44a3a4` |
| not re-run | gate 2, gate 8 and the builtins fixtures, because no render code or fixed-walk code changed and production reads the live table. The audit digest confirms that no PCM changed. |

Evidence kept in `/tmp/claude-1002/v1467a2/`:
- `run-tail.log` and the other gate logs;
- `mut/`, with each mutant's log, `README.txt` and `k1.py`;
- `verifier-probes.txt`, with the timing probe and the settled-grid probe.
