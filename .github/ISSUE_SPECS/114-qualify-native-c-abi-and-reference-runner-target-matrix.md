# 114 Qualify native C ABI and reference runner target matrix

## Outcome and readiness

Qualify the completed native C ABI control/render product and WAV/RF64 reference runner across the
launch native target boundary without changing product bytes or rerunning descriptive benchmarks.

**STATELESS SOL XHIGH BRIEF / READY FOR SOL HIGH PASS 1.** Issues 116 and 121 are closed with strict
Sol XHigh PASS, so their joined qualification dependency is satisfied. Sol High implements; Sol
XHigh briefs and verifies. One pass plus one bounded HOLD correction is the complete budget. A
second material HOLD stops. Benchmark and timing invocation counts are zero and must remain zero.
Remote issue synchronization remains root-owned; this rebrief makes no GitHub change.

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
