# 114 Qualify native C ABI and reference runner target matrix

## Outcome and readiness

Qualify the completed native C ABI control/render product and WAV/RF64 reference runner across the
launch native target boundary without changing product bytes or rerunning descriptive benchmarks.

**COMPLETE / SOL XHIGH PASS / READY TO CLOSE AFTER UPSTREAM AND CI SYNCHRONIZATION.** Issues 116
and 121 were qualified as one frozen candidate. Sol High completed the qualification-only pass and
the sole bounded correction; Sol XHigh returned strict terminal PASS. Benchmark and timing
invocation counts remained zero. Remote evidence synchronization and closure remain root-owned;
this record makes no GitHub change.

## Dependencies by exact title

- **Seal portable native PCM runner under an explicit output-directory ownership contract**
  (Issue 116)
- **Close CAPI-owned render events and primitive replacement resource authority** (Issue 121)

Stopped **Close portable native PCM runner publication and seal the reference tool** (Issue 115)
contributes technical input through accepted Issue 116 only; it is not a dependency and cannot be
qualified here.

Stopped **Close C ABI control/event transport and transactional plan replacement** (Issue 113) and
stopped **Complete C ABI transactions with two-phase protocol and plan reservations** (Issue 117),
stopped **Close C ABI replacement resource accounting and cross-component evidence** (Issue 118),
stopped **Preallocate C ABI controller resources and independently seal replacement semantics**
(Issue 119), and stopped **Seal production-identical C ABI replacement evidence and lifecycle
ownership** (Issue 120) contribute technical input through Issue 121 only. None is an accepted
dependency and none can be qualified here.

This issue gates **End-to-end release, performance, and listening qualification** (Issue 026). It
does not gate Issue 025, which consumes accepted Issue 121 directly.

## Frozen accepted candidate authorities

Qualification starts from clean synchronized `main` commit
`feb039765271ca62b0c905004689b88ad92df65b`, tree
`e3e11c343c6f6a5b5b380abe03c0431c6fe81579`. Every staged header, library, consumer, runner and
fixture result must be traceable to that exact source tree. Target artifacts already present under
`target/` are not authorities and must not be reused: they predate the final Issue-121 technical
checkpoint and are stale qualification inputs.

Accepted Issue 116 is bound by implementation commit
`45f8f5af8bdd578b5ccb27fdb787f7a663c39818`, tree
`7e0a7b7d48362c9b9eaa15b1cfce7180c935c5b5`, and these surviving runner authorities:

- runner library `a1395b95f9cab07ec516fd0da2583d7cb3e0083613ed86120cd51b93f4ed805a`;
- runner contract `ac80fc2112d3a060a0061eeb3e3db8f4c97aa1bfe475f38f98e49b830a09efc6`;
- portability checker `596f99c66e2a5398bbcf534cfd850b8e5786f55ccbfef6376f7b9111d25a2f42`;
- portability mutations `1e436ad55eabe45425a9bf0563065b05ca426d216dd680dc850a74b3c08cb3ad`;
- runner manifest `bd5cd87f0c2bcd0ae5e7faf5532b1869b39e72f296dee81b2f135345e728a8e1`,
  CLI `89b2acb0f56c6e249bbbd7bcb965c7c0545fa3c6078772b5026362ec6b888dd0`, and fixture manifest
  `8d251ad6b1eca8c95e24b8b4e2959e397d8ec954502307351f1c7fb3c01a9634`.

Accepted Issue 121 is bound by technical commit
`a9a975d8f679707701cc60ad102c817eb54c3082`, tree
`16728c5ea434dde1a75bdd4500568db8c283a2ca`, with exact implementation hashes:

- CAPI runtime `79ccb21cffa18e731e40dc8b8457f0dc58851c7e8c401dd6292510d5de71ae50`;
- external resource/lifecycle evidence
  `c4ac50a3bb397f5714f2ea1cba83c273554b30c301da648bafaa973aaa2b95d7`;
