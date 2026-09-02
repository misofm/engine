# Build canonical sessions directly from local FLAC stem directories

## Objective

Make the publishable `enginectl` executable turn one leaf directory of local FLAC stems into one
canonical, embedded-engine-accepted Session V1 document without Cargo, ffmpeg, ffprobe, a network,
or caller-supplied audio metadata. Preserve the existing strict JSON request mode byte-for-byte.
Package and reuse the already qualified FLAC decoder so content identities remain SHA-256 over the
canonical PCM preimage rather than over transport bytes.

## Dogfood evidence

The supplied root `/Users/bl/Desktop/between-the-doors/between-the-doors-stems` is a collection of
four independent song directories, not one session:

| directory | stems | sample rate | channels/depth | frames per stem |
| --- | ---: | ---: | --- | ---: |
| `ghost` | 8 | 44,100 | stereo PCM24 | 6,207,923 |
| `play-me` | 6 | 44,100 | stereo PCM24 | 7,717,500 |
| `war` | 8 | 44,100 | stereo PCM24 | 6,761,475 |
| `wide-open` | 8 | 44,100 | stereo PCM24 | 8,499,092 |

Session V1 has no timeline or song grouping. Recursing from the supplied root would start and sum
four unrelated songs at sample zero. The correct final output is four documents:
`ghost.session.toml`, `play-me.session.toml`, `war.session.toml`, and
`wide-open.session.toml`.

Current `enginectl session build` requires a caller to supply `channels`, `bitDepth`, `frames`, and
the canonical-PCM SHA-256 for every source. It cannot inspect FLAC, and the npm package does not
currently contain the repository's existing pinned FLAC decoder Wasm. Requiring an agent to invoke
external media tools or write its own decoder/identity pipeline defeats the standalone agent CLI.

## Decision and CLI contract

Extend the existing command additively:

```text
enginectl session build \
  (--request PATH|- | --stems DIRECTORY) \
  --output PATH|- \
  [--session-id ID] \
  [--quantum-frames POSITIVE_U32] \
  [--overwrite]
```

- Exactly one of `--request` and `--stems` is required.
- `--session-id` and `--quantum-frames` are valid only with `--stems`.
- Request-mode output, receipts, diagnostics, and exit behavior remain byte-for-byte compatible.
- `--stems` accepts one leaf directory whose directly owned regular files are `.flac`, matched
  case-insensitively. It never recurses.
- A directory containing child directories refuses before decoding with exit 3,
  `stems.collection`, `effect: "not_applied"`, and a byte-sorted `groups` array. An empty leaf,
  symlink entry, non-FLAC regular file, or unsupported entry refuses typed rather than being
  silently ignored.
- Session ID defaults to deterministic stable-ID normalization of the leaf basename;
  `--session-id` overrides it. Revision is 0. Quantum defaults to 128.
- Sample rate, channel count, source-native depth, frames, and canonical content identity are
  decoder facts. Mixed/unsupported sample rates refuse; there is no implicit SRC.
- Each FLAC becomes one source and one track. Stereo maps channel 0/1; mono duplicates channel 0
  to both track lanes. The session has one `main` output and one unity, 0 dB route per track from
  `post_matrix` to `main`. Builtins, racks, faders, pan, automation, and submixes remain at existing
  SDK defaults. Filenames never cause guessed gain, pan, effects, categories, or processing.
- IDs strip `.flac`, lowercase ASCII, preserve legal `_`, `.`, and `-`, replace other runs with
  `-`, and prefix names that do not begin with an ASCII letter. Normalization collisions and
  truncation use deterministic filename-byte SHA-256 suffixes, never directory iteration order.
  Discovery, diagnostics, receipt mappings, and builder insertion sort by raw filename bytes.

For `--output -`, stdout remains exactly canonical TOML and successful stderr is empty. File output
keeps the existing atomic no-clobber/overwrite contract and emits one compact JSON receipt after
publication. A file output whose physical parent is the physical stems leaf is refused before
decoder loading, including paths reached through symlink or filesystem case aliases: an output may
neither overwrite a source nor pollute the leaf and make the next build reject its own output.
Stems-mode receipts add input kind/path, derived session facts, and a sorted mapping
of each filename to source ID, track ID, identity, channels, depth, and frames. Request-mode
receipts gain no field. JSON escaping keeps hostile names data-only.

