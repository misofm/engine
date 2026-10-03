# #1219 *Ramp a send's coefficients with the indexed ramp kernel*: Sol verdict, attempt 2

- **Reviewed:** `git diff cdde008f2 0268a1c74` (4 files, +66/-13), judged with attempt 1
  (`1b5034c35..cdde008f2`). Branch `codex/batch-submix-k3`, worktree `/home/bl/misofm/wt-submix-k3`.
- **Binding documents:** `AGENTS.md`; the spec with its Attempt 1 and Attempt 2 records; the
  attempt 1 verdict (`1219-attempt1.md`); DESIGN P5; VERIFY-2.
- **How I ran it:** I did not modify the worktree, the branch or GitHub. I exported `0268a1c74`
  with `git archive` to `/tmp/claude-1002/v1219b/src`, with its own target directory, and ran every
  gate there. Scratch tests and mutations ran in a second copy. The probe is a standalone `cdylib`
  that calls `route_mix_ramp_block::<Simd4>`. It was built for `wasm32-unknown-unknown` with
  `+simd128` and the workspace release profile (fat LTO, one codegen unit, `panic = "abort"`). My
  scratch tests, mutation driver and probe source are saved beside this file as
  `1219-attempt2-verifier-scratch.rs`. Everything under `/tmp/claude-1002/v1219b/` was deleted
  afterwards.

## Verdict: PASS

There are no BLOCKER or MAJOR findings.

- **MAJOR-1 is resolved.** The probe is trap-free, the kernel shape is unchanged, M12 and M13 are
  red on the probe, and no bit moved.
- **NIT-3's deletion is justified.**
- **MINOR-A, required before close.** The new MINOR-1 doc text is still not correct. The bound
  `(length - 1) * 2^-150, under 2^-128` is exceeded, and I constructed two counterexamples.
  - The wrong figure came from the attempt 1 verdict's own suggested wording, so this is the
    verifier's error, not the implementer's.
  - The magnitude is at most `2^-128`, about -770 dBFS. Nothing audible or behavioural changes.
  - The fix is a comment-only change.

  Root applies it in the verdict follow-up commit, as K2 did, with the text below. #1220's
  uncommitted copy of the claim needs the same correction.

## Findings

### MINOR-A. The subnormal overshoot bound in the `INDEXED_RAMP_LENGTH_MAXIMUM` doc is false (it was MINOR-1's fix)

**The text** (`crates/lane/src/kernels.rs`, the doc on `INDEXED_RAMP_LENGTH_MAXIMUM`): "`c(k)` can
pass the target by at most `(length - 1) * 2^-150`, under `2^-128` absolute: inaudible,
monotonicity still holds".

**Why it is wrong.**

- Write `D = target - start`, exact. Then `round(k * step)` exceeds `D` by less than
  `k * 2^-150 <= (length - 1) * 2^-150`. That is the step's absolute error, accumulated; the
  product's own rounding is absorbed by the `(length - k) / length * D` headroom.
- That excess is not yet the overshoot. The final sum `round(start + p)` rounds onto the
  **target's** grid. When the gap above `target` is wider than `2^-149`, an excess of at least half
  that gap rounds up to a whole gap.
- So the overshoot can reach almost twice the excess: `round(x) - target <= 2 * (x - target)`,
  because `target` is a float.
- **Correct bounds:**
  - below `(length - 1) * 2^-149`;
  - at most `2^-128` absolute, because the excess is below `2^-128` for `length <= 2^22`;
  - the `2^-128` is reachable, so the text's "under `2^-128`" is false too.

**Counterexamples.** Both were run against the real `IndexedRamp::new` / `coefficients_at`, and
both are in the scratch file.

| `length` | `start` bits | `target` bits | `step` | Result | Doc bound `(length - 1) * 2^-150` |
|---|---|---|---|---|---|
| 7 | `0x01800000` (`2^-124`) | `0x01800001` (`+2^-147`) | `2^-149` | `c(6) = 0x01800002`, overshoot `2^-147` = **8** units of `2^-150` | 6 units |
| `2^22` | `0x0b000061` | `0x0b000064` (`D = 3 * 2^-128`) | `2^-148` | from `k = 3,670,017`, overshoot exactly **`2^-128`** (4,194,304 units) | 4,194,303 units, and not "under `2^-128`" |

**Random search.** I ran 3,784 subnormal-step ramps (targets with exponent field 0-30, lengths from
2 to `2^22`).