- protocol controller `7a4fa3549c611ef9f1c88ec0b0db0cb84ae8231bed22da7e16204664d275346a`;
  and
- protocol exports `bea15fa82401faa72af6f617bd6ed7d59ceab97e90a987a0bac1f1fd49888f1f`.

The joined aggregate authorities at the clean-main boundary are C header
`83880c2fd7b5bc835425a5a64cae19c8a0bba17f49b4802b4033a8e7dfeac37c` and `Cargo.lock`
`c89b195f0d31ad21852d0a931023c70e1eb4a0caa534bfd6e1692c1e1178fd52`. Issue 116's earlier
header hash is historical runner-seal evidence, not the post-Issue-121 joined ABI authority.

## Qualification-only matrix

Freeze the accepted #116/#121 candidate before implementation and pin the installed header, static/
shared libraries, runner binary/source, Cargo locks, fixture manifests and protocol/session corpora.
No product correction is permitted here. Static and shared libraries must be produced once into a
new qualification-owned staging directory from the frozen main tree, hashed before consumer use,
and never substituted with an existing `target/` artifact.

The exact matrix is:

- Linux x86_64: build and run C11 plus C++17 consumers against installed static and shared
  libraries; enumerate the exact exported V1 symbol set; verify ABI version, struct sizes/offsets,
  reserved-zero behavior, command/response/event buffers, source submit/seek, replacement boundary,
  retirement and destroy order;
- macOS x86_64 and AArch64 plus Windows GNU/MSVC: compile/link equivalent consumers and inspect
  exact symbols/imports without claiming a runtime where the environment lacks it;
- Android AArch64 and iOS AArch64 simulator/device compile boundaries: build the C ABI libraries,
  header consumer and reference-runner library portion without executing device code; and
- native runner: execute the frozen four-rate RIFF plus representative RF64 corpus and compare exact
  block-planar size/SHA-256, diagnostics and atomic no-clobber behavior to its independent manifest.

Each row records `PASS`, `FAIL`, or `UNAVAILABLE` with exact toolchain/target identity; unavailable
is allowed only for a missing installed target/tool and may not hide a compile or product failure.
Cross-compiled object inspection must distinguish exports from imports/internal references and use
synthetic parser mutations.

## Readiness and toolchain preconditions

Read-only inspection on 2026-08-23 found a Linux x86_64 host with Rust/Cargo 1.97.1 and LLVM 22.1.6.
Installed Rust standard-library targets are `x86_64-unknown-linux-gnu`,
`x86_64-pc-windows-gnu`, `x86_64-apple-darwin`, `aarch64-apple-darwin`,
`aarch64-apple-ios`, `aarch64-linux-android` and `wasm32-unknown-unknown`. Host GCC/G++ 13.3.0,
GNU ar/nm/readelf/objdump 2.42, Python 3.12.3 and Bash 5.2.21 are available, so the Linux C11/C++17,
ELF symbol/parser and host qualification-tool rows are implementable.

The following are candidly absent at rebrief time:

- the `x86_64-pc-windows-msvc` Rust target and MSVC `cl`/`link`/`lib`/`dumpbin` tools;
- MinGW C/C++ linkers and object tools despite the installed Windows GNU Rust target;
- Apple `xcrun`, SDK linker, `otool` and `lipo` despite installed macOS/iOS Rust targets;
- `aarch64-apple-ios-sim`, so the simulator row lacks even its Rust target;
- Android NDK Clang/linker despite the installed Android Rust target; and
- LLVM `llvm-nm`, `llvm-readobj` and `llvm-objdump`.

Sol High must preflight and freeze this availability table before any qualification build. A tool
installed before that freeze may be used with exact path/version recorded. Otherwise its dependent
row is `UNAVAILABLE`; invoking a missing linker or SDK and relabeling the resulting failure is not
allowed. Linux runtime is the only runtime row authorized on this host. Cross rows are compile/link
and object-inspection only, never emulated or device runtime claims. These absences do not block a
candid matrix PASS, but a present toolchain that exposes a product or checker failure is `FAIL`, not
`UNAVAILABLE`.

