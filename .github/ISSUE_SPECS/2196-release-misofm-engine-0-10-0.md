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

## Evidence and decision record (attempt 1, 2026-10-28)

- Worklet module: `11f93b33672b04b2009fdb15d19e455b673a29a8eaaee189c0b8c462e6b47b57`
  (6,162,728 B; named twin `902a5816...`, 6,973,579 B), built from this tree (main a1749dc1b's
  module inputs) with `nightly-2026-08-20`. It replaces 0.9.0's `b32a5429...`. Main a1749dc1b's
  own CI run recorded the same digest (commit status `audioworklet-sha256`), so this PR moves no
  byte against main. The pin file, `EXPECTED_WORKLET_SHA256`, and test-npm-publish-modes.py's
  accepted `release_pin` and its "wrong release pin" mutation carry it. `--check-pin` passes. A
  second build from a `git archive` export of the tree at another path and target directory gives
  identical bytes for all eight files of the closure.
- BLAKE3 module: `ee373d2f549b0d33bdd0324a8833698158274f516e01a7b0c4018347991a18b1` (44,861 B),
  unchanged. It equals its pin and `EXPECTED_BLAKE3_SHA256`; neither changes.
- Qualification (`--browser all --leg all --record-matrix --candidate-commit 56be629ec
  --self-test-mutations`, Playwright 1.62.1, null PulseAudio sink on a 102 B socket path):

  | browser | version | leg | result | time |
  | --- | --- | --- | --- | --- |
  | Chromium | 151.0.7922.34 | single | pass | 15.7 s |
  | Chromium | 151.0.7922.34 | worker | pass | 17.1 s |
  | WebKit | 26.5 | single | pass | 40.0 s |
  | WebKit | 26.5 | worker | pass | 33.6 s |

  Every render-allocation count is 0 on every leg, including #2191's `console-limits` row (a
  1,075,815 B document and an 85,024 B frame under the 131,072 B default frame, committed).
  `blockingRebuilds` is 1 for the structural edit on the single legs and 0 on the worker legs.
  `results.json` (four leg rows, `blake3Qualification` at the same candidate) and
  `BROWSER_DEPLOYMENT_MATRIX.md` record it. The candidate's tree differs from this PR's head only
  in the qualification records and this spec; no module input.
- Release identity: npm-publish.yml's five `0.10.0` literals and the pin; `EXPECTED_BLAKE3_SHA256`
  unchanged; `BROWSER_ARTIFACT_TOOLCHAIN` stays `nightly-2026-08-20`. test-npm-publish-modes.py
  follows (its "wrong release version" mutation moves 0.10.0 to 0.9.0) and passes. sdk/package.json
  and its lock read 0.10.0. docs/RELEASE.md names 0.10.0 (#2196) as the last release.
- sdk/README.md: install line and release-record link at 0.10.0. "Upgrading from 0.9.0" (#2191)
  already named the removed `MAXIMUM_DOCUMENT_BYTES`, the boot options `maximumControlFrameBytes`
  and `maximumDocumentBytes` (the boot words that replace `reserved1` and `reserved3`), the 128 KiB
  default frame, `host.controlFrameBytes`, and `refusedBudget` (was `refusedDocument`). This PR
  adds the #2190 bullet: `limit_exceeded` with its detail, and the raw `miso.error.v1` record now a
  `MisoEngineError`. The two paragraphs that still said the frame limit is "16,384 bytes today" and
  that #2191 would make it a boot setting now give the 128 KiB default and `maximumControlFrameBytes`.
- Identity report (`--event push --before a1749dc1b`): `ARTIFACT UNCHANGED` against the base's
  recorded digest, reproducible, the RELEASE line with every check passing (pin,
  `EXPECTED_WORKLET_SHA256`, toolchain, `results.json` at candidate 56be629ec), and the BLAKE3
  RELEASE line.
- Spec retirement: the specs of closed issues #1380, #1387, #2170, #2187, #2190 and #2191 leave
  `.github/ISSUE_SPECS/` at this batch (each `gh issue view` reads CLOSED).
- Local gates, all 62 steps pass, the same set as #2187: the route self-tests; docs-gates; the
  `artifact` build (with named twin) and producers check for both modules; the `sdk` job with an
  empty private TMPDIR and the source-only asset tests; `artifact-gates` (named-twin strip check,
  check-web-audioworklet.sh, check-browser-expected-resources, scalar-oracle-absent,
  test-web-audioworklet.sh, render-lock probe, V8 spill); the identity self-test and report;
  release-shape and its unwind workspace check; the sdk-deletions self-test; fmt, clippy, doc and
  every `lint` step except the three x86 lane-guard probes (Rust only; no `.rs` or Cargo file
  differs from a1749dc1b).
