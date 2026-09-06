# Issue 463 pre-edit lane-bounds baseline

This is a non-LTO inspection probe, not a shipped worklet or Cargo target. It was captured from
`60a3bb858d80ee186f0f38bef3beb11988a38f2a`; `crates/lane/src/kernels.rs` had SHA-256
`20dcb8d8abbc834f51e5357ff8fb8ee4c1e32e373bc5d7d8b32e6d358a9e469b` and Git blob
`da19c7c6d9aaff90600781d2de12dcbb6113f968`. The production kernel diff was empty.

## Toolchain and build identity

`rustc -Vv` reported Rust 1.97.1 (`8bab26f4f68e0e26f0bb7960be334d5b520ea452`,
2026-07-14) with LLVM 22.1.6. `wasm-objdump --version` reported 1.0.34. The native decoder was
the toolchain's `llvm-objdump`, from
`lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-objdump`.

Cargo first built `lane` with `--locked --release` into three isolated directories. Native used
`--target x86_64-unknown-linux-gnu`; scalar Wasm used `--target wasm32-unknown-unknown` and
`RUSTFLAGS='-C target-feature=-simd128'`; simd128 Wasm used the same target and
`RUSTFLAGS='-C target-feature=+simd128'`. All three commands exited 0.

The retained source was then compiled directly with `rustc`, `--edition=2024`, `--crate-type lib`,
`-C opt-level=3`, `-C debuginfo=2`, `-C codegen-units=1`, `-C lto=off`, the matching Cargo
dependency directory and `liblane.rlib`. Native additionally used `-C target-cpu=x86-64-v3`;
the two Wasm objects used `-C target-feature=-simd128` and `+simd128`, respectively. The source
SHA-256 is `c6a862fdb8dd3d3c56388330f508e68a021329006b9bafd195db80e6d8aa0ee7`.
Compiler statuses were native 0, scalar-Wasm 0, and simd128 0. An earlier direct-rustc attempt
without `-L dependency=.../release/deps` failed for all three with status 1 and E0463; the
`/tmp/463-sol2-*-probe.log` files preserve that failed attempt rather than attributing output to it.

The object SHA-256 values were native
`6e4c28ce76d66ff56a3c459a3aab26f2b9ae22f1931c4a5dfc1cc66960886b96`, scalar-Wasm
`feca32bae5a50593e27aad7185612819c5b4e4e0bfb043bdc0ed6c606778bd9c`, and simd128
`9f44c35823c0021d815d5d230b3063f9ca1137d8b34d2780531eb589d720165d`.
All three decoder commands exited 0. The retained decoded-output SHA-256 values are:

- native: `ed15243b8b8ef35a2d1e2886bc8eea3ead7218b3f03882f11a5ff6d3a0a66b15`
- scalar-Wasm: `d895c840ca1fb0bcc0f0dba0da99c83145d481ee792a5028a48f5d47358dff27`
- simd128 Wasm: `18fdce336ba9e2e8aa9d9b2ddc18189f18a8f3b5e75335c4d3437f8c6c769ef2`

## Named-body interpretation

Every required symbol is nonempty. Native has `issue463_{sum2,sum_into,mix2x2}_{simd8,scalar}`;
each Wasm object has the corresponding `{simd4,scalar}` set. Each ABI exposes independent runtime
lengths for all operands and opaque coefficients for the matrix. A returned checksum observes every
written output; the matrix checksum includes both planes.

The result is not an honest null: all three current bodies retain avoidable length-dependent checks
inside their vector loops. On native Simd8, for example, `sum_into` tests the accumulator span at
`0x40..0x50` and input span at `0x56..0x66` on every backedge to `0x40` at `0x8a`; only after that
does `0x6c..0x76` load/add/store eight lanes. Its scalar tail begins at `0x1a0`, has its necessary
loop test/backedge, and routes excess indices to slice-failure calls at `0x200` and `0x216`.
`sum2_simd8` and `mix2x2_simd8` have the same shape: repeated per-operand span tests in the vector
loop (`0x50..0x8c` and `0x60..0x86` respectively), vector-loop backedges at `0xb5` and `0xcb`,
separate scalar tails, and distinct failure blocks after the hot bodies.

The simd128 object confirms actual `Simd4` lowering: `sum2_simd4` performs `v128.load`/`f32x4.add`
at `0x7c2..0x7cc`, `sum_into_simd4` at `0xb96..0xba5`, and `mix2x2_simd4` performs the two-load,
four-multiply, two-add body at `0x39f..0x3cc`. In each case, slice comparisons and branches remain
inside the enclosing vector loop and have reachable `slice_index_fail` blocks. The scalar-Wasm
object has the same control-flow checks but no `f32x4` opcode, as expected with simd128 disabled.
The explicit `f32` bodies also retain loop-local bounds/panic branches on both Wasm builds; on the
simd128 build LLVM may autovectorize parts of an `f32` instantiation, which does not change its
identity as the explicit scalar/tail specialization. Panic imports elsewhere in either object were
not used as evidence; the conclusion comes from the branches and failure edges within each named
body. This source/control-flow evidence warrants the frozen one-time-prefix-validation rewrite,
subject to Astra's required pre-edit acceptance. It makes no timing or speed claim.

## Pin and execution baseline

The exact scalar command was
`PATH=/home/bl/.cargo/bin:$PATH CARGO_TARGET_DIR=/tmp/463-sol2-print-pins-target cargo run --locked --release -q -p wasm-gates -- --print-pins`.
It exited 0. Its 12,415-byte output has SHA-256
`72ab121357bdf8e50c6f737b4062ad0e5781b7c5801fda2770dcf2b343505661`, byte-identical to
`tools/wasm-gate-corpus/src/lane_digests.in`. Comparing comment-keyed maps against `9eb2e3eb^`
reported exactly 51 old entries present and unchanged, zero changed, and four additions:
`mix2x2_block/{noise,impulse,dc,subnormal}`. Thus the ordinary digest observes both matrix output
planes for all four existing signals while every old named pin remains fixed.

The exact G5 command was
`PATH=/home/bl/.cargo/bin:$PATH bash scripts/run-wasm-gates.sh /tmp/463-sol2-g5-artifact`.
It exited 0. Native, scalar-Wasm and simd128 each reported 139 cases, 349 comparisons, zero min/max
lowering mismatches and an empty mismatch list; the detector-residency check also passed for both
guest legs. The console log SHA-256 is
`3a35fb84fbac2f2b3697dd82e087537b6d65659f76f44e90310a83eef939589f`; the three JSON records are
retained at the command's artifact path with SHA-256
`6ad61ef8bfb758a0b50f886653c6ff3446c8be89965cc3ea6bd375f52c5640b2`.

## Root exact-command recapture

Root independently rebuilt and decoded the unchanged probe/kernels at9c2741ab. Sol’s original compiler command record used placeholders and is retained as a template, not represented as exact invocation provenance. `root-recapture/` supplies actual expanded argv/cwd/environment overrides, tool versions, numeric exits, source/rlib/object/decoded/body hashes and raw outputs. All12 commands returned0. All three object hashes match Sol’s recorded objects. All18 decoded instruction bodies match; five native extracted records additionally include the next section label, documented in comparison.json without modifying either raw capture. The compiler/decoder recapture is the authoritative exact-command baseline; no kernel rewrite or timing occurred.
