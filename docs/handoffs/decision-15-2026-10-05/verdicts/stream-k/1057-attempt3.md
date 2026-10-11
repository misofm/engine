PASS

# #1057 attempt 3 verdict: design note "one edit API on every host" (`c63f5f37d`)

- Reviewed: the whole note at `c63f5f37d`, `git diff 45c5a1819 c63f5f37d -- docs/handoffs/one-edit-api
  <#1057 spec>`, the attempt-1 and attempt-2 verdicts, AGENTS.md (at `c63f5f37d` and on `main`),
  decision 15 (D15-1, D15-10, D15-11), the no-shortcuts owner rule, and the specs #1280, #1293,
  #1294, #1305, #1332, #1381, #1382, #1394.
- The worktree was not touched. Files were read with `git show`; code was read and built in a
  `git archive c63f5f37d` export under `/tmp/claude-1002/v1057/` (deleted after).
- Verifier: fresh Opus 5.5, no part in the note.
- Line numbers are of the note at `c63f5f37d` (`docs/handoffs/one-edit-api/1057-design-note.md`)
  unless a path is given. Code anchors are on `c63f5f37d`, which has the same code as `8be19c86e`
  and `50fb23b5f` (checked below).

Result: no BLOCKER and no MAJOR. The two attempt-2 MAJORs are resolved decisively and correctly.
Six MINORs and four NITs remain. MINOR-1 to MINOR-3 are gate wording in proposed issues P3 and P2b.
Fix them before those issues are filed, because as worded each gate would fail or not run.

## Attempt-2 findings: status

- **MAJOR-1 (browser `ControlLimits`): resolved.**
  - I measured `ProtocolCodec::encoded_session_transaction_len` again (scratch test in the export,
    command below). `UpsertEffectParam` is 176 B per edit at 3-byte IDs and 416 B at 127-byte IDs.
    Its frame of 1,024 edits is 426,032 B. The console rack (`RackName::Console`) gives the same
    sizes, extreme `f32` and `u32` values change nothing, and `SetEffectBypass` is 352 B. All of
    these equal note:487-490 and the table at :817-826, and they agree with
    `/tmp/claude-1002/w1057b/evidence/sizes.txt`.
  - The rule gives 524,288 B. The replay bytes of 1,048,576 B are the minimum that
    `plan_preflight` accepts (`crates/protocol/src/controller.rs:356-361`: request length plus
    `max_response_bytes` must fit the arena). `max_response_bytes` equals the frame limit, as on
    the C ABI (`crates/capi/src/runtime/compile.rs:740`). The headroom statement is correct: the
    largest edit that fits is 504 B.
  - The live-edit list is complete. It matches every method of the SDK live controls
    (`sdk/src/core/live-controls.ts`):
    - strip: `pan`, `matrix`, `faderDb`, `mute`, `trimDb`, `polarityInvert`, `hpfHz`, `lpfHz`,
      `inputFilters`;
    - track: `solo`;
    - route: `gainDb`, `mute`, `matrix`;
    - VCA: `faderDb`, `mute`;
    - effect: `parameter`, `bypass`, `observe`.

    Each method maps to a listed edit, to a P1 setter or to a monitoring operation. Console-slot
    parameters use `UpsertEffectParam` with `RackName::Console`
    (`crates/session/src/model.rs:922-923`), so they are covered.
  - #1394 D1 appends an optional `ramp_samples` field (16 B) to `SetTrackMatrixOrPan` and to the
    route edits, but not to `UpsertEffectParam`. After #1394 the largest live edit is therefore
    still 416 B.
  - The P1 gate (at most 416 B per setter) can be met: a lane setter is about 200-232 B at 127-byte
    IDs. No order edge is needed (note:863-864).
  - The pre-P2c staging cost is measured, with its command and host.
