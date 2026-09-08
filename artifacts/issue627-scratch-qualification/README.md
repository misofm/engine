# Issue 627 scratch qualification

The first authorized scratch sequence ran from detached source
`dc14ca856e10cb5ad7f6fdacb0ea9322342a9251` in
`/home/bl/misofm/engine-issue627-scratch`. Its builder emitted exactly six files,
the Wasm independently hashes to
`63ef81c105d50aed41164aa3c6c6f8853a314b99d642e209e7cc3aefe3bdbca1`,
and all five non-Wasm hashes match the delivered #623 set. Matrix generation,
static checks, the separate expected-resource/native-witness gate and mutations,
hermetic checks, locked SDK and browser installs, SDK checks, one Chromium/
Firefox/WebKit qualification, and the final matrix check all exited zero.

Before that sequence, its local runner preflight detected the runner's own
newly created precondition records in the scratch status and aborted. Those
generated precondition files were removed before execution. The preflight did
not invoke the artifact builder or any qualification gate, so the numbered
sequence remains each material command's first invocation.

A concurrency race also launched a later builder-only invocation from
`/home/bl/misofm/engine-issue627-scratch-qualification` while the first sequence
was already running. It emitted the same six hashes. Root stopped that executor
before it ran any matrix, static, resource, hermetic, SDK, install, or browser
gate. This exceeds the brief's exactly-one scratch-builder rule even though it
was concurrent rather than a retry made after observing a result. The duplicate
record is retained as `build.*`, `build-output*`, `input-sha256-before-build.txt`,
and the adjacent source/overlay identity files. The first sequence is retained
as numbered `00` through `10` records and `commands.json`.

The scratch overlay for the completed qualification changes exactly the pin,
`candidateCommit`/`wasmSha256` lineage fields, and the generated matrix lineage.
Browser rows, versions, gates, and resource values are unchanged. No repository
promotion is authorized. Astra LOW must rule on the candidate evidence and the
procedural breach before further execution.

The two full overlay diffs contain generated Markdown blank-table lines with
trailing spaces. They are losslessly retained as deterministic gzip files so
branch-wide diff hygiene does not reinterpret raw evidence bytes as authored
patch errors. Reproduce either reviewed byte stream with `gzip -cd FILE.gz`.

- `second-overlay.diff`: 2596 bytes, SHA-256 `db8485f00051ba90bc1c3ac5e625565454a6140c9278d1e5f9a20a81e2445241`
- `final-overlay.diff`: 2596 bytes, SHA-256 `db8485f00051ba90bc1c3ac5e625565454a6140c9278d1e5f9a20a81e2445241`
