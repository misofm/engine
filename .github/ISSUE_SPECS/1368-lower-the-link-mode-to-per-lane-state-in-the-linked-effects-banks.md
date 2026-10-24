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
   effects, the other launch effects, and the test descriptors in `effect-contract`,
   `effect-compiler`, `conformance`, `graph`, `builtins-compiler` and `host-web`).
2. D3-D5 in the four effect crates.
3. Tests per gate.

## Authorized paths

- `crates/effect-contract/src/lib.rs`
- `crates/compressor/src/`, `crates/gate-expander/src/`, `crates/transient-shaper/src/`,
  `crates/true-peak-limiter/src/`, and each crate's `tests/`
- the descriptor literal only (every full `EffectDescriptor { .. }` literal on `main`):
  `crates/delay/src/lib.rs`, `crates/multiband-compressor/src/lib.rs`,
  `crates/parametric-eq/src/lib.rs`, `crates/soft-clip/src/lib.rs`, `crates/conformance/src/effect.rs`,
  `crates/graph/src/lib.rs` (`:3603`, `:6588`), `crates/builtins-compiler/src/lib.rs` (`:6020`),
  `crates/effect-contract/tests/response_analysis.rs` (`:72`),
  `crates/effect-compiler/tests/native_session.rs` (`:78`), `hosts/host-web/src/tests.rs` (`:6857`).
  The `..original` literals in `crates/conformance/tests/effect_contract.rs` need no edit.
