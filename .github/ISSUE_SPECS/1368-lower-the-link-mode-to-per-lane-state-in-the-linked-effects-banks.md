# Lower the link mode to per-lane state in the linked effects' banks

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-13 E4).
Slice L1 of *Let a strip override a console slot's link mode* (#1236).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

Two strips that run the same compressor, gate-expander, transient shaper or true-peak limiter at
different detector link modes share one bank. Nothing a host sends changes yet, and no existing
session renders a different bit: every bank whose lanes share one mode takes today's path. This is
the foundation for a per-strip console link override (#1369) and a live link switch (#1370, #1371).

## Context

- The prepared link mode is a field of `PreparedEffectMetadata` (`crates/effect-contract/src/lib.rs:1118`)
  and of `EffectProgramKey` (`:1171-1186`, field `:1179`; built by `program_key`, `:1188`). A rack
  chain's bank program is its slots' keys (`crates/graph-compiler/src/banks.rs:186-194`), and the
  bind check compares each member's key (`:644`). So two instances that differ only by link mode
  never share a bank today.
- Each member of a bank request carries its own `PrepareEffectRequest`, link mode included
  (`EffectBankPreparation.link_mode`, `crates/effect-compiler/src/prepare.rs:82`; the bank factories
  iterate `request.requests`, e.g. `crates/multiband-compressor/src/lib.rs:1560-1568`).
- How each kernel uses the link today (see #1236 "Context"):
  - compressor: `Invariants::new` splats `linked`/`averaged` masks from one mode
    (`crates/compressor/src/kernel.rs:290-313`), `link_frame` selects per lane (`:316-339`), the
    `dual_mono` settled arm is chosen per block (`:595`, `:610`; also `:1161`, `:1292`, `:1425`);
  - gate-expander: per-lane words `link_max`/`link_avg` splatted from one mode
    (`crates/gate-expander/src/lib.rs:425-426`), selected in `kernel.rs:205-207`;
  - transient shaper: a const-generic `link` (`crates/transient-shaper/src/lib.rs:286`) dispatched
    per block from `metadata.link_mode` (`:494-498`);
  - true-peak limiter: `LimiterCoef.link_max: bool` per bank (`crates/true-peak-limiter/src/lib.rs:408-416`),
    a lane mask in the per-lane body (`:2741`, `:2948`), and the linked-pair decision
    `link_max && gain_linked && designed_gain_agree` per block (`:3588-3594`).
- The precedent: #1087 took `bypass` out of the bank key for effects that bank (`prepare.rs:444-456`).

## Decisions frozen for this slice

- **D1. Descriptor flag.** `EffectDescriptor` gains `lane_link: bool`. It is `true` for exactly
  `miso.compressor`, `miso.gate-expander`, `miso.transient-shaper` and `miso.true-peak-limiter`, and
  `false` for every other descriptor (the multiband follows in #1367).
- **D2. Key.** `PreparedEffectMetadata::program_key` writes `LinkMode::DualMono` into the key's
  `link_mode` when the descriptor's `lane_link` is set. The metadata itself keeps the instance's
  real mode, so a scalar instance and every diagnostic are unchanged.
- **D3. Per-lane state.** Each of the four bank factories reads each member's `link_mode` and
  builds per-lane state from it:
  - compressor: `linked` and `averaged` masks stored per lane in the bank (not rebuilt from one
    mode per block);
  - gate-expander: `link_max` and `link_avg` lanes from each member;
  - transient shaper: per-lane `linked` and `averaged` masks used by a general arm (selects in
    `link`'s operation order);
  - limiter: a per-lane `link_max` mask in `LimiterCoef`.
- **D4. Arms chosen per block from the lanes.** A bank whose active lanes all share one mode takes
  today's arm: the compressor's `dual_mono` arm when every lane is `dual_mono`; the transient
  shaper's const arm for that mode; the limiter's linked-pair path when every lane is `maximum` and
  agrees. A mixed bank takes the general (select) arm. Each arm computes every lane's words exactly
  as that lane's own mode would, so no lane's bits depend on its bank-mates. The test is one mask
  reduction per block, computed when the lanes' modes change (at bind here; at a switch in #1370).
- **D5. Padding.** A padded lane takes the mode of the bank's first active lane, so padding never
  forces the general arm.

## Effect evidence (AGENTS.md list)

- Equations, coefficients, latency, tail, smoothing, NaN and denormal behaviour: unchanged. Only
  where the link law's selector lives changes.
- Citations: Giannoulis, Massberg and Reiss, "Digital Dynamic Range Compressor Design", JAES 60(6),
  2012 (the detector link); ITU-R BS.1770-5 (the limiter's true-peak detector).
- Listening evidence: not required; a lane renders its own mode's bits.

## Deliverables

1. D1 and D2 in `crates/effect-contract`; the descriptor field set in every descriptor (the four
   effects, the other launch effects, and the test descriptors in `effect-contract`, `conformance`,
   `graph` and `builtins-compiler`).
2. D3-D5 in the four effect crates.
3. Tests per gate.

## Authorized paths

- `crates/effect-contract/src/lib.rs`
- `crates/compressor/src/`, `crates/gate-expander/src/`, `crates/transient-shaper/src/`,
  `crates/true-peak-limiter/src/`, and each crate's `tests/`
- the descriptor literal only in `crates/delay/src/lib.rs`, `crates/multiband-compressor/src/lib.rs`,
  `crates/parametric-eq/src/lib.rs`, `crates/soft-clip/src/lib.rs`, `crates/conformance/src/effect.rs`,
  `crates/graph/src/lib.rs`, `crates/builtins-compiler/src/lib.rs`
- `crates/graph-compiler/tests/` (gate 2)
- Stream A edits payload code in the same effect crates; this slice changes no payload.

## Non-goals

- The session grammar (#1369), the ramp (#1370), any record or classifier (#1371).
- The multiband compressor's link (#1367).

## Hazards

- If one effect does not fit in half a day, split it into its own issue; the limiter is the
  likely one (its linked-pair record).

## Objective gates

1. **Per effect, both widths.** At `Simd4` and `Simd8`, a full bank whose lanes take every
   supported mode (mixed), fed the seeded signal with ramps in flight: each lane is bit-identical to
   its scalar instance at its own mode. A bank whose lanes all share one mode takes today's arm (a
   test-support arm counter) and renders today's bits.
2. **One bank per slot.** A console of the four effects over eight tracks whose strips use mixed
   modes (built by preparing each strip's effect at its mode): `graph-compiler` binds one bank per
   slot per group, and every lane equals a fresh plan of that strip alone.
3. **No bit moves.** Every checked-in render digest and corpus digest is unchanged; the console
   benchmark workload counts are unchanged (`cargo test --locked -p console-workload`).
4. **Cost.** On the console workload, each touched effect's settled all-`dual_mono` record keeps
   `isolated_cycles_per_lane_sample` within 2% of `main` (one invocation, one warmup, two measured
   rounds). If not, the slice stops and reports the gap for owner ruling.
5. **Realtime.** Each bank's render path still allocates nothing (each crate's `no_alloc_render`
   or equivalent test).
6. Commands:
   - `cargo test --locked --all-targets -p effect-contract -p compressor -p gate-expander -p transient-shaper -p true-peak-limiter --features math/lane,lane/test-support`
   - `cargo test --locked -p graph-compiler -p console-workload --features graph/test-support`
   - the touched effects' `KERNEL_ROSTER` rows; `bash scripts/build-web-audioworklet.sh --named-twin <B> <A>`,
     `bash scripts/check-web-audioworklet.sh <A> <B>/miso-engine-v1-audio-worklet.simd128.named.wasm`,
     `python3 -B scripts/check-browser-expected-resources.py --artifacts <A>`, `bash scripts/test-web-audioworklet.sh`
   - `bash scripts/check-cross-targets.sh`; `scripts/run-aarch64-tests.sh` (CI `aarch64-debug`/`aarch64-release` when no arm64 host)
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`;
     `cargo fmt --all -- --check`; `bash scripts/check-workspace-policy.sh`

## Test value

- Gate 1: an arm chosen from one lane's mode (lane 0's, say) renders a neighbour at the wrong law;
  it turns red on the mixed bank. Nothing banks mixed modes today.
- Gate 2: a key that still carries the real link splits the slot; it turns red.

## Dependencies

- *Carry console effect lanes across a plan swap* (#1279) and *Carry live-controlled effect lanes
  across a plan swap* (#1280): stream A's carry slices for these four crates land first (decision
  15's stream order), so this slice rebases on their payload code instead of racing it.
