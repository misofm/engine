# Render stored parametric EQ automation

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answers A1 (A1.5, A1.6), A5, A7 and A10, under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`.

## Product outcome

A saved session that automates a parametric EQ band or cut value (frequency, gain, Q, shelf
slope, and the cut filters' values) plays that automation on both hosts, on an insert or a
console slot. At each event, render designs the section's coefficients with the EQ's own designer
and the same validation the control plane runs, and glides to them over the EQ's fixed 64-sample
coefficient ramp. The section's other values come from a group cell that live edits write, so a
live edit of one band value never overwrites an automated one. An EQ automation edit is a carried
rebuild that completes `exact`. This is the last automation row to leave the classifier's mask.
Batch R4.

## Context

**The EQ's controls.**
- 22 `Block` parameters, all `PerLane`: the automatable fields of four bands and six cut
  parameters (`parameter`, `crates/parametric-eq/src/lib.rs:491-528`; `cut_parameter`,
  `:530-561`; table at `:564`). So 44 cells, in six sections (`EQ_SECTION_COUNT`, `:80`).
- The designer is `design_svf_words_f64` (`:747-787`): one `math::pow`, one `math::tan`, one or two
  `sqrt`, in `f64`; `design_svf` rounds it once (`:853`).

**The prepared-target path.**
- The factory's `prepare_targets` (`crates/parametric-eq/src/control.rs:464-481`) validates the
  request, designs the touched sections (`prepared_sections`, `:198`; `fill_targets`, `:287`) and
  validates the result (`validate_prepared_targets`, `:396`), all in fixed storage
  (`MAXIMUM_TARGETS`, `:24`).
- The processors apply a target with `apply_target_lane` (`crates/parametric-eq/src/lib.rs:2860-2894`)
  through `apply_prepared_target` and `start_ramp` (`:1441-1478`). The step is
  `(word - current) * RAMP_SCALE` over `RAMP_SAMPLES = 64` (`:117-119`). Sample `A` uses the old
  words and the exact target is used at `A + 64` (`docs/EFFECT_CONTRACT_V1.md:165-166`):
  completion `After`.
- The EQ refuses raw spans and says why (`crates/parametric-eq/src/lib.rs:3513-3521`; the comment
  is `:3518-3520`). The contract says the owner prepares fixed targets off render
  (`docs/EFFECT_CONTRACT_V1.md:157-168`), and the trait says "Render-side code never calls this
  method" (`crates/effect-contract/src/prepared_target.rs:73`, module text `:6-8`).

**Live edits today.** The classifier designs EQ targets on the control thread with
`EqTargetPreparer` (`crates/host-core/src/control_preparation.rs:304-380`; called from
`design_targets`, `crates/host-core/src/live_delta.rs:463-502`). #1345 D1 holds them in one
12-word target cell per `(section, channel)`. The browser took EQ edits as prepared submissions
(`hosts/host-web/src/lib.rs:5076-5100`); after *Admit browser live edits in the Worker through the
committed model* (#1382) they reach the shared commit through the Worker's apply and meet the shared
classifier.

**The render closure gate.** `scripts/check-web-audioworklet.sh` requires the render export's
call closure to reach no allocator and own no trap outside one documented site (`:389-421`), and
counts scalar arithmetic in the listed kernels (`KERNEL_ROSTER`,
`scripts/check-web-audioworklet-callgraph.py`).

## Decisions frozen for this slice

- **D1. The group.** One EQ section on one channel is a target group. If any of its cells is
  automated, render owns that section's target on that channel. Its cells take draft 07's builder
  with grid period 64, grid ramp 64, jump ramp 64 (A3) and completion `After`.
- **D2. One design per event.** At each event of the group (grid, jump, seek step, adoption jump,
  and the next block entry after its group cell changes), at a piece start (draft 17b):
  - the stage builds the EQ's 60-value candidate in descriptor order on the stack: each automated
    cell takes its curve value at the completion sample `τ + 64`; every other value of the section
    comes from the group cell; other sections keep their current semantic values;
  - it marks the section's values as changed and calls the EQ factory's `prepare_targets`, the
    same code the control plane runs, into a target buffer the stage allocates at bind, sized from the
    factory's `maximum_targets()` (`crates/effect-contract/src/prepared_target.rs:67`), so render
    never allocates. `MAXIMUM_TARGETS` (`crates/parametric-eq/src/control.rs:24`) is private, and
    rack may not depend on parametric-eq (`scripts/check-rack-policy.sh:23`);
  - it applies the section's target for that channel with `apply_prepared_target_lane` (bank) or
    `apply_prepared_target` (per node), which starts the 64-sample ramp.

  Two cells of one group with events at one sample give one design. The stage holds the
  instance's factory (`EffectPreparedEntry::factory`, `crates/effect-compiler/src/prepare.rs:37-38`),
  cloned at bind; render only reads it.
- **D3. A refused design** keeps the current target and increments a saturating
  `automation_designs_refused` count on the stage, read after render. Draft 03a's domain and order
  rules make it unreachable; the gates assert 0.
- **D4. The group cell.** For a section with an automated value, this slice gives #1345's
  section target cell the meaning "group cell": it holds the section's semantic values, not designed words. A live
  edit of a value of that section that is not automated writes the semantic value into the cell
  (`live`); the control thread designs nothing for it. A section with no automated value keeps
  #1345's designed target cell. #1345 is not amended: it lands first, and this slice adds the
  meaning for a section kind that exists only from this slice on.
- **D5. The classifier.**
  - The EQ's automation rows join `automation_row_renders` (draft 10 D1), which is then true for
    every row. The masked JSON comparison keeps copying `current.automation`
    (`crates/host-core/src/live_delta.rs:236`, the line draft 10 D1 keeps), so that comparison never
    sees an automation difference, and draft 10 D1's step after step 4 returns
    `LiveRebuild::Automation` for every automation edit. The rustdoc (`:181-185`, as draft 10 D1
    rewrote it) says that every row renders.
  - An EQ automation edit is `LiveRebuild::Automation`. Its cells carry by address and its group
    cells by #1280's cell carry (draft 19 D3-D6).
  - A static edit of an automated EQ value gives no record (draft 19 D2, now for the EQ too).
  - A live edit of a non-automated value of an automated section is D4's group cell write, not a
    target design.
  - With the mask empty, every automation edit is a rebuild, so the separate
    `LiveRebuild::AutomationTarget` routing that #1335 D4 adds and draft 02 D5 extends is deleted: an
    automation edit with a refusable entry returns `LiveRebuild::Automation`, and the rebuild's
    preparation refuses it with the same diagnostic.
  - **Earlier gates this slice turns red, rewritten here in place** (AGENTS.md: a change that
    supersedes a test rewrites it in the same PR):
    - the cases that pin `AutomationTarget` expect `Automation`: draft 02's gate 7, draft 10's
      gate 1, and #1335's gate 4, whose test
      `an_automation_on_a_prepared_effect_parameter_needs_a_rebuild`
      (`crates/host-core/tests/live_delta.rs` on `main`) also has its second half, the block-rate
      `band-1-gain` that "stays live with no records", rewritten to `Err(LiveRebuild::Automation)`;
    - draft 19's gate 1 case "an EQ automation change stays live with no records" expects
      `Err(LiveRebuild::Automation)`;
    - #1335's gate 3, the band-gain half of
      `an_automation_on_a_prepared_effect_parameter_is_refused_before_any_ack`
      (`crates/capi/src/runtime/live_tests.rs` on `main`, the "a block-rate target commits live: one
      revision, no candidate" assertion with `pending == 0`): the accepted edit now takes the
      rebuild path, so the test asserts one pending candidate, then completion `exact` and one
      revision after it, with the rendered blocks bit-identical to a twin prepared from the edited
      session.
    Draft 03a's gate 3 asserts the refusal itself, which stays.
- **D6. One rule, both hosts.** D4 and D5 live in the shared classifier, so a browser EQ edit meets
  them through #1382's Worker commit, as drafts 14b and 16b do for the pan and matrix group and the
  input filter pair. No host has its own admission for automated sections.
- **D7. No span, no window.** The EQ stages no span: `stored(i) = 0` (A7). An automated EQ with
  no live lane still uses the live-control stage with a channel-less lane (draft 17b D6), for its
  program and its group cells.
- **D8. The render-design exception** (finding F1, which root confirms before filing).
  `docs/EFFECT_CONTRACT_V1.md:157-168`, the EQ comment (`crates/parametric-eq/src/lib.rs:3518-3520`)
  and the trait's two sentences (`crates/effect-contract/src/prepared_target.rs:6-8`, `:73`) gain
  one exception: stored automation designs a section's target on the render thread, with the
  owner's own designer and validation, at bounded events (A1.6).
- **D9. #1337.** *Make a parametric EQ band's enabled and kind live* (#1337) lands before this
  draft (Dependencies). A band's `enabled` and `kind` are values of its group like the others (step
  only, draft 03a), and #1337 D3-D4 govern the designed ramp. Draft 03a's step-only rule holds
  whichever of draft 03a and #1337 lands first: until #1337 makes the two fields automatable, an
  entry on them gets #1335's `effect.automation.rate` (draft 03a gate 1).

## Deliverables

1. D1-D3 and D7 in the stage (`crates/rack/src/lib.rs`, `crates/graph/src/runtime.rs`) and the
   cell programs in `crates/host-core/src/prepare.rs`.
2. D4-D5 in `crates/host-core/src/live_delta.rs` and the cell writer.
3. D8's text.
4. Delete the `model_only_edits` cases that draft 10 moved to an EQ band-gain entry
   (`crates/host-core/tests/live_delta.rs` and `crates/capi/src/runtime/live_tests.rs`): with the
   mask empty, no automation edit is `model_only`, and the rebuild cases of drafts 10 and 19 and of this slice cover the path.
5. The tests below.

## Authorized paths

- `crates/parametric-eq/src/{lib.rs,control.rs}` (only to make `prepare_targets` trap-free and
  allocation-free on every path render reaches, and D8's comment).
- `crates/effect-contract/src/prepared_target.rs` (D8's doc text only).
- `crates/host-core/src/{prepare.rs,live_delta.rs,control_preparation.rs}`,
  `crates/host-core/tests/live_delta.rs`.
- `crates/rack/src/lib.rs`, `crates/graph/src/runtime.rs` (the EQ's events in the stage).
- `crates/capi/src/runtime/` tests, `hosts/host-web/src/tests.rs` (gate 7).
- `scripts/check-web-audioworklet-callgraph.py` (a `KERNEL_ROSTER` note, only if the gate needs
  one for the design path, with the reason).
- `docs/EFFECT_CONTRACT_V1.md`.

## Non-goals

- The input HPF and LPF (drafts 16a, 16b), which use the same scheme on the strip.
- A designer of its own: render calls the EQ's, unchanged in result.
- A faster or vectorised design: events are sparse scalar control work (A1.7).
- The EQ's #1306 live term (finding F5).

## Hazards

- **Finding F1.** This slice amends a contract rule. It waits for root's confirmation.
- **The render closure gate.** `prepare_targets` and `validate_prepared_targets` enter the shipped
  module's render closure. Any panic path in them (an `expect`, an unchecked index) is a trap the
  gate refuses. Fix the path in the EQ crate; never admit a new trap owner.
- **Cost.** A moving section designs once per 64 samples per channel and lane (A4's table). Gate 6
  bounds the count; drafts 24a and 24b time it.
- **Hot files.** `crates/host-core/src/live_delta.rs` (stream B), the stages (stream A),
  `hosts/host-web/src/tests.rs` (stream H). Root sequences the merge.

## Objective gates

1. **Target words equal the control plane's** (new `crates/host-core/tests/effect_automation.rs`
   cases). A linear frequency segment and an exponential gain segment on band 2's left channel:
   at each event, the applied target (test-support trace) equals, word for word, the target
   `prepare_targets` returns in the test for the same 60 values, where the curve values are the
   README formula at `τ + 64`.
2. **Flat equals static, both hosts.** A flat automation at `v` on a band gain renders the bits of
   the same session with static `v`: C ABI (`crates/capi/src/runtime/tests.rs`) and browser host
   (`hosts/host-web/src/tests.rs`).
3. **Group cell.** On a playing engine, a live edit of band 2's Q while its frequency is automated
   commits `live`; the next design uses the new Q and the automated frequency's curve value, and
   the frequency keeps following the curve.
4. **Banks equal scalar.** Four console EQ lanes at Simd4 and Simd8, two automated with different
   curves, render each lane's bits as its scalar instance does.
5. **Classifier** (`crates/host-core/tests/live_delta.rs`): an EQ automation edit gives
   `Err(LiveRebuild::Automation)`; a static edit of an automated EQ value gives no record; a delta
   that changes only the session ID is still live with no records (the mask is gone, nothing else
   moved); a fader entry in unit `linear` gives `Automation`, not `AutomationTarget`, and its
   rebuild refuses it with draft 02's diagnostic.
6. **Bounded designs.** One section group designs at most
   `(⌈q/64⌉ + 1) + Σ_cells(⌈q/64⌉ + 1) + 1 seek + 1 cell change` times per channel and lane per
   block, as draft 16b D4 states: the group's cells share one 64-sample grid, and each automated
   cell adds its jumps. At `q = 128`, band 2 with frequency, gain and Q automated, a seek and a
   group-cell change in one block designs at most `3 + 3·3 + 1 + 1 = 14` times (test-support
   counter); `automation_designs_refused` is 0 in every gate.
7. **Browser, one rule** (`hosts/host-web/src/tests.rs`): an EQ edit of an automated value replies
   `model_only` and moves no bit; an edit of a non-automated value of that section replies `live`
   and, after its ramp, equals a plan prepared with the new value and the same automation.
8. **Allocation.** 1,000 blocks with designs in every block make 0 allocations and 0 frees
   (`bench_support::alloc::current_thread_delta_since` after one warm block);
   `cargo build --locked --release -p audit -p capi && target/release/audit capi` reports every
   violation count 0.
9. **No rendered bit moved (PR evidence).** `target/release/audit capi` shows the same
   `pcm_digest` at base and head; the browser legs of `qualification.yml`'s `browser` job pass with
   unchanged digests.
10. **Artifact and workspace.**
    - `mkdir -p target/ci/qualification-artifacts target/ci/qualification-named-twin && bash scripts/build-web-audioworklet.sh --named-twin target/ci/qualification-named-twin target/ci/qualification-artifacts`,
      then
      `bash scripts/check-web-audioworklet.sh --without-metadata-regeneration target/ci/qualification-artifacts target/ci/qualification-named-twin/miso-engine-v1-audio-worklet.simd128.named.wasm`
      and `python3 -B scripts/check-browser-expected-resources.py --artifacts target/ci/qualification-artifacts`
    - `bash scripts/check-parametric-eq-render-contract.sh`
    - `cargo test --locked -p parametric-eq --features test-support`,
      `cargo test --locked -p host-core --features test-support`, `cargo test --locked -p capi`,
      `cargo test --locked -p host-web --features test-support`,
      `cargo test --locked -p graph --features test-support`
    - `for x in host-core realtime workspace; do bash scripts/check-$x-policy.sh && bash scripts/test-$x-policy.sh || exit 1; done`
    - `bash scripts/check-cross-targets.sh` (README F19, the iOS memset rule: no new
      `memset_pattern16` call; fix one in code, never by a ceiling)
    - `cargo fmt --all -- --check`,
      `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`

## Test value

- Gate 1 turns red if render designs with another designer, another validation, other values, or
  at `τ + 63` instead of the `After` completion.
- Gate 2 turns red if the prepared value or the first design differs from the static plan's.
- Gate 3 turns red if a live edit of a non-automated value overwrites the automated one, or is
  lost at the next event.
- Gate 4 turns red if one lane's design reaches a neighbour lane.
- Gate 5 turns red if the EQ stays masked (an edit never heard), if the mask's removal changes
  another row, or if the `AutomationTarget` route survives the empty mask.
- Gate 6 turns red if a group designs once per cell at a shared grid sample instead of once per
  event sample.
- Gate 7 turns red if the browser has a path that writes a designed target over automation.
- Gate 8 turns red if the design path allocates on the render thread.

## Dependencies

Batch R4. Direct dependencies:

- Draft 19 *Classify and carry effect automation edits*.
- *Make a parametric EQ band's enabled and kind live* (#1337), an ordering dependency (D9).
- Root's confirmation of finding F1.

Draft 17b (the pieces), draft 03a (the domain and shape rules that make a refused design
unreachable), *Hold effect parameter, bypass and EQ-target values in latest-target cells* (#1345, the cell D4
lays out) and *Carry live-controlled effect lanes across a
plan swap* (#1280) arrive through draft 19.
