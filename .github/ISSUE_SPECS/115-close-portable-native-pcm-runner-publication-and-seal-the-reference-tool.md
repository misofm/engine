# 115 Close portable native PCM runner publication and seal the reference tool

## Outcome and readiness

Accept the stopped Issue-073 reference runner without weakening its output contract: replace the
non-Linux pathname check/use fallback with a platform-specific publication boundary whose accepted
final is bound to the exact held file identity, prove Windows and Unix compilation, then run one
fresh full nonbenchmark runner seal.

**STATELESS SOL XHIGH BRIEF / READY AFTER ISSUE-073 STOP CHECKPOINT.** Sol High implements; Sol
XHigh briefs and adversarially verifies. One implementation pass plus one bounded HOLD correction
is the complete budget; a second HOLD stops. Benchmark, timing, real workload, playback and
listening counts start at and remain zero.

Read-only inspection on 2026-08-23 found remote Issue 115 unallocated. Root must create and
synchronize it under this exact title after the docs checkpoint is upstream. This record performs
no Git or GitHub mutation.

## Dependencies and technical input by exact title

Accepted product dependencies:

- **Stable C ABI and host-fed planar PCM render** (Issue 022)
- **Close native-source seek submission qualification and seal backpressure fix** (Issue 112)

Stopped **Native WAV/RF64 reference runner over frozen C ABI V1** (Issue 073) is technical input,
not an accepted dependency. Preserve its exact runner/manifest/checker hashes and 15-test focused
evidence from the terminal record. Issue 114 must depend on accepted Issue 115 plus Issue 113; it
may not qualify or repair stopped Issue 073.

## Sole product correction

Keep the Linux held-descriptor `linkat(AT_EMPTY_PATH)` path. Replace the generic non-Linux
`path_is_owned` then pathname `hard_link` fallback. Every supported target adapter must satisfy all
of these rules:

1. retain the exact create-new partial file handle through write, sync, length and digest
   verification;
2. publish with an OS primitive that is no-replace atomically and either names that held handle
   directly or atomically moves a private completed entry and binds the published result back to
   the held identity before success;
3. never accept a different file identity, never overwrite any final kind, and never delete a
   pathname unless it is proven to be the runner-owned identity or was atomically created by that
   exact publication operation;
4. on every reported failure, the requested final is absent and any concurrently substituted
   regular/symlink/hardlink sentinel remains byte/shape unchanged; and
5. remove the runner-owned partial only after successful publication leaves one final name.

Use explicit `cfg` adapters for Linux/Android, Apple Unix and Windows. No generic path-check-plus-
link fallback is permitted. A native OS without a proved adapter must return a stable
`preflight/platform.unsupported` before creating a partial or invoking the engine. Platform code
must use fixed OS declarations/constants behind the tool's existing unsafe allowlist; no new public
Rust or C ABI is introduced.

The identity abstraction must compile on Unix and Windows without an unconditional
`std::os::unix` import. Windows and Unix compare the strongest stable file identity available from
the held handle and post-publication handle. Exact result/error mapping remains address/path-free.

## Adversarial publication matrix

Retain every Issue-073 test and add nonvacuous adapter tests for:

- regular, symlink, hardlink and rename replacement immediately before publication;
- replacement between every precheck and path-based OS operation;
- final-name collision immediately before publication for every file kind;
- wrong-identity publication injection, partial unlink failure and final verification failure;
- short write, flush/sync, length/digest and OS publication failure; and
- success with exact bytes/digest, final link count/identity as applicable, no partial and no
  second public call overwriting the final.

Each failure freezes phase/code, zero accepted final, owned-only cleanup and sentinel preservation.
The platform-independent state machine/fake adapter must exercise every row on Linux; the real
Linux adapter exercises actual held-FD publication. Target-specific compile checks and static
mutation scans must reject unguarded Unix APIs, generic pathname hard-link fallback, replace-enabled
flags, unchecked identity and cleanup by unowned path.

## Windows/Unix compile boundary and full runner seal

Focused gates include the complete runner tests; independent fixture generator/manifest; C ABI
header/symbol checker; locked host all-target check; strict Clippy/rustdoc; format; realtime and
workspace policies/mutations; checker mutations; shell syntax; and compile-only library checks for
`x86_64-unknown-linux-gnu`, `x86_64-pc-windows-gnu` and `aarch64-apple-darwin`. Android/iOS runtime and broad target
qualification remain Issue 114. If an exact target is unavailable, the evidence must identify the
missing installed target and may not claim that row; Issue 115 cannot PASS until the Windows and
Apple Unix compile rows both pass once.

After Sol XHigh focused PASS and a clean exact-path checkpoint, run one fresh full runner
nonbenchmark seal on that immutable candidate: all original 15 Issue-073 tests plus successor rows,
the five real fixture outputs and all focused/static/target gates above. Stop on the first failure;
there is no retry outside the sole bounded HOLD correction. This is functional fixture evidence,
not a real workload or timing run.

## Frozen boundary and allowed paths

Frozen byte/semantic surfaces: C API/source/protocol/session/graph/DSP crates, installed header,
runner CLI/source resolution/decode/feed/render/diagnostics, all Issue-073 fixtures and expected PCM
hashes, `src/main.rs`, benchmark inputs and accepted corpora.

Allowed edits:

- publication/identity code plus colocated tests in
  `tools/miso-engine-native-pcm-runner/src/lib.rs`;
- the runner manifest and root manifest/lock only for minimal target-specific system bindings;
- `docs/NATIVE_PCM_REFERENCE_RUNNER_V1.md`;
- the existing runner/realtime checker and mutation scripts, plus at most one successor-specific
  portability checker and mutation script;
- this spec/brief and exact README/implementation-plan routing/evidence.

Any decoder, C ABI, fixture/output digest, command, resource or product behavior change is STOP.
No benchmark, timing, tuning, audio playback, listening, browser/device runtime or broad Issue-114
matrix is authorized.

## Acceptance

Issue 115 passes only when the portable publication path has no ownership check/use gap, both
Windows and Apple Unix compile boundaries pass, Sol XHigh verifies the full runner seal, and all
prohibited counters remain zero. Focused green without the fresh seal is not overall PASS. PASS
unblocks Issue 114; it does not claim Issue-113 control transport or cross-target qualification.
