# Derivations — #243 SDK boot integration

Every re-pinned number in this change, with the arithmetic that discharges it, per the ceremony
amended by [#239 ruling 5462028562 A](https://github.com/misofm/engine-v2/issues/239#issuecomment-5462028562):
a derivation may live in a linked derivations document naming the commit it discharges.

Binding context for the whole issue: the brief (#243), the audit batch
[5462128475](https://github.com/misofm/engine-v2/issues/239#issuecomment-5462128475) and its
adoption [5462139867](https://github.com/misofm/engine-v2/issues/239#issuecomment-5462139867),
whose findings 1–6 amend the brief directly.

## 1. The artifact set: five files → six

`scripts/check-web-audioworklet.sh` pins the release directory's contents exactly, and
`hosts/miso-engine-host-web/DEPLOYMENT.md` states the same number in prose. Both moved from five
to six because `miso-engine-v2-abi-layout.json` joined the set.

The count is a direct enumeration, not an estimate: `.d.ts` + host `.js` + worklet `.js` +
`.simd128.wasm` + `miso-engine-v2-parameter-metadata.json` = 5 before;
`+ miso-engine-v2-abi-layout.json` = 6. The gate compares a sorted `find -printf '%f\n'` against a
sorted literal list, so the two spellings of "six" (the list and the error message) are checked
against the filesystem on every run, and the prose in `DEPLOYMENT.md` is the only copy that a gate
does not read — it is updated in the same commit and named here so the pair is auditable.

## 2. `scripts/sweep.sh`: 96 rows → 99

Three rows added, each next to the gate it belongs beside:

| row | placed after | why there |
|---|---|---|
| `check-abi-layout-v1.py --self-test` | `check-parameter-metadata-v1.py` | the two gates validate the two documents one generator emits |
| `check-sdk-generated.sh` | `check-abi-layout-v1.py` | it re-derives the same two documents, one layer further down the chain |
| `check-sdk-headless.sh` | `check-web-audioworklet.sh` | both build the release artifact and check something against it, so they sit in the build-bound tail |

`96 + 3 = 99`, and `grep -c '^row ' scripts/sweep.sh` reports 99 on the resulting tree.

The header's second sentence also moved, from "the repo has 96 check-*/test-* scripts" to 103. The
old sentence was already inaccurate — it conflated the row count with the script count — and the
new number is the direct count `ls scripts/ | grep -E '^(check|test)-' | wc -l` = 103 (100 before
this change, plus the three new gates). The difference `103 − 99 = 4` is not a coverage gap: five
scripts carry no row of their own (`check-capi-object-symbols-v1.py`,
`check-capi-qualification-evidence-v1.py`, `check-flac-decoder.mjs`, `check-web-boot-budget.mjs`,
`test-web-audioworklet.mjs`), each driven by a rowed entry point, and one rowed script contributes
two rows.

## 2a. The export set: 25 functions

`miso-engine-v2-abi-layout.json` publishes the whole export surface so no SDK call site types a
symbol name as a string literal. The count is not chosen: it is
`scripts/check-web-audioworklet.sh`'s own `expected_exports` list — which that gate proves against
the disassembled module — minus `memory`, which is linear memory rather than a call.
`26 − 1 = 25`. `tools/miso-engine-parameter-metadata/tests/abi_layout.rs` reads that list out of
the gate script itself and requires the two to be one list, so the number cannot drift on either
side without a red.

## 3. `SOURCE_RING_RESERVE_QUANTA = 2`, and eval 2's `9906 = 78 × 127`

`miso-engine-v2-abi-layout.json` publishes the default source-ring rule as its two inputs rather
than as a rate-specific answer, because the ring is **not readable back** across the ABI: it is an
input word at boot-options offset 16 and no export reports the effective value. A consumer that
must size its own producer therefore has to apply the rule, and publishing the rule is what keeps
it from holding a private copy of `100`.

The engine's rule, `miso_engine_host_core::default_source_ring_frames` (`prepare.rs:49-61`):

```
stall_frames = sample_rate_hz * SOURCE_STALL_TOLERANCE_MS / 1000
quanta       = ceil(stall_frames / quantum_frames) + 2
frames       = quanta * quantum_frames
```

`SOURCE_STALL_TOLERANCE_MS = 100` is transcribed from the constant. The `+ 2` is the number
re-pinned here as `SOURCE_RING_RESERVE_QUANTA`, and it is structural rather than tuned: one quantum
is held by the consumer while it renders and one is in the recycle path, so a producer that keeps
the tolerance filled never finds the ring closed. It is proved rather than asserted —
`tools/miso-engine-parameter-metadata/tests/abi_layout.rs` re-derives the ring from the two
published inputs and requires equality with `default_source_ring_frames` at all four launch rates
crossed with ten quanta (40 shapes). Red mutation: publish `reserveQuanta: 1` and every row misses
by exactly one quantum.

The brief's eval-2 number is re-derived independently here:

```
stall_frames = 96_000 * 100 / 1000 = 9_600
127 * 75 = 9_525  < 9_600
127 * 76 = 9_652 >= 9_600   =>  ceil(9_600 / 127) = 76
quanta   = 76 + 2 = 78
frames   = 78 * 127 = 9_906
```

`78 × 127 = 9906`, matching the brief and the existing pin at
`crates/miso-engine-host-core/tests/prepare.rs:419`. Both the arithmetic and the equality are
asserted in `abi_layout.rs`, so the number in the brief has a witness that does not read the brief.

## 4. `bootResultAliases` is exactly three rows

Adopted ruling finding 2 fixes the table as `{1: refusedDocument, 2: refusedOptions,
3: refusedLifecycle}`. The brief's S2(b) sentence additionally names `refusedBudget` in "the boot
vocabulary", which would read as a fourth alias row. It is not one, and the count is derived from
the Rust rather than from either sentence:

`hosts/miso-engine-host-web/src/lib.rs:95-99` declares exactly three alias constants —
`RESULT_REFUSED_DOCUMENT = RESULT_INVALID_ARGUMENT`, `RESULT_REFUSED_OPTIONS =
RESULT_ABI_MISMATCH`, `RESULT_REFUSED_LIFECYCLE = RESULT_WRONG_STATE`. `RESULT_REFUSED_BUDGET = 5`
(`lib.rs:78`) is a **primary** code in the frozen ladder, not an alias of anything: it already
carries its own base name, so boot's return needs no alias row to spell it. `3 aliases + 1 primary
= the four refusal spellings the brief lists`, which is why the sentence and the table disagree on
the count without disagreeing on the vocabulary.

Both the generator test and the schema gate pin the three: `abi_layout.rs` compares the table
against the three alias constants in constant order and requires every alias value to be a value
`resultCodes` already names under a *different* name (an alias that equalled its base name would
not be an alias); `check-abi-layout-v1.py` repeats the rule as an independent implementation and
carries `an alias row is dropped`, `an alias repeats its base name`, and `an alias invents a value`
among its fifteen red mutations.

## 5. The staging sequence is four calls, not three

The brief and #240 both describe "the 3-call boot". The emitted `stagingSequence` names four
exports, and the count is read off the shipped call sites rather than off either sentence:
`scripts/check-web-boot-budget.mjs:25-36` and
`hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet.js:172-196` both call
`miso_engine_web_v1_abi_version`, then `miso_engine_web_v1_boot_options_ptr`, then
`miso_engine_web_v1_document_ptr`, then `miso_engine_web_v1_boot`. The options block must be
addressed before it can be written, so `boot_options_ptr` is a call and not a step of `boot`;
`3 + 1 = 4`. The rider to adopted ruling finding 2 requires the regenerated document to name the
true sequence, which retires the shorthand before a literalist counts to four.
`check-abi-layout-v1.py`'s `the staging sequence drops back to three calls` mutation is the red
proof.

## 6. Structure byte totals are tiled, not asserted

`check-abi-layout-v1.py` does not carry the structures' field lists. It carries their byte totals
(`bootOptions` 64, `status` 80, `resourceReport` 224, `meterHeader` 64, `commandReport` 48) and
requires each structure's rows to *tile* that total: every row starts exactly where the previous
row's declared width ended, and the final row ends exactly at `bytes`. A renamed row, a dropped
row, a widened row, or a hole all move the sum.

The totals themselves are the engine's `size_of::<T>()` values, emitted through the
`*_BYTES` constants, and are independently pinned in `hosts/miso-engine-host-web/src/tests.rs`.
Restating them in the Python gate is deliberate duplication: the gate is a second implementation,
and a total that agrees with a tiling it did not compute is worth more than an imported constant.

## 7. `scripts/sweep.sh`: 99 rows → 100

One row added, next to the gate it belongs beside:

| row | placed after | why there |
|---|---|---|
| `check-sdk-deletions.py --self-test` | `check-sdk-generated.sh` | the generated gate proves the SDK's ABI surface is *derived*; this one proves the surface it replaced is *gone*. Neither claim follows from the other, and the second is the one no eval can make, because every eval exercises code that exists |

`99 + 1 = 100`, and `grep -c '^row ' scripts/sweep.sh` reports 100 on the resulting tree.

The header's script-count sentence is reconciled in the same change, from "the repo has 103
check-*/test-* scripts" to 105, which is the direct count
`ls scripts/ | grep -E '^(check|test)-' | wc -l` on the resulting tree. Two scripts are owed to it,
one from each of two commits in this branch:

```
103  the count as §2 left it
  +1  scripts/check-sdk-types.sh      (added by dd17ddd, header not moved with it)
  +1  scripts/check-sdk-deletions.py  (this change)
= 105
```

`dd17ddd` added `check-sdk-types.sh` and its exclusion note at the bottom of the file but left the
header sentence at 103, so the header was one behind before this change touched it. Both are
settled here rather than one being absorbed silently into the other, and the "Three check-*/test-*
scripts are excluded" sentence becomes "Four" for the same reason — `check-sdk-types.sh` is the
fourth exclusion note, and `grep -cE '^# (check|test)-[a-z0-9.-]+\.(sh|py|mjs) --'` reports 4.

The difference `105 − 100 = 5` is unchanged in kind from §2: five scripts carry no row of their own
(`check-capi-object-symbols-v1.py`, `check-capi-qualification-evidence-v1.py`,
`check-flac-decoder.mjs`, `check-web-boot-budget.mjs`, `test-web-audioworklet.mjs`),
`check-sdk-types.sh` is a sixth that is deliberately unswept because `tsc` needs the network to
install, and one rowed script contributes two rows: `5 + 1 − 1 = 5`.
