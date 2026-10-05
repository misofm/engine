# Classify fader automation edits as carried rebuilds

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answers A2, A10 and A1.8, under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`.

## Product outcome

On both hosts, a transaction that adds, changes or removes stored fader automation on a playing
engine takes the rebuild path. Both hosts run the shared commit of `crates/control-plane`: the C
ABI directly, the browser in its Worker (#1382, which lands before draft 09a). The successor plan holds the new automation program, every owner
carries, the fader lane continues from the value it has, and the revision completes `exact` at
adoption. The response reports `rebuild`. A transaction that changes only the static fader value
of a lane the session automates commits as `model_only`: that value is the lane's fallback, and no
plan renders it while the automation exists. A browser live fader command on such a lane is
lowered to that same transaction (#1382 D2) and meets the same rule: the reply reports
`model_only`, and the curve keeps playing. This is one permanent rule in the shared commit, for
both hosts (README A2). Every other automation row stays model-only and inert
until its own rendering slice.

Today every automation edit is "live with no records": it commits with no plan change, so after
drafts 09a and 09b render fader automation, an edit would be acked and never heard.

## Context

- **The mask.** `classify_live_delta` (`crates/host-core/src/live_delta.rs:211-215`) copies
  `current.automation` into the masked model (`:234-236`). Its documentation says the first issue
  that renders stored automation must drop that line (`:181-185`). The numbered steps are
  `:149-179`; `LiveRebuild` is `:117-141`. These anchors are those of `6ee64f484`. On `main`,
  *Refuse automation on effect parameters that are not block-rate* (#1335, commit `0c19119d0`) has
  since added `LiveRebuild::AutomationTarget` and inserted its check as step 4, so the per-track
  step is step 5. This draft's step numbers follow #1335's.
- **Fader records.** The per-track step (step 5; step 4 at `6ee64f484`) emits one `FaderDb` per lane
  whose gain bits change and one `Mute` per lane whose mute changes (`live_delta.rs:269-306`).
- **The edit path.** *Report each transaction's edit path in its response* (#1313) D2 reports
  `rebuild` for `Err(_)` and `model_only` for an empty delta.
- **The rebuild path.** `commit_live` returns `LiveCommit::Rebuild` when the classifier refuses
  (`crates/capi/src/runtime/control.rs:1065-1077`); the successor is prepared with
  `prepare_host_runtime_with_live_lanes_successor` (`crates/capi/src/runtime/compile.rs:593`).
  #1309 moves both into `crates/control-plane`.
- **What carries.** The plan inventory has one row list per carried family
  (`PlanStateInventory`, `crates/host-core/src/prepare.rs:555-630`); a successor joins it against
  `SuccessorBase` (`:641-647`) and installs a carry program into the graph (`:1791-1793`,
  `:1820-1821`). *Carry fader, mute and pan ramps across a plan swap* (#1277) adds the fader and
  matrix stages, with move mode at the swap block (#1277 D8).
- **Tests that pin today's rule.** The "automation upsert" case of `model_only_edits`
  (`crates/host-core/tests/live_delta.rs:530-545`, the ride at `:508-528`, the removal case at
  `:582-588`) and of `model_only_edits` in `crates/capi/src/runtime/live_tests.rs:1419-1470`
  (run by `model_only_edits_commit_without_a_plan_rebuild`, `:1481`) use a fader ride (row 5).
- **Docs that say automation edits are model-only:** `crates/capi/include/miso_engine_v1.h:41-43`,
  `docs/CONTROL_PROTOCOL_SEMANTICS.md:15`, `docs/C_ABI_V1_QUALIFICATION.md:268-274`.

## Decisions frozen for this slice

- **D1. The mask becomes per target row.**
  - One function in host-core, `automation_row_renders(target: &AutomationTarget) -> bool`, names
    the rows that render. In this slice it is true only for `rack = builtins`, `parameter_id = 5`.
    Draft 09a's preparation compiles cells only for the targets it accepts, so the classifier and
    preparation read one list. Each later rendering slice adds its row; draft 20 adds the last.
  - The masked JSON comparison keeps copying `current.automation` (`:234-236`). A new step, after
    step 4, #1335 D4's `AutomationTarget` check (which draft 02 D5 extends to the builtin rules),
    and before the per-track step, compares the entries of rendered rows in `current` and `next`,
    in table order, by canonical bytes. Any difference returns a new `LiveRebuild::Automation`.
    The numbered rules and the model-only paragraph (`:181-185`) are rewritten to say so.
- **D2. The rebuild carries every owner** (A1.8; D15-7).
  - **The stage.** This slice extends the carry predicate of #1277 D4 (which lands first): gaining,
    losing or changing stored automation is neither a prepared value nor a change of control kind,
    so the fader stage carries. #1277 itself is not amended.
  - **The cell.** A cell's whole event state (draft 07 D2: cursor, current target bits, in-flight
    ramp end, held grid, last seek boundary) carries by its stable address `(strip ID, parameter ID,
    lane)`. Draft 09a D1 keeps that state in the fader bank stage beside the lane's ramp state, so
    the fader stage's carry program under #1277 moves it, with the ramp state, at the swap block in
    move mode (#1277 D8). The stage's inventory rows gain the cell addresses, and the join pairs
    them by address. No second carry program exists for cell state.
  - **At adoption.** The successor repositions each carried cursor in its own table (draft 07 D4).
    If the new curve's value at the adoption block's first sample differs in bits from the carried
    current target, the cell takes a jump there, over the fader jump length.
  - **A cell gained.** A lane that gains automation on a carried stage starts its cell with the
    carried stage target as its current target, then applies the adoption rule above. It is not set
    exactly: the strip is not added (A1.4 "New lanes").
  - **A cell lost.** A lane whose entry the edit removes gets a release cell in the successor: its
    static effective value (the `next` fader composed with the VCA offsets, as preparation bakes
    it), no table. At adoption it takes the carried state; if the carried target differs from the
    static value, it jumps to it over the fader jump length. Then it is inert. No control-plane
    record is written: render alone knows the carried target.
- **D3. A static fader edit on an automated lane gives no record** (A2).
  - One predicate, `automated_cell(model: &SessionModel, address: CellAddress) -> bool`, built on
    the same enumeration draft 09a's preparation compiles cells from. The later slices reuse it.
  - The per-track step (step 5) emits no `FaderDb` for a lane that `next` automates. After D1,
    `current` automates the same lanes. Since *Deliver value-only VCA edits to the running C ABI
    plan* (#1247) D2 that step diffs the effective faders, so the rule covers a VCA move that
    reaches an automated lane too; draft 11 makes that move reach the lane's offsets cell. Mute
    records are unchanged. A transaction that changes only such a value yields an empty delta and
    commits `model_only` (#1313 D2); D2's release cell reads the value when the automation goes.
  - **One rule, both hosts.** The rule lives in the shared classifier, which both hosts' commits
    call (#1309; #1382 D2 for the browser). No host refuses such an edit and no host has its own
    admission for automated lanes. README A2 and finding F9 give why `model_only` is not
    "acknowledged with no effect" in #1315's sense: the edit sets the committed document's fallback
    value, the response names the path, and the value is rendered when the automation goes.
- **D4. Docs.** The header sentence (`miso_engine_v1.h:41-43`), `CONTROL_PROTOCOL_SEMANTICS.md:15`
  and `C_ABI_V1_QUALIFICATION.md:268-274` say: a fader automation edit rebuilds and carries; a
  static fader edit on an automated lane is model-only; the other rows stay model-only until they
  render.
- **D5. The acked-batch question: can an ack ever precede a drop? No.** A rebuild acks only after
  its successor is prepared, and the carry moves every value. No queue is added.

## Deliverables

1. D1 and D3 in `crates/host-core/src/live_delta.rs`, with the classifier tests.
2. D2: the inventory rows, join, release cells and carry-program move.
3. D4.

## Authorized paths

- `crates/host-core/src/live_delta.rs`, `crates/host-core/tests/live_delta.rs`
- `crates/host-core/src/prepare.rs` (the inventory rows, the join and the release cells only),
  `crates/host-core/tests/successor_swap.rs`
- `crates/graph/src/lib.rs`, `crates/graph/src/runtime.rs`, `crates/builtins-compiler/src/lib.rs`
  (the fader stage carry's move of cell event state only; stream A's files, sequenced by root)
- `crates/control-plane/src/` (the commit path; #1309 is on `main` through draft 09a's
  dependency), `crates/capi/src/runtime/live_tests.rs`, `hosts/host-web/src/tests.rs` (gate 5)
- `crates/capi/include/miso_engine_v1.h` (comment text), `docs/CONTROL_PROTOCOL_SEMANTICS.md`,
  `docs/C_ABI_V1_QUALIFICATION.md`

## Non-goals

- Rendering fader automation (drafts 09a and 09b).
  VCA offsets on an automated member (draft 11). The jump length cell (draft 12). Mute, pan, matrix,
  trim, polarity, filter and effect rows (drafts 13a to 20).
- Options B, C and D of OQ1. The browser's Worker commit itself (#1382, which lands before draft
  09a).

## Hazards

- **Superseded cases.** The "automation upsert" and removal cases named in Context now rebuild.
  Move them to a mute `step` entry (row 6 stays masked until draft 13a). Each later slice that
  unmasks that row moves them again; draft 20 deletes them.
- **Order of the two automation checks.** Draft 02 D5 routes a refusable builtin entry to
  `LiveRebuild::AutomationTarget`. D1's check runs after it, so a refusable fader entry keeps that
  variant and its refusal; gate 1 holds the order.
- **A redundant jump moves bits.** The adoption rule compares bits; a jump to the same target
  re-enters the ramp kernel. Gate 3 holds it.

## Objective gates

1. **Classifier** (`crates/host-core/tests/live_delta.rs`, new tests). Adding, changing or removing
   a fader entry gives `Err(LiveRebuild::Automation)`. A mute (row 6) entry change gives
   `Ok` with no records. A static `left_db` change on a lane a `left` fader entry automates gives
   `Ok` with no records; the same change on the right lane gives one `FaderDb` for `Right`. A
   static change together with an automation change gives `Automation`. A fader entry in unit
   `linear` gives `AutomationTarget` (draft 02 D5), not `Automation`.
2. **A changed curve rebuilds and carries** (`crates/capi/src/runtime/live_tests.rs`, new). A
   playing `long_session` with a linear fader ride on track 0 takes an `UpsertAutomation` that
   changes the ride. The response path is `rebuild`; the watermark publishes `EXACT` at adoption
   (#1314); no source is reseeked. From the completion sample of the first grid ramp that starts
   after the adoption jump ends, every block is bit-identical to a fresh plan of the new session,
   session-seeked (draft 05) to the adoption block's timeline sample and fed the same PCM.
3. **An unchanged curve gives no jump** (same file). A transaction that adds a muted track and keeps
   the ride: every block from the swap is bit-identical to the uninterrupted render of the old
   session.
4. **Removal** (same file). Removing the ride: from the end of the release jump, every block equals
   a fresh plan of the new session (static fader) from the same timeline sample.
5. **Static edit, both hosts** (same file; `hosts/host-web/src/tests.rs`, new). A `left_db` change
   on the automated lane: path `model_only`, the same epoch, every block bit-identical to the
   uninterrupted render. In the browser, a live fader command (kind 3) on the automated lane,
   lowered by the Worker's commit, replies `path: "model_only"` with the revision advanced by 1,
   and every block is bit-identical to the uninterrupted render; the same command on the other
   lane replies `live`.
6. **Realtime.** The swap block allocates and frees nothing (`bench_support::alloc` thread counters
   in `crates/host-core/tests/successor_swap.rs`, statics warmed).
7. **Commands:**
   - `cargo test --locked -p host-core --features host-core/test-support --test live_delta`,
     `cargo test --locked -p host-core --features host-core/test-support --test successor_swap`,
     `cargo test --locked -p capi`, `cargo test --locked -p graph --features test-support`,
     `cargo test --locked -p host-web --features host-web/test-support`
   - the workspace debug leg (`test-debug-a` in `.github/workflows/qualification.yml`)
   - `cargo build --locked --release -p audit -p capi && ./target/release/audit capi` (all
     violation counts 0)
   - `bash scripts/check-capi-abi.sh`, `bash scripts/check-host-core-policy.sh`,
     `bash scripts/check-graph-policy.sh`, `bash scripts/check-realtime-policy.sh`,
     `bash scripts/check-workspace-policy.sh`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`
