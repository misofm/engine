# Issue 621 attempt 1, tranche 2 evidence

Capture date: 2026-09-08 UTC. This is a bounded, untimed current-source evidence
capture for the detector chunk change. It contains no source or specification
change. The worktree source was at `ae67ac444c36dbfb5c225b84150f97df52bd6942`
for every command recorded here.

## Identity and configuration

- Worktree: `/home/bl/misofm/engine-limiter-detector-bounds`
- Branch: `codex/limiter-detector-chunk-bounds`
- Limiter source: `crates/true-peak-limiter/src/lib.rs`, SHA-256
  `32ab4abf975b32d47c85a748e617e74c9547b22e1b585f0d36713be439a62908`
- `rust-toolchain.toml`: SHA-256
  `85a45cac04c296adac076f8f0609ca8f4c8ca658957f1f24cbd1e054b6cc44e0`
- `.cargo/config.toml`: SHA-256
  `03b0fbd88c069abb0a8fbdca5921ba6a9899298291fe087977b509a29ebb7d0e`
- `Cargo.toml`: SHA-256
  `471660ddca768e90ec87550807e140b4754454509f6f5f59986a80764b53effb`
- `Cargo.lock`: SHA-256
  `92db9698cc062bd5ccc0a90f0ee43f051d77ce7e52f785ce9718fae3f400b753`
- Toolchain: `rustc 1.97.1 (8bab26f4f 2026-07-14)`, LLVM `22.1.6`,
  `cargo 1.97.1 (c980f4866 2026-06-30)`, `wasm-objdump 1.0.34`.
- Native lowering used the repository x86-64-v3 configuration and release
  profile. All Cargo target directories were outside the worktree under `/tmp`.

## Gate results

Every command below exited 0. Its exact command, one-line status, and ordinary
stdout/stderr are retained beside it under `gates/`; six test stdout streams
whose whitespace is significant remain byte-for-byte in the deterministic raw
archive.

```text
full-suite: env PATH=/home/bl/.cargo/bin:$PATH CARGO_TARGET_DIR=/tmp/issue621-attempt1-gates-target cargo test --locked -p true-peak-limiter
focused-debug: env PATH=/home/bl/.cargo/bin:$PATH CARGO_TARGET_DIR=/tmp/issue621-attempt1-gates-target cargo test --locked -p true-peak-limiter detector_chunk_active_window_matches_old_shape --lib
focused-release: env PATH=/home/bl/.cargo/bin:$PATH CARGO_TARGET_DIR=/tmp/issue621-attempt1-gates-target cargo test --locked --release -p true-peak-limiter detector_chunk_active_window_matches_old_shape --lib
allocation-release: env PATH=/home/bl/.cargo/bin:$PATH CARGO_TARGET_DIR=/tmp/issue621-attempt1-gates-target cargo test --locked --release -p true-peak-limiter --test allocation
mono-debug: env PATH=/home/bl/.cargo/bin:$PATH CARGO_TARGET_DIR=/tmp/issue621-attempt1-gates-target cargo test --locked -p true-peak-limiter --test mono_collapse
mono-release: env PATH=/home/bl/.cargo/bin:$PATH CARGO_TARGET_DIR=/tmp/issue621-attempt1-gates-target cargo test --locked --release -p true-peak-limiter --test mono_collapse
clippy: env PATH=/home/bl/.cargo/bin:$PATH CARGO_TARGET_DIR=/tmp/issue621-attempt1-gates-target cargo clippy --locked -p true-peak-limiter --all-targets --all-features -- -D warnings
fmt: env PATH=/home/bl/.cargo/bin:$PATH cargo fmt --all -- --check
realtime-policy: env PATH=/home/bl/.cargo/bin:$PATH bash scripts/check-realtime-policy.sh
lane-policy: env PATH=/home/bl/.cargo/bin:$PATH bash scripts/check-lane-policy.sh
workspace-policy: env PATH=/home/bl/.cargo/bin:$PATH bash scripts/check-workspace-policy.sh
effect-runtime-policy: env PATH=/home/bl/.cargo/bin:$PATH bash scripts/check-effect-runtime-policy.sh
env-vocabulary: env PATH=/home/bl/.cargo/bin:$PATH bash scripts/check-env-vocabulary.sh
wasm-gates: env PATH=/home/bl/.cargo/bin:$PATH bash scripts/run-wasm-gates.sh /tmp/issue621-attempt1-wasm-gates
native-lowering: env PATH=/home/bl/.cargo/bin:$PATH CARGO_TARGET_DIR=/tmp/issue621-attempt1-native-target cargo rustc --locked --release -p true-peak-limiter --lib -- --emit=asm,llvm-ir
wasm-scalar-lowering: env PATH=/home/bl/.cargo/bin:$PATH CARGO_TARGET_DIR=/tmp/issue621-attempt1-wasm-scalar-target RUSTFLAGS='-C target-feature=-simd128' cargo rustc --locked --release --target wasm32-unknown-unknown -p true-peak-limiter --lib -- --emit=asm,llvm-ir
wasm-simd128-lowering: env PATH=/home/bl/.cargo/bin:$PATH CARGO_TARGET_DIR=/tmp/issue621-attempt1-wasm-simd128-target RUSTFLAGS='-C target-feature=+simd128' cargo rustc --locked --release --target wasm32-unknown-unknown -p true-peak-limiter --lib -- --emit=asm,llvm-ir
host-web-scalar-build: env PATH=/home/bl/.cargo/bin:$PATH CARGO_TARGET_DIR=/tmp/issue621-attempt1-web-scalar RUSTFLAGS='-C target-feature=-simd128' cargo build --locked --release --target wasm32-unknown-unknown -p host-web
host-web-simd128-build: env PATH=/home/bl/.cargo/bin:$PATH CARGO_TARGET_DIR=/tmp/issue621-attempt1-web-simd128 RUSTFLAGS='-C target-feature=+simd128' cargo build --locked --release --target wasm32-unknown-unknown -p host-web
git-diff-check: git diff --check
```