Exit status remains 2 for flag/scalar usage, 3 for discovery/read/FLAC/input-shape refusal, 4 for
embedded-engine refusal, 5 for output refusal, and 70 for a corrupt/missing/incompatible packaged
decoder or engine asset and unexpected internal failures. Every failure before publication reports
`effect: "not_applied"`; a receipt-channel failure after publication remains `effect: "applied"`.
The command never prompts, pages, opens another program, reads config/credentials, emits telemetry,
or uses a network.

## Decoder, identity, and resource contract

- Build and package the existing `sidecars/flac-decoder` Wasm, loader, declaration, and committed
  digest pin under package-relative `dist/assets`; add them to the package manifest and tarball
  closure without adding runtime or peer dependencies.
- Compile and instantiate the pinned decoder exactly once per command, then reuse it sequentially.
- Read each compressed FLAC exactly once. Incrementally hash its emitted canonical PCM blocks with
  Node SHA-256 and retain no complete decoded PCM.
- Require exact decoded bytes equal `frames * channels * bytes_per_sample` and reach decoder EOF so
  packet checksums, declared frame count, and FLAC MD5 have all been verified before admitting the
  source.
- Support only decoder-qualified PCM16/PCM24, engine-qualified mono/stereo, and launch sample rates.
- Enforce the decoder ABI's explicit encoded/canonical per-file `u32` ceilings with typed resource
  refusal. Bound accumulated filenames/authoring metadata by the existing 4 MiB request-equivalent
  budget; do not introduce a compiled track-count maximum.
- Preflight flags, output conflicts, directory shape, entries, and IDs before decoder compilation.
  Detect a source file changing during its single read and refuse rather than binding ambiguous
  bytes to a session.

## Scope

- `sdk/src/enginectl.ts` and narrowly factored `sdk/src/cli/*` implementation;
- SDK package staging/build scripts and manifest/types needed to ship the existing decoder;
- black-box enginectl, package, identity, mutation, and performance evidence;
- `sdk/README.md`; and
- this issue specification.

Do not change engine DSP, session grammar, content-identity rules, decoder implementation, C ABI,
browser host, runtime dependency graph, or user-owned package-manifest changes in the original
worktree.

## Objective gates

1. A leaf FLAC directory produces canonical TOML identical to an independently constructed public
   SDK builder model, and the embedded engine accepts it.
2. Corpus FLAC encodings of the same canonical PCM with different block layouts yield the same
   content identity; hashing container bytes, swapping stereo channels, or changing one PCM LSB
   turns the gate red.
3. PCM16/PCM24, mono/stereo, spaces, leading dashes, control characters, Unicode, case variants,
   normalization collisions, and long names pass through the built executable deterministically.
4. Corrupt/truncated FLAC, a changed sample, mixed rates, unsupported depth/channel count,
   missing/mutated decoder, nested collection, empty directory, symlink/unsupported entry,
   oversized input, and source mutation during read fail typed and publish nothing.
5. The supplied collection root refuses immediately and reports exactly four sorted groups; each
   of its four leaf directories produces one accepted session with the measured source count and
   shared per-song shape.
6. Every generated session contains exactly one `main` output and one unity `post_matrix` route per
   track. No filename-derived audio decision appears.
7. Existing request mode, raw stdout, no-clobber, overwrite, broken-pipe, post-publication failure,
   help/version, engine diagnostics, and all existing SDK surfaces remain compatible.
8. An extracted npm tarball succeeds with Cargo, ffmpeg, ffprobe, and repository discovery
   unavailable, and refuses decoder/engine asset mutations before trusting them.
9. Strict types, generated/deletion gates, complete headless SDK tests, package/tarball smoke,
   routing checker/mutations, and proportional SDK CI pass.
10. Publication order remains decode/hash every source -> builder -> embedded-engine acceptance ->
    atomic publication -> receipt. An acknowledgement/receipt never precedes a dropped source or
    failed publication.
11. Structural performance is one process, one decoder compilation/instance, one engine asset
    compilation, and one read/decode/hash pass per stem, with no subprocess/network/cache/daemon.
12. Sol-medium implementation receives fresh Sol-high adversarial PASS before upstream delivery.

## Mutation requirements

