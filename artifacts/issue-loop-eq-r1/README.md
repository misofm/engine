# Effect-optimization loop, EQ round 1: identity-section elision and the two-slot cohort chain

Two class-A changes, measured together against `artifacts/issue175`, which is the standing
authority for the intended strip.

* **Parametric-EQ identity-section elision.** A disabled band is `EqSvfWordsV1::IDENTITY` exactly,
  and the shipped console fixture enables one band of four on every track. The stationary path ran
  all four sections anyway. It now runs only the live ones, gated per block.
* **The two-slot cohort chain (#181).** The cohort planner has grouped whole rack chains since
  #99 F3 and `miso_engine_rack::BankChain` has always transposed once per chain whatever its slot
  count; the graph layer built one single-slot chain per bound slot. It now builds one per cohort.
  The intended strip's 24 bound slots become 16 chains.

| question | answer |
|---|---|
| Did any rendered bit move? | No. Every session row's `output_sha256` equals #175's, on every row and every leg. |
| Did the headline row improve? | Yes. Intended console 184.520 -> 173.871 us/block native (-5.77%), 368.761 -> 343.933 wasm (-6.73%). |
| Did anything get worse? | Yes, one arm: the hoist `moving` (ramping) arm, +4.8%. Recorded below, not tuned away. |

## Attempts

| arm | attempt | files | status | launches |
|---|---|---|---|---|
| native | 1 | `console-benchmark.attempt-1-refused.disposition.json` | FAIL `precondition_loadavg_above_ceiling` | 0 |
| native | 2 | `console-benchmark.{raw,accepted}.jsonl`, `.disposition.json` | PASS `complete` | 3 |
| wasm | 1 | `wasm-console-benchmark.attempt-1-refused.disposition.json` | FAIL `precondition_loadavg_above_ceiling` | 0 |
| wasm | 2 | `wasm-console-benchmark.{raw,accepted}.jsonl`, `.disposition.json` | PASS `complete` | 3 |

Both refusals are the runner refusing a measurement it could not control on a host shared with
other agents; neither launched a workload process. The wasm refusal is the failure #175's attempt 1
recorded and it has the same cause: the wasm runner builds the guest and the host *before* it
checks admissibility, so a cold build is enough to push the machine over its own ceiling. Attempt 2
pre-warmed both builds under the same frozen profile so the runner's build was a no-op.

## Native, p50 us/block, minimum of the two rounds

| row | #175 | EQ r1 | delta | % | digest |
|---|---:|---:|---:|---:|---|
| `sixty_four_track_console` | 184.520 | 173.871 | -10.649 | -5.77% | unchanged |
| `one_twenty_eight_track_stretch` | 372.278 | 348.812 | -23.466 | -6.30% | unchanged |
| `sixty_four_track_eq_comp_simd1` | 96.373 | 83.598 | -12.775 | -13.26% | unchanged |
| `sixty_four_track_console_legacy` | 94.520 | 87.917 | -6.603 | -6.99% | unchanged |
| `sixty_four_track_eq_only` | 43.693 | 37.401 | -6.292 | -14.40% | unchanged |
| `sixty_four_track_compressor_only` | 74.612 | 74.281 | -0.331 | -0.44% | unchanged |
| `sixty_four_track_idle` | 129.386 | 122.913 | -6.473 | -5.00% | unchanged |
| `nine_track_ragged_strip` | 33.043 | 31.671 | -1.372 | -4.15% | unchanged |
| `nine_track_baseline` | 9.017 | 6.011 | -3.006 | -33.34% | unchanged |
| `sixty_four_track_builtins_only` | 22.343 | 22.613 | +0.270 | +1.21% | unchanged |
| `sixty_four_track_dispatch_only` | 22.613 | 22.673 | +0.060 | +0.27% | unchanged |

The last two rows are the controls: neither carries an EQ and neither is banked differently. They
bound cross-campaign drift on this host at about +1.2%, which is the resolution below which a
delta in this table should not be read as a change.

## Wasm arm, p50 us/block, minimum of the two rounds, all three legs

| row | wasm #175 | wasm r1 | Δ | simd8 #175 | simd8 r1 | Δ | simd4 #175 | simd4 r1 | Δ |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `sixty_four_track_console` | 368.76 | 343.93 | -6.73% | 186.16 | 174.60 | -6.21% | 500.31 | 500.11 | -0.04% |
| `one_twenty_eight_track_stretch` | 742.87 | 689.07 | -7.24% | 372.86 | 350.66 | -5.95% | 1003.23 | 1005.89 | +0.27% |
| `sixty_four_track_eq_comp_simd1` | 172.58 | 149.78 | -13.21% | 96.40 | 84.26 | -12.60% | 363.47 | 364.21 | +0.20% |
| `sixty_four_track_console_legacy` | 172.31 | 158.05 | -8.27% | 96.00 | 89.36 | -6.92% | 362.97 | 363.58 | +0.17% |
| `sixty_four_track_eq_only` | 91.86 | 79.02 | -13.98% | 45.43 | 39.27 | -13.54% | 180.15 | 181.13 | +0.55% |
| `sixty_four_track_compressor_only` | 128.21 | 127.95 | -0.20% | 75.68 | 75.16 | -0.69% | 217.10 | 217.88 | +0.36% |
| `sixty_four_track_idle` | 271.20 | 257.42 | -5.08% | 130.30 | 124.18 | -4.70% | 177.26 | 176.43 | -0.47% |
| `nine_track_ragged_strip` | 63.75 | 60.67 | -4.83% | 34.02 | 32.15 | -5.48% | 72.59 | 72.44 | -0.21% |
| `nine_track_baseline` | 16.12 | 10.98 | -31.88% | 9.11 | 6.20 | -31.91% | 25.83 | 8.07 | -68.77% |
| `sixty_four_track_builtins_only` | 47.77 | 47.15 | -1.30% | 23.73 | 24.38 | +2.70% | 33.51 | 33.97 | +1.37% |
| `sixty_four_track_dispatch_only` | 47.54 | 47.23 | -0.65% | 23.79 | 24.32 | +2.23% | 33.48 | 34.01 | +1.56% |

`digest_identity` is `all_legs_identical` on all 22 records, and every leg's digest equals #175's.
The wasm arm's native legs drifted up to +2.7% on the controls, so that is this arm's resolution.

### The `native_simd4` leg barely moved, and that is a finding

Effect banks bind only at the host's own width (`bind_homogeneous_bank` returns `Ok(None)`
otherwise), so on the `native_simd4` leg every effect runs on the per-node scalar path at `W = 1`.
Two things follow, and both are visible above. The cohort merge has no banks to merge, so it
contributes nothing. And `Lane::SVF_CASCADE_DEPTH` is **4** for the scalar backend, so the section
list rounds a one-live-band configuration straight back up to the whole cascade and the elision
never engages -- except on `nine_track_baseline`, where eight of nine tracks have *zero* bands
enabled, the live count is zero, and the whole cascade goes. That row is -68.77%.