## Adversarial evidence

Run non-timed representative source lifetime, repeated command/event/replay, source-changing and
source-preserving replacement, full-retirement deferral/reclamation, destroy-order, malformed C
input, one-short buffer/limit and one-million render/swap audit rows. Audit counts are functional,
not performance measurements. Prove no allocation/free, lock, syscall, I/O, log, callback or render-
thread destruction in armed render intervals. The runner's accepted-output lifecycle must remain
atomic/no-clobber on success and every injected failure.

The checker rejects changed product/fixture authorities, missing/extra symbols, target omissions,
false-positive object references, fabricated unavailable rows, stale artifacts and any timing or
benchmark entry. Shell/parser mutations must prove those rejects.

Issue 116's prohibition on another Issue-116 seal or retry remains in force. This issue may execute
the frozen runner corpus exactly as its own qualification row; it must not modify the runner, call
the row a new Issue-116 seal, retry it, or broaden its exclusive-output-directory contract.

## Allowed paths and gates

Allowed changes are a new `tools/miso-engine-capi-qualification/**` package if needed,
`fixtures/capi-qualification/v1/**`, `docs/C_ABI_V1_QUALIFICATION.md`, new exact target/qualification
checker and mutation scripts, minimal manifests/lock rows, and this issue's evidence/routing docs.
Existing CAPI/source/protocol/session/graph/DSP code, installed headers, Issue-116 runner code and
all accepted fixtures are read-only.

Gates are locked compile/test for the qualification tool, warning-denied Clippy/rustdoc, installed
header/library and exact target matrix, fixture/manifest validation, realtime/policy mutations,
shell syntax and clean/static/diff scans. No benchmark target, timer, tuning, listening, browser or
device workload is allowed. Sol High hands off one immutable qualification checkpoint; Sol XHigh
returns strict PASS or the sole bounded HOLD. Overall PASS requires exact candid matrix evidence and
then unblocks Issue 026.

## Sol High qualification evidence — 2026-08-23

The focused qualification checkpoint changes only this issue record, qualification fixtures,
qualification scripts, and the qualification report. The accepted C API product/header, protocol,
Issue-116 runner, Cargo lock, and accepted fixtures remain byte-identical to the authorities pinned
in `fixtures/capi-qualification/v1/AUTHORITIES.sha256`.

Frozen preflight found a complete Linux x86_64 runtime toolchain and no usable MinGW, MSVC, Apple
SDK, iOS simulator, or Android NDK link/object-inspection toolchain. The resulting matrix records ten
Linux rows as `PASS` and seven cross-target rows as candid `UNAVAILABLE`; none of the unavailable
rows masks an attempted compile or product failure.

One fresh locked release build produced and hashed the installed static and shared C API libraries
before consumer use. Strict C11 and C++17 consumers each linked and ran against both forms. GNU
`nm` found exactly the fourteen frozen V1 definitions in each form and zero same-prefix imports.
The object checker independently distinguishes definitions from imports and rejects a
synthetic undefined-reference false positive.

The first C11-static consumer launch exposed a new qualification-consumer ordering error (exit 13):
it sought generation 1 before its initial submission. Only the new consumer was corrected; neither
library nor any frozen authority was rebuilt or changed. The corrected C11/C++17 static/shared rows
all passed against the original once-built libraries.

The locked C API/protocol regressions passed 18 C API unit tests, 3 exported-C tests, 93 protocol
unit tests, and the one-million-case protocol mutation test. The frozen runner corpus was invoked
exactly once and all 18 tests passed, including four launch-rate RIFF rows, representative RF64,
exact 8,192-byte output hashes, and atomic failure/no-clobber behavior. It was not retried.

