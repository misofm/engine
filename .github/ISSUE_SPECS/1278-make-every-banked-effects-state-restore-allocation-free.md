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
  (R6b), so its layout may change for this; bump the effect's `state_layout_version` and update its
  fixtures in the same commit.
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
