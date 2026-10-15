# Generate the SDK's RESPONSE_GET codes and drop its own checks

Successor of #1574 (batch 9 review), decision 17. Code anchors verified on the batch 9 branch `codex/controls-batch-9`. Issue number: assigned at filing (written here as #1623). Feature sprint mode (owner, 2026-10-13): one implementer, normal tests, a batch review.

## Product outcome

The browser SDK's response preview is a thin client: its frame field IDs and enum codes come from
the engine's protocol schema, and every refusal (grid, quality, addressing) is the core's, with the
same code the C ABI gives.

## Context

- `sdk/src/core/response-frame.ts` copies the `RESPONSE_GET` field IDs and the grid, lane, quality
  and link codes by hand.
- `sdk/src/core/response.ts` refuses a logarithmic grid with `minimumHz <= 0` (`:311`), a
  non-finite endpoint (`:310`), any quality but normal (`:193`), and checks shared or per-lane
  addressing; a browser caller gets `MisoUsageError` where a C ABI caller gets the core's code.
- #1569 generated the session key tables from the engine's walk; the same generator path
  (`tools/parameter-metadata`, `sdk/codegen`) can emit the response message table from
  `crates/protocol/src/schema.rs`.

## Decisions frozen for this slice

- **D1.** A generated SDK module holds the `RESPONSE_GET` request and reply field IDs and codes,
  from `protocol::schema`; the hand copies are deleted.
- **D2.** The SDK's grid, quality and addressing checks are deleted; it encodes only what it
  cannot encode as a usage error (a value with no wire spelling). The core refuses the rest.

## Objective gates

1. An SDK eval sends a logarithmic grid with `minimumHz = 0`, a non-normal quality and a bad lane,
   and gets the core's codes, equal to the C ABI's for the same frames.
2. `check-sdk-generated.sh` fails if the generated table drifts from the schema.
3. SDK headless, types and package checks; `cargo test -p parameter-metadata`.

## Authorized paths

- `sdk/src/core/{response.ts,response-frame.ts}`, `sdk/codegen/*`, `sdk/src/generated/*`,
  `sdk/assets/*` (generated), `sdk/test/response-evals.mjs`, `tools/parameter-metadata/**`.

## Dependencies

- #1574, #1569.

## Size

Half a day to a day.

## Decision record: #1623 (batch 14b)

Shipped. Anchors held except as noted (`response.ts` had moved: the checks were in `#query` and the
`gridName`/`qualityName`/`channelName`/`asF32` helpers).

**D1.** The protocol crate gained `protocol::response_frame_table()` (`crates/protocol/src/frame_table.rs`).
It reads the field IDs, wire types and mandatory flags from the `schema.rs` specs (a name is paired
with its spec constant and the function refuses a message whose names do not cover its spec in
order) and reads each code table (configuration kind, grid, lane, quality, link mode) from the
strict reader that refuses codes outside it. `parameter-metadata --print-response-frame` writes it
as `sdk/assets/miso-engine-v1-response-frame.json`, a source-only SDK asset like the session keys;
`sdk/codegen/generate.mjs` emits `sdk/src/generated/response-frame.ts`. `response-frame.ts` takes
every ID, wire type, mandatory flag and code from it; its field helpers take a table row of one
literal wire type, so a schema change of a field's type is a `tsc` error. The hand copies
(`GRID_CODES`, `LANE_CODES`, `QUALITY_CODES`, `LINK_CODES`, `KIND_*`, every field number and
`MESSAGE_RESPONSE_GET`) are deleted.

**D2.** Deleted from the SDK: the grid-kind, finite-endpoint, logarithmic-minimum, point-count,
rate and quantum, quality, channel-name-default-as-check, link-mode-fallback, request-ID-zero,
shared-lane and parameter-domain checks of the response path, the unknown-effect and
no-response-capability throws, the `1 << 20` request-frame bound and the `16 << 20` and `>= 1`
bounds on `maximumResultBytes`/`maximumAutomationSpansPerBlock`. What stays is a value with no wire
spelling, as `MisoUsageError`: a name the code tables lack, a number its field cannot carry
(`requireU32`/`requireU64`), a `fields` selector, a parameter name or label the catalog lacks, a
non-number parameter value, and the SDK's own `requestDeadlineMs` timer bound.

**Departures and mechanical edits outside the authorized paths.**
- `crates/protocol`: `Wire::name`, `ResponseConfigurationKind` (a `control_code_table!` replacing the
  literal `1`/`2` of the configuration kind in the encoder and decoder; same wire, same refusal),
  `frame_table.rs`, two re-exports. Needed because `schema.rs` is crate-private.
- `tools/parameter-metadata/Cargo.toml`: `protocol` moved from dev-dependencies to dependencies
  (already in the lockfile through `host-web`).
- `sdk/src/core/session.ts`: `parameterWireRows`, the encode-only half of `parameterRows`
  (`parameterRows` itself is unchanged in behavior); the response path no longer calls the
  authoring checks.
- A response limit the caller leaves out is now a field left out of the frame, so the core's own
  default applies (the SDK defaults `1 << 20`, `1 << 30`, `1 << 30`, `4096`, `16 << 20` equalled the
  core's and are gone). The frame of a default query is therefore shorter; the replies are equal.
- Rust rows added so the codes are held on both sides: `crates/capi/tests/response_get.rs`
  (draft and high quality) and `crates/host-core/tests/response_frame.rs`
  (`the_core_refuses_what_a_thin_client_sends_as_given`, `a_non_finite_endpoint_sent_raw_is_refused_by_the_decoder`).
- `effect()` still refuses draft/high quality, a shared parameter on one lane and an out-of-domain
  value at declaration: that is session authoring (`session.ts`), not the response path, and a
  response preview of a hand-built configuration now reaches the core.

**Gates.**
1. `sdk/test/response-evals.mjs`: "a grid, a rate, a quality and a value the SDK used to refuse are
   the core's refusals" (14 rows: logarithmic axis from zero `response.grid`, NaN and infinite
   endpoints `protocol.failure`, draft and high quality `effect.quality.unsupported`, ...),
   "a lane given twice is refused by the core with the C ABI's code" (`response.override.duplicate`,
   raw frames), "a name or a number with no wire spelling stays a usage error". The codes equal the
   rows of `capi` `every_reachable_refusal_is_typed_and_changes_nothing` and host-core
   `the_core_refuses_what_a_thin_client_sends_as_given`. The session-less web entry and a C ABI
   session cannot take one literal frame (the session endpoint refuses a rate and a quantum in the
   request), so equality is by request content and the shared `ResponseQueryError::refusal` table.
2. `bash scripts/check-sdk-generated.sh` fails on a hand edit of the asset or of the generated
   module (both tried red); `parameter-metadata` `response_frame_asset_is_current` holds the asset
   to the generator. Protocol test `frame_table::tests::every_code_is_the_one_the_encoder_writes`
   holds each name to the code its encoder writes.
3. `check-sdk-headless.sh` (563 tests pass), `check-sdk-types.sh`, `sdk-package.sh check`,
   `check-sdk-deletions.py` (and `--self-test`), `check-workspace-policy.sh`,
   `cargo test --locked -p parameter-metadata -p protocol`, host-core `response_frame`, capi
   `response_get`; clippy `-D warnings` and fmt clean on protocol, parameter-metadata, host-core,
   capi.

**Successor found.** `control-frame.ts` still holds the `WIRE_*` type codes, `FLAG_REVISION_ANY`,
the header magic and the status names by hand; the same frame table could carry them.
