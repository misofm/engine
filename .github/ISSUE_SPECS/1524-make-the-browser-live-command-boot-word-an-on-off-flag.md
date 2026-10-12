# Make the browser live-command boot word an on/off flag

DRAFT, not filed. Stream B of decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-2).
Successor of *Hold effect parameter, bypass and EQ-target values in latest-target cells* (#1345),
from its attempt 1 open item 4 and verdict m1 (`/home/bl/misofm/submix-verdicts/1345-attempt1.md`,
"m1"). Code anchors verified on `codex/d15-stream-b3` at `3bcd99ebe` (#1345 attempt 1 is on that
branch; attempt 2 is pending). The anchors move after #1345 attempt 2: re-verify every anchor
before you start. In a later check, the `lib.rs` anchors of `hosts/host-web` had already moved by
1 to 35 lines.

Root decisions 2026-10-12 after an adversarial review: layout option (A), a 0/1 flag at the same
offset, with every other value refused; this issue lands after #1345, before H #1332 and C #1406;
the in-flight bound counts records up to MAXIMUM_COMMAND_RECORDS.

Proposed file name: `NNNN-make-the-browser-live-command-boot-word-an-on-off-flag.md`.

**Why this is its own issue.** The whole removal of the live-control queue depth touches about 75
files: 30 Rust files for the host-core field and about 45 browser, SDK, harness and benchmark files
for the boot word. That does not fit half a day (AGENTS.md, "split it up front"). This issue is the
browser half. It is an ABI-mirror change with frozen decisions and is about half a day. It cannot be
split further: the layout checker, the host mirrors, the SDK option and the benchmark boot values
must all change in the same commit, or the 0/1 refusal fails them. The Rust half is the sibling
issue *Replace the live-control queue depth with an attach flag in host-core*.

## Product outcome

A browser app opts in to live commands with one on/off word, `liveControlCommands`, set to 0 or 1,
and through the SDK with `policy.liveControls.commands: true`. It no longer chooses a "command queue
depth" that the engine does not have. A plan booted without live commands stays the same generic
capability (#1509). The boot struct keeps its size and offsets, and `ABI_VERSION` does not change.

## Context

- **The word.** `WebBootOptions::live_control_command_queue_records: u64` at offset 32 of the
  64-byte struct (`hosts/host-web/src/lib.rs:1226-1232`). Its doc still names a per-track queue
  depth (#1345 attempt 2 corrects it).
- **Its roles today.**
  1. The opt-in, `!= 0`. It is read by:
     - `options_enable_commands` (`lib.rs:7465-7470`) → `ReadyOwnership::commands_enabled`
       (`:1529`). That gates the command entry, the input-filter shadows and the live VCA state;
     - `live_lanes` (all lanes, or fader and matrix only);
     - the command staging bytes (`:6336-6340`: `MAXIMUM_COMMAND_RECORDS` records, or 0);
     - the rule that observation taps need it (`:6476-6478`).
  2. The upper bound, `<= MAXIMUM_COMMAND_RECORDS` (`:6482`). It bounds a number that sizes
     nothing.
  3. The automation-span coupling, `.max(records)` (`:6764`). #1345 attempt 2 removes it (verdict
     m1).
  4. The main-realm host's in-flight command bound. `#saturated` refuses a command locally when
     `#inFlightCommands >= #commandQueueRecords`
     (`hosts/host-web/web/miso-engine-v1-audio-worklet-host.js:1181-1186`). The value comes from
     `Number(bootOptions.liveControlCommandQueueRecords) || 1` (`:1759`). Its comment calls this
     "the worklet-side queue depth", which no longer exists.
     - No command ring exists. The bound limits unsettled `miso.command.v1` and
       `miso.prepared-command.v1` messages in the port.
     - It cannot become unbounded. The worklet handles each message on the audio thread:
       `receiveCommand` copies the records into staging and submits synchronously
       (`miso-engine-v1-audio-worklet.js:1538-1566`). Without a local limit the port queue has no
       bound and no typed backpressure, which breaks the AGENTS.md rule that a full queue returns
       typed backpressure.
     - It cannot come from declared lanes. Lanes are latest-target cells with no capacity, and one
       message can address any lanes.
     - It cannot be 1. That makes parallel `host.command` calls fail locally. The SDK writer
       serializes its own flushes (`writer.ts:260`), but `EngineLiveControls` calls `host.command`
       directly (`sdk/src/browser/live-controls.ts:57`).
- **The opt-in must stay.** #1509 keeps "a plan prepared without live controls (browser boot option
  `liveControls.commandQueueRecords` = 0)" as a generic capability. Without the opt-in, every plan
  would pay the command staging (256 × 48 bytes), the input-filter shadows, the effect and route
  lanes, and the loss of route folds. A reserved must-be-zero word, or a smaller layout, would
  remove the opt-in's home, so each would need a new home for one bit. Root chose to keep the word
  and give it the opt-in meaning (D1).
- **Prior layout rules.**
  - D15-12's growth rule covers additions only: "a feature bit per addition"
    (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md:410`).
  - The browser records require reserved fields to be zero (`docs/rulings/825-browser-observation-abi-handoff.md:7`).
  - #1206 and #1243 renamed C words in place, with the layout unchanged. The unfiled #1058 draft 22
    proposes to reserve a C word in place
    (`docs/handoffs/stored-automation-1058/proposed-specs/22-reserve-the-c-abi-automation-span-limit.md`).
  - No rule covers narrowing a browser boot word. Root settled the layout as option (A), a rename in
    place (D1).
- **`DEFAULT_COMMAND_QUEUE_RECORDS = 64`** (`lib.rs:602-603`), published as
  `constants.defaultCommandQueueRecords`. Its readers:
  - `WebBootOptions::live_control_defaults` (`:1278`);
  - the SDK writer's default `maximumBatch` (`sdk/src/core/writer.ts:207`);
  - two benchmark scripts;
  - tests.
- **Every reader and writer of the word** (`git grep -c`; the camelCase spellings are
  `liveControlCommandQueueRecords`, `commandQueueRecords` and `COMMAND_QUEUE_RECORDS`; the
  benchmark record keys are `console_command_queue_records` and `live_control_command_queue_records`):
  - Engine:
    - `hosts/host-web/src/lib.rs` (12 sites);
    - `hosts/host-web/src/ffi.rs:516`, `:6748`, `:6842`;
    - `hosts/host-web/src/tests.rs` (47 sites);
    - `hosts/host-web/tests/render_locked_source.rs:53`, `render_locked_staging.rs:85`,
      `strip_cells_cross_host.rs:220`, `:402`.
  - Layout tool and checker:
    - `tools/parameter-metadata/src/abi_layout.rs:65`, `:296-299`, `:2174`;
    - `tools/parameter-metadata/tests/abi_layout.rs:210`, `:365`, `:383-384`;
    - `tools/parameter-metadata/tests/round_trip.rs:411`, `:566`, `:708`;
    - `scripts/check-abi-layout-v1.py:278-290` (`BOOT_OPTION_FIELDS`, `POLICY_WORDS`), `:505`,
      `:563`;
    - `scripts/fixtures/abi-layout-v1-self-test.json:142`, `:643`;
    - generated: `sdk/assets/miso-engine-v1-abi-layout.json:142`, `:643` and
      `sdk/src/generated/abi.ts:198`, `:2402`.
  - Worklet JS:
    - `hosts/host-web/web/miso-engine-v1-audio-worklet.js:75`, `:564`, `:889-898`, `:1550`;
    - `hosts/host-web/web/miso-engine-v1-audio-worklet-host.js:74`, `:497`, `:608-614`, `:726`,
      `:752`, `:767`, `:1181-1186`, `:1759`;
    - `hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts:43`, `:804`, `:811`, and the same
      lines in `sdk/src/browser/shipped-host.d.ts`.
  - SDK source:
    - `sdk/src/core/abi.ts:122`, `:144`, `:189`;
    - `sdk/src/core/boundary.ts:205`, `:211`, `:1222`;
    - `sdk/src/browser/engine.ts:634-636`;
    - `sdk/src/browser/host-mirror.ts:45-93`;
    - `sdk/src/core/writer.ts:207`;
    - `sdk/README.md:155`, `:187`.
  - SDK tests:
    - `sdk/test/live-controls-evals.mjs` (13), `console-evals.mjs` (9), `capability-evals.mjs` (8),
      `browser-defaults-evals.mjs` (7), `browser-evals.mjs` (5);
    - 3 each: `measurement-evals.mjs`, `package-tarball-smoke.mjs`, `writer-evals.mjs`;
    - 2 each: `observation-evals.mjs`, `observation-subscription-evals.mjs`;
    - 1 each: `agent-evals.mjs`, `host-mirror.ts`, `live-response-evals.mjs`,
      `response-subscription-evals.mjs`, `spectrum-browser-evals.mjs`.
  - Harnesses:
    - `scripts/test-web-audioworklet.mjs` (12; the saturation test is at `:1786-1803`);
    - `hosts/host-web/qualification/qualification.js` (7), `sdk-response-entry.ts` (6),
      `suspended-host.mjs` (4), `rebuild-cost.mjs` (3);
    - `hosts/host-web/tests/browser-v1/browser-correctness.js:13`,
      `direct-oracle.mjs:21`, `:379`, `:469`.
  - Benchmark records:
    - `scripts/web-mixing-automation-benchmark.mjs:104`, `:284`, `:386`, `:779`, `:1088`;
    - `scripts/web-mixing-automation-lib.jq:26`, `:86` (claim `== 64`);
    - `scripts/test-console-benchmark.sh:1482`, `:1569`, `:1823`.
  - The C ABI has no such word.
- **Records that are not edited.** `docs/derivations/243-sdk-boot.md`, `docs/handoffs/**`,
  `artifacts/steps/**` and closed or landed spec text. These open specs quote the old name and are
  root's issue-boundary sync: #1509 (`:15`), #1470 (`:80`, `:125`), #1326, #1391 and #379.

## Decisions frozen

- **D1. Rename in place; layout unchanged (layout option A).** Offset 32 stays a `u64`, renamed
  `live_control_commands` (JS `liveControlCommands`).
  - 0 means no live commands. 1 means live commands. Any other value is refused at boot with
    `web.options.live_controls`, by the engine, by `validBootOptions` in the main-realm host and by
    the SDK's `host-mirror.ts` (caller time). A writer that still passes 64 therefore fails loudly.
  - `BOOT_OPTIONS_BYTES` (64), every offset and `ABI_VERSION` stay unchanged.
  - The observation-taps rule keeps its form (taps need the flag).
- **D2. In-flight bound.** The main-realm host bounds unsettled command records, not messages: a
  `command` or `preparedCommand` whose `count` would take the unsettled record total above
  `MAXIMUM_COMMAND_RECORDS` is refused locally with 6, before any transfer, no message posted and no
  request ID burned. The `#commandQueueRecords` field and its constructor argument go. The reason,
  stated in the comment: the worklet copies each message into the one
  `MAXIMUM_COMMAND_RECORDS`-record staging on the audio thread, so the port holds at most one
  staging's worth of records. No engine queue sits behind the bound, and no lane count sizes it.
- **D3. Constants.**
  - `DEFAULT_COMMAND_QUEUE_RECORDS` and the layout constant `defaultCommandQueueRecords` are
    deleted.
  - `live_control_defaults()` sets `live_control_commands: 1`.
  - The SDK writer's default `maximumBatch` becomes `ABI_LAYOUT.constants.maximumCommandRecords`
    (256); `writer-evals.mjs` is updated to the new default.
- **D4. SDK option.** `policy.liveControls.commandQueueRecords?: number` becomes
  `liveControls.commands?: boolean`. Absent or `false` means no live commands, as 0 does today.
  - `engine.ts` and `boundary.ts` messages name `liveControls.commands`.
  - `host-mirror.ts` maps the option to `1n`/`0n`.
  - No SDK version bump; releases own that (`docs/RELEASE.md`).
- **D5. Benchmark records.** The web-mixing-automation record and the web-rebuild-cost record
  rename their key to `live_control_commands` (value `true`) and bump `schema_version` 1 → 2.
  `web-mixing-automation-lib.jq` claims `.live_control_commands == true`. Stored records under
  `artifacts/steps/` are not rewritten.
- **D6.** Names are unversioned and follow #1509's naming rule.

## Authorized paths

- Every file in the Context inventory, for the rename and for D1-D5 only.
- `hosts/host-web/src/lib.rs`: the field, its doc, `live_control_defaults`, the staging-bytes
  read, the validation, `options_enable_commands` and `live_control_request`.
- Regenerate `sdk/assets/miso-engine-v1-abi-layout.json` and `sdk/src/generated/abi.ts` with the
  generator. Never edit them by hand.
- This issue's own spec file.

## Non-goals

- Moving the opt-in elsewhere (a reserved word at offset 32 with the opt-in in `reserved0`, or a
  56-byte struct), or making live commands always on. Root chose option (A), and #1509 keeps the
  opt-in as a generic capability.
- `MAXIMUM_COMMAND_RECORDS` (the per-submission staging, a real bound).
- The Worker rewire (#1332). Records, handoffs and closed specs.

## Objective gates

1. **The word is an on/off flag.**
   - Rust: no Rust test refuses a too-large word today; only the JS harness does. Add two rows to
     `each_boot_option_rule_has_its_own_typed_refusal` (`hosts/host-web/src/tests.rs:782`):
     `live_control_commands: 2` and `1 << 32`, each refused with
     `RESULT_REFUSED_OPTIONS` and the diagnostic `web.options.live_controls`.
     `WebBootOptions::live_control_defaults()` (word 1) still boots in
     `observation_configuration_words_are_validated` (`:6909`).
   - Mutation (PR evidence): restore `> u64::from(MAXIMUM_COMMAND_RECORDS)` as the engine bound; the
     `2` row turns red. Mutation: check `(word as u32) > 1`; the `1 << 32` row turns red (it
     truncates to 0 and would boot as off).
   - JS (`scripts/test-web-audioworklet.mjs:836`, today "command queue bound" with `257n`): the
     case becomes `2n`, which is refused.
   - SDK: `commands: true` boots with live controls. Absent or `false` makes `getLiveControls`
     reject with the new message (`browser-defaults-evals.mjs:422-456`, rewritten).
2. **The opt-in is unchanged (rewritten, not new).**
   - `a_live_control_free_host_refuses_every_live_kind_as_unsupported` (`tests.rs:4915`) runs
     with word 0.
   - The resource-projection loop at `tests.rs:3503-3620` ("off"/"on", which charges the command
     staging and the input-filter shadow bytes) runs with 0 and 1.
   - Mutation (PR evidence): make `options_enable_commands` return `true` always. The
     live-control-free test turns red.
3. **In-flight bound (rewrite `test-web-audioworklet.mjs:1786-1803`).** Hold unsettled submissions
   that total `MAXIMUM_COMMAND_RECORDS` records (two of 128). The next one-record command and the
   next prepared command are refused locally with 6, no message is posted and no request ID is
   burned. After one settles, a 128-record command is admitted.
   - Mutation (PR evidence): count messages instead of records against any constant at least 3; the
     third submission is not refused, red.
   - Mutation: leave the bound reading the deleted option; red.
4. **Layout.**
   - `parameter-metadata`'s `abi_layout` tests: `liveControlCommands` is at offset 32, the struct
     is 64 bytes, and `defaultCommandQueueRecords` is absent. These pins are allowed because this
     is an ABI layout.
   - `scripts/check-abi-layout-v1.py --self-test` passes with the regenerated fixture.
   - `bash scripts/check-sdk-generated.sh` passes.
5. **No moved bit or byte (PR evidence).** No retained-bytes, budget or ceiling test is re-pinned.
   The struct size does not change, and #1345 attempt 2 already removed `.max(records)`.
6. **Commands.**
   - `cargo test --locked -p host-web --all-features`
   - `cargo test --locked -p parameter-metadata`
   - `python3 scripts/check-abi-layout-v1.py --self-test`
   - `bash scripts/check-sdk-generated.sh`, `bash scripts/check-sdk-types.sh`,
     `bash scripts/check-sdk-headless.sh`
   - `bash scripts/test-web-audioworklet.sh`, `bash scripts/test-console-benchmark.sh`
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
   - `bash scripts/check-workspace-policy.sh`

## Test value

- Gate 1 (two new Rust rows): red if the engine accepts a value other than 0 or 1. A writer could
  then still believe that it sizes a queue, or a stale 64 could boot without notice. Today no Rust
  test refuses any value of this word. The `1 << 32` row: red if a narrowing cast in the check boots
  a nonzero, non-1 word as "off".
- Gate 3: red if the host's local bound still reads the removed option. That defect makes the
  bound vanish without a sign. No other test holds this bound. It is also red if the bound counts
  messages, which lets the port carry an unbounded number of records.
- Gate 2 is rewritten, not new. It keeps its catch (the opt-in gates commands, lanes and shadows).

## Dependencies

- The sibling issue *Replace the live-control queue depth with an attach flag in host-core*. It
  removes the depth that the word fed.
- #1345 attempt 2. It removes `.max(records)` and corrects the field docs. This issue must not land
  first, or it would have to make the same edit.
- This issue lands before H #1332 (which rewires boot into the Worker) and before C #1406 (see
  Placement).

## Placement (root edits `STREAMS.md`)

Stream B, after #1345 and after the sibling issue *Replace the live-control queue depth with an
attach flag in host-core*. It lands before H #1332 and before C #1406. Hot-file rows:

- `hosts/host-web/src/lib.rs` (H owns): this issue lands before H #1332, like the other B slices in
  the row.
- `tools/parameter-metadata/src/abi_layout.rs` and `scripts/check-abi-layout-v1.py`: after B
  (#1399, #1349), before C #1406.
- `hosts/host-web/web/miso-engine-v1-audio-worklet-host.{js,d.ts}` and
  `sdk/src/browser/shipped-host.d.ts`: before C #1406. This issue edits the command arm of
  `#saturated`, and #1406 edits the ring arm (the per-source ring bound) of the same function. The
  two are independent changes to one function.
- `hosts/host-web/src/tests.rs`, `scripts/test-web-audioworklet.mjs`: in any order with stream H
  and J #1496; the later slice rebases.

## Attempt record

- Attempt 1: not started.
