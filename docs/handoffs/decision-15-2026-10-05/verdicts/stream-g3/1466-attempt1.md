PASS

# #1466 attempt 1 (slice B1): certify the live input section's decay, peak gain and both stalls

Commit under review: `5ea3149ec` (parent `3954dce33`), branch `codex/d15-stream-g3`. The earlier
stop-note commit `d12fd3940` holds only evidence. Verifier: opus-xhigh, 2026-10-08. Exported with
`git archive` into `/tmp/claude-1002/v1466/tree`, `CARGO_TARGET_DIR=/tmp/claude-1002/v1466/target`.
The worktree was not touched.

No BLOCKER and no MAJOR. The proof holds when checked as a proof. An independent brute force over
admitted live histories found no counterexample. Its margins are at least 52.3 dB on (N1) and about
111 dB on (N2) at `T`, and the margin does not shrink over `k = 0..12`. Every gate passes. All nine
recorded mutants reproduce exactly.

There are three MINORs:

- a gap that lets L4 pass without checking anything;
- the false L4 sentence, still in the spec text (for root);
- stale spec text after root's ruling and the corrected L6 (for root).

There are also six NITs.

## The proof, checked as a proof

The derivation is `docs/derivations/1379-graph-tail-composition.md`, "The live input section". I
read it against the code (`LiveBound::window`, `settled_system`, `settled_start`, `stall`,
`composition`) and against #1433's own derivation (`docs/derivations/1329-input-section-tail-and-rest.md`,
"#1433").

**(P1) Split by drive: sound.**

- Every inequality that #1433 propagates is a non-negative linear recursion in its drives
  `(g X, F)`:
  - the state step `E' <= rho_k E + beta_k |x| + F`, with the kernel's relative rounding folded
    into `rho_k` and `beta_k`;
  - the first section's output;
  - the second section's weighted sums;
  - the potential's telescoping and its Abel summation;
  - the settled system, including the joint flush's one-off `X` and its constant.
- Two majorant runs with the same zones and the same reset instants work as follows. Induction
  gives `E <= e^x + e^F`, because a reset makes the kernel's state 0, which is at most anything.
  Each run satisfies #1433's bounds with its own drive. `Phi` is already two separately computed
  least solutions in #1433 (`state[k]`, `flush[k]`).
- **The minimum.** `settled_start`'s
  `H = min(window.energy, Phi . (g, f))` is taken per run. Split is at most joint, because
  `min(a1, a2) + min(b1, b2) <= min(a1 + b1, a2 + b2)`. So the split needs the separate-majorant
  argument, and the note gives it. I confirmed the note's count by instrumenting the export: 21
  groups at every rate mix the two choices (60/61/62/63 groups).
- **The cap.** A capped value bounds the kernel, not a run. `LiveWindow::capped` is set at every
  site that applied the cap before:
  - each frame's `sigma`;
  - the final state;
  - the final energy.

  `composition` refuses when either window took the cap
  (`crates/math/src/tail.rs:1814`). The other place that reads the cap is
  `settled_start`'s `.min(self.cap)` on `H`. That is a no-op when the window energy is uncapped.
  The relative window's largest state is 1.07e7 (measured), against a cap of 6.3e38.
- The `capped` refactor moves no value. The `state` closure lost its `.min(cap)`, and every call
  site is wrapped in `cap(...)`. The live table test (table == computed) is green.

**(P2) The window's first frame bounds every frame: sound. This is the key claim, and I tried to
break it.**

- `W_0` reads only these quantities:
  - `sup Phi`, which holds at every frame of every admitted history;
  - the second section's state at `N`;
  - the first section's output coefficient at its supremum over every word.
- The second section's state at `N` is made of these terms:
  - the potential term `V = sup Psi Phi`, by Abel summation with the non-decreasing weights
    `rho^(N-1-j)`;
  - the charges `K`;
  - the direct-zone term `K_L`;
  - the feedthrough `D`;
  - the rounding `Omega`;
  - `F_sum`.
- Each of those bounds holds for a history of any length and any start. Abel summation gives
  `w_0 a_0 + sum (w_j - w_{j-1}) a_j - w_J a_{J+1} <= sup a`. A direct zone also satisfies the
  telescoping inequality with left side 0, because `Psi_k >= rho_k Psi_i`.
