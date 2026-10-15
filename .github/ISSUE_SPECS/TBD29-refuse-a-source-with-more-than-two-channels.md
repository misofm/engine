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

## Decision record: refuse a source with more than two channels (batch 8)

**Shipped.**

- D1. One rule in `session::validate`: `Source::channels` is 1 or 2. Zero and more than two are one
  rule with one new typed code, `DiagnosticCode::SourceChannelsUnsupported`
  (`source.channels.unsupported`, path `$.sources[i].channels`). It replaces the old
  `capacity.zero` for `channels == 0` (frames keep `capacity.zero`). Every load and every
  structural edit already validates the candidate model through this function, so both hosts
  refuse alike.
- D2. `session::SOURCE_CHANNELS_LAUNCH_MAXIMUM = 2` replaces `SOURCE_CHANNELS_MAXIMUM` (255,
  deleted: nothing else read it). Readers: the validator, the browser's staging
  (`hosts/host-web/src/lib.rs` `project_buffers`), the C ABI's source submission
  (`crates/capi/src/ffi.rs`: the plane-count bound and the stack plane array, 255 to 2, through
  `host_core::SOURCE_CHANNELS_LAUNCH_MAXIMUM`, a re-export beside `LAUNCH_SAMPLE_RATES`) and the
  SDK (new ABI-layout constant `maximumSourceChannels`, written by `parameter-metadata`, read by
  the session builder).
- D3. The browser's source PCM and plane staging are sized by the constant: `sourcePcmStagingBytes`
  130,560 to 1,024. The ID staging keeps its 127 bytes (#1562b B1).

**Departure.** `HostPrepareCaps::maximum_source_channels` stays `None` everywhere: the validator is
the one rule, a second per-host cap would be a second place to drift.

**C ABI.** A chunk with three or more planes is now `RESULT_INVALID_ARGUMENT` at the boundary
(it was `source.channels.mismatch` from the facade); the row "three planes for a stereo source" in
`source_rejections_reach_the_c_host_as_their_own_diagnostic` became "one plane for a stereo source"
and a separate assertion covers three planes.

**Fixtures and corpora.** No fixture under `fixtures/` declares a source of more than two channels.
Tests that did: `host-web` `three_source_session` (alpha, 4 channels, now 1) and
`plan_exchange_threads` (a six-channel source, now a stereo one; renamed
`a_rebuild_adds_the_longest_id_and_feeds_the_new_source`; #1562's record still names the old test
and is history). `hosts/host-web/tests/stem-pump-v1.mjs` pumps a 3-channel PCM file through the
stem pump: it is the pump's own channel-generic test and builds no session, so it stays.

**Gates.**

1. `control_parity::both_hosts_refuse_a_source_of_more_than_two_channels_alike`: a three-channel
   source is refused at load by the C ABI compile and the browser boot with identical diagnostics
   containing `source.channels.unsupported`; mono and stereo load; a structural `UpsertSource` of
   three channels is refused on both with identical reply bytes, the code
   `source.channels.unsupported` and the revision unmoved (four rates). Session crate:
   `diagnostic_parity` (`source-channels-zero`, `source-channels-three`) and `invalid_matrix`
   (`source_identity_and_shape_category_has_22_distinct_cases`).
2. Mono and stereo unchanged: the existing host-web, capi, control-plane and conformance suites.
3. Browser `expected.json`: `sourcePcmStagingBytes` 1,024, `bridgeRetainedBytes` ceiling 1,812,352
   to 1,667,648 (measured 1,611,330 on the simd128 module; 562,754 beyond the capture, plus 10 %,
   rounded up to 64), the three `memoryBytes` 3,014,656 to 2,883,584. `retained_ceilings.rs`
   `bridge_retained_bytes` 787,968 to 641,024 (measured 1,631,294 x86-64, 582,718 beyond the
   capture). Boot high-water growth (`check-web-boot-budget.mjs`): 20,447,232 bytes (before:
   20,447,232).
4. SDK: `builder-evals` "a source of other than one or two channels is refused at its own path";
   `sdk/src/generated/abi.ts` and `sdk/assets/miso-engine-v1-abi-layout.json` regenerated;
   `scripts/check-abi-layout-v1.py` and its self-test fixture know `maximumSourceChannels`.
   `docs/SESSION_SCHEMA_V1.md` and the author-session skill state the limit.

**Found in review of callers.** `crates/host-core/src/limiter_linked_session.rs` (a `#[cfg(test)]`
unit test, the 64-track linked-limiter corpus) widened its one source to 128 channels. It now
declares 64 stereo sources (`stream-NN`), track `k` reads source `k`'s two channels and the same
noise streams `2k` and `2k + 1` feed them; `maximum_sources` 16 to 64. The `PIN` digest is
unchanged and the three width tests pass, so no rendered bit moved.
