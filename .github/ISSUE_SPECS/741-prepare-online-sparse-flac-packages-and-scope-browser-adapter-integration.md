# Prepare online sparse-FLAC packages and scope browser adapter integration

## Problem and decision boundary

The current online release declares four recordings and 30 unique stem blobs. Its existing delivery assets are already native FLAC, so the earlier comparison against expanded WAV or a differently prepared single-song ZIP is not a valid savings baseline. This issue measures whether the frozen exact-zero sparse-FLAC representation reduces the actual complete bytes needed for this release relative to its 30 current FLAC objects.

This is a private research and asset-preparation issue. It downloads and verifies the current assets, creates four local candidate bundles, and scopes a later web-adapter integration. It does not change the release, publish blobs, mutate production Rust or TypeScript, add a codec to the engine, or measure network latency.

## Smallest closable outcome

For each of the four recordings, produce one self-contained ZIP_STORED candidate containing the sparse-FLAC representation and all metadata required to reconstruct every declared stem at its original length. Reconstruct and hash all 30 stems, then report exact current-FLAC bytes, exact candidate-package bytes including ZIP metadata, byte savings per recording and overall, and the limitations of using local package bytes as a transfer proxy.

Use the already qualified sparse-FLAC format and packer unchanged:

- PCM is stereo, signed 24-bit little-endian at 44,100 Hz; any source that does not match stops qualification.
- A frame is omitted only inside an exact all-zero run of at least 882 frames.
- Retained content is divided into independent FLAC units of at most 44,100 frames using the frozen ordinary FLAC setting.
- Each stem manifest binds its native frame count, gap map, canonical PCM identity, unit offsets/sizes, stored-unit identities and decoded identities.
- No near-zero threshold, gain change, fade, resampling, channel conversion, codec setting change, dictionary, alternate chunk size, or codec sweep is authorized.

Do not pad or align stems in this issue. Each downloaded FLAC's STREAMINFO frame count is the preserved output length. If a release/session declaration disagrees with a source's shape or length, stop and record the mismatch rather than inventing samples.

## Private acquisition and identity record

Treat the privately supplied release manifest as the frozen release inventory. Bind its SHA-256 before acquisition. It must contain exactly four recordings, 30 stems, 30 unique blob IDs and 30 unique declared digests.

Acquire each current FLAC once through the current caller-owned CDN locator and save it create-new in private ignored storage. HEAD is not an admission requirement because this endpoint returns 403 to HEAD while GET works. Record the successful GET/range response status, actual body byte count, relevant length/range and ETag headers when present, compressed-object SHA-256, and release blob mapping privately. A failed or partial response is not an asset; operational retry history is recorded and cannot select among alternate encodes.

The release `digest` semantics must be proved rather than inferred. For every stem, compute separately:

1. SHA-256 of the exact downloaded FLAC bytes; and
2. SHA-256 of the canonical headerless interleaved PCM24 bytes emitted by lossless decode.

Record which identity the declared digest matches. The expected web-adapter contract is canonical PCM identity (`StemIdentity` and `VerifiedStemStore`), but the preparation must fail closed if a digest matches neither form, differs across repeated verification, or conflicts with the corresponding engine-session source declaration. Never replace a declared identity silently.

## Bounded conversion and packaging protocol

Decode one current FLAC at a time into a temporary PCM24 WAV on disk with a fixed lossless decoder invocation. Use bounded streaming I/O; never retain a decoded full stem or corpus in memory. Verify the WAV rate, channels, depth, native frame count and canonical PCM SHA-256 before invoking the frozen sparse-FLAC packer. Delete the temporary WAV only after the sparse artifact passes full reconstruction and seek qualification. Preflight free space before acquisition/conversion, reject existing outputs, and publish each completed directory/archive by create-new or atomic rename.

Create one ZIP_STORED archive per recording. ZIP_STORED is packaging only and must not recompress or rewrite the independent FLAC units. Each archive contains:

- one canonical recording-level index with format version and ordered stem entries;
- each stem's canonical sparse manifest and required manifest checksum;
- every referenced independent FLAC unit; and
- enough shape, path, byte-count and identity metadata to reconstruct the complete native timeline without the source FLAC or external private inventory.

