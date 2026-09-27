# #943 adversarial verification: evidence

Base: detached worktree of `origin/main` 65671c21 (code identical to the brief's 14f2917b; #939 is docs only).
Host: 32-thread EPYC, `x86-64-v3`, `CARGO_INCREMENTAL=0`, runs pinned with `taskset`, one scratch target dir.
Prototype patch (P1 wired as the interface contract says, plus throwaway tests):
`scratchpad/meters-verify-prototype.patch`. wasm harness: `scratchpad/meters-verify-wasmcheck-lib.rs`, `-run.mjs`.
The worktree and target dir were removed afterwards.

## Verdict: brief with amendments

## Findings, by severity

### M1. The frozen kernel text fails `check-lane-policy.sh` (G6)
- The brief freezes `peak = c.max(peak)` in `crates/lane/src/kernels/builtins.rs`.
- The lane policy's D8 marker rule greps `\.(max|min|...)\(` across `crates/lane/src`. Every `.max(` needs a `LANE-OP-OK` marker. Run on the literal text:
  `crates/lane/src/kernels/builtins.rs:1574: peak = c.max(peak);` gives `lane policy failure: ... needs a LANE-OP-OK marker (D8)`.
- The crate's own convention is UFCS: `Lane::max(n, ..)` (`scalar.rs:171`) and `$crate::Lane::max(..)` (`wide_impl.rs:328`). `L::max(c, peak)` passes the policy.
- **Amend** interface item 1 to `peak = L::max(c, peak)`, and change the expected-green mutation to `L::max(peak, c)`.
- **Also amend** the loop to `for frame in words[..frames * L::WIDTH].chunks_exact(L::WIDTH)`, which is what every other kernel in the file uses. The `L::load(&words[f * W..])` form keeps a per-frame bounds-check branch: the wasm disassembly shows `local.get 4; local.get 1; i32.gt_u; br_if` inside the loop. That contradicts "branch-free per frame". Exactness is the same either way.

### M2. The G5 opcode census names opcodes that are not emitted, and it cannot isolate the pass
- LLVM folds `abs(x) >= MIN_POSITIVE && abs(x) < INF` into an integer range test:
  `v128.and 0x7fffffff; i32x4.sub 0x00800000; i32x4.lt_u 0x7f000000`.
  - The result is exact: it is the normal-finite test on the bits.
  - No `f32x4.ge` is emitted. `f32x4.lt` appears only where the dead `Simd8` arm's `max` lowered to `lt + bitselect` rather than `pmax`.
- Inlined as specified, the pass lands in `graph::GraphExecutor::render` (func 2295). That function holds 42 calls and other SIMD code (census: `pmax 2, abs 6, f32x4.lt 4, i32x4.lt_u 6, bitselect 10`), so a census of "the function holding the pass" does not discriminate.
- With `#[inline(never)] fn bank_sample_peak` in graph, the function census is: `f32x4.abs 6, v128.and 6, i32x4.sub 6, i32x4.lt_u 6, v128.bitselect 10, f32x4.pmax 2, f32x4.lt 4`, with zero scalar `f32.gt/abs/max`.
- Operand order is correct. `f32x4.pmax(peak, c)` is `peak < c ? c : peak`, which is the D8 `max(c, peak)`.
- **Amend** G5:
  - (a) Make the width dispatch an `#[inline(never)]` graph fn and census that function. Expect {abs, and, i32x4.sub, i32x4.lt_u, bitselect, pmax or lt+bitselect} and zero scalar compare/max.
  - (b) Preferably, add the kernel to `tools/wasm-gate-corpus` so `scripts/run-wasm-gates.sh` executes it and compares against native. That adds the corpus and its pins to the authorized paths.
- My executed check under V8 (node) found 0 mismatches, both over all 2^32 bit patterns (Simd4 sanitize against the Rust scalar oracle and against an independent JS bit oracle) and over 400 randomized carried 64-block kernel runs with the seeded-zero merge.

### L1. Scope statement is broader than the reach
- **Plans bound with an observation activation.** `selective_observation = runtime.has_observation_activation()` (`graph/src/lib.rs:2483`) routes *every* observer, permanent ones included, through `observe_active_unit` (`:2538-2556`; permanent rows are copied into the snapshot, `observation_activation.rs:210-212`). So P1 never runs in any plan bound with an activation.
- **Web legacy spectrum boot.** This boot binds **ALL** meters: `prepare_host_runtime_with_console_and_spectrum{,_collection}` pass `selected_meters: None` (`host-core/src/prepare.rs:544-584`), which maps to `MeterMetricSet::ALL` (`:1087-1095`, `hosts/host-web/src/lib.rs:7744-7771`).
- P1 therefore reaches only the default `(None, None)` web boot.
- **Amend** F3 and the Mission to say this. Add a G3 control: a permanent `SAMPLE_PEAK` plan plus one controlled observer runs 0 passes and stays bit-identical.

### L2. Small interface and hazard gaps
- Item 4: compute `members.len() - lanes` with `checked_sub`. Otherwise a malformed bank panics at bind.
- The hand-off gives `Some` to every observer on the member, including spectrum capture and ALL meters. This is harmless only because the meter fast path re-checks `metrics == SAMPLE_PEAK`. The field doc should say it is the max of the *sanitized* magnitude.
- Hazard 7's list is incomplete: `host-core/tests/observation_demand.rs:946-1030` also pin `test_only_peak_samples`. All of those tests are on the controlled path, and they stayed green.

## Confirmed claims

### F1: confirmed
- `bind_rack_banks_indexed` and `planned_builtin_bank_members` never read observers or meters.
- `chains_into` (`runtime.rs:7273-7307`) only declines merges.
- `has_observer` (`:6538`, `:6573`) counts every prepared binding, controlled or not.
- The web host requests `PostMatrix` / `SAMPLE_PEAK` (`host-web/src/lib.rs:7725-7737`, `:8300`), and the controlled policy refuses any other tap (`builtins-compiler:3388-3412`).
- The C ABI only publishes its own master-peak scan (`capi/src/runtime/control.rs:411-452`), with no builtin meter and no tap. Native and mobile hosts bind no meters.

### F4 exactness: confirmed
- **Sanitization, exhaustively.** Over all 2^32 patterns at `Simd8`, `Simd4` and `f32`, the kernel's sanitizer equals `normal_or_zero(x) ? x : 0` then `abs`, with 0 mismatches. This includes NaN, whose `abs` is NaN and whose ordered compare is false, giving `+0`.
- **The harness is live.** Mutation L-1 (`ge(zero)`) is caught at `0x00000001`.
- **Reassociation.**
  - Select-max is commutative and associative on the 47-value sanitized domain.
  - The domain restriction is load-bearing: `m(+0,-0)` and `m(-0,+0)` differ in bits, and so do `m(1,NaN)` and `m(NaN,1)`.
- **Kernel identity.** Carried kernel = serial scalar = `<f32>` kernel, and the seeded-`+0` partial merged into the window = serial. Tested at frames 1, 2, 3, 127, 128 and 129, 64 blocks, hostile and tone input.
- **Accumulator differential.**
  - Setup: the brief's `observe_input_with_block_peak` against `observe_input`, 8 meters.
  - Configurations: periods {1, 64, 127, 128, 129, 300, 512, 1536, 4096}; 7 metric sets; hold/decay {0/0, 7/0, 0/12, 100/60}; fixed 128-frame streams and random streams with skips (discontinuity), `restart_observation`, both `reset` kinds, zero-frame and variable blocks.
  - Result: 14,302,984 snapshots bit-identical on every field. Non-`SAMPLE_PEAK` sets never merge.
  - Mutations: B-1 goes red (snapshot count differs) and B-2 goes red (bits differ).
  - Merge counts: 512 at period 512, 0 at 64, 296 at 300, 512 at 1536.
- **DAZ/FTZ irrelevant.** No subnormal ever reaches `max`, the compares flush consistently, and observers run inside the render `fpenv` guard anyway.

### I4 and I5: confirmed
- Observer slices are never mutated after `new_with_observation_activation`; the only `observers.*` mutations are in bind and tests. `MeterObserver` metrics are fixed at prepare. The flag cannot go stale in a plan without activation, and plans with activation never call `observe_unit`.
- The pass is pure and precedes the member loop. `?` short-circuit is unchanged.

### Layout and layering: confirmed
- With `sample_peak: bool` added, `rt9_identity_metadata_has_no_retained_or_peak_layout_delta` passes (29 of 32 bytes used).
- Adding `sample_peak: Option<[f32;2]>` to the public block and a defaulted `accepts_sample_peak` breaks nothing:
  - `cargo check --workspace --all-targets --all-features` is clean, and the host-web wasm build succeeds.
  - The only constructors are `runtime.rs:3630` and `:3704`, and nothing destructures the block.
- `check-{lane,graph,builtins,realtime,workspace}-policy.sh` and `check-unfused-seal.sh` pass once M1 is fixed. Clippy `-D warnings` is clean on the lib crates.
- Callgraph rules 1-3 pass on the prototype artifact.

### Suites on the prototype
- Green: graph (except the predicted source-scan test `resident_meter_entry_has_one_final_output_dispatch_and_admission_control`), graph-compiler 75/75, builtins-compiler (test-support), host-core `--all-features`, host-web, console-workload, lane and rack.
- G3-like: concurrent and between-render-calls deliveries, periods 512, 300 and 1536.
  - PCM and every meter field are bit-identical, pass on against declined.
  - Shape is `[8,48]` in both arms, with folds 64, redirects 0 and transposes 192.
  - Passes are 192 (24 x 8) with the pass on, 0 declined, and 0 on the ALL arm.
  - Left and right peaks differ in at least one window.
- G4-like: 1,000 blocks, `audit.total() == 0`, passes = 8,000.

## Measured (pinned, same runtime, paired)

**Microbenchmark, ns per bank per block:**

| configuration | scalar | pass + merges |
|---|---:|---:|
| `Simd8`, 8 `SAMPLE_PEAK` meters | 2,194-2,226 | 275-293 (7.6-8.0x) |
| `Simd4`, 4-lane bank | 1,095 | 246 |
| ALL meters, ungated pass | 3,323 | 3,547 (+0.22 µs per bank, about +1.8 µs per 64-track block; the brief says +2.3) |

**In situ.** Web shape, 64 x `SAMPLE_PEAK` at `PostMatrix`, period 1536, 6,000 rounds alternated per block, two pinned runs:

| run | p50 pass on | p50 pass declined | paired median saving |
|---|---:|---:|---:|
| 1 | 135.1 µs | 152.5 µs | 17.4 µs |
| 2 | 135.6 µs | 152.3 µs | 16.7 µs |

- This matches the brief's -15.5 to -16.8 µs.
- The product period (12 x 128) takes the fast path on every block: the lease frames are fixed at the quantum, and the prototype recorded 1,536 merges in 24 blocks x 64 tracks.
