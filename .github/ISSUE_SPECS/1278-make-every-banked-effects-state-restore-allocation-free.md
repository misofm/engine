# Make every banked effect's state restore allocation-free

Slice 9 of *Swap a rebuilt plan without an audio gap* (#1269). Code anchors verified on
`54b0a1bf8` (unchanged at `24029badb`). It touches only the effect crates and the conformance
differential, so it may run beside the umbrella's current feature slice.

## Product outcome

Every native effect that can bank can write and read one lane's state on the render thread: the
snapshot and restore payload calls, per node and per bank lane, allocate nothing, free nothing and
run in bounded time, and a restored lane continues **bit for bit** like the lane it was taken from,
even with a parameter ramp in flight. The plan-swap carry (slices 10-11b) copies effect lanes through
these calls in the swap block.

## Context

- The payload calls: `PreparedNativeEffect::snapshot_state_payload` and `restore_state_payload`
  (`crates/effect-contract/src/lib.rs:1913`, `:1955`), and
  `PreparedNativeEffectBank::snapshot_track_state_payload` and `restore_track_state_payload`
  (`:2046-2056`), with caller-owned buffers (`StatePayloadOutput`/`StatePayloadInput`,
  `:1657-1666`). Their docs treat them as control-path calls: "no engine path snapshots a bank at all
  since #1037" (`:2036-2042`).
- The true-peak limiter's restore allocates: `restore_track_state_payload` calls `read_lane` per
  channel (`crates/true-peak-limiter/src/lib.rs:4230-4240`), which builds a `LaneRestore` holding four
  `Box<[f32]>` (`:3932-3947`) with `vec![..].into_boxed_slice()` (`:4089-4114`), then commits it.
- The delay already validates in place and commits without allocating (`crates/delay/src/lib.rs:1314-1340`,
  "validating the rings in place is what removes the two ring-sized allocations"). It never banks
  and is moved, not restored, by the carry (slice 12), so it is out of scope.
- The conformance differential (`crates/conformance/src/randomized.rs`, issue #1051) already restores
  each lane's snapshot into a scalar instance and its bank lanes and compares the following words,
  and checks that `process` never allocates. It does not count allocations in the payload calls. Its
  former narrowing `SubnormalStateRefusedOnRestore` was removed by #1071 (1199b53f9).
- The compressor and the transient shaper persist a ramp as `current`, `target` and `remaining` and
  re-derive its `step` on restore (`crates/compressor/src/state.rs:96-111`;
  `crates/transient-shaper/src/lib.rs:31-34`), so a lane restored mid-ramp is not bit-identical to the
  continued one. The differential says so: "Restored against *continued* is not a contract mid-ramp"
  (`crates/conformance/src/randomized.rs:393-395`). Ramps are 64 samples and start at a block start,
  so they cross a block boundary whenever the quantum is under 64, which a session may declare
  (`crates/session/src/validate.rs:44-50`).
- The differential's own `snapshot_scalar` allocates its buffers (`randomized.rs:1429-1441`).
- Effects that bank: `parametric-eq`, `compressor`, `gate-expander`, `true-peak-limiter`,
  `soft-clip`, `transient-shaper`, `multiband-compressor`.

## Decisions frozen for this slice

- **D1. Two-pass restore.** Each restore first validates the whole payload in place, reading from the
  input bytes, then commits by reading the bytes again into the instance's existing storage. A
  rejected restore changes nothing (as today). No `Vec`, `Box` or other heap value is created or
  dropped in either pass.
- **D2. Snapshot.** Each snapshot writes straight into the caller's buffers with no temporary heap
  value.
- **D2a. Exact mid-ramp.** A payload carries every word a lane's continuation reads, including a
  ramp's `step`, so restore never re-derives a value it could read. The payload is not persisted
  (R6b), so its layout may change for this; update the effect's fixtures in the same commit.
  *Amended at attempt 2:* `state_layout_version` stays `1`. AGENTS.md gives a genuine version its
  sole prelaunch identity, V1 (`effect-compiler`'s `launch_native_state_layouts_are_v1` pins it),
  and the exact section lengths refuse a payload of the old layout. The attempt-1 verdict accepted
  this deviation; `docs/EFFECT_CONTRACT_V1.md` says the same since attempt 2.
- **D3. Realtime regions.** The four payload methods of every banked effect sit inside
  `REALTIME_POLICY` regions, so `scripts/check-realtime-policy.sh` scans them.
- **D4. Docs.** The trait docs say the payload calls are render-safe and that the plan-swap carry
  calls them at the swap block.
- **D5. Split if large.** If the seven effects exceed half a day, the implementer splits this issue by
  effect, the limiter first, and files the rest as successors with this title and an effect suffix.

## Deliverables

1. D1-D3 in each banked effect crate that needs it (audit all seven; fix the ones that allocate).
2. The conformance differential gains a **restored against continued** oracle: at random points,
   including mid-ramp and at quantum 32, a lane's snapshot restored into a fresh twin renders the
   same words as the original from then on. The comment at `randomized.rs:393-395` is replaced by
   this contract.
3. The conformance differential runs every payload call it makes (snapshot and restore, scalar and
   bank) inside a render scope, exactly as it runs `process`, so its audited allocator
   (`allocation_audit_is_real`, `crates/conformance/src/randomized.rs:245-258`) counts them, and
   requires zero. Preallocate `snapshot_scalar`'s buffers outside the audited scope.
4. D4.

## Authorized paths

- `crates/parametric-eq/src/`, `crates/compressor/src/`, `crates/gate-expander/src/`,
  `crates/true-peak-limiter/src/`, `crates/soft-clip/src/`, `crates/transient-shaper/src/`,
  `crates/multiband-compressor/src/` and their `tests/`
- `crates/effect-runtime/src/state_payload.rs`
- `crates/effect-contract/src/lib.rs` (docs only)
- `crates/conformance/src/randomized.rs`, `crates/conformance/tests/`

## Non-goals

- No layout change beyond D2a's exactness words.
- No graph, rack or host change. No carry.

## Objective gates

1. **Zero allocation.** For every banked effect, at `Backend::Simd8` and `Backend::Simd4`, scalar and
   bank: snapshot and restore of a lane after random processing make zero allocations and frees
   (deliverable 2, and the crate's own allocation harness where it has one).
2. **Fixtures.** Every effect's payload tests and the conformance fixtures pass, re-pinned only for
   effects whose layout D2a changed, each named with its reason:
   `cargo run --locked -p conformance --example conformance_fixtures -- --check`.
3. **Differential.** The conformance randomized differential, with deliverable 2's oracle, passes for
   every banked effect at both widths, with its existing narrowings only.
4. Commands:
   - `cargo test --locked -p parametric-eq -p compressor -p gate-expander -p true-peak-limiter -p soft-clip -p transient-shaper -p multiband-compressor -p conformance`
   - `cargo build --locked --release -p bench && timeout 180s bash scripts/trace-effect-contract-audit.sh target/release/bench 1000000`
   - `bash scripts/check-effect-contract.sh target/release/bench`
   - the umbrella's inherited gates.

## Test value

- Gate 1: the limiter's restore as it is today (four boxed rings per channel) turns it red; no
  existing test counts allocations in a payload call.
- Gate 3: a restore that re-derives a ramp's step (the compressor and transient shaper today) diverges
  from the continued lane mid-ramp at quantum 32; the new oracle turns red. No existing test compares
  a restored lane with the continued one.

## Dependencies

- *Soft-clip refuses its own subnormal snapshot on restore* (#1071): a carry needs restore to accept
  every snapshot the effect itself writes.

## Attempt record

### Attempt 1 (implementer, 2026-10-04)

**Audit of the seven banked effects.** The limiter's restore allocated (four boxed rings per
channel). The compressor and the transient shaper re-derived each ramp's step on restore; the
compressor also rebuilt its attack/release coefficient ramps from the parameter ramps. The EQ,
gate/expander, soft-clip and multiband already carried every word a continuation reads and
allocated nothing on the paths the differential reaches. Separately, a new probe found that the
gate/expander, the multiband and the limiter refused their **own** mid-ramp snapshot near a domain
edge (the #1071 overshoot class; soft-clip is fixed by #1071 attempt 2, 983ac85bd; the delay, out
of scope, still refuses).

**Changes.**

- `true-peak-limiter`: `read_lane` validates every ring, the history and the box-sum recompute in
  place in the payload bytes and returns only the scalar words; `commit_lane` copies the rings and
  the history from the bytes (D1). The coefficient bounds' rounding budget grew from 4 to 64 ulps:
  a ceiling ramp to -24 dB rounds more than 4 ulps past its endpoint, and its own snapshot was
  refused.
- `compressor`: the channel section grows from 22 to 37 words: every parameter ramp's
  `(current, target, step, remaining)` plus the attack/release coefficient ramps (D2a). Restore
  designs the non-rate coefficient words from the current values and takes attack/release from
  their coefficient ramps' current values. A moving ramp is validated over its whole remaining
  path (`effect_runtime::state_payload::ramp_path_within`, 64-ulp budget); a settled one strictly.
  `Channel::redesign` and `restore_rate_ramps` are deleted.
- `transient-shaper`: 11 to 14 words per channel (each ramp's step), the same path validation.
- `gate-expander`, `multiband-compressor`: a moving ramp's `current` and a subnormal step (both
  written by the effect itself) are validated by the path rule instead of the strict domain and
  `normal_or_zero`. No layout change.
- `effect-runtime::state_payload`: `RAMP_WORDS`, `write_ramp`, `read_ramp`, `ramp_path_within`.
- **Version: a deviation from D2a, because AGENTS.md takes precedence.** `state_layout_version`
  stays **1** for the compressor and the transient shaper. D2a says to bump it, but AGENTS.md gives
  a contract version its sole prelaunch identity, V1, and `effect-compiler`'s
  `launch_native_state_layouts_are_v1` (outside this slice's paths) pins every launch-native layout
  at 1. It went red on the bump. The payload is never persisted (R6b), and the section lengths
  that `maximum_state` declares refuse an old-layout payload. Re-pinned for the layout change: the
  compressor's `contract` sizes (88 to 148 bytes per lane, 176 to 296 total) and the transient
  shaper's (88 to 112 total), plus every test that addressed payload words by their old offsets.
- D3: `REALTIME_POLICY` regions around the four payload methods of EQ, compressor, gate, limiter,
  transient shaper and multiband, plus the compressor and limiter codec bodies. **Soft-clip is not
  marked**: #1071 owns `crates/soft-clip/`, so its markers are a follow-up. The floors in
  `scripts/check-realtime-policy.sh` were not raised (the script is outside this slice's paths).
- D4: the trait docs say the payload calls are render-safe, exact against the continued instance,
  and called by the plan-swap carry at the swap block.
- Conformance (`randomized.rs`): every payload call (scalar and bank, snapshot and restore) runs
  inside the audited render scope, with buffers preallocated outside it. The **restored against
  continued** oracle (`Continuation`): a lane's snapshot is restored into a freshly prepared
  instance, which then renders and reports beside the lane at every drawn quantum, 32 included;
  coverage requires continuations after automation. `run_scalar` restores into the twin only,
  replacing the `:393-395` comment with the contract. `EffectDifferential::assert_edge_ramps_restore`
  is a seedless probe: for each smoothed parameter, each domain edge and each quality row, it walks
  a ramp to the edge from a start whose `f32` walk overshoots, and restores the effect's own
  snapshot after every sample. The compressor, gate, limiter, transient-shaper and multiband
  `tests/randomized.rs` call it. The limiter's randomized differential is reinstated (dropped under
  #1051): it is the only test that audits the limiter's payload calls and compares a restored
  limiter lane with the continued one.
- No crate-local payload-allocation test was added beside the differential, because it would catch
  nothing the differential does not (test-value rule).

**Mutation evidence** (each applied, observed red, reverted, observed green):

- Limiter restore as it was (boxed rings): differential red, `restore_state_payload: forbidden
  operations inside a render-thread call: allocations: 8, deallocations: 8`. A `vec!` inside the
  marked `read_lane`: `check-realtime-policy.sh` red, and the differential red (2 allocations).
- Pre-change compressor (step re-derived): the continuation oracle red, `the twin rendered
  0xbfe1c7a9 where the oracle rendered 0xbfe1c7a6`. Pre-change transient shaper: red, `the instance
  restored from its snapshot rendered 0xc00191dd where the lane, continuing, rendered 0xc00191d3`.
- Compressor commit re-derives the parameter step:
  `payload::a_mid_ramp_restore_continues_the_ramp_exactly` and the differential red. Compressor
  rebuilds the coefficient ramps the old way:
  `kernel::payload_restore_resumes_an_active_coefficient_ramp_exactly`,
  `payload::an_active_attack_restore_continues_one_partition_invariant_coefficient_path` and the
  differential red. Transient shaper re-derives the step: differential red.
- Path validation reduced to its endpoints (loop removed):
  `payload::a_step_that_leaves_the_domain_before_the_snap_is_refused` red.
- A moving `current` held to the strict domain: `the_effects_own_edge_ramp_snapshots_restore` red
  in the compressor (threshold and ratio edges) and the transient shaper (attack and sustain
  edges). Before the fix, the probe reported 8 or more refusals in the gate (threshold, ratio,
  range, hysteresis), 8 or more in the multiband (thresholds, ratios, attack, release) and 4 in the
  limiter (ceiling, at every rate).
- Superseded and deleted: `kernel::payload_restore_reconstructs_an_active_coefficient_ramp_from_remaining`,
  `payload::a_mid_ramp_restore_arrives_on_the_same_sample` (it pinned the class-B re-derivation)
  and `payload::an_active_attack_restore_reconstructs_one_partition_invariant_coefficient_path`.

**Gates.**

- `cargo test --locked -p parametric-eq -p compressor -p gate-expander -p true-peak-limiter
  -p soft-clip -p transient-shaper -p multiband-compressor -p conformance` (plus effect-runtime,
  effect-contract, delay): pass. Soft-clip at 983ac85bd passes the differential and the edge probe.
- `cargo test --workspace --exclude soft-clip`: pass after the V1 correction (which it caught).
  Soft-clip was excluded only because #1071's work in progress was in the tree during the run.
- `cargo run --locked -p conformance --example conformance_fixtures -- --check`: pass, with
  nothing re-pinned.
- `trace-effect-contract-audit.sh target/release/bench 1000000`: ok. `check-effect-contract.sh`:
  ok (8 factories).
- fmt, clippy `-D warnings` (workspace minus soft-clip while #1071 was mid-edit), rustdoc
  `-D warnings`, workspace policy check/test, realtime policy check/test, `check-capi-abi.sh`,
  `audit capi` (0 allocations, 0 syscalls), `check-cross-targets.sh` (PASS, with only the #1018 iOS
  `memset_pattern16` expected failures): pass.
- Worklet chain (build `--named-twin`, check, expected resources, test): pass. **ARTIFACT
  CHANGED**: the shipped module is now `5aae9805d2a7ba72...`, because the effect crates in it
  changed. Not re-pinned (docs/RELEASE.md pins at release).

**Open.** Soft-clip's D3 markers and the edge probe in soft-clip's tests (after #1071's review).
The delay refuses its own edge-ramp snapshots (feedback, mix, cross feedback). Slice 12 moves the
delay rather than restoring it, but a successor should fix it.

### Attempt 1 amendment: soft-clip completion (before attempt 1's review)

#1071 passed review (attempt 2, `983ac85bd`; its review minors closed in the commit before this
one), so `crates/soft-clip/` is free and attempt 1's soft-clip items are completed here, as part of
attempt 1.

- **D3:** `REALTIME_POLICY` regions around soft-clip's whole payload codec (`write_lane_words`
  through `runtime_state_error`: snapshot sections, decode, apply, restore sections) and around
  the four payload methods (scalar and bank). `check-realtime-policy.sh` now reports 72 regions
  in 23 files; its floors (41, 12) are unchanged (the script is outside this slice's paths).
  The pure validation helpers the decode calls (`converted_value_valid`, `ramp_current_valid`,
  `ramp_step_valid`) sit in the parameter-conversion section, which `prepare` also uses, and are
  unmarked, as the other effects' helpers are.
- **Edge probe:** `crates/soft-clip/tests/randomized.rs` calls
  `EffectDifferential::assert_edge_ramps_restore`, as the other five crates do. It is kept beside
  #1071's seeded `a_restored_near_edge_ramp_continues_bit_for_bit`; neither supersedes the other.
  The probe is the harness-wide, seedless contract: every quality row, a restore after every
  sample, and it reaches all six soft-clip edges (drive, output and mix, bottom and top: 24
  violations at four rates under the strict-current mutation). It only asserts acceptance. The
  seeded test also renders the restored lane (scalar and a native bank lane) and compares it bit
  for bit. Under the strict-current, subnormal-mix-step and two off-by-one rest-guard mutations
  (`remaining <= 1`, `<= 2`) both turn red; under "accept, then clamp the current into range"
  (`MK`) the probe stays green and the seeded test is red (and so are five deterministic
  `state_roundtrip` cases). No mutation was found that only the probe catches for soft-clip; it
  is kept as the uniform contract every banked effect runs, whose position coverage is by
  construction rather than drawn.
- **Allocation-free proof:** soft-clip's randomized differential (`known: &[]`, `banks_natively:
  true`) runs every payload call inside the audited render scope (attempt 1's harness change).
  Mutations: a `vec![0; 4]` in `restore_sections` turns it red (`restore_state_payload: forbidden
  operations inside a render-thread call: allocations: 1, deallocations: 1`) and
  `check-realtime-policy.sh` red (marked forbidden body); a `Box::new(0)` in
  `snapshot_track_state_payload` turns it red (`snapshot_track_state_payload: ... allocations: 1`).
  Green after each revert. The differential runs the native bank width (eight lanes on this AVX2
  host); the four-lane path runs in CI's NEON and Wasm jobs.
- **#1071 attempt-1 MINOR-2 (inf in `X` with finite output): partially fixed, with no rendered bit
  changed.** `X` is `flush(2 * drive * x)`, which overflows to `±inf` for a finite input above
  about `2.7e36` at `+36 dB`. One such word in the interpolation window gives `±inf`, the cubic
  clamps it to `±2/3`, and the output stays finite, so D7 never fires and the effect holds the
  infinity for 31 samples; its own snapshot was refused (`effect.state.history`). The restore now
  accepts `±inf` in `X` (`x_history_word_valid`); `NaN` stays refused in every history and `inf`
  stays refused in `e` (the cubic is bounded) and in the dry history. Only the control-plane
  decode changed: the kernel, the snapshot and every rendered bit are untouched (console digests
  and conformance fixtures unchanged; see the gates). New test
  `a_snapshot_holding_an_overflowed_x_word_restores_and_continues_bit_for_bit` (`1e37` left and
  `-1e37` right at `+36 dB`, mix 0.5 and 1.0, D7 not fired, output finite, one infinity per
  channel in `X`; restore, snapshot identity, 128 samples bit for bit) and rejection row
  `bad(43, inf)` (`e` age 0).
  Mutations: `X` back to zero-or-normal: the new test red (`effect.state.history`); `X` accepting
  any non-finite: `bad(12, NaN)` red; `e` accepting infinities: `bad(43, inf)` red.
- **Open successor item (soft-clip non-finite history, not fixed here).** Two cases still leave
  the effect holding a word its own restore refuses, so the carry would drop that lane to rest:
  1. *Identity path, two overflows.* With mix `0` and output `0 dB` (`identity`, the output is
     the dry path), two inputs of `±1e37` within the interpolation window at `+36 dB` (e.g. at
     samples 118 and 120 of a 128-sample block) put two infinities in `X` whose tap-weighted sum is `inf - inf`, so
     the interpolation is `NaN`; `e` then holds `NaN` (`0xffc00000` on x86-64) for 30 samples
     while the identity output stays finite. Off the identity path the same input makes the
     output `NaN` and D7 resets the lane, so nothing is held.
  2. *Non-finite input.* An `inf` or `NaN` input sample in a block's last 31 samples reaches the
     output only after the 31-sample dry delay, so the dry history (and `X`) hold it at the block
     boundary and the snapshot is refused until D7 fires a block later.
  Fixing either needs a decision this slice cannot make: accept `NaN` in `e` and non-finite dry
  words (weakening #1071's hostile-word gate, and the NaN payload bits are platform-dependent), or
  have D7 also check the histories at the block boundary (a render-path change that moves rendered
  bits for these inputs). Reproducer: prepare at drive `+36 dB`, output `0 dB`, mix `0`; render
  128 samples of a sine at `0.5` with `1e37` at 118 and `-1e37` at 120 (case 1) or `inf` at 120
  (case 2); `restore_state_payload` of the snapshot returns `effect.state.history`.

- **Gates (this amendment, on the committed tree):** fmt, workspace clippy `-D warnings`, rustdoc
  `-D warnings`, workspace policy check/test, realtime policy check/test (72 regions, 23 files),
  `check-capi-abi.sh`, `audit capi` (0 allocations, 0 deallocations, 0 locks, 0 syscalls),
  `cargo test --locked -p soft-clip -p conformance -p effect-compiler`, `cargo test --locked
  --release -p console-workload` (console digests unchanged), `conformance_fixtures -- --check`
  (unchanged), `check-cross-targets.sh` (PASS; only the #1018 iOS rows, soft-clip
  `memset_pattern16` 22, at its ceiling): all pass. Worklet chain (build `--named-twin`,
  `check-web-audioworklet.sh --without-metadata-regeneration`, expected resources, test): pass.
  **ARTIFACT CHANGED**: the shipped module is now `0f508f8ca5b3849d...` (attempt 1 built
  `5aae9805...`), because soft-clip's restore decode, which is compiled into it, changed. Not
  re-pinned (`docs/RELEASE.md`).

### Attempt 2 (implementer, 2026-10-04)

**Attempt-1 verdict: FAIL** (`submix-verdicts/1278-attempt1.md`). The product work was confirmed:
every banked effect's payload calls are allocation-free on the render thread and a lane restored
mid-ramp continues bit for bit, quantum 32 included; the limiter's allocation-free restore and the
`state_layout_version = 1` deviation were accepted. Findings: MAJOR-1, soft-clip's call of the
shared edge probe had no unique catch (the seeded `a_restored_near_edge_ramp_continues_bit_for_bit`
dominates it); MINOR-1, the compressor's coefficient-ramp bound admitted negative coefficients,
which make the smoother diverge; MINOR-2, the limiter widened its 4-ulp bound to 64 ulps for
targets and settled ramps too and had no path check; MINOR-3, the gate, multiband, transient
shaper and EQ marked only their trait wrappers, not their codec bodies; MINOR-4, the coverage
clause counted continuations after automation, not in flight, and the multiband reached 3
mid-ramp continuations and none at quantum 32; MINOR-5, stale docs; NIT-1 floors; NIT-2 soft-clip
in-flight current.

**Changes.**

- MAJOR-1: `crates/soft-clip/tests/randomized.rs`'s `the_effects_own_edge_ramp_snapshots_restore`
  is deleted. `EffectDifferential::assert_edge_ramps_restore` keeps its five other callers.
- MINOR-1: `effect_runtime::state_payload::ramp_path_inside(ramp, (low, high), max_remaining)` is
  the walk over an explicit closed interval (`ramp_path_within` now delegates to it with the
  symmetric slack). The compressor holds a coefficient ramp's target and a settled current to
  `[0, 1]` exactly and a moving path to `[0, 1 + 64 ulps]`: the budget is one-sided, because the
  smoother's pole is `1 - c` and every `c < 0` diverges. Comment corrected. New test
  `payload::a_coefficient_below_zero_or_above_its_design_is_refused` (settled `-1e-6` current, a
  settled current and a target 8 ulps above one, a moving path to `-9e-7`; refused with
  `effect.state.parameter` and nothing moved; a moving path inside `[0, 1]` restores).
- MINOR-2: the limiter's `read_lane` holds a coefficient ramp's target to the unrelaxed designed
  range, a settled current to its target's bits, and a moving ramp to `ramp_path_inside` over the
  64-ulp relaxed bounds (`coefficient_bounds` now only shapes the path), which also bounds its
  step. Four rejection rows in `state_round_trips_and_rejects_corruption`: settled limit one ulp
  above the ceiling, target one ulp above, a moving limit walked out by a `-1e30` step, a settled
  limit with a nonzero step. The linked randomized scenario's crafted in-flight limit ramp walked
  to zero or infinity, which is now refused; it now walks toward a range edge (factor 0.5 or 0.9)
  and snaps back, and is still accepted (19 of 20 moving crafted restores accepted in the debug
  run, measured once with a temporary print).
- MINOR-3: `REALTIME_POLICY` regions now cover the codec bodies: gate `write_lane` through
  `restore_lane` and `rederive_lane` (the commit calls it); multiband `lane_value` through
  `Instance::restore` (`write_side`, `stage_side`, `commit_side`); transient shaper `snapshot`
  through `state_error` (`restore`, `write_lane`, `read_lane`); EQ `RestoredBand` and
  `Channel::{snapshot_track, snapshot_cut_enables, restore_track}`, and `runtime_state_error`
  through `PreparedParametricEq::restore_track` (`read_payload`, `write_payload`). Pure
  helpers they call (design functions, `Channel::new`, value validators) stay unmarked, as in the
  other effects. `check-realtime-policy.sh` now reports **78 regions in 23 files** (attempt 1: 72
  in 23); its floors (41, 12) are unchanged, the script being outside this slice's paths
  (follow-up: raise them to 78/23).
- MINOR-4: `run_width` tracks, per lane, the sample before which a ramp its automation started may
  be in flight (last span sample + 64, `IN_FLIGHT_SAMPLES`; every banked launch effect smooths over
  64 samples or updates), and takes a continuation on such a lane with chance 1/2 at each boundary
  (replacing a running one), else as before. Coverage counts `continuations_in_flight` and
  `continuations_in_flight_at_32`, replacing `continuations_after_automation`, and
  `assert_reached` requires both when a bank was compared. Measured in the default runs
  (continuations / in flight / in flight at quantum 32): EQ 226/206/55 (wrong: no EQ ramp ever
  started; see attempt 3), compressor 291/210/49, gate 234/179/33, limiter 174/157/21, soft-clip
  84/79/7, transient shaper 242/211/50, multiband 120/57/9. The multiband's **actual** mid-ramp continuations, decoded once with temporary prints of
  its staged ramps' `remaining`: 44 of 119 (7 at quantum 32), against 3 (0 at quantum 32) at attempt
  1; no continuation the proxy called settled had a ramp moving.
- MINOR-5: `crates/compressor/tests/ramps.rs` header (words `1 + 4i`..`4 + 4i`, step carried),
  `crates/transient-shaper/tests/contract.rs` doc of the 63/64 test (step carried; the mutation
  re-run below), `crates/transient-shaper/tests/MUTATIONS.md` row 4 and its section,
  `crates/compressor/tests/MUTATIONS.md` row 13 and the `payload` row. D2a amended above.
  `docs/EFFECT_CONTRACT_V1.md` (authorized for this attempt): the state section now says nothing
  persists a payload and the payload calls are render-safe for the plan-swap carry; the W2-D2 bump
  sentence now says a prelaunch layout change keeps version `1`.

**Width.** On this AVX2 host the compressor, transient shaper, gate, soft-clip and EQ decline
`Four` by design; their payload calls run at `Eight` here and their four-lane (`Simd4`) path runs
only in CI's `aarch64-debug`/`aarch64-release` legs (and is compiled into the simd128 worklet). The
limiter and the multiband bind both widths here.

**Mutation evidence** (each applied, observed red, reverted, observed green):

- Compressor coefficient bound back to attempt 1's symmetric `ramp_path_within((0, 1), 64 ulps)`:
  `a_coefficient_below_zero_or_above_its_design_is_refused` red ("a settled negative current").
  Target check removed: red ("a target above one"). Settled-current check removed: red ("a settled
  current above one"). Path lower bound `-64 ulps`: red ("a moving path below zero"). The path walk
  reduced to its endpoints (in `ramp_path_inside`): red.
- Limiter: target check removed: `state_round_trips_and_rejects_corruption` red ("limit target one
  ulp above the ceiling was accepted"); settled bits check removed: red ("settled limit one ulp
  above the ceiling"); path walk reduced to endpoints: red ("moving limit walked past its bounds
  by its step"); settled `+0.0` step rule removed: red ("settled limit with a nonzero step");
  attempt 1's check restored whole: red. Path slack back to 4 ulps: the limiter's
  `the_effects_own_edge_ramp_snapshots_restore` red (its own ceiling ramp to -24 dB refused at
  every rate), so the 64-ulp path budget is still needed.
- Realtime regions: `let _probe = vec![0_u8; 4];` in the transient shaper's `read_lane`, the
  gate's `parse_lane` and `rederive_lane`, the multiband's `stage_side` and the EQ's
  `Channel::restore_track`: `check-realtime-policy.sh` red ("marked realtime forbidden-body
  predicate") for each.
- In-flight coverage: the preferential take disabled (`if false && ...`): the multiband's
  differential red ("no restored instance rendered beside its continued lane with a ramp in
  flight, at quantum 32 included"); the other six stay green. A multiband restore that re-derives
  a moving step: red in the new harness (the continuation oracle, "the instance restored from its
  snapshot rendered 0xbe960b42 where the lane, continuing, rendered 0xbe960b45"), and also red in
  attempt 1's harness through the restore round trip; the multiband carries no other ramp word a
  restore could re-derive (its `BandCache` is a memo keyed by the ramp values), so no mutation was
  found that only the in-flight preference catches beyond the coverage clause itself.
- Transient shaper `read_lane` replacing the carried step with `(target - current) /
  RAMP_SAMPLES`: `automation_updates_one_sixty_three_sixty_four_retargets_and_restores_exactly`
  red (`tests/contract.rs:239`).

**Test value (new or rewritten tests).**

- `compressor payload::a_coefficient_below_zero_or_above_its_design_is_refused`: a coefficient
  bound that lets a payload hold a negative (divergent) or above-design coefficient; attempt 1's
  bound is such a defect, and no other test is red on it.
- limiter `state_round_trips_and_rejects_corruption`'s four new rows: a target or settled
  coefficient held only to the relaxed bounds (a ceiling above the effect's own), a path checked
  at its endpoints only, a settled ramp with a stale step.
- `assert_reached`'s in-flight clauses: a harness that stops taking continuations mid-ramp, or never
  at quantum 32 (red on the multiband with the preference removed).

**Gates (this attempt, on the committed tree).** fmt; clippy `--workspace --all-targets
--all-features -D warnings`; rustdoc `-D warnings`; workspace policy check/test; realtime policy
check/test (78 regions, 23 files); `check-capi-abi.sh`; `audit capi` (0 allocations, 0
deallocations, 0 locks, 0 syscalls, 100k calls); `check-cross-targets.sh` (PASS; only the #1018
iOS `memset_pattern16` expected failures); `cargo test --locked -p parametric-eq -p compressor -p
gate-expander -p true-peak-limiter -p soft-clip -p transient-shaper -p multiband-compressor -p
conformance -p effect-compiler -p effect-runtime -p effect-contract`; `cargo test --locked --release
-p console-workload` (console digests unchanged); `conformance_fixtures -- --check` (nothing
re-pinned); `cargo build --release -p bench`, `trace-effect-contract-audit.sh target/release/bench
1000000` (ok) and `check-effect-contract.sh` (8 factories): all pass. Worklet chain (build
`--named-twin`, `check-web-audioworklet.sh --without-metadata-regeneration`, expected resources,
test): pass. **ARTIFACT CHANGED**: the shipped module is now `1928ba471a57c016...` (attempt 1
amendment: `0f508f8c...`), because restore decode compiled into it changed (compressor and limiter
bounds). Not re-pinned (`docs/RELEASE.md`).

**Open (successors, unchanged).** The delay refuses its own edge-ramp snapshots (feedback, mix,
cross feedback; slice 12 moves rather than restores it). Soft-clip's two open non-finite history
cases (attempt 1 amendment). Soft-clip validates an in-flight current by its line, not by
`ramp_path_within` (verdict NIT-2, #1071's design). Raise `check-realtime-policy.sh`'s floors to
78 regions / 23 files.

### Attempt 3 (implementer, 2026-10-04)

**Attempt-2 verdict: FAIL** (`submix-verdicts/1278-attempt2.md`). Every attempt-1 finding was
confirmed fixed and every gate green, but MAJOR-1: the conformance differential never started a
parametric-EQ ramp (the EQ refuses raw spans, #807, and the harness never applied a prepared
target), while the in-flight counter credited it with 206 continuations (55 at quantum 32); an EQ
restore that re-derives a moving step as `(target - current) / remaining` stayed green in every
test. NITs: the compressor accepted a coefficient of exactly `0.0`; the shared ramp walks sat
outside a realtime region; the EQ restore designed each band's coefficients twice.

**Changes.**

- MAJOR-1, fix (a): `run_width` now drives prepared targets for an effect with
  `target_preparation()` (the EQ). At a third of the boundaries (after the continuation take) it
  draws a candidate that moves one to three continuous values of one lane (both channels on a
  mono scenario), prepares it off the audited scope, and applies every target inside render
  scopes to the scalar instance, the bank lane, the disengaged mono arm's lane and the lane's
  continued twin; they must accept or refuse together. New coverage `prepared_targets`,
  `applied_targets`, `moving_targets`; `assert_reached` requires a moving target where a bank
  was compared.
- The in-flight counter no longer over-reports. A raw span credits a lane only when that block's
  report counts no invalid span (the EQ counts every raw span invalid, so it is never credited);
  a prepared target credits only when accepted and its words differ from the lane's known
  heading; any reset clears every lane (a discontinuity snaps ramps), and a restore of anything
  but the lane's own untouched snapshot clears that lane. New `EffectDifferential::in_flight`:
  an effect's own snapshot reading; given, the counters count decoded mid-ramp snapshots
  exactly. The EQ's `tests/randomized.rs` gives it (`remaining`, word 14 of each band). The
  macro takes an optional trailing `in_flight:`; the two direct constructors (EQ and multiband
  ignored tests) say `None` / the decoder.
- **Product defect found by the new harness, fixed.** With ramps driven, the EQ refused its own
  mid-ramp snapshots: `Channel::restore_track` walked the remaining path under the design's norm
  limit (`NORM_TOLERANCE`, one `f32` rounding above contractive), but the render's `f32` walk sits
  a few ulps above it near the contractive edge. Seed 0 refused lane 1's snapshot at section 3;
  a grid of every band kind with frequency, gain and Q moves at 48 kHz found 649 refused ramps,
  a 10 kHz bell at Q 0.1 from -24 dB to +24 dB at every sample. The walk is now held to
  `RAMP_PATH_NORM_TOLERANCE = 1 + 2^-12` (worst excess measured on that grid: `3.7e-6`, about
  `2^-18`; a forged path gains at most `(1 + 2^-12)^64 < 1.016` before the target snaps in).
  Designs and settled words keep the exact limit. This defect would have refused the carry of
  any EQ lane caught mid-ramp near the contractive edge.
- Option (b) as well, as the EQ's own gate: `crates/parametric-eq/tests/carry.rs`
  `a_mid_ramp_restore_continues_bit_for_bit_at_every_sample` restores at every sample 0..=64 of
  three prepared-target ramps (a bell moving in frequency, gain and Q; the 10 kHz swing above;
  the first ramp retargeted 17 samples in) into a fresh scalar instance and a fresh bank lane (the
  last lane) at every width this build binds, and compares 100 following frames bitwise.
- NIT-3, fixed: `Channel::decode_track` validates a lane and designs each band once (carried in
  `RestoredBand::words`), `Channel::commit_track` commits; `PreparedParametricEq::restore_track`
  decodes both channels, then commits both. The two candidate `Channel::new` builds (a design for
  every band of every lane) and the second validation pass are gone: a lane restore now designs
  six bands per channel, not four passes plus `2 W * 6` candidate designs.
  `Channel::restore_track` remains for the unit tests (`#[cfg(test)]`). New
  `a_restore_refused_on_one_channel_moves_neither` (scalar and bank lane) defends the all-or-none
  order the refactor now owns.
- NIT-1, fixed: the compressor holds a target and a settled coefficient to `(0, 1]` (no legal
  time designs `0.0`; the smallest is about 2.1e-6). Rows "a settled zero current", "a zero
  target". The moving path keeps the closed `[0, 1 + 64 ulps]`.
- NIT-2, fixed: `effect_runtime::state_payload`'s word and ramp codec (`write_u32` through
  `ramp_path_inside`) is one `REALTIME_POLICY` region. `check-realtime-policy.sh` now reports
  **79 regions in 24 files**; its floors are unchanged (follow-up: raise to 79/24).

**Coverage, default runs** (continuations / in flight / in flight at quantum 32): EQ 141/58/26
(decoded exactly; 320 targets applied, 295 moving), compressor 294/213/48, gate 214/157/29,
multiband 111/50/7, soft-clip 69/61/6, transient shaper 209/173/37, limiter 169/148/20.
Measured once with the verifier's snapshot decoder as a temporary print (actual mid-ramp /
proxy): compressor 209/213, gate 155/157, multiband 50/50, soft-clip 55/61, transient shaper
168/173, limiter 144/148. Before the decoder the EQ's proxy read 94 with 57 actual: an accepted
target that moves only a disabled band's parameters is stationary. Hence the decoder.

**Mutation evidence** (each applied, observed red, reverted, observed green):

- EQ `commit_track` storing a moving step as `(target - current) / remaining`: `carry.rs` red
  (342 of 390 scalar and bank comparisons, all three ramps); the differential red ("the instance
  restored from its snapshot rendered 0xbeb91443 where the lane, continuing, rendered
  0xbeb91440", quantum 64, 88.2 kHz); `contract.rs` and `bank.rs` stay green.
- EQ path walk back to `NORM_TOLERANCE`: `carry.rs` red (the 10 kHz swing refused at samples
  0-63); the differential red ("lane 1's snapshot refused by a fresh instance").
- EQ commit of the left channel before decoding the right: `a_restore_refused_on_one_channel_
  moves_neither` red ("the scalar instance moved"); no other EQ test is red.
- Compressor bound back to `[0, 1]`: `a_coefficient_below_zero_or_above_its_design_is_refused`
  red ("a settled zero current"); every other compressor test green.
- `vec![0_u8; 4]` in `ramp_path_inside`: `check-realtime-policy.sh` red ("marked realtime
  forbidden-body predicate").

**Test value.** `carry.rs` mid-ramp: a restore that re-derives the carried step, or refuses the
effect's own path, caught deterministically at every sample (the EQ's `tests/randomized.rs`
differential catches them only on some seeds: M1 on 10 of 24, M2 on 3 of 24). `carry.rs` one-channel refusal: a restore that
commits one channel before the other is validated. Compressor rows: a settled or target
coefficient of `0.0`. The differential's prepared-target drive: by reach, it now applies accepted
moving EQ targets at random boundaries, through scalar, bank, collapsed arm and restored twin.

**Gates (on the committed tree).** fmt; clippy `--workspace --all-targets --all-features -D
warnings`; rustdoc `-D warnings`; workspace policy check/test; realtime policy check/test (79
regions, 24 files); `check-capi-abi.sh`; `audit capi` (0 allocations, 0 deallocations, 0 locks,
0 syscalls, 100k calls); `check-cross-targets.sh` (PASS; only the #1018 iOS `memset_pattern16`
expected failures); `cargo test --locked` for parametric-eq, compressor, gate-expander,
true-peak-limiter, soft-clip, transient-shaper, multiband-compressor, conformance,
effect-compiler, effect-runtime, effect-contract; `cargo test --locked --release -p
console-workload` (console digests unchanged); `conformance_fixtures -- --check` (nothing
re-pinned); `trace-effect-contract-audit.sh target/release/bench 1000000` and
`check-effect-contract.sh`: all pass. Worklet chain (build `--named-twin`,
`check-web-audioworklet.sh --without-metadata-regeneration`, expected resources, test): pass.
**ARTIFACT CHANGED**: the shipped module is now `477f3f1c6c53c828...` (attempt 2: `1928ba47...`):
the EQ restore and the compressor bound are compiled into it. Not re-pinned (`docs/RELEASE.md`).

**Open (successors).** As attempt 2, with the realtime floors now 79 regions / 24 files.

### Verdicts

| Attempt | Verdict file | Result | Summary |
|---|---|---|---|
| 1 (`c2808bb04` + amendment `46ef4f263`) | `1278-attempt1.md` | FAIL | MAJOR (test value): soft-clip's edge-probe test caught nothing existing tests miss; product work confirmed allocation-free and bit-exact mid-ramp; 5 MINOR (one a compressor hostile-payload regression), NITs. |
| 2 (`a070cfa7d`) | `1278-attempt2.md` | FAIL | MAJOR: the conformance differential never started a parametric-EQ ramp and its in-flight counter over-reported; product code correct. |
| 3 (`251113c8f`) | `1278-attempt3.md` | PASS | EQ ramps driven, honest in-flight count, `RAMP_PATH_NORM_TOLERANCE = 1 + 2^-12` sound and derivable; 3 NIT (state the tolerance derivation, inaccurate `carry.rs` test-value sentence, an overlong compressor comment). |

### Phase-1 follow-ups

Closed (batch follow-ups commits on `codex/seamless-swap`; comments and records only, so no
new test):
- NIT-1: `crates/parametric-eq/src/lib.rs`'s `RAMP_PATH_NORM_TOLERANCE` rustdoc states the
  derivation (convexity, `L <= 4`, `u = 2^-25`, the induction to `2^-22 + 65 * 4 * 2^-25`, about
  `8.0e-6`, at every rate and through any retarget chain), with the `3.7e-6` grid measurement kept
  as a cross-check.
- NIT-2: the attempt-2 test-value sentence above now says `carry.rs` catches M1 and M2
  deterministically and the EQ's randomized differential only on some seeds.
- NIT-3: `crates/compressor/src/state.rs`'s coefficient comment is reflowed to 100 columns.
- The realtime-policy floors: `scripts/check-realtime-policy.sh` now requires 25 files and 87
  regions, the merged tree's counts (`realtime policy: ok (87 marked regions in 25 files)`), and
  `scripts/test-realtime-policy.sh` pads its fixture with thirteen files and forty-six regions so
  it sits exactly on both floors; its file-floor and region-floor mutation rows still turn red
  (`realtime policy mutation tests: ok`).

Stays open (successors, listed in #1269's "Phase 1 status"): the delay refuses its own edge-ramp
snapshots (feedback, mix, cross feedback); soft-clip's two open non-finite history cases;
soft-clip validates an in-flight current by its line, not by `ramp_path_within`. The EQ's `Simd4`
legs are verified only in CI aarch64.

CI follow-up (PR #1299, run 37196277320, `AArch64 product crate debug tests (NEON Simd4, FPCR)`):
the multiband differential's reach clause failed on arm64 with `continuations_in_flight_at_32: 0`.
A generator-reach defect, not a product one. The quantum was drawn per seed, and under
`RampCutsMoveBits` automation was on in half the scenarios. Both of the twelve seeds that drew
quantum 32 drew no automation at four lanes. An AVX2 build passed only because its second, `Eight`
pass draws its automation lane again. Reproduced on x86 with `Backend::current()`, `Native`,
`Backend::VECTOR` and `BankWidth::ALL` forced to `Simd4`/`Four` in a scratch copy: the coverage
matched CI's word for word. The fix (`crates/conformance/src/randomized.rs`), applied to every
differential that banks natively:
- the quantum is indexed by the seed (`QUANTA[seed % 5]`, so seed 3 runs at 32);
- `Shape::automate` alternates automation every five seeds (seed 3 automates);
- the first boundary with a ramp in flight always takes the continuation.
Evidence: all eight effect differentials pass at native `Eight` and at forced `Four`. Multiband
`continuations_in_flight_at_32` is 15 at `Eight`+`Four` and 9 at `Four` alone. Taking no
continuations, or none while a ramp is in flight, turns the clause red at both widths. The
scalar-only path (the delay) still draws its quantum, so its scenarios are unchanged. Indexing it
too exposed a separate, width-independent product defect, which needs a successor issue. The
delay's `read_ramp` re-derives a ramp's step from `(current, target, remaining)` on restore, so an
instance restored mid-ramp drifts 1-2 ulp from the instance it continues. `run_scalar`'s
restored-against-continued check catches it (forced `Four` seed 4 with the quantum indexed; x86,
old generator, `MISO_ENGINE_RANDOMIZED_SCALE=20`, seed 53). No product path reaches it today: the
carry moves the delay (slice 12). The nightly's scale-100 run includes seed 53. Its red was
confirmed here only in the debug profile, not in the nightly's release profile.
