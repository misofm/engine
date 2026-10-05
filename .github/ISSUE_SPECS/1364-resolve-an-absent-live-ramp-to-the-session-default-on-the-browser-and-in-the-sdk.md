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
