# 209 Repair SDK ABI artifact ordering gate and reseal Phase 0

## Mission

Close the sole gate defect exposed by issue #207 Phase 0 after its third and final attempt: the web
artifact checker compares a lexically sorted directory listing with an expected six-file list whose
new ABI-layout entry is out of lexical order. Preserve all accepted Phase 0 product work and rerun
the complete Phase 0 seal.

## Dependency and authority

Successor to #207 Phase 0 commits `6c8a967`, `9888bfe`, `7bd2930`, `56ea90f`, and `387b789`. The
terminal serialized sweep produced 91 PASS rows and one FAIL: `scripts/check-web-audioworklet.sh`
rejected the correct six-file directory only because `miso-engine-v2-abi-layout.json` was listed
after the audio-worklet files while `actual` was sorted.

GitHub issue #209 is the matching remote authority.

## Scope

- Put the six expected web artifact names in the exact same lexical order as the sorted actual
  listing.
- Add a bounded red probe proving an out-of-order expected list is rejected and the canonical list
  is accepted.
- Do not change emitted artifacts, metadata schemas, provenance contents, Rust ABI, frozen
  `hosts/miso-engine-host-web/web/*` bytes, DSP, or SDK API.
- Update issue #207 local Phase 0 evidence after the fresh seal.

## Deliverables

1. One exact-path gate correction.
2. A named ordering red mutation in the existing hermetic web-artifact test path.
3. Fresh real `scripts/build-sdk.sh` output with provenance verification.
4. Fresh `scripts/sweep.sh` all rows, `cargo fmt --check`, and workspace clippy with `-D warnings`.
5. Independent Sol adversarial verdict before #207 Phase 1 resumes.

## Objective gates

- The real six-artifact directory is accepted.
- Swapping the ABI-layout row out of lexical order makes the ordering probe red.
- The SDK build emits exactly seven files and provenance recomputes every one of its six sibling
  asset records.
- Sweep reports 92/92 PASS.
- Workspace fmt and clippy report PASS.

## Non-goals

No Phase 1 TypeScript code, artifact rebuild policy change, new schema, benchmark, push, merge, PR,
or issue #207 closure.

## Attempt budget

One Terra implementation attempt and one bounded Sol correction. If either cannot close the exact
gate, stop without weakening the acceptance bar.

## Evidence

- Attempt 1 `0fedc9e`: put the six-file production list into lexical order and added the first
  ordering self-test. Sol HOLD: the positive arm compared the canonical list with itself, so the
  exact former ordering could return without failing the probe.
- Allowed bounded Sol correction `c77a0d0`: compare against an independently generated
  `LC_ALL=C sort` oracle and use the exact pre-fix ordering as the red input. Terminal independent
  Sol verdict: PASS.
- Focused `scripts/test-web-audioworklet.sh`: PASS, including artifact-order, opcode, call-graph,
  metadata, ABI-layout, reason-vocabulary, and UTF-8 mutations.
- Fresh real `scripts/build-sdk.sh`: PASS with exactly seven outputs. Wasm is `2,494,615` bytes at
  SHA-256 `99c08301577dc27799bee3c13fe74dfee87db36b0b54864d97c92935666368d6`;
  provenance is `1,264` bytes at
  SHA-256 `37a01e6a54e9d0bd806682e80bc9c4fccacd1b99c717a4ce2a99e5d9009d63ef`;
  sibling byte/hash recomputation PASS.
- Full serialized bar: `scripts/sweep.sh` `92/92` PASS in `121s`; workspace fmt PASS; workspace
  clippy with `-D warnings` PASS.
- No push was made by owner instruction. This local evidence is not yet upstream, so GitHub #209
  intentionally remains open and no remote completion claim is made.
