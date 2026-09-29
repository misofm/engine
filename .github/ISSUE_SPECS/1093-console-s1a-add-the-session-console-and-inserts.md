# Add the session console and per-track inserts to the session schema

Slice S1a of *Console strip: session-level console effects with per-track inserts* (owner
decision 12, `docs/rulings/engine-footprint-2026-09-29.md`; Sol's H4, M5, M9, L6 and amendments 2,
4, 5 and 14 in `docs/handoffs/console-strip-2026-09-29/VERIFY.md`, commit `03aceb94`).

## Problem

Today every track declares its own `simd1`, `dynamic` and `simd2` racks
(`crates/session/src/model.rs:181-202`). Nothing makes a slot session-level, and nothing stops
tracks from diverging. Decision 12 replaces them with a session-level `console` and per-track
`inserts`. Three things block writing that directly:

- A per-track map keyed by slot name has no canonical order and no parser shape
  (`docs/SESSION_SCHEMA_V1.md`: schema-declared field order, unknown keys refused, entity arrays
  sorted by ID).
- The BTLV track fields 6-8 and the rack codes 1-3 are stable wire identity
  (`crates/session/src/visit.rs:89`; `crates/protocol/src/message_wire.rs:193-205`).
- The rack codes are derived as declaration index + 1 (`closed_tokens!`, `model.rs:10-47`), so
  retiring `simd1` would silently renumber `builtins`.

## Smallest closable slice

Work in two checkpoints.

### Checkpoint 1: model, grammar, validation, canonical writer, BTLV visitor and lowering

**The session root** gains `console`, at a declared position (proposed: immediately before
`tracks`):

```json
"console": {
  "pre_insert":  [ { "slot": "eq",   "identity": { "kind": "native", "effect_id": "miso.parametric-eq" },
                     "quality": "...", "link_mode": "..." },
                   { "slot": "comp", "identity": { ... }, "quality": "...", "link_mode": "..." } ],
  "post_insert": [ { "slot": "limiter", "identity": { ... }, "quality": "...", "link_mode": "..." } ]
}
```

**Each track** gains:
- `console`: an array of `{ "slot", "bypass", "params" }`, in exactly the session's slot order
  (`pre_insert`, then `post_insert`). `params` has today's effect-parameter shape and is
  canonicalized by `(parameter_id, channel)`.
- `inserts`, which replaces `dynamic` with the same semantics. It keeps the rack shape
  `{ "effects": [...] }`, so BTLV field 7 keeps its message type.

The track's `simd1` and `simd2` are removed. The proposed track field order is `builtins`,
`console`, `inserts`, `fader`, then pan or matrix. Either section and `inserts` may be empty.

**Tokens, per decision 12's wire-identity ruling:**
- Send taps keep wire codes 1-7 and take the new spellings: `input`, `post_input`, `insert_send`,
  `insert_return`, `pre_fader`, `post_fader`, `post_pan`.
- `RackName` switches to explicit wire codes: `inserts` = 2 and `builtins` = 4, with `console`
  appended. Codes 1 and 3 are retired, refused and never reallocated. The protocol's
  `ParameterRack` mirror follows.
- BTLV: track field 7 is `inserts`. Fields 6 and 8 are retired. Root `console` and track `console`
  are appended as the next unallocated IDs, and the nested console messages get registries of their
  own.

**Automation targets:**
- `rack: "console"`, `effect_id: <slot>` addresses a declared slot. Slot IDs are unique across both
  sections (L6).
- `rack: "inserts"` addresses an insert.
- `rack: "builtins"`, `effect_id: "strip"` is unchanged.

**Refusals.** Each is a typed diagnostic at its JSON path:
- identity, quality, link_mode, sidechain or id fields in a per-track console entry;
- an unknown, missing, duplicate or misordered console entry;
- a slot ID repeated across `pre_insert` and `post_insert`;
- a `sidechain` key in a slot declaration (console slots have no sidechain);
- a third-party (`cid`) identity in a slot;
- a slot whose effect is not on the eligibility list (`miso.parametric-eq`, `miso.compressor`,
  `miso.gate-expander`, `miso.soft-clip`, `miso.transient-shaper`, `miso.true-peak-limiter`). This
  is the compile diagnostic decision 12 requires. Raise it where native identities resolve, and
  name it (for example `console.slot.ineligible_effect`);
- the retired keys `simd1`, `dynamic` and `simd2`, the retired tap tokens and the retired rack
  tokens.

**Lowering (class A by lowering, amendment 14).** `pre_insert` lowers to `RackId::Simd1`, `inserts`
to `Dynamic` and `post_insert` to `Simd2`. Each console entry lowers to today's `Effect`:
- `id` = the slot;
- `identity`, `quality` and `link_mode` from the session slot;
- `bypass` and `params` from the track's entry;
- `sidechain` = none.

The internal names `RackId`, `TrackStage`, `MeterTap` and `RackLocation` stay. So does the sealed
`MISO-GRAPH-V1` canonical text. An equivalent session therefore compiles to the identical graph by
construction.

**The protocol's session-model encoding** (`crates/protocol/src/schema.rs`, `session_wire.rs`)
moves with the model so the workspace compiles and round-trips, and `COMPLETE_SCHEMA_HASH` is
repinned for that. Session edits are S1b's.

