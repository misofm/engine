# Resolve an absent live ramp to the session default on the browser and in the SDK

Stream E of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-1).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

A browser app that sends a live fader, mute, pan, matrix, solo, trim, polarity, send or VCA edit
without `smoothingSamples` hears it ramp over the session's `control_smoothing`, or over the
default table. Today it steps, because the SDK fills in 0. An explicit `smoothingSamples`, 0
included, still overrides. The SDK builder can author `controlSmoothing`. For the same session and
the same edits with no per-edit length, the browser and the C ABI render the same bits. Decision
14's F7 (the SDK half) is closed.

## Context

- **The SDK defaults to a step.**
  - `smoothing()` returns `options.smoothingSamples ?? 0` (`sdk/src/core/live-controls.ts:298-300`),
    and `trackEdit` defaults to 0 too (`:416`).
  - Every ramped row calls `smoothing()`: `pan` `:557`, `matrix` `:567`, `faderDb` `:579`, `mute`
    `:587`, `trimDb` `:595`, `polarityInvert` `:603`, `solo` `:734`, the send's `gainDb`, `mute` and
    `matrix` (`:766`, `:774`, `:782`), and the VCA's `faderDb` and `mute` (`:810`, `:819`).
- **The wire.** The 48-byte command record carries `smoothing_samples` as the `u32` at offset 16
  (`hosts/host-web/src/lib.rs:4118-4130`), decoded in `CommandRecord::decode` (`:4144-4205`).
  The rows read it in these places:
  - `into_track_record` (`:4242`), for pan, matrix, fader, mute, trim and polarity;
  - `into_route_edit` (`:4359`) and `into_vca_edit` (`:4391`);
  - the solo and VCA-fader coalescing in `admit_commands_staged` (`:4889`, `:4917`).

  Input filters and bypass require the word to be 0 (`:4210`, `:4492`). Observe reads it as a window
  (`:4480`). The effect-parameter path never reads it, because effect ramps are the descriptor's.
