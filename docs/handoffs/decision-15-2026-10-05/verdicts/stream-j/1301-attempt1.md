VERDICT: PASS

# #1301 attempt 1 -- adversarial verdict

Commit `ab9620a07` (base `492d0153d`), decision-15 stream J. I reviewed it in an export of the commit
at `/tmp/claude-1002/v1301/src` with `CARGO_TARGET_DIR=/tmp/claude-1002/v1301/target`. Every
mutation I applied in the export was reverted and `cmp`-checked against `git show ab9620a07:<path>`.
I did not touch the worktree.

## Scope check

- The diff touches two files: `crates/conformance/src/randomized.rs` and the spec. Both are
  authorized. No production code, caller test, `MUTATIONS.md`, workflow or script changed.
- D1/D4: `edge_restore_positions` (`randomized.rs:2855`) builds
  `{0, n-2, n-1, k-2, k-1, k} U {s : s mod 16 == 4r mod 16}` within `0..n`. It drops negatives
  with `checked_sub`, then sorts and dedups the result. With `every_sample` it returns `0..n`. Its
  signature matches D4.
- D2: `overshooting_start` (`:2825`) returns `(start, k)`, where `k` is the step count at which
  the walk first leaves the domain (`1 <= k < n`). It is otherwise unchanged.
- D3: `overridden()` is read once per probe call (`:2712`) and passed in.
- Hazard "keep rendering every sample": the skip (`:2786`) sits after `effect.process(block)`, so
  only the snapshot, the twin and the restore are skipped.
- D6: the violation format string and both exported signatures are unchanged. The doc comment
  (`:2658-2671`) now states D1 and D3.
- D5: the test module is at `:2898`.

## Gate 2 scrutiny: the implementation is not slower than the prototype

I instrumented the export temporarily and reverted it. The green per-PR delay probe makes
**412 snapshot/restore pairs, 13.4 % of the full walk's 3 072**:

| Rate | Pairs |
| --- | --- |
| 44.1 kHz | 88 |
| 48 kHz | 108 |
| 88.2 kHz | 108 |
| 96 kHz | 108 |

Row 0 has fewer pairs because its stride offset 0 overlaps samples 0, 32 and 48.

The spec gives the cost of each pair at each rate. From that split, 412 pairs predict about 9.3 s
of snapshot and restore, plus about 0.2 s of twin preparation, which is about 9.5 s in total. The
prototype measured 9.65 s.

The probe adds one cost per sample: a `binary_search` over at most 14 `u64`. It does not allocate
per sample. It computes the position `Vec` once per edge and reads `overridden()` once per call.
It makes no redundant clones. There is no avoidable per-sample cost.

My re-measurement, run once after a `--no-run` build, gave `finished in` **10.91 s**. Host load
was 25.72 / 26.22 / 36.12 before the run and 24.69 / 26.00 / 35.89 after (32 cores). That load
matches the implementer's ~26. The instrumented run, which adds one `eprintln` per edge, gave
9.74 s. So the implementer's 14.95 s was load variance on a busy host, not implementation cost.

## Findings

No BLOCKER, MAJOR or MINOR.

- **NIT-1 (evidence).** The attempt record's gate-2 line in the spec at `ab9620a07` reads
  "14.95 s ... almost no margin". It does not attribute that number to load.
  - Fix: append the pair count (412 of 3 072, the per-rate split above) and this re-measurement
    (10.91 s at load ~25). A later reader then does not mistake the margin for implementation cost.
- **NIT-2 (coverage).** No committed test pins two behaviours. PR evidence covers both, and the
  spec's D4/D5 design accepts the gap, so no change is required.
  - The probe wiring `overridden()` -> `every_sample` (`:2712`): a hard-coded `false` would
    silently drop the nightly's every-sample walk.
  - `k - 2`: D5's required list does not include it, so a window narrowed to `k-1 ..= k` keeps
    the test green. It also keeps every known catch, because all of them land at `k - 1`.
  - Evidence that the wiring works:

    | Run | Per-PR | Full walk |
    | --- | --- | --- |
    | Release, delay probe | 0.71 s | 5.22 s (`MISO_ENGINE_RANDOMIZED_SCALE=100`) |
    | Debug, delay probe under M18 | 8.36 s | 59.34 s (`MISO_ENGINE_RANDOMIZED_SCALE=1`) |

## Gates I ran in the export

### Gate 1: reach is not lost

Each mutation was applied, then the probe was run per-PR (no variable) and full
(`MISO_ENGINE_RANDOMIZED_SCALE=1`). I compared the "its own snapshot is refused" lines with `cmp`.

