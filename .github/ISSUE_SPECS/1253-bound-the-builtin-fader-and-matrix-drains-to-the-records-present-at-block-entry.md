# Bound the builtin fader and matrix drains to the records present at block entry

Core slice 1 of the umbrella *Deliver value-only fader, mute and pan transactions to the running
C ABI plan through the live console lanes* (#1053), its decision D11. Anchors verified on `main`
at `54b0a1bf8`.

## Product outcome

The render thread's fader/mute drain and matrix/pan drain do a bounded amount of work per block,
even while a control thread pushes records at the same time.

Today both drains loop `while let Ok(record) = control.try_pop()`. With a concurrent producer, the
loop keeps popping records that arrive while it runs: data-dependent, unbounded work on the render
thread, which `AGENTS.md` forbids. The C ABI becomes such a producer in #1257, because it prepares
with `Concurrent` delivery. The browser pushes only between render calls, so its behaviour does not
change.

## Context (verified at `54b0a1bf8`)

All paths are in `crates/builtins-compiler/src/lib.rs` unless named.

- **The two production drains.**
  - `drain_fader_controls` (`:970-1004`) and `drain_matrix_controls` (`:1006-1026`).
  - Their callers are `FaderBankProcessor::process` (`:645-660`), `MatrixBankProcessor::process`
    (`:704-717`, the call at `:713`) and the fused `FaderMatrixBankProcessor::process`
    (`:1073-1094`).
- **The two test-only scalar oracle drains.** `LiveControlMatrixProcessor::drain_controls`
  (`:4200`) and `LiveControlFaderProcessor::drain_controls` (`:4266`). Both are under
  `#[cfg(any(test, feature = "test-support"))]`, and both loop the same way.
- **The bounded shape to copy.** `BuiltinBankProcessor::begin_block` (`:469-472`):
  `let available = control.available_at_entry(); for _ in 0..available { let Ok(record) =
  control.try_pop() else { break }; ... }`. The effect lane does the same
  (`crates/effect-contract/src/live.rs:351-365`), and so does the route drain
  (`crates/graph/src/runtime.rs:897-900`).
- **`Consumer::available_at_entry`** (`crates/engine/src/realtime/spsc.rs:420`) is one `Acquire`
  load. A record pushed after it waits for the next block.
- **The realtime policy gate** (`scripts/check-realtime-policy.sh`).
  - It extracts every `REALTIME_POLICY_BEGIN`/`REALTIME_POLICY_END` region under `crates`, `hosts`
    and `tools` (`:45-65`) and applies one forbidden-body predicate to them (`:75-76`).
  - Its floors are 12 files and 41 regions (`:72-73`). `main` has 15 files and 54 regions: the
    floors were not raised as markers were added.
  - Its self-test, `scripts/test-realtime-policy.sh`, builds a synthetic tree of exactly 12 files
    and 41 regions (`create_fixture`, `:10-27`). Its floor-mutation cases match the literal
    messages "twelve" and "forty-one" (`:396-402`).
  - `crates/builtins-compiler/src/lib.rs` has no marked region today.
  - No marked region contains a `while let ... try_pop` loop today. The only `try_pop` calls in
    marked bodies are the bounded drains above and an `if let` in
    `crates/engine/src/realtime/plan_exchange.rs:360`.

## Decisions

- **D1.** Each of the four drains reads `available_at_entry()` once and pops at most that many
  records, in queue order, with the shape of `begin_block` above. Every record present at block
  entry is still applied at that block, so no record semantics change.
- **D2.** The two production drains become marked realtime regions (`// REALTIME_POLICY_BEGIN` and
  `// REALTIME_POLICY_END` around each function). The floors rise by exactly what this slice adds,
  one file and two regions: to 13 files and 43 regions.
  - `check-realtime-policy.sh`: the two floor checks and their messages ("thirteen",
    "forty-three"), and the comment above them.
  - `test-realtime-policy.sh`: `create_fixture` gains one fixture file with two marked regions, so
    its synthetic tree meets the new floors, and the three floor-mutation cases expect the new
    messages.
- **D3.** `check-realtime-policy.sh` gains one rule over the extracted marked bodies. It refuses a
  `while let Ok(...) = ....try_pop()` loop, with a message that names the bounded form
  (`available_at_entry`). `scripts/test-realtime-policy.sh` gains one mutation case: it inserts
  such a loop into a marked region of a scratch copy and expects the gate to fail with that
  message.

## Authorized paths

- `crates/builtins-compiler/src/lib.rs`: the four drains and the two pairs of markers only.
- `scripts/check-realtime-policy.sh`, `scripts/test-realtime-policy.sh`.
- This spec.

## Non-goals

- No change to the record types, the queues, the delivery modes, host-core, host-web or capi.
- No new Rust test. The bound is structural: a test on one thread cannot push while the drain runs.
  The policy rule and its mutation are the guard.

## Hazards

- The forbidden-body predicate (`:75-76`) also applies to the new regions. It forbids `.expect(`,
  `.unwrap(`, `drop(`, `Vec::` and similar. The drains use `map_err(render_error)?` and
  `#[cfg(any(test, feature = "test-support"))]` witness counters. Keep them inside the predicate.
- The fused processor drains the fader first and then the matrix. Keep that order.

## Objective gates

Run every command from the repository root.

1. **Behaviour is unchanged.**
   - `cargo test --locked -p builtins-compiler --features test-support`
   - The workspace test command (it runs the host-web and host-core live-control tests):
     `cargo test --locked --workspace --all-targets --exclude lane --exclude math --exclude effect-runtime --exclude delay --exclude compressor --exclude multiband-compressor --exclude gate-expander --exclude true-peak-limiter --exclude transient-shaper --exclude soft-clip --exclude parametric-eq --exclude builtins --exclude dsp-reference --exclude conformance --exclude audit --exclude bench --exclude console-workload --exclude wasm-gates --exclude wasm-gate-guest --exclude wasm-gate-corpus --features builtins-compiler/test-support,graph/test-support,host-web/test-support,host-core/test-support,effect-compiler/test-support,protocol/test-support,engine/realtime-audit`
   - `cargo test --locked -p builtins-compiler --no-run`
2. **The policy rule.**
   - `bash scripts/check-realtime-policy.sh && bash scripts/test-realtime-policy.sh`
   - The new mutation fails the mutated copy and passes the tree.
   - *Test value: it turns red if an unbounded `while let Ok(..) = ..try_pop()` drain enters a
     marked realtime region.*
3. **Static rendering is unchanged.** `cargo test --locked --release -p audit -p bench -p console-workload`
   (the `gain_pan_profile` digests).
4. **The realtime audits.**
   - `cargo build --locked --release -p audit -p bench -p capi -p session-validator`
   - `bash scripts/trace-builtins-audit.sh target/release/audit`
   - `bash scripts/trace-builtins-graph-audit.sh target/release/audit`
   - `./target/release/audit capi` reports zero allocations, frees, locks and syscalls.
5. **The shipped AudioWorklet module.**
   - Build it on this branch and on `54b0a1bf8`, and report both SHA-256 digests (expected:
     changed, because the drain code moved):
     `rm -rf target/ci/qualification-artifacts target/ci/qualification-named-twin && mkdir -p target/ci/qualification-artifacts target/ci/qualification-named-twin && bash scripts/build-web-audioworklet.sh --named-twin target/ci/qualification-named-twin target/ci/qualification-artifacts && sha256sum target/ci/qualification-artifacts/miso-engine-v1-audio-worklet.simd128.wasm`
   - `bash scripts/check-web-audioworklet.sh --without-metadata-regeneration target/ci/qualification-artifacts target/ci/qualification-named-twin/miso-engine-v1-audio-worklet.simd128.named.wasm`
   - `python3 -B scripts/check-browser-expected-resources.py --artifacts target/ci/qualification-artifacts`
   - `bash scripts/test-web-audioworklet.sh`
6. **Workspace and policy.**
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
   - `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`
   - `for x in host-core realtime workspace; do bash scripts/check-$x-policy.sh && bash scripts/test-$x-policy.sh || exit 1; done`
   - `bash scripts/check-cross-targets.sh`
   - 4-lane (NEON): `bash scripts/run-aarch64-tests.sh debug` runs only on an arm64 host. On this
     x86-64 box it is CI-only (the `aarch64-debug` job), so record it as not run locally.

## Evidence

- The output of every gate command, from the head commit.
- The new mutation case's name and its one-sentence test value.
- The new marker counts and floors.
- The module digests before and after.

## Dependencies

None.

## Standing rules for the implementer

- Work from this body. Change nothing outside the authorized paths.
- A test that greps source or prose is refused. The policy script is a gate, not a test, and its
  mutation case is the test.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).

