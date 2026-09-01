# Issue #241 follow-up — canonical source identities for the browser-v1 and qualification fixtures

`04d291dd` (`Implement canonical PCM source schema`) replaced the nested
`content = { identity, locator }` / `mapping.region` source shape with the flat
`{ id, content, channels, bit_depth, frames }` row. In the web browser fixtures the migration
minted each new `content` value from the **old locator name**, not from any audio:

| document | old identity | minted value | `sha256(name)` |
| --- | --- | --- | --- |
| `tests/browser-v1/session.toml` | `sha256:web-browser-one-track` | `923b494c…fcfdf` | `sha256(b"web-browser-one-track")` |
| `tests/browser-v1/command-session.toml` | `sha256:web-browser-one-track` | `923b494c…fcfdf` | same |
| `tests/browser-v1/observation-session.toml` | `sha256:web-browser-observation` | `689c8244…3378` | `sha256(b"web-browser-observation")` |
| `qualification/observation-session.toml` | `sha256:web-browser-observation` | `689c8244…3378` | same |
| `qualification/console-session.toml` | `sha256:web-browser-console` | `4ce4a7a7…5a9a` | `sha256(b"web-browser-console")` |
| `qualification/stall-session.toml` | `sha256:web-browser-stall-ring` | `1e28ad43…907d` | `sha256(b"web-browser-stall-ring")` |

Each minted value reproduces exactly as `hashlib.sha256(<name>.encode()).hexdigest()`. That is a
digest of an ASCII locator string, never of canonical PCM, so none of the six satisfied
`docs/STEM_IDENTITY_V1.md`. Because two documents shared the locator `web-browser-one-track`, they
also received the *same* identity under two different `frames` (256 and 2048) — an impossible pair:
the contract fixes the preimage length at `frames * channels * bytes_per_sample`, so 2,048 bytes
and 16,384 bytes cannot share a SHA-256 pin.

Sections 1–3 are the `tests/browser-v1` half, repaired in #271. Sections 4–6 are the
`hosts/host-web/qualification` half, repaired in #272; all six documents now declare
the canonical digest of the PCM their harness actually feeds.

## Canonical serialization used here

`docs/STEM_IDENTITY_V1.md` §"Canonical serialization": samples only, interleaved frame-major
(frame 0 ch 0, frame 0 ch 1, frame 1 ch 0, …), `bit_depth = "32f"` meaning each sample is its raw
four-byte IEEE-754 binary32 pattern little-endian, preimage length exactly
`frames * channels * 4`, digest SHA-256.

## 1. `tests/browser-v1/session.toml` — 256 frames

The fed content is `tests/browser-v1/source.json`, replayed identically by
`browser-correctness.js::blockPlanes` (browser leg) and `direct-oracle.mjs::blockPlanes` (raw-Wasm
leg). Both compute, per 128-frame block,

```
left[i]  = leftBase + leftStep * i        (i = 0 .. 127)
right[i] = 0
```

with the two declared blocks `(leftBase, leftStep) = (0.125, 0.0009765625)` and
`(-0.25, 0.00048828125)`. Both steps are exact powers of two (`2^-10`, `2^-11`) and both bases are
exact, so every value is exactly representable in binary32 and the f64→f32 store the JS typed
arrays perform is the identity map. The interleaved preimage is
`256 * 2 * 4 = 2048` bytes, and

```
sha256 = a7d052a7f6b3b881f4bde6090d87c4226d39e62010e9b6038088bb28b8742949
```

This is the same 256 frames the raw-Wasm oracle already pins by shape:
`direct-oracle.mjs` asserts `miso_engine_web_v1_source_frames(handle, 0) == 256n`.

## 2. `tests/browser-v1/command-session.toml` — 2048 frames

`direct-oracle.mjs::runCommandTimeline`'s `feed` writes `pcm.fill(0.25)` over
`QUANTUM * 2` floats, i.e. constant `0.25` on both channels, into every block of the timeline. The
declaration is 2,048 frames stereo, so the preimage is `2048 * 2 * 4 = 16384` bytes of
`0x3E800000` little-endian:

