FAIL

# #1057 attempt 1 verdict: design note "one edit API on every host" (`6ee64f484`)

- Reviewed: `git diff 8be19c86e 6ee64f484` in `/home/bl/misofm/wt-d15-k` (not edited, still clean).
  The commit touches exactly `docs/handoffs/one-edit-api/1057-design-note.md` (new) and the #1057
  spec's attempt record. Both are authorized paths.
- Verifier: fresh Opus 5.5, no part in the note. Export `git archive 6ee64f484` into
  `/tmp/claude-1002/v1057/tree`, target `/tmp/claude-1002/v1057/target` (both deleted after the run).
- Read: AGENTS.md, decision 15, engine-footprint 2026-09-28, the owner no-shortcuts rule, the #1057
  spec, and #1309, #1332, #1381, #1382, #1290, #1293, #1294, #1296, #1386 in full; the decision
  lines of #1312, #1313, #1314, #1327, #1348, #1387, #1400, #1305, #1058.

Result: no BLOCKER. Three MAJORs. The note is careful, the anchors are right, D1-D7 are not
reopened, and F1, F2, F3 are correct and useful. But section 8 leaves the render-half state of an
observation subscription undecided (and F5 removes the sample it depends on), P2 is scoped so that
it cannot meet its own gate, and the SDK batching rule creates an uncounted supersession of acked
calls that section 1.3 does not see.

## MAJOR

**MAJOR-1. Observation subscriptions have render-half state that the note does not place; F5
removes the sample it needs (note:30-32, :269-272, :650-653, :724-731, :738, :759-764).**
Today a subscribe/unsubscribe is not only an `Observe` record in the effect lane. Admission also
writes the browser's delivery mirror: `observation_armed[effect]` and
`observation_arm_samples[effect][tap] = applied_at_sample` (`hosts/host-web/src/lib.rs:1828-1853`
at `8be19c86e`; the comment says the armed set is updated at admission "so it can never disagree
with what the render side was told"). Render reads them in the meter fold (`lib.rs:3683`) and the
observation read bounds its window from the arm sample (`lib.rs:5874-5885`). #1381 D3 puts both
arrays in `RenderCompanions`, the render half (spec lines 112-113).
- Section 8 rules subscriptions into the Worker overlay but never says who writes that mirror for
  the active plan in Worker mode. F3's own argument ("a Worker-side lease would write that state
  from another thread") applies to it exactly. #1382 and P6 implement from this note, so the choice
  (render derives the mask when it pops the record, or a render-half message as for the lease, or
  something else) must be decided here.
- F5 removes `appliedAtSample` and says "completion is the watermark". But monitoring operations
  "never appear in a path or revision" (8.1), so the watermark cannot report when a subscription
  or a solo took effect. The arm sample above has no source in Worker mode, and the public SDK
  exposes it: `ObservationSubscription.appliedAtSample` and `ObservationSubscriptionReceipt.
  appliedAtSample` (`sdk/src/core/observation-subscriptions.ts:101`, `:112`, set from the command
  report at `:549`, `:627`). Also `shipped-host.d.ts:664-665` tells apps to correlate meter windows
  with `appliedAtSample`.
- The acked-batch row "Monitoring" (note:121) is affected too: a subscription acked after its
  record is pushed, while the render-half mask stays clear, delivers nothing.
Fix: rule where the armed mask and arm sample come from in both modes, state what replaces
`appliedAtSample` for monitoring operations (or why none is needed), and add it to P5/P6 or a new
proposed issue.

**MAJOR-2. P2 omits the classifier's dominant session-proportional cost, so it cannot meet its own
gate as scoped (note:507-513, :566-575; F1 at :622-634).**
`classify_live_delta` clones the whole next model and writes two whole canonical JSON strings on
every call (`crates/host-core/src/live_delta.rs:228`, `:257-263`; its doc at `:202` says so). The
note's own phase probe measures one canonical write at 0.56-1.00 ms native on the 64-track console
(1.33 ms on sends), so two writes are about all of the classifier's measured 1.20-2.04 ms (2.67 ms
on sends). The note instead attributes the classifier cost to "lowers every track twice (#1305
N2)", and P2's scope relies on "land #1305". #1305 explicitly excludes this term: "Non-goals: the
classifier's whole-session mask and canonical-JSON comparison" (`1305-*.md:103`). With the V8/native
ratio the note measured (about 1.4-2.7x), the two writes alone are about 1.6-5 ms in V8, above the
2.667 ms single-mode budget that P2's gate sets. F1 is right; its remedy is mis-scoped, and the
coordinator would file P2 from this text.
Fix: correct the decomposition and add the classifier's mask clone and canonical comparison to P2
(with a field-level comparison or a cached canonical text of the committed model), or state that
P2 is expected to miss and take F1 to root now.

