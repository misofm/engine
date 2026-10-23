# Skip silent lanes' work in console banks

## Problem

Console slots always bank (decision 12). A bank's silent fast path is bank-wide:
`block_is_positive_zero` runs over frames x lanes. One active lane therefore keeps its bank-mates
processing. S4 (#1099) measured this at `f7ba70a8` (p50 µs per block, the mean of two rounds):
- `sixty_four_track_console` (all active): 100.51;
- `sixty_four_track_console_sparse` (odd tracks exact zeros): 100.11;
- `sixty_four_track_idle`: 31.16.

With half of every bank silent, the fast path recovers 0.40 of the 69.35 µs that silence removes
when whole banks are silent. That is against the owner's standing priority that no stage processes
silence. Masking a lane inside a vector removes no arithmetic, because the lanes run in lockstep.
The work can fall only if silent lanes stop occupying bank vectors.

## Smallest closable slice

Decide, prototype and measure one mechanism by which a console slot does no vector work for a lane
whose input block is exact positive zero and whose state is at the kernel's exact fixed point.
For example: per block, pack a slot group's active lanes into as few banks as they fill, and
leave the silent lanes untouched.

- Bits: banking may couple lanes' cost, never their bits.
- Latent slots: a silent lane of a latent slot still feeds its latency line (amendment 11).
- Masking: D7 recovery and reports stay masked by active lanes.

Name what the mechanism does at W=4 and at W=8. If it needs a new benchmark row (for example one
active lane per bank), split that row into a qualification issue first.

## Objective gates

1. Class A: a committed randomized differential renders sessions with random per-lane silence
   runs, with the skip and without it, on Simd8 and Simd4. Every lane is bit-identical, with NaNs
   folded.
2. Render allocates nothing: `allocations == 0`, measured on the render thread after warm-up.
3. One native run and one V8 run of the console benchmark. The sparse row is reported against S4's.
   The all-active console row is not more than 2 % slower than S4's.

## Dependencies

- #1744, #1745, #1746, #1747 (see D3), #1765 and #1763 (see D4), and #1694 or a wasm32 run for the Simd4 leg. *Measure the console strip against its baseline* (S4, #1099) is closed as completed.
- Note (root-authorized, 2026-10-06; #1329 Amendment 3 addendum, A2): a lane's state is at the
  kernel's exact fixed point no later than the node's exact-rest bound, `RestSamples` (#1329 D2),
  which holds for every input up to its stated peak. Silence skipping uses `RestSamples`, not
  #1329's `T_rest`: `T_rest = max(T_decay, R(P*))` reaches exact zero only for inputs below the
  flush floor `P*` (above it, it is the `-144 dB` decay bound). #1329 delivers both for the builtin
  input section; each effect's bounds follow in #1372-#1376.

## Decision record

### D1 Design (attempt 1, 2026-10-22)

Mechanism: **swap-to-pack** inside one console chain (`rack::BankChain`, which runs every
console slot of a run over one resident AoSoA block per bank).

- The chain keeps a lane map, strip to (bank, lane), preallocated at prepare (`banks * W`
  entries, never a track limit). It starts as the compiler's order.
- Per block, a member lane is *active* unless its input block is exact `+0.0` on both planes
  and every slot of the chain reports that lane at its exact fixed point. With `A` active
  lanes, the chain runs only the first `K = ceil(A / W)` banks. An active lane in a bank
  `>= K` swaps with a silent member lane in a bank `< K`: the two lanes exchange their full
  per-lane state, coefficients, ramps, control lanes and taps in every slot of the chain.
  Padded lanes never swap, so D7 recovery and reports stay masked by the moved active mask.
- Gather and scatter read the lane map as a pointer list, so the audio copies stay one
  gather and one scatter per run. A skipped bank is not gathered; a strip whose output plane
  is not its input plane gets `frames` words of `+0.0` per channel.
- Bits: lanes run in lockstep with no cross-lane arithmetic (the detector link is L/R inside
  one lane), and a swap moves a lane's whole state, so the bits follow the strip. Shared
  per-bank cursors (limiter `Cursors`, soft-clip `pos`) still advance in a skipped bank, so
  every bank of a slot keeps equal cursors and a swapped lane lands on the same cursor.
- Latent slots (amendment 11): a lane at rest holds `+0.0` in its latency words (the limiter's
  `is_at_silent_rest` already proves `main_ring`); feeding `+0.0` into them is the cursor
  advance alone, which a skipped bank still does. The rack `BypassShunt` line needs the
  same proof per lane, or it keeps its capture.
- W=4 (Wasm, NEON): 64 tracks are 16 banks; the sparse row (odd tracks silent) packs 32
  active lanes into 8 banks. W=8 (AVX2): 8 banks pack into 4. One active lane per bank packs
  `B` banks into `ceil(B / W)`.
- All-active cost: the activity test is the existing `block_is_positive_zero` head (32 words
  per plane, which returns on the first non-zero word) plus a count; with `A` = members there
  is no swap and no other change. A swap costs the lane's state size (the limiter's rings
  dominate) and happens only when a lane's activity changes.

### D2 Blockers found before implementation (attempt 1 stops here)

1. **No console effect has an exact-rest bound.** Every console-eligible effect states
   `RestBound::Unstated` (parametric-eq `src/lib.rs:728`, compressor `:323`, gate-expander
   `:288`, soft-clip `:226`, transient-shaper `:175`, true-peak-limiter `:278`); #1372, #1375
   and #1376 are open. Under the brief's rule (the fixed point uses `RestSamples`; an effect
   without it does not skip), the mechanism skips no lane, so gate 3 can show nothing.
   Open question for root or the owner: does an *observed* exact fixed point satisfy the
   condition? The shipped bank-wide fast paths (#163: parametric-eq, compressor,
   true-peak-limiter) already skip on it (state bits equal before and after a zero block,
   `+0.0` output, stationary ramps), with no `RestSamples`.
2. **No per-lane fixed-point predicate and no render-time lane swap exist.** The three fast
   paths decide per bank; gate-expander, soft-clip and transient-shaper have none. The lane
   carry that exists (`EffectBankStage::carry_lane_from`, rack `src/lib.rs:911`) is a plan-swap
   restore through a payload, not a swap. The slice needs both on six effects and on
   `LiveControlEffectBankStage` (controls, taps, bypass shunt): more than half a day, so it
   splits up front.
3. **No V8 sparse row.** The V8 harness documents are `nine_track_eq`,
   `sixty_four_track_console`, `_app_shape` and `_console_sends`
   (`scripts/web-mixing-automation-benchmark.mjs:144-149`). Gate 3's V8 leg needs a new row,
   which the brief splits to a qualification issue first.
4. **No Simd4 plan leg on this host.** On an AVX2 build, five of the six console factories
   bind only the build's width (#1694, open), so gate 1's Simd4 leg needs #1694 or a wasm32
   test run.

Proposed split, in order: (a) qualification: the V8 sparse row; (b) the ruling in item 1, or
#1372/#1375/#1376; (c) a per-lane fixed-point predicate and an allocation-free per-lane swap
on the bank contract, effect by effect; (d) the chain compaction with gates 1-3.

### D3 Root ruling and split (2026-10-22)

- **Item 1, ruled.** An exact fixed point does not need a `RestSamples` bound. A lane is at rest
  when a state-only per-lane predicate holds whose meaning is: one block of exact `+0.0` input
  leaves every state word of the lane bit-identical and outputs exact `+0.0`. That is the same
  proof the shipped #163 fast paths use. `RestSamples` bounds only *when* the predicate becomes
  true; #1372, #1375 and #1376 stay open and independent.
- **Split (owner, 2026-10-22: "Split, do it next").** This issue keeps the chain compaction (D1)
  and gates 1-3. It starts after:
  - #1744 the V8 sparse row (item 3);
  - #1745 and #1746 the per-lane rest predicate (item 2), EQ/compressor/limiter, then
    gate/transient shaper/soft clip;
  - #1747 the allocation-free per-lane swap (item 2);
  - for gate 1's Simd4 leg (item 4): #1694, or a wasm32 run of the differential.

### D4 State at the start of attempt 2, and root's added constraints (2026-10-23)

Delivered since D3 (all merged): #1744 (the V8 sparse row: silent lanes save nothing in the
browser today, 242/247 against 247/251 us), #1745 and #1746 (`lanes_at_rest` on all six
console effects, each with its bank-wide fast path), #1747 (`swap_bank_lanes`,
`advance_clocks`, rack `BankStage::swap_lanes` and `BankChain::swap_lanes`), #1765 (builtin
input, fader and matrix stage swaps through `BuiltinStage`), #1763 (`BypassShunt::feed_line`
and the live effect stage's quiet-publication skip), #1767 (the fused fader+matrix stage is
gone; every builtin bank is a separate stage).

