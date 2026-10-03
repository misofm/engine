# #1208 *Meter any boundary of a submix strip and designate a master strip in host-core*: Sol verdict, attempt 1

- Reviewed: `git diff 7bdb45187 60d3289b3` (branch `codex/batch-submix-k2`, worktree
  `/home/bl/misofm/wt-submix-k2`). 5 files, +803/-19.
- Binding: `AGENTS.md`, with the rule that meters may observe any boundary without changing
  signal flow, and
  `.github/ISSUE_SPECS/1208-meter-any-boundary-of-a-submix-strip-and-designate-a-master-strip-in-host-core.md`
  with its Attempt 1 record. I also read #1209 and #1213, and DESIGN REVISION-2 M6, to judge the
  browser interim.
- Every changed path is on the authorized list:
  - `prepare.rs`;
  - `tests/prepare.rs`, three literals only;
  - the new `tests/strip_meters.rs`;
  - `host-web/src/lib.rs`, one line inside the `HostMeterRequest` construction;
  - the spec.

  The builtins compiler is untouched, and rightly so: K1 already validates meters against
  `normalized_model().strips()` (`builtins-compiler/src/lib.rs:3326-3335`).
- How I ran it:
  - I did not modify the worktree, the branch or GitHub.
  - I exported `60d3289b3` with `git archive` into `/tmp/claude-1002/v1208/src`, with its own
    `CARGO_TARGET_DIR`.
  - After every mutation and probe I restored the file and checked it with `diff` against
    `git show 60d3289b3:<path>`.
  - The probe code is in `1208-attempt1-verifier-scratch.rs` next to this file.

## Verdict: PASS

There is no BLOCKER and no MAJOR. There is one MINOR finding, about the record and the batch
boundary, plus three NITs and an INFO note. Nothing needs another attempt.

- **D1 to D4 are implemented as frozen:**
  - **D1:** `strip_id` is renamed, and a selected meter may name any strip. Rule 1 uses
    `canonical_index`, which runs over every strip since #1207. Rules 2 and 3 are unchanged. The
    default set is still `strips[..track_count]`.
  - **D2:** the codes keep their spellings.
  - **D3:** `< live_control_strips.len()`, with both docs rewritten as frozen.
  - **D4:** a field rename only.
- **No digest, fixture, ABI layout or canonical text moved.** The diff touches none, and no wire
  struct changed: `HostMeterRequest` is a Rust-only type.
- **Render stays allocation-free with seven bus meters.** That covers the reduction-fed `Input`
  stage and the banked observation path.

## Gates re-run on `60d3289b3` (x86_64 AVX2)

| Gate | Result |
|---|---|
| `cargo test -p host-core --test strip_meters` (gates 1-4) | 4 passed |
| `cargo test -p host-core` (gate 5: every suite) | all ok: unit 55, `prepare` 15+1 ign, `effect_observation` 9+1 ign, `strip_handles` 1, `submix_strip` 17, `spectrum` 9, … 0 failed |
| `cargo test -p host-web` (gate 5: browser meter and master tests) | unit 126 passed + 1 ign, `boot_transient_budget` 2, `retained_ceilings` 1 |
| `cargo clippy --locked -p host-core -p host-web -p capi -p parameter-metadata --all-targets --all-features -- -D warnings` (every crate that depends on host-core) | clean |
| `cargo fmt --all -- --check` | rc 0 |
| `RUSTDOCFLAGS='-D warnings' cargo doc -p host-core -p host-web --all-features --no-deps` | clean (see INFO 1) |
| `check-/test-host-core-policy.sh`, `check-/test-realtime-policy.sh`, `check-/test-workspace-policy.sh` | all ok |
| Workspace test command (DESIGN section 7) | **not re-run**: disk was tight (24-29 GB free); the record reports rc 0. The only consumers of the renamed field are host-core and host-web (grep), and both suites were run in full |
| Gate 7, aarch64 | not run: there is no arm64 host. It is deferred to CI `aarch64-debug` at the K2 push, as the spec allows |

