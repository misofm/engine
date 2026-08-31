# Issue #281 — the qualification harness's boot refusal, and the corpus-document audit that cleared

`hosts/host-web/qualification` failed at boot on unmodified `main` with
`miso.error.v1`, `requestId 0`, `result 1` — the corpus row, before any audio was rendered. #281
opened with the leading hypothesis that this was the last member of the #241-fallout class: stale
session documents carrying the pre-#241 source shape. **It is not.** Every document the harness
boots already conforms to the #241 schema. The refusal is a *caller* defect: the harness still
speaks the pre-#240 `createMisoAudioWorkletHost` argument shape.

This document records both halves — the audit that cleared the documents, and the derivation of the
real cause — because "we checked the documents and they were fine" is itself the finding that keeps
the next reader from re-opening the same hypothesis.

## 1. The reproduction

With the #280 artifact-set pin widened (five names → six), on this box:

```
$ npm run qualify -- --artifacts <build output> --browser chromium
Error: chromium: browser-execution: {"message":"corpus qualification failed: {
  \"error\":{\"tag\":\"miso.error.v1\",\"requestId\":0,\"result\":1},
  \"diagnostic\":{\"kind\":\"message\",\"message\":{\"tag\":\"miso.error.v1\",\"requestId\":0,\"result\":1}},
  \"globals\":{\"textEncoder\":\"undefined\",\"webAssembly\":\"object\",\"renderQuantumSize\":\"undefined\"}}"}
; console error: Failed to load resource: the server responded with a status of 404 (Not Found)
```

Byte-identical to the transcript in the issue. The 404 is chromium's `/favicon.ico`, as the issue
already noted.

Two features of that transcript matter. First, `result 1` is `invalidArgument` — not
`refusedDocument`; the browser ABI has no separate document code, which is exactly why the two
hypotheses look alike from outside. Second, the `diagnostic` leg — `diagnoseReady`, the harness's
own second opinion, which boots the same document through a raw `AudioWorkletNode` — returned the
*same* `result 1`. That is why the transcript points nowhere: both legs were stale in the same way,
so the instrument built to distinguish "your options are wrong" from "your document is wrong"
was itself refused before it could answer.

## 2. The document audit — all four already conform

`runQualification` fetches and boots exactly four documents:

| document | fetched as | booted by |
| --- | --- | --- |
| `tests/browser-v1/session.toml` | `/fixture/session.toml` | `runCorpusQualification` (twice, two fresh contexts) |
| `qualification/console-session.toml` | `/qualification/console-session.toml` | `runConsoleQualification` |
| `qualification/observation-session.toml` | `/qualification/observation-session.toml` | `runObservationRun` (armed and disarmed) |
| `qualification/stall-session.toml` | `/qualification/stall-session.toml` | `runStallQualification` and `typedUnsupportedAttestation` |

No session is authored inline: `qualification.js`, `run.mjs` and the corpus files contain no TOML
text at all — `tests/browser-v1/source.json` is a *PCM block description* (`startFrame`, `frames`,
`final`, `leftBase`, `leftStep`), not a session document, and `expected.json` is an oracle
transcript. So the boot surface is those four files and nothing else.

The post-#241 schema, read from the engine rather than from memory:

* root keys — `crates/session/src/visit.rs:85`: `schema_version`, `session_id`,
  `revision`, `sample_rate_hz`, `quantum_frames`, `render_profile`, `output_profile`, `sources`,
  `tracks`, `submixes`, `outputs`, `routes`, `automation`. **There is no `limits` key.**
* source row — `visit.rs:88` and `crates/session/src/model.rs:123`: exactly
  `{ id, content, channels, bit_depth, frames }`. **No per-source `sample_rate_hz`, no
  `start_frame`.**
* `bit_depth` tokens — `model.rs:138`: `Pcm16` / `Pcm24` / `F32` for `16` / `24` / `"32f"`.

All four documents match, key for key:

| document | `limits` line | per-source `sample_rate_hz`/`start_frame` | source row | `bit_depth` |
| --- | --- | --- | --- | --- |
| `tests/browser-v1/session.toml` | absent | absent | `{id, content, channels, bit_depth, frames}` | `"32f"` |
| `qualification/console-session.toml` | absent | absent | same | `"32f"` |
| `qualification/observation-session.toml` | absent | absent | same | `"32f"` |
| `qualification/stall-session.toml` | absent | absent | same | `"32f"` |

They were migrated in `04d291dd` (`Implement canonical PCM source schema`), the #241 commit itself.
The *identities* those rows carried were minted from locator names and not from PCM — the genuine
#241 fallout — but that half is already repaired: #271 for the `tests/browser-v1` half and #272 for
the `qualification` half, both derived in
`docs/derivations/241-browser-source-identities.md`. #272 also added
`qualification/session-identities.mjs`, which `run.mjs::main` runs before a browser launches; on
this branch it reports

