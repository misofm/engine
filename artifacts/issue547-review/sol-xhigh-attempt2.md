PASS — issue 547 attempt 2.

Exact review range:

- Head: `23536b5ea3dfd3e40aa3aa09458046e530cc046f`
- Base/merge-base: `86d5b4bd97999a376123f3be3d6c04d27b8733e1`

The coordinator correction closes the sole attempt-1 blocker:

- `stat -c %s accepted.f32le`: exit 0, stdout `8192`.
- Explicit accepted-partial absence probe: exit 0.
- Explicit rejected-final absence probe: exit 0.
- Explicit rejected-partial absence probe: exit 0.
- All probe stderr/stdout shapes are appropriate.
- All 20 packed coordinator captures match the [manifest](/home/bl/misofm/engine-native-runner-process-boundary/artifacts/issue547-coordinator/capture-manifest.json) hashes and decoded sizes.

The inaccurate original metadata records remain byte-unchanged and are candidly identified in the [issue record](/home/bl/misofm/engine-native-runner-process-boundary/.github/ISSUE_SPECS/547-native-runner-process-boundary.md). The prior FAIL is also preserved rather than rewritten.

The integration-test blob is identical between `b331b87b` and current head. No production, audio, parser, fixture, pin, dependency, manifest, lockfile, or script changed. Full committed `git diff --check 86d5b4bd..23536b5e` exits 0. The coordinator’s no-index check exit 1 is the normal “files differ” result; its empty output confirms no whitespace error.

No blockers remain. Required CI remains the later root delivery gate. No workloads were repeated and no state was mutated.