The private index may bind recording/stem references, blob IDs and digests. Public evidence uses anonymous recording groups and aggregates only. Archive entries use deterministic relative paths and order. The receipt records member names, uncompressed member sizes, file count, ZIP metadata bytes, final archive length and archive SHA-256. Verify the central directory contains each required file exactly once and no extra or unsafe path.

## Objective gates

1. **Inventory:** the frozen release JSON has four recordings and 30 unique stems; no declared stem is omitted or duplicated.
2. **Current transport integrity:** all 30 current FLAC downloads complete, have exact recorded byte lengths and compressed SHA-256 values, and parse as native FLAC. Actual downloaded bytes, not Content-Length or HEAD, define the baseline.
3. **Digest semantics:** every release digest is classified by comparison with both compressed-FLAC and canonical-PCM SHA-256. The result agrees with the engine-session declaration and the adapter's canonical-PCM identity contract, or the issue stops without proposing integration.
4. **Shape and duration:** every source, temporary WAV, sparse manifest and reconstructed output agrees on 44.1-kHz stereo PCM24 and that stem's original positive frame count. No padding, truncation, resampling or metadata-derived offset is applied.
5. **Exact reconstruction:** all 30 sparse artifacts reconstruct to byte-identical canonical PCM and the native frame count. Full canonical hashes match the independently decoded current FLACs and, when confirmed by gate 3, the declared release digests.
6. **Sparse-format invariants:** every omitted interval is at least 882 frames and contains only exact zero PCM; every retained unit is independently decodable, at most one second, within the existing manifest/unit/resource caps, and passes stored/decoded digest checks. Use one common deterministic set of start/end/hash-derived/gap-boundary windows per stem for source-versus-sparse seek equivalence.
7. **Complete packages:** all four ZIP_STORED archives pass central-directory/name/count/size validation and reconstruct all stems using only archive contents. Their actual file lengths equal content bytes plus computed ZIP metadata bytes. The four archive SHA-256 values and an aggregate receipt are preserved privately.
8. **Savings accounting:** for each anonymous recording and the release total, report current FLAC object count and exact summed bytes; sparse unit count and payload/manifest/index bytes; ZIP metadata bytes; actual candidate archive bytes; signed byte delta; and percentage delta against current FLAC bytes. A larger result remains a valid negative result. Do not quote the prior WAV/ZIP percentage as this release's saving.
9. **Evidence limits:** record commands, tool/library versions, release/input/output hashes, failures and filesystem facts. Do not collect codec timing, run a benchmark protocol, tune after seeing results, or label byte reduction as measured download time, CDN cost, HTTP overhead or Walrus latency.
10. **Privacy and publication:** detailed titles, parties, URLs, blob IDs, per-stem digests, filenames, audio, units and candidate archives remain in the private agent repository's ignored data area or other private artifact storage. The public issue/spec may state only the generic method, four-recording/30-stem aggregate shape, anonymous per-recording byte totals, aggregate result, limitations and decision. Nothing is uploaded or substituted into the online release.

## Web-adapter and Rust scope note

Read-only source inspection identifies the later integration boundary:

- `src/stems/flac-resolver.ts` currently locates and range-decodes one native FLAC transport object.
- `src/stems/types.ts` exposes `StemResolver` as a stream of canonical headerless PCM and keeps `StemIdentity` as the canonical PCM SHA-256.
- `src/stems/store.ts` publishes a source only after exact canonical byte-count and digest verification into OPFS.
- `src/session.ts` admits all declared sources before the session becomes ready, and the existing pump feeds canonical PCM to the engine.

The smallest later transport slice can add a package-aware resolver/cache in the web adapter: download and verify one recording bundle, validate its index/manifests, decode units plus declared zero gaps into the existing canonical `StemResolver` stream, and preserve the current verified-store/session contract. That slice should not require a Rust engine or session-schema change because Rust still receives the same declared canonical PCM frames and identity.