The exported-C render audit completed 100,000 calls with every forbidden-operation counter zero.
The functional realtime audit completed 1,000,000 render/swap blocks, observed two accepted swaps
and one full-retirement deferral, and found zero allocation, deallocation, lock, log, I/O, network,
syscall, or total violations; the armed trace contained zero syscalls. Benchmark, timing, playback,
listening, browser, and device invocation counts are all zero.

The authoritative human report is `docs/C_ABI_V1_QUALIFICATION.md`. Exact artifact hashes, symbol
lists, toolchain identities, audit records, execution counts, consumer exit/binary/library bindings,
raw-stage hashes, strict gates, and the complete target matrix are bound by
`fixtures/capi-qualification/v1/EVIDENCE.sha256` and independently pinned by the semantic checker.
Its preserved-stage mode cross-checks those records against the immutable generated manifest, raw
logs/audit JSON, binaries, libraries, symbols, and armed syscall trace.

## Sol XHigh review and terminal verdict

Pass 1 received the sole bounded **HOLD**. The first checker bound evidence-file hashes and matrix
shape but did not semantically validate the artifact, symbol, audit, execution-counter or strict-
gate records. The corrected consumers were not durably tied to their final binaries and libraries,
the report overstated unparsed `readelf` output, and the remaining strict gate results were not part
of the evidence seal.

Sol High corrected only qualification evidence and checker surfaces without rebuilding the product
or rerunning any consumer, runner, CAPI audit or realtime workload. The immutable technical
checkpoint is commit `7a7b3c1862cacf1387471f64209a0994261e7262`, tree
`d00b08ed41641f11ce08a9edd8d828cd3dcf2430`. Its final semantic authority includes:

- evidence manifest `6719a5027046695becd74696564d59392c8572faa4c6f6e003e5de943f1fac42`;
- semantic evidence checker `6c36bdc4188cf24003c6d36e99ffa9c1c20d71ecdfafdc7897cbeaf4725c51fc`;
- final qualification checker `5c9c50a662b500ef0ef05b50dd2de0ff51c892bac0edf3bb224d4ba2f2ccfe69`;
  and
- correlated mutation suite `e67d09a79e372133ff995f2054251d762427b7c609e9ac4607006e94c2d0d759`.

The semantic checker independently pins every artifact size/hash; both exact 14-symbol GNU `nm`
definition sets with zero same-prefix imports; both complete audit objects; all qualification,
execution and prohibited counters; each corrected consumer's source/header/library/binary/log and
zero exit; the preserved raw manifest/log/audit/binary/library/symbol/trace inventory; and all 22
strict gates. Correlated mutations recompute the checksum manifest after altering each artifact,
symbol, audit, count, prohibited-count, consumer, raw-stage, strict-gate and matrix family, and each
is rejected. The unsupported `readelf` independence claim was narrowed to the exact validated `nm`
claim.

The final matrix remains exactly ten Linux `PASS` rows and seven candid cross-target `UNAVAILABLE`
rows. Execution counters are exactly: one product build, one consumer-fixture correction after the
recorded initial C11-static exit 13, four corrected consumer passes, one combined CAPI/protocol test
invocation, one frozen runner-corpus invocation, one 100,000-call exported-C audit and one
1,000,000-block render/swap audit. The product build, runner corpus, consumers and functional audits
were not rerun during the correction. Benchmark, timing, playback, listening, browser and device
invocation counts are all zero.

Sol XHigh's terminal read-only review returned strict **PASS**. The accepted C API product/header,
protocol, runner, runner contract, Cargo lock and accepted fixtures remained byte-identical; the
allowed qualification-only fence and candid tool identities are exact. Issue 114 is therefore
complete and ready to close after this evidence is upstream, required CI is green and the GitHub
issue is synchronized. At that point it unblocks **End-to-end release, performance, and listening
qualification** (Issue 026).
