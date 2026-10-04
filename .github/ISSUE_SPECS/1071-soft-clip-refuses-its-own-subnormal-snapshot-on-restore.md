# Soft-clip refuses its own subnormal snapshot on restore

Found by #1051's randomized differential (`crates/conformance/src/randomized.rs`, "#1051 defect 3", seed 0).

## Problem

Soft-clip can produce an in-memory state snapshot containing a subnormal value, then refuses that same snapshot when it is restored (for example across a plan replacement). That breaks the snapshot/restore round trip and the effect's documented denormal behaviour: a state the effect produced must restore.

## Smallest closable slice

Either flush the subnormal when the snapshot is taken, consistent with the effect's denormal rule, or accept it on restore; state which and why against the effect contract. Un-ignore the reproducer.

## Gates

1. The reproducer passes; a snapshot/restore round trip is identity for every state the effect can produce, subnormals included.
2. Hostile (not self-produced) snapshots are still validated as before.
3. Console digests unchanged; render allocation-free.

## Attempt record

### Attempt 1 (implementer)

**Decision: accept on restore, not flush on snapshot.** The restore now accepts exactly the words
the effect itself can hold and still refuses the rest:

- ramp current and target: finite, not `-0.0`, inside the converted domain. The subnormal clause
  is gone; only the mix's `[0, 1]` can hold a subnormal (the gains' converted ranges start at
  `-24 dB`, a normal gain, so a subnormal gain is still refused as out of domain);
- ramp step: finite (a ramp toward or from a subnormal mix divides a subnormal difference);
- `X` and `e` histories: finite and zero-or-normal as before, because the kernel flushes both
  before they enter a history (D7), so a subnormal there is never self-produced;
- dry history: finite. It holds the input unflushed by design (the identity path reproduces any
  input sample), so a subnormal input sample is legal state.

Why not flush on snapshot: a flushed dry sample or mix changes rendered bits (a subnormal dry
sample is the output at `mix == 0`), which would break #1278 D2a's bit-exact continuation of a
restored lane. Accepting on restore keeps the payload layout (version 1) unchanged, so no fixture
moves.

Changes: `crates/soft-clip/src/lib.rs` (`converted_value_valid`, `decode_lane_words` and their
docs); `crates/soft-clip/tests/state_roundtrip.rs` (new
`a_snapshot_holding_subnormal_words_restores_and_continues_bit_for_bit`; three hostile words added
to the rejection test: subnormal `X` history word 12, subnormal drive current word 0, NaN dry word
73); `crates/soft-clip/tests/randomized.rs` (per-PR differential now `known: &[]`; the ignored
twin, which differed only by seed count, is deleted as superseded);
`crates/conformance/src/randomized.rs` (`Known::SubnormalStateRefusedOnRestore`, its two
narrowing sites and `SUBNORMAL_REFUSAL_CODES` removed; no other effect used them).

Mutation evidence (each applied, run, reverted; green after revert):

| # | Mutation | Test | Result |
|---|---|---|---|
| M1 | revert `lib.rs` to `41517fc35` | `randomized` (narrowing removed) | RED: `lane 0 refused its own snapshot` (Eight, block 9) |
| M1b | same | `state_roundtrip` new test | RED (restore refused) |
| M3 | treat `X` history as unflushed (accept subnormal) | rejection test, `bad(12, 1)` | RED |
| M4 | drop the dry history's finiteness check | rejection test, `bad(73, NaN)` | RED |
| M6 | keep `normal_or_zero(step)` | new test (subnormal mix step) | RED |
| M7 | treat dry history as flushed (refuse subnormal) | new test (subnormal newest dry) | RED |
| M5 | refuse subnormal only for the gains | all | GREEN, as expected: a subnormal gain is out of domain either way (equivalent mutant) |

Gates:

1. Reproducer passes: `cargo test -p soft-clip --test randomized` with `known: &[]` green; also
   `MISO_ENGINE_RANDOMIZED_SCALE=3` (24 seeds, as the deleted twin ran): green, 75 restores.
   Every other effect's `--test randomized` green.
