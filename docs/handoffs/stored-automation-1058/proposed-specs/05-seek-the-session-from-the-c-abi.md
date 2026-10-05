# Seek the timeline and every source in one C ABI call

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answer A1 (A1.2), under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`.

## Product outcome

A C host moves the playhead with one call. `miso_engine_v1_session_seek` (and its anchored form
`miso_engine_v1_session_seek_at`) seeks the session's timeline (drafts 04a and 04b) and every source of the
newest committed session to one timeline sample, under one new generation, all or nothing. Either
every consumer takes the seek, in the same render block, or nothing changes and the call returns a
typed refusal. The host then refills each source from that frame under that generation. Per-source
seeks stay, for a stem a transaction adds; they never move the timeline.

## Context

- **Per-source seeks today.** `SessionState::seek` and `seek_at`
  (`crates/capi/src/runtime/control.rs:1514-1543` on `6ee64f484`; in `crates/control-plane` after
  #1309) synchronize the plan epochs, then seek one
  producer of the newest committed session (`newest_providers`, `:1486-1490`): the pending
  candidate's producers while a swap waits, else the running plan's. The shared FFI body
  `source_seek_entry` (`crates/capi/src/ffi.rs:574-624`) checks the handle and the source ID, calls
  the state and maps a `SourceFailure` to a result code and diagnostic (`SourceFailure`,
  `crates/capi/src/runtime/control.rs:1546-1557`).
- **The producer table.** `SourceControlSet` (`crates/host-core/src/source.rs:127-130`) holds every
  source producer, its region `0..frames` (`ControlSource`, `:15-26`; filled from the model,
  `crates/host-core/src/prepare.rs:1231-1253`) and, after draft 04b, the timeline producer. Its
  `queue_seek` (`crates/host-core/src/source.rs:233-261`) checks the region, then calls the
  producer's `try_seek`. Its diagnostics are one table (`SourceControlError::diagnostic`,
  `:75-103`): `source.generation.stale`, `source.seek.backpressure`, `source.ring.vacated`,
  `source.seek.anchor_unaligned`.
- **Why all or nothing is possible.** Each consumer's command queue has one slot. Only the control
  thread pushes and render only pops, so the room a check sees cannot shrink before the push.
  Draft 04a D1 gives every producer `check_seek` (every rule except the push) and `try_seek`.
- **The shared control plane.** *Extract the C ABI control plane into a portable crate both hosts
  call* (#1309) moves capi's `SessionState` (today `crates/capi/src/runtime/control.rs`) into
  `crates/control-plane`, and lands before batch P1, so before this slice. After *Swap and retire
  browser plans through the Worker's service loop* (#1381) D2, the browser's source entry points
  call `SessionState`'s source routing too. So both hosts call one wrapper,
  `SessionState::session_seek`, over D1's function in `host-core`'s `SourceControlSet`.
- **Feature bits.** The last bit on `6ee64f484` is `MISO_ENGINE_V1_FEATURE_SOURCE_SEEK_AT` (32);
  the mask is 63 (`crates/capi/include/miso_engine_v1.h:136-142`; `crates/capi/src/abi.rs:53-71`).
  #1316 and #1323 each add a bit; D15-12's growth rule gives this slice the next free bit when it
  merges. `scripts/check-capi-abi.sh` holds the frozen exported symbol set (`:192-208`).
- **Header text.** "Sources across a structural transaction" (`miso_engine_v1.h:85-94`) and
  "Starting an added stem in time" (`:96-108`). *Document the seek contract and the C ABI growth
  rule in the header* (#1317) rewrites the seek text; this slice adds the timeline and the
  session seek to that text (D5).
- **The audit.** `audit capi` renders 100,000 calls with one structural transaction outside the
  render scope (`tools/audit/src/capi.rs:460-540`) and prints a `pcm_digest` that no gate pins
  (`.github/workflows/qualification.yml:759-779` checks only its shape). *Test held seeks across
  swaps and supersession, and add a seek to audit capi* (#1319, in this slice's closure through
  draft 04b and #1320) adds, right after the structural transaction,
  `miso_engine_v1_source_seek_at(session, "fixture-source", 2, A, A)` with `A = 4 * QUANTUM_FRAMES`
  and one generation-2 quantum from frame `A`, both asserted `RESULT_OK`; call 4's audited render
  applies that seek (#1319 D4).

## Decisions frozen for this slice

- **D1. One function, in host-core.** `SourceControlSet::seek_session(&mut self, generation: u64,
  timeline_sample: u64, anchor_sample: Option<u64>) -> Result<(), SourceControlError>`:
  1. `generation` 0 is `GenerationZero`.
  2. Check the timeline producer and every source producer with `check_seek` (draft 04a D1) for a
     command of generation `g`, frame `min(timeline_sample, region_end)` per source
     (`timeline_sample` for the timeline) and the anchor. The first failure is returned and
     nothing is pushed: a vacant entry is `Vacated`, a generation not above that producer's is
     `Seek(GenerationNotStrictlyIncreasing)`, a full slot is `Seek(Backpressure)`, an unaligned
     anchor is `Seek(AnchorUnaligned)`, a timeline frame over the limit is
     `Seek(FrameOutOfRange)`.
  3. Push every command (the timeline first, then the sources in ID order). After step 2 no push
     can fail, because only this thread pushes to those slots. A push that fails anyway is an
     engine invariant: the call returns `Chunk(HostChunkError::InternalInvariant)`, the code hosts
     already report as internal (`SourceControlError::is_internal`, `:115-119`), with a
     `debug_assert`.
  The C ABI's `SessionState::session_seek` synchronizes the epochs and calls it on
  `newest_providers_mut().sources`, exactly as `seek` does. `SessionState` is in
  `crates/control-plane` (#1309). The browser export (draft 06a) calls the same wrapper.
- **D2. Which consumers.** The timeline and every source of the newest committed session: while a
  structural transaction's candidate waits for its swap, that is the candidate's set, whose
  persisting producers feed the rings the running plan renders (#1273 D3). A source the candidate
  adds is sought too: it has its own new ring.
- **D3. Generation.** One `g` for every consumer, strictly above each one's current generation. The
  host chooses it: one above the largest generation it has used on this session.
- **D4. C entry points.** Session thread, serialized with the other session calls:
  ```c
  uint32_t miso_engine_v1_session_seek(miso_engine_v1_session *session,
                                       uint64_t generation, uint64_t timeline_sample);
  uint32_t miso_engine_v1_session_seek_at(miso_engine_v1_session *session,
                                          uint64_t generation, uint64_t timeline_sample,
                                          uint64_t anchor_sample);
  ```
  Results: `OK`; `INVALID_ARGUMENT` for a zero or stale generation, an unaligned anchor or a frame
  over the limit; `BACKPRESSURE` with `source.seek.backpressure` when any slot is full; the
  existing `SourceFailure::report` mapping for the rest, with the diagnostic in
  `miso_engine_v1_last_error`. One feature bit, `MISO_ENGINE_V1_FEATURE_SESSION_SEEK`, covers both
  symbols: the next free bit when this merges, never a reused one; code, tests and the smoke
  programs name it by its symbol. The mask grows to match; the frozen symbol list grows by two.
- **D5. Header text.** One more topic paragraph in #1317 D1's sequence of seek paragraphs (which
  replaces "Starting an added stem in time"), placed right after its `seek_at` paragraph: the timeline is the
  session's playhead; it starts at 0 and advances one quantum per render; stored automation follows
  it (#1058). `miso_engine_v1_session_seek` moves it and every source in one step, all or nothing;
  each source goes to `min(timeline_sample, its frames)`; the host then submits generation `g`
  from that frame. The anchored form takes the anchor of `miso_engine_v1_source_seek_at`. A
  per-source seek leaves the timeline where it is. A seek of every source is a declared
  discontinuity: the host calls `miso_engine_v1_declare_discontinuity` (#1323 D1) first.
  `docs/C_ABI_V1_QUALIFICATION.md` gains the same text beside the #1275 paragraph (`:106-122`).
- **D6. The audit.** #1319 D4's anchored source seek, its generation-2 quantum and its `RESULT_OK`
  assertions stay as they are. After call 4's audited render has applied that seek (so no seek is
  pending in any slot), still outside every render scope, `audit capi` calls
  `miso_engine_v1_declare_discontinuity` (#1323 D1), then
  `miso_engine_v1_session_seek(session, 3, B)` with `B = 8 * QUANTUM_FRAMES`, and submits one
  generation-3 quantum of `fixture-source` from frame `B`. It asserts that each call returns
  `RESULT_OK`. Its violation counts stay 0. Its `pcm_digest` moves (the seek changes the audio); no
  gate pins it. #1319 D4's liveness witness `replacements == 1` (`tools/audit/src/capi.rs:521-524`)
  stays true: on the audit session `miso_engine_v1_declare_discontinuity` publishes nothing.
  #1323 D2 prepares a successor only when the active plan has a floor above a node's natural
  arrival or a source-read offset above 0. The audit's one structural transaction (an
  `UpsertTrack`) raises no floor, and no plan of the audit has a source-read offset, so neither
  term holds, the call returns OK and no second plan is swapped in.
- **D7. The acked-batch question.** The call checks every slot before it pushes any (D1), and only
  the control thread pushes, so it is acknowledged only when every consumer holds the seek. A held
  anchored seek moves with its consumer across a swap. No ack can precede a drop.

## Deliverables

1. D1 in `crates/host-core/src/source.rs`, with unit tests.
2. D4 in `crates/capi/src/ffi.rs`, `crates/capi/src/abi.rs`, and the `session_seek` wrapper in
   `crates/control-plane/src/` (the file that holds `SessionState` after #1309);
   the header prototypes, constants and D5's text; `crates/capi/tests/c/abi_smoke.c`,
   `crates/capi/tests/c/header_smoke.cpp`; the frozen symbol list.
3. D5 in `docs/C_ABI_V1_QUALIFICATION.md`; D6 in `tools/audit/src/capi.rs`.
4. Tests in `crates/capi/src/runtime/tests.rs`.

## Authorized paths

- `crates/host-core/src/source.rs` (`seek_session` and its unit tests)
- `crates/capi/include/miso_engine_v1.h`, `crates/capi/src/ffi.rs`, `crates/capi/src/abi.rs`,
  `crates/capi/src/lib.rs` (exports), the file in `crates/control-plane/src/` that holds
  `SessionState` after #1309 (the `session_seek` wrapper only), `crates/capi/src/runtime/tests.rs`,
  `crates/capi/tests/c/abi_smoke.c`, `crates/capi/tests/c/header_smoke.cpp`
- `scripts/check-capi-abi.sh` (the frozen symbol list only)
- `tools/audit/src/capi.rs` (D6 only)
- `docs/C_ABI_V1_QUALIFICATION.md`
- `crates/capi/Cargo.toml` (the `graph` dev-dependency gains `features = ["test-support"]`, so
  gate 1 can call `graph::test_only_timeline_at`; today it is a plain dev-dependency,
  `crates/capi/Cargo.toml:25-27`), `Cargo.lock`

## Non-goals

- The browser export and the SDK (drafts 06a and 06b).
- The seek report's timeline row (draft 04b D6).
- The declaration itself (*Reset latency floors at a host-declared discontinuity*, #1323). This
  slice does not call it for the host.
- A loop feature. A host loops with an anchored session seek at the end of each pass (README A1.2).
- The protocol's stored transport position (draft 25 retires it).

## Hazards

- **Hot file.** The header is touched by #1316, #1317, #1323 and others in stream B
  (`docs/handoffs/decision-15-2026-10-05/STREAMS.md`). Root orders the merge; this slice takes the
  next free bit at that point and rewords nothing it does not own.
- **Pending candidate.** A test that seeks while a candidate waits must see the seek in the
  candidate's new rings and in the running plan's carried rings at the same block, through D2.
- **The timeline frame is not clamped.** Only sources clamp to their regions; the timeline takes
  `timeline_sample` as given (up to draft 04a's limit), so automation past the end of every stem
  still follows the playhead.

## Objective gates

1. **Both forms move everything at one block** (`crates/capi/src/runtime/tests.rs`, new). A
   two-source session (sources of 10,000 and 4,000 frames), 48 kHz, quantum 128, playing. After
   `miso_engine_v1_session_seek(session, 2, 6_000)` and one generation-2 quantum per source from
   its frame, the next rendered block reads frame 6,000 from the long source, the short source is at
   its end of region (frame 4,000) and the timeline reads 6,000 (draft 04b's test reader). With
   `session_seek_at(session, 3, 1_000, A)`, `A` two quanta ahead, the timeline and both sources
   change at the block whose sample is `A`, not before.
2. **Stale generation changes nothing.** After gate 1, `session_seek(session, 2, 0)` returns
   `INVALID_ARGUMENT` with `source.generation.stale`; the next block's output and the timeline equal
   a run without the call. A per-source seek to generation 5 first makes a session seek at
   generation 4 refuse in the same way.
3. **One full slot refuses all.** Push a per-source seek and render nothing, so that source's slot
   is full; `session_seek` returns `BACKPRESSURE` with `source.seek.backpressure`, the timeline's and
   the other source's slots stay empty (a following per-source seek on the other source is
   accepted), and the next block applies only the earlier per-source seek.
4. **Pending candidate.** Commit a structural transaction that adds a source, then
   `session_seek` before the swap: at the swap block the running plan's carried source, the added
   source and the timeline all read the seek's frame.
5. **Per-source seeks leave the timeline.** A plain `miso_engine_v1_source_seek` moves its source
   and not the timeline.
6. **ABI.** `FEATURE_SESSION_SEEK` is set in `miso_engine_v1_query_capabilities` and the mask
   equals the OR of every bit (`crates/capi/src/abi.rs` unit test); `bash scripts/check-capi-abi.sh`
   and `bash scripts/check-capi-abi.sh --self-test` pass with the two new symbols.
7. **Realtime and audio.** `./target/release/audit capi` reports 0 violations with the session seek
   in its run. Every existing capi and host-core test passes unchanged: no rendered bit moves for a
   host that does not call the new functions.
8. **Commands:**
   - `cargo test --locked -p capi`
   - `cargo test --locked -p host-core --features host-core/test-support`
   - `bash scripts/check-capi-abi.sh`, `bash scripts/check-capi-abi.sh --self-test`
   - `cargo build --locked --release -p audit -p capi && ./target/release/audit capi`
   - `bash scripts/check-workspace-policy.sh`, `bash scripts/check-cross-targets.sh` (README F19, the iOS memset rule)
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`

