# LANE-4 fix-forward evidence (#388)

This directory supplies the reproducible evidence missing from merged PR #384. The branch changes
tests and evidence tooling only; the LANE-4 production implementation is the merge commit
`2b38ba7fec33c6ddf8247ffff0d4b6b639aebbc9`.

## Bit identity

`cargo test --locked -p lane --release --test g1_op_identity` runs
`Lane::exp2_int_in_range` through G1's scalar/Simd4/Simd8 operation table over its full legal
integer domain. `cargo test --locked -p math --features lane --release --test m2_lane_identity`
adds a named differential test for both `fast_gain_from_db` and `exp2_lane` over NaN payloads,
infinities, signed zero, minimum and maximum subnormals, minimum normals, and the one-ULP
neighbourhoods of `-127`, `-126`, `126`, and `127`.

Both `cargo test --locked --workspace` runs pass: detached `origin/main` at
`879269886102664f1c2194ee15b44fab528075c2` and the committed evidence candidate at
`a55234beafca0222266af4308cabc8a1759c8a63`. The candidate adds exactly one named test case;
adding `exp2_int_in_range` to G1 expands the existing two table-driven identity tests instead of
adding another harness test.

## Console benchmark

Exactly one invocation was made from clean committed candidate
`a55234beafca0222266af4308cabc8a1759c8a63`:

```sh
MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1 \
  scripts/run-console-benchmark.sh --issue388-lane4-evidence
```

The new arm was unconsumed, so the runner did not delete or overwrite any tracked artifact. Its
disposition is `PASS/complete`: one runner invocation, one warmup, two measured rounds, CPU 63,
and three workload launches. The shared host exceeded the load-average ceiling, so the documented
override is recorded truthfully as `measurement_control=uncontrolled`; this capture makes no
performance acceptance claim.

For `sixty_four_track_compressor_only`, round 1 recorded p50/p95/p99
`47.000/47.631/50.916` us/block and round 2 recorded `46.529/47.151/50.426` us/block. Both output
digests are `35b1d89136c8253983f3f2d306c1b4d35e811ccdabaebcfa214802edd458303c`.

Evidence identities:

- disposition: `5f3a086d82af3d55b30c47d5d8937288039f8aaec84b14fe2d91f4b3788f9027`
- raw: `e2b41b41f74437089a4df99a54c8eeea65870ae160239fb31cbe5b7c28a973aa`
- accepted: `e2b41b41f74437089a4df99a54c8eeea65870ae160239fb31cbe5b7c28a973aa`

## Caller disassembly

`lane4_codegen_probe.rs` is the identical non-inlined Simd8 probe compiled at both requested
revisions with the workspace release profile. The retained files contain the full output of:

```sh
llvm-objdump -d --demangle --no-show-raw-insn \
  --disassemble-symbols='lane4_codegen_probe::issue388_exp2_lane_simd8,lane4_codegen_probe::issue388_fast_gain_from_db_simd8' \
  target/release/examples/lane4_codegen_probe
```

At `9c0623186943d97bf8949986cce72928b8631596`, each caller contains two `vmaxps` and two
`vminps`. At `2b38ba7fec33c6ddf8247ffff0d4b6b639aebbc9`, each contains one of each: the redundant
`exp2_int` clamp is gone and the caller-owned clamp remains.

- before objdump SHA-256:
  `df2200c7b79755e18422fc718cd55a309432f91417c3d69ef650d00cd136d570`
- after objdump SHA-256:
  `712af086d76dedd43a7300bf43771d87dad18c28101b6f279acf24e0273324aa`
- probe source SHA-256:
  `9573c59072c8913e7716c027c8a8ea36cc56ad310533b9a3ed3b2b53e9bbc4eb`

## Gate record

The following candidate gates pass:

- `cargo fmt --all -- --check`
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
- `cargo test --locked -p lane --release --test g1_op_identity` (9 passed)
- `cargo test --locked -p math --features lane --release --test m2_lane_identity` (4 passed)
- `cargo test --locked --workspace`
- `scripts/test-console-benchmark.sh`
- `scripts/check-realtime-policy.sh`
- `jq -s -e -L scripts -f scripts/console-benchmark-validator.jq` over the accepted capture

The detached `origin/main` workspace test also passes. No production Rust source, AArch64-specific
source, floor-accounting table, or `tools/bench/src/floor.rs` changes on this branch.
