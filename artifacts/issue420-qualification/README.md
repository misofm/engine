# RT-3 retained qualification evidence

These are the original logs and review records for #420's accepted source and immutable-source qualification. `manifest.json` records original paths, retained paths, byte lengths and SHA-256 values. Historical failed source reviews are explicitly separated from accepted evidence. Paths inside logs identify the original execution environment; the manifest does not imply those temporary build directories are portable or still present.

The candidate full workspace run completed 274 Cargo result blocks: 1,569 passed, zero failed, 24 ignored. The retained main455 baseline completed 274 blocks: 1,566 passed, zero failed, 24 ignored. The three additional graph tests explain the delta. The issue decision record records observed terminal tool statuses; a transcript hash alone does not prove a process exit status.

The independently built and verified worklet SHA-256 is `24f81af304e541ba0e734de5c7a3dc5221e71fa4de73f2545edea3c2960761fe`, from source `51e2aed211b30523076e0e8dd07973b13b57dc11`. Supported Wasm evidence covers the stated scalar/SIMD checks and named scalar non-LTO object population; it does not repair #427's production inspector or prove an unexecuted fallback branch.

The zero-launch preflight and exact runner-profile build completed. The untimed runner binary SHA-256 is `53dce85d8ff683693598da8dce79195ecfad1ad76300b68bb348196c00f81bab`. A bounded twelve-probe readiness check then ended exit 1 because load remained above 0.50. No runner/workload was invoked, no measurement authority was consumed, and no timing result is claimed. The quiet window ended so independent #411 work could proceed. Timing and actual PR/required-CI delivery remain pending.