- **MAJOR-2 (exact-0 allocation gate): resolved, and the argument is right.**
  - AGENTS.md at `c63f5f37d` (and on `main`, line 35) says that on a non-isolated page the worklet's
    message handler is "the only place this engine compiles or allocates on an audio thread, with
    every such allocation counted and reported, while the render-locked allocation count stays
    exactly zero". It says that `process()` never allocates or frees. Line 14 records the same
    exception under "Product principles".
  - The render rule ("zero allocations ... with no exception") applies to render, which is
    `process()`, not the handler.
  - D15-10's recorded resolution says the same: "Its control allocations are counted and reported;
    the render-locked allocation count stays exactly 0 in both modes. This is the one exception".
    #1332 D1 defines `singleModeControlAllocations` to count "control messages".
  - The note quotes AGENTS.md word for word (note:912-916).
  - The new P2a gate (2) can be met. It requires the classifier's allocation count and
    `requested_bytes` to be equal with and without 63 unnamed tracks (`bench_support::alloc` has
    both counters, in `tools/bench-support/src/alloc.rs:69-84`).
    - The whole-model clone is in `prepare_transaction` (`crates/protocol/src/model.rs:936`), and
      the compile clones and sorts again (`crates/session/src/compile.rs:133`). Both are outside the
      classifier, which the gate measures.
    - After P2a the classifier's output `Vec`s, the lowering of the named track and the EQ
      designer depend only on the named entities.
    - A scan for routes that follow a mute does not allocate.
    - The launch registry is built once at boot.
- **MINOR-1 (snapshot latency): resolved in design.** The design is right: a byte budget per
  handler call, a writer that can stop inside a string, continuation by a message and not by a
  timer, and a revision check. But the bound formula is wrong for multi-quantum callbacks
  (MINOR-1 below).
- **MINOR-2 (single-mode measurement): resolved.** Timing in a cross-origin-isolated Worker on the
  same module and exports is labelled an inference, and its basis is stated. Counting elapsed
  quanta with `currentFrame` is valid: it is an `AudioWorkletGlobalScope` attribute (Web Audio 1.1
  §1.32.3, checked).
- **MINOR-3 (arm tags): resolved.**
  - The tag is `u64` and there is no wrap path.
  - The tags travel as `{first, count}` in `CommandReply` (note:184), in the #1293 outcome record
    and in the #1294 reply. The outcome record grows from 24 to 40 B; I checked this against #1293
    D3.
  - `WebObservationResult` grows from 96 to 104 B. I checked the layout at
    `hosts/host-web/src/lib.rs:736-777`: replacing `reserved: [u32; 3]` at offset 84 with
    `u32 + u64 + u64` gives 104.
  - P7 lists the ABI layout files, and they exist.
  - `armed()` now settles in every case.
  - #1280 D3 is cited in the Monitoring row, in P5 and in P7.
- **MINOR-4 (P2b/P2c): resolved**, with two new defects (MINOR-2 and MINOR-3 below). P2c is
  correct:
  - `plan_structural_command` already calls the read-only `plan_preflight`
    (`controller.rs:1698`).
  - Moving the eviction and the copy to the commit leaves the cache byte-equal for a dropped
    prepared token.
  - Nothing can run between prepare and commit (`:1572-1574`).
  - See NIT-1 for two allocation sites that P2c does not name.
- **MINOR-5 (SDK queue): resolved.** The queue holds 64 calls. A refused call is never queued, sent
  or acked, and no queued call changes. The acked-batch answer holds: no ack can precede a drop,
  because the SDK drops nothing it accepted. A refused merged message is resent call by call and
  commits nothing first. See NIT-3.
- **MINOR-6 (replace owner): resolved.** P8 owns it in `crates/protocol`, with `response_len` 0 and
  the order edge P8 before #1386. See MINOR-4 for one gap.
- **NITs: all five resolved.** The per-call time is labelled an inference. The input-rate claim is
  removed. Lane-set overlap is defined. A merged call reports `rebuild`. P7 lists the resource
  rows, and I checked them at `hosts/host-web/src/lib.rs:7059-7097` and `:7164`.

## MINOR