```
session identities: 3 qualification documents declare their fed PCM
```

which is a live derive-not-pin re-derivation of all three qualification identities from the
harness's own generators, passing.

**So #281 required no new identity derivation and no document edit.** No byte of any session
document moved on this branch, which is also why no byte-length pin moves — see §5.

Positive confirmation rather than an argument from absence: with the caller migrated, the harness's
`diagnoseReady` leg now boots `tests/browser-v1/session.toml` through the real module and gets

```
{"tag":"miso.ready.v1","requestId":0,"result":0,"backend":"simd128",
 "resources":{"sampleRateHz":48000,"quantumFrames":128,"backend":1,"optionsBytes":"64",
  "statusBytes":"80","sessionTomlBytes":"1265","diagnosticBytes":"16384", …}}
```

The engine accepts the document, and the `sessionTomlBytes` it reports — **1265** — is exactly the
frozen resource pin in `tests/browser-v1/expected.json`. A stale-schema document could not have
produced a `miso.ready.v1` at all.

## 3. The real cause — the pre-#240 caller shape

`579fbce1` (`feat(#240): port browser worklet to atomic boot`) changed the host entry point's
argument shape:

| pre-#240 | post-#240 |
| --- | --- |
| `{ context, quantumFrames, sessionToml, limits, simd128ModuleUrl, workletModuleUrl }` | `{ context, document, options, simd128ModuleUrl, workletModuleUrl }` |
| `limits` — 27 fields: 21 capacity ceilings plus `sourceRingFrames` and the four console words | `options` — exactly 6: `sourceRingFrames`, `maximumMemoryBytes`, and the four console words |
| `quantumFrames` supplied by the caller | taken from `context.renderQuantumSize ?? 128` |

`579fbce1` updated `tests/browser-v1/browser-correctness.js`, `direct-oracle.mjs`,
`scripts/test-web-audioworklet.mjs` and `check-browser-expected-resources.py` — and did not touch
`qualification/qualification.js`, whose last edit before this branch was `94e87028` (#143, three
issues earlier). The qualification harness has therefore been unbootable since #240 merged.

Both guards are exact-field tests, so the mismatch is refused at the very top of
`createMisoAudioWorkletHost` (`miso-engine-v2-audio-worklet-host.js:829`):

```js
if (!hasExactFields(options, OPTION_FIELDS)         // ["context","document","options",…]
    …
    || !validBootOptions(options.options)           // hasExactFields(…, BOOT_OPTION_FIELDS)
    …) {
  throw webError(1);                                // miso.error.v1, requestId 0, result 1
}
```

`webError(1)` *is* the observed diagnostic, and it is thrown before the Wasm module is fetched, so
nothing downstream — not the artifact, not the document, not the browser — is implicated. The
worklet repeats the same guard on `processorOptions` (`miso-engine-v2-audio-worklet.js:173`,
`INIT_FIELDS = ["module","document","options"]`), which is the leg that refused `diagnoseReady`.

Six call sites carried the stale shape: `renderCorpusSegment`, `typedUnsupportedAttestation`,
`diagnoseReady`, `runConsoleQualification`, `runObservationRun`, `runStallQualification`. The
corpus row simply ran first.

## 4. The migration

`qualification.js::limits(...)` becomes `bootOptions(...)`, returning the six post-#240 words.
The mapping is total and mechanical:

| pre-#240 `limits` field | fate |
| --- | --- |
| `sourceRingFrames` | **kept**, same meaning, same per-row value at every call site |
| `consoleCommandQueueRecords`, `consoleMeterBlocks`, `consoleObservationTaps`, `consoleMasterTrackPlusOne` | **kept**, same values |
| `sessionTomlBytes`, `diagnosticBytes`, `sourceIdBytes`, `maximumSourceChannels`, `maximumAutomationSpansPerBlock`, `maximumTracks`, `maximumSources`, `maximumRoutes`, `maximumEffects`, `maximumGraphSessionPlusPlanBytes`, `maximumSourceTotalBytes`, `maximumSourceOverheadBytes`, `maximumEffectStateBytes`, `maximumEffectScratchBytes`, `maximumBuiltinRetainedBytes`, `maximumHostRetainedBytes`, `maximumNamedAllocationBytes`, `maximumMeterStreams`, `maximumMeterItems`, `maximumMeterBytes` | **deleted by #240** — the ceilings are the engine's now, not the caller's |
| — | `maximumMemoryBytes: 0n` **added**: "no caller-imposed memory ceiling", the same word `tests/browser-v1/browser-correctness.js::bootOptions` sends and the same one `direct-oracle.mjs::boot` writes at boot-options offset 24 |
| `quantumFrames` (a sibling argument, not a limit) | **deleted**; the context's `renderQuantumSize` is authoritative |

