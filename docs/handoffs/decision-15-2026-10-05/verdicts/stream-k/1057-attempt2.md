FAIL

# #1057 attempt 2 verdict: design note "one edit API on every host" (`45c5a1819`)

- Reviewed: the whole note at `45c5a1819`, `git diff 6ee64f484 45c5a1819 -- docs/handoffs/one-edit-api
  <#1057 spec>`, and the attempt-1 verdict. The worktree was not touched (read with `git show`;
  code read in a `git archive 8be19c86e` export under `/tmp/claude-1002/v1057/`, deleted after).
- Verifier: fresh Opus 5.5, no part in the note.
- Line numbers below are of the note at `45c5a1819` (`docs/handoffs/one-edit-api/1057-design-note.md`)
  unless a path is given; code anchors are on `8be19c86e`.

Result: no BLOCKER, two MAJORs. Attempt 2 resolves the three attempt-1 MAJORs well. The arm-tag
design is realtime-safe and answers the acked-batch question, and the same-address rule is
correct. The anchors are right. But the browser `ControlLimits` that #1381 D1 is told to state are
wrong by the note's own rule. The note could compute the right values today, and it defers them
as an "estimate" instead. Also, P2a carries a gate that its scope cannot meet.

## Attempt-1 findings: status

- **MAJOR-1 (observation arm state): resolved.** Section 8.4, F12 and P7: only render writes
  `armed`/`arm_sample`/`arm_tag`, under the slot's existing sequence lock. The slot has a single
  writer: `ObservationPublisher` is render-only (`crates/engine/src/realtime/observe.rs:121-160`),
  and the only other writer of tap state, `disarm_all`, has no production caller
  (`crates/rack/src/lib.rs:1120` is called only from tests). The writes are a few atomic stores in
  the drain (`crates/effect-contract/src/live.rs:402-412`, `:683-715`): no allocation, lock or
  wait. The design agrees with F3: the control side writes no render-read state. It agrees with
  #1381 D3: the slot moves with the publisher/reader pair that #1327 D2 swaps and #1381 D3 pairs,
  and the two arrays leave the companions. The acked-batch question is answered: the ack means
  "the record is in the FIFO", and it is "in effect" when the tag appears. Records still pending
  at a swap are drained by #1280 D3 through `ObservationLane::arm`, so P7's slot write covers them
  too (but see MINOR-3). Gaps in transport and settlement: MINOR-3.
- **MAJOR-2 (classifier cost): resolved in the decomposition**, which matches
  `crates/host-core/src/live_delta.rs:228` and `:257-263` and the evidence (`probe-phases.txt`:
  compile 1.017 ms, of which the write is 0.997 ms; classify 2.042 ms). The split of the classifier
  into two writes is an inference, but the note states it so. P2a-P2c target the right terms, and
  the timing gates are achievable. But P2a gate (2) is not achievable (MAJOR-2 below). Also,
  P2b/P2c have smaller defects (MINOR-4).
- **MAJOR-3 (SDK supersession): resolved decisively.** Cut 1 (note:379-381) never puts two values
  of one address into one transaction. So every acked call's value is in its revision's model, and
  every replacement is a later commit that `live_values_superseded` counts (D15-2 conditions 2 and
  4). The W1 row (note:115) and the P6 gate (note:821-823) state it.
- **MINOR-1 (commands): resolved** (section 5.5). The uncited second `probe_phases` run was removed
  and was not re-run. That is correct.
- **MINOR-2 (option C): resolved** (note:964-974).
- **MINOR-3 (monitor encoding, cuts): resolved** (sections 3.3 and 3.4). The field IDs are left to
  #1382, which is acceptable.
- **MINOR-4 (other reliable events): resolved** (F2, note:877-885). The adapter passes only
  `SESSION_TRANSACTION_APPLY`. `automation_canceled` comes only from protocol automation
  cancellations (`crates/protocol/src/controller.rs:1735`), and the browser makes none.
- **MINOR-5 (snapshot write in single mode): resolved as a decision** (note:487-501). Its cost is
  unquantified: MINOR-1 below.
