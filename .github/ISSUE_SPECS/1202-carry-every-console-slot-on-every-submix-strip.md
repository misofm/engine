# Carry every console slot on every submix strip

Slice 05 of *Submix strips and live aux sends* (#1196, the umbrella filed by *Record the submix, send and
VCA ruling*, #1197). Batch K1.

The design record cited below (`DESIGN`, `VERIFY-1` to `VERIFY-3`, `REVISION-1`, `REVISION-2` and
`APPLIED-3`) is committed in `docs/handoffs/submix-sends-2026-10-02/`.

## Product outcome

The owner's rule holds for buses: every submix carries every session console slot, with its own
`{slot, bypass, params}` per slot, exactly as tracks do. A drum bus gets the console EQ and
compressor. Its lanes bank with the same bank programs, padded, at the bus's dependency level
(decision 12). A browser that boots such a session with live controls keeps booting.

## Context (verified on `fe8ac679`; slices 01-04 changed the sites named "after")

- **Console entries are track-only.**
  - `validate_console_entries` (`crates/session/src/validate.rs:236`) enforces "every track carries
    every slot" with `console.entry_missing`, `console.entry_order` and `reference.missing_entity`.
    It takes `&Track`.
  - It is called per track from `validate_tracks` (`:371`).
  - Session paths are index paths.
- **Lowering.** `SessionModel::lower_track` (`crates/session/src/model.rs:335-345`) matches a strip's
  console entries to the session slots by position. Slice 01 generalised it to `lower_strip`.
- **After slices 02-04:**
  - a submix has an input section, inserts, a fader and pan, renders them on its summed input, and
    delays its sum;
  - its BTLV field 3 is reserved for `console`;
  - `Submix::unity(id)` builds a transparent strip with no console entries;
  - `strips()` yields tracks then submixes, and the effect, graph and builtins compilers walk bus
    strips;
  - live controls and observation attach only to track-owned effects (slice 03 D6, DESIGN P16), and
    `crates/host-core/tests/submix_strip.rs` and `hosts/host-web/src/tests.rs` hold the K1
    live-controlled boot gates (slice 03 gates 5 and 6).
- **Console banking** (decision 12): one group per (slot, pool class, dependency level), padded.
  - Grouping: `crates/graph-compiler/src/banks.rs:136-272` and `plan_bank_groups`
    (`crates/rack-compiler/src/lib.rs:244`; rack-compiler is a production dependency of
    graph-compiler, so its tests can call it).
  - Padding: `is_console_rack` (`banks.rs:54`) and `pads` (`:75`). Pad lanes clone the first
    member's request (`:563-569`).
  - Refusal: `unbanked_console_slot` (`:496-529`) gives `console.slot.unbanked` at
    `$.console.<section>[slot=..].bank[pool=..,level=..]`; the pool comes from
    `SessionPoolClasses::class_of(&chain.track_id)`, where slice 03 seeded every submix `Stereo`.
- **Levels.** Levels are longest-path (`crates/graph-compiler/src/schedule.rs:9-81`), so a bus's
  console slots sit after every contributor's chain.
- **Backends and widths.** `Backend::Simd4` is unconditional; `Backend::Simd8` exists only where
  `avx2` is enabled; `Backend::Scalar` is `cfg(feature = "test-support")`
  (`crates/lane/src/backend.rs:28-38`); `Backend::VECTOR` lists the build's vector widths (`:41`
  onward of the `impl`). A console slot binds only at the build's own width: the EQ and compressor
  bank factories decline a foreign width, so a compile at a foreign vector width may be refused with
  `console.slot.unbanked`. `crates/graph-compiler/tests/bank_levels.rs:923-941` tolerates exactly
  that as `FOREIGN_CONSOLE_REFUSAL` (`compiled_at`), and `widths()` (`:71`) iterates
  `Scalar` then `Backend::VECTOR`.
