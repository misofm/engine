# Verify: console strip design (`DRAFT-console-strip.md`)

Sol's adversarial design verification, 2026-09-29, against `codex/batch-console-1` at `5a1421a3`.
The design was read in full. Every citation below was read at that commit; the `misofm/app` citation
is from that repo's local `origin/main` ref (`0757a84`, 2026-09-25), which may be stale. Nothing was
built, run or timed. Cost figures are arithmetic on the existing native step record
`artifacts/steps/final-0400` (commit `901a1c88`, Simd8, `measurement_control: uncontrolled`), not
new measurements.

**Verdict: SOUND WITH AMENDMENTS.** The goal and the session-level shape can be built on machinery
that already exists: per-lane live bypass, rack partial-chain masks, builtin-bank padding and the
shared cohort planner. But six things are wrong as written, and each must be amended before any
slice is briefed:

- Bypass does not keep a lane in its bank today.
- The cost formula is wrong.
- Console slots have no place for `link_mode` or sidechain.
- The per-track map shape is not expressible in canonical JSON or BTLV.
- "No per-node fallback" is a cost regression for small remainders.
- S1 is too large and depends on work the design places after it.

## Findings, ranked by severity

### High

**H1. Bypass is part of the bank key, so "switching a slot off is `bypass`, so the track stays in
the bank" (draft lines 48-51) is false today, and S1's gate "every console slot must bind banked"
cannot pass.**

*What happens today:*
- `EffectProgramKey` includes `bypass` (`crates/effect-contract/src/lib.rs:984-999`). Its own doc
  explains why it is still there and what blocks removing it: every effect's bank reads one
  `metadata.bypass` for the whole bank, and "parametric-eq does not even run the wet path when
  bypassed" (`:950-983`).
- The session `bypass` becomes the prepared bypass (`crates/effect-compiler/src/prepare.rs:356`). A
  bypassed track therefore joins a different cohort from an enabled one.

*What already exists to fix it:*
- A per-lane mechanism already exists outside the effects:
  - `EffectControlRecord::Bypass` (`crates/effect-contract/src/live.rs:89-95`);
  - `BypassShunt`, which runs the wet path, preserves latency and selects whole blocks
    (`live.rs:761-883`);
  - `ConsoleEffectBankStage`, which holds one control lane per bank lane and one AoSoA shunt
    (`crates/rack/src/lib.rs:911-1030`).
- **The gap:**
  - The shunt is built only when some lane has a live control channel (`rack/src/lib.rs:996-999`).
  - Live lanes are seeded from the prepared bypass (`effect-compiler/src/prepare.rs:1331-1333`),
    which has already split the cohort.