## Attempt record

### Attempt 1 (implementer, on `11b7c92e4`)

**Change.**
- `crates/builtins-compiler/src/lib.rs`: `drain_fader_controls`, `drain_matrix_controls` and the
  two test-only oracle drains (`LiveControlMatrixProcessor::drain_controls`,
  `LiveControlFaderProcessor::drain_controls`) read `available_at_entry()` once and pop at most
  that many records (`for _ in 0..available { let Ok(record) = ..try_pop() else { break }; .. }`),
  the shape of `BuiltinBankProcessor::begin_block` (D1). The fused processor's fader-then-matrix
  order is untouched. The two production drains are now marked realtime regions (D2); their
  bodies pass the forbidden-body predicate as written (no edits needed for it).
- `scripts/check-realtime-policy.sh`: floors 12/41 -> 13/43 ("thirteen", "forty-three"), comment
  updated (D2). New rule over the marked bodies (D3): `gate_scan_forbidden 'marked realtime
  unbounded try_pop drain (bound it with available_at_entry)'` with pattern
  `while[[:space:]]+let[[:space:]]+Ok[[:space:]]*\(.*=.*\.try_pop[[:space:]]*\(`.
- `scripts/test-realtime-policy.sh`: `create_fixture` gains
  `crates/builtins-compiler/src/lib.rs` with two marked regions (fixture now 13 files, 43
  regions; also added to the `empty_bodies` list), the three floor cases expect the new messages,
  and one new mutation case, `marked-unbounded-try-pop-drain`.

