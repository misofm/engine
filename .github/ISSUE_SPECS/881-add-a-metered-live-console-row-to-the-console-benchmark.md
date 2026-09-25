# Add a metered live-console row to the console benchmark

## Product outcome

The standing console benchmark (`scripts/run-console-benchmark.sh`, `tools/bench/src/console.rs`) measures 64 tracks with no meters and no automation. The product workload is a browser DAW with a meter on every track and a fader moving. Two of the largest optimisation candidates (meter kernel, route fold under observation) are invisible on the current rows. This issue adds one descriptive row that makes them measurable. Tooling only; no engine change.

## Root evidence

- `tools/bench/src/console.rs` module doc (lines ~60-110) lists the rows: `sixty_four_track_console`, `_eq_only`, `_compressor_only`, `_builtins_only`, `_dispatch_only`, `_idle`, plus `console_hoist` and `console_automation` (one Point span per block through the live console queue).
- Meters are bound at prepare through `prepare_selected_session_builtins_between_render_calls` (`crates/host-core/src/prepare.rs`, the `SelectedMeterRequest` construction near line 1110 builds one request per track when `meter_period_frames` is set; defaults near line 1117 set `peak_hold_frames: 0`).
- `GraphNodeObserverBinding::new` vs `::controlled` at `crates/builtins-compiler/src/lib.rs:3582-3590` select the permanent vs controlled policy.

## Smallest closable slice

Authorized paths: `tools/bench/src/console.rs`, `scripts/run-console-benchmark.sh`, `scripts/console-benchmark-validator.jq`, `scripts/console-benchmark-record-validator.jq`, `scripts/console-benchmark-record-lib.jq`, `scripts/check-console-benchmark-fixture.sh`, `scripts/test-console-benchmark.sh`, and this spec.

Add one row, `sixty_four_track_console_metered`: the `sixty_four_track_console` plan with a `SAMPLE_PEAK` meter selected at `PostMatrix` on every track (the same request shape the web host builds, `hosts/host-web/src/lib.rs` near line 7734), bound through the same host-core prepare entry point the web host uses, with the meter lease held and every published snapshot consumed each block so the observer path is exercised. Record `us_per_block` with the same metadata as the other rows. Extend the validators so the new record shape is accepted and any missing field is rejected.

## Non-goals

No engine, host or effect change. No new automation arm (reuse `console_automation` if a moving fader is wanted later). No threshold; the row is descriptive.

## Objective gates

1. `scripts/test-console-benchmark.sh` passes with the new row and rejects a record missing it.
2. `scripts/check-console-benchmark-fixture.sh` passes.
3. One descriptive run: `sixty_four_track_console_metered` and `sixty_four_track_console` recorded back to back, attached as the artifact directory the script writes. Report the ratio; make no claim about it.
4. `scripts/check-bench-policy.sh` and `scripts/test-bench-policy.sh` pass.

## Dependencies

None. Unblocks measurement for "Bank-wide lane meter kernel for peak and count metrics" and "Keep the route fold eligible when the folded lane is observed".

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A means the change moves no rendered bit: same arithmetic in the same order, fewer passes, loads, stores, copies or branches. Every gate below that says "bit-identical" is a hard stop, not a tolerance.
- Render paths stay allocation-free, lock-free and syscall-free (`scripts/check-realtime-policy.sh` is mandatory). Only `crates/lane` may name `wide` or intrinsics (`scripts/check-lane-policy.sh`).
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, and the focused tests named below before every checkpoint. Commit on a `codex/<issue>-<slug>` branch from synchronized `main`; do not touch paths outside the authorized list.
- Do not quote a projected saving. If a benchmark row is listed, run it exactly once, one warmup and two measured rounds, and attach the record as descriptive evidence.
- Source of these findings: `docs/audits/render-path-cost-audit-2026-09-24.md` (PR #879) and tracker #349.
