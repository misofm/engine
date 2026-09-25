# Bank-wide lane meter kernel for peak and count metrics

## Product outcome

Each meter reads the resident AoSoA block one lane at a time with stride W through a scalar loop of about thirty instructions per sample with several data-dependent branches; a meter on each of 64 tracks costs more per block than all the builtins arithmetic. Replace the per-lane pass with one `Lane`-generic kernel that processes all W lanes of a bank at once for the metrics whose result is order-independent: sample peak, held peak, sanitised count, clipped count. Class A for those metrics.

## Root evidence

- `crates/builtins/src/lib.rs:4394` `MeterInput::samples` yields `chunks_exact(stride).map(|f| f[lane])`; `:4574` `observe_input` and `:4810`/`:4858` `observe_segment`/`observe_selected_segment` run per lane; `crates/builtins-compiler/src/lib.rs:4690` `observe_resident` calls them once per lane per block.
- `sanitize_gain_block` in `crates/lane/src/kernels/builtins.rs` already accumulates counts as `1.0 & mask` sums; the meter module forbids `f32::max` and requires select-form maxima (`crates/builtins/src/lib.rs` near line 4807).
- The energy (RMS) accumulator is an `f64` running sum whose summation order is pinned; reordering it is class B and is excluded here (#349 RT-10 step 2(b)).

## Smallest closable slice

Authorized paths: `crates/lane/src/kernels/builtins.rs` (new kernel), `crates/lane/tests/` (kernel identity test), `crates/builtins/src/lib.rs` (meter accumulator dispatch), `crates/builtins-compiler/src/lib.rs` (call `observe_resident` once per bank instead of once per lane, when all lanes share the metric set), their tests, and this spec.

Add `meter_peak_counts_block<L: Lane>(io: &[f32], frames, state: &mut [LaneMeterWords; ...])` that, per frame, loads one vector per channel and updates per-lane peak (select), held (select, honouring the hold/decay rule exactly as the scalar code does, or delegating to the zero-hold shortcut if present), sanitised and clipped counts (mask and add). Keep the `f64` energy accumulation on the existing scalar per-lane path, untouched. Wire the bank-wide kernel in for the four metrics and reduce to the existing per-lane snapshot words.

## Non-goals

No change to energy/RMS words, to snapshot cadence, to the observer binding policy, or to the wasm/native lane widths.

## Objective gates

1. New lane test: for random blocks at scalar, `Simd4` and `Simd8` widths, the kernel's per-lane peak, held, sanitised and clipped words are bit-identical to the current scalar per-lane loop on the same data, including `NaN`, `-0.0`, subnormal and `+inf` lanes.
2. Existing meter fixtures unchanged (`scripts/check-builtins-fixtures.sh`); energy/RMS snapshot words byte-identical before and after on the fixtures.
3. `scripts/check-lane-policy.sh`, `scripts/check-builtins-policy.sh`, `scripts/check-realtime-policy.sh` pass.
4. One descriptive run of the metered console row (see "Add a metered live-console row to the console benchmark") before and after, attached; no claim about the ratio.

## Dependencies

"Add a metered live-console row to the console benchmark" for gate 4 (may be run without it by omitting gate 4 and saying so). "Skip the held-peak state machine when hold is zero and decay is off" is independent but simplifies the held update.

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A means the change moves no rendered bit: same arithmetic in the same order, fewer passes, loads, stores, copies or branches. Every gate below that says "bit-identical" is a hard stop, not a tolerance.
- Render paths stay allocation-free, lock-free and syscall-free (`scripts/check-realtime-policy.sh` is mandatory). Only `crates/lane` may name `wide` or intrinsics (`scripts/check-lane-policy.sh`).
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, and the focused tests named below before every checkpoint. Commit on a `codex/<issue>-<slug>` branch from synchronized `main`; do not touch paths outside the authorized list.
- Do not quote a projected saving. If a benchmark row is listed, run it exactly once, one warmup and two measured rounds, and attach the record as descriptive evidence.
- Source of these findings: `docs/audits/render-path-cost-audit-2026-09-24.md` (PR #879) and tracker #349.
