# Seek the timeline and every source in one C ABI call

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answer A1 (A1.2), under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`.

## Product outcome

A C host moves the playhead with one call. `miso_engine_v1_session_seek` (and its anchored form
`miso_engine_v1_session_seek_at`) seeks the session's timeline (#1640 and #1641) and every source of the
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
  `crates/host-core/src/prepare.rs:1231-1253`) and, after #1641, the timeline producer. Its
  `queue_seek` (`crates/host-core/src/source.rs:233-261`) checks the region, then calls the
  producer's `try_seek`. Its diagnostics are one table (`SourceControlError::diagnostic`,
  `:75-103`): `source.generation.stale`, `source.seek.backpressure`, `source.ring.vacated`,
  `source.seek.anchor_unaligned`.
- **Why all or nothing is possible.** Each consumer's command queue has one slot. Only the control
  thread pushes and render only pops, so the room a check sees cannot shrink before the push.
  #1640 D1 gives every producer `check_seek` (every rule except the push) and `try_seek`.
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
  #1641 and #1320) adds, right after the structural transaction,
  `miso_engine_v1_source_seek_at(session, "fixture-source", 2, A, A)` with `A = 4 * QUANTUM_FRAMES`
  and one generation-2 quantum from frame `A`, both asserted `RESULT_OK`; call 4's audited render
  applies that seek (#1319 D4).

## Decisions frozen for this slice

- **D1. One function, in host-core.** `SourceControlSet::seek_session(&mut self, generation: u64,
  timeline_sample: u64, anchor_sample: Option<u64>) -> Result<(), SourceControlError>`:
  1. `generation` 0 is `GenerationZero`.
  2. Check the timeline producer and every source producer with `check_seek` (#1640 D1) for a
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
  `crates/control-plane` (#1309). The browser export (#1643) calls the same wrapper.
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

- The browser export and the SDK (#1643 and #1644).
- The seek report's timeline row (#1641 D6).
- The declaration itself (*Reset latency floors at a host-declared discontinuity*, #1323). This
  slice does not call it for the host.
- A loop feature. A host loops with an anchored session seek at the end of each pass (README A1.2).
- The protocol's stored transport position (#1635 retires it).

## Hazards

- **Hot file.** The header is touched by #1316, #1317, #1323 and others in stream B
  (`docs/handoffs/decision-15-2026-10-05/STREAMS.md`). Root orders the merge; this slice takes the
  next free bit at that point and rewords nothing it does not own.
- **Pending candidate.** A test that seeks while a candidate waits must see the seek in the
  candidate's new rings and in the running plan's carried rings at the same block, through D2.
- **The timeline frame is not clamped.** Only sources clamp to their regions; the timeline takes
  `timeline_sample` as given (up to #1640's limit), so automation past the end of every stem
  still follows the playhead.

## Objective gates

1. **Both forms move everything at one block** (`crates/capi/src/runtime/tests.rs`, new). A
   two-source session (sources of 10,000 and 4,000 frames), 48 kHz, quantum 128, playing. After
   `miso_engine_v1_session_seek(session, 2, 6_000)` and one generation-2 quantum per source from
   its frame, the next rendered block reads frame 6,000 from the long source, the short source is at
   its end of region (frame 4,000) and the timeline reads 6,000 (#1641's test reader). With
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

- #1641 *Give every plan a timeline that carries like a source* (it brings #1640).
- *Document the seek contract and the C ABI growth rule in the header* (#1317): this slice adds
  the session seek to the text #1317 writes.
- *Reset latency floors at a host-declared discontinuity* (#1323), which adds
  `miso_engine_v1_declare_discontinuity`, named by D5's sentence and called by D6.
- *Test held seeks across swaps and supersession, and add a seek to audit capi* (#1319): D6 builds
  on its audit seek (also in the closure through #1641).
- *Extract the C ABI control plane into a portable crate both hosts call* (#1309): the file that
  holds `SessionState`. It lands before batch P1.
- Batch: R1. #1643 *Seek the timeline and every source from the browser module export and the
  headless SDK* calls the same wrapper.

## Amendment (controls coordinator, 2026-10-14)

Checked against `12d4a4f9d`.

- **`SessionState`** is in `crates/control-plane/src/control.rs` (#1309 landed): `seek` `:2276`,
  `seek_at` `:2301`, `newest_providers_mut` `:2185`.
- **Feature bits.** The last bit is now `MISO_ENGINE_V1_FEATURE_SERVICE` (128), mask 255
  (`crates/capi/include/miso_engine_v1.h:244-252`); D4 takes the next free bit at merge, as written.
- **Decision 17 C13** says a feature reaches the C ABI as a protocol message through
  `miso_engine_v1_submit_command`, with no new export. The session seek is source transport (the
  host must refill every ring at the new generation; the protocol never carries PCM), the same
  family as the existing `miso_engine_v1_source_seek` exports, so D4's two exports follow that
  family. Root confirms this reading at filing; the alternative is a protocol command whose reply
  carries the generation.
- **Dependencies.** #1317, #1319 and #1323 are still open.

## Amendment (controls coordinator, batch 16 check, 2026-10-16)

Checked against `35ac17545`.

**Landed dependencies.** #1309, #1316, #1317, #1319 and #1323 are closed and on main. #1640 and
#1641 are open, in this batch, and land first (in that order).

**Moved anchors.**
- `SessionState` (`crates/control-plane/src/control.rs`): `seek` `:4048`, `seek_unserviced`
  `:4059`, `seek_at` `:4073`, `seek_report` `:4090`, `newest_providers` `:3547`,
  `newest_providers_mut` `:3580`, `declare_discontinuity` `:2154`, `SourceFailure` `:4106`.
- `SourceFailure::report` is the trait method in `crates/capi/src/runtime/error.rs:124-128`.
- `crates/capi/src/ffi.rs`: `miso_engine_v1_source_seek` `:525`, `_seek_at` `:556`,
  `source_seek_entry` `:582-639`, `_source_seek_report` `:652`, `_declare_discontinuity` `:774`.
- `crates/host-core/src/source.rs`: `SourceControlSet` `:137-140`, `ControlSource` `:21-31`,
  `queue_seek` `:271-296`, `seek_report` `:258`, `diagnostic` `:86-112`, `is_internal`
  `:127-129`. The producer table is filled at `crates/host-core/src/prepare.rs:2265-2286`.
- `ControlSource` now holds `region_start..region_end`, not `0..frames`. host-core always sets
  `region_start` to 0 (`prepare.rs:2269`, `:2283`), so D1's `min(timeline_sample, region_end)` is
  always inside the region.
- `scripts/check-capi-abi.sh`: the frozen list is `:248-268`, 19 names. It grows to 21.
- `audit capi` (`tools/audit/src/capi.rs`): `run` `:518-599`; the liveness witness
  `replacements == 1` is `:579-582`; `apply_structural_transaction` `:193-249`, with #1319 D4's
  anchored seek at `:225-237`; `SEEK_ANCHOR_SAMPLE` `:61`. The qualification job checks only the
  digest's shape (`.github/workflows/qualification.yml:834-854`). The digest on main is
  `4f52089e4a8cf83c`.
- `docs/C_ABI_V1_QUALIFICATION.md`: the #1275 paragraph is still `:106-122`; the session-call
  list `:168-169`; the seek report `:194-219` (frozen count "19" at `:215`); #1317's seek
  contract `:221-230`; ABI growth `:231`.

**Feature bits.** The last bit is `MISO_ENGINE_V1_FEATURE_SOURCE_SEEK_REPORT` (512), mask 1023
(`crates/capi/include/miso_engine_v1.h:418-428`; `crates/capi/src/abi.rs:53-87`). The next free
bit today is 1024 (mask 2047). D4 still takes the next free bit at merge. The new bit joins the
`features` array of `masks_and_result_codes_are_frozen` (`abi.rs:588-625`), which checks the mask
is the OR of every bit.

**Header text (#1317 shipped).** "Starting an added stem in time" is gone. The seek paragraphs
are "The source-read clock" `:272`, "Seek." `:281`, "Anchored seek." `:293-299`, "Choosing the
anchor." `:301-310`, then Held, Replacement, One seek in flight, "Seeks across a structural
transaction", "Far past anchors", Rate and "Where a seek landed" (`:350-370`). D5's paragraph
goes right after "Anchored seek." The prototypes are at `:629-643`. "Declared discontinuity"
(`:95-117`) already says the host declares "before a seek of every source". Two more lists name
every session call and must gain the two new symbols: the thread table (`:19-20`) and "Control
service" (`:181-182`).

**Stale text.**
- `crates/capi/Cargo.toml` needs no edit: the `graph` dev-dependency already has
  `features = ["test-support"]` (`:32`, #1639). Drop that path and `Cargo.lock`.
- `crates/capi/src/lib.rs` needs no edit: it re-exports `ffi::*` (`:11`).
- D1 "synchronizes the epochs": today `seek` and `seek_at` run the service step
  (`self.service()`), which includes the epoch sync. `session_seek` does the same.
- **Gap for D1 step 2.** `SourceControlSet` holds `HostChunkProvider`, which exposes only
  `try_seek` (`crates/source/src/lib.rs:1039`). #1640 D1 puts `check_seek` on the seek-producer
  type, but neither #1640 nor #1641 names a `HostChunkProvider::check_seek`. Either #1640 adds the
  forward (its paths cover `crates/source/src/lib.rs`), or this slice needs that path.

**D6 still holds.** #1323 shipped the condition D6 relies on:
`PlanStateInventory::runs_behind_natural_timing` is `raised_arrivals` non-empty or
`source_read_offset > 0` (`crates/host-core/src/prepare.rs:1017-1019`). When neither holds,
`declare_discontinuity` returns `Ok` and publishes nothing (`control.rs:2154-2162`). The audit's
transaction adds a new muted track (no carried floor on a new node) and its route. No audit plan
has a source-read offset. So the declaration publishes nothing and `replacements == 1` stays true.
Code reading only; gate 7's run is the proof. Declare and seek after call 4's render, outside
every render scope, as written.

**Decision 17.** C13 calls seeks transport under C1, with one core function behind both hosts
(`docs/rulings/one-control-abstraction-2026-10-13.md:287-309`). But the root's widened list of
2026-10-16 does not name `miso_engine_v1_session_seek` or `_session_seek_at`, and says a new
direct export not in the list needs its own root ruling. Root rules before this slice merges.
`SessionState::session_seek` is the one core function; capi keeps only the pointer checks and
the result mapping.

**Hazard.** A new `impl` block in a render-path crate can move the impl index that
`scripts/check-web-audioworklet-callgraph.py` pins (see `c48cc3875`). Add `session_seek` inside
the existing `impl SessionState`.

## Amendment (root rulings, batch 16, 2026-10-16)

- **B16-R1.** `miso_engine_v1_session_seek` and `_session_seek_at` are in decision 17 C13's list
  of approved direct exports (seek transport behind `SessionState::session_seek`).
- **B16-R2, the publish fence.** A session seek is one committed act, so every consumer sees it in
  the same render block, on both hosts. D1 as filed does not give that for a plain seek: render
  pops the timeline's queue and each source's queue at different instants while the session
  thread pushes, so a plain seek pushed during a block can split across two blocks. The fix:
  - One atomic published-generation word per session, shared by every source producer, the
    timeline producer and the source-set driver of every plan of that session. It is carried to a
    successor with the producers, so a fresh and a carried consumer of one plan read the same
    word. It is allocated on the control thread, never on render.
  - The driver loads it once per block entry (acquire), before the timeline and the sources, and
    passes the value to each consumer's observe step (#1640 D1's shared function).
  - A consumer whose next command (read with the queue's `peek`, which #1320 added) has a
    generation above the loaded value leaves it in the queue for a later block. Nothing is
    dropped; the slot stays full, so a new push is still backpressure.
  - Every seek push ends with a release store of `max(word, g)`: the per-source seek after its
    one push, the session seek after its last push (D1 step 3). So all of a session seek's
    commands become visible to render at one block entry.
  - The session seek also refuses a generation not above the word (`source.generation.stale`),
    so a source removed with a higher generation cannot let a command through early. This matches
    D3's rule for the host (one above the largest generation it has used on the session).
  - One atomic load per block. No allocation, lock or syscall on render; no `engine/realtime`
    edit is expected. If one is needed, one attempt; on a refusal stop and report.
- **Paths added for B16-R2.** `crates/source/src/lib.rs` (the word in the driver, the observe
  step and the producers' push), `crates/host-core/src/prepare.rs` and
  `crates/host-core/src/source.rs` (the word shared into each plan's source set and carried with
  the producers), and `crates/control-plane/src/control.rs` only if the carry needs it.
- **New gate 1b (concurrency).** A native test runs a session thread that pushes plain and anchored
  session seeks (each one strictly newer) while a render thread renders, for many iterations
  (at least 10,000 seeks). In every block, either the timeline and every source apply the seek or
  none of them does: no seek is split across blocks. Run at both bank widths the native build
  covers. The render thread's allocation and free counts stay 0 (`bench_support::alloc` thread
  counters after warming).

## Decision record: tranche 1, core (batch 16)

Built on `454d65688` (worktree `codex/controls-batch-16b`). Tranche 2 (D4-D6: C exports, bit,
header, docs, audit, capi gates 1-7) follows.

**What shipped:**

- **B16-R2, the publish fence** (`crates/source/src/lib.rs`). `PublishedGeneration`, one
  `Arc<AtomicU64>` per session: `new(initial)` (control thread), `publish(g)` (`fetch_max`,
  `Release`: the word is `max(word, g)` and never falls), `generation()`, `retained_bytes()`, and
  a crate-private block-entry load (`Acquire`). Equality is identity (`Arc::ptr_eq`).
  `SeekProducer` holds `Option<PublishedGeneration>`; its `push` is the old `try_seek` without a
  publication, and `try_seek` is `push` then publish. `PcmSourceProducer`, `HostChunkProvider` and
  `TimelineProducer` gain `try_seek_unpublished` and `share_published_generation`; their
  `try_seek` publishes after its one push. The shared observe step's pop is
  `SeekState::take_published(queue, published)`: `peek(0)` (#1320), and pop only a command whose
  generation is at most the loaded value; otherwise the command keeps its slot. The PCM consumer
  and the timeline call it at every observe. The driver holds `Option<PublishedGeneration>`,
  loads it once at `begin_block` entry, before the timeline and the sources, and passes the value
  to both (and to `prepare_source_seek`). `prepare_session_graph_source_set` seals a session's
  set with the word; `prepare_graph_source_set` (standalone, no word, no fence) is unchanged.
- **No acked PCM is dropped by the fence.** The PCM consumer's acquire now peeks each block before
  it pops it: a block of an unobserved generation gets the one re-observe, and if its seek is
  still unobserved (fenced) the acquire stops and leaves the block queued behind its command.
  Before, the acquire popped the block first and, had the re-observe found nothing, would have
  discarded it. For an unfenced consumer the outcome is the old one in every case the old
  reasoning allowed.
- **One word per session** (`crates/host-core/src/prepare.rs`). `PlanStateInventory` holds it: a
  first plan allocates it, a successor takes `base.inventory`'s, `at_discontinuity` keeps it and
  `across_withdrawn` clones it, so every plan of a session, its fresh rings (a source a candidate
  adds or redeclares), its carried consumers, its timeline and a donation's two sides share one
  object. `test_only_floors_only` (a fresh reference) gets a word of its own. Every fresh producer
  and the fresh timeline producer share it before their first seek; the driver gets it through
  `prepare_session_graph_source_set`; `SourceControlSet` holds it (`ControlSourceBuilder::finish`).
  `adopt_persisting` debug-asserts both sets hold the same word.
- **D1** (`crates/host-core/src/source.rs`). `SourceControlSet::seek_session(generation,
  timeline_sample, anchor_sample)`: zero is `GenerationZero`; a generation not above the word is
  `Seek(GenerationNotStrictlyIncreasing)` (`source.generation.stale`); then `check_seek` on the
  timeline and every source in ID order (vacant is `Vacated`), each source at
  `min(timeline_sample, region_end)` (then `max(region_start)`, a no-op while host-core sets 0);
  then every `try_seek_unpublished` (timeline first), then one `publish(g)`. A failed push after
  the checks is `Chunk(HostChunkError::InternalInvariant)` with a `debug_assert`; the pushed
  commands are published so none stays held. `published_generation()` reads the word.
- **SessionState::session_seek** (`crates/control-plane/src/control.rs`, inside the existing
  `impl SessionState`): the service step, then `seek_session` on `newest_providers_mut().sources`.

**Departures:**

- **The word is charged in the inventory, not the source rows.** `PlanStateInventory::retained_bytes`
  gains `PublishedGeneration::retained_bytes()` (24 bytes on x86-64), because the inventory is
  the owner that holds the word across plans and the capi drop oracle frees it there
  (`capi_retained_bytes_charge_every_byte_the_compile_retains` passes unchanged). No source row,
  pin or ceiling moved. Each kept inventory charges it once, an overcharge in the safe direction.
- **Four test literals** of `SourceGraphSourceSetDriver` in `crates/source/src/lib.rs` and one of
  `PlanStateInventory` in `prepare.rs` gain the new field (`published: None`, a fresh word):
  field additions only, no assertion changed.
- **`crates/host-core/tests/timeline.rs`**: `seek_both` (gates 1 and 3 of #1641) now issues one
  `seek_session` instead of a source seek plus `test_only_timeline_producer().try_seek`. The other
  `test_only_timeline_producer` uses stay: they seek the timeline alone.
- **A per-source seek pushed during a block** with a generation above the word now applies at the
  next block, not in the block the push raced (B16-R2's rule for every command). Only a
  concurrent host sees it; no existing test does.

**Gates (tranche 1):**

- D1 unit tests, `crates/host-core/src/source.rs` `source::tests`:
  `both_forms_move_the_timeline_and_every_source_clamped_to_its_region`,
  `a_refusal_pushes_nothing_and_publishes_nothing`, `a_vacant_entry_refuses_the_whole_seek`. PASS.
- Fence unit tests, `crates/source/src/lib.rs` `tests::published`:
  `an_unpublished_seek_and_its_pcm_wait_in_their_slots_until_published` (red with the fence
  removed), `a_seek_on_its_own_publishes_its_generation_and_the_word_never_falls`. PASS.
- **Gate 1b**, `crates/host-core/tests/session_seek.rs`
  `concurrent_session_seeks_reach_the_timeline_and_every_source_in_one_block`: a session thread
  pushes 10,000 session seeks (about half anchored, some late; generation `k + 2`; each pushed at a
  random instant up to 40 µs after a block starts) while a render thread renders; every timeline
  step is in the block both sources' seek reports name, at the seek's frames (the short source
  clamped), at Simd8 and Simd4; render-thread allocations, reallocations and frees 0 over every
  measured block. About 12 s for both widths (debug). With the fence removed (`take_published`
  ignoring the value) it fails in each of two runs, at seeks 297 and 1,368 (`short` one block
  ahead of the timeline and `long`).
- Commands: `cargo test --locked -p source`, `-p host-core --features host-core/test-support`,
  `-p control-plane`, `-p capi`: PASS. `./target/release/audit capi`: `pcm_digest`
  `4f52089e4a8cf83c` (unchanged), 0 violations. `realtime_policy`: ok, 153 regions (unchanged).
  Clippy `-p source -p host-core -p control-plane --all-targets --all-features -D warnings`,
  `cargo check --workspace --all-targets --all-features`, `cargo fmt --all`: clean.

## Decision record: tranche 2, the C ABI (batch 16)

Built on `51ec4fd67` (tranche 1).

**What shipped:**

- **D4.** `miso_engine_v1_session_seek(session, generation, timeline_sample)` and
  `miso_engine_v1_session_seek_at(session, generation, timeline_sample, anchor_sample)` in
  `crates/capi/src/ffi.rs`: one shared body (`session_seek_entry`) with the handle check and the
  `SourceFailure::report` mapping around `SessionState::session_seek`. `FEATURE_SESSION_SEEK = 1 <<
  10` (1024); the mask is 2047. `masks_and_result_codes_are_frozen` gains the bit in its `features`
  array (the mask is the OR of every bit) and pins it; `version_and_capabilities_are_exact` checks
  it is reported. No C export of the published generation (not approved).
- **Header.** The two prototypes after `miso_engine_v1_source_seek_report`; the constant and
  mask; D5's paragraph, "Session seek (issue 1642)", right after "Anchored seek.", which also
  says a host may pick the next generation as one above the largest it has used; the two symbols
  in the thread table and in "Control service" (that paragraph rewrapped, no word changed but the
  two names).
- **Smoke programs.** `abi_smoke.c` pins the bit and the mask, adds it to
  `reports_every_defined_feature`, checks it against the queried mask and calls both symbols
  with a null session for `INVALID_ARGUMENT`; `header_smoke.cpp` pins the bit inside the mask and
  both signatures. `scripts/check-capi-abi.sh`: the frozen list 19 -> 21 (the self-test is
  generic and needs no change).
- **Docs.** `docs/C_ABI_V1_QUALIFICATION.md`: the D5 paragraph (with the fence and the frozen count
  21), and the two calls in the service paragraph's list of session calls.
- **D6.** `tools/audit/src/capi.rs`: before call 5 (`SESSION_SEEK_CALL`, right after call 4's
  render applied the anchored seek), outside every render scope, `declare_discontinuity`, then
  `session_seek(session, 3, 8 * QUANTUM_FRAMES)`, then one generation-3 quantum of
  `fixture-source` from that frame, each asserted `RESULT_OK`.

**Departures:**

- **One `engine/realtime` edit (accepted on its one attempt).**
  `RealtimePlanOwner::active_plan_mut` in `crates/engine/src/realtime/plan_exchange.rs`, inside
  the existing `impl RealtimePlanOwner` (no new impl block), so a capi test can read the active
  plan's timeline (`crate::ffi::test_plan_timeline_at`, over `graph::test_only_timeline_at`
  through control-plane's test-support `test_owner_mut`). No other reader of the active plan
  existed; gate 1 names #1641's reader. It is a borrow for the plan's exclusive owner only.
- **The D5 paragraph sits after "Anchored seek." in the header** as the batch-16 check says; in
  the qualification doc it follows the seek-report paragraph (#1316), not the #1275 one, so the
  frozen counts there stay in order (15, 17, 19, 21).
- **Test helper.** `crate::ffi::test_session_seek` (the runtime test module forbids `unsafe`).

**Gates** (`crates/capi/src/runtime/tests.rs`, `runtime::tests::session_seek`):

1. `both_forms_move_the_timeline_and_every_source_at_one_block`: `session_seek(2, 6_000)` lands
   long at 6,000, short at 4,000 and the timeline at 6,000 in one block; `session_seek_at(3, 1_000,
   A)` holds all three two blocks and lands them at `A`.
2. `a_stale_generation_changes_nothing`: `session_seek(2, 0)` refuses `source.generation.stale`
   and the next block's bits and timeline equal a twin's; a per-source seek to 5 makes a session
   seek at 4 stale through the published generation alone (the timeline is at 2), and pushes
   nothing.
3. `one_full_slot_refuses_every_consumer`: `BACKPRESSURE` with `source.seek.backpressure`; the
   short source's slot and the timeline's stay empty.
4. `a_seek_before_the_swap_reaches_the_carried_and_the_added_source_and_the_timeline`.
5. `a_source_seek_leaves_the_timeline`.
6. `masks_and_result_codes_are_frozen`, `version_and_capabilities_are_exact`,
   `check-capi-abi.sh` and its `--self-test`.
7. `audit capi`: 0 violations, `replacements == 1` and the carry assertion hold. `pcm_digest`
   moved `4f52089e4a8cf83c` -> `bffc21e857b5e134`: call 5 now plays the generation-3 quantum where
   it rendered underrun silence before (the anchored seek's one quantum ran out at call 4).
- **`cargo doc -D warnings` repairs (mechanical, outside the paths).** Four pre-existing
  public-doc links to private items: three in `crates/source/src/lib.rs` (`TimelineConsumer`,
  `begin_block_at` and `prime_ready`, from #1640/#1641) and one in
  `crates/control-plane/src/compile.rs:734`, each turned into a code span.

**Root ruling B16-R8 (2026-10-16).** `RealtimePlanOwner::active_plan_mut` is gated behind a new
`engine/test-support` feature (`#[cfg(feature = "test-support")]`), because its only caller is test
code. capi's dev-dependency on `engine` turns the feature on. The workspace debug-test and doctest
steps of `qualification.yml` name `engine/test-support`, so `scripts/check-test-support-ci.py`
passes (13 packages). The release `capi` build does not see the accessor.