| Mutation | Per-PR | Full | Identical | After revert |
| --- | --- | --- | --- | --- |
| Delay M18 (`lib.rs:1623` strict `current_valid`, `:1626` `ramp_path_within` removed) | 24 red (8.36 s) | 24 red (59.34 s) | yes | green |
| Gate strict current (`gate-expander/src/lib.rs:801`) | 32 red | 32 red | yes | green |
| Limiter 4-ulp budget (`true-peak-limiter/src/lib.rs:3962`) | 4 red | 4 red | yes | green |

Where the refusals land:

- **Delay M18:** after sample 33 (k = 34) and after sample 48 (k = 49). They cover `feedback`,
  `mix` and `cross feedback`, both edges, at all four rates.
- **Gate:** all after sample 33.
- **Limiter:** all on `ceiling` toward -24 dB, after sample 0.

"After revert" means the full debug suite in gate 3 was green.

D5's test, three mutations, each red and then green on revert:

- Window changed to `k ..= k + 1`: red, `n 64, k 34, row 0: 33 missing from [0, 16, 32, 34, 35, 48, 62, 63]`.
- `Some(0)` dropped: red, `n 64, k 34, row 1: 0 missing from [4, 20, 32, 33, 34, 36, 52, 62, 63]`.
- `every_sample` early return removed: red on the `0..n` equality.

### Gate 2: cost, debug, one run each after `--no-run`

| Crate | `finished in` | Ceiling |
| --- | --- | --- |
| delay | 10.91 s | 15 s |
| true-peak-limiter | 0.14 s | 1 s |
| multiband-compressor | 0.07 s | 1 s |
| compressor | 0.08 s | 1 s |
| gate-expander | 0.01 s | 1 s |
| transient-shaper | 0.01 s | 1 s |

The CI leg is not measurable without a push. Root must check the items below.

### Gate 3: nothing else moved

- **Debug.** `cargo test --locked -p conformance -p delay -p compressor -p gate-expander
  -p true-peak-limiter -p transient-shaper -p multiband-compressor`: exit 0, 77 `test result: ok`.
  In that run the delay's `tests/randomized.rs` binary finished in 12.83 s, with both of its tests
  running in parallel.
- **Release, nightly shape.** `MISO_ENGINE_RANDOMIZED_SCALE=100 cargo test --locked --release
  -p delay -p compressor -p gate-expander -p true-peak-limiter -p transient-shaper
  -p multiband-compressor --test randomized`: exit 0, six binaries ok. The delay's probe alone in
  that shape finished in 5.22 s, which is the full walk. The spec measured 4.44 s and the
  implementer 4.86 s.

### Gate 4: policy

All pass:

- `cargo fmt --all -- --check`
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` (exit 0; it
  checked conformance)
- `bash scripts/check-workspace-policy.sh`
- `bash scripts/test-workspace-policy.sh`
- `bash scripts/check-conformance-boundaries.sh`

## What root must check on the PR's qualification run (gate 2, CI leg)

1. **`test-debug-b` ("DSP crates debug tests").** The delay's `tests/randomized.rs` binary
   reports `finished in` <= 15 s. Before this change it was 80.18 s in run 37201085656.
2. **Both `test-debug-b` and `aarch64-debug`.** No "has been running for over 60 seconds" line
   names `the_effects_own_edge_ramp_snapshots_restore`.
3. **Record the evidence.** Record the `aarch64-debug` `finished in` for that binary (before:
   113.30 s). Paste both CI numbers into the spec's Evidence section.

Expectation: before the change, CI ran at about 1.1 times the local probe (80.18 s against
72.45 s). Locally the binary now takes 10.9-12.8 s, so `test-debug-b` should land at about
12-14 s. That is under the ceiling but not by much. If it exceeds 15 s, the spec's hazard
applies: stop and report, and do not tune the stride.

No `MISO_ENGINE_RANDOMIZED_*` variable is set anywhere in `qualification.yml` or
`scripts/run-aarch64-tests.sh`. Only `nightly.yml:176` sets one. So CI takes the per-PR set, as
intended.

## Test value

One new test: `randomized::tests::edge_restore_positions_keep_the_reach_critical_samples`.

Which plausible defect turns it red that no existing test catches? An edit to
`edge_restore_positions` that narrows the `k` window, drops sample 0, `n - 2` or `n - 1`, or
ignores `every_sample` keeps all six effect probes green today, because no effect is defective. It
would silently lose the per-PR catches for delay M18, gate strict-current and the limiter's 4-ulp
budget, or the nightly's every-sample walk. This test is red on each such edit. I reproduced three
of them above.

No other test was added or rewritten. The six caller tests are unchanged.
