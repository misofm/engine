Verdict: PASS

# #1254 attempt 1: adversarial verdict (Sol)

- Commit under review: `7436a64f9` (parent `b5489f00d`), branch `codex/1053-live-updates`.
- Reviewed from the export `/tmp/claude-1002/v1254-a1`. Builds used `CARGO_TARGET_DIR=/tmp/claude-1002/vtarget-1254`.
- The base export is `/tmp/claude-1002/v1254-a1-base`. Mutations ran in a second export, `/tmp/claude-1002/v1254-a1-mut`, with its own target directory.
- Logs are in `/tmp/claude-1002/v1254-a1-logs/`.
- I did not edit, build in or check out `/home/bl/misofm/wt-1053`.

## Summary

The slice does what D1-D4 ask.

- **builtins-compiler.**
  - `TrackControlRequest::input_lane` gates whether the input ring is created and whether it is charged.
  - The tail rule is the chain's own tail in both the prepared tails and `expected_tails`.
  - The seal records `(track, capacity, input_lane)` and is compared with `input.is_some()`.
  - A strip with no input consumer binds the plain `InputProcessor` in the scalar oracle. The bank lane takes `None`.
- **host-core.**
  - Adds `HostLiveLanes` (`ALL` and `FADER_AND_MATRIX`) and `prepare_host_runtime_with_live_lanes`, which uses `Concurrent` delivery.
  - Every other entry passes `ALL`.
  - Effect, input and route lanes are each gated by their own flag.
  - Observation taps with `effects: false` are refused with `host.observation.live_controls`.
- **host-web.**
  - A missing input lane is unaddressable: `queue_available` returns `None`.
  - The room pass then refuses the command with `COMMAND_REASON_UNSUPPORTED_KIND` before any push, so no ack can precede a drop.
  - It cannot panic, and the browser always prepares with `ALL`, so the case never arises there.

**Paths outside the authorized list.**

- `crates/graph-compiler/src/lib.rs`: four `TrackControlRequest` literals only, each gaining `input_lane: true`. The spec's "any other `TrackControlRequest` literal" covers this, and the attempt record notes it.
- `hosts/host-web/src/tests.rs`: one queue-counter closure now reads `owner.input.as_ref().expect(..)`. Compilation forced the change, and the attempt record notes it as a deviation.
- Neither changes behaviour.

**Results.**

- Every gate I re-ran is green.
- The AudioWorklet module digests match the implementer's exactly, for both base and head.
- All three browsers pass real browser qualification. Firefox needed reruns; see below.
- Eight of the nine mutations I ran went red and then green again. The ninth is my own probe V2, which survived and is MINOR 1.

There are no BLOCKER or MAJOR findings. There are three MINOR findings and two NITs.

## Findings

### BLOCKER
None.

### MAJOR
None.

### MINOR

1. **No test checks that the new entry uses `Concurrent` delivery (D2).**
   - Where: `crates/host-core/src/prepare.rs:651-661` and `crates/host-core/tests/live_lanes.rs`.
   - The probe: in V2 I set `between_render_calls` to `true` in `prepare_host_runtime_with_live_lanes`. All four `live_lanes` tests stay green, because fused and unfused passes render the same bits.
   - Why it matters: umbrella D5 and #1256 depend on this clause, since a concurrent C ABI producer cannot declare `BetweenRenderCalls`.
   - Fix: in gate 1(a), assert that the plan formed no fused fader-matrix bank. Either read the test-support fader-matrix witness (`FADER_MATRIX_FACTORY_CALLS`, through `builtins_compiler::test_only_fader_matrix_witness`) or check that every builtin bank's `control_delivery` is `Concurrent`.

2. **The `control_queue_depth` doc that D3 requires states the wrong rule.**
   - Where: `crates/host-core/src/prepare.rs:300-303`.
   - The doc says the input, effect and route queues are attached "only under `HostLiveLanes::ALL`". In fact each is gated by its own flag. Gate 1(c)'s own selection, `{ effects: false, ..ALL }`, attaches both input and route lanes.
   - Fix: say "the input queue when `strip_input` is set, the effect queues when `effects` is set, and the route queues when `routes` is set; every entry except `prepare_host_runtime_with_live_lanes` passes `ALL`".

