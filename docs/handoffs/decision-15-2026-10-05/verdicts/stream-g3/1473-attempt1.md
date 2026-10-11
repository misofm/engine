PASS

# #1473 attempt 1: adversarial verdict

Commit `7c6cdb1b4` (parent `3c1f4ee23`), branch `codex/d15-stream-g3`, worktree
`/home/bl/misofm/wt-d15-g3`. Exported with `git archive` to `/tmp/claude-1002/v1473/tree` and built
with `CARGO_TARGET_DIR=/tmp/claude-1002/v1473/target`. I did not touch the worktree.

The two cases do what the spec asks, and both pins are scalar-oracle rows. No existing pin moved and
no product code changed. Every gate passes. Every mutant in gates 2 and 3 goes red on the new case
only, at scalar, simd4 and simd8. The one equivalent mutant ("every segment armed") stays green, and
the record's reason for that is true. Under wasm, each new case catches a defect that nothing else
in the wasm leg sees (measured below). One sentence in the record overstates the uniqueness (MINOR-1).
The NaN docs outside `corpus.rs` were not all brought up to date (MINOR-2). No BLOCKER or MAJOR.

## Findings

### MINOR-1: one test-value sentence in the record is false

Spec `:204-208` (Gate 4) says that a `simd128` lowering of `Lane::min`/`max` that swaps operands or
hides a NaN "turns them red under wasm and nothing else in the wasm leg". That is false. The wasm leg
already runs the `Lane::max`/`Lane::min` truth table (`minmax_lowering_mismatches`,
`tools/wasm-gate-corpus/src/lib.rs:629-686`; `tools/wasm-gates/src/main.rs:62-76` fails the leg on any
count above zero), over both signed zeros and NaN payloads. Measured: I changed the wasm arm of
`Lane::min` (`crates/lane/src/wide_impl.rs:354`, `b.fast_min(self)` to `self.fast_min(b)`), built the
guest and ran the wasm leg. Result: `minmax_lowering_mismatches: 36`, exit 1, and case 98 also red at
simd4 (`mut/W_minswap.log`). The new case is a second catch of that defect, not the only one.
`tools/wasm-gates/MUTATIONS.md:149-157` already records the same truth-table catch (count 144 at its
time).

