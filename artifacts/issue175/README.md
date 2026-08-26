# Issue #175 — the intended-placement strip, measured, and the fixture-authority handover

**This directory is the standing 64-track qualification authority, from 2026-08-26.** The fixture
is `fixtures/session/v1/console-sixty-four-track-intended.toml`: EQ and compressor as one two-slot
chain on `simd1`, and a true-peak limiter alone on `simd2`. The previous authority,
`artifacts/issue163-phase2/` on `console-sixty-four-track.toml`, is history; the note at the foot
of this file says exactly what transferred.

Three things were asked of this measurement. Two of them answered the opposite of what was
expected, and both answers are more useful than the expected ones would have been.

| question | answer |
|---|---|
| Is the intended layout **faster**, by one AoSoA round-trip per track per block? | **No. It is marginally slower**, by 1.09 µs / 17 ns per track, and it saves **zero** round-trips. |
| Does a placement change move a rendered bit? | **No** — on both targets, on all three legs, block by block. #166 confirmed, and confirmed harder than #166 stated it. |
| What does the true-peak limiter cost? | **+88.1 µs native (×1.92), +196.2 µs wasm (×2.14).** It is the single most expensive element of the intended strip. |

## Attempts

| arm | attempt | files | status | launches |
|---|---|---|---|---|
| native | 1 | `console-benchmark.attempt-1-refused.*` | FAIL `precondition_loadavg_above_ceiling` | 0 |
| native | 2 | `console-benchmark.{raw,accepted,disposition}` | PASS `controlled` | 3 |
| wasm | 1 | `wasm-console-benchmark.attempt-1-refused.*` | FAIL `precondition_loadavg_above_ceiling` | 0 |
| wasm | 2 | `wasm-console-benchmark.{raw,accepted,disposition}` | PASS `controlled` | 3 |

Both arms refused once and both refusals are kept: they launched nothing and timed nothing
(`raw_sha256: null`). The wasm refusal has a cause worth recording — that runner builds its guest
and host with its own frozen release settings *before* it evaluates admissibility, so a cold build
puts the one-minute load average over the ceiling by itself. The second attempt's build is cached
and the check passes. Attempt 2 on both arms: cpu affinity 15, SMT sibling quiet, cooldown
honoured, `raw` and `accepted` byte-identical. Round-to-round spread is at most **0.6%** on any
native row.

## The variant table

`Simd8` native and `wasm_simd128` under wasmtime 47.0.3, 48 kHz, 128-frame quantum, 1 000
observations, p50 µs/block, minimum of the two rounds. µs/track in brackets.

| row | tracks | layout | native | wasm | digest |
|---|---|---|---|---|---|
| **console — the intended strip (c)** | 64 | `simd1:eq+compressor,simd2:limiter` | **184.52** (2.883) | **368.76** (5.762) | `0e527225d5e7` |
| console, synthetic — variant (c) | 128 | same | 372.28 (2.908) | 742.87 (5.804) | `0970171176c6` |
| **eq+comp on simd1 — variant (b)** | 64 | `simd1:eq+compressor` | **96.37** (1.506) | **172.58** (2.697) | `ede41bb7e6fd` |
| **console_legacy — today's layout** | 64 | `simd1:eq,dynamic:compressor` | **94.52** (1.477) | **172.31** (2.692) | `ede41bb7e6fd` |
| **eq on simd1 — variant (a)** | 64 | `simd1:eq` | **43.69** (0.683) | **91.86** (1.435) | `83a4b205c383` |
| decomposition: compressor | 64 | `simd1:compressor` | 74.61 (1.166) | 128.21 (2.003) | `35b1d89136c8` |
| decomposition: builtins | 64 | `builtins` | 22.34 (0.349) | 47.77 (0.746) | `5bf3c3772d4c` |
| decomposition: identity — **control** | 64 | `builtins` | 22.61 (0.353) | 47.54 (0.743) | `2b015145fd33` |
| idle (silence input) | 64 | `simd1:eq+compressor,simd2:limiter` | 129.39 (2.022) | 271.20 (4.238) | `7b331c02e313` |
| nine-track ragged strip | 9 | `simd1:eq+compressor,simd2:limiter` | 33.04 (3.671) | 63.75 (7.083) | `262cfca89298` |
| `parametric-eq-nine-track` fixture | 9 | `simd1:eq` | 9.02 (1.002) | 16.12 (1.791) | `d5df5ebe109d` |

Variant (a) is the pre-existing `sixty_four_track_eq_only` derivation, and #175 asked whether that
row already *is* the EQ-on-`simd1` shape. It is, and the digest proves it: `83a4b205c383` is what
`artifacts/issue163-phase2/` recorded for the same row on the retired fixture. No row was added
for it.

