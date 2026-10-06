FAIL

# #1058 attempt 1: adversarial verdict (stream K, stored automation research)

Reviewed: `436cc137d` on `codex/d15-stream-k` (parent `6ee64f484`), worktree `/home/bl/misofm/wt-d15-k`,
read only through `git show`. Scope: `docs/handoffs/stored-automation-1058/README.md`, the 37 drafts
under `proposed-specs/`, and the spec's decision and attempt record. Gate 3 is judged as reworded by
the coordinator: each slice must have a complete, stateless, fileable draft that lists its
dependencies.

The answers A1-A11 are mostly sound, and the evidence is good. 222 of the 225 anchors checked are
exact, and the one quoted time is real. But the staged plan (A11) contradicts itself and breaks the
binding no-shortcuts principle on the browser side (M1). Because of that MAJOR, the attempt fails.

## BLOCKER

None.

## MAJOR

**M1. Browser refusals are placeholders that #1382 deletes, and the plan's own dependencies make
the R2 refusals dead on arrival.**

- **The placeholder.** `proposed-specs/10b-refuse-browser-live-commands-on-automated-lanes.md:15-17`
  says: "The refusal stays until *Admit browser live edits in the Worker through the committed
  model* (#1382) replaces the browser's admission". README A2 (`README.md:308-312`) says the same.
  #1382 deletes worklet admission. Its product outcome (`.github/ISSUE_SPECS/1382-…:17`) is "The
  worklet no longer admits commands", and its D5 (`:121-125`) deletes host-web's strip, solo, VCA
  and follow passes in `admit_commands_staged`. After #1382, a browser fader edit on an automated
  lane goes through the shared classifier (10a D3). It then commits `model_only` instead of being
  refused.
  - So 10b ships behaviour that a filed issue reverses: refused becomes accepted as `model_only`.
  - 10b also adds a public reason, `COMMAND_REASON_AUTOMATED` (10b D2/D4, `:46-57`), to the
    six-source reason vocabulary and the generated SDK files. That reason becomes dead with #1382.
  - Decision 15's principle (`docs/rulings/live-updates-…-2026-10-05.md:26-28`) forbids this
    pattern by name: "no interim placeholder that a later issue must undo; defer only when a
    dependency forces the order, and then sequence the correct solution".
  - #1382 is already scheduled (STREAMS.md:276). The correct sequence is to put the browser half
    of R1 behind #1382, so that the browser gets 10a's rule from the shared commit.
- **The incoherence.** Batch R2 is one push (`README.md:666-667`), and 13a must land with 13b
  (`13a…:6-8`). 13b depends directly on #1342 (`13b…:178`; `README.md:640`). The #1342 spec
  depends on #1382 and #1290 (its Dependencies section; STREAMS.md:279). Its context says "#1382
  D5 deletes that pass" (the follow pass at `hosts/host-web/src/lib.rs:5274-5290`). So every R2
  slice lands after #1382. Yet R2 asks for code that #1382 has already deleted:
  - 13a D2 (`:55-57`): refuse `COMMAND_MUTE` in admission "until #1382".
  - 14a (`:15-16`, `:80-82`), 15 (`:16`, `:84`) and 16a (`:18`, `:94-96`): admission refusals
    "until #1382".
  - 13b D3 (`:66-70`) and its authorized path (`:95`, "the follow pass only"): changes to the
    host-web follow pass.
  - The note's own amendment row for #1382 D3 (`README.md:697`: "no browser-only path")
    contradicts these deliverables.
- **Fix (one of two).**
  - (a) Make R1's browser-facing slices depend on #1382. Then delete 10b, take the browser halves
    out of 11, 13a, 14a, 15 and 16a, and rewrite 13b's browser half onto the shared commit (#1382
    D3, #1342).
  - (b) Decide one permanent cross-host rule (a typed refusal on both hosts, the F9 alternative)
    that survives #1382, and state it in A2 and A10.
  - Either way, rewrite A2's bullet 3 and every "until #1382" line.

**Ruling on A2 ("automation wins until the ruling"; static edits on automated lanes are
`model_only`), as the coordinator asked.** On the shared commit (C ABI, and the browser after
#1382), this is a correct decision pending the owner's ruling, not a shortcut. A's rules hold under
B and C, which each add a stored field and an edit and change nothing that has shipped. So nothing
shipped is undone, and R1 does not need to wait for OQ1. The `model_only` reading of #1315 is a
decision the note made and stated as F9, with its alternative. That is acceptable. The only part
of A2 that breaks the principle is the browser refusal "until #1382" (M1). One gap in the argument
is m3.

## MINOR

- **m1. P1 is tied to all of R1 for one doc line, so a live acked-with-no-effect defect stays
  longer.**
  - The coupling: 21a (`:6`, `:234`) and README (`:670-671`) put P1 "after batch R1", only
    because 21b deletes the hold rule's old home (`CONTROL_PROTOCOL_REGISTRY.md:45`) after draft
    01 restates it. Draft 01 has no dependencies (`01…` Dependencies: "None").
  - The consequence: the amendment moves #1315 D1-D3 into 21a. The `AUTOMATION_ENQUEUE` refusal
    therefore waits behind R1's ~15 decision-15 prerequisites, instead of #1315's single
    dependency (#1309). Today the command is acked and never reaches PCM: the acked-batch class
    of defect.
  - Fix: make P1 depend on draft 01 alone (01 can land first or in P1).
- **m2. A6 and 13b put a send's follow ramp at the fader node's time, which contradicts A1.3 for
  taps upstream of a latent insert.**
  - 13b D2 (`:52-54`) and its test value (`:158-159`) make the route op use "the source fader
    node's arrival `a(n)`".
  - The route op runs before the send's compensation delay (`delayed` routes feed their line,
    `crates/graph/src/runtime.rs:916-920`). For an input, post-input or insert-send tap behind
    inserts of latency L, that choice mutes send content L timeline samples after the strip's
    content. A1.3 (`README.md:139-142`) promises that "automation lines up with the audio it acts
    on at every node, latency included".
  - Matching #1226 D5's live rule (ramp together in render time) may be the intended trade-off,
    but the note does not state or argue it. Fix: state and justify the choice, or use the tap's
    own arrival.
- **m3. The "no answer undoes anything" claim covers only B and C.**
  - `README.md:313-314` says "Options B and C add a stored field and an edit; they never change the
    rules above".
  - Option D (touch, `:738-740`) would put a live override on an automated cell. A10's "an
    automated cell takes no live record" (`:247`, 17b D5) and A7's #1306 live-term amendment
    (`:481-483`) both assume that never happens.
  - Fix: state that D, too, is built as a cell word read at events, not a live record, or list
    what D would change.
- **m4. Some drafts look too large for half a working day, and some interim code survives between
  same-batch slices.**
  - **04** factors the seek state machine for the producer and the consumer, and also adds the
    timeline consumer and producer, the history, the host-core carry, the byte reports and a test
    reader. Split it into crate-local and host-core parts.
  - **13b** spans graph, host-core, control-plane, host-web and lane.
  - **21a** has an inventory of about 20 files and about 25 tests to rewrite or delete.
  - 14a (`:15`, `:74`) and 16a (`:17`, `:88-89`) classify a static edit of an automated group's
    other cell as a carried rebuild. 14b and 16b then replace that with a live group-cell write in
    the same batch. That is the "interim code across same-batch slices" pattern that the
    Verification section (`README.md:858-861`) says was removed.
  - The five must-land-together pairs (09a/09b, 10b/11, 13a/13b, 18a/18b, 21a/21b) are each one
    product outcome. That is acceptable under batch delivery, but root should file them knowingly.
- **m5. Decisions handed to root that the note's own design creates or that a ruling already
  settles.**
  - F4 (`README.md:781-783`): A1.2 adds a second playhead beside `TRANSPORT_SET`'s stored
    position. The note should decide whether that position reports the timeline or is retired.
  - F6 (`:787-791`): the owner's removal ruling (`engine-footprint-2026-09-28.md:20-21`) settles
    it, and A9 already applies that ruling. The note should decide to remove the unused
    `AutomationSpanKind` and `AutomationRate::Sample` vocabulary, or give the reason it stays.
- **m6. Inconsistent numbers and rows.**
  - `18a…:125` charges "`48 + 32·n` per cell, A4", but A4 (`README.md:388`, `:569`) is
    `80 + 32·n`.
  - The #1054 D10 amendment row (`:692`) states the live-path change as decided, while F12
    (`:813-816`) says "Root decides". Draft 08 D2 (`:59-61`) leaves it to #1054.
  - The #1225 D3 row (`:698`, "yields no record") contradicts 13b D3 (`:68`, "#1225's route
    record in D1's layout").
- **m7. Citations are accurate in substance but imprecise in places.** A sub-agent fetched every
  source.
  - [S6]'s URL is a JavaScript loader directory, not a page. Cite the deep pages.
  - [S7]'s URL list omits the Delay-Line Interpolation page, which carries the zipper-noise claim.
  - [S8]'s labels read as section titles, but §8.1.3 is "Wavetable Generators" (Eq. 8.1.48 is
    there) and §8.3.1 is "Noise Reduction Filters" (the smoother is Example 8.3.1).
  - [S1] and [S5] are verified, and so is the Web Audio 1.1 Working Draft of 22 September 2026.
    [S2], [S3] and [S4] (pp. 1325-1331) are verified as well.

## NIT

- **Loop granularity.** A1.2 "Loop" (`README.md:128-129`) does not say that an anchored seek lands
  only on a quantum boundary, so loop seams are quantum-granular. The spec's Q1 asks how looping
  works.
- **Cursor bound.** A1.7's cursor bound (`:266-267`) leaves "continuous joints the grid passes"
  without a number. It is at most `q` per cell and block, because every segment is at least one
  sample. The A4 CPU bound (`:610-612`) omits cursor advances.
- **17b's #1345 dependency.** 17b is "written on the lane of" #1345 (`:145-146`, `:201-202`) but
  does not list it as a direct dependency.
- **Attempt record.** The spec's attempt record (`1058-…:132-136`) mentions only the first internal
  verdict (FAIL). README `:855-861` also records a PASS-WITH-FIXES review.
- **Anchor ranges.** Three anchors stop before the end of their function. None of them breaks an
  argument:
  - `crates/builtins/src/lib.rs:1352-1397`: the function runs to `:1426`.
  - gate-expander `:534-593`: the function runs to `:595`.
  - multiband-compressor `:1228-1279`: the function runs to `:1282`.
- **F3.** F3 says "root may reword". AGENTS.md's text still holds through the stored automation
  edits `0x0600`-`0x0603`, so the rewording is optional.
- **F10.** F10 defers advancing ramps through silent blocks to the silence work. That is
  acceptable, but the owner ranks silence work high, so root should file it promptly.

## What holds (checked)

- **A1 realtime.**
  - No allocation, lock or syscall. The per-block work is bounded by `2⌈q/64⌉ + 3` events per cell
    and at most `q` pieces.
  - The seek path is all-or-nothing on SPSC slots. The control thread only pushes and render only
    pops, so the check-then-push is sound.
  - Generation tags are reused from the source seek machine (04 D1).
  - Node time `timeline(s − a(n))` lines up automation with PDC. The grid in node time removes the
    first draft's seek-landing dependence.
- **A4.**
  - **Memory.** `80 + 32·n` per cell, plan-owned. Duration is not an input. `n` is authored
    segments, which the committed model already holds. The bound of 32/99 of the document is
    right: the smallest canonical segment is 98 bytes plus a comma. The argument is honest, and
    O(authored points) is acceptable.
  - **CPU.** The quoted time is real. In `artifacts/steps/bus-send-base/console-benchmark.accepted.jsonl`
    at `6ee64f484`, lines 31 and 62 are `console_mixing_automation` rounds 1 and 2:
    `paired_ramp_delta_median_ns` 1643 and 1823, Simd8, 48 kHz, q 128, EPYC 7313P, uncontrolled,
    `candidate_commit` 244a52a0c, 8 automated controls, one per bank. That is the newest of the 8
    recorded runs. The pushes are outside the clock (`tools/bench/src/console.rs:1931`, `:2107`),
    as the note says.
- **A5.**
  - Scalar `f64` through vendored `crates/math` at events only, with no lane math, so LANE-3
    cannot reach it.
  - `Lane::fma` is unfused, and Rust does not contract scalar code.
  - The canonical FP environment is MXCSR `0x1F80` and FPCR 0, with FTZ and DAZ clear. That
    equals wasm's IEEE semantics, so native and browser agree on subnormals.
  - The node-time grid is integer-exact. The EQ uses std `f64::sqrt`, which is correctly rounded
    and exempt in `clippy.toml`.
- **A6.** `graph::gated_route_coefficients` is the one const derivation (`crates/graph/src/lib.rs:777-812`).
  The composition is the live path's. The timing choice is m2.
- **A7.** `stored(i) = |C(i)|` is evaluable at preparation from the table and the descriptor. It
  does not depend on song length and is a summand of #1306 D1. It is per piece, so the window
  bound holds, with `Staged.dropped` at 0. 14 for the compressor and 9 for the delay match the
  metadata.
- **A8.** A clear "no". #1306 drops its third term.
- **A9.**
  - The refusal is permanent and the command, event and queue leave the registry under the
    in-place amendment rule.
  - The acked-batch answer is right: refused at decode, before any state change, and no queue
    remains.
  - The #1315 D3 hand-off is named (21a).
- **A10.**
  - An automation edit as a carried rebuild is consistent with decision 14 rule 1, since the
    successor needs a new compiled table, which is new memory, with D15-7 (carry, then retarget)
    and with D15-17 (`exact` at adoption).
  - The static-edit `model_only` path exists in D15-17's set.
  - The note names the slice that removes the first row (10a) and the last (20).
- **A11.**
  - The dependency edges, direct plus "arrives through", are acyclic.
  - Each draft I read in full has Product outcome, Context, frozen decisions, authorized paths,
    non-goals, objective gates, test-value lines and dependencies. I read 04, 07, 08, 10a, 10b,
    13a, 13b, 17a, 17b, 18a, 18b and 21a in full, 21b nearly in full, and the key parts of 09a,
    12, 14a, 15, 16a, 19, 20, 22 and 24a.
  - Slices 18a and 18b are named as the effect-rendering batch for #1306.
- **F1.** Correct. AGENTS.md's render rule forbids allocation, locks, I/O, logging, syscalls,
  structural plan mutation and data-dependent unbounded calls. A bounded design (one `pow`, one
  `tan`, up to four `sqrt` in `f64`, no allocation) is none of those. So designing filter
  coefficients in render is acceptable, and the change is a contract amendment, not a realtime
  violation. The alternatives fail as the note says:
  - Designs per grid sample grow with the length of the motion (A4).
  - Designs streamed by the control plane depend on when the host calls the service step (A5).
  - A third option the note does not list also fails: a domain-quantized design table. It changes
    the sound, so it needs DSP evidence, and it cannot follow live edits to the other values of a
    group cell.
  - Slices 16a and 20 correctly wait for root to confirm.
- **Findings.** F2, F5, F8, F9, F11, F13, F14 and F15 are correct. I checked F5 against #1345 D1
  (44 EQ parameter cells that the EQ never stages), F14 against the fixture (`matrix-ll` on a pan
  track) and F15 against `main` (`0c19119d0` is on `main`). For F4 and F6 see m5, and for F12 see
  m6.
- **Amendment table.** The table has 32 rows, not 30. Every row I checked names a real section and
  a needed change. The exceptions are the #1054 D10 and #1225 D3 rows (m6), and the #1382 D3 row,
  which contradicts the drafts (M1).
- **Owner questions.**
  - OQ1 and OQ2 each have background, options with costs, and a recommendation.
  - OQ1 states its link to #1057's part 3. #1057 section 7 defers part 3 to #1058, and OQ1 does
    not decide #1057's parts 1 and 2.
  - No decision is quietly handed off, except F4 and F6 (m5).
- **Anchors.** A sub-agent checked 225 anchors on `6ee64f484`: 222 exact, 3 short end ranges (see
  NIT). I re-read 7 more myself (`plan.rs:496-501`, `source/lib.rs:1154-1168`,
  `live_delta.rs:234-236`, `runtime.rs:3522-3525`, `builtins-compiler:743-747`, `queue.rs:799-803`,
  `CONTROL_PROTOCOL_REGISTRY.md:45`), and all are exact. These 7 are part of the 225, not extra.

## Tests

No tests were added or changed. This is a docs-only research note and draft specs.

## Gates run

- **Gate 4.** `git diff --name-only 6ee64f484 436cc137d` lists only
  `docs/handoffs/stored-automation-1058/` (README and 37 drafts) and
  `.github/ISSUE_SPECS/1058-research-render-stored-session-automation-in-the-engine-identically-on-every-pla.md`.
  PASS.
- **Policy scripts.** I ran these in an export of `436cc137d`
  (`/tmp/claude-1002/v1058/tree`, made a git repo so the scripts can run `git ls-files`, deleted
  afterwards):
  - `bash scripts/check-workspace-policy.sh`: "workspace policy: ok", exit 0.
  - `bash scripts/check-dsp-research.sh`: "dsp research corpus: ok", exit 0.
- **Gate 1.** A1-A11 headings and the decision-record links are present and resolve to the
  GitHub slugs. Each answer is a decision, and A2 is an option list with a recommendation. PASS.
- **Gate 2.** A7 is a formula, A8 is "no", and A9 names 21a and records the permanent refusal.
  PASS.
- **Gate 3, as reworded.** Every slice has a fileable draft with dependencies. It FAILS on
  coherence, because the R2 drafts target code their own dependency chain deletes (M1).
- **Not run, by design.** No build and no benchmark; the issue is docs only.
