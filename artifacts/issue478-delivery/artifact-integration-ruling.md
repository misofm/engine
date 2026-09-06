# Astra #478 observed artifact integration ruling

**Approved: one observed artifact-pin update and the existing delivery-consumer route. This is delivery integration, not another implementation attempt or final delivery PASS.**

Read-only review of clean accepted head `2d70b1592f50bc4b4066096f6fb10b9cb55f0824` in `/home/bl/misofm/engine-rt8-plan`, the ordinary builder, tracked pin, actual `/tmp/issue478-delivery/artifact-initial.{command.json,stdout,stderr,status}`, and the current qualification consumers. No reviewer builds/tests, timing, checkout/Git/GitHub edits or mutations occurred; only this requested temporary ruling was written.

The captured invocation is `bash scripts/build-web-audioworklet.sh /tmp/issue478-delivery/artifact` on clean accepted source. Compilation completed, then the actual builder's mandatory comparison rejected:

- Existing tracked/expected SHA-256: `c06f9517763890a081f9ee60c70fc8531fdcafbd9630495e4de5782bcf0dc79a`.
- Observed SHA-256: `e3f47856ad917edf2cd99cdc6669a4f3e6c6a9074b15c560a8337bf842187fef`.
- Exit status: **1**.

The source identities in the capture match the accepted rack/graph/allocation files. Compared with integrated main `c34383fc`, the only changed ordinary runtime source is rack's accepted private packed-slot representation/conversion and three predicates/guarded lane lookup. Graph edits are feature-gated test support; the allocation file is a test. Cargo, lockfile, compiler configuration, builder, host/browser code and metadata-generation inputs are unchanged. The digest observation is therefore attributable to the identified accepted build candidate; it is not evidence of a resource or PCM discrepancy.

The builder checks its digest before copying the module and removes its temporary build directory on exit. This ruling validates the captured hash observation, not an independently hashed retained module from that failed attempt. Do not report the failed ordinary build as successful or overwrite its raw evidence.

## Exact allowed integration

After **all currently running immutable source qualification commands have terminal statuses**, root may change only `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256` to the exact observed digest above, plus the numbered decision/evidence record. Keep the source frozen until those commands finish; their outcomes must be reported separately.

Then:

1. Run the unchanged ordinary builder into a verified empty output directory, with no REPIN bypass or new build/profile/strip overrides. Capture source identity, effective relevant environment and terminal result. If it reports any digest other than the one approved here, preserve and return that discrepancy rather than substitute a new value.
2. Independently hash and record the byte length of the successfully published `miso-engine-v1-audio-worklet.simd128.wasm`, proving it matches the pin. Attribute subsequent consumers to that exact module and candidate source.
3. Run `bash scripts/check-web-audioworklet.sh ARTIFACT_DIRECTORY`: the existing export/import/memory/opcode/vector and render-closure checks remain unchanged.
4. Run `python3 -B scripts/check-browser-expected-resources.py --artifacts ARTIFACT_DIRECTORY`, retaining its current direct/native parity, numerical rows and rejection controls, then the existing hermetic worklet tests.
5. Run the established pinned browser qualification for Chromium, Firefox and WebKit, including current mutation self-tests and generated deployment-matrix check. Candidate/module identity records and successful qualification output may be refreshed from those actual runs; this permits no expectation changes by assumption.

The remaining native/workspace/supported-target/ABI, actual-PR-head review, required qualification and GitHub synchronization obligations are unchanged. Already valid source-phase evidence need not be rerun merely because the content-addressed artifact pin changes.

No resource-number, memory-budget, PCM/native-digest, fixture/corpus, metadata, schema, export, gate/script/CI or runtime changes are authorized by this ruling. Native P=24/B=32 and unchanged native chain layouts do not establish a wasm32 byte adjustment; #511's reservation totals remain frozen. If an existing resource/static/browser/PCM consumer rejects the rebuilt candidate, preserve its exact observed inputs/output and obtain the smallest separately derived ruling before changing expectations. Do not preemptively repin rows to make them fit.
