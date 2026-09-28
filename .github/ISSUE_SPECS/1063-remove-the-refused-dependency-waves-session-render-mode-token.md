# Remove the refused `dependency_waves` session render-mode token

Owner ruling (2026-09-28, `docs/rulings/engine-footprint-2026-09-28.md`, decision 8): remove it. The dependency-wave multicore scheduler was removed from the engine (AGENTS.md: reintroducing multicore render needs a new issue that re-earns it). The session grammar still knows the token and refuses it with a specific error (`crates/session/src/model.rs:97`; `docs/SESSION_SCHEMA_V1.md:57-59`; tests `crates/session/tests/render_mode_tiers.rs:59,72`). Source: the test-value audit, ruling R7, and its verification.

## Smallest closable slice

Remove the token from the session model, parser, canonical writer and protocol encoding, so a session naming it is refused as an unknown value; update `docs/SESSION_SCHEMA_V1.md`; delete or adapt the tests that exercised the known-but-refused behaviour; check the SDK (`sdk/`) and generated surfaces for the spelling.

## Objective gates

- A session naming `dependency_waves` is refused (unknown value) at parse, with a test; `single_thread` sessions and every fixture are unchanged.
- Protocol encoding and wire vectors stay consistent (no silent renumbering of other tokens; state what changed).
- All four targets build; console digests unchanged; clippy, fmt, the session and step-vocabulary policies and the SDK tests pass.