2. Hostile snapshots still refused: the rejection test, with the three added words, green.
3. `cargo test -p console-workload --release` green (every digest row unchanged); soft-clip's
   `allocation` test green; `audit capi` 0 allocations, 0 syscalls. `cargo test -p soft-clip -p
   conformance` green; `conformance_fixtures --check` exit 0.

Inherited gates: fmt, clippy (workspace, all targets/features, `-D warnings`), rustdoc
`-D warnings`, `check-/test-workspace-policy`, `check-/test-realtime-policy`, `check-capi-abi`,
`check-cross-targets` (PASS, #1018 expected iOS failures only): all green.

Worklet chain: `build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh`,
`check-browser-expected-resources.py --artifacts`, `test-web-audioworklet.sh`: all green.
**ARTIFACT CHANGED**: shipped module `30d075d3ce6382f21235675996184c675753acf6d451e11d7d676a3d50aaeff4`
(built with `lib.rs` at `41517fc35`) becomes
`726f429104606359f9eddaac0001b3a01936501287411d8e2329cc16dddbe4d7`. Per `docs/RELEASE.md` no
per-change re-pin; the pin file is untouched.

### Attempt 1 verdict (verifier): FAIL

MAJOR-1: a D11 ramp the effect runs itself overshoots a domain edge (the step is rounded once,
then added up to 63 times), so its own mid-ramp snapshot is refused: mix `40` subnormal units to
`0.0` (a negative subnormal current), mix to `1.0`, drive to `+36 dB`, output to `-24 dB`.
MINOR-1: a subnormal gain step was accepted though the effect never writes one. MINOR-2: a finite
`1e37` input at `+36 dB` leaves `inf` in `X` with finite output. NIT-1: the `X`/`e` doc. NIT-2:
#1278's brief cites a deleted symbol.

### Attempt 2 (implementer)

**MAJOR-1: accept a ramp's own rounding overshoot, and nothing else outside the range.**
`ramp_current_valid` (`crates/soft-clip/src/lib.rs`) keeps every rule of attempt 1 and adds one
allowance: an in-flight ramp's current outside the converted range is accepted when it lies within
`64 * ulp(2 * high)` of the ramp's own line `target - remaining * step`. Targets stay strictly in
range (every stored target came through `convert_parameter`), and a ramp at rest is checked exactly
as before. Derivation (in the function's doc): with `step = fl(fl(t - s) / 64)` and
`k = 64 - remaining` additions, `current = t - remaining * step + a + 64b + c` with `|a|`, each of
the `k <= 63` addition roundings in `c` at most half an ulp of `2 * high`, and `|64b| <= 2^-144`;
so the current is within `32 * ulp(2 * high) + 2^-144` of the line, whatever the start (an earlier
overshoot included). A chained-retarget simulation (200k starts per range, five retargets each,
near-edge biased; scratch only) measured at most 7.9 ulp(2) for the mix and 15.75 ulp(2 * high)
for the gains. Why not the verifier's "range widened by `64 * (|step| + ulp)`": with the drive ramp
of the fixture in flight that widening is about `6.0`, so it would admit a subnormal drive current
(`bad(0, 1)`), weakening hostile validation; the line check keeps that row refused. No rendered
bit moves: only the control-plane decode changed; the kernel, ramp and snapshot are untouched.

**MINOR-1:** `ramp_step_valid`: the mix step must be finite, a gain step zero or normal (a gain
step is at least `ulp(gain(-24 dB)) / 64`, about `1.2e-10`). **NIT-1:** the `decode_lane_words`
doc now states the real set: `lane::flush` zeroes magnitudes below `FLUSH_EPS = 1e-20`, so the
kernel writes zeros and magnitudes of at least `1e-20`; the check stays the looser zero-or-normal.
**MINOR-2** (`inf` in the `X` history after a finite `1e37` input at `+36 dB`, output finite, so D7
does not fire and the effect's own snapshot is refused for up to 31 samples) is out of #1071's
subnormal scope and is **handed to #1278**: its carry must either make D7 also catch a non-finite
history word or define what it does with a refused own snapshot. NIT-2 belongs to #1278's brief.

Tests:

- `state_roundtrip.rs`: four deterministic cases, one per verifier row, each asserting the
  snapshot's current really is past the edge (negative subnormal mix; mix above `1.0`; drive gain
  above `gain(+36 dB)`; output gain below `gain(-24 dB)`), then restore into a fresh instance,
  snapshot identity and 128 continued samples bit for bit. The rejection test's subnormal-step row
  moved to word 6 (the left output's ramp at rest), because on the in-flight drive ramp (word 2) a
  subnormal step is already refused by the line check, so that row could not see MINOR-1.
- `randomized.rs`: new seeded `a_restored_near_edge_ramp_continues_bit_for_bit` (256 seeds per
  PR, about 1.3 s in debug; scales with `MISO_ENGINE_RANDOMIZED_SCALE`): a random parameter ramped
  from 1-40 dB ulps (1-256 ulps for the mix) inside a random edge to that edge, other parameters
  drawn in domain, snapshot 1-63 frames in (three quarters in the last 32), restored into a fresh
  scalar instance and into a random lane of a bank at the build's native width (eight lanes on
  AVX2, four on NEON and Wasm, so CI covers both), and 96 continued samples compared by bits. It asserts every one of the six edges was
  crossed by some snapshot (per PR: drive bottom 6, top 5; output bottom 7, top 6; mix bottom 5,
  top 3). The shared harness's `craft` hook cannot reach this: a crafted payload's acceptance is not
  asserted, and its own-snapshot restores seldom land inside a ramp's last samples. A craft writing
  near-edge ramps was tried and stayed green under the strict-current mutation at 1x, 3x, 100x and
  400x seeds against the committed harness, so it was not kept. #1278's harness work adds its own
  edge-ramp check harness-wide.

Mutation evidence (each applied in an export of HEAD plus these files, run, reverted; green
after revert):

| # | Mutation | Red | Green |
|---|---|---|---|
| MA | in-flight current back to the strict range (attempt 1) | all four new `state_roundtrip` cases; `a_restored_near_edge_ramp_continues_bit_for_bit` (seed 48: mix to `1.0`, 54 frames) | rest |
| MC | in-flight current: any finite (no line check) | rejection test, at its first in-flight row `bad(0, 1e6)` (the assertion stops there) | the rest |
| MD | gain step: any finite (MINOR-1 reverted) | rejection test, `bad(6, 1)` | the rest |
| ME | tolerance `1 * ulp(2 * high)` instead of `64` | mix-to-one and drive-to-top cases; the seeded differential | the rest |

Not covered by a test in attempt 2: a sign error in the line (`t + remaining * step`). The
attempt-2 claim that "no self-produced state tells the two apart" was **false** (review MINOR-1):
a ramp that starts from an overshoot and is retargeted inward is still past the edge with
`remaining = 63`, and there `remaining * step` is several times the tolerance. The review
follow-up below adds that test.

Gates: run in an export of HEAD plus this attempt's files, because the shared worktree holds
#1278's uncommitted, mid-edit `crates/conformance/src/randomized.rs` (it does not compile at the
time of this attempt, so workspace gates in the worktree fail on that file, not on this slice).

| Gate (export) | Result |
|---|---|
| `cargo fmt --all -- --check` | 0 |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | 0 (and `-p soft-clip` again after the last test edit: 0) |
| `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | 0 |
| `check-/test-workspace-policy`, `check-/test-realtime-policy`, `check-capi-abi` | all 0 |
| `audit capi` | 0 allocations, 0 deallocations, 0 locks, 0 syscalls, 0 violations |
| `cargo test --locked -p soft-clip -p conformance` | green |
| `cargo test --locked --release -p console-workload` | green: console digests unchanged |
| `cargo run -p conformance --example conformance_fixtures -- --check` | 0: conformance fixtures unchanged |
| `check-cross-targets.sh` | PASS (#1018 expected iOS rows only; soft-clip `memset_pattern16` 22, at its ceiling). Its first run caught `BankWidth::Eight` in the new test on four-lane targets; the test now uses the native width, as the crate's other bank tests do |
| Worklet chain: `build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh --without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts`, `test-web-audioworklet.sh` | all 0 |

**ARTIFACT CHANGED**: the shipped module is now
`94c443c08f93ac6ee5b5f1fe14b7d488d3c92d6a35af2bd2efc4d17e154240ad` (attempt 1 built
`726f4291...`, the parent `30d075d3...`); the restore decode compiled into it changed. Per
`docs/RELEASE.md` there is no per-change re-pin, so the pin file is left alone.

### Attempt 2 verdict (verifier): PASS with two MINORs and two NITs

The bound in `ramp_current_valid` was re-derived and confirmed (current within
`32 * ulp(2h) + 2^-144` of the ramp's line, independent of the start); a 40,000-seed chained
retarget probe restored all 1,339,910 of the effect's own snapshots (max `|current - line|` 15.75
ulp(2h) for the gains, 7.875 ulp(2) for the mix). All gates green; **ARTIFACT CHANGED** to
`94c443c0...` confirmed. Findings: MINOR-1, the line's `remaining * step` term was untested
(mutations `line = t + remaining * step` and `line = t` left every committed test green) and the
attempt record wrongly said no self-produced state could test it. MINOR-2, the two new hostile
exclusions (`remaining == 0`, `-0.0` in flight) had no rejection rows. NIT-1, a crafted accepted
payload can evolve into an own snapshot the restore refuses; the doc should say the accepted set
is not closed under render. NIT-2 (for #1278's brief, outside this slice).

### Review follow-ups (after the attempt 2 PASS)

- **MINOR-1:** `state_roundtrip.rs` gains
  `a_drive_overshoot_retargeted_inward_restores_and_continues`: the drive ramps to `+36 dB` over 63
  frames from the start (of the 40 decibel ulps below the top) that ends furthest above it; a
  1-frame block retargets it 50 decibel ulps inward. The test asserts `remaining == 63`, an inward
  step and a current still above `gain(+36 dB)`, then restores into a fresh instance (snapshot
  identity) and continues 128 samples bit for bit. The false sentence above is corrected.
- **MINOR-2:** the rejection test gains two rows, each with its own fixture: an output prepared at
  `+24 dB`, at rest, with its current set to `gain(+24 dB).next_up()`; and a mix ramp from 40
  subnormal units to `0.0`, 48 frames in, with its current set to `-0.0`. Both must be refused
  with `effect.state.parameter`.
- **NIT-1:** `ramp_current_valid`'s doc says the accepted set is not closed under render once a
  crafted word is in (the effect's own later snapshot of a crafted near-tolerance ramp may be
  refused; only a crafted restore reaches it, and the plan-swap carry moves only self-produced
  states), and `decode_lane_words`'s rule statement is narrowed to lanes that only held their own
  words.
- NIT-2 belongs to #1278 and is handled there.

Mutation evidence (each applied to `crates/soft-clip/src/lib.rs`, the whole `cargo test -p
soft-clip` suite run, the file restored and compared byte for byte):

| # | Mutation | Red |
|---|---|---|
| MS | `line = target + remaining * step` | `a_drive_overshoot_retargeted_inward_restores_and_continues` (own snapshot refused, `effect.state.parameter`) |
| ML | `line = target` | `a_drive_overshoot_retargeted_inward_restores_and_continues` |
| MR | `remaining == 0 ||` dropped from the in-flight guard | rejection test, row "an output at rest one ulp above its top" |
| MZ | `is_negative_zero(current) ||` dropped from the in-flight guard | rejection test, row "an in-flight mix current of -0.0" |

Every other test stayed green under each mutation, and the suite is green after each restore.