**Marker counts.** Tree: 56 regions in 16 files (was 54 in 15). Floors: 13 files, 43 regions.

**Test value (`marked-unbounded-try-pop-drain`).** It turns red if an unbounded
`while let Ok(..) = ..try_pop()` drain enters a marked realtime region.

**Mutation runs.**
- A. Reverted the production `drain_matrix_controls` to `while let Ok(record) = control.try_pop()`:
  `check-realtime-policy.sh` red with `realtime policy failure: marked realtime unbounded try_pop
  drain (bound it with available_at_entry)` at `lib.rs:1025`. Reverted; gate green.
- B. Removed the new rule from `check-realtime-policy.sh`: `test-realtime-policy.sh` red with
  `realtime policy mutation unexpectedly passed: marked-unbounded-try-pop-drain`. Reverted; green.
- C. Left the file floor at 12: `test-realtime-policy.sh` red (`marked-file-count-floor` no longer
  fails with the file-floor class). Reverted; green.

**Gates (all from the worktree root, on the attempt's tree).**
1. `cargo test --locked -p builtins-compiler --features test-support` PASS (52 + 10 + 3 passed,
   1 ignored); workspace test command PASS (115 `test result: ok` lines, no failure);
   `cargo test --locked -p builtins-compiler --no-run` PASS.
2. `bash scripts/check-realtime-policy.sh && bash scripts/test-realtime-policy.sh` PASS
   (`realtime policy: ok (56 marked regions in 16 files)`, `realtime policy mutation tests: ok`).
3. `cargo test --locked --release -p audit -p bench -p console-workload` PASS.
4. `cargo build --locked --release -p audit -p bench -p capi -p session-validator` PASS;
   `trace-builtins-audit.sh` PASS; `trace-builtins-graph-audit.sh` PASS;
   `./target/release/audit capi`: allocations 0, deallocations 0, locks 0, syscalls 0,
   total_violations 0.
5. AudioWorklet module: `54b0a1bf8` `30d075d3ce6382f21235675996184c675753acf6d451e11d7d676a3d50aaeff4`
   (2849915 B); this branch `6eb292980c2f31a7a2b86ef8388a17b1f0b5efa1b70544ec1a955d1bd207188f`
   (2850019 B, +104 B) -- changed, as expected. `check-web-audioworklet.sh
   --without-metadata-regeneration` PASS; `check-browser-expected-resources.py --artifacts` PASS;
   `test-web-audioworklet.sh` PASS.
6. `cargo fmt --all -- --check` PASS; `cargo clippy --locked --workspace --all-targets
   --all-features -- -D warnings` PASS; `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace
   --no-deps` PASS; host-core/realtime/workspace check+test policy PASS;
   `scripts/check-cross-targets.sh` PASS (iOS `memset_pattern16` expected failures unchanged:
   transient-shaper 268, true-peak-limiter 104); `scripts/check-capi-abi.sh` PASS.
   `run-aarch64-tests.sh debug`: not run locally (x86-64 host; CI `aarch64-debug` job).

**Outside authorized paths.** None.

### Follow-ups applied (after the attempt 1 PASS)

- NIT 2: the host-web mutation prose (`hosts/host-web/src/tests.rs`, the two `Red mutation` docs,
  and `hosts/host-web/MUTATIONS.md`) now names "the `drain_controls` call" instead of the old
  `while let ... try_pop()` drain. Prose only; no test behaviour changed.
- NIT 1 (the realtime-policy `try_pop` rule is a single-line regex) is not implemented here; it
  stays a recorded follow-up.
- Gates on the follow-up tree (`978463341`, x86-64-v3, one run for the whole #1253-#1257
  follow-up set): `cargo fmt --all -- --check`; `cargo test --locked` for `host-core
  --all-features`, `builtins`, `builtins-compiler`, `capi` and `host-web`; workspace clippy
  `--all-targets --all-features -D warnings`; `cargo doc` with `-D warnings`; release build and
  `./target/release/audit capi` (100,000 calls, allocations 0, syscalls 0, total_violations 0);
  `check-capi-abi.sh`; realtime, workspace and host-core policy (check, plus the realtime and
  workspace self-tests); `check-cross-targets.sh` (iOS `memset_pattern16` expected failures
  unchanged): all pass. Worklet chain not run: no line compiled into the browser module changed
  (host-core, host-web, builtins and builtins-compiler changed only in docs, comments and
  tests).

### Verdict

**Verdict.** Sol attempt 1: PASS (verdict file `docs/handoffs/live-updates-1053/1253-attempt1.md`). No MINOR. NIT 2 (host-web drain prose) applied in `2142efb98`; NIT 1 (the structural `try_pop` policy rule) stays a recorded follow-up candidate (`docs/handoffs/live-updates-1053/README.md`).
