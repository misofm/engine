# Ship the session console and inserts in the SDK

Slice S1d of *Console strip: session-level console effects with per-track inserts* (owner
decision 12, `docs/rulings/engine-footprint-2026-09-29.md`; Sol's M5 in
`docs/handoffs/console-strip-2026-09-29/VERIFY.md`, commit `03aceb94`).

## Problem

After S1a-S1c the engine speaks the console vocabulary, but the SDK still builds, types, writes and
addresses per-track `simd1`, `dynamic` and `simd2` racks:
- `sdk/src/core/types.ts`, `core/session.ts`, `core/writer.ts` and `internal/session-json.ts`;
- the live-controls module (`core/live-controls.ts` after S1r), with its `effect("simd1", ...)`
  addressing and its `{ simd1: 0, dynamic: 1, simd2: 2 }` rack map;
- `core/live-response.ts`, `core/observation.ts` and `core/response.ts`;
- the CLI's `cli/session-request.ts`.

The app cannot move EQ -> compressor into `console.pre_insert` until the SDK can express it.

## Smallest closable slice

1. **Builder and types.**
   - The session-level `console { pre_insert, post_insert }` with slot declarations (`slot`,
     `identity`, `quality`, `link_mode`).
   - Per-track `console` entries (`slot`, `bypass`, `params`) in slot order.
   - Per-track `inserts`.
   - The renamed tap tokens.
   - The builder refuses what the engine refuses: missing, duplicate or misordered entries, an
     ineligible effect, and a console sidechain. It refuses them with the engine's diagnostic
     codes, so an agent sees one vocabulary.
2. **The canonical writer.** It emits S1a's field order, byte for byte.
3. **Live controls.** Console slots are addressed by slot ID (resolved to S1c's slot index) and
   inserts by ID or index. The rack map uses S1c's codes.
4. **Live response, observation and spectrum** decode S1c's encodings and target names.
5. **The CLI** (`enginectl` session requests) accepts `console` and `inserts` and refuses `simd1`,
   `dynamic` and `simd2`.
6. **The author-session skill** (`.claude/skills/author-session`) teaches the new shape.
7. **The app handoff note** gives the old-to-new map of every public SDK name, the new session
   shape, and the migration of the app's EQ -> compressor. That migration is: every track carries
   the pair today, and unselected tracks are `bypass`; under the console, the pair becomes two
   `pre_insert` slots, and each track's entry keeps its bypass (decision 12, "The app").

Authorized paths: `sdk/**` (including the generated files, regenerated only by
`sdk/codegen/generate.mjs`), `.claude/skills/author-session/**`, the host-web SDK-driven
qualification entry points, and this spec.

## Owner decisions that bind this slice

Decision 12's "Shape", "Wire identity" (the app updates in lockstep) and "The app".

## Dependencies

- *Add the session console and per-track inserts to the session schema* (S1a).
- *Address console slots and inserts in live control* (S1c, #1096), which itself follows S1b and
  S1r.

S1d closes batch C3 (S1r, S1a, S1b, S1c, S1d). The batch is pushed once, after S1d's gates pass.

## Objective gates

1. `bash scripts/check-sdk-generated.sh`, `bash scripts/check-sdk-types.sh`,
   `bash scripts/check-sdk-headless.sh`, `bash scripts/sdk-package.sh check` and the SDK test
   suites pass. At S1a's C3 rebase, 86 headless and 4 package tests failed, all because the SDK
   still sends the old schema (#1093 C3 rebase verdict, low 3); every one must pass.
2. Documents from the SDK builder parse under S1a's grammar and round-trip byte for byte through
   the engine's canonical writer. This holds for an empty console, both sections, empty inserts and
   the app shape.
3. Each builder refusal has an eval that asserts the engine's code.
4. The SDK-driven host-web qualification passes in the three browsers in CI mode, which S1c
   deferred. That includes a live bypass toggle on a console slot and on an insert.
5. The author-session skill's worked session parses, compiles and prepares through the real
   pipeline.

## Standing rules for the implementer

- Work only from this body, the umbrella issue and decision 12. Read the cited code first.
- Class A: every gate that says "bit-identical" is a hard stop, not a tolerance. NaNs fold to one
  value (decision 10).
- Render stays allocation-, lock- and syscall-free. Only `crates/lane` names `wide` or intrinsics.
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features --
  -D warnings` and the focused tests before every checkpoint.
- Every new test names the plausible defect that turns it red (AGENTS.md "Test value"). A
  one-time "no bit moved" comparison against the pre-change base is PR evidence, not a committed
  test.
