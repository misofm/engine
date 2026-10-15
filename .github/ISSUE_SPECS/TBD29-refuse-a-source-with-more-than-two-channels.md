# Refuse a source with more than two channels

Owner answer (2026-10-15): sources are mono or stereo only. Decision 17 programme. Code anchors verified on `4be10cd67` (PR #1680). Issue number: assigned at filing (written here as TBD29). Feature sprint mode (owner, 2026-10-13): one implementer, normal tests, a batch review.

## Product outcome

A session whose source declares more than 2 channels is refused at session load with a typed
error, the same on both hosts, and the browser's source staging is sized for 2 channels (about
1 KB instead of the 130 KB the grammar's 255-channel domain needs).

## Context

- `session::SOURCE_CHANNELS_MAXIMUM` is `u8::MAX` (`crates/session/src/model.rs:217`), the
  domain of `Source::channels`; nothing narrower is validated.
- #1562b's review fold (B1) sized the browser's source PCM and plane staging by that maximum
  (`hosts/host-web/src/lib.rs`, `project_buffers`), so a rebuild can add any valid source.
- Product rule (dual mono by source file): only mono source files get dual-mono processing; stereo
  files stay stereo. No source has more than 2 channels.

## Decisions frozen for this slice

- **D1.** One rule in the session validator: `channels` is 1 or 2; any other value is refused with a
  typed code (for example `session.source.channels`), at load and in a structural edit's
  validation, so both hosts refuse it the same way.
- **D2.** A launch constant (for example `SOURCE_CHANNELS_LAUNCH_MAXIMUM = 2`) in the session crate
  is the one source of the limit; the browser staging, the C ABI's source checks and the SDK
  builder read it (the SDK through its generated constants).
- **D3.** The browser's staging is sized by D2.

## Objective gates

1. A session with a 3-channel source is refused at load on both hosts with the typed code; a
   structural edit that adds one is refused before commit; a parity row.
2. Mono and stereo sources load, feed and render as today (existing tests green).
3. Browser `sourcePcmStagingBytes` falls back to the 2-channel size; `expected.json` and
   `retained_ceilings.rs` re-pinned from measurements; the boot budget passes.
4. `cargo test --locked -p session -p session-validator -p capi -p host-web --features
   host-web/test-support`; SDK checks; the author-session skill and `SESSION_SCHEMA_V1.md` state
   the limit.

## Authorized paths

- `crates/session/src/{model.rs,validate.rs,diagnostic.rs}`, `crates/session/tests/`,
  `hosts/host-web/src/lib.rs` (the staging size), host-web tests and `expected.json` rows,
  `crates/capi` tests, `sdk/` generated constants and evals, `docs/SESSION_SCHEMA_V1.md`,
  `.claude/skills/author-session/SKILL.md`, this spec's record.

## Dependencies

- #1562 (B1 staging).

## Size

Half a day.
