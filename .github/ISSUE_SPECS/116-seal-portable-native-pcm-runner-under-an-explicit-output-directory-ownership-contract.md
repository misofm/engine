# 116 Seal portable native PCM runner under an explicit output-directory ownership contract

## Outcome and readiness

Accept the stopped Issue-115 technical runner under the smallest honest filesystem contract: the
caller gives the invocation exclusive ownership of the output directory while the runner is live.
Within that boundary, complete portable held-handle/no-replace publication, prove the Linux,
Windows and Apple compile seams, and run one fresh functional runner seal.

**STATELESS SOL XHIGH BRIEF / READY.** Sol High implements; Sol XHigh briefs and adversarially
verifies. One implementation pass plus one bounded HOLD correction is the complete budget; a second
HOLD stops. Benchmark, timing, real workload, playback and listening counts start at and remain
zero.

Read-only inspection on 2026-08-23 found remote Issue 116 unallocated. Root must create and
synchronize it under this exact title after the docs checkpoint is upstream. This record performs
no Git or GitHub mutation.

## Dependencies and technical input by exact title

Accepted product dependencies:

- **Stable C ABI and host-fed planar PCM render** (Issue 022)
- **Close native-source seek submission qualification and seal backpressure fix** (Issue 112)

Stopped **Close portable native PCM runner publication and seal the reference tool** (Issue 115)
is technical input, not an accepted dependency. Preserve checkpoint
`99f081f327bd250343a80f928aa099c994ca8e59` and its exact runner/checker/docs hashes, but do not
inherit its impossible claim against a concurrent same-privilege directory-entry mutation. PASS
here plus accepted **Close C ABI control/event transport and transactional plan replacement**
(Issue 113) gates **Qualify native C ABI and reference runner target matrix** (Issue 114).

## Explicit output-directory ownership contract

The caller must supply an existing output directory exclusively owned for the complete runner
invocation. From entry through return, no other thread or process may create, remove, rename, link,
replace, chmod or otherwise mutate any directory entry in that directory. The runner does not and
cannot infer this authority from Unix ownership bits, ACLs or process identity; it documents the
precondition and treats violation as outside its guarantee.

This issue makes no claim that a pathname can be identity-conditionally unlinked while a
same-privilege actor concurrently mutates the directory. It must not describe the resulting
check/use window as closed. Under the exclusive-directory precondition, the runner must still:

1. reject an existing final of every kind without modifying it;
2. create the partial with create-new semantics, retain its exact handle through write, flush,
   sync, length and digest verification, and accept only that identity;
3. publish with an atomic OS-native no-replace operation;
4. verify that the final is the held identity, remove only the runner-owned partial/final as the
   outcome requires, and leave exactly one accepted final on success;
5. clean boundedly on every write/verification/publication failure without retry, globbing or
   deleting a pre-existing sentinel; and
6. return stable phase/code diagnostics without addresses or environment-dependent paths.

There is no hidden lock, directory lease, global singleton or public API/resource-shape change.
Concurrent writers require external coordination and remain outside this tool's contract.

## Portable publication boundary

Keep explicit target adapters and the shared bounded publication state machine:

- Linux/Android: held-file `linkat(AT_EMPTY_PATH)` followed by owned-partial cleanup, or an equally
  exact `renameat2` no-replace design selected and documented under the ownership precondition;
- Apple Unix: an explicit `renamex_np` exclusive strategy or a documented held-handle `linkat`
  strategy with no-replace behavior and post-publication identity binding;
- Windows: retained `HANDLE` identity plus `FileLinkInfo`/`SetFileInformationByHandle` with
  replacement disabled, followed by handle-based identity confirmation; and
- every other native target: `preflight/platform.unsupported` before partial creation, source
  resolution or engine invocation.

Fixed OS declarations/constants remain behind the tool's existing unsafe allowlist. Unix imports
must remain cfg-guarded. Publication does not use a generic pathname hard-link fallback, does not
overwrite any final kind and does not introduce a new dependency or system binding unless the
focused target compile proves it necessary.

