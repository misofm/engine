# C ABI: a refused render-side call overwrites the plan holder's last error

Found by the implicit-state-machine audit of 2026-11-06 (tracker #2258, item HC-8), and recommended
by the #2276 reviewer's ruling (its Verdict, minor 5).

## Mission
A render-side call refused because another holds the plan must leave the plan's diagnostic as the
holder left it, and the refusal must use the code that means "another call holds this handle" on
every handle: `MISO_ENGINE_V1_BUSY` (9, added by #2276).

## Verified defect (base `875a4c17e`)
`crates/capi/src/ffi.rs`:
- `claim_render_side` (243-257) stores `plan_error::BUSY` (`render.busy`) into the plan's
  `last_error` word whenever `HandleClaim::try_claim` fails. Its two callers,
  `miso_engine_v1_render_f32_planar` (1356-1358) and `miso_engine_v1_plan_prepare_seek`
  (1477-1479), then return `RESULT_RENDER_REJECTED`.
- The word is otherwise written only by the call that holds the claim (every other store in both
  entries is under `_claim`). So a losing call overwrites the holder's sticky diagnostic (a
  `render.time.discontinuity`, say), and the holder's next store (a success's `NONE`) overwrites
  the loser's: last-writer-wins on one word with two owners.
- The header (`crates/capi/include/miso_engine_v1.h` 41-45) documents the race instead of removing
  it: "The result code is authoritative: the winner may overwrite the plan's last error."
  `docs/C_ABI_V1_QUALIFICATION.md` (423-425) says the same.

**Severity.** L. Reachable only through host misuse (two render-side calls at once); the failure is
a misleading diagnostic, not memory corruption, since the claim already excludes the state.