- **The table.** *Session `controlSmoothing`: configurable ramp lengths for live mute, fader and
  pan changes* (#1054) adds `SessionModel::control_smoothing_samples`, and `LiveRamps` with a field
  per key and `for_row(row)`. *Carry an optional per-edit ramp length on live session edits*
  (#1394) adds `LiveRamps::resolve(row, edit_ramp)` and the transaction field. #1054's D3 maps
  each row to a key: fader, trim, send gain and VCA offset use `fader`; mute, solo, polarity, send
  mute, send `follows_mute` and VCA mute use `mute`, polarity at twice its length (`for_row` gives
  `2 * mute_samples`; root, 2026-10-05, from #1055); pan, matrix and send matrix use `pan`. #1054
  D5 sets the precedence for a strip's pan or matrix record: the strip's model
  `smoothing_samples` (`MatrixOrPan`, `crates/session/src/model.rs:655-677`) if non-zero, else
  `pan_samples`. #1054 makes `control_smoothing` a required tagged root key (`default` or
  `explicit`), teaches it to the JSON schema (`docs/session-v1.schema.json`) and to the SDK's
  canonical writer (`sdk/src/internal/session-json.ts:13-29`, `:50-65`), and has the builder write
  `{ "kind": "default" }`.
- **The authoring request.** The CLI's strict request decoder lists the root keys it accepts
  (`sdk/src/cli/session-request.ts:504`) and builds through `session()` (`:508-513`). Without a
  `controlSmoothing` key there, a request cannot author the setting.
- **Browser boot.** host-web prepares through host-core
  (`hosts/host-web/src/lib.rs:2027`, `:6661`).
- **The SDK builder** emits the session model in `sdk/src/core/session.ts:1645-1662`.
- **Existing gates.**
  - The native host-web leg is tied to the shipped artifact by the #106 F4 native-to-simd128
    session parity gate (`hosts/host-web/Cargo.toml` dev-dependency note).
  - No crate links both hosts today.

## Decisions frozen for this slice

- **D1. One sentinel.** `SESSION_DEFAULT_RAMP: u32 = u32::MAX` in the wire word means "no length
  given". It is documented in the record table (`lib.rs:4118-4130`) and exported by the SDK.
- **D2. The engine resolves it.**
  - At boot, host-web computes `LiveRamps::for_session` from the session it prepared, and keeps it
    with the live controls. That is control-thread work, done once.
  - For the ramped kinds, admission replaces a sentinel with `for_row(row)` for the kind's #1054 D3
    row (the D3 key's field; twice `mute_samples` for polarity; root, 2026-10-05, from #1055),
    before the kind's own checks.
  - For a strip's pan and matrix kinds, the replacement follows #1054 D5's precedence: the strip's
    `smoothing_samples` in the session host-web prepared, if non-zero, else `pan_samples`. Host-web
    keeps those per-strip windows beside `LiveRamps`, computed on the control thread whenever it
    prepares a session. Send matrix records take `pan_samples`.
  - Every other word value passes through unchanged. A finite length the SDK sends is still
    honoured.
  - The kinds with no ramp keep their rules. A sentinel on input filters or bypass is `MALFORMED`,
    as any non-zero word is today.
- **D3. SDK.**
  - `smoothing()` returns `SESSION_DEFAULT_RAMP` when `smoothingSamples` is undefined. An explicit
    `u32::MAX` is a `MisoUsageError`, because the sentinel is not a length.
  - `trackEdit`'s default stays 0 for the kinds that carry no ramp.
  - `runtimeSmoothing` (effect parameters) is unchanged.
  - (Amendment 1) A literal `LaneEdit` with no `smoothingSamples` resolves by its kind through one
    shared SDK helper: `SESSION_DEFAULT_RAMP` for the twelve ramped kinds, 0 for bypass, input
    filters and observation. `encodeLaneEdits` (`sdk/src/core/writer.ts`) and `browserCommand`
    (`sdk/src/browser/live-controls.ts`) use it, and `smoothing()` is the same rule, so the builder
    and a hand-written edit cannot disagree. An explicit `u32::MAX` on a ramped kind is a
    `MisoUsageError` on every path.
- **D4. Builder.** `SessionOptions.controlSmoothing?: { muteMs, faderMs, panMs }` emits
  `control_smoothing: { kind: "explicit", mute_ms, fader_ms, pan_ms }`. It checks #1054 D1's
  bounds with the builder's existing finite and range helpers. Omitted, the builder writes
  `{ kind: "default" }` as #1054 left it; the key is never omitted (V1 has no optional fields).
- **D5. Acked-batch question.** No queue, admission order or ack changes. Resolution is a pure
  rewrite of one word before the existing checks.

## Deliverables

1. D1 and D2 in `hosts/host-web/src/lib.rs`.
2. D3 in `sdk/src/core/live-controls.ts`, and the docs of every `smoothingSamples` option.
3. D4 in `sdk/src/core/session.ts` (`SessionOptions`, `:117-124`, and the model emit), and the
   authoring request's `controlSmoothing` key in `sdk/src/cli/session-request.ts` (`:504`), passed
   to `session()`. The canonical writer (`sdk/src/internal/session-json.ts`) and the JSON schema
   already carry the key from #1054, so `toJson()` writes what D4 emits; this slice does not edit
   them.
4. The parity test (gate 3), with `capi` added as a dev-dependency of `host-web`, tests only.
5. The tests below.

## Authorized paths

- `hosts/host-web/src/lib.rs`, `hosts/host-web/src/tests.rs`, `hosts/host-web/Cargo.toml`
  (dev-dependency only), `hosts/host-web/tests/control_smoothing_parity.rs` (new). Stream H owns
  host-web; root sequences the merge.
- `sdk/src/core/live-controls.ts`, `sdk/src/core/session.ts`, `sdk/src/cli/session-request.ts`,
  `sdk/test/live-controls-evals.mjs`, `sdk/test/builder-evals.mjs`
- (`sdk/src/internal/session-json.ts` and `docs/session-v1.schema.json` are #1054's.)
- `sdk/src/browser/shipped-host.d.ts` (docs only)
- (Amendment 1) `sdk/src/core/writer.ts`, `sdk/src/browser/live-controls.ts`

## Non-goals

- No effect-parameter ramp change: effect ramps come from the descriptor.
- No bypass crossfade: *Crossfade the bypass switch over the session ramp* (#1341), and for the
  browser command *Crossfade the browser's live bypass command over the session ramp* (#1393),
  which reads the `LiveRamps` D2 keeps.
- No transaction edit of the key: *Edit control_smoothing by a session transaction, model-only*
  (#1365).
- No change to the browser's control-plane location (decision 15 D15-10). If the Worker control
  plane has landed, D2 goes wherever admission lives then: *Admit browser live edits in the Worker
  through the committed model* (#1382) lowers the sentinel to `None` in the `ramp_samples` field
  of *Carry an optional per-edit ramp length on live session edits* (#1394 D1) and a finite word
  to `Some`, and the classifier resolves it with `LiveRamps::resolve` (#1394 D5).

## Hazards

- **SDK evals and host-web tests** that send an edit without `smoothingSamples` and compare it
  with a rebuilt or authored session will see a ramp. Where the claim is the switch, give
  `smoothingSamples: 0`. Otherwise compare after the ramp.
- **Moved digests.** Each moved digest is listed with the reason "absent ramp now uses the session
  default (decision 15 D15-1)" and re-pinned on its own, never in bulk.

## Objective gates

1. **Resolution** (`hosts/host-web/src/tests.rs`, new). For each of the 12 ramped kinds, a record
   with the sentinel is admitted with the D3-mapped length at 48 kHz: 960 for fader, trim, send gain
   and VCA offset, 480 for mute, solo, send mute, `follows_mute` and VCA mute, 960 for polarity
   (twice the mute length; root, 2026-10-05, from #1055), and 960 for the pan rows. Because the
   defaults give polarity and fader the same 960, a session with `mute_ms` 5 and `fader_ms` 15 also
   resolves polarity to 480 and fader to 720. A record with 37 keeps 37. A session with all keys at
   0 resolves to 0. On a strip whose session pan says `smoothing_samples: 96`, a pan and a matrix
   sentinel resolve to 96; a send matrix sentinel still resolves to 960. A sentinel on input filters
   or bypass is `MALFORMED`.
2. **SDK** (`sdk/test/live-controls-evals.mjs`, new).
   - Every ramped builder method writes `0xFFFFFFFF` at offset 16 when no length is given, and the
     given value otherwise.
   - `smoothingSamples: 4294967295` throws.
   - `builder-evals.mjs`: `controlSmoothing` round-trips into the model and through `toJson()`
     to the canonical bytes the Rust writer gives (`explicit`), an omitted one writes
     `{ "kind": "default" }`, and 1000.5 throws. A strict request with
     `controlSmoothing` builds the same session; one with an unknown key inside it is refused at
     `$.controlSmoothing`.
3. **Parity** (`hosts/host-web/tests/control_smoothing_parity.rs`, new).
   - One session with two tracks, at each launch rate: 44.1, 48, 88.2 and 96 kHz. One track's pan
     has `smoothing_samples: 0`, the other's a non-zero value.
   - A scripted sequence with no per-edit length: a fader move, a mute, an unmute and a pan move
     at fixed blocks.
   - The browser (native host-web, edits as commands) and the C ABI (`capi`, edits as
     transactions) render bit-identical output on every block, during the ramps and after them.
4. **Allocation.** Admission with resolution allocates nothing on the render thread (the existing
   host-web allocation gate passes).
5. **Commands:**
   - `cargo test --locked -p host-web --features test-support`, `cargo test --locked -p capi`
   - the workspace debug leg (`test-debug-a` in `.github/workflows/qualification.yml`)
   - `mkdir -p target/ci/qualification-artifacts target/ci/qualification-named-twin && bash scripts/build-web-audioworklet.sh --named-twin target/ci/qualification-named-twin target/ci/qualification-artifacts`
   - `bash scripts/check-web-audioworklet.sh --without-metadata-regeneration target/ci/qualification-artifacts target/ci/qualification-named-twin/miso-engine-v1-audio-worklet.simd128.named.wasm`
   - `bash scripts/test-web-audioworklet.sh`
   - `bash scripts/check-sdk-generated.sh target/ci/qualification-artifacts`,
     `bash scripts/check-sdk-types.sh`,
     `bash scripts/check-sdk-headless.sh target/ci/qualification-artifacts`,
     `bash scripts/sdk-package.sh check target/ci/qualification-artifacts`
   - `bash scripts/check-workspace-policy.sh`, `bash scripts/check-cross-targets.sh`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`

## Test value

- Gate 1 turns red if a kind skips resolution, if the row map is wrong (for example polarity on the
  fader key, which only the non-default session tells apart, or polarity at the plain mute length;
  root, 2026-10-05, from #1055), if a pan record ignores the strip's own window (#1054 D5), or if a
  finite length is overwritten.
- Gate 2 turns red if the SDK still writes 0 for an absent length, or lets the sentinel through as
  a length.
- Gate 3 turns red if the two hosts compute different lengths (rounding, rate or row map) or
  resolve at different points. No test compares the hosts today.

## Dependencies

- *Session `controlSmoothing`: configurable ramp lengths for live mute, fader and pan changes* (#1054)

*Refuse automation on effect parameters that are not block-rate* (#1335) edits
`sdk/src/core/session.ts` too. Land after it, or rebase.

## Amendment 1 (root, 2026-10-12)

Attempt 1 failed on one MAJOR (verdict `1364-attempt1.md`, M1): a literal `LaneEdit` with no
`smoothingSamples` still encoded 0, a step, through `encodeLaneEdits` (`writer.ts:160`, used by
`LiveControlWriter` and the headless engine) and through `browserCommand`
(`browser/live-controls.ts:29`). Both files were outside the authorized paths. Decision 15 D15-1
("SDK defaults stop being 0") covers them. Ruling:

- **SDK.** The two files join the authorized paths, and a D3 bullet (above) makes the absent default
  depend on the kind through one shared helper. `LaneEdit.smoothingSamples` is documented. One eval
  each for the writer path and the browser path, and each goes red when the absent value reverts to 0.
- **The cached `LiveRamps` in host-web (`SessionRamps`, computed once in `compile_ready`).** The
  question was whether a browser host can commit a `control_smoothing` change, or a change to a
  strip's model `smoothing_samples`, without a rebuild, through a path that exists on this branch.
  It cannot, so no refresh code is added. The reason: `compile_ready` has one caller, `boot`
  (`hosts/host-web/src/lib.rs:2274`), and host-web holds the `SessionModel` only inside that call. No
  entry point of the host (`submit_commands`, `submit_prepared_commands`, `submit_source`,
  `seek_source`, the ffi exports) edits a model, because the browser has no session transaction and
  no committed model. Its 15 command kinds write plan lanes only. A different `control_smoothing` or
  strip window needs a new boot, which runs `compile_ready` and computes `SessionRamps` again. The
  first path that commits a model-only edit in the browser is the Worker admission of *Admit browser
  live edits in the Worker through the committed model* (#1382), after the control-plane crate of
  #1309. **#1382 must drop `SessionRamps`** (the sentinel then lowers to `None` and the classifier
  resolves it with `LiveRamps::resolve` over the committed model, #1394 D5) **or refresh it on every
  model-only commit**, with a test that goes red without it. #1365's `control_smoothing` edit
  reaches the browser only through that path.
- Verdict m1 and the NITs n1 and n2 are folded in attempt 2. NIT n5 (`sdk/README.md`) is not an
  authorized path and is a follow-up (Attempt record).

## Attempt record

**Attempt 1** (implementer, branch `codex/d15-stream-e2`, base `5abf225f2`, the end of #1394). Commits in
deliverable order: D1/D2 with the `LiveRamps` move and the gate 1 tests (`11269ef80`), D3/D4 with the SDK
evals and the shipped declaration (`d0439731d`), the parity test with its dev-dependencies
(`751523524`), then the realtime-policy allowlist row and this record.

**Where the spec and the tree differed (recorded, not improvised around).**

1. *`LiveRamps` was not reachable from host-web.* It lived in `host-core`'s `live_delta` module, which is
   behind the `control-provider` feature (that feature pulls `protocol` and the classifier into the
   crate). Enabling it for host-web would link both into the browser module. `LiveRamps` and
   `LiveRampRow` moved unchanged to a new ungated `crates/host-core/src/live_ramps.rs` and are
   re-exported at the same names; `live_delta.rs` imports them. No logic moved or changed.
2. *The shipped declaration has two copies.* `scripts/check-sdk-generated.sh` requires
   `sdk/src/browser/shipped-host.d.ts` to equal `hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts`
   byte for byte, so the docs-only edit the spec authorizes in the SDK copy is made in the origin too.
3. *Gate 1, input filters.* A sentinel on a bypass record is refused end to end through
   `submit_commands`. A plain `submit_commands` refuses an input-filter record as `UNSUPPORTED_KIND`
   before it reads the word (only a prepared submission reaches the shape check), so the test checks the
   kind at the decode level: the sentinel survives `with_session_ramp` for input filters, bypass, effect
   parameter and both observation kinds, and `into_input_filter_edit` refuses it as `MALFORMED` while the
   same record with 0 is well formed.
4. *The `follows_mute` row* is a record the follow pass builds from the strip mute record that moved the
   send, not a wire kind. It has its own test (`a_followed_mute_send_ramps_over_the_resolved_mute_length`):
   it takes the resolved mute length, 240 at `mute_ms` 5.
5. *Gate 4.* The existing host-web allocation gates pass. They do not send a sentinel record, so a new test
   (`resolving_a_sentinel_allocates_nothing`, `ffi::live_response_ffi_tests::measured`) admits one record of
   each of the twelve kinds and asserts 0 allocations and 0 frees.
6. *The per-strip pan windows are charged.* `SessionRamps::strip_pan` (4 bytes per strip) is retained
   bridge state, so `compile_ready` adds it to the bridge metadata and retained rows with the VCA state.
   The two exact-sum tests that predict those rows (`decoded_command_resource_is_exact_for_live_control_modes_without_effects_or_meters`,
   `effect_control_browser_table_and_payload_reach_exact_budget_gate`) gained a `ramp_table_bytes` term. The
   browser expected-resources budgets and `expected.json` are unchanged (every row within its ceiling).
7. *Parity scope.* The browser keeps the pan window the session prepared; it has no committed model to
   change it (#1382 moves admission to the committed model). The C ABI keeps the committed model's window.
   The test's pan edit therefore carries the window the track was prepared with. A C ABI pan edit that
   changes `smoothing_samples` has no browser counterpart until #1382.
8. *Effect parameter and observation records* carry the word unchanged, as before: the parameter path never
   reads it and the observation path reads it as a window in blocks. The SDK never writes the sentinel there.

**Files outside the authorized list (compiler- or gate-forced).** `crates/host-core/src/{live_ramps.rs,
live_delta.rs,lib.rs}` (item 1); `hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts` (item 2);
`tools/realtime-policy/src/policy.rs` and `docs/REALTIME_DEPENDENCY_POLICY.md` (the parity test calls the exported
C entries, so it is an `unsafe` file; one `UNSAFE_ALLOWLIST` entry and its "Unsafe-code ownership" entry); `Cargo.lock` and `hosts/host-web/Cargo.toml` (`capi` as the spec says, and
`protocol` for the test's command frames, both dev-dependencies only).

**What changed.** `SESSION_DEFAULT_RAMP` (`u32::MAX`) and `SessionRamps` (`LiveRamps::for_session` plus one
pan window per strip, built once in `compile_ready`); `CommandRecord::with_session_ramp` runs right after
decode, by kind, for the twelve ramped kinds, before any shape check; a pan or matrix record takes its
strip's window, else `pan_ms`; a send matrix takes `pan_ms`. SDK: `SESSION_DEFAULT_RAMP`, `smoothing()`
returns it for an absent length and throws on `4294967295`; `ControlSmoothingOptions` /
`SessionOptions.controlSmoothing` emit `{ kind: "explicit", mute_ms, fader_ms, pan_ms }`, checked as finite
`f32` in `[0, 1000]`; the strict authoring request takes a root `controlSmoothing`. Three existing evals
that compared a literal 0 for an absent length now say `smoothingSamples: 0` (the send edit, the VCA edit
and the left-lane VCA mute); the absent case has its own eval.

**Digests that moved: none.** `audit capi` `pcm_digest a9738fa800afcab3`, 0 allocations, 0 syscalls;
`check-browser-expected-resources.py --artifacts` green with no edit (32 red mutations); no render digest,
fixture or byte pin moved, and none was re-pinned.

**Test value and mutation runs** (defect applied by a text edit, test run, file restored; "R" red).

| Test | Plausible defect (mutation) | Result |
|---|---|---|
| `an_absent_ramp_resolves_to_the_sessions_row_and_a_given_one_is_kept` (gate 1) | trim arm removed; polarity on the fader key; polarity at the plain mute length; solo on the fader key; send gain on the mute key; VCA mute on the fader key; a finite word replaced; the session table ignored | R each (8 runs) |
| `a_followed_mute_send_ramps_over_the_resolved_mute_length` | the follow record built with a step; the session table ignored | R, R |
| `a_pan_or_matrix_without_a_length_takes_its_strips_window` | strip windows ignored; window read from strip 0; a send matrix reading a strip window; a window-less strip falling back to the fader length; a finite word replaced | R each |
| `the_kinds_without_a_ramp_keep_the_sentinel_and_refuse_it` | every kind resolved (`_ => Mute`) | R |
| `resolving_a_sentinel_allocates_nothing` | one boxed value built per record in `with_session_ramp` | R |
| `control_smoothing_parity.rs` (gate 3) | strip windows ignored; a window-less strip at `pan_ms + 1`; the window taken even when 0; fader on the mute key; mute on the fader key; sentinel never resolved; the table rounded at 48 kHz at every rate | R each (7 runs) |
| SDK `every ramped builder method writes the session-default marker...` | `smoothing()` defaults to 0; the marker constant off by one | R, R |
| SDK `the kinds that carry no ramp still write 0` | `trackEdit` defaults to the marker | R |
| SDK `the marker is not a length` | the marker accepted as an explicit length | R |
| SDK `a fader move without a length ramps over the session's fader length...` (through the shipped module) | `smoothing()` defaults to 0 | R |
| builder-evals `explicit controlSmoothing round-trips...` / `omitted ... default` / `refused at its path` | the option dropped; an unset option written as `explicit`; upper bound 1001; lower bound dropped; a missing key tolerated | R each |
| builder-evals strict request | the request key dropped; two lengths swapped | R, R |

Which defect each new test catches that no existing test does: gate 1, a kind that skips resolution, a row
on the wrong key (polarity on the fader key is told apart only by the non-default table) or a pan that
ignores its strip's window (no test sent a sentinel before); the follow test, a send follow record that does
not take the resolved mute length; the strip-window test, the #1054 D5 precedence on the browser; the
allocation test, a resolution that allocates per record; gate 3, two hosts that disagree on a length, a rate
or a window (no test compares them); the SDK evals, an absent length still written as a step, and a marker
that passes as a length. Not mutated: the engine side of the SDK's engine-level eval (the sentinel left
unresolved in the shipped module is covered by gate 1 and the parity test at the Rust level, and a module
rebuild per mutation was not repeated).

**Gates run (final tree).** `cargo test --locked -p host-web --features test-support` and `-p capi` inside
the `test-debug-a` leg (`--no-fail-fast`, the CI feature list: 1574 passed, 0 failed) and its doctest leg
(24 passed); `scripts/build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh
--without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts`,
`test-web-audioworklet.sh` (private `TMPDIR`, nothing left behind); `check-sdk-generated.sh`,
`check-sdk-deletions.py`, `check-sdk-types.sh` (after an offline `npm ci`, `node_modules` deleted again),
`check-sdk-headless.sh` (371 evals), `sdk-package.sh check`; `check-workspace-policy.sh`,
`check-realtime-policy.sh` and `test-realtime-policy.sh`, `check-host-core-policy.sh`,
`check-session-policy.sh`, `check-test-support-ci.py`, `check-ci-path-routing.py`, `check-cross-targets.sh`
(PASS), `check-capi-abi.sh`, `audit capi`; `cargo clippy --locked --workspace --all-targets --all-features
-- -D warnings`; `cargo fmt --all -- --check`. `cargo test -p host-web` without `test-support` does not
compile its existing unit tests (they use the feature's counters); that predates this slice. Not run: the
Chromium legs and anything AArch64-only.

**Open items.** `sdk/README.md` still shows only `smoothingSamples: 64` in its live-controls example and
says nothing about the absent-length default or `controlSmoothing`; it is outside this slice's paths. #1382
(Worker admission) must keep the browser's strip windows in step with the committed model, which item 7
describes. #1393 reads `LiveRamps::mute_samples` from `SessionRamps::live`.

**Attempt 2** (implementer, branch `codex/d15-stream-e2`, base `b16fb4d14`, after attempt 1's verdict M1 and
Amendment 1). The attempt-1 row for `a_followed_mute_send_ramps_over_the_resolved_mute_length` is superseded:
that test is deleted (below).

**What changed.**

- *M1, the SDK.* One rule in `sdk/src/core/writer.ts`: `SESSION_DEFAULT_RAMP` moved there (it was in
  `core/live-controls.ts`; `index.ts` re-exports both modules, so the public name is unchanged),
  `checkedRampLength` (an explicit length is checked, `u32::MAX` and non-`u32` throw `MisoUsageError`, an absent
  one stays absent), and `laneEditSmoothingWord(edit)` (a ramped kind resolves an absent length to
  `SESSION_DEFAULT_RAMP`; bypass, input filters, effect parameters and observation write 0 or the given word).
  `encodeLaneEdits` and `browserCommand` call `laneEditSmoothingWord`; `LiveControlWriter.stage` calls it too, so a
  refused length never sits in the pending map and fails every later flush. `LaneEdit.smoothingSamples` is
  documented.
- *Deviation from attempt 1, recorded.* The builders no longer put the marker into the `LaneEdit`: `smoothing()`
  returns the checked length or `undefined`, and `trackEdit` no longer defaults to 0. A `LaneEdit` that carried
  `0xFFFFFFFF` could not be told from a hand-written one with an explicit `u32::MAX`, which Amendment 1 says must
  throw on every path. So the absent length becomes a word in one place only, the encoder. The bytes on the wire
  are unchanged. `LaneEdit.smoothingSamples` is typed `number | undefined` (`exactOptionalPropertyTypes`).
  Explicit `u32::MAX` still throws on the ramped kinds only; on the kinds with no session ramp the word is carried
  as given, as before (the engine refuses a non-zero bypass or input-filter word as `MALFORMED`).
- *B, the cached `LiveRamps`.* No code. Host-web cannot commit a model change without a rebuild on this branch:
  `compile_ready` has one caller, `boot`, and no entry point edits a model. Recorded in Amendment 1 of this spec and
  of #1365; #1382 must drop or refresh `SessionRamps`.
- *m1.* `a_followed_mute_send_ramps_over_the_resolved_mute_length` is deleted. Every mutation of the follow record's
  ramp it caught turns gate 1 (the `solo` and `vcaMute` rows stage the follow records) and five #1224 follow tests
  red too.
- *n1.* `resolving_a_sentinel_allocates_nothing` names its real catch: an allocation on the sentinel path alone.
- *n2.* `live_ramps.rs` no longer links `crate::live_delta`. (`cargo doc -p host-core --no-deps` with
  `-D warnings` still fails on `SessionControlProvider` at `lib.rs:19`, which predates this stream and is not in a
  file this slice owns.)
- No host-web non-test code changed: the shipped module is the attempt-1 module. The artifact was rebuilt anyway to
  run the SDK evals.

**Tests and mutation runs** (defect applied by a text edit, evals run, file restored; "R" red; SDK evals are
`live-controls-evals`, `writer-evals` and `builder-evals`, 91 evals).

| Test | Plausible defect (mutation) | Result |
|---|---|---|
| SDK `a literal LaneEdit without a length encodes the marker through the writer's encoder, by kind` | `encodeLaneEdits` writes `smoothingSamples ?? 0` again | R (with the builder wire eval and the engine-level eval) |
| same | `vcaMute` dropped from the ramped set | R |
| same | `effectBypass` added to the ramped set | R (also in four existing evals) |
| same | the marker accepted as a length | R |
| same | `stage` does not check the length | R (this eval only) |
| SDK `a literal LaneEdit without a length reaches the worklet host with the marker, by kind` | `browserCommand` writes `smoothingSamples ?? 0` again | R (this eval only) |
| same | `vcaMute` dropped; `effectBypass` ramped; the marker accepted | R each |
| existing builder eval (`every ramped builder method ...`), changed | `smoothing()` resolves the word itself instead of leaving it absent | not mutated; the eval now asserts the record leaves the length absent and the encoder writes the marker |

Which defect each new eval catches that no existing test does: the writer eval, a literal ramped `LaneEdit` that
steps through `LiveControlWriter` or the headless engine (no test sent one without a length), a kind put on the
wrong side of the ramped/unramped line, and an unchecked length staged into the writer; the browser eval, a literal
edit that steps on its way to the worklet (the only red mutation that touches no other eval is `browserCommand`
reverted). Rust: the deleted test and the doc fix change no behaviour, so no mutation was applied to them.

**Gates run (final tree).** `cargo fmt --all -- --check`; `cargo clippy --locked --workspace --all-targets
--all-features -- -D warnings`; `test-debug-a` (the CI feature list, `--no-fail-fast`, 0 failed) and its doctest
leg (24 passed); `cargo test --locked -p host-web --features test-support`; `cargo test --locked -p capi`;
`check-workspace-policy.sh`, `check-realtime-policy.sh`, `check-cross-targets.sh` (PASS); the worklet chain
(`build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh --without-metadata-regeneration`,
`check-browser-expected-resources.py --artifacts`, `check-scalar-oracle-absent.py --wasm`,
`test-web-audioworklet.sh` with a private `TMPDIR`, left empty); the SDK job (`check-sdk-generated.sh`,
`check-sdk-deletions.py`, `check-sdk-types.sh` after an offline `npm ci` with `node_modules` deleted again,
`check-sdk-headless.sh`: 373 evals, `sdk-package.sh check`). No digest moved. Not run: the Chromium legs and
anything AArch64-only.

**Open items.** `sdk/README.md` (not an authorized path) still shows only `smoothingSamples: 64` and says nothing
of the absent-length default, `controlSmoothing` or the literal-`LaneEdit` rule; a follow-up edit to that file
fixes it. #1382 must drop or refresh `SessionRamps` (Amendment 1).