Native share of one core at 64 tracks: **3.54% → 6.92%**. Browser: **6.46% → 13.83%**.

## The chain-shape row-pair, and the hypothesis it falsified

The hypothesis: a two-slot `simd1` chain pays **one** planar/AoSoA transpose round-trip where two
one-slot chains (`simd1` + `dynamic`) pay **two**, so the intended layout should be faster by one
round-trip per cohort per block. Measured by paired alternation, arms swapped observation by
observation:

| round | split chains | merged chain | paired delta | per track | transposes/block |
|---|---|---|---|---|---|
| 1 | 95.031 µs | 96.303 µs | **+1 252 ns** | +19.6 ns | 24 vs **24** |
| 2 | 95.261 µs | 96.183 µs | **+932 ns** | +14.6 ns | 24 vs **24** |

**The merged chain saves no round-trip at all, and costs about 1.09 µs.** The wasm arm finds the
same sign independently: +0.27 µs, +4.2 ns per track.

### The transpose accounting, and where the saving actually went

The hypothesis was right about the architecture and wrong about this tree. Both halves matter:

* **`miso_engine_rack::BankChain` is built for it.** It takes an ordered `slots` vector,
  transposes exactly once in `run` whatever the slot count, and its own unit tests drive three
  slots through one round-trip (`crates/miso-engine-rack/src/lib.rs`).
* **The cohort planner does group the slots.** On the merged model
  `bound_slots_in(Simd1)` goes 8 → 16 and `Dynamic` empties: the planner forms one two-slot cohort
  per eight tracks, exactly as #99 F3 intended.
* **The graph runtime then throws the grouping away.** `miso-engine-graph`'s `runtime::chain_for`
  returns one `BankChain` per prepared effect bank, and `runtime::bank_chain` constructs each with
  `vec![BankSlot { .. }]` — a single slot. A two-slot cohort becomes **two independent one-slot
  chains, with two gathers and two scatters.**

So at 64 tracks and eight lanes both layouts run 16 effect chains plus 8 builtin chains = **24
round-trips per block**, and G5 ("one round-trip per bank chain per block") holds on both sides
while being unable to distinguish "per chain" from "per bound slot" — today they are the same
number. The counts are now recorded per arm in the `console_placement` record rather than argued,
so the day the graph layer takes the saving, the validator's equality goes red and says so.

Pinned in `crates/miso-engine-graph-compiler` as
`intended_placement_merges_two_chains_into_one_bit_identically`.

**The residual ~1 µs is not the round-trip; it is the cost of the grouping itself** — one cohort
of two slots schedules and dispatches slightly differently from two cohorts of one. It is under
1.2% of the block and well inside the round-to-round spread of the sequential rows, which is why
it is only visible as a paired statistic.

## Bit identity across placements

The #166 property, stated four independent ways in this record:

1. **In-run assertion, native.** `PlacementMeasurement::run` compares the two arms' digests before
   it reports a delta and panics if they differ. Both rounds passed.
2. **In the records.** `sixty_four_track_console_legacy` and `sixty_four_track_eq_comp_simd1` both
   render `ede41bb7e6fd…` — same fixture-pair, different racks.
3. **Cross-backend.** On the wasm arm those two rows render `30256f812f25…` on **all three legs**
   (`native_simd8`, `native_simd4`, `wasm_simd128`), so the property is not a property of one host.
4. **Block by block.** `merging_the_compressor_into_the_simd1_chain_moves_no_rendered_bit` and
   `the_two_placements_agree_block_by_block` in `tools/miso-engine-console-workload/tests/`
   compare 64 consecutive blocks, so an early difference cannot cancel against a later one.

## The limiter's cost row

`console (c) − eq+comp on simd1 (b)`, the only difference being the `simd2` true-peak limiter:

| arm | without limiter | with limiter | increment | per track | factor |
|---|---|---|---|---|---|
| native `Simd8` | 96.37 | **184.52** | **+88.15 µs** | +1.377 µs | **×1.92** |
| wasm `simd128` | 172.58 | **368.76** | **+196.18 µs** | +3.065 µs | **×2.14** |

The limiter costs more than the EQ and the compressor together (native: 88.1 µs against
21.1 + 52.7 = 73.8 µs measured as increments over the identity control). It banks correctly — the
graph test pins eight bound `simd2` slots at eight lanes with nothing on the per-node path — so
this is the banked cost, not a scalar fallback. It is the price of a four-phase BS.1770-5 Annex-2
detector, a van Herk sliding minimum and a box ramp, per lane, per sample.

