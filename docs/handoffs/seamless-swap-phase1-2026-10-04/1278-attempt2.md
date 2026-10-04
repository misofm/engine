# #1278 attempt 2 verdict: FAIL

One MAJOR. Every attempt-1 finding is fixed and every gate I re-ran is green, but the fix for
MINOR-4 hides a hole that is bigger than MINOR-4 was. The conformance differential **never starts a
parametric-EQ ramp**. The EQ refuses raw automation spans; it takes only prepared targets, and the
harness never applies one. Yet the new in-flight coverage counter credits the EQ with 206
"in flight" continuations, 55 of them at quantum 32. In fact none of them is in flight.

I checked what this leaves exposed. I made the EQ's restore re-derive a moving step as
`(target - current) / remaining`. That is exactly the defect Gate 3 exists to catch. Every EQ test
and the differential stay green (13 test binaries). A 60-line probe turns red. The EQ's product code
is correct today: the probe is green on `a070cfa7d`. The fix is test-only.

- **Commit reviewed:** `a070cfa7d`. The fix is `git diff 46ef4f263 a070cfa7d`. I also read the
  cumulative `git diff 983ac85bd a070cfa7d` for regressions. `d6217a79d` was reviewed at attempt 1.
- **Export:** `/tmp/claude-1002/v1278/attempt2/`. Mutations ran in a separate copy,
  `scratch/mut/`. Each was reverted and `cmp`-checked against the export afterwards.
- **Logs and probes:** in `/tmp/claude-1002/v1278/attempt2/scratch/`:
  - `probe_eq_mid_ramp.rs`;
  - `probe_midramp_count.patch`, the mid-ramp decoder;
  - `cont-*.log`, `mut-*.log`, `old-*.log`, `negcoef.log`.

## Gates I re-ran

