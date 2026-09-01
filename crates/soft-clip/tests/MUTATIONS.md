# Red-mutation record for the soft-clip gates (issue #91)

Master plan for issue #83, §1.6: *every gate is proven red*. A test that has never failed is not a
gate. Each row below was applied to the working tree, the named test binary was run in release, the
failure was recorded, and the mutation was reverted in the same session. Rows 11 and 13-14 of
`crates/lane/tests/MUTATIONS.md` cover the half-band module this crate depends on.

Host: `x86_64` (Zen 5 class), `rustc 1.97.1`, workspace `.cargo/config.toml` pin
`-C target-feature=+avx2,+fma`.

Reproduce one row with:

```
# apply the "mutation" edit, then
cargo test --release --locked -p soft-clip --test <test binary>
# and revert
```

| # | mutation | file | test binary | result |
|---|---|---|---|---|
| 1 | the interpolator walks its taps in descending order — the same products, reassociated | `lane/src/kernels/halfband.rs` | `polyphase_identity` | RED |
| 2 | the cubic's `p1 / 3` becomes `p1 * (1/3)` (issue #91 F8's class-B option) | `src/kernel.rs` | `polyphase_identity` | RED |
| 3 | the odd-phase sample is read from row `base - 30` instead of `base - 31` | `src/kernel.rs` | `polyphase_identity` | RED |
| 4 | the bypass mask is inverted | `src/kernel.rs` | `polyphase_identity` | RED |
| 5 | the segment length is the first lane's countdown instead of the minimum over all lanes and parameters | `src/lib.rs` | `lane_identity` | RED |
| 6 | the kernel never writes `h.pos` back, keeping the history position in a local | `src/kernel.rs` | `partition_invariance` | RED |
| 7 | the §4.4 boundary check is skipped and every block is accepted | `src/lib.rs` | `boundary_check` | RED |
| 8 | a `Vec::with_capacity(frames)` is added to the per-block driver | `src/lib.rs` | `allocation` | RED |
| 9 | a restore writes the payload's ages in reverse order into the history | `src/lib.rs` | `state_roundtrip` | RED |
| 10 | the D11 snap moves after the segment: the last ramping sample steps instead of assigning the target | `src/lib.rs` | `ramp_law` | RED |
| 11 | one even tap is perturbed by one ulp (`h[22]`) | `lane/src/kernels/halfband.rs` | `determinism`, `contract` | RED |
| 12 | `Channel::reset_full` rebuilds itself with `Self::new` instead of clearing in place | `src/lib.rs` | `allocation` | RED |

## Recorded failures

### 1 — descending tap order (E1)

```
assertion `left == right` failed: case noise/-24dB sample 4:
  polyphase 1.2065722e-10 vs 63-tap 1.2065719e-10
```

The mutation keeps every product and only changes the order they are summed in, which is exactly
the reassociation master plan §2 class C forbids. It separates on the fourth sample of the first
case, so the corpus does not depend on a rare input to catch it.

### 2 — reciprocal multiply in the cubic (E1)

```
assertion `left == right` failed: case noise/-24dB sample 64603:
  polyphase -1.4175984e0 vs 63-tap -1.4175985e0
```

This is issue #91 F8's proposed class-B change, and this row is the evidence for why it is *gated*
rather than taken: it moves rendered bits, so it needs an owner decision and a re-pin, not a
tidy-up. The deviation is one ulp, well inside the `3.0e-6` oracle bound, which is why `contract`'s
oracle test stays green under it — E1 is the gate that separates the two.

### 3 — odd-phase row off by one (E1)

```
assertion `left == right` failed: case noise/-24dB sample 30:
  polyphase -9.83617e-4 vs 63-tap 7.8076235e-4
```

`s[2n-31] = c(0.5 * X[n-31])` is the one place the polyphase form recomputes a value the 63-tap form
stored, so an off-by-one there is the most likely way to get the decomposition subtly wrong.

### 5 — segment length from one lane (E2)

Fails `the_hosts_bank_width_matches_the_scalar_instantiation` on the block where two lanes have
ramps of different lengths in flight: the lane whose ramp ends first snaps a segment late, so its
last ramping sample steps instead of assigning. The corpus ramps all three parameters, on different
lanes, starting in different blocks, for this reason.

### 7 — no boundary check (E7)

All three `boundary_check` tests fail: the NaN block is not zeroed, the counter does not advance,
and the state is not reset, so the block after it does not match a fresh instance.

### 10 — the snap after the segment (E10/ramp law)

```
the_driver_reproduces_the_runtime_ramp_law_bit_for_bit: sample 63
```

The mutation is the plausible one — decrement the countdown, then snap when it reaches zero — and it
is wrong by exactly one sample: `LinearRamp::next_value` *assigns* the target on the final sample
rather than adding to it. This row is why the driver is tested against `next_value` sample by sample
instead of only at block boundaries.

### 12 — a reset that allocates (E8)

```
assertion `left == right` failed: render path allocated 8 times
```

This row is not hypothetical: rebuilding with `Self::new` is what the code did until the gates were
run, and `tests/allocation.rs` is what caught it. A `FullToDefaults` reset allocates three histories
and a ramp table, and a reset reaches the render thread from a seek or a transport stop — so the
mutation and the defect are the same edit. Both reset kinds are now inside the measured region.

## Not proven here

* The wasm leg of the corpus (`tools/wasm-gates`) is exercised by
  `scripts/run-wasm-gates.sh`; its own mutations live in that crate's `MUTATIONS.md`.
* `tests/descriptive_bench.rs` is descriptive, not a gate, and has no mutation.
