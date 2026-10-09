PASS

# #1495 attempt 1 -- adversarial verdict

Commit `f7a625d92` on `codex/d15-batch-fp` (worktree `/home/bl/misofm/wt-d15-fp`), parent `1c838fba9`
(#1494, verified PASS). I reviewed `git diff 1c838fba9 f7a625d92`. I exported the commit with
`git archive` to `/tmp/claude-1002/v1495/tree` and the parent to `/tmp/claude-1002/v1495/base`, and
built only there (`CARGO_TARGET_DIR=/tmp/claude-1002/v1495/target*`). I did not build, edit or check
out anything in the worktree. AArch64 code was read statically; nothing was run or emulated.

Scope is clean. The commit changes 2 paths: the spec (attempt record only, +114) and
`crates/lane/src/fpenv.rs` (6 lines out, 6 in). `git diff -U0 -- crates/` has only `//!` lines. Both
passages keep their line counts (`:16-17` 2 -> 2, `:77-80` 4 -> 4), so no line below moves. The edits
are inside the authorized passages after #1494's shift (`:10-17` and "Realtime properties", now
`:75-80`, body `:77-80`). `softfma.rs`, `ffi.rs` and the policy doc are not touched, which is correct
(see D3 and D2 below).

Every sentence of `fpenv.rs:10-17`, `fpenv.rs:77-80` and `capi/src/ffi.rs:813-816` is true against
the disassembly or the cited source (truth table below). I found no BLOCKER and no MAJOR.

## MINOR

**m1. The universal DAW claim that D3 narrowed is still stated in three other live files, and the
attempt record does not name them.**

- `crates/host-core/src/render_session.rs:8-9`: "and every DAW audio callback arrives with FTZ+DAZ
  set."
- `crates/host-core/src/lib.rs:78`: "Every DAW audio callback arrives with hardware FTZ and DAZ set".
- `crates/capi/src/runtime/tests.rs:3080`: "Every DAW audio callback arrives with FTZ and DAZ already
  set".

None of them cites `fpenv.rs:16-17` (that citation was `softfma.rs`'s, and #1494 removed it), so no
citation is stale. But each one repeats the claim that this issue found unevidenced. They are outside
the authorized paths, so the attempt was right not to edit them. The record's "Open items" names only
`fpenv.rs:33` and `ffi.rs:798`, which are conditional and true. It should also name these three. Root
decides: widen #1495 for attempt 2, or file a one-hunk successor.

## NIT

- **n1. `ffi.rs:815-816`, "One control-word read, two writes and two empty assembly barriers per
  block".** This is true as the guard's count: read `0x80897`, writes `0x808a4` and `0x808f6`, two
  barriers. The first block of a plan also reads the word once more for attestation (`0x80928`,
  `ffi.rs:848`, skipped when byte `0x198` is set). That read is commented at its own site (`:844-846`,
  "Later blocks skip it"), so the comment is not false in context. A reader who counts the entry's
  total reads gets one too few on a plan's first block. If someone edits this line later, "The guard
  costs one control-word read, ..." would remove the doubt. D2 (correct only if false) correctly left
  it.
- **n2. Record, exit list.** "Every exit of the render entry jumps to `0x808eb`/`0x808f1` (`0x80915`,
  ..., `0x80e94` and the fall-throughs)". The six early conditional branches (`0x808ad`, `0x808c2`,
  `0x808c9`, `0x808d3`, `0x808d8`, `0x808df`) are jumps, not fall-throughs, and the list leaves them out.
  `0x809cb` reaches the exit through `0x80e90`/`0x80e94`. The conclusion is true: there is one `ret`
  (`0x8090d`), and no branch leaves the function.
- **n3. Record, "No other `miso_engine_v1_*` export enters the guard".** This is true if it means "no
  other export contains a guard site". It is false if it means "no other export reaches the guard":
  builtins preparation (`prepare_session_builtins_with_live_controls_and_policy`, `0x16482f` through
  GOT `0x43c020`) calls `fixed_input_bound`. The cdylib is linked with section GC, so that code is
  there only because an export reaches it.
