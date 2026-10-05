# State a bounded tail and an exact-rest bound for every node

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-4(b), D15-4(c)).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

A plan's reported tail means one thing everywhere: after the input stops, the output stays below
-144 dB relative to the input's peak from a stated sample on. This slice defines that contract and
the exact-rest bound, and delivers both for the builtin input section. With it, a strip with
enabled input filters, or with a live input lane, reports a finite tail at its rate instead of
`Infinite`, so the C ABI and browser hosts can report bounded tails (#1261, #1262). Each effect
gets its own slice (see "Non-goals").

## Context

- **The type.** `TailSamples::{Finite(u64), Infinite}` (`crates/effect-contract/src/lib.rs:131-134`)
  has no stated meaning beyond "terminates". It is carried per quality descriptor
  (`QualityDescriptor::tail`, `:513-521`), composed by PDC with `Infinite` winning and finite
  tails added along a path after latency (`crates/graph-compiler/src/pdc.rs:105-137`), and capped by
  `maximum_finite_tail_samples` (`:132-135`). The C ABI reports it as `TAIL_FINITE`/`TAIL_INFINITE`
  plus samples (`crates/capi/src/abi.rs:36-39`, `crates/capi/src/runtime/compile.rs:649-650`). No
  render path reads it.
- **The builtins today.** `BuiltinTail::{FiniteZero, Infinite}` (`crates/builtins/src/lib.rs:224-227`).
  `InputBuiltins::tail` (`:3433-3445`) says `Infinite` while any HPF/LPF is enabled or a filter
  target is ramping. `BuiltinChain::tail` (`:3262-3264`) returns it. The compiler forces `Infinite`
  for every strip with a live input lane (#1254 D1; `crates/builtins-compiler/src/lib.rs:3256-3266`
  in `expected_tails`, and `:3560-3568`), and `graph-compiler` maps the two variants
  (`crates/graph-compiler/src/compile.rs:141-150`). `docs/BUILTINS_AND_METERING_V1.md:50-52` says
  "Enabled filters declare an infinite tail".
- **The input section.** Trim (`-144..=24` dB, `crates/builtins/src/lib.rs:3284`), polarity, then
  HPF then LPF, both Butterworth TPT SVF sections with fixed `k = sqrt(2)` designed by
  `SvfSection::design` (`:722-760`). Cutoff domain: 10 Hz (`crates/builtins/src/filter_control.rs:41-42`)
  to `builtin_filter_cutoff_maximum_hz` (`lib.rs:322-330`), or disabled. Live filter targets ramp
  their words over `INPUT_FILTER_RAMP_SAMPLES = 64` (`filter_control.rs:9`).
- **Evidence (decision 15, round 1 B4 and round 2 B4(a)).** The exact-zero tail is infinite without
  *Flush the SVF jointly so builtin and EQ filters reach exact rest* (#1328). Worst case over the
  reachable domain, -144 dB re input peak: 383,571 samples at 44.1/88.2 kHz, 381,428 at 48/96 kHz
  (slowest pole at the *maximum* cutoff, radius 1 - 5.2e-5). With #1328, exact rest at +24 dBFS:
  ≤ 934,193 (44.1/88.2 kHz), ≤ 929,225 (48/96 kHz); any sanitized input: ≤ 2.27M. D15-4 states the
  contract figures 1.0M and 2.4M. Every step matrix of these sections is non-expansive in the
  2-norm (bilinear map of a passive `M`).
- **Tests that pin today's rule:** `input_tail_is_infinite_while_a_filter_target_is_ramping`
  (`crates/builtins/tests/filter_liveness.rs:313-336`); `builtins-compiler` unit tests at
  `crates/builtins-compiler/src/lib.rs:11885`, `:11915`, `:11931`; `crates/host-core/tests/live_lanes.rs:202-211`
  and `:251-265` (`Infinite` under `HostLiveLanes::ALL`). Canonical plan text carries tails:
  `fixtures/graph/v1/direct-route.canonical.txt:4,59` (`infinite`) and the digest
  `ZERO_DELAY_CANONICAL_SHA256` (`crates/graph-compiler/tests/track_delay.rs:244-266`).

## Decisions frozen for this slice

- **D1. Tail.** `TailSamples::Finite(T)` means: for every input of peak `P > 0` that is zero from
  sample `N` on, under any control history the node admits before `N` (block-boundary targets and
  their ramps; a ramp may be in flight at `N`), and with no control event at or after `N`,
  `|y[n]| < P * 10^(-144/20)` for every `n >= N + latency + T`. `T` is counted beyond latency, as
  PDC already composes it. Document it on the type and in `docs/EFFECT_CONTRACT_V1.md`.
  `Infinite` keeps its meaning ("no bound stated") for nodes whose slice has not landed.
- **D2. Exact rest.** New `pub struct RestSamples { pub peak_plus_24_dbfs: u64, pub any_sanitized_input: u64 }`
  in `effect-contract`: under D1's conditions, with input peak at most +24 dBFS (respectively below
  the input sanitizer's `1e30`), from `N + latency + R` on (a) every output sample is `+0.0` or
  `-0.0` (a polarity-inverted zero is `-0.0`), and (b) every *signal-state word* equals, under `f32`
  `==`, the same word of the node's *rest state* `Z`. `Z` is the state a freshly reset instance
  with the same parameters and settled ramps reaches after `R` zero input samples, and it must be a
  fixed point of the zero-input step (the derivation of each slice proves it; a node whose
  zero-input step has no reachable fixed point states no bound). Signal-state words are the words
  an input sample can reach (filter integrators, envelopes, gain smoothers, hold counters, rings
  and their running sums). Parameter words, ramp words, payload headers and ring cursors are not
  signal state. `Z` need not be zero: the true-peak limiter's rings rest at `1.0` and its box sum
  at `Wb`, which is its reset state (`crates/true-peak-limiter/src/lib.rs:624-643`); the gate
  resets open (`gain_db = 0`, `crates/gate-expander/src/lib.rs:504-515`) but rests closed, its gain
  word at the range floor. Documented beside `TailSamples`. For the builtin input section `Z` is
  the reset state: the eight SVF integrator words at `+0.0`.
- **D3. Certified, computed, never pinned.** Both values are computed on the control thread at
  preparation from the designed `f32` words (evaluated in `f64`), per rate. The reported value is a
  certified upper bound on D1's smallest `T`: the exact-arithmetic tail sum
  `S(j) = sum_{m >= j} |h[m]|` must fall below `eps / 2` (`eps = 10^(-144/20)`), and the derivation
  proves that the `f32` kernel's deviation, with #1328's flush, stays below the other `eps / 2`
  (rate inflation `rho_f = rho + 6 * 2^-24 * kappa`, the stall radius, and exact rest below
  `REST_EPS`). `S` is summed exactly to a horizon `H` plus a closed-form remainder; for the HPF→LPF
  cascade the remainder uses the union bound
  `sum_{m>=H} |h| <= |h_L|_1 * S_H(ceil(H/2)) + S_L(ceil(H/2)) * |h_H|_1`. The derivation goes in
  `docs/derivations/1329-input-section-tail-and-rest.md`.
- **D4. Fixed design (no live input lane).** `T` and `R` come from the strip's own designed sections
  (max over left and right), with the prepared trim as a gain on `P`. Disabled filters: `T = 0`,
  `R = 0` (trim and polarity are memoryless).
- **D5. Live input lane.** `pub fn input_section_live_bound(rate) -> (TailSamples, RestSamples)` in
  `builtins`: the bound over the whole cutoff domain (each section disabled or in `[10 Hz, max]`),
  trim +24 dB, any history of filter targets and their 64-sample ramps. All builtin sections share
  `k = sqrt(2)`, so every designed step matrix is the bilinear map of one fixed `M` and they share
  eigenvectors `V`; the derivation bounds the state at `N` in the `V`-norm across any target history,
  including ramp steps whose interpolated words are off that curve (their `V`-norm bound
  `q_ramp` over the domain is part of the derivation), then bounds the free response. Production
  evaluates the closed form at the worst-case pair below (round 1: HPF one ulp below the maximum
  into LPF at the maximum, both channels). #1262's gate 4 enables this pair live:

  | rate | HPF (Hz, bits) | LPF (Hz, bits) |
  |---|---|---|
  | 44,100 | 22049.48046875 `0x46ac42f6` | 22049.482421875 `0x46ac42f7` |
  | 48,000 | 23999.431640625 `0x46bb7edd` | 23999.43359375 `0x46bb7ede` |
  | 88,200 | 44098.9609375 `0x472c42f6` | 44098.96484375 `0x472c42f7` |
  | 96,000 | 47998.86328125 `0x473b7edd` | 47998.8671875 `0x473b7ede` |

  Expose it as `pub fn input_section_worst_case_pair(rate) -> Option<(f32, f32)>` (`None` off the
  launch rates). Gate 1(b) confirms the pair. If the scan finds a worse one, this table and the
  function change in this slice; #1262 reads the function, not the numbers. If the derivation
  cannot prove a finite bound, the slice stops and reports it to Sol; there is no fallback value.
- **D6. Gain-only parts** (trim, polarity, fader, mute, matrix) report `T = 0`, `R = 0` beyond their
  latency (0).
- **D7. One value per plan.** `BuiltinTail` is deleted. `InputBuiltins::tail()` returns `TailSamples`
  and a new `InputBuiltins::rest()` returns `RestSamples`, both computed once at preparation and
  stored. The dynamic "ramping ⇒ `Infinite`" rule (`lib.rs:3433-3445`) goes: a live target cannot
  change a plan's tail, because D5 already covers it. `builtins-compiler` replaces both
  forced-`Infinite` sites with `input_section_live_bound(rate)`: the seal check's `expected_tails`
  (`crates/builtins-compiler/src/lib.rs:3261-3267`) and preparation (`:3562-3568`). Both hosts
  prepare builtins through this crate (`crates/host-core/src/prepare.rs` for the C ABI,
  `hosts/host-web/src/lib.rs` for the browser), so both report the bound for every strip with a
  live input lane. `graph-compiler` takes `TailSamples` directly. Render reads
  neither value; nothing is computed or allocated on the render thread.
- **D8. Composition is unchanged.** PDC still adds node tails along a path. Gain in other nodes is
  not folded into a node's floor; the graph extent's meaning across gain, and the tail term of a
  strip's `delay_samples` line (a pure delay, not latency), are
  *Define how node tails compose through gain in the graph extent* (#1379).

## Deliverables

1. `effect-contract`: D1 docs on `TailSamples`, the `RestSamples` type (D2).
2. `crates/math/src/tail.rs` (new, `pub mod tail`): the certified `f64` bounds for TPT SVF
   sections and their cascades, control-plane only, so the EQ and the multiband crossover reuse
   them. It lives in `math`, not `lane`: `lane` is `no_std` with `wide` as its only dependency
   (`crates/lane/Cargo.toml`), so it has no `f64` logarithm, while `math` exports `log`/`exp`
   (`crates/math/src/lib.rs:68`, `:86`) and `builtins`, `parametric-eq` and
   `multiband-compressor` already depend on it (`effect-runtime` is not a `builtins` dependency).
   It uses `math::log`/`math::exp`, never `f64::ln`/`f64::exp`;
   `builtins`: `crates/builtins/src/tail.rs` (new) composing them for the input section;
   `input_section_live_bound`, `input_section_worst_case_pair`; `InputBuiltins::{tail, rest}` and `BuiltinChain::{tail, rest}`.
3. Plumbing: `builtins-compiler` and `graph-compiler` on `TailSamples`; `BuiltinTail` removed.
4. Tests (gates 1-3); superseded tests replaced (gate 4).
5. Docs: `docs/EFFECT_CONTRACT_V1.md`, `docs/BUILTINS_AND_METERING_V1.md:50-52`,
   `dsp-research/filters.md` "Latency and tail", the derivation note (D3, D5) with equations,
   numerical limits, NaN behaviour (a non-finite state is reset by the per-block check, so it never
   starts a tail) and citations ([SIMPER-SVF], [ZAVALISHIN-TPT], [ORFANIDIS-ISP], [SMITH-SASP];
   the W3C Web Audio `tailTime` notion as the external analogue).
6. Canonical plan digests re-pinned one by one (gate 5), with the reason in the commit message.

## Authorized paths

- `crates/effect-contract/src/lib.rs` (the tail types and docs only), `docs/EFFECT_CONTRACT_V1.md`
- `crates/math/src/tail.rs` (new), `crates/math/src/lib.rs` (module line)
- `crates/builtins/src/lib.rs`, `crates/builtins/src/tail.rs` (new), `crates/builtins/tests/tail_contract.rs` (new),
  `crates/builtins/tests/filter_liveness.rs`
- `crates/builtins-compiler/src/lib.rs` (tail rule, tail type, its unit tests)
- `crates/graph-compiler/src/compile.rs` (the tail mapping), `crates/graph-compiler/src/lib.rs`
  (the unit test `builtins_replace_only_the_three_internal_track_bindings`, whose `Infinite`
  assertions at `:14130` and `:14175` come from the fixture's input HPF),
  `crates/graph-compiler/tests/track_delay.rs` (the digest), `fixtures/graph/v1/direct-route.*`
  (regenerated by `crates/graph-compiler/src/bin/graph_fixture.rs`)
- `crates/host-core/tests/live_lanes.rs` (the tail assertions)
- `docs/BUILTINS_AND_METERING_V1.md`, `dsp-research/filters.md`,
  `docs/derivations/1329-input-section-tail-and-rest.md` (new), this spec

`builtins-compiler`, `graph-compiler`, `host-core` and `fixtures` sit outside stream G's column:
coordinate with stream F (#1261, #1262 edit `builtins-compiler`).

## Non-goals

- Any effect's tail or rest, the composition rule, and retiring `Infinite`. Each is its own slice:
  - First: *Carry each effect's tail and exact-rest bound in its prepared metadata* (#1377). The
    per-effect slices need the home it creates.
  - Second: *Define how node tails compose through gain in the graph extent* (#1379). It adds the
    per-node gain and decay that the per-effect slices then state once.
  - After #1379, in any order: *State the parametric EQ's bounded tail and exact-rest bound* (#1372),
    *State the multiband compressor's bounded tail and exact-rest bound* (#1373), *State the
    delay's bounded tail and exact-rest bound* (#1374), *Report a zero tail beyond latency for the
    compressor and the true-peak limiter* (#1375), and *State exact-rest bounds for the gate,
    transient shaper and soft clip* (#1376). #1373, #1374 and #1376 reuse #1375's one-pole helper,
    so they follow #1375; #1373 also follows *Make the multiband compressor's crossover live*
    (#1338), because it owns the glide-in-flight case.
  - Last, after #1372-#1376: *Retire the Infinite tail* (#1378).
- A render-side consumer (silence skipping is #1107). Any C ABI or browser wire change.

## Hazards

- `maximum_finite_tail_samples` (`pdc.rs:132-135`) can now refuse a plan that `Infinite` passed;
  hosts set `u64::MAX` (`crates/host-core/src/prepare.rs:1559`), `graph_fixture` 10,000,000. Values
  here are below 1M.
- Depends on #1328's `REST_EPS`; without it D2 has no finite value at the top of the domain.

## Objective gates

1. **Recomputation** (`tail_contract.rs`, every launch rate in release, 48 kHz only in debug).
   (a) Fixed design: for every HPF/LPF pair from {disabled, 10 Hz, 1 kHz, one ulp below maximum,
   maximum} and trims {0, +24 dB}, an independent brute force computes the `f64` cascade impulse
   response `h` (designed `f32` words, `f64` arithmetic) to 4,000,000 samples and its suffix sums
   `S_b(j) = g * sum_{j <= m < 4e6} |h[m]|` (`g` the linear trim). Define `T_b(e)` as the smallest
   `j` with `S_b(j) < e`. Assert soundness `T_b(eps / 2) <= tail` (the exact-arithmetic half of D3
   is covered) and tightness `tail <= T_b(eps * 2^-5)`: the certified tail is never longer than the
   exact tail at a floor 30 dB lower. The 30 dB margin absorbs the `eps / 2` split (6 dB), the
   cascade union bound and the `f32` rate inflation (`6 * 2^-24 * kappa` against `1 - rho = 5.2e-5`
   at the extreme: about 0.7 % of the decay rate per unit of modal condition number `kappa`, so the
   margin holds for `kappa` up to roughly 15); a Putzer `m * rho^m` factor costs about 13 nats, roughly
   250,000 samples at the extreme against the margin's roughly 66,000, so it is red. Disabled
   filters assert `tail == 0`. A certified value above the tightness line is a finding for Sol
   with the measured ratio, not a gate change. (b) Live bound:
   assert no scanned design exceeds `input_section_live_bound(rate)`: 100,000 log-spaced cutoffs
   plus the last 65,536 `f32` values below each rate's maximum, for both sections.
2. **Soundness on the real kernel** (release): the domain extreme of D5 at every rate, `P` in
   `{1.0, 10^(24/20), 1e29}`, input = `P * sign(h[T + i])` reversed over the last 1,000,000 samples
   before `N`, one live filter target applied 32 samples before `N`. Assert `|y[n]| < P * eps` for
   every `n` from `N + T` until exact rest, and from `N + R` on (`R` = `peak_plus_24_dbfs` for the
   first two `P`, `any_sanitized_input` for the last) D2's rest: every output sample is `±0.0`
   (one run with polarity inverted, so `-0.0` is reached), and the eight SVF integrator words
   equal those of a freshly reset `InputBuiltins` with the same targets (`Z`), under `f32` `==`.
3. **Contract figures:** `input_section_live_bound` at every rate has
   `peak_plus_24_dbfs <= 1_000_000` and `any_sanitized_input <= 2_400_000`; record `T` and `R` per
   rate in the evidence. A certified value above a figure is a finding for Sol, not a gate change.
4. **Superseded:** delete `input_tail_is_infinite_while_a_filter_target_is_ramping`; rewrite the
   three `builtins-compiler` tail tests to the finite values; `live_lanes.rs:265` asserts the live
   bound at the fixture's rate and `:202-211` keeps the plain fixture finite and the EQ fixture
   `Infinite` (the EQ slice has not landed).
5. **Re-pins, one at a time:** regenerate `fixtures/graph/v1/direct-route.*` with `graph_fixture`
   and update `ZERO_DELAY_CANONICAL_SHA256`; the only expected byte change is `infinite` →
   `finite:<T>` on post-input-builtins rows. Any other moved byte stops the slice.
6. Commands:
   - `cargo test --locked --all-targets -p lane -p math -p effect-runtime -p delay -p compressor -p multiband-compressor -p gate-expander -p true-peak-limiter -p transient-shaper -p soft-clip -p parametric-eq -p builtins -p dsp-reference -p conformance --features math/lane,parametric-eq/test-support,builtins/test-support,lane/test-support`
   - `cargo test --locked --release -p builtins --features builtins/test-support --test tail_contract`
   - the `test-debug-a` workspace command from `.github/workflows/qualification.yml` (it covers
     `builtins-compiler`, `graph-compiler`, `host-core` and `capi` with their `test-support` features)
   - `bash scripts/check-graph-determinism.sh`, `bash scripts/check-builtins-policy.sh`,
     `bash scripts/check-effect-contract.sh`, `bash scripts/check-dsp-research.sh`,
     `bash scripts/check-workspace-policy.sh`
   - `cargo build --locked --release -p audit && ./target/release/audit capi` (`qualification.yml:715`)
   - `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`

## Test value

- Gate 1(a): a tail computation that drops the trim gain or the cascade's second section falls
  below `T_b(eps / 2)`, and a needlessly loose one (a Putzer factor) exceeds `T_b(eps * 2^-5)`; no
  test computes a tail today.
- Gate 1(b): a live bound evaluated at the wrong extreme (for example the minimum cutoff, which
  is not the slowest pole) is exceeded by a scanned design.
- Gate 2: a bound that ignores `f32` rounding, the ramp in flight or the flush stall is violated by
  the production kernel at the worst-case input, and a state that settles at a non-reset value (a
  limit cycle the flush misses) fails the fresh-instance comparison; gate 1 only checks `f64`
  arithmetic.
- Gate 3: a sound but uselessly loose bound (for example a Putzer `m * rho^m` factor where the
  eigenvector bound applies) breaks the D15-4 figures.

## Dependencies

- *Flush the SVF jointly so builtin and EQ filters reach exact rest* (#1328)
