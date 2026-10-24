# Charge the fader offset writers in the strip control table

Filed by root from #1074's verification (backlog sprint, 2026-10-24).

## Finding

`FaderCells.offsets: Option<Box<[Option<OffsetsWriter>; 2]>>` (`crates/builtins-compiler/src/lib.rs` near line 301, added with #1649 in 5e82ba245) is a heap owner that `strip_control_table_bytes` (`crates/host-core/src/prepare.rs` near lines 645-651) does not charge: that charge covers only `[TrackControlProducer]` plus the ID bytes. On the `builtins-automation.json` fixture the strip producer table frees 1016 bytes while 936 are charged. So capi's resource ledger under-reports a session with automated faders, and #1074's plan-equality row for that fixture cannot be kept.

## Smallest closable slice

1. Charge every heap owner of the strip control table, including the offset writers, exactly (capacity, not length).
2. Add the `builtins-automation.json` row to `crates/capi/tests/resource_lifecycle.rs` that #1074 dropped: `owners.plan == walked` (96771 on that fixture at #1074) and the strip table charge equal to its freed bytes. This row also holds #1074's automation walkers (FollowCursors, LaneFaderAutomation, `GraphExecutor.automation_program`).

## Paths

`crates/host-core/src/prepare.rs`, `crates/builtins-compiler/src/lib.rs` (a byte accessor only, if needed), `crates/capi/tests/resource_lifecycle.rs`, this spec. After #1074 merges.

## Decision record (attempt 1, 2026-10-24, from main 97d1796ff)

- D1 (root R1): the accessor is `TrackControlProducer::retained_heap_bytes` in `builtins-compiler`, a `size_of::<[Option<OffsetsWriter>; 2]>()` when the strip's offsets box exists, else 0. `FaderCells` keeps its field private; a future producer heap owner belongs in the same accessor.
- D2 (root R2): `host_core::strip_control_table_bytes` takes a third `u64` (`producer_heap_bytes`, `checked_add`), not a new report row. `control_plane::prepare_runtime` sums the accessor over `strip_controls` (checked) before boxing and passes it to `prepared_capi_resources`. The `largest` row is unchanged.
- D3 (root R3): builtins' processor accumulator charges `offsets_cell_bytes` (the shared `OffsetsLane`), not the writer box, so the box is charged once, to the capi cap. #1256 D4's producer-vector double charge is unchanged.
- D4: control-plane needs `crates/control-plane/src/compile.rs` (`prepared_capi_resources` and `prepare_runtime`'s call) beyond the spec's paths, per the scope note.

## Evidence (attempt 1)

- Red first (fixture row added, charge unchanged): `builtins automation` capi row observed 139476 against charged 139396 (80 bytes = one 2 x 40-byte offsets box).
- Green: capi 139476 = 139476; strips charge equals freed bytes; plan 96771 walked = 96771 freed. Numbers are PR evidence, not committed literals.
- Test value: a strip-producer heap owner the strip-table charge omits turns the `builtins automation` row red; no other fixture in the loop has automated faders.
