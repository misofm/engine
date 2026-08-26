# Red-mutation record for the #88 gates

Master plan for issue #83, section 1.6: *every gate is proven red*. A test that has never failed is
not a gate. Each row below was applied to the working tree, the named test binary was run, the
result was recorded, and the mutation was reverted in the same session. Nothing in this file is a
claim about code that was not run.

Host: `x86_64` (AMD Ryzen 7 9700X, Zen 5), workspace `.cargo/config.toml` pin
`-C target-feature=+avx2,+fma`, debug profile unless noted.

Reproduce one row with:

```
# apply the "mutation" edit, then
cargo test --locked -p miso-engine-compressor --test <test binary>
# and revert
```

## Gated

| # | mutation | file | test binary | result |
|---|---|---|---|---|
| 1 | the detector gather uses `delay[0]` for every lane | `src/kernel.rs` | `lane_identity` | RED |
| 2 | `flush` dropped from the recursive word | `src/kernel.rs` | `nonfinite` | RED |
| 3 | the `mix == 1` identity select removed | `src/kernel.rs` | `identity` | RED |
| 4 | the ring wrap dropped: `next = write + 1` with no compare-select | `src/kernel.rs` | `partition` | RED |
| 5 | the release coefficient used for both ballistic arms | `src/kernel.rs` | `oracle` | RED |
| 6 | makeup dropped from the gain step | `src/kernel.rs` | `oracle` | RED |
| 7 | the recursive word kept in a local and never written back to the channel | `src/kernel.rs` | `cross_target` | RED |
| 8 | the ramping prefix never runs (`ramping = 0`) | `src/kernel.rs` | `ramps` | RED |
| 9 | the `bypass` term removed from `dry_identity` | `src/kernel.rs` | `identity` | RED |
| 10 | the ballistic coefficient is the retention, not the rate: `exp` for `1 - exp` | `src/design.rs` | `oracle` | RED |
| 11 | the ballistic coefficient designed in `f32`: `0.001 * ms * fs` as an `f32` product | `src/design.rs` | `cross_target` | RED |
| 12 | the per-lane coefficient scatter writes lane 0 for every lane | `src/design.rs` | `lane_identity` | RED |
| 13 | the restored ramp step ignores `remaining` and divides by 64 | `src/state.rs` | `payload` | RED |
| 14 | the left channel is committed before the right section is validated | `src/lib.rs` | `payload` | RED |
| 15 | the backend-availability fallback moved above the per-request validation | `src/lib.rs` | `contract` | RED |
| 16 | a Point ramps over 63 samples instead of the descriptor's 64 | `src/lib.rs` | `ramps` | RED |
| 17 | the boundary check no longer zeroes or resets the failing channel | `src/lib.rs` | `nonfinite` | RED |
| 18 | `offsets_are_ordered` returns `true` unconditionally | `src/lib.rs` | `contract` | RED (index out of bounds — the failure the check prevents) |
| 19 | the corpus renders one block instead of the frozen partition | `src/corpus.rs` | `cross_target` | RED |
| 20 | `inv_two_knee` designed as `1 / knee` instead of `1 / (2 * knee)` | `miso-engine-effect-runtime/src/dynamics.rs` (applied temporarily; that crate is **not** modified by this branch) | `static_curve` | RED |

Row 20 is applied to a foundation crate only to prove that this crate's E1 gate would catch a
change to the shared curve. `miso-engine-effect-runtime` and `miso-engine-lane` are byte-identical
to `origin/main` on this branch.

## Equivalent mutations (applied, GREEN, and recorded rather than gated)