```
sha256 = 680aca77ba6b819a4489730f3e42f69ba9f6d7a5921e748a8a46eb1974d0867c
```

## 3. `tests/browser-v1/observation-session.toml` — 2048 frames

`direct-oracle.mjs::runObservationTimeline`'s `feed` writes `.fill(0.5)` over `QUANTUM * 2` floats
— constant `0.5` on both channels, which is what puts the fixture well above the compressor's
`-30 dBFS` threshold so an armed tap has a real reduction to publish. Preimage
`2048 * 2 * 4 = 16384` bytes of `0x3F000000` little-endian:

```
sha256 = 66e39e41bccc0a57ae90a77b426f4075e81ba877b0653c3aabe0a9e00762769c
```

## Why no render digest moves, browser-v1 path

`content` is grammar-checked only — `crates/session/src/validate.rs`
`valid_source_content_identity` tests `^sha256:[0-9a-f]{64}$` and nothing reads the value
afterwards. The browser and raw-Wasm legs both push PCM through
`miso_engine_web_v1_source_submit`; no resolver ever fetches by identity here. Each edit also
substitutes 64 hex characters for 64 hex characters in place, so
`hosts/host-web/tests/browser-v1/session.toml` stays exactly **1,265 bytes** and the
`sessionTomlBytes = 1265` row of `expected.json` — the one resource row that is a byte count of
this document — is unchanged. `direct-oracle.mjs` re-derives and re-asserts all three PCM digests
(`pcmF32leSha256`, `nativeCommandTimelinePcmF32leSha256`,
`nativeObservationPcmF32leSha256`) against `expected.json` on every `--check`, and they pass.

## Frozen pins in the browser-v1 gate

`scripts/web-audioworklet-browser-correctness.py` asserted the pre-#241 spelling
`length_samples = N`, which no longer exists in any of the three documents — that is the red gate
on `main`. The pins are now the post-#241 facts: `sample_rate_hz`, `quantum_frames`, the source
row's `channels = 2, bit_depth = "32f", frames = N`, the source row's `content = "sha256:…"`, and
the unchanged `effect_id` rows. Shape and identity are separate strings so each goes red alone, and
re-declaring the identity session's digest on the 2,048-frame command session — the exact `04d291dd`
defect — is now refused.

## The qualification half (#272)

The three `hosts/host-web/qualification/*.toml` documents belong to the browser
qualification harness, not to `tests/browser-v1`. They are fetched by
`qualification/qualification.js::runQualification` and handed to the host as session bytes; the PCM
they describe is submitted block by block through `host.submitSource`, one 128-frame quantum at a
time, `startFrame = block * quantum_frames`, with `endOfRegion` on the last block. The fed region is
therefore exactly `blocks * 128` frames and covers the whole declared `frames` — nothing is fed
twice and nothing is left unfed, which is what makes a single digest over the generator the
document's truthful identity.

One trap is worth naming. `qualification.js::pcmDigest` — the function behind the harness's
`expectedDigest`/`renderedDigest` render pins — serializes **planar** (all of channel 0, then all of
channel 1). That is a different question and a different answer. `STEM_IDENTITY_V1` identity is
**interleaved** frame-major. The two layouts agree only for a mono or a constant-across-channels
source, so reusing `pcmDigest` here would have produced a right-looking wrong number on the console
and stall rows and a coincidentally right one on the observation row.

## 4. `qualification/console-session.toml` — 16,640 frames

`runConsoleQualification` feeds `CONSOLE_BLOCKS = 130` blocks (`130 * 128 = 16640` frames, one full
128-block telemetry window plus slack) from `qualification.js::sourcePlanes(block)`. With the global
frame index `n = block * 128 + frame`, that generator is

```
left[n]  =  (n + 1) / 8192        (= (n + 1) * 2^-13)
right[n] = -(n + 1) / 16384       (= -(n + 1) * 2^-14)
```

