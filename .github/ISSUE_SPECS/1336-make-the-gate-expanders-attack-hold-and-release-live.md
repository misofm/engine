# Make the gate-expander's attack, hold and release live

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-1, D15-7, D15-13 E2).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

A host changes `miso.gate-expander`'s attack (id 5), hold (id 6) or release (id 7) on a playing
strip, as a console slot or an insert, and hears the change at the next block with no click and no
plan rebuild: the browser's effect-parameter command (kind 5) admits it, and the C ABI commits it as
a live update. Decision 14's finding F2 is closed for the gate. A bank lane renders the same bits as
the same track rendered alone.

## Context

- The three rows are `AutomationRate::None`, `SmoothingRule::None`, 0 samples
  (`crates/gate-expander/src/lib.rs:174-213`). Decision 14 records no reason for that
  (`docs/rulings/live-update-versus-rebuild-2026-10-04.md`, "Launch effects, by descriptor" and F2).
- Times become coefficients once per lane: `rederive_lane` (`lib.rs:476`) writes
  `coef.hold_samples = floor(ms * fs / 1000 + 0.5)` (`rounded_samples`) and
  `coef.attack`/`coef.release = attack_release_coefficient(ms, fs)`
  (`crates/effect-runtime/src/envelope.rs:36`, `1 - exp(-1000 / (ms * fs))`).
- The kernel reads them from the shared `GateCoef` (`crates/gate-expander/src/kernel.rs:54`):
  the hold reload at `kernel.rs:232` and the one-pole rate select at `:246`,
  `g += c * (C - g)` with `c = attack` when the target is above `g`, else `release`.
- Only the first four parameters ramp (`RAMP_COUNT = 4`, `kernel.rs:12`); `apply_automation`
  refuses any span with `parameter_index >= RAMP_COUNT` (`lib.rs:534`, `:562`).
- The state payload holds 22 words per channel (`STATE_LANE_HEADER_WORDS`, `lib.rs:51`): gain,
  open flag, hold countdown, the three times (`write_lane`, `lib.rs:704`), then four ramp quadruples
  from `STATE_RAMP_WORD = 6` (`:868`). `commit_lane` (`:819`) re-derives the coefficients from the
  times on restore, so a coefficient in flight would not survive a carry today.
- **The precedent.** `miso.compressor`'s attack and release are live. A retarget sets the time ramp
  and a coefficient ramp to `rate_coefficient(target)` over `SMOOTHING_SAMPLES`
  (`crates/compressor/src/kernel.rs:144-171`), and the coefficient word follows that ramp each sample
  (`:191-229`). The coefficient is interpolated linearly between two exact designs.
- Hosts need no change. The C ABI classifier makes any `Block` effect parameter live
  (`crates/host-core/src/live_delta.rs:441`); the browser admits it (`hosts/host-web/src/lib.rs:4513-4518`).
- The randomized bank differential draws automation for every non-`None` parameter
  (`crates/conformance/src/randomized.rs:1752-1764`), so it reaches the new rows by itself.

## Decisions frozen for this slice

- **D1. Descriptor.** Attack and release become `AutomationRate::Block`, `SmoothingRule::Linear`,
  64 samples (as the four live rows). Hold becomes `AutomationRate::Block`, `SmoothingRule::None`,
  0 samples (a legal pair, `crates/effect-contract/src/lib.rs:2625`). Domains, defaults and mappings
  do not change. `GATE_SPECS` (`lib.rs`, "The parameter domains") stays equal to the descriptor.
- **D2. Attack and release: a coefficient ramp.** Each channel's `GateState` gains two `GateRamp`
  lane words, `rates[0]` (attack) and `rates[1]` (release), and the kernel reads `c` from
  `state.rates[i].current` instead of `GateCoef`; `GateCoef.attack` and `.release` are deleted.
  A span for time `t` on lane `l` sets `timing.attack_ms` (or `release_ms`) to `t` and retargets the
  lane's rate ramp with `LinearRamp::set_target(attack_release_coefficient(t, fs), RAMP_SAMPLES)`
  from its current coefficient (`crates/effect-runtime/src/ramp.rs:79`). The RAMPING prologue of
  `channel_step` advances the two rate ramps exactly as it advances the four parameter ramps
  (`kernel.rs`, `if RAMPING`), and `ramp_frames_left` covers them. The time itself does not ramp: the
  `timing` words hold the committed time, and the coefficient is what moves.
- **D3. Equations and update rule.** With `c0` the coefficient in effect and `c1 = 1 - exp(-1000 /
  (t * fs))`, for `k = 1..64`: `c[k] = c[k-1] + (c1 - c0) / 64`, and `c[64] = c1` exactly (the D11
  snap). The recurrence is `g[n] = flush(g[n-1] + c[n] * (C[n] - g[n-1]))`. A one-pole with
  `0 < c < 2` is stable; every `c[k]` lies within rounding of `[min(c0, c1), max(c0, c1)]`, and over
  the launch domains and rates `c` stays inside `[5.2e-6, 0.203]` (release 2000 ms at 96 kHz, attack
  0.1 ms at 44.1 kHz). So the envelope never overshoots `C` and the gain stays continuous: no sample
  step beyond what either endpoint design produces.
