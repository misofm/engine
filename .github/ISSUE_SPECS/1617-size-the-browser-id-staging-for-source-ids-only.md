# Size the browser ID staging for source IDs only

Successor of #1565B, decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`). Code
anchors verified on the batch 8 integration branch `codex/controls-batch-8`. Issue number:
assigned at filing (written here as #1617). Feature sprint mode (owner, 2026-10-13): one
implementer, normal tests, a batch review.

## Product outcome

The browser host reserves ID staging only for the IDs that still cross it. Since #1565B, only
source IDs do: track, submix, route and VCA IDs reach the SDK through the core's control list.

## Context

- `project_buffers` (`hosts/host-web/src/lib.rs`, the boot path near `:1820`) sizes the ID staging
  for the longest source, track, submix, route and VCA ID. The test mirrors in
  `hosts/host-web/src/tests.rs` (near `:85` and `:827`) repeat that sizing.
- #1565B deleted `live_control_{track,submix}_{count,id}`, the last readers of the non-source IDs.
- Shrinking the staging moves `id_staging_bytes` in the resource report and the `memoryBytes`
  rows of `hosts/host-web/tests/browser-v1/expected.json`.

## Decisions frozen for this slice

- **D1.** The ID staging is sized by the longest source ID alone. Nothing else changes in the
  boot path.
- **D2.** Re-pin `expected.json`'s `memoryBytes` rows from a measured browser run, with the before
  and after numbers and the reason in the decision record. The boot-budget multiplier does not
  rise.

## Objective gates

1. A host-web test boots a document whose longest track, submix, route and VCA IDs are longer than
   every source ID and shows `id_staging_bytes` equals the source-ID bound.
2. `cargo test --locked -p host-web --features host-web/test-support`,
   `bash scripts/check-web-audioworklet.sh` (boot budget), and the browser qualification for
   chromium (firefox and webkit where installed) with the re-pinned rows.

## Test value

- Gate 1 turns red if the staging still reserves room for an ID kind that no longer crosses it.

## Authorized paths

- `hosts/host-web/src/lib.rs` (the staging sizing only), `hosts/host-web/src/tests.rs`,
  `hosts/host-web/tests/browser-v1/expected.json` (re-pinned rows only), this spec's decision
  record.

## Dependencies

- #1565B. Lane W.

## Size

Half a day, one sonnet-high implementer.
