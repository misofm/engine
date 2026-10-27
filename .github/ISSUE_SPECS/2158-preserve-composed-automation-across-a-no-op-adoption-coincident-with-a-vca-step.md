Filed by root from fresh Astra medium verification of #2096 (2026-10-27). This is a distinct existing running-ramp case; #2096 fixes no-ramp adoption.

## Problem and product outcome

A table-only plan replacement with no session change, adopted at sample 1,100 while a continuous own-fader ride coincides with a VCA step, changes the first audible output sample compared with uninterrupted rendering. The lane still has 52 ramp samples remaining, so the no-ramp comparison corrected by #2096 does not apply.

The replacement must render the uninterrupted lane's bits while retaining the real VCA step's entry jump under B17-R13. Suppressing the step to obtain equality is not a fix.

## Stateless reproducer

Use `a_carried_automated_lane_renders_the_uninterrupted_bits` in `crates/builtins-compiler/src/lib.rs`, delivered by #2096. Its `entry-step` row has a constant own curve to isolate the boundary-jump guard. Replace its own-ride expression:

```rust
ride_table(4_096, -6.5, if case == "entry-step" { -6.5 } else { -2.0 })
```

with:

```rust
ride_table(4_096, -6.5, -2.0)
```

For an isolated reproducer, restrict the existing cases to `case == "entry-step"`; keep the composed layout, `continued_cells = 0`, 100-frame blocks and adoption at sample 1,100. The VCA steps from -6 dB to -12 dB at 1,100, the own ride runs from -6.5 dB to -2 dB over 4,096 samples, and the session jump length is 480 samples. Run:

```sh
cargo test --locked -p builtins-compiler a_carried_automated_lane_renders_the_uninterrupted_bits
```

Observed first difference: the first post-adoption block, lane 1 left, sample 0, actual bits `1049322181`, uninterrupted bits `1049321647` (Simd8). The existing entry-time comparison runs at 1,100 with 52 stage-ramp samples remaining.

Fresh independent verification transplanted only this restricted test onto production from main `7be08dd1856ef9d86c1b85199a973515a1fca41f`, without any #2096 production changes: the same case is red. Thus this is a pre-existing defect rather than a regression introduced by #2096. Local evidence is preserved outside worktrees in `engine-work-evidence/easy-fixes-20261027/fresh-astra/coincident-step-base.log` and `check-coincident-base.py`; the reproduction above requires neither file.

## Smallest closable slice and decisions

Correct only the running composed lane's no-op adoption at an entry discontinuity, using the existing adoption/composed-target rules. Start in `crates/builtins-compiler`; touch `crates/automation` only if the shared composition owner requires the correction. Keep unrelated automation features and host API changes out.

This is Class B because it corrects audible words in the defective case. Root's brief must state the exact adoption and coincident-event ordering before implementation, preserve B17-R13, and explain its authority under the existing same-node-time continuity ruling. No new implementation has been authorized by this filing.

## Gates

- Fold the moving-own/entry-step reproducer into the owning carry test: red on the baseline production, bit-equal to uninterrupted after the fix.
- Preserve #2096's held/free, repeated-adoption and on/off-grid cases, plus the existing step-at-entry jump behavior. Exercise the owning test's scalar, W4 and W8 backends.
- Existing automation and builtins-compiler suites pass, including their realtime allocation gates; no render allocations, locks or syscalls are added.
- One test per claim: extend the owner instead of creating a second fixture framework or output-digest owner.

## Dependency

#2096 supplies the expanded regression. This issue does not block its bounded no-ramp correction.


## Approved brief and attempt 1 (2026-10-27)

Root approved the product ordering before implementation, under #1716 fix D and #2096's same-node-time continuity rule: a table-only/no-op Reposition at node time T preserves the ordinary merged walk's arithmetic and event precedence on a running own ride. The real VCA step at T is processed once, at its existing ordering and prospective completion target. Adoption must introduce neither a second mismatched retarget nor suppression of B17-R13's genuine boundary jump.

The smallest slice is the composed fader adoption adapter and shared adoption rule in `crates/builtins-compiler`, plus one row in its existing carry owner. No DSP algorithm, rounding, automation core, host interface or fixture framework change is intended. This is Class B: rendered words may move in the defective running-composed coincident case only. The user requested Sol 6.1 high for implementation; root coordinates fresh independent Astra review, GitHub synchronization and batch delivery.

The existing `entry-step` row retains its static own curve. The added `entry-step-running` row uses the same VCA step with the moving own ride, so the carry owner continues to defend both B17-R13 and no-op running continuity, as well as #2096's held/free, repeated-adoption and on/off-grid cases.

Test value: a running composed no-op adoption that replaces the genuine VCA boundary event with a landing targeting every curve at the jump's completion now fails the existing audible sample comparison; its former static-own entry row cannot reveal that arithmetic change.

