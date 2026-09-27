# EQ: write a target's lane with a vector select, and snap ended lanes by mask

Source: `docs/handoffs/effects-2026-09-27/AUTOMATION-DIAGNOSIS.md`, verified in `VERIFY-AUTOMATION.md`. **The Amendments section at the end supersedes the body wherever they conflict.** Draft names map to issues: automation-5 = #1003, automation-1 = #1004, automation-2 = #1005, automation-3 = #1006, automation-4 = #1007.

Automation follow-up E2 (research 2026-09-27, base `codex/batch-plumbing-floor-2` at `49f696c7`;
every `file:line` is `crates/parametric-eq/src/lib.rs` on that tree). Evidence:
`docs/handoffs/effects-2026-09-27/AUTOMATION-DIAGNOSIS.md`, sections 3 ("Target application")
and 5 (E2). The prototype is bit 1 of `parametric_eq::diag::MODE` in
`docs/handoffs/effects-2026-09-27/automation-diagnosis-prototypes.patch`. It is evidence only; do
not commit it.

## Product outcome

Every automated EQ lane receives a prepared target every block. The render thread applies it
(`crates/rack/src/lib.rs:1270` → `apply_target_lane`, `:2148` → `start_ramp`, `:1247`), and
64 samples later it snaps it (`process_section`, `:1376` → `snap`, `:1329`).

Both write one lane of a lane vector with `lane_set` (`:932`): a full-width store, a 4-byte write,
and a full-width reload that the store buffer cannot forward. That is 12 per retarget, 12 more
per lane at the ramp's end, and 18 per stationary hoist (`settle`, `:1157`). It costs about
0.2 µs per target natively and under V8. That is what separates automating every track from
automating one per bank: +22 µs natively for 112 more targets.

This slice writes lanes with `select(one_hot(lane), splat(word), vector)` from a static one-hot
table, and snaps every lane that ended in a segment with one masked `select` per word. Measured on
top of E1, in µs per 64-track block, EQ isolate, Δ against settled:

| arm | native `Simd8` E1 / E1+E2 | native `Simd4` E1 / E1+E2 | V8 E1 / E1+E2 |
|---|---:|---:|---:|
| all 64 | +29.8 / **+18.0** | +33.7 / **+20.2** | +38.7 / **+23.9** |
| 8 of 64 | +8.9 / **+7.5** | +8.7 / **+6.3** | +11.2 / **+9.1** |
| one lane | +1.8 / **+1.6** | +1.5 / **+0.5** | +2.9 / **+2.3** |

Without E1, E2 alone takes `Simd8` all 64 from +88.7 to +76.6. These figures are descriptive
only.

## Invariants

* **Class A.** `select` is bitwise, so every word stored is exactly the word `lane_set` stores,
  `+0.0` steps included. Every output word, coefficient, step, target, `remaining`, identity flag
  and payload is unchanged, at `f32`, `Simd4` and `Simd8`, dual and collapsed.
* The snap of the ended lanes of one segment happens at the same point as today: after the
  segment's kernel, before the next segment. `refresh_identity` runs once per section when any
  lane snapped, as today.
* `lane_get` stays. A narrow load from a wide store forwards.
* Allocation-free. The one-hot table is a `static [[f32; 8]; 8]`. No `unsafe`.

## Interface contract

1. `static ONE_HOT`, `fn lane_mask<L>(lane) -> L::Mask` (a load of the static row, then
   `eq(1.0)`), and `fn lane_put<L>(value, mask, word)`.
2. `settle` and `start_ramp` use `lane_put` for every coefficient, target and step word.
   `start_ramp` stays `#[inline(never)]`.
3. `process_section` builds the ended mask as the `mask_or` of `lane_mask(track)` over the lanes
   that were ramping and reached `remaining == 0` in this segment. For each of the six words it
   then sets `coef = select(ended, target, coef)` and `step = select(ended, 0, step)`.
4. `snap` remains for `restore_track` and any other caller.

## Smallest closable slice

Authorized paths:

