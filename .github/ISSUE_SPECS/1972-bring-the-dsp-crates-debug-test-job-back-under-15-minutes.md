# Bring the DSP crates debug test job back under 15 minutes

Filed by root (backlog sprint, 2026-10-24).

## Defect

The required job "DSP crates debug tests" (`.github/workflows/qualification.yml`, `test-debug-b`) now takes 13 to 15 minutes and hit its 15-minute limit on main once and on four PR branches on 2026-10-24 (#1951, #1964, #1965, #1967), with no test failure. Root raised the limit to 25 minutes as a stop-gap in the housekeeping PR that files this issue; this issue owns the real fix.

The tests that run longer than 60 s in that job include:
- `a_long_polarity_flip_on_a_bank_member_stays_inside_its_trim`
- `live_bound_carries_every_term_an_independent_recomputation_requires`
- `padding_tests::every_padded_bank_renders_its_members_per_node_bits`
- `padding_tests::padded_banks_render_the_fixtures_per_node`

## Smallest closable slice

1. Measure the per-test wall time of the job's test binaries once (debug profile, as CI runs them) and list the ten slowest.
2. For each slow test, cut its cost without losing its claim: fewer blocks, a smaller grid, one backend where the claim is not per lane width, or move a long sweep to `nightly.yml` when it is a sweep and not a per-PR claim (one test per claim; a test that moves keeps its claim). A test that loses its claim is not acceptable.
3. Bring the job back under 10 minutes on the CI runner, and set `timeout-minutes` back to 15.

## Paths

The slow tests' files (crates/builtins, crates/*/src/padding_tests.rs, and those the measurement names), `.github/workflows/qualification.yml` (the timeout line) and `nightly.yml` only if a sweep moves, this spec.

## Evidence and decision record (attempt 1, 2026-10-25)

**Measurement.** The job's exact command (`test-debug-b`, its package list and features), built
first, then run once under the exclusive machine lock with libtest's `--report-time`
(`RUSTC_BOOTSTRAP=1`, which changes no test program). Local: 32 cores; CI: `ubuntu-24.04`, 4 vCPU.

**Root cause of most of the time.** The padding and rest tests built every prepare request's
`tail_bound` with `conformance::tail_bound_of`, which admits a whole registry and derives every
quality row's bound: about 40 ms per request in debug, so 51 s of the transient shaper's
fixture test was registry admission and 0.5 s was the render. The builtins `tail_contract` tests
each recomputed `input_section_live_bound` (its composition costs seconds per rate in debug) at
four rates.

**The ten slowest tests, and what changed** (local debug wall, before -> after):

| Test | Before | After | Change; why the claim holds |
|---|---|---|---|
| transient-shaper `padding_tests::padded_banks_render_the_fixtures_per_node` | 51.2 s | 0.5 s | The request's tail-bound entry is computed once per launch rate (`tail_bound`, a `OnceLock` per rate). It is a pure function of the rate, so every request carries the same entry as before; the render and every assertion are unchanged. |
| builtins `tail_contract::live_bound_carries_every_term_an_independent_recomputation_requires` | 49.5 s | 16.9 s | `input_section_live_bound`, `input_section_live_cascade` and the composition are computed once per launch rate per binary, the four rates on their own threads (`per_launch_rate`). Pure functions of the rate: every compared value is the same. The independent oracle is untouched. |
| builtins `input_liveness::a_long_polarity_flip_on_a_bank_member_stays_inside_its_trim` | 47.8 s | deleted (its leg adds 1.2 s) | Folded into `every_trim_body_keeps_a_flip_inside_its_trim` as its "identity dual" leg (the Simd4 bank's `identity_chain_ramp_block`, the body the deleted test reached), plus a check that the uncommanded members hold `g`. The shorter -1 dB / 2^20 flip leaves `[-g, g]` under the unclamped law, which the leg's own guard asserts. The 22,137,669-frame #1408 reproducer stays on the single input (`a_long_polarity_flip_stays_inside_its_trim`, 9.7 s), which reaches the same generic body. |
| multiband-compressor `tail_contract::the_kernel_meets_the_stated_tail_and_rests_by_the_stated_bound` | 37.3 s | 28.6 s | The fresh reference instance (`Z` and its noise render) had the same parameters, input and start sample for both glides; it is now rendered once and both glides compare against it. The same values are compared. |
| lane `g2_kernel_identity::g2_skewed_cascade_equals_the_interleaved_cascade` | 27.2 s | 9.3 s | The long block is 256 frames in debug and stays 1,024 in release, which `test-release` runs on every PR (`cargo test --release -p lane ...`). Every recorded #978 mutant trips at 2 frames. |
| transient-shaper `padding_tests::every_padded_bank_renders_its_members_per_node_bits` | 27.2 s | 0.4 s | As the first row. |
| builtins `tail_contract::fixed_design_decay_law_is_sound_tight_and_above_its_floor` | 24.5 s | 24.6 s | Unchanged: a 4,000,000-frame `f64` brute force per design pair, already 48 kHz only in debug. A shorter response could move a suffix crossing, and sharing the responses across the three fixed-design tests would hold about 450 MB. |
| multiband-compressor `rest_tests::a_lane_reported_at_rest_stays_at_rest_through_silence` | 20.3 s | 20.3 s | Unchanged: the render itself (800-block schedules per seed and width); the request memo made no difference here. Not run in release per PR, so a smaller grid would lose per-PR coverage. |
| gate-expander `padding_tests::padded_banks_render_the_fixtures_per_node` | 20.0 s | 0.6 s | As the first row (the same memo in the gate/expander, soft-clip, compressor and multiband padding modules and the multiband rest module). |
| transient-shaper `padding_tests::swap_tests::a_swapped_strip_renders_its_unmoved_twins_bits` | 19.9 s | 1.2 s | As the first row. |

Also: math `m3_corpus_exercises_each_domain` (17.5 s) is ignored in debug and runs in the
release job on every PR: the distinct-result count is a property of the corpus, the same bits in
every profile; `m3_corpus_digests_match_pins` still replays every case in debug under overflow
checks. builtins `meter::a_banked_block_commit_publishes_the_scalar_meters_snapshots` (15 s, about
60 CPU-seconds on nine threads) is unchanged: it is a per-width bit differential over its whole
grid, and nothing runs it in release.

**Mutants (each red, then restored).**

- `identity_chain_ramp_block`'s trim update reverted to `current + step` (lane): the new leg is red,
  `identity dual: channel 0 trim -0.8966807 after frame 1044480 passes 0.8912509`.
- `BuiltinInputBank::set_polarity_invert` applied to members `0..=max(lane, 1)`: red,
  `identity dual: uncommanded member 1 channel 0 holds its trim`.
- Transient shaper bank `active` mask set to every lane: four padding tests red (`padded lane 1 was
  reported`), the fixture test among them.
- `input_section_live_bound`'s tail plus one frame: `live_bound_table_is_the_computed_live_bound_at_every_launch_rate`
  red through the memo (`44100 Hz`).
- Multiband band-gain smoother without its `flush`: the kernel test red,
  `48000 Hz, P = 1e29, glide false: not at Z by N + 12164081`.
- The skewed cascade's epilogue deleted (#978-M3): `g2_skewed_cascade_equals_the_interleaved_cascade`
  red at 256-frame debug scale.
- m3: unchanged test; `cargo test --release -p math --features math/lane --test m3_determinism`
  runs all three tests (green, 1.4 s).

**Job time (local, the test step's command, one run each, exclusive lock).** Before: the 187
binaries' own times sum to 496 s, on 1,087 CPU-seconds. After: 334 s, on 708 CPU-seconds; the
whole command ran in 5 min 45 s wall. Every binary passed both times (187 `test result: ok`).

**CI estimate.** Three green `main` runs of 2026-10-24 spent 436 s, 703 s and 832 s in their test
binaries (the same sum) against this machine's 496 s: a factor of 0.9 to 1.68, the runner's noise.
At the worst observed factor, 1.68, the binaries take 561 s (on CPU: 708 x 0.77 = 545 s), and the
step's cached build adds about 54 s: about 10 minutes at worst, about 8.5 minutes at the middle
factor (1.4). The old worst case was 14.8 minutes, at the 15-minute limit. The remaining slow
tests are listed above with the reason each stays.

`timeout-minutes` is back to 15 (root authorized that line and its comment).
