**Approve the bounded artifact-pin update**, following the existing #475 precedent.

Verified clean HEAD `406303595ca2a15173984957a16ff6080bb9fd9a`, matching captured source hashes and unchanged DSP/build source since source PASS. Native shared/static ABI completed with status0. The ordinary builder compiled successfully, then exited1 solely at the pin comparison:

- Existing: `04f938f667180d0e7b972e1cc2236af3a5d5e3f7b73d1ed503dde72e83aa9bce`
- Builder-observed: `e70a4311da3f053ea77fc0ec7ac00348d534dfb2919e05591fefed515988802d`

Root may checkpoint this ruling, then authorize Luna to update only [the artifact pin](/home/bl/misofm/engine-gate-detector-access/hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256) to that exact observed hash, with retained provenance.

Conditions:

1. Preserve original commands, source identities, output and failure status. The builder hashes its temporary module before comparison; failure leaves the output directory empty and deletes the temporary build. This is a recorded builder observation, not an independently rehashed retained artifact.
2. Run the ordinary verified builder without repin bypass on unchanged DSP/build source. It must reproduce the pinned hash and publish the actual output.
3. Complete existing static/object, resource checks with 26 red controls, hermetic tests, actual Chromium/Firefox/WebKit qualification with mutations, and matrix verification.
4. Generated candidate/hash records must identify the actual qualified source/output. No numerical, corpus, resource-expectation, policy, CI or matrix changes are authorized. Any further discrepancy requires concrete assessment.

Source PASS remains unchanged. This ruling grants neither passing artifact qualification nor PR/merge approval. Review was read-only; the worktree remains clean.