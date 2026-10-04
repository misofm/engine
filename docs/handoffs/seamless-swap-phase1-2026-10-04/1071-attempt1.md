# #1071 attempt 1 verdict: FAIL

Commit reviewed: `1199b53f9` (parent `41517fc35`), exported from `/home/bl/misofm/wt-swap-fx` to
`/tmp/claude-1002/v1071/attempt1/`. Uncommitted #1278 work in the worktree was ignored. Probes and
logs are in `/tmp/claude-1002/v1071/attempt1/verifier-scratch/`.

The choice to accept these words on restore, rather than flush them on snapshot, is right. The
positive-subnormal half is done well and the tests for it earn their place. **Gate 1 still fails.**
A ramp the effect runs itself writes a mid-ramp `current` that its own restore refuses, and a
negative subnormal mix is one such value (MAJOR-1). #1278's carry needs restore to accept every
snapshot the effect writes, so that dependency is still unmet.

## Gates I re-ran in the export

| Gate | Result |
|---|---|
| `cargo fmt --all -- --check` | 0 |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | 0 |
| `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | 0 |
| `check-workspace-policy.sh`, `test-workspace-policy.sh` | 0, 0 |
| `check-realtime-policy.sh`, `test-realtime-policy.sh` | 0, 0 |
| `check-capi-abi.sh` | 0 |
| `cargo build --release -p audit -p capi && audit capi` | 0 allocations, 0 deallocations, 0 locks, 0 syscalls, 0 violations |
| `check-cross-targets.sh` | PASS (#1018 expected iOS rows only; soft-clip `memset_pattern16` 22, at its ceiling of 22) |
| `cargo test --locked -p soft-clip -p conformance` | all green (soft-clip `allocation` 3/3) |
| `cargo test --locked --release -p console-workload` | green (console-workload does not instantiate soft-clip; the kernel is untouched) |
| `conformance_fixtures --check` | 0 |
| `MISO_ENGINE_RANDOMIZED_SCALE=3 cargo test --release -p soft-clip --test randomized` | green: 24 seeds, 75 restores, 12 refused (hostile) |
| `cargo test -p effect-compiler` | green |
| Worklet chain: `build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh --without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts`, `test-web-audioworklet.sh` | all 0 |

I independently confirmed **ARTIFACT CHANGED**: my build of the shipped module is
`726f429104606359f9eddaac0001b3a01936501287411d8e2329cc16dddbe4d7`, the digest the implementer
reports. Under `docs/RELEASE.md` a change is never re-pinned, so leaving the pin untouched is
correct.

## Mutations I ran (each reverted; the export was byte-identical to the commit afterwards)

| Mutation | Red | Green |
|---|---|---|
| M1: `lib.rs` reverted to parent | `randomized` (Eight, block 9, "lane 0 refused its own snapshot"); new roundtrip test | the rest |
| M6: `step` back to `normal_or_zero` | new roundtrip test only | `randomized` at 8 seeds |
| M7: dry history treated as flushed | new roundtrip test and `randomized` | |
| M8: subnormal clause restored in `converted_value_valid` | new roundtrip test only at 8 seeds; `randomized` red at scale 3 and at scale 100 | |
| M3: `X` history treated as unflushed | rejection test (`bad(12, 1)`) | `randomized` |
| M4: dry finiteness check dropped | rejection test | |
| M4b: dry check refuses only infinities | rejection test, through `bad(73, NaN)` alone (`bad(103, inf)` stays green) | |
| M9: any subnormal accepted regardless of domain | rejection test, through `bad(0, 1)` alone | |

## Test value, one sentence each

- `a_snapshot_holding_subnormal_words_restores_and_continues_bit_for_bit`: a restore that refuses a
  subnormal mix current, target or step, or restores a subnormal dry sample inexactly, turns it red.
  The per-PR 8-seed differential does not reach the mix half (M6 and M8 stay green there).
- `randomized.rs` at `known: &[]` (rewritten): a restore that refuses a self-produced subnormal dry
  sample turns it red (M1, M7). Nightly at 100x also reaches the subnormal mix (M8 red at scale 3 and
  at scale 100).
- `bad(12, 1)`: a restore that stops treating `X` as a flushed history (M3) turns it red. The only
  earlier subnormal-history row covers `e`.
- `bad(0, 1)`: a restore that lets a subnormal skip the domain check (M9) turns it red. The existing
  `bad(0, 1e6)` covers only the upper bound.
- `bad(73, NaN)`: a dry finiteness check that refuses infinities but lets NaN through (M4b) turns it
  red. `bad(103, inf)` does not catch that.
- Deleting the ignored 24-seed twin was acceptable. It differed only in seed count, and because it
  was ignored it never ran per PR or in nightly. Nightly does not pass `--ignored`. The un-narrowed
  differential now runs per PR at 8 seeds and nightly at 800, and per PR the new deterministic test
  covers what 8 seeds miss (M6, M8). The attempt record states this deviation from "un-ignore the
  reproducer". No coverage that used to run was lost.

## Findings

### MAJOR-1: an in-flight ramp's own `current` can leave the converted domain, and its snapshot is still refused

The code is `crates/soft-clip/src/lib.rs:661`, which calls `converted_value_valid(parameter, current)`
(`:260`). Under D11, `step = (target - current) / 64` is rounded once, and `current += step` repeats
63 times before the snap. Once the step is rounded, 63 additions can carry `current` past the
target. If the target sits at a domain edge, `current` leaves the domain. The restore then refuses
the effect's own snapshot with `effect.state.parameter`. I reproduced four cases through the public
API: prepare plus one block-rate point, then snapshot and restore into a fresh instance
(`verifier-scratch/probe_overshoot.rs`, log in `verifier-scratch/logs/probe-overshoot.log`).

| Ramp | After | Snapshot `current` | Restore |
|---|---|---|---|
| mix `f32::from_bits(40)` -> `0.0` | 48 frames (remaining 16) | `-1.1e-44` (`0x80000008`, a **negative subnormal**) | refused |
| mix `1 - 103 ulp` -> `1.0` | 60 frames (remaining 4) | `1.000001` (`0x3f800008`) | refused |
| drive `35.99998 dB` -> `+36 dB` | 60 frames | `63.095825`, above `db_to_gain(36)` = `63.09574` | refused |
| output `-23.999964 dB` -> `-24 dB` | 60 frames | `0.063095555`, below `db_to_gain(-24)` | refused |

The first row is this issue's own subject: a subnormal state the effect produces and then refuses.
Starting from a subnormal mix, the subnormal step rounds up in magnitude: `-40/64` units becomes
`-1` unit. The ramp therefore crosses `0` after 40 additions and holds negative subnormals until the
snap. The implementer's own reasoning anticipated subnormal steps ("a ramp toward or from a
subnormal mix divides a subnormal difference") but missed the overshoot. The other three rows are
the same root cause on normal values. Gate 1 says "identity for every state the effect can
produce", so they fail it too. The doc at `lib.rs:637` ("accept every word the effect itself can
hold") is false as written.

This matters for #1278 and its umbrella. P2 moves a lane copy through `restore_track_state_payload`
at the swap block. A refused restore drops a mid-ramp lane to rest, and that is the audible gap the
umbrella exists to remove.

**Fix.** Keep `target` strictly in its domain, since every target the effect stores came through
`convert_parameter`. Bound `current` by what the ramp can reach. Two options:

- For `remaining == 0`, keep today's domain check. For `remaining > 0`, accept a finite,
  non-`-0.0` `current` inside the converted domain widened by `RAMP_SAMPLES * (|step| + ulp(2 * high))`.
  Derive this bound in the doc: per-addition rounding is at most half an ulp of the running value,
  and step rounding is at most `|step|`.
- Or accept any finite, non-`-0.0` `current` while `remaining > 0`, and argue that this is no weaker
  than today. A hostile finite `step` can already drive an in-flight ramp anywhere, and D7 recovers
  the output.

Add the four cases above to `state_roundtrip.rs` as deterministic snapshot, restore and bit-exact
continuation tests. Give the differential a crafted near-edge start, a subnormal mix ramped to `0`
and a value within 64 ulp of each domain edge ramped to that edge, so the differential reaches the
overshoot as well.

### MINOR-1: a hostile subnormal gain `step` is now accepted, which gate 2 forbids

The code is `crates/soft-clip/src/lib.rs:663`, and the check `!step.is_finite()` applies to all
three parameters. A gain step is `|Δ|/64` with `|Δ| >= ulp(db_to_gain(-24))`, which is about
`1.2e-10`. So a self-produced drive or output step is always normal or `+0.0`. A subnormal one is
hostile. The parent refused it, and gate 2 says hostile words are "validated as before". The effect
is harmless: the step is absorbed exactly by gains of at least `0.063`, so no rendered bit moves.
Still, the slice's own rule says "refuse the rest".

**Fix.** Validate step per parameter: the mix needs only `is_finite()`, the gains need
`normal_or_zero`. Add `bad(2, 1, "effect.state.parameter")`.

### MINOR-2 (pre-existing, out of the subnormal scope; record for #1278): a finite huge input leaves `inf` in `X` with a finite output

With drive at +36 dB and one input sample of `1e37` (finite), `X = flush(2g * x)` becomes `+inf`.
The interpolated `u` is `±inf`, and `cubic(±inf)` is `±2/3`, so `e`, the wet path and the output
stay finite. `check_block` sees nothing, D7 recovers nothing, and for 31 samples the effect's own
snapshot is refused with `effect.state.history` (`verifier-scratch/probe_nonfinite.rs`: non-finite
left word 19, report all zeros). An `inf` input does the same for up to 31 samples before its
delayed dry output trips D7.

The restore is right to refuse non-finite words. The gap is that D7 keys only on the output. This
does not block #1071. #1278 or a successor should either make D7 also catch a non-finite history
word, or define the carry's behaviour when it meets a refused own snapshot.

### NIT-1: the `X`/`e` doc is imprecise

At `lib.rs:645`, "zero or normal, because the kernel flushes both" is not the exact set. `flush`
zeroes anything below `FLUSH_EPS = 1e-20` (`crates/lane/src/lib.rs:177`), so the kernel only ever
produces zero or `|x| >= 1e-20`. The restore still accepts normal words in `(1.2e-38, 1e-20)` that
the kernel never writes. This is pre-existing and harmless. Either state the real set, or tighten
the check to `x == 0 || |x| >= FLUSH_EPS` if the rule is meant to be exact.

### NIT-2: #1278's brief now cites a deleted symbol

`.github/ISSUE_SPECS/1278-*.md` "Context" still cites `SubnormalStateRefusedOnRestore`
(`randomized.rs:79-87`), which this commit removed. That file is outside this slice's paths, so the
#1278 owner should refresh the line.

## Other checks I made

- Render, kernel and allocation behaviour are unchanged: only the control-plane decode changed, and
  it still validates into stack arrays before applying.
- The restore stays all-or-nothing: the rejection test's "untouched" leg passes.
- NaN and infinity are still refused in every word, and the claim that `X`/`e` are flushed in the
  kernel is true (`kernel.rs`, `flush` on `X` and `e`, dry unflushed). Accepting a positive
  subnormal mix, a subnormal mix step or a subnormal dry sample breaks no denormal or NaN rule.
  Each is a value the effect already holds and renders through its parameter API and its unflushed
  dry history.
- Commit hygiene is fine: the commit touches 5 files with exact paths, the attempt record is in the
  slice spec, and the message ends with the co-author trailer.
- No other crate used `Known::SubnormalStateRefusedOnRestore`, and clippy over the whole workspace
  is clean.
