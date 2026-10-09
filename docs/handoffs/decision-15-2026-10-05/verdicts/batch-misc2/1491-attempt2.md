PASS

# #1491 attempt 2 verdict (commit bb84a30fc, parent fb303f836, branch codex/d15-batch-misc2)

Verifier: opus verifier, 2026-10-09. Read-only on the worktree. I ran everything on an export of
bb84a30fc at `/tmp/claude-1002/v1491/tree`, and I confirmed with `cmp` that the exported test,
worklet, layout and `qualification.js` are the same as the commit's blobs. Evidence is in
`/home/bl/misofm/submix-verdicts/evidence/1491-attempt2/` (`mutation-table.txt`, one log for each
run, the scripts `mut2.sh`, `mut2b.sh` and `mut2c.sh`).

Scope: #1491's two commits (453f3038a, bb84a30fc) touch only the authorized paths: the spec,
`hosts/host-web/MUTATIONS.md` (eight rows) and `scripts/test-web-audioworklet.mjs`. Every hunk of
the test file is inside `testQualificationBoot` (`:2500-2966`). No Rust, worklet, host, bridge or
`qualification.js` change. The other commits in `453f3038a^..bb84a30fc` belong to #1488, #1489,
#1490 and #1492, and they do not touch the test file. Between d1b17d216 and bb84a30fc no JavaScript
that this suite loads changed, other than the suite itself. Thus the pre-#1491 suite is a valid
base for the comparison runs.

## Attempt 1 findings

- **M1 (hand-typed layout): fixed.** The decode, the fake's collection sizing from #1477 and D2's
  expected value now get every offset, field type, struct size, entry stride, the ABI version and
  the target and channel codes from `preparedAbiLayout`. This is the generated
  `sdk/assets/miso-engine-v1-abi-layout.json` (`:18`, `:2586-2605`, `:2648-2661`, `:2889-2917`). I
  looked for numeric literals in `testQualificationBoot` and found no remaining layout literal. The
  fake's memory addresses (42000/42100/42600) and the `4 * index` element step that the `u32[N]`
  type implies are not layout values.
- **The decode cannot be green on a layout it misreads.** `readLayoutStruct` decodes every
  generated field by its name, offset and type. An unknown type, an unknown structure or a missing
  field name causes an assertion failure or a throw. A new or renamed generated field adds a key that
  the strict `deepEqual` refuses. A `u64` that becomes `u32` changes a BigInt to a number, which
  `deepEqual` also refuses. Probes on the drifted layout (below) give red for a code drift, an offset
  drift, an entry-size drift, a request-size drift, an ABI version drift, a target/channels offset
  swap and an entry-count offset drift. A rewrite of the layout with no value change is green on all
  three suites, so the reformatting alone does not cause red.
- **n3 (reserved words): fixed.** All reserved words must be zero: the request's `reserved0` and
  `reserved[2]`, and each entry's `reserved[3]` (`zeroReserved`, `:2893`). A worklet that writes 1
  to a reserved word at request offset 28, request offset 12 or entry offset 20 is red on the new
  suite and green on the pre-#1491 suite.
- **m1 (D3 test value): fixed and true.** The attempt record (spec `:168` ff.) and a
  `MUTATIONS.md` row record D3's test value and its `qualification.js` probe. I reran the probe. It
  is red on the new suite with `renderCorpusSegment stages a spectrum collection; only
  runStagingReadRun boots one`, and green on the pre-#1491 suite. Thus no earlier assertion reads
  those callers' `spectrumCollection`.
- **n1 (D3 message): fixed** (`:2924-2925`).
- **n2 (BOM): fixed.** The decoder uses `ignoreBOM: true` (`:2585`).

## BLOCKER / MAJOR / MINOR

None.

## NIT

- **n1.** `.github/ISSUE_SPECS/1491-...md:42-55`: the decision text still says "each 24-byte
  entry" (D1) and "the worklet's constants" (D2). The attempt-2 record correctly says that the
  values come from the generated layout. If the spec is edited again, change D1 and D2 to name the
  generated layout as the source. This does not change behavior.

