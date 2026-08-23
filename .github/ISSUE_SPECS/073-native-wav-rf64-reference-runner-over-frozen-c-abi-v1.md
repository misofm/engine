# 073 Native WAV/RF64 reference runner over frozen C ABI V1

## Outcome and readiness

Ship the smallest native end-to-end reference tool: read one strict session, resolve its native
WAV/RF64 inputs, feed the accepted host-source boundary, render only through frozen C ABI V1, and
atomically publish deterministic block-planar `f32le` PCM.

**SOL XHIGH READINESS PASS / READY FOR SOL HIGH PASS 1.** This issue is tooling-only and
dependency-ready after accepted Issues 022 and 112. Sol High implements; Sol XHigh briefs and
adversarially verifies. One implementation pass plus one bounded HOLD correction is the complete
budget; a second HOLD is STOP/rescope. Benchmark, timing and real-user workload counts start at and
must remain zero.

Remote Issue 073 remains open under its earlier combined title/body. Root must synchronize it to
this exact title and body after the docs checkpoint is upstream; this local brief performs no Git
or GitHub mutation.

## Dependencies by exact accepted title

- **Stable C ABI and host-fed planar PCM render** (Issue 022)
- **Close native-source seek submission qualification and seal backpressure fix** (Issue 112)

Issue 043 is already consumed transitively and is not a direct dependency. This issue neither
depends on nor implements the complete Issue-005 provider or transactional plan replacement; that
product work is Issue 113.

## Frozen vertical

Add `tools/miso-engine-native-pcm-runner` with one shared, testable runner function and a thin CLI.
It must use the public `miso_engine_source::NativeWaveDecoder`/WAV-RF64 parser
surface for file decoding and only these installed C ABI V1 operations for engine work:
`engine_create`, `compile_session`, `source_submit_planar_f32`, `render_f32_planar`, resource/error
queries and matching destroys. It may not call a Rust session compiler, graph renderer or source
ring directly to bypass the ABI.

The exact CLI is:

```text
miso-engine-native-pcm-runner \
  --session SESSION.toml --source-root DIRECTORY \
  --frames POSITIVE_U64 --output OUTPUT.f32le
```

Each option occurs exactly once, no positional or unknown argument is accepted, and `--frames`
must be an exact positive multiple of the compiled session quantum. The session determines the
explicit sample rate and quantum; the tool accepts exactly 44,100, 48,000, 88,200 and 96,000 Hz and
performs no SRC. All arithmetic is checked before decoder, C compile or output publication.

### Runner-local source identity adapter

The product's locator and identity strings remain opaque. The runner alone recognizes:

- `file:<relative-path>` where the suffix is nonempty UTF-8 with `/` separators, contains no empty,
  `.` or `..` component, backslash, NUL, absolute/root/prefix form, and resolves beneath the
  canonical `--source-root` to one regular non-symlink file; and
- `sha256:<64 lowercase hex>` equal to SHA-256 of the complete resolved file bytes.

Every declared source must use those forms. Resolve and hash all sources before C compilation or
creating any output path. Duplicate canonical paths are allowed only when every occurrence has the
same identity. Locator interpretation, path policy and SHA-256 are tool contracts, not new session,
source or C ABI semantics.

### Decode, feed and render order

For each source, validate WAV/RF64 structure, exact declared channel count/sample rate and checked
region bounds before rendering. Decode no more than one quantum per source into preallocated
planar scratch. For each absolute output quantum, visit sources in canonical source-ID byte order,
submit at most one complete/short/final chunk with the compiled absolute source frame, and then call
the C render function exactly once. Backpressure or any non-OK submission/render result is a
terminal tool error; there is no spin, sleep or unbounded retry. No whole stem or whole output may
be retained in memory.

The output is concatenated quantum records. Each record is exactly the left `quantum` samples then
the right `quantum` samples, each IEEE-754 `f32` encoded little-endian. Total length is exactly
`frames * 2 * 4` bytes. The runner canonicalizes no finite sample and preserves product-permitted
signed zero bit-for-bit.

### Atomic no-clobber publication

Refuse an existing output or exact sibling partial path before work. Write only to
`OUTPUT.f32le.issue073.partial` with create-new semantics, flush and sync it, verify its checked
length and SHA-256, then publish by a same-directory no-replace operation. The accepted output must
never be observed partially and must never overwrite a regular file, symlink or hardlink target.
Any parse, decode, C, short-write, sync, identity or publication failure leaves the requested final
path absent and preserves a typed stderr diagnostic; a preexisting final/partial sentinel is byte-
and-shape unchanged. Successful publication removes only the owned partial entry.

## Fixtures and objective gates

Create a small checked manifest with RIFF fixtures at all four launch rates and at least one
representative RF64 fixture using `ds64`. It must include a nonzero region origin, a final short
source chunk, signed zero and finite sanitation witnesses without claiming an implicit SRC. Freeze
the exact session/file/output byte sizes and SHA-256 values. Independently parse the resulting
block-planar bytes in tests and compare exact bits plus exact total length; the oracle may not call
the runner encoder.

Negative rows cover CLI multiplicity, zero/non-quantum/overflow frames, locator traversal and
absolute forms, symlink escape, uppercase/malformed/mismatched identity, missing/truncated RIFF and
RF64, wrong channels/rate/region, unsupported rate, source backpressure, C compile/render failure,
existing final/partial entries and injected short-write/sync/publication failure. Each row freezes
the phase/code, produces no accepted output and proves caller/source/session cleanup.

Focused gates are locked tool/package tests and check; warning-denied Clippy/rustdoc; native C ABI
symbol/header checker; fixture-manifest validation; a scalar-Wasm compile exclusion proving the
native-only tool is not reachable; format; applicable workspace/realtime policies and their
mutations; shell syntax; conflict/trailing-whitespace/artifact/diff scans. A broad workspace
nonbenchmark seal is required only if Sol XHigh finds the dependency or policy reach broader than
the focused packages. No benchmark target, timer, real file workload, playback or listening runs.

## Allowed paths

- `tools/miso-engine-native-pcm-runner/**`;
- root `Cargo.toml` and `Cargo.lock`, only for the new workspace member/direct dependencies;
- `fixtures/native-pcm-runner/v1/**`;
- `docs/NATIVE_PCM_REFERENCE_RUNNER_V1.md`;
- new `scripts/check-native-pcm-runner-v1.sh` and
  `scripts/test-native-pcm-runner-v1-policy.sh`;
- the narrow existing workspace/realtime policy checker and mutation scripts only if the exact new
  native tool path requires an allowlist row; and
- this spec/brief plus exact README/implementation-plan status routing.

`crates/miso-engine-capi/**`, `crates/miso-engine-source/**`, protocol/session/graph/runtime product
code, installed headers and existing accepted fixtures are immutable. Any required product/API
change is STOP and must move to Issue 113 or a stateless rescope.

## Explicit non-goals and evidence

No complete BTLV provider, event drain, session mutation, replacement-plan publication, C ABI
symbol/layout change, platform matrix, mobile/browser host, codec, interleaving, variable-frame
render, benchmark, timing, tuning or listening. Issue 114 owns cross-target qualification after
both this runner and Issue 113 exist.

Record exact changed paths/hashes, manifest/output hashes, command lines, test counts, diagnostic
rows, allocation/resource evidence, accepted dependency identities and all zero prohibited counts.
Sol High stops after a coherent focused-green checkpoint. Sol XHigh returns strict PASS or the sole
bounded HOLD; focused PASS makes Issue 073 complete without waiting for Issues 113/114.
