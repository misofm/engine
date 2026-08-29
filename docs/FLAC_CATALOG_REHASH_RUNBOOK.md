# One-time FLAC catalog re-hash runbook

This is the operations handoff for the pre-launch container-hash to canonical-PCM migration. The
repository ships the closed migration tool, mapping format, and synthetic end-to-end oracle. The
real catalog run is an owner-scheduled operations step under the
[#245 ruling](https://github.com/misofm/engine-v2/issues/245#issuecomment-5458432753); repository
qualification does not require or authorize access to application or production storage.

## Owner action flags

| Gate | Required owner action | Current repository status |
| --- | --- | --- |
| `catalog_storage_access` | Provide an authorized read-only snapshot of every catalog WAVE master and the current container identities. | **OWNER REQUIRED — not provided; real run not executed.** |
| `maintenance_window` | Schedule one pre-launch write freeze and cutover window covering mapping generation, #246 document regeneration, verification, and publication. | **OWNER REQUIRED — not scheduled; cutover not executed.** |

Do not start the real run until both flags are explicitly cleared by the catalog owner. Do not run
the tool against a changing live catalog. Retain a recoverable snapshot of the old manifests and
documents before the one-way cutover; there is no dual-identity compatibility window.

## Prepare the isolated inputs

In an owner-approved staging directory, materialize:

- `catalog.tsv`: `name`, old container `sha256:` identity, and a relative path to each signed
  PCM16 or packed PCM24 WAVE master;
- every referenced WAVE master from the same immutable catalog snapshot; and
- `embeddings.tsv`: every old identity occurrence classified as `manifest_row`, `mix_document`,
  `app_fixture`, `package_pin`, or `server_record`.

Use `fixtures/flac-delivery/v1/mini-catalog/` only as the format and behavior example. It is
synthetic qualification data, not a substitute for the owner-provided catalog snapshot.

## Generate and verify the mapping

Choose a new, empty output directory. From the repository root, run:

```sh
cargo run --locked -p miso-engine-catalog-migrate -- \
  migrate \
  --catalog /owner-approved-staging/catalog.tsv \
  --embeddings /owner-approved-staging/embeddings.tsv \
  --output-dir /owner-approved-staging/rehash-output

cargo run --locked -p miso-engine-catalog-migrate -- \
  check \
  --catalog /owner-approved-staging/catalog.tsv \
  --embeddings /owner-approved-staging/embeddings.tsv \
  --expected-dir /owner-approved-staging/rehash-output
```

The tool refuses a master whose bytes do not match its old container identity, an incomplete
embedding-kind inventory, duplicate or malformed rows, non-launch bit depths, and any reproduced
output mismatch. Preserve the three generated files together:

- `identity-mapping.tsv` — the one-way old-to-new authority;
- `manifest.tsv` — canonical identity and decoded shape; and
- `document-replacements.tsv` — every classified replacement target.

Before cutover, the owner must reconcile input and output row counts, review every replacement
target, and archive the input snapshot plus all three outputs under the maintenance change record.

## Cut over in the maintenance window

With catalog writes frozen, use the verified mapping as #246's sole regeneration input. Apply
manifest, mix-document, app-fixture, package-pin, and server-record replacements in the same
window; publish regenerated documents and delivery metadata only after their referenced FLAC
objects decode to the declared canonical identities. Abort publication and restore the retained
pre-cutover snapshot on any mismatch. Remove the write freeze only after no old identity remains
in the enumerated stores and the new documents pass ingest verification.

Record the owner, snapshot identity, start/end time, mapping artifact location, verification
result, and publication decision in the operations change record. Those production details and
artifacts must not be committed to this repository.