## Engagement rate of the elision, on the standing fixture

Measured with a temporary counter in `cascade_sections` over 200 rendered blocks per workload, at
`6fcfc26`. The counter was reverted before any measurement in this directory was taken; it is not
in the tree.

| workload | EQ instances | engagement rate | cascade sections run |
|---|---|---:|---|
| `sixty_four_track_console` | 8 banks | 100.00% | 2 of 4 (50.0% elided) |
| `sixty_four_track_eq_only` | 8 banks | 100.00% | 2 of 4 (50.0% elided) |
| `sixty_four_track_eq_comp_simd1` | 8 banks | 100.00% | 2 of 4 (50.0% elided) |
| `nine_track_baseline` | 1 bank + 1 scalar | 100.00% | 1 of 4 (75.0% elided) |
| `nine_track_ragged_strip` | 1 bank + 1 scalar | 50.00% | 3 of 4 (25.0% elided) |
| `sixty_four_track_idle` | 8 banks | 100.00% (8 evaluations) | 2 of 4 (50.0% elided) |

`sixty_four_track_idle` is evaluated only eight times in two hundred blocks because the phase-4
silent fixed point returns before `process_channels` is reached. That makes its -5.0% a clean
attribution: on the idle row the elision is not running at all, so the whole saving is the chain
merge.

`nine_track_ragged_strip` engages on half of its EQ instances for the reason the `native_simd4`
leg barely moved: its ninth track is unbanked and runs the scalar path at depth 4.

## The chain-shape row-pair: #175's designed signal, fired

#175 wrote its transpose-count equality as `==` so that the day the graph layer took the saving it
would go red and say so. It did.