- **MINOR-1. The single-mode snapshot bound counts quanta, but a continuation waits for the next
  system audio callback (note:594-598; P3 gate at :1022-1024).**
  - Web Audio 1.1 §2.6, which the note cites as [S1], says: "Each call has a system-level audio
    callback buffer size, which is a varying number of sample-frames", and the UA renders quanta
    "by filling up the requested buffer size".
  - Each callback renders several quanta back to back. The reply of a slice and the main thread's
    continuation cannot come back inside that burst, so each slice costs about one callback, not
    two quanta. Typical desktop callbacks are 256-1,024 frames, that is 2-8 quanta.
  - Example: with 480-frame callbacks (3.75 quanta), the 379,298-byte document (S = 6) takes about
    (S + 1) × 3.75 ≈ 26 quanta. The gate allows 2S + 2 = 14. So P3's gate likely fails in Chromium
    for a reason the implementer cannot change.
  - The design is right; only the bound's unit is wrong.
  - Fix: state the bound in system callbacks, for example S + 2 callbacks, or
    (S + 2) × ceil(B / 128) quanta for a callback of B frames. Record B in the gate's evidence.
- **MINOR-2. P2b's length update leaves out the revision field (note:943-949).**
  - The rule says that a value-setter transaction's length is "exact from the old length and the
    canonical lengths of the named fields before and after".
  - Every commit also rewrites `revision` (`crates/protocol/src/model.rs:943`), and the canonical
    text writes it as a decimal string (`crates/session/src/visit.rs:220`; fixtures show
    `"revision": "1"`). Its length grows at 9→10, 99→100, and so on. No edit names it.
  - Gate (1) catches this only when a generated sequence crosses a digit boundary.
  - Fix: name the revision as a field that changes on every commit.
- **MINOR-3. P2b gate (4) cannot run as worded (note:956-958).** The gate asks for "every byte
  budget from 1 to the text length on the corpus documents". Each budget writes the whole text, so
  the cost is about L² bytes per document. For the five corpus documents of 240-380 KB that is
  about 4.3e11 bytes, about 25 minutes in release at the measured 3.5 ns per byte, and far longer
  in a debug `cargo test`.
  Fix: test every budget on small documents, and on the large documents test a set that covers
  every stop kind: every budget from 1 to 4,096, powers of two plus and minus one, and stops at
  each string, number and escape boundary.