* `crates/parametric-eq/src/lib.rs`;
* `crates/parametric-eq/tests/` (extend `automation-2`'s differential file);
* `crates/parametric-eq/tests/MUTATIONS.md`;
* this spec.

## Non-goals

* The rest of the per-target cost: queue pop, decode, `refresh_identity`'s six scans, the
  permitted check. That is about 95 ns per target natively after this change. Skipping
  `refresh_identity` in `start_ramp` is exact (the coefficients do not move there), but it breaks
  a maintenance rule the code keeps deliberately. It is not in this slice.
* E1 and E3.

## Objective gates

1. **Identity.** `automation-2`'s bank differential, run with this change. The prototype ran it
   as modes 2 and 3 at 300 scenarios × 96 blocks per width in release and 40 in dev. Compare
   every output word and payload by bits after every block. Add a unit test that drives
   `start_ramp`, `settle` and a segment snap on every lane of `Simd4` and `Simd8` with words that
   include `+0.0`, subnormals and `f32::MAX`, and compares each vector word by bits against the
   `lane_set` path.
2. **Scenario pinned on base.** `automation-2`'s scenario digest, recorded on `49f696c7`,
   unchanged.
3. **Existing gates unchanged:**
   * the crate's suite in dev and release, including the #144 hoist tests (`restated` must stay
     bit-identical to `quiet`);
   * `console_hoist`'s in-run assertions;
   * every console digest.
4. **Mutations**, each recorded red:
   * one-hot row off by one lane;
   * the snap mask built from `was_ramping` alone, without `remaining == 0`;
   * the snap step set to the target instead of `+0.0`;
   * `settle` without the target word.
5. **Realtime and wasm.** `check-web-audioworklet.sh`: roster unchanged, with no scalar `f32`
   arithmetic added to either EQ `process_bank`. The callgraph closure gains no trap owner (the
   one-hot index is bounded by `W`: use `get` or a `min`, never a panicking index).
6. **No regression through the shipped artifact.** A paired V8 run of `web_auto.mjs eq_gain all`,
   base against change, 6 rounds.
   * The `settled` isolate must not be slower than base by more than 2 %.
   * Record `all_64`. This is descriptive.

## Dependencies

After `automation-2`, which edits the same `process_section`.

## Amendments (Sol verification, 2026-09-27)

Evidence: `docs/handoffs/effects-2026-09-27/VERIFY-AUTOMATION.md` and
`verify-automation-raw-timings.txt`.

The slice stands.

* **Reproduced on top of E1**, V8 EQ Δ: 8 of 64 goes +11.4 → **+9.85 µs**, and all 64 goes +38.4 →
  **+24.0 µs**. Natively at `Simd8`, all 64 goes +30.6 → +18.8 µs.
* **Differential:** the bank differential passed at 300 × 96 per width for E2 alone and for
  E1 + E2.
* **Wasm:** the roster is unchanged, and the artifact is 2,762 bytes smaller than E1 alone.

### A1. Unit test (gate 1)

* Add a **`Both`** target, which writes both channels. That is the SDK's shape for a symmetric
  both-channel edit.
* Add the last lane, `W - 1`, of `Simd4` and of `Simd8`.
* `lane_mask` must not be able to panic. Use `ONE_HOT.get(lane)` with a `debug_assert!` and a
  declining fallback, never `ONE_HOT[lane]` as the prototype does.

### A2. Differential

* Run `automation-2`'s amended differential for E2 alone and for E1 + E2: `Both` targets, resets
  mid-ramp and boundary targets.
* The bank output is read after the §4.4 check, so compare it strictly by bits.

### A3. Gate 6, made self-contained, with a mono row

* Use the recipe of `automation-2` amendment A5, with subjects `eq_gain,mono_eq` and arms
  `settled,eight_of_64,all_64`.
* **No regression.** The `settled` isolate must not be more than 2 % slower on either subject.
* **Descriptive.** Record `all_64`.
* **Identity.** `DIGEST=150` must print "all identical".