## Adversarial matrix under the stated boundary

Retain all Issue-073/115 functional tests and add nonvacuous rows for the exact supported contract:

- pre-existing regular, symlink, hardlink and directory final sentinels remain unchanged;
- controlled regular/symlink/hardlink/rename substitution before and after each defined state-
  machine phase is detected and preserved before the next phase begins;
- wrong-identity publication, short write, flush, sync, length, digest, publication, partial-cleanup
  and final-verification failures freeze exact phase/code and bounded disposition;
- a second invocation cannot overwrite the accepted final;
- success has exact bytes/digest, held/final identity, final link count as applicable, no partial
  and no retry; and
- unsupported-platform preflight performs no output, source, engine or publication action.

The fakes must not pretend to prove an atomic identity-conditional pathname unlink. Tests and docs
must explicitly state that no external directory mutation occurs inside an OS path operation under
the exclusive-directory contract. Static mutations reject reintroduction of the broad Issue-115
concurrency claim, replace-enabled flags, unguarded Unix imports, generic pathname publication,
unchecked final identity and cleanup of a known-unowned entry.

## Focused gates and fresh seal

Focused evidence includes the complete runner tests; independent fixture generator/manifest; C ABI
header/symbol checker; locked host all-target check; warning-denied Clippy/rustdoc; format;
workspace and realtime policies/mutations; successor checker/mutations; shell syntax; frozen-product
diff fence; and compile-only runner-library checks for `x86_64-unknown-linux-gnu`,
`x86_64-pc-windows-gnu` and `aarch64-apple-darwin`. Missing installed tooling is candidly
`UNAVAILABLE`, not PASS; Windows and Apple compile rows are mandatory for issue PASS.

After a strict Sol XHigh focused PASS and clean exact-path checkpoint, run one fresh full runner
nonbenchmark seal on that immutable candidate. It runs the complete retained functional test set,
five real fixture outputs with exact manifest bytes/digests, and all focused/static/target gates
once. Stop on the first failure; no retry exists outside the sole bounded HOLD correction. These are
functional fixture invocations, not benchmark, timing or real workload evidence.

## Frozen boundary and allowed paths

Frozen byte/semantic surfaces: C API/source/protocol/session/graph/DSP crates, installed header,
runner CLI/source resolution/decode/feed/render/diagnostics, fixture WAV/TOML/PCM bytes and digests,
`tools/miso-engine-native-pcm-runner/src/main.rs`, accepted corpora and benchmark inputs.

Allowed edits:

- publication/identity/cleanup code plus colocated tests in
  `tools/miso-engine-native-pcm-runner/src/lib.rs`;
- `docs/NATIVE_PCM_REFERENCE_RUNNER_V1.md` for the exact ownership contract;
- the Issue-115 portability checker/mutations and at most one Issue-116-specific checker/mutation
  pair;
- runner/root manifests and `Cargo.lock` only if an unavoidable minimal target system binding is
  proved and all frozen semantic authorities remain unchanged; and
- this spec/brief plus exact README/implementation-plan routing/evidence.

Any decoder, C ABI, source, fixture/output digest, CLI argument, resource or engine behavior change
is STOP. No benchmark, timing, tuning, real workload, audio playback, listening, browser/device
runtime or Issue-114 broad matrix is authorized.

## Acceptance

Issue 116 passes only when the exclusive-directory contract is literal and consistent across code,
docs, tests and diagnostics; portable no-replace publication and bounded cleanup satisfy that
contract; Linux/Windows/Apple compile rows pass; Sol XHigh verifies the sole fresh full runner seal;
and all prohibited counters remain zero. PASS unblocks Issue 114 together with accepted Issue 113.
It does not rehabilitate Issue 115 or claim safety against concurrent same-privilege directory
mutation.
