FAIL

# #1496 attempt 1 verdict: prove that every export the worklets call after construction is render-locked

Commit `e6dd440e0` (parent `58cdcaf5f`) on `codex/d15-stream-j3`. I exported it to
`/tmp/claude-1002/v1496/tree` and built with `CARGO_TARGET_DIR=/tmp/claude-1002/v1496/target`.
For wasm `cargo check`/`clippy` I used `/tmp/claude-1002/v1496/target-wasm` with
`RUSTFLAGS=-C target-feature=+simd128`. I did not touch the worktree. The reproducers are in
`/home/bl/misofm/submix-verdicts/evidence/1496-attempt1/`.

D1's table, D3's two harness checks, the gates and the record's mutations are correct, and I
reproduced them. The attempt fails on F1. Plain written `#[cfg]` and `#[export_name]` attributes,
with no macro, let the shipped browser module export an unwrapped body while the tool passes.
#1438's threat model, which J's tools work under, refuses such a false pass. The cause is partly in
D2's own wording (SP-1).

## MAJOR

- **F1. D2's rule ignores `#[cfg]`, `#[cfg_attr]` and `#[export_name]`. A cfg-split export
  passes the tool, and the shipped module exports the unwrapped body**
  (`tools/realtime-policy/src/render_wrap.rs:49-56` takes every top-level `#[unsafe(no_mangle)]`
  fn and does not read its other attributes; `:153-161` counts the import whatever its attributes
  are).
  - Reproducer (`evidence/1496-attempt1/cfg_split_pair.ffi.diff`). Keep the real wrapped
    `miso_engine_web_v1_source_seek` and add `#[cfg(not(target_family = "wasm"))]` to it. Add a
    copy whose body has no `render_locked`, `#[cfg(target_family = "wasm")]
    #[unsafe(export_name = "miso_engine_web_v1_source_seek")] pub extern "C" fn seek_wasm(..)`.
    - The tool gives no new finding. Only the known `live.rs:366` is reported.
    - `cargo clippy --locked -p host-web --lib -- -D warnings` exits 0 on both native and
      `wasm32-unknown-unknown`.
    - `scripts/build-web-audioworklet.sh --module-only` builds a module
      (`081df9c0...`, not `9784d350...`) that exports `miso_engine_web_v1_source_seek` from the
      unwrapped body.
    - As the spec's Problem says, no other gate sees a lost wrap (the allocation count reads zero
      with no window).
  - Second form (`cfg_import_plus_glob.*.diff`). Put `#[cfg(any())]` on
    `use crate::render_lock::render_locked;` and add `use crate::fake::*;`, where a module outside
    `ffi.rs` defines a pass-through `render_locked`. The tool still counts the import, sees no
    shadow (it skips globs) and passes all 31 exports. In the build the import is gone and the glob
    supplies the fake. This form compiles. Only the incidental `dead_code` warning on the real
    `render_locked` would fail clippy, and one `#[allow]` or a second caller silences it.
  - Why this is a finding and not a limit: #1438 Hazards, Amendment 2, "Over that source it fails
    closed ... a false pass is not [acceptable]". Neither form uses a macro (limit 4), a registry
    crate (limit 1) or type-level behaviour (limit 2), and no spec stages it (limit 3). The module
    doc (`render_wrap.rs:9-12`) and the record ("each failure names the export and the rule")
    claim resolution that the tool does not do here. The precedent is #1442 attempt 1 F1, graded
    MAJOR for a compiling written-form false pass.
  - Fix scope (fail closed, over-refusal is fine): refuse any `#[cfg]`/`#[cfg_attr]` on the
    `render_locked` import, on a listed export and on the wrap call. Refuse any `export_name`
    attribute in `ffi.rs`, or more simply any `export_name` under `hosts/host-web/src` whose value
    has the table prefix. Add one case per form. With no cfg allowed on the listed export, a second
    definition of the same symbol anywhere in the crate is a duplicate-symbol compile error. I
    checked the other tries, and they are already red or do not compile:
    - `use ...::render_locked as r` + `r(..)`: red, `RenderLockedBody`.
    - `#[export_name]` in place of `no_mangle`: red, `ExportMissing`.
    - a top-level `macro_rules!` that expands to `fn render_locked`: E0255 against the import (and
      macro assembly is limit 4 anyway).
    - an early return or a statement before the call: red, `RenderLockedBody`.
    - an export moved to another file: red, `ExportMissing`. A duplicate is a link error.

## MINOR

