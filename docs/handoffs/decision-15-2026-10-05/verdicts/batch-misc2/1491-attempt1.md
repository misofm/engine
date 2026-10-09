FAIL

# #1491 attempt 1 verdict (commit 453f3038a, parent d1b17d216, branch codex/d15-batch-misc2)

Verifier: opus verifier, 2026-10-09. Read-only on the worktree. Everything was run on an export of
453f3038a at `/tmp/claude-1002/v1491/tree`. Evidence is in
`/home/bl/misofm/submix-verdicts/evidence/1491-attempt1/`.

Scope: the diff touches only the authorized paths. All test changes are inside
`testQualificationBoot`, and the diff adds three `MUTATIONS.md` rows and the attempt record. No
worklet, host, bridge or `qualification.js` change.

## Summary

The new witness works. The three mutations named in the spec are red on this suite and green on the
base suite, and gates 1 and 3 pass. The attempt fails on one MAJOR finding: the witness and the
expected value hand-type the bridge's layout. The struct offsets, struct sizes, ABI version and the
target and channel codes are the same values the worklet hand-types. The generated layout, which is
derived from the Rust structs, is already loaded in this file. Because of this, a layout change in
the real bridge that the worklet misses leaves this test green. I showed this with probes.

## MAJOR

**M1. The staging witness and D2's expected value use a hand-typed copy of the bridge's layout. They
must read the generated layout already loaded in the file.**
`scripts/test-web-audioworklet.mjs:2591,2596-2600,2609-2611` (decode offsets 8/16, stride 24,
entry offsets 4/8), `:2868-2869` (code tables), `:2874-2875` (`structBytes: 32`,
`abiVersion: 0x00010000`). The same applies to #1477's fake sizing accessors in the same function,
at `:2630,2632,2636,2640` (`32`, `24`, `+ 8`).

