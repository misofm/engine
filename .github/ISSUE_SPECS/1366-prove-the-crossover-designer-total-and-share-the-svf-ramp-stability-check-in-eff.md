# Prove the crossover designer total and share the SVF ramp stability check in effect-runtime

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-13 E2).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

No audible change. This is the precursor of *Make the multiband compressor's crossover live*
(#1338). After it, the multiband's crossover designer has an infallible form that render may call,
proven to pass its own check for every legal crossover at every launch rate, and the SVF stability
predicate the EQ uses for ramps lives once, in `effect-runtime`, where the multiband can use it too.
No rendered bit and no EQ refusal moves.

## Context

- **The designer.** `design_lr4` (`crates/multiband-compressor/src/lib.rs:519-561`) designs
  `(c1, a2, a3)` in `f64` from `g = tan(pi fc / fs)`, `t = g (g + k)`, `c1 = t / (1 + t)`,
  `a2 = g (1 - c1)`, `a3 = g a2`, rounds once to `f32`, then checks the rounding back (half-power
  point within 0.005 dB, doc `:515-518`). It returns `None` on a zero rate, an invalid crossover
  (`parameter_value_valid(&SPECS[0], ..)`, `:520`) or a failed check. It runs only at prepare and
  restore today, so a `None` is a preparation error. Render cannot accept a fallible call: an
  acked crossover edit must never be dropped (#1338 D3).
- **The stability predicate.** The EQ's `word_spectral_norm` (`crates/parametric-eq/src/lib.rs:796-807`)
  is the spectral norm of the zero-input transition matrix `M = [[1-2c1, -2a2], [2a2, 1-2a3]]` the
  SVF iterates. It is convex in the words, so checking both ends of a linear ramp checks every point
  (doc `:790-795`). Its two bounds: `NORM_TOLERANCE` (`:810`) for a rounded design, and
  `RAMP_PATH_NORM_TOLERANCE` (`:812-835`, with its derivation) for a restored in-flight walk. The
  derivation uses only `c1, a2, a3 < 1`, which the crossover words also satisfy.
- `word_spectral_norm` is public and used by `crates/parametric-eq/tests/analytic.rs:390`, `:426`,
  `:527`, `:534`. `effect-runtime` is `no_std` (`crates/effect-runtime/src/lib.rs:43`) and depends
  on `math` (`crates/effect-runtime/Cargo.toml:14`), which provides `math::tan` and `math::sqrt`
  (`crates/math/src/lib.rs:125`, `:235`).

## Decisions frozen for this slice

- **D1. Split the designer.** `design_lr4_words(sample_rate: u32, crossover_hz: f32) -> [f32; 3]`
  is the computation alone: no domain test, no check-back, no `Option`. `design_lr4` keeps its
  signature and behaviour: it tests the domain, calls `design_lr4_words`, then runs the existing
  check-back. Every current caller keeps calling `design_lr4`.
- **D2. Shared module.** A new `effect_runtime::svf` module holds
  `transition_norm(c1: f32, a2: f32, a3: f32) -> f64` (the body of `word_spectral_norm`, operation
  for operation, with `math::sqrt`), `NORM_TOLERANCE` and `RAMP_PATH_NORM_TOLERANCE`, both public,
  with the derivation comment moved verbatim. `f64` square root is correctly rounded in both
  `math::sqrt` and `f64::sqrt`, so the result is bit-identical.
- **D3. The EQ delegates.** `word_spectral_norm(words)` stays public with the same signature and
  becomes one call to `transition_norm(words.c1, words.a2, words.a3)`. The EQ's two constants are
  deleted and its two uses (`:899`, `:2742`) and the doc link (`:816`) name the shared ones.
- **D4. Totality is a test, not a branch.** Gate 1 proves `design_lr4` returns `Some` for every
  `f32` in `[80, 8000]` at 44.1, 48, 88.2 and 96 kHz, and that every designed triple has
  `transition_norm <= NORM_TOLERANCE`. That is what licenses #1338 to call `design_lr4_words` on
  render with no failure branch.

## Effect evidence (AGENTS.md list)

- Equations and coefficients: unchanged (above). Numerical limits: D4's sweep is the evidence.
- Latency, tail, smoothing, NaN and denormal behaviour: unchanged.
- Citations: Zavalishin, *The Art of VA Filter Design*, rev. 2.1.2, ch. 3-4 (the TPT SVF);
  Wishnick, "Time-Varying Filters for Musical Applications", DAFx-14 (a contractive transition
  matrix bounds the state under arbitrary coefficient variation); Laroche, "On the Stability of
  Time-Varying Recursive Filters", JAES 55(6), 2007.
- Listening evidence: not required; nothing renders differently.

## Deliverables

1. D1 in `crates/multiband-compressor/src/lib.rs`.
2. D2 in `crates/effect-runtime/src/svf.rs` and its `pub mod` line.
3. D3 in `crates/parametric-eq/src/lib.rs`.
4. `crates/multiband-compressor/tests/designer_total.rs` (gate 1).

## Authorized paths

- `crates/multiband-compressor/src/lib.rs` (D1 only), `crates/multiband-compressor/tests/designer_total.rs` (new)
- `crates/effect-runtime/src/lib.rs` (the module line), `crates/effect-runtime/src/svf.rs` (new)
- `crates/parametric-eq/src/lib.rs` (`word_spectral_norm`, the two constants and their uses only;
  #1337 edits other parts of this file, so rebase on whichever lands first)
- Root-authorized batch follow-up (2026-10-06, attempt 1 verdict MINOR2):
  `.github/workflows/qualification.yml`, one step in the required `test-release` job only
- Batch follow-up part B (2026-10-06, attempt 1 verdict NIT4, assigned by the stream G
  coordinator): `crates/parametric-eq/tests/MUTATIONS.md`, row 18's location cell only

## Non-goals

- Making the crossover live, its descriptor, word ramps or payload (#1338).
- Any change to the EQ's designs, refusals or bits.

## Objective gates

1. **Designer total over the domain.** `#[ignore]`, run in release: for every `f32` in
   `[80.0, 8000.0]` (by bit pattern, both ends included) at 44 100, 48 000, 88 200 and 96 000 Hz,
   `design_lr4` returns `Some(w)`, `w == design_lr4_words(..)` bit for bit, and
   `transition_norm(w) <= NORM_TOLERANCE`. A per-PR test runs the same three checks on every 1024th
   `f32` of the domain plus both ends. The PR records the largest norm excess found.
2. **No bit moves.** The multiband and EQ suites pass unchanged, `corpus_digests.in` and the EQ's
   render contract are untouched.
3. Commands:
   - `cargo test --locked --all-targets -p effect-runtime -p multiband-compressor -p parametric-eq --features math/lane,parametric-eq/test-support,lane/test-support`
   - `cargo test --locked --release -p multiband-compressor --test designer_total -- --ignored`
   - `bash scripts/check-parametric-eq-render-contract.sh`
   - `bash scripts/check-effect-runtime-policy.sh`
   - `bash scripts/check-cross-targets.sh`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
   - `cargo fmt --all -- --check`
   - `bash scripts/check-workspace-policy.sh`

## Test value

- Gate 1: a crossover whose rounded words fail the check-back, or a design outside the contractive
  set, turns it red. No existing test sweeps the domain, and #1338 would otherwise apply unchecked
  words on render.
- No test is superseded: `analytic.rs`'s norm tests keep calling `word_spectral_norm`, which now
  delegates, so they also guard D3.

## Dependencies

- none

## Attempt record

### Attempt 1 (Terra, on `codex/d15-stream-g` over `8439972be`)

**Implementation.**
- D1: `design_lr4_words` is the bare design; `design_lr4` keeps its signature and behaviour (domain
  test, `design_lr4_words`, unchanged check-back). Both are now `pub` (the gate-1 integration test
  must call them; `design_lr4` was private). Every existing caller still calls `design_lr4`.
- D2: `crates/effect-runtime/src/svf.rs` holds `transition_norm` (operation for operation, both
  square roots through `math::sqrt`), `NORM_TOLERANCE` and `RAMP_PATH_NORM_TOLERANCE`, with the
  derivation comment moved verbatim except one token: the intra-doc link `` [`RAMP_SAMPLES`] ``
  became a plain code span, because it names a private constant of `parametric-eq` that rustdoc
  cannot resolve from `effect-runtime`.
- D3: `parametric_eq::word_spectral_norm` is one call to `transition_norm`; the EQ's two constants
  are deleted and its two uses import the shared ones. Re-located anchors after #1328: the norm at
  `lib.rs:796`, constants `:810`/`:835`, uses `:899` and `:2779`.

**Gate 1.** `tests/designer_total.rs`. Exhaustive (`--release --ignored`): 224,919,556 designs
(56,229,889 `f32` crossovers in `[80, 8000]` x 4 rates), all `Some`, all bit-equal to
`design_lr4_words`, all within `NORM_TOLERANCE`; 5.9 s. **Largest norm excess `5.40e-8` at
7943.455 Hz, 44.1 kHz** (tolerance excess `2.38e-7`, margin 4.4x); the triples are not all
strictly contractive, so the one-rounding tolerance is load-bearing. Per-PR stride 1024: 219,652
designs, largest excess `5.29e-8` (7984 Hz, 44.1 kHz), 0.04 s in debug.

**Mutation evidence** (each applied alone to the per-PR test, red, reverted, green):
- domain tightened in `design_lr4` (`crossover_hz > 7_999.0` refused): red, "design_lr4 refused
  7999.5 Hz at 44100 Hz";
- `c1` perturbed by `1.000_01` in `design_lr4_words`: red, norm `1.00000024` exceeds at 2981 Hz;
- `design_lr4` returning a word one ulp off `design_lr4_words`: red, "checked and infallible designs
  differ at 80.0 Hz";
- `NORM_TOLERANCE = 1.0`: red, norm `1.000000000008` at 80.0625 Hz;
- `transition_norm` `a11 = 1 - 1.9 a3`: red here and in four `parametric-eq/tests/analytic.rs`
  tests (so the EQ's norm tests guard D3's delegation, as the spec says).

**Gate 2 (no bit moves).** The three suites pass unchanged; `corpus_digests.in` and the EQ render
contract untouched. One-time PR evidence: the shipped AudioWorklet module built at `8439972be` and
with this change differ in sha256 (`89a909d0...` vs `748efd3d...`), but `wasm-objdump -d` of the two
named twins is identical and every function size matches; the only difference is in the data
segment, the panic-location line numbers of the two edited source files. No code byte moved.

**Gate 3 commands.** All pass: the `cargo test --all-targets` line; the release `--ignored` sweep;
`check-parametric-eq-render-contract.sh`; `check-effect-runtime-policy.sh`;
`check-cross-targets.sh` (PASS; its note "parametric-eq 128 calls, down from 132: lower its row"
is identical at `8439972be`, not from this change); `cargo clippy --workspace --all-targets
--all-features -D warnings`; `cargo fmt --check`; `check-workspace-policy.sh`. Worklet chain
(browser-compiled code changed): `build-web-audioworklet.sh --named-twin`,
`check-web-audioworklet.sh --without-metadata-regeneration`,
`check-browser-expected-resources.py --artifacts`, `check-scalar-oracle-absent.py`,
`test-web-audioworklet.sh`: all pass. The V8 spill gate (`run-wasm-gates.sh`) was not run; it is
#1328's known red and not a gate of this issue, and no code byte of the module moved.

**Test value.** The per-PR and exhaustive sweeps turn red on a crossover the checked designer
refuses, a checked design that diverges from the infallible one, or a design outside the
contractive set; no existing test sweeps the domain.

## Follow-up record

### Batch follow-up (2026-10-06, stream G part A; root ruling on attempt 1 verdict MINOR2)

- **The exhaustive designer sweep runs in required CI.** Made by the decision-15 root coordinator
  under the owner's no-shortcuts delegation (2026-10-06). The `#[ignore]` sweep
  `designer_is_total_on_every_f32_in_the_domain` (`crates/multiband-compressor/tests/designer_total.rs`)
  ran in no CI job. `qualification.yml`'s required `test-release` job has a new step,
  `Multiband crossover designer exhaustive sweep (designer_total) in release`:
  `cargo test --locked --release -p multiband-compressor --test designer_total -- --ignored`.
  No job added, so the router and the verdict's expectation table are unchanged. If
  `test-release` then exceeds every other required job, #1428's amended D2 moves the release
  sweeps into their own parallel required job.
- **Local measurement** (x86-64, 32 hardware threads, release dependencies already built,
  worktree `codex/d15-stream-g`): 13.1 s wall including the test crate's build; the sweep itself
  4.16 s (1 passed, the per-PR test filtered out by `--ignored`). The CI step time is pending-CI
  until the batch push. `check-ci-path-routing.py`, `test-ci-path-routing.py`,
  `check-test-support-ci.py`, `test-test-support-ci.py` and `check-workspace-policy.sh` exit 0.

### Batch follow-up part B (2026-10-06, stream G; attempt 1 verdict MINOR1, NIT3, NIT4)

Documentation only; no code, test or bit changes.
- **MINOR1.** `crates/effect-runtime/src/svf.rs`'s module doc no longer says both effects meet
  "the same preconditions" without qualification: `transition_norm` and `NORM_TOLERANCE` need only
  `c1, a2, a3 < 1`, while `RAMP_PATH_NORM_TOLERANCE` also needs a ramp of at most 64 samples and a
  step that is the remaining distance times an exact power of two (the EQ's `2^-6`), and a consumer
  with another ramp re-derives the bound. The constant's doc gains the same preconditions, with
  the `N`-sample condition `2^-22 + (N + 1) * 2^-23 <= 2^-12` (holds to about 2,000 samples) and
  the note that the 30-fold margin and the `(1 + 2^-12)^64` growth figure are for 64 samples;
  the unexplained `RAMP_SAMPLES` code span became "64 samples".
- **NIT3.** The carry citation names `crates/parametric-eq/tests/carry.rs`.
- **NIT4.** `crates/parametric-eq/tests/MUTATIONS.md` row 18 keeps its historical location and
  notes that the body is now `effect_runtime::svf::transition_norm`.
- Gates: `cargo fmt --all -- --check`; `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace
  --no-deps` (part-B batch gates).
