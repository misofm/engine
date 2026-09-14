# #807 assignment2 evidence: EQ preparation and numerical validation

Worktree: `/tmp/miso-engine-807` (`codex/807-live-eq`), after assignment1 checkpoint
`6376fb2f364ab0c336a4cce6bc478bf336bf3f63`. Root owns the checkpoint commit and push; this
assignment leaves the implementation uncommitted. The production factory getter remains at its
assignment1 default `None`; no queue/application/state/host/SDK wiring was added.

## Changed paths

- `crates/parametric-eq/src/control.rs`
  - Implements `NativeEffectTargetPreparation` for `ParametricEqFactory`.
  - Validates the complete canonical 60-row candidate (`30` descriptor rows, alternating
    `Left`/`Right`) before touching caller output.
  - Maps descriptor rows to physical `HPF, band1..band4, LPF` sections, designs only touched
    final section/lane pairs once into fixed stack storage, and emits ordered `Left`/`Right` or
    coalesced `Both` targets (maximum `12`).
  - Validates target shape, canonical semantic words, exact disabled identity, finite/stable
    coefficients, fixed HPF/LPF mixes, touched coverage, ordering and overlap without design or
    mutation.
  - Adds focused launch-rate, all-kind, capacity, malformed-selector, rollback, identity, fixed
    mix and stable-forgery tests.
- `crates/parametric-eq/src/lib.rs`
  - Adds the control module.
  - Factors the rounded SVF numerical validator out of `design_svf` and shares it with target
    validation.
  - Freezes the conservative finite output-mix bound at `128.0f32`.

## Numerical decision

The legal gain range gives `A = 10^(gain/40)` with `1/4 < A < 4` and `1/A < 4`. With
`Q >= 0.1` and `S >= 0.1`,
`shelf_k = sqrt((A + 1/A) * (1/S - 1) + 2) < sqrt(74) < 9`.
Thus high-shelf `|m1| < 9*3*4 = 108`, low-shelf `|m1| < 27`, bell
`|m1| = |A - 1/A|/Q < 40`, and every other mix magnitude is at most `16` (including
`1/Q <= 10` for the fixed high-pass family). The accepted finite f32 mix bound is frozen at
`128`, with room above the legal rounded designer output and rejection of arbitrary huge finite
words. Enabled HPF uses exact rounded `m0=1`, `m1=round_f32(-1/q)`, `m2=-1`; enabled LPF uses
`m0=0`, `m1=0`, `m2=1`. Disabled sections admit only the exact identity tuple.

## Focused gates

All commands ran from `/tmp/miso-engine-807` and passed after the final edits:

```text
cargo test --locked -p parametric-eq --tests
31 unit tests + 7 analytic + 5 bank (1 ignored) + 1 conformance + 16 contract +
3 determinism (1 ignored) + 3 mono-collapse + 18 response + 3 silent-fixed-point +
4 stationary-hoist + 5 time-domain tests passed; no failures.

cargo check --locked -p parametric-eq --target wasm32-unknown-unknown
Finished successfully.

cargo test --locked -p effect-contract
16 unit + 12 lattice + 7 live-control + 9 observation + 2 response-analysis tests passed;
no failures.

cargo fmt --all -- --check
pass

git diff --check
pass

cargo clippy --locked -p parametric-eq --all-targets -- -D warnings
pass
```

The target-specific tests directly cover all six kinds at all four launch rates; full touched
section/lane output reaches twelve records; asymmetric lanes are ordered left then right; and
disabled numeric edits retain their semantic header while carrying exact identity coefficients.
Malformed 60-row shape, row order/channel, changed-mask shape, duplicate/missing/extra/overlapping
selectors, unsorted selectors and over-capacity target slices are refused. Preparation errors leave
sentinel output untouched. The negative-contract test changes an enabled bell's cutoff in the
coefficient words while retaining the expected semantic header; the alternate words are finite,
stable and accepted. This is deliberate: `validate_targets` proves numerical safety and semantic
header agreement, not arbitrary coefficient-to-cutoff/Q equivalence. Only the trusted Rust
preparer supplies that equivalence.

## Remaining scope

This checkpoint does not register the factory capability, append prepared queue records, stage or
apply targets, add owner shadows, or connect browser/headless/public semantic admission. Those
remain later assignments in the stateless #807 workflow.