Corrections to D1:
- Banks of one slot do NOT keep equal cursors (a whole-bank D7 reset moves one bank's cursors);
  a swap is still exact because each payload is written and restored relative to its own
  bank's cursor (#1747 D1).
- A skipped lane's clocks advance through `advance_clocks(lanes_mask, frames)`, which a caller
  may call only for lanes `lanes_at_rest` reports, on a block with no automation span, prepared
  target or live write for those lanes, with exact `+0.0` input.
- The bypass shunt's latency line: a skipped lane must leave its line words as a rendered
  `+0.0` block would (feed `+0.0` through `feed_line`, or prove the words are already `+0.0`
  and advance the cursor only).

Constraints from the reviews of 2026-10-23 (Fable design review S5; #1747, #1765 verifiers):
1. **Swap budget and hysteresis.** A lane whose activity toggles block to block must not swap
   every block, and one block must not move every lane. Bound the swaps per block per chain (a
   preallocated constant, recorded with its reason) and keep a newly silent lane in place for a
   minimum residency before it may be packed out. Record the toggle case's cost in gate 3.
2. **Equal collapse arming.** `BankChain::swap_lanes` refuses a cross-chain swap when the two
   chains' collapse arming differs (#1747), so packing crosses only chains of equal arming.
3. **Stored automation.** A lane with stored fader automation cannot move into a fader bank with
   no automation slots (#1765); the packer must plan around the refusal, never hit it at render.
4. **Graph-level lane maps.** A chain swap does not update the graph's lane maps (bank members,
   gather/scatter, meter and observer lanes, response-snapshot lanes, the later plan-carry
   pairing, #1765 verifier); the lane map of D1 must feed every one of them, or the slice must
   prove that each reads through the chain.
5. **Plan-level test.** #1765's swap test builds chains from the real stages by hand; this issue
   owes one swap test on a graph-compiled plan, and its gate 1 differential runs on compiled
   plans.
6. **R7.** With real stages, a stale right channel restored after an in-chain swap on a collapsed
   chain is guarded only by a debug assertion (#1747, #1765); gate 1 runs collapsed chains.

### D5 Attempt-2 design and proposed split (implementer, 2026-10-23; for root's approval)

Facts read for this design (`rt` = `crates/graph/src/runtime.rs`, anchors at a0b950874):
- The standard console strip is **one** `RuntimeUnit::Bank` per bank: one `BankChain` holds input
  builtins, the console and banked insert slots, fader and matrix, and the route is folded into
  the scatter epilogue (`graph-compiler/src/lib.rs` near 11453). The 64-track console is 8
  consecutive peer units at W=8 and 16 at W=4. Packing is therefore a **cross-unit** move
  (`ChainLanePeer::Other`), and the lane map is graph-level.
- Still missing, not delivered by #1745-#1765: a rest predicate and a clock advance on the rack
  `BankStage` trait (only `PreparedNativeEffectBank` has them) and on the builtin input, fader and
  matrix banks. Without them no production chain can report a lane at rest.
- Hazards for a cross-unit move that D1 did not name:
  - **Master order.** On a folded plan, `route_fold` (rt near 9875) proves that the master's
    edge order equals the chain-lane render order. If a strip moves to another unit, its add
    into the master happens at a different position, and the master bits move. AGENTS.md: "a
    placement change must not move a rendered bit".
  - **Signed zeros.** A `store` lane writes the master, and the route of a `+0.0` lane can give
    `-0.0` (a negative coefficient). Skipping a `+0.0` add changes a `-0.0` master word.
  - **Schedule.** A slot-0 member's reduction and PDC staging run in its home unit's `execute`.
    If a strip moves to an earlier unit, it would read an input that is not produced yet.
  - **Resident pairs** (`arm_resident_inputs`, rt near 9073) must be permuted together.
- Lane-keyed consumers: every per-lane graph object is **member-owned**, so it follows the strip
  if `members[slot * lanes + lane]` is exchanged with the chain swap:
  - slot-0 inputs, PDC lines and scatter redirects;
  - last-slot outputs and their observers and meters.

  The objects that need a map:
  - `fold[lane]`: a `FoldLane` holds the route coefficients and the positional `store`;
  - `source_lanes` (a u8 mask);
  - response-snapshot bindings `{unit, member}` (rt near 3936 and 9002);
  - the predecessor side of a plan carry (raw `(unit, slot, lane)` from the bind-time
    inventory, `adopt_predecessor`, lib near 5300);
  - the evidence rows of `unit_eligibility`.

#### Proposed split (the whole slice does not fit one attempt under the half-day rule)

1. **#1107a: the rest predicates on every production stage.** This is a building block, the same
   kind as #1745 and #1746.
   - `BankStage::{lanes_at_rest, advance_clocks}`, declining by default, on:
     - `EffectBankStage`: forwards to the effect.
     - `LiveControlEffectBankStage`: the effect's mask, minus every lane with a staged span, a
       staged prepared target, a bypass, a running crossfade or an armed observation tap after
       this block's drain. The clock advance also feeds the shunt line a `+0.0` block through
       `feed_line` (D4), so the line words stay as a rendered zero block leaves them.
     - `BuiltinStage`: forwards through `GraphPreparedBuiltinBankProcessor`.
   - Builtin input, fader and matrix banks:
     - Each has a state-only predicate under the D3 ruling: one `+0.0` block leaves every state
       word bit-equal and writes `+0.0`.
     - Input: all integrators at `+0.0` bits and no trim or filter ramp in flight. A lane whose
       signed trim or polarity can turn `+0.0` into `-0.0` declines unless the kernel proves
       otherwise.
     - Fader: no gain or mute ramp in flight.
     - Matrix: not ramping, and no output row whose two coefficients both carry the sign bit.
     - Processor level: no automation span this block (`automation[lane].span`).
   - Clocks: the input's silence counter (`lane::silence_advance`). The fader and matrix have
     none.
   - `BankChain::lanes_at_rest()`: the AND over the slots. A slot that is an identity on a lane
     counts as at rest on that lane.
   - Gate: one rest-probe test per new predicate, in the crate that owns it (as #1745). The test
     renders a `+0.0` block and fails when a reported bit is not an exact fixed point.
2. **#1107: the whole-bank skip on compiled plans** (this issue, narrowed). This is the smallest
   closable product slice. It removes all work of a bank whose every lane is silent, which the
   owner's priority asks for. It moves no lane, so the hazards above do not arise. Design:
   - `BankChain::run_with_input` is split. Step 0 (`begin_block` on every slot) and the collapse
     dispatch (disengage, agreement maintenance, counters) run on every block, unchanged. Then,
     when every lane's gather source is exact `+0.0` on both planes and the chain's
     `lanes_at_rest()` covers all lanes (padded lanes included), the chain calls
     `skip_block(frames)`.
   - The input test runs first. It is the `block_is_positive_zero` head on each lane's source,
     which returns on the first non-zero word. The rest masks are read only when every input is
     zero. So an all-active block pays W head checks per bank and nothing more.
   - `skip_block` calls `advance_clocks(all lanes, frames)` on every slot and does no gather, no
     process and no scatter transpose. On a collapsed block it also advances the frozen right
     clocks. Nothing can see them before the disengage copies the left channel over them, and
     the swap and the carry paths disengage first.
   - The resident scratch is zeroed once per run of skips (a `resident_zero` flag), so that
     `final_output_lane`, the observers and the meters read the `+0.0` that a rendered block
     leaves there.
   - Unfolded lanes: their output buffers (redirects included) get `frames` words of `+0.0` per
     plane.
   - Folded lanes keep the master's sequence exactly. A `store` lane stores its
     route-of-`+0.0` constants (bind-time, per plane: `-0.0` only if both of the plane's
     coefficients are negative). Each add lane's constant is added in order. An add of `-0.0` is
     an identity for every word, so only a `+0.0` add is performed, and that is one
     `sum_into_block` per plane per skipped bank, not per lane.
   - A chain with a slot that reads a sidechain never skips. That is decided at bind.
   - The drain and the observation queues still run on every block, so no acknowledgement and
     no publication is held.
   - Allocation-free and bounded by the lanes, the slots and the frames.
   - Gate 1, a class A differential on compiled plans:
     - It renders sessions with random per-lane silence runs (whole banks silent at times,
       mid-ramp edits, a bypass edge, stored fader automation, mono-armed collapsed chains for
       R7), with and without the skip. Every strip output and the master must be equal by
       `to_bits`, with NaNs folded.
     - The no-skip twin needs one `test-support` switch on the plan (root approval needed: a
       hook in `crates/graph/src/lib.rs`).
   - Gate 2: allocations == 0 on the render thread with the skip active.
   - Gate 3: one native and one V8 console run. The idle and sparse rows are read against S4 and
     the current main; the sparse row is not expected to move in this slice. The all-active row
     must be within 2 % of S4.
3. **#1107b: packing (swap-to-pack)** on top of #1107a and #1107. This is where D4's constraints
   1-6 apply. Design sketch, to be refined in its own brief:
   - **Pack groups.** A pack group is a maximal run of consecutive Bank units in the schedule
     that are peers. Peers have an equal slot program and an equal collapse arming (D4-2). A
     group has no resident pairing and has only direct gather sources (`bank_gather_source` is
     `Some` for every slot-0 member, so a move changes no reduction order). Either every unit
     folds into one master or none folds. Other units only whole-bank skip.
   - **The lane map.** It lives in the group as `home[strip] -> (unit, lane)`, its inverse, and
     a per-strip `automated` bit. A move exchanges the two chains' lanes and, in the same call,
     `members[s * lanes + lane]` for every slot `s`, `fold[lane]` and the `source_lanes` bits.
     Then gather, scatter, PDC, redirects, observers and meters follow the strip with no reader
     change. The other readers translate through the map:
     - response-snapshot bindings;
     - the predecessor side of the carry;
     - `unit_eligibility`.
   - **Folded groups.** The epilogue fold is deferred to a group-level fold after the group's
     last unit, in home order, and it reads each strip's staged tile through the map. That
     keeps the master's association exactly as `route_fold` proved it. Skipped strips take the
     signed-zero rule of item 2.
   - **Packing decision.** It is made per block, after the drains.
     - An active lane in a bank that would otherwise be skipped is always moved in, or that bank
       runs. Running is always exact.
     - Empty out the bank with the fewest active lanes into lanes that have been idle for at
       least `R` blocks in running banks.
     - Hysteresis: a lane idles for `R` blocks before it may be packed out.
     - Budget: at most `B = W` moves per group per block, so the worst block moves one bank's
       worth of state.
     - `R` and `B` are preallocated constants, recorded with their reasons in #1107b.
     - Automated strips move only into fader banks with automation slots (D4-3). This is a
       bind-time per-unit bit, checked before a move is chosen, so the render path never
       reaches the refusal.
   - **Tests.** A plan-level swap test (D4-5), and gate 1 rerun with packing. The sparse row is
     read against S4.
4. **#1107c, qualification, before #1107b's timing:** the toggle row (D4-1) and a row with one
   active lane per bank, native and V8.

W=4 and W=8: the skip is per bank at both widths. At W=4 the 64-track console has 16 banks and
at W=8 it has 8. #1107 skips a bank only when all of its lanes are silent. The sparse row (odd
tracks silent) skips nothing until #1107b packs it: 32 active lanes into 8 of 16 banks at W=4,
into 4 of 8 at W=8.

#### Gate 1's Simd4 leg

The differential will be a `graph-compiler` test, and graph-compiler is a product crate. The
arm64 CI legs (`aarch64-debug` and `aarch64-release`, `scripts/run-aarch64-tests.sh`) run it
natively at Simd4 (NEON). Locally it runs at Simd8 only. That means:
- No wasm32 run: no runner exists here for a compiled-plan test.
- No #1694 dependency: it is open, and AVX2 cannot bind Simd4 console banks.
- No qemu on this host.

The Simd4 result exists only after root's batch push. If root needs it before the merge, the
alternative is #1694.

#### Authorized paths (for #1107a and #1107; root to confirm)

- `crates/rack/src/lib.rs`, `crates/rack/tests/*`
- `crates/effect-contract/src/live.rs` (only if `feed_line` needs a zero-source form)
- `crates/builtins/src/lib.rs`, `crates/builtins-compiler/src/lib.rs`, and their tests
- `crates/graph/src/runtime.rs`, and `crates/graph/src/lib.rs` (only the trait methods on
  `GraphPreparedBuiltinBankProcessor` and the test-support skip switch)
- `crates/graph-compiler/tests/` (one new differential file)
- this spec

Not touched: the effect crates (their predicates exist), benchmark tools and scripts (#1107c owns
rows; the runs use the existing scripts), host-core, capi, host-web.