Retaining sparse compressed units in OPFS and decoding only local windows after download is a separate adapter storage/worker/pump issue. It must preserve generation-tagged seeks, bounded admissions, contiguous render quanta, exact zero synthesis, EOF and underrun distinctions, and duration-independent decoded memory. Any Rust change would require a concrete missing PCM-ingress capability demonstrated by that issue; source compression alone does not justify one. DSP skipping and Walrus transport qualification remain separate.

This issue produces only a private integration-scope note naming these seams. It makes no production adapter or Rust edits.

## Deliverables and workflow

Private deliverables are the frozen release receipt, acquisition/identity table, bounded conversion glue, all reconstruction/seek results, four archive receipts, aggregate comparison, and the read-only adapter/Rust scope note. Downloaded FLACs, temporary WAVs, sparse units and ZIPs are ignored artifacts and are never committed. The public deliverable is one synchronized sanitized issue/spec with aggregate results after Sol review.

Terra owns one minimal preparation implementation attempt in a dedicated private worktree and must pause at a focused-green synthetic/preflight checkpoint. Root owns current-asset acquisition and any external side effect. Sol then reviews the exact conversion, identity and packaging gates before the 30-source run. A final Sol review checks all artifacts and claims before any public result is synchronized.

If identity semantics fail, a source is not PCM24 stereo/44.1 kHz, the current locator cannot retrieve a complete immutable body, or the frozen packer needs an algorithm/parameter change, stop and rebrief. Production adapter work, Rust changes, release-manifest migration, publishing, network timing and alternate codecs are bounded successor issues.

## Root acquisition checkpoint

Root completed the read-only input acquisition while this preparation issue was being scoped. The private receipt preserves an immutable copy of the source release manifest. Exactly 34 current objects were fetched: 30 unique native-FLAC stems and four engine-session documents. The 30 current stem objects total **431,666,252 bytes**, which is the frozen comparison baseline.

All 30 FLAC headers report stereo PCM24 at 44,100 Hz. Frame counts are uniform within each recording and differ across recordings: 6,207,923; 6,761,475; 7,717,500; and 8,499,092 frames. No padding or resampling is needed or authorized. Every downloaded engine session declares the corresponding release digest as its `sha256:` source content identity and agrees with the FLAC header shape. Canonical decoded-PCM hash verification remains mandatory before conversion; the session declarations corroborate the expected digest semantics but do not replace that byte-level gate.

Treat these downloads as frozen private inputs. Terra's attempt begins with receipt/file/hash admission and adds only conversion, sparse packing, reconstruction, packaging and reporting. It must not redownload or select alternate source objects.

## Sol scope verdict

**PASS to create and synchronize one sanitized preparation/research issue.** The slice answers the user's immediate question against the correct existing-FLAC baseline, creates reviewable private candidate assets, and produces a concrete integration scope without crossing into production code or publication. Its success criterion is exact reconstruction and truthful complete-package byte accounting, not a required positive savings result.

## Implementation checkpoint and readiness review

The private bounded conversion/packaging tranche is upstream. Attempt 1 failed Sol review on aggregate-baseline accounting, workspace admission, an absent eligible-gap fixture and incomplete input/package controls; no real conversion ran on that attempt.

Sol attempt 2 readiness PASS covers strict release/session/digest/shape admission, conservative workspace accounting, manifest-versus-independent-census checks, correct total baseline and exact archive member/byte evidence. Root independently reran focused synthetic tests and whitespace checks. Controls prove 30 sparse reconstructions, 240 common windows, 30 manifest checks and 30 archive-only reconstructions on synthetic data. The previously qualified codec and packaging helper remain unchanged. The single untimed real preparation and final evidence review remain outstanding.

The user's clarified target includes a future, more separated source set with potentially much more exact silence. Preserve the engine's existing PCM contract: sparse transport and a later sparse packed-PCM browser cache can both be adapter capabilities. A compact span inside Rust or DSP skipping requires a separate measured need. The candidate transport groups each recording's stem manifests and units in one uncompressed archive for complete offline download; Walrus quilt grouping remains a deployment choice, not codec granularity.

## Real preparation and measured bytes

