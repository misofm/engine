# Document the seek contract and the C ABI growth rule in the header

Stream B of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-12).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

A C host author can write correct seek code and correct version checks from
`miso_engine_v1.h` alone. The header states the seek contract in full (the source-read clock and its
origin, held behaviour, replacement, the in-flight BACKPRESSURE string, displacement by a
transaction, far-past anchors, choosing the anchor, the source-rate assumption, and how a plain
seek's landing is reported) and the ABI growth rule (a feature bit per addition, never compare the
mask with `==`, and the bit does not protect a directly linked host). The repository's own C smoke
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
  `ABI_VERSION` changes only on a break. Each addition (a symbol, a named former reserved word, a
  new result or enum value a host can receive) gets its own feature bit. Hosts test each bit they
  need with `&`, tolerate unknown bits, and never compare `feature_mask` with `==`. A feature bit
  protects only a host that resolves the symbol at run time; a host that links a symbol directly
  fails to load against an older library before it can test anything.
- **D3. Qualification record.** `C_ABI_V1_QUALIFICATION.md:112-113` is corrected to D2's wording,
  and a short "ABI growth" paragraph states D2 once for the record.
- **D4. Smoke test.** `abi_smoke.c` drops the `!= MISO_ENGINE_V1_FEATURE_MASK` comparison and
  instead requires each bit the header defines, one `&` test per bit. The exact-mask check stays in
  the Rust tests named above.

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