**The limiter is transparent on silence.** The idle row's digest, `7b331c02e313…`, is unchanged
from `artifacts/issue163-phase2/` even though that row now carries a limiter the phase-2 fixture
had on no track: a safety limiter presented with digital silence outputs digital silence. Its idle
*cost* is not free — 129.39 µs against phase-2's 36.61 µs — because no gate exists to skip it.

## Digest continuity across the handover

Everything the two fixtures share renders identically. This is what makes the handover auditable
rather than announced.

| row | phase-2 digest | this record | |
|---|---|---|---|
| `sixty_four_track_console_legacy` vs phase-2's `console` | `ede41bb7e6fd` | `ede41bb7e6fd` | the retired fixture, unmoved |
| `sixty_four_track_eq_only` | `83a4b205c383` | `83a4b205c383` | EQ path unperturbed |
| `sixty_four_track_compressor_only` | `35b1d89136c8` | `35b1d89136c8` | **across a rack move** |
| `sixty_four_track_builtins_only` | `5bf3c3772d4c` | `5bf3c3772d4c` | |
| `sixty_four_track_dispatch_only` | `2b015145fd33` | `2b015145fd33` | control |
| `sixty_four_track_idle` | `7b331c02e313` | `7b331c02e313` | limiter transparent on silence |

`compressor_only` is the strongest row here: the compressor moved from the `dynamic` rack to
`simd1` between the two fixtures and rendered the same bits, which is #166 restated as evidence
rather than as a claim.

Timing continuity is just as close. `console_legacy` measures **94.52 µs** native against the
retired authority's **95.10 µs** (0.6%), and **172.31 µs** wasm against **172.74 µs** (0.25%) —
two different trees, two different runs, the same fixture.

## Measurement boundary — what may not be quoted from this

* **Wasm numbers are not browser numbers.** wasmtime's Cranelift compiles ahead of time and does
  not tier, deoptimise or recompile on feedback the way a browser JIT does, on hardware that is
  not a phone. Every wasm record carries `browser_field_measurement: false` and
  `comparable_with_console_records: false`. This is the determinism-pinned reference.
* **The chain-shape delta is a paired statistic and only a paired statistic.** The sequential
  session rows put it at +1.85 µs and an uncontrolled sequential run put it at −34 µs; both are
  drift. Only the alternated arms measure it. Do not re-derive it by subtracting two rows.
* **The `console` row is not comparable with any pre-#175 `sixty_four_track_console` row.** The
  name is the same and the workload is not: this one carries a limiter. Use
  `sixty_four_track_console_legacy` for that comparison, this once.
* **This is not a limiter qualification.** #049 owns that. This measures one placement of one
  parameter set, chosen for musical sanity from the published metadata and documented in
  `scripts/derive-intended-console-fixture.py`.
* Descriptive only. No row here is a threshold.

## Authority handover — standing note, 2026-08-26

The standing 64-track qualification fixture is now
`fixtures/session/v1/console-sixty-four-track-intended.toml`, and the standing record family is
this directory. `fixtures/session/v1/console-sixty-four-track.toml` is retired: it stays in the
tree, unmodified, and is rendered by exactly one row — `sixty_four_track_console_legacy` — which
exists for one transition record and should be removed once this comparison has been read.

The standing fixture is **generated, not authored**.
`scripts/derive-intended-console-fixture.py` derives it from the retired one by moving the
compressor's declaration verbatim and adding the limiter, and takes its canonical spelling from
`miso-engine-session-validator --canonical`. `scripts/check-intended-console-fixture.sh`
regenerates it and compares byte for byte on every sweep, so it cannot be hand-edited into
disagreement with its own provenance.

The retired family's README (`artifacts/issue163-phase2/README.md`) carries the matching note.

## Links

* Fixture: `fixtures/session/v1/console-sixty-four-track-intended.toml`; generator
  `scripts/derive-intended-console-fixture.py`; check
  `scripts/check-intended-console-fixture.sh`.
* Retired authority: `artifacts/issue163-phase2/` (native + wasm), baseline
  `artifacts/issue163-phase2-wasm-baseline/`.
* Runners: `scripts/run-console-benchmark.sh --issue175`,
  `scripts/run-wasm-console-benchmark.sh --issue175`, and their preflights.
* Validators: `scripts/console-benchmark-{record-lib,record-validator,validator}.jq`,
  `scripts/wasm-console-benchmark-validator.jq`; mutation harnesses
  `scripts/test-console-benchmark.sh`, `scripts/test-wasm-console-benchmark.sh`.
* Chain shape and G5: `crates/miso-engine-graph-compiler/src/lib.rs`
  (`intended_placement_merges_two_chains_into_one_bit_identically`); the unrealised saving is at
  `crates/miso-engine-graph/src/runtime.rs` (`bank_chain`, `chain_for`).