3. **Docs on the handles are now stale.**
   - Where: `crates/host-core/src/prepare.rs:418-419` (`effect_controls`) and `:436-442` (`route_controls`).
   - Both still say "empty when no channel was requested". Neither mentions that they are also empty when `effects` or `routes` is false.
   - `route_controls` still says C ABI plans get route lanes "once #1053 attaches live controls there". Umbrella D5 now contradicts that: the C ABI prepares with `FADER_AND_MATRIX` (#1256), and its route lanes arrive with #1225.
   - Fix: add the `HostLiveLanes` condition to both docs, and replace the C ABI sentence with the D5 statement.

### NIT

1. **Two comments still say "three rings".**
   - Where: `crates/builtins-compiler/src/lib.rs:3690-3693` ("Three bounded rings per controlled track ... All three are charged here") and `:3675-3676` ("the three bounded rings charged below").
   - The input ring is now conditional. The new comment at `:3710` corrects this locally.
   - Fix: change both to "two or three".

2. **The precondition guard is a separate test.**
   - `only_the_plain_session_has_a_finite_lanes_free_tail` is gate 1(a)'s non-vacuity guard, but it lives in its own test function. A filtered run of 1(a) skips it.
   - Fix (optional): assert it at the top of 1(a) instead.

## Test value (one sentence per new test)

- **`only_the_plain_session_has_a_finite_lanes_free_tail`** (precondition): red if the plain fixture's lanes-free tail stops being finite, through fixture or tail-rule drift. Without it, an input lane attached under `FADER_AND_MATRIX` (M1/M1b) would pass 1(a) silently. The spec asks for this assertion explicitly.
- **`fader_and_matrix_lanes_attach_nothing_else_and_render_the_lanes_free_bits`** (1a): red if `FADER_AND_MATRIX` still attaches the input lane, an effect lane or a route lane, or changes a rendered bit.
  - My re-run of M1b went red at `live_lanes.rs:227` (`Infinite` against `Finite(0)`).
  - No existing test prepares a subset of the lanes.
- **`fader_and_matrix_lanes_render_as_the_session_with_their_values_baked`** (1b): red if the fader or matrix consumers stop reaching the banks once a strip has no input lane.
  - My own mutation V1 (`fader: None` when `!input_lane`) went red at `live_lanes.rs:280`.
  - Existing tests run only `ALL`.
- **`observation_without_effect_lanes_is_refused`** (1c): red if an observation tap is attached without the effect queue it rides. My re-run of M5 went red at `:298`.
- **`input_free_live_strips_match_between_the_scalar_oracle_and_the_banks`** (1d, builtins-compiler unit): red if the scalar oracle still requires an input consumer, or binds an input-less strip differently from the banks.
  - My re-run of M6 panicked at `lib.rs:2259`, and V1 went red here too.
  - Its `validate_for_session` assertion also catches seal drift: V6 and V7 went red at `lib.rs:7328`.
- **`live_control_rings_are_charged_exactly_as_allocated_with_and_without_the_input_lane`** (gate 3): red if an unrequested input ring is still allocated or still charged.
  - My re-run of M7 went red at `allocation_tracker.rs:1396`.
  - V6 and V7 (the seal flag forced to `true`) also turn it red.

## Mutations I ran

Each was applied in `/tmp/claude-1002/v1254-a1-mut`, run, and reverted. The baseline there was green first.

| # | Mutation | Result |
|---|---|---|
| M1b (re-run) | the tail rule ignores `input_lane`, in the prepared tails and in `expected_tails` | red: 1(a) |
| M5 (re-run) | the observation refusal drops `\|\| !lanes.effects` | red: 1(c) |
| M6 (re-run) | the scalar oracle panics on a missing input consumer | red: 1(d) |
| M7 (re-run) | the input ring is charged unconditionally | red: gate 3 |
| V1 (mine) | the fader consumer becomes `None` when `!input_lane` | red: 1(b) and 1(d) |
| V6 (mine) | `validate_for_session` reads the input lane as always present | red: 1(a), 1(b) (`builtin.prepared.control_set`), 1(d) and gate 3 |
| V7 (mine) | the seal always records `input_lane = true` | red: 1(a), 1(b), 1(d) and gate 3 |
| V2 (mine) | the new entry prepares with `BetweenRenderCalls` | **green**: MINOR 1 |

## Gates I re-ran (export of `7436a64f9`, x86-64-v3)

**Gate 1.** All pass.
- `cargo test -p host-core --features test-support --test live_lanes`: 4 passed.
- 1(d), the builtins-compiler unit test: 1 passed.
- `allocation_tracker`: 11 passed, 1 ignored. The ignored case is the existing nightly-only 65,537-track case.

**Gate 2.** All pass.
- The workspace test command: exit 0, 116 result lines, 1300 passed, 0 failed, 10 ignored. This matches the attempt record.
- `cargo test --locked -p builtins-compiler --no-run`: ok.
- `cargo test --locked --release -p audit -p bench -p console-workload`: 113 passed, 0 failed.

**Gate 3.** Passes; it ran as part of gate 1.

**Gate 4.** All pass.
- Module digests: base `b5489f00d` is `6eb292980c2f31a7a2b86ef8388a17b1f0b5efa1b70544ec1a955d1bd207188f`, and head is `04ce0d44483b5ebce30619c3abcf7991e9dd0d4d69719c3df30def529b3e6f52`. The module changed, as the spec expects, and both digests match the attempt record.
- `check-web-audioworklet.sh --without-metadata-regeneration`: ok.
- `check-browser-expected-resources.py --artifacts`: digests and exact rows agree, within budget, and its self-test passed.
- `check-scalar-oracle-absent.py`: ok. This matters because the scalar-oracle change is test-only and must not ship.
- `test-web-audioworklet.sh`: ok.

**Browser qualification.** Run as CI's `browser` job does: `npm run qualify -- --check-matrix --self-test-mutations` against a private pulseaudio null sink.
- Chromium: pass.
- WebKit: pass.
- Firefox: the first run failed `sdk-spectrum-hop`, an SDK spectrum publication-timing gate, on a host with load average around 27.
  - Reruns passed twice on head and twice on base.
  - This is an environmental flake, not this commit.
- No browser qualification stub needs a change: no wire, session or request field changed. `hosts/host-web/qualification/*.json` and `hosts/host-web/web/` are untouched, and the frozen export-set check passed.

**Gate 5.** All pass except AArch64, which I did not run.
- `cargo fmt --all -- --check`: ok.
- `cargo clippy ... -D warnings`: ok.
- `cargo doc` with `-D warnings`: ok.
- The host-core, realtime and workspace check and test policy scripts: ok.
- `check-cross-targets.sh`: PASS. The iOS `memset_pattern16` expected failures (#1018) are unchanged.
- AArch64 (`run-aarch64-tests.sh debug`) is CI-only and was not run locally.

**Batch rules.**
- `check-capi-abi.sh`: ok (x86_64, shared and static linkage).
  - My first run failed only because my `CARGO_TARGET_DIR` moved `libcapi.so` out of the path the script reads.
  - The rerun with the export-local target passed.
- `audit capi`: 100,000 calls, 0 allocations, 0 deallocations, 0 syscalls.
- `check-ci-path-routing.py`: ok. The new test file routes `full`.

## Other checks

- **Realtime.**
  - The render path is unchanged.
  - The host-web push path is allocation-free and panic-free (`ok_or`/`?`), and its render closure check passed.
- **The acked-batch question.**
  - This slice adds no new admission.
  - host-web still checks room on every queue before it pushes anything, and a missing input lane is refused in that first pass.
  - So no ack can precede a drop.
- **Moving `control_capacity` before the phase-two guard.**
  - The allocation tracker records every allocation in the window, not net bytes, so a transient lookup map inside the window shows up as an uncharged allocation.
  - Moving the map out of the window matches the #1242 D2 precedent (`effective_faders`) and changes nothing in production.
- **The tail rule.**
  - `BuiltinChain::tail()` reads only the input section's filters. A live fader or matrix lane therefore cannot raise the tail, and `FADER_AND_MATRIX`'s finite tail is correct.