*Cost the design does not admit:* under the shunt, a bypassed lane runs the full wet path. Today a
prepared-bypassed EQ skips it (`crates/parametric-eq/src/lib.rs:3666-3668`). A latent slot, such as
the limiter, also copies and swaps its dry line on every block, bypassed or not (#892, open).

**H2. Console slots after the insert point do not form one group per slot. They split by
dependency level, so "cost scales with `ceil(tracks / lanes)` banks per slot" (draft lines 96-98)
is wrong for any session whose tracks have different insert counts.**

- Levels are longest-path (ASAP) (`crates/graph-compiler/src/schedule.rs:9-80`).
- A bank may not cross a level (#96 F12, #966; `crates/graph-compiler/src/banks.rs:508-530`).
- The builtins already show the effect: "The fader and the matrix sit one level later for a track
  that carries a strip than for a bare one, so each of those two stages splits"
  (`crates/graph-compiler/src/lib.rs:9738-9745`; also `:8871-8873`).
- A `post_insert` slot on a track with `i` inserts sits `i` levels later than on a track with none.
  Groups per slot are therefore `sum over (pool class, level) of ceil(n / W)`, not `ceil(N / W)`. On
  the app's shape (H6), where inserts vary per track, `post_insert` fragments into several padded
  banks.
- **Knock-on cost:** chain fusion (`crates/graph/src/runtime.rs:6674-6700`, `chains_into`) needs the
  same lanes in the same order. Level-split `post_insert` banks do not line up with the
  `pre_insert` banks, so each misaligned cohort pays an extra planar/AoSoA round trip.
- A sidechained insert can also lift a track's later levels (`banks.rs:170-196`).

**H3. The design places neither `link_mode` nor sidechain, and a sidechained slot can never bank.**

*`link_mode`:*
- Today each `Effect` carries `link_mode` and `sidechain` (`crates/session/src/model.rs:255-271`),
  and both are in the key (`effect-contract/src/lib.rs:984-999`).
- The standing console fixtures run the limiter with `link_mode: maximum` on all 64 tracks
  (`fixtures/session/v1/console-sixty-four-track-intended.json`).
- The draft's session-level slot declares `effect` and `quality` only (lines 30-32).

*Sidechain:*
- `EffectProgramKey::blocks_banking` is true for a connected sidechain
  (`crates/rack/src/lib.rs:240-249`).
- The rack scratch has no sidechain planes (`rack/src/lib.rs:310-313`).
- The compressor and gate bank factories decline a sidechain (`crates/compressor/src/lib.rs:786-795`,
  `crates/gate-expander/src/lib.rs:1060-1070`).
- A per-track key source also lifts that slot's dependency level (H2).

*Consequence:* the draft's "every console slot binds banked, with no per-node fallback" (line 83)
cannot hold for a keyed console compressor.

**H4. The per-track `"console": { "eq": {...}, "comp": {...} }` map (draft lines 36-37) is not
expressible in the schema, and the restructure is a wire change the design does not rule on.**

*Canonical JSON:*
- Canonical output follows a fixed schema walk. Object fields are in schema-declared order, every
  object rejects unknown keys, and entity arrays sort by stable ID
  (`docs/SESSION_SCHEMA_V1.md:17-21, 34-38`).
- An object keyed by user-chosen slot names has no defined canonical order and no parser shape.

*BTLV:*
- The session model has a BTLV encoding with stable field IDs: track `simd1`=6, `dynamic`=7,
  `simd2`=8 (`crates/session/src/visit.rs:5-21, 89`). The protocol mirrors them
  (`crates/protocol/src/schema.rs:944-961`, `session_wire.rs:899-909`).
- Within a major, "IDs, field types, requiredness, enums ... never change or get reused", and a
  semantic change needs a major increment (`docs/CONTROL_BTLV_V1.md:59`;
  `docs/CONTROL_PROTOCOL_REGISTRY.md:74`).
- AGENTS.md forbids claiming a later generation before launch.
- The only lawful route is an explicit in-place V1 amendment on the #1063 precedent: a removed code
  is retired, never reallocated (`docs/SESSION_SCHEMA_V1.md:56-61`).

*Rack codes:*
- `RackName` and `ParameterRack` codes 1-3 (`crates/protocol/src/message_wire.rs:193-205`) are
  derived as declaration index + 1 (`crates/session/src/model.rs:10-47`).
- Retiring `simd1`/`simd2` while keeping `builtins`=4 therefore needs explicit codes or placeholder
  variants. Otherwise `builtins` silently renumbers, against "Appended, never inserted"
  (`model.rs:557-558`).

**H5. "The compiler must bind every console slot banked, with no per-node fallback" (draft line 83)
is a cost regression for one- or two-member remainders at W=8. The design should admit this or
amend it.**

Derived from `final-0400` (p50 µs per block, rounds 1/2):

| Row | µs per block | What it gives |
|---|---|---|
| 64-track console (EQ + compressor + limiter) | 100.34 / 99.68 | |
| builtins only | 25.31 / 25.38 | effect cost ≈ 74.7 µs, so ≈ 9.3 µs per full 8-lane bank of the three slots |
| 9-track ragged strip | 19.83 / 19.83 | the one per-node tail track's three slots cost ≈ 19.83 − 6.3 (two padded builtin banks) − 9.3 ≈ **4.2 µs** |

- A padded bank costs about a full bank whatever its member count. Per node wins at 1 member (4.2
  against 9.3 µs) and is about even at 2 (8.4 against 9.3 µs). Padding wins from 3 members.
- This is arithmetic on uncontrolled records, not an isolate. No per-node isolate at current HEAD
  exists, and no wasm per-node figure exists.
- At W=4 (wasm and NEON) the break-even is probably near one member, but it is unmeasured.

### Medium

**M1. Padding is feasible, but "neutral configuration" (draft line 86) must be a per-effect rule,
and S2 duplicates open issues.**

*What stops padding today:*
- The shape check `validate_shape` refuses `requests.len() != lanes`
  (`crates/effect-contract/src/lib.rs:917-923`). So does the compressor's hand-written check.
- `effect_bank_resource` refuses a mask that is not all-true
  (`crates/graph-compiler/src/banks.rs:561-565`), which fails the compile.
- `GraphPreparedEffectBank.active_mask` already exists "so a padded group can be bound without a
  second bank shape" (`crates/graph/src/lib.rs:884-886`).

*Per effect:*
- **Gate:** fills memberless lanes with all-zero parameters, which are outside its declared domains
  (`crates/gate-expander/src/lib.rs:1049`, `:308-315`).
- **Limiter:** takes its fast body only when every lane shares window shape and phase
  (`crates/true-peak-limiter/src/lib.rs:1424-1427`, `:716-728`). A dummy lane with a different
  lookahead drags the whole bank onto the slow body.
- **Whole-bank D7 recovery** zeroes or resets every lane in EQ (`crates/parametric-eq/src/lib.rs:2926-2930`),
  the compressor, the multiband, soft-clip and the limiter (`:3527-3533`). Soft-clip also charges
  every lane (`crates/soft-clip/src/lib.rs:1035-1048`).
- **EQ:** already pads internally with identity words (`parametric-eq/src/lib.rs:3123-3170`).
- **Rack:** already runs partial chains. `BankChain` zero-fills scratch, and inactive lanes are
  never gathered or scattered.

*Overlap:* #887 (tiled gather/scatter), #888 (EQ identity lanes) and #889 (compressor identity
lanes) are open. None covers the limiter.

**M2. The lane-coupling rule is not stated.**
- No bank kernel does cross-lane arithmetic in its output path. Every link mode combines L and R of
  one lane (compressor `kernel.rs:338-377`; limiter `lib.rs:295`).
- Cross-lane influence comes only from:
  - whole-bank fast-path gates, which are bit-neutral by design but couple cost (see
    `docs/rulings/compressor-idle-lane-guard-console-under-resolved.md`: "one automated track drags
    every other lane of its bank");
  - whole-bank D7 recovery;
  - bank-wide `bypass` and `link_mode`.
- **Open exception:** #1069 (multiband: a ramp cut moves a lane's bits in a bank).

**M3. The interaction with pool classes and #971 is unspecified.**
- A single bank signature per slot does hold, because class is not in the key. But groups multiply
  per class and per level (H2).
- A per-track console knob with L ≠ R moves a mono-source track into the stereo pool (DESIGNED
  witness, `crates/builtins-compiler/src/lib.rs:3862-3905`).
- #971's stranded-mono demotion keeps a move only when it binds more banks (`banks.rs:243-356`).
  Under padding every group binds, so that objective stops measuring anything.
- The mono pool is worth ≈ 36 %: `console_mono` 63.8 / 63.3 µs against `console` 100.3 / 99.7 µs.

**M4. The silence and bypass trade-off is omitted.**
- The silent fast path is bank-wide: `block_is_positive_zero` runs over `frames * lanes`
  (`crates/effect-runtime/src/bank.rs:137`).
- The `sixty_four_track_idle` row renders the same strip on silence in 30.50 / 30.64 µs against
  100.3 / 99.7 µs on tone. Silence removes ≈ 93 % of effect cost, but only when every lane of a bank
  is silent.
- Forcing every track into banks means one active stem keeps its seven bank-mates processing. Per
  node, each track would skip on its own. This works against the owner's "no processing on silence"
  preference for sparse stems.
- An "all lanes bypassed" skip is not free for a latent slot. The latency line must still be fed
  (`live.rs:818-840`), and a bank-wide skip needs every lane bypassed, which is the same
  all-lanes condition.

**M5. The addressing surface is much larger than "SDK types, capi session paths" (draft lines
112-113).**

*Taps are four enums:*
- session `SendTap` (`crates/session/src/model.rs:490-509`, JSON and wire 1-7);
- graph `TrackStage` (`crates/graph/src/lib.rs:254-262`);
- builtins `MeterTap` (`crates/builtins/src/lib.rs:4274-4282`), whose Debug names are pinned in
  builtins fixtures;
- the browser spectrum targets `trackPostInputBuiltins`/`trackPostMatrix`, pinned by
  `scripts/check-abi-layout-v1.py:89`.

*Racks have six encodings:*
- `RackName` 1-4;
- `ParameterRack` 1-4;
- `RackId` 1-3;
- `RackLocation` 1-3, in a different order (`crates/rack/src/lib.rs:213-225`);
- `EffectRack` (`crates/effect-compiler/src/prepare.rs:84-88`);
- the browser's frozen 48-byte command record: rack `0` simd1, `1` dynamic, `2` simd2, `255` n/a
  (`hosts/host-web/src/lib.rs:3760-3770`). The
  live-response owner record uses a second encoding: `0` input filters, `1..3` = `RackId`.

*Protocol session edits:*
- `SetTrackRack` (0x0204) and 0x0205-0x020e carry `rack_name` (`crates/protocol/src/model.rs:46-48,
  199-300`).
- The design needs session-level and per-track console edits it does not name.
- `COMPLETE_SCHEMA_HASH` (`crates/conformance/src/protocol_corpus.rs:249`) moves.

*Live control:* live controls are addressed `(track_id, rack, effect_index)`
(`host-core/src/prepare.rs:337-352`; `effect-compiler/src/prepare.rs:1450-1473`,
`miso.command.v1`).

*SDK:*
- The hand-written `core/types.ts`, `core/session.ts` and `core/console.ts` (`effect("simd1", ...)`)
  change.
- The shipped `.d.ts` is byte-pinned (`scripts/check-sdk-generated.sh:64-76`).
- The browser `sessionDocumentBytes` row is pinned (`check-browser-expected-resources.py:80-87`).

*Automation:* stored automation is inert today (`docs/SESSION_SCHEMA_V1.md:95-100`), so renaming
automation targets is syntax only. The live addressing above is the real change.

**M6. "console" and "strip" are already taken.**
- "console" names the live-console attachment (`HostConsoleRequest`/`HostConsoleHandles`,
  `crates/host-core/src/prepare.rs:273-352`), `ConsoleEffectBankStage`, the
  `miso_engine_web_v1_console_track_*` exports and their pinned boot words, the console benchmark,
  the console fixtures and `sdk/src/core/console.ts`.
- `"strip"` is the builtins automation `effect_id` literal (`crates/session/src/validate.rs:454`).
- A session key `console` is workable only with a vocabulary ruling.

**M7. The ordering violates AGENTS.md.**
- The chain line ("SIMD rack 1 -> dynamic rack -> SIMD rack 2") and the stable tap enum are stated
  architecture.
- AGENTS.md says "Do not make cross-cutting architecture changes without a new or amended issue".
  S3 (the AGENTS.md change) must therefore precede S1, not follow it.
- Separately, S1 bundles:
  - grammar, model, validation, canonical writer and BTLV;
  - lowering;
  - tap and rack renames across six encodings;
  - protocol session edits;
  - live addressing;
  - SDK and capi;
  - migration of 19 session documents;
  - digest repins (below).

  It also requires H1's bypass work to meet its own gate. That is several days, not half a day.

**M8. The problem statement about the app is inaccurate, so S0's "app-shaped workload" would
measure the wrong thing.**
- On app `origin/main` (`0757a84`), EQ → compressor is compiled "onto every track"
  (`src/lib/mixer/engine/session-document.ts:252-275, 476-479`).
- "Selected" is a per-track prepared `bypass` (`effectiveBypass`, `:371-383`; used at `:548-561`).
- Today a bypassed EQ track sits in its own cohort and skips the wet path. Under the console model
  it would share a bank and run the wet path.
- The honest app-shape row is "every track carries EQ → compressor in `dynamic`, a subset
  bypassed", not "effects on a subset of tracks".

**M9. Class-A gate 1 needs a mechanism, and migration churn is unaccounted.**
- If the session lowers `pre_insert` → `RackId::Simd1`, `inserts` → `Dynamic` and `post_insert` →
  `Simd2` internally, then equivalent sessions produce the identical graph, including the sealed
  `MISO-GRAPH-V1` canonical text (`crates/graph-compiler/src/canonical.rs:9-25`). Gate 1 then holds
  by construction and can be checked as "identical canonical graph text and render digests".
- The documents themselves change, and their digests are pinned:
  - `canonical.json` SHA-256 → `fixtures/builtins/v1/benchmark/prepare_256_tracks-*.toml` →
    `fixtures/builtins/v1/MANIFEST.tsv` → `ACCEPTED_MANIFEST_SHA256`
    (`tools/audit/src/builtins_graph.rs:48`);
  - the graph `MANIFEST.tsv`;
  - `fixtures/session-canonical/v1/canonical-writer-corpus.json`;
  - `console-sixty-four-track-{intended,mono}.json`, which are regenerated by
    `scripts/derive-*-console-fixture.py` and compared with `cmp`
    (`scripts/check-console-fixtures.sh:34-37`).
- All 19 rack-bearing documents carry all three keys. The fixtures whose per-track racks differ (the
  9-track ragged strip, compressor-dynamic, observation-frame-shape) migrate to `inserts`.
  Placement invariance (#163) keeps their bits, but bank counts and pinned plan shapes move.

### Low

- **L1. Parameter values never reach the key.**
  - Latency, tail, state and scratch come from the (quality, rate) row
    (`effect-contract/src/lib.rs:2271-2351`), and each rate row has one quality today.
  - The limiter's `lookahead` has no automation rate, and latency is fixed per rate
    (`true-peak-limiter/src/lib.rs:197-241`; test `:5459-5470`).
  - So per-track knobs never split a bank, and "fixed latency per slot" holds. State this as the
    verified basis.
- **L2. There is no static "bankable" flag.**
  - `EffectDescriptor` (`effect-contract/src/lib.rs:442-455`) has none. Bankability is whatever
    `bind_homogeneous_bank` returns.
  - The delay always returns `Ok(None)` (`crates/delay/src/lib.rs:605-609`).
  - EQ and the compressor refuse non-native widths (`parametric-eq/src/lib.rs:3210`,
    `compressor/src/lib.rs:800-802`).
  - The de-esser and dynamic EQ do not exist; the registry lists 8 effects
    (`effect-compiler/src/prepare.rs:98-109`).
  - "Only native effects that carry the contract" (draft line 47) needs an explicit eligibility
    list and a compile refusal.
- **L3. Gate 2's "Simd4 and Simd8" must name its runner.** Simd4 cannot be exercised on the x86
  build for EQ or the compressor. It needs `scripts/run-aarch64-tests.sh` or the wasm gates.
- **L4. Latency is always paid.** A bypassed console slot still carries its latency on every track
  (`live.rs:776-779`). A `post_insert` limiter adds `rate/100 + 6` samples of output latency session-wide
  (`true-peak-limiter/src/lib.rs:234-241`) even when every lane is bypassed. PDC stays correct and
  unchanged in mechanism (`graph-compiler/src/pdc.rs`), and needs no new delay lines between tracks
  because the latency is uniform.
- **L5. Taps cost fusion.** A send or leased meter at `insert_send`, `insert_return` or `pre_fader`
  adds a reader, so that cohort's chain cannot fuse across it (`graph/src/runtime.rs:6650-6700`).
- **L6. Slot namespace.** Slot IDs must be unique across `pre_insert` and `post_insert`, because the
  address `(track, "console", slot)` does not name the section. The existing automation-target
  shape `{entity_id, rack, effect_id, ...}` carries `rack: "console", effect_id: <slot>` with no
  shape change.
- **L7. The draft is untracked** on this branch. This verification refers to an uncommitted file.

## Amendments to the design

1. **Bypass.** A console slot's per-track `bypass` lowers to prepared `bypass = false` plus the
   initial state of the lane's `BypassShunt`.
   - Build the shunt for every console slot that has any bypassed lane, whether or not a live
     console is attached.
   - Class-A proof obligation: a shunt-bypassed lane is bit-identical to today's per-node
     prepared-bypass render, including `-0.0` and a latent slot's first block.
   - Admit the cost: a bypassed lane runs the wet path. Consider #892 for the latent limiter.
2. **Slot declaration.** The session-level slot declaration carries `slot` (a stable ID unique
   across both sections), native `identity`, `quality` and `link_mode`.
   - Per-track console entries carry only `params` and `bypass`.
   - Console slots have no sidechain. A keyed compressor is an insert.
   - A banked sidechain port is a separate future contract issue (#96 F9), with its own level-lift
     problem.
3. **Grouping.** Replace the cost sentence with the true grouping: one bank group per
   `(slot, pool class, dependency level)`, each padded.
   - Record that differing insert counts split `post_insert` and cost chain fusion.
   - Optional successor: ALAP alignment of `post_insert` banks where no cross-track dependency
     forbids it. Only on measured need.
4. **Schema shape.** Per-track `console` is an array of `{ "slot", "bypass", "params" }` in exactly
   the session's slot order (`pre_insert` then `post_insert`). Refuse:
   - any other order;
   - a missing or duplicate slot;
   - effect fields.

   Session `console` is `{ "pre_insert": [...], "post_insert": [...] }` at a declared root
   position.
5. **Wire identity**, as an explicit in-place V1 ruling on the #1063 precedent. Nothing is
   renumbered, and every retired code is refused, not reallocated.
   - Tap tokens are renamed with wire codes 1-7 unchanged.
   - `dynamic` → `inserts` keeps rack code 2 and BTLV track field 7 (same semantics).
   - `simd1`/`simd2` codes 1 and 3 and fields 6 and 8 are retired.
   - `console` is appended.
   - The browser's 48-byte command record gains a console rack byte, and the ruling decides
     `ABI_VERSION`.
   - Internal Rust names (`RackId`, `TrackStage`, `MeterTap`, `RackLocation`) stay unchanged unless
     a separate cleanup issue repins the graph and builtins manifests.
6. **Padding contract.**
   - `PrepareEffectBankRequest` gains an explicit active mask, and factories accept
     `members ≤ lanes`.
   - A dummy lane is a clone of an active member's prepared request, never zeros, fed `+0.0`, with
     its output discarded.
   - D7 recovery and reports are masked by active lanes.
   - Console slots need only absent-member lanes, not per-lane identity slots, because every track
     has every slot. That is narrower than #888's scope.
7. **Coupling rule.** "Banking may couple lanes' cost, never their bits." Any whole-bank decision
   must be bit-neutral per lane.
8. **Eligibility.** An explicit list: EQ, compressor, gate/expander, soft-clip, transient shaper,
   true-peak limiter. The delay never banks. The multiband is excluded until #1069 closes. Enforce
   it with a compile diagnostic.
9. **No-fallback rule.** Either the owner accepts "always banked" knowing a 1-2 member remainder
   costs more at W=8 (H5), or the rule becomes "banked when members ≥ a per-width threshold
   measured in S0". Record which.
10. **#971.** Restate its objective under padding (for example, minimise planes × banks) or retire
    the demotion for console slots.
11. **Silence.** State the trade-off (M4). Add a sparse-activity row to S0. Keep the
    all-bypassed skip out of scope, as drafted, and note that it must still feed latency lines.
12. **App statement.** Correct it (M8), and make S0's app shape "all tracks carry EQ → compressor, a
    subset bypassed".
13. **Vocabulary.** Rule on "console" against the live-console names and on the `builtins`/`"strip"`
    token before S1.
14. **Class A by lowering.** Lower `pre_insert` → `Simd1`, `inserts` → `Dynamic` and
    `post_insert` → `Simd2` internally, so gate 1 is "identical canonical graph text and render
    digests" for the migrated console fixtures. List every document digest the migration repins.

## Revised slice plan

Each slice is its own issue with one adversarial verdict. Sizes are estimates. Docs travel with the
slice that changes the behaviour, so there is no trailing docs slice.

| Slice | Scope | Depends on | Gates |
|---|---|---|---|
| **R0 Ruling + AGENTS.md** (docs, ≈2 h) | Amend AGENTS.md: the chain line, the tap enum, and the #163 sentence (console slots always bank; inserts bank opportunistically). Record amendments 1-14 as rulings. Create the GitHub issues. | none | `scripts/check-workspace-policy.sh`; every ruling cites evidence; the issue list matches `.github/ISSUE_SPECS/`. |
| **B0 Benchmark rows** (tooling, ≈½ day) | Add rows in `tools/console-workload` and `tools/bench/src/console.rs` only. The `timed_subjects` ratchet counts files (`scripts/check-bench-policy.sh:193`), so no new subject. Rows: EQ → comp → limiter at N ∈ {9, 10, 13, 16, 64} (9 exists as the ragged row: a remainder of 1, and 10 gives 2); app shape (every track EQ → comp in `dynamic`, one third bypassed); sparse activity (half the tracks silent). Make `strip_layout` layout-neutral now, so S4 needs no validator rewrite. Add V8 documents for N=64 console and app shape only; per-N V8 rows would be a second framework. | R0 | Preflight (`scripts/operator/preflight-console-benchmark.sh`) passes; `scripts/test-console-benchmark.sh` passes; validator record counts updated; no timed run. |
| **S0 Baseline** (evidence, one invocation) | `run-console-benchmark.sh --step console-strip-base` plus the V8 run. One warmup, two rounds. Report the `uncontrolled` status honestly; the runner has no lock, so name the operator `flock`. | B0 | Accepted records attached; no tuning or retry. |
| **P1 Per-lane bypass in banks** (engine, ≈½ day) | Amendment 1, for every bankable native effect, not just console slots: session bypass → shunt state; shunt built whenever any lane is bypassed. `effect-compiler`, `graph-compiler`, `rack`. | R0 | Mixed-bypass cohorts bind one bank; per-lane bits equal today's prepared-bypass render on random input, `-0.0` and latent slots; realtime audits and callgraph gates pass. |
| **P2a Contract padding** (≈½ day) | Active mask in `PrepareEffectBankRequest`; `validate_shape` accepts `members ≤ lanes`; `effect_bank_resource` accepts partial masks; the planner binds absent-member lanes. Every factory still declines until P2b-P2e opt in. | R0 | Existing digests unchanged; contract and graph tests pass. |
| **P2b-P2e Per-effect padding** (each ≈½ day) | P2b EQ (#888, narrowed), P2c compressor (#889), P2d limiter (new: dummy clones lookahead), P2e gate, transient shaper and soft-clip (new; gate defaults fixed; soft-clip D7 masked). Multiband after #1069. #887 is a performance follow-up, not a prerequisite: the rack already runs partial chains. | P2a | For 1..W-1 members: bits identical to per-node on random input and each effect's fixtures; dummy lanes stay `+0.0`; D7 attribution masked. |
| **S1a Schema, model and lowering** (≈1 day, two checkpoints) | Grammar, validation, canonical writer and BTLV visitor for `console`/`inserts` and the tap and rack tokens (amendments 2, 4, 5 and 14). Mechanical migration script plus regeneration of all 19 documents, with digest repins listed. | R0 | Strict parse; canonical round trip; refusal of per-track effect fields, unknown, missing, duplicate or misordered slots, console sidechain, ineligible effect and retired tokens; migrated console fixtures produce identical canonical graph text and render digests. |
| **S1b Protocol** (≈½ day) | Session-level and per-track console session edits, retired opcodes and fields, `COMPLETE_SCHEMA_HASH` repin. | S1a | Protocol corpus and wasm parity; every retired code refused. |
| **S1c Live addressing** (≈½ day) | `EffectRack`, `HostConsoleHandles`, `miso.command.v1`, the browser 48-byte rack byte and the observation and live-response rack encodings. | S1a, S1b | `scripts/check-abi-layout-v1.py` and `scripts/check-capi-abi.sh` repinned with the ruling; host-web qualification. |
| **S1d SDK** (≈½ day) | Builder API, types, `core/console.ts`, shipped `.d.ts`, the author-session skill, app handoff notes. | S1a, S1c | `scripts/check-sdk-generated.sh`, `scripts/check-sdk-types.sh`, SDK tests. |
| **S2 Banking guarantee** (≈½ day) | A test that every console slot binds banked (or follows the amendment 9 rule) at N ∈ {1, 3, 5, 9, 10, 13}, with mixed bypass and mixed insert counts. Simd8 on x86 and Simd4 on aarch64 or wasm gates. | P1, P2b-P2e, S1a | That test plus realtime audits. |
| **S4 After** (evidence) | Rerun B0's rows on the new shape. Report against S0. | S2, S1d | One invocation; before/after table. |

P1, P2a and S1a can run in parallel after R0, because they touch different crates. The app
migration follows S1d.
