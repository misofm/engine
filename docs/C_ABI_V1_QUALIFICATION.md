# C ABI V1 native qualification

Issue 114 qualifies the joined, accepted Issue-116 native PCM runner and Issue-121 C ABI product.
It does not change or reseal either product. The accepted C header, CAPI/protocol implementation,
runner, runner contract, portability checks, Cargo lock, session fixture, and native runner corpus
are pinned by `fixtures/capi-qualification/v1/AUTHORITIES.sha256`.

The source authority is clean-main commit `feb039765271ca62b0c905004689b88ad92df65b`, tree
`e3e11c343c6f6a5b5b380abe03c0431c6fe81579`. Issue 116 is bound to commit/tree
`45f8f5af8bdd578b5ccb27fdb787f7a663c39818` /
`7e0a7b7d48362c9b9eaa15b1cfce7180c935c5b5`; Issue 121 is bound to commit/tree
`a9a975d8f679707701cc60ad102c817eb54c3082` /
`16728c5ea434dde1a75bdd4500568db8c283a2ca`. The joined header SHA-256 is
`83880c2fd7b5bc835425a5a64cae19c8a0bba17f49b4802b4033a8e7dfeac37c`, and the joined lock
SHA-256 is `c89b195f0d31ad21852d0a931023c70e1eb4a0caa534bfd6e1692c1e1178fd52`.

## Frozen preflight

Tool availability was frozen before the qualification build. The Linux x86_64 host has Rust and
Cargo 1.97.1 with LLVM 22.1.6, GCC/G++ 13.3.0, GNU binutils 2.42, Python 3.12.3, Bash 5.2.21,
strace 6.8, and jq 1.7. Exact paths and installed Rust targets are in `TOOLCHAINS.tsv`.

Cross-target outcomes are candid `UNAVAILABLE`, not compile failures:

- Windows GNU has an installed Rust standard library but no MinGW C/C++ compiler, linker, or
  object inspector.
- Windows MSVC has neither its Rust target nor `cl`, MSVC `link`/`lib`, or `dumpbin`.
  `/usr/bin/link` is GNU coreutils and was explicitly rejected as an MSVC tool.
- macOS x86_64/AArch64 and iOS AArch64 device Rust targets are installed, but `xcrun`, Apple SDK
  linkers, `otool`, and `lipo` are absent.
- The iOS AArch64 simulator Rust target and Apple SDK tools are absent.
- Android AArch64 has its Rust standard library but no Android NDK Clang/linker or LLVM object
  inspectors.

No cross row was executed or relabeled after the preflight. Only the Linux runtime was run.

## Linux artifact and consumer boundary

Fresh `target/capi-qualification/v1` staging was required to be absent. One locked release Cargo
command produced the static and shared libraries there. The accepted header and both libraries
were copied into qualification-owned `installed/` staging and hashed before any consumer linked
them. Existing artifacts elsewhere under `target/` were never inputs.

The same warning-denied source compiled as strict C11 and C++17. Each language linked and ran once
against each frozen library form. The consumer verifies version/layout constants, reserved-zero
rejection, engine/session/plan construction, source generation 1 submission, generation 2 seek and
submission, two render blocks, resource rows, malformed event lane, one-short command canary,
exact command replay bytes, empty reliable egress, and both plan/session destruction orders.
The accepted Rust exported-C regressions supply the complete 11-command, six-event, transactional
replacement, retirement/reclaim, source-preserving/source-changing, failure, replay, and lifecycle
matrix without copying protocol semantics into the qualification consumer.

The first C11-static launch found one qualification-fixture error: it attempted generation-1 seek
before the initial generation-1 submission and exited 13. No product byte or staged library was
changed or rebuilt. The new consumer was corrected to submit generation 1 first, then seek and
submit generation 2; all four consumer rows passed against the same once-built libraries. This is
recorded as one consumer-fixture correction in `QUALIFICATION.tsv`.

GNU `nm` found exactly the 14 frozen `miso_engine_v1_*` definitions in both library forms. The
object parser classifies undefined references separately; a synthetic
mutation replacing a definition with an identically named undefined reference is rejected.

## Runner and realtime evidence

The frozen runner package test command was invoked exactly once. Its 18 tests passed, including
the single test that executes all four RIFF launch rates and representative RF64, compares every
8,192-byte output SHA-256 to the independent manifest, and covers atomic success/failure cleanup
and no-clobber behavior. The runner, its fixtures, and its exclusive-output-directory contract were
not modified, retried, or described as a new Issue-116 seal.

The exported C render audit completed 100,000 calls with stable caller storage and zero allocation,
deallocation, lock, feature-detection, log, file/network I/O, syscall, unwind, or render errors. A
separate functional one-million-block render/swap audit observed two accepted swaps, one retirement
deferral, zero forbidden-operation counters, and zero syscalls between the explicit realtime trace
markers. Neither audit selected a benchmark mode or recorded durations.

The exact matrix is `fixtures/capi-qualification/v1/MATRIX.tsv`. `ARTIFACTS.tsv`, `SYMBOLS.tsv`,
`AUDITS.jsonl`, `QUALIFICATION.tsv`, `CONSUMER_RESULTS.tsv`, `RAW_EVIDENCE.tsv`, `GATES.tsv`, and
`TOOLCHAINS.tsv` contain its independent evidence, and `EVIDENCE.sha256` binds those files. The
semantic checker independently pins every artifact size/hash, symbol set, audit field, result
counter, consumer exit/binary/library binding, raw-log hash, and strict gate; updating the checksum
manifest cannot bless correlated fabricated data. Preserved-stage mode additionally checks the raw
manifest, logs, audit JSON, binaries, libraries, symbols, and armed syscall trace in place. The
checker mutations cover each evidence family as well as authority drift, target omissions,
undefined-reference false positives, fabricated tool availability, timing surfaces, stale staging,
and generated source-tree artifacts.