- **n4. Record, AArch64.** "`builtins`' object (the other crate that enters the guard ...)". The rlib
  objects of `host-core` (`StartedRenderSession::render_planar`) and `lane` (`attest_fp_environment`)
  also hold guard code. All of it is inline `mrs`/`msr`, so the passage stays true. The phrase is
  true only for the linked library.
- **n5. `fpenv.rs:16-17`, "FTZ and DAZ" for AArch64 hosts.** AArch64 has one bit, `FZ`, that does
  both (`fpenv.rs:50-51` says so). This text was already there before the attempt, and it is not
  false. I record it only for completeness.

## Per-sentence truth table

`fpenv.rs:10-17` (at `f7a625d92`)

| # | Text | Verdict | Evidence |
|---|------|---------|----------|
| 1 | "It does not cover the whole render." | True | `tools/wasm-gates/tests/g6_full_corpus_ftz.rs:6-8` |
| 2 | "Issue #144's full-corpus reproducer (...) found 69-70 of 331 corpus rows rendering off-pin under hardware FTZ+DAZ: transient intra-block denormals in the recursive SVF, the feed-forward lane, scalar math and the effect/builtin chains, none of which is a *state* word the D7 flush can reach." | True (cited source; unchanged; not re-measured) | `g6_full_corpus_ftz.rs:6-8`, `:70` ("70 of 331"); `tools/wasm-gates/MUTATIONS.md:34`, `:43` |
| 3 | "Browser Wasm is unaffected -- the core specification mandates denormal correctness and forbids a flush-to-zero mode, confirmed by the three-browser digest parity --" | True as a paraphrase (unchanged) | WebAssembly core spec, Numerics, floating point: IEEE 754-2019 arithmetic, round-to-nearest ties-to-even, no non-default rounding or exception attributes, so there is no flush mode. The spec does not say "subnormal" or "flush" in so many words. The three-browser leg is `hosts/host-web/qualification/run.mjs:8`, `:22` (Chromium, Firefox, WebKit) and `docs/TARGET_MATRIX.md:12`. I did not re-run the browsers. |
| 4 | "so the exposure is exactly the native hosts," | True | The guard has code only under `cfg(any(x86_64, aarch64))` (`fpenv.rs:406`, `:497-500`) |
| 5 | "whose audio thread may run with FTZ and DAZ set" | True (a possibility, not a universal claim) | The tests install that word on purpose: `crates/lane/tests/fp_env.rs`, `capi/src/runtime/tests.rs` |
| 6 | "(a host can set them to avoid denormal stalls;" | True (a statement of what a host can do; it cites nothing, so it invents no citation) | -- |
| 7 | "the C ABI contract does not forbid it)" | True | `crates/capi/include/miso_engine_v1.h` has no FTZ/DAZ/MXCSR/FPCR/floating-point/denormal/rounding text (grep, case-insensitive). `ffi.rs:795-799` says the environment is borrowed and restored. |

`fpenv.rs:77-80`