**Where #2213's ruling is recorded.** #2213's root ruling 2 (2026-10-30), "A busy refusal is
`RESULT_RENDER_REJECTED` with `render.busy`. No new result code", lives in the retired spec
`.github/ISSUE_SPECS/2213-c-abi-a-stopped-host-cannot-refill-a-full-ring-before-restart-after-a-seek-resid.md`
(git history, e.g. `51b7d421d`), and its effects in the header's thread-ownership block and
plan-diagnostic list, `docs/C_ABI_V1_QUALIFICATION.md`, `runtime/error.rs` (`plan_error::BUSY`),
the rustdoc of both entries and `abi.rs`'s `Plan`, and the frozen test
`session_seek::a_render_side_call_during_render_is_refused_not_raced` in
`crates/capi/src/runtime/tests.rs`. No docs ruling under `docs/rulings/`, no SDK source, no
`hosts/host-web` code and no C/C++ smoke test matches `render.busy`; `abi_smoke.c` already pins
`BUSY == 9` (#2276). `sdk/README.md`'s "Upgrading from 0.10.3" and the closed release spec 2225
mention `render.busy` as 0.10.4 history and stay as written.

## Decision / model
- **Amend #2213's root ruling 2.** A refused `render_f32_planar` or `plan_prepare_seek` returns
  `RESULT_BUSY` and writes no diagnostic. The ruling chose an existing code because the plan's
  diagnostic was one atomic word a refused call *could* write; #2276 then added `BUSY` for the
  session, whose diagnostic a refused call cannot write. With `BUSY` in the ABI, the plan has the
  same reason not to write: the word belongs to the holder. One code now has one meaning across
  plan and session.
- **The ownership is now explicit.** `last_error`'s writes belong to the call that holds
  `render_busy`, as `state` does; reads stay any-thread. `claim_render_side` is just the claim.
- **`render.busy` (plan diagnostic 9) is retired**, its constant and its text arm removed. The
  header's plan-diagnostic list drops it and gains the one string it omitted,
  `render.fp_environment.invalid` (#146), so it names every rule.
- **ABI growth.** No symbol, struct, size, feature bit or `ABI_VERSION` change; the result code
  already exists. A host that matched `RENDER_REJECTED` for a concurrent render-side call now sees
  `BUSY`; only a host that breaks the plan's thread rule receives it.
- **The claim is unchanged** (same compare-exchange, same drop), so `handle_claim_loom` already
  covers it and is not extended.

## Non-goals
- `plan_destroy` against an in-flight render-side call (frees the plan without taking
  `render_busy`): a separate follow-up in #2258, a different outcome (memory safety, not the
  diagnostic).
- The engine handle's `RefCell` diagnostic under concurrent calls (the other #2276 follow-up).
- The render path's arithmetic; `hosts/host-web` (its render path has no claim word).

## Release note
If this merges before release #2253 (0.10.5) pins the engine, that release's "Upgrading from
0.10.4" section in `sdk/README.md` must carry this bullet:

> Changed: `miso_engine_v1_render_f32_planar` and `miso_engine_v1_plan_prepare_seek` refuse a
> concurrent call on the same plan with `MISO_ENGINE_V1_BUSY` and write no diagnostic (issue
> #2293). In 0.10.4 they returned `MISO_ENGINE_V1_RENDER_REJECTED` with `render.busy`, which is
> retired. The plan's last error now stays the one the last call that held the plan left. No
> symbol, struct, size, feature bit or `ABI_VERSION` changes.

#2276's added result code needs the same treatment: the same section must also carry its "Added:
`MISO_ENGINE_V1_BUSY`" bullet (a session call made while another holds the session is refused
with it), which #2276's spec does not record.

## Gates
- `session_seek::a_render_side_call_during_render_is_refused_not_raced`, amended and red on base:
  after a refused render leaves `render.time.discontinuity`, a held claim refuses both render-side
  calls with `BUSY` and the plan's diagnostic is unchanged; released, prepare and render succeed
  and clear it.
- No rendered bit moves: a one-time comparison at base and head, as evidence.
- `cargo test --locked -p capi`; clippy `-D warnings`; `cargo fmt --check`;
  `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps -p capi`;
  `bash scripts/check-workspace-policy.sh`; `bash scripts/check-capi-abi.sh`;
  `cargo run --locked --release -p realtime-policy`.

## Evidence (attempt 1)

**Files.** `crates/capi/src/ffi.rs` (`claim_render_side` is the claim alone; both entries return
`RESULT_BUSY`; rustdoc), `crates/capi/src/runtime/error.rs` (`plan_error::BUSY` and its text
removed), `crates/capi/src/abi.rs` (`Plan`'s field-ownership docs), the header's thread-ownership
block, plan-diagnostic list and `MISO_ENGINE_V1_BUSY` comment, `docs/C_ABI_V1_QUALIFICATION.md`
(the #2213 paragraph and a new #2293 amendment paragraph after #2276's), and the amended test.
`abi_smoke.c` and `abi.rs`'s frozen result-code test already pin `BUSY == 9`; no new test.

**Red on base** (`875a4c17e`, the amended test only):
```
assertion `left == right` failed
  left: 8
 right: 9
```
the held-claim render returned `RENDER_REJECTED`. With that test's two `BUSY` expectations set
back to `RENDER_REJECTED` (scratch only, to show the overwrite itself):
```
assertion `left == right` failed: a refused call leaves the holder's diagnostic
  left: "render.busy"            (bytes [114, 101, 110, 100, 101, 114, 46, 98, 117, 115, 121])
 right: "render.time.discontinuity"
```
so on base the losing call stored `render.busy` over the holder's sticky diagnostic. At head the
test passes.

**Mutations (each reverted; red in every case).**
- The diagnostic store restored in `claim_render_side` (with `PLAN_REJECTED`, since `BUSY` is
  gone): red, the diagnostic reads `render.plan.rejected`, not the holder's
  `render.time.discontinuity`.
- `render_f32_planar`'s refusal returning `RESULT_RENDER_REJECTED`: red, `left: 8, right: 9`.
- `plan_prepare_seek`'s refusal returning `RESULT_RENDER_REJECTED`: red, `left: 8, right: 9`.
- `plan_prepare_seek` without the claim: red, `left: 0, right: 9`.

**Test value.** Red if a refused render-side call writes the plan's diagnostic over the holder's
(the defect), returns any code but `BUSY`, or skips the claim. No other test holds the plan's claim
with a sticky diagnostic in place; the base version of this test only checked that the loser's own
`render.busy` was there, which is the defect.

**Loom.** Unchanged claim; `handle_claim_loom` passes under the CI leg's flags.

**No rendered bit moved.** The render path's change is the refusal branch only (one atomic store
removed, another return code). One-time comparison, never committed: `audit capi` (release,
100,000 render calls through the claim) `pcm_digest` `30dcfd806e7ba058` at base `875a4c17e`
and at head, `total_violations: 0`, 0 allocations, 0 syscalls in both.

**Gates (head).**
- `cargo test --locked -p capi --no-fail-fast`: 190 lib tests, the nine integration binaries and
  the doc tests pass. (A first full run went red once in `plan_swap_race`'s
  `a_two_strip_transaction_lands_in_one_block_and_the_watermark_names_it` on its timing floor
  (`vacuous: ...`) under a loaded machine; it touches no busy path and passed alone and on the
  full rerun.)
- `cargo clippy --locked -p capi --all-targets --all-features -- -D warnings`: clean.
- `cargo fmt --check`: clean.
- `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps -p capi`: clean.
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
- `bash scripts/check-capi-abi.sh`: `C ABI check: ok (x86_64-unknown-linux-gnu, shared and static
  linkage)`.
- `cargo run --locked --release -p realtime-policy`: `realtime policy: ok (292 marked regions in
  46 files)`.
- Loom leg `handle_claim_loom`: pass.

**Follow-ups (not this issue).** `plan_destroy` racing an in-flight render-side call (it frees
the plan without taking `render_busy`; refuse-and-leak as `session_destroy` does is the likely
shape); the engine handle's `RefCell` diagnostic under concurrent calls. The next release's SDK
upgrade notes should say a concurrent render-side call now returns `BUSY` instead of
`RENDER_REJECTED` with `render.busy`.

## Verdict (attempt 1)
**PASS**, with two minors folded on the branch. The reviewer confirmed that only the claim holder
writes the plan's diagnostic at head, and that the header's added `render.fp_environment.invalid`
is a correction of an omission, kept.

The minors, folded:
1. *Release note*: the "Release note" section above, with the 0.10.5 "Changed" bullet conditional
   on merging before #2253's pin, and #2276's "Added `BUSY`" bullet noted as needing the same.
2. *ABI disposition*: `docs/C_ABI_V1_QUALIFICATION.md`'s #2293 paragraph now says this is an
   in-place V1 amendment (prelaunch V1): a host built against 0.10.4's header sees `BUSY` as an
   unknown code, which the growth rule requires it to treat as a failure, the same disposition
   `RENDER_REJECTED` gave it, so no `ABI_VERSION` bump is needed.

The `plan_destroy` follow-up is filed by the coordinator.
