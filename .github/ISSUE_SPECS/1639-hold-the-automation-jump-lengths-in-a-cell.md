# Hold the automation jump lengths in a plan cell

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answers A3 and A10, under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`.

## Product outcome

A host that changes a playing session's `control_smoothing` hears the new ramp lengths on the next
automation jump, with no plan rebuild. On a session with stored automation the edit's response
reports `live`; on a session without it, `model_only`, as today's rule for that edit says. A jump
already in flight keeps the length it started with.

## Context

- **Jump lengths** (README A3 and A1.4): a jump of a fader or trim cell ramps over the
  session's `fader_ms`, of a mute or polarity cell over `mute_ms`, of a pan or matrix cell over
  `pan_ms`, each converted to samples by `SessionModel::control_smoothing_samples()` (*Session
  `controlSmoothing`: configurable ramp lengths for live mute, fader and pan changes*, #1054 D2-D3).
  HPF, LPF and EQ targets use their fixed 64-sample ramp; other effect parameters their descriptor's
  `smoothing_samples`. A session `explicit` 0 makes a jump a step.
- **No such field at HEAD.** `SessionModel` has no `control_smoothing` yet, and
  `LiveRamps::for_session` returns 0 for every row (`crates/host-core/src/live_delta.rs:33-58`).
  #1054 adds both. This slice is written against #1054.
- **The edit.** *Edit control_smoothing by a session transaction, model-only* (#1365) adds opcode
  `0x0008` and masks `control_smoothing` in the classifier (#1365 D4), so its delta is empty and its
  path `model_only` (#1313 D2). #1365 lands before this slice and is not amended: this slice
  makes the path `live` when stored automation reads the lengths (D3).
- **The cell primitive.** #1312 D1: a triple-buffered latest-target cell with one writer (control)
  and one reader (render); a write that completes before a block's drain is applied in that block,
  and render never skips the newest completed write.
- **The block entry.** The graph's render begins each block with the source set's `begin_block`
  (`crates/graph/src/lib.rs:3254-3256`).
- **#1637 D2** gives each cell program a jump-length key, never a copied `J`: a builtin row's key
  names a word of this slice's cell (`Fader`, `Mute`, `Pan`); filter and effect cells keep a fixed
  length. This slice lands before #1646, the first program that reads the cell.

## Decisions frozen for this slice

- **D1. One plan-level cell.** Every host-core plan owns one #1312 latest-target cell of three
  words: `mute`, `fader`, `pan`, in samples, the value of `control_smoothing_samples()`.
  Preparation writes it on both hosts. A builtin cell program reads the word its row names (fader
  and trim: `fader`; mute and polarity: `mute`; pan and matrix: `pan`; #1637 D2's key). Filter,
  EQ and effect cells keep their fixed length. The cell's bytes are fixed and do not depend on the
  session.
- **D2. One reader.** The cell has one reader, so render reads it once per block: the graph reads it
  at block entry, beside the source set's `begin_block`, and holds the three values for that block.
  #1647 passes them, with the node-time mapping, in the block context it gives every strip
  stage. A jump reads its row's value when it starts. A ramp in flight keeps its length. No stage
  reads the cell itself.
- **D3. The edit path.** In `classify_live_delta`:
  - when `control_smoothing_samples()` differs between `current` and `next`, and `next` has at least
    one automation entry on a row whose jumps read a word of D1 (builtin rows 1, 2, 5, 6, 7-10 and
    12), the delta carries one `jump_lengths: Option<[u32; 3]>` value. `LiveDelta::is_empty` reads
    it, so the path is `live`. The test reads the session only, not which rows render yet: the cell
    is in every plan, so the write always lands, and a row whose rendering slice is still to come
    reads the new value from its first jump;
  - otherwise nothing changes: the delta stays empty and the path `model_only` (#1365 D4);
  - equal sample counts give no value, even when the milliseconds differ (never a redundant write).
  The commit writes the cell after every fallible check. A write to the newest candidate follows
  #1053 D7, as every live write does.
- **D4. Swaps.** A successor is prepared from the committed model, which holds the edit, so its cell
  starts with the new lengths. Nothing carries.
- **D5. The browser.** Preparation fills the cell, through the same host-core preparation on both
  hosts. D3 lives in the shared classifier, which the browser's Worker commit calls after *Admit
  browser live edits in the Worker through the committed model* (#1382). So a browser transaction
  that edits `control_smoothing` meets the same rule as the C ABI's, from when the browser's apply
  export (*Export transaction apply and anchored seek from the browser engine module*, #1293, before
  #1643 in batch R1) can carry one. This slice adds no browser path and no browser gate.
- **D6. #1365.** D3 is the `live` classification of #1365's `control_smoothing` edit. This slice
  depends on #1365, which lands first, and changes the classifier code #1365 adds; #1365 is not
  amended.
- **D7. Docs.** The `controlSmoothing` paragraph that #1054 adds to `docs/SESSION_SCHEMA_V1.md`
  gains one sentence: the same row lengths are the jump ramps of stored automation (README A3).
  #1054 is not amended.
- **D8. The acked-batch question: can an ack ever precede a drop? No.** The cell write cannot fail,
  every check precedes it, and a replaced value is in the committed model.

## Deliverables

1. D1 and D2: the cell, preparation's seed and the graph's block-entry read. The strip stages' reads
   come with their rendering slices (#1647 for the fader; #1651, #1657 and #1658 for their
   words).
2. D3 in the classifier and the commit path; D7.

## Authorized paths

- `crates/host-core/src/prepare.rs` (the seed only), `crates/host-core/src/live_delta.rs`,
  `crates/host-core/tests/live_delta.rs`
- `crates/graph/src/lib.rs` (the block-entry read and its `test-support` reader only; stream A's
  file, sequenced by root)
- `crates/control-plane/src/` (the commit path), `crates/capi/src/runtime/live_tests.rs`
- `docs/SESSION_SCHEMA_V1.md` (D7's sentence only)
- Tests that pin a plan byte row the cell moves (re-pin each with the reason "the jump-length cell,
  #1639 D1", never in bulk)
- `crates/capi/Cargo.toml` (the `graph` dev-dependency's `test-support` feature, as #1642 adds
  it; only if #1642 has not landed), `Cargo.lock`

## Non-goals

- The `control_smoothing` grammar, defaults and rounding (#1054). The edit's opcode (#1365). A
  per-edit ramp length (#1394): stored automation has no per-edit length.
- Changing a ramp already in flight.

## Hazards

- **One reader only.** A second reader of a #1312 cell breaks its triple buffer. The reader half is
  one owned value, moved into the graph at preparation and not `Clone`, so a second reader cannot be
  written.
- **A transaction that changes `control_smoothing` and a rendered automated row together** is a
  rebuild once #1648 classifies that row (its D1); the successor is prepared with the new
  lengths (D4), so D3's write is not needed there.

## Objective gates

1. **Block-entry read** (`crates/graph` test, new, through a `test-support` reader of the block's
   held values). A fresh plan holds the prepared lengths at its first block. A write of new lengths
   before a block entry is held from that block on, not before; two writes before one block give
   the last. The audible effect on a jump is #1647's gate 2.
2. **Classifier** (`crates/host-core/tests/live_delta.rs`, new). With a fader entry, and with a mute
   entry: a `control_smoothing` change gives a live delta with `jump_lengths` and no other record.
   Without stored automation, or with only an effect entry: an empty delta. A change of `fader_ms`
   from 10.0 to 10.001 at 48 kHz (both 480 samples): an empty delta.
3. **C ABI** (`crates/capi/src/runtime/live_tests.rs`, new). A playing session with a fader entry takes a `SetControlSmoothing`
   edit: the response path is `live`, the provider epoch is unchanged, and the plan's cell holds the
   new lengths from the next block (gate 1's reader). The same edit on a session without automation
   reports `model_only`.
4. **Realtime.** The render block that reads a written cell allocates and frees nothing
   (`bench_support::alloc` thread counters, statics warmed), and
   `cargo build --locked --release -p audit -p capi && ./target/release/audit capi` reports all
   violation counts 0.
5. **Commands:**
   - `cargo test --locked -p host-core --features host-core/test-support --test live_delta`,
     `cargo test --locked -p builtins-compiler --features test-support`,
     `cargo test --locked -p graph --features test-support`, `cargo test --locked -p capi`
   - the workspace debug leg (`test-debug-a` in `.github/workflows/qualification.yml`)
   - `bash scripts/check-realtime-policy.sh`, `bash scripts/check-graph-policy.sh`,
     `bash scripts/check-host-core-policy.sh`, `bash scripts/check-workspace-policy.sh`
   - `bash scripts/check-cross-targets.sh` (README F19, the iOS memset rule: no new
     `memset_pattern16` call; fix one in code, never by a ceiling)
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`
6. **No rendered bit moves** for a session with no stored automation: `audit capi` gives the same
   `pcm_digest` at base and head (PR evidence), and the browser legs of the `browser` job pass with
   unchanged digests.