8. **No rendered bit moves** for a session with no stored automation: `audit capi` reports the same
   `pcm_digest` at base and head (PR evidence), and the browser legs pass with unchanged digests.

## Test value

- Gate 1: red if the mask still hides fader entries, hides nothing (a mute entry would rebuild), or
  if a static edit on an automated lane still emits a record that fights the curve, or if D1's check
  runs before the refusal route (a refusable entry would report `Automation`).
- Gate 2: red if the cell does not carry (the lane restarts), the cursor is not repositioned, or
  the adoption jump is missing or has the wrong length.
- Gate 3: red if adoption emits a jump to an unchanged target.
- Gate 4: red if a removed entry leaves the lane on its last automated value.
- Gate 5: red if a static edit on an automated lane rebuilds or retargets, or if either host has a
  path that writes a live record onto an automated lane.
- Gate 6: red if the carry of cell event state allocates or frees on the swap block.

## Dependencies

- Draft 09b *Render moving stored fader automation, seeks and latency* (same batch). It brings
  draft 09a's cells and event state, draft 07's reposition, draft 05's session seek (gate 2's
  reference) and draft 02's D5 route (which runs first).
- *Carry fader, mute and pan ramps across a plan swap* (#1277): the stage carry D2 extends.
- *Report each transaction's edit path in its response* (#1313).
- *Publish an applied-revision watermark and complete edits asynchronously* (#1314), for `EXACT`.
- *Refuse automation on effect parameters that are not block-rate* (#1335): its step 4 and
  `LiveRebuild::AutomationTarget`.
- Batch: R1, with drafts 09a and 09b. *Admit browser live edits in the Worker through the
  committed model* (#1382) arrives through draft 09a.
