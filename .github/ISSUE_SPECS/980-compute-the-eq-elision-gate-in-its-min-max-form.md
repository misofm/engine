# Compute the EQ elision gate in its min/max form


EQ optimisation, slice 5 (research 2026-09-27, base `6ca203f8`). Evidence:
`docs/handoffs/effects-2026-09-27/EQ-DIAGNOSIS.md`, sections 1 and 5 (EQ-5); the equivalence and
timing harness is `gate_forms` in `docs/handoffs/effects-2026-09-27/eq-diagnosis-prototypes.patch`
(`tools/console-workload/tests/eq_rows.rs`).

## Product outcome

`block_admits_elision` (`crates/parametric-eq/src/lib.rs:997`) scans both input planes of every
stationary bank block:

```rust
rejected |= u32::from(bits == NEGATIVE_ZERO_BITS);
rejected |= u32::from((bits & MAGNITUDE_MASK) > ELISION_MAGNITUDE_CEILING);
```

LLVM turns each `u32::from(bool)` into a compare plus a shift, so the loop is 7 integer vector
operations per vector of words. It costs 0.22-0.26 cycles per lane-sample natively and 0.71 in the
browser build: 3.2 us of a 64-track block in V8, the largest non-kernel term of the EQ after the
kernel itself.

The same predicate in min/max form is 4 operations per vector:

```rust
nearest = nearest.min(bits ^ NEGATIVE_ZERO_BITS);   // 0 exactly when some word is -0.0
largest = largest.max(bits & MAGNITUDE_MASK);
// admitted iff nearest != 0 && largest <= ELISION_MAGNITUDE_CEILING
```

Replica per two 1,024-word planes: native `Simd8` 431 -> 309 cycles (-28 %), V8 617 -> 402
(-35 %). In process, per 64-track block: wasm -1.0 us, native `Simd8` -0.1 to -0.25 us, `Simd4`
-0.1 us.

## Lessons carried from #944

Dev and release; the equivalence gate below is exhaustive over its bit patterns; mutations recorded
red in `crates/parametric-eq/tests/MUTATIONS.md`.

## Invariants

- **Class A.** The predicate's value is unchanged on every input, so the elision decision, and with
  it every rendered and integrator word, is unchanged.
- `u32::min`/`u32::max` on bit patterns only; no float compare (a float compare would treat `-0.0`
  as `+0.0` and order NaNs away).
- Allocation-free; the function stays scalar source that LLVM vectorises (`i32x4.min_u`/`max_u` in
  `simd128`, `vpminud`/`vpmaxud` on x86).

## Interface contract

`block_admits_elision(io: &[f32]) -> bool` keeps its signature and doc; its body becomes the min/max
form above, and the doc states why `nearest == 0` is exactly "some word is `-0.0`" (`x ^ c == 0`
iff `x == c`) and why `largest <= ceiling` is exactly "every magnitude is finite and within
`BLOCK_LIMIT`" (the existing argument about monotone magnitude bits).

## Smallest closable slice

Authorized paths: `crates/parametric-eq/src/lib.rs` (`block_admits_elision` and the `elision` test
module); `crates/parametric-eq/tests/MUTATIONS.md`; this spec.

## Non-goals

Removing the gate, fusing it with `block_is_positive_zero` or with the previous stage's scan,
changing the ceiling.

## Objective gates

1. **Equivalence (in-crate, exhaustive).** Keep the old body as a `#[cfg(test)]` oracle. For every
   ordered pair `(a, b)` from the edge set `{0, 0x8000_0000, 1, 0x8000_0001, CEILING, CEILING + 1,
   0x7f80_0000, 0xff80_0000, 0x7fc0_0000, 0xffc0_1234, 0x3f80_0000, 0xbf80_0000,
   CEILING | 0x8000_0000, (CEILING + 1) | 0x8000_0000, 0x7fff_ffff, 0xffff_ffff}`, planted at two
   positions of a block of ordinary words, and for block lengths `{0, 1, 7, 8, 9, 1024}`, the new
   body equals the oracle. Dev and release.
2. The existing refusal tests stay green unchanged: `a_negative_zero_input_refuses_elision`,
   `a_non_finite_or_oversized_input_refuses_elision`, `the_negative_zero_refusal_is_load_bearing`.
3. `cargo test -p parametric-eq`, dev and release; every `WORKLOADS` digest unchanged.
4. **Mutations**, each alone, red:
   - M1: `nearest.min(bits)` (no xor): gate 1 (a block holding `+0.0` is refused).
   - M2: `largest < ELISION_MAGNITUDE_CEILING`: gate 1 (`CEILING` itself is admitted today).
   - M3: drop the `& MAGNITUDE_MASK`: gate 1 (a negative ordinary word is refused).
