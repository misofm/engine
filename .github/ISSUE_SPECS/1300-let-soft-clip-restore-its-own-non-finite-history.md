# Let soft-clip restore its own non-finite history

Stream A of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-7, D15-8).
Code anchors verified on `main` at `6fb211594`: no file this spec cites changed since `d2fe0555a`.
Stream A order: this issue, then the effect-lane carries (#1279-#1282). Those carries restore
soft-clip lanes through the same payload calls (D15-7), so an own-snapshot refusal would leave a
carried lane at rest.

Successor item of *Make every banked effect's state restore allocation-free* (#1278, closed). Its
attempt-1 amendment and attempts 2 and 3 left it open as "soft-clip's two open non-finite history
cases". It started as #1071 attempt-1 verdict MINOR-2
(`docs/handoffs/seamless-swap-phase1-2026-10-04/1071-attempt1.md:130-141`): D7 keys only on the
output, so a non-finite history word with a finite output is refused by the effect's own restore. The #1278 attempt-1 verdict (item 4) accepted it as a successor. The fix
is control-plane only. No rendered bit moves.

## Problem (verified on `main` at `d2fe0555a`)

The effect contract says a restored payload continues exactly like the instance it was taken from
(`crates/effect-contract/src/lib.rs:1927-1929`). Soft-clip's restore follows #1071's rule: accept
every word the effect itself can hold, and refuse the rest (`crates/soft-clip/src/lib.rs:706`). Two
kinds of state break that rule today. The effect holds non-finite history words at a block
boundary, D7 does not fire, and its own snapshot is refused with `effect.state.history`.

The kernel (`crates/soft-clip/src/kernel.rs`), per frame:

- `:191` pushes the raw input `xin` into the dry history, unflushed;
- `:192` pushes `X = flush(2 * drive * xin)` into the interpolation history;
- `:196` pushes `e = flush(cubic(u))` into the decimation history, where `u` is the interpolated
  `X`;
- `:208-209` outputs the delayed dry sample on the identity path (`bypass`, or `mix == 0` and
  `output == 1`), and `output * ((1 - mix) * dry + mix * wet)` otherwise.

`flush` (`crates/lane/src/lib.rs:186-188`) leaves a NaN or an infinity unchanged. `cubic` clamps
`±inf` to `±2/3` and returns NaN for NaN (`kernel.rs:146-155`). D7 is
`effect_runtime::bank::check_block`, applied to the output only: it refuses a block with a NaN or
any `|y| >= 1e30` (`crates/effect-runtime/src/bank.rs:32`, `:80-95`). Then
`SoftClip::recover_lanes` zeroes the failing lane's histories (`crates/soft-clip/src/lib.rs:973`).

The restore (`decode_lane_words`, `crates/soft-clip/src/lib.rs:733`; the rules are at `:764`)
accepts:

- in `X`: zero, a normal or `±inf` (`x_history_word_valid`, `:788`);
- in `e`: zero or a normal (`normal_or_zero`, `:793`);
- in dry: any finite word (`f32::is_finite`).

Payload words: `X` is 12..43 (31 ages), `e` is 43..73 (30 ages) and dry is 73..104 (31 ages)
(`:62-77`).

The reproducer was run on a scratch copy of `d2fe0555a`. Drive is `+36 dB` on both channels, and
128 samples of `0.5 * sin(0.1 i)` are fed on both channels. Planted words are on the left only. The
snapshot is taken at the block boundary:

| Row | Output, mix | Left input | D7 fired | Non-finite words in the snapshot | Own restore |
|---|---|---|---|---|---|
| case 1 | 0 dB, 0 (identity) | `1e37` at 118, `-1e37` at 120 | no | `X` `±inf`; `e` NaN (`0xffc00000` on x86-64) | `effect.state.history` |
| case 2, inf | 0 dB, 0 / 0 dB, 1 / -6 dB, 0.5 | `+inf` at 120 | no | `X` `+inf` (word 19); dry `+inf` (word 80) | `effect.state.history` |
| case 2, NaN | 0 dB, 0 (identity) | NaN at 120 | no | `X`, `e` and dry NaN | `effect.state.history` |

- Case 1: two opposite infinities in the `X` window give `inf - inf`, so `u` and `e` are NaN. The
  identity output is the dry sample, so it stays finite. Off the identity path, the same input
  makes the output NaN and D7 resets the lane in the same block.
- Case 2: a non-finite input in a block's last 31 samples reaches the output only after the
  31-sample dry delay. So at the block boundary the dry history (and `X`) hold it, and the output
  is still finite. D7 fires one block later.

**Why every such word is harmless (the soundness argument).** Soft-clip has no recursive state. Its
three histories are FIR delay lines, and the kernel reads no history word older than 31 samples.
So a non-finite history word has two possible outcomes:

- it reaches the output, and D7 zeroes that block and resets the lane;
- it ages out unread within 31 samples (a NaN in `e` on the identity path).

The ramps are validated separately and stay finite.

**Production reachability.** Probably none. The track input stage zeroes any `|x| >= 1e30`
(`sanitize_gain_block`, `crates/lane/src/kernels/builtins.rs:62-99`; `NONFINITE_LIMIT`, `:26`). A
trim of at most `+24 dB` (`crates/builtins-compiler/src/lib.rs:5037`) cannot raise that past about
`1.6e31`. Every upstream effect's output is under `1e30` by D7. Both cases need `>= ~2.7e36` or a
non-finite input. The fix is still owed: the contract applies per effect, and a direct caller or the
conformance differential can reach these states. It is low priority.

**Carry consequence.** The console carry (#1279 D6) defines what happens when a
restore refuses: the lane is left at rest and a carry-refusal counter goes up. That fallback stays
as it is. After this issue, soft-clip never sends its own snapshot down it.

## Decision to make first, and the recommendation

The decision is how soft-clip's own non-finite history words get through a snapshot and restore. It
is an engineering decision. No owner ruling is involved: option C changes no contract text and no
rendered bit, and it is what #1071's rule already requires.

- **(A) Widen D7 to the histories.** At each block boundary, also scan the live rows of `X`, `e` and
  dry, and recover a lane that holds a non-finite word. Rejected:
  - it adds render-path work to every block of every soft-clip lane (about 96 words per lane per
    block, a large share of `check_block`'s cost at quantum 32), for inputs that no ordinary chain
    delivers;
  - it moves rendered bits. A bypassed or identity lane would be muted because its unused wet path
    overflowed, and finite audio before a late non-finite sample would be zeroed a block early.
- **(B) Define the carry's behaviour on a refused own snapshot.** This is already defined: #1279 D6
  leaves the lane at rest and counts it. Leaving it there breaks the contract's exact-continuation
  promise for these states, and it counts as "refusals" states the effect produced itself. Rejected
  as the fix, but kept as the generic fallback.
- **(C) Accept the effect's own non-finite words on restore. Recommended.** Widen
  `decode_lane_words`'s history rules to exactly the words the kernel can hold:
  - `X`: every word except a subnormal. That covers zero, a normal, `±inf` (overflow or an infinite
    input) and NaN (a NaN input). A drive gain is at least `-24 dB`, so `2 * drive * x` is never
    `0 * inf`.
  - `e`: zero, a normal or NaN. Refuse `±inf`: `cubic` never returns an infinity.
  - dry: every word. It holds the raw input, and a host may submit any `f32`.

  This is control-plane decode only. The kernel, the snapshot and every rendered bit stay as they
  are, and the restored lane continues bit for bit, D7 report included. This was checked on the
  scratch copy for all three rows of the table at mix 0, 1 and 0.5, over three continuation blocks.
  A crafted payload gains nothing beyond what the effect's own input can cause: by the soundness
  argument, every non-finite word is D7-recovered or ages out within 31 samples, whatever words
  surround it.

  **The NaN-bits concern in #1278.** That note said NaN payload bits are platform-dependent. A
  restore copies the word bit for bit, so continued and restored agree on any one target. A NaN
  never leaves the effect: D7 zeroes the block that would carry it. No fixture or digest holds
  such a state.

**Stop condition.** Implement C. Stop and record the evidence in this spec, without switching to A
or B, if you find either of these:

- a self-produced non-finite word that survives longer than 31 samples;
- a restored non-finite word that reaches the output without D7 firing.

**NIT-2 is not folded in.** The #1278 attempt-1 verdict's NIT-2 says soft-clip validates an
in-flight ramp current by its line, not by `ramp_path_within`. That is a different defect: it is
crafted-payload hardening of the *parameter* words. It is subtle near domain edges (the #1071
attempt-2 overshoot bound, subnormal mix steps), and it needs its own tests. See Non-goals.

## Deliverables

1. Option C in `decode_lane_words`. Replace the rule array at `crates/soft-clip/src/lib.rs:764` with
   three named predicates (`fn` items, no allocation), each documented with the kernel line that
   produces the words it admits:
   - `x_history_word_valid`: not subnormal;
   - a new `e` predicate: `normal_or_zero(v) || v.is_nan()`;
   - a new dry predicate: always `true`. Write it as an explicit predicate, so the reason is
     documented in one place.
2. Rewrite the doc comment at `:704-731`:
   - the history bullets state the sets above and the soundness argument;
   - delete "A NaN is never accepted, in any history" (`:724-725`) and "the dry history must be
     finite" (`:726`).
3. Tests (gates 1 and 2).

## Authorized paths

- `crates/soft-clip/src/lib.rs`: `decode_lane_words`, its history predicates and their docs only
- `crates/soft-clip/tests/state_roundtrip.rs`
- this spec

## Non-goals

- No change to `kernel.rs`, to D7 (`check_block`, `recover_lanes`), to the snapshot, to the payload
  layout or to `state_layout_version` (stays `1`).
- No change to the ramp-parameter rules (`ramp_current_valid`, `ramp_step_valid`,
  `converted_value_valid`).
- **Soft-clip's in-flight ramp current validated by `ramp_path_within` (#1278 attempt-1 NIT-2).** A
  separate item. It stays open and needs its own issue.
- The #1071 attempt-1 NIT-1 tightening of `X`/`e` to `0 or |x| >= FLUSH_EPS`.
- `ProcessReport`'s per-frame `nonfinite_*_blocks` count in the scalar wrapper (#1073).
- No carry, graph, rack or host change. #1279 D6 stays the generic refusal fallback.
- No extension of the conformance randomized generator to plant non-finite or huge inputs.

## Hazards

- **Do not pin NaN bits.** Tests assert `is_nan()` or `is_infinite()`, never a bit pattern. Case 1's
  NaN is `0xffc00000` on x86-64 and may differ on AArch64 or Wasm.
- **Compare reports by equality.** In the continuation, assert continued and restored reports are
  equal. Do not assert a count of `1`: the scalar wrapper adds `frames`, not one
  (`crates/soft-clip/src/lib.rs:1159-1162`; #1073).
- **Realtime region.** `decode_lane_words` sits inside `REALTIME_POLICY_BEGIN/END`
  (`:656`/`:892`). The predicates must be plain functions or non-capturing closures, with no
  allocation, formatting or panic path.
- **The shipped worklet module changes digest.** The restore decode is compiled into it, as at
  #1071 and #1278. Record **ARTIFACT CHANGED** with both digests. Do not re-pin (`docs/RELEASE.md`).

## Objective gates

1. **Reproducer, red before the fix.** Generalize
   `a_snapshot_holding_an_overflowed_x_word_restores_and_continues_bit_for_bit`
   (`crates/soft-clip/tests/state_roundtrip.rs:346`) into one table-driven test,
   `a_snapshot_holding_non_finite_history_restores_and_continues_bit_for_bit`, and delete the old
   test as superseded. Rows, all at drive `+36 dB`, on a 128-sample block of `signal`:
   - R1 (the old test's row): output `(-6, 3)` dB, mix `(0.5, 1.0)`, `1e37` left at 120, `-1e37`
     right at 118. Expect one `±inf` per channel in `X`.
   - R2 (case 1): output `0 dB`, mix `0`, left `1e37` at 118 and `-1e37` at 120. Expect a NaN in
     `e`.
   - R3 (case 2, inf): output `0 dB`, mix `1`, left `+inf` at 120. Expect `+inf` in dry and in
     `X`.
   - R4 (case 2, NaN): output `0 dB`, mix `0`, left NaN at 120. Expect NaN in `X`, `e` and dry.

   For each row:
   - assert that the snapshot block did not fire D7 and that the expected word classes are present;
   - restore into a fresh instance prepared with different values: `Ok`, and its snapshot equals
     the payload;
   - render three further 128-sample blocks on both instances, and require equal output bits and
     equal `ProcessReport`s per block;
   - require equal snapshots at the end.

   Extend `restores_and_continues` (`:187`) for this, or add a sibling helper.

   **Red evidence (PR, not committed).** On `d2fe0555a`, R2-R4 fail at the restore with
   `effect.state.history`; R1 passes. Record the output.
2. **Rejection test** (`a_restore_rejects_..._every_invalid_word`, `state_roundtrip.rs`).
   - Delete the rows that are now self-produced words: `bad(12, NaN)` (`:521`), `bad(73, NaN)`
     (`:529`) and `bad(103, inf)` (`:530`).
   - Keep `bad(12, 1)`, `bad(43, 1)` and `bad(43, inf)` (`:527`, `:522`, `:533`).
   - Add `bad(43, -inf)`.

   Mutations, each applied, run, then reverted, and recorded:
   - M1: restore the `:764` rules → gate 1 R2-R4 red;
   - M2: `e` admits infinities → `bad(43, inf)` red;
   - M3: `X` admits subnormals → `bad(12, 1)` red;
   - M4: `e` admits subnormals → `bad(43, 1)` red.
3. **No rendered bit moves.**
   - `git diff --stat` touches only the authorized paths;
   - `cargo run --locked -p conformance --example conformance_fixtures -- --check` exits 0, with
     nothing re-pinned;
   - `cargo test --locked --release -p console-workload` passes, with the console digests unchanged.
4. **Allocation-free restore, realtime.**
   - `cargo test --locked -p soft-clip` passes, including `tests/allocation.rs`
     (`the_scalar_render_path_never_allocates` and `the_bank_render_path_never_allocates`, which
     count `restore_*` with `bench_support::alloc` and require `(0, 0)`) and `tests/randomized.rs`
     (every payload call audited, `known: &[]`);
   - `bash scripts/check-realtime-policy.sh` and `bash scripts/test-realtime-policy.sh` pass.
5. **Inherited gates.**
   - `cargo fmt --all -- --check`;
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`;
   - rustdoc with `-D warnings`;
   - `bash scripts/check-workspace-policy.sh` and `bash scripts/test-workspace-policy.sh`;
   - `cargo test --locked -p conformance -p effect-compiler`;
   - `bash scripts/check-cross-targets.sh` (only the #1018 iOS rows may fail);
   - the worklet chain:
     - `bash scripts/build-web-audioworklet.sh --named-twin <twin-dir> <artifacts-dir>`;
     - `bash scripts/check-web-audioworklet.sh --without-metadata-regeneration <artifacts-dir>
       <twin-dir>/miso-engine-v1-audio-worklet.simd128.named.wasm`;
     - `python3 -B scripts/check-browser-expected-resources.py --artifacts <artifacts-dir>`;
     - `bash scripts/test-web-audioworklet.sh`.

*Test value.*

- Gate 1: a restore that refuses a self-produced NaN in `e`, or a non-finite dry or `X` word,
  turns R2-R4 red; no existing test reaches those states. R1 keeps the overflowed-`X` coverage of
  the test it replaces.
- Gate 2 (`bad(43, -inf)`): an `e` rule that refuses only `+inf` (for example
  `v != f32::INFINITY`) turns it red, and the existing `bad(43, inf)` row does not catch that.

## Evidence

- Gate 1's red output on `d2fe0555a` and green after.
- Gate 2's mutation table.
- The worklet module digest before and after (**ARTIFACT CHANGED**).

## Dependencies

- None open. #1071 and #1278 are on `main`.

This issue lands before *Carry console effect lanes across a plan swap* (#1279) and the other effect-lane
carries (#1280-#1282), so no carried soft-clip lane meets an own-snapshot refusal.

## Standing rules for the implementer

- Work only from this body. Read the cited lines first; do not survey the workspace.
- Change nothing outside the authorized paths. No render-path change.
- A test that greps source or prose is refused.
- Commit on its own branch from synchronized `main`.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).
