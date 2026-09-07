# Evidence packaging exclusions

This package preserves the readable payloads captured under `/tmp/issue555-postpin-success`.
The captured environment files are ordinary newline-delimited text; no NUL-byte environment stream was emitted, so no NUL stream hash is included.

`qualified-entry-manifest.tsv` is intentionally excluded. It was a redundant intermediate generated during the final audit; `qualified-manifest.tsv` and `qualified-sha256sums.txt` are the authoritative comparison records, and `postpin-entry-manifest.tsv` records the retained artifact entries.

No repository, issue spec, source, pin, Git history, or GitHub state was changed by this packaging step.
