# Issue 587 attempt 2 implementation record

Luna HIGH/XHIGH supplied the attempt 2 selected-pair fixture correction. Root
checkpointed the coherent source as
`751c6f3854c79e47e239e7b6912aa930b998b209`.

## Corrected proof

The native case now constructs its endpoint through the public
`prepare_builtin_batch_endpoint` function; only the forced-scalar case uses the
private backend-selected helper. Both execute against an independently prepared
`Concurrent` reference.

The repository's three-track observation-shape session supplies stable `t0`/`t1`/`t2`
owner IDs understood by the existing scalar state oracle. Track `t2` receives six
successive batches: immediate targets, a 256-sample ramp, a 128-sample retarget while
that ramp remains active, explicit settled targets, smoothed mute, and immediate
unmute. Each render resets the shared witness separately for reference and endpoint,
compares every PCM `to_bits()` word, requires exactly one fader and one matrix record
drain for each owner, and confirms each selected endpoint factory/process cohort and
member executes once. The scalar path additionally rejects trace overflow and
compares the addressed `t2` fader and matrix state words after every block.

The attempt 1 PostFader test remains the accepted bank/scalar decline proof. Its
zero-pair witnesses and bitwise PCM/state/meter comparisons are unchanged. The two
accepted attempt 1 mutations remain the selection and arithmetic discriminators.

## Focused results

```text
cargo fmt --all -- --check
  PASS
cargo test -p host-core --lib --features control-provider builtin_batch_endpoint::tests::
  PASS (6 passed)
cargo test -p host-core --test builtin_batch_endpoint --features control-provider
  PASS (14 passed)
cargo test -p host-core --release --lib --features control-provider \
  builtin_batch_endpoint::tests::endpoint_selects_existing_pair_factories_without_observer_barriers
  PASS (1 passed)
cargo clippy -p host-core --all-targets --features control-provider -- -D warnings
  PASS
cargo doc -p host-core --no-deps --features control-provider
  PASS
git diff --check
  PASS
```

Cargo reordered two existing lockfile dependency rows during commands; root restored
the frozen lockfile and confirmed the checkpoint is clean and upstream.

Workspace policies, CI routing, native x86-64-v3 and Wasm target checks remain for
the final source/integrated qualification pass.
