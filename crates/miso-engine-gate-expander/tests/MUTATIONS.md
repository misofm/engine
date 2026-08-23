# Red-mutation record for the gate/expander gates (#89)

Master plan for issue #83, §1.6: *every gate is proven red*. A test that has never failed is not a
gate. Each row below was applied to the working tree, the named test binary was run, the failure was
recorded, and the mutation was reverted in the same session. Nothing in this file is a claim about
code that was not run.

Host: `x86_64` (Zen 5 class), workspace `.cargo/config.toml` pin `-C target-feature=+avx2,+fma`,
debug profile unless a row says otherwise.

Reproduce one row with:

```
# apply the "mutation" edit, then
cargo test --locked -p <crate> --test <test binary>
# and revert
```

| # | mutation | file | crate / binary | result |
|---|---|---|---|---|
| 1 | the hold expiry is tested *after* the decrement instead of before it | `dsp-reference/src/gate_expander.rs` | `miso-engine-dsp-reference` / `--lib` | RED |
| 2 | the opening threshold becomes strict: `level_db > threshold` instead of `>=` | `dsp-reference/src/gate_expander.rs` | `miso-engine-dsp-reference` / `--lib` | RED |
| 3 | the in-band re-arm stops reloading the hold | `dsp-reference/src/gate_expander.rs` | `miso-engine-dsp-reference` / `--lib` | RED |

## Recorded failures

### 1 — the hold expiry is tested after the decrement

```
} else {
    slot.hold_remaining = slot.hold_remaining.saturating_sub(1);
    if slot.hold_remaining == 0 { slot.open = false; }
}
```

closes the gate one sample early, so a hold of `n` keeps it open for `n - 1` samples.

```
test gate_expander::tests::a_level_inside_the_hysteresis_band_keeps_reloading_the_hold ... FAILED
test gate_expander::tests::hold_expiry_closes_exactly_one_sample_after_the_countdown_reaches_zero ... FAILED
test result: FAILED. 5 passed; 2 failed
```

### 2 — the opening threshold becomes strict

`level_db > threshold` leaves a closed gate closed on the one sample whose level is exactly the
threshold, which brief 014 opens on.

```
test gate_expander::tests::a_level_exactly_at_the_threshold_opens_a_closed_gate ... FAILED
test result: FAILED. 6 passed; 1 failed
```

### 3 — the in-band re-arm stops reloading the hold

With the reload removed, a gate whose countdown is already partly spent when the level returns to
the hysteresis band closes from what was left of it instead of from a full hold.

```
test gate_expander::tests::a_level_inside_the_hysteresis_band_keeps_reloading_the_hold ... FAILED
test result: FAILED. 6 passed; 1 failed
```

Note on an *equivalent* mutant found while writing row 3: with a test whose hold is full when the
band is entered, removing the reload changes nothing, because the in-band arm is an `else if` and
short-circuits the decrement. The test was rewritten to spend part of the countdown before the band
is entered rather than accepting the mutant as covered.