- Hash FLAC bytes instead of PCM; swap channel order or one PCM LSB.
- Trust STREAMINFO without verified EOF.
- Compile/instantiate the decoder once per file.
- Recurse into child directories.
- Route from a different tap or omit one route.
- Remove decoder digest verification.

Each mutation must make its named gate red; production is restored byte-for-byte afterward.

## Performance evidence

Freeze the extracted built executable and real `wide-open` leaf: 8 files, 160,668,363 compressed
bytes, 407,956,416 canonical PCM bytes, plus exact sorted filename/container-digest manifest,
commit, macOS/Apple-Silicon host, Node version, and filesystem. Run exactly one warmup and two
measured rounds without retry or tuning. Record wall time, time to first useful stdout byte, peak
RSS, compressed/canonical bytes, and TOML bytes. Separately measure `--help` and the collection-root
preflight refusal. These observations are descriptive and do not invent a release budget.

## Non-goals and successors

This slice does not add recursive/batch atomic output, persistent source-resolver manifests,
request scaffolding for later effect edits, a content cache, WAVE/RF64 import, or live
inspect/patch/control commands. Record those as bounded successors only if real dogfood evidence
shows they materially improve the agent workflow. In particular, a cache must be correctness-keyed
by full transport content plus decoder-contract identity, never size/mtime alone.

## Rollout

1. Create matching local/GitHub issue #333 and checkpoint this Sol-high-approved brief.
2. Implement attempt 1 with Sol medium, keeping the implementation tranche isolated.
3. Commit the coherent green tranche and obtain fresh adversarial Sol-high review.
4. Make at most two Sol revisions if needed; after attempt 3 HOLD, stop and rescope.
5. Run proportional local gates and the frozen one-warmup/two-round measurement once.
6. Push the CI-conscious feature branch once, open one SDK-only PR, and verify all three aggregate
   contexts report while only SDK heavy work runs. This also supplies issue #331's real SDK-only PR
   observation.
7. Merge only after PASS and green required checks, synchronize issue evidence upstream, then
   dogfood the extracted package on all four supplied song leaves.

## Evidence

Sol-high briefing on 2026-09-03 independently inspected the current CLI, SDK packaging, session
model, canonical-PCM contract, existing FLAC decoder, and all 30 supplied stems. It approved this
leaf-directory contract as the smallest closable product slice, ruled that the collection must
produce four sessions rather than one simultaneous mix, froze machine I/O/resource/publication and
performance gates, and separated batch mode, manifests, caching, WAVE import, scaffolding, and live
control as potential successors. Implementation and adversarial evidence will be appended without
weakening these gates.

### Attempt 1 — Sol-medium implementation evidence

Checkpoint `ad240967` added the exclusive `--stems` path, raw-byte-sorted nonrecursive discovery,
deterministic digest-suffixed IDs, packaged pinned decoder loading, single-pass canonical PCM
hashing, public-builder construction, embedded-engine validation, stems receipts, and decoder
artifact staging. Focused qualification passed: the built enginectl suite was 18/18, full SDK
headless was 129 pass / 0 fail / one platform capability skip, strict types and deletion policy
passed, the FLAC artifact/provenance/vector gate passed against CI artifact 9860928419 from browser
run 33666706481, and the extracted 69-file npm tarball imported a FLAC fixture with an empty
`PATH` and no repository discovery. Exact scope classified `sdk`.

The implementation corrected the frozen `wide-open` compressed corpus total from a briefing typo
to 160,668,363 bytes; the canonical total remains 407,956,416 bytes. A local Darwin arm64/Rust
1.97.1 rebuild produced digest-different decoder and AudioWorklet Wasm from the Linux-qualified
pins. No repin occurred: local qualification used the exact non-expired CI artifact. Cross-host
artifact reproducibility is recorded as a tooling successor rather than weakening this product
slice.

### Attempt 1 — Sol-high adversarial HOLD

Fresh Sol-high review held exact checkpoint `ad240967` on one destructive output conflict and one
non-discriminating structural assertion. In an isolated extracted package, a valid source at
`stems/session.flac` was passed as both the discovered input and `--output` with `--overwrite`.
The command exited 0, emitted a success receipt, and replaced the FLAC with 1,255 bytes of TOML.
A previously absent output directly inside the leaf also succeeded once, then caused the next run
to reject the CLI's own non-FLAC output. Attempt 2 must refuse any file output whose physical parent
is the physical stems leaf, including symlink/case aliases, before decoder compilation or a source
read. Regression cases must cover both existing-source overwrite and a new in-leaf output with the
decoder made unavailable to prove ordering.

