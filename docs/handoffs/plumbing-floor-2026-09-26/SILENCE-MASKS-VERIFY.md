# Adversarial verification: #940, #941, #942 (silence masks), 2026-09-26

Base: `main` at `14f2917b`; prototype `silence-masks-prototype.patch` applied in a scratch detached
worktree. Host: EPYC 7313P, rustc 1.97.1, taskset core 28, CARGO_INCREMENTAL=0, loadavg ~2.5.
GitHub bodies of #937/#940/#941/#942 equal the local specs (trailing newline only).

## 1. Skip theorem (#940 Research findings (2)) -- CORRECT

Randomized checker (standalone Rust, scalar SSE f32, unfused `(lr*r)+(ll*l)`, left-to-right
chain, first stored; skip model = compacted live inputs in pairs, `fix[p]` from the silent mix,
fill `+/-0` when no input is live, one `x + (+0.0)` after the chain):

* 1,000,000 finite hostile cases (fan-in 2-20 and 64; frames 1-16; coefficients +-0, -0.0,
  all-rows-negative sets, tiny normals whose products underflow, subnormal coefficients, large;
  data +-0, subnormals incl. min subnormal, 2^-24..2^25, whole -0.0 and +0.0 blocks, cancellation
  partners; played sets none/first/last/first-two/leading/all-but-one/all/alternating/random):
  **0 mismatches**. Fix-up changes bits in 116,229 cases (so class B is real).
* 1,000,000 cases with +-inf coefficients, overflowing data, inf and NaN inputs (x86 NaN
  propagation): **0 mismatches** (857,200 cases had NaN outputs; non-finite silent mixes took the
  "served silence, treated as live" fallback).
* Fix-up placed at the first `+0.0` skipped position instead of after the chain: 0 mismatches.
* Under MXCSR FTZ (0x9F80) and FTZ|DAZ (0x9FC0): skip != full in 1 of 62,500 sampled cases each,
  confirming the brief's remark that the rule needs gradual underflow.
* Mutation detection rates in the same corpus (host planes pre-filled with a nonzero sentinel):
  drop fix-up 11.6 %; fill +0 when all unplayed 2.5 %; fix-up whenever anything skipped 4.8 %;
  swap within pair 7.1 % (only non-first pairs can show it: IEEE `a+b == b+a`); first pair
  `initial_store=false` 78.9 %.

FP environment: `CANONICAL_MXCSR = 0x1F80` (fpenv.rs:84) entered at host-core
render_session.rs:109/126/144, capi ffi.rs:744, scalar_point_endpoint; AArch64 FPCR 0 (FZ=0,
DN=0); wasm has no control word and the core spec mandates subnormals. host-web calls
`plan.render` directly (wasm only). Confirmed.

## 2. Engine reproduction of the prototype (instrument (b))

p50 cycles/block, three interleaved repeats, same runtime per row:

| k | base | restr | skip |
|--:|--:|--:|--:|
| 0 | 8,637-8,711 | 7,597-7,668 | 7,597-7,671 |
| 16 | 8,226 | 7,594-7,597 | 6,636-6,669 |
| 48 | 6,854-6,858 | 6,854-6,891 | 4,302-4,335 |
| 56 | 7,227-7,264 | 7,449 | 3,780-3,817 |
| 64 | 6,336-6,373 | 6,632-6,669 | 3,188-3,222 |

Matches the brief. But: `skip` and `restr` are one function (`route_reduce_skip`) whose `skip`
flag is read only on a `None` input, so at k = 0 the two arms execute identical code. "The skip
itself costs nothing" and G4 Stop 1 are tautological; the -1,000 at k = 0 is the prototype's
restructure, not #937's kernel, and nothing compares the new kernel to #937's. The restructured
declined arm is slower than base at k >= 56 (+200 to +300 cycles).

Scratch test on the prototype tree (dense digest `57535244...` reproduced):
* #941 gate 2 on main's kernel, claims 16..63 absent: counts `[0, 1024, 3072]` = `[0, 16x64, 48x64]`.
* #941 gate 3 on main's kernel: None digest == Some(all +0.0) digest for k = 1, 48, 63, 64.
* prototype skip == restr == main digests at every k; the prototype drops all source-plane
  counts (`[0,0,0]`), as the brief warns.
* `check-realtime-policy.sh` and `check-graph-policy.sh` pass on the prototype tree.

## 3. Scope/value

* Only `GraphCompiler::compile` (builtins-less) produces a plan where the Output reads a claim in
  place (clause b' in `source_plane_table`, runtime.rs ~5900: the claim's only reader must be the
  retired route). Product hosts (host-core prepare.rs:1195 `compile_with_builtins`; capi
  compile.rs:410 and host-web via `prepare_host_runtime*`) always prepare builtins, so the skip
  can never fire in a product plan (stronger than "bankless only": even a scalar-backend
  builtins plan has no Output claims). #940 moves only the ring rows; after #937 the dense ring
  row has k = 0, so only the new sparse row can move.

## 4. #941 authorized paths are insufficient

Adding a workload kind requires, as #928 did (commit ed1ce679): `scripts/console-benchmark-record-lib.jq`
(`session_kinds`, `driver_fed_kinds`, per-kind facts incl. `input_signal == "tone+absent"`,
`floor_pins`), `scripts/console-benchmark-validator.jq` (record count `== 48`, ring pairing),
`scripts/test-console-benchmark.sh` (kind table :868, :939, tolerance table :959), and
`tools/bench/src/floor.rs` (exhaustive `floor_row` match; the `None` arm asserts
`NineTrackBaseline` only, :419-420; jq/Rust floor-table comparison). None is authorized.

## 5. #942 codegen and timing (verbatim replica, workspace flags)

x86-64 `+avx2,+fma`, fat LTO: today's `chunks(32)` IS vectorised (`vpor ymm` inner loop), but pays
a runtime trip-count dispatch plus `vextracti128`/2x`vpshufd`/`vmovd` reduction per 32 words.
`chunks_exact(64)` becomes 8 x `vpor ymm` + `vptest`, no reduction. Gate 3 ("a packed OR in the
loop") is satisfied by today's code, so it discriminates nothing.

wasm32 `+simd128`, release/fat LTO (the AudioWorklet build flags): `chunks_exact(64)` is emitted
as 64 scalar `i32.load` + 64 `i32.or` per chunk (only the remainder loop uses `v128.or`); today's
loop uses `v128.or`.

Cycles (x86, min of 40x2000) and ns (Node 22 / V8, min of 30x20000):

| input | today | chunks_exact(64) | chunks(128) | chunks(64) |
|---|--:|--:|--:|--:|
| x86 2x128 silent | 66 | 26-30 | 38 | 46 |
| x86 2x1024 silent | 501-508 | 210 | 217 | 299 |
| x86 live, first chunk | 13-17 | 8-10 | 19 | 18 |
| wasm 128 silent | 15-16 ns | 52-54 ns | 10.2 ns | 12.1 ns |
| wasm 1024 silent | 118-120 ns | 402-406 ns | 68 ns | 90 ns |
| wasm live, first chunk | 4-6 ns | 25-27 ns | 10 ns | 6.7 ns |

Property check: every variant agrees with `iter().all(bits == 0)` on 12,600 single-word
patterns (-0.0, min subnormal, qNaN, 1.0) over lengths 0..=2100. Callers (parametric-eq
2089-2175, compressor 513-592, true-peak-limiter 644-646 on state arrays and 2208-2327) use only
the truth value.

## Cleanup

Scratch worktree removed with `git worktree remove --force`; scratch target directory deleted.