- **F2. A false sentence in the attempt record** (`.github/ISSUE_SPECS/1496-...md:156-158`). It
  lists `spectrum_target_id_*` among the exports the engine worklet calls "only from
  `initialize`'s path ... so they are outside the table". `receiveSpectrum` also calls
  `spectrum_target_id_ptr` and `spectrum_target_id_capacity` after construction
  (`hosts/host-web/web/miso-engine-v1-audio-worklet.js:1296-1297`), and the table correctly lists
  both. Remove them from that sentence.

## NIT

- **N1.** The record says `tests/policy/render_wrap.rs` has "11 cases". It has 10 `#[test]`
  functions. The integration count goes from 186 to 196.
- **N2.** `tools/realtime-policy/src/lib.rs:2`, the crate doc line, is 142 characters. Rewrap it.
- **N3.** `scripts/test-web-audioworklet.mjs:21-34` loads the table at module scope, outside
  `testProcessor`. The named exception is "`testProcessor`'s recording proxy and its final check
  only". The load serves only that check, so I count it as in scope. I note it for root's path
  ruling.

## Observations for root (not graded)

- **`serde_json = "=1.0.151"` and the `Cargo.lock` line.** This is necessary given D1's JSON table,
  which both harnesses parse natively. The only way to avoid it is a hand parser or a non-JSON
  format. It is pinned exactly as `crates/session`, `tools/session-validator` and
  `hosts/host-web` pin it (same `=1.0.151`, default features). The lock gains one dependency edge
  under `realtime-policy` and no new package. I see no reason to refuse it. The path ruling is
  root's.