Verification uses `/home/bl/misofm/engine-work-evidence/easy2-20261027/run-serial.sh` exclusively, separate worktree targets, Cargo jobs two and test threads one. Evidence is retained outside the worktree at `/home/bl/misofm/engine-work-evidence/easy2-20261027/2158/`. Local implementation and review do not claim upstream synchronization or completion.


### Attempt 1 implementation and supported paths

Root refined attempt 1 before its independent verdict after identifying that a previous jump can leave each component with a different target origin from the composed output's completion. A T - 1/output-completion comparison is insufficient and was discarded before verification. The approved implementation records each composed cursor's heard-value origin and last non-held target origin. Held grid events can advance the first without advancing the second; output holds may overwrite ramp completion without changing either.

For a free running composed Reposition with a discontinuity at T, evaluate each successor curve at both carried origins. When every component agrees, retain its heard value and full held/deferred/event state, reindex only its discontinuity cursor in the successor table, and let the ordinary merged walk process the real T step once. A changed component value keeps the existing entry-landing path. A changed cursor layout or unapplied prepared offset/own-value backstop cannot take the preservation path. This uses fixed preallocated metadata and bounded table reads; the existing size-based cursor and composite accounting charges the new words. No DSP arithmetic, host, schema or wire change is introduced.

Root approved the correction to the original scalar gate: the production carry owner exercises W4 and W8. `VcaRig` requires a bank width, which `Backend::Scalar` does not have; the scalar oracle passes `stage_gain: None` and has no supported carried-bank adoption comparison. The existing scalar/shared-event arithmetic suites remain required, without introducing unsupported scalar swap plumbing.

The existing carry owner also changes the successor's own table in the moving-entry-step case and checks one entry jump of length 480 targeting the changed curve at sample 1,579. This catches an overbroad no-op shortcut that would preserve an actually changed table instead of retaining its existing entry landing. No additional test function or corpus owner is introduced.


### Additional bounded prior-jump row

The same carry owner adds `entry-step-running-prior-jump`: VCA steps from -3 to -6 at sample 900, then to -12 at adoption 1,100 while the earlier jump and own ride are running. This catches treating the output's ramp completion as every component's origin, or resetting its held/deferred event state. Both moving-entry rows use the issue's coincident 100-frame layout; every previous 128-frame/on-grid row remains. An analytical after-step/1,152 observation is retained externally in `after-step-observation.md` for fresh independent review; it is not claimed as a measured separate defect.

Baseline production plus only the extended owner is RED: `cargo test --locked -p builtins-compiler a_carried_automated_lane_renders_the_uninterrupted_bits`, exit 101, `entry-step-running`, composed true, continued 0, Simd8, 100-frame blocks. `baseline-red.log` records the failing audible comparison. The corrected implementation remains awaiting focused green and independent review.


### Focused green and recovery checkpoint

`cargo test --locked -p builtins-compiler a_carried_automated_lane_renders_the_uninterrupted_bits` is GREEN on the principled implementation (exit 0, one owner passed, 35 filtered out; `focused-green.log`). This runs both W4 and W8 across the preserved cases, the moving entry step inside a grid ramp and an earlier jump, and the changed-table landing guard. The same owner is RED on unchanged baseline production (`baseline-red.log`, replayable `baseline-only.patch`). Formatting of automation and builtins-compiler and `git diff --check` passed.

Both known origins are copied during carry. The existing origin-free `set_heard(value)` API remains compatible with graph callers and explicitly makes provenance unknown; it cannot establish equivalence. Prepared values have the known held origin at node time zero. The original value evaluation still uses the same clipped target-time arithmetic, now factored to retain that time as metadata. Composed cursor storage is charged by `Composed::bytes`, builtin sizes use the live `size_of` report, and graph route-cursor accounting calls that same composition helper; no hardcoded byte charge was introduced.

The owner pauses at this compiling, focused-green tranche for an exact-path local checkpoint. Full automation/builtins-compiler suites, independent adversarial verdict and upstream synchronization remain pending. No remote completion is claimed.


### Full focused suites (implementation checkpoint `9aa721bea`)

`cargo test --locked -p automation -p builtins-compiler --features builtins-compiler/test-support` passed once with exit 0: 102 tests passed, three pre-existing tests ignored, zero failures; both crates' doc-tests passed (`full-green.log`). This includes the existing composed/cell event differentials, scalar/bank arithmetic parity, checked resource reports and realtime allocation gates, including `a_builtin_lane_swap_allocates_nothing`. No duplicate full workspace or target matrix was run locally.

All commands used the serial wrapper. Verification initially waited for external engine tests/release builds. During this full suite an external engine Wasm probe began compiling only after our offline compilation had finished and our tests were running; no owned offline compiler subtree remained to pause, and running tests were not suspended. These results are correctness evidence, not timing or benchmark measurements.

Implementation is locally checkpointed and full focused verification is green. Fresh independent Astra review, proportional batch qualification, upstream delivery and matching GitHub synchronization remain root's responsibility. No PASS or remote completion is inferred from implementation evidence alone.
