# C ABI: the plan and engine handles race an in-flight call or destroy instead of refusing it

A follow-up from #2276 and #2293 (both name it in their non-goals), filed under tracker #2258.

## Mission
Finish the "refuse, don't race" model of #2276 on the two handles it left out. A plan destroy that
overlaps a render-side call, and any engine call or engine destroy that overlaps another engine
call, must become a defined refusal rather than undefined behaviour. The shared `HandleClaim` is
the one mechanism.

## Verified defect (base `5f75ca78d`)
`crates/capi/src/ffi.rs`:
- **Plan destroy.** `miso_engine_v1_plan_destroy` (1683-1694) checks the handle kind and drops the
  `Box<Plan>` without touching `render_busy`. `render_f32_planar` and `plan_prepare_seek` hold
  `render_busy` (`claim_render_side`, 250-253) for the whole time they use `PlanState`. A destroy
  that overlaps one of them therefore frees `PlanState` under the running call: a use-after-free.
  `session_destroy` (1653-1675, #2276) already claims, and refuses and leaks when the claim is held.
- **Engine.** `Engine` (`crates/capi/src/abi.rs` 483-489) has no claim word.
  `compile_session` (431-538) writes the engine's `RefCell<[u8; 256]>` and `Cell<usize>`
  diagnostic through `set_engine_error` (36-45) and `clear_engine_error` (47-49). `last_error` on
  an engine (1607-1613) borrows the same `RefCell`. `engine_destroy` (407-422) frees with no check.
  Two threads on one engine therefore race a non-atomic `RefCell` borrow flag and a `Cell`, which
  is a data race. A destroy can also free the engine under a running compile.

The header requires every destroy to be quiescent, and allows only one thread at a time on an
engine. So only a host that breaks those rules reaches the defect. A control-thread teardown
racing an audio callback is a plausible mobile bug, and the failure is silent memory corruption.

**Same outcome, one issue.** The engine half is #2276's second non-goal. It has the same shape as
both #2276 and the plan destroy: a handle whose exclusive state a second call can reach. The fix
is the same: claim on entry, refuse with `BUSY`, and make the destroy refuse and leak. It touches
the same files (`ffi.rs`, `abi.rs`, the header's thread-ownership block,
`docs/C_ABI_V1_QUALIFICATION.md`) and uses the same claim and loom model. After it, every C-ABI
handle refuses concurrent use the same way.

**Severity.** M, as for #2276: reachable only through host misuse, but the failure is memory
corruption, not a result code.

## Decision / model
- **One destroy rule.** `HandleClaim::claim_for_destroy(word) -> bool` claims the word and
  forgets the claim, so the word goes with the freed handle. It returns `false` when another call
  holds the word. `engine_destroy`, `session_destroy` and `plan_destroy` all go through it.
  `session_destroy` used to claim through `claim_session` and `mem::forget`; its behaviour is
  unchanged. A refused destroy frees nothing, and the handle stays live and valid. The call in
  flight, every later call and a later quiescent destroy then proceed normally. A host that never
  destroys the handle again leaks it. #2276's reviewer ruled refuse-and-leak the only memory-safe
  option: freeing would free under the call, and deferring the free to the holder would free the
  handle from inside its own call.
- **The plan destroy claims `render_busy`**, so it excludes the two render-side calls only. The
  plan's any-thread queries (`plan_resources`, `plan_watermark`, `last_error`) take no claim and
  must not, because they run concurrently with render. So a destroy overlapping one of them stays
  the host's to prevent, as does a call that starts after a successful destroy (freed memory
  carries no claim).
- **The engine gains `busy: ClaimWord`.** `compile_session` claims it right after the handle check
  and holds it for the whole compile. `last_error` on an engine claims it too. A call that finds it
  held returns `MISO_ENGINE_V1_BUSY`, touches no engine state and writes neither the engine
  diagnostic nor any bytes output. A refused compile still leaves `out_session` and `out_plan`
  null, because the frozen atomic-publication rule clears both before the handle check, as on every
  rejection. Moving the claim before that clear would mean dropping the clear on `WRONG_HANDLE`.
  The holder borrows `&Engine`, whose fields are all interior-mutable, so a refused call's read of
  the word aliases nothing exclusive. `Engine::new` stops being `const fn`, because loom's atomic
  constructor is not const (it had no const caller).
