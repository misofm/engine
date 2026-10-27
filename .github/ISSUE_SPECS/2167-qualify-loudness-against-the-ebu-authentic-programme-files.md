# Qualify loudness against the EBU authentic-programme files

## Origin and product slice

Split from #2127 before implementation on 2026-10-27. #2127 qualifies the shipped stereo meter against all 19 mono/stereo-applicable ITU-R BS.2217-2 resources. This issue owns the original EBU authentic-programme qualification: EBU Tech 3341 cases 7–8 and Tech 3342 cases 5–6. It adds official stimuli and evidence, not a new DSP algorithm or multichannel metering.

## Current acquisition dependency

Official landing page: https://tech.ebu.ch/publications/ebu_loudness_test_set . Its v5 archive is https://tech.ebu.ch/files/live/sites/tech/files/shared/testmaterial/ebu-loudness-test-setv05.zip . Ordinary local requests on 2026-10-27 return HTTP 403 with a Cloudflare denial page, including the documented download and old official aliases. No official binary was acquired, so native metadata and file hashes are unverified. Preserve this fact rather than claim a completed EBU test. Evidence: /home/bl/misofm/engine-work-evidence/easy3-20261027/2127/ebu-v05-http-headers.txt and ebu-v05-http-body.html. An authorized official archive, or restored public download access, unblocks implementation.

Terms: https://tech.ebu.ch/files/live/sites/tech/files/shared/testmaterial/use%20of%20EBU%20AUDIO%20test%20sequences.pdf . Use the sequences only for permitted internal R&D assessment, acknowledge © EBU, and do not commit or redistribute their audio/archive. No unofficial download is substituted for verified provenance.

## Contract and paths

Extend #2127’s sole official loudness fixture manifest, external downloader, README and existing owning file-conformance test under crates/loudness/tests/. Add only this issue spec outside those paths. No production loudness, render, host, protocol or Cargo dependency changes without an amended brief. Determine actual archive members and native PCM shape from verified bytes; do not assume a filename, resample, silently downmix or duplicate mono.

Primary expectations: https://tech.ebu.ch/docs/tech/tech3341.pdf Table 1 cases 7–8 require integrated loudness −23.0 ±0.1 LUFS. https://tech.ebu.ch/docs/tech/tech3342.pdf Table 1 cases 5–6 require LRA 5 and 15 ±1 LU respectively. Read integrated loudness at actual EOF; follow the publication’s documented trailing-silence procedure for the file-based LRA reading, without changing samples in the source file.

## Objective gates

1. Acquire the official archive legitimately, pin archive/member SHA-256 in the existing corpus-owner manifest, record native rate/channels/PCM format and terms. No audio is committed.
2. Fetch/verify outside cargo test. Qualification invocation fails if any required file is missing, corrupt or of an unexpected shape; no per-case skips or fabricated expected values.
3. Feed every selected authentic programme at its native supported rate through the shipped LoudnessObserver/LoudnessAnalysis and meet all four published expectations at their stated tolerances. Do not weaken tolerances on failures.
4. Run focused owning tests and formatting sequentially through the shared build/test queue. Fresh independent review explains which real-programme behavior these rows defend beyond synthetic recipes.
5. Record actual readings, source identities, commands and limits. Close only after upstream evidence, required qualification and GitHub synchronization.

The acquisition dependency is explicit. Extended licensing negotiation, fixture redistribution, a new generic data framework and production DSP changes are outside this slice; any discovered measurement defect receives its own stateless issue before expanding implementation.
