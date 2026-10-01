# Remove the materialized target-array copy from the hoist benchmark subject

## Finding and bounded product slice

The complete bench housekeeping audit #1145 found an actual per-lane target-array copy in HoistArm::render. In the unchanged source at tools/bench/src/console.rs, targets selects one of two precomputed immutable target tables, then `let (prepared, count) = targets[index * self.lanes + lane]` copies the twelve-target tuple before iterating the valid prefix. The pinned AVX2/FMA shipping-profile generated assembly from checkpoint `753fb5c7` has a 936-byte frame and memcpy length 672 at that tuple load, followed by the count load. Root independently inspected the source and worker B's /tmp/engine-housekeeping-b-1145/hoist-render.s.txt. No timed invocation, cycle estimate, projected speedup or production-engine copy claim is made.

Smallest closable slice: borrow the selected tuple and its valid prepared-target prefix while retaining the exact same bank/lane/application order. Confirm that synchronous target application cannot retain or mutate that input. This is a bounded tooling/optimization successor to the fixed untimed result-ownership slice #1145; it does not reopen a DSP algorithm or pursue a descriptive timing number.

## Frozen contract and scope

Only the bench target-selection/copy statements and their accurate comment, plus this spec. Existing workload construction, two target tables, active counts, parity selection, target words, bank/lane order, application/process calls, resource/audit boundaries, warmup/observation schedule, output/digest/record fields and class-A floors remain unchanged. No manifest/lock/policy edits, new benchmark runner, test framework, permanent generated-code/digest pin, SIMD algorithm, API or other crate change. Four unused direct dependency edges reported by #1145 are a separate configuration question and are outside this slice.

## Gates and test value

- Review immutable tuple ownership and target-application lifetime semantics; no reference escapes or crosses a target mutation.
- Locked existing bench tests and strict package lint/format/policy gates; existing untimed preflight/console fixture as proportional. No new test for this local borrowing substitution.
- One-time current shipping-profile code inspection must show that the identified full-array memcpy is gone; retain the before/after excerpts as review evidence, not a committed source-grep test. Do not claim every remaining copy gone.
- Do not launch a descriptive benchmark or timing optimization loop. Existing short record qualification tests may execute their timers without interpreting/quoting the values. If later timing is desired, it belongs to the frozen weekly workload/validator and one invocation with one warmup/two measured rounds.
- First focused-green exact-path product checkpoint pauses for root commit/push before evidence layering. Root adversarial review against this brief gets one verdict per coherent attempt, maximum two attempts; preserve any failed evidence. PASS and evidence upstream precede GitHub closure and required qualification precedes merge.

## Sol brief approval

Approved by root Sol, 2026-10-01 within the user's practical copy-efficiency request. Implementation starts only after #1145 is remotely synchronized. Two requested GPT-6.1 Sol xhigh workers remain the team; worker B implements and root reviews. No owner decision, measured gain or implementation is claimed yet.

## Attempt 1 evidence

Worker B (GPT-6.1 Sol xhigh) implemented the sole target-selection substitution: `let (prepared, count) = &targets[index * self.lanes + lane]`, then iterates `&prepared[..*count]`. The existing comment now accurately describes the borrowed prefix. Root checkpointed the only product path, `tools/bench/src/console.rs`, as `1085e80d`, integrated/pushed through `77828ae5`; no further product edits followed. Rust LoC is unchanged (four lines added/four removed, including the comment).

Ownership/order proof: both owned target tables are prepared before observations and remain immutable during `render`. The chosen parity table, index, prefix count, outer bank order, inner lane order and per-prefix call order are identical. `PreparedEffectTarget` contains only scalar/word fields, with no interior mutability. The launch EQ bank's synchronous `apply_prepared_target_lane` delegates to `apply_target_lane`; `control::decode_prepared_target` returns owned section/channel/BandTarget/EqSvfWords values, which are then copied into existing ramp state. Neither the shared target reference nor any input word is retained or mutated. Counts, target construction, processing, snapshots, schedules, record fields and floors are untouched.

One-time shipping-code inspection (not a permanent generated-code test):