- `scripts/check-web-audioworklet-callgraph.py` (`KERNEL_ROSTER`, `:204`: the four effects' rows
  only, if an arm's symbol is renamed or a general arm is added)
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

- *Carry console effect lanes across a plan swap* (#1279)
- *Carry live-controlled effect lanes across a plan swap* (#1280)

Stream A's carry slices #1279 and #1280 for these four crates land first (decision 15's stream
order), so this slice rebases on their payload code instead of racing it.

## Decision record

- **2026-10-24, split (root).** The slice ships in two tranches, as the hazard allows:
  **1368a** is D1, D2, the gate-expander and the compressor; **1368b** is the transient shaper and
  the true-peak limiter. Until 1368b lands, `lane_link` is `true` for `miso.compressor` and
  `miso.gate-expander` only: a descriptor may set it only when its bank kernel reads the link per
  lane, so the shaper and the limiter keep the real mode in their key and render as today.
- **2026-10-24, D1 shape (root).** Option A, the self-describing descriptor field, not an id-keyed
  list beside `program_key`: each effect describes itself (#1340 removed the analogous
  `PREPARED_BYPASS_EFFECTS` list). `EffectDescriptor` has no `Default`, so each full literal gained
  `lane_link: false` (19 literals in 18 files; the `..base` literals need no edit).
- **1368a facts.** Gate-expander: the link words were already per-lane; the bank now sets each
  lane's `link_max`/`link_avg` from its member's mode (`with_lane_links`). Compressor: the kernel's
  `link: LinkMode` parameters became one `kernel::Links<L>` record (per-lane `linked`/`averaged`
  masks plus the all-DualMono reduction that picks the #984 `abs` arm), built once at bind from
  each member's mode. D5: a padded lane takes the first member's mode in both effects. A uniform
  bank has the masks a single-mode instance had, so no bit moves.
- **Forced edit outside the list.** `crates/conformance/src/randomized.rs`: its heterogeneous-cohort
  draw treated a member at another link mode as another program; for a `lane_link` effect it is now
  expected to bank.
- **1368a cost (gate 4, descriptive).** The first run of each arm was refused before any
  workload launched (`loadavg_above_ceiling`: other streams' builds were running). The rule is to
  record a refusal and not retry; the run below is a retry, taken after the load average fell
  under the ceiling, so its numbers are descriptive only. One console run each on base
  `24a850abb` and on the candidate (controlled, one warmup, two rounds, cpu 31). The host has no perf counters, so the
  records carry no `isolated_cycles_per_lane_sample`; p50 per block instead:
  `sixty_four_track_compressor_only` +0.30 %, `sixty_four_track_console` +0.30 %; the worst row
  is untouched EQ-only, +1.6 %. No workload row runs the gate-expander. Every record's
  `output_sha256` is equal to the base's (no bit moved).
- **Attempt 2: the render-time lane swap (#1747).** Once the key carries no link mode, two lanes
  at different modes pass `can_swap_lanes`/`lane_swap_fits`, and the mode was positional (the
  compressor's `Links`, the gate's `link_max`/`link_avg`), so a swapped strip rendered at the other
  lane's law. The lane's mode is now one word of its prepared defaults
  (`effect_contract::exchange_lane_link_word`, `LANE_DEFAULT_BYTES` + 4 in both effects): the
  compressor stores `[LinkMode; MAX_WIDTH]` and rebuilds `Links` after an exchange, and the gate
  rewrites the lane's link words. Each crate's `swap_tests` probe now draws a link per lane; both
  were red before the fix.
- **Plan-swap carry (#1279) across a link-mode change: no defect.** The carry matches lanes on
  `EffectLaneLayout` (effect, layout version, sizes, bypass, latency), which never held the link
  mode, so the key change does not alter which lanes carry. Neither effect's state payload holds
  a link word, so the successor keeps its own prepared mode.
- **1368b facts.** `lane_link` is now `true` for `miso.transient-shaper` and
  `miso.true-peak-limiter` as well. Transient shaper: the shaper keeps `[LinkMode; W]` and a
  `LinkLanes` record (per-lane `linked`/`averaged` masks plus the arm). A bank whose lanes share one
  mode takes that mode's const arm, as before; a mixed bank takes a fourth arm, `LINK_MIXED`, which
  computes `max` and `0.5 * l + 0.5 * r` in the const arms' order and selects per lane (no blend).
  Limiter: `LimiterCoef.link_max: bool` became a per-lane mask plus `link_all` (every lane
  `Maximum`); the #990 linked pair needs `link_all`, so a bank with any `DualMono` lane renders the
  dual body, which renders a `Maximum` lane's bits equal to the linked pair's. The #990 record is a
  statement about gain words only, so it needs no update on a lane swap. D5 in both: a padded lane
  takes the first member's mode. Swap: both put the mode in the lane's defaults (+4 bytes,
  `exchange_lane_link_word`) and re-derive the masks; each crate's `swap_tests` probe now draws a
  link per strip, red when the exchange drops the mode.
- **Limiter latency.** The declared latency is `N + 6` per rate and the main delay does not read
  the link, which only selects the peak a lane's gain path reads; the mixed-link tests compare each
  lane from the first block, line fill included, so no lane's latency moved.
- **Web roster.** The shaper's bank constructor is now outlined and carries the follower
  coefficients' four control-side `f32x4.sub`, so the row's pattern matched two functions. The
  row is pinned to `Shaper<f32x4>::process_block` (narrower, not looser); the constructor is not a
  render kernel and stays under the generic shape rule. The body went from 522 to 700 vector
  operations, 0 scalar (the mixed arm). The limiter bodies are unchanged at 913 and 424.
- **1368b cost (descriptive).** One console run on each arm (base `fd2a49d84`, candidate), with the
  build done first and the run under the exclusive lock. The runner refused both before any workload
  launched: base `loadavg_above_ceiling smt_sibling_busy`, candidate `loadavg_above_ceiling` (other
  streams' builds). Recorded and not retried. The cost row waits for a later unloaded run; it is
  not a gate for this issue (root, 2026-10-24).
- **1368b class A (PR evidence, not committed).** Every console `*output_sha256` (60 digest
  fields over 37 records of one untimed `bench console` warmup run, `MISO_ENGINE_BENCH_ROUND=warmup`)
  is identical on base `fd2a49d84` and on this branch. The committed corpus digests are unchanged
  (`CROSS_TARGET_DIGESTS`, `D90_DIGESTS`; `wasm-gates` `g5_native_corpus`, the shaper's
  `cross_target` and the limiter's `determinism` pass in release).
- **Roster reason (root accepted).** One row pinned to the block body rather than two rows: a row
  pinned to the outlined constructor would fail as a false de-vectorisation the day LLVM inlines it
  again, and the constructor runs on the control side only.