### Checkpoint 2: migrate every document and repin

Migrate mechanically, rack by rack. A rack is *uniform* when every effect in it is eligible and
every track carries an identical declaration sequence in it: the same IDs, identity, quality and
link mode, and no sidechain. A uniform rack becomes console. Each of its effects becomes a slot
whose ID is the effect's ID, and each track's entry takes that track's params and bypass.

- `simd1` becomes `pre_insert` when it is uniform.
- `simd2` becomes `post_insert` when it is uniform.
- `dynamic` becomes console when it is uniform and the document has two or more tracks. This is the
  rule that moves the app shape (below).
  - It is appended to `pre_insert` when `simd1` is console or empty.
  - Otherwise it is prepended to `post_insert` when `simd2` is console or empty.
  - Otherwise it stays in `inserts`.

  A single track shows no strip shared across tracks, so a single-track document keeps `dynamic`
  as `inserts`.
- Every rack that does not become console folds into `inserts` in chain order: `simd1`, then
  `dynamic`, then `simd2`. Chain order is preserved in every case.
- An ID collision is refused, never silently renamed. That covers inserts folded together and a
  slot ID repeated across `pre_insert` and `post_insert`.

**Named exceptions.** These documents keep a uniform `dynamic` as `inserts`. They are the
console-versus-insert placement witnesses that AGENTS.md's "a placement change must not move a
rendered bit" needs:
- `compressor-dynamic-bank-observation.json`, the insert twin of `compressor-bank-observation.json`;
- `console-sixty-four-track.json`, the legacy row: EQ as a console slot and the compressor as an
  insert. Its twin is the `sixty_four_track_eq_comp_simd1` row, which has both as console slots.

**The app shape.** B0's app-shape row, native and V8, carries EQ -> compressor in `dynamic` on
every track, with the EQ and the compressor bypassed on tracks whose index is 2 mod 3. Under the
rule it becomes two `pre_insert` slots, and each track's entry keeps its bypass, so the 2-mod-3
pattern is unchanged. S1a owns that move:
- in the `tools/console-workload` builder;
- in B0's app-shape V8 document;
- in the V8 harness's fixture lookup (`scripts/web-mixing-automation-benchmark.mjs:136-140` reads
  `track[["simd1","dynamic","simd2"][rack]]`), which must read the console entries and `inserts`
  instead.

The harness's rack byte (`:279`) and the rack codes in the controls example
(`tools/console-workload/examples/mixing_automation_controls.rs`, through `src/mixing_automation.rs`)
are S1c's.

Until S1c, the browser record still addresses the lowered racks: `pre_insert` as `0`, `inserts` as
`1` and `post_insert` as `2`, each with an index within its section. S1a keeps
`mixing_automation.rs` emitting exactly that, reading the console entries in place of
`track.simd1`/`dynamic`/`simd2`.

