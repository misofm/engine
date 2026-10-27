# #2127 — Add file-based stereo loudness conformance with ITU-R BS.2217-2

## Origin and smallest closable slice

Successor of #2115, filed 2026-10-26. Qualify the shipped stereo-output loudness meter
against the complete mono/stereo-applicable ITU-R BS.2217-2 file subset, at the
files' native 48 kHz rate. The original request also covered authentic EBU Tech
3341 cases 7–8 / Tech 3342 cases 5–6; root split that blocked fixture acquisition
into [#2167](https://github.com/misofm/engine/issues/2167) before implementation.
Existing recipe/true-peak tests remain the owners of their cases.

Root's final scope ruling (2026-10-27): do not expand production into multichannel
metering, downmix material, or build a test-only multichannel composite. Inventory every
published ITU resource and explicitly classify unsupported channel layouts outside the
current product contract. This is no claim of complete ITU multichannel compliance.
Root approved this bounded ITU slice on 2026-10-27; GitHub synchronization precedes
any builds/tests.

## Boundaries and proposed paths

- `crates/loudness/tests/official_files.rs`: one owning file-conformance integration test,
  a small PCM WAV reader, native-rate block feed, and document tolerance assertions.
- `crates/loudness/tests/data/official-loudness.tsv`: sole fixture inventory/identity
  manifest, official URLs, archive and selected WAV SHA-256, native PCM metadata,
  expected readings, applicability and exclusions; no audio.
- `crates/loudness/tests/data/fetch-official-loudness.py`: standard-library-only fetch,
  exact member extraction and checksum verification into a caller-supplied external
  directory; never a test-time network request. Verify existing local files too.
- `crates/loudness/tests/data/README.md`: licence/source credit, acquisition and mandatory
  explicit qualification command. Ordinary offline tests do not claim these files ran;
  the external-data test is explicitly ignored until invoked with its documented command.
  Once invoked, absent/corrupt fixtures fail; there are no per-case skips.
- This issue spec. No production/Cargo/workflow/shared gate edits.

## Official evidence and licence/access feasibility

- EBU current landing page: <https://tech.ebu.ch/publications/ebu_loudness_test_set>.
  Official v5 download:
  <https://tech.ebu.ch/files/live/sites/tech/files/shared/testmaterial/ebu-loudness-test-setv05.zip>.
  Terms: <https://tech.ebu.ch/files/live/sites/tech/files/shared/testmaterial/use%20of%20EBU%20AUDIO%20test%20sequences.pdf>.
  July 2019 terms permit internal R&D assessment, prohibit redistribution and commercial
  use of sequences, and require the credit **© EBU** when reporting their use.
  No audio/archive is committed or published. The test is an internal R&D assessment.
- EBU expectations: <https://tech.ebu.ch/docs/tech/tech3341.pdf> Table 1 cases 7–8:
  integrated −23.0 ±0.1 LUFS; <https://tech.ebu.ch/docs/tech/tech3342.pdf> Table 1
  cases 5–6: LRA 5/15 ±1 LU respectively. Tech 3342's file-based MATLAB note asks
  for at least 1.5 seconds trailing silence before the final LRA reading; capture I
  at actual EOF and LRA after this documented flush, with no resampling or clipping.
- ITU report: <https://www.itu.int/dms_pub/itu-r/opb/rep/R-REP-BS.2217-2-2016-PDF-E.pdf>,
  published 2016, defines all files as 16-bit PCM, 48 kHz and file-based tolerance
  ±0.1 LKFS. Catalogue <https://www.itu.int/oth/R1102000001/en> lists an aggregate
  archive plus 47 individual archives, currently updated 2022-07-18.
  <https://www.itu.int/home/copyright/index.html> permits internal product development
  and implementation use, prohibits external distribution without permission.
  Download selected archives directly from ITU; commit hashes/metadata, never audio.
- Local access 2026-10-27: ITU catalogue/report/individual ZIP download succeed. EBU's
  landing page and v5 archive return Cloudflare HTTP 403 to ordinary local curl,
  including `/docs/testmaterial/` and old `/webdav/site/tech/shared/testmaterial/`
  aliases. Web lookup reads EBU landing/terms and recognizes the official ZIP but
  cannot export binary data. Preserve this blocker; no unofficial mirror substitutes.
  EBU's native WAV rate/channels/bit depth remain unverified until actual bytes arrive.

## Exact ITU applicability matrix (catalogue archive suffix)

Every suffix below expands to
`https://www.itu.int/dms_pub/itu-r/oth/11/02/R1102000001NNNNZIPM.zip`.
All report rows are native 48 kHz. The final manifest records each resource separately,
using the actual archived WAV spelling rather than assuming the report's typography.

| Suffix | Published resource label | Channels | Decision / expected I |
| --- | --- | --- | --- |
| 0001 | Aggregate of component archives | mixed | acquisition convenience only; not an additional case |
| 0002 | 1770-2_Comp_18LKFS_FrequencySweep | 1 | mandatory; −18.0; mono goes left, right zero |
| 0003–0008 | 1770-2_Comp_23LKFS_{25,100,500,1000,2000,10000}Hz_2ch | 2 | all six mandatory; −23.0 |
| 0009–0014 | 1770-2_Comp_23LKFS_ChannelCheck{Centre,Left,LFE,Ls,Right,Rs} | 6 | outside stereo contract; no channel dropping |
| 0015 | 1770-2_Comp_23LKFS_SummingTest | 6 | outside stereo contract; no test-only channel summing |
| 0016–0021 | 1770-2_Comp_24LKFS_{25,100,500,1000,2000,10000}Hz_2ch | 2 | all six mandatory; −24.0 |
| 0022–0027 | 1770-2_Comp_24LKFS_ChannelCheck{Centre,Left,LFE,Ls,Right,Rs} | 6 | outside stereo contract; no channel dropping |
| 0028 | 1770-2_Comp_24LKFS_SummingTest | 6 | outside stereo contract; no test-only channel summing |
| 0029 | 1770-2_Comp_AbsGateTest | 2 | mandatory; −69.5 |
| 0030 | 1770-2_Comp_RelGateTest | 2 | mandatory; −10.0 |
| 0031–0036 | 1770-2_Conf_6ch_{VinCntr,VinL+R,VinL-R-C}-{23,24}LKFS | 6 | outside stereo contract; no downmix |
| 0037–0038 | 1770-2_Conf_Mono_Voice+Music-{23,24}LKFS | 1 | both mandatory; −23.0 / −24.0; mono left, right zero |
| 0039–0040 | 1770-2_Conf_Stereo_VinL+R-{23,24}LKFS | 2 | both mandatory; −23.0 / −24.0 |
| 0041–0042 | 1770Conf-{24,23}LKFS-8channel | 8 | outside stereo contract |
| 0043–0044 | 1770Conf-{24,23}LKFS-10channel | 10 | outside stereo contract |
| 0045–0046 | 1770Conf-{24,23}LKFS-12channel | 12 | outside stereo contract |
| 0047–0048 | 1770Conf-{24,23}LKFS-24channel | 24 | outside stereo contract |

Nineteen mono/stereo cases are mandatory. Twenty-eight multichannel cases are outside
this product contract, not passing/skipped conformance rows. Mono unit weighting and
stereo unit weights are documented in BS.2217-2's channel ordering/weighting table;
mono left + zero right preserves exactly that measurement without a gain transform.

## Frozen gates and test value

1. Official-origin archive and exact selected WAV identities pinned with SHA-256 in the
   sole corpus-owner manifest; changed/absent content fails before measurement. SHA-256
   pins fixture acquisition identity, never production output or resource byte budgets.
2. Parse actual RIFF PCM metadata, refuse unsupported rate/channel/sample-format drift,
   preserve original samples, feed blocks from sample zero at the native rate.
3. Each selected ITU I reading meets its report's ±0.1 LKFS. Print every reading and
   explicit total; no tolerance relaxation. The successor alone owns EBU I/LRA tests.
4. Run proportional loudness tests, formatting and lint through the shared serial wrapper,
   with CARGO_BUILD_JOBS=2 / RUST_TEST_THREADS=1 and no concurrent test subprocesses.
5. Fetcher preflight/failure checks: existing verified cache, corrupt payload rejection,
   absent member/file rejection, no overwrite of mismatching data. No generic harness.
6. Fresh adversarial review (root owns Astra review), maximum five coherent attempts.

Test-value claim: authentic, spectrally/time-varying published PCM catches K-weighting,
and gating errors that the existing synthetic-tone recipes can agree on while
mis-measuring real programme material; published frequency/sweep and gate edge cases
also hold those algorithms to independent official byte-level stimuli and expected values.
No new production DSP, benchmark, listening panel, target matrix or realtime path is
introduced; existing owners retain realtime/portability claims.

## Decisions/evidence

2026-10-27 Sol implementation stream: brief only; no builds/tests before root approval.
An unavailable EBU archive remains an explicit acquisition blocker, never qualification.
Root bounded delivery to the obtainable ITU slice before implementation and owns the
separate EBU successor. No EBU placeholders or unverified hashes belong in this slice.
All 19 ITU archives were successfully obtained directly from the official catalogue;
actual metadata confirms 48 kHz 16-bit PCM. The two −24 LKFS programme files use
WAVE_FORMAT_EXTENSIBLE PCM; the test reader must validate the PCM subtype GUID.

2026-10-27 root approved exact corpus-header compatibility: the official −24 LKFS
mono/stereo programme files (0038/0040) declare a RIFF size eight bytes short.
Archive and WAV hashes pass; individual fmt/data chunk sizes span physical EOF.
The manifest pins delta 8 only there and 0 elsewhere, with no sample modification.
Both verifiers require the pinned delta and complete bounded chunks/aligned frames.
The first acquisition run refused strict RIFF equality before any DSP ran; evidence
`2127/attempt1-official.log` records this source-header defect, not a meter failure.

Attempt 1 implementation checkpoint (2026-10-27): sole TSV inventory contains all
48 catalogue resources (aggregate + 19 required + 28 outside stereo contract).
The standard-library downloader verifies both archive/WAV hashes and PCM headers;
the explicit ignored integration test verifies the entire private cache offline,
then measures all 19 native PCM files through the shipped observer/analysis.

Focused gate: PASS, 19/19 within ±0.1 LKFS, with no skip and no production changes.
Largest absolute error: absolute-gate case 0.04845 LKFS; relative-gate error
0.02880 LKFS; authentic mono/stereo programme errors 0.00644–0.02700 LKFS.
`cargo fmt -p loudness` passed. Exact wrapper command and all case readings are in
external `2127/attempt1-qualified.log`; the shared serial log records no overlapping
batch CPU work. Broader crate/lint and acquisition failure checks passed as recorded below.

Test value, `itu_bs2217_published_mono_stereo_files`: wrong channel power handling,
spectral K-weighting or gating on authentic time-varying programme PCM and the
published near-threshold gate files turns this red where recipe-only sine cases
cannot establish the published programme/file-based readings.

Attempt 1 proportional verification (2026-10-27): PASS. All commands ran
sequentially under the shared serial wrapper in this worktree:

- `python3 .../2127/acquisition-preflight.py`: five expected refusals (missing official
  archive, missing WAV, corrupt existing WAV, corrupt existing archive, unexpected
  archive member). Corrupt data remained unchanged; no network used in failure checks.
- `python3 crates/loudness/tests/data/fetch-official-loudness.py <private corpus> --verify`:
  all 19 archive/WAV identities and metadata verified offline.
- `cargo fmt -p loudness --check`: PASS.
- `cargo test -p loudness -- --test-threads=1`: all 11 ordinary tests PASS, explicit
  official-data test visibly ignored as designed (its separate 19-file run passed above).
- `cargo clippy -p loudness --tests -- -D warnings`: PASS.

External `2127/attempt1-proportional.log` preserves output and the serial log preserves
queue execution. These checks add no committed harness tests or production changes.
Author stream is complete with a clean checkpoint; fresh adversarial review, integration
and GitHub synchronization remain root's delivery work. No upstream push or issue
closure has been made by this stream, and no EBU/multichannel compliance is claimed.

## Fresh independent adversarial verdict — conditional PASS (2026-10-27)

Astra, with no implementation role, independently reviewed the official BS.2217-2 report,
catalogue, copyright terms, the sole manifest, acquisition verifier and PCM reader. The amended
nineteen-file mono/stereo scope is complete; EBU programmes remain explicitly owned by #2167.
Both readers enforce the pinned container delta, bounded complete chunks and unmodified PCM;
mono plus zero right preserves unit channel energy and measurement ends at actual EOF.

[Evidence run 40471281487](https://github.com/misofm/engine/actions/runs/40471281487) tested source
`21c38051cba9abeb706fcaa1622485cb267da702` through workflow-only proof commit
`7f14f6fbf88c1ecb438976d63a1599e6dddf6d50`. Fresh private downloads and offline archive/WAV
verification preceded all nineteen readings; the explicit test passed with zero ignored cases.
Independent recalculation found maximum absolute error **0.04844452 LKFS**, on the absolute-gate
file, within the published 0.1 tolerance. No audio or archive was uploaded in the evidence artifact.
The run's later #988 performance failure does not invalidate this completed gate.

Test value: `itu_bs2217_published_mono_stereo_files` detects disagreement with independently
authored official programme/frequency/gating PCM and published targets that the synthetic recipe
tests cannot establish. This is one fixture-identity owner, with no new production dependency.

**Conditional source-review PASS, attempt 1.** The loudness changes are byte-unchanged between the
proved source and reduced tree `da4f72f60d29bade17cd86b7a299b3d2f8734b9f`; removal of the failed
limiter screen introduces no loudness edit. Normal required qualification on the final reduced
product tree remains mandatory before delivery/closure. Raw artifact and independent judgment:
`engine-work-evidence/easy3-20261027/fresh-astra/remote/result/` and
`fresh-astra/remote/artifact-judgment.json` outside Git.

## Root source co-review and final delivery gate (2026-10-27)

PASS for the scoped official ITU corpus owner after fresh independent review `0a29fc46fddd4aaf126897b09c4e09ec1e18fcfe`. Official byte identities, native channel/rate handling, bounded header exceptions and the published ±0.1 LKFS expectations are explicit. All 19 required cases executed; the EBU and multichannel boundaries remain candid. No production DSP or redistributed audio enters the batch.

The limiter screen is excluded after its real performance failure. Its removal leaves this slice’s reviewed source unchanged. Final normal required qualification on the actual combined product tree is mandatory before merge, issue closure and completed-worktree cleanup. This record does not claim that pending workflow passed.

## Required-CI integration verdict and bounded attempt 2 (2026-10-27)

Attempt 1 failed required CI for [PR #2169, run 40475574564](https://github.com/misofm/engine/actions/runs/40475574564), lint job `122093153624`: after formatting, full workspace Clippy and earlier policies passed, `check-env-vocabulary.sh` refused the fixture environment name because it lacked the mandatory `MISO_ENGINE_` prefix. The earlier conditional source approvals did not establish this integration policy; attempt 1 is not a final PASS.

Bounded attempt 2, Sol commit `f7707d436e8117ed4bb0ddd957ba753d326405de`, changes exactly three active string occurrences in `official_files.rs` and its data README to `MISO_ENGINE_LOUDNESS_FIXTURES`. There is no alias, vocabulary-table entry, new test, manifest/byte change, DSP change or tolerance change. Historical frozen proof scripts and logs retain the name they actually used.

The exact blocking command `bash scripts/check-env-vocabulary.sh` passed through the shared serial wrapper (63 documented names; one mandatory engine prefix). Evidence: `engine-work-evidence/easy3-20261027/2127/attempt2-env-vocabulary.log`. The author worktree is clean; no local Cargo, test binary or timing workload ran for this correction. The existing vocabulary owner discriminated the defect, so no new test is introduced. Fresh Astra narrow verification and full required qualification on the corrected head remain mandatory before merge/closure.

### Fresh independent bounded attempt 2 verdict — PASS, final CI still required

Astra reviewed author commit `f7707d436e8117ed4bb0ddd957ba753d326405de`, integrated at
`6209df13285c701d277e9d67b80df9db25932465`. The entire correction is three occurrences in two
files: the environment lookup, its missing-variable diagnostic and the README command now use
`MISO_ENGINE_LOUDNESS_FIXTURES`. There is no compatibility alias, manifest, PCM reader,
measurement, acquisition, tolerance or production change. The frozen remote proof remains an
accurate record of its original environment spelling and is not rewritten.

The actual `check-env-vocabulary.sh` owner requires the `MISO_ENGINE_` prefix across scanned
repository files, while its table equality applies only to names used under `tools/` or
`scripts/`; a crate-only table row would therefore be incorrect. The author ran this unchanged
owner through the serial wrapper and recorded `env vocabulary: ok (63 names, one MISO_ENGINE_
prefix)` in `2127/attempt2-env-vocabulary.log`. This directly addresses the observed required-CI
failure without weakening the gate. No new/rewritten test or duplicate official-corpus run is
needed for the constant-string correction; the original test-value judgment and nineteen-file
measurement evidence remain applicable. **PASS this bounded attempt 2 correction**, contingent
on the mandatory corrected qualification run for final delivery.