- **D4. Hold: the window keeps its start.** Hold is an integer count and moves no sample value
  itself; the release one-pole already smooths the close it times. A span for hold `h` sets
  `timing.hold_ms = h`, computes `H1 = rounded_samples(h, fs)`, and with `H0 = coef.hold_samples`
  and `r` the lane's countdown:
  - an open lane: `r' = max(0, r + H1 - H0)`. This is the countdown the lane would show had its last
    reload used `H1`: lengthening extends the current window, shortening ends it sooner, and
    `r' <= H1` holds because `r <= H0`;
  - a closed lane: `r' = min(r, H1)`;
  - then `coef.hold_samples = H1`. All values are integers below `2^24` (at most 96,000), so the f32
    words are exact. The next reload uses `H1` (`kernel.rs:232`).
  This is the one live value of this slice with no ramp, and the reason is the one above: it changes
  when the target switches, never a sample value.
- **D5. Banking.** Every new word is per lane and every update is per lane, on the control side of
  the block or in the shared lane prologue. A resting ramp's prologue update is an exact no-op, so
  the `RAMPING` split stays partition-invariant (`run_block`, `lib.rs:604`). A lane's bits never
  depend on a bank-mate's records.
- **D6. Payload.** The channel section grows from 22 to 30 words: two rate quadruples
  `(current, target, step, remaining)` after the four parameter quadruples. Restore checks each rate
  target bit-equal to `attack_release_coefficient` of its time word, and a moving rate ramp's path
  with `payload::ramp_path_within` over the coefficient domain of D3 (the 64-ulp slack rule the
  parameter ramps use, `parse_lane`, `lib.rs:743`). Restore commits the rate words as written and no
  longer re-derives the attack and release coefficients; hold is still re-derived and the countdown
  still checked `<= hold_samples`. `STATE_LAYOUT_VERSION` stays 1: there is no persisted state (R6b),
  both sides of a carry are one build, and `payload::validate_lengths` refuses a 22-word section.
- **D7. Resets.** `FullToDefaults` (and D7 recovery) seeds the rate ramps at rest on the default
  designs. `DiscontinuityKeepParameters` snaps each rate ramp's `current` to its `target`.
- **D8. Denormal and NaN.** Spans are validated against `GATE_SPECS` before any write, as today. A
  coefficient is a normal number in the D3 interval, so its step `(c1 - c0) / 64` is normal or zero.
  The envelope keeps its `flush`. A non-finite gain still trips the lane-local D7 recovery.
- **D9. Cost.** At most two scalar `exp` calls (vendored `math`, D6 of the runtime) per lane, channel
  and record, on the block's control side, as the compressor already pays; two more lane selects
  and adds per sample only while a rate ramp is in flight.
- **D10. Carry.** After this slice a changed attack, hold or release is a live value: under D15-7 the
  successor carries the lane, then the classifier's ramped record retargets it. The carried payload is
  D6's, so a ramp in flight crosses a swap bit-exactly.

