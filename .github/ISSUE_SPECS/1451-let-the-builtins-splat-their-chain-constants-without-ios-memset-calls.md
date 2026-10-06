# Let the builtins splat their chain constants without iOS memset calls

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`).
Filed 2026-10-06 by root order after the #1328 follow-up stopped at the owner's 2 % allowance.
Code anchors verified on `codex/d15-stream-g` at `4264bb190` (the #1328 follow-up checkpoint).
Ordered directly after *Flush the SVF jointly so builtin and EQ filters reach exact rest* (#1328).

## Product outcome

The browser pays no cost for the iOS build's encoding. Today the builtin input chain loads its
vector constants from its prepared coefficients instead of splatting them, only because a splatted
`f32x4` constant becomes a `memset_pattern16` libc call on `aarch64-apple-ios` (known defect
#1018). That encoding costs the shipped AudioWorklet module about 3.5 % to 3.9 % p50 on the app
shape. After this slice the cause of the libc calls is found and removed, the chain constants are
splatted again, the browser documents are within 2 % of the per-word build, and the iOS release
assembly has no more `memset_pattern16` calls than today.

## Context

- **The cost (the #1328 follow-up, commit `4264bb190`).** With the EQ's unarmed form and arming
  only on lanes that hold state, the app-shape browser document is +3.5 % / +3.9 % p50 (two rounds)
  against **P0**, the per-word build (the per-word flush law, no joint rule) that the #1328
  follow-up measured against. The owner's allowance is 2 %
  (`two-percent-allowance-for-better-code`). The V8 (TurboFan) builtins dual loop is 218
  instructions against P0's 196 for the same arithmetic: the carried words occupy registers or are
  reloaded where P0 had `v128.const` immediates.
- **Why the constants are carried (#1328 attempt 5, A9; spec "Two forms per builtin chain body").**
  Each builtin input-chain body runs in two forms (armed and unarmed). The first cut doubled the
  bodies' splatted constants, and `check-cross-targets.sh` refused it: `memset_pattern16` calls rose
  from 186 to 397 in `builtins` and from 566 to 1128 in `multiband-compressor`. A run-time flag in
  place of a const generic changed nothing (LLVM unswitches the loop: 397). Carrying the four
  constants as words brought `builtins` to 71. The splatted two-form chain measured about +1.6 % to
  +4.5 % against A8 on the same host (attempt 5 record), the carried form more.
- **Code anchors (carried-word constants added for the ratchet).**
  - `crates/lane/src/kernels/builtins.rs`: `InputChainCoef::constants` (`:530-531`),
    `InputChainConstants` with `limit`, `one`, `flush_eps`, `rest_eps` and its doc naming #1018
    (`:534-573`); the bodies load them once per block (`:674-679`, `:834-839`, `:946-951`,
    `:1092-1097`, `:1344-1349`, `:1918-1923`, `:2031-2036`) and pass `flush_eps` to
    `svf_step_when` and `one`/`rest_on` to `silence_step_hoisted`; test constructors at `:2325`,
    `:2368`.
  - `crates/builtins/src/lib.rs:1071`: `InputChainConstants::new()` in the bank's prepared
    coefficients (stream A's file).
  - `crates/lane/src/kernels.rs`: `RestThresholds::flush_eps` (`:60-66`), `ArmedRest::flush_eps`
    (`:80-110`), `UnarmedRest::flush_eps` (`:111-135`), and `svf_step_when`'s `flush_eps`
    parameter with its #1018 doc (`:1043-1071`).
  - `crates/lane/src/lib.rs`: `flush_with` (`:245`), `flush_pair_with` (`:374-376`) and
    `silence_step_hoisted` (`:276-285`, doc naming #1018), the `_with`/hoisted forms of `flush`,
    `flush_pair` and `silence_step`.
  - `crates/parametric-eq/src/lib.rs`: the prepared EQ's `flush_eps: L` word (`:2979-2985`, doc
    naming #1018), read at `:3140`, `:3158`, `:3176`, `:3239-3247`, set at `:3483`.
- **Other shapes chosen for the ratchet (not carried constants; listed for D3's inventory).** The
  #1407 filter-ramp leading countdown carried as a per-lane word (`INPUT_FILTER_LEADING_UPDATES`,
  `crates/lane/src/kernels/builtins.rs:1266-1274`); the route ramp's frame index built from a `u32`
  counter (#1220, `crates/lane/src/kernels.rs:1700-1703`, commit `8c6268967`); the EQ's
  `lanes_mask` from one flag vector (#1089, `crates/parametric-eq/src/lib.rs:1066-1071`, commit
  `858ccb848`); the limiter's out-of-line `clear_runtime` (#1091,
  `crates/true-peak-limiter/src/lib.rs:626-636`, commit `cd1f02bb2`); soft clip's loop-invariant
  branch in place of two body copies (#1409 D5, `crates/soft-clip/src/kernel.rs:186-189`); and the
  multiband compressor's single body with the counter and joint flush on every frame (#1328 A9).
- **The ratchet.** `scripts/check-cross-targets.sh` (`:100-139`) emits each product crate's
  `aarch64-apple-ios` release assembly as an rlib (`cargo rustc --release --target
  aarch64-apple-ios --crate-type rlib --emit asm`; no Xcode, no link) and counts
  `^\tbl\t_memset_pattern16$`. `scripts/lib/aarch64-known-defects.py` `IOS_MEMSET_CEILINGS` judges
  the counts: today `builtins` 71, `compressor` 970, `gate-expander` 91, `graph` 10, `host-core` 4,
  `multiband-compressor` 566, `parametric-eq` 48, `soft-clip` 22, `transient-shaper` 268,
  `true-peak-limiter` 104, all owned by #1018. A count above its row fails; a count below passes and
  asks for the row to be lowered; a crate at zero with a row fails until the row is deleted. The
  check runs locally: the `aarch64-apple-ios` target is installed on the development host, and
  earlier slices (#1328, #1408) ran it there. The AArch64 test legs (`scripts/run-aarch64-tests.sh`)
  and native AArch64 timing run only in CI.
- **#1018 (open; spec `.github/ISSUE_SPECS/1018-remove-the-libc-memset-calls-from-the-eq-s-svf-flush-on-apple-targets.md`).**
  Filed for the EQ's `L::splat(FLUSH_EPS)` in `svf_step`; #1017's scan widened it to the shape "a
  stored `f32x4` splat constant" in ten crates (3,494 calls at filing) and left root to rule whether
  to widen #1018 or split. Root ruled on 2026-10-06: widen, executed by this slice (Amendment 1
  A2). Its gates ask
  for zero libc calls in render, class-A bits, no worklet regression and removal of the
  `docs/TARGET_MATRIX.md` register entry (`:178`). The ceilings have since fallen through the work
  in the commits that name #1018 (#1112 removed eight-lane AArch64 code; #1089, #1091, #1220,
  #1328 each avoided or carried a constant). No commit has diagnosed why LLVM emits the call.
- **A lead for D1 (not verified).** `wide` 1.6.1 builds every `splat` as
  `transmute::<[T; N], Simd>([elem; N])` (`wide-1.6.1/src/simd.rs:326-328`), and `lane`'s
  `Lane::splat` forwards to it (`crates/lane/src/wide_impl.rs:139-141`). An array repeat
  expression may reach LLVM as a short store loop; LLVM's loop-idiom pass turns a loop that stores
  a loop-invariant 16-byte pattern into `memset_pattern16` on Darwin, the only targets that provide
  that routine, which fits the call appearing on iOS and not on Android. If so, the call comes from
  how a splat is written, not from splat constants as such, and building the vector from an array
  literal (`[x, x, x, x]`) or from the backend's broadcast could remove every call at once. D1
  confirms or refutes this.

## Decisions frozen for this slice

- **D0. Root decision (2026-10-06).** The decision-15 root coordinator, under the owner's
  no-shortcuts delegation (`no-shortcuts-correctness-first`), accepts the #1328 follow-up's browser
  cost now and orders this scheduled fix, not an exception to the 2 % allowance. Reason: the extra
  cost is an encoding artifact with a known fix (splat the constants), not the price of the
  correctness change; the encoding exists only to keep the iOS ratchet of known defect #1018 from
  rising. Stream G files and owns it, ordered directly after #1328.
- **D1. Codegen evidence first.** Before any fix, show why a splatted `f32` constant in a lane
  kernel produces `bl _memset_pattern16` on `aarch64-apple-ios`. Take one small kernel that shows
  the call (for example the builtins two-form body with `InputChainConstants::new()` inlined in
  place of the carried words) and record: the Rust construction of the splat, the LLVM IR before
  and after optimisation (`--emit=llvm-ir`, and the pass that introduces the call, for example with
  `-C llvm-args=-print-after-all` or `-print-changed` narrowed to the function), and the final
  assembly around each call (is it in the frame loop or in the block prologue, and what does it
  fill). Compare `aarch64-linux-android` for the same function. Name the root cause in one
  sentence and record it in this spec's attempt record. If D1 finds that no single construction
  causes the calls, stop after D1 and return the evidence to root (the fix is then rescoped).
- **D2. Fix the root cause, one shape for all targets.** Change the construction D1 names so that a
  splatted constant produces no `memset_pattern16` call, at the narrowest point that covers every
  kernel (expected: `Lane::splat` in `crates/lane/src/wide_impl.rs`, and `Lane::zero` if it shares
  the form). No target-specific code (`no-target-specific-code`): the same Rust source on every
  target; no `cfg(target_os)`, no Apple-only path, no disabling of an LLVM pass, no
  `#[inline(never)]` wrappers. Class A: no rendered bit moves on any target (a splat is the same
  word in every form). The x86-64-v3 and `simd128` codegen of the hot kernels must not get worse
  (gate 4).
