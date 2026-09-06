# Issue 475 Sol attempt 3 evidence

Final source checkpoint is `019ac62a`. Production compressor behavior is unchanged: the only
`kernel.rs` delta after Sol2 is inside its `cfg(test)` child module, and the other source delta is
the test-only allocation child environment-marker rename.

The W4 test fixture constructs the existing private `PreparedCompressorBank<Simd4>` and `Instance`
only after calling the production metadata, defaults and ring-length validators. It then invokes
the actual `PreparedNativeEffectBank` restore, reset, dual process, mono process, desymmetrize and
snapshot methods. Its two restore directions, full-default reset and asymmetric-channel mono
reopen compare the first subsequent W4 PCM and all four lanes' complete serialized state. A final
assertion confirms this x86-64-v3 build's public factory still returns `None` for unavailable W4;
the fixture makes no W4 native-admission claim.

The two compile failures and first executed oracle failure are retained verbatim. The oracle
failure exposed a fixture error: its dual oracle used unequal planes despite mono collapse's
mirrored-plane precondition. The corrected finite frame/lane-varying input mirrors both planes,
and the exact test passes in debug and release.

`MISO_ENGINE_COMPRESSOR_ALLOCATION_AUDIT_CHILD` is the sole renamed marker. The exact allocator
test still passes in debug and release, and `check-env-vocabulary.sh` reports 114 accepted names
under the one `MISO_ENGINE_` prefix. All requested affected debug/release suites, full compressor
debug suite, strict all-target Clippy, fmt/diff, realtime, lane, workspace and environment
vocabulary policies pass. No timing, object rebuild, G5 rerun, immutable delivery qualification,
full-workspace qualification or PR/CI claim is part of this attempt.

