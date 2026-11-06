# Graph: a fade arm whose fader lane does not resolve reports fired and leaves the lane armed and muted

Found by the implicit-state-machine audit of 2026-11-06.

## Problem
`step_fade_arms` (`crates/graph/src/lib.rs`) fired a fader arm only if
`runtime::strip_lane_mut` and `runtime::builtin_processor_mut` both resolved. When either failed it
did nothing, but the arm still moved to `Settling`/`Done` and the watch still raised `WATCH_FIRED`
and `WATCH_SETTLED`. The fader stage keeps its own `armed`/`opens` flags (`FaderRampStage`,
`crates/builtins/src/lib.rs`), and only `fire_fade_in` clears them. So a failed lookup would
leave the lane muted while the plan reported the fade settled, and the host would drop the arm's
inventory row. The fade-arm lifecycle exists twice: the graph's `FadeArmState` and the builtins'
`armed`/`opens` flags.

## Contract
- Make the arm's target explicit so that render cannot fail to resolve it. Either resolve and
  validate the fader lane at install, or prove by construction that it always resolves.
- Firing returns a typed outcome, never a silent no-op. If a failure path remains, the arm stays
  unfired, the failure is counted, and the arm is never reported fired.
- Find out, and record, whether the failed lookup is reachable today.
- Keep the change minimal: `FaderRampStage`'s SIMD lane words do not become enums.

## Non-goals
Host plumbing of the new counter (`crates/host-core`). No change to `builtins-compiler`, the
processor trait's signatures, or any rendered bit.

## Gates
- If the failure is reachable, a regression test reaches it, red on current main. If not, a
  recorded proof that it is unreachable, plus a type-level change that keeps it so.
- No rendered bit moves (a one-time comparison against base, as PR evidence).
- `cargo test -p graph -p builtins`, clippy, fmt, `scripts/check-workspace-policy.sh`, the
  realtime policy tool.

## Evidence (attempt 1)

**Reachability: not reachable today.** Each step that could break resolution, checked:
- *Install.* `install_fade_arms` validated the home location with `has_strip_lane` and armed it
  through the same raw lookup. The only production caller is `crates/host-core/src/prepare.rs`. It
  runs on a freshly bound plan before publication, so its lane map is the identity
  (`LaneMap::discover`, `displaced == 0`; only render moves lanes), and the home location equals
  `current_location(home)`.
- *Units.* After bind, a plan's `units` and each chain's slots never change, so the unit, the slot
  and the processor's downcast stay valid.
- *Lane moves (#1782a, #2142).* The map changes only in `swap_bank_lanes`, through
  `record_swap`, and `restore_home_lanes` is a series of those swaps. The swap first checks that
  both positions are populated in every row (`at.1 < lanes`). It then checks each row's chain
  (`lane_swap_check`), and records the swap only after every row has traded. Peers have one slot
  program (`pack_peers` needs `can_swap_lanes` in every slot, and a fader processor accepts only
  another fader processor whose lane is below `members`). So `current_location(home)` is always
  a populated lane of a fader stage in the same slot. A fader stage's `carried_strip_lanes` is its
  `active_lanes` (`crates/builtins-compiler/src/lib.rs`), a constant. `armed`/`opens` travel with
  the lane in `export_lane`/`import_lane`.
- *Plan swaps and carry (#1392, #1747).* The successor is fresh, and its arms are installed on
  the identity map. A strip's arm is paired with a predecessor entry only when its fader lane
  carries (`prepare.rs`, `fade_strips`). So the lane import and the copied fired state agree.
- *Collapse.* Collapse does not change a chain's slots. `current_location` falls back to `at` only
  if a unit index overflows `u32` or a lane index overflows `u16`, which cannot happen.
- *Route targets.* Route units are fixed. Install checks `has_live_route`, and render's
  `carried_route_mut` asks the same question of the same unit.

**Model chosen: one resolver and a typed fire outcome.** A typed handle that cannot fail would
need a raw pointer into the boxed processor (`unsafe`), so it was rejected.
- `Runtime::fader_lane_mut(home)` maps `current_location`, then `strip_lane_mut`, then
  `builtin_processor_mut`. It is now the only resolution of a fader arm's target: install
  validates and arms through it, and render fires through it, so the lane armed is the lane fired
  under any lane map. Under today's identity map at install, this is the same lookup as before.
- `FadeFire::fire` returns `Fired` or `Unresolved`. Only `Fired` moves an arm to `Settling` and
  raises `WATCH_FIRED`. `Unresolved` leaves the arm due (`FireAt`) and unreported, and increments
  `fade_arms_unresolved`, which `graph::fade_arm_unresolved_count(plan)` reads. The arm is retried
  each block, so it fires at the first block its lane resolves.
- `Runtime::fire_route_fade_in` now returns whether it reached a route (`#[must_use]`), and the
  route fire in `carry_fade_arms` follows the same rule.
- **The two copies are provably consistent.** An entry exists only if the processor's
  `arm_fade_in` returned true, and the entry is fired exactly when `fire_fade_in` reached that
  lane. The builtins' `armed` doc now states this pairing, and that a lane move or carry moves
  the flag with the lane. The SIMD lane words are unchanged.

**Red/green.** New test: `runtime::tests::a_fade_arm_whose_lane_does_not_resolve_stays_unfired_and_counted`
(`crates/graph/src/runtime.rs`). It installs a due arm on a `FadeProbe` lane, then shrinks the
probe below the lane. After three blocks the arm must still be unfired, unsettled and pending,
with the count at 3. Once the probe is restored, the arm fires at block 3, and the probe records
that one fire.
- On base `8f46ab035` (the test without its two counter asserts, since the counter does not
  exist there) it fails: `an unresolved arm after block 0  left: (true, false, 1)  right: (false, false, 1)`.
- At head it passes.

**Test value.** The test turns red if render moves an arm to settling, or raises its watch flag,
without its fire reaching the lane. In that case the plan reports the fade done while the lane
waits, muted. No existing test catches this: the gate-2 test only covers lanes that resolve.

**No bit moved.** A scratch digest of every block rendered through `successor_swap.rs`'s
`render_into`, an FNV-1a hash per test thread, was added for this run only, never committed, and
reverted. It covered `cargo test -p host-core --features test-support --test successor_swap`:
126 tests, 69 of which render through it, 15,904 blocks. These include every #1288, #1363,
#1392, #1953b and #2080 fade gate (added tracks and routes, pending fades across a second swap,
fired fades, fading copies, restarted strips). All 69 digests are identical at base `8f46ab035`
and at head.

**Gates (head).**
- `cargo test --locked -p graph -p builtins`: all pass.
- `cargo test --locked -p host-core --features test-support` (host-core is unchanged, but it is
  the one production caller): all 58 test binaries pass.
- `cargo clippy --locked -p graph -p builtins --all-targets -- -D warnings`, with and without
  `graph/test-support`: clean.
- `cargo fmt --check -p graph -p builtins`: clean.
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
- `cargo run --release -p realtime-policy`: `realtime policy: ok (292 marked regions in 46 files)`.

**Open.** No host reads `fade_arm_unresolved_count` yet. Wiring it into host-core counters or
telemetry is a follow-up outside this issue's file set.