## Test value

- Gate 1: red if the graph reads the cell only at preparation, under no snapshot, or reads an
  older write, or if the cell's allocations drift from its retained row.
- Gate 2: red if the edit is `live` with no entry that reads a word (a needless write), `model_only`
  with one (the new lengths never reach render), or if equal sample counts still write.
- Gate 3: red if the commit does not write the cell, or reports the path from anything but the
  classifier's value; with a structural edit beside it (D4), red if the successor's cell is not
  seeded from the committed model.
- Gate 4: red if the block-entry read or the hand-off to the stages allocates on render.

## Dependencies

- *Session `controlSmoothing`: configurable ramp lengths for live mute, fader and pan changes*
  (#1054).
- *Hold live values in latest-target cells on both hosts* (#1312).
- *Edit control_smoothing by a session transaction, model-only* (#1365), for D3. It brings the
  edit's `model_only` path from *Report each transaction's edit path in its response* (#1313).
- *Admit browser live edits in the Worker through the committed model* (#1382), for D5.
- Batch: R1. #1646 builds on it; no dependency on #1648.

## Amendment (controls coordinator, 2026-10-14)

Checked against `12d4a4f9d`.

- **The keys are setting rows.** `control_smoothing`'s keys are the session rows `fader_ms`,
  `mute_ms` and `pan_ms`, family `Setting` (`crates/controls/src/session_rows.rs:38-75`), set by
  `SET_CONTROL`; `SetControlSmoothing` (`0x0008`) is retired (#1561;
  `crates/protocol/src/model.rs:2281-2285`). A setting change gives no record and sets
  `LiveDelta::settings_changed`. D3's "edit path" is that row change; the `live` classification
  rule of D3 stands.
- **The cell holds the three key lengths**, the values of `controls::LiveRamps`
  (`crates/controls/src/live_ramps.rs:14-24`); a stage resolves a row's length with
  `LiveRamps::for_row` (polarity is twice `mute_ms`; sends and VCAs have their own rows), as #1637
  (amended) says. D1's row-to-word list is replaced by that function.
- **Context.** #1054 and #1365 have landed; `LiveRamps` moved from host-core to `controls`.
- **The browser** is the same `SessionState` (#1562a); D5 holds by construction.

## Decision record: #1639 (batch 13b)

**Shipped.** `crates/graph`: `GraphJumpLengths { fader_samples, mute_samples, pan_samples }` (the
three `LiveRamps` key values), one `LatestCell<3>` per plan with its dirty word in one `Arc`, the
boxed one reader in the executor (`JumpLengthReader`, not `Clone`, built only by
`graph::attach_jump_lengths`), the writer `GraphJumpLengthProducer::write(stamp, lengths)`, and
`jump_length_retained_bytes` / `jump_length_largest_allocation_bytes`. `render` reads the cell once
per block right after it takes the block's live snapshot, beside the source set's `begin_block`
(`read_counted`, so a replaced write joins `live_values_superseded`), and holds the three values for
the block. The test-support reader is `graph::test_only_block_jump_lengths()` (a thread-local the
block entry sets). `crates/host-core`: `session_jump_lengths(model)` (from
`LiveRamps::for_session`), preparation attaches and seeds the cell on every plan after bind and
charges it in the graph row, `HostLiveControlHandles::jump_lengths`; the classifier's
`LiveDelta::jump_lengths: Option<GraphJumpLengths>` (read by `is_empty`) and
`automation_jump_row(target)`. `crates/control-plane`: the epoch's `StripLanes` holds the writer (preparation refuses a plan
without one, review fold 1); `commit_live_once` writes the value with the records, under the stamp,
before the revision's publication. D7's sentence is in `docs/SESSION_SCHEMA_V1.md`.

**Departures and choices (code truth).**

1. **Attach after bind, not in `PreparedGraphPlan`.** The cell is attached to the bound plan by
   `graph::attach_jump_lengths(&mut PreparedRenderPlan, ...)` (a downcast, as the carry installers
   do), so no builtins-compiler pass-through and no change to `bind` were needed. The seed is the
   reader's initial held value, not a cell write: a fresh plan holds the prepared lengths at its
   first block with no stamp at preparation.
2. **D3's row set** is `automation_jump_row`: a builtin, strip-own, route or VCA row whose live
   motion is `Motion::Ramp(row)` (`controls::row_motion`), plus an effect's bypass row
   (`BYPASS_CONTROL` -> `LiveRampRow::BypassCrossfade`, the `mute_ms` crossfade; its descriptor's
   motion is `Fixed` because it takes no per-edit length). No hard-coded row-number list. Filters,
   the delay, effect parameters and the (rebuild-only) link mode read no length.
3. **The count is charged on every plan that made its own.** The cell's reader holds a clone of
   `live_values_superseded` in every plan, so the graph row now charges the preparation's own count
   whenever the host passed none (before: only with strip controls).
4. **The fast classifier** (`classify_rows`) never sees a `control_smoothing` change
   (`changed_rows` refuses session rows, so the full classifier runs); it leaves `jump_lengths`
   `None`, with a comment.
5. **Known gap, by the spec's design:** a `control_smoothing` change committed while the session has
   no reading entry is not written; an entry added later (model-only until #1648 classifies
   automation edits) reads the cell's older lengths. The hazard section's rebuild rule (#1648 D1)
   closes it for rendered rows.

**Re-pinned rows** (each "the jump-length cell, #1639 D1"; measured: the cell's two allocations are
168 bytes, a count 24 bytes, the revision gate 72 bytes; since the review fold gate 1 measures the
cell's bytes and its two allocations against `jump_length_retained_bytes()` with the thread
counters):

| Test | Row | Before | After |
|---|---|---|---|
| `host-core` `tests/prepare.rs` `the_graph_row_counts_the_plans_gate_and_a_count_it_made_and_holds` | plain preparation (no strip controls), graph row less incremental | 72 (gate) | 264 (gate + cell + its own count) |
| same | strip controls, the preparation's own count | 96 | 264 |
| same | strip controls, the session's count | 72 | 240 |
| `host-core` `tests/live_routes.rs` `live_send_lanes_are_charged_against_the_graph_cap` | live total less idle total | route lanes + 24 (count) | route lanes (both plans hold the cell and their count); the test pins no total; live total 118304 at head (read from a failing run), 168 bytes (one cell) above the base it derives from |
| `host-web` `tests/retained_ceilings.rs` `the_graph_row_charges_the_plans_gate_and_not_the_sessions_superseded_count` | browser graph row less incremental | 72 | 240 |

`capi` `model_only_edits_commit_without_a_plan_rebuild`: the `control_smoothing` edit now comes
before the automation upsert (on a session with a fader entry it is `live` by D3); not a byte row.

**Gates.**

| Gate | Test | Result |
|---|---|---|
| 1 Block-entry read | `graph` `tests/jump_lengths.rs` `the_block_entry_holds_the_newest_published_lengths` (prepared lengths at block 0; an unpublished write not held; held from the first block after publication; two writes give the last and count one superseded; a second attach refused; the attach leaves exactly `jump_length_retained_bytes()` live in 2 allocations, 0 frees). The probe is set right after the block's snapshot, so the test does not show where in the block the read is | pass |
| 2 Classifier | `host-core` `tests/live_delta.rs` `a_control_smoothing_change_writes_the_jump_lengths_only_for_automation_that_reads_them` (fader, mute, pan entries live; none, EQ parameter entry and 10.0 -> 10.001 ms empty; effect bypass entry live), `the_rows_that_read_a_jump_length_are_the_ramped_rows` | pass |
| 3 C ABI | `capi` `runtime::live_tests::a_control_smoothing_edit_on_an_automated_session_writes_the_jump_lengths_live` (SET_CONTROL of the three smoothing rows: `live`, same epoch, no candidate, next block holds 96/48/144; without automation `model_only`, prepared 960/480/960 held). Red with the commit's write removed. No candidate is pending, so it does not show which plan a write with a candidate reaches. D4: `a_successor_prepared_with_a_control_smoothing_edit_holds_the_new_lengths` (an added track and the three rows in one transaction: `rebuild`, one candidate; after adoption, a new epoch, the next block holds 96/48/144). Red with the seed made a constant 960/480/960: the successor held 960/480/960. | pass |
| 4 Realtime | gates 1 and 3 count 0 allocations and 0 frees in the block that applies the write; `audit capi`: every violation count 0 | pass |
| 5 Commands | `graph --features test-support`, `host-core` (all features), `capi`, `control-plane`, `host-web` suites; the `test-debug-a` workspace leg (2228 passed, 0 failed); `realtime_policy` (149 regions in 32 files; floor 147 -> 149 for this issue's two regions), graph, host-core and workspace policy scripts; `check-cross-targets.sh` (PASS, iOS memset only the known #1018 builtins/lane calls); workspace clippy `-D warnings`; `fmt --check` | pass |
| 6 No bit moves | `audit capi` `pcm_digest` `bf7dcedd1d92621f` at base and head. Browser legs: run by the coordinator | native pass |

**Review fold (batch 13b).** Five minor findings of the adversarial review, folded:

1. **The invariant is typed.** The epoch's `StripLanes::jump_lengths` holds the writer, not an
   `Option`: `prepare_runtime` refuses a handle set without it with the cell's own attach failure
   (`host.jump_lengths.attach`, stage `Graph`), and `commit_live_once` lost its unreachable
   `Internal` refusal. `HostLiveControlHandles::jump_lengths` stays an `Option` (handle sets built
   without a preparation) and converts at the control-plane boundary.
2. **The cell's bytes are measured.** Gate 1 counts the attach with the thread counters (statics
   warmed on a throwaway plan): requested less released bytes equal
   `jump_length_retained_bytes()`, in 2 allocations and 0 frees. The counters carry no largest
   allocation, so `jump_length_largest_allocation_bytes()` is not measured.
3. **Two claims removed.** Gate 1 no longer claims "a block late": its probe is set right after the
   block's snapshot. Gate 3 no longer claims "a plan that is not the newest": that test has no
   candidate.
4. **D4 has a C ABI test** (gate 3's row): `a_successor_prepared_with_a_control_smoothing_edit_holds_the_new_lengths`,
   red with the seed made a constant.
5. **`session_jump_lengths`** moved out of the `use` block in `crates/host-core/src/prepare.rs`.
   `prepare.rs` also moves the graph row's charge (the superseded count charged when the
   preparation made its own), the largest-allocation row and a `HostLiveControlHandles` field,
   past the authorized "the seed only": all three are needed for D1 and are covered by the root
   ruling on byte rows.