| | #175 r1 | #175 r2 | r1 r1 | r1 r2 |
|---|---:|---:|---:|---:|
| `split_chains_transposes_per_block` | 24 | 24 | 24 | 24 |
| `merged_chain_transposes_per_block` | 24 | 24 | **16** | **16** |
| `split_chains_p50` us | 95.031 | 95.261 | 91.233 | 89.851 |
| `merged_chain_p50` us | 96.303 | 96.183 | 86.635 | 85.362 |
| `paired_delta_median_ns` | +1252 | +932 | **-4569** | **-4539** |

Both arms' `output_sha256` is `30256f812f25...` in both campaigns: the four-way property holds --
placement and chain shape never move a bit. The merged layout was 1.0-1.3 us/block *slower* than
the split one in #175 and is 4.5-4.6 us/block faster now.

## Projections against measurement

| claim | projected | measured | verdict |
|---|---|---|---|
| elision, EQ increment | -9.0 us | ~-6.6 us (the `console_legacy` row, where no merge applies) | under |
| elision, intended console | 184.5 -> ~175.5 | -- | see composed |
| cohort chain, intended row | -3.2 us net | -4.55 us paired (placement record) | over |
| composed, native | ~-6.5% | -5.77% on `sixty_four_track_console` | under, same order |
| composed, wasm | -7..9% estimated | -6.73% (`sixty_four_track_console`), -7.24% (stretch) | at/just under the low end |

Attribution within the composed number: `sixty_four_track_console_legacy` puts EQ and compressor in
different racks, so the merge cannot apply there and its -6.603 us is the elision alone.
`sixty_four_track_eq_comp_simd1` is the same two effects in one rack and moves -12.775 us, so the
merge is worth about -6.2 us on that pair -- consistent with the placement record's -4.55 us paired
delta measured a different way.

## The one arm that got worse

| hoist arm, ns/block | #175 r1 | #175 r2 | r1 r1 | r1 r2 |
|---|---:|---:|---:|---:|
| `sixty_four_track_console` quiet | 14748 | 14708 | 9488 | 9478 |
| `sixty_four_track_console` restated | 18715 | 18795 | 13676 | 13626 |
| `sixty_four_track_console` **moving** | 42431 | 42500 | **44475** | **44495** |
| `nine_track_ragged_strip` **moving** | 10640 | 10660 | **11151** | **11171** |

Round-to-round spread is about 0.1%, so +4.8% on both moving rows is real and not noise. The
mechanism is the elision's own bookkeeping: `Channel::refresh_identity` runs per lane inside
`snap`, and the moving arm is the one that ends a ramp on every lane of every section. The quiet
and restated arms, which are what a rendering console actually does, improved 27-36% and still
render identical digests (`quiet_output_sha256 == restated_output_sha256`, the #144 item 6
property).

This is recorded rather than fixed. Batching the refresh to once per section instead of once per
lane is an obvious reduction, but tuning a subject after measuring it and re-running the same arm
is exactly what the benchmark discipline forbids, so it belongs in a successor issue with its own
one-shot.

## Follow-ups this measurement identified

1. **Batch `refresh_identity` per section.** Removes most of the +4.8% ramping cost above.
2. **Let the scalar backend take a shorter final interleave pass.** At `SVF_CASCADE_DEPTH = 4` the
   per-node path can only elide when every band is disabled. The extra instantiation is roster-safe
   there: the scalar monomorphisation is `...7ChannelfKj1_...` and does not match `KERNEL_ROSTER`'s
   `miso_engine_parametric_eq.*4wide6f32x4` pattern, which is why the banked path had to be padded
   and this one need not be.

## Measurement boundary -- what may not be quoted from this

* Descriptive, on one host, at one commit each. Not a release budget and not a portability claim.
* The native and wasm arms are separate runs at different commits, so a native row and the wasm
  arm's `native_simd8` leg for the same workload are not the same number.
* `comparable_with_console_records` is `false` on every wasm record: the wasm family and the console
  family are not row-comparable, only self-comparable across campaigns.
* The engagement rates came from a temporary counter, not from this directory's records.

## Links

* `artifacts/issue175` -- the baseline every row here is read against.
* `crates/miso-engine-parametric-eq/src/lib.rs` -- `cascade_sections`, and the proof beside it.
* `crates/miso-engine-graph/src/runtime.rs` -- `cohort_runs`, `chains_into`, `op_dataflow`.
* `crates/miso-engine-graph/src/program.rs` -- `lower`, whose window argument #181 restated.
