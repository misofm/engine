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
publication. Stems-mode receipts add input kind/path, derived session facts, and a sorted mapping
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

Fresh Sol-high adversarial review, the real 44.1 kHz four-leaf dogfood, named red mutations, and the
frozen one-warmup/two-measurement performance record remain pending. A Darwin arm64 rebuild with the
pinned Rust 1.97.1 produced byte-identical-size but digest-different decoder and AudioWorklet Wasm;
no pin was changed, and all attempt evidence used the qualified CI bytes. Cross-host artifact
reproducibility is a separate delivery-tooling risk for review rather than evidence for this slice.