- **ABI growth.** No symbol, struct, size, feature bit, result code or `ABI_VERSION` change.
  `Engine` is opaque, and `CapiAdapter::ADAPTER_ALLOCATIONS` charges `Session` and `Plan` only, so
  no retained-bytes budget moves. `Plan` already had `render_busy`. `BUSY` (9, #2276) gains two
  more callers. A host built against an older header sees it as an unknown code, which the growth
  rule treats as a failure.
- **The render path is unchanged.** `render_f32_planar` and `plan_prepare_seek` take the same
  claim as before, and `HandleClaim` only gains a method.

## Non-goals
- A plan destroy against an in-flight any-thread plan query. Ruled at review: an accepted
  residual, of the same kind as the window between a call's entry and its claim. Both are the
  host's to prevent under the quiescence rule. A reader count would not close it either, because a
  query could still enter after the free and before it reads the count.
- `hosts/host-web` (its render path has no claim word), the render arithmetic, and the plan's
  busy refusal (#2293).
- `miso_engine_v1_engine_create` takes no live engine, so it has nothing to claim.

## Gates
- `ffi::tests::a_plan_destroy_while_a_render_side_call_holds_the_plan_frees_nothing`, red on base.
  With the plan's render claim held, `plan_destroy` disposes of nothing (the lifecycle counters
  stay zero). Released, the plan renders a finite block and a quiescent destroy frees it.
- `ffi::tests::an_engine_call_while_another_holds_the_engine_is_refused_not_raced`, red on base.
  With the engine's claim held, `compile_session` and `last_error` on the engine return `BUSY`,
  leave the holder's diagnostic, `diagnostics` and the `last_error` output untouched, and publish
  no handle. `engine_destroy` makes no deallocation (`bench_support::alloc` thread counters).
  Released, the diagnostic is the holder's and a destroy deallocates.
- `ffi::tests::handle_claim_loom::destroy_racing_a_call_loom`: a destroy racing one call on the
  same handle. In every interleaving no call reaches freed state (loom's `UnsafeCell` and a
  `FREED` check), the two are never both refused, a refused destroy leaves the word released and a
  later quiescent destroy succeeds, and a freed handle's word stays claimed. It runs on the
  existing capi loom leg, whose filter `handle_claim_loom` matches it, so there is no workflow
  change.
- The header's thread-ownership block and `BUSY` comment, the rustdoc of `RESULT_BUSY`, the three
  destroys, `compile_session` and `last_error`, and `docs/C_ABI_V1_QUALIFICATION.md` state the
  refusal.
- `cargo test --locked -p capi`; clippy `-D warnings` (also under the loom flags);
  `cargo fmt --check`; `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps -p capi`;
  `bash scripts/check-workspace-policy.sh`; `bash scripts/check-capi-abi.sh`;
  `cargo run --locked --release -p realtime-policy`.

## Release note
If this merges before release #2253 (0.10.5) pins the engine, that release's "Upgrading from
0.10.4" section in `sdk/README.md` must carry this bullet, next to #2293's:

> Changed: `miso_engine_v1_plan_destroy` and `miso_engine_v1_engine_destroy` free nothing when
> another call holds the handle (issue #2321), as `miso_engine_v1_session_destroy` already did:
> the handle stays valid and a later quiescent destroy frees it. `miso_engine_v1_compile_session`
> and `miso_engine_v1_last_error` on an engine refuse a concurrent call on the same engine with
> `MISO_ENGINE_V1_BUSY` and write no diagnostic. Only a host that breaks the engine's or the
> plan's thread rule sees any of this. No symbol, struct, size, feature bit or `ABI_VERSION`
> changes.

## Evidence (attempt 1)

**Files.** `crates/capi/src/ffi.rs` changes in four places:
- `HandleClaim::claim_for_destroy`, `engine_busy_slot` and `claim_engine`;
- the claim in `compile_session` and on `last_error`'s engine branch;
- the three destroys, which now share one rule;
- rustdoc, a `test_hold_engine_claim` hook, two unit tests and the loom model.

`crates/capi/src/abi.rs` gains `Engine::busy`, updated `Engine` and `Plan` field-ownership docs
and an updated `RESULT_BUSY` doc. The header's thread-ownership block (engine and `*_destroy` rows)
and its `BUSY` comment change, and `docs/C_ABI_V1_QUALIFICATION.md` gains a #2321 paragraph after
#2293's. There is no workflow change, because the existing capi loom leg's `handle_claim_loom`
filter picks up the new model. No existing test is made useless: #2276's session test still holds
the session's claim through the new shared destroy rule.

**Red on base** (`5f75ca78d`, the two unit tests only; the engine hook is a no-op there, because
base has no engine word):
```
---- a_plan_destroy_while_a_render_side_call_holds_the_plan_frees_nothing ----
assertion `left == right` failed: a destroy refused by a held render claim frees nothing
  left: TestOwnerCounters { ..., current_plan_disposed: 1, ... }
 right: TestOwnerCounters { ..., current_plan_disposed: 0, ... }
---- an_engine_call_while_another_holds_the_engine_is_refused_not_raced ----
assertion `left == right` failed
  left: [5, 0]
 right: [9, 9]
```
On base, the held plan was freed (and the test's later render would have been a use-after-free).
The "held" compile ran to `COMPILE_REJECTED` and wrote its diagnostic, and `last_error` on the
engine read the `RefCell`. At head both tests pass.

**Mutations (each reverted; red in every case).**
- `plan_destroy` without the claim: unit red, `current_plan_disposed: 1`.
- `engine_destroy` without the claim: unit red, `a destroy refused by a held engine claim frees
  nothing`, `left: 1, right: 0`.
- `compile_session` without the claim: unit red, `left: [5, 9]`.
- `last_error` on an engine without the claim: unit red, `left: [9, 0]`.
- `claim_for_destroy` that always returns `true` (no claim). All three destroy tests go red,
  session included, and loom goes red: `Causality violation: Concurrent write accesses to
  UnsafeCell`.
- `claim_for_destroy` that releases the claim before the free (`map(drop)` for `map(forget)`).
  The unit tests stay green, because they hold the word from outside. Loom goes red with the same
  causality violation (a call claims the released word while the free runs), and with `a freed
  handle's word was released`. Only the two-thread model reaches this one.

**Test value.**
- Plan unit test: red if `plan_destroy` frees a plan whose render claim is held (the defect), or
  if a refused destroy leaves the plan unable to render. No existing test destroys a plan while
  its claim is held.
- Engine unit test: red if `compile_session`, `last_error` on an engine or `engine_destroy`
  reaches the engine without claiming it, or if a refused call writes the diagnostic or a bytes
  output. Before this issue no engine call could be refused at all.
- Loom model: red if a destroy frees without claiming, or releases its claim, so that a call can
  reach freed state; if a refused destroy keeps the word; or if a destroy and a call are both
  refused. `handle_claim_loom` has no destroy, and the unit tests hold the word from one thread,
  so neither sees the release-before-free ordering.

**No rendered bit moved.** The diff has no hunk in `render_f32_planar` or `plan_prepare_seek`. The
render path's only claim is `claim_render_side`, unchanged, and `HandleClaim` gains a method
only. A one-time check, not committed: `audit capi` (release, 100,000 render calls through the
claim) at head gives `pcm_digest` `30dcfd806e7ba058`, `total_violations: 0`, 0 allocations and 0
syscalls. That is the digest #2293 recorded at its base `875a4c17e` and at its head.

**Gates (head).**
- `cargo test --locked -p capi --no-fail-fast`: 192 lib tests, the nine integration binaries and
  the doc tests pass.
- Loom leg (`RUSTFLAGS='--cfg loom --check-cfg=cfg(loom) -C target-feature=+avx2,+fma' cargo
  test --locked -p capi --lib handle_claim_loom`): `handle_claim_loom` and
  `destroy_racing_a_call_loom` pass.
- `cargo clippy --locked -p capi --all-targets --all-features -- -D warnings`: clean. Clippy of
  the lib and tests under the loom flags: clean.
- `cargo fmt --check`: clean.
- `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps -p capi`: clean.
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
- `bash scripts/check-capi-abi.sh`: `C ABI check: ok (x86_64-unknown-linux-gnu, shared and static
  linkage)`.
- `cargo run --locked --release -p realtime-policy`: `realtime policy: ok (294 marked regions in
  46 files)`.

## Verdict (attempt 1)
**PASS**, with three minors and one nit folded on the branch after a merge of `origin/main`
(the merged tree was green). The reviewer's rulings:
- Folding the engine handle into this issue was right: same shape, same outcome, same files.
- No claim leaks. Only a destroy keeps a claim, and only on the word of the handle it frees.
- The any-thread-query residual is not a blocker, and a reader count would not close it (the
  non-goal above now records the ruling).
- The loom model is sound.

The minors, folded:
1. *Non-goal wording*: the plan-destroy-versus-query item records the ruling instead of "a
   separate follow-up".
2. *Comment wording*: the three destroys' SAFETY comments and the loom model's doc say that no
   call *holds* the handle, not that none is in flight. The header's `*_destroy` row adds that a
   call which has entered but not yet claimed its handle is, like one after a successful destroy,
   the host's to prevent.
3. *Spec filename*: renamed (a tracked move) to match the GitHub title.
4. *Nit*: the engine test's quiescent destroy asserts `freed > 0`, not exactly one deallocation.
