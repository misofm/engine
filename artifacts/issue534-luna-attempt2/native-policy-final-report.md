# Issue 534 Luna attempt 2: native lowering and frozen policy package

This package is evidence only; it is not an acceptance verdict. The source checkout is
`/home/bl/misofm/engine-gate-detector-access`, branch `codex/gate-detector-access`, at clean
HEAD `8b74e9508a88e7159cb88766ee5b4c9998780275` (origin synchronized when captured).

## Source attribution

The native and policy captures were made at the same clean source checkpoint. Exact SHA-256 and
Git blob identities are:

| path | SHA-256 | Git blob |
|---|---|---|
| `crates/gate-expander/src/kernel.rs` | `0cc1b0115f8878559c1b492174d1e2e906a83b3858bcee4587750f9a3c21a8bd` | `28fe7f6cab3699261eefb9b9bf7351f4a3f40b09` |
| `crates/gate-expander/src/lib.rs` | `24e31168d4488da30a8c519971b4af7c1e723e8485bdcdf67127c45054ce64a1` | `814980fa9cd4d6c3b0c1b80aa7c30b15add1a226` |
| `crates/gate-expander/tests/identity.rs` | `36baa96332ba8e5e2d81639046cd430bbfe4f500a23242f6eb5d646c92b88aa1` | `75218554cdc6b4a2746676e26840e3803a413307` |
| `crates/gate-expander/tests/state.rs` | `786702191ed9cc3e682dffb113712498b461b8d83b51b558ae23fa2dc8443645` | `358788b81dde97a341974ca28b1c6bd6116cee9b` |
| `Cargo.lock` | `ef85bfaac8b4df80b651fa89e5e9a67bfef141b58076bdaac513044161b0b853` | `1b74fb016a04a090d2d1d37ea15fd2917fb85491` |
| `.cargo/config.toml` | `03b0fbd88c069abb0a8fbdca5921ba6a9899298291fe087977b509a29ebb7d0e` | `de2180972c37406f1f16913901bd1d4fc2e660e3` |
| `rust-toolchain.toml` | `85a45cac04c296adac076f8f0609ca8f4c8ca658957f1f24cbd1e054b6cc44e0` | `6b6739839ba23906921aeb4c268d275dcc459fb6` |

## Native candidate lowering

The one permitted invocation was run with `PATH=/home/bl/.cargo/bin:$PATH`, `RUSTFLAGS` unset,
and isolated target directory `/tmp/issue534-luna2-native-target`:

    cargo rustc --locked --release -p gate-expander --lib -- --emit=asm,llvm-ir

Capture files are `/tmp/issue534-luna2/native-lowering.command.json`, `.stdout`, `.stderr`, and
`.status`; status is `0`. The metadata records cwd, branch, HEAD, source identities, rustc
`1.97.1 (8bab26f4f 2026-07-14)` (LLVM `22.1.6`), cargo `1.97.1`, and the rustup override
`1.97.1-x86_64-unknown-linux-gnu`. The repository config supplies the approved x86-64-v3
features (`+avx2,+fma`); no extra ISA flag was supplied.

The original compiler outputs are retained and immutable at:

- `/tmp/issue534-luna2-native-target/release/deps/gate_expander-a68433203a7e3c7a.ll`, 3,995,559 bytes, SHA-256 `6cc506e70472ed0c81f66c76edb94da175d3376fd181ee29bbed17a6c51093cf`
- `/tmp/issue534-luna2-native-target/release/deps/gate_expander-a68433203a7e3c7a.s`, 2,015,972 bytes, SHA-256 `47bba1981af552981ef1b8f3d99688dc660001225ffeaedc3f80d8ded3bcb04e`

Complete selected caller bodies, extracted by the existing baseline technique, are listed in
`/tmp/issue534-luna2/native-selected/manifest.txt` and include both LLVM and assembly for each
body:

