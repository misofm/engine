# Issue 539 evidence packaging correction

Root's post-review delivery audit found that 32 historical raw captures made
`git diff --check origin/main...HEAD` fail because their captured bytes contain
literal trailing whitespace or an extra final blank line. This correction
changes only their repository packaging: each raw file was compressed once with
`gzip -n -9`, which omits timestamps and original filenames. No captured byte,
verdict, source file, command result, attempt attribution, or product behavior
was changed.

`manifest.tsv` records the SHA-256 and byte count of both the decompressed raw
content and the deterministic gzip file. All 32 archives pass `gzip -t`; every
decompressed SHA-256 matches the pre-compression identity. Historical paths
inside raw agent transcripts remain part of those immutable transcripts. This
manifest is the current repository locator for the compressed copies.
