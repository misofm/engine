FAIL — evidence-integrity blocker.

Reviewed exact head `b331b87b7df697f3c6306f95731d20c9cb8ce0e3` against base/merge-base `86d5b4bd97999a376123f3be3d6c04d27b8733e1`.

Implementation itself satisfies issue 547:

- The single integration test spawns Cargo’s executable via `CARGO_BIN_EXE_native-pcm-runner` for both cases.
- Accepted case requires exit 0, empty stdout/stderr, exactly 8192 bytes, and no partial.
- Rejection requires exit 2, empty stdout, exact `native-pcm-runner.v1\tcli\tframes.zero\n`, and no final/partial.
- Temporary cleanup has explicit success cleanup plus `Drop` fallback.
- Full diff contains no production, audio, parser, fixture, pin, manifest, dependency, or lockfile changes.
- Preserved results show 19 library tests plus 1 integration test passing; strict Clippy, corrected formatting, workspace policy, and committed-range diff check pass.
- Original failures are retained: focused compile exit 101 for missing crate documentation and initial formatting exit 1, followed by corrected exit-0 captures.
- All 92 gzip artifacts match the [capture manifest](/home/bl/misofm/engine-native-runner-process-boundary/artifacts/issue547-attempt1/capture-manifest.json) for compressed hash, decoded size, and decoded hash.

Blocker: both direct metadata command records are materially inaccurate. They claim a single `stat -c %s` invocation, but their stdout was produced by compound shell probes:

- `direct-accepted-metadata.command.json.gz` cannot produce its five key/value lines.
- `direct-rejected-metadata.command.json.gz` claims `stat` on an absent file, yet records exit 0 and custom metadata output.

The actual compound probes survive in `luna-attempt1.stderr.gz`, so this can be corrected without repeating Cargo suites: accurately encode the probe command/script and regenerate the affected compressed artifacts and manifest hashes.

Required CI remains the later root delivery gate and is not part of this FAIL. No files or repository state were mutated during review.