| # | Text | Verdict | Evidence |
|---|------|---------|----------|
| 8 | "No allocation, lock or syscall;" | True | The helpers are 3-4 instructions with no call (`0x3e6bb0`, `0x3e6bd0`). AArch64 uses inline `mrs`/`msr`. The whole-entry audit gives allocations 0, locks 0, syscalls 0. |
| 9 | "the barriers emit nothing." | True | `fpenv.rs:436` is an empty template. On x86_64, `0x808aa` follows the `0x808a4` call directly. In the AArch64 `.s` render function, the barrier `//APP`/`//NO_APP` pairs are empty. |
| 10 | "Shipped `x86_64` cdylib: `enter` calls `softfma::read_mxcsr` then `write_mxcsr` out of line," | True for `cargo build --locked --release -p capi` | `0x80897 call *GOT 0x43b740` (R_X86_64_RELATIVE -> `0x3e6bb0` `read_mxcsr`), `0x8089f mov $0x1f80,%edi`, `0x808a4 call *GOT 0x43b748` (-> `0x3e6bd0` `write_mxcsr`). `fixed_input_bound`: `0x3cda9d`, `0x3cdaaa`. See open item 1. |
| 11 | "and `Drop` calls `write_mxcsr` on each exit path;" | True | Render entry: one `ret` (`0x8090d`). All 13 exit branches reach `0x808eb`/`0x808f1`, then `0x808f6 call *GOT 0x43b748` with `%edi = %ebx` (the saved word). `fixed_input_bound`: one `ret` (`0x3cdfb1`) after `0x3cdf69`. Its cold `unwrap_failed` path (`0x3cdffc`) does not return (`panic = "abort"`, `Cargo.toml:121`). |
| 12 | "each helper is a stack-slot `STMXCSR`/`LDMXCSR`." | True | `movl $0x0,-0x4(%rsp); vstmxcsr -0x4(%rsp); mov -0x4(%rsp),%eax; ret` and `mov %edi,-0x4(%rsp); vldmxcsr -0x4(%rsp); ret` (VEX encodings of the same SDM instructions) |
| 13 | "AArch64 (Android object): inline `mrs`/`msr`, no call." | True; iOS was not read, and the text claims nothing about iOS | (a) Crate object `capi.capi.*-cgu.0.rcgu.o`, `aarch64-linux-android` (non-LTO; 157 undefined symbols): `0x18 mrs x23, FPCR`, `0x20 msr FPCR, x8`, `0x84 msr FPCR, x23`, `0xbc mrs x9, FPCR` (attestation), `0x31c msr FPCR, x23`. Its only FPCR accesses are these 5. (b) The rlib objects of `builtins` (4 functions), `host-core` and `lane` have only inline `mrs`/`msr`. (c) The fat-LTO Android staticlib, which `scripts/check-cross-targets.sh:156-163` treats as the library an Android app links (`cargo rustc --release --target aarch64-linux-android -p capi --lib --crate-type staticlib -- --emit asm`): 5 FPCR accesses in `miso_engine_v1_render_f32_planar` and 4 in the builtins-preparation iterator. `read_fp_control_word`/`write_fp_control_word` appear only as DWARF inline-record strings. |
| 14 | "Cost: `artifacts/issue146/fp-environment-benchmark.raw.jsonl`." | True | The file exists. It records `fp_environment_guard`, `guard_ns_per_block` `[2.667376,1.322708]`, quantum 128. |

`capi/src/ffi.rs:813-816`

| # | Text | Verdict | Evidence |
|---|------|---------|----------|
| 15 | "first statement of the entry and last thing undone: the whole call runs in the canonical floating-point environment, validation included, so a rejected call also hands the caller's word back unchanged." | True | The write at `0x808a4` comes before the first validation (`0x808aa`). Every rejection leaves through `0x808f6`. |
| 16 | "One control-word read, two writes and two empty assembly barriers per block;" | True (guard count); see n1 | `0x80897`, `0x808a4`, `0x808f6`. The attestation read `0x80928` happens on a plan's first block only, and has its own comment. |
| 17 | "measured in `artifacts/issue146/`." | True | As row 14 |

## D3 and the softfma citation

At `f7a625d92`, `softfma.rs` has no "DAW" text and no `fpenv.rs:` line citation (grep). #1494's D1
rewrote `write_mxcsr`'s `SAFETY` to say only what the body relies on, so D3's "`softfma.rs` keeps
citing `fpenv.rs:16-17`" clause has nothing to act on. The attempt correctly changed nothing there and
said why. Outside the specs and the handoff verdicts, no live file cites `fpenv.rs:16-17`. Three live
files repeat the claim (m1). The narrowing follows D3's wording, adds no source, and every part of it
is true (rows 5-7).

## Line citations