- **MINOR-4. P8 does not say which `origin_request_id` the typed commit's `SESSION_COMMITTED`
  event carries (note:198, :1101-1112).**
  - Replace "takes no request ID", but the event's `origin_request_id` is a `RequestId`, that is a
    `NonZeroU64` (`crates/protocol/src/queue.rs:375-381`; the commit passes `prepared.request_id`,
    `controller.rs:1999`).
  - P8's gate requires "the same `SESSION_COMMITTED` event" as the framed apply. That cannot hold
    literally, because the framed apply has a request ID.
  - The browser drains these events and discards them (F2), so the stakes are small. But P8's
    implementer must choose.
  - Fix: state the value (for example the message's own `requestId`) and compare the events
    without that field.
- **MINOR-5. "The reply lists" the overlay entries that a commit removes, but no reply has a field
  for them (note:1037-1038, :1394-1397).**
  - `CommandReply` (note:184), the #1382 D3 amendment, the #1293 outcome record and the #1294
    reply (note:1122-1124) carry the path, the revision and the arm tags only.
  - `armed()`'s owner-removal rejection depends on this transport. This is the same class as
    attempt-2 MINOR-3.
  - Fix: either add the removed entries to `CommandReply` and to the records, or say that the SDK
    detects the removal when it re-reads the session map after a `rebuild` (#1296 D3, as amended
    at note:1125). A removal is always structural.
- **MINOR-6. The owner question misstates option C's cost (note:1247-1253).** These lines did not
  change since attempt 2, but they now contradict section 3.2. In the owner-facing text:
  - "P1 adds them for the live values (faders, mutes, trims, filters, routes, VCAs)": P1 adds no
    route setter (note:381).
  - "any other field a fan may change (for example an effect's bypass, or a route target) needs
    its own setter": both examples already have field-level setters, `SetEffectBypass`
    (`crates/protocol/src/model.rs:295`) and `SetRouteDestination` (`:392`).

  These lines overstate the cost of option C, which the note recommends, so they do not bias the
  owner toward the recommendation. But the question is a deliverable of the issue and must be
  correct.

## NIT

- **NIT-1 (note:959-978).** P2c names the response buffers at `:1613` and `:2278`. Protocol-level
  refusals also allocate and zero `min(output_capacity, max_response_bytes)` at
  `controller.rs:1916` (`prepare_planned_immediate`) and `:1934` (`process_immediate_command`).
  `RESPONSE_STAGING_VECS` counts neither. A refused single-mode edit therefore still zeroes
  512 KiB in the handler, and P2c's counter gate cannot detect it. Name both sites, or count them.
- **NIT-2 (note:341-343).** "The ramp option has one name in every builder". The effect parameter
  builder sends `smoothingSamples` today (`sdk/src/core/live-controls.ts:926`), but neither
  `UpsertEffectParam` (#1394 D1 leaves it out) nor `EffectControlRecord::Parameter`
  (`crates/effect-contract/src/live.rs:63-70`) carries a ramp. P6 should remove that option, not
  rename it.
- **NIT-3 (note:289, :442-443).** `pendingCalls` is documented as "at most 64". After a merged
  message is refused, its calls are resent ahead of the queue, so more than 64 calls can be
  pending. The 64-call bound applies to new calls only; say so.
- **NIT-4 (note:1008-1009).** `currentFrame` is cited as [S1] (§2.6). It is defined in §1.32.3.

## Regressions and other checks

- **No new open choice for #1309, #1332 or #1382**, except the reply field of MINOR-5. The monitor
  field IDs stay with #1382, which is acceptable.
- **No interim placeholder.** No "TODO", "TBD", "interim" or "for now" in the note. The pre-P2c
  costs are sequenced before #1382's single-mode leg.
- **D1-D7 are not reopened.** F1 and F11 go to root. Replace as a typed entry stays consistent with
  D4, as in attempt 2.
- **Anchors: about 95 spot-checked, most of them new in attempt 3.** All say what the note claims.
  - `controller.rs:146`, `:268-276`, `:327`, `:356-361`, `:402-415`, `:1566`, `:1572-1574`,
    `:1595`, `:1613`, `:1629-1639`, `:1735-1744`, `:1954`, `:1974`, `:2278`, `:2851-2870`,
    `:3040`, `:3059-3076`, `:3108-3109`, `:3470`, `:15-16`;
  - protocol `model.rs:352-356`, `:397-404`, `:434-437`, `:900`, `:917`, `:936-942`;
  - `session_wire.rs:60`; `session/id.rs:10-14`; `session/model.rs:579-588`;
    `session/compile.rs:130-131`;
  - `live_delta.rs:202-203`, `:211`, `:228`, `:257-263`, `:315`, `:340`, `:395`, `:407`, `:424`,
    `:435`, `:444`, `:456`, `:475`;
  - `live.rs:81-88`, `:402-412`, `:659-662`, `:683-715`; `live_control.rs:48`;
    `observe.rs:1`, `:75`, `:94-96`;
  - capi `compile.rs:128`, `:510`, `:572`, `:740`; capi `control.rs:237-238`, `:353`, `:522`,
    `:783`, `:843`, `:939`, `:959-961`, `:1008`, `:1019`, `:1025`, `:1039-1065`, `:1071`, `:1079`,
    `:1145`, `:1187`, `:1335`, `:1392`, `:1501`, `:1552`;
  - capi `ffi.rs:635`, `:725`, `:807`; `plan.rs:53`, `:201`; `plan_exchange.rs:265`, `:375`,
    `:512`; `miso_engine_v1.h:18-22`;
  - host-web `lib.rs:668-680`, `:736-777`, `:782`, `:851`, `:853`, `:869`, `:1602`, `:1607`,
    `:1828-1853`, `:2336`, `:2820-2840`, `:3212`, `:3256`, `:3322`, `:3337`, `:3388`, `:3430`,
    `:3683`, `:4596`, `:5874-5885`, `:7022-7097`, `:7164-7165`; host-web `ffi.rs:3839`, `:3847`,
    `:3919`;
  - worklet `:1526`, `:1630`, `:1678`, `:1859`; host JS `:775-778`, `:781`;
    `prepared-control.js:330`; `rebuild-cost.mjs:10-11`; `host-core/Cargo.toml:15`;
  - `solo.rs:129`, `:160`, `:259`;
  - SDK: `observation-subscriptions.ts:101`, `:112`, `:549`, `:553-568`, `:603-606`, `:627`,
    `:631`, `:674`; `boundary.ts:754-773`, `:1416`; `abi.ts` (`backpressure`, `wrongState`,
    `internal`); `browser/engine.ts:197`, `:205`; `browser/live-controls.ts:21-68`;
    `core/live-controls.ts:28-30`, `:557`, `:567`, `:728-740`;
  - the 1305 spec at `:103`; the ABI layout files of P7.
- **"No code changed between the commits": true.** `git diff --stat 8be19c86e 50fb23b5f -- crates
  hosts sdk tools scripts` is empty, and so is the same diff from `50fb23b5f` to `c63f5f37d`. The
  spec-anchor claim holds too: from `6fb211594` to `8be19c86e` only
  `crates/capi/tests/resource_lifecycle.rs` changed.
- **Every new measured number has its command, commit and host:** section 5.6, `sizes.txt`
  (commit, `uname`, CPU, load, `rustc`). The derived numbers match: 3.5-3.8 ns per byte, S = 6 and
  14 quanta (37 ms), 0.35-0.52 s for 64 applies, and the monitor sizes 352 B, 184 B and
  360,456 B (TLV rule of `docs/CONTROL_BTLV_V1.md`).
- **The personal-mix owner question is still OPEN** (note:1220-1277) and self-contained. Its
  recommendation is labelled "not a decision", and no design choice depends on its answer.
  MINOR-6 covers its two wrong examples.
- **Scope.** `git diff --name-only 50fb23b5f c63f5f37d` lists only the note and the #1057 spec.

No tests were added. This issue is docs only, and the spec's test-value section says so.

## Gates run

- `git diff --name-only 50fb23b5f c63f5f37d`: only
  `docs/handoffs/one-edit-api/1057-design-note.md` and the #1057 spec.
- `git diff --stat 8be19c86e 50fb23b5f -- crates hosts sdk tools scripts`: empty.
  `git diff --stat 50fb23b5f c63f5f37d -- crates hosts sdk tools scripts`: empty.
- `bash scripts/check-workspace-policy.sh`, run twice, exit 0 both times
  (`workspace policy: ok`):
  - in a `git archive c63f5f37d` export with no git metadata;
  - in a `git clone --shared` of the repository checked out at `c63f5f37d`, with git metadata.
- Scratch encoded-size test `crates/protocol/tests/v1057c_size.rs`, in the export only:
  `CARGO_TARGET_DIR=/tmp/claude-1002/v1057/target cargo test --offline --locked --release
  -p protocol --test v1057c_size -- --nocapture`. Result per edit / frame of 1 / frame of 1,024:
  - `UpsertEffectParam` (inserts and console), 3-byte IDs: 176 / 224 / 180,272 B;
  - the same at 127-byte IDs: 416 / 464 / 426,032 B;
  - `SetEffectBypass`: 112 B at 3-byte IDs, 352 B at 127-byte IDs.
- Web Audio 1.1 (`https://www.w3.org/TR/webaudio-1.1/`, fetched 2026-10-05): §2.6 (system-level
  audio callback, rendering loop) and §1.32.3 (`currentFrame`) read in full text.
- The export, the clone, the target and the fetched page were deleted after the run.
  `df -h /` showed 54 GB free before the build.
