# Build submix strips and bus taps in the SDK and teach agents to author them

Slice 08 of *Submix strips and live aux sends* (#1196, the umbrella filed by *Record the submix, send and
VCA ruling*, #1197). Batch K1 closes here: the SDK is back in step with the engine grammar, and the root
pushes K1 once after this slice's verdict.

The design record cited below (`DESIGN`, `VERIFY-1` to `VERIFY-3`, `REVISION-1`, `REVISION-2` and
`APPLIED-3`) is committed in `docs/handoffs/submix-sends-2026-10-02/`.

## Product outcome

An app or agent using the TypeScript SDK, or the `enginectl` CLI, can:

- declare a submix with a full strip: builtins, console entries, inserts, fader, and pan or matrix;
- route or key a sidechain from any of a submix's seven taps.

The SDK-authored document is the engine's canonical JSON, byte for byte, and it boots and renders in
the shipped browser module. A spec-less `submix(id)` is a transparent strip, so an existing app's
bare buses keep their sound.

An agent using the repository's `author-session` skill, and the misofm app team, know how to author
buses: the submix strip and its taps, the return pattern, the bus-compressor hazard, latency growth
with bus depth, and how to migrate a saved document K1 now refuses.

## Context (verified on `fe8ac679`; slices 02-07 changed the engine side as stated)

- **The engine side**, after slices 02-07:
  - a submix is `{ id, builtins, console, inserts, fader, pan|matrix }` in that canonical order;
  - a submix route or sidechain source is `{ kind: "submix", submix_id, tap }`, and `submix_output`
    refuses with `schema.invalid_enum`;
  - the writer corpus and `.claude/skills/author-session/worked-session.json` were migrated by those
    slices.
- **Builder** (`sdk/src/core/session.ts`):
  - `submix(id)` (`:726-732`) takes only an ID. `BuilderState` stores `submixes: string[]` (`:637`),
    emitted as `{ id }` (`:1450`).
  - `track(id, spec)` (`:716`) validates through `#validateTrack` (`:833`).
  - `console(spec)` (`:685`) must precede tracks.
  - A track's console entries are normalized by `normalizeConsoleEntries` (`:1161-1224`). When slots
    exist, every entry is required, in slot order, with the engine's codes (`console.entry_missing`,
    `console.entry_order`, `reference.missing_entity`, `id.duplicate`).
  - Route source handling is at `:567-571` (normalization) and `:590-593` (emission).
- **Types** (`sdk/src/core/types.ts`):
  - `TrackSpec` (`:128-142`): `source`, then optional `builtins`, `console`, `inserts`, `fader` and
    `pan`;
  - `RouteSource` (`:195-197`) includes `{ kind: "submix_output"; submixId }`, which is also the
    sidechain source (`SidechainSpec.source`);
  - `RouteSpec` (`:217-224`).
- **Canonical writer** (`sdk/src/internal/session-json.ts`):
  - `OBJECT_KEY_ORDERS` (`:67-88`): `pan` `:79`, `matrix` `:80`, `submixes: ["id"]` `:81`,
    `routes` `:83`;
  - the track order (`objectOrder`, `:112-117`) is `id, source_id, left_source_channel,
    right_source_channel, builtins, console, inserts, fader, pan|matrix`;
  - the console key resolves to the root declaration or a strip's entry array (`:120-122`);
  - the source tags (`taggedOrder`, `:100-103`) spell `submix_output` as `["kind", "submix_id"]`.
    They are shared by route and sidechain `source`;
  - any object keyed `left` and `right` is treated as a channel-builtins lane (`:124`);
  - an unlisted numeric leaf throws (`INTEGER_KEYS` `:37`, `FLOAT_KEYS` `:49`).
- **The `enginectl` CLI** (`sdk/src/cli/session-request.ts`):
  - `submixes?: unknown[]` (`:37`), each a bare string passed to `builder.submix(id)` (`:444-446`);
  - `routeSource` (`:115-131`) accepts `track` or `submix_output`, and the sidechain parser uses it
    (`:215`); routes call it at `:360`; root keys at `:418`;
  - `sdk/README.md`'s request paragraph (`:325-330`) lists `submixes` as a request array.
