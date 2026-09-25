# Issue #880 MB-1 — L3 lane logarithm

## Implementation and numerical evidence

`log2_lane` now uses the owner-approved L3 atanh reduction. After clamping and `frexp`, the
mantissa fold leaves `t = m - 1` in `[-0.292893…, 0.414214…]`. Set `s = t / (t + 2)` and
`z = s²`; then `ln(1 + t) = 2 atanh(s) = t - s·(t - z·P3(z))`. The result uses the frozen,
unfused sequence `t·(log2(e)-1) - u·log2(e) + t + e`. The exact f32 words are:

| value | f32 word |
|---|---:|
| `P3`, highest order first | `0x3e99004a`, `0x3eccaefc`, `0x3f2aaab1` |
| `log2(e) - 1` | `0x3ee2a8ed` |
| `log2(e)` | `0x3fb8aa3b` |

The coefficients came from LP minimax on the atanh residual, sequential f32 rounding/refit, then
f32 coordinate search scored with the full two-rounding evaluation. That procedure records
coefficient provenance; M1 is the accuracy proof.

The four-worker exhaustive M1 run checked every positive normal (`2,130,706,432` values): maximum
error **1.298297 ulp** at `0x3f35ebc3` (`0.7106287`), zero decreasing steps. The unchanged exp2 row
checked `2,247,753,730` values and measured 1.461546 ulp at `0xbefb6655`, also with zero decreases.
The 2 ulp ceiling is unchanged. `m1_measured_worst_points` now uses per-row neighborhood floors:
1.4 ulp for exp2 and 1.25 ulp for L3 log2.

Two fresh exhaustive red mutations against L3 failed the same M1 gate:

- Removing the residual correction (`r = 0`) measured 165,937.506253 ulp at `0x3f3504f7`, with
  zero decreasing steps.
- Disabling the mantissa fold (threshold `2.0`) measured 1,481,070,020.477765 ulp at
  `0x3f7fffff`, with 3,194 decreasing steps.

M2 compared scalar, Simd4 and Simd8 results for every one of the `2^32` input bit patterns and
found no differences. This includes NaNs and infinities; `log2_lane` maps these inputs to its
finite clamp result. The existing one-million-point scalar digest moved only for `log2_lane`.

## Moved pins

All digests are SHA-256 over the existing corpus words in little-endian order. Only these rows
moved:

| Pin | Old | New |
|---|---|---|
| M2 `log2_lane` | `c154dc2db90e4c981251cf94d37a8e5e1171b66a418da2df9e6ad33ca767c738` | `22bbd53762cbfb3ae81c024386d5787c2d62d8293f6004f5235195f01b2ca058` |
| D1 `level_db` | `262081c8744ab2565cb908d546b355a711d0eb14984af73c753f7296036996df` | `c757e83d38d5cf74aea6ff5c2bfba66bbcb0a46fdcb7f96b60a3735cea02e460` |
| WASM `LANE_DIGESTS` `log2_lane` | `a50126cabc9545007d8f7c99d87f21006b136cea94733abbafd0c549c20f6361` | `de49ccf26b7a11d41e40acd93c02f925c147f853ad332cffda1080082059f276` |

M2 `exp2_lane` and every other D1 and WASM lane-corpus row are unchanged. The exact-tier F1
reference for `level_db` was updated from the earlier rounded `1.538e-5` dB to `1.5382e-5` dB;
the fresh exhaustive ratio check measured `1.538161e-5` dB exact versus `2.810286e-5` dB fast,
ratio 1.827, within the unchanged 2x bound.

## Division audit and MB-1 checkpoint boundary

L3 adds one `Lane::div` for `t / (t + 2)`; its denominator is in `[1.7071…, 2.4143…]`. The
production-source census found the other current uses in the true-peak limiter (required gain and
box mean), soft-clip (`u³ / 3`), and the transient shaper's fast/slow envelope ratio. The
gate-expander corpus uses division only to construct test inputs. No other production lane
division was found.

At the MB-1 checkpoint, the transient shaper still called the exact tier, so its stored corpus pin
depended on the changed `log2_lane`. MB-2's proposed shaper oracle tolerance of `2.5e-5` absolute
PCM amplitude error had no amended ruling yet. That tranche therefore did not edit or re-pin
`crates/transient-shaper/src/corpus.rs`, claim the shaper integration gate, or run
`scripts/run-wasm-gates.sh`; the gate included the deferred shaper corpus row. No timing was run at
the MB-1 checkpoint.

## Integrated status after MB-2

Under the owner's explicit delegation, root accepted the amended MB-2 bound of strict `< 2.5e-5`
absolute PCM amplitude error only for `scalar_matches_the_independent_f64_oracle`. MB-2 applied
the fast dB tier to the shaper, updated the three scalar-generated shaper pins, and passed its
focused package and oracle gates; the `0.01` dB limits and other F1/M1 gates were unchanged. In
the later combined F1 run, all eight full-domain sweeps passed. The exact-versus-fast maximum gain
difference was `1.654020e-5` dB in that integrated run (the earlier MB-2 corner report was
`1.654115e-5` dB); the fast and exact absolute gain errors against the oracle remained
`1.287460e-5` and `2.199415e-6`. Those gain figures are measurements at the four
`attack/sustain ∈ {-1, +1}` corners, not a uniform bound on arbitrary amount settings.

The integrated release transient-shaper package, workspace native gates, workspace clippy and
policy checks, combined F1 suite, and native/scalar/SIMD Wasm gates passed. MQ-1 now has one
post-change timed record on the integrated candidate; see the [MB-2 evidence](issue880-mb2.md#integrated-mq-1-timing-evidence)
and [preserved record](../artifacts/issue880/mq1-mb2/mq1-mb2.json). That timing is descriptive and
does not isolate MB-1's contribution from the integrated MB-2 change.

## Focused checks

- `cargo test --locked --release -p math --features lane --test m1_exhaustive -- --ignored --test-threads=1 --nocapture` — PASS; both full M1 sweeps.
- `cargo test --locked --release -p math --features lane --test m2_lane_identity m2_log2_lane_identity_all_f32_bit_patterns -- --ignored --test-threads=1` — PASS; all input patterns, four workers.
- `cargo test --locked --release -p math --features lane -- --test-threads=1` — PASS; 12 passed, 6 ignored across math tests.
- `cargo test --locked --release -p effect-runtime --test determinism -- --test-threads=1` — PASS; 3 passed, re-pinned level conversion matches at all widths.
- `cargo test --locked --release -p math --features lane --test f1_fast_db_bounds f1_fast_tier_stays_within_twice_the_exact_tier -- --ignored --exact --test-threads=1 --nocapture` — PASS; ratios 1.059, 1.439 and 1.827.
- `cargo clippy --locked -p math --all-targets --all-features -- -D warnings` — PASS.
- `cargo fmt --all` and `git diff --check` — PASS.