Final prohibited counters are: benchmark 0, timing 0, playback 0, listening 0, browser 0, and
device 0.

## Issue 369 control-provider refresh

IO-4 replaces the C ABI's conformance-only `MockProvider` with the opt-in
`host_core::SessionControlProvider`. The production parameter catalog is snapshotted directly from
each accepted `EffectPreparedEntry`'s `metadata.descriptor` and
`bank_preparation.initial_values` before graph lowering consumes those entries. Parameter state
therefore uses the exact prepared values; automation domain
checks therefore address real revision-scoped handles. Current/effective sample reads the active
plan's existing release/acquire next-sample publication, while transport remains endpoint-local.
Protocol telemetry counters and the existing bounded CAPI render-diagnostic slots feed the
provider's counter and diagnostic pages. The three provider-owned counters retain an independent
three-slot minimum even when the frame-derived telemetry configuration capacity is smaller.
Candidate catalogs allocate before structural commit and are included in double-live resource
admission. Host-core's default feature graph remains protocol-free; only capi enables the optional
`control-provider` edge.

The `resource_lifecycle` primitive oracle re-derives the soft-clip fixture's provider as 9,072
bytes of descriptor rows, 864 bytes of state rows, 864 bytes of descriptor text, and 282 bytes of
fixed diagnostic projection storage. Active CAPI retained bytes are 160,933; double-live CAPI
admission is 204,375; the 58,804-byte canonical writer remains that fixture's largest named
allocation. The C response vectors now pin session-derived metadata/state and registered telemetry
counter rows. `MockProvider` and `MockProviderConfig` are absent from a normal protocol library
build and available only to unit tests or consumers explicitly selecting `protocol/test-support`.
The exact AudioWorklet rebuild remains protocol-free but changes crate identity because host-core's
declared feature surface changed. The subsequent #371 marker-only integration was rebuilt
and reproducibly qualified as `a89c9606bfa72d69ced42b606cc4b7000d1b53f2b419b12ec63649a385b3eaf1`.
The RT-1 (#399) artifact from source candidate
`e46bc0d1a7917de8c65204cdee931877aea671d8` has SHA-256
`60c23ee23e7f16c1f71c503baa07a462a8ce94c5287bec4580060e27a4651503`; its reproducibility and browser evidence are recorded in #399.
The RT-2 (#419/#422) artifact from source candidate
`0a0e39e42e4ae2585d5f5ee507a4cb9aaf7b741a` has SHA-256
`518b5aa864c0a825cd324112b24270a7e0714fc63db6bd1029779f21066ea9de`.
The independent rebuild, static/resource checks and three-browser matrix passed;
retained workspace and descriptive measurement delivery are recorded in #419.
The RT-3 (#420) artifact from source candidate
`51e2aed211b30523076e0e8dd07973b13b57dc11` has SHA-256
`24f81af304e541ba0e734de5c7a3dc5221e71fa4de73f2545edea3c2960761fe`.
Independent builds, static/resource/mutation checks and all three browser engines passed;
workspace evidence belongs to #420; its uninvoked descriptive measurement is tracked by #436.
The RT-4 public full-chain (#429) artifact from source candidate
`e4bcaa2feae13c9f016bb7b2e1eaff8bd7314547` has SHA-256
`10b0581f72d921b520e4066b82dc32cb7bea90b757c20ccca3dfc52cf7b9e098`.
Independent builds, static/resource/mutation checks and all three browser engines passed;
workspace and supported-Wasm evidence are retained in `artifacts/issue429-qualification`.
Live integration and descriptive full-chain measurement remain in #430 and #431.
The current RT-14 lease cleanup (#435) artifact from source candidate
`69fd0bfb0504075db4d302df08ff480faab4102e` has SHA-256
`766848a4688b2ec34c96e81c243286216a7d7e647b6b42f842c0f85a654fc326`.
Independent builds, static/resource/hermetic checks and all three browser engines passed;
workspace and supported-Wasm evidence are retained in `artifacts/issue435-qualification`.
This is an intentional public Rust lease API retirement; wire/C ABI identities are unchanged.

Refresh gates: `cargo test -p capi`; `cargo test -p capi --test resource_lifecycle`;
`scripts/check-capi-abi.sh`; `scripts/check-abi-layout-v1.py`; and `cargo test --workspace` against
both the issue worktree and `origin/main`. Exact outcomes and the worktree comparison are attached
to issue #369's implementation pull request.

## Live fader/matrix qualification candidate (#430/#459)

The fresh SIMD AudioWorklet digest build from immutable source candidate
`7951736605fa64870bc1d91342d00d5fdb6417c5` produced SHA-256
`a08a868cf1b62bb466a8fa5b826b214fa708265669fc730398706c869c9e43bd`. This is the current candidate pin; independent
rebuild, artifact/resource gates and browser qualification remain pending. The initial
builder invocation refused a missing output directory before compilation (exit 2);
the corrected invocation with an existing empty directory completed successfully.
Historical artifact and qualification records above retain their original identities.