- Two exceeded `(length - 1) * 2^-150`; both sit at `length` near `2^22`.
- The maximum absolute overshoot was exactly `2^-128`.
- None was non-monotone below the snap.

**"Monotonicity still holds"** is true only for `k < length`. Whenever there is an overshoot,
`c(length - 1) > target = c(length)`, so the snap steps back by the overshoot.

**Replacement text** for the two sentences after "That bound assumes relative rounding." I verified
it against the derivation and both counterexamples:

```rust
/// That bound assumes relative rounding. When `|target - start| < length * 2^-126` the step is
/// subnormal (or exactly `2^-126`) and its rounding error is absolute, up to `2^-150`: `k * step`
/// can then exceed `target - start` by less than `(length - 1) * 2^-150`, and the sum's rounding
/// onto the target's grid can double that, so `c(k)` can pass the target by less than
/// `(length - 1) * 2^-149` and never by more than `2^-128` (reached at `length = 2^22`). That is
/// inaudible; `c(k)` stays monotone for `k < length`, and the snap assigns `target` exactly,
/// stepping back by the overshoot. A caller refuses a longer length before a ramp exists.
```

**Elsewhere:**

- The spec's Attempt 2 record repeats the old figure. Amend it when the follow-up is recorded.
- **#1220 (not this issue's path):** the uncommitted `docs/BUILTINS_AND_METERING_V1.md:236-238`
  in the shared worktree already carries "at most `(length - 1) * 2^-150`, which is below `2^-128`
  ... It stays monotone". #1220's verifier should require the same correction.
- The normal-step claim, no overshoot while `(length - 1) * 3u < 1`, is correct. In that regime
  `round(k * step) <= D`, and rounding is monotone onto a representable target, so the sum cannot
  pass it.

### NIT-A. Nothing in CI sees M12 or M13 until #1220 links the kernel

This is by design: the spec gives the artifact gate to #1220. After #1220 lands,
`check-web-audioworklet.sh --callgraph miso_engine_web_v1_render` (no `--trap-owner`) will refuse
either regression. Until then, the probe evidence and the `MUTATIONS.md` rows are the record. No
action is needed.

## MAJOR-1: verified resolved

**Probe at `0268a1c74`:**

```
probe_ramp: closure=4 traps=0 trap_owners=[] entries=[]                      (exit 0)
unequal_probe: closure=4 traps=0 trap_owners=[] entries=[]                   (exit 0; planes of independent lengths)
vector=22 scalar=0 _RINv...4lane7kernels20route_mix_ramp_block...4wide6f32x4_5f32x4E...   (gate's kernel_arithmetic)
  route_mix_ramp_tail     vector=0 scalar=42   (outlined, non-generic: no 4wide6f32x4 name)
  route_mix_settled_tail  vector=0 scalar=18   (outlined, non-generic)
```

