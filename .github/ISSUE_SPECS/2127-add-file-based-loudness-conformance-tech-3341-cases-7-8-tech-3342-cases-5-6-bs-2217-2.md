# #2127 — File-based loudness conformance from official EBU/ITU audio

## Origin and smallest closable slice

Successor of #2115, filed 2026-10-26. Qualify the shipped stereo-output loudness meter
against authentic EBU Tech 3341 cases 7–8 / Tech 3342 cases 5–6 and the complete
mono/stereo-applicable ITU-R BS.2217-2 file subset, at the files' native rates.
Existing recipe/true-peak tests remain the owners of their cases.

Root's provisional scope ruling (2026-10-27): do not expand production into multichannel
metering, downmix material, or build a test-only multichannel composite. Inventory every
published ITU resource and explicitly classify unsupported channel layouts outside the
current product contract. This is no claim of complete ITU multichannel compliance.
Root must approve the final brief and synchronize GitHub before implementation.

## Boundaries and proposed paths

- `crates/loudness/tests/official_files.rs`: one owning file-conformance integration test,
  a small PCM WAV reader, native-rate block feed, and document tolerance assertions.
- `crates/loudness/tests/data/official-loudness.json`: sole fixture inventory/identity
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
3. Each selected ITU I reading meets its report's ±0.1 LKFS. EBU I meets ±0.1 LUFS,
   EBU LRA meets ±1 LU. Print every reading and explicit total; no tolerance relaxation.
4. Run proportional loudness tests, formatting and lint through the shared serial wrapper,
   with CARGO_BUILD_JOBS=2 / RUST_TEST_THREADS=1 and no concurrent test subprocesses.
5. Fetcher preflight/failure checks: existing verified cache, corrupt payload rejection,
   absent member/file rejection, no overwrite of mismatching data. No generic harness.
6. Fresh adversarial review (root owns Astra review), maximum five coherent attempts.

Test-value claim: authentic, spectrally/time-varying published PCM catches K-weighting,
gating and percentile errors that the existing synthetic-tone recipes can agree on while
mis-measuring real programme material; published frequency/sweep and gate edge cases
also hold those algorithms to independent official byte-level stimuli and expected values.
No new production DSP, benchmark, listening panel, target matrix or realtime path is
introduced; existing owners retain realtime/portability claims.

## Decisions/evidence

2026-10-27 Sol implementation stream: brief only; no builds/tests before root approval.
An unavailable EBU archive remains an explicit acquisition blocker, never qualification.
If root bounds delivery to the obtainable ITU slice, it must amend scope and preserve
EBU acquisition as a stateless successor rather than close this full brief as passed.
