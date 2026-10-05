# Reserve the C ABI's automation span limit

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answer A9, under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`. Batch P2, after batch R3.

## Product outcome

A C ABI caller no longer chooses an automation span limit: the engine sizes every effect window
from its own plan (#1306), and the protocol has no automation queue (drafts 21a-21c). The word at offset
8 of `miso_engine_v1_compile_limits` becomes a reserved word that must be zero, like `reserved0`
beside it. The struct stays 208 bytes, no symbol or `ABI_VERSION` changes, and a nonzero word is
refused before the call reads anything else, so no session is created.

## Context

- **The field.** `uint32_t maximum_automation_spans_per_block` at offset 8, then `reserved0`
  (`crates/capi/include/miso_engine_v1.h:163-196`, field at `:166`); the Rust mirror is
  `CompileLimits` (`crates/capi/src/abi.rs:88-96` and on), size and offsets asserted at
  `:445-469` and in C at `crates/capi/tests/c/abi_smoke.c:26`, `:44-48`, compiled by
  `scripts/check-capi-abi.sh` (`:143-144`).
- **Its readers today.** `all_limits_nonzero` requires it nonzero
  (`crates/capi/src/runtime/compile.rs:482-508`, term at `:483`); `protocol_queue_config` makes it
  the protocol's `per_block_automation_density` (`:130-133`); `prepare_caps` passes it to effect
  preparation (`:528`). #1306 D3 removes the preparation read and keeps the density read "until
  slice 22 reserves the field" (amendment to #1306 D3); draft 21c D4 removes the density read. After
  both, `all_limits_nonzero` is its only reader: a validated value nothing uses.
- **The reserved-field rule.** `limits_are_valid` (`compile.rs:510-515`) requires `reserved0 == 0`
  and `reserved == [0; 2]`; `miso_engine_v1_compile_session` returns
  `MISO_ENGINE_V1_INVALID_ARGUMENT` on failure before it reads the document or writes diagnostics
  (`crates/capi/src/ffi.rs:340-342`; result code at `miso_engine_v1.h:114`). No test sets a
  reserved compile-limit word nonzero today.
- **The precedent, reversed.** #1206 and #1243 named the former `reserved[0]` and `reserved[1]`
  in place, with a header comment "Formerly reserved[0]; the layout is unchanged"
  (`miso_engine_v1.h:188-194`; `docs/C_ABI_V1_QUALIFICATION.md:83-105`). D15-12's growth rule
  gives a feature bit per addition (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md:383-384`);
  this is a removal of a role, not an addition.
