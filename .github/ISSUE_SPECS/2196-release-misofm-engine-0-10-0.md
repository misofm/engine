# Release @misofm/engine 0.10.0

Filed by root for the control-frame and document limits on 2026-10-28. It ships main a1749dc1b
through the manual qualify-then-publish workflow.

## Product outcome

`@misofm/engine` 0.10.0 on npm. It adds to 0.9.0 (#2187):
- **#2190:** a transaction frame larger than the core's `maximumFrameBytes` is not sent. It rejects
  with `MisoControlRefusal`, `status` and `code` `limit_exceeded`, with the detail
  `frame_bytes=<n>,maximum_frame_bytes=<m>`, on both hosts. A raw `miso.error.v1` record that the
  browser host rejects with becomes `MisoEngineError`.
- **#2191:** the control frame and the document limit are boot words (`maximumControlFrameBytes`,
  `maximumDocumentBytes`), charged to the memory budget. The default frame is 128 KiB (was
  16 KiB). The export `MAXIMUM_DOCUMENT_BYTES` is removed, which is why this is a minor release.
  A document over the budget's parse projection is `refusedBudget` (was `refusedDocument`).
- Qualified on desktop Chromium and WebKit, each on both legs (`single`, `worker`).

## Smallest slice (docs/RELEASE.md, "The release PR")

Follow docs/RELEASE.md exactly, as #2187 did for 0.9.0 (git history: commits 3bf0793e5,
0468f477d, 9ed5f861f).
- Re-pin the AudioWorklet module (#2190 and #2191 changed it). Use
  `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256` and `EXPECTED_WORKLET_SHA256`.
- Check the BLAKE3 module against its pin and `EXPECTED_BLAKE3_SHA256`; re-pin only if its bytes
  changed.
- Record a fresh two-browser qualification with both legs: `--browser all --leg all
  --record-matrix --candidate-commit <sha> --self-test-mutations` (four rows). Firefox is out of
  scope (owner, 2026-10-25).
- Set the 0.10.0 release identity in npm-publish.yml, test-npm-publish-modes.py, sdk/package.json
  and its lock, sdk/README.md (install line, release-record link, "Upgrading from 0.9.0") and
  docs/RELEASE.md.
- Retire the specs of closed issues at this batch boundary (AGENTS.md, issue-first execution).
- Run the identity report. This PR changes no Rust source.

## Release window

Between this PR's merge and the npm publish, root merges no engine PR that can change the browser
module (see the memory rule "release window: hold module merges").

## Gates

- Every release-PR gate named in docs/RELEASE.md.
- The qualification record and the identity report: every check passes.
- After the merge: npm-publish.yml qualify (expected_sha = main head), then publish
  (qualification_run_id), then verify mode. `npm view @misofm/engine version` must read 0.10.0.
