# Rule how class-A identity treats NaN encodings across CPUs

**Owner ruling (2026-09-29, decision 10 in `docs/rulings/engine-footprint-2026-09-29.md`):** option A. Class-A identity treats every NaN as one value; the engine does not canonicalize NaNs at render.

Found by the AArch64 CI leg (#1017). Owner ruling needed before implementation.

## Problem

When an operation produces a NaN (for example `inf - inf`), x86 gives `0xFFC00000` and AArch64
gives `0x7FC00000`, and the two CPUs choose differently when both inputs of an operation are NaN.
The WebAssembly spec leaves the sign and payload of such NaNs unspecified, so the shipped browser
module inherits the host CPU's rule: V8 on an x86 laptop and V8 on an Apple-silicon Mac or an
Android phone can emit different NaN bits for the same session.

Under emulation on #1017's AArch64 leg, six tests differ only in NaN bits, in debug and release:
compressor `kernel::settled_body_tests::scenario_{981,983,985,995,1006}` and the EQ bank's
`admitted_blocks_render_the_base_bits_without_selects`. With every NaN folded to one value, all
six compressor scenarios and all three EQ legs give identical digests on both CPUs. #1017 carries
them as named expected failures under this issue.

Since #1049, two remain: compressor `kernel::settled_body_tests::scenario_1006_ramping_prefix_is_pinned`
and the EQ bank's `admitted_blocks_render_the_base_bits_without_selects`. #1049 deleted the
compressor's `scenario_{981,983,985,995}` pins (and `scenario_982`) as dominated, with their
expected-failure rows.

This is not LANE-3 (#1019). LANE-3 is LLVM folding `max`/`min` into `fmaxnm`/`fminnm`, which a
code shape can prevent. NaN generation and propagation is the CPU's arithmetic rule, and no code
shape changes it short of canonicalizing NaNs.

## Owner decision

- **A. Class-A identity treats all NaNs as one value (recommended).** Class-A comparisons, digests
  and differentials fold every NaN to one canonical word before hashing. The claim becomes: same
  bits on every target, except that a NaN may carry a different sign or payload. This matches what
  the wasm spec already guarantees, costs nothing at render, and keeps the existing NaN-safety
  rules: finite input must not produce NaN, and each effect's documented NaN behaviour still holds.
- **B. The engine canonicalizes NaNs.** Every kernel that can produce a NaN, or the output
  boundary, rewrites NaNs to one word. That gives exact bits everywhere, at a per-sample cost in
  hot kernels and a new rule that every future kernel must follow.

## Smallest closable slice (option A)

Fold NaNs in the class-A digest and differential helpers (one shared helper, test-side only);
document the rule in the effect contract and target matrix; turn #1017's remaining named expected
failures into ordinary passing tests on the AArch64 leg; keep one test that proves a NaN-producing
scenario still yields NaN (so folding cannot hide a NaN appearing where finite output is required).

## Objective gates

1. The two remaining tests pass on the AArch64 leg and on x86, with no expected-failure entries left
   for them.
2. A planted change that turns a finite output into NaN still fails the finite-output tests.
3. #1019's gate 1 refers to this rule for NaN payloads instead of requiring raw NaN-bit identity.

## Attempt 1 evidence

Terra, 2026-09-29, branch `codex/1065-nan-one-value` from `codex/batch-slim-4` (`b695758b`).
Implementation commit `88f276df` (19 files, +316 / −135), then this record. No product line
moved: every Rust hunk is in a `#[cfg(test)]` module, a `tests/` file, the test-only
`dsp-reference` crate or the `audit` tool.

### The helper

`crates/dsp-reference/src/class_a.rs` (`dsp-reference` has no dependencies and no production
dependent): `NAN_WORD = 0x7FC0_0000`, `word(bits)`, `bits(f32)`, `same(a, b)` and `le_words(bytes)`
(a payload or plane folded word by word; hashing its chunks equals hashing the buffer when it holds
no NaN). Unit tests: the x86 default NaN, the AArch64 one, payloads and both signalling forms fold
to one word; both zeros, both infinities, `±MAX` and the subnormal boundaries keep their bits; a NaN
is never `same` as a number; `-0.0` and `+0.0` stay two values. `lane` gains it as a
dev-dependency (policy-clean: `check-lane-policy.sh` admits workspace crates, and
`check-conformance-boundaries.sh` reads only `[dependencies]` and `src/`).

### Which digests can see NaN: measured, not guessed

A NaN spy (a scratch copy of `digest` 0.11.3, patched in with `cargo --config patch…`, counting
every hashed 4-byte word in the NaN range per test thread) ran over a copy of the base tree with
the CI test sets: `test-debug-a`'s workspace set, `test-debug-b`'s DSP set, `test-release`
(`lane`, `math`, `wasm-gates` incl. G5 and G6) and `cargo test --release -p audit -p bench -p
console-workload`, then `check-builtins-fixtures.sh`, `check-effect-contract.sh` and
`check-console-fixtures.sh` with spied binaries. Three pinned digests hash NaN words:

| digest | NaN words | on x86 |
|---|---|---|
| compressor `scenario_1006_ramping_prefix_is_pinned` | 28,800 (12,520 already folded: the all-wet kernel words) | `0xFFC00000` ×9,348, `0x7FC01234` ×6,444, `0xFFE00001` ×6,237, `0x7FC00001` ×5,941, `0x7F800001` ×451, `0xFFA00001` ×379 |
| EQ `bank` `admitted_blocks_render_the_base_bits_without_selects` | 48 (16 per leg, the poisoned dry lane's integrators) | `0xFFC00000` ×48 |
| limiter `seedless` `the_seedless_scenario_renders_the_pinned_base_words` | 40 | `0x7FC00000` ×40 (the planted `f32::NAN`, copied) |

The only other hits were halves of `math` M3's `u64` result words (M3, and G5/G6 which replay
it); a rerun ignoring 8-byte updates gave zero hits for G5, G6 and M3, and no 8-byte update was an
`f64` NaN. So the G5 corpus (its rule 2 and `g5_lane_corpus_is_finite`), the console digests
(`console-workload`), the builtins fixtures and the effect-contract harness never hash a NaN
word, and are unchanged.

### What folds through it

- The three digests above: output words, recursive and state words, and payload words
  (`le_words`). Masks and report counters are integers and are hashed as before.
- The compressor's grid and randomized differentials (`assert_words`, `assert_state`): every
  word class-A in every block. #982's relaxation was granted only in blocks whose witness showed
  the all-wet arm ran; under the ruling it is universal. The per-step assertion that a NaN payload
  differs only in a block the boundary check rejects stays.
- The EQ ramping differential (`ramping_elision::scenario`: outputs, `fingerprint`, `payloads`).
- The ad-hoc "both NaN" comparisons that already existed now call the helper: `lane`
  `g2_kernel_identity::same_or_both_nan` and `fader_matrix::same_word`, the limiter's #1013
  annex-2 induction and `assert_same_words` (its randomized differential), and the `audit unfused-fma`
  sweeps (each rewrite is the same predicate: `!same(u, flush(y))` is the old two-clause test,
  since `flush` keeps a NaN a NaN).

Left alone, on purpose: strict `to_bits` comparisons that do not fold NaN today. They compare two
renders on one CPU, pass on both CPUs (#1017's leg), and are stricter than class A; relaxing them
buys nothing. G5's op-level counts: the max/min lowering pool compares which operand's NaN a
`select` returns (D8's contract, and how LANE-3 shows), and the `f64`-lane counts already compare
an `f64` NaN as one class; the helper is `f32`-only. `math` M2 is #1019's.

### Re-pins: only NaN words moved

The fold is applied with temporary instrumentation (not committed) that could hash the old way
and count what it folded.

- **Compressor `SCENARIO_1006`**: `162979dd…` → `bd3d711f86bbd00f015a0ead7e04116daabd154b2b8e9d8ef17dc6e616b7382c`.
  The same render hashed the old way (raw everywhere except the all-wet kernel words) gives
  `162979dd…` on the new tree, so the stream is unchanged; the old pin hashed 16,280 NaN words
  raw, none of them `0x7FC00000`, and those are the only words the fold changes.
- **EQ `SELECT_DIGESTS`**: scalar `9316456b…` → `3719d502178e4c1e65fd01d18b9664d4a50259a1fc61d733cd229cd1e7e9f6e3`,
  bank `d4a1dc9d…` → `d68a2494011d118d10eb295683957bd55c09092259d9161ae0bd0a0cfc6ac5b7`,
  bank-mono `f442a0d3…` → `e5db81b9acb69a451505fbd48add97d960c928f934b9fc20c6fd136a88b725a6`.
  Hashed raw, the new tree still gives the three #977 pins; the fold changes exactly the 48
  `0xFFC00000` words. **The folded x86 scalar digest is the digest the AArch64 leg printed** (the
  deleted row's reason, `left: "3719d502…"`): AArch64's arithmetic NaN is already `0x7FC00000`, so
  folding x86's reproduces AArch64's stream.
- The limiter's pins do not move: its 40 NaN words are already `0x7FC00000`. Every other pin in
  `bank.rs` (`ODD_LIVE_DIGESTS`, the skew and block-limit pins) is unchanged.

### Gate 1: the two tests pass, with no rows left

- x86: both pass in dev and release (below).
- AArch64: **qemu is not available here** (no `qemu-aarch64`, no `aarch64-unknown-linux-gnu`
  target), so the arm64 CI legs are the confirmation. The EQ case is shown above without it. For the
  compressor, #1017's evidence found the digests identical on both CPUs with every NaN folded, and
  this fold covers every float word the digest hashes (kernel, recursive, finished and payload
  words).
- `scripts/lib/aarch64-known-defects.py`: the two #1065 rows are gone and `debug` is `[]`.
  `run-aarch64-tests.sh` refused a leg with no rows (`((${#rows[@]} > 0))`); it now fails only when
  the table cannot be read, and an empty table names nothing to skip (simulated for both modes).
- x86 resolution of both legs: the debug set (25 product crates, `dsp-reference`, `conformance`,
  `target-smoke`, the leg's features) builds with `--no-run` and lists 1,789 tests, both former
  expected failures among them, and `judge-skips debug` passes; the release set (`lane`, `math`)
  builds and `judge-skips release` passes, each #1019 row naming exactly one test.

### Gate 2: a planted finite-to-NaN change still fails

Two guards were added. `scenario_1006` asserts that NaN still reaches the kernel words (the
non-vacuity guard: `nan_words > 0`) and that every finished word, after the boundary check, is
finite. The EQ bank's `fold_words` asserts every digested output word finite. Three plants, each
reverted byte for byte:

| plant | failing tests | includes |
|---|---|---|
| P1: compressor all-wet settled output × NaN (before the boundary check) | 32 | all three randomized differentials, both grids, `scenario_1006`, `the_corpus_is_finite`, the f64-oracle and partition tests |
| P2: compressor `finish_channel` writes NaN into word 0 of an accepted block (oracle and candidate both see it, so the differentials cannot) | 20 | `scenario_1006` ("a finished word is not finite (block 0, W1)"), `passes_effect_contract_conformance`, the ramping bank scenario, partition and dry-bit tests |
| P3: EQ writes NaN into word 0 of an accepted dual block | 19 | `admitted_blocks…` ("an output word is not finite: 0x7fc00000"), every other `bank.rs` pin, conformance, time-domain tests |

P2 is the case the fold could hide: the differential compares NaN with NaN. The finiteness guard
is what catches it.

### Other gates

- `cargo check --workspace --all-targets --all-features`: ok. `cargo clippy --workspace
  --all-targets --all-features -- -D warnings`: ok. `cargo fmt --all -- --check`: ok.
- Dev: `cargo test -p dsp-reference -p compressor -p parametric-eq -p true-peak-limiter -p lane
  --all-targets --features math/lane,parametric-eq/test-support,lane/test-support`: all pass;
  `parametric-eq --test bank` without features: 11 pass. Release: the same set: all pass.
- `cargo test --locked --release -p audit -p console-workload`: all pass; the console digests are
  unchanged (and hash no NaN).
- `scripts/run-wasm-gates.sh` (native, wasm simd128 and the V8 spill gate): ok.
- Policy scripts, 55 runs, all ok: every `check-*`/`test-*` policy pair (workspace,
  conformance-boundaries, lane, unfused-seal and its `--self-test`, env-vocabulary, realtime,
  bench, builtins, effect-runtime, graph, host-core, protocol-control, rack, session,
  realtime-audit-leak, artifact-evidence-leak, dsp-research), `check-parametric-eq-render-contract`,
  the effect-runtime and builtins fixture scripts, `check-console-benchmark-fixture`,
  `check-bench-preconditions`, and with `python3 -B`: `check-`/`test-ci-path-routing`,
  `check-`/`test-script-reachability`, `check-`/`test-test-support-ci`,
  `check-release-shape --self-test`, `check-scalar-oracle-absent --self-test`, `check-sdk-deletions`
  (and `--self-test`), `aarch64-known-defects --self-test`, `check-web-audioworklet-v8-spill
  --self-test`, `web-audioworklet-identity --self-test`, `test-npm-publish-modes` and the
  stem-identity fixture check.
- Not run: the 2^32 `audit unfused-fma` sweep (a manual report, not CI; the rewrite is
  predicate-for-predicate).

### For root

- **#1051** adds `dsp_reference::randomized::same_word`, the same predicate as `class_a::same`.
  At merge, make it delegate to (or re-export) `class_a::same` so the fold lives in one place.
  `Cargo.lock`: #1051 and this branch touch different stanzas. The compressor `settled_body_tests`
  hunks do not overlap #1051's `randomized_width` hunk.
- The compressor's `MUTATIONS.md` (#1006) and the EQ's (#977) name the old relaxation and pins;
  each got a one-clause note rather than a rewrite.

## Sol verdict, attempt 1

**PASS.** Sol, 2026-09-29. Verified on a scratch merge of `f1ad03f9` into the batch head
`codex/batch-slim-4` at `adddb4e5`. Rust 1.97.1, Node 22.23.2, x86-64 only (no qemu, no
`aarch64-unknown-linux-gnu` target here).

### The merge

`git merge --no-ff` auto-merges with no conflicts. There is no semantic conflict either:
- #1046's deletions (the EQ `descriptive_bank_throughput` and two `interleave_identity` tests, the
  compressor and limiter test files) touch none of #1065's hunks.
- Every finiteness test the plants below rely on still exists.
- #1049's pin deletions are already in the base (`b695758b`).

**#1051 is a trial merge on top, not committed.** It auto-merges and
`cargo check --all-targets` passes. `randomized::same_word` (`bits equal || both NaN`) is the
same predicate as `class_a::same`: the fold maps a NaN to `NAN_WORD` and leaves every other word
unchanged, and no other word equals `NAN_WORD`. At merge, #1051 should make `same_word` a
re-export of, or a one-line delegate to, `class_a::same`, and cite it in its docs. #1051's own
review should check its raw `to_bits` comparisons for NaN exposure:
- `parametric-eq/src/randomized_restores.rs:338` and `:469`;
- `host-core/tests/randomized.rs:210`.

### Checks

1. **The fold is narrow.** An exhaustive scratch program ran over all 2^32 words, using the
   committed `class_a.rs` through `#[path]`:
   - `word(b) == NAN_WORD` exactly when `b` is a NaN: 16,777,214 encodings, both signs, quiet
     and signalling. Every other word maps to itself, and none of them equals `NAN_WORD`.
   - A pair table keeps these unequal: `±0`, adjacent subnormals, subnormal against zero, the
     largest subnormal against the smallest normal, `1.0` against the next float, `±inf`, `MAX`
     against `inf`, and a NaN against a number.
   - `le_words` refuses a ragged length.
   - The `audit unfused-fma` rewrites are the same predicates as before (`flush` keeps a NaN a
     NaN).
2. **No NaN-hashing comparison was missed.** I built my own spy: `digest` 0.11.3 patched to log
   every hashed NaN-range word, in one atomic line per update. The workspace's `sha2` 0.11 is its
   only user; `sha2` 0.10 serves only wasmtime. I ran it over the merge with test-debug-a,
   test-debug-b, test-release (`lane`, `math`, `wasm-gates`), audit/bench/console-workload in
   release, and `target-smoke`.
   - Only the three sites hash 4-byte NaN words: `scenario_1006` (28,800), EQ admitted-select
     (48) and limiter seedless (40). All of them are now `0x7FC00000`, with none left
     non-canonical.
   - G5, G6 and M3 show only 8-byte updates, and none of those is an `f64` NaN (M3 is guarded
     NaN-free).
   - The two FNV digests that bypass `digest`, graph `SourceRun::digest` and host-core
     `source_in_place`, needed a separate spy. They hash NaN only in three #936 inert tests:
     6 `HOST_PAD` words per master, which is pad memory the render never touches. The hash is
     independent of the CPU and correctly left raw.
   - Raw NaN-literal assertions in the tree (`disjoint`, `rack`, `graph`, `conformance`) check
     poison and sentinel copies, not arithmetic results.
   - Real arm64 hardware agrees. Main run `36507292157` (job `109211631805`, `a8955ad4`) passed
     the debug leg with only the two #1065 rows failing, each for its stated reason (`d13e0831…`,
     `3719d502…`). So every other raw comparison already passes on AArch64.
3. **Finiteness is still guarded.** Each plant was reverted afterwards.
   - **C1.** In the compressor's shared `applied_gain`, a gain below −6 dB becomes NaN, for the
     oracle and the candidate alike. 19 tests go red, including:
     - `scenario_1006` (digest `77888f08…`);
     - `the_corpus_is_finite` ("word 949 is NaN");
     - the f64 oracle and the partition tests.

     The grid differentials stay green, as expected when both sides are NaN.
   - **E1.** In the EQ's `process_channels`, any output above 0.3 becomes NaN. 27 tests go red,
     including `admitted_blocks…` ("an output word is not finite: 0x7fc00000"), conformance and
     the DFT oracle.
4. **Empty-leg acceptance cannot pass vacuously.** The real `run-aarch64-tests.sh` ran on x86
   with `CARGO_BUILD_TARGET=aarch64-unknown-linux-gnu`, a stub `cargo` for test and build, and the
   real `cargo tree`.
   - The baseline passes: "25 product crates, 0 expected failures".
   - These plants go red:
     - the `capi` closure comes back empty;
     - `product_crates` returns nothing, with its guard bypassed (real cargo refuses `-p ""` with
       "package name cannot be empty");
     - the `debug` key is missing from `TEST_ROWS`;
     - the `rows` subcommand raises.
   - A stray blank line means no rows, which is correct.
   - The release rows are still read and judged.
5. **The re-pins are honest.** I hashed the merged tree the pre-#1065 way: the all-wet kernel
   words canonical, everything else raw, payloads raw. It gives `162979dd…`, `9316456b…`,
   `d4a1dc9d…` and `f442a0d3…`, and every other `bank.rs` pin passes raw. Only NaN words moved.
6. **Gates on the merge.**
   - `cargo check` and `clippy -D warnings` (`--workspace --all-targets --all-features`) and
     `fmt --check`: ok.
   - Dev and release tests of `dsp-reference`, `compressor`, `parametric-eq`,
     `true-peak-limiter`, `lane` and `math` with the CI features: 392 pass in each profile. EQ
     `--test bank` without features: 11 pass.
   - Release `audit`, `bench` and `console-workload`, which include the console digests: 107 pass.
   - `run-wasm-gates.sh` (native, simd128 and the V8 spill gate): ok.
   - x86 `--no-run` and `--list` of the debug leg: 25 product crates and 1,756 tests, with both
     former rows present. The release leg lists 114. `judge-skips` passes for both modes, and so
     do `rows` and `--self-test`.
   - 52 policy runs: all ok. They cover every `check-`/`test-` pair for the policies,
     `unfused-seal` with its `--self-test`, and the Python checks run under `python3 -B`. Three
     further scripts need an argument, which a bare run does not give.

### Findings, by severity

1. **LOW: residual risk on real AArch64.** Real hardware has not shown these three things:
   - the compressor's folded `bd3d711f…`;
   - the EQ `bank` and `bank-mono` legs at `Simd4`, which x86 cannot bind;
   - the removal of both rows.

   #1017's qemu evidence and hardware's raw scalar EQ digest (the folded pin) support them. The
   batch's first arm64 CI run confirms them; if it fails, the fold sites are the three above.
   The evidence's "the EQ case is shown above without it" holds for the scalar leg only.
2. **LOW: the debug leg lost an incidental non-vacuity signal.** `judge-skips` no longer proves
   that named tests are in the listing. The guard that remains, `product_crates` (at least 20
   crates and six named), together with the fixed packages, suffices; a minimum test count would
   be optional hardening.
3. **LOW: the docs claim more than the code does.** `EFFECT_CONTRACT_V1` and `TARGET_MATRIX` say
   every class-A comparison folds. Strict same-CPU `to_bits` differentials and the poison/pad
   checks remain, which is stricter and correct. Wording such as "folds, or compares strictly"
   would be exact.
4. **LOW: `assert_state` also folds integer fields.** It folds `ramp.remaining` through
   `class_a::word`. A `u32` of `0x7F800001` or more would collapse, but these are sample counts,
   so there is no practical effect. `le_words` documents the same point; `assert_state` does not.