| production body | LLVM lines in original | ASM lines in original | extract SHA-256 (LLVM / ASM) |
|---|---:|---:|---|
| `PreparedGate<f32,false>::process` | 13932–16994 (3,063) | 27114–33661 (6,548) | `503df41ec754897b3d298dc3f5b07bfb07d597b7dda3a1d54ea10a42dd07f099` / `6da6865a623e44f557fcf55778e250b58ec4e992318c44461e7bb67be2ec7a88` |
| `PreparedGate<f32,true>::process` | 17737–18167 (431) | 34756–35566 (811) | `2ffb88ec8861e420719963a034c1490230a430fbec5049bfde4ab1db5daa64d0` / `788b7b692f642c24abdb81833daca641e280958b67171f535e01987531513846` |
| connected `PreparedGate<f32,true>::run_block` | 4695–10887 (6,193) | 8982–22770 (13,789) | `787d6f07cbc1427ed68bf14647fc74ebe821fd444174cfe4547e5049e4163cde` / `ec6a0ebadebc783049125050f32a218ead356cc3e87a793f8208f46275899358` |
| emitted `PreparedGate<Simd4,false>::process_bank` | 18179–21108 (2,930) | 35688–40615 (4,928) | `74ccb8f131bf6cc267108af6c8d58b51cfcae0ec2602df2fdde85101f738ef52` / `f43543443f442fa092d1659423b83e6aed1431e17a1eefceadeb5f0d0a4182e4` |
| supported `PreparedGate<Simd8,false>::process_bank` | 21962–25682 (3,721) | 41966–48108 (6,143) | `e80950285a1861a93ba9558659b02a2d6adc6fb8d10cb4b62746d1e1293733ab` / `b9340a8cd263b0e71b4269fa7910023470dbd6a8352858fd9d25273e842d8bd3` |

`PreparedGate<f32,true>::process` is a wrapper that calls the selected connected `run_block`; its
own extracted body contains no detector source loads. The complete connected run-block contains
the access arms. The W4 body is emitted evidence only; public native W4 support is not claimed.
The W8 body and scalar bodies are the supported caller evidence.

The source access implementation is `kernel.rs:229-272`: DualMono loads two own words and
writes `[LL,LL,RR,RR]`; LinkedEqual loads two own words and writes `[LL,RR,RR,LL]`; LinkedUnequal
loads four words and writes `[left(iL),right(iL),right(iR),left(iR)]`. In complete LLVM bodies,
source-debug-location counts for each of source lines 236, 237, 251, 252, 266, 267, 268, and
269 are respectively 5 each in scalar disconnected, 12 each in connected `run_block`, 8 each in
W4, and 16 each in W8. Tap-index address arithmetic at lines 248 and 263 and stores into `taps`
are excluded from these f32 load counts.

The actual optimized callers retain conditional access classification. Scalar disconnected has
`link_mode == Linked` followed by active tap equality and selects access values 1 (equal) or 2
(unequal), with DualMono selecting 0 (LLVM original lines around 14400–14405 and 15791–15796).
The connected run-block has the same pattern around LLVM lines 4773–4774 and 7937–7938. W4
compares all four active tap pairs (LLVM lines 18312–18347); W8 compares all eight (around
22092–22159). The final switch dispatches access values 0/1/2 to the corresponding arms. This
establishes presence of the conditional and both access-value shapes in production callers; it
makes no performance, cycle, or code-size claim.

## Frozen policy captures

Each invocation used `/tmp/issue534-luna2-capture.py`, isolated
`CARGO_TARGET_DIR=/tmp/issue534-luna2-target`, and source-attribution metadata at the clean HEAD
above. Each status is `0`; stdout/stderr are retained at the listed paths:

| capture | exact argv | stdout |
|---|---|---|
| `/tmp/issue534-luna2/policy-realtime.*` | `bash scripts/check-realtime-policy.sh` | `realtime policy: ok (42 marked regions in 12 files)` |
| `/tmp/issue534-luna2/policy-lane.*` | `bash scripts/check-lane-policy.sh` | `lane policy: ok` |
| `/tmp/issue534-luna2/policy-workspace.*` | `bash scripts/check-workspace-policy.sh` | `workspace policy: ok` |
| `/tmp/issue534-luna2/policy-env.*` | `bash scripts/check-env-vocabulary.sh` | `env vocabulary: ok (114 names, one MISO_ENGINE_ prefix)` |

