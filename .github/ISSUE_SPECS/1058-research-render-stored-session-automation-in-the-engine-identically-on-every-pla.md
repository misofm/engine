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
  `crates/capi/src/runtime/compile.rs:130-133`), which becomes #1306 D1's third term. If they do
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

## Decision record

The findings note is `docs/handoffs/stored-automation-1058/README.md` (attempt 4; anchors checked on
`6ee64f484`, whose code equals `45c5a1819`'s, `c63f5f37d`'s and `423b9d4a1`'s; every finding of the
attempt-1 to attempt-3 verdicts folded in). One
line per answer:

- **A1** ([Design](../../docs/handoffs/stored-automation-1058/README.md#a1-design)): the control
  plane compiles each automated cell into an immutable segment table in the plan; render evaluates it
  in node time on a 64-sample grid and at exact jump samples, and feeds the existing strip ramps,
  effect `Point` spans (in pieces) and filter targets; hosts move a new timeline clock with one
  session seek.
- **A2** ([Live changes against automation](../../docs/handoffs/stored-automation-1058/README.md#a2-live-changes-against-automation)):
  owner question OQ1; recommendation: offset on level rows, OR on mute, automation wins elsewhere;
  until the ruling, automation wins by one permanent rule in the shared commit on both hosts: a live
  edit of an automated cell commits as its fallback value, `model_only`, with no host-only path; no
  answer (B, C or D) undoes it.
- **A3** ([Format](../../docs/handoffs/stored-automation-1058/README.md#a3-format)): the table is
  enough; it gains the hold rule, one entry per lane, the pan or matrix form, 64 samples between
  jumps, unit, domain, shape and filter-order rules, and jump ramps from `control_smoothing`.
- **A4** ([Cost](../../docs/handoffs/stored-automation-1058/README.md#a4-cost)): `80 + 32·n` bytes per
  automated cell, owned by the plan, independent of song length (plus a VCA offsets cell of
  36 bytes per automated fader lane, whatever the VCA count: root's decision F16 sums the reaching
  offsets first, a class B change to #1242's order that slice 09a makes); CPU per block at most
  `2⌈q/64⌉ + 3` events per cell times per-row operation counts.
- **A5** ([Bit-identity](../../docs/handoffs/stored-automation-1058/README.md#a5-bit-identity)): the
  same session and host operations give the same bits on every target (scalar `f64` through
  `crates/math` at events only, unfused kernels, node-time grid); after a seek, independent of when it
  landed and of the quantum.
- **A6** ([Automated mute and follows_mute sends](../../docs/handoffs/stored-automation-1058/README.md#a6-automated-mute-and-follows_mute-sends)):
  every route into a submix whose source mute is automated gets a lane with `follows_mute` in its
  cell; render calls `graph::gated_route_coefficients` at each automated mute event, at the render
  sample where the send's tap carries the curve's timeline sample (A1.3), over the strip's ramp.
- **A7** ([Stored span bound, for #1306 D1](../../docs/handoffs/stored-automation-1058/README.md#a7-stored-span-bound-for-1306-d1)):
  `stored(i)` is the number of distinct automated cells of the instance, 0 for a target-owning
  effect; at most its `Block` cell count.
- **A8** ([AUTOMATION_ENQUEUE and effect windows, for #1306 D1](../../docs/handoffs/stored-automation-1058/README.md#a8-automation_enqueue-and-effect-windows-for-1306-d1)):
  no; #1306 D1 has no third term.
- **A9** ([Serving AUTOMATION_ENQUEUE, for #1315 D3](../../docs/handoffs/stored-automation-1058/README.md#a9-serving-automation_enqueue-for-1315-d3)):
  never served; the refusal is permanent and the command, its event and its queue leave the protocol
  registry (slices 21a-21c in batch P1, which needs only slice 01; slice 26 retires the span kinds
  and sample rate no producer uses; slice 22 reserves the C limit); no ack can precede a drop.
- **A10** ([The classifier mask, for #1260](../../docs/handoffs/stored-automation-1058/README.md#a10-the-classifier-mask-for-1260)):
  the mask becomes per row; slice 10 removes the fader row and slice 20 the last; an automation edit
  is a carried `rebuild` that completes `exact`; a static-value edit on an automated lane gives no
  record on either host; a group's other cell is a `live` group-cell write from the slice that
  renders the group.
- **A11** ([The staged plan](../../docs/handoffs/stored-automation-1058/README.md#a11-the-staged-plan)):
  forty-one slices in twenty-six steps and seven batches, drafted in
  `docs/handoffs/stored-automation-1058/proposed-specs/`; every rendering slice lands after #1382;
  slices 18a and 18b render stored effect automation and #1306 lands in their batch.

## Attempt record

**Attempt 1** (stream K research worker, `436cc137d`). No product code changed and no benchmark ran.
Two fresh internal verifiers reviewed it: the first returned FAIL on the first draft (one blocker: a
render-clock grid made the bits after a seek depend on when it landed; six majors), the second
PASS-WITH-FIXES on the revision (eight majors on package consistency); the note records each fold in
its "Verification" section. Gate 3 deviates by root's instruction: the slices are drafts under
`docs/handoffs/stored-automation-1058/proposed-specs/` for root to file as specs and GitHub issues.
Gates run: `bash scripts/check-workspace-policy.sh` and `bash scripts/check-dsp-research.sh` (the
latter checks the DSP research corpus, which this issue does not change).

**Attempt 2** (stream K research worker). The attempt-1 adversarial verdict returned FAIL (one major,
seven minors, seven nits). No product code changed and no benchmark ran. Per finding:

- **M1** (browser placeholders): slice 09a depends on *Admit browser live edits in the Worker through
  the committed model* (#1382), so every rendering slice acts on the shared commit on both hosts.
  Draft 10b and `COMMAND_REASON_AUTOMATED` are deleted; the browser refusals of drafts 11, 13a, 14a,
  15, 16a, 19 and 20 are removed; 13b is rewritten onto the shared commit and split. The permanent
  rule is decided: automation wins and a live edit of an automated cell commits `model_only` in the
  shared commit, not a typed refusal (A2, F9). Amendments that made an earlier-landing spec name a
  later slice became the slice's own change ("Not amended" list), which also removed a cycle
  (draft 04 depended on #1316, which an amendment asked to report the timeline). The graph is
  acyclic and no slice edits code an earlier slice deletes (A11, "No placeholder and no cycle").
- **m1**: batch P1 depends on draft 01 alone; 01 moves into P1.
- **m2**: a send's follow ramp is timed at its tap's arrival, which A1.3 requires (A6, draft 13b D2).
- **m3**: A2 states what option D would change and requires it as a cell word read at events, so
  A10, draft 17b D5 and the #1306 live term stay true; B and C likewise.
- **m4**: 04 split into 04a/04b, 13b into 13b/13c, 21a into 21a/21b (the queue is 21c); 14 and 16
  re-split by layer so no same-batch slice holds an interim rebuild classification; must-land-together
  groups fell from five to four.
- **m5**: F4 decided (slice 25 retires the protocol's stored transport position); F6 decided
  (slice 26 retires the span kinds and the sample rate no producer uses).
- **m6**: 18a D8 reads `80 + 32·n`; F12 decided and the #1054 D10 row states it, read by draft 08
  D2; the #1225 row is gone and draft 13c writes the route cell consistently.
- **m7**: [S6] cites the deep pages, [S7] adds Delay-Line Interpolation, [S8] uses the real section
  titles and the author URL; each was fetched again.
- **Nits**: loop points are quantum-granular (A1.2); the cursor bound is `q` per cell and block and
  joins the CPU bound; draft 17b lists #1345; this record names both internal reviews; the three
  anchor ranges are fixed; F3 needs no change; F10 asks root to file promptly.
- **New finding** F16: a VCA reach change on an automated fader lane is a carried rebuild (decision
  14 rule 1), a narrowing of D15-6's live set that root records.

The plan is now forty-one slices in twenty-six steps and seven batches (P1, R1, R2, R3, R4, P2, Q).
Gate 3 deviates as in attempt 1. Gates run: `bash scripts/check-workspace-policy.sh` and
`bash scripts/check-dsp-research.sh`.

**Attempt 3** (stream K research worker). The attempt-2 adversarial verdict returned FAIL (one major,
three minors, five nits). No product code changed and no benchmark ran. Per finding:

- **MA1**: draft 06b is rewritten for *Move browser source submission and seeks into the Worker*
  (#1387): the session seek goes to the control half by #1387's routing, the handler records the
  accepted generation, and the MSB1 drain reads it in the realm #1387 D5 runs it; gates cover
  `worker` and `single` mode; #1387, #1332 and #1294 are dependencies. A sub-agent checked "no
  slice edits code that an earlier slice deletes" mechanically for all 41 drafts against each
  closure (70 specs). Its definite findings are fixed: 06a (lifecycle refusal, the
  `SessionState` wrapper, a pending-candidate gate), 05 (wrapper in `crates/control-plane`), 14b
  and 11 (ramp words through `LiveRamps::resolve`), 02 (calls draft 07's evaluator and depends on
  it), 10 and 20 (superseded test cases), #1306 (amendment rows for S's old role), and stale paths
  in 10, 12, 13c, 21a-21c, 22 and 25.
- **m1**: the middle layout of the VCA offsets cell is bit-exact and keeps every membership change
  live, but adding or removing a VCA still rebuilds on a session with stored fader automation, which
  #1247 lists as live. So finding F16 is a root decision with three layouts and costs (recommended:
  the middle one); drafts 09a and 11 are written for it and depend on the ruling; the note's
  Authority line is corrected.
- **m2**: must-land-together groups 09a-09b-10-11, 13a-13c, #1306-18a-18b-19 and 21a-21c, each
  with its reason.
- **m3**: drafts 25 and 26 list every file a `git grep` finds, the audit tool and the policy pin
  included.
- **Nits**: browser gates name "a browser live edit through the Worker's apply" (compatible with the
  #1057 note's F4); A2 keeps the static edit's meaning under every option; A1.4 gives `τ` for
  `L = 0`; the "Acyclic" wording; the hot-file hazards of 19 and 20; F17 asks root to reopen GitHub
  #1306 (closed by a keyword in `6b8bc7c96`'s message while its spec is open).
- **New finding** F18: the specs of #1407 and #1408 exist only on `codex/d15-stream-g`.

The plan stays forty-one slices in twenty-six steps and seven batches; draft 02 now depends on draft
07 (both R1). Gate 3 deviates as in attempt 1. Gates run: `bash scripts/check-workspace-policy.sh`
and `bash scripts/check-dsp-research.sh`.

**Attempt 4** (stream K research worker). The attempt-3 adversarial verdict returned FAIL (one major,
three minors, nine nits). No product code changed and no benchmark ran. Per finding:

- **MA1**: F16's false premise is removed. Root ruled layout 4 during this attempt, under the
  standing summation-order ruling: the reaching VCA offsets sum first in ascending VCA-ID order,
  then the member is added and clamped, on the static path too; 36 bytes per automated fader lane
  at any VCA count; every VCA edit, adding and removing a VCA included, is a live write. F16
  records the authority (and that the memory's text names a measured speed-up), the amendment to
  #1242's order (`docs/SESSION_SCHEMA_V1.md:80`, `crates/session/tests/vca_composition.rs:79-82`),
  which slice 09a makes with the pinned test rewritten in the same change, the bit-move bound, and
  layouts 1-3 as rejected alternatives. Layout 3's sizing is corrected on both hosts. Drafts 09a
  and 11 are written for layout 4 and wait for no ruling. The memory formula is `36·F`.
- **m1**: draft 25 authorizes the `TransportState` payload in `queue.rs`; draft 02 the builtins
  policy pin. A fresh read-only sub-agent checked every draft's gates against the policy scripts
  and its paths; each finding is fixed in its draft (README "Verification", attempt 4).
- **m2**: draft 05 D6 builds on #1319 D4 (a generation-3 session seek); draft 20 D5 keeps 10 D1's
  copy line. The README's no-conflict claim states what each attempt checked.
- **m3**: 13a, 20 and 22 rewrite or delete the earlier gates they turn red.
- **Nits**: all nine (README "Verification", attempt 4).

The plan stays forty-one slices in twenty-six steps and seven batches. Gate 3 deviates as in
attempt 1. Gates run: `bash scripts/check-workspace-policy.sh` and
`bash scripts/check-dsp-research.sh`.

**Follow-ups after PASS.** The attempt-4 verdict returned PASS with four minors and five nits
(`docs/handoffs/decision-15-2026-10-05/verdicts/stream-k/1058-attempt4.md`, beside the verdicts of
attempts 1-3). One follow-ups commit folds each one. No product code changed and no benchmark ran:

- **m1**: the six realtime gates (drafts 11, 13a, 13b, 14b, 15, 16b) move to
  `crates/control-plane/tests/`, which reaches the shared commit; each draft authorizes
  control-plane's `bench-support` dev-dependency (allowed for a `crates/` manifest,
  `scripts/check-bench-policy.sh:257-280`) and runs `cargo test -p control-plane`. No host-core
  dev-dependency on control-plane.
- **m2**: draft 09a D6 rewrites the three member-first oracles (`crates/host-core/tests/vca.rs:268-279`,
  `hosts/host-web/src/tests.rs:13358-13371` and `:13697-13715`) and authorizes them; README F16
  names them.
- **m3**: README F19 states the iOS memset rule once and records why the trap-owner risk does not
  apply (direct-call gate, `dyn` dispatch). Every draft that changes product code in `capi`'s
  closure runs `bash scripts/check-cross-targets.sh`, R3 and R4 included; no ceiling row is
  authorized.
- **m4**: draft 24a puts the C ABI calls in a dedicated `tools/bench/src/console_capi.rs`, the only
  new unsafe owner, and runs the six policy scripts whose pins it edits.
- **Nit, 09a gate 1**: it asks for a case whose two orders give different gain bits (an example is
  given) and asserts that first.
- **Nit, 11 gate 2**: three nested VCAs, so the summation order is observable.
- **Nit, 05 D6**: it states why `replacements == 1` stays true (#1323 D2 finds no floor and no
  offset, so the declaration publishes nothing).
- **Nit, A5 and 09a D2**: render's conversion is `vca_compose_db`.
- **Nit, F16 Authority**: the private memory citation is gone. No file in `docs/rulings/` records
  the standing summation-order ruling; F16 says so and asks root to record it in decision 15.

Gates run: `bash scripts/check-workspace-policy.sh` and `bash scripts/check-dsp-research.sh`.
