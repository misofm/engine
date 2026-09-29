# Add the session console and per-track inserts to the session schema

Slice S1a of *Console strip: session-level console effects with per-track inserts* (owner
decision 12, `docs/rulings/engine-footprint-2026-09-29.md`; Sol's H4, M5, M9, L6 and amendments 2,
4, 5 and 14 in `.github/ISSUE_SPECS/DRAFT-console-strip-VERIFY.md`, commit `03aceb94`).

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

Migrate mechanically. A rack becomes a console section when every track carries an identical
declaration sequence in it: same identity, quality and link mode, no sidechain, and every effect
eligible. The slot is the effect's ID, and each track's entry takes its own params and bypass. So
`simd1` becomes `pre_insert` and `simd2` becomes `post_insert`. Any other rack folds into `inserts`
in chain order: `simd1` (if not console), then `dynamic`, then `simd2` (if not console). A folded ID
collision is refused, not silently renamed.

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
- `dynamic` becomes `inserts` in every document, so compressor-dynamic and observation-frame-shape
  keep their effects where they are.
- Sol's M9 listed the 9-track ragged strip among the folds. That workload truncates the intended
  fixture, whose racks are uniform, so it becomes console.

**Digest chain to repin (M9)**, with a one-line reason for each:
- `fixtures/session/v1/canonical.json` SHA-256, then
  `fixtures/builtins/v1/benchmark/prepare_256_tracks-{48000,96000}.toml`, then
  `fixtures/builtins/v1/MANIFEST.tsv`, then `ACCEPTED_MANIFEST_SHA256`
  (`tools/audit/src/builtins_graph.rs:48`);
- `fixtures/graph/MANIFEST.tsv`;
- `fixtures/session-canonical/v1/canonical-writer-corpus.json`;
- the console fixtures, which `scripts/check-console-fixtures.sh` compares with `cmp`;
- the browser `sessionDocumentBytes` row (`scripts/check-browser-expected-resources.py:80-87`).

**Docs that travel with this slice:** `docs/SESSION_SCHEMA_V1.md` (shape, tokens, retired codes on
the #1063 precedent, lowering) and `docs/session-v1.schema.json`.

Authorized paths: `crates/session/**`, `crates/protocol/src/{schema,session_wire,message_wire}.rs`
(model encoding only), the lowering call sites that read `simd1`/`dynamic`/`simd2`, the documents
and digest pins above, the two derive scripts, the two docs above, and this spec.

## Owner decisions that bind this slice

All of decision 12's "Shape", "Chain", "Sidechain", "Eligibility", "Wire identity" and "Class A by
lowering". There is no `ABI_VERSION` bump.

## Dependencies

None beyond decision 12. Merge after *Add the console-strip benchmark rows* (B0).

## Objective gates

1. Strict parse and canonical round trip of the new shape, with empty sections and empty `inserts`
   included. BTLV round trip through the visitor.
2. Each refusal listed above has a test with its code and JSON path. Retired tokens, keys and wire
   codes 1 and 3 are refused, never reinterpreted.
3. `RackName` and `ParameterRack` wire codes are explicit and round-trip (`builtins` stays 4). A
   planted index-derived code turns
   `parameter_enum_wire_mappings_are_exhaustive_and_roundtrip` red.
4. PR evidence, not a committed test: every migrated console fixture produces identical canonical
   graph text and identical render digests against the pre-change base. List every repinned digest
   with its reason.
5. `bash scripts/check-session-policy.sh`, `bash scripts/check-console-fixtures.sh`, the protocol
   corpus and `cargo test --workspace` pass.

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