All four captured stderr files are empty. No broader policy or test-harness matrix was run.

## Preserved test evidence and failures

The focused private kernel capture passed 2 tests and the early independent scalar oracle passed 1
test; captures are `/tmp/issue534-luna2/private-debug.*` and
`/tmp/issue534-luna2/scalar-oracle-early.*`, both status `0`. The independent oracle command was:

    cargo test --locked -p gate-expander --test oracle oracle_pcm_within_derived_tolerance_scalar

Identity debug and release each passed 3 tests. Final state debug and release each passed 9 tests,
including `populated_restore_reclassifies_equal_taps_before_the_next_render` (the transition
capture passed 1 selected test). Captures are `/tmp/issue534-luna2/identity-debug.*`,
`identity-release.*`, `state-final-debug.*`, `state-final-release.*`, and
`state-transition-final-debug.*`, all status `0`.

The retained populated-state diagnostic sequence is seven labeled captures. `state-transition-
debug`, `-2`, `-4`, `-5`, and `-6` are status `101` open/insensitive results; `-3` is status `101`
from a temporary test-format compile error (`8 positional arguments in format string, but there
are 6 arguments`); `-7` is status `0` after the causal construction and before final diagnostic
cleanup. The first open results were explained by the existing track-3 hold values of 5 ms left
(240 samples) and 6 ms right (288 samples): the earlier 400..600 quiet interval was only 200
frames, and with the 144-sample right tap left only 56 detector frames, so neither hold had
expired. The final bounded fixture uses a populated quiet prefix longer than tap plus hold and
then loud continuation; it retains nonzero, differing alternate tap words and proves next-render
reclassification after restore.

The one permitted dispatch-to-four-read mutation is preserved at
`/tmp/issue534-luna2/mutation-dispatch-to-fallback.diff` (SHA-256
`ca94fcf2f16dd346b1e2b04f61f5734bf23bdfe49f9f119bd8df0ed1a9860737`). The mutation capture
`mutation-fallback-debug.*` is status `101`: old consumed-word checks passed before the route
assertion, which failed with `left: FourReads`, `right: OwnOnly`. Exact restored captures
`mutation-restored-debug.*` and `mutation-restored-debug-2.*` are status `0`; the second is a
same-test duplicate and is reported as such.

After restoration, lib debug (4 tests), lib release (4 tests), and the one whole gate-expander
suite (9 state tests plus the zero-test harness) passed; captures are `lib-debug.*`,
`lib-release.*`, and `full-suite.*`, all status `0`. The first all-target strict Clippy invocation
is retained as `/tmp/issue534-luna2/clippy-gate-all-targets.*`, status `101`, for the existing
`gather_detector` `too_many_arguments (9/7)` finding. The bounded `#[allow(clippy::too_many_arguments)]`
source fix was checkpointed in `8b74e950`; focused Clippy test and the fixed strict all-target
command both passed status `0` (`clippy-fix-focused.*`, `clippy-gate-all-targets-fixed.*`).
Dedicated final fmt check `fmt-final.*` and post-fix `fmt-after-clippy-fix.*` both passed status
`0`. Earlier plain `cargo fmt --all` and `cargo fmt --all -- --check` invocations during state
work were not separately captured; their observed shell status was `0`.

The existing supported corpus capture `/tmp/issue534-luna2/wasm-corpus.*` is status `0`, with
JSONL retained at `/tmp/issue534-luna2-wasm-gates/wasm-gates.jsonl`: native, Wasm scalar, and
Wasm simd128 each report 139 cases, 349 comparisons, zero min/max lowering mismatches, and an
empty mismatch list. Its existing runner note says detector history is resident in locals on both
guest legs. This corpus wrapper is fallback-only evidence and does not by itself prove the
optimized production gate caller path.

A malformed first policy-helper attempt is also retained rather than hidden: the attempted argv
used `python3 /tmp/issue534-luna2-capture.py --label policy-realtime -- bash ...` while the helper
expects a positional label, so it tried to execute `policy-realtime` and then collided on
`/tmp/issue534-luna2/--label.command.json`; no policy script ran in that malformed attempt. The
four correctly captured policy invocations above are the only policy results.
