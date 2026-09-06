# Issue 524 immutable delivery checkpoint

Accepted source: d7b67839 (Astra Sol3 source PASS on d8c14721). Workspace: 1,669 passed, zero failed, 24 ignored, 278 result blocks; stdout is losslessly gzipped. Native ABI and scalar/SIMD host/protocol/target-smoke checks pass.

The initial artifact command rejected a missing output directory before compilation (status 2). The next ordinary build observed a pin mismatch (status 1), ruled and updated exactly in 63c2fbb2. The normal rebuild passes; independent identity is d77d7558105c29d38751ab26c330689da829fbe0f011a6ffcc7edf55d583bc87, 2,698,540 bytes. Static/object, existing resource/native PCM parity with 26 negative controls, and worklet isolation checks pass. No resource or PCM expectations changed.

The first browser invocation rejected an abbreviated candidate SHA before launching browsers; corrected invocation uses the full 63c2fbb25db21d61ed16b7f56f7646b9c78f17c0 identity. Chromium 151.0.7922.34, Firefox 153.0 and WebKit 26.5 all pass with self-test mutations. Recorded matrix/check pass; only candidate/artifact identities changed in results and matrix. Every original preflight failure remains preserved.

User requested pause after this checkpoint. No PR has been opened for #524, no actual-PR/current-base review or required CI has run for it, and #524 remains OPEN. Resume with final delivery review/PR/qualification, not another implementation attempt. #518 and #140 remain open.
