FAIL

# #1345 attempt 2: adversarial verdict

Under review: `git diff 17b641a66 bbc6b6e22` (commits `01f859ba1`, `f05ef8066`, `aacf6afaa`, `bbc6b6e22`;
the spec-only `eab062557` ignored), and the whole #1345 change (`710f77df7..bbc6b6e22`) against the spec
with Amendments 1-3 and the attempt-1 verdict. I exported `bbc6b6e22` with `git archive` to
`/tmp/claude-1002/v1345b/tree`, git-initialized it, and built it with private target dirs. I read
nothing in the worktree except through `git diff`/`git show` of the named hashes. Two helpers ran the
gate sets and the mutation probes in their own copies; I checked their key results against the code.

**Summary.** Amendment 3's engine side is exact, M2 is fixed and exact, D8 is met, and every finding
m1-m12 and the NITs of attempt 1 is resolved or honestly recorded. Every gate job is green. The attempt
fails on one MAJOR finding: the SDK retry added for gate 6 bullet 3 is an unbounded wait. On the
headless engine a caller that awaits `close()` (or `subscribe`/`update`) before it renders waits forever,
and Node exits silently with the promise unsettled. This is not the writer's policy that the ruling
names. Root must rule on the wait shape (see "SDK wait: hang, reorder, alternatives").

## MAJOR

**M1. The observation owner's backpressure retry waits without bound. On the headless engine an awaited
`close`, `subscribe` or `update` deadlocks a caller that must render itself.**
- Where: `sdk/src/core/observation-subscriptions.ts:871-896` (`#submit` loop), the headless hook
  `sdk/src/headless/engine.ts:368-373` with `#renderWaiters` (`:122-123`, released at `:326-335`, `:350`,
  `:362`), and the browser hook `sdk/src/browser/engine.ts:996-1006`.
- Probe (helper, unmutated artifact, `/tmp/claude-1002/v1345b/probe/`):
  - `subscribe` (window 2), then `handle.close()` with no render: still pending after 2 s. It neither
    resolves nor rejects. While it is pending, `subscribeObservations` and `pump()` reject with
    `MisoUsageError: an observation subscription operation is already in flight` (the `#enqueue` busy
    guard, `:1052-1060`). After one `render()` the close resolves.
  - A plain script `await handle.close(); engine.render();`: Node prints "Detected unsettled top-level
    await" and exits with code 13. The line after the await never runs. In a process with other live
    handles (a server, a test runner) it is a silent hang.