These live citations are exact at `f7a625d92` (no line moved):
- `docs/REALTIME_DEPENDENCY_POLICY.md`: `fpenv.rs` `:145` (`read_mxcsr()`), `:187` (`write_mxcsr`),
  `:205`/`:254` (AArch64 `unsafe` blocks), `:431-438` (`fn scheduling_barrier`), `:449`/`:473` (the
  guard's two writes). The policy's `ffi.rs` `:807` is also exact.
- The record's `ffi.rs:795-799`, `:813-816` and `:848`.

The other `fpenv.rs`/`softfma.rs` citations are in the specs of other issues (#1321, #1422, #1446,
#1478, #1489, #1494) and in `docs/handoffs/` verdicts. Each is a record of its own commit, and
#1495 has no authority over them.

## Test value

No test is added. The spec says so: the defect is false prose, gate 1 is the check, and a test that
greps prose is refused. There is no mutation run. No queue is touched, so the acked-batch question
does not apply.

## Open items for root (outside this slice)

1. **`lto = "fat"` does not apply to the cdylib the C ABI gates build, and the x86_64 sentence (row
   10) depends on that.** `cargo build --locked --release -p capi -v` passes no `-C lto` to `capi`.
   `crates/capi/Cargo.toml:11` lists `rlib` among the crate types. That is the only difference from the
   `--crate-type cdylib` build below, which does get LTO, so it is the likely cause. So
   cross-crate calls stay GOT calls (`read_mxcsr`, `write_mxcsr`, `PlanarBufferMut::try_new` at
   `0x80982`). The same source built as `cargo rustc --release -p capi --lib --crate-type cdylib` *is*
   fat-LTO'd. There, the guard is inline: `vstmxcsr 0x10(%rsp)` `0x72b96`, `vldmxcsr 0x10(%rsp)`
   `0x72ba8`, exit `vldmxcsr 0xc(%rsp)` `0x72cdb`, and no helper call. Row 10 is true for the artifact
   the spec named (`check-capi-abi.sh`, `qualification.yml:876`). But if the x86_64 cdylib is ever
   built LTO'd, the sentence becomes false. The mobile gates already read the `--crate-type
   staticlib` (fat-LTO) form, and row 13 holds there. The #1489 attempt-1 verdict's "shipped
   `libcapi.so` (fat LTO)" premise was wrong for the same reason, but its observations stay true.
   Root decides whether the non-LTO x86_64 cdylib is intended.
2. m1's three copies of the universal DAW claim.

## Gates run (export of `f7a625d92`, `CARGO_TARGET_DIR=/tmp/claude-1002/v1495/target`)

- Gate 1: `cargo build --locked --release -p capi` (the shipped feature set: `-p capi` alone, as
  `check-capi-abi.sh:28`/`:206` builds it), then `objdump -d -C`, `readelf -rW`. The parent
  `1c838fba9` built the same way gives `miso_engine_v1_render_f32_planar`, `fixed_input_bound`,
  `read_mxcsr` and `write_mxcsr` identical up to addresses (the whole `.so` differs only because the
  export paths differ). Two builds of the head into separate target dirs give identical `.so` bytes.
  AArch64: `cargo rustc --locked --release -p capi --target aarch64-linux-android -- --emit asm`. The
  link fails with no NDK linker (`/usr/bin/ld: ... file in wrong format`) after the crate object and
  `capi.s` are written. I also built the fat-LTO staticlib asm with the command from
  `check-cross-targets.sh` and the fat-LTO x86_64 cdylib with `--crate-type cdylib` (open item 1).
  `llvm-objdump` 18.1.3.
- Gate 2: `cargo build --locked --release -p audit -p bench -p capi -p session-validator`, then
  `target/release/audit capi`: `pcm_digest` `cb10fbface44a3a4`, `allocations` 0, `deallocations` 0,
  `locks` 0, `syscalls` 0, `panic_unwinds` 0, `total_violations` 0, exit 0.
  `bash scripts/build-web-audioworklet.sh --named-twin <twin> <out>`: exit 0, module
  `9ea229e2f9c3158ac1e00896c509a83deeb83840b07969edacf9eb06cb70c939` (= base).
- Gate 3: `cargo fmt --all -- --check` exit 0; `RUSTDOCFLAGS="-D warnings" cargo doc --locked -p lane
  --no-deps` exit 0; `bash scripts/check-workspace-policy.sh` "workspace policy: ok";
  `bash scripts/check-realtime-policy.sh` "realtime policy: ok (89 marked regions in 25 files)";
  `bash scripts/check-lane-policy.sh` "lane policy: ok".

Evidence kept in `/tmp/claude-1002/v1495/ev/`: `render.dis`, `fib.dis`, `x86-helpers.txt`,
`x86-got-slots.txt`, `x86-cdylib-helper-calls.txt`, `x86-lto-mxcsr.txt`, `x86-lto-render-head.dis`,
`a64-render.dis`, `a64-rlib-fpcr.txt`, `android-lto-fpcr.txt`, `capi-audit.json`, `wasm-build.tail`,
`aarch64-build.tail`. The trees, target directories and large disassemblies were deleted.
