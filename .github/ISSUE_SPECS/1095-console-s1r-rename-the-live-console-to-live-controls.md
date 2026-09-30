# Rename the live console to live controls

Slice S1r of *Console strip: session-level console effects with per-track inserts* (owner
decision 12, `docs/rulings/engine-footprint-2026-09-29.md`, "Naming"; Sol's M6 and amendment 13 in
`docs/handoffs/console-strip-2026-09-29/VERIFY.md`, commit `03aceb94`).

## Problem

The session key is `console` (decision 12), but "console" already names the live-control
attachment: the channel through which a host changes a prepared plan's fader, mute, pan, solo,
effect parameters and bypass while it renders, and reads its meters and observations. Once S1a
lands, "console" would mean two things in the same code, and S1c and S1d, which edit that code,
would inherit the ambiguity.

The owner ruled that the engine's internal live-console names are renamed, for example to "live
controls". The question the owner answered listed the fader, mute and pan lanes, the
`miso_engine_web_v1_console_track_*` exports and the benchmark names. So the owner decided to
rename the internal names (checkpoint 1) and the two exports (checkpoint 2). The console benchmark
and fixture names stay, because they describe console sessions.

The four boot-option words and the SDK's public live-console API are renamed with the exports, so
that one vocabulary crosses the boundary. That is root's application of the owner's decision, not a
separate owner decision (decision 12, "Naming").

## Why this is its own slice

R0 was asked to fold the rename into S1c or give it its own slice, depending on its size. It gets
its own slice. The attachment's vocabulary reaches about 80 code, script and SDK files (about 780
lines) and about 8 live docs, and it includes:
- two sealed wasm exports;
- four pinned boot-option words;
- the SDK's public live-console API.

That is more than S1c's half day on its own. Folding it in would put a class-A mechanical rename
and a semantic addressing change under one verdict. Landing it first, before S1a, lets its SDK
gates run against an unchanged schema. It also means S1a, S1c and S1d edit code in which "console"
has one meaning.

## Smallest closable slice

Work in two checkpoints.

### Checkpoint 1: internal Rust names (class A, nothing pinned moves)

Rename every identifier whose "console" means the attachment. Types take `LiveControl`, and
functions and fields take `live_controls` or `live_control`, whichever reads as the attachment.
The minimum set (M6) and its map:

| Today | Renamed |
|---|---|
| `HostConsoleRequest` (`crates/host-core/src/prepare.rs:279`) | `HostLiveControlRequest` |
| `HostConsoleHandles` (`prepare.rs:342`) | `HostLiveControlHandles` |
| `ConsoleEffectBankStage` (`crates/rack/src/lib.rs:911`) | `LiveControlEffectBankStage` |

The same rule covers the rest of the attachment's names, among them:
- `prepare_host_*_with_console*`, `prepare_*session_builtins_with_console*` and
  `attach_effect_console`;
- `ConsoleInputProcessor`, `ConsoleMatrixProcessor` and `ConsoleFaderProcessor`;
- `ConsoleSoloState` and `ConsoleMuteDelta`;
- `LiveConsoleRecord` and the graph runtime's `ConsoleEffect`;
- host-web's `console_tracks`, `console_attached`, `copy_console_track_id`, `console_solo` and
  `console_request`.

**Keep** the names that describe console sessions or the console benchmark:
- `tools/console-workload`, `tools/bench/src/console.rs` and the `sixty_four_track_console*` rows;
- the `console-sixty-four-track*` fixtures and their derive and check scripts;
- test helpers that build or render those sessions, such as `intended_console` and
  `compile_console_model_*`.

Record the per-identifier classification (rename or keep, with a one-line reason) in the PR
description, not in a committed ledger. Where a name is ambiguous, read what it constructs.

### Checkpoint 2: the boundary (sealed names; in-place V1 amendment)