- Command: `cargo rustc --locked -p bench --release --bin bench -- --emit=asm`, using the dedicated target directory and unchanged AVX2/FMA configuration. Toolchain: rustc 1.97.1 / LLVM 22.1.6. Build log and compiler identity: `/tmp/engine-housekeeping-b-1171/shipping-codegen-build.log`, `rustc-version.log`.
- Exact symbol: `_RNvMs2_NtCslkMvpcQZPNF_5bench7consoleNtB5_8HoistArm6render`. Current complete function is at `/home/bl/misofm/engine/target/housekeeping-b/release/deps/bench-330bd57c19ab82fe.s:8737`.
- Before excerpt: `/tmp/engine-housekeeping-b-1171/hoist-render-before.s.txt`, preserved from #1145 checkpoint `753fb5c7`; lines 185–191 contain source-line-951 materialization, `movl $672, %edx` followed by `callq *memcpy@GOTPCREL(%rip)` and the count read.
- After excerpt: `/tmp/engine-housekeeping-b-1171/hoist-render-after.s.txt`; lines 179–215 calculate the same 680-byte tuple stride, read the count at offset 672, retain the original table address and pass its target pointer directly to the bank application vtable call, advancing by the 56-byte target size. The identified full-array memcpy is gone. The emitted stack reservation is 600 bytes versus the saved 936 bytes; neither fact is a timing/cost estimate or speed claim. Required word transfers into EQ state and input restoration copies remain.

Proportional existing gates, all PASS; artifacts are in `/tmp/engine-housekeeping-b-1171/` and builds used `CARGO_TARGET_DIR=/home/bl/misofm/engine/target/housekeeping-b`:

- `cargo test --locked -p bench`: **13 passed**, `debug-tests.log`; `cargo clippy --locked -p bench --all-targets --all-features -- -D warnings`: `clippy.log`; `cargo fmt --all --check`: `fmt.log`; `git diff --check`: PASS.
- `check-bench-policy.sh`, `check-workspace-policy.sh`, `check-realtime-policy.sh`, `check-console-benchmark-fixture.sh`: `bench-policy.log`, `workspace-policy.log`, `realtime-policy.log`, `console-fixture.log`.
- `test-console-benchmark.sh`: `console-validator-mutations.log`, with **real runner/workload/timing invocations 0/0/0**. Fresh shipping binary `bench console --preflight`: `console-preflight.log`, all eight current controls resolved and current collapse/equality/nonvacuity claims passed. Logs contain no compiler warnings/errors.

No test was added, rewritten or deleted. The eight floor families still qualify independent Rust/jq numeric tables, divisors, inventories and control isolation; the five console families still qualify actual feed/meter/automation/strip records, malformed-record refusal and warmed allocation/RT counters, as fully reviewed in #1145. Existing automation owners exercise synchronous prepared-target application. No ordinary unit or this untimed preflight calls `HoistArm::render` directly: the local borrow equivalence, callee ownership proof and required shipping-code inspection qualify this exact substitution without claiming an executed Hoist PCM sweep. Its existing quiet/restated/moving assertions and generators remain unchanged. Short record tests internally use timers only for qualification; no duration is interpreted or quoted.

No descriptive workload, new test/harness, benchmark percentage, dependency change or additional target matrix was run. This native-only tooling substitution changes no DSP arithmetic, ABI or platform kernel; #1145's target evidence remains applicable. Completion is paused for root's single attempt-1 verdict, remote synchronization and required delivery qualification.

## Root adversarial verdict: PASS — attempt 1

Root Sol inspected the complete four-line source substitution/comment, both shipping assembly excerpts, immutable table/index/count ownership and the actual EQ bank delegation/owned decode/ramp-state path. The selected prefix contains the same words, is read synchronously and is never retained or mutated; parity, bounds refusal and every bank/lane/call order remain unchanged. Borrowing removes only the temporary whole-array materialization. The before assembly copies 672 bytes at the tuple load; the after assembly addresses that tuple directly and passes each 56-byte element to the same vtable call. This directly satisfies the frozen copy claim without inferring elapsed cost or a production-engine gain.

Root read actual 13-test, strict lint and untimed fixture/validator/preflight evidence. No existing test or this preflight directly executes HoistArm::render; the local borrowing equivalence, concrete callee lifetime proof and mandatory generated-code evidence are stated honestly. No new/revised test or committed artifact pin is needed. All workload, schedule, record/floor, algorithm, dependency and target boundaries remain intact. The identified copy is removed; necessary transfers and restoration copies remain.
