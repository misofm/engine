# Prove or correct fpenv's realtime-cost and DAW-callback claims against the shipped x86_64 codegen

Stream G follow-up of decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-0). Filed
2026-10-09 by root from the #1489 verdicts' open item 2
(`/home/bl/misofm/submix-verdicts/1489-attempt1.md`, "Open items for root", item 2;
`/home/bl/misofm/submix-verdicts/1489-attempt2.md`, "Open items for root") and the batch-misc2
verdict's NIT on the DAW-callback claim
(`/home/bl/misofm/submix-verdicts/batch-misc2-batch-verdict.md`, "Not blockers").

Root's ruling (2026-10-09), verbatim:

> (3) File both successors: [...] (b) correct or prove fpenv.rs:73-75 and :16-17 against the
> shipped x86_64 cdylib's codegen.

Smallest slice: make the two passages of the `fpenv` module doc true, each sentence backed by the
shipped `x86_64` cdylib's disassembly or by a primary source, or removed. Comment lines only; no
codegen change.

## Problem (verified on `codex/d15-batch-misc2` at `a998adf21`)

- **`fpenv.rs:73-76`, "Realtime properties".** "Entering and leaving the guard is a register read,
  two register writes and two empty assembly blocks that emit nothing. No allocation, no lock, no
  syscall, no call at all: the bodies are `#[inline]` and the `Drop` is a barrier and a single
  store." On the shipped `x86_64` cdylib this is false:
  - `lane::softfma::read_mxcsr` and `write_mxcsr` (`crates/lane/src/softfma.rs:87`, `:103`) carry
    no `#[inline]`, and the C render entry calls both out of line. Root's re-check
    (`cargo build --locked --release -p capi`, `objdump -d target/release/libcapi.so`, at
    `a998adf21`): `miso_engine_v1_render_f32_planar` (`0x80880`) makes indirect calls through the
    GOT slots `0x43b740` (relocated to `read_mxcsr`, `0x3e6bb0`) at `0x80897` and `0x80928`, and
    `0x43b748` (relocated to `write_mxcsr`, `0x3e6bd0`) at `0x808a4` (after `mov $0x1f80,%edi`) and
    `0x808f6`. This matches the #1489 attempt-1 verdict (`0x80897`, `0x808a4`; "Drop makes a
    third").
  - The helpers are stack-slot forms, not register moves: `read_mxcsr` is
    `movl $0x0,-0x4(%rsp); vstmxcsr -0x4(%rsp); mov -0x4(%rsp),%eax; ret` and `write_mxcsr` is
    `mov %edi,-0x4(%rsp); vldmxcsr -0x4(%rsp); ret` (`LDMXCSR`/`STMXCSR` take only a memory
    operand). The `Drop` is a barrier and a call, not "a single store".
  - The C entry's own comment repeats the count ("One control-word read, two writes and two empty
    assembly barriers per block", `crates/capi/src/ffi.rs:813-816`); it is in scope for the same
    check, because it is the claim the policy section points readers to.
- **`fpenv.rs:16-17`.** "every DAW audio callback arrives with FTZ and DAZ already set" is a
  universal claim with no evidence in the file. The batch verdict graded the same claim, quoted in
  `softfma.rs:116-117`, as unevidenced; root's `f23041842` reworded the `softfma.rs` copy to "as a
  host audio callback that runs with FTZ and DAZ set does (`fpenv.rs:16-17`)", so it now cites this
  line. The soundness argument does not need the claim; the module's motivation does (why native
  hosts are exposed).
- **What is not in question.** The barrier blocks emit nothing (`fpenv.rs:322-329`; the #1489
  verdict confirms the only `asm!` sites are `fpenv.rs:167`, `:182`, `:327`); no allocation, lock
  or syscall (C ABI caller audit: 0 allocations, 0 locks, 0 syscalls).

## Decisions

- **D1. The disassembly decides.** The implementer builds the shipped cdylib
  (`cargo build --locked --release -p capi`, the shipped feature set: `qualification.yml:876`
  notes that `check-capi-abi.sh` builds `-p capi` alone for that reason) and reads the
  render entry and both helpers. For AArch64 the shape is read from the cross-compiled release
  object (`cargo rustc --locked --release -p capi --target aarch64-linux-android -- --emit asm`, or
  the iOS target); that is a static read, not emulation. The attempt record holds both excerpts.
