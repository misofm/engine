# Issue #241 follow-up — canonical source identities for the browser-v1 fixtures

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

## Reproduction

```python
import hashlib, struct

def identity(samples):                       # interleaved f32 LE
    return hashlib.sha256(b"".join(struct.pack("<f", s) for s in samples)).hexdigest()

ramp = []
for base, step in ((0.125, 0.0009765625), (-0.25, 0.00048828125)):
    for i in range(128):
        ramp += [struct.unpack("<f", struct.pack("<f", base + step * i))[0], 0.0]

identity(ramp)          # a7d052a7f6b3b881f4bde6090d87c4226d39e62010e9b6038088bb28b8742949
identity([0.25] * 4096) # 680aca77ba6b819a4489730f3e42f69ba9f6d7a5921e748a8a46eb1974d0867c
identity([0.5] * 4096)  # 66e39e41bccc0a57ae90a77b426f4075e81ba877b0653c3aabe0a9e00762769c
```

## Why no render digest moves

`content` is grammar-checked only — `crates/miso-engine-session/src/validate.rs`
`valid_source_content_identity` tests `^sha256:[0-9a-f]{64}$` and nothing reads the value
afterwards. The browser and raw-Wasm legs both push PCM through
`miso_engine_web_v1_source_submit`; no resolver ever fetches by identity here. Each edit also
substitutes 64 hex characters for 64 hex characters in place, so
`hosts/miso-engine-host-web/tests/browser-v1/session.toml` stays exactly **1,265 bytes** and the
`sessionTomlBytes = 1265` row of `expected.json` — the one resource row that is a byte count of
this document — is unchanged. `direct-oracle.mjs` re-derives and re-asserts all three PCM digests
(`pcmF32leSha256`, `nativeCommandTimelinePcmF32leSha256`,
`nativeObservationPcmF32leSha256`) against `expected.json` on every `--check`, and they pass.

## Frozen pins in the gate

`scripts/web-audioworklet-browser-correctness.py` asserted the pre-#241 spelling
`length_samples = N`, which no longer exists in any of the three documents — that is the red gate
on `main`. The pins are now the post-#241 facts: `sample_rate_hz`, `quantum_frames`, the source
row's `channels = 2, bit_depth = "32f", frames = N`, the source row's `content = "sha256:…"`, and
the unchanged `effect_id` rows. Shape and identity are separate strings so each goes red alone, and
re-declaring the identity session's digest on the 2,048-frame command session — the exact `04d291dd`
defect — is now refused.

## Out of scope, same root cause

The three `hosts/miso-engine-host-web/qualification/*.toml` documents still carry name-minted
identities (`689c8244…`, `4ce4a7a7…`, `1e28ad43…`). They belong to the separate browser
qualification harness, are not read by any `scripts/sweep.sh` row, and their console (16,640-frame)
and stall (5,120-frame) sources need their own derivations. For the record, the qualification
observation session feeds `OBSERVATION_LEVEL = 0.5` over the same 2,048-frame stereo declaration
(`qualification/qualification.js:23,414-415`), so its truthful identity is the *same*
`66e39e41…` derived in §3 — legitimately equal, because the bytes and the declaration are equal.
