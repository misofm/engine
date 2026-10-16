# Compose VCA offsets with stored fader automation

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answers A1.5 and A2, under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`. Batch R1. The offsets cell's layout is root's decision in
README finding F16 (layout 4: the offsets sum first).

## Product outcome

A producer automates a vocal fader; a fan rides the "vocals" VCA, in the browser or through a C ABI
host. The VCA move is heard on top of the ride: the member lane follows its curve plus the VCA's
offset, and after the fader jump ramp the output equals a plan prepared with that VCA offset and
the same automation, bit for bit, whatever the number of VCAs that reach the lane. On both hosts
the move goes through the shared commit's VCA row (*Deliver value-only VCA edits to the running C
ABI plan*, #1247; the browser through #1382), which writes the lane's offsets cell instead of a
fader record, and the path is `live`. Every other VCA edit that #1247 makes live stays `live` on an
automated member too: a change of a VCA's members, nested VCAs included, and adding or removing a
VCA. A member whose fader is not automated keeps today's records. This slice moves no rendered bit
of a session without stored automation (#1646 D6 is the one order change).

## Context

- **The one composition.** Today `vca_effective_db(member_db, offsets)` sums in `f64`, the
  member's own value first, then each offset in the order given, clamps to `[-144, 24]` and rounds
  once to `f32`; with no offset it returns the member bit for bit (`crates/session/src/vca.rs:19-34`).
  `effective_strip_faders` passes each strip's reaching VCAs' offsets in ascending VCA-ID order
  (`:96-120`), and `crates/session/tests/vca_composition.rs:79-82` pins the order. #1646 D6
  (README F16, root's decision) changes it: `vca_offsets_sum` sums the offsets first and `vca_compose_db(member,
  S)` adds the member, on every path.
- **#1646** creates and seeds the offsets cell (#1646 D1): for each automated fader lane,
  in every session, a #1312 cell of three words, the lane's `S` as one `f64` and a ramp word; `S`
  is `+0.0` when no VCA reaches the lane. #1646 D1 shows that the cell keeps the static gain
  bits. Render computes each event's gain as `checked_fader_gain(vca_compose_db(v, S))` from the
  cell's newest slot. Nothing writes the cell after preparation yet.
- **The shared VCA row.** #1247 D2: the classifier's strip step compares `effective_strip_faders()`
  of `current` and `next`, lane by lane, and emits one `FaderDb` per lane whose effective gain bits
  change. #1247 makes VCA rides, mutes and membership live on the C ABI (D15-6), because the C ABI's
  render plane holds no VCA state. After #1382 a browser VCA edit reaches the same shared commit
  through the Worker's apply and takes the same row.
- **#1648 D3** makes that step emit no `FaderDb` for a lane that `next` automates, so until this
  slice a VCA move does not reach an automated member.
- **The cell primitive.** #1312 D1: a cell has three slots of words and a sequence; the writer fills
  every word of its back slot, then publishes; render reads the newest completed write in one pass
  and never sees a mixed slot.

## Decisions frozen for this slice

- **D1. The composition is #1646 D6's.** This slice calls `vca_offsets_sum` and
  `vca_compose_db` and changes neither. A flat curve at `v` renders the bits a static `v` prepares,
  however many VCAs reach the lane.
- **D2. The offsets cell is #1646's.** This slice adds only the write that rewrites it, a whole
  slot at a time (D4), and the jump at the next block entry (D3).
- **D3. Render.**
  - At each fader event, the gain is `checked_fader_gain(vca_compose_db(curve, S))`, with `S` from
    the cell's front slot (the newest write render has read), as #1646 D1 composes it.
  - At block entry, when the cell is dirty, render reads the new slot. If the composed gain at the
    jump's completion sample differs in bits from the lane's current gain target, the lane takes a
    jump at the block's first sample over the slot's ramp word (D4): the length that #1247 D5 gives
    the same VCA edit's `FaderDb` records on members that are not automated. Otherwise nothing
    happens. The comparison is on the gain, never on the dB sum, so an `S = +0.0` that turns a
    `-0.0` value into `+0.0` causes no jump. Grid events during that jump are held
    (README "Held events").
- **D4. The shared commit writes the cell.**
  - In #1247's strip step, for a lane that `next` automates, the classifier computes the lane's `S`
    for `current` and `next` (`vca_offsets_sum` over each model's reach offsets for the lane's
    channel, in ascending VCA-ID order, `+0.0` for an empty reach). If the bits differ, it emits one
    offsets-cell write that carries `next`'s `S` and the ramp word
    `LiveRamps::resolve(VcaFader, the edit's own ramp)` (*Carry an optional per-edit ramp length on
    live session edits*, #1394 D5-D6; #1247 D5), so one VCA edit ramps its automated and its plain
    members over one length. A ride, a membership change, a nested-VCA change, adding a VCA and
    removing one are all such writes: each changes only `S`, and the cell exists on every automated
    fader lane (#1646 D1). #1312's publication rule needs every word of the slot, so the writer
    never writes one word alone. The path is `live`; no VCA edit rebuilds.
  - For a lane that is not automated, #1247's `FaderDb` records are unchanged.
- **D5. Memory and work.** Each offsets cell holds three 4-byte words in each of #1312's three
  slots: 36 bytes per automated fader lane, whatever the session's VCA count, plus the cell's fixed
  sequence and index words (#1646 D4 charges them). Render adds one `f64` (`S`) per event;
  nothing depends on song length or on the VCA count.
- **D6. Out of scope here:** a VCA mute on a member whose mute is automated (#1651).
- **D7. The acked-batch question: can an ack ever precede a drop? No.** The commit checks the whole
  transaction before the first cell write, and a cell write cannot fail; a replaced slot is in the
  committed state and counted (#1312 D2).

## Deliverables

1. D3: render's dirty read at block entry and the jump.
2. D4 in the shared classifier and the commit path: the `S` comparison and the cell write.

## Authorized paths

- `crates/host-core/src/live_delta.rs` (the strip step's automated-lane branch),
  `crates/host-core/tests/live_delta.rs`, `crates/host-core/tests/vca_live.rs`
- `crates/control-plane/src/` (the offsets-cell write)
- `crates/builtins-compiler/src/lib.rs` (the fader bank's dirty read of the offsets cell and the
  jump only)
- `crates/capi/src/runtime/live_tests.rs`, `hosts/host-web/src/tests.rs`,
  `crates/control-plane/tests/vca_automation_realtime.rs` (new)
- `crates/control-plane/Cargo.toml` (a `bench-support` dev-dependency for that binary, only if an
  earlier slice has not added it), `Cargo.lock`

`crates/session/src/vca.rs` is called, never changed (#1646 D6 changes it).

## Non-goals

- VCA automation (out of scope by decision 13, `docs/rulings/submix-strips-sends-and-vca-2026-10-02.md:129`).
- A VCA mute on an automated mute lane (#1651).
- Any change to the composition beyond #1646 D6.

## Hazards

- **A redundant jump moves bits.** D3 compares bits before it jumps; D4 writes only on a change.
- **Batch R1, one push with #1646, #1647 and #1648** (README "Must-land-together groups"). #1648
  D3 drops the `FaderDb` of an automated member; without this slice an acknowledged VCA ride on that
  member is not heard.

## Objective gates

1. **Automated member, both hosts** (`crates/capi/src/runtime/live_tests.rs` and
   `hosts/host-web/src/tests.rs`, new). Track 0 has a linear fader ride and is a member of VCA `v`;
   track 1 is a member with no automation. A transaction (C ABI) and a browser live VCA edit through the Worker's apply
   move `v` by -6 dB on a playing engine: path `live`. From the completion sample of the first grid
   ramp after the jump, the output equals a plan prepared with `v` at -6 dB and the same
   automation, fed the same PCM from frame 0. Track 1 receives exactly one `FaderDb`, as today. The same move
   with its own ramp of 256 samples: track 0's jump and track 1's `FaderDb` ramp both last 256
   samples.
2. **Many moves, one block** (browser, same file). Ten browser live edits of `v`'s offset before one render: the
   output equals a twin that made only the last. Three nested VCAs reach track 0, declared in an
   order that is not their ID order and nested so that the nesting order is not their ID order
   either. Their offsets are `24`, `-24` and `2e-30` in ascending VCA-ID order. A move of the
   third VCA to `1e-30` writes the `S` that a fresh plan of the edited session computes, bit for
   bit: the sum in ID order, `(24 + -24) + 1e-30 = 1e-30`, has other `f64` bits than an order that
   adds `1e-30` before `-24`, which gives `0.0`. With two offsets, no order can change the sum
   (`f64` addition is commutative), so the case needs three.
3. **No jump for an unchanged offset** (same files). A move of a VCA that does not reach track 0, or
   that moves `v` to its current value, writes no cell and moves no bit.
4. **Reach change and VCA count change** (C ABI and browser, same files). Each of these is path
   `live`, and from the end of the jump the output equals a fresh plan of the new session at the
   same timeline sample: adding track 0 to a second existing VCA; adding that VCA as a member of `v`
   (nested); adding a new VCA (`UpsertVca` of a new ID) that reaches track 0; adding the first VCA to
   a session that had none; removing a VCA that reaches track 0.
5. **Classifier** (`crates/host-core/tests/live_delta.rs`, new). The move of gate 1 gives one
   offsets-cell write for track 0's lanes and one `FaderDb` per lane of track 1.
6. **Realtime** (`crates/control-plane/tests/vca_automation_realtime.rs`, new integration binary in control-plane, the crate that
   holds the shared commit, `control_plane::SessionState`, #1309 D1-D9. A host-core binary cannot
   reach the commit without a dev-dependency cycle, since control-plane depends on host-core.
   `scripts/check-bench-policy.sh:257-280` allows a `bench-support` dev-dependency only in a
   `crates/` manifest, so no host-web binary can link it. The binary links `bench_support::alloc`
   and calls `assert_installed()` first). It drives the script through control-plane's shared
   commit and host-core's render session, the code the browser Worker (#1382) and the C ABI both
   run. For gate 1's script on the render
   thread: `allocations == 0 && frees == 0` around every render call after warm-up.
7. **Commands:**
   - `cargo test --locked -p session` (`vca_composition.rs` as #1646 leaves it),
     `cargo test --locked -p host-core --features host-core/test-support`,
     `cargo test --locked -p capi`, `cargo test --locked -p host-web --features host-web/test-support`,
     `cargo test --locked -p builtins-compiler --features test-support`
   - `cargo test --locked -p control-plane --features test-support`
   - the workspace debug leg (`test-debug-a` in `.github/workflows/qualification.yml`)
   - `bash scripts/check-web-audioworklet.sh`, `bash scripts/test-web-audioworklet.sh`
   - `bash scripts/check-host-core-policy.sh`, `bash scripts/check-realtime-policy.sh`,
     `bash scripts/check-workspace-policy.sh`
   - `bash scripts/check-cross-targets.sh` (README F19, the iOS memset rule: no new
     `memset_pattern16` call; fix one in code, never by a ceiling)
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`
8. **No rendered bit moves** for any session without stored automation, VCA sessions included:
   `cargo build --locked --release -p audit -p capi && ./target/release/audit capi` gives the same
   `pcm_digest` at base and head (PR evidence), and the browser legs pass with unchanged digests.

## Test value

- Gate 1: red if a VCA move does not reach the offsets cell (the lane stays on the old
  composition), if the hosts differ, or if a non-automated member loses its record.
- Gate 2: red if render reads an older slot, the writer writes one word alone (a slot mixes the
  halves of two sums), or the classifier sums nested VCAs in another order than preparation.
- Gate 3: red if an unchanged offset still writes the cell or retargets the lane.
- Gate 4: red if any VCA edit on an automated lane rebuilds, if a lane no VCA reached has no cell to
  write when a VCA is added, or if a removed VCA leaves its offset in `S`.
- Gate 5: red if the classifier emits a `FaderDb` that the next curve event would contradict.
- Gate 6: red if the cell read or the composition allocates on render.
- The composition order and the no-VCA rule are #1646's gates 1 and 6.

## Dependencies

- #1646 *Prepare stored fader automation and render it flat* (the offsets cell it creates and
  seeds). It brings *Admit browser live edits in the Worker through the committed model* (#1382),
  which brings #1247's shared VCA row.
- #1648 *Classify fader automation edits as carried rebuilds* (the carry and the strip step's
  automated-lane rule).
- #1639 *Hold the automation jump lengths in a plan cell* (the `fader` jump length of the
  adoption jump). *Carry an optional per-edit ramp length on live session edits* (#1394), whose
  `resolve` gives the ramp word, arrives through #1382.
- Batch: R1, in one push with #1646, #1647 and #1648.


## Amendment (controls coordinator, 2026-10-14)

Checked against `12d4a4f9d`.

- **Composition** is `controls::effective` over `session::VcaSum` (#1646's amendment); D4's
  "#1247's strip step" is the classifier's effective diff (`EffectiveDiff`,
  `crates/host-core/src/live_delta.rs`, C6 step 3).
- **`S` includes the hand offset** (OQ1 = C, #1650 D6): `S = offset + Σ reaching VCA offsets`,
  the lane's own offset first. The cell, its write and the jump rule are unchanged; #1650 reuses them.
- **VCA automation** is no longer out of scope by decision 13: decision 17 computes `automatable`
  from the row, and #1566 accepts a VCA `offset_db` or `mute` target (the fixture holds `vca-ride`).
  No slice renders it yet (README F20); if root files one, it composes into this slice's `S`.
- **Moved files.** `crates/host-core/tests/vca_live.rs` is gone (use `crates/host-core/tests/vca.rs`);
  `hosts/host-web/src/tests.rs`'s VCA oracles were deleted (#1565): the browser leg goes through
  `hosts/host-web/tests/control_parity.rs`.
- **Must-land group** is #1646, #1647, #1648, #1649 and #1650.

## Amendment (controls coordinator, batch 10, 2026-10-14)

- **VCA automation renders in #1655** (root's F20 ruling, README "Root's rulings
  (2026-10-14)"). It extends this slice's offsets cell and write: automated VCA curves add to `S` at
  each event. This slice's non-goal stands for itself.

## Amendment (controls coordinator, batch 17 spec check, 2026-10-16)

Checked against `31fa33fee`. This amendment overrides the filed text and the earlier amendments
where they differ.

- **Anchors moved.** `vca_effective_db` `crates/session/src/vca.rs:26` and `VcaSum` `:37-66`
  (still member first until #1646 D6). `effective_strip_faders` is gone: `controls::effective`
  (`crates/controls/src/effective.rs:399`, per strip `effective_strip` `:311`) composes every
  effective fader. The order test is `crates/controls/tests/composition.rs`;
  `crates/session/tests/vca_composition.rs` is gone. #1247's "strip step" is
  `LevelDiff::level_records` (`crates/host-core/src/live_delta.rs:1015-1019`), implemented by
  `EffectiveDiff` (`crates/host-core/src/prepare.rs:4449-4540`, full path and strip join) and
  `LocalDiff` (`live_delta.rs:1085`, `:1246`, the fast path of #1612/#1613). The record length
  rule is `LevelRules::lengths` (`prepare.rs:4310`). The live-ramp row is
  `LiveRampRow::VcaOffset` (`crates/effect-contract/src/control.rs:77-78`); `VcaFader` does not
  exist. Fader cells: `FaderCells` `crates/builtins-compiler/src/lib.rs:298`, `FaderReader`
  `:396`, `FaderBankProcessor` `:1519`, `FaderMatrixBankProcessor` `:1831` (the fused bank also
  holds the fader). `scripts/check-bench-policy.sh`'s host ban is `:257-283`.
- **Landed deps and what they give.** #1247 is the effective diff (VCA rides, membership, nested
  VCAs, added and removed VCAs are live today; step 1 masks the VCAs whole). #1312: `LatestCell<N>`
  is generic (`crates/engine/src/realtime/latest_cell.rs:590`), so a three-word cell needs no
  realtime edit. #1309: the commit writes cells in `commit_live_once`
  (`crates/control-plane/src/control.rs:2905`), to the newest epoch's producers. #1639: the
  jump-length cell. #1562b: the browser commits the same way.
- **Departure 1: one length rule.** D4's ramp word is the lane's fader length from
  `LevelRules::lengths` (own edit: row `Fader`; else the first reaching VCA whose offset changed:
  row `VcaOffset` with that edit's length; else `VcaOffset` with no edit length). So an automated
  member and a plain member of one VCA edit take one length by construction.
- **Departure 2: both classifier paths.** The `S` comparison and the offsets-cell record go in the
  shared `level_records` (both `EffectiveDiff` and `LocalDiff`), in the branch #1648 adds for
  automated lanes. A fast-path-only or full-path-only rule would make the result depend on the
  edit's shape.
- **Departure 3: `S` has one author.** `S` per strip lane is computed in `controls::effective`
  (C7), beside the effective fader, so preparation (#1646), this slice's classifier and #1650's
  offset read one function. `EffectiveStrip` gains the lane's `S` (`f64`), written by
  `effective_into` with no allocation. If #1646 adds it, this slice only reads it.
- **Departure 4: a new record in the strip-level family.** `LiveStripRecords` gains the
  offsets-cell write (`S` and the ramp word per lane); `FaderCells` (or a sibling writer on the same
  lane) gains the three-word cell writer that #1646 creates. This stays in decision 17's strip
  level family; no family is added.
- **Departure 5: rebuilds.** A rebuild seeds `S` from the successor's model (#1646), and #1648's
  adoption rule compares the composed gain (its amendment, departure 4), so a VCA edit inside a
  structural transaction is heard at adoption. This slice adds nothing to the carry.
- **VCA automation** renders in #1655 (batch 10 amendment); the non-goal "out of scope by decision
  13" is superseded.
- **Authorized paths, as they now must read:**
  - `crates/host-core/src/live_delta.rs` (the `LiveStripRecords` field, `LocalDiff`'s automated-lane
    branch), `crates/host-core/src/prepare.rs` (`EffectiveDiff`'s automated-lane branch only)
  - `crates/controls/src/effective.rs` (the lane's `S`, only if #1646 did not add it),
    `crates/controls/tests/effective.rs`
  - `crates/control-plane/src/control.rs` (the offsets-cell write in `commit_live_once`)
  - `crates/builtins-compiler/src/lib.rs` (the offsets-cell writer, the fader and fused banks'
    dirty read at block entry and the jump)
  - `crates/host-core/tests/live_delta.rs`, `crates/host-core/tests/vca.rs`,
    `crates/capi/src/runtime/live_tests.rs`, `hosts/host-web/tests/control_parity.rs`,
    `crates/control-plane/tests/vca_automation_realtime.rs` (new)
  - `crates/control-plane/Cargo.toml` and `Cargo.lock` drop out: control-plane already has the
    `bench-support` dev-dependency (`crates/control-plane/Cargo.toml:29`).
  - No path under `crates/engine/src/realtime/`. The render read in builtins-compiler sits in
    realtime-policy regions; their count may move.
- **Gates' test files.** Gates 1-4: `crates/capi/src/runtime/live_tests.rs` and
  `hosts/host-web/tests/control_parity.rs` (not `hosts/host-web/src/tests.rs`); gate 5:
  `crates/host-core/tests/live_delta.rs`; gate 6: `crates/control-plane/tests/vca_automation_realtime.rs`.
  Commands: `cargo test --locked -p controls` replaces `-p session` for the order test;
  `bash scripts/check-realtime-policy.sh` is replaced by
  `cargo build --locked --release -p realtime-policy && ./target/release/realtime_policy`.
- **Size.** About 12-15 implementer hours: classifier branch on both paths and `S` 3 h, the record
  and the control-plane write 2 h, the cell writer, the dirty read and the jump on both bank kinds
  3-4 h, tests (both hosts, the nested-order case, realtime binary) 5 h. One attempt; no split
  (the classifier and the render halves are not useful apart).

## Rulings (root, batch 17, 2026-10-16)

No change to the decisions, paths or gates of this issue. Two notes, so a brief does not assume the
other options.

- **R17-1 = A and departure 2.** The automation entries that drive a lane are an owner of the
  in-place edit. `LocalDiff` (the fast path) reads the lane's entries through the same `ModelView`
  as the full path, so the automated-lane branch of `level_records` decides the same on both paths.
- **R17-6.** `S = offset + Σ reaching VCA offsets` is the fader's hand offset only. The trim and
  send offsets live in #1658 and #1654, which keep their own cells. This issue's `S` does not change
  and its offsets cell is the fader's.