The bullet at `:211` also lists "(or a `ramp_toward` rewrite)". A source rewrite is caught natively
by existing tests (the record's own gate-2 table), so it is not what the case uniquely defends.

What the cases do uniquely defend, measured (see "Test value" below): a wasm-only change to
`ramp_toward`'s composed result, and a wasm-only change to the multiband per-segment dispatch. The
Test value bullets are true as worded ("no other wasm *digest*"), so the tests keep a real value.
Fix in the follow-ups: replace the Gate 4 sentence and the bullet with the measured unique catches
below. Root's standing ruling is that every test-value claim must be true, so this must be fixed
before the spec closes.

### MINOR-2: NaN-rule docs that now say less than the truth, two of them in authorized paths

The corpus now hashes a NaN token for one case. These sentences still say that no NaN reaches a
digest, or that every case is NaN-free:

- In authorized paths, missed by this attempt:
  - `crates/effect-runtime/tests/determinism.rs:1`: "every case is NaN-free". The test below it
    was amended, but the module line was not.
  - `tools/wasm-gate-corpus/src/lib.rs:637`: "rule 2 of this corpus is that no NaN reaches a
    digest". Rule 2 (`:18-22`) now says "No NaN *payload*" and has an exception.
- Outside the authorized paths (root must authorize them for the follow-ups):
  - `tools/wasm-gates/tests/g5_native_corpus.rs:6`: "the corpus carries no NaN into a digest".
    The implementer reported it as an open item. Correct: its `g5_lane_corpus_is_finite` covers
    lane cases only, which stay NaN-free, so only the sentence is wrong, not a behaviour.
  - `crates/effect-runtime/tests/MUTATIONS.md:207`: "D1's corpus is NaN-free".
  - `tools/wasm-gates/MUTATIONS.md:153-157`: "the frozen corpus cannot see this defect" (the
    swapped wasm `max`/`min` lowering). That is no longer true: case 98 now moves under it
    (measured above).

### NIT-1: a doc line of 121 columns

`crates/effect-runtime/src/corpus.rs:330` ("states the clamp's in-range and NaN bits directly. These
pins were produced ..."). The edit did not rewrap it. rustfmt does not wrap comments here, so no gate
catches it. The rest of the file wraps at 100.

### NIT-2: the seed doc says more about the seeds than is shown

`crates/multiband-compressor/src/corpus.rs:300-301` calls each seed "the plan-swap state a long
silence after a quiet low-crossover tail hands over". Seed 0 holds stage `a` at about `3e-16` with
stage `b` at exactly `+0.0`. A render does not leave that state: stage `b`'s input is stage `a`'s
low-pass tap on every frame, so `b` is not zero while `a` holds words of that size. The validator
admits the state, and that is all the case needs (see question 3). Suggested wording: "a validated
plan-swap payload".

## Root's questions

1. **Is the test-value claim true and enough under AGENTS.md?** Partly true, and enough. The claim
   that existing native tests catch every gate-2 and gate-3 mutant is true. I confirmed it for the
   NaN check (R1, R3), for `either_channel_and_either_stage_arm_the_crossover` (D2b), and for
   `each_channel_counts_its_own_input` and `the_silence_counter_is_carried_and_validated` (my extra
   D5). But "only these cases catch it under wasm" is false for a `Lane::min`/`max` lowering
   (MINOR-1). The value that remains, and that AGENTS.md asks for, is a defect in the wasm build only.
   The native suites cannot see it, because they do not run in the guest. The truth table cannot see
   it, because it tests the two operations alone and not their nesting. Before this commit nothing in
   the wasm leg saw it. I measured both:
   - **A wasm-only `ramp_toward` body.** `#[cfg(target_arch = "wasm32")]` returns
     `L::min(L::max(next, low), high)`, and the native body is unchanged. Result: case 98 red under
     wasm at scalar and simd4, truth table 0, every other digest matches
     (`mut/W_wasm_only_body.log`).
   - **A wasm-only arming test that reads the left channel only.** Result: case 110 red under wasm
     at scalar and simd4, nothing else (`mut/W_wasm_only_dispatch.log`).

   The same holds for a code-generation difference in these compositions. On `simd128`, `wide`
   emits `f32x4_pmin`/`pmax`, which LLVM is free to rewrite in the composition, while the x86 path
   uses `minps`/`maxps`. This is the standing value of a G5 case, and G5 is the one cross-target
   corpus where AGENTS.md allows a digest. The spec's own Test value section (`:105-110`) is true
   as worded.

2. **The amended `the_corpus_is_nan_free` and the NaN token.** It does not weaken the guarantee for
   the other cases. The skip applies only when `*name == "ramp_toward" && *word == NAN_TOKEN`. Every
   other case still fails on any NaN, the token value included. For `ramp_toward`, a NaN that is not
   the token fails, and a run with no token fails. The mapping is an integer test on the bits
   (`word & 0x7fff_ffff > 0x7f80_0000`), so no target's float rules decide it. The NaN step's payload
   `0x7fd0_1473` is kept by a native `addss`, so the test bites natively. Measured: with the mapping
   removed, the test is red at `determinism.rs:32` ("is a NaN payload"). G5 case 98 is also red
   natively at all three widths, because the pin hashes the token. With R1 or R3 the test is red at
   `:38` ("never reaches its NaN arm"). The test still has its own value next to the pin. After a
   deliberate re-pin from the scalar oracle, the pin would accept a case that hashes payloads or that
   no longer reaches NaN, and this test would not. The wasm leg passes, with both new cases at scalar
   and simd4 and 0 mismatches, so the token mapping holds where wasm canonicalises.

3. **Seeding through snapshot, patch and restore.** This is a real product door, and it is
   legitimate here. `Instance::restore` (`crates/multiband-compressor/src/lib.rs:1644-1659`) is the
   same function that `restore_state_payload` and `restore_track_state_payload` call (`:1874`,
   `:1939`). These are the plan-swap payload calls of #1278 D3, inside the realtime-policy region. The
   door validates every word: the filter words with `normal_or_zero`, and the counter as `+0.0` or an
   integer in `[1, 2^24]`. Under R6b, in-memory state that is handed across a plan replacement is not
   persisted state. The crate's own native tests seed the same way
   (`the_crossover_joint_flush_arms_after_its_inputs_silence`,
   `either_channel_and_either_stage_arm_the_crossover`, `tests/product.rs:1404`, `:1464`). Arming
   needs 4,096 silent frames, and a 512-frame render does not reach that. The case is a determinism
   stimulus: the dispatch's contract covers every state the validator admits, so reachability by
   render is not needed. The only problem is the doc's description of the seeds (NIT-2). The case
   also mirrors `process_bank` exactly: per-lane `apply_automation`, then `render::<L, W, false>`
   (`lib.rs:1900-1926`).

4. **"Every segment armed" stays green. Is the reason true?** Yes. `process_block` runs the unarmed
   form only when `Side::arms` is false on both sides. Then `silence_armable_holding`
   (`crates/lane/src/lib.rs:290-318`) guarantees that every lane either carries only `+0.0`
   thresholds over the segment, or starts it at rest and stays at rest until any armed frame. So
   `flush_pair(n1, n2, rest)` gives the per-word bits, and `silence_skip_block` leaves each counter
   where `silence_step` would. Forcing `armable = true` is therefore an equivalent mutant. Measured:
   G5 is green, and all of `cargo test --release -p multiband-compressor` is green under it
   (`mut/D4_all_armed.*`).

5. **The stale module doc of `g5_native_corpus.rs`.** Confirmed stale, and outside the authorized
   paths. It is listed in MINOR-2 with the other stale sentences. The implementer was right not to
   edit it. It needs root's authorization for the follow-ups commit.

## Mutation runs (mine, in the export; each reverted and confirmed by file hash)

Each run used `cargo test --locked --release -p wasm-gates --test g5_native_corpus
g5_native_digests_match_pins`. "Only" means that no other G5 case printed.

| mutant | G5 result | extra |
|---|---|---|
| R1 `L::min(L::max(next, low), high)` | case 98 red, scalar/simd4/simd8, only | `the_corpus_is_nan_free` red (`:38`) |
| R2 `L::max(L::min(next, high), low)` | case 98 red, all three, only | |
| R3 NaN hider `select(r == r, r, low)` | case 98 red, all three, only | `the_corpus_is_nan_free` red (`:38`) |
| R4 `L::max(low, L::min(high, next))` (mine) | green | equivalent: also a strict clamp that keeps in-range bits and a NaN |
| token mapping disabled (mine) | case 98 red, all three, only | `the_corpus_is_nan_free` red (`:32`) |
| D1a arming reads the left channel only | case 110 red, all three, only | |
| D1b arming reads the right channel only | case 110 red, all three, only | the scalar digest differs from the simd4/simd8 digest under this mutant: the bank's armed segment then also flushes tracks 0 and 1 |
| D2a `svf_state_held([a])` | case 110 red, all three, only | same width split as D1b |
| D2b `svf_state_held([b])` | case 110 red, all three, only | `either_channel_and_either_stage_arm_the_crossover` red |
| D3 `armable = BYPASS` | case 110 red, all three, only | |
| D4 `armable = true` | green | multiband crate tests all green (equivalent mutant) |
| D5 `silence_skip_block` not called (mine) | case 110 red, all three, only | `each_channel_counts_its_own_input`, `the_silence_counter_is_carried_and_validated` red |
| panic in each of the four `segment!` arms (mine) | each reached by case 110 | all four forms (ramping x armable) run |
| wasm `Lane::min` arm swapped, wasm leg (mine) | case 98 red at simd4 | `minmax_lowering_mismatches: 36`, so the leg was already red |
| wasm-only `ramp_toward` body, wasm leg (mine) | case 98 red at scalar and simd4, only | truth table 0 |
| wasm-only left-channel arming, wasm leg (mine) | case 110 red at scalar and simd4, only | truth table 0 |

Logs: `/tmp/claude-1002/v1473/mut/` (`R.out`, `D.out`, `W_*.log`, per-mutant logs).

## Test value (one sentence per new or rewritten test)

- **`runtime/ramp_toward` (G5 case 98).** A wasm-only change to `ramp_toward`'s composed clamp turns
  this red under wasm, and nothing else does. Such a change can be a target-gated body or a
  code-generation rewrite of the nested `pmin`/`pmax`, and it can break the signed-zero tie, the
  operand order or the NaN pass-through. The native suites do not run in the guest, and the truth
  table checks the operations only alone. Measured: wasm-only swapped body gives case 98 red, truth
  table 0.
- **`multiband/process_block/segment_dispatch` (G5 case 110).** A wasm-only change to which segments
  of `process_block` arm turns this red under wasm, and nothing else does. The same holds for a
  change to the unarmed form's bits, or to `silence_skip_block`'s counter advance on the unarmed
  head. Measured: wasm-only left-channel arming gives case 110 red, truth table 0. No other wasm
  digest reaches `process_block`.
- **`the_corpus_is_nan_free` (amended).** This turns red when a re-pinned `ramp_toward` case hashes
  a NaN payload instead of the token, or no longer reaches its NaN arm. The G5 pin cannot catch
  that, because a deliberate re-pin from the scalar oracle would accept it. Measured: mapping
  disabled gives red at `:32`, and R1 or R3 gives red at `:38`.

## Spec gates and deliverables

- D1 to D4: met. Case 98 has 270 directed triples (5 x 9 x 6) plus a `2^-16` grid. The NaN token is
  defined, and the "No NaN payload" doc is in `corpus.rs`. Case 110 renders through `render` and so
  `process_block`, in `DualMono` and `Maximum`, read back lane-major. The diff touches no product
  file, and the AudioWorklet module that `run-wasm-gates.sh` built is
  `70fd8a43...29deddcc` (3,115,873 B), the digest the record gives for the parent.
- Authorized paths: only the six authorized files, plus the spec's attempt record. The diff appends
  to the record and changes no line of the body.
- Pins: each crate table gains one row: `D1_DIGESTS[9]` and `corpus_digests.in` row 7. The new rows
  equal the native output at all three widths, so they are the scalar oracle. No existing row
  changed. G5 is the single owner of a cross-target corpus, so the digests are allowed.
- Naming: the new items have no version suffix. The new corpus items are private, except
  `NAN_TOKEN`, which the integration test needs. No test greps source or prose. No test was
  superseded.
- Acked-batch question: no queue is involved.

## Gates run

| gate | command | result |
|---|---|---|
| 1 | `cargo test --locked --release -p lane -p math -p wasm-gates --features math/lane` | pass, 128 tests |
| 1 (wasm) | `bash scripts/run-wasm-gates.sh` | pass. Native leg: 146 cases, 370 comparisons. Wasm `simd128` leg: 146 cases, 258 comparisons. 0 mismatches and all three truth tables 0 on both legs. V8 spill gate ok. |
| crates | `cargo test --locked --release -p effect-runtime -p multiband-compressor` | pass, 149 tests |
| 5 | `cargo fmt --all -- --check` | pass |
| 5 | `cargo clippy --locked --workspace --all-targets -- -D warnings` | pass |
| 5 | `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | pass |
| 5 | `bash scripts/check-workspace-policy.sh` | pass |
| extra | `bash scripts/check-realtime-policy.sh` | pass |
| 2, 3, 4 | mutation table above | as the record says, except for MINOR-1 |