- **Mutations, each applied alone:**
  - **M12** (the kernel's cut restored to `let count = left.len(); let right = &mut right[..count];`)
    is RED on the probe: `closure=5 traps=3`, owner `route_mix_ramp_block<f32x4>`, entry
    `slice_index_fail`. Native `route_ramp` stays green (4/4).
  - **M13** (the settled tail passes uncut planes) is RED on the probe: `closure=5 traps=3`, owner
    `route_mix_settled_tail`. Native stays green.
  - Both match the `MUTATIONS.md` rows exactly.
- **The kernel shape is unchanged** under every cut variant: 22 vector, 0 scalar.
- **Release behaviour with unequal planes.** I tested lengths (130, 125), (125, 130), (7, 3),
  (3, 7), (0, 5) and (5, 0), at positions 0, 30 and 37, at every width. The kernel processes the
  common prefix bit-identically to `f32` and leaves both remainders untouched. It does not panic.
  The `debug_assert_eq!` still rejects this in debug builds.
- **The record's claim that no other index can fail is consistent with the probe**: zero traps,
  with `split_at_mut` at `min(count, ..)` and zipped iteration elsewhere.
- **The comments on both cuts are accurate.**
- **The shipped module is unchanged**: `1900ef1b...0e6`. The kernel is not linked until #1220, and
  the shipped render closure is still `closure=8 traps=5 trap_owners=[render_inner]`.

## Bits unchanged: my own frame-by-frame law check

- **Method.** The kernel ran over whole ramps at `f32`, `Simd4` and `Simd8` (AVX2). Blocks were
  128, 125 and 3 frames, with the caller's saturating advance, until two settled blocks.
  - Every output frame was compared, bit for bit, against an independent oracle built from
    `(start, target, length)` alone, with explicit `f32` roundings and then the D3 mix.
  - Every `coefficients_at(k)` was compared with the oracle's `c(k)`.
- **Lengths:** `2^22`, `2^22 - 1`, `2^22 - 3`, 4,000,037, 48,000, 4,801 and 5, with two
  coefficient sets (including `-0.0` and `1e-30` targets).
- **Result: 0 mismatches** in every combination.
- The spec's `route_ramp` tests are 4/4 in debug (test-debug-b) and green on every
  non-value-changing mutation above.

## NIT-3: deletion justified

The deleted lines were `position = ramp.length.min(position + frames as u32); assert_eq!(position,
ramp.length)`, with `position` starting at `ramp.length`. That is `min(length + frames, length) ==
length`, which can never fail.

- Gate 3 still renders three blocks at `position == length` per block size, against
  `mix2x2_block`.
- The spec's gate 3 wording now correctly puts saturation with #1220.

## My mutations

| # | Mutation | Native `route_ramp` (debug) | Probe `--callgraph` | Result |
|---|---|---|---|---|
| S1 | the kernel's cut uses `max(left.len(), right.len())` | green | RED: `closure=5 traps=4`, owner `route_mix_ramp_block<f32x4>` | RED (probe) |
| S2 | the settled tail cuts only `right`, to `left.len()` | green | RED: `closure=5 traps=3`, owner `route_mix_settled_tail` | RED (probe) |
| S3 | the settled tail cuts to `min(..) - 1`, so it drops its last frame | RED, all 4 tests (`width 4, 125 frames at position 0: left frame 124`) | green (`traps=0`) | RED (native) |
| S4 | the kernel's cut swaps the planes (`(&mut right[..count], &mut left[..count])`) | RED, all 4 tests (`width 1, case 0, length 0, ... left frame 0`) | green | RED (native) |

Together with M12 and M13, every cut-related regression is red on exactly one of the two gates.

## Gates (head `0268a1c74`, x86-64-v3 with AVX2, AMD EPYC 7313P, rustc 1.97.1)

| Gate | Result |
|---|---|
| test-debug-b (DESIGN 7) | exit 0; 146 `test result: ok`; `route_ramp` 4/4 |
| `check-lane-policy.sh` / `test-lane-policy.sh` | ok / ok, exit 0 |
| `check-unfused-seal.sh` / `--self-test` | ok (8 registered audit calls) / 57 passed, 0 failed |
| `cargo fmt --all -- --check` | exit 0 |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | exit 0 |
| `build-web-audioworklet.sh --named-twin` | exit 0; module `1900ef1b...0e6` (unchanged; named twin `6749f73e...`) |
| `check-web-audioworklet.sh` (full, with metadata regeneration) | exit 0; render `closure=8 traps=5 trap_owners=[render_inner]`; `f32x4_arith=9350 kernels=12` |
| `check-browser-expected-resources.py --artifacts` | exit 0 |
| `test-web-audioworklet.sh` | exit 0 (its FAIL lines are the self-tests' expected reds) |
| wasm probe `--callgraph probe_ramp` | `closure=4 traps=0`; `f32x4` instance 22/0 |
| `run-aarch64-tests.sh debug` | **not run**: no arm64 host or qemu. It runs in CI `aarch64-debug` at the K3 push. Attempt 2 adds no arithmetic, only two `min` cuts |

## Test value

Attempt 2 adds no test. It deletes one tautological assertion (NIT-3), which needs no answer.

The four attempt 1 tests keep their attempt 1 verdicts (PASS). Each catches the defects named
there, and they are re-run green here.

M12 and M13 are evidence rows, not committed tests. Their committed guard is #1220's artifact
`--callgraph` gate.

## Required follow-up before #1219 closes

1. Replace the two subnormal sentences in the `INDEXED_RAMP_LENGTH_MAXIMUM` doc with the MINOR-A
   text.
2. Amend the Attempt 2 record's MINOR-1 bullet to match.
3. Pass the same correction to #1220's `docs/BUILTINS_AND_METERING_V1.md` "Live routes" paragraph.

Nothing else in attempt 2 needs a change.
