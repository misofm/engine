# Diff a replacement document against the committed model and export replace from the browser engine module

Stream H(c) of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-11).
Code anchors verified on `main` at `6fb211594`.

Cut from *Export transaction apply and anchored seek from the browser engine module* (#1293).

## Product outcome

The browser engine module accepts a whole session document while it plays and applies it as one
transaction: the control half diffs the document against the committed model and runs the normal
transaction apply. The SDK's `replaceSession` (*Apply session transactions from the browser SDK*,
#1296) calls this and nothing else, so there is still one edit path (D15-11).

## Context

- `SessionModel` (`crates/session/src/model.rs:89`) holds the session-level fields (`session_id`,
  `revision`, `sample_rate_hz`, `quantum_frames`, `render_profile`, `output_profile`, `console`)
  and the keyed lists `sources`, `tracks`, `submixes`, `vcas`, `outputs`, `routes`, `automation`.
- `SessionEdit` (`crates/protocol/src/model.rs:185`) has an `Upsert*` and a `Remove*` for each keyed
  kind, `SetConsole`, and a setter for each session-level field. `SessionStore::apply_transaction`
  (`model.rs:906`) resolves every edit in wire order and validates only the final candidate, so a
  transaction may remove a source before it removes the track that references it.
- The classifier compares models (`classify_live_delta`, `crates/host-core/src/live_delta.rs:211`),
  so a whole-entity upsert classifies exactly as the field edits it contains.
- *Replace the running browser session in the Rust host* (#1290) delivers the control half's
  `apply(transaction)` (its D1). Its D8 sketches `apply_document` (parse, diff, apply) and its gate
  9 tests it. **Responsibility split, so nothing is done twice:** this slice owns the diff,
  `apply_document` and the export. #1290 keeps `apply` only; root removes #1290's D8 and gate 9
  in the same checkpoint that files this spec.
- Export staging and the outcome record come from #1293 (`miso_engine_web_v1_edit_ptr`,
  `miso_engine_web_v1_edit_outcome_ptr`). The export-list mirrors are the ones #1293 lists
  (`scripts/check-web-audioworklet.sh:203`, `tools/parameter-metadata/src/abi_layout.rs:135`,
  `scripts/check-abi-layout-v1.py`, `sdk/assets/miso-engine-v1-abi-layout.json`,
  `sdk/src/generated/abi.ts`, `scripts/check-sdk-generated.sh`).

## Decisions frozen for this slice

- **D1. Diff.** `hosts/host-web/src/document_diff.rs`:
  `fn diff(committed: &SessionModel, next: &SessionModel) -> Vec<SessionEdit>` emits, in order:
  1. a setter for each changed session-level field (`session_id`, `sample_rate_hz`,
     `quantum_frames`, `render_profile`, `output_profile`);
  2. a `Remove*` for each keyed entity present only in `committed`, in the order routes,
     automation, VCAs, tracks, submixes, outputs, sources;
  3. `SetConsole` when the console differs;
  4. an `Upsert*` for each keyed entity that is new or differs, in the order sources, outputs,
     submixes, tracks, VCAs, routes, automation.

  It never emits a finer edit. `revision` and `schema_version` are not diffed: the store owns the
  revision, and a document with another schema version is refused at parse.
- **D2. Method.** `AudioWorkletEngineHost::apply_document(&mut self, document: &[u8])` on the control
  half runs the boot's document size and parse budgets, parses into a model, runs D1, and applies
  the edits through #1290's `apply` at the committed revision. An empty diff commits nothing and
  returns the unchanged revision with path `model_only`. A rate or quantum change is refused by
  `apply` as #1290 D1 states.
- **D3. Export.** `miso_engine_web_v1_replace(handle, len) -> u32` runs D2 on the bytes staged
  through `miso_engine_web_v1_edit_ptr` and writes #1293's outcome record and, on refusal, the host
  diagnostic. Like #1293's exports, it runs on the control half only.
- **D4. Mirrors.** Add the export to every list in the context.

## Deliverables

1. D1 in `document_diff.rs`; D2 in `hosts/host-web/src/lib.rs`; D3 in `hosts/host-web/src/ffi.rs`.
2. D4 in the checkers, the generator and the generated SDK files.
3. Native tests in `hosts/host-web/src/tests.rs`.

## Authorized paths

- `hosts/host-web/src/document_diff.rs` (new), `hosts/host-web/src/lib.rs`,
  `hosts/host-web/src/ffi.rs`, `hosts/host-web/src/tests.rs`
- `scripts/check-web-audioworklet.sh` (export list only), `scripts/check-abi-layout-v1.py` and its
  fixture, `tools/parameter-metadata/src/abi_layout.rs`,
  `sdk/assets/miso-engine-v1-abi-layout.json`, `sdk/src/generated/abi.ts`,
  `scripts/check-sdk-generated.sh`

## Non-goals

- No change to `apply` or classification (#1290, #1309). No SDK API (#1296).

## Objective gates

1. **Diff coverage.** For each keyed kind (source, track, submix, output, route, automation, VCA)
   one case adds, one removes and one changes an entity; one case changes the console; one the
   session ID. For each, `apply_document(B)` commits a snapshot equal to B's canonical text, and
   `diff` returns exactly one edit per differing entity.
2. **Dependent removal.** Removing a source and the track that reads it in one document commits.
3. **Same as a transaction.** `replace` with A plus a muted track returns the same outcome record
   (path `rebuild`, revision +1) and renders the same blocks as #1290's gate 1 transaction. A
   document that changes only one fader returns path `live`. An identical document returns
   `model_only`, the unchanged revision, and publishes no candidate.
4. **Refusals.** A malformed document, a document over `MAXIMUM_DOCUMENT_BYTES`, another schema
   version and another sample rate each refuse with their typed result and a diagnostic; the
   committed snapshot and revision are unchanged.
5. **Mirrors.** `python3 -B scripts/check-abi-layout-v1.py` with its self-test and
   `bash scripts/check-sdk-generated.sh <artifacts>` pass, and fail if one list omits the export.
6. Commands:
   - `cargo test --locked -p host-web --features host-web/test-support`
   - `rm -rf target/ci/h1386 && mkdir -p target/ci/h1386/a target/ci/h1386/n && bash scripts/build-web-audioworklet.sh --named-twin target/ci/h1386/n target/ci/h1386/a && bash scripts/check-web-audioworklet.sh target/ci/h1386/a target/ci/h1386/n/miso-engine-v1-audio-worklet.simd128.named.wasm`
   - `bash scripts/check-sdk-generated.sh target/ci/h1386/a`, `bash scripts/test-web-audioworklet.sh`
   - `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`,
     `bash scripts/check-workspace-policy.sh`

## Test value

- Gate 1: a diff that misses an entity kind silently leaves the committed model different from the
  document the app asked for; one that emits unchanged entities fails the exact edit count, which
  keeps the `live` and `model_only` paths reachable through `replace`.
- Gate 2: a diff that orders a source removal so that an intermediate check refuses it turns red.
- Gate 3: a `replace` that reboots or takes a second edit path diverges from the transaction's
  outcome or blocks.
- Gate 4: a refusal that moved the committed model before failing turns red.

## Dependencies

- *Replace the running browser session in the Rust host* (#1290).
- *Export transaction apply and anchored seek from the browser engine module* (#1293).