- **The slot set.**
  - A transaction that changes the slot set must rewrite every track's entries in the same
    transaction (`docs/CONTROL_PROTOCOL_REGISTRY.md:93`, under "Decision 12: the session console").
  - Set console is opcode `0007` (`crates/protocol/src/model.rs:35`); upsert submix is `0300`
    (`:79`). The existing slot-set cases are in `crates/protocol/tests/console_session_edits.rs`.
- **The randomized generator** (`crates/host-core/tests/randomized.rs`):
  - it emits 0-2 submixes (`draw.below(3)`, `:369`) and sends from any of the seven taps into them
    (`:483-503`);
  - its `Reach` counters (`:665-671`) have no submix-console counter;
  - its tests run on `Backend::current()`, so they cover 4-lane only on arm64.
- **Wire pins.** `COMPLETE_SCHEMA_HASH` is spelled at four sites:
  - `crates/conformance/src/protocol_corpus.rs:666`;
  - `scripts/check-protocol-wasm-parity.sh:172-173`;
  - `docs/CONTROL_PROTOCOL_CONFORMANCE.md:3`;
  - `fuzz/corpus/complete-schema-manifest.md:10`.

## Decisions frozen for this slice

- **D1. Grammar.** The submix `console` is field 3, a repeated console entry, using the track's
  entry codec. Its JSON key `console` sits between `builtins` and `inserts`, so the canonical order is
  `id, builtins, console, inserts, fader, pan|matrix`. It is an array of `{ slot, bypass, params }`
  in session slot order, under exactly the track rules.
- **D2. Validation.** `validate_console_entries` takes the strip's entry slice (not `&Track`) and runs
  per strip. For a submix it reports at index paths, `$.submixes[<i>].console[<j>]`, with the
  existing codes.
- **D3. Banking.** A bus console lane joins the group its (slot, pool class `Stereo`, level)
  selects. That group may also hold other buses and any track lane at that level and class. Padding,
  bypass-in-bank and always-paid latency are decision 12's, unchanged.
- **D4. Transparent default (DESIGN P15).** `Submix::unity(id, &console)` builds one
  `{ slot, bypass: true, params: [] }` per declared slot, in slot order. A bypassed entry is
  transparent, and its latency is still paid. It is not `bypass: false` with default parameters,
  which would run every console slot on a migrated bus (VERIFY-1 MINOR-5).
- **D5. The slot set.** A transaction that changes the slot set must rewrite every **strip's**
  entries in the same transaction: submixes through `0300`, tracks as before. The store validates
  only the final candidate.

## Deliverables

1. **Session:** D1, D2 and D4.
2. **Protocol codec:** submix field 3. Re-pin `COMPLETE_SCHEMA_HASH` at its four sites, with one
   sentence of reason in the doc paragraph above it. The corpus's `UpsertSubmix` row carries console
   entries.
3. **Lowering and banking.** The console racks of a submix strip lower through `lower_strip` and bank
   per D3 in the graph and builtins compilers. No new diagnostic code; `console.slot.unbanked` keeps
   its path spelling.
4. **Generators.**
   - `crates/host-core/tests/randomized.rs` emits submix strips that carry every slot, with random
     `bypass` and params, plus random inserts. Add a `Reach` counter for submix console entries at
     dependency level 1 or more.
   - `crates/graph-compiler/tests/bank_levels.rs` does the same.
5. **Migration.**
   - Every `Submix::unity(id)` call gains the console argument.
   - Regenerate the writer corpus
     (`MISO_ENGINE_UPDATE_CANONICAL_WRITER_CORPUS=1 cargo test --locked -p session canonical_writer_corpus_is_rust_generated_and_current`).
   - `worked-session.json`'s submix gains its entries.
6. **Docs.**
   - `docs/SESSION_SCHEMA_V1.md`: "every strip carries every slot", and that a latent console slot
     is paid on every strip, so latency grows with bus depth.
   - `docs/CONTROL_PROTOCOL_REGISTRY.md`: submix field 3 in the nested registry (`:76`), and D5 in the
     slot-set paragraph (`:93`).
   - `docs/session-v1.schema.json`.

## Authorized paths

