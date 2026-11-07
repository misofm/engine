# C ABI: the FP-environment attestation is cached on the plan, so a render moved to another thread is never re-attested

Tracker #2258, item HC-6. Pattern: a one-shot `Cell<bool>` encoding a lifecycle phase
("attested") that really belongs to something else (the OS thread).

## Mission
`miso_engine_v1_render_f32_planar` must prove, on the thread each block actually runs on, that
the canonical floating-point control word it installs took effect, and refuse the block with
`render.fp_environment.invalid` otherwise. The proof must not be remembered by the plan, because
a C plan handle is not tied to one OS thread.

## Verified (base `8147ab6b8`)
- The entry pins the canonical word on every call: `let _fp_env = CanonicalFpEnv::enter();`
  (`crates/capi/src/ffi.rs` 1398). The pin saves the caller's MXCSR (`x86_64`, canonical
  `0x1F80`: exceptions masked, round-to-nearest-even, FTZ and DAZ clear) or FPCR (`aarch64`,
  canonical `0`: `RMode` nearest, `FZ`/`FZ16` clear, no traps, no FEAT_AFP bits), installs the
  canonical word, and restores the caller's word on drop (`crates/lane/src/fpenv.rs`).
- The check that the pin took (`lane::fpenv::in_canonical_fp_environment()`: read the word back
  and compare its control bits) runs only while `!state.fp_env_attested()`, and then sets the flag
  (`ffi.rs` 1431-1440). The flag is `PlanState::fp_env_attested: Cell<bool>`
  (`crates/control-plane/src/plan.rs` 59, set by `attest_fp_env` 299), so it is per plan, set on
  the plan's first block, and never cleared.
- The C ABI does not tie a plan to one OS thread: the header's thread table says the render call
  is "render thread only, never concurrently with itself", and the handle is a raw pointer a host
  may hand to a new callback thread. That happens in practice: an Android Oboe stream reopened
  after a device disconnect calls back on a new thread, and Core Audio does not promise a stable
  IO thread across a stop/start.
- **What a moved render gets today:** the pin still runs on the new thread every block, so on
  conforming hardware the block renders in the canonical environment. What it does not get is the
  proof: the readback is never taken on the new thread. If the pin did not take there, the block
  would render off-pin silently, which is exactly what #146's attestation exists to refuse.
- Conforming hardware never refuses the pin (`LDMXCSR` loads the word or raises #GP; FPCR `0` has
  no read-as-one control bit), so no test can make a real thread refuse it. The reproduction marks
  the second thread with a `#[cfg(test)]` thread-local in `ffi.rs` that makes the readback answer
  "not canonical": the closest honest stand-in, and it isolates exactly the claim under test,
  that the check runs on the block's thread.

### The two models
`host-core::StartedRenderSession` (`crates/host-core/src/render_session.rs` 67-111) is a typestate:
`start` runs `lane::attest_fp_environment()` on the render thread, and the handle is `!Send` and
`!Sync`, so the compiler proves every later block runs on the attested thread. The C ABI cannot use
it: its plan is `control-plane::PlanState`, not a `PreparedRenderPlan`, and above all C has no
`Send`, so a handle cannot be pinned to a thread by its type. The C ABI's equivalent of the
typestate's guarantee is therefore a runtime check on every block, which is one control-word read
and a compare on top of the read and two writes the pin already does. That removes the implicit
state outright; no thread id is read (which `docs/REALTIME_DEPENDENCY_POLICY.md` would not allow
cheaply anyway), and no shared state or atomic is introduced, so no loom model applies.

## Decision / model
- `ffi.rs`: the entry calls `canonical_fp_env_in_effect()` on every block, after the descriptor
  checks (the same position as before, so rejection order is unchanged). It is
  `lane::fpenv::in_canonical_fp_environment()` plus, under `#[cfg(test)]` only, the thread-local
  stand-in described above (`test_refuse_fp_pin_on_this_thread`).
- `control-plane/src/plan.rs`: `fp_env_attested`, `fp_env_attested()` and `attest_fp_env()` are
  deleted. They had no other user.
- host-core's typestate is unchanged: for a Rust host it is a stronger, compile-time proof.

## Non-goals
- No change to the C header's frozen thread table or to any result code or diagnostic.
- No change to host-core, the web host (wasm32 has no control word) or `lane`'s guard.
- No per-block restore check (`attest_fp_environment`'s second half): the guard's restore is
  covered by the existing restore tests and is not thread-dependent.

## Gates
- Red-on-base test: `runtime::tests::a_render_moved_to_a_thread_whose_fp_pin_does_not_take_is_refused`
  (`crates/capi/src/runtime/tests.rs`). It renders block 0 on the test thread, moves the handle to
  a spawned thread whose pin is marked refused, and requires `RESULT_RENDER_REJECTED` with
  `render.fp_environment.invalid`; then block 1 renders on the original thread.
- `cargo test --locked -p capi`, `-p control-plane --features test-support`,
  `-p lane --features test-support`; clippy `--all-targets --all-features -D warnings` on the
  three; `cargo fmt --check`; `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps`;
  `scripts/check-workspace-policy.sh`, `scripts/check-lane-policy.sh`, `scripts/check-capi-abi.sh`,
  `realtime_policy`.
- One-time no-bit-moved comparison against base (PR evidence, not a committed test).

## Evidence (attempt 1)
**Red on base** (base `8147ab6b8` plus only the test seam and the test):
```
assertion `left == right` failed: a render on a thread whose canonical word did not take was not re-attested
  left: (0, "")
 right: (8, "render.fp_environment.invalid")
test result: FAILED. 0 passed; 1 failed; ... 192 filtered out
```
Green on head.

**Test value.** Removing the attestation check entirely was killed by no test on base (no real
thread can fail it; M-146-2 kills only the guard's removal). The new test kills it, and kills any
return of a first-block-only or per-plan cache.

**Mutations** (`cargo test -p capi --lib fp_`):

| Mutant | Result |
|---|---|
| check only on the first block (`absolute_sample == 0 && ...`) | RED: new test, `left: (0, "")` |
| check removed (`false && ...`) | RED: new test, `left: (0, "")` |
| guard removed (M-146-2, `let _fp_env = ();`) | RED: `the_c_render_entry_is_canonical_under_a_caller_that_set_ftz_and_daz`, `left: 8` (now refused on every block) |

**No bit moved.** A temporary (uncommitted) probe in the `x86_64` `fp_environment` module hashed
(FNV-1a) every word of `render_through_the_c_entry` (12 blocks of 128 frames, subnormal fade tail,
through the C entry) under a clear caller word and under FTZ+DAZ, release build. Base and head
both print `words=3072 fnv=67a5c0b1a441e4e7 leak=0x0` for both words.

**Gates.** `cargo test --locked -p capi` (all suites, lib 193 passed),
`-p control-plane --features test-support`, `-p lane --features test-support`: all pass. clippy
(`-p capi -p control-plane -p lane --all-targets --all-features`, and `-p capi --all-targets`),
fmt, doc, workspace policy, lane policy, C ABI check (shared and static) and `realtime_policy`
(298 marked regions in 48 files): clean. Not run locally: AArch64 targets (not installed); CI's
cross-target leg covers them.
