# Issue 463 candidate evidence

Candidate source is pushed checkpoint `09c26a98d270d031d566feb72d93863d60e21447`.
`crates/lane/src/kernels.rs` has SHA-256
`f1b0af6ea4bc266aaa6649a956cf100221bae235d8a51e9d7f5092b80d3287a4`; the retained probe
source is unchanged from the accepted baseline at
`c6a862fdb8dd3d3c56388330f508e68a021329006b9bafd195db80e6d8aa0ee7`.

## Target-correct code generation

The candidate used the baseline's Rust 1.97.1 / LLVM 22.1.6 / wasm-objdump 1.0.34 toolchain and
the same non-LTO probe settings: opt-level 3, debuginfo 2, one codegen unit, x86-64-v3 natively,
and explicit `-simd128` / `+simd128` Wasm builds. Cargo lane builds, direct probe compiles and
decoders all exited 0 for native, scalar-Wasm and simd128.

Exact compile commands were:

```text
PATH=/home/bl/.cargo/bin:$PATH rustc artifacts/issue463-lane-bounds-baseline/issue463_probe.rs --edition=2024 --crate-name issue463_probe --crate-type lib --target x86_64-unknown-linux-gnu -C opt-level=3 -C debuginfo=2 -C codegen-units=1 -C lto=off -C target-cpu=x86-64-v3 -L dependency=/tmp/463-sol2-candidate-native-target/x86_64-unknown-linux-gnu/release/deps --extern lane=/tmp/463-sol2-candidate-native-target/x86_64-unknown-linux-gnu/release/liblane.rlib --emit=obj=/tmp/463-sol2-candidate-codegen/native.o
PATH=/home/bl/.cargo/bin:$PATH rustc artifacts/issue463-lane-bounds-baseline/issue463_probe.rs --edition=2024 --crate-name issue463_probe --crate-type lib --target wasm32-unknown-unknown -C opt-level=3 -C debuginfo=2 -C codegen-units=1 -C lto=off -C target-feature=-simd128 -L dependency=/tmp/463-sol2-candidate-wasm-scalar-target/wasm32-unknown-unknown/release/deps --extern lane=/tmp/463-sol2-candidate-wasm-scalar-target/wasm32-unknown-unknown/release/liblane.rlib --emit=obj=/tmp/463-sol2-candidate-codegen/wasm-scalar.o
PATH=/home/bl/.cargo/bin:$PATH rustc artifacts/issue463-lane-bounds-baseline/issue463_probe.rs --edition=2024 --crate-name issue463_probe --crate-type lib --target wasm32-unknown-unknown -C opt-level=3 -C debuginfo=2 -C codegen-units=1 -C lto=off -C target-feature=+simd128 -L dependency=/tmp/463-sol2-candidate-wasm-simd128-target/wasm32-unknown-unknown/release/deps --extern lane=/tmp/463-sol2-candidate-wasm-simd128-target/wasm32-unknown-unknown/release/liblane.rlib --emit=obj=/tmp/463-sol2-candidate-codegen/wasm-simd128.o
```

Object SHA-256 values are native
`1ef147e2689eb4081e2f33cead76a498bd75f02355ed5508cc13b203d95aee37`, scalar-Wasm
`495b69754a5fccd6de869f1d357ec99a2f70dd56db79e778952e5c0892784907`, and simd128
`655ca0e11fd161cd34d3f85783ff138b60668d16869cea9b39923e8c4d1ea6d0`.
The retained decoded output hashes are respectively
`7a0164aec7d332070d4a4290e01fc219ad3769bcf2acb852b8c648a8d2144108`,
`306c4e338c024d27dda6d8a1d858738d7be5cb771271b5b66c3837b80ec1064a`, and
`53e01bc6fea02a85c3ca3936c65e4dec8d271119133d0661808ef469217685f9`.

All 18 required named bodies are nonempty. Every body starts with the necessary controlling-prefix
validation: sum2 checks `out_len <= a_len` and `out_len <= b_len`; sum_into checks
`acc_len <= x_len`; matrix checks `left_len <= right_len`. These entry checks lead to the retained
slice failure block and are required release short-input rejection, rather than residual hot-loop
bounds checks.

