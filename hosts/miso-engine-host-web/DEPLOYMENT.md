# Browser AudioWorklet deployment

Build the release directory with `scripts/build-web-audioworklet.sh EMPTY_DIRECTORY`. Deploy all five files together under one release-scoped URL; the host JavaScript, worklet JavaScript, declaration, `miso-engine-v2-audio-worklet.simd128.wasm`, and `miso-engine-v2-parameter-metadata.json` are one indivisible artifact set.

`miso-engine-v2-parameter-metadata.json` (issue 137) is the app's parameter vocabulary: effect IDs, parameter IDs, names, units, ranges, defaults and enumerations, plus the builtin track surface and the command kind/reason tables. It is generated from the engine's own effect registry at build time, so it always describes the module beside it — serve it from the same release URL and never hand-edit it. Read its `liveUpdatable` field before sending a `miso.command.v1` record: it says which parameters the control path can actually move, and the browser ABI's `.d.ts` header explains why the answer is what it is.

The page must be delivered over HTTPS in production. `http://127.0.0.1` and `http://localhost` are suitable for the checked-in local demo because browsers treat loopback origins as potentially trustworthy. Load `miso-engine-v2-audio-worklet-host.js` from the main realm and give `createMisoAudioWorkletHost` same-origin URLs for the Wasm and worklet modules. The worklet is registered by `audioWorklet.addModule()`; callers must handle the exact `miso.unsupported.v1` simd128 refusal separately from runtime `miso.error.v1` failures.

Serve the Wasm response as exactly `Content-Type: application/wasm`. Serve both JavaScript modules as `text/javascript` (with an optional charset), the metadata as `application/json`, and send `X-Content-Type-Options: nosniff`. The WebAssembly streaming contract rejects a response whose media type is not `application/wasm`, even though the current host deliberately compiles an `ArrayBuffer` so it can transfer the module into the worklet.

Use versioned or content-addressed release URLs. Those immutable files may use `Cache-Control: public, max-age=31536000, immutable`; use `Cache-Control: no-cache` for an unversioned entry page so it revalidates and selects one coherent release. Do not publish changed bytes at an immutable URL or mix files from different releases.

This host does not need extra response headers for the qualification in [BROWSER_DEPLOYMENT_MATRIX.md](./BROWSER_DEPLOYMENT_MATRIX.md). That statement is limited to this shipped message-copy artifact: the matrix does not qualify cross-origin isolation, `SharedArrayBuffer`, or a shared-memory Wasm build.

For a local run:

```sh
artifacts=$(mktemp -d)
bash scripts/build-web-audioworklet.sh "$artifacts"
cd hosts/miso-engine-host-web/qualification
npm ci
npm run demo -- --artifacts "$artifacts"
```

Open the printed loopback URL and press **Run qualification**. The server intentionally sends `Cache-Control: no-store` so a local run cannot reuse stale release bytes. It also intentionally sends no cross-origin-isolation headers.

Normative references: [Web Audio API AudioWorklet](https://webaudio.github.io/web-audio-api/#AudioWorklet), [WebAssembly Web API media type](https://webassembly.github.io/spec/web-api/#streaming-modules), and [RFC 8246 immutable responses](https://www.rfc-editor.org/rfc/rfc8246).
