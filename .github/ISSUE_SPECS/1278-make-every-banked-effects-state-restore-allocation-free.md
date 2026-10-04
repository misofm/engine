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
  narrowing `SubnormalStateRefusedOnRestore` (`:79-87`) is owned by #1071.
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
