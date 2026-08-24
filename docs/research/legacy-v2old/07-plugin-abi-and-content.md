<!--
Provenance: copied from misofm/engine-v2-old docs/research/07-plugin-abi-and-content.md on 2026-08-24 for issue #144 item 8.
Legacy research archive only; current Engine V2 contracts and rulings remain authoritative.
-->

# Plugin ABI and content

Plugins have two planes. WIT/component metadata supports discovery and control, following the [component-model explainer](https://github.com/WebAssembly/component-model/blob/main/design/mvp/Explainer.md). The RT ABI is deliberately not WIT: it is flat numeric calls over preallocated linear memory with explicit offsets, capacities, event spans, and no strings or allocation in `process`.

Launch executes no plugins and exposes this vocabulary with capability false. Immediate post-launch hosting mounts only certified/allowlisted modules after manifest/schema/version/cost/memory validation. Separate plugin memory means required, explicitly counted input/output copies. WebAssembly’s safety model does not solve realtime scheduling ([Wasm security](https://webassembly.org/docs/security/)); untrusted execution therefore defers to isolation and watchdog design. WASI capability principles inform least authority ([WASI capabilities](https://github.com/WebAssembly/WASI/blob/main/docs/Capabilities.md)); host limits must be concrete, as illustrated by [Wasmtime security guidance](https://docs.wasmtime.dev/security.html).

`miso-engine-walrus` canonically addresses plugin blobs using a tagged Walrus blob identifier. It must not call that value a CID: Walrus object semantics are documented by [Walrus](https://docs.wal.app/docs/system-overview/core-concepts), while CID is a distinct multiformats identifier ([CID specification](https://specs.ipfs.tech/cid/)). The tag records identifier scheme and version; it does not claim universal multiformat compatibility.