- The parent (`710f77df7`, 64-record shared queue) resolved this sequence at once; attempt 1 threw a
  typed `MisoEngineError` with code `backpressure`. Attempt 2 turns a typed refusal into a silent
  unbounded wait. The `subscribeObservations` doc (`headless/engine.ts:221-227`) states the trap ("a
  caller that awaits it must render from elsewhere"), but for a caller-driven, single-threaded engine
  "from elsewhere" means "do not await", which is the natural use of a Promise API.
- It is not the writer's policy. `writer.ts:245-253` and `:313-331` classify backpressure as flow
  control and **return** a refused outcome at once; the caller decides when to retry, and `drain()`
  (`:334-349`) is bounded by attempts and says why: "the queue is full and nothing is rendering is a
  legitimate steady state -- a paused transport -- and a writer that spun there would be a busy loop
  rather than a retry". The owner shares the writer's classification (retry only backpressure) but not
  its contract: it blocks on a render that a paused or caller-driven transport never makes.
  `MAXIMUM_ADMISSION_RETRIES = 8` (`:46`) bounds renders, not time.
- Browser: `nextRender` polls `host.status()` every 4 ms until `renderedQuanta` moves or the engine
  closes (`browser/engine.ts:998-1005`). It is not a CPU busy loop (a timer plus one port round trip per
  pass), but it has no time bound: while the AudioContext does not render (suspended before a user
  gesture, or paused), an awaited subscribe/update/close polls about 250 status requests a second for
  as long as that lasts. The engine already has a deadline vocabulary (`requestDeadlineMs`, default
  5,000 ms, `browser/engine.ts:691`) that this wait does not use. Browsers are not on this machine, so
  the browser wait is read from the code, not run.
- Gate 6 bullet 3 ("Subscribe, then close, with no render between them: both resolve") is green only
  because the eval (`sdk/test/observation-subscription-evals.mjs:255-287`) does not await the close
  before it renders. Root's Amendment 3 prescribed "retry after the applied watermark moves", so the
  implementer followed the text; as with attempt 1's M1 this is a spec-level defect: the owner principle
  does not allow a shipped API that can wait forever without an error. Fix after root's ruling below.

## MINOR

- **m1. No test holds "retry on backpressure only".** Mutation: drop `report.reasonName !== "backpressure"`
  from `#submit` (`observation-subscriptions.ts:890`), so every refusal waits and retries up to 8 renders:
  367/367 SDK evals stay green. Under that defect a malformed or unbound managed submission on the
  headless engine would wait (and, awaited, hang) instead of throwing at once. Add an eval in which a
  non-backpressure refusal (for example `observationUnbound` or a stale owner) rejects with no render.

## NIT

1. Stale "command queue" prose for the observation queue: `crates/host-core/src/prepare.rs:344` ("rides
   the effect's existing command queue"), `:1477` ("rides the effect's own command queue"),
   `crates/effect-compiler/src/prepare.rs:62` ("drains its queue"). `hosts/host-web/web/miso-engine-v1-audio-worklet-host.js:1181`
   ("the worklet-side queue depth") is the implementer's open item 3; that file is outside the paths.
2. `hosts/host-web/src/tests.rs:6969-6977` (`a_submission_names_each_observation_tap_once`) claims red if
   the check "runs after the room pass or a cell write (the fader value or the bus below would move)".
   Mutation 2 below (check moved after `write_strip_cells`) is red, but only because the room pass then
   answers BACKPRESSURE first: with launch effects (one tap, one slot) a duplicate always overflows the
   queue, so the check's position relative to the room pass is not observable and the fader assertion is
   never what fails. The doc should say the catch is the reason code.
3. `tools/parameter-metadata/tests/round_trip.rs:597-600` keeps one render between a subscribe and its
   unsubscribe. They are two submissions; Amendment 3 accepts that the second is BACKPRESSURE until a block
   drains the first, and one submission may not name the tap twice. This is the accepted residual, not a
   workaround, and the comment says so. Asserting the BACKPRESSURE before the render would document the
   residual instead of stepping around it (optional).

## SDK wait: hang, reorder, alternatives

**(1) Hang, reorder, ack-before-drop.**
- **Hang: yes**, as M1. Headless: an awaited subscribe/update/close whose submission meets an occupied
  tap slot never settles unless something else calls `render()`, `loadSession()` or `dispose()` (those
  three release the waiters; `dispose` and reload also bump the epoch, so the retry then throws the
  stale-owner error, eval `:289-301`). Every other owner operation rejects "already in flight" meanwhile.
  Browser: unbounded in time while the context does not render.
- **Reorder: no.** `#enqueue` serializes every owner operation and refuses a concurrent one
  (`:1052-1060`), so at most one managed submission is outstanding and a retry resends the same edits
  before any later operation runs. Per tap and across taps, managed records reach the engine in operation
  order. A caller mixing raw `liveControls.submit` or a `LiveControlWriter` carrying `observe` edits with
  the owner can interleave, as before this change.
- **Double submit: no.** A refused submission admits nothing (all-or-nothing staging, then
  `discard_owner`/rollbacks in `admit_commands`, `hosts/host-web/src/lib.rs:4660-4700`), so a resend
  after a refusal cannot apply twice. A report that is not a refusal is never resent.
- **Ack before drop: no.** The duplicate check refuses in staging (`lib.rs:5345-5365`), before the room
  pass (`:5887-5906`), `write_strip_cells` and the stamp; the retry only follows a refusal that admitted 0.

**(2) Alternatives.**
- **(a) Headless refuses at once with a typed, retryable error; only the browser waits.**
  - Fits the existing design. The typed error already exists: `commandFailure` (`observation-subscriptions.ts:380-386`)
    builds a `MisoEngineError` whose `code` is the report's (`backpressure`), which is what attempt 1
    threw. The writer behaves the same in both engines: flow control is returned, not waited on
    (`writer.ts:313-331`), and a caller-driven drain is bounded (`:341-349`). Nothing in `writer.ts` waits
    for a render in either engine.
  - Changes: remove `nextRender` and `#renderWaiters` from `sdk/src/headless/engine.ts` (the transport
    field is already optional, `observation-subscriptions.ts:150`, and absent means "throw"); state in
    the `subscribeObservations` doc that a backpressure refusal is typed and clears after one `render()`.
    Keep the browser wait, but bound it in time (the engine's `requestDeadlineMs`, or the 8-render cap
    plus a deadline) and throw the same typed backpressure error when it expires, so a suspended context
    gives an error, not an endless poll.
  - Gate 6 bullet 3 must be amended per engine: browser, both resolve; headless, the close rejects with
    typed `backpressure`, admits nothing, and the same close resolves after one render (the handle stays
    open, `state.closing` is cleared on rejection, `:728-732`, so a retry is already possible).
- **(b) The owner resolves close (and update) at once on SDK state and sends the record later, FIFO per tap.**
  - Does not fit. The owner keeps bindings, ref counts, handles and one serialized mutation chain
    (`#bindings`, `#handles`, `#mutation`, `#mutationBusy`) but no outbound record queue. `subscribe` and
    `update` return receipts whose `appliedAtSample` comes from the admission report (`:566-567`,
    `:649-651`), so only `close` (receipt `void`) could resolve early without changing the receipt
    contract.
  - It would need a per-tap pending-record queue drained on renders or pumps, with every later operation
    on that tap ordered behind it, and a channel for a later refusal: there is none today, so a deferred
    refusal could only surface on a later `pump()`/`onUpdate` or as an owner error state, which breaks the
    rule that a refused operation is refused to its own caller. Meanwhile the engine keeps an armed tap the
    SDK has forgotten.
- **Recommendation: (a).** A caller-driven engine must not wait for something only the caller can do;
  the writer contract already says a paused transport is a steady state answered with a typed refusal.
  The browser keeps the convenience of waiting because its renders run on their own, with a time bound so
  it also ends in a typed error. (b) adds a second queue and an orphan-error channel to hide one block of
  flow control.

## Judged points

1. **Amendment 3 (M1 of attempt 1): met on the engine side.**
   - `hosts/host-web/src/lib.rs:5345-5365`: a per-effect tap bit mask (`observe_named`, `:1572-1577`,
     cleared at `:4940` with `command_wanted`, sized `total_effects` at `:7416`) refuses a second `Observe`
     for one (strip, rack, effect, tap) with `COMMAND_REASON_MALFORMED` at that record's wire index, in
     staging, before the room pass, any cell write and the stamp. Nothing coalesces; the push loop queues
     Observe records in wire order. `checked_shl` refuses a tap index >= 32 as malformed; preparation and
     boot bound taps at 16 (`MAXIMUM_OBSERVATION_TAPS`), so that path is unreachable.
   - The reason docs are widened in `lib.rs:1022-1023` and both `.d.ts` twins (byte-identical, `cmp`).
     No new constant; the ABI layout and generated `abi.ts` do not change (`check-sdk-generated`, ABI
     layout gates green).
   - SDK `update()` sends one re-arm for a window change on a tap the handle alone holds
     (`observation-subscriptions.ts:622-630`); duplicates in one request are refused by
     `validateObservationSelections` (`sdk/src/core/observation.ts:177-190`, key without channel).
   - Gate 6 bullets 1-3 are real-engine evals (`observation-subscription-evals.mjs:193-287`), plus a
     dispose eval (`:289-301`). The one-submit re-arm in `observation-evals.mjs:171-180` is restored (one
     `observe(true, 2)`, no withdrawal, no render before it).
   - The C ABI has no Observe path (`observation_taps` 0); unchanged, correctly.
   - The render-between-records workarounds of attempt 1 are gone from `observation-evals.mjs` and the
     host-web tests; `round_trip.rs` keeps the accepted residual (NIT 3).
2. **The SDK retry:** see M1, m1 and the section above. Not the writer's policy; hangs; no reorder,
   double submit or ack-before-drop; browser poll non-busy but unbounded.
3. **D8: met.** `crates/effect-contract/src/live.rs:1032-1054` `drain_observations` is the direct form:
   `if let Some(observe) = self.observe.as_mut() { let available = observe.available_at_entry(); ..
   while remaining != 0 { remaining -= 1; let Ok(record) = observe.try_pop() else { break }; .. } }`.
   The `.map(Consumer::try_pop)`/`map_or(0, Consumer::available_at_entry)` forms are gone; the count and
   the pop share the receiver (#1426). It sits in the `live.rs` region opened at `:844`.
   `check-realtime-policy.sh`: `ok (106 marked regions in 27 files)`, unchanged count, self-test green.
   With #1418 attempt 2's gate (floors raised to 27/106; its stale unsafe-file allowlist lines disabled,
   which concern files that gate predates, not D8) on a scratch copy: `realtime policy: ok (106 marked
   regions in 27 files)`.
4. **M2 of attempt 1: fixed and exact.**
   - `crates/graph-compiler/src/estimate.rs:280-314` charges `queued_lanes * size_of::<(usize,
     Consumer<EffectControlRecord>)>()` (48 B), matching `rack::LiveControlEffectBankStage::new`.
   - Reference rows, re-measured here (`cargo test -p capi --test resource_lifecycle
     reference_session_retained -- --nocapture`): 405,798 / 405,774 / 197,601 / 281,513 / 90,720 against
     ceilings 421,248 / 421,248 / 203,648 / 282,432 / 99,840. The move is -640 B = 2 banks x 8 lanes x
     40 B. No ceiling file is in the diff. `docs/C_ABI_V1_QUALIFICATION.md:474-476` records them.
   - New tests are exact equalities against the audited allocator (see test value).
5. **Attempt-1 findings.** m1 (`.max(records)` removed from `prepare_caps`, `lib.rs:6797`; option docs
   `:1227-1236`; the boot word stays, open item 4), m2 (both twins), m3, m4 (header "same cell"; per-lane
   vs shared cell counts; reflowed to 100 columns), m5 (doc corrected and the preparation test extended
   to write and stage every cell), m6 (new graph test), m7 (rack test deleted), m8 (lib test uses the
   allocator-held accounting and checks attach equals direct construction), m9 (exact
   `kept - 3 * PREROLL_BLOCKS`), m10 (`[1, 2, 0]` and the paused split asserted), m11 (deviation stated
   in the test doc and the record), m12 (new capi and host-web tests): all resolved. m13: see point 6.
   NITs 1-11 resolved or stated (NIT 9 prose only, unchanged; new stale prose in NIT 1 above).
6. **Files outside the path lists, for root to ratify.**
   - Attempt 2: `sdk/src/headless/engine.ts` and `sdk/src/browser/engine.ts` (forced by the `nextRender`
     design; under alternative (a) the headless edit goes and the browser edit stays);
     `sdk/src/core/writer.ts` (prose, NIT 1 of attempt 1); `sdk/test/writer-evals.mjs` (m10);
     `tools/console-workload/{src/lib.rs,tests/automation.rs}` (m9, prose); `tools/audit/src/capi.rs`
     (prose); `tools/parameter-metadata/tests/round_trip.rs` (comment); `crates/graph-compiler/`
     (M2, forced); `crates/host-core/src/{live_delta.rs,prepare.rs}` (prose);
     `docs/rulings/live-updates-...-2026-10-05.md` (D15-2 parenthetical, NIT 3 of attempt 1).
   - Attempt 1 (m13, unchanged): graph-compiler (STREAMS hot-file row `:95`), `host-core/src/prepare.rs`
     (`:75`), the callgraph script (`:110`), the ABI self-test fixture (C #1406's exception), and the
     remaining attempt-1 SDK and test files. STREAMS' "Watermark bound on `main` today" still needs root's
     text (open item 2). `docs/handoffs/.../STREAMS.md` changed in the range only through the spec-only
     `eab062557`.
7. **Doc set: true of the code.** Header (`miso_engine_v1.h:49-75`), the AGENTS.md sentence (watermark
   sentence, exactness clause removed in attempt 1), D15-2's parenthetical, `C_ABI_V1_QUALIFICATION.md`
   (#1256, #1263-#1266 sections, rows), both `.d.ts` twins (byte-identical; cells, the one-record rule,
   BACKPRESSURE "a render clears it"). The twins' boot-option doc now says the number is the main-realm
   in-flight bound, which `worklet-host.js` still uses that way. Exceptions: the NIT 1 lines.

## Test value (new or rewritten tests)

- host-web `a_submission_names_each_observation_tap_once`: red if the duplicate-tap refusal goes (the
  batch becomes BACKPRESSURE, a retry that can never succeed); unique with the two rewritten tests below.
  Mutation 1: red, with `idle_render_skips_admission_counter_clear_without_losing_queue_credit` and
  `observation_misuse_is_typed_and_all_or_nothing`.
- host-web `observation_misuse_is_typed_and_all_or_nothing` (rewritten): red if a duplicate is answered
  BACKPRESSURE or partly applied, or if a later submission into the occupied slot is not BACKPRESSURE.
- host-web `idle_render_skips_..._queue_credit` (rewritten): red if a refused single submission leaves
  pending ownership; its refusal is now the duplicate (mutation 1 red).
- host-web `a_flood_of_effect_values_is_admitted_whole_and_the_last_is_heard`: red if an effect value is
  again charged against a room, or a later bypass record of one batch is lost to an earlier one.
- host-web `command_flood_is_typed_backpressure_...`, `a_mixed_batch_is_one_transaction_across_every_queue`,
  `a_refused_solo_submission_...`, `overfilling_a_bus_queue_...` (rewritten to fill the slot with an
  earlier submission): red if a full observation queue does not refuse the whole later batch (cells
  included) at the subscription's index; the parent's catches, kept under the one-record rule.
- SDK gate 6 bullet 1 (`:193`): red if `update()` sends disarm plus arm for one tap (mutation 3: the only
  red, "invalidArgument").
- SDK gate 6 bullet 2 (`:229`): red if admission answers a duplicate with BACKPRESSURE or applies part of
  it (mutation 1 in the rebuilt module: the only SDK red, reason 8 for 1).
- SDK gate 6 bullet 3 (`:255`) and the dispose eval (`:289`): red if the owner throws on backpressure
  (mutation 4: both red, the only reds). They hold the wait M1 questions; under alternative (a) bullet 3
  is rewritten.
- SDK `writer-evals.mjs` paused split (rewritten): red if a writer batch is taken from the back of the
  pending map (mutation 6: the only red).
- `observation-evals.mjs` re-arm (restored): red if a one-record re-arm of an armed tap is refused.
- graph-compiler `an_observed_lanes_queue_is_charged_exactly_what_bind_retains`: red if a tapped bank's
  queue table is charged at any size but what bind retains (mutation 7: red, 5,888 for 6,656; the lib
  test also goes red on that formula through the untapped over-charge, so this test's unique catch is the
  tapped under-charge, which breaks the ceiling).
- graph-compiler `a_lanes_own_accounting_is_what_the_allocator_sees`: red if a lane's `retained_resources`
  omits or over-counts an array (mutation 8: red 3,664 vs 3,672; the existing ceiling test catches only
  the under-count, so the exact equality is the unique catch for an over-count).
- graph `a_per_node_effect_waits_for_the_revision_that_covers_its_value`: red if the per-node stage reads
  under `LiveSnapshot::ALL` (mutation 9: the only red across graph, capi, host-core, host-web).
- capi `one_transaction_with_more_cells_than_the_old_queue_held_stays_live`: red if a transaction is
  rebuilt for outgrowing a 16-record bound (mutation 10a: the only red; at 3, three older tests also red).
- effect-compiler `every_launch_effect_is_prepared_with_a_window_for_all_its_live_cells` (extended): red if
  preparation stops raising the window to the live cells (mutation 11: the only red).
- effect-contract `subscriptions_apply_in_order_and_a_full_queue_refuses_the_newest` (extended): red if a
  disarm does not carry its window or order stops deciding the final state (mutation 12: the only red).
- console-workload `the_eq_lowering_keeps_the_collapse_when_a_pair_lands_in_one_block` (tightened): red if
  a split pair retires a different number of cohort blocks than the three EQ cohorts for the preroll.
- Deleted tests (rack m7, `paired_spans` duplicate, `live_control` rig-loop assertion): attempt 1's
  verdict showed no unique catch; deletion is correct.
- Mutation 13 (re-adding the `.max(records)` coupling) turns nothing red: expected, it is a resource
  coupling under ceilings, no test is owed.

## Gates run (export of `bbc6b6e22`, this machine, no browsers, no AArch64)

| gate | result |
|---|---|
| `lint`, all 32 steps (fmt, clippy `-D warnings`, rustdoc, every policy script and its mutation test, realtime `ok (106 marked regions in 27 files)` and self-tests, leak checks, rack/builtins/graph/effect-runtime policies, conformance, EQ seal, release shape, npm publish, stem store and identity, the sub-v3 and AVX2+FMA probes) | PASS |
| `test-debug-a` (1,573 passed, 0 failed; doctests 24) | PASS |
| `test-debug-b` (914 passed; doctests 7; `conformance_fixtures --check`) | PASS |
| `test-release` (lane/math/wasm-gates 133; filter_liveness, tail_contract, ramp_endpoint, designer_total; FMA cfg; loom 13; M1 and F1 exhaustive) | PASS |
| `audit-native`, every step (`audit capi` 100,000 calls, all counts 0, `pcm_digest cb10fbface44a3a4`, validator; effect, builtins, graph traces; protocol audit; realtime probes; `check-capi-abi.sh` + self-test; scalar-oracle `--native`; graph determinism 100/100; builtins fixtures; console fixtures; conformance bench) | PASS |
| `check-cross-targets.sh` | PASS (the declared #1018 expected rows only) |
| worklet chain: `build-web-audioworklet.sh --named-twin`, `strip-wasm-names` self-test + check, `check-web-audioworklet.sh --without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts`, `check-scalar-oracle-absent.py --wasm`, `test-web-audioworklet.sh` (TMPDIR left empty), V8 spill self-test + gate | PASS |
| `check-abi-layout-v1.py` on the shipped layout and `--self-test` (22 mutations caught) | PASS |
| `sdk` job: `npm ci`, `check-sdk-generated`, `check-sdk-deletions` (+ self-test), `check-sdk-types`, `check-sdk-headless` (367/367), `sdk-package.sh check`; `sdk/dist` deleted | PASS (no separate source-bundle switch exists in these scripts; `check-sdk-headless` builds and points the evals at the artifact) |
| reference resource rows (`resource_lifecycle reference_session_retained`) | PASS, rows as recorded, no ceiling moved |
| #1418 attempt-2 realtime gate on a scratch copy (D8, gate 10) | `realtime policy: ok (106 marked regions in 27 files)` |
| 14 mutation probes and the hang probe (above; scripts, patch script and log tails kept in `submix-verdicts/evidence/1345-attempt2/`) | as listed |

Not run: the `browser` job legs and Playwright (no browsers), AArch64 jobs, `wasm-guests` (no `wasmtime`).
