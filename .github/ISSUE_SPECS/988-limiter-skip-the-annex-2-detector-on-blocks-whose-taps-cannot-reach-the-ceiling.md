# Limiter: skip the Annex-2 detector on blocks whose taps cannot reach the ceiling

## Product slice and current baseline

Implement the smallest class-A slice: one whole-bank, both-channel detector screen on stationary,
uniform dual blocks. Skip only the detector and replace its required gain with exactly `1.0`;
retain every minimum-filter, quantisation, box, release, delay, bypass and reporting operation.
No unity-rest path (#989), ramp/ragged/collapsed screen, layout, ABI, wire or persisted-state change.
The user explicitly authorizes Sol implementation in this batch in place of the default Terra first
attempt. Five attempts maximum, each with one fresh adversarial verdict.

Current baseline: `0d326c5c867c8ce4309c6ad034beab5d17e06326`, stable Rust 1.99.1. Read-only brief
anchors in `crates/true-peak-limiter/src/lib.rs`: `ANNEX2_FIR` 481; `History` 1645;
`channel_frame_uniform` 2563; `linked_frame_uniform` 2633; `limiter_block_linkable` 3107;
`detector_chunk` 3185; `limiter_block_uniform` 3468; `LimiterCore::process_block` 4351.
#1013 removed the additive zero FIR seed; #991 already supplies stationary hot loads/stores;
#1014 and linked streams now split the stationary uniform gain loop into two passes. The original
brief's proposed code integration and 133/223 instruction counts belong to its older compiler/tree.

Research, not a patch to apply: [full diagnosis](../../docs/handoffs/effects-2026-09-27/LIMITER-DIAGNOSIS.md)
and [adversarial verification](../../docs/handoffs/effects-2026-09-27/VERIFY-LIMITER.md).
The latter previously measured native unscreened regressions and proposed a hard no-regression gate;
that evidence must be reconciled explicitly below before implementation is approved. Historical
prototype speedups are not measurements of this implementation or a capacity claim.

## Algorithm and numerical proof

For a phase, `v_p = sum(k=0..11, h[k,p] * x[n-k])`; the detector takes
`P = max(abs(x[n-6]), abs(v_0), ..., abs(v_3))`. Let `M` be the largest tap magnitude,
`A = max_p sum_k abs(h[k,p]) = 2.0228271484375`, `u = 2^-24`, and
`gamma_13 = 13u/(1-13u)`. Each phase uses twelve rounded products and eleven rounded additions;
`abs(fl(v_p)) <= A*M*(1+gamma_13)` covers the product-plus-sequential-sum error when normal.
For flush-to-zero or gradual-underflow intermediates add a conservative absolute `64*MIN_POSITIVE`
allowance (23 rounded operations, with unit-magnitude coefficients); there is no overflow on admitted
taps. The threshold multiply adds one `(1+u)` factor.

For each lane use `threshold = 0.49 * min(limit_left.current, limit_right.current)`.
Both channels' twelve history taps and every block input must satisfy ordered
`abs(tap) < threshold`. The minimum is essential under Maximum link: either peak can reach either
channel's gain computer. Valid stationary limits are finite and at least `10^(-25/20)`;
`0.49*A*(1+gamma_13)*(1+u) + 64*MIN_POSITIVE/min_limit < 0.9915 < 1`.
The alignment sample is below the same limit. Thus every required gain is exactly `1.0`.
NaN and either infinity decline by ordered comparison. Signed zero and subnormals are admitted
without changing their stored bits; the existing D7 recursive flush remains unchanged.

The screen reads all taps before output overwrites the in-place input. `history_after` preserves
the newest-first twelve words exactly: direct final twelve inputs for blocks of at least twelve
frames; otherwise repeated `History::shift` from the old history. Arbitrary block lengths, including
the corpus's 1024 frames, work without allocation. Zero-length blocks preserve history.

The FIR coefficient table, ceiling guard, limiter gain law, coefficient design and ramp rules stay
unchanged. Latency is `Fs/100+6` samples; the declared tail and reset semantics stay unchanged.
Ceiling units are dBFS, release/lookahead milliseconds; no new parameter or smoothing rule exists.
Primary source: [ITU-R BS.1770-5, Annex 2](https://www.itu.int/rec/R-REC-BS.1770-5-202311-I).
IEEE-754 round-to-nearest error is bounded above explicitly; no fused arithmetic or reassociation is
introduced. Existing quality/reference/conformance fixtures remain the DSP acceptance evidence.
Listening: bit-identical output warrants no changed-sound claim; no new listening trial is claimed.

## Implementation contract and authorized files

- `crates/true-peak-limiter/src/lib.rs`: predicate, history advancement, test-only forced-unscreened
  switch and engagement witness, plus the integration described here.
- `crates/true-peak-limiter/src/screen_tests.rs` (new): one private owning-crate test module, reusing
  existing core construction/state/snapshot helpers rather than adding a public test API.
- `crates/true-peak-limiter/tests/MUTATIONS.md`: actual mutation results only.
- This numbered spec: decisions, commands, evidence, verdicts.

Proposed integration: retain today's uniform unscreened two-pass and linked-stream body. Dispatch
stationary uniform blocks admitted by the screen to a separate screened body; specialize the
existing `channel_frame_uniform` and `linked_frame_uniform` at compile-time `SCREENED=true` to use
required gain one. Existing frame callers specialize `false` and retain their arithmetic. The
screened body advances history before writing input, then follows the frozen frame law, including
the linked mirrored ring writes and block-end hot-word copy. No runtime screening branch enters an
unscreened hot loop. No `unsafe`, locks, allocation, syscall or unbounded work is introduced.
Shared scripts/gates and floor accounting are root-owned and outside this slice.

## Objective gates and test value

1. One bound/worst-case test recomputes the stored table's phase L1 norms in f64, applies the exact
   gamma and underflow margins, and feeds each attaining sign pattern at nextbelow(threshold)
   through the actual detector. It defends a too-large screen factor and a wrong analytical margin.
2. One differential family at f32/W4/W8 compares screened and forced-unscreened shipped kernels,
   every output word, retained state word, track payload and report after every block in dev and
   release. Block lengths 0, 1, 5, 11, 12, 13, 31, 32, 33, 64, 127, 128, 1024; all launch rates;
   Maximum and DualMono; bypass both ways; per-lane and asymmetric L/R limits (at least 7 dB apart).
   Signals: quiet noise at 0.45/0.48/0.495 of limits, exact threshold and one ULP below, each phase's
   attaining pattern, loud-to-quiet history, signed zeros/subnormals/log magnitudes, isolated
   channel/lane spikes, NaN and infinities. It defends incorrect history/required gain and corruption
   hidden by delayed output, rather than making another permanent output digest.
3. The same differential family witnesses actual engagement on admissible quiet blocks and refusal
   for threshold equality, any loud history/input lane or channel, a ceiling/release ramp, ragged
   cohort or collapsed body. It defends admission outside the proof's stationary uniform domain.
4. Existing allocation, determinism and conformance gates remain; existing G5 remains the sole
   cross-target corpus digest owner. Do not add persistent scenario/output hashes or change pins.
   A one-time external baseline comparison of output/state is review evidence, not a committed test.
5. Apply meaningful mutations individually: factor 0.51; omit history admission; max instead of min
   channel limit; omit screened history write; admit ramping. Record actual red tests, never claim
   rejection from an unrun mutation. Refusals must be independently witnessed where output alone
   could coincidentally agree.
6. Focused dev/release tests and existing allocation/determinism; current lane/realtime policy;
   root's merged tree target/AudioWorklet checks. Any changed wasm roster assumptions need explicit
   root-owned amendment, not relaxed silently.
7. Current proportional codegen evidence is proposed in place of stale absolute instruction pins:
   compare baseline/candidate release disassembly, record unchanged FIR arithmetic/unscreened target
   and stream inventory, and demonstrate the screened body has no FIR. Runtime no-regression
   acceptance is unresolved pending root's explicit brief approval (see below).

Every local build/test/formatter/heavy check runs sequentially through
`/home/bl/misofm/engine-work-evidence/easy3-20261027/run-serial.sh` from this worktree; wrapper owns
its target directory and exports two build jobs and one test thread. No background command children.
Evidence lives outside git in `/home/bl/misofm/engine-work-evidence/easy3-20261027/988/`.
Checkpoints are exact-path local commits; no pushes, PR updates or GitHub mutations by this stream.
Root synchronizes this amendment before implementation and synchronizes completed evidence at batch
delivery. Pause after a coherent green tranche for root's commit/status audit.

## Brief decisions and evidence

2026-10-27: Sol read AGENTS, current code, complete diagnosis and its adversarial verification.
No code, builds or tests performed. Amendment proposed to root; implementation approval pending.
Historical benchmark/codegen evidence is retained as a risk, not reproduced or asserted current.
