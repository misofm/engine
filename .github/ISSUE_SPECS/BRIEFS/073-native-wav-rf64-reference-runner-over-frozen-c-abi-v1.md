# Sol implementation brief — issue 073 native WAV/RF64 C ABI runner

## Decision

**SOL XHIGH READINESS PASS / READY FOR SOL HIGH PASS 1.** Implement exactly one tooling vertical:
strict session plus native WAV/RF64 inputs enter through a runner-local locator/identity adapter,
decoded chunks cross frozen C ABI V1 host submission, and rendered stereo leaves as atomically
published block-planar `f32le`.

Sol High is implementer; Sol XHigh is briefer/verifier. The entire budget is one pass and one
bounded HOLD correction. A second HOLD stops. Benchmark, timing, real-user workload, playback and
listening counts remain zero. Root separately synchronizes the open remote Issue 073 title/body.

## Dependencies and separation

Direct dependencies are accepted **Stable C ABI and host-fed planar PCM render** (Issue 022) and
accepted **Close native-source seek submission qualification and seal backpressure fix** (Issue
112). Issue 043 is transitive. Issue 113, not this issue, owns complete control/event transport and
transactional replacement. Issue 114 later joins 073 and 113 for target qualification.

Do not edit C ABI/source/protocol/session/graph/runtime product code, the installed header, or
accepted fixtures. If the public `NativeWaveDecoder` plus frozen ABI cannot implement the vertical,
stop with the exact missing seam.

## Exact implementation contract

Create one workspace tool and shared testable runner. Require exactly one each of `--session`,
`--source-root`, positive quantum-multiple `--frames`, and `--output`; the named output path must be
absent. Interpret only safe
`file:<relative-path>` locators under the canonical root and exact lowercase
`sha256:<64 hex>` whole-file identities. Resolve, hash and validate all sources before compile or
output creation.

Use public native WAV/RF64 decode into quantum-bounded planar scratch. At each quantum, submit
sources in canonical ID order through `miso_engine_v2_source_submit_planar_f32`, then call
`miso_engine_v2_render_f32_planar` once. Any backpressure/non-OK is terminal; no retry loop. Encode
each output quantum as left plane then right plane, little-endian `f32`, for exactly
`frames * 8` bytes.

Write a create-new exact sibling `.issue073.partial`, sync and verify it, then publish with a
same-directory no-replace operation. Never overwrite or expose partial accepted output. Preserve
preexisting sentinels and prove every negative path leaves final output absent.

## Evidence

Freeze a compact independent manifest: RIFF at 44.1/48/88.2/96 kHz plus representative RF64,
nonzero region origin, short final chunk, signed zero and exact output hashes. Cover all validation,
decoder, ABI, backpressure, output and cleanup failures with stable phase/code diagnostics. Test an
independent output parser rather than reusing the encoder.

Run only focused locked tests/check, strict Clippy/rustdoc, C ABI checker, manifest/policy mutations,
format/shell/static/diff gates and the native-only target exclusion. No benchmarks or timing. Sol
High hands off one immutable checkpoint; Sol XHigh issues PASS or the sole HOLD.

## Path fence

Allowed: `tools/miso-engine-native-pcm-runner/**`, its minimal root manifest/lock rows,
`fixtures/native-pcm-runner/v1/**`, `docs/NATIVE_PCM_REFERENCE_RUNNER_V1.md`, the two new exact
runner checker scripts, narrow policy allowlist/mutation rows if necessary, and exact issue/routing
docs. Everything else is frozen.
