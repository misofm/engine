# Issue #534 pre-edit production lowering baseline

## Scope and disposition

This is the authorized pre-edit baseline for #534, captured before any detector-access source or
fixture change. The normal native release build is inspectable. The baseline retains both access
cases named by the issue in identifiable production lowering:

1. DualMono still performs the partner source access even though its detector law uses only its
   own channel.
2. Linked modes still perform both cross-plane accesses separately when the left and right tap
   values happen to be equal at runtime.

The baseline therefore selects the issue's branch (a): one candidate may address both cases with
the shared access classification and helper described in the issue. This is a lowering decision,
not a speed or cycle claim. No source edit is authorized by this report; Astra's bounded
implementation/narrowing ruling remains required.

## Capture provenance

The first successful baseline capture used exactly:

```text
cargo rustc --locked --release -p gate-expander --lib -- --emit=asm,llvm-ir
```

It ran from `/home/bl/misofm/engine-gate-detector-access` on branch
`codex/gate-detector-access`, with `CARGO_TARGET_DIR=/tmp/issue534-baseline-target` and
`PATH=/home/bl/.cargo/bin:$PATH` (the capture metadata records the resulting complete PATH). The
compiler exited with status `0`; stdout is the empty file
`/tmp/issue534-baseline/baseline.stdout`, and stderr is the original compiler log
`/tmp/issue534-baseline/baseline.stderr` (status and hashes are retained alongside them in
`baseline.status` and `baseline.command.json`).

The build used the workspace release profile (`lto = "fat"`, `codegen-units = 1`, `debug = 1`)
and the repository x86-64 native configuration (`+avx2,+fma`). No supplemental LTO=false build
was needed. Toolchain identity was rustc 1.97.1 (LLVM 22.1.6), cargo 1.97.1, active toolchain
`1.97.1-x86_64-unknown-linux-gnu` from the worktree's `rust-toolchain.toml`.

Source SHA-256 and Git blob identities for `crates/gate-expander/src/kernel.rs`,
`crates/gate-expander/src/lib.rs`, `Cargo.lock`, `.cargo/config.toml`, and
`rust-toolchain.toml`, plus HEAD `867bbfd327f5fb0c2f57769a50da2aa26a013774`, branch, remote and
clean status, are recorded in `baseline.command.json`. The capture's original compiler artifacts
are retained at:

| artifact | bytes | SHA-256 |
| --- | ---: | --- |
| `/tmp/issue534-baseline-target/release/deps/gate_expander-a68433203a7e3c7a.ll` | 3,680,496 | `70d9633d05b0f040c4f8d60e5b4b1ee99de184264fb05ce39aa30b93023ea911` |
| `/tmp/issue534-baseline-target/release/deps/gate_expander-a68433203a7e3c7a.s` | 1,927,109 | `c49178247fb551f89d6f4c5b46e8b2a7416fd2e00801146c4624341624dc300f` |

The LLVM payload is a historical capture identity only: issue #625 removed the selected `.ll` files from current main. The original byte size/hash, source mapping and conclusion remain; the selected assembly and text records remain available.

One preliminary capture-helper invocation failed before writing metadata because its Git status
argument was malformed; no cargo/rustc process ran and no compiler output was produced. The
corrected, recorded invocation above is the only compiler capture and completed successfully.

## Production definitions and callers

`gate_block` and `run_segment` are `#[inline(always)]`; neither has a standalone emitted function
definition in this LTO output. LLVM debug metadata names their monomorphizations, but those names
are not treated as hot symbols. The selected complete caller extracts and manifest are under
`/tmp/issue534-baseline/selected/`.

The real production boundary mapping is:

| production boundary | emitted identity | access body location |
| --- | --- | --- |
| `PreparedGate<f32, false>::process` | scalar disconnected `process` | `run_segment<false,true>` and `<false,false>` inlined into process |
| `PreparedGate<f32, true>::process` | scalar connected `process` | calls the emitted `run_block<f32,true>` |
| `PreparedGate<f32, true>::run_block` | scalar connected `run_block` | both connected `run_segment` variants and `gate_block` body inlined here |
| `PreparedGate<Simd4, false>::process_bank` | W4 bank `process_bank` | unconnected `run_segment`/`gate_block` variants inlined here |
| `PreparedGate<Simd8, false>::process_bank` | W8 bank `process_bank` | unconnected `run_segment`/`gate_block` variants inlined here |

W4 is included as emitted monomorphization evidence only; it does not claim supported public
native W4 execution. The repository's public native path uses its documented supported backend
policy. Connected W4/W8 banks are not production bank paths.

## Access finding

The source access region is `kernel.rs:271-277`. In valid render iterations, the emitted LLVM IR
has four separate value loads corresponding to the four source expressions:

```text
taps[0] = source_left[own]       (left own)
taps[1] = source_right[own]      (right own)
taps[2] = source_right[partner]  (right partner)
taps[3] = source_left[partner]   (left partner)
```

The selected IR mapping identifies the optimized value loads by their generated result families:
`%_78` is left-own, `%_82` right-own, `%_85` right-partner, and `%_87` left-partner. The scalar
caller extracts show four `load float` instructions in each valid loop variant. W4/W8 extracts
show the same four source-level loads for each lane, emitted as scalar `load i32` operations in
LLVM IR and as `movl`/`vmovd`/`vpinsrd` memory operands in native assembly. The ring pointer bases
and the own/partner index expressions remain distinct; no link-mode test dominates these loads.

Consequently, DualMono's runtime link coefficients do not remove the partner loads. For a linked
case where `tap_left[lane] == tap_right[lane]`, the two index values become equal at runtime, but
the native lowering still contains separate left/right own and partner memory accesses. The
source-mapped native contexts and complete sections are retained in `asm-source-context.txt` and
the per-caller `.s` extracts. The scalar assembly maps the load implementation to the lane scalar
loader in places; the surrounding four-memory-access sequence and the LLVM caller mapping provide
the production identity. W4/W8 have direct `kernel.rs:271-277` `.loc` records around their bounds
checks and access sequence.

Assignments into the four source arrays are not counted as executed memory loads. Bounds-check
branches and panic-only paths are retained in the mapping for completeness, but the finding is
based on the valid-path value loads in the actual callers.

## Candidate scope recommendation and limits

After the bounded Astra ruling, candidate attempt 1 can implement both surviving cases in one
shared helper: classify once per actual segment; use own-only access for DualMono; reuse the
opposite own vector for linked lanes whose left/right taps are equal; retain all four accesses for
unequal linked taps. The candidate must preserve the issue's channel-step arithmetic, ring and
cursor rules, and state/automation behavior. This baseline supplies no projected speedup, cycle
count, timing, benchmark, or sound-quality claim.

The normal LTO output was sufficiently mapped, so there is no supplemental LTO=false baseline.
No probe binary, new fixture framework, source edit, dependency change, or test run was performed.
