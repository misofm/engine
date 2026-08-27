# Mono-collapse M2 — the collapsed execution, measured

One authorised capture per arm, both `controlled` on cpu 15, both at candidate commit
`1742c7e673c1`. Native: `scripts/run-console-benchmark.sh --mono2` (46 records, 2 rounds).
Wasm: `scripts/run-wasm-console-benchmark.sh --mono2` (32 records, 2 rounds, 3 legs each).

## The claim this capture exists to make

A cohort the collapse fires on renders **byte-identical** to the same cohort rendered dual. It is
stated three ways here, and the third is the one no earlier capture could make:

1. **In-run, paired.** The `console_mono` record's two arms are the same fixture with the collapse
   taken and forced off, alternated observation by observation; the runner asserts their digests
   agree before it emits a number.
2. **Across the 16-row set.** Every session row's `output_sha256` in this capture equals its
   `strip4` value — all 32 native rows, all 96 wasm legs. Nothing moved.
3. **Against a pre-collapse seal.** The `sixty_four_track_console_mono` digest here,
   `62f138a5766ec6cb…`, is the digest `strip4` recorded *before the collapse existed*. The
   collapsed render is bit-for-bit the render the tree produced without it.

## The number

Paired, in-run, native `Simd8` (`console_mono`, ns per block, 64 tracks):

| round | collapse_eligible | collapse_forced_off | paired delta | per track |
|---|---|---|---|---|
| 1 | 50 266 | 75 564 | 25 117 | 392.5 ns |
| 2 | 50 345 | 76 375 | 25 769 | 402.6 ns |

**−33.5% and −34.1%.** The session rows agree from the other side: `sixty_four_track_console_mono`
moves 74 361 → 48 682 ns (**−34.5%**) against its `strip4` number while
`sixty_four_track_console_mono_dual`, the same fixture with the collapse forced off, sits at
74 842 (+1.2%).

Per leg, `sixty_four_track_console_mono` p50 ns/block, `strip4` → this capture:

| leg | strip4 | mono2 | delta |
|---|---|---|---|
| native `Simd8` | 75 394 | 50 496 | **−33.0%** |
| wasm `simd128` | 189 941 | 117 563 | **−38.1%** |
| native `Simd4` | 359 062 | 331 299 | −7.7% |

`sixty_four_track_console_half_mono`, whose four all-mono cohorts collapse and whose four all-stereo
ones do not, lands where half a collapse should: −13.4% native `Simd8`, −18.5% wasm.

### Why the native `Simd4` leg gains so little, and why it is not a defect

The collapse fires there — 32 of 32 cohorts, every block. What differs is how much of the strip is
*banked* at that width: the plan realises `[32 chains, 64 slots]` at `Simd4` against `[8 chains, 48
slots]` at `Simd8`, so two of the six strip stages per cohort are chain slots and the rest are
per-node ops the collapse does not reach. That shape difference predates this milestone — it is
visible in `strip4`, where the same row costs 359 µs at native `Simd4` and 190 µs on the *slower*
wasm target at the same lane width. The wasm `simd128` leg is the four-lane number to read.

## What is not in the numbers

A collapsed block renders the bits a dual block renders, so nothing here can show that the
mechanism fired. That evidence is a counter, and it lives in the gates rather than in a record:
`PreparedRenderPlan::bank_collapse_counters` reports `[collapsed blocks, collapsible cohorts]`, and
`miso-engine-console-workload`'s `chain_shape` suite pins 8-of-8 cohorts on the mono row, 4-of-8 on
half-mono and 0-of-0 on every other row of the standing set.

## Files

| file | what |
|---|---|
| `console-benchmark.accepted.jsonl` | 46 native records, 2 rounds |
| `console-benchmark.raw.jsonl` | byte-identical to the accepted set (no record was dropped) |
| `console-benchmark.core-clock.csv` | the #184 perf-counter evidence behind the cycle columns |
| `console-benchmark.disposition.json` | PASS, controlled, cpu 15, one runner invocation |
| `wasm-console-benchmark.*` | the same for the three-leg wasm arm, 32 records |

The native arm was refused once before this capture, for `loadavg_above_ceiling` while the sweep was
still draining; that refusal wrote no records and the run was repeated after the machine settled.
An earlier authorised capture at the same commit was **discarded** rather than sealed: it measured a
build in which the dual and collapsed bodies shared one function behind a `bool`, and the shipped
dual path had regressed — `sixty_four_track_eq_only`, a row that never collapses, read 28% above its
seal. The bodies were split by a const generic and the arm re-captured; that is the regression the
`KERNEL_ROSTER` derivation note records, and the reason its EQ rows read 168 + 84 rather than 252.
