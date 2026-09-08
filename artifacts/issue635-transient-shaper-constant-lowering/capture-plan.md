# Issue 635 stage-1 lowering capture plan

This evidence tranche is the one Astra-LOW-authorized, untimed Luna-HIGH
stage-1 capture for issue #635. It is static compiler inspection only. No
audio workload, benchmark, tuning, retry, source/test/manifest/lock edit, or
artifact work is planned. Full compiler payloads stay in temporary directories
outside the repository; only selected excerpts and their source identities may
be retained here.

## Frozen identity before compilation

- Worktree: `/home/bl/misofm/engine-transient-shaper-constant-lowering`
- Branch: `codex/transient-shaper-constant-lowering`
- HEAD: `186e6b3080b040d4a6e7c25b1516762224297381`
- Required base/main: `62045f40048ec230298fe0fd3935da3333f90b83`
- Authorization commit: `21191981` (`docs: authorize FX4 lowering capture`)
- Authorized executor: `/root/issue635_luna_high`
- Source and input SHA-256 values were computed before compilation:

| Path | SHA-256 |
| --- | --- |
| `crates/transient-shaper/src/lib.rs` | `f3b0ce78b4e1972f2d43a3570a844667c17740c975776ac90fe471beaacadcda` |
| `crates/transient-shaper/src/corpus.rs` | `9e8649ae73f15d7296f15c53eec86198423508cc2a7fd93b7e5f4d6e81c5f05e` |
| `crates/transient-shaper/Cargo.toml` | `a2c2f26fbcefef0ac1fbd5e3a8729393dee16c38d5bb5db1cfc55d18c64eeabf` |
| `crates/lane/src/lib.rs` | `a3d790da6c50b91064f7d1fbbe731c7169563c75446a736f36a7fdd9bfc6e1c7` |
| `crates/lane/src/backend.rs` | `0a7feeec7527002a57a020b8c921d9baeec5992fe808d09031dd8d5c4c7b5467` |
| `crates/math/src/lib.rs` | `53f016c245198a19f3521ee9fa9638804407762705dba95ecd5593995048e2cf` |
| `crates/effect-runtime/src/envelope.rs` | `48ce3f99338ef1b7c22280bc743878ad6ff3747c0e10bbdd8606b7a3ddc0ef19` |
| `Cargo.toml` | `471660ddca768e90ec87550807e140b4754454509f6f5f59986a80764b53effb` |
| `Cargo.lock` | `92db9698cc062bd5ccc0a90f0ee43f051d77ce7e52f785ce9718fae3f400b753` |
| `.cargo/config.toml` | `03b0fbd88c069abb0a8fbdca5921ba6a989929829fe087977b509a29ebb7d0e` |
| `rust-toolchain.toml` | `85a45cac04c296adac076f8f0609ca8f4c8ca658957f1f24cbd1e054b6cc44e0` |

## Toolchain and target configuration

- `rustc 1.97.1 (8bab26f4f 2026-07-14)`, LLVM `22.1.6`.
- `cargo 1.97.1 (c980f4866 2026-06-30)`.
- Installed targets: `x86_64-unknown-linux-gnu` and
  `wasm32-unknown-unknown`.
- `wasm-objdump 1.0.34`, `wasm2wat 1.0.34`, and `llvm-objdump` are
  available for static inspection if needed; no executable audio path is used.
- Native release configuration is the repository x86-64-v3 setting from
  `.cargo/config.toml`: `-C target-feature=+avx2,+fma`. The repository release
  profile is `lto = "fat"`, `codegen-units = 1`, `debug = 1`, `panic = "abort"`.
- Wasm scalar uses `RUSTFLAGS='-C target-feature=-simd128'`.
- Wasm SIMD uses `RUSTFLAGS='-C target-feature=+simd128'`.
- No `target-cpu` override is planned.
- All Cargo target directories are temporary paths under `/tmp` and no compiler
  payload will be copied into this evidence directory.

## Exact planned commands and expected outputs

Each command is one primary shape and will run once, with `--locked
--release`, `--emit=asm,llvm-ir`, and an isolated target directory. Cargo
stdout, stderr, and exit status will be retained below the corresponding shape.
The expected status for each primary command is `0`; any nonzero status stops
the sequence before another primary command.

1. Native scalar plus native AVX2 W8 production monomorphizations (one native
   crate release capture; scalar and `Simd8` bodies are selected from the same
   emitted files):

   ```sh
   env PATH=/home/bl/.cargo/bin:$PATH \
     CARGO_TARGET_DIR=/tmp/issue635-transient-native-target \
     cargo rustc --locked --release -p transient-shaper --lib -- --emit=asm,llvm-ir
   ```

2. Wasm scalar:

   ```sh
   env PATH=/home/bl/.cargo/bin:$PATH \
     CARGO_TARGET_DIR=/tmp/issue635-transient-wasm-scalar-target \
     RUSTFLAGS='-C target-feature=-simd128' \
     cargo rustc --locked --release --target wasm32-unknown-unknown \
       -p transient-shaper --lib -- --emit=asm,llvm-ir
   ```

3. Wasm `simd128` (`Simd4`; this is the supported Wasm SIMD width):

   ```sh
   env PATH=/home/bl/.cargo/bin:$PATH \
     CARGO_TARGET_DIR=/tmp/issue635-transient-wasm-simd128-target \
     RUSTFLAGS='-C target-feature=+simd128' \
     cargo rustc --locked --release --target wasm32-unknown-unknown \
       -p transient-shaper --lib -- --emit=asm,llvm-ir
   ```

The capture will map the actual `process`/`process_bank` callers through
`Shaper::run`, the stationary suffix (`Ramps::current` and the `tail_left` /
`tail_right` loop), the ramping prefix (`Ramps::advance` and the head loop),
and all three monomorphized link modes into `step`/`frame`. Candidate values
`FLOOR`, `DB_PER_OCTAVE`, `CONTRAST_LIMIT_DB`, `SHAPE_LIMIT_DB`,
`OCTAVES_PER_DB`, `SHAPE_LIMIT_DB`, the `0.5` average-link factor, and the
prepare-time coefficient lanes will be classified as repeated broadcast,
folded memory operand, loop-entry materialization, or spill/reload. A finding
must be tied to an actual loop body and source-to-lowering map; source `splat`
spellings alone are not evidence.

## Prerequisite stop conditions

Before command 1, the capture records status for: exact HEAD and base, clean
worktree, unchanged source/input hashes, locked manifest availability, Rust
toolchain version, installed target list, required disassembly tools, and
unused temporary target paths. A missing prerequisite stops the sequence with
the recorded status and no compilation. A failed primary target likewise stops
immediately; no successful-target retry or alternate flag is permitted.