- None of these bounds uses silence or the absence of a control event at or after `N`. The prefix
  of an admitted history is an admitted history. So `W_0` bounds the states at every frame.
- I checked each way the claim could fail. None does:
  - **A ramp in flight at frame `n`:** the pre-`N` analysis never assumed the ramp was complete.
  - **The trim ramp and polarity:** the trim stays inside `|trim| <= trim_max`
    (#1408, and `a_polarity_flip_crosses_zero...` is green).
  - **Disable and enable:** these are #1433's moves with zero state.
  - **The mono collapse's channel copy:** the copy carries state and words together, so the
    copied channel's history is the other channel's history.
  - **The joint flush and the per-block recovery:** these are resets, which only lower the state.
  - **Pumping by switching between words:** every reachable word contracts in the one `V`-norm, so
    switching cannot pump the state.

**(N1): sound.**

- At frame `n`, the current input adds `delta |x'|` through the first section's feedthrough, then
  `delta` through the second's. `delta` is the envelope's `output_input` for both sections, and it
  includes `omega_input`.
- This gives `X (W_0(g, 0) + delta^2 g) + W_0(0, F)`. The code computes
  `(outputs[0] + delta^2 g SLACK) SLACK`, with `g = trim_max (1 + u)`.
- `sigma_p = stall(0)`, which is at least `outputs[0]` of the flush window. Measured,
  `stall(0) / W_0(0, F)` is 1.0016 / 1.0015 / 1.0008 / 1.0008, which matches the note's
  0.08-0.16 %.
- The live bound folds the `f32` deviation into its constants. So no separate `dev_loud` term is
  needed.

**(N2): sound.**

- `sigma_t = min(stall(T), stall(0))`. Both bound the flush part from `M + T` on, so rule (g)
  holds by construction.
- The relative system on `(tau, H, X)` with `f = 0` is the top-left `3 x 3` block, because column
  3 is zero and the row's component 3 is zero.
- The order is `H, tau, X`. `v_H = z_H`. `v_tau = max(z_tau, a_H v_H / (lambda - rho_s))`.
  `v_X = z_X`.
- I re-derived the carry's rounding for `n = 3`:
  - at most 1,260 rounding sites, which is below `2^11`;
  - `r < 2^61` (diagonal below `2^29`, off-diagonal at most `8 * 2^58`);
  - `2^11 * 2^-1075 * (3.45 * 2^254 + 1.07 * 2^61) = 2^-808.2`, which is below `2^-807`. That is
    far below `tau = 2^-600`.
  - The verification margin is as for the fixed design (`lambda >= rho_s`, `v_p >= tau`).
- `T >= 65` at every launch rate. `T_lambda - T = 0` at every rate, so every group's `a` is below
  0, and `D = D_inf` (the L4 output agrees).
- `D = max_c D_c` covers the unknown HPF zone. `T + k D >= T + j_k^c` holds per group, by the
  fixed design's proof.

The descriptive numbers I checked agree with the note: the window's largest relative state, the
lambda values, the group counts, `46,099` against `45,220`, and the table of values (bit-exact in
my run). The note's rounding paragraph and its cap paragraph agree with the code.

## Independent brute force (`f64`, admitted live histories)

The probe was an uncommitted `crates/builtins/tests/probe_1466.rs`, in the export only. It is kept
at `/tmp/claude-1002/v1466/probe_1466.rs`, with the mutation driver `mutate.py` and the run logs.

**Method.**

- For a fixed control history, the output is linear in the input. So the supremum of `|y[n]|` over
  every input with `|x| <= 1` is exactly the `l1` norm of the time-varying impulse-response row
  `h(n, .)`. I computed it in `f64` by the adjoint recursion.
- The words at each frame were recorded from the real kernel (`input_section_words` after
  one-frame blocks, with retargets applied by `apply_prepared_filter`). I checked the model against
  the real kernel: the error was 2.8e-5 on outputs of about 19, with a one-frame lag.
- The trim is fixed at the +24 dB word. A trim ramp or a polarity flip keeps `|trim| <= trim_max`.
  So the fixed maximum trim covers every trim and polarity history: the input's sign is free.

**(N1) result.**

- I ran 282 histories per rate:
  - every pre pair of HPF {top - 1 ulp, top/2, 1 kHz, 100 Hz, 10 Hz, off} and LPF {top, 1 kHz,
    20 Hz, off}, retargeted at frame 0 to every other valid pair;
  - mid-ramp reversals (HPF top -> 10 Hz -> back at 8/16/32/48/63 -> forward again, and the
    reverse).
- For each history, `n` covered every frame from -2 to 1,500 after the retarget, and every 97th
  frame up to 40,000.
- Worst exact suprema:

  | rate | worst sup | at | `g_p` | margin |
  |---|---|---|---|---|
  | 44.1 kHz | 3.934e4 | `(top-,off)->(10,off)`, frame 6 | 1.629e7 | 52.34 dB |
  | 48 kHz | 3.912e4 | same history, frame 6 | 1.613e7 | 52.30 dB |
  | 88.2 kHz | 3.934e4 | same history, frame 6 | 1.632e7 | 52.35 dB |
  | 96 kHz | 3.913e4 | same history, frame 6 | 1.615e7 | 52.32 dB |

- The worst input over every input is only 14 % above L1's alternating drive (3.4456e4). No
  counterexample. `G_p`'s 52 dB of looseness is #1485's.

**(N2) result.**

- Cases:
  - the fixed top pair, fully charged (input over the whole past);
  - the fixed `(10, top)` pair;
  - HPF `top/2 -> top-`, `10 -> top-` and `top- -> 10`;
  - LPF `20 -> top`;
  - `(off, top) <-> (10, 20)`.
- Each case ran with silence from `M = 1, 65, 5,000, 200,000` frames after the retarget, for
  `k = 0..12`. The supremum over every input before `M` was sampled at 10 frames per `k` in
  `[M + T + kD, M + T + kD + 40,000)`.
- The largest ratio to `(eps/2) 10^-k` was **2.74e-6**, for the fully charged top pair. Per `k` it
  goes 2.0e-6, 2.7e-6, 2.7e-6, 1.9e-6, ..., 8e-7 at `k = 12`. So `D` keeps pace with the real
  decay, and the ratio falls slowly. No counterexample.

## Spec requirements and gates

- **Deliverables.**
  - 1: `math::tail`'s `LiveComposition`, `live_cascade_composition` and `live_cascade_crossing`.
  - 2: the derivation's live part.
  - 3: L1, L3 and L4, with the mutation table.

  Authorized paths: all changed files are authorized. `builtins/src/tail.rs` and `STREAMS.md` are
  unchanged.
- **L1**: green. Real-kernel peak 3.4456e4 / 3.4281e4 / 3.4460e4 / 3.4285e4 against
  `g_p + sigma_p`, ratio 472.8 / 470.5 / 473.5 / 471.2. Reproduced exactly.
- **L3**: green.
  - `D` equal at every rate.
  - Module above the recomputation: `G_p` by 3e-8; `sigma_p` by 1.0e-6 to 2.2e-6; `sigma_t` by
    9e-8 to 8e-7.
  - `sigma_t <= sigma_p` is gated raw and in mB at `tail_contract.rs:1870-1876`. M9 is red there.
- **L4**: green. `max (T(k) - T)/k` is 46,099 / 45,846, against `D` 46,678 / 46,421. `D_inf`
  is above the floor (44,466 / 44,224). See MINOR 1.
- **L5**: green.
  - The release `tail_contract`: 19 passed.
  - `Unstated` is pinned through the table literal (`builtins/src/tail.rs:514`) and the
    table-equals-computed test.
  - `audit capi`: `pcm_digest` `cb10fbface44a3a4`, 0 allocations, 0 deallocations, 0 syscalls.
  - `check-builtins-fixtures.sh`: ok (50 files).
  - `run-wasm-gates.sh` (default form, native + wasm simd128 + V8 spill): ok.
- **L6**: green.
  - Gate 8 passed.
  - Gate 2 (one invocation, `taskset -c 7`, descriptive), with loadavg 2.19 / 4.45 / 4.36 before
    and 2.37 / 4.38 / 4.34 after:
    - exit 0;
    - frame-equivalent needed 16.5 ns (committed 17 ns);
    - worst median design work 24.823 ms, which is 96.7 % of 25.67 ms.
  - The attempt's 98.7 % was measured at a higher 5- and 15-minute load. Neither figure is the
    figure of record; the batch verifier's is.
  - L6 is correctly reduced to a regression run, as root's corrected text provides. The edits to
    `math::tail` touch only the live code, and the fixed walk is unchanged.
- **Mutation requirement** ("at least the mutants below; each is red"): met. The L1 mutant is M1.
  The L3 mutants are M7 and M8. The L4 mutants are M5 and M6. M6 is red at the crossing check, not
  at the floor (MINOR 2).
- **AGENTS.md.**
  - Names are unversioned.
  - The change is control-plane only: no render path, and the live bound is never computed at
    preparation.
  - No digest or byte pin moves.
  - No source-grepping test.
  - No queue, so the acked-batch question does not apply.
  - No interim shortcut: the 53.5 dB looseness of `G_p` has root's successor, #1485.

### Is a deliverable undone with `crates/builtins/src/tail.rs` unchanged?

No.

- Deliverable 1 is "`math::tail`'s live cascade ... with test-support accessors". The design note's
  H8 also says B1 "certifies `D`, `G_p` and the two stalls in `math::tail`'s live cascade ... and
  exposes them through test-support accessors". `live_cascade_composition` and
  `LiveComposition`'s methods meet that. They follow the pattern of A2's `DecayCertificate`
  accessors.
- The authorized path "`crates/builtins/src/tail.rs` (the accessors only)" grants permission. It
  does not make a builtins accessor a requirement.
- The implementer's reason is correct: a builtins accessor would need `lib.rs`. `tail` is a
  private module, and its `pub use` list is in `lib.rs`. `STREAMS.md:367` grants `lib.rs` only to
  #1464.
- The one literal leftover is NIT 1: L-D2 and L-D3 say "rounded up to millibels", and no code in
  B1 does that rounding.

## Findings

### MINOR 1: L4 passes without checking anything if `live_cascade_crossing` is broken

The test is `crates/builtins/tests/tail_contract.rs:2746-2783`; the function is
`crates/math/src/tail.rs:1968-1979`.

- My mutant X5 makes `live_cascade_crossing` ignore `decades`
  (`limit = TAIL_FLOOR / 2`). With it, every `T(k)` equals `T`, and L4 stays **green**. It
  prints `max (T(k) - T) / k 0.0` at every rate.
- L4's claim is "`D` covers the module's own crossings". With a broken crossing function, L4
  checks only `T <= T + kD`.
- The fix is one assertion, for example `T(1) > T`, or `T(k)` strictly increasing in `k`. A
  stronger version is NIT 2's.
- The mutant is plausible: a sign or exponent slip in the evidence function's threshold.

### MINOR 2 (spec text, for root): the false L4 sentence is still in the spec text

The sentence is `.github/ISSUE_SPECS/1466-...md:154-155`:

> a certificate rate that omits the state rounding falls below the floor (as A2's F1(c) and F1(d))

- The sentence is false, and I reproduced why. M6, mirrored, gives `D` = `D_inf` = 45,264 /
  45,032. That is above the floor of 44,466 / 44,224. The mutant is red only at L4's crossing
  check, where `T(1) - T = 46,099` is above 45,264.
- The implementer says so truthfully, but only in the Attempt record (`:386-390`).
- The Test value text itself is unchanged.
- The coordinator rules give the implementer only the Attempt record for a spec problem. So this
  is root's to reword, or to authorize a follow-up to reword. A possible wording: "a certificate
  rate without the settled state rounding (M6) is red at the crossing check; the floor assertion
  is L4's invariant and claims no unique catch".
- The L6 reduction is truthful in the spec text: root's corrected Test value text provides for
  it, and the record applies it.

### MINOR 3 (spec text, for root): stale one-sigma and cost text after root's ruling and the corrected L6

- **One-sigma text.** These places still read one `sigma`:
  - Product outcome, `:11-15` ("three certified composition values ... the flush stall `sigma`",
    "adds the fourth value", "the four values are stated together");
  - L-D3, `:61`;
  - Deliverable 1;
  - L1, `:121` ("`g_p + sigma`");
  - L3, `:123-125` ("`D`, `G_p` and `sigma`").

  Root's ruling in the Attempt record governs, and the code follows it: `sigma_p` in L1, both
  stalls in L3. #1484's MINOR 2 listed the same kind of text in other specs, but not in #1466.
- **Cost text.** These places still describe a live computation that is cached per rate and
  costed at preparation:
  - Context `:25-26`;
  - L-D5 `:65`;
  - the Hazard "Cost" `:111-112`.

  Root's corrected L6 says preparation never computes the live bound.
- Neither of these changes a value or a gate.

### NIT 1 (for root): the millibel rounding of L-D2 and L-D3 is not done by any code in B1

- `LiveComposition` carries linear values. The record's mB table is plain `ceil(2000 log10)`,
  without `ceil_millibels`' margins. No value is within `2^-30` mB of an integer, so the figures are
  right.
- In A2's pattern, the rounding is builtins' `stated_composition`, at statement time. That is B2's
  (#1467).
- Root should make sure that #1467's L-D7 takes this rounding (with `Rounding::Computed`) and both
  stalls. #1484's verdict already flagged that #1467's text still has one stall.

### NIT 2: `LiveComposition::crossing(k)` is read by no test

- The accessor is at `crates/math/src/tail.rs:646`. A mutant that makes it return 0 survives.
- L4 could also assert `T + composition.crossing(k) >= live_cascade_crossing(k)` for `k >= 1`.
  That is the certificate's own per-`k` claim, which is stronger than `T + kD`. It would also close
  MINOR 1.

### NIT 3: the cap flag does not see NaN

At `crates/math/src/tail.rs:1526`:

- `capped |= value >= self.cap` is false for a NaN value.
- `NaN.min(cap)` returns `cap`.
- So a NaN state would continue as the cap, unflagged, and would pass `LIVE_STATE_LIMIT`
  (6.3e38 < 2^132).

This cannot happen at the launch rates. `live_cascade_zones` refuses NaN contractions, but other
envelope fields are not checked. `!(value < self.cap)` would close it.

### NIT 4: the new refusal guards are not exercised by any test

- The guards are the capped window (`:1814`) and the carry limits (`:1850`).
- Mutants X1 and X6, which remove them, survive. Both guards are unreachable at the launch rates.
- A `math` unit test on a synthetic `LiveCascade` would pin them, as #1465 did for its `t0 = 0`
  path. This is optional.

### NIT 5: two wording slips in the derivation note

- `:365-366`, (P2): "each at its supremum `1 / (1 - rho)` times the largest drive" does not
  describe the telescoped potential term `V = sup Psi Phi`. That term is bounded by Abel summation,
  not by a weighted sum of drives. The claim is still true.
- `:4`: "asks every node to state four values" should be five after #1484. The attempt edited this
  paragraph.

### NIT 6: one sentence of the record mixes up its figures

At `:341`, the record says the module is "above it by 3e-8, 1e-6 and 2e-6 relative at most" for
`G_p`, `sigma_p` and `sigma_t`. Measured, the figures are 3e-8 for `G_p`, up to 2.2e-6 for
`sigma_p` (96 kHz) and up to 8e-7 for `sigma_t` (88.2 kHz).

## Mutation runs (redone)

Method:

- Each mutant was applied in the export.
- I ran the release `tail_contract -- live_` (every test that reads the composition).
- I restored the files. After the run, the restored sources are byte-identical to `5ea3149ec`.
- "Mirrored" means the same defect was also applied to `live_oracle`.

| mutant | red | matches record |
|---|---|---|
| M1 `G_p = g x 3.51` (mirrored) | L1 only | yes (`55.63`) |
| M2 `sigma_p = stall(T)` | L3 only | yes |
| M3 `sigma_t = stall(0)` | L3 only | yes |
| M4 `D` anchored at the settled start | L3 only, `D` 922,826 | yes |
| M5 `D = ceil(ln10 / -ln rho_max)` (mirrored) | L4 only, `D` 45,220 / 44,970 | yes |
| M6 certificate diagonal without `mu_settled` (mirrored) | L4 crossing only, `D` 45,264 / 45,032 | yes |
| M7 `G_p` without `delta^2 g` | L3 only, 9.4e-7 below | yes |
| M8 `G_p` over settled groups only | L3 only, 2.6 % below | yes |
| M9 stalls swapped | L3 at `sigma_t <= sigma_p` (`:1870`) | yes |
| X1 capped refusal removed (mine) | survives | unreachable guard (NIT 4) |
| X2 `tail_stall` without `.min` (mine) | survives | equivalent at the launch rates; a by-construction guard |
| X3 `sigma_p = W_0(0, F)` alone (mine) | L3 (0.16 % below) | sound but not the ruled value; caught |
| X5 `live_cascade_crossing` ignores `decades` (mine) | **survives** | MINOR 1 |
| X6 carry-limit refusal removed (mine) | survives | unreachable guard (NIT 4) |
| X7 certificate order `tau` first (mine) | survives | sound: the verification still holds and gives the same `D` |

## Test value (one sentence per new or rewritten test)

- **L1 `live_peak_gain_bounds_a_retarget_after_a_nyquist_drive_on_the_real_kernel`.** A live `G_p`
  taken from the settled designs' `l1` (M1, mirrored, so the recomputation agrees) is beaten by the
  real kernel's retarget transient by 55.8 dB. It is the only real-kernel check of `G_p`, and the
  only test that drives a retarget after a Nyquist drive.
- **L3, the rewritten `live_bound_carries_every_term_an_independent_recomputation_requires`.** It
  turns red for a module value outside the band of the independent recomputation:
  - `sigma_p` taken from `T` (M2);
  - `sigma_t` taken at every frame (M3);
  - `D` anchored at the settled start (M4);
  - `G_p` without the current input (M7);
  - `G_p` without the window frame (M8);
  - `sigma_p` reduced to `W_0(0, F)` (X3);
  - swapped stalls (M9, at the rule-(g) assertion).

  Nothing else reads these values, and the 53.5 dB slack of L1 cannot see M7, M8 or X3.
- **L4 `live_decay_covers_the_module_crossings_and_its_floor`.** It turns red for a `D` derived
  without the transient (M5) or with a certificate rate that drops the settled state rounding
  (M6). Both are mirrored, so L3 agrees. Both are red at the crossing check, because
  `T(1) - T = 46,099` is above `D`. The floor assertion has no unique catch (MINOR 2), and the test
  passes without checking anything under X5 (MINOR 1).

## Gates run (export, release unless stated)

| gate | result |
|---|---|
| `cargo test --locked --release -p builtins --features builtins/test-support --test tail_contract` | 19 passed |
| debug `cargo test --locked --all-targets -p lane -p math -p builtins -p dsp-reference --features math/lane,builtins/test-support,lane/test-support` | exit 0, no failure |
| gate 8 `every_sixty_four_track_console_document_is_bounded_exactly_at_every_launch_rate` | passed |
| gate 2 `input_bound_budget`, one invocation, `taskset -c 7` | exit 0; 16.5 ns needed (17 committed); 96.7 % of budget (descriptive) |
| `audit capi` | `pcm_digest cb10fbface44a3a4`, 0 alloc, 0 dealloc, 0 syscalls |
| `check-builtins-fixtures.sh . <audit>` | ok (50 files) |
| `run-wasm-gates.sh` (native + wasm simd128 + V8 spill) | ok |
| `check-workspace-policy.sh` | ok |
| `check-realtime-policy.sh` | ok (89 regions, 25 files) |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | clean |
| `cargo fmt --all -- --check` | clean |
| `check-cross-targets.sh` | PASS (x86-64-v3; aarch64 iOS and Android checked and linted; the #1018 expected failures; wasm simd128) |
| brute force (probe, `f64`, all four rates) | no counterexample; (N1) margin >= 52.30 dB, (N2) ratio <= 2.74e-6 |
