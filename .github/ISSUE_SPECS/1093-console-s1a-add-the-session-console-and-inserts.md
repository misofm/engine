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
