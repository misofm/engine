# Use the official Ubuntu archive for browser qualification dependencies

## Smallest closable slice

Repeated required browser qualification jobs hit their unchanged 10-minute limit during Ubuntu package downloads from azure.archive.ubuntu.com, before browser tests. #1157 removed unnecessary recommended PulseAudio packages and passed full qualification, but it explicitly required a separate bounded mirror/download issue for remaining failures. This issue changes only browser-job APT archive selection to the official Ubuntu archive already listed by the runner, preserving signed package verification, suites, pinned Playwright/browser versions, required dependencies, private audio, gates and budgets.

## Frozen decisions and ownership

Root Sol briefs/implements, one of the existing Sol workers adversarially reviews; no extra agent. Own only the browser Install pinned browser block in .github/workflows/qualification.yml and this spec. Prefer one local configuration write selecting https://archive.ubuntu.com/ubuntu/ from the existing runner mirror mechanism before install-deps. Preserve APT source signatures, release/suite/components and existing repository setup; no unsigned/allow-unauthenticated settings, package deletion, new cache/container, runner change, time-budget increase, workflow route/expectation/job/concurrency change or weakened browser tests.

Two coherent implementation attempts maximum. If official-archive selection still cannot complete required qualification, preserve logs, stop this shape and record a bounded external availability blocker; no escalating infrastructure project.

## Evidence and objective gates

Capture original and failed-job replay logs with exact job/run/step and archive download facts. Primary runner source: https://github.com/actions/runner-images/blob/main/images/ubuntu/scripts/build/configure-apt-sources.sh lists Azure priority1, official archive priority2 and security archive priority3 in /etc/apt/apt-mirrors.txt and uses mirror+file for Ubuntu24.04. Choosing the official archive is an inference for bypassing observed Azure stalls, not an asserted controlled speed gain.

Validate shell syntax and configuration contents offline, existing CI-routing/test-support coverage policy and mutation gates, and normalized complete workflow diff. No new source/prose test or benchmark harness. Live objective gate is unchanged required qualification with actual Chromium/Firefox/WebKit tests passing; local metadata/config checks alone cannot earn PASS. Push exact checkpoints/evidence promptly, synchronize GitHub body/state, close only after reviewed evidence is upstream; merge through required qualification and verify main.