**MAJOR-3. SDK batching supersedes acked calls inside one transaction, outside D15-2's rule and its
counter (note:108, :317-329).**
Section 3.4 merges queued live calls into one transaction, "edits in call order", and says "the
last value of an address wins inside the transaction, which is the same rule as the latest-target
cells (D15-2)". It is not the same rule. AGENTS.md (line 49) and D15-2 define supersession as "a
live value superseded by a later committed value before the same render drain", recorded by an
exact `live_values_superseded` counter (D15-2 condition 4). In a merged transaction the earlier
call is resolved with a revision whose model never holds its value, and no counter records it. The
W1 row of 1.3 answers "No" without this case.
Fix: either count SDK-merged supersessions in the same counter (or an SDK counter that the status
reports), or merge without collapsing the same address, or raise it as a finding for the
coordinator to amend D15-2's wording. Any of these is small; leaving it is not.

## MINOR

- **MINOR-1. Gate 3 is not fully met (note:358-367, :415-419, :507, :524-530).** Section 4.2 says
  "Commands are in section 5.5", but 5.5 lists files only. The native snapshot timings
  (`probe_snapshot`), the `probe_phases` build line and the twiggy grouping command are not given.
  "Two runs" of `probe_phases` are cited (1.20-2.04 ms and so on), but
  `evidence/probe-phases.txt` holds one run. A second invocation is also a retry under AGENTS.md's
  benchmark rule.
- **MINOR-2. The owner question understates option C (note:682-683, :694-698, :715-717).** The
  overlay holds field values ("fader of track `vox` = -3 dB"), but #1386's diff is entity-level and
  "never emits a finer edit" (`1386-*.md` D1). So "the same diff `replaceSession` uses" cannot make
  the list, and U2 with whole-entity upserts would overwrite a producer's other new fields. "The
  engine can already give the app the whole edited session" is true on the C ABI only until P3.
- **MINOR-3. Small open choices that #1294/#1382/P6 must agree on (note:310-313, :317-322).** The
  byte encoding of `monitor` is not defined. The split rule names only `maximum_transaction_edits`,
  not `maximum_control_frame_bytes` or the replay arena (F10). It is not stated how the queue is
  cut when an `apply`/`replaceSession` call sits between live calls.
- **MINOR-4. P4 handles only `SESSION_COMMITTED` (note:635-641).** The reliable lane also carries
  `transport_state` and `automation_canceled` events (`crates/protocol/src/controller.rs:3040`,
  `:3470`). P4 must say whether the browser forwards them or refuses their commands.
- **MINOR-5. P2 moves the canonical write into the snapshot path (note:379-380, :571).** In single
  mode that write then runs in the worklet's message handler, on the audio thread, so 4.3's
  "about 1 ms for 380 KB" understates it. P3's single-mode gate should cover it.
- **MINOR-6. Single mode on a suspended context is not discussed (note:80-81, :259).** Web Audio
  1.1 §2.6 runs the associated task queue (worklet port messages) inside the rendering loop. If an
  engine stops that loop while suspended, a single-mode `apply` cannot resolve, against "never waits
  for render". Chromium's behaviour was not checked; the note should state the contract.
- **MINOR-7. The hop probe's limits leave out the real Worker's other duties (note:515-522).** The
  design's Worker also runs the one-quantum service tick and the source drain (#1381 D6, #1387); a
  message that arrives during them waits. The probe's Worker did nothing else.
- **MINOR-8. F10 and replace leave the browser limits and the replace entry undecided
  (note:174, :666-669).** No rule derives the browser's `ControlLimits` (they set the replay arena
  that every edit clones). It is not stated how the crate's replace enters the controller (request
  ID, replay entry, frame-size limits) for a diff that can approach document size.

## NIT

- note:441-443: 93 commits lie between `0b477c585` and `8be19c86e`; 36 of them touch the named
  paths. "+636 lines" is the changed-line count (+597/-39).
- note:590: "a strip added while any track is soloed starts solo-muted": a submix is solo-safe
  (`solo.rs:255-263`); say "a track".
- note:196: "the telemetry lease (section 8)": section 8 does not discuss it.
- note:329: "worst-case latency is one apply in flight plus one" ignores the call-by-call resend
  after a refused merge.