## Test value (per new test)

- **D1/D2 (the staged collection equals the options and the generated layout):** the test is red
  when the worklet skips collection staging (p7). It is also red when the worklet stages a target
  code, channel code, capture budget, ABI version, struct size, nonzero reserved word or identity
  order that differs from the options and the bridge's rules. It is also red when the worklet's
  hand-typed layout drifts from the bridge's generated layout. Each of these is green on the
  pre-#1491 suite, and each layout drift is also green on attempt 1's suite.
- **D3 (the other callers stage nothing):** the test is red when one of the five other
  qualification callers starts to boot a spectrum collection. No earlier assertion reads those
  callers' `spectrumCollection`, because the pre-#1491 suite is green on the probe.
- **"exactly one boot per caller"** (`:2776-2778`) supports the witness bookkeeping. It is not a
  claimed test.

## Gates run

1. `bash scripts/test-web-audioworklet.sh` in qualification.yml's wrapper (private
   `mktemp -d` `TMPDIR`, `set -o pipefail`, leftover `find`): rc 0, leftover empty. The output
   contains `qualification boot contract passed: callers=6 real-ready=6 real-disposed=6
   diagnose-ready=1` two times (`gate1.log`).
2. Mutation table. I applied each mutation alone. I then restored the file from a saved copy of the
   committed file and confirmed it with `cmp`. At the end all three files matched the committed
   blobs (sha256 recorded), and the new suite was green again (`after-restore.new.log`).

   | mutation | file | new suite | pre-#1491 (d1b17d216) | attempt 1 (453f3038a) |
   | --- | --- | --- | --- | --- |
   | p7 | worklet | red: D2, actual `null` | green | - |
   | collection `both` -> `SPECTRUM_CHANNEL_LEFT` | worklet | red: D2, `channels: 1` vs 3 | green | - |
   | `maximumCaptureBytes / 2` | worklet | red: D2, `1048576n` vs `2097152n` | green | - |
   | `request.setUint32(28, 1, true)` | worklet | red: D2, `reserved` `[0, 1]` | green | - |
   | extra: request offset 12 = 1 | worklet | red: D2, `reserved0: 1` | green | - |
   | extra: entry offset 20 = 1 | worklet | red: D2, entry `reserved` | green | - |
   | extra: `ABI_VERSION + 1` | worklet | red: D2, `abiVersion` | green | - |
   | extra: struct size + 8 | worklet | red: D2, `structSize: 40` | green | - |
   | extra: `trackPostPan` -> `TRACK_POST_INPUT` | worklet | red: D2, `target: 1` vs 2 | green | - |
   | extra: identities written in reverse order | worklet | red: D2, `targetId` | green | - |
   | layout rewrite, no value change (control) | layout | green | green | green |
   | `both` = 4 | layout | red: D2, `channels: 3` vs 4 | green | green |
   | `maximumCaptureBytes` at 24, `reserved` at 16 | layout | red: D2, `0n` vs `2097152n` | green | green |
   | 28-byte entry (`reserved` `u32[4]`) | layout | red: worklet refuses the size, `host guard rejected (result=255)` | green | green |
   | extra: `output` = 4 | layout | red: D2, `target: 3` vs 4 | green | green |
   | extra: entry `target`/`channels` offsets swapped | layout | red: D2 | green | green |
   | extra: `entryCount`/`reserved0` offsets swapped | layout | red: host guard (fake capacity 0) | green | green |
   | extra: 40-byte request | layout | red: host guard | green | green |
   | extra: `abiVersion` 65537 | layout | red: D2, `abiVersion` | green | green |
   | D3: a default collection in `bootOptions` | qualification.js | red: D3, `renderCorpusSegment stages ...` | green | - |

3. `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.

No Rust changed, and the hermetic suite does not need the built module. Thus I did not build the
module.