- **D11. Citations and evidence.** The one-pole ballistics and the attack/release time-constant
  convention: Giannoulis, Massberg and Reiss, *Digital dynamic range compressor design*, JAES 2012
  (the gate shares the compressor's smoothing stage); linear interpolation of a designed coefficient
  between two stable designs, as the compressor does (above). The implementation note in the gate's
  crate docs states D3-D8 and records one listening pass (a drum loop through the gate while attack
  and release sweep their domains), as evidence, not as a gate.

## Deliverables

1. D1-D10 in `crates/gate-expander`.
2. `crates/dsp-reference/src/gate_expander.rs`: a reference that applies one timing change at a
   given frame with D3's f64 coefficient ramp and D4's hold rule (the static reference is unchanged).
3. Tests of the gates below; the regenerated parameter metadata and SDK catalogue.

## Authorized paths

- `crates/gate-expander/src/lib.rs`, `crates/gate-expander/src/kernel.rs`,
  `crates/gate-expander/tests/` (stream A owns the payload code in `lib.rs`, `write_lane` to
  `restore_lane`: coordinate D6 with stream A's #1279/#1280 owner before editing it)
- `crates/dsp-reference/src/gate_expander.rs`
- `sdk/assets/miso-engine-v1-parameter-metadata.json` and `sdk/src/generated/catalog.ts`, regenerated
  only (`node sdk/codegen/assets.mjs`, then `node sdk/codegen/generate.mjs`)

- `crates/host-core/tests/live_delta.rs`, the gate half of `prepared_parameter_changes_need_a_rebuild`
  (`:1073-1086`) only: a gate attack change now classifies live (one `EffectControlRecord::Parameter`),
  so move that assertion to the live cases and keep the test's prepared subject on the EQ half (which
  *Make a parametric EQ band's enabled and kind live*, #1337, flips) or, if #1337 has landed, on the
  limiter's lookahead (id 3, still prepared). Stream B owns the file: coordinate the merge.

## Non-goals

- No host change and no grammar change. No change to the compressor or to the other gate rows.
- The browser comments that say no launch effect declares `AutomationRate::None`
  (`hosts/host-web/src/lib.rs:1041-1043`, `:4513-4514`) are already stale on `main` and stay stale
  until #1337 and #1338 land too; `hosts/host-web` belongs to stream H, which corrects them.
- No tail change (`TailSamples::Finite(0)` stands; *State a bounded tail and an exact-rest bound for
  every node* (#1329) owns tails).

## Hazards

- `GateCoef` loses two words, so any test that builds one by hand changes with it.
- `crates/gate-expander/src/gate_digests.in` and the console benchmark must not move: at rest the
  kernel reads the same coefficient bits from `state.rates` as it read from `coef`.

## Objective gates

1. **Coefficient ramp.** Per lane at 48 kHz: retarget attack 1 -> 50 ms, then release 100 -> 2000 ms
   mid-ramp. Every sample's coefficient equals an independent `LinearRamp` fed the same targets, bit
   for bit; it never leaves `[min(c0, c1), max(c0, c1)]` by more than one ulp; sample 64 is
   bit-equal to `attack_release_coefficient` of the new time; a retarget mid-ramp starts from the
   live coefficient, not the old target.
2. **Hold rule.** Scalar, a lane held open (open, countdown 3,000 of `H0 = 4,800` at 48 kHz) with
   the input below the re-arm band: hold 100 -> 20 ms gives `r' = 0`, and the gate starts releasing
   in that block's first frame; 100 -> 200 ms gives `r' = 7,800`, and it closes after exactly 7,800 more
   holding frames; a closed lane keeps `r = 0` and the next opening reloads `H1`.
3. **Oracle.** The rendered gain of a corpus signal with an attack change and a hold change mid-render
   agrees with deliverable 2's f64 reference inside `TOLERANCE_DB` (`tests/oracle.rs:44`, never
   loosened), scalar and native bank.
4. **Bank bit-identity.** The randomized differential (`tests/randomized.rs`, 24 seeds), which now
   draws spans for ids 5-7, passes at both widths against scalar instances, whole and chunked,
   continued and restored.
5. **Carry mid-ramp.** A snapshot taken at frame 10 of an attack ramp, restored into a fresh lane at
   every launch rate, renders bit-identical to the uninterrupted lane; a payload whose rate target
   differs from the design of its time word by one ulp is refused with `effect.state.parameter`.
6. **Live on the C ABI.** `the_parameter_readback_after_a_live_edit_equals_a_rebuilds` and
   `live_effect_parameter_edits_render_like_the_browsers_lane` (`crates/capi/src/runtime/live_tests.rs`)
   now cover ids 5-7 through `eligible_values`, with no edit to them.
7. **Unchanged.** `gate_digests.in` and the console benchmark workload counts unchanged; the
   descriptor and `GATE_SPECS` agree; allocations during a block that applies all three records are 0
   (`bench_support::alloc` thread counters).
8. Commands:
   - `cargo test --locked -p gate-expander -p dsp-reference -p conformance -p capi`
   - `cargo test --locked --release -p audit -p bench -p console-workload`
   - `cargo run --locked -p conformance --example conformance_fixtures -- --check`
   - `bash scripts/check-sdk-generated.sh`
   - `bash scripts/check-realtime-policy.sh`, `bash scripts/check-effect-runtime-policy.sh`,
     `bash scripts/check-workspace-policy.sh`
   - `bash scripts/build-web-audioworklet.sh --named-twin <B> <A>`, then
     `bash scripts/check-web-audioworklet.sh <A> <B>/miso-engine-v1-audio-worklet.simd128.named.wasm`,
     `python3 -B scripts/check-browser-expected-resources.py --artifacts <A>` and
     `bash scripts/test-web-audioworklet.sh`
   - `bash scripts/check-cross-targets.sh`; `scripts/run-aarch64-tests.sh` (CI `aarch64-debug` and
     `aarch64-release` when no arm64 host)
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`;
     `cargo fmt --all -- --check`

## Test value

- Gate 1: a coefficient that jumps to the new design at a record (the envelope's rate steps), or
  retargets from the old target, turns it red; nothing tests the gate's rate words today.
- Gate 2: a hold that only applies at the next reload (shortening ignored) or a countdown left above
  `H1` turns it red.
- Gate 3: a wrong rate direction or a hold rule that differs from the reference's turns it red.
- Gate 5: a restore that re-derives the coefficient from the time (today's `commit_lane`) moves bits
  mid-ramp and turns it red.
- Gate 7: a record path that allocates on the render thread turns its allocation count red.
- Gates 4 and 6 are existing tests whose generators now reach the new rows; they need no new code.

## Dependencies

- *Carry console effect lanes across a plan swap* (#1279)
- *Carry live-controlled effect lanes across a plan swap* (#1280)
