# Check realtime regions with a Rust syntax-tree tool: put every non-test pop in a marked region or on the control-side allowlist

Stream J follow-up of decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-0, D15-2).
This is slice B2b-1 of nine (C1 (#1445), A1 (#1438), A2 (#1439), B1a (#1440), B1b (#1441), B2a (#1442), C2 (#1446), B2b-1, B2b-2 (#1444); A1's spec lists what
each holds).
- B1a and B1b ported `main`'s drain rule; B2a refused pops in closures, macros and function values.
- **B2b-1 (this issue)** closes the limit "a pop outside every region" (root rulings R2 and R4,
  2026-10-05):
  - every non-test `try_pop` under the scan roots must sit in a marked region or in a function on
    an exact control-side allowlist;
  - every `try_pop` token must be classified by the syntax walk (the census);
  - the render-thread drains that sit outside every region are marked here, except
    `poll_meters`, which the commit "H #1448's guard, landed by J after C2" marks as this slice's
    first commit (root ruling 2 on the fourth review, option (C); its spec is the section
    "#1448 guard" below).
- B2b-2 then adds the call rule (root ruling R3). This slice also makes the one restructure that
  the call rule needs (D6), so that every line that moves in `crates/builtins-compiler/src/lib.rs`
  moves in this one marker commit.
- It waits for three production issues, which bound the render-thread drains first (root rulings
  R1 and R2), and it comes after C2 and the #1448 guard commit in the J batch (Dependencies).

Production changes: region markers in three files, and one restructure of test-support code (D6).

## Problem (verified on the stream B batch-1 tree)

The tree is `codex/d15-stream-b` at `b8392df66` (stream B batch 1: #1309, #1343, #1314, #1311,
#1348) merged with `origin/main` at `6d28a80ec` (`git merge-tree --write-tree`, `a330d420a`). The
two do not conflict in any `.rs` file. The merge's two conflicts are in
`.github/workflows/qualification.yml` (`test-debug-a`) and `scripts/test-test-support-ci.py`; no
line cited here is in them, and the scans read only `.rs` files, so either resolution gives the
same counts. Every line cited below is the same on `b8392df66` and on that merge, except
`crates/builtins-compiler/src/lib.rs`'s test module, whose lines are `main`'s (`6d28a80ec` moved
them), and `crates/graph/src/lib.rs`, cited below with both trees' lines.

- **Pops outside every region.** A pop in a helper outside any region is a documented limit
  (`scripts/check-realtime-policy.sh:133-136` on `b8392df66`; `:131-134` on `main`). Measured
  with a scratch `syn` program outside the repository (the D1 definition of test code): the tree
  has 93 marked regions in 26 files, and 39 non-test `try_pop` sites outside every region, in 27
  functions.
  - **13 functions (14 sites) run on a render thread.** Each gets a marker here, in B1b, or (for
    `poll_meters`) in the #1448 guard commit:
    - `crates/builtins-compiler/src/lib.rs`: `BuiltinBankProcessor::drain_controls` (`:474`; B1b
      marks it); and three test-support drains under `#[cfg(any(test, feature = "test-support"))]`:
      `GraphRuntimeProcessor for LiveControlInputProcessor::process` (`:4254`),
      `LiveControlMatrixProcessor::drain_controls` (`:4353`) and
      `LiveControlFaderProcessor::drain_controls` (`:4423`);
    - `crates/source/src/lib.rs`: `PcmSourceConsumer::observe_seek_at_block_boundary` (`:1324`),
      `PcmSourceConsumer::acquire_current_block` (`:1394`) and
      `PcmSourceProducer::take_recycled_block` (`:813`; the browser runs it on the worklet's audio
      thread, from `receiveSource`, root ruling R2);
    - `hosts/host-web/src/lib.rs`: `AudioWorkletEngineHost::poll_meters` (`:3466`);
    - `crates/host-core/src/spectrum.rs`: `SpectrumCapture::cancel` (`:510`), `try_read_record`
      (`:535`, `:546`), `commit_continuous` (`:637`), `stop_continuous` (`:671`) and
      `try_read_continuous_record` (`:761`) (worklet message handlers, root ruling R2).
  - **14 functions (25 sites) run on a control, teardown or harness thread.** They are the day-one
    allowlist (D7). Stream B batch 1 removed one entry that `main` still has:
    `impl Drop for RealtimePlanOwner` no longer exists, and `RealtimePlanOwner::enter_block` reads
    #1343's mailbox (`crates/engine/src/realtime/plan_exchange.rs:716-770`) and no longer pops.
- **Render loops that call a popping function, with no pop of their own.** D2 does not see them,
  because they hold no `try_pop`. Root ruled (2026-10-05) that each goes into the issue that owns
  its file before the tool lands:
  - `SpectrumCaptureCollection::cancel` (`crates/host-core/src/spectrum.rs:874-878`) and `select`
    (`:928-932`) call `SpectrumCapture::cancel` once per capture, on the worklet's audio thread.
    Stream H's *Bound every drain the AudioWorklet runs on its audio thread* moves the pop into the
    collection's loop (its D9). This slice marks the function that holds that loop (D6).
  - `LiveControlEffectBankStage::drain` (`crates/rack/src/lib.rs:1257-1284`, render) calls
    `EffectControlLane::stage` (which pops) inside `for lane in 0..lane_count` (`:1261`, call
    `:1268`). Stream B's #1345 rewrites both functions; its Amendment 1 makes that loop markable
    and marks it. Slice B2b-2 waits for it.
- **The render-thread drains fail the drain rule today.** Marked one function at a time on an
  export of `origin/main`, `main`'s gate refuses `acquire_current_block` (`:1394`), `poll_meters`
  (`:3466`) and the spectrum drains at `:510`, `:637`, `:671` and `:761`. It accepts
  `observe_seek_at_block_boundary`, `take_recycled_block`, `try_read_record` and the three
  `LiveControl*` drains. Root ruled (R1, R2) that production issues bound the refused drains in
  their owning streams, and that none of these sites goes on the allowlist:
  - stream B, *Bound the source acquire by the blocks queued at its entry, and observe a late seek
    outside the loop* (#1447);
  - stream H, *Bound the browser meter poll by each queue's count at entry*
    (#1448);
  - stream H, *Bound every drain the AudioWorklet runs on its audio thread*
    (#1449).

## Decisions

- **D1. Test code (R4).** A pop is test code, and is ignored by D2, if and only if one of these
  holds:
  1. Its file is under `<package>/tests/`, where `<package>` is a directory that holds a
     `Cargo.toml`, and no `#[path]` attribute in a file outside such a directory resolves to it.
  2. An enclosing item, statement, expression or match arm carries `#[test]`, or carries
     `#[cfg(P)]` where P is false under three-valued (Kleene) logic with `test` false, every
     `feature = ".."` true, and every other atom unknown. Several `#[cfg]` attributes on one node
     are joined with AND. `cfg_attr` is ignored, which fails closed. An inner `#![cfg(P)]` counts
     for its module or file.
  3. Every declaration that reaches its file meets rule 2. A declaration is a `mod x;` item,
     resolved by Rust's default lookup (`x.rs` or `x/mod.rs` beside a `lib.rs`, `main.rs` or
     `mod.rs`; under `<stem>/` beside any other file; through enclosing inline modules) or by its
     `#[path]` (relative to the declaring file's directory, or to the inline module's directory).

  A file that no declaration reaches is not test code. No module is exempt by its name.
  - So `#[cfg(any(test, feature = "test-support"))]` and `#[cfg(feature = "test-support")]` are not
    test code, `#[cfg(not(test))]` is not test code, and `#[cfg(test)]`, `#[cfg(all(test, loom))]`
    and `#[test]` are.
  - **Measured** on the batch-1 tree: 39 non-test sites outside regions, in 27 functions
    (Problem). The definition brings in two harness helpers that a name rule would have hidden,
    inside `#[cfg(any(test, feature = "test-support"))] mod tests`
    (`crates/builtins-compiler/src/lib.rs:5123-5125`): `render_post_input_bits_with_variant`
    (`:6487`, pop `:6597`) and `carry_drain_applies_every_record_past_a_refused_one` (`:12597`, pop
    `:12650`). Its other pops are in `#[test]` functions and stay out.
- **D2. Every non-test pop is checked.**
  - Every non-test pop under the scan roots (a method call, a path value or a macro token,
    B1a-D5) must be in a marked region or admitted by an allowlist entry (D3). Anything else is a
    finding: "unmarked try_pop outside the control-side allowlist".
  - **Token census.** Every `try_pop` identifier token that the lexer finds in any `.rs` file under
    the roots must be classified by the syntax pass: a `fn try_pop` definition, test code (D1),
    judged in a marked region, or admitted by the allowlist. Any other token is a finding: "try_pop
    token the syntax walk did not classify". This covers a pop in a `Verbatim` node or in an
    attribute's tokens outside marked files.
  - **Parse failures.** A `.rs` file that holds a `try_pop` token and that `syn::parse_file` cannot
    parse is a finding (A1-D3 widened).
- **D3. The control-side allowlist.**
  - It is a field of `Policy` (A1-D1). `Policy::workspace()` holds the D7 entries. A library case
    passes its own entries, so no case depends on the real tree's files. The binary's cases run
    through A1-D1's injectable entry point with the case policy, so they do not depend on the real
    allowlist either.
  - An entry has five fields: the file path, the canonical function path, the exact number of pop
    sites, the named thread, and a one-line reason.
  - **The canonical function path** is the path inside the file, with generics removed:
    `<fn>` for a free function; `<Type>::<fn>` for an inherent method; `<Trait> for <Type>::<fn>`
    for a trait method; with `<mod>::` prefixes for each enclosing inline module
    (`tests::render_post_input_bits_with_variant`).
  - **What an entry admits.** The pops in that function's own body, macro tokens included. Pops in
    a closure or a nested `fn` inside it are not admitted (none of the D7 sites is in one). Each
    entry admits exactly its site count: one more pop in a listed function is a finding.
  - **A stale entry** is a finding, "stale control-side allowlist entry", when its file is missing,
    when no function or more than one function matches its path, or when the function's non-test,
    unmarked pop count differs from the entry's count.
- **D4. The thread field.** It names the thread on every host that runs the function. A function
  that a browser message handler or `process()` runs is render-thread code (R2) and cannot be
  listed. Each reason states why the pop never runs inside a product render callback (the C ABI
  render call, the worklet's `process()` or a worklet message handler): it runs on a control
  thread, at teardown, or on a harness thread that renders and drains in turn. Copy the reason
  from the doc comment where one exists (for example `crates/protocol/src/queue.rs:797-800`).
- **D5. Documentation.** The drain rule's module documentation drops the "pop outside every region"
  limit and states D1, D2 and D3.
- **D6. Markers and the one restructure.**
  - **Mark each function below as a region.** Each marker replaces a blank line where one exists,
    so that no line moves. Adjacent functions with one blank line between them share a region.
    Find each by name; the line numbers are those on the batch-1 tree, before the production
    issues.
    - `crates/source/src/lib.rs`: `take_recycled_block` (blank lines `:799`, `:818`);
      `observe_seek_at_block_boundary` (`:1321`, `:1366`); `acquire_current_block` (`:1387`, and the
      blank line after the stream-B issue's `settle_block`, which that issue puts directly after
      the acquire, inside this region; today `:1430`).
    - `hosts/host-web/src/lib.rs`: not in D6. `poll_meters` is marked by the commit "H #1448's
      guard, landed by J after C2", this slice's first commit (section "#1448 guard").
    - `crates/builtins-compiler/src/lib.rs`, `impl BuiltinBankProcessor`, **only if** B1b skipped
      its D5 because a blank line was gone and `drain_controls` still pops (B1b-D5's no-fallback
      rule): put that `impl` in its own region here, with markers on new lines where the blank
      lines are gone. Record which slice marked it.
    - `crates/host-core/src/spectrum.rs`: `cancel` to `try_read_record` (`:507`, `:554`; the region
      also holds `try_read`, which does not pop); `commit_continuous` and `stop_continuous`
      (`:635`, `:678`); `try_read_continuous_record` (`:710`, `:779`); and the
      `SpectrumCaptureCollection` function that the stream-H issue's D9 writes to hold the
      collection's capture loop.
    - `crates/builtins-compiler/src/lib.rs`: the `LiveControlMatrixProcessor` and
      `LiveControlFaderProcessor` `impl` blocks that hold `drain_controls` (`:4348-4368` and
      `:4418-4450`, each with its `#[cfg]` line), and the new input `impl` block below. Only
      `:4451` is blank there; the other markers are new lines.
  - **The restructure (root's one-off decision, 2026-10-05).**
    `GraphRuntimeProcessor for LiveControlInputProcessor::process` pops directly
    (`crates/builtins-compiler/src/lib.rs:4251-4284`). That puts the name `process` in B2b-2's name
    set, and the call rule then refuses three real calls whose receivers cannot reach it. Move its
    drain (`:4252-4279`) into a new
    `LiveControlInputProcessor::drain_controls(&mut self) -> Result<(), RenderError>` in its own
    `#[cfg(any(test, feature = "test-support"))] impl LiveControlInputProcessor` block, as its two
    siblings have it (`:4348-4368`, `:4418-4450`), and call it once at the top of `process`.
    Nothing else changes. Mark that `impl` block.
    - **Receiver evidence, one row per refused call.** The verifier checks each row on the tree it
      reviews (find each by name; the lines are the batch-1 tree's):

      | Refused call | Receiver's declared type | Why it cannot reach the input drain |
      |---|---|---|
      | `crates/graph/src/runtime.rs:3393`, `delays[staged.line as usize].process(staged_left, staged_right)` in `execute_op` | `delays: &mut [CompensationDelay]`, a parameter of `execute_op` (`:3325`, `:3328`) | `CompensationDelay` is a struct (`:940`); `process` is its inherent method (`impl CompensationDelay`, `:969-970`). The call is static dispatch to that method, which does not pop. |
      | `crates/rack/src/lib.rs:2566`, `slot.stage.process(block)` | `slot` iterates `self.slots: Box<[PreparedSlot]>` (`:1874`); `PreparedSlot.stage: Box<dyn BankStage>` (`:1476-1479`) | `BankStage::process` (`:570-571`) is a different trait from `GraphRuntimeProcessor` (`crates/graph/src/lib.rs:2450` on `b8392df66`; `:2452` on `main` and on the merge tree). `LiveControlInputProcessor` implements only `GraphRuntimeProcessor` (`crates/builtins-compiler/src/lib.rs:4250`), so no `dyn BankStage` dispatches to it. |
      | `crates/rack/src/lib.rs:2592`, `slot.stage.process(BankBlock { .. })` | the same `slot` over `&mut self.slots[self.collapse_prefix..]` | as the row above |

    - The real dynamic call into the input drain is `processor.process(..)` at
      `crates/graph/src/runtime.rs:3472` (`NodeKind::Bound`). It is in no loop in `execute_op`, so
      the call rule never sees it; it is reached two calls deep from the unit loops. The
      restructure hides nothing that the rule can see.
    - This decision applies to these three calls only. It is not a general remedy (B2b-2-D3).
    - If #1346 has removed the input ring when this lands, the restructure is void: record it.
  - **Floors.** Raise the region floor in `Policy::workspace()` to the re-measured count. C2 has
    deleted the awk gate and its self-test, so no other floor exists. On the batch-1 tree plus B1b,
    the production issues and the #1448 guard commit: 93, plus B1b's one region (or this slice's,
    under the conditional item above), plus the guard's one, plus this slice's ten (three in
    `crates/source`, four in `spectrum.rs`, and the matrix, fader and input blocks in
    `builtins-compiler`), so 105, plus any region #1345 Amendment 1 added if it has landed; count
    them on the tree. Every file is already marked, so the file floor does not change (26);
    re-measure it.
  - If a newly marked function fails any rule of the tool, stop and report the rule and the
    line. Do not change the function; its owning stream does.
- **D7. The day-one allowlist** (R4's measurement on the batch-1 tree, after R2 removes the seven
  audio-thread sites): 14 functions, 25 sites. Re-measure them at implementation time.

  | File | Function | Sites | Thread |
  |---|---|---|---|
  | `crates/protocol/src/queue.rs` | `ProtocolQueues::try_dequeue_automation` | 1 (`:802`) | C ABI control thread; not built into the browser module |
  | `crates/protocol/src/queue.rs` | `ProtocolQueues::try_dequeue_event` | 1 (`:856`) | as above |
  | `crates/protocol/src/queue.rs` | `ProtocolQueues::try_dequeue_telemetry` | 1 (`:1031`) | as above |
  | `crates/protocol/src/queue.rs` | `ProtocolQueues::try_dequeue_counter_telemetry` | 1 (`:1036`) | as above |
  | `crates/engine/src/realtime/plan_exchange.rs` | `PlanRetirer::try_reclaim` | 1 (`:841`) | the control/retirement owner (its doc comment, `:838-839`) |
  | `crates/engine/src/realtime/plan_exchange.rs` | `Drop for PlanRetirer::drop` | 1 (`:856`) | teardown of the retirement owner (its doc comment, `:853-854`) |
  | `tools/console-workload/src/lib.rs` | `SessionRuntime::drain_meter_snapshots` | 1 (`:1906`) | benchmark driver; between blocks, on the thread that renders |
  | `tools/audit/src/builtins_graph.rs` | `run_retirement_worker` | 1 (`:169`) | audit harness worker |
  | `tools/audit/src/builtins_graph.rs` | `drain_values` | 2 (`:462`, `:465`, a macro token) | audit harness, between blocks |
  | `tools/audit/src/builtins_graph.rs` | `drain_fixture` | 2 (`:480`, `:488`, a macro token) | audit harness, between blocks |
  | `tools/audit/src/fixture_builtins.rs` | `graph_tap_fixtures` | 1 (`:552`) | audit harness |
  | `tools/audit/src/fixture_builtins.rs` | `meters` | 10 (`:1259`-`:1349`) | audit harness, between blocks |
  | `crates/builtins-compiler/src/lib.rs` | `tests::render_post_input_bits_with_variant` | 1 (`:6597`) | test harness (test-support builds) |
  | `crates/builtins-compiler/src/lib.rs` | `tests::carry_drain_applies_every_record_past_a_refused_one` | 1 (`:12650`, a macro token) | test harness (test-support builds) |

- **D8. Class sets that this slice changes in earlier cases.** With markers gone or dropped, the
  pops in these trees are outside every region:
  - A1's `open-tail-valid`, `marked-file-count-floor` and `no-marked-files-uses-floor`, and B1a's
    `open_tail_last_file`, each gain {unmarked try_pop outside the control-side allowlist}.
  - No other committed case changes. The base tree has no pop outside a region (measured).
- **D9. New cases.** One each, in A1's case format:
  - **Test code (D1):**
    - a `#[cfg(test)]` function and a `#[cfg(all(test, loom))]` block with unmarked pops (ignored);
    - a `#[test]` function inside a module with no cfg (ignored);
    - a `#[cfg(not(test))]` function with an unmarked pop (refused);
    - a `pub mod tests` with no cfg that holds an unmarked pop (refused);
    - a `#[cfg(any(test, feature = "test-support"))]` function with an unmarked pop (refused);
    - a file that no `mod` declaration reaches, with an unmarked pop (refused);
    - a test module declared with `#[cfg(test)] #[path = "x_tests.rs"] mod tests;` (ignored);
    - a `src/foo/tests/bar.rs` reached by a `mod tests;` with no cfg (refused: not under
      `<package>/tests/`).
  - **The allowlist (D3):**
    - an allowlisted control function with a `while let` pop (accepted);
    - the same function under a name that is not on the list (refused);
    - a listed function with one pop more than its count (refused, stale);
    - a listed function that no longer pops (refused, stale);
    - a listed file that is missing (refused, stale);
    - a path that matches two functions (refused, stale);
    - a pop in a closure inside a listed function (refused: not admitted).
  - **The census (D2):** a `try_pop` inside an attribute's tokens in an unmarked file (refused); a
    file that holds `try_pop` and does not parse (refused).
  - **The helper outside every region:** `v1302/adv/a08` with its helper `next_record` moved out of
    the region and its caller left in it (refused: {unmarked try_pop}). B2b-2 adds the call-rule
    class to this case.
  - **Twins (B1b-D6's rule, for every D6 shape):**
    - `take_recycled_block`'s single pop in a method chain (`.try_pop().map_err(..)`): name the
      base-tree or B2a case that holds a single pop in no loop, or add one;
    - `observe_seek_at_block_boundary`'s single pop as a `match` scrutinee:
      `v1302/a2/v/v05_single_pop_match_receiver` (B2a);
    - the two-phase acquire with a re-observe between the phases (`let rest = .. .min(unpopped)`):
      new;
    - the meter poll's per-queue counted pop: B1a-D8b's `per_queue_meter_poll`;
    - the spectrum drains (`for _ in 0..available { if .. .is_err() { break; } }` with a named
      cap, and the read with a `match` and returns after the loop): new;
    - the spectrum collection's `cancel_except`, as #1449 D9 writes it: a drain loop
      `for (index, capture) in self.captures.iter_mut().enumerate()` with the `keep` check, which
      names `capture` only in its count and its pop, then a reset loop of the same header with the
      same check and a pop-free call: new;
    - the `LiveControl*` drains: B1b-D6's route-drain twin.

## #1448 guard (this slice's first commit)

This section is the spec of the commit "H #1448's guard, landed by J after C2" (root ruling 2 on
the fourth review, option (C)). It was #1448's D5. It lives here because #1448 (stream H, *Bound the
browser meter poll by each queue's count at entry*) closes at its own PASS and its spec leaves
`.github/ISSUE_SPECS/` at the next batch, long before the J batch (fifth review, MINOR-5). It is
this slice's first commit, after C2, and is judged in this slice's verdict.

- **G-D1. The markers.** `// REALTIME_POLICY_BEGIN` replaces the blank line before
  `poll_meters`'s doc comment, and `// REALTIME_POLICY_END` the blank line after its closing
  brace, in `hosts/host-web/src/lib.rs` (`:3422` and `:3759` on `6d28a80ec`, before #1448; find
  both by name). No line moves. If either blank line is gone, stop and report; do not insert a
  line.
- **G-D2. The floor.** The region floor in `Policy::workspace()` rises by one (re-measured). The
  awk gate is gone (C2), so no awk floor or self-test changes. The file floor does not change
  (`hosts/host-web/src/lib.rs` already holds regions).
- **G-D3. No other change.** If the tool refuses #1448's `poll_meters` once marked, stop and report
  the rule. Do not change the rule or the code in this commit.

*Gates of the guard commit* (each is PR evidence in this slice's verdict):
1. `cargo run --locked --release -q -p realtime-policy` passes with the markers, on the tree with
   #1448 merged.
2. On a scratch export with the same markers around #1448's parent `poll_meters` (today's
   capacity-bound loop), the tool refuses the pop (`:3466` on `6d28a80ec`) with the drain class.
3. On a scratch export with the markers around #1448's minimum-bound mutant (#1448 gate 2, mode 1
   of `mtr-review4-probe-modes.diff`), the tool refuses its pop with the drain class: B1a has no
   minimum form.
4. `artifact-identity` reports the worklet module UNCHANGED (the markers replace blank lines;
   measured by the fifth review: the marked and unmarked change give the same shipped module and
   named twin).
5. `cargo fmt --all -- --check` and `bash scripts/check-workspace-policy.sh` exit 0.

*Test value.* The markers are the committed guard of #1448's bound: gates 2 and 3 show that a
change that restores the capacity bound or the minimum form is refused, which no host-web test
can show on today's thread layout (no producer runs during the call).

## Authorized paths

- `tools/realtime-policy/**`
- D6's marker comments, each a cross-stream exception that root names in STREAMS
  (STREAMS):
  - `crates/source/src/lib.rs` (stream B; stream C for #1320 and #1355);
  - `crates/host-core/src/spectrum.rs` (stream A by its #1327 exception; stream H's production
    issue);
  - `crates/builtins-compiler/src/lib.rs` (stream A): the markers and D6's restructure of the
    test-support `LiveControlInputProcessor` only.
- `hosts/host-web/src/lib.rs` (stream H's): the two `poll_meters` markers of the guard commit
  only, by the exception in STREAMS.
- This spec

## Non-goals

- Bounding a drain (the three production issues do that). Marking `poll_meters` is the guard
  commit's only edit to `hosts/host-web` (section "#1448 guard").
- The call rule (B2b-2) and the rack bank loop (#1345 Amendment 1).
- #1418's and #1426's rules.

## Hazards

- **Re-measure at implementation time:** the allowlist entries and counts, the render-thread
  sites and the floors. Each later slice re-measures the floors when it lands (the realtime-policy
  floors row in STREAMS). #1346 may remove the test-support input drain's ring (then D6's
  restructure is void: record it), and #1312 the fader and matrix ones. Mark whatever still pops,
  and record what is gone.
- **Hot files.** All three D6 files are merged in STREAMS order. Find each function by name.
- **Every later pop or region edits `Policy::workspace()`.** After this slice, any slice in any
  stream that adds a control-side `try_pop` adds its allowlist row, and any slice that adds a
  marker raises the floors there. STREAMS names this as a standing exception for every stream
  (STREAMS).
- **Over-refusal is acceptable, a false pass is not.**

## Objective gates

1. **The real tree.** `cargo run --locked --release -q -p realtime-policy` passes, with the region
   count raised by D6's regions.
2. **The cases.** `cargo test --locked -p realtime-policy` passes, with every earlier case (with
   D8's class sets) and every D9 case.
3. **Red on revert (PR evidence).** Remove D6's markers and run the tool: it reports every
   unmarked function as an unmarked pop, which shows the gap is closed by the markers and not by
   the allowlist.
4. **Mutations (PR evidence).** Apply each alone. Each turns red the named cases, and the real
   tree still passes:
   - D2 disabled: the unlisted-function case and the helper-outside case;
   - the cfg evaluator without `not`: the `cfg(not(test))` case;
   - a name rule for `mod tests`: the `pub mod tests` case;
   - every cfg that mentions `test` read as test code: the `test-support` case (and gate 1 then
     passes without the `LiveControl*` markers);
   - site counts not pinned: the one-pop-more case;
   - the stale-entry check dropped: the four stale cases;
   - the census dropped: the attribute-token case.
5. **Shipped artifacts.** Markers that replace blank lines move nothing. The builtins-compiler
   markers and the restructure move lines in that file. So `artifact-identity` may report ARTIFACT
   CHANGED, and the `libcapi` digests may differ, only in `core::panic::Location` line fields and
   line tables (root's ruling on the second review's MAJOR-8; this is the one marker commit it
   names). PR evidence, as the stream-J2 batch verdict made it: the function-level comparison of
   the base and head named twins shows no other difference, the render closure is
   byte-identical, and every rendered-digest gate passes with the base's pins (no PCM moved). Any
   other difference fails.
6. `cargo fmt --all -- --check`,
   `cargo clippy --locked -p realtime-policy --all-targets -- -D warnings`,
   `cargo clippy --locked -p builtins-compiler --all-targets --features test-support -- -D warnings`,
   `cargo test --locked -p builtins-compiler --features test-support` (the restructured code's
   tests) and `bash scripts/check-workspace-policy.sh` exit 0.

*Test value.*
- The unlisted-function and helper-outside cases are red if a pop outside a region passes again.
  That is the gap behind the render-thread drains that `main` leaves unmarked.
- The D1 cases are red if the test-code boundary moves in either direction: a cfg read as test
  code that can compile without `test`, a module exempted by its name, or a file that nothing
  declares.
- The allowlist cases are red if an entry can admit more pops than it names, outlive its code, or
  match the wrong function.
- The census cases are red if a `try_pop` that the syntax walk does not see is silently skipped.
- The twins are red if a later change to the rule refuses a shape that the real tree now holds,
  and keep that defence after the cell slices or the Worker move remove the real drains.

## Evidence

- Gates 1-6 output. The re-measured allowlist, the marked functions, and the receiver evidence of
  D6 checked on the reviewed tree.
- The #1448 guard commit's gates 1-5 (section "#1448 guard"): the tool passes with its markers,
  refuses the same markers around the parent's `poll_meters` and around the minimum-bound mutant,
  and `artifact-identity` reports the worklet module UNCHANGED (fifth review, NIT-7).

## Dependencies

- After (same stream): C2. The commit "H #1448's guard, landed by J after C2" is this slice's
  first commit (section "#1448 guard"); the rest of the slice follows it, because D2 refuses an
  unmarked render-thread pop. C2 needs only A1 to B2a to cover every check of the awk gate (fourth review, BLOCKER-1, option B's
  measurement), so it moves before this slice.
- After (other streams), root rulings R1 and R2:
  - stream B, *Bound the source acquire by the blocks queued at its entry, and observe a late seek
    outside the loop*;
  - stream H, *Bound the browser meter poll by each queue's count at entry* (its code, unmarked;
    the guard commit above marks it);
  - stream H, *Bound every drain the AudioWorklet runs on its audio thread* (it also moves the
    collection's pop into its loop);
  - stream B batch 1 (#1309, #1343, #1314, #1311, #1348), through A1.
- Not ordered against #1312, #1346, #1347 and #1345 (root's ruling, 2026-10-05), nor against C
  #1320 and #1355 or later B source slices (`crates/source/src/lib.rs`), A #1327 and #1395
  (`spectrum.rs`), or A's builtins-compiler slices: the later slice rebases.
- Before: B2b-2.

## Standing rules for the implementer

- Work only from this body. Read the cited lines first; do not survey the workspace.
- A test that greps source or prose is refused. The cases are the tool's own input.
- Attempt budget: three attempts, one adversarial verdict each.
- Size: half a day.