The structure test counted two `WebAssembly.compile` calls but did not count instances, while gate
11 requires exactly one decoder and one engine compilation/instance. Attempt 2 must count
`WebAssembly.instantiate` as well and prove a per-file-instantiation mutation makes the assertion
red.

All other adversarial work passed: exact request-mode bytes matched the base across valid and
invalid cases; actual collection refusal reported the four song groups; generated unsupported
rates, channels, depth, and mixed-rate inputs refused typed; and isolated mutations for container
hashing, one PCM LSB, early EOF, per-file decoder compilation, recursion, wrong route tap, and
removed decoder verification all turned their named gates red. Raw-byte/O_NOFOLLOW/read-once/u32,
publication, package isolation, and no-network review found no second implementation defect.
Attempt 1 remains **HOLD**.

### Attempt 2 — Sol-medium correction evidence

Attempt 2 compares the file output's physical parent with the physical stems leaf by filesystem
`(device, inode)` identity before importing discovery or loading the decoder. It therefore refuses
an existing source selected with `--overwrite`, a new destination in the leaf, a symlink spelling
of that parent, and—where the filesystem exposes one—a differently cased alias with exit 5 and
`effect: "not_applied"`. The regression makes the packaged decoder unavailable during all cases;
they still report `output.publish`, the source digest is unchanged, and neither new destination
exists afterward. A missing or unreadable stems path remains the established exit-3 input refusal
rather than being misclassified by this preflight. Request mode returns before the new comparison
and is unchanged.

The structural black-box probe now wraps both `WebAssembly.compile` and
`WebAssembly.instantiate`. Three stems report exactly three file-handle reads, two compiles, and two
instances: one decoder plus one embedded engine. In isolated mutation evidence, moving decoder
loading into the stem loop made the same probe fail with `compiles: 4, instantiates: 4` against the
required `2, 2`; production was restored before qualification. Final focused qualification is
green: the built-executable suite is 19/19 PASS; the extracted 69-file tarball imports a real FLAC
with an empty `PATH`; strict SDK types and deletion policy pass; generated assets/modules are
current; and the static CI routing contract passes. The exact attempt-2 scope is this evidence
record, `sdk/src/enginectl.ts`, and `sdk/test/enginectl-cli.mjs`; fresh Sol-high review remains
required.

Sol-medium attempt 1 on 2026-09-03 implemented the additive `--stems` path, raw-byte-sorted leaf
discovery, deterministic collision/truncation IDs, single-instance packaged decoding, incremental
canonical-PCM hashing, source-change detection, public-builder construction, embedded-engine
acceptance, and stems-only receipts. Package staging now carries the four pinned decoder files and
accepts an explicit already-qualified decoder artifact directory so repeated local/CI packaging can
avoid another Rust build. The request parser and stems importer are loaded only on their selected
paths; help/version do not initialize either workload. The independently frozen `wide-open`
compressed-byte total was corrected from `107,668,363` to `160,668,363`; its canonical-byte total
remains `407,956,416`.

Focused attempt-1 evidence against browser-run `33666706481` decoder artifact `9860928419`
(`a9fc3301cb6f290909e165fd5d21d7ded5fb3535d8c41472c93beed66173b65e`): strict SDK types PASS;
decoder provenance/vectors/red mutation/pump PASS; enginectl built-executable suite 18/18 PASS;
complete headless SDK suite 129 PASS, 0 FAIL, 1 filesystem-capability skip; SDK deletion, generated,
workspace, and CI-routing checks PASS; extracted npm tarball smoke PASS with an empty `PATH`, one
real FLAC import, and no runtime dependencies. The CLI suite proves public-builder TOML identity,
engine acceptance, PCM identity across two FLAC block layouts, mono/stereo and PCM16/PCM24,
byte-sorted hostile/colliding/long filenames, collection preflight without decoder loading,
empty/non-FLAC/symlink/truncated refusals, no-clobber preflight, packaged decoder mutations,
one stem read each, exactly two Wasm compiles (one decoder plus one engine), and changed-source
non-publication. Existing request-mode and publication tests remain green.

