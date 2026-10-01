# Use the official Ubuntu archive for browser qualification dependencies

## Smallest closable slice

Repeated required browser qualification jobs hit their unchanged 10-minute limit during Ubuntu package downloads from azure.archive.ubuntu.com, before browser tests. #1157 removed unnecessary recommended PulseAudio packages and passed full qualification, but it explicitly required a separate bounded mirror/download issue for remaining failures. This issue changes only browser-job APT archive selection to the official Ubuntu archive already listed by the runner, preserving signed package verification, suites, pinned Playwright/browser versions, required dependencies, private audio, gates and budgets.

## Frozen decisions and ownership

Root Sol briefs/implements, one of the existing Sol workers adversarially reviews; no extra agent. Own only the browser Install pinned browser block in .github/workflows/qualification.yml and this spec. Prefer one local configuration write selecting https://archive.ubuntu.com/ubuntu/ from the existing runner mirror mechanism before install-deps. Preserve APT source signatures, release/suite/components and existing repository setup; no unsigned/allow-unauthenticated settings, package deletion, new cache/container, runner change, time-budget increase, workflow route/expectation/job/concurrency change or weakened browser tests.

Two coherent implementation attempts maximum. If official-archive selection still cannot complete required qualification, preserve logs, stop this shape and record a bounded external availability blocker; no escalating infrastructure project.

## Evidence and objective gates

Capture original and failed-job replay logs with exact job/run/step and archive download facts. Primary runner source: https://github.com/actions/runner-images/blob/main/images/ubuntu/scripts/build/configure-apt-sources.sh lists Azure priority1, official archive priority2 and security archive priority3 in /etc/apt/apt-mirrors.txt and uses mirror+file for Ubuntu24.04. Choosing the official archive is an inference for bypassing observed Azure stalls, not an asserted controlled speed gain.

Validate shell syntax and configuration contents offline, existing CI-routing/test-support coverage policy and mutation gates, and normalized complete workflow diff. No new source/prose test or benchmark harness. Live objective gate is unchanged required qualification with actual Chromium/Firefox/WebKit tests passing; local metadata/config checks alone cannot earn PASS. Push exact checkpoints/evidence promptly, synchronize GitHub body/state, close only after reviewed evidence is upstream; merge through required qualification and verify main.

## Attempt 1 checkpoint evidence

Run 36897477817, first WebKit job 110489205904 and its single replay 110493917477 both timed out in Install pinned browser, before browser tests. Replay fetched 125 MB in 9 min 42 sec (215 kB/s) from Azure and was canceled at 17:36:04 UTC. Raw logs are preserved at /tmp/engine-housekeeping-compressor-builtins-webkit.log and /tmp/engine-housekeeping-compressor-builtins-webkit-replay.log; PR #1160 links the remote jobs and status. Other qualification jobs passed. No further rerun of that candidate is planned.

The candidate adds a comment and one printf/tee write selecting the official archive at priority 1 and official security archive at priority 2 before Playwright install-deps. No source stanza, suite, component or signing key changes. The normalized whole-workflow comparison equals the base after removing only these two added lines. Bash syntax and the exact generated two-line mirror file PASS in an offline check without changing this machine's APT configuration. Existing check-ci-path-routing.py, test-ci-path-routing.py, check-test-support-ci.py and test-test-support-ci.py all PASS. One initial local invocation used nonexistent shell-script names and failed before any gate; corrected Python script names completed successfully. No new committed test or timing claim. Worker B's local adversarial review and actual required browser qualification remain pending; no PASS is claimed yet.

### Worker B local adversarial review: PASS

Worker B reviewed the exact two lines, official runner setup and APT transport semantics (https://raw.githubusercontent.com/Debian/apt/main/doc/apt-transport-mirror.1.xml). Lower priority numbers are preferred; trusted literal printf writes the expected tab-separated entries and sudo tee errors remain visible. Only the mirror list changes; the existing signed source stanzas, suites/components/keyrings and all dependencies, browser pins, private audio, three actual browser gates, routes, jobs, matrix, concurrency and budgets remain intact. This proves selection semantics, not network availability or a measured speed gain. Implementation checkpoint is 6b9d3ea5. Final issue verdict requires the unchanged live qualification run.

## Root adversarial verdict: PASS — attempt 1

PR #1162, required qualification run 36902802153, now passes every expected job with the unchanged gate matrix. Actual Chromium 110507018435, Firefox 110507018419 and WebKit 110507018525 all passed their browser/AudioWorklet/attestation/native-parity/stall/mutation checks. The preserved WebKit log (/tmp/engine-housekeeping-official-archive-webkit.log) confirms official archive downloads: 125 MB in 37 seconds, then minimal PulseAudio and all browser gates. This is one runner observation, not a controlled performance estimate; no speed percentage is claimed.

The first cross-target job 110506130307 separately timed out at its unchanged 25-minute limit during Azure package-index update, before cargo checks (log /tmp/engine-housekeeping-five-crates-cross-target.log). Its single failed-job replay 110517866640 completed installation and cross-target checks; required qualification is SUCCESS. Successful browser/product jobs were retained. Root reviewed candidate/local-review/live evidence and confirms this bounded archive-selection outcome without weakening signing, dependencies, budgets or required checks. No new committed test or owner decision. Evidence is synchronized and merging remains through the passed PR.
