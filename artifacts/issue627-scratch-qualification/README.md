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

## Attempt 2 evidence disposition (no execution)

Attempt 1 remains **FAIL**. This revision does not relabel it, rerun any
qualification, or claim repository promotion. The complete disposition is in
`attempt-2-disposition.json`.

The numbered `00` through `10` records are one internally coherent sequence:
each records `/home/bl/misofm/engine-issue627-scratch`, frozen head
`dc14ca856e10cb5ad7f6fdacb0ea9322342a9251`, and status zero. Their recorded
time window is `2026-09-08T13:18:23.463944+00:00` through
`2026-09-08T13:21:54.508183+00:00`; the complete per-command timestamps and
arguments remain in `commands.json`. The numbered build owns
`/tmp/issue627-qualified-output`, whose six hashes are retained in
`six-file.sha256`; `identity.json` and `final-proof.json` retain the complete
qualification and frozen-row/resource/lineage proof.

The separate build-only records (`build.command.txt`, `build.status`,
`build-output-files.txt`, `build-output.sha256`, and their retained streams and
input/status records) identify checkout
`/home/bl/misofm/engine-issue627-scratch-qualification` and output
`/tmp/issue627-scratch-output-050242`. They report the same six hashes and no
downstream gate records, so they receive no qualification credit under the
exactly-one-builder rule. The duplicate record has no durable timestamp; the
evidence therefore cannot prove its relative launch order or overlap with the
numbered sequence. It establishes only the recorded command, paths, status, and
hash identity.

The runner preflight is documented separately: it saw the runner's own newly
created precondition files in scratch status and stopped before invoking a
builder or gate; those generated files were removed before the numbered
sequence. No numbered command record is attributed to that preflight, and it
receives no credit.

## Attempt 2 documentation-only disposition

`attempt-2-disposition.json` is the authored disposition for the exact feature
head `123fcc803fa827a2b22b42dbd4095029aee1f9e5`. It mechanically links every
numbered `00`–`10` command to frozen `dc14ca85`, the first scratch checkout,
and its first output path, and proves that none names the later duplicate
checkout or output. `numbered-path-proof.txt` records that check.

The retained normalized comparison is explicit: `normalized-all.diff` has
status 1 solely because the candidate Wasm digest is
`63ef81c105d50aed41164aa3c6c6f8853a314b99d642e209e7cc3aefe3bdbca1` instead
of delivered `ac71c64033b0cfc637cf14edcacaa6ed1b3bbf7093a5caa641ef84adea7e88e3`;
`normalized-nonwasm.diff` has status 0 and zero bytes. The duplicate build-only
record and its identical six hashes remain explicitly non-credit. No retained
timestamp or process record independently proves process count or exact
duplicate chronology. Attempt 1 remains FAIL; this disposition does not claim
candidate PASS or promotion authorization.
