# Research: render stored session automation in the engine, identically on every platform

Stream K of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-16): research now.
Its design feeds *Size each effect's automation span window from the producers its plan has* (#1306, D15-5),
*Refuse automation on effect parameters that are not block-rate* (#1335, D15-13 E1), the live
classifier's automation mask (`host_core::classify_live_delta`, see below) and
`AUTOMATION_ENQUEUE`, which *Refuse commands that would be acknowledged with no effect* (#1315)
refuses until this research's first rendering slice wires it.

Owner ruling (2026-09-28, `docs/rulings/engine-footprint-2026-09-28.md`): a producer's automation (fades, sweeps, rides) is **rendered by the core engine from the session file**, so fans hear it bit-identically on web and mobile; apps do not play it back.

## Today

The session schema's `automation` table accepts builtin (fader, mute, pan) and effect-parameter targets and the SDK can author them, but it "renders nothing" (`docs/SESSION_SCHEMA_V1.md:220-225`; `docs/REALTIME_MEMORY.md:13`), gated on #140, which is being closed as descoped. The engine already has absolute sample-time parameter events, bounded point batches and step/linear/exponential segments (AGENTS.md "Interfaces and transports"), the live console lanes, and per-effect automation spans.

## Questions to answer, with evidence

1. **Design.** How the compiled plan evaluates stored automation sample-accurately in render with no allocation, locks or unbounded work: where curves are compiled (control plane), how they feed the existing fader/matrix ramps and effect spans, and how seeking and looping work.
2. **Live changes against automation.** What happens when a fan (or producer) moves a control that has automation: override until release, offset, or latch. Present the options for an owner ruling; the personal-mix ruling (see the session-ownership research issue) interacts with this.
3. **Format.** Whether today's `automation` table is enough (curve shapes, resolution, units), aligned with the session's `controlSmoothing` settings (#1054).
4. **Cost.** Memory per automated lane, as a formula that does not grow with song length. CPU per block as a bounded analytic estimate: the most spans and ramp starts one block can stage, times the per-span cost of the paths they feed. The existing `console_mixing_automation` row (`tools/bench/src/console.rs:138-158`, issue #1003) already times the live span and ramp paths; quote a per-span cost only from an existing recorded run of it, with its source, and otherwise keep the estimate in operation counts. This issue runs no benchmark and adds no row. The rendering slices own their own measurement gates.
5. **Bit-identity.** How browser and C ABI render identical bits for the same session and seek position.

## Coordination with *Submix strips and live aux sends* (#1196, decision 13)

A route with `follows_mute: true` (*Let a route into a submix follow its source strip's mute in the session*, #1218) zeroes the matrix columns of its source strip's muted lanes. Builtin automation of the fader's `mute` (parameter 6) is valid and inert today (`BUILTIN_AUTOMATION_TARGETS`, `crates/session/src/validate.rs:823-840`; `validate_builtin_automation_target` documents the inertness at `:842-853`). When stored mute automation renders, a following send must follow the automated mute through the same gated route composition the live path uses (*Let a send follow its source strip's mute live in the browser*, #1224); otherwise a saved session with automated mute leaks its pre-fader sends. The design must answer how.

- **The C ABI's live classifier masks `automation` (#1260).** `host_core::classify_live_delta` (`crates/host-core/src/live_delta.rs:181-185`, the mask at `:234-236`) treats a stored-automation edit as model-only, committing it with no plan rebuild and no live record, because nothing renders stored automation today. The first issue that renders stored automation must remove `automation` from that mask, or an automation edit would commit without reaching the running plan.

## Required answers

Each answer is a heading in the findings note, with its evidence: code anchors (`path:line`) checked
against `main`, or primary citations. Q1-Q5 above are answers A1-A5, and the mute question above is
A6. The answers below are what other decision-15 issues consume, so each must be a decision, not an
option list.

- **A7. Stored span bound, for #1306 D1.** For one prepared effect instance, the most stored-automation
  spans that one block can stage into its span window: the formula preparation evaluates from the
  session's `automation` table, its inputs, and its upper bound in terms of the instance's automated
  parameters. It must not grow with song length. It is a summand of #1306 D1's `capacity`, not a
  second sizing.
- **A8. `AUTOMATION_ENQUEUE` and effect windows, for #1306 D1.** Whether accepted
  `AUTOMATION_ENQUEUE` batches stage spans into effect span windows. If they do, the per-block,
  per-instance bound of those spans (at most S, the protocol's `per_block_automation_density`,
  `crates/control-plane/src/compile.rs:158-161`), which becomes #1306 D1's third term. If they do
  not, say so, and #1306 drops that term.
- **A9. Serving `AUTOMATION_ENQUEUE`, for #1315 D3.** Whether the C ABI serves the command at all.
  If it does: the render-side drain that consumes `ProtocolQueues::try_dequeue_automation`
  (`crates/protocol/src/queue.rs:801`, no production render caller today), the slice that sets
  #1315's `ProviderFeatures::automation` true, and why no accepted batch can be dropped after its
  ack. If it does not: the refusal is permanent, and the note says whether the command leaves the
  protocol registry.
- **A10. The classifier mask, for #1260.** Which slice removes `automation` from the mask above,
  and what a stored-automation edit becomes then (live record, or rebuild).
- **A11. The staged plan.** The rendering slices, smallest first, each closable in half a working
  day, with the slice that renders stored effect automation named, because #1306 lands in its
  batch.

## Output

A findings note at `docs/handoffs/stored-automation-1058/README.md` with A1-A11, the costs, the
staged plan and the owner questions. This spec's decision record lists A1-A11, one line each, with
the note's heading for each. No product code change.

## Objective gates

1. **Every answer is recorded.** The note has one heading for each of A1-A11, and this spec's
   decision record links each one. Each answer states a
   decision (A2 lists its options for an owner ruling), and every code claim carries an anchor
   that points at the stated text on `main`.
2. **The consumers can act.** A7 gives a formula #1306 can evaluate at preparation, A8 gives a
   bound or a clear "no", and A9 names the slice that flips #1315 D3 or records the permanent
   refusal.
3. **The plan is filed.** Each A11 slice has a spec under `.github/ISSUE_SPECS/` and a GitHub issue
   with the same number and title, and each lists its dependencies.
4. **No code moved.** `git diff --name-only origin/main...HEAD` lists only
   `docs/handoffs/stored-automation-1058/`, `.github/ISSUE_SPECS/` and this spec.

## Dependencies

- None. Q3 aligns the format with the `controlSmoothing` spec (#1054) as written; it does not wait
  for #1054 to land. #1306 and #1315 consume this issue's answers; this issue does not depend on
  them.