- **Evals that touch submixes**, red since slice 02 inside the batch:
  - `sdk/test/builder-evals.mjs` `:91`, `:308`, `:442`, `:818-830` (the Rust-generated writer corpus,
    which now holds a submix strip and a tapped submix source) and `:949-960`;
  - `sdk/test/console-evals.mjs`: `rebuild()` (`:586-662`; `built.submix(id)` at `:624`,
    `submix_output` at `:653`) and the worked-session row (`:665-676`), which compares the skill's
    document with `engineCanonical()` (`:470-479`, the engine's canonical re-serialization through
    `session-validator validate --canonical`) and boots it;
  - `sdk/test/enginectl-cli.mjs` `:61-66`, `:214`, `:242-244` and `:463-468`.

  `sdk/test/writer-evals.mjs` tests the `LiveControlWriter` queue contract, not canonical JSON; it is
  not a writer-parity gate (VERIFY-2 M9).
  `bash scripts/check-sdk-headless.sh <A>` runs every `sdk/test/*-evals.mjs` against the shipped
  module. `bash scripts/sdk-package.sh check <A>` runs `enginectl-cli.mjs`.
- **The skill.** `.claude/skills/author-session/SKILL.md:66` still says a submix is
  `{"id":"buss"}`. `tools/session-validator/tests/skill.rs`
  (`the_shipped_skill_validator_commands_run_verbatim`, `:61-110`) runs every `session-validator`
  line of the skill's fenced shell blocks, but substitutes `fixtures/session/v1/canonical-minimal.json`
  for every `.json` argument (`:68-79`). It proves the commands' syntax only; the real proof that the
  skill's document is canonical and boots is `console-evals.mjs`'s worked-session row (VERIFY-2
  MINOR 15).
- **Handoff precedent.** `docs/handoffs/console-strip-2026-09-29/` holds `APP-CONSOLE-SDK.md`,
  `APP-LIVE-CONTROLS.md` and `migrate-console-inserts.py`, a one-off migration of saved documents
  (`--check` reports without writing) that no gate runs. `scripts/check-script-reachability.py`
  exempts `docs/` (`NOT_CARRIERS`, `:49`).
- **What K1 changed for saved documents:** a bare `{"id": ..}` submix is refused (`builtins`,
  `console`, `inserts`, `fader` and `pan`/`matrix` are required), and a `submix_output` route or
  sidechain source is refused; its replacement is `{ "kind": "submix", "submix_id": .., "tap":
  "post_pan" }`, which is the same node.
- **The bus-compressor hazard** (DESIGN 2.2b). A console slot's `link_mode` is session-level, and
  every bus carries every console slot. A console compressor declared `dual_mono` for tracks
  therefore runs **unlinked** on every stereo bus, and shifts the image under asymmetric material.
- **Latency.** Every strip pays every latent console slot, even when bypassed (decision 12, L4). With
  a console limiter, each bus level adds 486 samples at 48 kHz.

## Decisions frozen for this slice

- **D1. `submix(id, spec?: SubmixSpec)`.** `SubmixSpec` is `TrackSpec` without `source`.
  - With a spec, every field follows the track's rules and defaults. Console entries are checked
    against the declared console exactly as a track's are: all entries are required when slots
    exist, so a submix with a spec must follow `console()`.
  - With no spec, it is the transparent strip (DESIGN P15): identity input section; no inserts; a
    0 dB unmuted fader; the identity matrix (`ll = rr = 1`, `lr = rl = 0`, smoothing 0), as
    `Submix::unity` builds it; every console entry `{ slot, bypass: true, params: [] }`, one per declared slot in slot order.
    Latency is still paid.
- **D2. `RouteSource`** gains `{ kind: "submix"; submixId; tap: SendTap }`, for routes and
  sidechains. The `submix_output` variant is removed from the type and refused at runtime with a
  message naming the replacement.
- **D3. The writer.**
  - It emits submix keys in the track's order minus the source fields:
    `id, builtins, console, inserts, fader, pan|matrix`.
  - It emits the `submix` source as `["kind", "submix_id", "tap"]`, for routes and sidechains.
- **D4. `enginectl`.**
  - A `submixes` entry may be a bare string (a transparent strip, D1) or an object
    `{ id, ...SubmixSpec }`.
  - `routeSource` accepts `{ kind: "submix", submixId, tap }`, refuses `submix_output` naming the
    replacement, and serves the sidechain parser too.
- **D5. Docs say only what the engine enforces.** The skill and the handoff state no performance
  claim.
