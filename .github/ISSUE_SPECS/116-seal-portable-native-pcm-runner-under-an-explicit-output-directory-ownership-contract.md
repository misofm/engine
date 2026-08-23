# 116 Seal portable native PCM runner under an explicit output-directory ownership contract

## Outcome and readiness

Accept the stopped Issue-115 technical runner under the smallest honest filesystem contract: the
caller gives the invocation exclusive ownership of the output directory while the runner is live.
Within that boundary, complete portable held-handle/no-replace publication, prove the Linux,
Windows and Apple compile seams, and run one fresh functional runner seal.

**COMPLETE / SOL XHIGH PASS / READY TO CLOSE.** Sol High implemented and sealed the bounded
contract; Sol XHigh supplied the brief, one adversarial HOLD and final verification. The sole
bounded correction closed the known-unowned cleanup evidence defect. The one fresh functional seal
then passed without retry or edit. Benchmark, timing, real workload, playback and listening counts
are all zero.

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

## Terminal implementation and review evidence

The exact implementation checkpoint is commit
`45f8f5af8bdd578b5ccb27fdb787f7a663c39818`, tree
`7e0a7b7d48362c9b9eaa15b1cfce7180c935c5b5`. Its tracked delta is exactly the runner library,
contract document, portability checker and portability mutation suite. The final authorities are:

- runner library `a1395b95f9cab07ec516fd0da2583d7cb3e0083613ed86120cd51b93f4ed805a`;
- contract document `ac80fc2112d3a060a0061eeb3e3db8f4c97aa1bfe475f38f98e49b830a09efc6`;
- portability checker `596f99c66e2a5398bbcf534cfd850b8e5786f55ccbfef6376f7b9111d25a2f42`;
  and
- portability mutations `1e436ad55eabe45425a9bf0563065b05ca426d216dd680dc850a74b3c08cb3ad`.

Frozen authorities remained byte-exact: runner manifest
`bd5cd87f0c2bcd0ae5e7faf5532b1869b39e72f296dee81b2f135345e728a8e1`, runner CLI
`89b2acb0f56c6e249bbbd7bcb965c7c0545fa3c6078772b5026362ec6b888dd0`, fixture manifest
`8d251ad6b1eca8c95e24b8b4e2959e397d8ec954502307351f1c7fb3c01a9634`, installed C header
`e7ba468361e0255cb465828c5dd317f1e5293213662c7bf9a5225cb2afaba4e7`, and `Cargo.lock`
`c89b195f0d31ad21852d0a931023c70e1eb4a0caa534bfd6e1692c1e1178fd52`.

Sol XHigh's focused HOLD found that the fake removed `WrongPublished` and that its policy mutation
was a marker rather than executable cleanup broadening. The sole correction made fake cleanup
remove only `Owned`, froze wrong identity as exact `publish/path.replaced` with the partial absent
and known-unowned final preserved, pinned production final cleanup to the retained identity, and
made the mutation change that live guard to `if true`. The checker rejects the real broadening.
Sol XHigh returned strict focused PASS and authorized the one fresh seal.

## Sole fresh full functional seal

Sol High ran the ordered nonbenchmark seal once on the immutable commit/tree above, with no retry,
edit or alternate invocation. Pre/post branch, HEAD, tree, index, tracked diff and worktree checks
were identical and clean. The seal passed:

- `cargo fmt --all -- --check`;
- locked runner all-target tests: 18 library tests passed, zero binary tests, zero failed, ignored or
  measured;
- independent fixture generator `--check` and an exact manifest oracle with five output rows, each
  8,192 bytes and one lowercase 64-hex digest;
- C ABI header/symbol validation, locked runner all-target check, warning-denied all-target Clippy
  and warning-denied all-feature no-dependency rustdoc;
- Issue-116 portability and original runner checkers plus both mutation suites;
- workspace and realtime policies plus their mutations, syntax for every tracked shell script, and
  frozen-product/diff/conflict/artifact/final-identity scans; and
- compile-only runner-library checks for `x86_64-unknown-linux-gnu`,
  `x86_64-pc-windows-gnu` and `aarch64-apple-darwin`.

The real C ABI fixture row verified these exact output digests:

- `rf64-48000`: `bc5feb6a7706ff56dcb0015c2ebe8f35bd609ba49489f61c894d7248b09d8d29`;
- `riff-44100`: `f468dd547cb9b63fb9c582f5e4388e1e8caa262b39dd9878a892087e28886f50`;
- `riff-48000`: `9e19d4279126b7a6374d8868d8a9741bfb4bd55f6575b6335f5548759d51ed34`;
- `riff-88200`: `aac8064f01239d981951a51fe9b0edc9e44f014dbd1cacc00fbcf5407d6951f7`;
  and
- `riff-96000`: `965aee1857cb901404e849e58a006da39ce8a51d04609dd60fbc7b1028b9339b`.

Benchmark, timing, real-workload, playback and listening invocation counts are exactly zero. This
is the required functional seal, not performance or listening evidence. Issue 116 is **COMPLETE**
with **SOL XHIGH PASS** and **READY TO CLOSE**. It unblocks Issue 114 together with accepted Issue
113. Upstream evidence synchronization and remote closure remain root work after this docs-only
checkpoint; they are not claimed here.