| # | mutation | why it survives |
|---|---|---|
| 21 | the `G == 0 && makeup == +0` term removed from `dry_identity` | Applied and run: **GREEN** in `identity` and `nonfinite`, and it is genuinely equivalent for a finite sample. `gain_from_db(+0.0)` is exactly `1.0` (pinned by gate M1: `0 * LOG2_PER_DB` is `+0.0` and `exp2_lane(0)` is exact), so `wet = z * 1.0` is `z` bit for bit, and `mixed = fma(mix, z - z, z)` is `fma(mix, +0.0, z)`, which is `z` for every `mix`. The select is kept because it is the identity BRIEFS/013 freezes, and because keeping it makes this crate's identity independent of another crate's exactness pin rather than silently dependent on it. |
| 22 | the `Average` link written with an `fma`: `fma(0.5, |l|, 0.5 * |r|)` | Applied and run: **GREEN**, and provably so. `0.5 * x` is exact for every finite `x` (halving only decrements the exponent), so the fused form's internal product is the same number the unfused form rounds to, and both then round one addition. The plan's hazard note expected this to change bits; it cannot, for this multiplier. The unfused form is kept because BRIEFS/013 states the operation order, and because a multiplier that is not exactly `0.5` — which a future link law could have — would make the two differ. |
| 23 | the detector floor's D8 operands swapped: `level_floor.max(detected)` | Applied and run: **GREEN**. Two clamps stand between a NaN detector and the curve, and either alone suffices: `Lane::max(a, b)` returns `b` on an unordered pair, and `log2_lane` clamps its own argument up to `f32::MIN_POSITIVE` the same way. `tests/nonfinite.rs` pins the resulting behaviour — a NaN detector produces the level floor, never a NaN in `G` — rather than the operand order, because the behaviour is what a caller can observe. |
| 24 | the ramping/idle split removed: `ramping = frames`, so the ramping body runs for the whole block | Applied and run: **GREEN** in `partition` and `cross_target`, and it is exactly equivalent. `advance_ramps` is a no-op on a lane whose ramps all have `remaining == 0`: `next_value` returns `current` unchanged and `changed` stays zero, so no coefficient is redesigned. The split is a cost optimisation, not a semantic one — which is the right shape for it to have. The mutation that *is* gated is row 8, `ramping = 0`, which stops the ramps advancing at all. |

## Gates in this crate

| binary | proves |
|---|---|
| `contract` | E13: descriptor rows, latency, payload sizes, `scratch_fixed_bytes: 64`, parameter domains against the runtime's specs, the `L`/`D` lookahead derivation, the bank fallback ordering, the strengthened bank block guard, the three link laws |
| `static_curve` | E1: Giannoulis, Massberg and Reiss equation 4 against an `f64` transcription over a 3x4x3x737 grid — worst deviation **4.578e-6 dB**, gate 1e-4; knee continuity at both edges; the hard-knee threshold sample exact |
| `oracle` | E5: two configurations against the independent `f64` `ReferencePeakCompressor` — worst **4.694e-7** and **1.192e-7**, gate 2e-5 |
| `lane_identity` | E2: a bound bank against `W` scalar instances with per-track parameters — output bits, per-track payload bytes; plus the corpus at `W = 1`, 4 and 8 word for word |
| `partition` | E3: 4,096 frames in blocks of {1, 7, 64, 128, 512}, scalar and bank, output bits and payload bytes identical, with a Point on all seven smoothed parameters of both channels |
| `cross_target` | E4: pinned SHA-256 over the four-case corpus at all three widths, plus finiteness and non-vacuity; the same corpus is replayed under wasmtime by `tools/miso-engine-wasm-gates` |
| `identity` | E8: bypass, `mix == 0`, `mix == 1`, `G == 0 && makeup == +0`, the `Average` link's exact level, and that every identity keeps the state warm |
| `ramps` | E6, D11: one division at the event, iterated additions, the exact snap on update 64, a restart from the value reached, automation validation, and that a finished ramp equals a fresh preparation |
| `payload` | E7: idle restore bit-exact against an uninterrupted render, transactional rejection across both channels, the class-B mid-ramp restore, subnormal round trip, both resets |
| `nonfinite` | E9, D7: the boundary check trips once per block at the latency and not per sample, the left channel is untouched, the limit row, a NaN detector is clamped, and `flush` brings `G` to exactly `+0.0` |
| `stall` | E14, descriptive: the `f32` release stall floor, printed and handed to issue 046 |

## Issue #140 — the automation-span feed, the live fader, and GR observation

Every row below was applied to the working tree, the named test was run, the failure was observed,
and the mutation was reverted in the same session. Host: `x86_64`, workspace `.cargo/config.toml`
pin `-C target-feature=+avx2,+fma`, debug profile. Sweep driver: one mutation at a time,
`cargo test -p <pkg> <test>`, tree restored before the next row.

| # | mutation | file | test | result |
|---|---|---|---|---|
| 140-13 | `PreparedCompressor::gain_reduction` returns a hardcoded zero pair instead of reading `Channel::gain_reduction_db` | `compressor/src/lib.rs` | `gain_reduction::the_compressor_reports_the_reduction_its_kernel_smoothed` | RED (`a signal well over the threshold is audibly reduced: 0`) |

## Issue #143 — the effect observation surface

