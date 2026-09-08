# Issue #470 Wasm resource delta derivation

This records the finite Astra LOW pre-pin derivation for the candidate artifact at source head
`d6f7880304f22ee7bd518506ed3dc1f61c66e474`. It does not alter the browser expectation or the
repository artifact pin.

## Candidate observation

The detached qualification artifact was the ordinary six-file output at
`/tmp/issue470-qualified-artifact`:

```text
sha256: 63dd5f8b0febf193847b697fa8e4d92e791b7e4775f3b4b6b61252783f153e9f
wasm bytes: 2,747,774
```

Running the fixture's direct oracle against that Wasm produced these resource rows:

```text
graphSessionPlusPlanBytes: 29578
graphIncrementalPlanBytes: 29578
graphMetadataBytes: 3739
```

The prior pinned values are 29,498, 29,498 and 3,659, respectively, so every changed row is
`+80`. The native mirror was run independently and reported 45,050, 45,050 and 6,159; those
native values are target-sensitive witnesses and are not used in the Wasm arithmetic.

## Compiled fixture population

`hosts/host-web/tests/browser-v1/session.json` has one track, one route (`track-main`) and one
output (`main-out`), with no effects. `crates/graph-compiler/src/ids.rs:149-158` defines seven
unconditional track stages. `crates/graph-compiler/src/compile.rs:235-244` emits all seven for
each track, while `crates/graph-compiler/src/compile.rs:331-353` emits the output and route
nodes. Therefore this compiled fixture has exactly

```text
7 track stages + 1 route + 1 output = 9 emitted graph operations
```

This is the `levels` population folded into `emitted_op_count` by
`crates/graph-compiler/src/compile.rs:536-544`; no native 82-entry population is reused.

## Wasm layout witness

The command below compiled the actual graph crate for `wasm32-unknown-unknown`, including its
existing unit-test code, without adding a source or test file:

```text
cargo test --locked -p graph --lib --target wasm32-unknown-unknown --no-run
```

`llvm-dwarfdump --debug-info` on the resulting Wasm reported:

```text
RuntimeOp                         byte_size 0xb0 (176)
RuntimeUnit                       byte_size 0xb0 (176)
Option<SplitPairSlot>             byte_size 0x08 (8)
SplitPairSlot                     byte_size 0x08 (8)
Runtime.split_pairs               one Wasm slice header (two 4-byte words)
Runtime                           byte_size 0x58 (88)
```

The emitted LLVM IR for the live layout helpers independently contains the target constants
`176 - 168` for both `RuntimeOp` and `RuntimeUnit`, and `88 - 80` for `Runtime` versus
`RuntimeWithoutSplitPairTable`; the first value returned by the runtime helper is the Wasm
8-byte slice-entry size. These are the actual `size_of` results used by the compiled helper, not
native layout assumptions.

The live source mirrors are in `crates/graph/src/runtime.rs:584-610` and `1099-1109`:
`RuntimeOpWithoutSplitPairSlot` removes exactly the 8-byte `split_pair` field, and
`RuntimeWithoutSplitPairTable` removes exactly the 8-byte `split_pairs` slice header. The
containing `RuntimeUnit` mirrors the `RuntimeOp` payload in its `Op` variant, so its delta is the
same 8 bytes. Consequently the actual Wasm deltas used by
`GraphRuntimeMetadataResourceEstimate::checked_for` are:

```text
runtime table field:       8 bytes
op layout delta:           8 bytes
unit layout delta:         8 bytes
emitted-op bound:          9
```

The independently derived bound is therefore:

```text
8 + 9 × max(8, 8) = 80 bytes
```

It agrees with all three observed Wasm resource-row movements. The candidate direct oracle also
passed its raw-Wasm and PCM checks; the native fixture example completed with the independent
target-sensitive witness above.

## Focused commands

```text
cargo test --locked -p graph --lib --target wasm32-unknown-unknown --no-run   # PASS
CARGO_TARGET_DIR=<fresh-temp> cargo rustc --locked -p graph --lib \
  --target wasm32-unknown-unknown -- --emit=llvm-ir                              # PASS
node hosts/host-web/tests/browser-v1/direct-oracle.mjs \
  /tmp/issue470-qualified-artifact hosts/host-web/tests/browser-v1/expected.json  # PASS
cargo run --quiet --locked -p host-web --example browser_fixture_resources       # PASS
```

The expected JSON and artifact pin remain unchanged by this evidence tranche.