- `crates/session/**`
- `crates/protocol/src/{schema.rs,session_wire.rs,session_wire/tests.rs}`
- `crates/protocol/tests/console_session_edits.rs`
- `crates/conformance/src/protocol_corpus.rs`
- `scripts/check-protocol-wasm-parity.sh`, `fuzz/corpus/complete-schema-manifest.md` and
  `docs/CONTROL_PROTOCOL_CONFORMANCE.md` (the hash only)
- `crates/graph-compiler/**`
- `crates/rack-compiler/src/lib.rs` (only if grouping needs it)
- `crates/builtins-compiler/src/lib.rs`
- `crates/effect-compiler/src/prepare.rs`
- `crates/host-core/tests/{randomized,submix_strip,collapse_arming}.rs`
- `hosts/host-web/src/tests.rs` (gate 7's extension of the slice-03 boot test only)
- `fixtures/session-canonical/v1/canonical-writer-corpus.json` (regenerated)
- `.claude/skills/author-session/worked-session.json`
- `docs/SESSION_SCHEMA_V1.md`, `docs/CONTROL_PROTOCOL_REGISTRY.md`, `docs/session-v1.schema.json`
- this spec

## Non-goals

- No per-strip `link_mode` override (owner question Q1). A console slot's `link_mode` is the
  session's, so a stereo bus compressor that must hold the image is an insert.
- No alignment of bus levels to share banks. That is deferred item O5, triggered by the successor
  issue *Record the bus-and-send baseline and its route-work profile* (#1229).
- No live control of a bus console slot (K2).

## Hazards

- **A bus at a level no track occupies** forms its own padded group per console slot. That is
  correct but costly. Decision 12 accepted padding.
- **Entry order.** Entries follow session slot order, never canonical ID order: console entries keep
  their declared order.
- **Pad lanes clone the first member's request** (`banks.rs:563-569`). When the first member is a
  bus lane, the clone is a bus lane's request. That is fine; the padding contract is unchanged.
- **Bypass must stay in the bank.** A bypassed bus lane is still a bank member, with its latency
  paid.
- **Foreign widths.** On an x86 AVX2 build a `Backend::Simd4` compile of a console session may be
  refused; that is the build's contract, not a defect. A test that demands "no refusal" at a foreign
  width cannot pass, and a test that returns early on a width is refused by
  `run-aarch64-tests.sh`'s no-silent-skip scan.

## Objective gates

1. **The bus equals a track, with console slots.** Extend `crates/host-core/tests/submix_strip.rs`
   gate 1 (*Render a submix strip on its summed input*, #1200):
   - the session declares `pre_insert: [eq, compressor]` and `post_insert: [true-peak limiter]`;
   - the bus and the reference track carry random entries, including random `bypass`.
   - The three contributor tracks carry every entry with `bypass: true`, so each passes its source
     delayed by exactly `L = rate/100 + 6` samples (the bypassed limiter's latency). Session B's
     reference source is the D3 sum delayed by the same `L` (`L` leading zeros), so both strips see
     the same samples at the same frames; then the outputs are compared sample for sample.

   Over 32 seeds the outputs stay bit-identical, with NaNs folded.
   *Test value: it turns red if a bus's console entry is bound to another lane's parameters, if a
   bus lane is dropped from or misplaced in its padded bank, or if bypass takes a lane out of its
   bank.*
2. **Every console group binds** (graph-compiler test; VERIFY-2 M9).
   - The sessions have 1, 3, 5 and 9 first-order buses at one level, and buses at three distinct
     levels, each with the gate-1 console.
   - At `Backend::current()` and at `Backend::Scalar`, the compile is accepted (no refusal). At
     `Backend::current()` only (`Backend::Scalar` binds no banks), per (slot, pool, level) the count
     of console bank groups equals `ceil(n / W)` at the build's width `W = Backend::current().width()`.
   - For every other width in `Backend::VECTOR`, the compile is either accepted or refused with
     exactly `FOREIGN_CONSOLE_REFUSAL` (reuse `compiled_at`'s rule).
   - No width returns early or is skipped silently.

   *Test value: it turns red if bus console lanes are not banked at the build's width, are grouped
   across levels, or are padded wrongly at the build's width. Until now no console lane lived after level
   0's chains.*
3. **A bypassed bus slot keeps its latency.**
   - The bus's `post_insert` limiter entry is `bypass: true`, and a contributor is also routed
     straight to the output.
   - The contributor carries the limiter slot too, so its own path arrives at 486 samples (48 kHz)
     and the bus path at 972.
   - The direct edge's inserted compensation is exactly 486, the plan's output latency is 972, and
     the impulse aligns on both planes.

   *Test value: it turns red if bypass on a bus drops latency (decision 12, L4) and misaligns
   parallel paths.*
4. **Refusals.** A submix with a missing entry, entries out of order, or an entry naming an
   undeclared slot is refused at `$.submixes[<i>].console[<j>]` with the existing codes.
   *Test value: it turns red if submix entries escape the track rules or report a track path.*
5. **Slot-set transactions.**
   - A `0007` set-console that adds a slot without rewriting a submix refuses whole, with that
     submix's `console.entry_missing`. Nothing is committed and the revision does not advance.
   - The same transaction that also rewrites the submix through `0300` commits.
   - Both cases live in `crates/protocol/tests/console_session_edits.rs`, beside the existing
     slot-set cases.

   *Test value: it turns red if the store validates only tracks' entries after a slot-set change.*
6. **Generator reach.** The randomized differentials pass, and their generators reach submix strips
   with console entries at level 1 or more. Record the `Reach` counts in the PR.
   *Test value: a randomized differential is judged by what its generator reaches; it turns red if a
   bus console lane at level 1 or more diverges from its scalar reference on any generated
   topology, which the fixed gate-1 sessions do not span.*
7. **The live-controlled browser boot still renders** (VERIFY-2 N2). Extend slice 03's host-web boot
   test so the session also declares one console slot (`miso.parametric-eq`) and every strip,
   the submix included, carries its entry; it boots with live controls and observation taps on and
   renders 8 blocks with `RESULT_OK`. Extend slice 03's host-core gate 5 the same way: no producer
   or observation handle names the submix.
   *Test value: it turns red if a bus console slot gets a live channel during K1, which host-web
   would refuse at boot.*
8. **Render allocates nothing.** `allocations == 0` and `frees == 0` around every render call for
   gate 1's session after warm-up, measured with `bench_support::alloc`'s thread-scoped counters.
   *Test value: it turns red if a bus console lane's bank membership or padding allocates on the
   render thread, which no existing allocation test reaches because none banks a bus.*
9. **Wire, workspace, 4-lane and policy.**
   - the test-debug-b command (with conformance) and `bash scripts/check-protocol-wasm-parity.sh`;
   - the test-debug-a workspace command;
   - `bash scripts/run-aarch64-tests.sh debug`, or CI's `aarch64-debug` at the batch push;
   - `bash scripts/check-graph-determinism.sh`,
     `cargo run --locked -p graph-compiler --bin graph_fixture -- --check` and
     `bash scripts/check-console-fixtures.sh target/release/session_validator` (after
     `cargo build --locked --release -p audit -p bench -p capi -p session-validator`), with sessions
     without submixes unchanged;
   - `cargo fmt --all -- --check`;
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`;
   - `for x in session protocol-control graph builtins workspace; do bash scripts/check-$x-policy.sh && bash scripts/test-$x-policy.sh || exit 1; done`

## Evidence

- Bank-group counts per width, and which widths compiled.
- Re-pins with reasons.
- Generator reach counts.

## Dependencies

- *Delay a submix strip's summed input* (#1201)

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- "Bit-identical" gates are hard stops. NaNs are folded (decision 10). Banking couples lanes' cost,
  never their bits (decision 12).
- Render stays allocation-, lock- and syscall-free.
- One implementation shape for every target.
- In-place V1 amendment: append, never renumber.
- A test that greps source or prose is refused.
- Commit on the K1 batch branch; no push until the batch closes.
- Attempt budget: five attempts, one adversarial verdict each.