| Today | Renamed |
|---|---|
| `miso_engine_web_v1_console_track_count` | `miso_engine_web_v1_live_control_track_count` |
| `miso_engine_web_v1_console_track_id` | `miso_engine_web_v1_live_control_track_id` |
| boot words `consoleCommandQueueRecords`, `consoleMeterBlocks`, `consoleObservationTaps`, `consoleMasterTrackPlusOne` (offsets 32, 40, 48, 56) | `liveControlCommandQueueRecords`, `liveControlMeterBlocks`, `liveControlObservationTaps`, `liveControlMasterTrackPlusOne` (same offsets and types) |
| SDK `sdk/src/core/console.ts`, `EngineConsole`, `ConsoleRack` | `sdk/src/core/live-controls.ts`, `EngineLiveControls`, `LiveControlRack` |
| SDK `sdk/src/browser/console.ts`, `createBrowserConsole`, `browser.console()` | `sdk/src/browser/live-controls.ts`, `createBrowserLiveControls`, `browser.liveControls()` |

How decision 12 treats the exports (the `_v1` wire identities stay):
- They are class-2 contract identity (`docs/rulings/de-versioning-inventory.md`, "wasm export
  symbol"), pinned by `scripts/check-abi-layout-v1.py` and `sdk/assets/miso-engine-v1-abi-layout.json`.
- The version-suffix rule governs only their `_v1`, which stays.
- The stem rename is an in-place V1 amendment with no `ABI_VERSION` bump. The app and SDK update in
  lockstep.
- A retired spelling is never exported again, for any meaning. There is no alias.

Regenerate the layout JSON and `sdk/src/generated/abi.ts` from `tools/parameter-metadata`, and
update the AudioWorklet host (`hosts/host-web/web/*.js`, `.d.ts`), the qualification harness,
`scripts/check-browser-expected-resources.py` and the SDK tests. The shipped artifact's bytes
change; the artifact-identity job reports it, and the release pin moves at release
(`docs/RELEASE.md`).

Update the live docs that name the old spellings. Historical handoffs, rulings and closed specs
stay as written. Update the open specs that name them (#1053, #1054, #763).

Write an app handoff note: the old-to-new map for every public SDK name and boot word.

Authorized paths: every file that names the attachment's vocabulary, the generated layout and
bindings, the host-web web files, the listed scripts and open specs, and this spec. No behaviour
change anywhere.

## Owner decisions that bind this slice

- Decision 12's "Naming": the owner renamed the internal names and the exports, and the benchmark
  and fixture names stay. The boot words and the public SDK API follow as root's application.
- Decision 12's "Wire identity": the prelaunch identity stays V1, `_v1` stays in every export, and
  the app updates in lockstep.

## Dependencies

None beyond decision 12. It is the first slice of batch C3, and it must merge **before** *Add the
session console and per-track inserts to the session schema* (S1a, #1093). After S1a, the SDK
suites, `check-sdk-generated.sh` and the SDK-driven browser qualification are red until S1d, so
gate 3 below could not pass. It therefore also precedes *Address console slots and inserts in live
control* (S1c, #1096) and *Ship the session console and inserts in the SDK* (S1d, #1097).

## Objective gates

1. Class A. PR evidence: console digests and the browser qualification outputs are unchanged
   against the pre-change base.
2. `python3 -B scripts/check-abi-layout-v1.py` passes with the renamed exports and words. Its exact
   export list lacks the retired spellings, so an alias or a rename-back fails it.
3. `bash scripts/check-sdk-generated.sh`, `bash scripts/check-sdk-types.sh`,
   `bash scripts/check-web-audioworklet.sh` and `node scripts/test-web-audioworklet.mjs` pass, as do
   the SDK tests and the three browsers in CI mode.
4. `cargo test --workspace`, `bash scripts/check-workspace-policy.sh` and
   `python3 -B scripts/check-script-reachability.py` pass.
5. The PR lists every renamed and every kept identifier with its reason, and the app handoff note
   exists.

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

Terra, 2026-09-30. Branch `codex/1095-rename-live-controls` from `51a1514a` (the head of batch C2,
`codex/batch-console-2`). Checkpoint 1 is `acbb362d` (internal Rust names), checkpoint 2 is
`b07c32f0` (exports, boot words, SDK API), then this record. "Base" below is `51a1514a`.

### What landed

- **Checkpoint 1** (68 files). Every internal identifier whose "console" meant the attachment:
  `HostLiveControlRequest`/`HostLiveControlHandles`, `LiveControlEffectBankStage`,
  `LiveControl{Input,Matrix,Fader}Processor`, `LiveControlSoloState`/`LiveControlMuteDelta`,
  `LiveControlRecord`, `graph`'s `LiveControlEffect`, `prepare_host_{session,runtime}_with_live_controls*`,
  `prepare_{selected_,}session_builtins_with_live_controls*`, `attach_effect_live_controls`, host-web's
  `live_control_tracks`/`live_controls_attached`/`copy_live_control_track_id`/`live_control_solo`/
  `live_control_request`, the estimate's `live_control_{effect,stage}_*` terms, the bindings of
  those types, 21 test names, the test helpers that build a host or plan with live controls, and
  three test files (`effect_live_controls.rs`, `input_liveness_live_controls.rs`,
  `live_control_bank.rs`). The prose that named the attachment follows, and so do the MUTATIONS
  ledgers that cite a renamed test.
- **Checkpoint 2** (74 files). The two exports, the four boot words (offsets and types unchanged)
  with `WebBootOptions`' fields and `live_control_defaults()`, and the SDK's public live-control API
  (`core/live-controls.ts`, `browser/live-controls.ts`, `EngineLiveControls`, `LiveControlEdits`,
  `LiveControlRack`, `LiveControlChannel`, `LiveControlSubmit`, `LiveControlBeforeSubmit`,
  `LiveControlWriter`, `createBrowserLiveControls`, `engine.liveControls()`, the `liveControls`
  options key). Regenerated from `tools/parameter-metadata`: `sdk/assets/miso-engine-v1-abi-layout.json`,
  `sdk/src/generated/abi.ts`, `scripts/fixtures/abi-layout-v1-self-test.json`. The worklet host, its
  `.d.ts` and the SDK mirror, the qualification harness, the V8 harness, the open specs #1053, #1054
  and #763, and the app handoff note
  (`docs/handoffs/console-strip-2026-09-29/APP-LIVE-CONTROLS.md`: every public SDK name, boot word,
  export and diagnostic code, old to new).
- **Gate-pinned spellings moved with their names, in the same commit:**
  `scripts/check-scalar-oracle-absent.py` (checkpoint 1: its FORBIDDEN roster names the three
  per-node live strip owners; the self-test symbols are the #1059 captures with the length-prefixed
  type name re-spelled), `scripts/check-abi-layout-v1.py`, `scripts/check-web-audioworklet.sh`,
  `scripts/check-session-map-shape.py`, `scripts/check-browser-expected-resources.py`.
- **Judgment calls for the verdict.**
  - The internal diagnostic codes are renamed (`web.options.live_controls`,
    `web.live_controls.{config,input_filter,effects,observation}`, `web.internal.live_controls`,
    `host.observation.live_controls`). The spec's list does not name them. They are not pinned by
    any gate or document, and `web.live_controls.effects` would otherwise read as the S1a session
    console's effects. They are in the handoff note.
  - The browser qualification row keeps its name and output schema (`result.console`,
    `stall.console{CommandResult,MeterLeaseResult,MeterFrames}`, `stall-console-load`,
    `runConsoleQualification`, `CONSOLE_*`), so gate 1's outputs stay comparable with the base and
    `--check-matrix`'s `results.json`. Its boot words, SDK calls and prose are renamed.

### Gates

| Gate | Evidence | Result |
|---|---|---|
| 1. Class A: console digests | `console-workload`'s ignored `digests` harness in release (`gain_pan_profile`), base tree against head, same target dir: **22 of 22 rows identical**. `cargo test --locked --release -p audit -p bench -p console-workload`: 114 passed (the pinned console digests included). | identical |
| 1. Class A: browser | `check-browser-expected-resources.py --artifacts`: the committed `expected.json` digests and exact rows agree with the new module (and its self-test, 32 red mutations). Browser qualification in CI mode (`run.mjs --check-matrix --self-test-mutations`, pulseaudio null sink) on chromium 151.0.7922.34, firefox 153.0 and webkit 26.5: every gate passes and every row equals the committed `results.json`. | unchanged |
| 1. Class A: V8 harness | `run-web-mixing-automation-benchmark.sh prepare` + `preflight` at `b07c32f0`: all seven preflight digests equal the base record's (`artifacts/steps/console-strip-base/web-mixing-automation.jsonl`, `preflight_output_sha256`: quiet `014e5f5b…`, automated `e7025b5c…`, EQ `2540aff4…`, compressor `c29d12a7…`, limiter `8db18991…`). | identical |
| 1. Artifact | Shipped module `f767076a03548350a35dc6d758716c328f108de59a40cb31442ace59435a09e8` (3,270,838 B) at base -> `9003bc7d1d75f975b3c6ee8b1a27beb4e76f388e0ffd5e1918e23cc1ed6ca01f` (3,271,342 B, +504 B: export names, symbol names, diagnostic strings). The twin build in `run-wasm-gates.sh` and the V8 harness's `--module-only` build reproduce it. Not re-pinned; the release pin moves at release. | changed, reported |
| 2. ABI layout | `check-abi-layout-v1.py sdk/assets/miso-engine-v1-abi-layout.json` passes; `--self-test` 19 mutations. A copy with `miso_engine_web_v1_console_track_count` appended is refused ("116 module functions, not 117"). | green |
| 3. SDK and worklet | `check-sdk-generated.sh`, `check-sdk-deletions.py`, `check-sdk-types.sh`, `check-sdk-headless.sh` (284 of 284), `sdk-package.sh check` (tarball gate), `check-web-audioworklet.sh`, `test-web-audioworklet.sh` (runs `test-web-audioworklet.mjs`), `check-scalar-oracle-absent.py --wasm`, `run-wasm-gates.sh` (native + simd128 + V8 spill). The three browsers are row 1. | green |
| 4. Workspace | `cargo test --locked --workspace --no-fail-fast` at each checkpoint: 291 binaries, 0 failed (2,076 passed at `b07c32f0`). `check-workspace-policy.sh`, `check-script-reachability.py`. | green |
| 4. Lint job | `cargo fmt --all --check`; `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`; `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`; and all 48 script steps of the lint job, including `test-ci-path-routing.py` and `check-ci-path-routing.py`. | green |
| 4. Targets | `check-cross-targets.sh` (AArch64 iOS and Android product crates checked and linted, iOS memset ratchet at its #1018 expected counts, wasm simd128, armv7 and scalar wasm refused); the `simd128` checks of `target-smoke`, `protocol`, `dsp-reference` and `conformance`; `check-protocol-wasm-parity.sh`. | green |
| 4. Benchmarks | `test-console-benchmark.sh` (validators, no timed invocation). | green |
| 5. Classification | Below, for the PR description; the handoff note has the app's map. | done |

### Renamed and kept (for the PR description)

Renamed: every name above, with its prose, plus these test names:
`disabled_live_control_input_has_infinite_tail_but_plain_input_does_not`,
`the_per_node_live_control_input_processor_drains_and_folds_its_witness`,
`live_control_requests_are_validated_sealed_and_charged_per_track`,
`a_live_control_lane_starts_from_the_session_bypass`,
`the_observation_lane_costs_one_nullable_pointer_in_the_live_control_effect`,
`a_live_control_parameter_command_applies_at_the_next_block_boundary`,
`idle_live_controls_change_no_rendered_bit`,
`the_live_control_effect_drops_nothing_within_its_prepared_capacity`,
`live_controls_attach_bounded_control_and_meter_halves_in_canonical_track_order`,
`no_live_control_request_attaches_nothing_and_charges_nothing`,
`live_control_and_meter_preparation_keeps_spectrum_in_one_transaction`,
`uncommanded_live_controls_render_the_live_control_free_bytes`,
`the_scalar_live_control_effect_arm_maintains_its_own_live_terms`,
`seam_side_live_control_traffic_leaves_every_lane_eligible`,
`a_live_control_free_bypass_binds_at_any_automation_capacity`,
`a_bypass_or_live_controls_are_charged_at_least_what_they_retain`,
`decoded_command_resource_is_exact_for_live_control_modes_without_effects_or_meters`,
`a_live_control_free_host_refuses_every_live_kind_as_unsupported`,
`live_control_track_id_longer_than_every_source_id_boots_and_round_trips`,
`a_refused_solo_submission_leaves_the_live_controls_untouched`,
`live_controls_that_never_solo_render_what_they_always_did`.

Kept, by class (the leftover grep, `git grep -i console` outside history, is all of these):
- **Console sessions, benchmark and fixtures** (the owner's decision): `tools/console-workload`,
  `tools/bench/src/console.rs`, the `sixty_four_track_console*` rows and `SixtyFourTrackConsole*`,
  the `console-sixty-four-track*` and `parametric-eq-bank-console` fixtures and their scripts, the
  console benchmark scripts and records (including the V8 record key
  `console_command_queue_records`, which committed records carry), and the test helpers that build
  or render those sessions (`intended_console`, `compile_console_model_*`, `render_console_*`,
  `scalar_console_registry`, `console_track_*_binding`, `ArmedConsoleRender`, `bypassed_console`,
  `representative_console`, the bypass tests' `console(tracks, masks)`, the `Console` structs that
  hold a prepared console session in `limiter_linked_session.rs`, `effect_live_controls.rs` and
  `symmetry_witness.rs`, `the_hot_console_*`, `console_run`, the randomized console generators).
- **The session console** (decision 12): "console strip", console slots, `console pre-insert`,
  AGENTS.md's chain line.
- **Real mixing consoles**: "physical console", "console-correct", "a live mixing console",
  `dsp-research`.
- **JavaScript's `console`**: `console.log`, `page.on("console")`, and the worklet's
  `console.`-policy mutations.
- **The browser qualification row and its fixture identities** (output schema, above):
  `console-session.json`, `console-source`, `web-browser-console`.
- **History**: `docs/handoffs`, `docs/rulings`, `docs/audits`, `docs/derivations`, `artifacts/`, and
  closed or batch-C2 specs. #1053's title stays, since it is the GitHub issue's title.
- **External**: `misofm/engine-web-adapter/src/console.ts` (a cited source baseline).

## Sol verdict, attempt 1

**PASS.** Sol, 2026-09-30, on head `bca6d777`. Every objective gate holds, and every claim that
could be tested was reproduced independently. Nothing but names and diagnostic strings moves. There
are no H findings. There is one M: the browser qualification harness keeps attachment-sense
names on a rationale that does not hold. It is harness-only, and it is best fixed on this branch
before the batch boundary. Every probe below was reverted, and the tree is clean.

### Findings

- **M1. The browser qualification row keeps live-control names on an inaccurate rationale.**
  - **Where.**
    - `hosts/host-web/qualification/qualification.js`:
      - `:10-15`: `CONSOLE_BLOCKS`, `CONSOLE_FRAMES`, `CONSOLE_COMMAND_QUEUE_RECORDS`,
        `CONSOLE_METER_BLOCKS`;
      - `:273`: `runConsoleQualification`;
      - `:561-563`: `consoleCommandResult`, `consoleMeterLeaseResult`, `consoleMeterFrames`;
      - `:624` and `:698`: `console`.
    - `hosts/host-web/qualification/run.mjs:26` and `:584`: `stall-console-load`, and
      `:145` and `:193-194`.
    - `hosts/host-web/MUTATIONS.md:34`.
  - **These are the attachment sense.** The row boots with live controls (queue 64, meter blocks
    2), admits a command and leases a meter (#137 E6/E8). It is neither a benchmark nor a console
    session, so "benchmark names stay" does not cover it. The prose beside it was renamed
    (`qualification.js:265`, "live-control row"; `run.mjs:25`, "live-control load"), so the code
    and its comments now disagree.
  - **The stated reason does not hold** (spec line 188). The reason given is that the keys stay
    comparable with the base and with `--check-matrix`'s `results.json`. But `results.json`
    carries none of these keys: its gates are `controlPath`, `observation`, `mainThreadStall` and
    the rest. The in-run digests do not depend on key names either. Renaming would not have cost
    that comparability.
  - **Correctly kept.** `console-session.json`, `console-source` and `web-browser-console` are
    fixture identities. The source ID is inside the document's canonical identity
    (`session-identities.mjs`).
  - **Fix.** A mechanical, harness-only rename. No artifact or `results.json` byte moves.
- **L1. `scripts/test-web-audioworklet.sh:164-201` is misclassified as JavaScript's `console`**
  (spec line 249).
  - The names: `console_mutations`, `worklet-console.js`, "console policy mutation matched
    nothing", "console process-policy mutation escaped", and "web AudioWorklet console policy
    mutations passed".
  - These are the #137 D2/D3 rules for the live controls' posts, lease guard and clock, not
    `console.` calls. Rename them with M1.
- **L2. The published export list is no longer sorted.**
  - `tools/parameter-metadata/src/abi_layout.rs:125` documents "Every function the module
    exports, sorted".
  - But `:142-143` puts `live_control_track_*` between `command_submit` and `dispose`. So do
    `scripts/check-abi-layout-v1.py:116-117`, `sdk/assets/miso-engine-v1-abi-layout.json`,
    `sdk/src/generated/abi.ts:44-45` and `scripts/fixtures/abi-layout-v1-self-test.json:17-18`.
  - No gate depends on the order: `check-web-audioworklet.sh` sorts both sides, and the layout
    gate compares the exact frozen sequence. Either re-sort the list (after
    `input_filters_prepare`) or correct the comment.
- **L3. The recorded method for the gate 1 digests can silently compare head with itself.**
  - The record says: "base tree against head, same target dir".
  - A base tree extracted with `git archive` has mtimes older than the build. Run into the same
    target dir, cargo reused head's test binary: 0 crates compiled, finished in 0.07 s. That
    compares head with itself.
  - The base rebuilt in a separate target directory compiled 63 crates and still matched 22 of
    22, so the claim holds. The record should say how the base was built.
- **L4. Borderline kept names, for S1c.**
  - The names:
    - `render_console_fader_script` (`crates/graph-compiler/src/lib.rs:11722`) renders the
      intended console fixture under a live fader script.
    - The `Console` test structs (`crates/host-core/tests/symmetry_witness.rs:104`,
      `crates/host-core/src/limiter_linked_session.rs:315`) hold a prepared console fixture with
      its `HostLiveControlHandles`.
  - Read by what they construct, the session reading is defensible. No action is needed unless
    S1c edits them.

### The judgment calls

- **The diagnostic codes: renaming them is correct.**
  - **Not frozen.** `docs/EFFECT_CONTRACT_V1.md:202` ("Stable diagnostics") freezes only
    `effect.*`. At base, nothing matched `web.console.*`, `web.options.console`,
    `web.internal.console` or `host.observation.console` except their producers, one host-core
    test and a comment in `sdk/src/browser/host-mirror.ts`. No gate and no SDK code matched them.
  - **App-visible, though.** They reach the app as a boot error's `diagnosticCode`, so the
    handoff note is the right place for them.
  - **Every consumer is updated:**
    - `crates/host-core/tests/effect_observation.rs:814`;
    - the `host-mirror.ts:55` comment;
    - the handoff note's table.
  - **The wasm diff agrees.** Only two string-length immediates moved: 18 -> 24
    (`web.console.config`) and 19 -> 25 (`web.options.console`).
- **The qualification row: not consistent with "benchmark names stay".** See M1.

### Evidence

- **Completeness.**
  - I walked through every `console` token that `git grep -i console` finds outside history: 160
    distinct tokens.
  - Every M6 name and every name in the spec's list is renamed. The retired export, boot-word and
    SDK spellings have zero hits outside history and this spec.
  - Nothing that means a console session was renamed:
    - the benchmark rows and records, including `console_command_queue_records`;
    - `console-workload`;
    - the fixtures;
    - decision 12's key;
    - the console-strip docs.
  - The kept list is right except M1 and L1.
  - Assert, `#[test]` and `#[ignore]` counts are unchanged: 14,919, 2,099 and 46.
  - No hex digest pin changed in the diff.
- **Sealed spellings.**
  - The `_v1` and `miso.*.v1` tokens in the diff's removed and added lines are identical except
    the two exports. `ABI_VERSION` is unchanged.
  - The boot-word offsets (32, 40, 48, 56) and types are unchanged.
  - The worklet `.d.ts` and its SDK mirror are byte-identical.
  - The layout is byte-equal to the generator's output.
- **Planted probes, all reverted.**
  - The layout gate refused:
    - a copy with the retired export appended ("116 module functions, not 117");
    - a rename-back ("frozen module function sequence");
    - `consoleMeterBlocks` restored.
  - The scalar-oracle gate is not weakened. Its re-spelled roster, run `--native` against a real
    debug `libbuiltins_compiler` rlib, matched real `LiveControl{Input,Fader,Matrix}Processor`
    symbols (10, 41 and 41 symbols). On the shipped module: 2,481 symbols, none of the 17 roster
    names.
- **Class A.**
  - **Module.** Base, built from `51a1514a` source, is `f767076a…` (3,270,838 B). That equals
    the digest CI recorded on `origin/main` `398e8988`. Head is `9003bc7d…` (3,271,342 B).
    - The 117 exports differ only in the two renamed exports.
    - The type section is identical, and the code section has the same size (2,786,773 B) and
      the same 2,481 functions.
    - With names and static addresses normalized, 2,474 bodies are identical. The other 7
      differ only in mangled names or in the two string lengths above.
    - The data section grew by 40 B: the strings.
  - **Console digests.** `gain_pan_profile digests` in release: base rebuilt from source against
    head, 22 of 22 identical. `cargo test --release -p audit -p bench -p console-workload`: 114
    passed.
  - **V8.** Head's `prepare` and `preflight`: all seven digests equal the base record's. A base
    preflight (base module, byte-identical `controls.json`) against head: the seven arm digests
    and both document digests are identical.
  - **Browsers.** Chromium 151.0.7922.34, Firefox 153.0 and WebKit 26.5, with `--check-matrix
    --self-test-mutations` and a PulseAudio null sink: every gate passed.
- **Gates, all green.**
  - Rust:
    - `cargo fmt`;
    - clippy with `-D warnings` on all targets and all features;
    - rustdoc with `-D warnings`;
    - `cargo test --workspace`: 291 binaries, 2,076 passed, 0 failed.
  - Lint job: all 46 script steps and the three x86 probes.
  - Routing and benchmark: `test-ci-path-routing.py`, `check-ci-path-routing.py`, and
    `test-console-benchmark.sh` (0 timed invocations).
  - SDK:
    - generated, deletions and types;
    - headless: 284 of 284;
    - the package tarball gate.
  - Artifact:
    - `check-web-audioworklet.sh`;
    - `check-browser-expected-resources.py --artifacts`: 32 red mutations;
    - the scalar-oracle gate with `--wasm` and `--self-test`;
    - `test-web-audioworklet.sh`;
    - the V8 spill gate and its self-test;
    - `check-session-map-shape.py`;
    - `check-abi-layout-v1.py` and its self-test: 19 mutations.
  - Targets and wasm:
    - `run-wasm-gates.sh`: native, simd128 and V8;
    - `check-cross-targets.sh`: AArch64 iOS and Android checked and linted, at the #1018 expected
      counts, and wasm simd128;
    - the simd128 compile probes;
    - `check-protocol-wasm-parity.sh`.
- **Not run.** AArch64 tests on hardware (there is no arm64 host), and CI's twin-path
  `artifact-identity` job. The base module built from a different path reproduced CI's digest,
  which is evidence that the recipe is path-independent.

### Merge

- **Onto `origin/main` `398e8988`: clean.** `git merge-tree` exits 0. `origin/main` differs from
  the base only in the umbrella spec.
- **Against S1a `c5699847`: three files conflict.**
  - Two of them conflict in exactly the same way when S1a is merged with the base alone. They are
    C2's conflicts, not the rename's:
    - `crates/effect-compiler/src/prepare.rs` (1 hunk);
    - `crates/effect-compiler/tests/native_session.rs` (2 hunks).
  - The rename adds one hunk: the `effect_compiler` import in `crates/graph-compiler/src/lib.rs`
    (`attach_effect_live_controls` against S1a's
    `prepare_native_session_effects_with_console_eligibility`).
  - S1a's added lines use no retired live-control spelling. Its `with_console*` names are the
    session sense.
  - `codex/1093-session-console-inserts` has since moved to `616427aa`, which merges this branch
    into S1a.
