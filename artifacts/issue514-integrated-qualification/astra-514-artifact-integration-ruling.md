# #514 artifact integration ruling — approved, bounded

Reviewed clean frozen `d9dbb3147bc273f13082cedda30766ea5c5eac35` in `/home/bl/misofm/engine-idle-admission-clear`, the accepted integrated-base review, `/tmp/514-delivery-build.py`, actual builder command/log/status and unchanged ordinary builder source. The only change from reviewed `e789c777` is the #514 issue-spec adoption. Accepted host source/test bytes and build inputs remain unchanged.

The actual ordinary command was `bash scripts/build-web-audioworklet.sh /tmp/engine-514-qualified`. Its metadata identifies the frozen source, worktree and outer target directory. The wrapper checks accepted HEAD and cleanliness before invoking the script and checks unchanged HEAD/cleanliness afterward. The ordinary builder uses its own temporary Wasm target internally and removes it on exit. The log records successful release compilation, then the real artifact comparison failure:

- Existing expected SHA-256: `eb573b1e5fa083eb9d12f90a21de99536310d5c8379d5d2c671370a1dbfb32c4`.
- Observed SHA-256: `c06f9517763890a081f9ee60c70fc8531fdcafbd9630495e4de5782bcf0dc79a`.
- Actual builder status: **1**. The capture wrapper returning 0 does not turn this recorded inner failure into a pass; its code explicitly writes the subprocess status and reports it without exiting with that status.

The expected digest matches the tracked pin. The output directory is still empty: comparison failed before publication, and the temporary module was cleaned up. This review verifies the authentic recorded observation and source/command lineage. It does not independently hash a nonexistent retained/published module or claim that consumers have passed.

Approve exactly the observed SHA-256 replacement in `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256`, its corresponding decision/evidence checkpoint, and the unchanged seven-step delivery route **after both active immutable qualification processes are terminal**:

1. Ordinary verified rebuild with the exact approved pin into the existing empty output directory or another fresh empty directory, without a REPIN bypass.
2. Existing static/object/ABI/render-closure checks on the published artifact.
3. Existing resource/current direct-oracle/native-row gate, including its rejection controls.
4. Existing hermetic worklet tests.
5. Pinned qualification npm installation.
6. Current Chromium/Firefox/WebKit qualification with existing mutation self-tests and candidate/module records.
7. Existing generated matrix check.

After the ordinary builder succeeds, independently verify the published module's actual byte count and SHA-256 against this exact pin and attribute all consumer results to those bytes and their source. Existing candidate/module identity records may change as required by that route. Preserve the original failed command/log/status unchanged.

This ruling authorizes no resource-number or PCM/digest expectation repin, canonical/corpus change, schema, gate/script/CI/lint change or additional runtime implementation. A real numerical discrepancy must be captured and separately derived before any expectation amendment. In particular, #514's native +8-byte observation does not predict Wasm padding, and the inherited #511 +204 graph ruling supplies no authority to alter additional rows. If another artifact digest is observed, return that discrepancy rather than substituting it under this ruling.

Root reports immutable workspace/targets process 26942 and release-host/static process 95223 still active. Do not edit any tracked source, pin, spec or generated record, or carry the #511 closure cherry-pick, until **both** have terminal statuses. This ruling neither interrupts them nor certifies their pending outcomes. The reported #511 post-main run success and planned closure commit `32a1365a` may be recorded afterward; no live GitHub status was independently queried here.

Final integrated review, actual-PR source identity, required CI and GitHub delivery synchronization remain necessary. This is an observed artifact-pin integration ruling, not final delivery acceptance or a new implementation attempt.

No builds/tests, timing, source/spec edits, Git mutations or GitHub operations were performed. Read-only inspection/hash comparisons were used; only this requested `/tmp` ruling was written.
