# Web host: a later failure overwrites a disposed session's state and the first failure's diagnostic

Child of #2258 (implicit state machines), item WEB-9. Found by the audit of 2026-11-06 and re-verified at base `b1fc70c0f`.

## Mission
Make the web host's session lifecycle word monotone: once a session has failed or been disposed, no writer on either thread moves it back, and the first failure is the session's failure. Every writer goes through one transition on one owner type, and a loom model covers the race.

## Verified defect (base `b1fc70c0f`)
The lifecycle word is `type Lifecycle = Arc<AtomicU32>` (`hosts/host-web/src/lib.rs` 939), shared by the control half and the render half. Its states are `STATE_READY` (2), `STATE_FAILED` (3) and `STATE_DISPOSED` (4) (lines 202-206). It is written by four plain stores:

| Writer | Thread | Line | Guard before the store |
|---|---|---|---|
| `AudioWorkletEngineHost::dispose` | control | 2737 | none (stores `DISPOSED`) |
| `AudioWorkletEngineHost::fail` (every control failure, `fail_session` included) | control | 2899 | the call's `control_ready()` check at entry: a check-then-store |
| `RenderHalf::reject_output_quantum` | render | 3191 | the half's **own** `status.state`, not the word |
| `RenderHalf::fail` (render failures, and `RenderHost::fail`, the stack canary) | render | 3358 | none |

The render half's own status learns of a control failure or a dispose only at its next `render_next` (line 3217). Until then, its `status.state` is still `STATE_READY`. The worklet's render export (`hosts/host-web/src/ffi.rs` 1186-1196) checks that status and then calls `fail` for a bad stack canary or `reject_output_quantum` for a wrong output shape. The worklet's release path calls `fail` for a bad canary too (line 1683). So two sequential interleavings, which need no race, reach a regression:

