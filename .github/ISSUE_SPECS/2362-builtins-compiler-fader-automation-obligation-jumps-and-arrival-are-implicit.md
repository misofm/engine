# builtins-compiler: fader automation's held-ramp obligation, jump lengths and input arrival are bools, correlated Options and a placeholder

Tracker #2258, item GR-9. Patterns: several bools whose combinations encode one phase, three
`Option`s documented as "`None` exactly when" each other, and a field that holds a placeholder
`0` until a later install writes the real value, where `0` is also a valid value.

## Mission
Model the stored fader automation's per-channel hold phase, per-block jump lengths and per-lane
input arrival `a(n)` so that the illegal combinations cannot be represented and the arrival is
never observable before it is known, without moving a rendered bit.

## Verified (base `5c71b3e8d`)
All in `crates/builtins-compiler/src/lib.rs` unless named.

- **`HeldRampObligation { owed, due_at_entry, held_through }`** (1210-1225): three bools, 8
  combinations. Writers: `FaderCellCursor::rehold` (2193-2234), `HeldRampObligation::block`
  (2758-2791), `AutomatedFaderCell::offsets_jump` (1826), `FaderEventDriver::apply` (3097-3135,
  `owed` only).
  - `due_at_entry` is set `true` only together with `held_through = true` (2230-2231,
    2771-2772), and both are reset by `block` (`mem::take`) and by `rehold`.
  - `offsets_jump` clears `held_through` alone (1826). It runs in the block drain, at the entry
    of the block whose first sample is the one a `due_at_entry` hold frees, so
    `due_at_entry && !held_through` **is reachable**. It is indistinguishable from
    `due_at_entry && held_through`: `block` never reads `held_through` when `due` is set (the
    held arms read neither, `due && owed` does not read it, and the free arm computes
    `held_through && !due`).
  - So the two hold bools carry three phases (free / held through the block's last sample / held
    ending exactly on it), one of which has two spellings; `owed` is independent of them (a
    remembered event survives a ramp that ends muted, constraint 4). Six distinct states, eight
    representable.
- **`fader_jump: Option<u32>`, `mute_jump: Option<u32>`, `vca_jumps: Option<[u32; 2]>`**
  (1266-1277): all three are written together from one `Option<BlockJumps>` in
  `take_block_time` (1456-1461), and documented as `None` exactly when each other is. The event
  driver re-joins them with a four-way `let (Some, Some, Some, Some) = ... else continue`
  (2415-2418); `mute_terms` reads only `mute_jump` as the presence of all three (1499).
- **`arrival: u64`** (1259-1262): `LaneFaderAutomation::new` sets `0` (1325), "`0` until"
  `install_builtin_input_arrivals` writes the member's `a(n)` after lowering
  (`graph-compiler/src/compile.rs` 1282-1294 → `graph::PreparedGraphPlan::install_builtin_input_arrivals`
  (`crates/graph/src/lib.rs` 2865) → `FaderBankProcessor::set_input_arrivals` (5326); the scalar
  oracle's `set_input_arrival` (9399, 9639)). `0` is also the genuine `a(n)` of a member absent
  from `input_arrivals`. Readers: `take_block_time` (`node_span(a(n))`, render), `adopt`
  (`carried_arrival != arrival`, the swap block), and the test probe.
  - **No defect is observable today.** The lanes are built inside
    `into_graph_artifact_with_banks` (7486; `bound_lane_automation` 7135, 7637), and the one
    production caller, `attach_builtins`, installs the arrivals on the next line, before bind and
    before any render or adoption reads the field. No reader branches on `0` as "not installed".
  - What the placeholder does allow: any lowering that does not make the second call (today the
    builtins-compiler unit tests and `graph-compiler/tests/scale.rs`, which call
    `into_graph_artifact_with_banks` directly) silently renders every lane at `a(n) = 0`, and
    the type cannot say whether the value was installed.

## Decision / model
- `HeldRampObligation { owed: bool, hold: HoldAtBlockEnd }`, `enum HoldAtBlockEnd { Free,
  HeldThrough, EndsAtBlockEnd }`: `EndsAtBlockEnd` is "held through the block's last sample and
  free from the next block's first sample, the owed move is due there" (the old
  `due_at_entry`, which implied `held_through`). `offsets_jump`'s release turns `HeldThrough`
  into `Free` and leaves `EndsAtBlockEnd` as it is, which is what the base computed (its cleared
  `held_through` was never read while `due_at_entry` was set). `owed` stays a bool: it is
  orthogonal to the phase.
- `jumps: Option<BlockJumps>` replaces the three Options; the driver destructures one
  `Some(BlockJumps { .. })`.
- `a(n)` is a lowering input, not a later install: `into_graph_artifact_with_banks` (and the
  oracle's `into_graph_artifact`) take the compile report's `input_arrivals`, and every lane is
  built with its member's `a(n)` (`graph::floored_input_arrival`, the same lookup). The
  two-phase install is deleted: `PreparedBuiltinsGraphArtifact::install_builtin_input_arrivals`,
  `graph::PreparedGraphPlan::install_builtin_input_arrivals`,
  `GraphPreparedBuiltinBankProcessor::set_input_arrivals`, and
  `GraphRuntimeProcessor::set_input_arrival` / `GraphNodeBinding::set_input_arrival`. A caller
  that has no report passes `&[]`, which is the report's own encoding of "every arrival is 0",
  not a placeholder.
- SIMD lane words are untouched.

## Non-goals
- No change to `crates/builtins/src/lib.rs` or the graph fixtures (#2354 / PR #2360 own them).
- No change to the automation rules (D1.6, R17-8, R17-10, constraint 4/7) or to any event the
  driver emits.

## Gates
- No observable defect, so no structural red-on-base test is added (test-value rule). The
  refactor is covered by the existing tests, shown by mutation: each changed transition reverted
  or perturbed turns a named existing test red.
- One-time no-bit-moved comparison against base (PR evidence, not a committed test).
- `cargo test --locked` for builtins-compiler, graph, graph-compiler, host-core (with
  `test-support` where the suites gate on it); clippy `--all-targets -D warnings` on the four;
  `cargo fmt --check`; `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps`;
  `scripts/check-workspace-policy.sh`; `realtime_policy`.

## Evidence (attempt 1)
(filled in by the attempt)