- **D2 stops at the path.** The rule proves that the callee is `crate::render_lock::render_locked`.
  It does not prove that `crate::render_lock` is `hosts/host-web/src/render_lock.rs`. A
  `#[path = "other.rs"] mod render_lock;` in `lib.rs` (a form #1442 D8 accepts), or
  `#[cfg]` on `mod ffi;`, moves what ships away from what the rule read. The F1 fix should also
  refuse attributes on `mod ffi;` and `mod render_lock;` in `lib.rs`, or root should name this as
  a limit.
- The prose copies of the set (the `ffi.rs` header, the `render_lock.rs` header, the policy's
  "set 1") remain, as the record's Open item says.

## What I checked

- **D1.** The table has 33 entries, 31 wrapped. The unwrapped ones are `dispose` and
  `render_allocation_count`, each with a reason. It equals set 1 in
  `docs/REALTIME_DEPENDENCY_POLICY.md`. I mapped every `miso_engine_web_v1_*` site in both worklets
  to its enclosing method.
  - Engine worklet, after construction: `process`, `postMeterFrame`, `receive`, the eleven
    `receive*` handlers and `readSpectrumStreamMetadata`. Together they call exactly the 33. The
    other exports are called only from `initialize`, `bindLiveControls`, `stageSpectrumRequest`,
    `stageSpectrumCollectionRequest` (both called only from `initialize`, `:320-321`) and
    `failInitialization`.
  - Feed: `source_submit` and `source_seek` (`:386`, `:445`).
  - The table's handler names match.
  - `UNWRAPPED_EXPORTS` in `policy.rs` means a table edit alone cannot add an exception.
- **D2 on the real tree.** All 31 wrapped exports pass with no `ffi.rs` change (D4 holds). The
  tool's only finding on the tree is the known `crates/effect-contract/src/live.rs:366`.
- **Gate 2, reproduced on the real `ffi.rs`.** Each mutation went red, and the tree went green
  again after the revert:
  - (1) `source_seek` unwrapped: one new line, `RenderLockedBody`, `export miso_engine_web_v1_source_seek: ...:3961`.
  - (2) a local `fn render_locked` in place of the import: 33 new lines, `NotImported` +
    `Shadowed` (`:44`) + 31 `Unresolved`. That is 34 with the known finding, as recorded.
  - (3) `let _before = handle;` before the call in `spectrum_select`: one new line, naming it at `:2968`.
- **Gate 3 and the harness hooks, reproduced.**
  - Engine worklet copy via `MISO_ENGINE_WEB_WORKLET_TEST_MODULE` with
    `spectrum_selection_epoch()` at the top of `receiveSource`: red, `actual: [ 'spectrum_selection_epoch' ]`.
  - The same call in `initialize` before the ready post: green (construction is not checked).
  - `buffer_ptr` called in `postMeterFrame`: red, names `buffer_ptr`.
  - Ready flag never set, or the proxy not installed: red, "saw the engine worklet call render".
  - SDK: `source_seek` removed from the table: test 19 red, names it.
  - SDK: `spectrum_selection_epoch()` added to the feed's `applySharedSeek`: red, names it.
  - SDK: `recordFeedCalls` removed from both fakes: red, "saw the feed worklet call source_submit".
- **Rule mutations.** Each turns at least one `render_wrap` case red:
  - bare callee always resolved: 4 red;
  - only the first statement judged: 1 red;
  - `PatIdent` shadows not scanned: 1 red;
  - shadowing `use` not scanned: 1 red;
  - any path ending in `render_locked` accepted as full: 1 red.
- **Hazards.** The engine proxy forwards every property (`Reflect.get`). The non-prefixed `memory`
  passes through, and a function cached before ready is recorded when it is called after ready.
  The CI router sends `render-thread-exports.json` and `hosts/host-web/src/ffi.rs` to the full
  route, which runs `audit-native` and the SDK job.

## Test value

- `an_export_whose_body_lost_its_wrap_is_a_finding_naming_it`: an export whose body runs without
  `render_locked` (the #1488 gate-2 shape, green in every earlier test).
- `a_statement_beside_the_wrap_is_a_finding_naming_the_export`: a rule that judges only the first
  statement, so code runs before or after the window unseen.
- `a_local_render_locked_in_place_of_the_import_turns_every_wrapped_export_red`: the #1488
  "D1 reverted" pass-through `render_locked`, which every allocation count misses.
- `any_other_definition_of_the_name_is_a_shadow`: a resolution that ignores a macro, const, mod,
  local or parameter binding, or a foreign `use` of the name.
- `a_renamed_import_is_not_the_import`: a resolution that accepts the real path imported under
  another name while the exports call a bare `render_locked`.
- `the_full_path_is_a_wrap_and_the_import_may_be_a_separate_item`: a rule that refuses the
  full-path callee or a split import, or that lets a full path's resolution excuse a shadowed bare
  callee.
- `the_call_must_be_render_locked_of_one_closure`: a call-shape check that accepts another callee,
  a non-closure, a closure with parameters, extra arguments, `self::`, a parenthesized callee or
  no call.
- `a_table_entry_that_names_no_export_or_two_is_a_finding`: a table entry silently skipped when
  its export is renamed, lacks `#[unsafe(no_mangle)]` or is defined twice.
- `an_unwrapped_entry_outside_the_named_exceptions_is_a_finding`: a table edit alone that adds an
  unwrapped export.
- `a_tree_without_the_host_web_ffi_is_a_finding`: a missing `ffi.rs` treated as a pass.
- `walk.rs` `a_scan_root_that_is_a_file_is_a_finding`, extended: this is the same defence, now
  with the `MissingWebFfi` class.
- `table.rs` `the_good_table_parses_with_prefixed_names` and
  `a_table_that_could_hide_a_missing_wrap_is_refused`: a loader that drops a misspelled key
  (`wraped`), accepts an unwrapped entry with no reason, or accepts duplicates, unknown worklets,
  a wrong prefix or an empty caller list.
- `testProcessor`'s final check: the engine worklet starts to call an export after `miso.ready.v1`
  that the table does not list.
- `browser-pcm-evals` test 19: the same for the PCM-feed worklet.

Each claim is true. I reproduced the harness claims and five rule mutations above. None of these
tests catches F1. That is the gap.

## Gates run

- `cargo test --locked -p realtime-policy`: 196 integration + 2 unit pass.
- `cargo clippy --locked -p realtime-policy --all-targets -- -D warnings`, `cargo fmt --all --
  --check`, `bash scripts/check-workspace-policy.sh` and `python3 -B scripts/check-ci-path-routing.py`:
  all exit 0.
- `bash scripts/build-web-audioworklet.sh --named-twin`: shipped `9784d3509dba...`, named twin
  `75fdb18eac4d...`.
- `bash scripts/check-web-audioworklet.sh --without-metadata-regeneration`: exit 0.
- `python3 -B scripts/check-browser-expected-resources.py --artifacts`: exit 0.
- `bash scripts/test-web-audioworklet.sh` under a private TMPDIR: exit 0, nothing left over.
- `bash scripts/check-sdk-headless.sh <artifacts>`: 363 pass, 0 fail.
- The realtime-policy tool on the tree: only `live.rs:366`.

## Spec problems for root

- **SP-1.** D2 defines resolution as "imported by `use`, with no item, local function or macro
  named `render_locked` defined in `ffi.rs`", and the export as "the `#[unsafe(no_mangle)]` export".
  It does not cover conditional compilation or `export_name`, so a literal implementation passes
  F1. Amend D2 before attempt 2:
  - refuse `#[cfg]`/`#[cfg_attr]` on the import, the listed exports, the wrap call, and `mod ffi;` /
    `mod render_lock;`;
  - refuse `export_name` carrying the prefix under `hosts/host-web/src`.
  Alternatively, name these forms as a limit. #1438's threat model does not allow an implementer
  or a verifier to do that.
