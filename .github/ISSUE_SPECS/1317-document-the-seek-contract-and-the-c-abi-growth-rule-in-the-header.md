# Document the seek contract and the C ABI growth rule in the header

Stream B of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-12).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

A C host author can write correct seek code and correct version checks from
`miso_engine_v1.h` alone. The header states the seek contract in full (the source-read clock and its
origin, held behaviour, replacement, the in-flight BACKPRESSURE string, displacement by a
transaction, far-past anchors, choosing the anchor, the source-rate assumption, and how a plain
seek's landing is reported) and the ABI growth rule (a feature bit per added symbol or named
struct word, none for a new value of an existing enum, never compare the mask with `==`, and the
bit does not protect a directly linked host). The repository's own C smoke
test stops modelling the `==` comparison it tells hosts never to write.

## Context

- The seek text today is the paragraph "Starting an added stem in time"
  (`crates/capi/include/miso_engine_v1.h:96-108`). It anchors on "the clock
  miso_engine_v1_render_f32_planar takes" (`:98-99`), which D1-a replaces by the plan's source-read
  clock (#1316 D1). It does not state held behaviour for an already playing source, replacement,
  the one-seek-in-flight refusal, displacement, far-past anchors, the clock origin or the rate
  assumption. (#1318 deletes its false clause "those blocks count as source underruns".)
- Behaviour to document, each verified in code:
  - Clock origin: a plan starts at absolute sample 0 (`crates/engine/src/realtime/plan.rs:582`),
    renders contiguous quanta (`render_contiguous`, `:870-882`), and a swapped-in plan continues
    the outgoing clock (`crates/engine/src/realtime/plan_exchange.rs:418`). Anchors must be quantum
    multiples (`crates/source/src/lib.rs:771-774`).
  - Held: while a seek waits for its anchor, queued PCM of the playing generation plays, then the
    source holds silence (`crates/source/src/lib.rs:1108-1114`; reported as held by #1318).
  - Replacement: a newer command, plain or anchored, replaces a held one
    (`crates/source/src/lib.rs:1325-1335`).
  - In flight: the command queue has one slot; a second seek before render pops the first is
    refused as `MISO_ENGINE_V1_BACKPRESSURE` with `source.seek.backpressure`
    (`crates/source/src/lib.rs:776-785`; string at `crates/host-core/src/source.rs:99`). A held seek
    has been popped, so it does not block the next one.
  - Displacement: a persisting source's held seek moves with its consumer across a swap; a source
    a transaction changes restarts in a new ring at generation 1 and its held seek is discarded with
    the old ring; a removed source refuses as `source.id.unknown`
    (`miso_engine_v1.h:85-94`).
  - Far past: an anchor already rendered applies at the next block at `F` plus the lateness
    (`crates/source/src/lib.rs:1355-1363`); a frame that lateness carries past the region end
    renders silence as end of region, neither underrun nor held (`:2229-2236`).
  - Rate: lateness is added in source frames; that is exact because every chunk must be at the
    session rate (`source.rate.mismatch`, `crates/host-core/src/source.rs:167-172`).
  - Plain seek landing: reported by `miso_engine_v1_source_seek_report` (#1316).
- Growth rule today: `docs/C_ABI_V1_QUALIFICATION.md:112-113` says the host checks the bit "since
  a library older than #1275 does not export it", which implies the bit protects a directly linked
  host; it does not (the load fails first). `crates/capi/tests/c/abi_smoke.c:120-126` requires
  `capabilities.feature_mask != MISO_ENGINE_V1_FEATURE_MASK` to be false, the `==` pattern. The
  exact-mask equality is already asserted in Rust (`crates/capi/src/ffi.rs:1458-1472`,
  `crates/capi/src/abi.rs:482-486`).

## Decisions frozen for this slice

- **D1. Seek paragraph.** Replace `miso_engine_v1.h:96-108` with one paragraph per topic, in this
  order: the source-read clock ("equal to the render clock in this version"; origin 0 at compile;
  advances one quantum per render; a plan swap continues it); `seek` (applies at the next block render
  begins; where it landed is the seek report); `seek_at` (quantum-multiple anchor, `source.seek.anchor_unaligned`);
  choosing `A` (a few quanta past the last rendered block, enough to cover the host's decode and
  submit latency); held behaviour; replacement; the in-flight refusal and its string; displacement
  by a transaction; far-past anchors and the region end; the rate assumption. Every statement names
  its diagnostic string where one exists. No statement promises anything the code does not do.
- **D2. Growth rule** in the header, beside `MISO_ENGINE_V1_ABI_VERSION` (`:111`):
  `ABI_VERSION` changes only on a break. Each added symbol and each former reserved struct word
  given a name gets its own feature bit, the next free one when it merges. A new value of an
  existing enum or result code, and an in-place V1 protocol field, takes no bit (decision 15,
  D15-3 recorded resolution), so hosts tolerate enum values they do not know and treat an
  unknown result code as a failure. Hosts test each bit they need with `&`, tolerate unknown
  bits, and never compare `feature_mask` with `==`. A feature bit protects only a host that
  resolves the symbol at run time; a host that links a symbol directly fails to load against an
  older library before it can test anything.
- **D3. Qualification record.** `C_ABI_V1_QUALIFICATION.md:112-113` is corrected to D2's wording,
  and a short "ABI growth" paragraph states D2 once for the record.
- **D4. Smoke test.** `abi_smoke.c` drops the `!= MISO_ENGINE_V1_FEATURE_MASK` comparison and
  instead requires each feature bit the header defines, one `&` test per `MISO_ENGINE_V1_FEATURE_*`
  symbol, never a bit number. The exact-mask check stays in the Rust tests named above.

## Deliverables

1. D1 and D2 in `crates/capi/include/miso_engine_v1.h` (comment text only; no declaration changes).
2. D3 in `docs/C_ABI_V1_QUALIFICATION.md`.
3. D4 in `crates/capi/tests/c/abi_smoke.c`.

## Authorized paths

- `crates/capi/include/miso_engine_v1.h`, `crates/capi/tests/c/abi_smoke.c`
- `docs/C_ABI_V1_QUALIFICATION.md`

## Non-goals

- Any behaviour change; any new symbol. The browser SDK's seek documentation (stream H).

## Objective gates

1. **Header still compiles as both languages, at the strictest flags.** `bash scripts/check-capi-abi.sh`
   (C11 and C++17 consumers at `-Wall -Wextra -Werror -pedantic`, symbol set, smoke run) and
   `bash scripts/check-capi-abi.sh --self-test`.
2. **Smoke test is a bit test.** Build the smoke test against a library whose capability report
   clears one defined bit (the checker's existing mutation mechanism or a local patch, recorded in
   the PR): it fails. Against a library that reports one extra unknown bit, it passes.
3. **Rust exact-mask tests unchanged and green:** `cargo test --locked -p capi`.
4. **Review gate (no automated test):** the reviewer checks every D1 statement against the code
   anchor listed in Context. A test that greps the header is refused by AGENTS.md.
5. `bash scripts/check-workspace-policy.sh`, `cargo fmt --all -- --check`.

## Test value

- Gate 2 (amended `abi_smoke.c`): red if the shipped library stops reporting any bit the header
  defines; it no longer goes red for an extra unknown bit, which is the host-tolerance behaviour
  D2 requires. The library-side exact equality stays caught by `version_and_capabilities_are_exact`.
- No other test is added: the deliverable is prose, and prose is never the subject of a test.

## Dependencies

- *Anchor every seek on the plan's source-read clock* (#1316): the clock name and the seek report
  the text documents.
- *Report held source blocks apart from underruns* (#1318): the held reporting the text documents.

## Amendment (controls coordinator, batch 15 check, 2026-10-15)

Checked against `228fca26c`.

**Moved anchors.** The header grew by about 110 lines (the live-edit, solo, response-preview,
completion, service and engine-memory paragraphs). Each behaviour in Context still holds; only
the lines moved.

- D1 replaces the paragraph "Starting an added stem in time" at
  `crates/capi/include/miso_engine_v1.h:204-216` (was `:96-108`). Its clock clause is `:205-207`;
  the false clause that #1318 deletes is `:214-215`.
- D2 goes beside `MISO_ENGINE_V1_ABI_VERSION` at `:219` (was `:111`). The feature bits are
  `:244-252`; the mask is 255.
- Displacement is the paragraph "Sources across a structural transaction" at `:193-202` (was
  `:85-94`).
- Clock origin: a new plan's clock is 0 at `crates/engine/src/realtime/plan.rs:834`;
  `render_contiguous` is `:1172-1185`; a swapped-in plan continues the clock at
  `crates/engine/src/realtime/plan_exchange.rs:779`. Anchors must be quantum multiples at
  `crates/source/src/lib.rs:771-775`.
- Held: `begin_block_at` doc `crates/source/src/lib.rs:1106-1114`. Replacement:
  `observe_seek_at_block_boundary`, `:1324-1346`. In flight: `try_seek`'s push and its
  `Backpressure`, `:776-786`; the string is still `crates/host-core/src/source.rs:99`.
- Far past: `:1350-1365`. The region-end tests are `:2264-2310`
  (`a_late_anchored_seek_past_the_region_end_is_the_end_of_region` and the on-time short-block
  case).
- Rate: still `crates/host-core/src/source.rs:167-172`.
- Growth rule: `docs/C_ABI_V1_QUALIFICATION.md:112-113` is unchanged. The `==` model in
  `crates/capi/tests/c/abi_smoke.c` is now `:221` (the block is `:215-223`). The Rust exact-mask
  tests are `version_and_capabilities_are_exact` (`crates/capi/src/ffi.rs:1558`) and
  `crates/capi/src/abi.rs:550-554`.

**Code notes for the implementer.**

- The smoke test also pins header constants at compile time (`abi_smoke.c:22-27`, for example
  `ABI_ASSERT(MISO_ENGINE_V1_FEATURE_MASK == 255)`). They pin the header, not the library's
  report, and `scripts/check-capi-abi.sh --self-test` relies on the fixtures' pins (`:87-88`).
  D4 covers the runtime check only. Keep the static pins.
- Gate 2: the checker's self-test mutates the staged header and the `nm` output. It cannot
  change the library's capability report. Use the "local patch, recorded in the PR" route.
- Since #1348 the seek calls run the service step first (header `:148-153`). D1's `seek`
  paragraph should not contradict that paragraph. A cross-reference is enough.
- "Equal to the render clock in this version" stays true after #1396: only a warm successor
  (#1355) makes the offset nonzero in production. #1355 must amend this text when it lands.
  Record that in #1355.

**Dependencies on GitHub.** #1316 and #1318 are open. Both are in the batch-15/16 list. #1275,
#1314 and #1348 are closed. No dependency is outside the batch.

**Decisions.**

- D1, D3 and D4 hold against the code and decision 17.
- **B15-R3, root decision under owner delegation (2026-10-15): D2 keeps its text and adds two sentences: "A feature added as a
  protocol message is announced by its protocol capability flag in the CAPABILITIES reply, not by
  a feature bit. The registry, not this rule, governs protocol flags."** (D2 against decision
  17 C13.) The header states the two sentences and points to the registry for the rest. The
  finding follows. Since decision 17, new features reach
  the C ABI as protocol messages through `miso_engine_v1_submit_command`, with no new export
  (C13). Each is announced by a protocol capability flag in the `CAPABILITIES` reply, not by a C
  feature bit (header `:105`, `RESPONSE_GET`, "capability flag bit 14"). The protocol decoder
  refuses an unknown capability flag (`docs/CONTROL_PROTOCOL_REGISTRY.md:13`), which is the
  opposite of D2's "tolerate unknown bits" for `feature_mask`. D2 says nothing about protocol
  flags. So a host author who reads only D2 can apply the C rule to protocol flags. Proposed
  resolution: D2 keeps its text and adds two sentences: "A feature added as a protocol message
  is announced by its protocol capability flag in the CAPABILITIES reply, not by a feature bit.
  The registry, not this rule, governs protocol flags." Root decides whether the header states
  the protocol rule or points to the registry.

**Gates and commands.** `scripts/check-capi-abi.sh`, its `--self-test`, both C fixtures and
`scripts/check-workspace-policy.sh` exist. `cargo test --locked -p capi` runs both Rust
exact-mask tests.

**Realtime-path edits.** None. This slice is header text, one doc and one C test.

## Decision record: #1317 (batch 15c)

**Shipped.** D1-D4 and B15-R3, comment text and one C test; no declaration, symbol or Rust change.

- Header (`crates/capi/include/miso_engine_v1.h`): "Starting an added stem in time" and the two-line
  seek-report note are replaced by ten paragraphs in D1's order: the source-read clock, seek,
  anchored seek, choosing the anchor, held, replacement, one seek in flight, seeks across a
  structural transaction, far-past anchors, rate, and where a seek landed. A new comment block
  before `MISO_ENGINE_V1_ABI_VERSION` states the growth rule (D2) with B15-R3's two sentences. The
  cross reference in the live-edit paragraph now points at the new paragraphs, and the #1310
  paragraph is rewrapped to 100 columns (one line was 135 wide); "placeholder #1699" became "issue
  1699 is the successor issue for one".
- `docs/C_ABI_V1_QUALIFICATION.md` (D3): the `seek_at` paragraph no longer implies the bit protects
  a directly linked host and names the source-read clock; a paragraph records the seek contract and
  one "ABI growth" paragraph states D2 once.
- `crates/capi/tests/c/abi_smoke.c` (D4): `reports_every_defined_feature` requires one `&` per
  `MISO_ENGINE_V1_FEATURE_*` symbol (nine); the `!= MISO_ENGINE_V1_FEATURE_MASK` comparison is gone.
  The compile-time pins are kept.

**Statements checked against the code** (file, function): clock origin 0 and `render.time.discontinuity`
(`plan.rs` `render_contiguous`, `plan_exchange.rs` `adopt_absolute_sample`, `ffi.rs` render test);
quantum-multiple anchor and the check order (`source/src/lib.rs` `try_seek`); producer switches to
`g` at the call and refuses other generations (`validate_submission_metadata`); held, replacement,
late apply and region end (`observe_seek_at_block_boundary`, `begin_block_play`, `settle_block`,
tests `a_late_anchored_seek_past_the_region_end_is_the_end_of_region`); one command slot
(`command_capacity = 1`, `source.seek.backpressure` in `host-core/src/source.rs`); consumer carry
including `held_seek` (`adopt_sources`); report words and 64-attempt bound (`publish_seek_report`,
`seqlock.rs`); source rate equals the session rate (`prepare.rs` pushes `compiled.sample_rate()`).

**Departures.**

- The text says a seek takes effect "no later than the first block that begins after the call
  returns", not "at the next block render begins": a producer that pushes a seek and its PCM
  between a block's command pop and its data pop is observed by the acquire's re-observe in that
  same block (#1274 MINOR-3).
- Far-past text says the seek applies in the block that pops it (not "the next block after it is
  popped"), as `observe_seek_at_block_boundary` pops and applies in one block.
- The end-of-region sentence is conditional on a region the host closed with `end_of_region`; with
  no end known the block is an underrun.
- Feature mask is 511 (bit 8 landed with #1316), not the amendment's 255.
- #1320's prime has no host-visible API, so the header says nothing about it. #1396 has not landed,
  so "equal to the render clock in this version" stands and #1396 is not named in the header.

**Gates.**

1. `bash scripts/check-capi-abi.sh`: ok (C11 and C++17 at `-Wall -Wextra -Werror -pedantic`, symbol
   set, smoke run, shared and static); `--self-test`: ok.
2. Smoke is a bit test, by a local patch of `FEATURE_MASK` in `crates/capi/src/abi.rs`, reverted
   after each run: clearing `FEATURE_SERVICE` makes `check-capi-abi.sh` exit 1; OR-ing in unknown bit
   `1 << 9` passes.
3. `cargo test --locked -p capi`: 158 lib tests and every integration test pass (the Rust exact-mask
   tests are unchanged).
4. Review gate: every D1 statement is checked above against its code anchor.
5. `bash scripts/check-workspace-policy.sh`, `cargo fmt --all -- --check`,
   `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`: pass.

**Successor work.** #1355 must amend the "equal to the render clock in this version" sentence (the
header's first seek paragraph, the qualification record and `SourceControlSet::seek_at`'s doc) when
a warm successor makes the source-read offset nonzero; the amendment asks for this to be recorded in
#1355, whose spec does not yet carry it. #1699 owns the counter the header says does not exist yet.