## Adversarial checks

**Gate 1's oracle is a real equivalence.**

- **The setup.**
  - Session A has three transparent tracks with distinct per-lane noise. They route into `bus`
    through drawn gains and full 2x2 matrices with signed coefficients of magnitude 0.1 to 1, and
    the route IDs are a drawn permutation.
  - Session B is a single track, `ref`, carrying the same strip `S` and fed the test's own D9 sum
    in route-ID order.
- **What the `Input` comparison proves.** Comparing the two `Input` meters proves that the bus
  meter sees the full reduction: it does not see a partial sum or the first contribution, and the
  summation order is right.
- **What the downstream comparisons prove.** The six downstream boundaries prove that the meter
  is attached at the same stage of an identically processed strip.
- **Why a stage or lane mix-up cannot pass.**
  - The pairwise check `assert_ne!` over the reference's seven meters ensures this. It exempts
    only the two pairs that coincide by construction on the console-less strip.
  - The two lanes differ, because the trims are +3 dB and -2 dB and the per-lane noise is
    independent.
  - Every snapshot word is compared, handle and window included, with NaN folded per decision 10.
- **The deviation is acceptable, and it strengthens the gate.** The spec is self-contradictory:
  with no console slots, `post_input` equals `insert_send` and `insert_return` equals
  `pre_fader`, so "every tap differs" cannot hold. Even seeds keep the spec's strip literally. Odd
  seeds add a live EQ to each console section, which makes all seven boundaries pairwise
  distinct. The bus-against-ref comparison is the same on every seed.

**Probes beyond the committed gates** (scratch file; all pass bit-identically):

- **Probe A: other metric sets.** It runs gate 1 with `SAMPLE_PEAK` alone, `COUNTS`,
  `ENERGY_RMS`, `SAMPLE_PEAK|COUNTS` and `HELD_PEAK`, on 8 seeds each. The committed gate uses
  only `ALL`, and `SAMPLE_PEAK` alone is the browser's #943 bank block-peak path, which #1209 will
  use on buses.
- **Probe B: two buses side by side.** Two buses carry strip `S` and receive different sums.
  Their meters are interleaved with a track meter in one selection, `[bus2, t1, bus] x 7 taps`,
  and each bus equals its own one-track reference, on 8 seeds. This rules out cross-bus lane
  assignment in a bank, which gate 1's single bus cannot see.
- **Probe C: more input processing.** I added input delays (37 and 5 samples), an HPF, an LPF
  and a polarity inversion to `S`. Gate 1 on 16 seeds and gate 4 still pass, so the bus `Input`
  and post-input boundaries agree with a track's even when the #1201 input delay is active.

**Mutations** (each applied alone, run, then reverted):

| # | Mutation | Result |
|---|---|---|
| M1 | master bound `< live_control_tracks.len()` | gate 3 red; every other test green |
| M2 | selected meters stably partitioned, tracks first, so a bus meter moves after the track meters while handles stay `index + 1` and rule 2 is satisfied | **only gate 2 red**; gate 1 and `tests/prepare.rs` green |
| M3 | builtins compiler: a submix's `PostFader` meter attached at `PostMatrix` | gate 1 red at seed 0 |
| M4 | `Box::new` in `MeterObserver::observe_resident`, the banked path | gate 4 red (SIGABRT from the render audit) |

The implementer's own mutations cover the `<= strips.len()` bound, a lane swap, a track-only
known set and a track-only `canonical_index`.

## Test value, one sentence per new test

- `a_bus_meter_reports_the_words_of_the_same_meter_on_a_track_fed_its_sum`: it turns red if a
  submix's meter is bound to the wrong stage or lane, or sees anything but the full D9 reduction
  (M3). No other test meters a bus.
- `selected_strip_meters_keep_the_callers_order_and_an_unknown_strip_is_refused`: it turns red if
  selected bus meters are reordered relative to track meters (M2). It is the only test with a
  mixed bus-and-track selection, and the only one where a valid bus meter must not draw a second
  `unknown_track` beside the invalid ID.