Both divisors are powers of two and `n + 1 <= 16640 < 2^24`, so every value is a binary32 with an
exact 15-bit significand: the f64 arithmetic JavaScript performs and the f32 store into the
`Float32Array` are both exact, and no host rounding mode can move a bit. The interleaved preimage is
`16640 * 2 * 4 = 133120` bytes, and

```
sha256 = 8e7350ab6d22bf4a3e9357474ae4622a55d630dfc020f27010a69a72965d51cf
```

The console row's own gate is unaffected: its `expectedDigest` is the *rendered* output after the
matrix retarget halves the left coefficient, which is a function of this PCM, not of this identity.

## 5. `qualification/observation-session.toml` — 2,048 frames

`runObservationRun` feeds `OBSERVATION_BLOCKS = 16` blocks (`16 * 128 = 2048` frames) from
`qualification.js::observationPlanes`, which fills both channels with
`OBSERVATION_LEVEL = 0.5` — well above the session's `-30 dBFS` compressor threshold, so an armed
tap has a real reduction to publish. Preimage `2048 * 2 * 4 = 16384` bytes of `0x3F000000`
little-endian:

```
sha256 = 66e39e41bccc0a57ae90a77b426f4075e81ba877b0653c3aabe0a9e00762769c
```

This is the **same** digest derived in §3 for `tests/browser-v1/observation-session.toml`, and that
is correct, not a collision: equal bytes under an equal declaration are one identity. The two
documents feed 2,048 stereo frames of constant `0.5` from different code paths
(`direct-oracle.mjs::runObservationTimeline` and `qualification.js::observationPlanes`) and arrive
at the same canonical serialization. `STEM_IDENTITY_V1` §"Declaration and interpretation" says so
outright: reusing canonical bytes under a declaration in another document is a coherent
reinterpretation. What is impossible is the pairing named in the introduction — one digest under
two different `frames`, which would need one SHA-256 to cover two preimage lengths — and that is not
what happens here.

## 6. `qualification/stall-session.toml` — 5,120 frames

`runStallQualification` feeds `STALL_FRAMES / QUANTUM_FRAMES = 40` blocks (`40 * 128 = 5120` frames,
the default ring) from the same `sourcePlanes(block)` generator as §4, so it is the §4 formula
truncated to `n = 0 .. 5119`. Preimage `5120 * 2 * 4 = 40960` bytes:

```
sha256 = 938d3a47555b54df6321fc9e1b40c9581d316870f70fa17be8ea40cc154436d7
```

It is *not* a prefix relation on the digest — SHA-256 of a prefix shares nothing with SHA-256 of the
whole — which is exactly why the console and stall documents, fed by one generator, must still
carry two different identities. Sharing one would be the #241 defect again in a new spelling.

## Reproduction — all five distinct identities

```python
import hashlib, struct

def identity(samples):                       # interleaved f32 LE
    return hashlib.sha256(b"".join(struct.pack("<f", s) for s in samples)).hexdigest()

def f32(x):
    return struct.unpack("<f", struct.pack("<f", x))[0]

def source_planes(frames):                   # qualification.js::sourcePlanes, interleaved
    out = []
    for n in range(frames):
        out += [f32((n + 1) / 8192), f32(-(n + 1) / 16384)]
    return out

ramp = []
for base, step in ((0.125, 0.0009765625), (-0.25, 0.00048828125)):
    for i in range(128):
        ramp += [f32(base + step * i), 0.0]

identity(ramp) # a7d052a7f6b3b881f4bde6090d87c4226d39e62010e9b6038088bb28b8742949
identity([0.25] * 4096) # 680aca77ba6b819a4489730f3e42f69ba9f6d7a5921e748a8a46eb1974d0867c
identity([0.5] * 4096) # 66e39e41bccc0a57ae90a77b426f4075e81ba877b0653c3aabe0a9e00762769c
identity(source_planes(130 * 128)) # 8e7350ab6d22bf4a3e9357474ae4622a55d630dfc020f27010a69a72965d51cf
identity(source_planes(40 * 128)) # 938d3a47555b54df6321fc9e1b40c9581d316870f70fa17be8ea40cc154436d7
```

