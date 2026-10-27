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