`sessionToml` → `document` at each call site, `limits:` → `options:`; the helper parameter becomes
`sessionDocument` and `runQualification`'s locals become `corpusDocument`, `consoleDocument`,
`observationDocument`, `stallDocument`. Renaming the *parameters* is not cosmetic and not optional:
`document` is a live global in the main realm, so an object literal using `{ document, … }`
shorthand beside a parameter still spelled `sessionToml` would have quietly handed the host the
page's `Document` object instead of the session bytes.

Not one boot-relevant *value* changes. Every call site keeps the `sourceRingFrames` it had —
`frames` for the corpus segments, `CONSOLE_FRAMES`, `OBSERVATION_FRAMES`, `DEFAULT_RING_FRAMES`
for stall — and the four console words are unchanged per row.

## 5. Why no digest and no byte-length pin moves

**No document changed.** `git diff` on this branch touches three files:
`qualification/server.mjs` (the #280 artifact-set pin), `qualification/qualification.js` (the
caller migration) and `qualification/run.mjs` (the #280 red proofs). No `.toml`, no `.json`
fixture, no `expected.json`. So:

* **Byte-length pins.** `sessionTomlBytes` is a function of a document's byte length; no document's
  byte length moved, so `tests/browser-v1/expected.json`'s frozen `"sessionTomlBytes": "1265"` and
  the `console`/`stall`/`observation` documents' 1,265 / 1,263 / 2,402 bytes (recorded in
  `241-browser-source-identities.md`) are all untouched. The live boot in §2 reports `1265`,
  agreeing with the pin. Nothing needed re-deriving.
* **Rendered digests.** Every digest the harness gates on is computed at run time from rendered or
  fed PCM: `console.expectedDigest`/`renderedDigest`, `stall.expectedDigest`/`renderedDigest`, the
  observation rows' `identicalAudio`, and `corpus.browserDigests`. The one *frozen* audio pin on
  this leg is `corpus.nativeDigest` — `expected.directOracle.nativePcmF32leSha256` from
  `tests/browser-v1/expected.json`, an unchanged file — and the gate is

  ```js
  corpus.nativeDigest === corpus.shippedArtifactDigest
    && corpus.browserDigests.every((digest) => digest === corpus.nativeDigest)
  ```

  That gate **passes** on chromium, firefox and webkit. The browser's rendered corpus PCM hashes
  bit-for-bit to the pre-existing native pin, which is the strongest available proof that rendered
  audio did not move: it is not an argument that the change *should* be inert, it is the frozen
  pre-#281 number being reproduced by the migrated harness. Nothing was re-pinned.
* The stall row's `ringFrames === 5120`, `renderedFrames === 5120`, `nextAbsoluteSample === "5120"`
  and `renderedQuanta === "40"` gates also pass, which is the same statement for the ring geometry
  the surviving `sourceRingFrames` word controls.

The audit's counterfactual is worth stating: had the harness's boot options actually *encoded*
something the six-word shape expresses differently, one of those digests would have moved and this
document would be reporting that instead. None did.

## 6. Results on this box

`npm run qualify -- --artifacts <build> --browser NAME --check-matrix --self-test-mutations`:

| browser | version | outcome |
| --- | --- | --- |
| chromium | 151.0.7922.34 | all qualification gates passed |
| firefox | 153.0 | all qualification gates passed |
| webkit | 26.5 | all qualification gates passed |

`--check-matrix` passing means each row's recorded floor and outcome in `results.json` still match
this run, and `BROWSER_DEPLOYMENT_MATRIX.md` is still the document generated from it — so the
matrix did not need re-recording either.

## 7. Issue #280 — the artifact set, five → six

`server.mjs::exactArtifacts` required the exact five-file set last frozen in #139.
`scripts/build-web-audioworklet.sh` has emitted six since #243 added
`miso-engine-v2-abi-layout.json`, so the qualification server refused the very directory the
workflow's own build step produces, before any browser started. The two other enumerations of the
set — `scripts/check-web-audioworklet.sh:131` and
`scripts/web-audioworklet-browser-correctness.py:25` — were already six; `server.mjs` was the only
straggler. (`flac-decoder-server.mjs`'s exact *four*-file set is a different artifact and is
correct.) The `.d.ts`-wide sweep found no fourth enumeration.

The pin stays **exact**, not a minimum, and `run.mjs::artifactSetProofs` proves both halves of that
against the real built directory under `--self-test-mutations` — the same flag CI already passes.
Red proofs are recorded in `hosts/host-web/MUTATIONS.md`.