The real 44.1 kHz four-leaf dogfood and frozen one-warmup/two-measurement performance record remain
pending. A Darwin arm64 rebuild with the pinned Rust 1.97.1 produced byte-identical-size but
digest-different decoder and AudioWorklet Wasm; no pin was changed, and all attempt evidence used
the qualified CI bytes. Cross-host artifact reproducibility is a separate delivery-tooling risk
rather than evidence for this slice.

### Attempt 2 — Sol-high adversarial PASS

Fresh Sol-high review returned **PASS** on exact checkpoint `86ae00c0`. In an independent extracted
build with the decoder made unavailable, the reviewer reproduced the former existing-source,
new-in-leaf, and symlink-alias cases; every case refused before decoder work with exit 5, empty
stdout, `output.publish`, and `effect: "not_applied"`. The source SHA-256 remained unchanged and
both new destinations remained absent. Physical `(dev, ino)` comparison correctly follows aliases,
and candidate/base request-mode bytes and statuses matched for valid raw output, invalid JSON,
invalid UTF-8, an unknown flag, and a refused request shape.

The reviewer independently moved decoder loading into the three-file loop. The structural gate
failed exactly at `{ compiles: 4, instantiates: 4 }` versus `{ 2, 2 }`, while stem reads remained
three. The exact package gate passed 19/19 CLI tests and standalone tarball smoke; strict types,
generated/deletion checks, FLAC provenance/vectors, routing checker/mutations, and full headless
129 pass / 0 fail / one Darwin capability skip were green. Attempt-2 scope is exactly three files,
the cumulative range classifies `sdk`, and the shared worktree remained clean. The frozen
performance measurement, real four-leaf dogfood, and remote SDK-only PR rollout remain rollout
steps, not review findings.

## Frozen performance record

The benchmark ran exactly once after Sol-high PASS, with one warmup and two measured rounds and no
retry or tuning. The subject was the extracted 69-file `@misofm/engine@0.1.0` package built from
checkpoint `86ae00c0`: tarball SHA-256
`c6febfa65a4383c8d98c3cea4421cea14713dae11684dd1f2829c05fac39b467`, enginectl SHA-256
`2fc19994f2bf3bdfa179770114c9167dc7718a4d3d8c2f551dc0a06fba19ba39`, pinned decoder
`a9fc3301cb6f290909e165fd5d21d7ded5fb3535d8c41472c93beed66173b65e`, and local qualified engine
asset `7a6f3d544dc9a65e5a89ea92833e2f6d62b22b44c33df09c14348cb13ee1950f`.

The host was a 32 GB Apple M5 MacBook Pro (`Mac17,2`), Darwin arm64, APFS, Node v26.8.1. Child
`PATH` was empty. The `wide-open` corpus was exactly 8 FLACs, 160,668,363 compressed bytes and
407,956,416 canonical bytes. Its raw transport manifest, sorted by filename bytes, was:

| file | bytes | container SHA-256 |
| --- | ---: | --- |
| `BASS.flac` | 19,381,374 | `b26c9ef76ad937aa778253cdf07e08dbfb0a5338ea8a88382c78c79fcd26b108` |
| `BV_S.flac` | 22,194,776 | `118fd5d82d0610a1be518ba08dc68ddb95e12601635ec7313b64e023745d35fb` |
| `DRUMS.flac` | 25,085,323 | `c808d71fecb540581d610f094229c99631b4eb6a886f2fc02ea2455802f70594` |
| `FX.flac` | 11,626,669 | `85f1011553ec01f4c938a244f67dd63105e038c7742b149ff07057f60445cccb` |
| `LEAD VOX.flac` | 26,749,440 | `f619570ffe223969d621e31ed7d0170a568baa1ea01fc475dd082adb5028f587` |
| `OUTRO FX.flac` | 7,103,280 | `8b392f1355a8815a376108dc14f595d0cfb073c421b381dadb448553b4a784b2` |
| `SYNTH.flac` | 26,580,304 | `d381e6518e95c2662986101ce11dfbba1edca24bb1d35251e85b67049380c96a` |
| `VOCAL FX.flac` | 21,947,197 | `ef74035c58dd50c76857568358b97b26c74358ad5145650b85712c52e4b7a0e6` |