- **D2. Correct the text; do not change the code.** `fpenv.rs:73-76` states the shape D1
  measured, per target: on `x86_64`, how many out-of-line helper calls the entry and each exit path
  make and that each helper is a stack-slot `STMXCSR`/`LDMXCSR`; on AArch64, what the
  cross-compiled object shows; no allocation, lock or syscall. Adding `#[inline]` to the `softfma`
  helpers to make the old text true is out of scope: it changes the C ABI's emitted code, the
  `artifacts/issue146/` measurement already covers today's code, and no budget is missed.
  `capi/src/ffi.rs:813-816` is checked against the same disassembly and corrected only if false.
- **D3. `fpenv.rs:16-17` is either cited or narrowed.** Keep a universal claim only with a primary
  source for it. Otherwise narrow it to what the engine needs: a native host's audio thread can run
  with FTZ and DAZ set (for example, a host that sets them to avoid denormal stalls), and nothing in
  the C ABI contract forbids it. `softfma.rs:116-117` keeps citing `fpenv.rs:16-17`; if the line
  numbers move, update that citation in the same commit.
- **D4. Keep line counts.** Comment lines only. Each edited passage keeps its line count, so every
  `fpenv.rs` and `softfma.rs` line citation in `docs/REALTIME_DEPENDENCY_POLICY.md` stays exact
  (`fpenv.rs:141`, `:148`, `:166`, `:181`, `:322-329`). If a passage cannot keep its count, the
  policy's citations are refreshed in the same commit.

## Authorized paths

- `crates/lane/src/fpenv.rs` (module doc lines `:10-17` and `:71-76` only)
- `crates/lane/src/softfma.rs` (the `fpenv.rs:16-17` citation in `write_mxcsr`'s `SAFETY`
  comment, only if D3 or D4 moves it)
- `crates/capi/src/ffi.rs` (B's; the comment `:813-816` only, only if D2 finds it false)
- `docs/REALTIME_DEPENDENCY_POLICY.md` (citations only, only if D4 needs it)
- this spec

## Non-goals

- Any code change in `lane` or `capi`, including `#[inline]` on the helpers (see D2).
- The writers' soundness (the sibling successor *Make the floating-point control-word writers sound
  by construction*).