The three-leg corpus reported 139 cases, 349 comparisons per leg, and zero
min/max lowering mismatches for native, Wasm scalar, and Wasm SIMD128. The raw
JSONL and command output are retained under `corpus/`.

## Lowering map

The source change is `lib.rs:1750-1765`: `detector_chunk` validates equal,
width-aligned windows once, walks `chunks_exact(width)`, stores each detector
result into the matching output chunk, and writes all twelve history words once
after the loop. The caller slices are established at `lib.rs:1816-1828` and
`1968-1980` from `active_base = chunk * width` and `active_words = span * width`.

The prior capture in `artifacts/issue621-current-lowering/README.md` records the
old repeated input-slice guards. The candidate selected bodies show the
corresponding guard outside the detector arithmetic loop, with no repeated
input-window guard in that loop:

| Path | Candidate active-window check and detector loop | Peak store; history writeback |
| --- | --- | --- |
| Native scalar | `selected/native-scalar-core.s:695-709`; `.LBB36_77` at `854`, backedge `1210` | `1192`; twelve-word writeback begins `1213` |
| Native x86-64-v3 W8 dual | `selected/native-w8-core.s:560-571`; `.LBB34_268` at `709`, backedge `1060` | `1050`; writeback begins `1063` |
| Native x86-64-v3 W8 mono | `selected/native-w8-process-bank-inner.s:724-742`; `.LBB1_78` at `872`, backedge `1222` | `1212`; writeback begins `1232` |
| Wasm scalar | `selected/wasm-scalar-scalar-core.s:1370-1399`; `.LBB22_93` at `1644`, backedge `2136` | `2101`; writeback begins `2141` |
| Wasm SIMD128 W4 | `selected/wasm-simd128-w4-core.s:1344-1380`; `.LBB26_93` at `1625`, backedge `2095` | `2060`; writeback begins `2100` |

The remaining checks visible around these bodies are caller frame/ring accesses,
the W8/W4 lane-width controls, and the outer chunk/span controls. They are
separate from the detector loop's input window. The inspected selected bodies
showed the same detector arithmetic operation sequence, peak store, and single
post-loop twelve-word history writeback shape. This evidence makes no historical
comparison-count, cycle, throughput, speedup, or sound-quality claim.

## Wasm and host portability inspection

The complete limiter native, Wasm scalar, and Wasm SIMD128 LLVM/assembly outputs
were inspected, then removed from durable delivery under tracker ruling
`80f6e715`. Their original paths, sizes, and hashes remain in `provenance.json`;
the focused line-addressed evidence remains in `lowering-map-excerpts.txt` with
raw-source and raw-excerpt hashes in its companion JSON. The scalar limiter
assembly had zero `v128.`/`f32x4.` opcode matches and the SIMD128 limiter assembly
had 4,671. The existing host-web builds were portability and inspection legs
only; they were not artifact promotion, qualification, browser, benchmark, or
pin work.

The scalar host module is 24,678,565 bytes, SHA-256
`1f002b131c3e572d25e3ecf51b7e6506db5685b8298e69f1aaf995cadc9fae5a`; the SIMD
module is 23,721,592 bytes, SHA-256
`1ecca3833b874d4b6a64e939d8c8b48f741889dbaa26e7e6b86f5d00ce19fdc7`. Selected
ABI export rosters and focused `miso_engine_web_v1_render` extracts are retained
under `lowering/host-web-selected/`. The full objdump outputs were
preserved at `/tmp/issue621-attempt1-host-web-raw/` and their paths, sizes, and
hashes are recorded in `provenance.json`; they are intentionally not duplicated
in this bounded repository evidence tree. The scalar host disassembly had zero
SIMD opcode matches; the SIMD host disassembly had 44,076.

## Completeness and hygiene

- `lowering/selected/manifest.json` records the original complete selected
  function intervals, source locations, byte counts, and hashes.
- `lowering-map-excerpts.json` records the surviving focused excerpts, original
  line ranges, full-source identities, and raw excerpt hashes.
- Gate stdout/stderr and status files are retained for every command above;
  failed compiler/disassembly attempts are not hidden (none of the required
  gates failed).
- The recorded `git diff --check` exited 0; the source checkpoint remains
  unchanged.
- No `target/` directory, Cargo registry, dependency tree, secret, token, or
  generated source was copied into this evidence directory.

## Durable delivery form

Tracker ruling `80f6e715` excludes full compiler IR and redundant assembly from
default-branch evidence. `provenance.json` preserves every retired file's
logical path, uncompressed byte count, and SHA-256. The focused display excerpts
strip line endings and trailing horizontal whitespace; the companion JSON's raw
excerpt hashes remain authoritative. Original paths and line numbers in this
record refer to the capture identities in the selected and excerpt manifests.

The six raw test stdout streams are stored byte-for-byte in
`raw-emitted-output.tar.gz`; `raw-emitted-output.members.json` records their
logical paths, sizes, and hashes. The archive uses deterministic zeroed
ownership and timestamps.