| invocation | wall ms | first useful output ms | maximum RSS bytes |
| --- | ---: | ---: | ---: |
| help | 31.794 | 30.043 stdout | 46,841,856 |
| collection preflight refusal | 30.339 | 28.665 stderr | 48,103,424 |
| warmup | 1,663.489 | 1,657.981 stdout | 334,577,664 |
| measured 1 | 1,600.794 | 1,596.088 stdout | 305,643,520 |
| measured 2 | 1,601.134 | 1,596.448 stdout | 305,610,752 |

All three workload runs produced byte-identical 7,642-byte canonical TOML with SHA-256
`a6884dc3dedbfbbe4ae1b034a71a565fedcaf379af241ea85922e653a5c1b92c` and eight source mappings.
First stdout intentionally coincides with completion: the CLI does not acknowledge until all FLACs
reach verified EOF, all canonical identities are complete, the builder and embedded engine accept,
and the file is atomically published. The black-box structural gate separately proves one process,
one decoder and engine compile/instance, and exactly one file-handle read per stem. These numbers
are descriptive; no release budget was invented and no optimization was performed from them.

## Remote rollout and real four-song dogfood

The CI-conscious feature branch was pushed once at exact head `b89a7fb2`; the ordinary branch push
created zero workflow runs. Pull request #334 targeted exact main base `db56d1b1`. Its SDK
qualification run <https://github.com/misofm/engine/actions/runs/33675688367> passed the substantive
package/generated/headless job in 2m53s and the required `SDK qualification` aggregate. The engine
run <https://github.com/misofm/engine/actions/runs/33675688483> and browser run
<https://github.com/misofm/engine/actions/runs/33675688390> reported their stable required
aggregates while every heavy engine and browser job was skipped. All three required contexts were
green, and PR #334 merged once as `49c153f7` without another feature-branch update. The merge push
created only SDK run <https://github.com/misofm/engine/actions/runs/33676075580>, whose substantive
job and aggregate passed in 2m54s without retry.

The reviewed extracted 69-file package then built all four supplied song leaves with an empty
child `PATH`, absolute input/output arguments, no repository discovery, and a sibling output
directory at `/Users/bl/Desktop/between-the-doors/between-the-doors-sessions`:

| session | sources/tracks/routes | frames per stem | TOML bytes | TOML SHA-256 |
| --- | ---: | ---: | ---: | --- |
| `ghost` | 8/8/8 | 6,207,923 | 7,638 | `695ae935659cab266c6dd35b8212243d09832565992978c7683c1c25821ec589` |
| `play-me` | 6/6/6 | 7,717,500 | 5,800 | `90c3094bff4c93f914330c62402cf01696496a1d08a971f96b51ac70c92e4ab3` |
| `war` | 8/8/8 | 6,761,475 | 7,656 | `b49020b5c671215bd70917fbee77fd3d4403bf748666027895bdb51a2691bdad` |
| `wide-open` | 8/8/8 | 8,499,092 | 7,642 | `a6884dc3dedbfbbe4ae1b034a71a565fedcaf379af241ea85922e653a5c1b92c` |

Every file is 44.1 kHz/stereo PCM24, has one `main` output, matching source/track identities, and
one unity `post_matrix` route per track. The native `session_validator` independently passed all
four real stages—TOML grammar, typed model, session compilation, and builtins preparation—for each
document. Fresh Sol-high dogfood review also booted every byte stream through public
`createOfflineEngine()` and observed `ready` with matching rate, quantum, source count, and track
count.

The supplied collection root independently refused before decode with exit 3, empty stdout,
`effect: "not_applied"`, code `stems.collection`, and exactly
`["ghost","play-me","war","wide-open"]`. Before/after SHA-256 manifests for all 30 source FLACs
matched exactly; no `.enginectl-*` or session file appeared in a source leaf. This proves the real
dogfood portion of gates 5 and 10 without inferring playback/source resolution from session
authoring. Sol-high found no correctness or measured-performance reason to add recursive batching
or a cache. Its three bounded receipt/help durability findings became successor issue #335 rather
than reopening this accepted importer. Issue #333 is complete once this evidence is upstream and
the remote body/state are synchronized.