- **D3. Return the carried constants to splats.** With D2 in place, remove the words that exist
  only for the ratchet: `InputChainConstants` and `InputChainCoef::constants` (the bodies splat
  `NONFINITE_LIMIT`, `1.0`, `FLUSH_EPS` and `REST_EPS` again), the EQ's `flush_eps` word and the
  `flush_eps` fields of `ArmedRest` and `UnarmedRest` (and `RestThresholds::flush_eps` if it has
  no other reason to exist), and `flush_with`, `flush_pair_with`, `silence_step_hoisted` and
  `svf_step_when`'s `flush_eps` parameter where no caller needs them. Each removed item's #1018
  doc goes with it. The other ratchet-chosen shapes in Context (the #1407 countdown, the #1220
  counter, the #1089 mask, the #1091 out-of-line reset, the #1409 D5 branch) stay as they are:
  each is also a correct form on its own, none is in the measured cost, and changing them is not
  this slice. The multiband compressor's single body (#1328 A9) is not a carried constant, so D3
  does not apply to it; whether it should gain an unarmed form once splats are free is recorded
  for root in the attempt record, not done here.
- **D4. The ceilings only fall.** Run `check-cross-targets.sh` after D2 and after D3. Lower each
  row it reports below its ceiling, and delete each row that reaches zero, as the check asks. No
  row is raised. If every row reaches zero, also remove the `docs/TARGET_MATRIX.md` register entry
  and the `ios-asm-memset-pattern16` wording in `TARGET_MATRIX.md:10`, and record in this spec that
  #1018's gates 1 and 4 are met. If some rows remain, they stay #1018's, and the attempt record
  names the constructions D1 shows are left. Amendment 1 (A2) widens this: every row of every
  crate is re-measured, and #1018 closes when the rows reach zero.
- **D5. P0 is #1328's.** The base for the timing gate is the same P0 the #1328 follow-up measured
  (per-word flush law, no joint rule), built with the same recipe. If the follow-up's record does
  not name P0's tree exactly, the implementer asks root before timing; the base is not chosen after
  the measurement.

## DSP evidence (AGENTS.md)

- **Equations, coefficients, update rules:** none change. The words splatted and the words carried
  are the same constants (`NONFINITE_LIMIT`, `1.0`, `FLUSH_EPS = 1e-20`, `REST_EPS = 1e-14`), so
  every kernel computes the same values in the same order.
- **Numerical limits, latency, tail, units, smoothing:** unchanged (#1328, #1329).
- **Denormal/NaN:** unchanged; the flush and rest thresholds keep their values and comparisons.
- **Fixtures and objective tests:** gates 1-6. **Benchmarks:** gate 3, as the root's acceptance
  measurement. **Listening:** none; no bit moves.

## Deliverables

1. D1's codegen evidence and root-cause sentence in this spec.
2. The D2 fix, with a doc at the changed construction that states why it is written that way (the
   Darwin libc call it avoids).
3. D3's removals and the kernel docs updated to the splatted form.
4. `scripts/lib/aarch64-known-defects.py` rows lowered or deleted per D4 (and the comment above
   `IOS_MEMSET_CEILINGS` updated); `docs/TARGET_MATRIX.md` per D4.
5. The PR evidence: the base-versus-head differential (gate 1), the timing table (gate 3), the V8
   listing counts (gate 4) and the per-crate memset counts before and after (gate 2).

## Authorized paths

- `crates/lane/src/wide_impl.rs` (and `crates/lane/src/simd4.rs`, `simd8.rs`, `scalar.rs` only if
  D2's construction lives there)
- `crates/lane/src/lib.rs`, `crates/lane/src/kernels.rs`, `crates/lane/src/kernels/builtins.rs`
  (D3), and `crates/lane/tests/` only where a test constructs a removed item
- `crates/builtins/src/lib.rs:1071` (`InputChainCoef::constants`, the `InputChainConstants::new()`
  field of the bank's coefficients, only; stream A's file, named exception, Amendment 1 A1) and
  `crates/builtins/tests/` only where a test constructs a removed item
- `crates/parametric-eq/src/lib.rs` (the `flush_eps` word only) and its tests only where a test
  constructs a removed item
- `scripts/lib/aarch64-known-defects.py` (the ceilings and their comment)
- `docs/TARGET_MATRIX.md` (the #1018 register entry and row 10's wording, D4 only)
- `docs/rulings/effect-floor-accounting.md` and the floor pins (`tools/bench/src/floor.rs`,
  `scripts/console-benchmark-record-lib.jq`, `scripts/test-console-benchmark.sh`) only if D3 changes
  a lane-op count they record
- this spec

## Non-goals

- Any change to the #1328 law (joint flush, silence counter, arming), to rendered bits, to state
  records or sealed sizes.
- The other ratchet-chosen shapes listed in D3, and an unarmed form for the multiband compressor.
- Closing #1018 before its rows reach zero, or editing its GitHub issue body (the ruling is recorded
  on #1018 by root; Amendment 1 A2).
- Chasing a ceiling that the D2 fix does not lower: what is left is recorded, not fixed here
  (Amendment 1 A2).
- Undoing the other ratchet-chosen shapes: #1452 owns that (Amendment 1 A3).
- Native AArch64 timing (CI-only, no funded runner).
- Rejected alternatives:
  - An exception to the 2 % allowance for the carried form: root ruled a scheduled fix instead (D0).
  - Raise the iOS ceilings to admit the splatted form: the ratchet exists so that the realtime
    rule's breach on iPhone can only shrink.
  - Apple-only splat code, or turning off LLVM's loop-idiom pass for Apple targets: target-specific
    machinery for a defect whose cause is in the shared construction, and a pass flag would affect
    every loop in the build.
  - Keep carrying the words and tune the V8 loop around them: tunes the encoding artifact instead
    of removing it.

## Hazards

- `wide` is a dependency; a fix that changes how `lane` builds vectors must keep `wide`'s types and
  must not patch or fork `wide`. If D1 shows the cause is inside a `wide` method `lane` cannot
  avoid, stop and return to root.
- Splatting may change register allocation in other kernels on x86 and `simd128`. Gate 4 compares
  the codegen; a regression in another hot loop is reported, not tuned away.
- `check-lane-policy.sh` refuses some spellings inside `crates/lane`; use the crate's accepted
  forms.
- Hot files: `crates/lane/src/kernels/builtins.rs` and `crates/parametric-eq/src/lib.rs` are shared
  with streams A, B and E (STREAMS.md hot-file rows). This slice lands after #1328 and touches only
  the constant plumbing.
- Another implementer may be working in the same worktree; commit exact paths only.

## Objective gates

1. **No rendered bit moves.** A base-versus-head differential (base: the tree before this slice,
   head: after D3), over the same scenario set the #1328 follow-up used (724 scenarios) or a
   superset: every output identical. PR evidence, not a committed test. Every pinned artifact is
   unchanged: `g5_native_digests_match_pins` (G5 `lane_digests.in`), `BUILTINS_DIGESTS`,
   `E9_DIGESTS`, multiband `DIGESTS`, `conformance_fixtures --check`,
   `check-builtins-fixtures.sh`, `check-graph-determinism.sh`. On AArch64 the CI legs
   (`aarch64-debug`, `aarch64-release`) pass with the same expected-failure rows as before.
2. **iOS memset ceilings only fall.** `bash scripts/check-cross-targets.sh` passes with every row
   at or below today's ceiling, and with each row lowered or deleted as the check asks (D4). Every
   row of `IOS_MEMSET_CEILINGS` (all ten crates, not only `builtins` and `parametric-eq`) is
   re-measured after D2 and after D3, and no row is raised (Amendment 1 A2). The `builtins` and
   `parametric-eq` counts after D3 are at or below their counts before D3 (today 71 and 48). The
   before and after count of every crate is in the PR.
3. **Browser cost within 2 % of P0.** `scripts/web-mixing-automation-benchmark.mjs run` on the
   shipped module of head and of P0 (D5): one warmup and two measured rounds each, interleaved,
   pinned to one CPU, host load average recorded before and after; not retried. p50 per render of
   each of the four browser documents -- the mono console (its quiet, restated and automated arms),
   the sixty-four-track console, the app shape and the bus-and-send console -- is at most +2 % of
   P0 in both rounds. Every output digest is equal between the two modules. A miss is reported with
   the table and the V8 listings and goes to root; it is not tuned and re-run.
4. **V8 and native codegen.** `bash scripts/run-wasm-gates.sh` exits 0 (native, `simd128`, V8 spill
   gate with every held row clean). The PR records the TurboFan instruction count of the builtins
   dual loop (head, against P0's 196 and the carried form's 218) and of each held spill row; and,
   for D2, the x86-64-v3 and `simd128` instruction counts of the builtins and EQ frame loops before
   and after (no increase, or each increase named).
5. **Workspace gates.**
   - `cargo test --locked --all-targets -p lane -p math -p effect-runtime -p delay -p compressor -p multiband-compressor -p gate-expander -p true-peak-limiter -p transient-shaper -p soft-clip -p parametric-eq -p builtins -p dsp-reference -p conformance --features math/lane,parametric-eq/test-support,builtins/test-support,lane/test-support`
   - `cargo test --locked --release -p lane -p math -p wasm-gates --features math/lane`
   - the `test-debug-a` workspace command from `.github/workflows/qualification.yml`
   - `cargo run --locked -p conformance --example conformance_fixtures -- --check`
   - `cargo build --locked --release -p audit && bash scripts/check-builtins-fixtures.sh . target/release/audit`
   - the worklet chain, as in `qualification.yml`:
     `bash scripts/build-web-audioworklet.sh --named-twin target/ci/qualification-named-twin target/ci/qualification-artifacts`,
     `bash scripts/check-web-audioworklet.sh --without-metadata-regeneration target/ci/qualification-artifacts target/ci/qualification-named-twin/miso-engine-v1-audio-worklet.simd128.named.wasm`,
     `python3 -B scripts/check-browser-expected-resources.py --artifacts target/ci/qualification-artifacts`,
     `bash scripts/test-web-audioworklet.sh`
   - `bash scripts/check-lane-policy.sh`, `bash scripts/check-builtins-policy.sh`,
     `bash scripts/check-graph-determinism.sh`, `bash scripts/check-workspace-policy.sh`,
     `python3 -B scripts/lib/aarch64-known-defects.py --self-test`
   - `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`
6. **Resources.** `fixtures/builtins/v1/resources.jsonl` and the graph resource fixtures either are
   unchanged or shrink only by the removed `InputChainConstants` words (four words per input
   stage), each re-pin named in its commit with that reason.

## Test value

This slice adds no new test. Its claims are held by existing gates: the iOS ratchet
(`check-cross-targets.sh` with `aarch64-known-defects.py`) turns red if a splat brings a libc call
back or a ceiling is raised; the pinned digests and the differential turn red if a constant's value
or an operation's order moves; the V8 spill gate turns red if a held loop spills. Any test that only
constructed a removed item is updated, not added.

## Dependencies

#1328

## Amendment 1 (2026-10-06, root decision)

The decision-15 root coordinator made three decisions on 2026-10-06. They govern where the
sections above seem to disagree.

- **A1. Named exception and hot file.** `crates/builtins/src/lib.rs:1071`
  (`InputChainCoef::constants`, the `InputChainConstants::new()` field in the bank's prepared
  coefficients) is stream A's file. This slice edits it by named exception, for D3 only (the field
  and its construction go). Stream G's Owns line does not change. STREAMS.md's hot-file table
  orders the edit: G #1451 before stream A's slices that edit `crates/builtins/src/lib.rs`
  (#1277, #1327), which rebase onto it, as G #1407 and #1408 land before A in the other builtins
  rows.
- **A2. This slice takes #1018's open ruling: widen.** Root rules that #1018 is widened to the
  stored-splat shape in every crate (not split), and that #1451 executes it.
  - If D1 confirms that the cause is the splat lowering (for example `wide`'s
    `transmute([elem; N])` array repeat, reached through `Lane::splat` in
    `crates/lane/src/wide_impl.rs`), the one-shape fix lives in the `lane` crate and applies to
    every crate that builds a splat through it. No crate gets its own fix.
  - Every memset ceiling in `scripts/lib/aarch64-known-defects.py` may only go down, and each one
    is re-measured after D2 and after D3 (gate 2).
  - #1018 closes when every ceiling reaches 0 (the rows deleted, D4's register removal done, and
    #1018's gates 1 and 4 met). If some do not reach 0, the attempt record names each remaining
    crate, its count, the construction D1's method shows makes the calls, and why the D2 fix does
    not reach it; those rows stay #1018's and #1018 stays open.
  - The slice stays to the cause, the fix and the builtins constants (half a day). It does not
    chase a remaining row with crate-local changes.
- **A3. Follow-up filed.** *Undo the iOS memset ratchet workarounds once splats are free* (#1452,
  filed the same day, Stream G, directly after this slice) undoes the five other ratchet-chosen shapes
  that D3 leaves (#1407, #1220, #1089, #1091, #1409 D5), each gated bit-identical. This slice does
  not touch them.

## Attempt record

None yet.
