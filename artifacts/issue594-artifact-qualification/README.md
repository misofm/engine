# Issue #594 AudioWorklet artifact identity probe

Lane B probed the exact integrated #594 source head
`cb4e4bed4f84ca5c76a0dd02384c235f2836ff24`. The current shipped pin was
`39ebe7cd3f71f34ab11260f27fa1eaad281dd61642c50d9ed6210e703d95dd55`.

The independent repin-mode builder returned candidate digest
`39ebe7cd3f71f34ab11260f27fa1eaad281dd61642c50d9ed6210e703d95dd55` with status 0. The ordinary
six-file builder also passed with status 0. Its six-file inventory and hashes are recorded in
`six-file-list.txt` and `six-file-sha256.txt`; every member matches the canonical #587 qualified
artifact identity. Therefore the #594 source change is artifact-byte-identical to the current
qualified delivery. No pin, qualification results, deployment matrix, source, or generated
consumer changed.

The repin and ordinary-build stdout/stderr streams are retained both plainly and as deterministic
gzip copies. `command.txt`, `context.txt`, and `status.txt` record the invocations, exact source
and pin identities, clean source diff state, and exit statuses. This is an identity probe only;
the full lane-B qualification and pinning path was not rerun because all six shipped bytes are
unchanged.

The directory checksum manifest covers every retained payload except itself.
