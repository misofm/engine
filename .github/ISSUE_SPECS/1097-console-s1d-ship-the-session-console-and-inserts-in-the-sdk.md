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

## Attempt 1 evidence

Terra, attempt 1, on `codex/1097-sdk-console` from `75294894` (C3's S1r, S1a, S1b with S1c merged),
then `1e1af655` merges S1c's verdict (`6cecc4cc`). Commits `d96cc526` (SDK), `c587625b`
(qualification), `56ebf895` (skill, handoff), `53090295` (one more refusal eval), `64b32124`
(S1c verdict M1/L1/L3, root-directed) and `58e56690` (handoff line). Every gate below ran on
`58e56690`, x86-64-v3, `CARGO_INCREMENTAL=0`, one `target/`. The class-A base is `75294894`.

### What landed

- **Builder and types** (`sdk/src/core/{types,session}.ts`). `session().console({ preInsert,
  postInsert })` declares each slot once (`slot`, `effectId`, `quality?`, `linkMode?`), before the
  first track, because each track's entries are checked against it (the builder's reference-order
  rule). `TrackSpec` carries `console` (one `{ slot, bypass?, parameters?, channel? }` per slot, in
  slot order; parameters by catalog name and display value, typed like `effect()`'s) and `inserts`
  (`effect()` declarations; unnamed ones default to `insert-N`). `Rack` is `console | inserts`,
  `AutomationRack` adds `builtins`, and `SendTap` has the seven new spellings. The model gains the
  root `console` and each track's `console`/`inserts`.
- **The builder refuses what the engine refuses, with the engine's code.**
  `MisoUsageError.diagnosticCode` carries it: `console.entry_missing`, `console.entry_order`,
  `id.duplicate` (entry, cross-section slot, insert), `reference.missing_entity`,
  `schema.unknown_field` (effect fields on an entry; `sidechain`/`bypass`/`params` on a slot;
  `simd1`/`dynamic`/`simd2` on a track), `schema.invalid_enum` (link mode, retired taps and
  automation racks), `id.invalid`, `console.slot.ineligible_effect`. The entry walk mirrors
  `validate_console_entries`, so the builder's first refusal is the engine's first diagnostic.
  `CONSOLE_ELIGIBLE_EFFECTS` is a copy (the metadata does not publish the list), held to the
  engine effect by effect by an eval.
- **Canonical writer** (`internal/session-json.ts`): root `console` between `sources` and `tracks`,
  slot `slot, identity, quality, link_mode`, entry `slot, bypass, params`, track
  `..., builtins, console, inserts, fader, pan|matrix`.
- **Live controls** (`core/live-controls.ts`). `LiveControlRack` is `console | inserts`, and the
  record byte comes from the layout's `racks` table (`3`, `1`, `255`). `TrackEdits.console(slot,
  effectId)` resolves a slot ID to its index in `pre_insert`-then-`post_insert` order, and
  `insert(idOrIndex, effectId)` an insert; both check the effect. `effect(rack, index, effectId)`
  takes the live address and refuses the retired tokens. **Choice recorded:** the SDK never
  parses a document (ruling 5438024085), so IDs resolve against the builder. `createOfflineEngine`
  and `createEngine` keep the builder they booted from; `loadSession` replaces it. For a document
  booted from text, `EngineLiveControls.withSession(builder)` supplies it and refuses a builder
  whose tracks are not the engine's.
- **Decoders.** Observation racks `console`/`inserts` from the `racks` table (`0`/`2` decode to
  nothing); live-response owners `0` input, `2` inserts, `4` console from `liveResponseRacks`
  (`1`/`3` refused); spectrum `trackPostInput`/`trackPostPan`; the prepared-config refusal
  classifies a rack outside `1`/`3` as `unknownRack`, as the worklet does.
- **CLI.** Requests carry a root `console` and per-track `console`/`inserts`. The retired rack keys
  are refused by name, with what replaced them, and a builder refusal's engine code is in the
  stderr document's `diagnostics`.
- **Qualification** (`sdk-response-entry.ts`, `run.mjs`, L4). The `sdk-*` gates use the new
  names. The new `sdk-live-bypass` gate is gate 4 (below).
- **Skill.** `author-session` teaches the shape, the eligibility list, the taps and racks, and each
  console refusal with its code and stage. Its worked session is
  `.claude/skills/author-session/worked-session.json`.
- **Handoff.** `docs/handoffs/console-strip-2026-09-29/APP-CONSOLE-SDK.md` has the new shape, the
  app's EQ -> compressor migration (two `pre_insert` slots, each track's bypass kept, live address
  console `0`/`1`), every public SDK name old to new, and the raw worklet codes.
- **README** documents the builder, the live-control addressing and the CLI request.

### S1c verdict findings (root-directed)

- **M1.** `host-web` `ffi::a_post_insert_observation_is_read_at_its_console_slot_index` moves the
  observation fixture's compressor to `post_insert`, behind an untapped `pre_insert` EQ. The map
  reports it at console slot 1. A numeric read at rack `3`, index `1` succeeds with that address.
  Slot 0 is `RESULT_UNSUPPORTED`, and slot 2 and insert 0 are refused. Each mutation was planted
  once and reverted:
  - S4 (`resolve_observation` searches `pre_insert` only) turns this test red, and only this one
    of host-web's 15 observation tests.
  - Numbering `post_insert` from 0 in `declared_live_addresses` turns it red at boot.
- **L1.** The stale "rack bytes 0/1/2 ... until #1096" comment (`lib.rs:5907`) is rewritten.
- **L3.** The shipped `.d.ts` now says only raw-export callers get `UnknownRack` for `0`/`2`. The
  shipped host's `command()` rejects them locally with a `miso.error.v1` invalid argument. The
  SDK mirror is refreshed.
- **L4.** `run.mjs`'s `sdk-*` gates spell the new names.

### Gates

| Gate | Result |
|---|---|
| 1. `check-sdk-generated.sh ARTIFACTS`, `check-sdk-deletions.py`, `check-sdk-types.sh` | pass |
| 1. `check-sdk-headless.sh ARTIFACTS` | **313 passed, 0 failed** (was 86 of 284 failing; 29 new in `console-evals.mjs`) |
| 1. `sdk-package.sh check ARTIFACTS` | pass: `enginectl` 13 of 13 (was 4 of 11 failing), tarball smoke |
| 2. Round trip through the engine's canonical writer | `console-evals.mjs`: four builder documents -- empty console, both sections (with delay and multiband inserts), empty inserts (with a `pre_fader` route), the app shape (9 tracks, EQ -> compressor `pre_insert`, 2-mod-3 bypass) -- equal `session_validator validate --canonical` of themselves byte for byte, and each boots. The builder also rebuilds the engine-written `console-sixty-four-track-app.json` and `-intended.json` byte for byte. |
| 3. Each refusal has an eval with the engine's code | 17 builder/engine twins: each defect is refused by the builder with `diagnosticCode` X, and the same defect written by hand into a valid document is refused by the booted engine with first diagnostic X. The seven effect fields on an entry are refused one by one. Every catalog effect is swept as a slot: the builder admits exactly `CONSOLE_ELIGIBLE_EFFECTS`, and the engine boots exactly those and refuses the rest with `console.slot.ineligible_effect`. The CLI passes the code through (`console.entry_order`, `console.slot.ineligible_effect`). |
| 4. SDK-driven qualification, `--check-matrix --self-test-mutations`, private PulseAudio null sink | chromium 151.0.7922.34, firefox 153.0, webkit 26.5: "all qualification gates passed", each row equals `results.json`, the 7-file set is pinned. See the note after this table. |
| 5. The skill's worked session | `session_validator validate` passes all five stages, `--canonical` reproduces it byte for byte, and it boots in the wasm engine. `console-evals.mjs` gates all three. |
| `cargo fmt --all --check`; clippy `--workspace --all-targets --all-features -D warnings`; `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps` | pass |
| `cargo test --locked --workspace --all-features --no-fail-fast` | 294 result lines, 2,114 passed, 0 failed, 40 ignored |
| `scripts/run-wasm-gates.sh` (full) | `ok (native + wasm simd128 + V8 EQ loops)` |
| Artifact job (`build-web-audioworklet.sh`) | module `e7f2ad317b37187eaa77c25618003e3521ee4bb68376bef9c4b75ac85cdc80be`, **3,283,569 B**, unchanged from S1c (S1d moves no module byte); closure `19783d70e8b9ac37192a7df2229216143a9bece2a3d2465fd13168805fa1fa56` (the `.d.ts` comment moved it) |
| Artifact gates | pass: `check-web-audioworklet.sh --without-metadata-regeneration`; `check-browser-expected-resources.py --artifacts` (digests agree, 32-mutation self-test); `check-scalar-oracle-absent.py --wasm`; `test-web-audioworklet.sh`; the V8 spill gate and its self-test |
| ABI layout | `check-abi-layout-v1.py` on the artifact and on `sdk/assets` (identical files), and `--self-test` (22 mutations caught), pass; `check-capi-abi.sh` and `--self-test` pass |
| `test-console-benchmark.sh`; `check-ci-path-routing.py`, `test-ci-path-routing.py` | pass |
| Lint job, every `check-*`/`test-*` step, plus `check-dsp-research.sh`, `check-builtins-listening.sh`, `check-sdk-deletions.py --self-test` | 52 of 52 pass (`TMPDIR` unset, as in CI) |
| `npm ci` | `sdk/` and `hosts/host-web/qualification/` installed from their lockfiles |
| Benchmarks still run (untimed) | `operator/preflight-console-benchmark.sh --step s1d-1097-scratch-preflight`: PASS, 0 launches, nothing written. V8 `prepare` then `preflight` in an empty scratch directory: PASS. |

**Gate 4, the new `sdk-live-bypass` row.**
- **The session.** It is built with the SDK: a `pre_insert` EQ, a `post_insert` EQ and an insert
  EQ, each on its own band. It boots from the builder in an `OfflineAudioContext`.
- **The edits.** `console("desk-hi")` and `insert("ins-mid")` each bypass, and each lift an
  authored bypass, live before the first quantum.
- **What the gate requires.**
  - Each render's SHA-256 equals the render of the document that authors that state.
  - The three states are audible and distinct.
  - The addresses are console `[3, 1]` (the `post_insert` slot crosses the section split) and
    insert `[1, 0]`.
  - Every edit applies at sample 0.
- **Proof.** Three self-test mutations prove the gate. A planted SDK off-by-one (console index
  minus one) turns it red in chromium ("a live bypass toggle on the post_insert console slot did
  not render its authored bypass state"). The native twin in `console-evals.mjs` covers all four
  instances (two `pre_insert`, one `post_insert`, one insert) of track `b` in a two-track strip.
  The same off-by-one turns all three of its live tests red.

### Class A (PR evidence)

- **Console digests.** `git diff 75294894 HEAD -- crates tools Cargo.toml Cargo.lock` is empty, so
  nothing `console-workload` builds from changed. The head's `gain_pan_profile digests` (release,
  64 blocks) prints 22 rows. `sixty_four_track_app_shape` is `c740fa2dd904` and
  `sixty_four_track_eq_comp_simd1` is `f68febb7`, as S1a and S1c recorded.
- **V8 harness.** `preflight` digests equal S0's record
  (`artifacts/steps/console-strip-base/web-mixing-automation.jsonl`, `preflight_output_sha256`)
  on all seven arms:
  - quiet and restated `014e5f5b…`;
  - automated `e7025b5c…`;
  - EQ `2540aff4…`, compressor `c29d12a7…`, limiter `8db18991…`;
  - `restated_eq_only` `014e5f5b…`.

  The documents are `d913ad961d2d` (console) and `3dd8b2fff4b9` (app), as S1a and S1c recorded.

### Test value (one line each)

- `console-evals.mjs`, refusal twins: a builder check that is dropped or given its own code, or
  an engine code that moves, turns its row red.
- Eligibility sweep: an SDK list that drifts from `CONSOLE_ELIGIBLE_EFFECTS` turns it red.
- Round trip and fixture rebuilds: writer field-order drift (an entry, the root `console`, the
  track's `console`/`inserts`) turns them red.
- Live addressing: `post_insert` numbered within its section, inserts sent at the console code, or
  an ID resolved to the wrong instance, turns them red.
- `withSession`: a silent empty layout for text documents turns it red.
- Worked session: a skill example the engine refuses turns it red.
- `live-response-evals`, retired owner codes: decoding the old positional table turns it red.
- `enginectl-cli`: dropping the named retired-key refusal or the code pass-through turns it red.
- `live-controls-types.ts`: accepting the retired rack tokens at the type level turns it red.
- `ffi::a_post_insert_observation_…`: S4 and a `post_insert` numbered from 0 turn it red.

### Files outside the spec's list

- `hosts/host-web/src/{ffi,lib}.rs` and `hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts`
  are root-directed (S1c verdict M1, L1, L3).
- `run.mjs` is L4.
- `docs/handoffs/console-strip-2026-09-29/APP-CONSOLE-SDK.md` is item 7's note.

### Not run

- The AArch64 legs and `check-cross-targets.sh`. S1d changes no Rust outside `host-web`'s test
  module and one comment.
