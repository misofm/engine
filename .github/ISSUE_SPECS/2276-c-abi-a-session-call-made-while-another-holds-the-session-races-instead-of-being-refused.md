# C ABI: a session call made while another holds the session races instead of being refused

Found by the implicit-state-machine audit of 2026-11-06 (tracker #2258, item HC-3).

## Mission
Make concurrent use of one C-ABI session handle a defined, refused call instead of undefined
behaviour: every entry point that touches a session's state or diagnostic claims the session with
one compare-exchange, and a call that finds it claimed touches nothing and returns a result code
that says so. The plan's render-side claim (#2213) is the pattern, and the two share one claim type.

## Verified defect (base `76ff20b54`)
`crates/capi/src/ffi.rs`:
- Eight entry bodies form `&mut Session` from the raw handle on the strength of a comment ("the ABI
  serializes session calls"): `source_submit_planar_f32` (567), `source_seek_entry` (691, behind
  `source_seek` and `source_seek_at`), `session_seek_landing` (841, behind `session_seek`,
  `session_seek_at` and `session_seek_at_once`), `source_seek_report` (920), `service` (977),
  `declare_discontinuity` (1032), `submit` (1162, behind `submit_command` and
  `submit_document_command`) and `dequeue_event` (1224).
- `borrowed_source_id` (138-140) writes the session's `RefCell` diagnostic before that borrow, and
  `last_error` on a session (1558-1563) borrows the whole `Session` and its `RefCell`.
- `session_destroy` (1585-1596) frees the box with no check that a call is in flight.
- `Session` (`crates/capi/src/abi.rs` 493-510) is `Send` (557) and has no claim word. Its
  `!Sync` marker binds Rust callers only; a C host reaches every entry with a raw pointer.

So a host that breaks the header's session thread rule (two threads, or a call reentered from a
callback) gets two `&mut SessionState` at once, a racing `RefCell` flag, or a box freed under a
running call: undefined behaviour, where the plan handle already returns a refusal (`RenderClaim`,
`ffi.rs` 222-252, #2213).

**Severity.** M. Reachable only through host misuse, but the failure is silent memory corruption
in the control plane rather than a result code, and the plan handle already shows the host-facing
shape the session lacks.

## Decision / model
- **One claim type.** `RenderClaim` becomes `HandleClaim`, the RAII claim on a handle's claim
  word: one `compare_exchange(0, 1, Acquire, Relaxed)` to claim, one `Release` store of 0 in
  `Drop`, on every exit, an unwind included. It never waits, spins or makes a syscall. `Plan`'s
  `render_busy` and the new `Session::busy` are both its word, and the word is loom's `AtomicU32`
  under `--cfg loom` (the `ModelledWord` pattern of `crates/host-core/src/spectrum.rs`).
- **Every session entry claims, right after the handle-kind check:** the twelve control entries
  above (through their five shared bodies) and `last_error` on a session. The claim yields a
  `SessionCall` holding the guard and the two field projections (`&mut SessionState`,
  `&RefCell<FixedBytes>`); `ffi.rs` no longer forms a reference to the whole `Session`, so a
  refused call's read of the word never aliases the holder's exclusive borrow (the `Plan`
  projection rule).
- **A refused call returns a new result code, `MISO_ENGINE_V1_BUSY` (9).** It touches no session
  state, writes no diagnostic and leaves every out pointer untouched. Why not an existing code
  (#2213's root ruling 2 kept `RENDER_REJECTED` for the plan): the plan's diagnostic is one atomic
  word the loser can write, but the session's diagnostic is the holder's `RefCell`, so a refused
  session call cannot set one, and an existing code such as `BACKPRESSURE` would leave
  `last_error` naming some other failure. The ABI growth rule (`docs/C_ABI_V1_QUALIFICATION.md`,
  #1317 D15-3) adds a result code without a feature bit; a host treats a code it does not know as
  a failure. Ruled at review: keep `BUSY` (Verdict below).
- **Destroy claims too.** `session_destroy` claims the word and frees the session holding it. One
  that finds it claimed is refused: it frees nothing and returns, because freeing would free state
  under the running call. The session stays live and valid, the call in flight and every later
  call proceed normally, and a later quiescent destroy frees it; a host that never destroys it
  again leaks it. The other half of the destroy rule (no call may start after a successful
  destroy) stays the host's: freed memory carries no claim.
- **Retained bytes.** The word grows `size_of::<Session>()` from 11,224 to 11,232 bytes, which
  `CapiAdapter::ADAPTER_ALLOCATIONS` charges: +8 bytes of `capi_retained_bytes` per session; a
  caller whose cap was exact must raise it.
- **Render and session calls may overlap; that is the design.** The `Session` and the `Plan` are
  separate handles with disjoint state, joined only by the engine's lock-free exchange (rings,
  plan exchange, seqlocks), so no claim is needed between a render call and a session call, and
  nothing here takes a claim against the plan.

## Non-goals
- No change to the plan's busy refusal (`RENDER_REJECTED`, `render.busy`), to the render path's
  arithmetic, or to `hosts/host-web`.
- Follow-ups, the same class on other handles, not this issue (filed in #2258 by the
  coordinator):
  - `plan_destroy` against an in-flight render-side call (`render_f32_planar`,
    `plan_prepare_seek`): it frees the plan without taking `render_busy`.
  - The engine handle's `RefCell` diagnostic under concurrent `compile_session` (and
    `last_error`) calls on one engine.

## Gates
- A unit test, red on base: with the session's claim word held, every session entry returns
  `BUSY`, changes no state (transaction snapshot, diagnostic, out pointers) and runs no service
  step, also with a null or over-long source ID (the claim precedes the source-ID check's
  diagnostic write); `session_destroy` frees nothing; released, the same session serves calls and
  a quiescent destroy frees it.
- `handle_claim_loom`: two threads race to claim one word, one leaving by an early error return
  and one by an unwind; in every interleaving at most one holds it (a `loom::cell::UnsafeCell`
  access under the claim), at least one wins, and the word ends released. A CI leg in
  `.github/workflows/qualification.yml`.
- The header's thread-ownership block, the result-code list, `abi_smoke.c` and
  `docs/C_ABI_V1_QUALIFICATION.md` state the refusal.
- No rendered bit moves: a one-time comparison of rendered output at base and head, as evidence.
- `cargo test --locked -p capi`; clippy `-D warnings`; `cargo fmt --check`;
  `bash scripts/check-workspace-policy.sh`; `bash scripts/check-capi-abi.sh`;
  `cargo run --release -p realtime-policy`.

## Evidence (attempt 1)

**Beyond `ffi.rs` and `abi.rs`.** `crates/capi/Cargo.toml` names `loom = "=0.7.2"` under
`[target.'cfg(loom)'.dependencies]` (one `Cargo.lock` line), because the library's `ClaimWord`
names loom under that cfg; a cfg dependency also reaches the lib tests, so no dev-dependency is
added. The header gains `MISO_ENGINE_V1_BUSY` and the refusal in its thread-ownership block,
`abi_smoke.c` pins `BUSY == 9`, `abi.rs`'s frozen result-code test lists it, and
`docs/C_ABI_V1_QUALIFICATION.md` records the amendment. `.github/workflows/qualification.yml` gains
the loom leg in `test-release`, in the **debug** profile: capi's lib tests turn on the
`test-support` features of engine, lane, graph and builtins, so a release build rebuilds nearly the
whole graph in release (the capi test crate alone took 2:01 on 32 cores, loaded), while the debug
build of the whole graph under the leg's flags took 22 s cold; the model runs in 0.00 s in both.
The job's timeout is re-derived (28 to 31 minutes; to be re-derived from the first `main` run).

**Red on base.** `ffi::tests::a_session_call_while_another_holds_the_session_is_refused_not_raced`
run on base `76ff20b54` with only `RESULT_BUSY` and the test hook added (the hook a no-op, since
base has no claim word), so every call reaches the "held" session:
```
assertion `left == right` failed
  left: [0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 0, 0]
 right: [9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9]
```
Every call ran: the submit was accepted, the seeks, the service step, the declaration, the dequeue
and the diagnostic copy all reached the state. At head it passes.

**Loom.** `ffi::tests::handle_claim_loom::handle_claim_loom` passes at head under the leg's flags
(every interleaving explored).

**Mutations (each reverted; red in every case).**
- Release store removed from `HandleClaim::drop`: loom red, `a claim was not released`.
- Release skipped while unwinding (`if !std::thread::panicking()`): loom red, `a claim was not
  released`. This is the unwind leg's own catch.
- Claim's success ordering `Acquire` weakened to `Relaxed`: loom red, `Causality violation:
  Concurrent write accesses to UnsafeCell`.
- Compare-exchange replaced by a separate load and store: loom red, the same causality violation.
- Release store's ordering `Release` weakened to `Relaxed` (the reviewer's mutation, confirmed
  here): loom red, `Causality violation: Concurrent write accesses to UnsafeCell`.
- `miso_engine_v1_service` without the claim: unit test red, `left: [9, 9, 9, 9, 9, 9, 9, 0, ...]`.
- `last_error` on a session without the claim: unit test red, the thirteenth result `0`.
- `session_destroy` without the claim: unit test red, `current_provider_disposed: 1` and
  `replay_current_disposed: 1` against the expected zero counters (the held session was freed).
- `borrowed_source_id` moved before `claim_session` in `source_seek_entry` (minor 1 of the review,
  green on the attempt-1 test): the folded source-ID rows are red, `left: [9, 1, 9]` (the seek ran
  the source-ID check and wrote `source.id.invalid` into the holder's diagnostic).

**Test value.**
- Unit test: red if any session entry point, `last_error` on a session or `session_destroy`
  reaches the session without claiming it, or if a refused call writes the state, the diagnostic
  or an out pointer. No existing test holds a session's claim; before this issue no session call
  could be refused for concurrency at all.
- Loom model: red if the claim lets two holders reach the guarded state (a non-atomic or
  unordered claim), refuses both of two racing calls, or leaves the word held after an early
  return or an unwind. Only a two-thread schedule reaches the ordering defects; the existing
  `a_render_side_call_during_render_is_refused_not_raced` holds the word from one thread and
  cannot see them.

**No rendered bit moved.** The render path's only change is the claim type's name and its word's
type alias (`AtomicU32` outside loom). Two one-time comparisons, never committed:
- `audit capi` (release, 100,000 render calls, every one through the claim): `pcm_digest`
  `30dcfd806e7ba058` at base `76ff20b54` and at head, 0 violations in both.
- A scratch debug test rendering 64 blocks of the nine-track fixture fed with a sine pair, FNV over
  every result code and output bit: `7b6a72dd2cba099b` at base and at head.

**Gates (head).**
- `cargo test --locked -p capi`: 190 lib tests, the nine integration binaries and the doc tests
  pass.
- `cargo clippy --locked -p capi --all-targets --all-features -- -D warnings`: clean. The same
  clippy of the lib tests under the loom leg's flags: clean.
- `cargo fmt --check`: clean.
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
- `bash scripts/check-capi-abi.sh`: `C ABI check: ok (x86_64-unknown-linux-gnu, shared and static
  linkage)`.
- `cargo run --locked --release -p realtime-policy`: `realtime policy: ok (292 marked regions in
  46 files)`.
- `audit capi`: `total_violations: 0`, 0 allocations, 0 syscalls.

**Judgements the brief asked for.**
- Every session entry claims: the twelve control entries (through their five shared bodies), the
  session branch of `last_error`, and `session_destroy`. The engine handle and the plan's destroy
  do not (non-goals above).
- A render call and a session call overlapping is the design and needs no claim: they own
  different handles, `Plan`'s render state and `Session`'s control state, joined only by the
  engine's lock-free exchange. Nothing here claims against the plan.
- The two open questions were ruled at review (Verdict below).

## Verdict (attempt 1)
**PASS**, with five minor findings folded on the branch. The reviewer's rulings on the two open
questions:
- **Keep `MISO_ENGINE_V1_BUSY`.** #2213's no-new-code ruling rested on the plan's diagnostic being
  one atomic word a refused call can write. The session's diagnostic is the holder's `RefCell`,
  which a refused call cannot write, so every existing code would mislead; the ABI growth rule
  (#1317 D15-3) admits a new result code without a feature bit.
- **Keep refuse-and-leak on a held destroy.** It is the only memory-safe option: freeing would free
  state under the running call, and deferring the free to the holder would free the handle from
  inside its own call.

The minors, folded:
1. *Test gap at the source-ID sites*: the test never reached the pre-claim diagnostic write, so
   moving `borrowed_source_id` before the claim stayed green. Folded: held calls with a null and a
   128-byte source ID to `source_submit_planar_f32`, `source_seek` and `source_seek_report`, each
   `BUSY` with the diagnostic unchanged; mutation-checked (above).
2. *Retained bytes*: `size_of::<Session>()` grew from 11,224 to 11,232 bytes (11,232 confirmed at
   head), +8 bytes of `capi_retained_bytes` per session. Folded into the decision record and
   `docs/C_ABI_V1_QUALIFICATION.md`.
3. *Loom gate wording*: the gate claimed "a refusal only happens while the other holds it", which
   the model does not assert. Folded: the gate now states what is asserted, and the reviewer's
   `Release`-to-`Relaxed` mutation is in the mutation list.
4. *Destroy wording*: folded into the header, the `session_destroy` rustdoc, the qualification doc
   and the decision record: refused, the session stays live and valid, the call in flight and later
   calls proceed normally, and a later quiescent destroy frees it.
5. *Deferred follow-ups*: kept in the non-goals; the coordinator files them in #2258, with the
   reviewer's recommendation that HC-8's fix moves the plan's busy refusal to `BUSY` with no
   diagnostic write.

Gates re-run after the fold, all green: `cargo test --locked -p capi` (190 lib tests, nine
integration binaries, doc tests); the loom leg; clippy `-D warnings` normally (`--all-targets
--all-features`) and under the loom flags; `cargo fmt --check`; `RUSTDOCFLAGS='-D warnings' cargo
doc --locked -p capi --no-deps`; `bash scripts/check-capi-abi.sh`; `bash
scripts/check-workspace-policy.sh`.
