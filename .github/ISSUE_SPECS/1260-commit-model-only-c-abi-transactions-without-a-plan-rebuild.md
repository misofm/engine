# Commit model-only C ABI transactions without a plan rebuild

Slice of the umbrella *Deliver value-only fader, mute and pan transactions to the running C ABI
plan through the live console lanes* (#1053). It closes follow-up F8 of decision 14
(`docs/rulings/live-update-versus-rebuild-2026-10-04.md`, #1259). Anchors verified on `main` at
`54b0a1bf8`; re-verify them after #1257 lands.

## Product outcome

A C ABI transaction that changes only fields no prepared plan reads commits without a plan
rebuild. It needs no live record either: the running plan, its source rings and its effect state
continue untouched. The fields are:

- the session ID;
- the render profile's ID;
- the output profile's ID;
- the stored automation table, while nothing renders it.

Today each such transaction is a full rebuild, with a source-ring reset and a silent block.

## Context (verified at `54b0a1bf8`)

- **The edits.** `SetSessionId` (`0x0001`), `SetRenderProfile` (`0x0004`) and `SetOutputProfile`
  (`0x0005`) replace `session_id`, `render_profile` and `output_profile`
  (`crates/protocol/src/model.rs:534-544`). The automation edits are `0x0600`-`0x0603`.
- **The profiles.** `RenderProfile { id, mode }`, where `mode` has one value; `OutputProfile { id,
  channels, sample_format }` (`crates/session/src/model.rs:124-158`).
- **What preparation reads.** It reads `output_profile.channels` (`crates/host-core/src/shape.rs:62`).
  It reads neither profile ID and not the session ID. Nothing in builtins-compiler, graph-compiler
  or effect-compiler reads `automation`; their only uses clear it in tests. Stored automation
  renders nothing on any host (#1058, `docs/CONTROL_PROTOCOL_SEMANTICS.md:15`).
- **The classifier (#1255).** It masks each track's `fader` and `matrix_or_pan` and compares the
  canonical JSON bytes. A delta whose masked model equals the current one is live, and an empty
  delta commits with zero records through #1257's `commit_live`.
- **`SetSessionId` is the capi tests' rebuild trigger today.** It drives a structural replacement
  in these tests:
  - `crates/capi/src/runtime/tests.rs:627`, `:862`, `:1125`, `:1260`, `:1397`, `:1475`, `:1545`,
    `:1792`, `:1876` and `:1925`;
  - `crates/capi/src/ffi.rs:1637`;
  - the `command()` helper of `crates/capi/tests/resource_lifecycle.rs` (`:192-209`). That helper
    feeds the double-live oracle (`:1263`), the race tests (`:1541`, `:1770`) and `lifecycle()`
    (`:241-244`), which `exported_c_candidates_replay_render_and_both_destroy_orders_balance_exactly`
    (`:387`) uses. `lifecycle()` relies on a pending candidate refusing the second `SetSessionId`,
    so it needs a rebuild trigger too.

  The protocol's own tests use it too, but they test the controller and do not move.

## Decisions

- **D1. The mask grows.** `classify_live_delta` also copies `current`'s `session_id`,
  `render_profile.id`, `output_profile.id` and `automation` into the masked model before it
  compares. Every other field of the two profiles is still compared.
- **D2. The automation coupling.** Masking `automation` is correct only while no host renders
  stored automation. The function's documentation says so and names #1058. #1058's spec gains a
  coordination note (D4).
- **D3. The tests keep their rebuilds.** Each capi test that needs a rebuild changes its trigger
  from `SetSessionId` to a structural edit that renders identically. Use a `SetSourceContent` that
  changes only a source's `content` string, keeping its length where a test compares byte counts.
  A test that only needs a committed transaction, with no swap (for example the pinned response
  and event vectors), keeps `SetSessionId`. Its bytes are unchanged, because a live commit emits
  the same response and the same `SESSION_COMMITTED`.
- **D4. #1058's spec.** Append one coordination bullet to
  `.github/ISSUE_SPECS/1058-research-render-stored-session-automation-in-the-engine-identically-on-every-pla.md`,
  and sync it to GitHub: the C ABI's live classifier treats automation edits as model-only, so the
  first issue that renders stored automation must remove `automation` from that mask.

## Authorized paths

- `crates/host-core/src/live_delta.rs` (the mask only) and `crates/host-core/tests/live_delta.rs`.
- `crates/capi/src/runtime/tests.rs`, `crates/capi/src/runtime/live_tests.rs`,
  `crates/capi/src/ffi.rs` (its test module only) and `crates/capi/tests/resource_lifecycle.rs`.
- `.github/ISSUE_SPECS/1058-*.md` (one bullet).
- `docs/C_ABI_V1_QUALIFICATION.md` (the list of value-only transactions).
- This spec.

## Non-goals

- No change to the sample rate, the quantum, the output channel count or the sample format. They
  stay structural, or are refused by validation.
- No rendering of stored automation.

## Objective gates

Run every command from the repository root.

1. **The classifier.** New cases in `crates/host-core/tests/live_delta.rs`. #1255's gate 1(k)
   case for a `session_id` change (`Structure`) is superseded: invert it in the same PR.
   - a session ID change, a render-profile ID change, an output-profile ID change and an
     automation upsert each give `Ok` with no entries;
   - the same edits together with a fader change give exactly the fader records;
   - an output-profile `channels` change gives `Structure`.

   *Test value: it turns red if a model-only field is still compared, or if a profile field that
   preparation reads is masked.*
2. **The C ABI.** New cases in `crates/capi/src/runtime/live_tests.rs`. Each of the four edits,
   through `miso_engine_v1_submit_command`, does all of these:
   - returns `OK` and raises the revision by one;
   - emits one `SESSION_COMMITTED`;
   - leaves `providers.epoch` and `pending_providers` unchanged;
   - shows in the canonical snapshot;
   - lets the host keep submitting with no seek;
   - keeps the output bit-identical to an uninterrupted render of the same session.

   *Test value: it turns red if a model-only edit still rebuilds the plan.*
3. **Rebuilds are still tested.** Every migrated test passes, and still observes the swap it
   observed before: the same epoch, publication and retirement counts.
   *Test value: it turns red if a migrated trigger no longer rebuilds, which would leave the old
   assertions unexercised.*
4. **Nothing else changes.**
   - `cargo test --locked -p capi`
   - `cargo test --locked -p host-core --features control-provider,test-support --test live_delta`
   - The workspace test command:
     `cargo test --locked --workspace --all-targets --exclude lane --exclude math --exclude effect-runtime --exclude delay --exclude compressor --exclude multiband-compressor --exclude gate-expander --exclude true-peak-limiter --exclude transient-shaper --exclude soft-clip --exclude parametric-eq --exclude builtins --exclude dsp-reference --exclude conformance --exclude audit --exclude bench --exclude console-workload --exclude wasm-gates --exclude wasm-gate-guest --exclude wasm-gate-corpus --features builtins-compiler/test-support,graph/test-support,host-web/test-support,host-core/test-support,effect-compiler/test-support,protocol/test-support,engine/realtime-audit`
   - `cargo build --locked --release -p audit -p bench -p capi -p session-validator`, then
     `./target/release/audit capi` reports zero allocations, frees, locks and syscalls.
   - `bash scripts/check-capi-abi.sh && bash scripts/check-capi-abi.sh --self-test`
5. **Workspace and policy.**
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
   - `for x in host-core realtime workspace; do bash scripts/check-$x-policy.sh && bash scripts/test-$x-policy.sh || exit 1; done`
   - `bash scripts/check-cross-targets.sh`
   - 4-lane (NEON): `bash scripts/run-aarch64-tests.sh debug` is CI-only here; record it as not
     run locally.

## Evidence

- The output of every gate command, from the head commit.
- Each new test's name, with its one-sentence test value.
- The list of migrated tests, each with its new trigger.

## Dependencies

- *Apply value-only track fader, mute and pan transactions to the running C ABI plan* (#1257)
- *Qualify live C ABI edits against a concurrently rendering plan* (#1258). It edits
  `resource_lifecycle.rs` too; land after it.

## Standing rules for the implementer

- Work from this body. Change nothing outside the authorized paths.
- A superseded test is deleted or migrated in the same PR, never weakened.
- A test that greps source or prose is refused.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).