The single untimed preparation completed. All 30 full sparse reconstructions, 240 common seek windows, 30 manifest/census comparisons and 30 archive-only reconstructions passed. All 30 declared release digests match canonical PCM; none matches its compressed FLAC byte hash. All source shapes and lengths were preserved. Every downloaded body matched its recorded response length. No upload, release mutation, codec tuning or benchmark occurred.

| Anonymous recording | Stems | Existing FLAC bytes | Complete candidate bytes | Size change | Sample frames omitted from decode |
|---|---:|---:|---:|---:|---:|
| 1 | 8 | 92,362,259 | 92,567,704 | +0.222% | 16.33% |
| 2 | 8 | 70,669,885 | 70,853,875 | +0.260% | 49.15% |
| 3 | 6 | 107,965,745 | 109,009,593 | +0.967% | 24.37% |
| 4 | 8 | 160,668,363 | 161,930,103 | +0.785% | 14.82% |
| Total | 30 | 431,666,252 | 434,361,275 | +0.624% | 25.71% |

The four complete archives contain 4,184 independent FLAC units. Accounting is exact: 432,203,890 bytes of FLAC payload, 1,223,273 bytes of manifests/checksums/recording indexes, and 934,112 bytes of ZIP structure. Total growth is 2,695,023 bytes. Unchanged engine-session documents and runtime assets are excluded from both sides. These are local file-body sizes, not measured network traffic, download time or Walrus charged storage.

**Negative download result:** this fixed candidate is slightly larger than the existing already-FLAC release and does not achieve a download-byte reduction for this corpus. No positive saving is claimed. Old encoder settings were not established, so the result does not isolate silence removal from FLAC settings or unit boundaries.

There are 56,054,821 omittable frames in 218,052,920 total stereo source frames. This is a decoder-sample reduction, not a measured CPU-time reduction: FLAC silence is cheap, and hashing/storage/chunk overhead remain. The current adapter does all FLAC decoding during offline preparation and plays from dense cached PCM; these changes therefore initially affect preparation, not playback codec work.

## Integration decision

Make sparse sources a first-class adapter capability while preserving the Rust PCM input contract. The first production slice adds bounded manifest/container validation and a package-aware sparse resolver feeding today's verified canonical PCM store. Canonical identity, shape, timeline, full verification, seeks, backpressure and underrun semantics remain unchanged. Caller-side delivery selection and publishing mappings must support the new container before online references change.

A second independently closable adapter slice stores packed active canonical PCM plus intervals. It preserves decode-once preparation while avoiding gap writes/reads: this corpus has 1,308,317,520 dense canonical PCM bytes versus 971,988,594 active-region bytes, a potential 336,328,926 logical-byte reduction before cache metadata. Physical allocation and browser time remain unmeasured. Full canonical hashing still includes implicit zeros. Retaining compressed FLAC for playback instead would introduce new playback decode work and needs separate qualification.

Neither slice requires Rust changes. Rust source-queue gap records or graph/DSP skipping require separate measured justification; a JavaScript-only compact gap expanded before the existing planar boundary also need not change Rust. Effects, tails, lookahead/PDC, sidechains and automation still determine valid DSP work.

For complete download, the current candidates group recording stem assets in one ZIP_STORED container suitable for an ordinary Walrus blob. Quilts may instead group packaged stems; decoder units should not automatically become individual patch requests. The user's future more-separated stems may expose much higher silence, but no future size or speed percentage is established here.

Private upstream evidence includes conversion code/tests, immutable input/execution receipts, four archive receipts, per-stem counts, a complete report and file-level adapter/app/CDN/Rust scope. Candidate audio/archives remain local. Final Sol review passed.

## Final adversarial verdict

**Sol attempt 2 / final evidence: PASS.** Independent review reproduced immutable input/code identities, all qualification counts, every archive's actual size/hash/CRC/member inventory, exact byte/frame arithmetic and the preserved successful HTTP body lengths. The private report and source-level scope make the negative wire-size result explicit and contain no unsupported time, filesystem, network or DSP claims. The private evidence and sanitized public decision record are upstream. The preparation/scoping outcome is complete; production implementation and publication remain separate future work.