5. **Browser artifact.** As EQ-1 gate 5; record the loop's `i32x4.min_u`/`max_u` in
   `wasm-objdump -d`.
6. Toolchain and policy scripts as EQ-1 gate 7.

## Console benchmark rows

- **Can move** (down, slightly): every row carrying the EQ, most visibly the wasm `eq_only`.
- **Must not move:** rows without the EQ.

## Dependencies

None.

## Standing rules for the implementer

As EQ-1.

## What the implementer will hit

- **The inversion is easy to get backwards.** Today's loop accumulates *rejections*; this one
  accumulates two witnesses and decides at the end. Gate 1's oracle is the guard.
- **Do not reach for `Lane`.** The gate reads raw bits and `Lane` has no integer lanes; plain `u32`
  arithmetic is what vectorises here.


## Attempt 1 evidence

Implementer: attempt 1, 2026-09-27, branch `codex/977-eq-elision-and-passes` from `1d8c4851` (the
local optimisation batch after #976). The verification comment on the GitHub issue (start values
stated explicitly) is applied. Host: AMD EPYC 7313P (Zen 3), `rustc 1.97.1`, `x86-64-v3`, every
build `CARGO_INCREMENTAL=0`.

### The change

`block_admits_elision` (`crates/parametric-eq/src/lib.rs`) keeps its signature. Its body carries
two witnesses, `nearest = u32::MAX` and `largest = 0`, updated per word as
`nearest = nearest.min(bits ^ NEGATIVE_ZERO_BITS)` and `largest = largest.max(bits & MAGNITUDE_MASK)`,
and decides once: `nearest != 0 && largest <= ELISION_MAGNITUDE_CEILING`. The doc states the start
values, why `nearest == 0` is exactly "some word is `-0.0`" (`x ^ c == 0` iff `x == c`; `+0.0`
contributes `0x8000_0000`), and why `largest <= ceiling` is exactly "finite and within
`BLOCK_LIMIT`" (monotone magnitude bits; every infinity and NaN sits at or above `0x7f80_0000`).
The old body is kept as the `#[cfg(test)]` oracle `block_admits_elision_oracle`. No float compare,
no `Lane`, no allocation.

### Gate 1: equivalence

- `elision::the_min_max_gate_equals_the_rejection_oracle` (in-crate): every ordered pair of the 16
  edge patterns, planted at two positions (`(len / 3, len - 1)` and `(0, len / 2)`) of a block of
  ordinary words of both signs, at lengths 0, 1, 7, 8, 9 and 1,024: the new body equals the oracle.
  Green in dev and release (it is part of `cargo test -p parametric-eq`).
- `elision::every_word_gets_the_rejection_oracles_verdict` (in-crate, `#[ignore]`, run explicitly):
  all `2^32` words, each alone and in every run of 64 consecutive words (the vectorised body), under
  `std::hint::black_box` so the optimiser cannot fold the two bodies together; it also asserts the
  admitted count `2 * (CEILING + 1) - 1`. `cargo test -p parametric-eq --lib every_word -- --ignored`:
  pass in dev (33.3 s, 8 threads) and release (3.8 s). A first release run without `black_box`
  finished in 0.16 s, which is the optimiser proving the lone-word comparison equal rather than the
  test running it; the committed form cannot be folded.

### Gate 2: the existing refusal tests

`a_negative_zero_input_refuses_elision`, `a_non_finite_or_oversized_input_refuses_elision` and
`the_negative_zero_refusal_is_load_bearing` are unchanged and green, dev and release.

### Gate 3: suite and rows