A `dynamic` rack moved into a console section lowers to `Simd1` or `Simd2` rather than `Dynamic`.
Its graph text therefore changes by design. Placement invariance (#163) keeps its bits.

The migration tool is a one-off. Commit it only if a gate or a documented operator path reaches it
(`scripts/check-script-reachability.py`); otherwise attach it to the PR. Update
`scripts/derive-intended-console-fixture.py` and `scripts/derive-mono-console-fixture.py` to emit
the new shape.

**Documents** (18 sessions and the JSON Schema):
- `fixtures/session/v1/` `builtins-automation`, `canonical`, `compressor-bank-observation`,
  `compressor-dynamic-bank-observation`, `compressor-dynamic-observation`,
  `console-sixty-four-track{,-intended,-mono}`, `observation-frame-shape`,
  `parametric-eq-bank-console` and `parametric-eq-nine-track`;
- `hosts/host-web/qualification/{console,observation,stall}-session.json`;
- `hosts/host-web/tests/browser-v1/{command-session,observation-session,session}.json`;
- `crates/graph-compiler/tests/data/reduced-nobus-from-970-verify.json`;
- `docs/session-v1.schema.json`;
- plus the inline session documents in Rust tests and builders.

R0 classified the 18 documents under this rule:
- Only `reduced-nobus-from-970-verify.json` has divergent `simd1`/`simd2` racks. They fold into
  `inserts`. Placement invariance (#163) keeps its bits, but its plan shape may move; explain every
  moved pin.
- The `dynamic` clause moves no committed document. Its only multi-track uniform cases are the two
  named exceptions. The single-track documents keep `inserts`: compressor-dynamic-observation, the
  host-web sessions and canonical, whose `dynamic` is not uniform anyway. So does
  observation-frame-shape, whose `dynamic` diverges. The clause moves the app-shape row.
- Sol's M9 listed the 9-track ragged strip among the folds. That workload truncates the intended
  fixture, whose racks are uniform, so it becomes console. So do B0's other strip rows and its
  sparse-activity row.

**Digest chain to repin (M9)**, with a one-line reason for each:
- `fixtures/session/v1/canonical.json` SHA-256, then
  `fixtures/builtins/v1/benchmark/prepare_256_tracks-{48000,96000}.toml`, then
  `fixtures/builtins/v1/MANIFEST.tsv`, then `ACCEPTED_MANIFEST_SHA256`
  (`tools/audit/src/builtins_graph.rs:48`);
- `fixtures/graph/MANIFEST.tsv`;
- `fixtures/session-canonical/v1/canonical-writer-corpus.json`;
- the console fixtures, which `scripts/check-console-fixtures.sh` compares with `cmp`;
- the browser `sessionDocumentBytes` row (`scripts/check-browser-expected-resources.py:80-87`).

**Docs and specs that travel with this slice:**
- `docs/SESSION_SCHEMA_V1.md` (shape, tokens, retired codes on the #1063 precedent, lowering);
- `docs/session-v1.schema.json`;
- `docs/BUILTINS_AND_METERING_V1.md`, which spells the old taps (`:4`, `:171-174`);
- the open specs #973 and #987, and any other open spec whose gates name a retired rack or tap
  token. Map their vocabulary without changing their scope.

**Benchmark tooling that reads the old shape, and that this slice keeps running:**
- `tools/console-workload/**`: its builders edit `track.simd1`, `track.dynamic` and `track.simd2`;
- `tools/bench/src/console.rs`;
- `scripts/check-console-benchmark-fixture.sh`, the required qualification step "Console benchmark
  fixture integrity". It reads `simd1`/`dynamic`/`simd2` and `tap == "post_matrix"`;
- `scripts/check-console-fixtures.sh`;
- `scripts/test-console-benchmark.sh`;
- the V8 harness's fixture lookup and B0's V8 documents (above).

Authorized paths:
- `crates/session/**`;
- `crates/protocol/src/{schema,session_wire,message_wire}.rs`, for the model encoding only;
- the lowering call sites that read `simd1`/`dynamic`/`simd2`;
- the documents and digest pins above;
- the two derive scripts;
- the docs, specs and benchmark tooling above;
- any other file that one of this slice's gates reads and the schema change breaks, named in the PR
  with the gate that needed it;
- this spec.

## Owner decisions that bind this slice

All of decision 12's "Shape", "Chain", "Sidechain", "Eligibility", "Wire identity" and "Class A by
lowering". There is no `ABI_VERSION` bump.

## Dependencies

- *Rename the live console to live controls* (S1r, #1095) merges before this slice. It is first in
  batch C3, so its SDK and browser gates run against an unchanged schema.
- *Add the console-strip benchmark rows* (B0, #1085) and *Record the console-strip baseline
  benchmark* (S0, #1086) have landed in batch C1.

From this slice until S1d, the SDK suites, `scripts/check-sdk-generated.sh` and the SDK-driven
browser qualification are knowingly out of step: the SDK writes `simd1`/`dynamic`/`simd2`
(`sdk/src/internal/session-json.ts:113-117`). They are not this slice's gates, and batch C3 is not
pushed until S1d passes them.

## Objective gates

1. Strict parse and canonical round trip of the new shape, with empty sections and empty `inserts`
   included. BTLV round trip through the visitor.
2. Each refusal listed above has a test with its code and JSON path. Retired tokens, keys and wire
   codes 1 and 3 are refused, never reinterpreted.
3. `RackName` and `ParameterRack` wire codes are explicit and round-trip (`builtins` stays 4). A
   planted index-derived code turns
   `parameter_enum_wire_mappings_are_exhaustive_and_roundtrip` red.
4. PR evidence, not a committed test, against the pre-change base:
   - identical canonical graph text for every migrated document whose racks keep their internal
     rack (`simd1` to `pre_insert`, `simd2` to `post_insert`, `dynamic` to `inserts`);
   - identical render digests for every migrated document and for the app-shape row, including
     those whose racks moved.

   List every repinned digest with its reason.
5. `bash scripts/check-session-policy.sh`, `bash scripts/check-console-fixtures.sh`,
   `bash scripts/check-console-benchmark-fixture.sh`, `bash scripts/test-console-benchmark.sh`, the
   protocol corpus and `cargo test --workspace` pass.
6. The benchmarks still run on the new shape. Both checks are untimed, on a committed clean tree:
   - `bash scripts/operator/preflight-console-benchmark.sh --step <an unused scratch name>`;
   - `bash scripts/run-web-mixing-automation-benchmark.sh prepare WORKDIR`, then `preflight
     WORKDIR`, in an empty scratch directory.

   The app-shape row's compiled plan carries EQ and compressor as `pre_insert` slots, with the
   2-mod-3 bypass pattern.

## Standing rules for the implementer

- Work only from this body, the umbrella issue and decision 12. Read the cited code first.
- Class A: every gate that says "bit-identical" is a hard stop, not a tolerance. NaNs fold to one
  value (decision 10).
- Render stays allocation-, lock- and syscall-free. Only `crates/lane` names `wide` or intrinsics.
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features --
  -D warnings` and the focused tests before every checkpoint.
- Every new test names the plausible defect that turns it red (AGENTS.md "Test value"). A
  one-time "no bit moved" comparison against the pre-change base is PR evidence, not a committed
  test.

## Attempt 1 evidence

Terra, attempt 1, on `codex/1093-session-console-inserts` (rebased onto `11ea359b`, R0 attempt 2;
the pre-change base for every comparison is `6fdf5db2`, whose code is identical to `11ea359b`'s).
The migration follows the amended rule at `11ea359b`: uniform `simd1`/`simd2` become console
sections; a uniform `dynamic` joins the console only with two or more tracks and outside the two
placement witnesses, so it moves no committed document.

### What landed

- **Session crate.** `Console { pre_insert, post_insert }` of `ConsoleSlot { slot, identity,
  quality, link_mode }` at the root between `sources` and `tracks`; `Track { .., builtins,
  console: Vec<ConsoleEntry { slot, bypass, params }>, inserts: Rack, fader, pan|matrix }`.
  Parser, validator, canonical writer and visitor follow the schema order. `SendTap` takes the new
  spellings with codes 1-7 unchanged. `RackName` moves to explicit wire codes (a second
  `closed_tokens!` arm): `inserts` 2, `builtins` 4, `console` 5; 1 and 3 decode to nothing.
  Visitor fields: root `console` 15, track `inserts` 7, track `console` 11, fields 6 and 8 retired;
  the slot and entry messages have registries of their own. `SessionModel::lower_track` is the
  one lowering (`pre_insert` -> first rack, inserts -> second, `post_insert` -> third).
- **Refusals** (each tested with code and path, `crates/session/tests/console_schema.rs`,
  `diagnostic_parity.rs`, `strict_unknowns.rs`, `visit_model.rs`): effect fields in an entry and
  `sidechain`/`bypass`/`params` on a slot (`schema.unknown_field` at the key); unknown
  (`reference.missing_entity`), duplicate (`id.duplicate`), misordered (`console.entry_order`) and
  missing (`console.entry_missing`) entries; a slot ID repeated across sections (`id.duplicate`); a
  `cid` slot (`console.slot_not_native`); retired `simd1`/`dynamic`/`simd2` track keys, tap spellings
  and rack tokens; rack codes 1 and 3 (session and protocol tables); BTLV track fields 6 and 8
  (`UnknownRequiredField`). Ineligible slots are refused where native identities resolve:
  `effect_compiler::CONSOLE_ELIGIBLE_EFFECTS` and `console.slot.ineligible_effect` at
  `$.console.<section>[slot=<id>].identity` (delay, multiband, unknown identity; the delay as an
  insert still prepares).
- **Protocol model encoding.** Track message: field 7 inserts, 11 repeated `ConsoleEntry` (slot 1,
  bypass 2, param 3); `ParameterRack` explicit codes; the mapping test also holds
  `RackName::wire()` to the table (an index-derived code turns it red). `rack_mut` answers
  `NotFound` for `console` until S1b.
- **Lowering call sites.** effect-compiler, graph-compiler (compile passes, bank planner),
  host-core, host-web's effect index, console-workload. Graph stage names, graph diagnostic paths
  and the `MISO-GRAPH-V1` text are unchanged; `graph_fixture --check` passes on the unchanged
  manifest.
- **Migration.** `docs/handoffs/console-strip-2026-09-29/migrate-console-inserts.py` (the PR
  attachment, since the batch opens no per-slice PR; no gate runs it) reproduces all 19 migrated
  documents byte for byte from `6fdf5db2`. Shapes: `pre_insert` from a uniform `simd1` in
  compressor-bank-observation (`comp`), console-sixty-four-track (`eq`), -intended and -mono (`eq`,
  `comp`), parametric-eq-bank-console and -nine-track (`eq`) and qualification/observation-session
  (`eq-simd1`); `post_insert` in -intended/-mono (`limiter`) and qualification/observation-session
  (`eq-simd2`); `dynamic` -> inserts everywhere; reduced-nobus folds all three racks into inserts.
  The derive scripts emit the console shape and regenerate both console fixtures byte for byte.
- **Tests restated.** Builders whose per-track chains differ now build inserts, and their rack
  assertions read `Dynamic`; uniform strips stay console slots and keep `Simd1`/`Simd2`. Randomized
  generators (host-core, `bank_levels`, graph-compiler) place their chains by the migration rule.
  The placement witness compares a keyless compressor strip as a `pre_insert` slot against the
  same compressors as inserts, and checks each is planned in its own rack.
  `cross_index_effect_session` keeps its premise (one program in SIMD-1 on one track and in the
  dynamic rack on another) with six owners instead of four. No graph-compiler numeric pin moved.

### Class A (PR evidence, not committed)

A scratch harness compiled each document through the launch pipeline at `Backend::Simd8` for the
canonical graph text, and rendered 48 quanta of a deterministic per-source signal through
host-core's prepare path (NaNs folded). Base = old documents at `6fdf5db2`; candidate = migrated
documents here. Compared with `cmp`; digests are SHA-256 prefixes of the candidate bytes.

| Document | Graph text | Render |
|---|---|---|
| `builtins-automation` | identical `6b05f0a198d8` | identical `de82a43a1812` |
| `canonical` | identical `977d39a69c6b` | identical `61c35ef8db84` |
| `canonical-minimal` | identical `b573214358d3` | identical `35e54ac3342b` |
| `compressor-bank-observation` | identical `2423f3bdf300` | identical `1a330c619014` |
| `compressor-dynamic-bank-observation` | identical `cb926c5212ff` | identical `1a330c619014` |
| `compressor-dynamic-observation` | identical `7a9980baa753` | identical `d82243d18d59` |
| `console-sixty-four-track` | identical `81470da0a39f` | identical `31a68e1cef05` |
| `console-sixty-four-track-intended` | identical `beefd603ef4d` | identical `27fdee3ad6da` |
| `console-sixty-four-track-mono` | identical `beefd603ef4d` | identical `46f8400598e2` |
| `observation-frame-shape` | identical `3a0a475de6df` | identical `e6339fa93f72` |
| `parametric-eq-bank-console` | identical `71d450ae757c` | identical `1e1e42533b07` |
| `parametric-eq-nine-track` | identical `957e97ca86f8` | identical `03e6e94589c7` |
| `qual-console` | identical `c9fa0104db70` | identical `d82243d18d59` |
| `qual-observation` | identical `2367f1246bf6` | identical `d717de9efbaa` |
| `qual-stall` | identical `c9fa0104db70` | identical `7930d2b61e37` |
| `browser-command` | identical `b585c4a492d4` | identical `363adc8df203` |
| `browser-observation` | identical `4bc9d01929f3` | identical `0f06fdc4fa04` |
| `browser-session` | identical `c9fa0104db70` | identical `b16b4d3ab3f9` |
| `reduced-nobus` | moved `26b904b44b59` | identical `f69c09c30897` |

`canonical` refuses at effect preparation (its unprefixed `parametric-eq`) and `canonical-minimal`
at source preparation, identically on both sides. `reduced-nobus` is the one document whose racks
move (all three fold into inserts; graph text `6b571db317ac` -> `26b904b44b59`); its bits are
identical.

**App shape (placement change).** The intended fixture with EQ -> compressor moved to `dynamic`
on all 64 tracks and both bypassed on tracks 2 mod 3, rendered at the base; its migration
(`dynamic` -> two `pre_insert` slots, bypass kept per entry) rendered here: identical PCM
(`3f7523f1714a`), graph text differs by design (Dynamic -> Simd1). B0's app-shape builder and V8
document are not on this branch (B0 has not landed); porting them is a rebase item (below).

### Repinned digests and why

- `fixtures/session/v1/canonical.json` SHA-256 `1ed6ca31...b58ff4fd` -> `5f887676...a13c07`
  (root console, inserts, `post_pan`, rack `inserts`), carried by
  `benchmark/prepare_256_tracks-{48000,96000}.toml` (also `route_source_tap` `post_matrix` ->
  `post_pan`) and the audit's field table.
- `fixtures/builtins/v1/MANIFEST.tsv`: only those two rows (963 -> 960 bytes); regenerated by
  `audit fixture-builtins --write`, every PCM, meter, response and resource payload unchanged
  (the audit's graph-tap fixture keeps a delay in each lowered rack).
- `ACCEPTED_MANIFEST_SHA256` (and the audit test's copy) `9161d2ca...5b4ff9d3` ->
  `fced289f...b1e2f9`.
- `fixtures/graph/MANIFEST.tsv`: unchanged.
- `canonical-writer-corpus.json`: regenerated (the full-surface document now carries console
  slots, entries and a keyed third-party insert; `canonical-minimal` gains the empty console).
- Console fixtures: regenerated by the derive scripts; `check-console-fixtures.sh` passes.
- Browser `sessionDocumentBytes` 1919 -> 1905 (`browser-v1/session.json` migrated); the
  browser-correctness digests in `expected.json` still match the built module.
- `COMPLETE_SCHEMA_HASH` `e4dec003302d891a` -> `af1b9b71a0a31727` (track encoding, `inserts` rack
  code, a console entry in the corpus track), with the parity self-test literal, the conformance
  doc and the fuzz manifest.
- `parametric-eq-nine-track.json` FNV pin 16,712 bytes / `0x95f30d0f2c185ce0` -> 13,729 /
  `0x6af0899538c903b6`.
- capi pinned response vectors: snapshot length `0x4148` -> `0x35a1`; the first metadata row's rack
  1 -> 5 (`console`).
- `ProtocolController` 6,032 -> 6,080 and `PreparedStructuralCommand` 728 -> 776 bytes
  (`SessionModel` gains the root console).
- `PARSE_TRANSIENT_MULTIPLIER` 17 -> 20: the minimal document is now 511 bytes with an 8,944-byte
  parse/model/compile peak (17.503 per byte; base 447 / 6,780 / 15.168); dense documents stay below
  13.1. Gate: host-web `boot_transient_budget`.
- `bank_levels` reduced #970 reproducer: misaligned slots `[1, 1]` -> `[0, 0]` (the soft-clip that
  was a later chain now extends `t11`/`t13`'s insert chain); the four-lane compressor-bank pin stays
  1, derived from the four-lane plan (factories decline four lanes on x86). The #966 case stays in
  the four `console_less_eqs` tests (now inserts) and #970's later-chain case in host-core's
  `collapse_arming.rs`.

### Minimal changes outside S1a, for the later slices

- S1b (#1094): console session edits (`rack_mut` refuses `console`; capi's catalog test removes a
  track instead of an EQ, and the swap-window test edits an insert); an explicit refusal of the
  retired track fields 6 and 8 when a peer flags them optional (today only the mandatory form is
  refused); `CONTROL_PROTOCOL_REGISTRY.md`'s session-edit registry beyond the one rack-code line.
- S1c (#1096): host-core's parameter metadata names a lowered console effect
  `ParameterRack::Console` and an insert `Inserts`; the web host's rack bytes, console-workload's
  `mixing_automation.rs` and the V8 harness keep addressing the lowered racks `0/1/2`.
- S1d (#1097): the SDK writer, types and tests, the browser qualification scripts and the shipped
  AudioWorklet artifact pin (every Rust change here moves the module).
- Rebase onto B0: port B0's app-shape builder and V8 document to `console.pre_insert` (the rule's
  one moving case), and reconcile `tools/console-workload`.
- `prepare_native_session_effects_with_console_eligibility` lets test and audit registries admit
  the conformance test double as a console slot; every host prepares through the fixed list.

### Gates

All on the final tree (`0c71b280`), x86-64-v3:
- `cargo check --workspace --all-targets --all-features` (through clippy), `cargo clippy --locked
  --workspace --all-targets --all-features -- -D warnings`, `cargo fmt --all --check`: pass.
- `cargo test --locked --workspace --all-features --no-fail-fast`: 284 test binaries green; the two
  that failed (the audit's second manifest pin, session-validator's fixture edit) were fixed in
  `0c71b280` and rerun green (`-p audit -p session-validator`).
- The session, protocol, conformance, effect-compiler, graph-compiler (lib and every integration
  suite), builtins-compiler, host-core, capi, host-web, console-workload, parameter-metadata and
  session-validator suites pass within that run.
- `check-console-fixtures.sh` (both fixtures regenerate byte for byte), `check-console-benchmark-fixture.sh`,
  `test-console-benchmark.sh`, `check-builtins-fixtures.sh`, `graph_fixture --check`,
  `check-protocol-wasm-parity.sh` and its `--self-test`, `check-browser-expected-resources.py`
  (built module; digests unchanged, one exact row repinned).
- Policy scripts: session, workspace, graph, host-core, rack, realtime, builtins,
  protocol-control, lane, env-vocabulary, effect-runtime, unfused-seal, artifact-evidence-leak,
  conformance-boundaries, bench, dsp-research; and with `python3 -B`: command-kind and
  command-reason vocabulary, session-map-shape, script-reachability, ci-path-routing,
  test-support-ci, release-shape, sdk-deletions. The argument-taking Python checks
  (parameter-metadata, abi-layout, scalar-oracle) run inside the web-artifact gates, which are the
  batch boundary's.
- AArch64 legs on x86 with `--no-run`: the debug leg (capi's product closure plus dsp-reference,
  conformance, target-smoke, `builtins-compiler/test-support,graph/test-support`), the release
  gates (`lane`, `math` with `math/lane`) and `console-workload` release all compile; the
  known-defect rows judge their listings (`judge-skips debug` and `release` pass); the no-silent-skip
  scan finds nothing.
- Gate 6, untimed on the committed clean tree: `operator/preflight-console-benchmark.sh --step
  s1a-scratch-preflight` passes (workload launches 0); `run-web-mixing-automation-benchmark.sh
  prepare` then `preflight` in an empty scratch directory pass (the module is not the released
  pin, as expected before the batch's artifact rebuild).
- Not run: the SDK suites, `check-sdk-generated.sh` and the SDK-driven browser qualification, which
  are knowingly out of step until S1d; the shipped AudioWorklet artifact pin, which the batch
  boundary rebuilds.
- B0's app-shape row: not on this branch, so its compiled-plan gate (EQ and compressor as
  `pre_insert` slots with the 2-mod-3 bypass) is shown above on an equivalent probe and must be
  rerun on B0's row after the rebase.

## Sol verdict, attempt 1

**PASS**, for `64f51417` (11 commits on `11ea359b`), with one binding rebase obligation (finding 1).
Everything below was re-derived independently on x86-64-v3, not read from the evidence above.

### What was verified

- **Schema against decision 12.** Slots are exactly `{slot, identity, quality, link_mode}`, slot
  IDs are unique across both sections, and there is no sidechain. The per-track `console` is
  exactly `{slot, bypass, params}` in slot order. `inserts` replaces `dynamic`, and `simd1`/`simd2`
  are gone. Taps are renamed with codes 1-7 unchanged. `RackName` and `ParameterRack` carry explicit
  codes: `inserts` 2, `builtins` 4, `console` 5, with 1 and 3 refused. BTLV track fields are `inserts`
  7 and `console` 11, and 6 and 8 are retired. Root `console` is field 15. Root field 8 was the
  retired `limits`, so 15 is the next unallocated ID. There is no `ABI_VERSION` change. Planting an
  index-derived `RackName::wire()` turns
  `parameter_enum_wire_mappings_are_exhaustive_and_roundtrip` red, which is gate 3.
- **Refusals** (gate 2). I ran 55 crafted documents through `session_validator`, and each refusal
  carries its code at its path:
  - each effect field in an entry;
  - `sidechain`, `bypass`, `params` and `id` on a slot;
  - unknown, missing, all-missing, duplicate and misordered entries;
  - a slot repeated within a section and across sections;
  - a `cid` slot;
  - `miso.delay`, `miso.multiband-compressor` and an unknown native slot, refused with
    `console.slot.ineligible_effect` at stage 5;
  - a missing root `console`, a missing section and a missing track `console`;
  - the retired `simd1`, `dynamic` and `simd2` keys;
  - all five retired tap spellings;
  - `simd1`, `dynamic` and `simd2` as automation racks.

  These are accepted: all seven new taps; empty sections with empty inserts; a delay insert; an
  insert ID equal to a slot ID; `rack: "console"` automation on a declared slot parameter; and
  root keys out of order, which the base accepts too.
- **Canonical round trip.** For all 18 documents that the validator can re-serialize, running the
  migration script over the *base* canonical output gives exactly the branch's canonical output, and
  that output is a fixed point. `canonical.json` is byte-canonical on both sides (checked through
  `canonical_session_json`). Which documents are byte-canonical is unchanged from the base.
- **Migration.** Rerun on the `6fdf5db2` documents, the committed script reproduces all 19
  migrated documents byte for byte. My own rack classification of the base documents agrees with the
  rule:
  - the only multi-track uniform `dynamic` racks are the two named witnesses;
  - `observation-frame-shape`'s `dynamic` diverges;
  - `canonical`'s is ineligible;
  - `reduced-nobus` is the one document whose `simd1` and `simd2` diverge.
- **Class A** (gate 4), from my own harness. The harness compiles each document at `Backend::Simd8`
  to get `MISO-GRAPH-V1` text, then renders 48 quanta through `prepare_host_session` from a
  per-source deterministic signal, with NaNs folded. The renders are not silent.
  - **Render bits.** Every document that renders has identical PCM bits: 17 documents, plus B0's
    own app fixture against its migration. `canonical` refuses at effect preparation and
    `canonical-minimal` at source preparation, with identical diagnostics on both sides.
  - **Graph text.** Every document's graph text is identical except `reduced-nobus` and the app
    shape. My graph digests match the table above, for example `reduced-nobus` `6b571db317ac` ->
    `26b904b44b59`. The `reduced-nobus` move is legitimate: its per-track racks cannot be console
    slots, because every track must carry every slot. The rule folds them into `inserts`, the
    diff is placement (`simd1`/`simd2` -> `dynamic`), and the bits hold.
  - **App shape.** The app shape moves Dynamic -> Simd1: 64 `simd1:eq` and 64 `simd1:comp` nodes,
    with no Dynamic effect node left.
- **Digest chain.**
  - `canonical.json` SHA-256 `5f887676...` equals the `session_template_sha256` in both
    `prepare_256_tracks` TOMLs.
  - Those TOMLs' 960-byte sizes and SHA-256s equal their `MANIFEST.tsv` rows.
  - `sha256(MANIFEST.tsv)` = `fced289f...` = `ACCEPTED_MANIFEST_SHA256`, and the audit test's copy
    matches.
  - The graph `MANIFEST.tsv` and the builtins graph PCM and meter digests are unchanged.
  - The `console-workload` release digests pass unchanged.
- **The escape hatch.** `prepare_native_session_effects_with_console_eligibility` is called only by
  `graph-compiler`'s `#[cfg(test)]` module, `graph-compiler/tests/compile_shapes.rs` and
  `tools/audit`. `host-core`'s prepare path (`prepare.rs:797`), and through it `capi` and
  `host-web`, uses the fixed list.
- **Edits outside S1a.** Each is compile-, gate- or spec-driven:
  - `rack_mut`;
  - `protocol_rack`;
  - `count_effects`;
  - `host-web`'s per-track rack counts;
  - the size pins and the `COMPLETE_SCHEMA_HASH` repin, with its parity, conformance-doc and fuzz
    copies;
  - the registry's rack-code line;
  - test adaptations;
  - the #973 and #987 vocabulary maps.

  The follow-ups listed above are covered by the #1094, #1096 and #1097 specs as written: #1094
  gate 2 covers fields 6 and 8, and #1097 covers the SDK, the qualification entries and the
  `author-session` skill.
- **Gates.** All of the following pass:
  - clippy `-D warnings` over the workspace (all targets, all features), and fmt;
  - `cargo test --locked --workspace --all-features --no-fail-fast`: 286 result lines, 0 failed,
    covering session, protocol, graph-compiler, builtins-compiler, effect-compiler, host-core and
    capi;
  - `console-workload` in release;
  - `check-console-fixtures.sh`, `check-console-benchmark-fixture.sh`, `test-console-benchmark.sh`,
    `check-builtins-fixtures.sh` and `graph_fixture --check`;
  - `check-protocol-wasm-parity.sh` and its `--self-test`;
  - `check-browser-expected-resources.py` against a freshly built module;
  - `check-session-policy.sh` and 17 other bash policy and check scripts;
  - 10 Python checks and 4 `--self-test`s, all run with `python3 -B`;
  - both AArch64 legs on x86 with `--no-run`: the debug product closure with its features, and the
    release `lane`/`math`, `console-workload` and `audit` builds, with both `judge-skips` green and
    the no-silent-skip scan clean.

  The benchmark preflights ran untimed in a clean scratch worktree:
  - `operator/preflight-console-benchmark.sh --step sol-1093-scratch-preflight` PASS, with 0
    workload launches;
  - `run-web-mixing-automation-benchmark.sh prepare` then `preflight` PASS, with module
    `bb33308b...` (not the release pin, as expected).

  Not run: the SDK suites and the SDK-driven browser qualification, which belong to S1d, and the
  AudioWorklet artifact pin, which belongs to the batch boundary.

### Findings, by severity

1. **Medium (carried; binds the rebase).** Gate 6's app-shape row cannot be checked on this branch.
   B0 (#1085) has not landed, although the spec says it has.
   - **Trial merges.** `c4906414` merges clean. B0 merges without a textual conflict but breaks:
     - `cargo check --keep-going` fails in `tools/console-workload/src/lib.rs` only (18 errors:
       `.simd1`, `.dynamic` and `.simd2`, and `SendTap::PostMatrix`);
     - these fail at run time: `fixtures/session/v1/console-sixty-four-track-app.json`,
       `scripts/derive-app-console-fixture.py`, B0's app block in
       `scripts/check-console-fixtures.sh` (it reads `simd1`/`dynamic`, and its `outer(app) ==
       outer(intended)` must now exclude `console`), and B0's `DOCUMENT_KINDS`/`SECTIONS` block in
       `scripts/web-mixing-automation-benchmark.mjs`.
   - **The rebase must:**
     1. Migrate B0's fixture with the committed script. I ran it: `dynamic` is appended to
        `pre_insert`, because `simd1` is empty. That gives `pre_insert` [`eq`, `comp`], an empty
        `post_insert`, and 21 tracks with both entries bypassed (2 mod 3). The result validates
        through stage 5 and renders bits identical to B0's fixture at the base.
     2. Port the derive script to the migrated intended fixture: clear `post_insert` and each
        limiter entry, and bypass both entries on 2-mod-3 tracks, so that `cmp` reproduces the
        fixture.
     3. Port the builder, its tests, the witness block and the V8 block.
     4. Re-run gate 6 and the app-row class-A render on the real native and V8 rows before C3 is
        pushed.
   - **P1 (#1087).** P1 conflicts in `effect-compiler/src/prepare.rs` (adjacent constants: keep
     both) and in `effect-compiler/tests/native_session.rs` (two tests appended in the same place;
     port P1's `bypassed_console()`). Once those are resolved, P1 fails to compile only in its
     `graph-compiler/tests/bypass_cohorts.rs` and `host-core/tests/symmetry_witness.rs`. P1 and B0
     do not conflict with each other.
2. **Low.** The escape hatch is an ungated `pub fn` in the production crate. Nothing shipped calls it
   today, but the repo's convention is `#[cfg(any(test, feature = "test-support"))]` plus
   `#[doc(hidden)]`. `effect-compiler/test-support` exists, and `tools/audit` already enables other
   `test-support` features. Gating it would make "not reachable from production" structural.
3. **Low.** `validate_console_entries` is O(tracks x slots^2), because it searches `slots().any` per
   entry and `console.iter().any` per slot, while the validator is otherwise linear. One track with
   6,000 slots takes 1.81 s in the debug `session_validator`, against 0.57 s for 6,000 inserts. The
   cost is on the control plane and bounded by the document size. A slot -> position map built once
   in `validate_console` makes it linear.
4. **Low.** `PARSE_TRANSIENT_MULTIPLIER` 17 -> 20 is still a real, asserted bound: every case is at
   most 17.503 bytes per input byte.
   - **Where the growth is.** It is all in `json-syntax`'s parse: the raw parse peak goes from
     6,780 to 8,944 bytes (+13 allocations for 64 input bytes), while model plus compile goes from
     524 to 568 bytes. The 64-byte empty `console` object costs about 2 KiB. That comes from the
     frontend's containers and code map, not from the model, and the parser cannot avoid it
     without changing the frontend.
   - **The cost.** Dense documents stay below 13.0, so the slack for them grew from 30% to 54%, and
     a 1 MiB document's pre-parse projection grows from 17 to 20 MiB.
   - **Suggested follow-up.** An affine bound (a fixed overhead plus k x bytes) would restore
     discrimination.
5. **Low.** The #970 pin `[1,1]` -> `[0,0]` is justified, and no guard was lost.
   - **The #966 mutation** (966-M1: the base binder, which does not check levels) turns 9 of 9
     `bank_levels` tests red at the base and 8 of 9 here. The reduced reproducer no longer
     discriminates it, but seven deterministic reproducers still do, including the mono-collapse
     one, and so does the probe.
   - **The #970 mutation** (966-M970: arming without `gathers_track_input`) is red here in the
     probe (8 armed lines over 6 seeds, against 5 over 4 at the base) and in 7 of 9
     `collapse_arming` tests.
   - **Stale mutation records.** `graph-compiler/tests/MUTATIONS.md`'s #966 rows (M1 "9 of 9" and
     the probe's seed and line counts) and `protocol/tests/MUTATIONS.md` P3-M45 (`track.simd1`)
     are now stale. Refresh them. A replacement reproducer is optional: the original shape cannot
     be expressed, and the closest one (divergent inserts plus a uniform `post_insert` later chain)
     is already `console_less_eqs`.
6. **Info.**
   - A retired track field 6 or 8 sent with the optional flag is skipped by the generic
     unknown-field rule and not refused; #1094 gate 2 owns that.
   - Inside the unpushed C3 batch, `host-core` and `capi` catalogs report lowered console slots as
     rack `5` while the browser record still addresses `0`/`1`/`2`; #1096 owns that.
   - The migration script lives under `docs/handoffs/` rather than as a PR attachment. That is
     acceptable because batch mode opens no per-slice PR, the script sits outside
     `check-script-reachability.py`'s `scripts/` scope, and the B0 rebase and #1097 need it.
   - `docs/IMPLEMENTATION_PLAN.md:31` still spells the old chain.