- **MINOR-6 (suspended context): resolved** (F11, section 3.1). Checked against Web Audio 1.1
  (fetched 2026-10-05). §2.6 processes the control message queue, then the associated task queue
  ("All tasks posted from an AudioWorkletNode"), then "Process a render quantum", once per quantum
  of the system-level audio callback. The `suspend()` control message says "Attempt to release
  system resources", and "AudioWorkletNodes ... will cease to have their processing handlers
  invoked while suspended". `[[rendering thread state]]` is initially "suspended" (§1.1). The spec
  does not say outright that port messages stop. A UA that keeps the callback running would still
  run the task-queue step before the "not running → return false" check. So the note's "may" and
  its "Chromium's behaviour was not checked" are exact. F11 correctly goes to root as a finding.
- **MINOR-7: resolved** (limit 6, note:666-669).
- **MINOR-8: partly resolved.** The replace entry is decided (note:187-198). The limits rule is
  stated, but its values are wrong (MAJOR-1 below).
- **NITs: all five resolved.** 93 commits, +597/-39 (both confirmed); "a track"; the telemetry
  lease mention is gone; the resends are counted; "approximate size", stable non-atomic build.

## MAJOR

**MAJOR-1. The browser `ControlLimits` values contradict the note's own rule, and the note
defers as an "estimate" a value it can compute today (note:415-433, F10 at :911-915, order edges
:713-717).**
The rule says the frame "holds `maximum_transaction_edits` (1,024) live edits of up to 128 bytes
each". So the note sets 131,072 / 262,144 and has #1381 D1 "check it against P1's encoded sizes".
I measured the real encoder (scratch test of `ProtocolCodec::encoded_session_transaction_len` in
the export; per-edit size is the frame of two edits minus the frame of one):

