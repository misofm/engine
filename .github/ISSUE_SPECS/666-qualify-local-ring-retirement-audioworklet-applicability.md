# Qualify `LocalRing` retirement AudioWorklet applicability

Parent/delivery peer: #664
Audit parent: #560
Coordination: #559
Frozen feature source: `e9c48b47cb2584ccaa3e74f35ce63cb71dc5f763`
Delivered artifact pin: `580e3cb4cd11e996598103f27b02d94559f6ef7ad57ef22732d18c0b4f98be10`

## Problem

#664 has SOURCE PASS for removing the unused `engine::realtime::LocalRing`
wrapper, its tests/re-export, and the smoke-only target-smoke API, plus the direct
realtime-policy inventory recalibration. `target-smoke` and the policy scripts do
not enter the shipped AudioWorklet, but `host-web` directly depends on the changed
`engine` crate. An unused generic is expected not to reach the final link; that
expectation is not artifact byte-identity evidence.

This issue owns one exact-source identity probe. It does not own #664 product
source, artifact qualification, promotion, pin edits, browser runs, SDK runs, or
delivery.

## Frozen scope

Hypatia, Luna HIGH agent `issue583_luna_impl`, is the sole executor after Astra
LOW returns scope PASS. Run exactly one repin-report builder invocation at the
clean pushed issue head derived from frozen #664 source. Before execution, record:

- executor/time, cwd, head/upstream, frozen feature source, current main and
  merge-base;
- Rust/Cargo versions and the exact artifact pin including newline shape;
- literal command and environment;
- clean tree and unchanged #664 product/policy paths;
- absence, including dangling symlinks, of
  `/tmp/issue666-repin-output` and `/tmp/issue666-repin-evidence`.

Exercise the evidence capture wrapper with harmless status-0 and expected
status-1 controls and independently read them back. Create the output directory
once as an empty non-symlink directory and the evidence directory once without
overwrite. Then run exactly once:

```text
MISO_ENGINE_WEB_AUDIOWORKLET_REPIN=1 bash scripts/build-web-audioworklet.sh /tmp/issue666-repin-output
```

Retain separate complete stdout/stderr, numeric status, timestamps, exact
identity/environment, postflight tree/output census, current pin, and hashes in a
self-excluding verified manifest whose hash is recorded separately. Require:

- status 0;
- stdout is exactly one lowercase 64-hex digest plus LF;
- the output directory remains empty, as repin-report mode exits before copying
  the six-file artifact set;
- repository paths and the committed pin remain unchanged.

Stop on any changed precondition, capture-control failure, nonzero status,
malformed output, nonempty output directory, tree drift, or concurrent activity.
Do not retry, run the ordinary builder, keep or inspect its temporary Cargo
target, create a six-file candidate, invoke static/resource/browser/SDK gates,
edit the pin, or change source.

## Decision and delivery

If the observed digest equals the delivered pin, Astra LOW decides whether the
unchanged builder/copied inputs and prior qualified artifact identity support an
unchanged-pin applicability PASS. If it differs, this probe establishes drift
only. Candidate assembly, static/resource/ABI/native-PCM checks, browser/SDK
qualification, pin promotion, and post-pin reconstruction require a pushed scope
amendment and fresh Astra LOW PASS; no automatic repin is authorized.

Root records only compact command/source/toolchain/pin/digest/status/census/hash
evidence. Full streams and generated build output stay temporary. No compiler
dump, `.ll`, assembly, object, archive, binary, `rlib`, `rmeta`, Cargo target,
benchmark, timing, allocation, or performance claim enters Git.

#664 remains the product delivery owner and cannot enter PR readiness until this
artifact dependency is delivered. #664 and #666 are the two active, overlapping
delivery issues; only #666 executes artifact work. Required exact-head/current-
main review, PR qualification, guarded merge, post-main qualification, GitHub
synchronization, and clean delivered-worktree removal apply to both issues in
dependency order.

## Probe authorization

Astra LOW returned **SCOPE PASS** at exact clean pushed head/upstream
`1826766f974158a6f25882ad96d17782c2333b70`, current main/merge-base
`1bcce704ca12d531cbdaec4b641acca302543426`, and tracker
`75af15c0b2e050092de9660f256e401de0efe638`. GitHub #666 matches; only this
brief differs from frozen source `e9c48b47`; both temporary paths are absent
including symlinks; builder inspection confirms repin mode exits before artifact
copying.

Hypatia alone is authorized, after immediate identity/freshness and capture-
control preflight, to run the single command exactly once with the required
65-byte digest, empty-output, clean-tree, unchanged-pin, and verified-manifest
checks. Stop on failure or concurrent drift. No retry, ordinary build, candidate,
pin edit, SDK/browser qualification, or PR is authorized.

## Probe result

At exact clean head/upstream `4a92bd8aa0a678ec11e1f4f84bb8acb24395f1ab`,
Hypatia verified main/merge-base `1bcce704`, frozen source identity, fresh paths,
pin shape, toolchains, and capture-control statuses 0 and 1. The single authorized
repin-report builder invocation returned 0. Stdout was exactly 65 bytes containing
digest
`580e3cb4cd11e996598103f27b02d94559f6ef7ad57ef22732d18c0b4f98be10`
plus LF, byte-equal to the delivered pin. The output directory remained empty and
the repository tree and pin remained unchanged; no competing Cargo/rustc process
was present.

The self-excluding manifest under `/tmp/issue666-repin-evidence` verifies and
hashes to
`df937f1bad0fa06747ebca74b781074e69a5db8a46b915a8c1462693d715c307`.
Full streams remain temporary and no compiler or generated artifact payload
entered Git. Astra LOW same-hash applicability review is pending; no ordinary
builder, candidate, pin, SDK/browser, PR, or cleanup action is authorized yet.

## Applicability verdict

Astra LOW returned **PROBE PASS / UNCHANGED-PIN APPLICABILITY PASS** at exact
clean probe source `4a92bd8aa0a678ec11e1f4f84bb8acb24395f1ab` and compact
record head `d1495c46b803f1d2377ccd4392beabd9e05dad97`. The one-shot command,
65-byte same-pin digest, empty output, unchanged tree/pin, and all 13 manifest
entries verify. The builder and copied host-web files are unchanged from main.

Matching Wasm identity plus unchanged copied inputs carries the prior qualified
six-file, browser, and SDK evidence with its original attribution; this probe
supplies no new browser/SDK execution credit. No ordinary builder or pin change
is needed. #666 and #664 may proceed to combined exact-head/current-main PR
readiness, required qualification, guarded merge, post-main qualification, and
GitHub synchronization. Hold all temporary and inherited evidence until final
delivery cleanup review.

## Combined PR readiness

Astra LOW returned **PR-READINESS PASS** for #664/#666 at exact clean pushed
head/upstream `8ddeadcfff342f2e966d5f04aee460b4541e85fc`, live main/merge-base
`1bcce704ca12d531cbdaec4b641acca302543426`, and tracker
`eb7cf145d5d8b6329f8d4b33594253862e159c8c`. The exact seven-path scope,
branch diff, synchronized issue bodies, reviewed source, and unchanged-pin
applicability all pass. One combined PR may proceed after documentation-only
exact-head confirmation, then required qualification and guarded live-head/main
review. Hold all inherited and probe evidence through post-main success and
cleanup review.