- Spec D1 says the witness decodes the collection "the way the bridge lays it out". The bridge's
  layout has one authority: `sdk/assets/miso-engine-v1-abi-layout.json`. `parameter-metadata` makes
  it from `offset_of!` on `WebSpectrumCollectionRequest`/`WebSpectrumCollectionEntry` and from the
  frozen `SPECTRUM_TARGET_*`/`SPECTRUM_CHANNEL_*`/`ABI_VERSION` constants. `check-abi-layout-v1.py`
  validates it, and `check-sdk-generated.sh` checks it for drift on the merge-gating `sdk` job. The
  test already reads this file as `preparedAbiLayout` (`:18`). Its `structures.spectrumCollection*`
  rows, `constants.spectrumTargets` and `constants.spectrumChannels` use names that are the option
  strings exactly (`trackPostPan`, `output`, `both`, and so on). The repo already rules against
  hand-written JavaScript copies of the ABI (`check-abi-layout-v1.py` docstring: "a document nothing
  validates is just a sixth copy"; `abi_layout.rs:16`: "never from a table anybody types twice").
- The defect: the worklet also hand-types these values (`miso-engine-v1-audio-worklet.js:1,60-70`).
  The fake and the assertion copy the same values, so a bridge change that the worklet misses also
  passes the hermetic test. Probe: I edited the generated layout the way a Rust change would
  regenerate it, in three ways: `both` = 4, `maximumCaptureBytes` at offset 24 with `reserved` at
  16, and a 28-byte entry. **The committed suite stays green** (`layout-drift.log`). Today only the
  browser legs would catch this drift.
- The fix is small, and I proved it. A prototype that builds the offsets, sizes, ABI version and
  code maps from `preparedAbiLayout` (`layout-derived-prototype.diff`, about 20 changed lines, all
  inside `testQualificationBoot`) gives these results:
  - It is green on the clean tree (`proto-clean.log`).
  - It is red on each of the three layout drifts (`proto-drift-code.log`, `proto-drift-offset.log`,
    `proto-drift-size.log`; the size drift goes red through the worklet's own `entryBytes` check
    against the fake).
  - It is still red on p7, channel `both`->`LEFT` and the halved budget (`proto-*.log`).

  The expected value still comes from the caller's witnessed options, and the layout still does not
  come from the worklet under test. The table is independent of the worklet and tied to the bridge.

- Answer to root's question: the test should not keep its own code table. Take the codes, offsets,
  sizes and ABI version from the generated layout. That source is generated, drift-checked and
  independent of the worklet. A hand-typed table is a third transcription of the same frozen values.

## MINOR

**m1. D3 has a test value, but the attempt does not record it. It catches caller drift, not worklet
behavior.** `.github/ISSUE_SPECS/1491-...md:163` says only that D3 "holds that the five callers
without a collection stage nothing". I could not find a plausible worklet defect that turns D3 red.
The worklet calls `..._collection_request_ptr` only when options carry a collection. A mutated
`hasSpectrumCollection` that also accepts `undefined` returns early from
`stageSpectrumCollectionRequest`. A header with count 0 gives `null` as the spec intends. D3's real
red path is a qualification caller that boots a collection. Probe: in `qualification.js`, add
`spectrum: null, spectrumCollection: { entries: [{ target: "output", targetId: "main-out",
channels: "both" }], maximumCaptureBytes: 1048576 }` to `bootOptions`. D3 is red on this suite at
`renderCorpusSegment` and green on the base suite (`d3-bootoptions.*.log`). No #1477 assertion reads
the other callers' `spectrumCollection`. D3 needs a test-value sentence in the attempt record and a
`MUTATIONS.md` row for this mutation.

## NIT

- **n1.** `:2889`: when D3 is red on its real path, its message is wrong. The probe printed
  "renderCorpusSegment staged a spectrum collection it does not boot", but that caller does boot
  one. Suggested text: "stages a spectrum collection; only runStagingReadRun boots one".
- **n2.** `:2579`: `TextDecoder("utf-8", { fatal: true })` removes a leading U+FEFF, but the
  bridge's `core::str::from_utf8` keeps it. For exact bridge parity, add `ignoreBOM: true`. Today's
  identities are not affected.
- **n3.** The witness does not decode the reserved words. These are the request's `reserved0` and
  `reserved[2]` and each entry's `reserved[3]`, and `staged_spectrum_request` (`ffi.rs:952` ff.)
  refuses them when they are not zero. The spec's D1 list does not name them, so this is not a spec
  miss. If the decode becomes layout-driven (M1), they are cheap to add with expected zeros.

## Test value (per new test)

- **D1/D2 (staged collection equals options):** this test is red when the worklet skips collection
  staging (p7), or stages a channel code or capture budget that differs from the options. No
  existing test catches these, because before this change the fake sized the staging and nothing
  read it. (After M1, this test is also red when the worklet's hand-typed layout drifts from the
  bridge's generated layout.)
- **D3 (other callers stage none):** this test is red when one of the five other qualification
  callers starts to boot a spectrum collection (probe: a default collection in `bootOptions`). No
  existing assertion reads those callers' `spectrumCollection`. It holds no worklet defect that I
  could construct.
- **"exactly one boot per caller"** (`:2757-2758`): this is support for D1's witness bookkeeping, not a
  claimed test. No finding.

## Gates run

1. `bash scripts/test-web-audioworklet.sh` with qualification.yml's wrapper (`mktemp -d`,
   `export TMPDIR`, `set -o pipefail`, leftover `find`): rc 0, leftover empty. The boot contract
   line `qualification boot contract passed: callers=6 real-ready=6 real-disposed=6
   diagnose-ready=1` appears twice (`gate1.log`).
2. I applied each mutation alone to the exported worklet. I ran each against this suite and against
   the parent's suite (`d1b17d216:scripts/test-web-audioworklet.mjs`, placed beside it), then
   reverted it (`cmp` clean) and reran the suite green (`after-revert.log`):

   | mutation | this suite | base suite |
   | --- | --- | --- |
   | p7 (collection branch -> `stageSpectrumRequest`) | red: D2, actual `null` | green |
   | collection `both` -> `SPECTRUM_CHANNEL_LEFT` | red: D2, `channels: 1` where 3 | green |
   | `BigInt(options.maximumCaptureBytes / 2)` | red: D2, `1048576n` where `2097152n` | green |

3. `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.

Extra probes: the three generated-layout drifts (M1), the layout-derived prototype (M1) and the D3
caller probe (m1). No Rust changed and the test does not need the built module, so I did not build
the module.