- note:421: "fair upper bound" while listing under-counts; and the measurement is the stable
  non-atomic build, not the nightly atomics artifact of D15-10 (#1334). "Approximate size" is the
  honest label.

## What was checked and holds

- **Anchors.** About 80 `path:line` anchors on `8be19c86e`, weighted to sections 1, 2, 4, 6.4 and
  8 (C1-C11, W-row code, the acked-batch rows, F1-F10, solo, meter lease, observation kinds, SDK
  types, protocol snapshot and replay, header threading, build line, workflow line, the
  `234,436 ns` render row at `f7ba70a8b`). All say what the note claims. The claim that only
  `crates/capi/tests/resource_lifecycle.rs` changed since `6fb211594` is true.
- **Decision 15.** D1-D7 are recorded, not changed. F1 (single mode), F3 (lease), F4 and F7 are
  written as findings and amendments. Choosing thin builders is the choice the spec delegated.
- **F1 is correct.** #1382 D1 runs the shared apply in the worklet's message handler; Web Audio 1.1
  §2.6 processes the associated task queue before rendering each quantum; the measured Worker apply
  (2.66-7.38 ms p50, 64 tracks) is at or above one 2.667 ms quantum; today's `admit_commands`
  compiles nothing (`lib.rs:3318-3321`). The worklet number is an inference, and the note says so.
- **F2 is correct and important.** Every transaction reserves one reliable event
  (`controller.rs:1639`), the lane holds two (`compile.rs:128`), a full lane is refused at
  `:1735-1744`, `service` does not drain it (#1348 D1), and no browser spec mentions the lane.
- **F3 is correct.** The lease flag, activation sample and generation are render-half fields read
  by `render_next` and `poll_meters` (`lib.rs:2820-2840`, `:3256`, `:3430`).
- **Acked-batch, C ABI.** Every refusal in `SessionState::command` and `commit_live` (plan not
  alive, buffer too small, live admission and room, epoch lag, prepare, peak, pending candidate,
  reservation, report table) returns before `prepared.commit` (`control.rs:1008`); after it only
  infallible steps run. The C rows of 1.3 are true. The W rows are design claims consistent with
  #1294, #1381, #1382 and #1387 D7, except MAJOR-1 and MAJOR-3.
- **Measured costs.** The size proxy is a reasonable bound: it links all of capi and protocol,
  exports all 15 C entry points (named-twin twiggy rows present), over-counts the C ABI glue and
  keeps host-web's admission (`admit_commands` 18,327 bytes, confirmed). Baseline digest
  `a9a518625f4c...` equals the official script's. The Chromium probe (sources in
  `/tmp/claude-1002/w1057/scripts/`) measures what the note says: page `t0` to the Worker's
  post-`Atomics.store` stamp, one edit after each reply, isolated (`crossOriginIsolated` true on
  page and Worker), 500 edits, one warmup and two rounds. `probe-hop.json` matches the table. The
  #1289 re-run was justified (`prepare.rs` +597/-39 since `0b477c585`) and its report matches.
- **Primary sources.** S1 (Web Audio 1.1 §2.6 loop order, §7.4 glitch "MUST be avoided", quantum
  128), S2 ("Only shared and dedicated worker agents allow the use of JavaScript Atomics APIs to
  potentially block"), S3 (SharedArrayBuffer serialization throws `DataCloneError` without the
  cross-origin isolated capability) and S6 (port message queue is a task source) were read in the
  current specs and agree. S4, S5 and S7 agree with the specs as I know them; I did not re-fetch
  them. S6 caveat: queued port messages can be removed when a port is closed or a worker ends; the
  note covers that through the failure path.
- **Owner question.** Personal mixes stay open, with options, costs and a labelled recommendation,
  in a self-contained block. The design does not assume an answer. No other decision is pushed to
  the owner; F1's possible escalation goes through root.
- **Owner principle.** No "for now", placeholder or interim default found. P2's "if the gate cannot
  be met, root rules again" is a sequenced escalation, not a shortcut.

No tests were added (docs-only issue; the spec's test-value section says so).

## Gates run

- `git diff --name-only 8be19c86e 6ee64f484`: only the note and the spec. `git status --short` in
  the worktree: clean.
- `cargo tree -p host-web -e normal --offline` in the export: 169 lines, 66 distinct crates
  (`--prefix none --no-dedupe | sort -u`), 7 direct dependencies, `protocol` absent;
  `protocol` depends only on `engine` and `session`. Matches section 5.2.
- `bash scripts/check-workspace-policy.sh` in the export: `workspace policy: ok` (run with and
  without git metadata).
- Read-only review of the worker's evidence and probe sources under `/tmp/claude-1002/w1057/`
  (not re-run).