| Gate | Result |
|---|---|
| `cargo fmt --all -- --check` | ok |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | ok |
| `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | ok |
| `check-workspace-policy.sh` / `test-workspace-policy.sh` | ok / ok |
| `check-realtime-policy.sh` / `test-realtime-policy.sh` | ok (**78 regions in 23 files**) / ok |
| `cargo test --locked --workspace` (debug) | ok: 308 result lines, 0 failed |
| `cargo run --locked -p conformance --example conformance_fixtures -- --check` | ok. Nothing re-pinned, and `fixtures/` is untouched in the cumulative diff |
| `cargo test --locked --release -p console-workload` | ok. Console digests unchanged |
| `cargo build --release -p audit -p capi && audit capi` | 0 allocations, 0 deallocations, 0 locks, 0 syscalls, 0 violations (100k calls) |
| `check-capi-abi.sh` | ok (shared and static) |
| `cargo build --release -p bench` and `trace-effect-contract-audit.sh target/release/bench 1000000` | ok (1M blocks) |
| `check-effect-contract.sh target/release/bench` | ok (8 production factories, 0 failed gates) |
| `check-cross-targets.sh` | PASS. Only the expected #1018 iOS `memset_pattern16` rows fail |
| Worklet chain: `build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh --without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts`, `test-web-audioworklet.sh` | all ok. The module's sha256 is `1928ba471a57c01668ea089947d0b4e15f9fd40fcadc7e93726e659a691a5f6d`, the same as the implementer's build |

**Width.** On this AVX2 host only the limiter and the multiband bind both widths: banks `[8, 8]` and
`[12, 12]`. The compressor, transient shaper, gate, soft-clip and EQ decline `Four` by design, so
the count is `[0, n]`. Their `Simd4` payload calls run only in CI's aarch64 legs. I could not verify
them locally.

## Attempt-1 findings: each fixed?

- **MAJOR-1 (soft-clip edge probe): fixed.**
  - The call is deleted.
  - `assert_edge_ramps_restore` keeps its five other callers: compressor, gate, multiband,
    transient shaper and limiter.
  - The seeded `a_restored_near_edge_ramp_continues_bit_for_bit` is untouched.
- **MINOR-1 (compressor coefficient bound): fixed.**
  - A target and a settled current are held to `[0, 1]`. A moving path is held to
    `[0, 1 + 64 ulps]` through the new `ramp_path_inside`.
  - I re-ran attempt 1's `probe_negative_coefficient.rs`. The settled `-7e-6` release coefficient
    that used to run away is now refused with `effect.state.parameter` (`scratch/negcoef.log`).
  - I applied four mutations to the new test. Each turns
    `a_coefficient_below_zero_or_above_its_design_is_refused` red:
    - the path's lower bound at `-64 ulps`;
    - the target check removed;
    - the settled-current check removed;
    - attempt 1's symmetric bound restored whole.
  - Under the last of these, every other compressor test is green (`--no-fail-fast`). So the catch
    is unique.
  - The comment is now correct. Exactly `0.0` is still accepted; see NIT-1.
- **MINOR-2 (limiter): fixed.**
  - `read_lane` holds the target strictly in range and the settled current equal to the target
    bitwise. It walks a moving ramp with `ramp_path_inside` over the relaxed bounds, so the step is
    bounded too.
  - Each of the four new rejection rows has its own red run:
    - target check removed: "limit target one ulp above the ceiling was accepted";
    - settled-bits check removed: "settled limit one ulp above the ceiling";
    - walk reduced to its endpoints: "moving limit walked past its bounds by its step";
    - settled step forced to zero: "settled limit with a nonzero step".
  - With the path slack set to 4 ulps, the limiter's
    `the_effects_own_edge_ramp_snapshots_restore` is red, so 64 ulps is still needed.
  - **The rewritten randomized linked scenario is not weakened.**
    - Its old crafted ramps walked the limit to zero, below zero, or to infinity. They are now
      refused at restore, so the only thing they could still exercise is the refusal.
    - The new crafted ramp is an admissible state that no retarget produces: `target == current`
      with a nonzero step that walks 0.5 or 0.9 of the way to an edge of the ceiling range, then
      snaps back. That still drives a large discontinuous limit jump through the linked body and
      the reference kernel, inside the domain.
    - Measured with a temporary print in the debug run (both widths): 17 of 18 crafted restores
      were accepted. The record says 19 of 20. The difference is immaterial.
    - Both arms must still agree on every verdict and every word.
- **MINOR-3 (realtime regions over the codec bodies): fixed.** I inserted `vec![0_u8; 4]` into six
  places. `check-realtime-policy.sh` is red for each, with "marked realtime forbidden-body
  predicate":
  - the transient shaper's `read_lane`;
  - the gate's `parse_lane`;
  - the gate's `rederive_lane`;
  - the multiband's `stage_side`;
  - the EQ's `Channel::restore_track`;
  - the EQ's `read_payload`.

  The floors are not raised. That is accepted: the script is outside the slice's paths, and the
  follow-up is recorded.
- **MINOR-4 (in-flight coverage): fixed for six effects, vacuous for the EQ.**
  - My decoder logs, at each `Continuation::take`, whether a ramp's `remaining` word is nonzero
    (`probe_midramp_count.patch`). It counts continuations with a ramp actually in flight,
    against the proxy's count. Each cell reads actual / proxy; numbers in brackets are at
    quantum 32:

    | Effect | Actual / proxy | At quantum 32 |
    |---|---|---|
    | Compressor | 169 / 210 | [35 / 49] |
    | Gate | 152 / 179 | [26 / 33] |
    | Multiband | 44 / 57 | [7 / 9] |
    | Soft-clip | 60 / 79 | [2 / 7] |
    | Transient shaper | 166 / 211 | [33 / 50] |
    | Limiter | 131 / 157 | [15 / 21] |
    | **EQ** | **0 / 206** | **[0 / 55]** |

  - Wherever a ramp moved, the proxy also counted it.
  - The multiband rose from attempt 1's 3 [0] to 44 [7]. This matches the record.
  - With the preference disabled, the multiband differential is red on the new clause.
  - The new default harness now catches **both** of `983ac85bd`'s re-deriving crates without
    forcing quantum 32, through the continuation oracle (`old-*.log`):
    - transient shaper: "rendered 0xbe3b34d3 where the lane, continuing, rendered 0xbe3b34d2";
    - compressor: "0x3e05681b ... 0x3e05681c".

    At attempt 1, the transient shaper needed the harness forced.
  - For the EQ, see MAJOR-1.
- **MINOR-5 (docs): fixed.**
  - `compressor/tests/ramps.rs` header.
  - `transient-shaper/tests/contract.rs` doc. I re-ran its new mutation, and it is red at
    `contract.rs:239` as the doc says.
  - `transient-shaper/tests/MUTATIONS.md` row 4 and its section.
  - `compressor/tests/MUTATIONS.md` row 13. I verified it: a `/remaining` re-derivation turns
    `a_mid_ramp_restore_continues_the_ramp_exactly` red, and the differential too.
  - The spec's D2a amendment.
  - `docs/EFFECT_CONTRACT_V1.md`, both passages: "render-safe", and version `1` kept before launch.
    They are accurate, apart from the EQ claim that MAJOR-1 leaves undefended.

**Also verified:**

- **Allocation-free restore.** A `vec!` in the limiter's new `read_lane`, or in the compressor's
  `validate_channel`, turns its differential red: "restore_state_payload: forbidden operations ...
  allocations: 2, deallocations: 2". The audit is real on the new paths.
- **Console digests and conformance fixtures** are unchanged.
- **No new `unsafe`.**
- **Authorized paths only.** `docs/EFFECT_CONTRACT_V1.md` was authorized for this attempt.
- **The commit** ends with the `Co-Authored-By` trailer.
- **ARTIFACT CHANGED.** Confirmed: the module is now `1928ba47...` (attempt 1 amendment: `0f508f8c...`), because the compressor's and the limiter's restore bounds are compiled into it. The pin `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256` still reads `6c952a2c...` and is untouched by the branch, as `docs/RELEASE.md` requires.

## Findings

### MAJOR-1: the restored-against-continued oracle never reaches an EQ ramp, and the new in-flight clause claims it does

**The code involved.**

- `crates/conformance/src/randomized.rs:906-937` (the preferential take and the counters) and
  `:1028-1032` (`in_flight_until`).
- `crates/parametric-eq/src/lib.rs:3479-3483` and `:3689-3693`. The scalar and bank `process`
  count every raw span as `invalid_spans` and render the existing state unchanged. EQ automation
  enters only through `apply_prepared_target` (#807, `docs/EFFECT_CONTRACT_V1.md` "Parametric EQ
  uses this route").
- The harness never calls `apply_prepared_target` or `target_preparation()`.

**Evidence.**

1. **No EQ ramp starts in the differential.** I added a temporary `eprintln!` in `start_ramp`. In
   the default EQ differential it printed nothing: zero ramp starts across 24 seeds, with 5,575
   spans delivered.
2. **The decoder confirms it.** It reads EQ `remaining` at word `19*s + 14` of each section. It is
   0 in all 226 continuations.
3. **The clause is not tied to the EQ.** With the preference disabled, the EQ's coverage clause
   still passes. It is satisfied by spans the EQ refused.
4. **The defect class goes undetected.** I mutated `Channel::restore_track`'s commit
   (`crates/parametric-eq/src/lib.rs:2729-2733`) to store
   `(target - current) / remaining` as the step of a moving band, instead of the carried step. That
   is the class-B re-derivation the compressor and transient shaper had before this slice.
   `cargo test --no-fail-fast -p parametric-eq` stays **green in all 13 binaries**
   (`scratch/mut-eq-rederive.log`), including:
   - `bank.rs`, which restores lanes into themselves, so both legs re-derive alike;
   - `contract.rs`'s `prepared_target_applies_and_settles_at_sample_a_plus_64`. It restores only
     after sample 1 of the ramp, where `/63` happens to be exact.
5. **The probe catches it.** `scratch/probe_eq_mid_ramp.rs` restores a bell ramp into a fresh
   instance after each of samples 1 to 63 and compares 100 frames bitwise. It is green on
   `a070cfa7d` and red under the mutation from k = 5 on.

**Why it matters.**

- Deliverable 2 asks for "at random points, including mid-ramp and at quantum 32", and Gate 3 asks
  for "every banked effect at both widths". The EQ is on the slice's list of effects that bank.
- The spec's own test-value line for Gate 3 is "a restore that re-derives a ramp's step ... the
  new oracle turns red". That is false for the EQ.
- The attempt record reports "EQ 226/206/55" as in-flight continuations.
- The EQ is the console-slot effect on most tracks, so it is the carry's most frequent restore.
  Nothing defends its mid-ramp exactness except a single sample-1 point.
- Attempt 1's MINOR-4 was ranked MINOR only because the multiband's round-trip check catches a
  step re-derivation (verified red). For the EQ, that check does not (verified green).

**Fix.** Either option is within the slice's authorized paths.

- **(a) Drive EQ ramps through the harness.** When `factory.target_preparation()` is `Some`, lower
  a drawn span's value through `prepare_targets` off the audited scope. Apply the target through
  `apply_prepared_target` on the scalar and the continuation twin, and
  `apply_prepared_target_lane` on the bank lane, at the block boundary. Count in-flight only for a
  lane whose automation was admitted: a refused span (`invalid_spans`) or a stationary target does
  not start a ramp. This also gives the EQ's bank-against-scalar differential the ramp coverage it
  has lacked since #1051.
- **(b) Minimal.**
  - Add an EQ crate test that restores at every sample of a prepared-target ramp into a fresh
    scalar instance **and** a fresh bank lane (`restore_track_state_payload`), at both shipped
    widths, and compares bits. Start from `probe_eq_mid_ramp.rs`.
  - Make the proxy ignore spans the lane's report refused, so the EQ's in-flight clause is honest.
  - Record the EQ's oracle narrowing explicitly in `crates/parametric-eq/tests/randomized.rs`.

In both cases, record the step re-derivation mutation red, and correct the attempt record's EQ
numbers.

### NIT-1: the compressor still accepts a settled coefficient of exactly `0.0`

- **Where:** `crates/compressor/src/state.rs:144-148`.
- **What happens:** `0.0` is inside `[0, 1]`, and a zero release coefficient freezes the smoother.
  Verified: a settled release coefficient of `0.0` restores. The gain reduction then holds at
  -12.68 dB through 3 s of input at -40 dB, until an automation retargets the coefficient.
- **Why it is only a NIT:** it is bounded, finite, hostile-only, and the attempt-1 fix text allowed
  `[0, ...]`.
- **Fix:** a strictly positive lower bound for targets and settled values, for example
  `rate_coefficient(5000 ms, rate)`, would make validation match what the effect can reach.

### NIT-2: the shared walk is not inside a realtime region

`effect_runtime::state_payload::{ramp_path_inside, ramp_path_within, read_ramp}` run inside the
payload codecs, but no realtime region covers them. A `vec!` there would pass the static scan. The
runtime audit in the differential would still catch it, because the compressor, limiter and
transient shaper call them. For a successor: mark them, together with the floor raise to 78/23.

### NIT-3: the EQ's restore runs the coefficient designer twice per band

`band.target.words(sample_rate)`, that is `design_svf`, runs at validation and again at commit:
up to 24 designs per lane restore. That is now on the render thread in the swap block. It is
bounded, loop-free and allocation-free (audited), so it does not break AGENTS.md. It does go
against the EQ's own rule, "never design coefficients on the render thread" (the `process`
comment), and against the contract's "do not invoke the coefficient designer". Two ways to settle
it:

- record it for the umbrella's swap-block cost budget (slices 10-11b); or
- reuse the validation pass's words at commit, which halves the cost.

## Test value: one sentence per new or rewritten test

| Test | The plausible defect that only it catches |
|---|---|
| compressor `payload::a_coefficient_below_zero_or_above_its_design_is_refused` | A coefficient bound that admits a negative (divergent) or above-design coefficient. Attempt 1's symmetric bound is red only here (verified with `--no-fail-fast`) |
| limiter `state_round_trips_and_rejects_corruption`, row "settled limit one ulp above the ceiling" | A settled coefficient held only to the relaxed bounds (red when the bits check is removed) |
| limiter, row "limit target one ulp above the ceiling" | A target held only to the relaxed bounds, which lets a ceiling sit above 0 dBFS (red when the target check is removed) |
| limiter, row "moving limit walked past its bounds by its step" | A path checked at its endpoints only (red) |
| limiter, row "settled limit with a nonzero step" | A settled ramp that keeps a stale step (red) |
| limiter `randomized_scenarios_match_the_current_frame_law` (rewritten crafted ramp) | A randomized differential, judged by reach: it now reaches an accepted, non-monotone in-flight limit ramp with a large snap (17 of 18 accepted), where the old crafting reached only refusals |
| conformance in-flight preference and `assert_reached`'s in-flight clauses | A harness that stops taking continuations mid-ramp or at quantum 32 (the multiband is red with the preference off). It is **vacuous for the EQ** (MAJOR-1) |

## Open items carried forward (unchanged, recorded in the spec)

- The delay refuses its own edge-ramp snapshots.
- Soft-clip has two open non-finite history cases.
- Soft-clip validates an in-flight current by its line, not by `ramp_path_within` (NIT-2 at
  attempt 1).
- Raise the realtime floors to 78 regions and 23 files.