## Test value

- Gate 1 turns red if the timeline and the sources apply the seek in different blocks, if a source
  is not clamped to its region, or if the anchored form ignores its anchor. No test seeks more than
  one consumer today.
- Gate 2 turns red if the generation check runs per consumer while earlier pushes already happened.
- Gate 3 turns red if any slot is pushed before every slot is checked: the partial seek the
  acked-batch rule forbids.
- Gate 4 turns red if the call seeks the running plan's producers instead of the newest committed
  session's.
- Gate 5 turns red if a per-source seek moves the playhead.
- Gate 6 turns red if the symbols are not exported or the bit is missing or reused.

## Dependencies

- Draft 04b *Give every plan a timeline that carries like a source* (it brings draft 04a).
- *Document the seek contract and the C ABI growth rule in the header* (#1317): this slice adds
  the session seek to the text #1317 writes.
- *Reset latency floors at a host-declared discontinuity* (#1323), which adds
  `miso_engine_v1_declare_discontinuity`, named by D5's sentence and called by D6.
- *Test held seeks across swaps and supersession, and add a seek to audit capi* (#1319): D6 builds
  on its audit seek (also in the closure through draft 04b).
- *Extract the C ABI control plane into a portable crate both hosts call* (#1309): the file that
  holds `SessionState`. It lands before batch P1.
- Batch: R1. Draft 06a *Seek the timeline and every source from the browser module export and the
  headless SDK* calls the same wrapper.
