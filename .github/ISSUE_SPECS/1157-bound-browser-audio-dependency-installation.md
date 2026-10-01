# Bound browser qualification audio dependency installation

## Product outcome and smallest closable slice

Unblock required qualification during the crate-housekeeping workflow by installing only the headless audio packages the existing browser job uses. Preserve the pinned Playwright browsers and install-deps command, all Chromium/Firefox/WebKit artifact/AudioWorklet/mutation gates, real private PulseAudio sink/socket, pactl preflight, qualification expectations, trigger/concurrency rules and the existing ten-minute job budget. No engine/product/DSP behavior changes.

## Evidence and decision record

Main push run [36880192313](https://github.com/misofm/engine/actions/runs/36880192313) timed out in Chromium while installing browser dependencies; its one unsuccessful-job replay also timed out. PR #1156 run [36882471290](https://github.com/misofm/engine/actions/runs/36882471290) timed out in Chromium and Firefox during installation before browser tests; one unsuccessful-job replay is pending. All other product legs passed, and #1155's full pre-merge qualification passed. Raw local logs are /tmp/engine-housekeeping-main2-chromium.log and /tmp/engine-housekeeping-pair3-chromium.log; GitHub retains the authoritative run logs.

The workflow installs `pulseaudio` with recommendations; its log requests 38.7 MB of extra archives and 127 MB installed, including codec/desktop packages. The next step launches PulseAudio's core native protocol/null-sink modules and calls pactl. Freeze the bounded candidate as `apt-get install -y --no-install-recommends pulseaudio pulseaudio-utils`, explicitly retaining pactl rather than relying on a recommendation. The [official apt-get manual](https://manpages.debian.org/bookworm/apt/apt-get.8.en.html) explains that this flag omits recommended packages; dependency resolution remains in effect. Reduced installation work is an inference to qualify, not a claimed timing win.

Root Sol approves this necessary delivery repair as a separate issue before implementation. GPT-6.1 Sol xhigh worker B supplies attempt 1 under the user's model instruction, then resumes its crate queue; root independently reviews/checkpoints/synchronizes. Scope is only .github/workflows/qualification.yml and this issue spec. Maximum two coherent implementation attempts; if minimal package selection is insufficient, preserve logs and brief a separate mirror/download issue instead of widening this into caching infrastructure or raising timeouts.

## Objective gates and delivery

- Inspect the workflow/audio consumer and package dependency metadata; confirm pulseaudio-utils supplies pactl and the two module owners stay installed. Use read-only apt simulation if available; do not install packages on the shared host solely for this change.
- Change only the bounded install command. Run applicable existing workflow/router/self-test checks and YAML/diff validation; no new source/prose-grep test or benchmark.
- Stop when the minimal tranche is locally valid so root commits/pushes exact paths before extra work. Root records one adversarial verdict per attempt.
- Qualification of the changed snapshot must pass all three actual browser legs and required verdict, with the same artifact and job budget; local dependency simulation alone is insufficient. Preserve each failed run and report installation/execution limitations candidly. Update the GitHub body at material checkpoints; close only after PASS evidence is upstream and verify state.

## Attempt evidence

Pending implementation; no success or performance result claimed.
