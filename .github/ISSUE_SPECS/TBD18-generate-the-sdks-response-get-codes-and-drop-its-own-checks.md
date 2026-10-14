# Generate the SDK's RESPONSE_GET codes and drop its own checks

Successor of #1574 (batch 9 review), decision 17. Code anchors verified on the batch 9 branch `codex/controls-batch-9`. Issue number: assigned at filing (written here as TBD18). Feature sprint mode (owner, 2026-10-13): one implementer, normal tests, a batch review.

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
