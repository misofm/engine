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
  a failure. Open for root review: the alternative is `BACKPRESSURE` with no diagnostic.
- **Destroy claims too.** `session_destroy` claims the word and frees the session holding it; one
  that finds it claimed frees nothing and returns (the session stays live and is leaked, because
  freeing it would free state under the running call). The other half of the destroy rule (no call
  may start after destroy) stays the host's: freed memory carries no claim.
- **Render and session calls may overlap; that is the design.** The `Session` and the `Plan` are
  separate handles with disjoint state, joined only by the engine's lock-free exchange (rings,
  plan exchange, seqlocks), so no claim is needed between a render call and a session call, and
  nothing here takes a claim against the plan.

## Non-goals
- No change to the plan's busy refusal (`RENDER_REJECTED`, `render.busy`), to the render path's
  arithmetic, or to `hosts/host-web`.
- `plan_destroy` against an in-flight render-side call and the engine handle's `RefCell`
  diagnostic under concurrent `compile_session` calls are the same class on other handles; they are
  follow-ups, not this issue.

## Gates
- A unit test, red on base: with the session's claim word held, every session entry returns
  `BUSY`, changes no state (transaction snapshot, diagnostic, out pointers) and runs no service
  step; `session_destroy` frees nothing; released, the same session serves calls and is destroyed.
- `handle_claim_loom`: two threads race to claim one word; in every interleaving at most one holds
  it (a `loom::cell::UnsafeCell` access under the claim), at least one wins, a refusal only happens
  while the other holds it, and both an early error return and an unwind release it. A CI leg in
  `.github/workflows/qualification.yml`.
- The header's thread-ownership block, the result-code list, `abi_smoke.c` and
  `docs/C_ABI_V1_QUALIFICATION.md` state the refusal.
- No rendered bit moves: a one-time comparison of rendered output at base and head, as evidence.
- `cargo test --locked -p capi`; clippy `-D warnings`; `cargo fmt --check`;
  `bash scripts/check-workspace-policy.sh`; `bash scripts/check-capi-abi.sh`;
  `cargo run --release -p realtime-policy`.

## Evidence (attempt 1)
Pending.