- A committed codegen test. The claim is prose; the evidence is the attempt record's disassembly
  (as #1489's D-test chose).

## Hazards

- `fpenv.rs` and `softfma.rs` carry #1446's comment lines (STREAMS hot-file row 108). Keep #1446's
  wording when rebasing over it.
- The sibling successor edits the same two files (code and `SAFETY` lines); the later slice
  rebases.

## Objective gates

1. **Every sentence of the two passages is true (verifier).** The verifier rebuilds the cdylib on
   the slice's head, reads the same functions, and checks each sentence of `fpenv.rs:10-17` and
   `:71-76` (and `capi/src/ffi.rs:813-816`) against it or against the cited source. Any false
   statement fails the attempt.
2. **No codegen change.** The C ABI caller audit (`target/release/audit capi`) reports
   `pcm_digest` `cb10fbface44a3a4` (or the base's value), 0 allocations, 0 syscalls; the browser
   module digest equals the base's (`scripts/build-web-audioworklet.sh --named-twin`).
3. **Existing gates.** `cargo fmt --all -- --check`, `RUSTDOCFLAGS="-D warnings" cargo doc --locked
   -p lane --no-deps`, `bash scripts/check-workspace-policy.sh`,
   `bash scripts/check-realtime-policy.sh`, `bash scripts/check-lane-policy.sh` exit 0.

*Test value.* No test is added: the defect is false prose, and a test that greps prose is refused;
gate 1's verifier read is the check, so there is no mutation run.

## Evidence

- The x86_64 and AArch64 disassembly excerpts (D1); gate 2's audit line and module digest.

## Dependencies

- After (other streams): none.
- After (same stream): none. In any order with the sibling successor on the same files; the later
  slice rebases.

## Standing rules for the implementer

- Work only from this body. Read the cited lines and both #1489 verdicts first.
- AArch64 is read from a cross-compiled object, never emulated.
- Attempt budget: two attempts, one adversarial verdict each.
- Size: two hours.

## Attempt record

### Attempt 1 (2026-10-09, on `codex/d15-batch-fp` over #1494 attempt 1 at `1c838fba9`)

**Line shift.** #1494 attempt 1 moved the passages: the DAW sentence is still `fpenv.rs:16-17`
(the passage is `:10-17`), and "Realtime properties" is now `:77-80` (body; was `:73-76`). #1494
also removed the `softfma.rs` copy that cited `fpenv.rs:16-17` (`write_mxcsr`'s doc no longer
mentions a DAW), so D3's `softfma.rs` citation no longer exists and nothing there changes. Both
edited passages keep their line counts (D4), so every live line citation stays exact at this head:
`docs/REALTIME_DEPENDENCY_POLICY.md`'s `fpenv.rs` `:145`, `:187`, `:205`, `:254`, `:431-438`,
`:449`, `:473` (re-read: `read_mxcsr()` call, `write_mxcsr` call, the two AArch64 `unsafe` blocks,
`scheduling_barrier`, the two guard writes). No file outside this spec and `fpenv.rs` changes.

**What changed (comment lines only).**

- `fpenv.rs:16-17`: "every DAW audio callback arrives with FTZ and DAZ already set" becomes
  "whose audio thread may run with FTZ and DAZ set (a host can set them to avoid denormal stalls;
  the C ABI contract does not forbid it)". No primary source states the universal claim, so D3's
  narrowing applies. "Does not forbid it": `crates/capi/include/miso_engine_v1.h` states no
  floating-point environment requirement (no FTZ, DAZ, MXCSR or FPCR text), and
  `miso_engine_v1_render_f32_planar`'s doc (`ffi.rs:795-799`) says the caller's environment is
  borrowed and restored.
- `fpenv.rs:77-80` now reads: "No allocation, lock or syscall; the barriers emit nothing. Shipped
  `x86_64` cdylib: `enter` calls `softfma::read_mxcsr` then `write_mxcsr` out of line, and `Drop`
  calls `write_mxcsr` on each exit path; each helper is a stack-slot `STMXCSR`/`LDMXCSR`. AArch64
  (Android object): inline `mrs`/`msr`, no call. Cost:
  `artifacts/issue146/fp-environment-benchmark.raw.jsonl`."
- `capi/src/ffi.rs:813-816` (now `:813-816` still): "One control-word read, two writes and two
  empty assembly barriers per block" is the guard's count, and it is true (below). The entry's
  separate first-block attestation read (`ffi.rs:848`, `in_canonical_fp_environment`) is not the
  guard's and is commented at its own site. Not false, so not changed (D2).

**x86_64 evidence** (`cargo build --locked --release -p capi`, `objdump -d target/release/libcapi.so`,
identical before and after the edit):

```
0000000000080880 <miso_engine_v1_render_f32_planar>:
   80897: call *0x3baea3(%rip)  # 43b740  -> R_X86_64_RELATIVE 3e6bb0 = lane::softfma::read_mxcsr
   8089d: mov  %eax,%ebx                  (saved word)
   8089f: mov  $0x1f80,%edi
   808a4: call *0x3bae9e(%rip)  # 43b748  -> R_X86_64_RELATIVE 3e6bd0 = lane::softfma::write_mxcsr
   808aa: test %r14,%r14                  (barrier: no instruction between the call and the code)
   ...
   808f1: mov  %ebx,%edi                  (single exit block)
   808f3: vzeroupper
   808f6: call *0x3bae4c(%rip)  # 43b748  -> write_mxcsr(saved)
   ...
   8090d: ret                             (the function's only ret)
   80928: call *0x3bae12(%rip)  # 43b740  -> read_mxcsr: the plan's first-block attestation, taken
                                            only while fp_env_attested (byte 0x198) is clear
00000000003e6bb0 <lane::softfma::read_mxcsr>:
  3e6bb0: movl $0x0,-0x4(%rsp); vstmxcsr -0x4(%rsp); mov -0x4(%rsp),%eax; ret
00000000003e6bd0 <lane::softfma::write_mxcsr>:
  3e6bd0: mov %edi,-0x4(%rsp); vldmxcsr -0x4(%rsp); ret
```

Every exit of the render entry jumps to `0x808eb`/`0x808f1` (`0x80915`, `0x8091c`, `0x80958`,
`0x809de`, `0x809f1`, `0x80a04`, `0x80e94` and the fall-throughs), so each call makes exactly one
exit `write_mxcsr` call: per block, two guard calls on entry and one on exit, plus one attestation
read on a plan's first block. `panic = "abort"` in the release profile, so there is no landing pad.
The only other site in the cdylib that enters the guard is `builtins::tail::fixed_input_bound`
(`0x3cda80`, reached from session builtins preparation, off the render thread): the same shape,
`read_mxcsr` `0x3cda9d`, `write_mxcsr` `0x3cdaaa` on entry, `write_mxcsr` `0x3cdf69` on its single
exit block. No other `miso_engine_v1_*` export enters the guard (`grep` of the disassembly for the
two GOT slots finds only these seven calls). The `#[inline]` `fpenv` wrappers are inlined; the
`softfma` helpers carry no `#[inline]` and are called out of line, so the old "no call at all"
was false.

**AArch64 evidence** (static read; `cargo rustc --locked --release -p capi --target
aarch64-linux-android -- --emit asm`; the final link fails for want of an NDK linker, after the
crate's object `capi.capi.*-cgu.0.rcgu.o` is written; `llvm-objdump -d` of that object):

```
miso_engine_v1_render_f32_planar:
   18: mrs x23, FPCR        (enter: read)
   20: msr FPCR, x8         (enter: write 0, x8 = xzr)
   84: msr FPCR, x23        (exit copy 1: every rejection path branches here)
   9c: ret
   bc: mrs x9, FPCR         (first-block attestation)
  120: bl  ...              (PlanarBufferMut::try_new, relocation)
  14c: bl  ...              (PlanState::render, relocation)
  31c: msr FPCR, x23        (exit copy 2: the success path)
  334: ret
```

The guard emits no call on AArch64: its `mrs`/`msr` are inline and the barriers emit nothing (no
instruction between `//APP`/`//NO_APP` in the `.s`). `builtins`' object (the other crate that enters
the guard, `libbuiltins-*.rlib`) also has only inline `mrs`/`msr FPCR`. This is the crate object
before linking, not a linked `.so`; the cross-crate calls it shows are relocations. iOS was not
read.

**Per-sentence check of `:77-80`.** "No allocation, lock or syscall": C ABI audit below.
"the barriers emit nothing": no instruction at `0x808aa`/`0x808f1` boundaries on x86_64, empty
`//APP` blocks on AArch64. "`enter` calls ... then ... out of line": `0x80897`, `0x808a4`.
"`Drop` calls `write_mxcsr` on each exit path": `0x808f6`, the single exit. "stack-slot
`STMXCSR`/`LDMXCSR`": `0x3e6bb8`, `0x3e6bd4`. "AArch64 ... inline `mrs`/`msr`, no call": object
offsets `0x18`, `0x20`, `0x84`, `0x31c`. "Cost": the raw JSONL records `guard_ns_per_block` over a
128-frame prepared render.

**Gates.**

- C ABI caller audit (`cargo build --locked --release -p audit -p bench -p capi -p
  session-validator`; `./target/release/audit capi`): `pcm_digest` `cb10fbface44a3a4`,
  `allocations` 0, `locks` 0, `syscalls` 0.
- `bash scripts/build-web-audioworklet.sh --named-twin <twin> <out>`: exit 0, module
  `9ea229e2f9c3158ac1e00896c509a83deeb83840b07969edacf9eb06cb70c939` (= base).
- `cargo fmt --all -- --check`, `RUSTDOCFLAGS="-D warnings" cargo doc --locked -p lane --no-deps`,
  `bash scripts/check-workspace-policy.sh`, `bash scripts/check-realtime-policy.sh`,
  `bash scripts/check-lane-policy.sh`: all exit 0.

**Open items.** None in scope. Outside the authorized lines, `fpenv.rs:33` ("Refusing a DAW's
callback thread") and `ffi.rs:798` ("A DAW audio callback that arrives with FTZ and DAZ set") are
conditional, not universal, and were left alone.