- **D6. The handoff folder is `docs/handoffs/submix-strips-and-sends/`**, literally. Later slices
  (*Drive submix strips from the SDK live controls* (#1214), *Enumerate sends and drive them from the SDK*, #1223)
  add `APP-LIVE.md` to this same folder (VERIFY-2 MINOR 17).
- **D7. The migration script** is a one-off on the `migrate-console-inserts.py` precedent. It adds
  the transparent strip keys (D1's values, one `bypass: true` entry per declared slot) to every bare
  submix and rewrites every `submix_output` source (route or sidechain) to
  `{ kind: "submix", tap: "post_pan" }`. It preserves key order and number spellings elsewhere, so a
  canonical document stays canonical. `--check` reports without writing.

## Deliverables

1. **SDK:** builder, types, validation and normalization (D1, D2); canonical writer (D3); `enginectl`
   request parser (D4) and `sdk/README.md`'s request paragraph.
2. **Evals:** update every red eval listed in the Context (including `console-evals.mjs`'s
   `rebuild()`, which passes each model submix's strip as a `SubmixSpec` and its sources as
   `{ kind: "submix", submixId, tap }`), and add the gates below.
3. **`author-session` skill** (`SKILL.md`, replacing `:66`):
   - submix strips (the same keys and rules as a track's strip, minus the source fields);
   - the seven bus taps;
   - "a return is a submix with an effect insert";
   - the bus-compressor hazard, with its guidance: bypass the console compressor on buses, and put a
     linked compressor (`link_mode: maximum` or `average`) as a bus insert for glue;
   - latency growth with bus depth;
   - the transparent spec-less submix.
4. **App handoff** `docs/handoffs/submix-strips-and-sends/APP-SDK.md`, on the `APP-CONSOLE-SDK.md`
   model: `submix(id)` is now a transparent strip; `submix_output` is renamed; the saved-document
   migration; the hazard and its guidance.
5. **The migration script** `docs/handoffs/submix-strips-and-sends/migrate-submix-strips.py` (D7).
6. `AGENTS.md`: remove the decision-13 qualifier from the K1 sentences (*Record the submix, send and
   VCA ruling* D5: the dual-mono strip, chain, console-slot, strip-insert and seven-tap sentences),
   and in "Effects and plugins" change "A native effect may run track-locally as an insert" to
   "strip-locally". In the banking paragraph of "Approved audio architecture" ("Audio buffers are
   planar `f32`, ..."), change "tracks" to "strips" where it describes the bank layout: "banked
   AoSoA across strips", "the same dual-mono lane from four Wasm/NEON strips or eight AVX2 strips",
   "parameters/state remain per-strip", "incompatible strips form another cohort" and "Scalar tails
   support every strip count" (*Carry every console slot on every submix strip* D3 banks a bus's
   console lane with the tracks'). Change nothing else. The parenthetical that cites decision 13 and
   its authority marks stays.
7. `docs/IMPLEMENTATION_PLAN.md`, "Non-negotiable release shape": "Tracks are dual-mono and run"
   becomes "Strips (tracks and submixes) are dual-mono and run", and "a track's inserts to the
   dynamic rack" in the same sentence becomes "a strip's inserts to the dynamic rack", changing
   nothing else in that paragraph.

## Authorized paths

- `sdk/src/core/{session.ts,types.ts}`
- `sdk/src/internal/session-json.ts`
- `sdk/src/cli/session-request.ts`
- `sdk/README.md`
- `sdk/test/{builder-evals,console-evals}.mjs`, `sdk/test/enginectl-cli.mjs`
- `.claude/skills/author-session/SKILL.md`
- `docs/handoffs/submix-strips-and-sends/` (new folder: `APP-SDK.md`, `migrate-submix-strips.py`)
- `AGENTS.md` (those qualifiers, the one word "track-locally" and the banking paragraph's "tracks"
  only)
- `docs/IMPLEMENTATION_PLAN.md` (that one sentence only)
- this spec

## Non-goals

- No engine code change.
- No live controls or meters for submixes (batch K2).
- No route mute (*Mute a route in the session*, #1216).
- No generated-code change: `sdk/src/generated/` moves only if `check-sdk-generated.sh` says so.

## Hazards

- **The `left`/`right` rule.** The writer treats any object with `left` and `right` as a
  channel-builtins lane (`session-json.ts:124`). A submix's `builtins` must take the same path as a
  track's. A submix's `pan` (`left`, `right`, `smoothing_samples`) must take the pan path, exactly as
  a track's does.
- **Numeric leaves.** A new numeric key the writer does not list throws. The submix keys are the
  track's, so no new numeric key is expected. If one appears, the writer and the engine disagree.
- **Bare strings in `enginectl`.** Existing request files with bare submix IDs must keep working as
  transparent strips.
- **Stale guidance.** The skill is read by agents as truth. Every sentence must match the merged K1
  grammar; a wrong key order in the skill produces documents the engine refuses.

## Objective gates

1. **Writer parity** (VERIFY-2 M9).
   - In `console-evals.mjs`, an SDK-built session declares console slots, two tracks, a drum submix
     with a non-trivial strip, a `verb` submix with a `miso.delay` insert, and routes from a track
     `pre_fader` tap and a bus `post_fader` tap; a sidechain is keyed from the drum bus's
     `pre_fader`. Its `toJson()` equals `engineCanonical()` of the same text, byte for byte.
   - In `builder-evals.mjs`, the writer-corpus row (`:818-830`) passes over the regenerated corpus,
     whose submix carries a non-trivial strip and whose bus route source carries a tap.

   *Test value: it turns red if the SDK writer orders submix keys or spells the bus source
   differently from the engine grammar. No SDK test writes a submix strip.*
2. **Builder refusals** (`builder-evals.mjs`).
   - `console()` after a spec'd submix refuses with the track rule's message (the rule refuses at
     `console()`, `sdk/src/core/session.ts:690-695`).
   - A submix spec whose entry list is missing a slot refuses with `console.entry_missing`.
   - `{ kind: "submix_output" }` refuses, as a route source and as a sidechain source, with a message
     naming `submix`.

   *Test value: it turns red if the builder lets through a submix the engine would refuse at boot.*
3. **The transparent default** (`console-evals.mjs`, booting and rendering through the shipped module
   as its worked-session row does).
   - `submix("bus")` with no spec, in a session whose console slots all declare zero latency (one
     `miso.parametric-eq` slot), writes every entry with `bypass: true`.
   - A session routing one track through it renders, in the shipped module, bit-identically to the
     same track routed straight to the output.
   - The source is finite with no `-0.0` sample, because the identity input section's only change is
     `-0.0` to `+0.0`.

   *Test value: it turns red if a spec-less submix runs its console slots, which would change the
   sound of every migrated bare bus.*
4. **It boots and renders headless.** Gate 1's session boots through the headless path and renders
   a block without error (`console-evals.mjs`).
   *Test value: it turns red if an SDK-authored bus session fails engine preparation.*
5. **`enginectl`** (`enginectl-cli.mjs`). A request with a bare-string submix, an object submix with a
   fader, and a `submix` route source with a tap produces the engine's canonical JSON. A
   `submix_output` source is refused with the replacement named.
   *Test value: it turns red if the CLI still accepts the retired source or drops a submix strip
   field.*
6. **The skill's document.** `console-evals.mjs`'s worked-session row (`:665-676`) passes: the skill's
   `worked-session.json` is the engine's canonical JSON and boots. That row is the real proof;
   `cargo test --locked -p session-validator --test skill` (also in the test-debug-a command) proves
   only that the skill's validator command lines still run, because it substitutes
   `canonical-minimal.json` for every document argument.
   No new test: the skill text and the handoff are documentation.
7. **The migration script, as PR evidence and not a committed test** (the
   `migrate-console-inserts.py` precedent). Run it on the pre-K1 worked session
   (`git show <K1 base>:.claude/skills/author-session/worked-session.json`). Its output is accepted by
   `target/release/session_validator validate --canonical <file>`, and differs from the migrated
   checked-in document only by that document's deliberate non-transparent values, if any. Record
   both outputs.
8. **SDK checks.**
   - `bash scripts/check-sdk-types.sh`
   - `bash scripts/check-sdk-generated.sh <A>`
   - `bash scripts/check-sdk-headless.sh <A>`, which must pass with every eval, including the
     previously red `builder-evals.mjs` and `console-evals.mjs` rows
   - `bash scripts/sdk-package.sh check <A>`, which runs `enginectl-cli.mjs`
   - `bash scripts/check-workspace-policy.sh`
9. **K1 boundary gates, run once at the batch head before the root pushes K1** (DESIGN 7 forms):
   - the test-debug-a and test-debug-b commands;
   - `cargo test --locked --release -p audit -p bench -p console-workload`;
   - `bash scripts/run-aarch64-tests.sh debug`, or CI's `aarch64-debug` at the push;
   - `bash scripts/build-web-audioworklet.sh --named-twin <B> <A>`;
   - `bash scripts/check-web-audioworklet.sh <A> <B>/miso-engine-v1-audio-worklet.simd128.named.wasm`;
   - `python3 -B scripts/check-browser-expected-resources.py --artifacts <A>`;
   - `bash scripts/test-web-audioworklet.sh`;
   - `bash scripts/check-protocol-wasm-parity.sh`;
   - `cargo build --locked --release -p audit -p bench -p capi -p session-validator`, then
     `./target/release/audit capi`, `bash scripts/trace-graph-audit.sh target/release/audit`,
     `bash scripts/check-graph-determinism.sh`,
     `cargo run --locked -p graph-compiler --bin graph_fixture -- --check`,
     `bash scripts/check-console-fixtures.sh target/release/session_validator` and
     `bash scripts/check-builtins-fixtures.sh . target/release/audit`;
   - `bash scripts/check-capi-abi.sh` and `bash scripts/check-capi-abi.sh --self-test`;
   - `cargo fmt --all -- --check`;
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`;
   - `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`.

## Evidence

- Writer-parity output.
- The gate-7 outputs.
- The K1 boundary gate log.
- The ARTIFACT CHANGED/UNCHANGED report for the shipped module.
- The K1 batch push link and its CI run.

### Attempt 1 record (Terra)

- **SDK.** `submix(id, spec?: SubmixSpec)` (`SubmixSpec = Omit<TrackSpec, "source">`); a spec runs
  the track's strip validation (`#validateStrip`, shared with `#validateTrack`) and `knownKeys`
  (`schema.unknown_field`); `normalizeSubmix` writes a spec-less submix as `Submix::unity` (catalog
  defaults, identity `matrix`, one `{ slot, bypass: true }` entry per slot). `console()` now also
  refuses after a spec'd submix (message "declare the console before the first track or submix
  strip"); a spec-less submix may precede it. `RouteSource` is `track | submix` (with `tap`);
  `submix_output` refuses as route and sidechain source with `schema.invalid_enum` and a message
  naming `{ kind: "submix", submixId, tap }` and `post_pan`. Writer: submix order `id, builtins,
  console, inserts, fader, pan|matrix`; `submix` source `kind, submix_id, tap`. `enginectl`: a
  `submixes` entry is a bare string or `{ id, ...strip }` (one `strip()` parser shared with
  tracks); `routeSource` takes `submix` and refuses `submix_output` (`request.shape`, diagnostic
  `schema.invalid_enum`). `sdk/README.md` request paragraph updated. No generated-code change.
- **Docs.** `SKILL.md` (submix shape, route shapes, the seven taps on every strip, a new "Buses"
  section: return pattern, bus-compressor hazard and guidance, latency growth, the transparent
  strip, migration); `docs/handoffs/submix-strips-and-sends/APP-SDK.md` and
  `migrate-submix-strips.py`; `AGENTS.md` (five K1 qualifiers removed, "strip-locally", banking
  paragraph "strips"; the decision-13 parenthetical and the route-mute, follow-mute and VCA
  qualifiers stay); `IMPLEMENTATION_PLAN.md` (the one sentence). Facts checked against the engine:
  a bare `{"id"}` submix is `schema.missing_field`; a cycle through buses passes all five
  `session_validator` stages and is refused at boot (`engine.refused`), so the skill says so.
- **Tests and their test value** (every one red on a revert of `sdk/src` to `7f767ae0c`):
  - `builder-evals.mjs` "submix strips: the builder refuses what the engine refuses, with the
    engine's code" (gate 2): red if the builder lets through a submix the engine refuses at boot
    (a spec'd submix before `console()`, a bus missing a console entry, `submix_output` as a route
    or sidechain source) or names a different code; the hand-edited documents prove the engine
    gives `console.entry_missing` and `schema.invalid_enum`. The rich session's bus now carries a
    strip off every default and a tapped bus route, so the existing boot and reverse-construction
    rows cover submix key order too.
  - `console-evals.mjs` "a bus session is the engine's canonical JSON, byte for byte, and boots
    and renders headless" (gates 1 and 4): red if the writer orders submix keys or spells the bus
    source differently from the engine, or an SDK bus session fails preparation.
  - `console-evals.mjs` "a spec-less submix is the transparent strip: every slot bypassed, and it
    renders as no bus" (gate 3; one zero-latency EQ slot, finite source with no `-0.0`): red if a
    spec-less submix runs its console slots, takes the default pan, or moves off unity.
  - `console-evals.mjs` "the builder rebuilds an engine-written submix strip and bus tap byte for
    byte" (`rebuild()` now passes a `SubmixSpec` and `{ kind: "submix", submixId, tap }`): red if a
    spec'd submix is written other than as authored. No SDK test wrote a submix strip before.
  - `enginectl-cli.mjs` "submix strips and bus taps build the engine's canonical JSON;
    submix_output is refused by name" (gate 5; bare `bus`, object `low` with a fader, a
    `submix`/`post_fader` route; output compared with `session-validator validate --canonical`):
    red if the CLI drops a strip field or accepts the retired source. The shared `request()`, the
    direct-builder parity row and the cycle row moved to the new shapes.
- **Mutations** (scratch drivers, one at a time, file restored; all RED):
  M1 writer submix `console` after `inserts` -> gate 1, rebuild row, writer corpus row; M2 writer
  `tap` before `submix_id` -> same; M3 spec-less entries `bypass: false` -> gate 3 and gate 2's
  bare-entry assertion; M4 spec-less default `pan` -> gate 3 (render); M5 `console()` ignores
  spec'd submixes -> gate 2; M6 `submix()` skips strip validation -> gate 2; M7 `submix_output`
  accepted -> gate 2; M8 spec'd submix written transparent -> gate 1 and rebuild row; M9 spec-less
  fader -0.01 dB -> gate 3 (render); C1 CLI object submix's strip dropped -> gate 5 and the parity
  row; C2 CLI `submix_output` not refused by name -> gate 5.
- **Gate 7** (`git show 0fb85d2c5:.claude/skills/author-session/worked-session.json`): the pre-K1
  document is refused (`schema.invalid_enum` at `$.routes[0].source.kind`, `schema.missing_field`
  for `$.submixes[0]` pan/matrix, `builtins`, `console`, `fader`, `inserts`). The script reports
  `transparent strips ['band']; submix_output -> submix post_pan ['band']`; its output passes all
  five stages of `target/release/session_validator validate --canonical`, is its own canonical form
  (`cmp` equal), and is byte-identical to the checked-in `worked-session.json` (no deliberate
  non-transparent values).
- **SDK gates** (final tree): `check-sdk-generated.sh` ok; `check-sdk-deletions.py` ok;
  `check-sdk-types.sh` ok; `check-sdk-headless.sh` 333/333 pass (including every `builder-evals`
  and `console-evals` row red since slice 02); `sdk-package.sh check` ok (`enginectl-cli.mjs`
  15/15, tarball gate passed); `check-workspace-policy.sh` ok; `cargo test -p session-validator
  --test skill` ok.
- **K1 boundary gates** (x86-64 AVX2 host, this tree, all rc 0): test-debug-a (99 binaries, 1143
  passed); test-debug-b (787 passed) and `conformance_fixtures --check`; release `audit`/`bench`/
  `console-workload` tests; `build-web-audioworklet.sh --named-twin`; `check-web-audioworklet.sh`;
  `check-browser-expected-resources.py`; `test-web-audioworklet.sh`;
  `check-protocol-wasm-parity.sh`; release build, `audit capi` (0 allocations, 0 syscalls, 0
  violations), `trace-graph-audit.sh` (PASS, 1000000 blocks), `check-graph-determinism.sh`
  (100/100), `graph_fixture --check`, `check-console-fixtures.sh`, `check-builtins-fixtures.sh`
  (50 files); `check-capi-abi.sh` and `--self-test`; `cargo fmt --check`; clippy `-D warnings`;
  `cargo doc -D warnings`. **Not run locally:** `run-aarch64-tests.sh debug` (no aarch64 linker or
  qemu on this host); CI's `aarch64-debug` at the K1 push carries it.
- **ARTIFACT UNCHANGED.** No engine code changed. The shipped module built on this tree is
  `6aec3733e17dacf415f5199b740d91d01f2930845e9163f23985b419058a5ecc` (named twin `4c0e960a...`),
  equal to the build of `7f767ae0c` made before this slice's edits.
- **Not done here:** the K1 push and its CI run (root).

## Dependencies

- *Address submix strips in session edits* (#1204)

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Prefer the SDK's existing patterns (track validation, `knownKeys`, `MisoUsageError`); add no new
  framework.
- A test that greps source or prose is refused.
- Commit on the K1 batch branch. The root pushes K1 once after this verdict.
- Attempt budget: five attempts, one adversarial verdict each.