R5 removed `PreparedNativeEffect::gain_reduction` and its test file is re-expressed on the declared
tap (`tests/observation.rs`); row 140-13 above is superseded by 143-E6-b, which is the same
mutation applied to the same kernel read through the new address. Same host and profile as above.

| # | mutation | file | test | result |
|---|---|---|---|---|
| 143-E6-a | `PreparedCompressor::observe_resident` advances the smoother in the read (`self.instance.left.gain_reduction_db *= 0.9;`) | `compressor/src/lib.rs` | `cargo build -p miso-engine-compressor` | RED — **does not compile**: `observe_resident` takes `&self`, so "resident" is enforced by the signature rather than asserted. This is the `&self` half of E6 |
| 143-E6-b | `observe_resident` writes `0.0` into both lanes instead of reading `Channel::gain_reduction_db` | `compressor/src/lib.rs` | `observation` (whole binary) | RED — 5 of 6 tests fail; `the_compressor_reports_the_reduction_its_kernel_smoothed` reports `0` where reduction was required |
| 143-E2-a | `observe_resident_bank` broadcasts lane 0's reading to every lane | `compressor/src/lib.rs` | `observation::every_bank_lane_reads_its_own_reduction` | RED — `lane 1 left reading is its own, not a neighbour's`, left `0` vs right `3239051021` |

## Round 2 — the staged idle body and the pre-gathered detector taps

`kernel::process_block` sends an idle segment to `idle_frames_staged` when every live lane's
detector distance `D` is at least the segment length, and to `frames_loop` otherwise. The claim is
bit identity, and `tests/staged_idle.rs` is its gate: the same input at partitions that straddle
`D` puts the same frames through both bodies, with the 512-frame partition — always the per-frame
body, being longer than the staged bound — as the reference.

Every row below was applied to the working tree, `cargo test -p miso-engine-compressor --test
staged_idle` was run, the result was recorded, and the mutation was reverted in the same session.
Host: `x86_64` (AMD Ryzen 7 9700X, Zen 5), workspace `.cargo/config.toml` pin
`-C target-feature=+avx2,+fma`, debug profile.

| # | mutation | file | test binary | result |
|---|---|---|---|---|
| 25 | the legality guard loosened by one frame: `min_delay(..) + 1 >= len` | `src/kernel.rs` | `staged_idle` | RED — `left, D = 64, partition 65`, and `forced true, bank partition 1`. The first is the guard's boundary on the nose: at `D = 64` a 64-frame segment is legal and a 65-frame one is not. The second is the `D == 0` lane, whose tap is the row the frame itself writes |
| 26 | `fill_taps` uses `delay[0]` for every lane | `src/kernel.rs` | `staged_idle` | RED — `forced false, bank partition 1`. The per-lane stride of the pre-gather, which is row 1's mutation applied to the block-level gather |
| 27 | the delay test dropped from the guard, leaving only `len <= MAX_STAGED_FRAMES` | `src/kernel.rs` | `staged_idle` | RED — `left, D = 120, partition 127` and `forced false, bank partition 128` |
| 28 | the ring wrap dropped from `fill_taps`: `first = len` | `src/kernel.rs` | `staged_idle` | RED — `range end index 967 out of range for slice of length 961`, the out-of-bounds read the two-run split prevents. Row 4's mutation applied to the block-level gather |
| 29 | pass A of the staged body binds the linked levels the wrong way round: `let (level_right, level_left) = link_frame(..)` | `src/kernel.rs` | `staged_idle` | RED — `left, D = 960, partition 1` and `DualMono, bypass false, sidechain false, partition 1`. The three link laws, the bypass flag and a connected sidechain are all outside the frozen corpus's reach on the staged path, so they are covered here |

### Equivalent mutation (applied, GREEN, and recorded rather than gated)

| # | mutation | why it survives |
|---|---|---|
| 30 | `segment_is_stageable` returns `false` unconditionally, so every idle segment takes the per-frame body | Applied and run: **GREEN** in `staged_idle`, and necessarily so — that is the whole point of the guard. The staged body is a cost optimisation with no semantics of its own, exactly as the ramping/idle split of row 24 is, and no bit-identity test can distinguish a renderer from itself. What distinguishes the two is the benchmark: `examples/lane_sample_timing` reports 2.61 → 1.93 ns/lane-sample at `W = 8` with the staged body in and out. The mutation that *is* gated is the one that takes the staged body where it is illegal, which is rows 25 and 27 |
