# Mono-collapse M2 — the collapsed execution, measured

One authorised capture per arm, both `controlled` on cpu 15, both at candidate commit `57b47ba`
(the F1 fix). Native: `scripts/run-console-benchmark.sh --mono2` (46 records, 2 rounds). Wasm:
`scripts/run-wasm-console-benchmark.sh --mono2` (32 records, 2 rounds, 3 legs each).

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
| 1 | 50 627 | 75 845 | 25 017 | 390.9 ns |
| 2 | 50 336 | 76 555 | 25 969 | 405.8 ns |

**−33.2% and −34.2%.** The session rows agree from the other side: `sixty_four_track_console_mono`
moves 74 361 → 49 194 ns (**−33.8%**) against its `strip4` number while
`sixty_four_track_console_mono_dual`, the same fixture with the collapse forced off, sits at
74 892 (+1.2%).

Per leg, `sixty_four_track_console_mono` p50 ns/block, `strip4` → this capture:

| leg | strip4 | mono2 | delta |
|---|---|---|---|
| native `Simd8` | 75 394 | 49 965 | **−33.7%** |
| wasm `simd128` | 189 941 | 117 062 | **−38.4%** |
| native `Simd4` | 359 062 | 331 160 | −7.8% |

`sixty_four_track_console_half_mono`, whose four all-mono cohorts collapse and whose four all-stereo
ones do not, lands where half a collapse should: −12.3% native `Simd8`, −17.6% wasm.

Every non-collapsing row is within +2.5% of its `strip4` seal on every leg, which is the band two
capture sessions on this machine agree to.

### Why the native `Simd4` leg gains so little, and why it is not a defect

The collapse fires there — 32 of 32 cohorts, every block. What differs is how much of the strip is
*banked* at that width: the plan realises `[32 chains, 64 slots]` at `Simd4` against `[8 chains, 48
slots]` at `Simd8`, so only part of each cohort's strip is chain slots and the rest is per-node ops
the collapse does not reach. That shape difference predates this milestone — it is visible in
`strip4`, where the same row costs 359 µs at native `Simd4` and 190 µs on the *slower* wasm target at
the same lane width. The wasm `simd128` leg is the four-lane number to read.

## What is not in the numbers

A collapsed block renders the bits a dual block renders, so nothing here can show that the mechanism
fired. That evidence is a counter, and it lives in the gates rather than in a record:
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

## Two captures were taken before this one, and neither is sealed here

Both were authorised, both passed their validators, and both were discarded rather than kept with a
footnote. An artifact directory says "this is what the tree at this commit does"; when the tree
moves, so does the capture.

* The first measured a build in which each effect's dual and collapsed bodies shared one function
  behind a `bool`. Every digest matched, and the shipped **dual** path had still regressed —
  `sixty_four_track_eq_only`, a row that never collapses, read 28% above its seal, because one body
  carrying both cascades is one body for the inliner to weigh. The bodies were split by a const
  generic; that is the regression the `KERNEL_ROSTER` derivation note records and the reason its EQ
  rows read 168 + 84 rather than 252.
* The second measured a build carrying the F1 defect that adversarial verification found: the
  collapsed path handed the bypass shunt's latency **line** the ungathered right plane. No row it
  recorded was wrong — the measured rows run with no live console, so the shunt it mis-fed does not
  exist on that path, and this capture's digests are identical to that one's on all 32 native rows
  and all 96 wasm legs. It was retired anyway, because the binary is not the shipped one.

Each arm was also refused once by the preconditions for `loadavg_above_ceiling` while the machine
was still draining from a build; a refusal writes no records, and the run was repeated after it
settled.