`hosts/host-web/qualification/session-identities.mjs` reproduces the last two (and the
observation row) independently in Node, from the harness's own exported generators, and agrees. So
does the repository's own reference oracle, on the canonical preimages written out of the Python
above:

```sh
cargo run --locked -p stem-hasher -- raw \
  --input console.pcm --channels 2 --bit-depth 32f --frames 16640   # 8e7350ab…
cargo run --locked -p stem-hasher -- raw \
  --input stall.pcm --channels 2 --bit-depth 32f --frames 5120      # 938d3a47…
cargo run --locked -p stem-hasher -- raw \
  --input observation.pcm --channels 2 --bit-depth 32f --frames 2048 # 66e39e41…
```

Three independent implementations — Python, the Node check, and the Rust oracle — agree on all
three.

## The qualification gate (#272)

The `tests/browser-v1` half is pinned by frozen strings in
`scripts/web-audioworklet-browser-correctness.py`. The qualification half had no reader at all:
`content` never leaves the session document on this leg, so a name-minted value stayed green
forever. `session-identities.mjs` closes it, and `run.mjs::main` calls it before a browser is
launched, so `npm run qualify` cannot seal a matrix over a false identity.

It **derives** rather than pins. For each row of `qualification.js::qualificationSessionSources` it
walks the exported generator (`sourcePlanes` / `observationPlanes` — the same functions the browser
feeds), serializes per `STEM_IDENTITY_V1`, and requires the document to carry the whole source row
verbatim, shape and identity together:

```
{ id = "console-source", content = "sha256:…", channels = 2, bit_depth = "32f", frames = 16640 },
```

A pinned hex string would need editing in step with any generator change, and "edited out of step"
is precisely the pre-#272 state. Derived, a changed generator moves the expected identity and the
unchanged document goes red on its own. The check also refuses a document carrying more than one
`content` identity, so a truthful row cannot sit beside a stale one, and it carries its own
flipped-digit red proof so the comparison cannot be loosened into a vacuous pass.

## Why no render digest moves, qualification path

For the qualification leg specifically: `runQualification` fetches the three documents and passes
their bytes to `createMisoAudioWorkletHost` as `sessionToml`. Nothing in
`hosts/host-web/web/miso-engine-v2-audio-worklet-host.js` computes or exposes a digest,
and `host.sessionMap()` returns track and tap structure only — the harness reads `map.tracks` and
`map.metersAttached` and nothing else. Engine-side, `content` reaches
`crates/session/src/validate.rs::valid_source_content_identity`, a
`^sha256:[0-9a-f]{64}$` grammar test, and `visit.rs`, which re-serializes it into the canonical
document form; no resolver ever fetches bytes by identity on this leg, and the PCM arrives instead
through `miso_engine_web_v1_source_submit`. Every digest the qualification gates compare
(`corpus.nativeDigest`, `console.expectedDigest`/`renderedDigest`, `stall.expectedDigest`/
`renderedDigest`, the observation rows' `identicalAudio`) is computed from rendered or fed PCM, so
none of them can see this edit. Each fix also substitutes 64 hex characters for 64, so
`console-session.toml` stays 1,265 bytes, `stall-session.toml` 1,263, and
`observation-session.toml` 2,402 — no resource estimate or byte pin moves either.

## Not this class — the qualification harness's boot refusal (#281)

While #272 was in flight the qualification harness was found to fail at boot with `miso.error.v1`,
`requestId 0`, `result 1` (issue #281), and the natural reading was that it was the last member of
this class: a stale document the six repairs above had missed. It is not. All four documents the
harness boots — the three above plus `tests/browser-v1/session.toml` — already carry the post-#241
flat source row, and their identities are the derived ones recorded here. The refusal came from the
*caller*: `qualification/qualification.js` still passed `createMisoAudioWorkletHost` the pre-#240
`{ quantumFrames, sessionToml, limits }` shape. See
`docs/derivations/281-qualification-harness-boot.md` for the audit that cleared the documents and
the derivation of the real cause. The #241 fallout class is closed at six documents.
