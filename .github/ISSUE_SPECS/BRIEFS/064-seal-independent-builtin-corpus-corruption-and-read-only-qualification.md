# Sol implementation brief — issue 064 seal independent builtin corpus corruption and read-only qualification

## Decision and budget

**BLOCKED only on exact-title PASS of Issues 061, 063 and 065.** Once all three are clean and
pushed, this issue is READY for one Terra attempt and one bounded Sol correction/review; a second
failure stops. It may not change accepted expected values. Workload and timed benchmark counts
remain zero.

## Frozen join and corruption matrix

Bind the exact accepted dependency commits and verify their joined corpus before writing any test.
Keep exactly 50 manifest-listed payloads: cases, response, metadata, diagnostics, resources, two
meter files, 33 PCM files and ten benchmark input bundles.

Execute exactly these four mutations for each of six classes—TOML, `f32le`, CSV, meter JSONL,
diagnostics JSONL and resources JSONL:

1. delete the selected required payload;
2. alter one byte without updating the manifest;
3. add one unlisted file; and
4. remove one required semantic tuple/path while keeping the payload syntactically canonical and
   recomputing its manifest length/hash.

The fourth case must reach semantic coverage validation. Empty-file, malformed-syntax or stale-
manifest failures do not count. Record the exact class, mutation and stable rejection identity for
all 24 rows.

## Frozen read-only seal

Keep authoring under explicit scratch `--write`. Prove structurally and with unit/static checks
that the `--check` dispatch reaches only regular-file reads, parsers and validators—not `generated`,
production DSP/graph rendering, directory creation or writes. Hash path, length and bytes for the
complete tree before and after a successful checked-in-corpus validation and require equality.

Run focused fixture/reference tests, checked-corpus validation, format, warning-denied package
Clippy and applicable nonbenchmark workspace/policy/diff/static scans on one clean commit. Record
the final immutable corpus and manifest hashes. Stop for any expected-byte regeneration/change,
production/audit/target/benchmark work or a second failure. PASS alone unblocks **Builtin direct
and graph realtime audit and target qualification**.