1. **A dispose, then a render failure.** The Worker disposes its host while the render half is away; `dispose` documents that case (line 2733). The half then rejects an output shape or finds a bad canary. The word goes from `DISPOSED` back to `FAILED`, and the call returns `RESULT_REPREPARE_REQUIRED` or `RESULT_INTERNAL` for a session that no longer exists.
2. **A control failure, then a render failure.** The Worker fails the session (`fail_session`, #2267, or any engine-recorded control failure). Before its next block, the half rejects a shape or fails its canary. That second failure is reported as new: `RESULT_REPREPARE_REQUIRED` or `RESULT_INTERNAL`. Its diagnostic replaces the first failure's when the Worker adopts the half back (`adopt_render`, line 2821).

There is also a real race, which #2267 recorded as benign. The control check-then-store can interleave with a render failure. Then `fail_session` returns `RESULT_OK` for a session the render half has already failed, and it overwrites that failure's code in the Worker's status.

**Impact today.** After a dispose, no reader tells `FAILED` from `DISPOSED`: the render half compares the word only with `STATE_READY`, and the control half that would publish it is gone. So case 1's word regression is not observable today; its wrong result code is. Case 2 is observable: the page sees the wrong diagnostic for the session's failure. **Severity L** (the tracker's), with a concrete sequential path.

Red on base: see the Evidence section.

## Decision / model
- **One owner type.** A new module, `hosts/host-web/src/lifecycle.rs`, defines `SessionLifecycle`, a `repr(transparent)` word. `Lifecycle` becomes `Arc<SessionLifecycle>`. Its only writes are two transitions, and each is one `fetch_max` (`AcqRel`) over the ordered states:
  - `fail() -> Ending`;
  - `dispose()`.

  `READY < FAILED < DISPOSED` is the existing ABI numbering, held by a `const` assertion, so terminal states never regress. `Ending::First` means this call moved the session out of `READY`; `Ending::AlreadyEnded` means another writer did first. Reads are `is_ready()` and `state()`. A test-only `force_for_test` keeps the one test that drives a refusal and then a recovery.
- **The first failure stands.** A failure whose transition is `AlreadyEnded` is refused as a call on a session that is not ready, and it records nothing of its own:
  - control `fail` returns `RESULT_WRONG_STATE` and writes no diagnostic, last result or silence. `fail_session` keeps its documented refusal, and the check-then-store is gone;
  - render `fail` and `reject_output_quantum` mark the half's own status failed (and `fail` silences, as before). They return `RESULT_WRONG_STATE` and keep no diagnostic, so an adoption cannot replace the first failure's.
- **The released status shell.** It also refuses `reject_output_quantum`, as `render_next` on it already does (line 3208). Today it records `FAILED` in a status that is meant to mirror the word.
- **Realtime.** The render-side writers are in `REALTIME_POLICY` regions. A `fetch_max` is one read-modify-write: wasm lowers it to a strong `cmpxchg` loop, and the loop retries only when the word changed. The word only increases through three values, so it retries at most twice. It takes no lock, allocates nothing and makes no syscall. `docs/REALTIME_DEPENDENCY_POLICY.md` already permits render RMWs (the revision gate's `fetch_or`). The per-block check in `render_next` stays one `Acquire` load.
- **Loom.** `lifecycle_loom` models the two threads on the real type: a control dispose or failure racing a render failure. It asserts:
  - the final state is the greatest one written, so a failure never undoes a dispose;
  - exactly one of two racing failures is `First`.

  It needs `loom` as a `cfg(loom)` dependency of `host-web` (the capi precedent, #2276), and a CI leg on the host-core leg's target dir and flags, in the debug profile.

## Non-goals
- **The half's own status after a dispose.** A render half on a disposed session reports `STATE_FAILED` in its own status, not `STATE_DISPOSED`. That is what it reports today, and the JS reads it. Changing it is a JS-visible change in files other agents own.
- **Storing the failure code in the word.** No reader needs it today.
- **The JS files under `hosts/host-web/web/`.**
- **Other tracker items.**

## Gates
- A regression test, `a_later_failure_or_a_dispose_never_moves_the_lifecycle_back` in `hosts/host-web/src/tests.rs`, red on base.
- `lifecycle_loom`, run as: `RUSTFLAGS='--cfg loom --check-cfg=cfg(loom) -C target-feature=+avx2,+fma' cargo test --locked -p host-web --lib lifecycle_loom`. It also runs as a qualification leg.
- A one-time no-bit-moved comparison of the rendered output against base, across the failure paths. It is PR evidence, not a committed test.
- `cargo test --locked -p host-web --all-features`, clippy `-D warnings` on `host-web --all-targets --all-features`, `cargo fmt --check` and `cargo doc` with `-D warnings`.
- `bash scripts/check-workspace-policy.sh`, `cargo run --release -p realtime-policy` and `bash scripts/test-web-audioworklet.sh`.

## Evidence
### Attempt 1 (Terra, 2026-11-07)
Base `b1fc70c0f`.

**Red on base.** The regression test's two halves were run at base, with only the test added (the word read as a plain `load`). A probe printing each value gave:

```
control first:  reject_output_quantum=9 (REPREPARE_REQUIRED)  fail=255 (INTERNAL)
                diagnostic after adopt_render = "web.instance.stack_canary\t$\n"   (expected "web.host.failed\t$\n")
dispose first:  word after dispose=4 (DISPOSED)  reject_output_quantum=9  word after it=3 (FAILED)  fail=255
```

The committed test failed on base at its first ready-state assertion. It also covers the released status shell, which on base returned `9` and not `3`:

```
assertion `left == right` failed: a shape mismatch on a failed session is refused, not a second failure
  left: 9
 right: 3
```

**Mutations of the fix.** Each one was applied, run and restored.

| Mutation | Check | Result |
|---|---|---|
| `fail` as load, then `store(max)` | loom | red: "a failure undid the dispose" (3 vs 4) |
| `fail` takes its `Ending` from a load before the `fetch_max` | loom | red: "exactly one of two failures is the session's" (both first) |
| `dispose` as load, then `store(max)` | loom | red: "a failure undid the dispose" |
| `fail` as `swap` (a plain store) | unit | red: "a failure after a dispose leaves the session disposed" (3 vs 4) |
| render `reject_output_quantum` ignores `AlreadyEnded` | unit | red: `9` vs `3` |
| render `fail` keeps its diagnostic when `AlreadyEnded` | unit | red: "the first failure's diagnostic stands across the adoption" |
| the shell's `|| self.released` refusal removed | unit | red: "the released status shell renders nothing, so it rejects no shape" (`9` vs `3`) |
| control `end_session` ignores `AlreadyEnded` | unit | the #2267 test red: a second `fail_session` accepted (`0` vs `3`) |

The last row matters because `fail_session` no longer has its own readiness check: the #2267 test's "a second failure keeps the first code" now defends the transition.

**No bit moved.** A temporary probe, not committed, rendered the identity fixture session (`tests/browser-v1/session.json`, a 4,096-sample source of ramps) on a released half for 8 blocks of 128 frames. It covered six scenarios, each acting at block 4:
- `fail_session`;
- the half's `reject_output_quantum(64)`;
- the half's stack-canary `fail`;
- `dispose`;
- `fail_session`, then the half's `fail`;
- nothing.

It hashed the bits of every output block (SHA-256) and listed every return code.

Both base `b1fc70c0f` and the fix gave the digest `9b85639d43b9e9bb8112c43cfd7d159473895479103573cc7a7c630cb69c6ccd`. The return codes, per scenario, were (the action's code, then each `render_next`):

```
fail_session        0 0 0 0 | 0   | 3 3 3 3
reject(64)          0 0 0 0 | 9   | 3 3 3 3
fail (canary)       0 0 0 0 | 255 | 3 3 3 3
dispose             0 0 0 0 | 0   | 3 3 3 3
fail_session, fail  0 0 0 0 | 255 | 3 3 3 3   (the fix: 3)
nothing             0 0 0 0 | 0   | 0 0 0 0
```

The one code that differs is in the control-then-render scenario: the half's `fail` there returns `3` (`WRONG_STATE`), not `255`. That is the intended change. No rendered sample moved: the render paths' silence and their per-block `Acquire` check are unchanged.

**Gates run (all green):**
- `cargo test --locked -p host-web --all-features` and `cargo test --locked -p host-web`;
- `cargo clippy --locked -p host-web --all-targets -- -D warnings`, with and without `--all-features`;
- `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps -p host-web` and `cargo fmt --check`;
- `bash scripts/check-workspace-policy.sh`, which reported `workspace policy: ok`;
- `cargo run --locked --release -p realtime-policy`, which reported `realtime policy: ok (292 marked regions in 46 files)`;
- `bash scripts/test-web-audioworklet.sh`, exit 0;
- the loom leg as CI runs it: `lifecycle_loom ... ok`, 39 s to build on 32 cores.

**Also fixed here:** `set_state_for_test` (and the new `force_for_test`) are compiled only with `test-support`, their only callers' feature. That removes the dead-code warning that host-web's default test build already gave on base.

**Not run here:** the wasm artifact build and its artifact-backed gates, including the call graph. The pinned browser nightly is not installed. CI's artifact job builds fresh. The render-side change is two `fetch_max` calls on the failure paths, inside existing `REALTIME_POLICY` regions, and no call is added.

**Test value.**
- Unit test: red if a render-side failure writes the lifecycle word with a plain store, so a dispose is undone. Also red if it records a second failure, whose code and diagnostic would replace the first one's at the adoption, or if the released shell records a shape failure. No existing test fails the render half after the session ended.
- `lifecycle_loom`: red if either transition is not one read-modify-write, a defect no sequential test can see.