`cargo test -p parametric-eq`: 103 passed, 3 ignored in dev and in release, with and without
`--features test-support` (one new test and one new ignored test over #976's 102/2).

Rows: a scratch harness (never committed; `SessionRuntime::build_with_dispatch`, 64 blocks,
`hash_output`) in clean `git archive` copies of the base (`1d8c4851`) and of this change. Native:
all 15 `WORKLOADS` rows at `Scalar`, `Simd4` and `Simd8` identical before and after, and equal to
the table in #976's record. The scratch copies also carry a knob (scratch-only) that enables a
second live bell on every EQ (general band 2), which puts a depth-2 pass into every EQ row: those
45 lines are identical before and after as well. Wasm: `wasm-console-guest` (`+simd128`, release)
fed `console_workload::source_block` words, 30 digests (15 rows, one and two bands) identical
before and after, and each equal to its native digest.

### Gate 4: mutations

Recorded in `crates/parametric-eq/tests/MUTATIONS.md` ("Issue #980"), each alone, release,
`--lib elision -- --include-ignored`:

| # | mutation | red on |
|---|---|---|
| M1 | `nearest.min(bits)` | gate 1 (length 1, `+0.0` refused), the exhaustive run, and three older refusal/identity tests |
| M2 | `largest < CEILING` | gate 1 (length 1, the ceiling word `0x7149f2ca` refused), the exhaustive run |
| M3 | no `& MAGNITUDE_MASK` | gate 1 (length 1, `0x80000001` refused), the exhaustive run, eight older tests |

### Gate 5: browser artifact

`host-web` built with `scripts/build-web-audioworklet.sh`'s cargo line (`RUSTFLAGS="-C
target-feature=+simd128 -C strip=debuginfo --remap-path-prefix=…"`, release, `wasm32-unknown-unknown`)
at the base and after (artifacts `820e96dd…` and `e4fc2382…`; not repinned), `wasm-objdump -d`
into `scripts/check-web-audioworklet-callgraph.py`: `--callgraph miso_engine_web_v1_render`
closure=8 traps=5 (one trap owner, `render_inner`); `--kernel-shape --kernel-pattern
'4wide6f32x[48]' --kernel-min 11` ok, kernels=14, rule 3 ok, `parametric-eq f32x4 dual` one kernel
vector=312 scalar=0, `collapsed` vector=156 scalar=0; `meter_poll` and `command_submit
--allocation-only` ok. Every line of the four checks is identical to the base's. The EQ
`process_bank` gate loop in the artifact:

```text
loop
  local.get 2 ; v128.load 2 0 ; local.tee 46
  v128.const 0x7fffffff x4 ; local.tee 48 ; v128.and ; local.get 39 ; i32x4.max_u ; local.set 39
  local.get 46 ; v128.const 0x80000000 x4 ; local.tee 44 ; v128.xor ; local.get 36 ; i32x4.min_u ; local.set 36
  (pointer and counter) br_if 0
end
```

Four vector operations per vector of words (the base loop's `i32x4.eq`/`i32x4.gt_u` accumulation
is gone). Natively (release, `PreparedParametricEq<f32x8, 8>::process_bank`) the loop is
`vpxor`/`vpminud` and `vpand`/`vpmaxud`, unrolled by four.

### Gate 6: toolchain

`cargo fmt --all --check`; `cargo clippy --locked --workspace --all-targets --all-features -- -D
warnings`; `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`; `cargo test -p
lane` dev and release (68 passed); `-p effect-runtime` (86); `-p console-workload` (39) and release
`--test chain_shape` (23); `-p builtins-compiler --features test-support` (79); `-p wasm-gates` (9);
`-p bench floor` (9); `check-lane-policy.sh` (ok), `check-realtime-policy.sh` (ok, 57 regions),
`check-parametric-eq-render-contract.sh` (PASS), `test-console-benchmark.sh` (PASS, 0 real runs):
all green.

### Descriptive A/B (not a gate)

Clean scratch builds of the base and this change, built outside the lock; one hold of
`flock -w 7200 …/timing.lock`, `taskset -c 31`: the native harness alternated three times (per run:
1,000 warm-up blocks, six rounds of 1,500 blocks per row at `Simd8`), then both wasm guests in one
Node 22.23.2 process (six interleaved rounds of 1,000 blocks). Host load average 15-16 during the
hold (two other builds), and the Node rounds were bimodal, so each cell is the median / the minimum
of the per-round p50s, in us:

| row | native base | native after | wasm base | wasm after |
|---|---:|---:|---:|---:|
| `eq_only` minus `builtins_only` | 8.67 / 8.70 | 8.39 / 8.17 | 37.59 / 20.88 | 19.50 / 19.28 |
| two-band `eq_only` minus `builtins_only` | 14.32 / 14.41 | 14.23 / 14.35 | 58.50 / 42.95 | 43.93 / 42.61 |

On the minima: native -0.5 us and wasm -1.6 us on the standing isolate (the brief projected -0.1 to
-0.25 and -1.0). The two-band row moves by noise. No projected saving is claimed.

### Deviations and notes for the verifier

1. The exhaustive run is an `#[ignore]` test, not part of the default suite (33 s in a dev build).
2. `docs/rulings/effect-floor-accounting.md` ("EQ inventory") still describes the gate as "five
   integer comparisons per lane-sample"; the ruling is outside this issue's paths. It is corrected
   with #977, which is authorized to edit that section.
3. The descriptive A/B ran under a load average of 15-16; its wasm medians straddle two modes.