- **`ControlLimits`** (#1309 D3) mirrors every numeric field of `CompileLimits` without the
  reserved words, so it loses this field too.
- **Literals that set the word** (each sets 128 or 4): `crates/capi/src/ffi.rs:1315`,
  `crates/capi/src/runtime/tests.rs:22`, `crates/capi/tests/plan_swap_race.rs:37`,
  `crates/capi/tests/resource_lifecycle.rs:164`, `:3036`, `tools/audit/src/capi.rs:583`;
  `crates/capi/src/runtime/live_tests.rs:2423-2426`, `:2809-2812` (S = 4; #1306 D5 records that
  neither test survives #1345). `resource_lifecycle.rs:524-529` (`host_caps`) mirrors
  `prepare_caps` and loses its line with #1306.
- **Docs.** `docs/C_ABI_V1_QUALIFICATION.md:346` and `:360-363` describe S's role in the
  reference figures and the window formula (#1306 D6 rewrites the formula).

## Decisions frozen for this slice

- **D1. Reserved, renamed.** The field becomes `uint32_t reserved1; /* Formerly
  maximum_automation_spans_per_block; must be zero in ABI V1; the layout is unchanged. */` in the
  header, and `reserved1: u32` in `CompileLimits` with the same doc. The rename makes a caller that
  still sets the old field fail to compile, the clearest signal; the offset, size and alignment do
  not change, and the size and offset assertions stay as they are.
- **D2. The existing rule refuses it.** `limits_are_valid` adds `limits.reserved1 == 0` beside
  `reserved0`; `all_limits_nonzero` drops the term. A nonzero word returns
  `MISO_ENGINE_V1_INVALID_ARGUMENT`, writes no diagnostics and creates no session or plan. No new
  code or diagnostic.
- **D3. Every literal sets zero.** Each literal above sets `reserved1: 0`; `ControlLimits` and the
  conversion lose the field.
- **D4. Docs.** `docs/C_ABI_V1_QUALIFICATION.md` gains a paragraph beside `:83-105`: the word at
  offset 8 is reserved since this slice, why (#1306 sizes windows from the plan, drafts 21a-21c retired
  the queue), and that a library older than this slice refuses a zero word with
  `RESULT_INVALID_ARGUMENT` while a caller written before it must now pass zero. `:346` drops "S =
  128".
- **D5. No feature bit.** The change removes a role; D15-12's rule is for additions. A directly
  linked caller and the library move in lockstep, as every prelaunch V1 amendment requires.

## Deliverables

1. D1-D3 in `crates/capi`, `crates/control-plane` (after #1309) and the literals.
2. D4.
3. The gate tests.

## Authorized paths

- `crates/capi/include/miso_engine_v1.h`, `crates/capi/src/{abi,ffi}.rs`,
  `crates/capi/src/runtime/` (or their `crates/control-plane/src/` successors)
- `crates/capi/tests/plan_swap_race.rs`, `crates/capi/tests/resource_lifecycle.rs` (literals only)
- `tools/audit/src/capi.rs` (the literal only)
- `docs/C_ABI_V1_QUALIFICATION.md`

## Non-goals

- Any other compile limit. The browser's response-request field of the same name
  (`hosts/host-web/src/lib.rs:541`) and `EffectCompileCaps` / `PrepareEffectLimits` (effect
  preparation's own ceiling, #1306 D3).
- The effect contract's span vocabulary (finding F6).

## Hazards

- **Every existing C caller breaks at runtime** if it is not rebuilt: it passed a nonzero S
  because the field was required nonzero. The rename turns that into a compile error for any
  caller rebuilt against the new header. Apps and SDK move in lockstep before launch.
- **Order.** Merging before #1306 or draft 21c would remove a value something still reads. The
  slice lands in batch P2, after batch P1 (draft 21c) and batch R3 (#1306). Until then `main`
  validates a value that nothing reads; that changes no rendered bit and costs nothing at render.

## Objective gates

1. **Refused** (`crates/capi/src/ffi.rs` tests, new): `miso_engine_v1_compile_session` with
   `reserved1 = 1` returns `MISO_ENGINE_V1_INVALID_ARGUMENT`; the session and plan out-pointers are
   untouched and the diagnostics `BytesOut` is not written. The same call with `reserved1 = 0`
   compiles. A second case sets `reserved0 = 1` and gets the same result (no test covers it today).
2. **Layout** (`bash scripts/check-capi-abi.sh` and `bash scripts/check-capi-abi.sh --self-test`):
   the header compiles in C and C++ with the unchanged size and offsets.
3. **No rendered bit moves.** `cargo build --locked --release -p audit -p bench -p capi -p
   session-validator`, then `./target/release/audit capi`, shows the same `pcm_digest` at base and
   head (PR evidence).
4. **Commands:**
   - `cargo test --locked -p capi`, `cargo test --locked -p control-plane --features test-support`
     (after #1309)
   - the workspace debug leg (`test-debug-a` in `.github/workflows/qualification.yml`)
   - `bash scripts/check-workspace-policy.sh`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`

## Test value

- Gate 1 turns red if the word is still read as a limit (a nonzero value accepted), or if the
  refusal creates a session or writes diagnostics first; its `reserved0` case turns red if the
  reserved rule loses a word. No test sets a reserved compile-limit word today.
- Gate 2 turns red if the rename moves an offset.

## Dependencies

Batch P2, after batch R3 (#1306). Direct dependencies:

- Draft 21c *Delete the protocol automation queue, its records and counters* (removes the density
  read).
- *Size each effect's automation span window from the producers its plan has* (#1306) (removes the
  preparation read).

*Extract the C ABI control plane into a portable crate both hosts call* (#1309), for
`ControlLimits`, arrives through drafts 21a-21c.