- `the_last_submix_can_be_designated_master_and_one_past_it_is_refused`: it turns red if the
  master is bounded by the tracks (M1) or by anything past the strip list.
  `effect_observation.rs`'s bound test has no submix, so it cannot see either.
- `seven_bus_meters_render_without_allocating`: it turns red if a bus meter's observation
  allocates on the render thread (M4). It is the only test that renders under the audit with a
  meter on a submix stage, the reduction-fed `Input` included.

## The browser interim

When `live_control_master_track_plus_one` is in `T+1 ..= T+S`, a browser boot now prepares where
it used to refuse.

- **What happens in the meter frame.** The header echoes the index, and `master_gr_present`
  stays 0, so `masterGrDb` is `null`. The fold skips strip indices `>= T` (`lib.rs:3412-3416`),
  and `observation_present[bus]` is never set.
- **No bounds problem.** `observation_present` is sized to `strip_count` and every access is
  guarded with `get`. No JS reader indexes anything with `master_track_plus_one`: grep finds no
  `getUint32(40, …)` in `web/` and none in the SDK.
- **It is consistent with the existing contract.** `masterGrDb` is defined as "absent when the
  designated strip published nothing", and a bus cannot publish in the browser before #1213,
  because kinds 7 and 8 do not yet address it.
- **It is planned and documented.** REVISION-2 M6 moved the browser bus-master gate to slice 16,
  and #1213's gate 6 now owns it. The Attempt 1 record states the consequence.

**Acceptable as an interim inside the unpushed K2 batch.** See MINOR 1 for the condition.

## Findings

### MINOR 1: D4's frozen text and the batch boundary

- **The problem.** The spec's D4 says host-web "changes no behaviour", but a browser boot with a
  bus master now prepares where it used to refuse. The Attempt 1 record says this candidly, but
  the frozen decision still reads as if nothing changed.
- **Fix:**
  - Add one line to D4, or to the record's D4 bullet: "Consequence: the browser boot now admits
    a bus index; `masterGrDb` stays `null` until #1213 (gate 6)."
  - The root must not push K2 to `main` without #1213. If #1213 fails its attempts, either keep
    a track-only bound in host-web's boot (`lib.rs:5590`) or revert D3 at the batch boundary.

### NIT 1: two stale "per-track" docs in `prepare.rs`

Both now also cover a selected bus meter:

- `HostLiveControlRequest.meter_queue_depth` (`:294`), "Bounded per-track meter snapshot queue
  depth": say "per-meter".
- The handle comment (`:990`), "stable in canonical track order": for a selection, handles
  follow the caller's order. Say "stable in request order (canonical track order for the default
  set)".

The second one predates this slice, but a selection that may now mix strips makes it misleading.

### NIT 2: the browser docs are stale, but #1213 owns them

`shipped-host.d.ts:729-733` and the `.d.ts` copy still say "V1 has no structural master bus --
submixes … carry no effect racks". `host-web/src/lib.rs:5579-5580` says it refuses "what cannot be a
track index". The spec assigns these to #1213, so no action is needed here. I list them only so
#1213's verifier checks them.

### NIT 3: gate 1 covers only `MeterMetricSet::ALL` and one bus

Probes A and B show both gaps are clean today. Neither is required here. #1209's gate, which
requests `SAMPLE_PEAK` bus meters at `PostMatrix`, should cover the `SAMPLE_PEAK` path with a
bus-against-track or bus-against-sum comparison, not just "nonzero".

### INFO 1: `cargo doc -p host-core` fails on its own

`cargo doc -p host-core --no-deps` without features fails on `lib.rs:19`, an intra-doc link to
the feature-gated `SessionControlProvider`. The workspace doc gate passes through feature
unification. This predates the slice and the diff does not touch it.

## Scratch cleanup

`/tmp/claude-1002/v1208` (export plus target, about 2.5 GB) was deleted after the run.
