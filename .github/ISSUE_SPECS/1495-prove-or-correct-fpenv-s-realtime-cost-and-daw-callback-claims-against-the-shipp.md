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
