# Issue 503 Luna attempt 1

Base: `e02b1f14fd73fb6ce4863bcb8288624fd169bd21`
Worktree: `/home/bl/misofm/engine-compressor-allocation-proof`
Branch: `codex/503-compressor-allocation-attribution`

Changed paths:

- `crates/compressor/tests/conformance.rs`
- `crates/compressor/Cargo.toml`
- `Cargo.lock` (existing `engine` edge in `compressor` only)

The conformance test now uses the existing thread-local `engine::realtime::audit` scope for the
actual 32 staged, D=0, and native-ragged render calls. It retains first render inside the armed
interval and asserts zero allocations, deallocations, and total violations. A named control test
proves same-thread initial allocation, actual Vec capacity growth/reallocation (growth snapshot
adds an `Allocation`; existing realloc hook records it in that field), and drop/deallocation. A
preconstructed Barrier worker performs the same opaque heap operations while the test thread is
armed; the test-thread snapshot remains default. Child isolation and the installed allocator
liveness assertion remain in place. No engine, allocator, framework, production, workflow, or
spec files were changed.

## Raw command records

Effective environment for all records: `PATH=/home/bl/.cargo/bin:$PATH` (expanded PATH began
`/home/bl/.cargo/bin:`); cwd was `/home/bl/misofm/engine-compressor-allocation-proof`.

Command: `PATH=/home/bl/.cargo/bin:$PATH cargo fmt --all -- --check`

Exit: `1` (initial formatting failure retained in agent transcript; formatter then applied).
Output: rustfmt reported only formatting changes in `crates/compressor/tests/conformance.rs`.

Command: `PATH=/home/bl/.cargo/bin:$PATH cargo test --locked -p compressor --test conformance --features engine/realtime-audit -- --exact scoped_allocator_attribution_controls_are_live_and_isolated --nocapture`

Exit: `0`; one test passed, two filtered.

Command: `PATH=/home/bl/.cargo/bin:$PATH cargo test --locked -p compressor --test conformance --features engine/realtime-audit -- --exact uniform_and_ragged_render_paths_allocate_and_free_nothing --nocapture`

Exit: `0`; parent and isolated child each passed the exact render test, two filtered in each.

Command: `PATH=/home/bl/.cargo/bin:$PATH cargo fmt --all -- --check`

Exit: `0`; no output.

Command: `git diff --check`

Exit: `0`; no output.

## Changed source blobs

```
4840842b3d74fcfb181f18db6189a0371951e276578897cebda79373907dbbd8  crates/compressor/tests/conformance.rs
8eb1e329f51d85a6d521ed55bd8de1b95067040cbb35ad9e14287fd941685393  crates/compressor/Cargo.toml
3da9c28d3f6f5cfca08c56c5cf962a336ff4e0f1a7b9a28a7bdd3bbc617627a5  Cargo.lock
```

## Post-checkpoint qualification

Root checkpoint: `ce88bf12e0a0c0473aa743115c80260327180d9d` (clean exact worktree). All finite
remaining gates passed; the original DSP integration command was intentionally left to root.

Debug and release exact control/render tests and complete conformance passed. Strict affected
Clippy (`cargo clippy --locked -p compressor --all-targets --all-features -- -D warnings`), fmt,
diff, realtime policy (42 marked regions/12 files), and realtime policy mutation suite passed.

Immutable raw logs (each records command/argv, cwd, effective PATH, and exit):

```
/tmp/503-luna1-debug-controls.log   7488ddb2d7743119ccac2e0286d7c7c686a7756729a28eeab09198329c43859a
/tmp/503-luna1-debug-render.log     f906a36198e09072cc82e30e3980dd125de893f84331e77798ce705dd48189f8
/tmp/503-luna1-release-controls.log 1379058e726800e01ecb1a364cb4790e4532edd1ed4d222b2600eb50477e0f0e
/tmp/503-luna1-release-render.log   183e57d0fc5c74bddcc42058d0ee80c7ac9fec7744164ce5e3a0e8894f162c3f
/tmp/503-luna1-full-debug.log        71594a64d40d710621d0851139e9510c2c01d1fb074f8fb3add79ed81f1f0d54
/tmp/503-luna1-full-release.log      48c7d5d9240540a220b4afcfe989abaa651de20f83bb4f8fb1360ba848ec7dac
/tmp/503-luna1-policy.log            dae29bbb6415ad335f7af98039d1da14590e5055195bb1c692d0ad219358b4e1
```

No timing, benchmark, or duplicate DSP integration command was run.