Native Simd8 illustrates the candidate shape precisely. `sum2` performs its two entry comparisons
at `0xb..0x17`; its main AVX2 loop is `0x70..0xcc`, with only the unrolled loop termination compare
at `0xc9`. `sum_into` validates at `0x7..0xa`; its main loop is `0x60..0xbc`, with termination at
`0xb9`. Matrix validates at `0xb..0xe`; its main two-vector body is `0x80..0xf3`, with termination
at `0xf0`. None of those vector loops branches to a bounds/panic block. Separate remainder code
begins after `0x10b`, `0xfb`, and `0x131`; LLVM applies its own finite unrolling, alias checks and
autovectorization there. Those branches are tail traversal/selection, not repeated slice-shape
validation in the primary lane loop. Matrix still loads both old planes before either plane's
stores and uses separate multiply then add instructions.

The simd128 bodies have the same separation. Sum2's main `v128.load/load/f32x4.add` loop begins at
`0x792`, sum_into's at `0xb80`, and matrix's old-plane load plus four-multiply/two-add body at
`0x393`. Their loop branches terminate prevalidated chunk traversal; their only slice-failure calls
are in entry-rejection blocks at the ends of the named functions. Scalar-Wasm uses scalar arithmetic
with the same entry-only slice rejection and bounded chunk/tail termination. Explicit `f32` bodies
also retain necessary entry rejection and compiler-generated loop/tail control. The checksum
observer remains a distinct named function and its loops were excluded from kernel conclusions.
This evidence supports removal of repeated vector-loop shape checks on all required instances; it
does not claim removal of every branch or any measured speed change.

## Correctness, reachability and realtime evidence

All commands used `PATH=/home/bl/.cargo/bin:$PATH` and isolated
`CARGO_TARGET_DIR=/tmp/463-sol2-qualification-target` unless the existing script owns its target.
Raw logs are `/tmp/463-sol2-*.log`; every result below has numeric status 0 unless called out.

- Lane `g2_kernel_identity` and `p1_partition`, debug and release: 7 + 1 tests per profile.
- Four frozen exact graph filters, debug and release: one nonempty test each, including actual
  Route execution, asymmetric matrix arithmetic, folded identity and real `fold_plane(store=false)`.
- Host-web identity-session and command-timeline digest filters, debug and release: one test each.
- Existing `rt1_direct_bank_alloc`, debug and release: one test each; its live allocator and
  repeated prepared-render zero allocation/free assertions remain green.
- Lane, realtime (42 marked regions in 12 files), graph and workspace policy checks passed.
- Focused lane and graph Clippy passed. Existing unreachable-disallowed-method configuration
  notices from dependencies remained warnings and were not suppressed.
- `cargo fmt --all -- --check` and `git diff --check` passed.

The unchanged-pin candidate G5 command was
`PATH=/home/bl/.cargo/bin:$PATH bash scripts/run-wasm-gates.sh /tmp/463-sol2-candidate-wasm-gates`.
It exited 0: native/backend 2, scalar-Wasm/backend 0 and simd128/backend 1 each executed 139 cases
and 349 comparisons with zero mismatches; both detector-residency legs passed. The candidate JSONL
hash `6ad61ef8bfb758a0b50f886653c6ff3446c8be89965cc3ea6bd375f52c5640b2` matches the accepted
baseline JSONL. The checked-in lane pin file remains byte-identical to the accepted scalar output,
SHA-256 `72ab121357bdf8e50c6f737b4062ad0e5781b7c5801fda2770dcf2b343505661`.

The normal worklet builder compiled successfully but exited 1 at its content-addressed artifact pin:
expected `da36c7503d9d4e1994cec6f22abd3fd97a41ede0551e3355adacdf9068bbead1`, observed candidate
`6dcf5e3a6f5a56feffce22133eb4c997d594db4bbc3624fa9cd0fe79111e10b7`. No pin was changed.
Rebuilding the same stripped/remapped simd128 module into a disposable directory and supplying the
unchanged JS/metadata let the existing direct oracle execute that observed candidate; it exited 0
with `web AudioWorklet independent raw-Wasm oracle passed`. Thus candidate PCM/command identity is
green, while content-addressed artifact promotion remains an explicit delivery seam for root to
route under the issue's bounded qualification-successor rule.

Root independently reran only the existing direct oracle against the retained hash6dcf5e3a candidate because the earlier shell pipeline returned tee status. Fresh subprocess invocation records actual oracle exit0 with no pipeline in raw/463-root-direct-oracle.*. This supplies direct exit evidence without relabelling the earlier pipeline; no rebuild or timing.