| Edit | 3-byte IDs | 16-byte IDs | 127-byte IDs (grammar maximum, `crates/session/src/id.rs:10-14`) |
|---|---|---|---|
| `UpsertEffectParam` (the SDK's live effect-parameter edit, note:350-353) | 176 B | 192 B | 416 B |
| `SetTrackFader` | 144 B | 152 B | 264 B |
| `SetRouteGainDb` (ID + one value) | 80 B | 88 B | 200 B |
| frame of 1,024 `UpsertEffectParam` | 180,272 B | 196,656 B | 426,032 B |

- Today's live effect-parameter edit is already above 128 bytes with the shortest IDs. A P1
  setter (ID + lane + value + ramp) is about 112 B with a short ID, about 232 B at the longest.
  By the note's own rule ("raises the frame limit to the next power of two"), the frame limit is
  524,288 bytes and the replay entry is 1,048,576 bytes. Every input is known today: the 127-byte
  ID grammar, the 8-byte TLV header and padding and the 8-byte `MESSAGE` prefix
  (`docs/CONTROL_BTLV_V1.md` "TLV encoding"), and the setter shapes.
- The check that the note gives to #1381 D1 covers only "P1's encoded sizes". So even when it is
  applied faithfully, it misses the effect-parameter edit. No order edge puts P1 before #1381
  (note:713-717; P1 is stream B, #1381 is stream H). So #1381 can ship the 131,072 "estimate",
  and a later issue then has to change it. The owner's no-shortcuts principle forbids exactly
  this ("no interim placeholder that a later issue must undo; defer only when a dependency forces
  the order"). Here no dependency forces it.
- Effects as written: in the browser, a valid transaction of fewer than 1,024 edits is refused by
  the frame limit. The C ABI's default is 1 MiB. The merged batches are 2-4x smaller than the note
  says. The "256 KiB per edit" clone cost (note:426-427) is understated by the same factor.
Fix: derive the values now, from the largest encoded live edit at the 127-byte ID
(`UpsertEffectParam`, and P1's shapes as specified). Or state the rule as
"`maximum_transaction_edits` × the largest encoded live edit, rounded up to a power of two", with
the computed numbers and the edit kinds it covers. Then add the order edge P1 → #1381 D1, or keep
the computation independent of P1's code.

**MAJOR-2. P2a gate (2) cannot pass within P2a's scope (note:739-755, gate at :753).**
"A value-only classification allocates nothing (`bench_support::alloc` counters, exact 0)". But
`classify_live_delta` returns `LiveDelta`, whose `strips` and `effects` are `Vec`s that it pushes
into (`crates/host-core/src/live_delta.rs:107-113`, `:315`, `:395`). Effect records and EQ targets
allocate (`:407`, `:444`, `:456`, `:475-489`, and the doc at `:202-206` names "the lowered racks,
the launch registry, the resolved values and an EQ's target designer"). P2a's scope removes the
clone and the two writes, and it narrows the loops to the named entities. It does not change the
output storage, the effect lowering or the EQ designer. Those are other crates, and `commit_live`
consumes the output (`crates/capi/src/runtime/control.rs:1065-1100`). No rule asks for this
gate: D15-10 counts single-mode control allocations and accepts them, and the apply still clones
the model in `prepare_transaction`. AGENTS.md forbids weakening a gate to close an issue. So a P2a
filed from this text fails or has to be re-briefed. This is the same defect class as attempt-1
MAJOR-2.
Fix: drop gate (2). Or restate it as "no allocation proportional to the session (none sized by
the model or by its canonical text); allocations are bounded by the edit count". Or add the
output-storage redesign to P2a's scope explicitly, with its crates.

## MINOR

- **MINOR-1. The latency of the single-mode snapshot is not bounded or gated (note:487-501, P3 at
  :788-795).** The revision pinning is correct. The SDK barrier and the one-message-in-flight rule
  mean that nothing commits while a snapshot is in flight. Service ticks commit nothing (#1348 D6).
  The check-and-restart catches anything else. But "one slice per handler call", continued only on
  service ticks, takes about one tick per entity. A 64-track document has a few hundred entities.
  The tick is a main-realm `setInterval` (#1381 D6), which browsers clamp to at least 4 ms when
  nested and throttle to at least 1 s (or less often) in hidden tabs. So a single-mode save can
  take seconds or minutes, for example an autosave on `visibilitychange`. Neither the note nor P3
  states or gates the total. Write slices up to a time budget per handler call, continue on
  messages (they are not throttled like timers), and give P3 a gate on the total snapshot latency.
- **MINOR-2. The single-mode budget gates cannot be measured as worded (note:781-786; P3 at
  :793-795).** Each gate sums two p99s of sub-millisecond quantities measured inside the worklet.
  The repository's own harness records that Chromium's `AudioWorkletGlobalScope` exposes no
  `performance`, so the clock is `Date.now()` at 1 ms resolution
  (`hosts/host-web/qualification/rebuild-cost.mjs:10-11`;
  `hosts/host-web/web/miso-engine-v1-audio-worklet.js:119-132`). A non-isolated page also has no
  `SharedArrayBuffer` clock. With 1 ms quantization, the gate cannot tell 0.3 ms from 1.9 ms.
  These gates decide F1. Name the method, for example an isolated page with single mode forced and
  a SharedArrayBuffer clock, or timing batched over many edits per call.
- **MINOR-3. The arm tag's transport and settlement are open (note:178, :826-839, :847-849,
  :1064-1068, :1082-1090).**
  - #1294 D1's reply `miso.edit.v1 {tag, requestId, result, revision, path, diagnostic}` and
    #1293's fixed outcome record have no field for `arm_tags`. The amendment table changes #1382
    ("the reply carries the arm tags") but not #1294 or #1293. One way: tags are consecutive per
    call, so the reply could carry the first tag and a count.
  - The observation read record that must carry `{armed, arm_sample, arm_tag}` to the SDK (the
    ABI layout and the generated SDK types) is not in P7's scope.
  - The note does not say how `armed()` settles on close, on disposal, on removal of the owner,
    or when the slot already shows a newer tag.
  - The note does not name the result code of the refusal before wrap, or say why the tag is
    `u32` and not `u64`. The 64-byte record bound allows `u64`, which removes the refusal path.
  - The Monitoring row (note:128) and the gates of P5 and P7 do not cite #1280 D3, which carries
    a pending `Observe` record of a carried owner across a swap.
- **MINOR-4. The P2b and P2c sketches do not meet their own gates (note:756-774).**
  - P2b writes no text "for a value-only commit". But the compile (C4) runs before the classifier
    (C5), so the compile does not yet know which commits are value-only. State the predicate, for
    example the incremental length for every entity-scoped transaction.
  - P2c stages "through the reservation used by preflight and complete". But `preflight` evicts
    and writes the request into the arena (`crates/protocol/src/controller.rs:402-410`). With the
    browser's single entry, gate 3 ("a refused transaction leaves the cache byte-equal") then
    fails, unless the eviction and the copy are deferred to the commit.
  - Every transaction also allocates and zeroes a `max_response_bytes` response buffer
    (`controller.rs:1613`). That is another per-edit cost proportional to the frame limit. The
    note does not name it, and it is unclear whether P2c's "staging" count includes it.
- **MINOR-5. The SDK queue has no bound (note:372-413).** Cut 1 costs one apply per call on a
  control that moves fast. When input comes faster than the applies (single mode before P2a-P2c,
  or a slow device), the queue and the latency grow without limit until the input stops.
  "pendingCalls lets apps coalesce" moves the duty to every app. AGENTS.md wants bounded queues
  with typed backpressure. Bound the queue with a typed refusal, or let the SDK replace a queued,
  unsent call with an explicit non-ack outcome. That outcome is not D15-2 supersession, because
  nothing was committed.
- **MINOR-6. The replace entry has no owner (note:187-198, :851).** A typed entry with "no replay
  entry" needs a new commit path in `protocol`. Today `commit_prepared_structural` installs the
  prospective replay cache (`controller.rs:1628-1660`, `:1974`). #1386's authorized paths are
  host-web only, and the amendment names `control-plane` but not `protocol`. The note also does
  not say what `response_len` holds for a replace.

## NIT

- note:495-496: "a few hundredths of a millisecond natively" per slice is an unlabelled inference.
- note:405-407: "Input events come at most once per display frame per control" is not guaranteed
  (`pointerrawupdate`, key repeat, calls made by the app's own code).
- note:379-381: cut 1 does not say how a `Both`-lane address overlaps `Left`/`Right`
  (`EffectParam.channel`, or any P1 setter that takes both lanes).
- note:376-378: a call merged with a call that the classifier sends to a rebuild reports
  `rebuild`. That is correct, but the note does not say it.
- P7 (note:830-839) does not list the resource-report rows that its deletion moves
  (`hosts/host-web/src/lib.rs:7059-7090`) or the slot's retained-bytes formula.

## What was checked and holds

- **Scope.** `git diff --name-only 436cc137d 45c5a1819` lists only the note and the #1057 spec.
- **D1-D7 not reopened.** F1 and F11 (D6, D15-10) go to root as findings. Replace as a typed
  entry is consistent with D4. The other changes are amendments of specs, not decisions.
- **No other interim shortcut found.** The pre-P2b and pre-P2c costs are sequenced (order edges
  at note:713-717). The only placeholder is MAJOR-1.
- **Anchors.** About 75 checked on `8be19c86e`, mostly the new ones. Examples:
  - `live.rs:81-88`, `:402-412`, `:683-715`; `observe.rs:1-30`, `:75`;
    `effect-contract/tests/live_control.rs:48`;
  - `lib.rs:668-680`, `:851`, `:853`, `:869`, `:1602`, `:1607`, `:1828-1853`, `:2336`, `:3337`,
    `:3388`, `:3683`, `:5874-5885`, `:7022-7069`, `:7164-7165`;
  - `controller.rs:146`, `:214`, `:268-276`, `:437-461`, `:1631`, `:1639`, `:1735-1744`,
    `:3040`, `:3470`; `capi/compile.rs:128`, `:737-741`; `session/compile.rs:130-131`;
    `solo.rs:125-129`, `:160`, `:255-263`;
  - `observation-subscriptions.ts:101`, `:112`, `:549`, `:627`; `shipped-host.d.ts:664-665`;
    `boundary.ts:1416`; `live-controls.ts:28-30`, `:432`, `:960`, `:976`;
  - the `1305` spec at `:103`; the capi control/ffi/plan anchors; the worklet and host JS
    anchors; `miso_engine_v1.h:18-22`.

  All of them say what the note claims.
- **Personal-mix owner question.** It is still OPEN (note:940-997) and self-contained: background,
  three parts, options with costs, and a recommendation labelled "not a decision". The design does
  not assume an answer.
- **Measured numbers.** They come from the attempt-1 evidence, with commands now in section 5.5.
  `probe-phases.txt` matches section 5.4.

No tests were added (docs-only issue; the spec's test-value section says so).

## Gates run

- `git diff --name-only 436cc137d 45c5a1819`: only the note and the spec.
- `bash scripts/check-workspace-policy.sh` in a `git archive 45c5a1819` export, with and without
  git metadata: `workspace policy: ok` (exit 0) both times.
- Web Audio 1.1 (`https://www.w3.org/TR/webaudio-1.1/`, fetched 2026-10-05): §1.1, §1.2.3 and §2.6
  read in full text.
- A scratch encoded-size test (`crates/protocol/tests/v1057_size.rs` in the export only,
  `CARGO_TARGET_DIR=/tmp/claude-1002/v1057/target cargo test --offline --locked -p protocol
  --test v1057_size`). Results are in MAJOR-1. The export and the target were deleted after the
  run.
