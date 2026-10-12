# Prove that every export the worklets call after construction is render-locked

Stream J follow-up of decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-10). Filed
2026-10-09 by root from the #1488 attempt-1 verdict's observation
(`/home/bl/misofm/submix-verdicts/1488-attempt1.md`, "Observations") and the batch-misc2 verdict
(`/home/bl/misofm/submix-verdicts/batch-misc2-batch-verdict.md`).

Root's ruling (2026-10-09), verbatim:

> (5) File a stream J issue so the syntax-tree realtime tool (or a dedicated check) proves every
> export the worklet calls after construction is wrapped in render_locked (red when a wrap is
> removed).

Smallest slice: one shared table of the exports either AudioWorklet calls after construction, one
syntax-tree rule that each listed export's body is a `render_locked` call, and one recording check
in each worklet's hermetic harness that the worklet calls nothing after construction that the table
does not list.

## Problem (verified on `codex/d15-batch-misc2` at `a998adf21`)

- **The rule.** D15-10 and #1488: every export that either AudioWorklet calls on its render thread
  after boot wraps its body in `render_locked` (`hosts/host-web/src/ffi.rs:14-39`; "boot" is the
  engine worklet's whole construction path through `initialize`, up to `miso.ready.v1`). The
  post-construction calls are the policy's set 1, 33 exports
  (`docs/REALTIME_DEPENDENCY_POLICY.md`, "The browser render-locked allocator"); 31 are wrapped,
  and `dispose` and `render_allocation_count` are the named exceptions. The batch verifier derived
  this set by hand, export by export (`batch-misc2-batch-verdict.md`, "Coherence across slices").
- **No gate holds the wrap itself.** The #1488 verdict: "No gate defends the wrap itself. Gate 2's
  D1-reverted runs are green by design: if a later edit removes a wrap, nothing goes red until an
  allocation is also there, and then the browser count does not see it either." Its gate 2 replaced
  the wrap with a block-local `render_locked` that only calls the body, kept an injected allocation,
  and every run stayed green (`1488-attempt1.md`, "Gates run", gate 2).
  - `tests/render_locked_source.rs` and `tests/render_locked_staging.rs` count allocations inside
    windows. Without the wrap there is no window, so they read zero.
  - The browser qualification's `render-allocations` and `sdk-render-allocations` rows read the
    same count (`hosts/host-web/qualification/run.mjs:266-270`): a lost wrap shows only when real
    code in that export also allocates, and then it reads zero for the same reason.
  - The static call-graph gate (`scripts/check-web-audioworklet-callgraph.py`) runs on three exports
    only: `render` (`--render-thread`), `meter_poll` and `command_submit`
    (`scripts/check-web-audioworklet.sh:486-512`). It checks the direct-call closure for allocator,
    deallocator and drop-glue names, thread-local destructor registration, atomic waits and
    `call_indirect` sites; it does not look for the render-locked flag, and it does not run on the
    other 28 wrapped post-construction exports.
- **No gate holds the set.** Nothing fails when a worklet handler starts calling an export after
  construction that is not wrapped (for example a new port handler that calls a staging
  accessor); the set lives in prose in three places (the `ffi.rs` header, `render_lock.rs`'s
  header and the policy section).

## Decisions

- **D1. One table, both readers.** A data file in `tools/realtime-policy` (for example
  `render-thread-exports.json`) lists every export either worklet calls after construction: its
  name (without the `miso_engine_web_v1_` prefix), the worklet and handler that calls it, and
  whether it is wrapped. Exactly the two named exceptions are unwrapped, each with its reason
  (`dispose`: teardown, it frees the host by design and is also the boot-failure path;
  `render_allocation_count`: the reader of the count). `Policy::workspace()` loads it
  (`include_str!`); the harnesses in D3 read the same file. The table starts as the policy's set 1.
- **D2. The syntax-tree rule.** The realtime-policy tool parses `hosts/host-web/src/ffi.rs` with
  `syn` and, for each wrapped entry, fails unless: the `#[unsafe(no_mangle)]` export exists; its
  body is exactly one expression, a call whose callee resolves to
  `crate::render_lock::render_locked` (imported by `use`, with no item, local function or macro
  named `render_locked` defined in `ffi.rs`), whose single argument is a closure that holds the
  whole body; and no statement precedes or follows that call. Each failure names the export and the rule. A table entry that names no
  export fails too.
- **D3. The table is the worklets' real calls.** `testProcessor` in
  `scripts/test-web-audioworklet.mjs` (which runs the real engine worklet against fake exports
  through a stubbed `WebAssembly.Instance`) wraps the fake exports in a recording proxy once the
  processor has posted `miso.ready.v1`, records each `miso_engine_web_v1_*` call across its cases,
  and fails if a recorded name is not in the table (engine worklet entries). The PCM-feed worklet's
  harness in `sdk/test/browser-pcm-evals.mjs` does the same for the feed worklet. A recorded call
  before `miso.ready.v1` is construction and is not checked.
- **D2 Amendment 1 (root, 2026-10-12; attempt-1 F1).** Root's ruling, verbatim. These are direct
  written forms, so they are closed, not listed as limits.

  > Under hosts/host-web/src, the tool refuses:
  > (a) a #[cfg] or #[cfg_attr] attribute on the render_locked import, on each export listed in
  > render-thread-exports.json, on the wrap call inside it, and on `mod ffi;` and `mod render_lock;`
  > in lib.rs;
  > (b) any #[export_name] (unsafe or not) whose value starts with `miso_engine_web_v1_`; every
  > exported name comes from a listed fn's own identifier;
  > (c) a #[path] attribute on `mod ffi;` or `mod render_lock;`;
  > (d) a glob `use` that can bring a `render_locked` name into the ffi module (a `use ...::*` in
  > ffi.rs or render_lock.rs).
  > A cfg-split twin and a glob pass-through bypass are refused by these rules. An export or
  > attribute that a macro expansion assembles from parts is #1438 limit 4.

  Root's ruling on (d)'s scope (2026-10-12), verbatim:

  > (d) refuses a glob `use` (including `pub use` and a glob nested in a grouped tree) that is an
  > item of ffi.rs's or render_lock.rs's top level, or of any module that holds a listed export or
  > the render_locked import; globs in other child modules (the five #[cfg(test)] `use super::*;`
  > lines) are not refused, because they cannot change how a listed export or the import resolves.

  Root added: if the tool does not already require each listed export to be a top-level item of
  ffi.rs, the "any module that holds a listed export" clause covers a moved export. The tool
  requires it (a listed export that is not a top-level `#[unsafe(no_mangle)]` function of ffi.rs is
  `ExportMissing`), so a moved export is `ExportMissing`, and (d) reads the two files' top levels.
  The measurement and the ruling: on the real tree the five `use super::*;` lines are
  `ffi.rs:4853`, `ffi.rs:4883`, `ffi.rs:5351`, `ffi.rs:6443` and `render_lock.rs:141`, all in child
  `#[cfg(test)]` modules; the tool has 0 refusals under (a) to (d) on the real tree.

  *Placement.* The rules live in `tools/realtime-policy/src/web_host.rs` (the module-level forms
  and `#[export_name]`) and in `render_wrap.rs` (the forms on the import, the exports and the wrap
  call). Cases are in `tools/realtime-policy/tests/policy/web_host.rs`, one per rule, each built
  from a verifier reproducer (`/home/bl/misofm/submix-verdicts/evidence/1496-attempt1/`):
  the cfg-split twin with `export_name` (rules (a) and (b)); the `#[cfg(any())]` import with a glob
  pass-through (rules (a) and (d)); a `#[path]` on `mod render_lock;` (rule (c)); a glob `use`
  (rule (d)); and one accepted case, a `use super::*;` inside a child `mod` that holds no listed
  export. Each case has a one-sentence test value and a mutation in the Attempt 2 record.

- **D4. No product change.** No export, worklet or header changes. If D2 finds an export the table
  lists unwrapped, the slice stops and reports it to root.

## Authorized paths

- `tools/realtime-policy` (D1's table and loader, D2's rule, its unit cases and its CLI output)
- `scripts/test-web-audioworklet.mjs` (stream H's; `testProcessor`'s recording proxy and its final
  check only, by named exception)
- `sdk/test/browser-pcm-evals.mjs` (stream H's; the feed worklet harness's recording proxy and its
  check only, by named exception)
- `tools/realtime-policy/Cargo.toml` and the one `Cargo.lock` line it adds: the dependency
  `serde_json = "=1.0.151"` (the version `crates/session`, `tools/session-validator` and
  `hosts/host-web` pin), which D1's JSON table needs; ratified by root as a named exception
- `scripts/test-web-audioworklet.mjs`: the module-scope load of the table, as a minimal named
  exception (root, 2026-10-12): the load only, its uses stay in `testProcessor`
- this spec

## Non-goals

- Extending the static call-graph gate to more exports, or proving allocation freedom statically.
- Wrapping, unwrapping or renaming any export; editing either worklet or either header.
- Checking construction-time calls (the 28 unwrapped exports `initialize` calls after the `boot`
  export are boot by definition).
- A test that greps JavaScript or Rust source: the set comes from recorded calls, and the wrap
  comes from the parsed syntax tree.

## Hazards

- `tools/realtime-policy` lands on `main` only with J's batch (#1438-#1446). This slice lands after
  it.
- The harnesses' fake exports are plain objects; the proxy must forward every property read, so a
  case that inspects `exports.memory` keeps working.
- A handler the harness never exercises records nothing. D3 proves "no unlisted call", not "every
  listed export is called"; the table is the reviewed list.

## Objective gates

1. **Green on the real code.** The realtime-policy tool passes on the tree;
   `bash scripts/test-web-audioworklet.sh` and the SDK evals
   (`bash scripts/check-sdk-headless.sh <artifacts>`, which runs `node --test 'test/*-evals.mjs'`,
   `scripts/check-sdk-headless.sh:91`) pass.
2. **Red when a wrap is removed (mutations, recorded; PR evidence).** Each applied alone, then
   reverted:
   - `source_seek`'s body runs without its `render_locked` call: the tool is red and names
     `source_seek`;
   - #1488 verdict's "D1 reverted" shape: `ffi.rs` defines a local
     `fn render_locked<R>(f: impl FnOnce() -> R) -> R { f() }` in place of the import: the tool is
     red for every wrapped entry;
   - a statement placed before the `render_locked` call in `spectrum_select`: the tool is red and
     names `spectrum_select`.
3. **Red when the set grows (mutations, recorded; PR evidence).**
   - The engine worklet's `receiveSpectrum` (or any post-construction handler a `testProcessor`
     case reaches) also calls `miso_engine_web_v1_spectrum_selection_epoch`:
     `testProcessor` is red and names it;
   - `source_seek` removed from the table: `browser-pcm-evals` is red and names it.
4. **Existing gates.** The tool's own tests, `cargo clippy --locked -p realtime-policy --all-targets
   -- -D warnings`, `cargo fmt --all -- --check`, `bash scripts/check-workspace-policy.sh`, and the
   hermetic worklet chain exit 0.

*Test value.* D2's rule is red when a post-construction export loses its `render_locked` wrap or the
wrap is shadowed by a local function, which no existing test catches (the #1488 verdict's
D1-reverted runs are green); D3's checks are red when either worklet starts calling an export after
construction that the table does not list, which no gate checks today.

## Evidence

- Gate 1's outputs; gate 2's three and gate 3's two red/green runs.

## Dependencies

- After (other streams): H #1488 and H #1492 (the wraps the table lists).
- After (same stream): J #1446 (the realtime-policy tool on `main`, the awk gate gone).

## Standing rules for the implementer

- Work only from this body. Read the cited lines, the #1488 verdict and the batch verdict first.
- A test that greps source or prose is refused.
- Attempt budget: two attempts, one adversarial verdict each.
- Size: half a day.

## Attempt 1 record

Base `58cdcaf5f` (`codex/d15-stream-j3`, J batch 3). Everything below was measured on this branch.

**Re-measured, differing from the Problem.**
- Set 1 of the policy section (`docs/REALTIME_DEPENDENCY_POLICY.md`, "The browser render-locked
  allocator") is 33 exports, 31 wrapped; each is defined once in `hosts/host-web/src/ffi.rs` as a
  top-level `#[unsafe(no_mangle)]` function and the rule passes on all 31 with no ffi change (D4 holds).
  The engine worklet calls `buffer_ptr`, `prepared_companion_*`, `observation_id_*` and the other
  construction-path exports only from `initialize`'s path (`bindLiveControls`, `stageSpectrumRequest`),
  so they are outside the table, as the ffi header says. (Corrected in attempt 2, F2: both
  `spectrum_target_id_ptr` and `spectrum_target_id_capacity` are also called after construction, by
  `receiveSpectrum`, and are in the table.)
- `scripts/check-realtime-policy.sh` no longer exists (#1446); the tool runs as
  `cargo run --locked --release -q -p realtime-policy`.
- Gate 1 "the tool passes on the tree" does not hold on this branch for a reason that is not this
  slice: the tool reports exactly one finding, `crates/effect-contract/src/live.rs:366`
  (`.map(Consumer::try_pop)`), before and after this change (checked with the change stashed). Its fix
  belongs to #1345 D8 (#1426 names it: "The `live.rs:366` rewrite ... belongs to #1345"). This slice adds
  no finding on the real tree.

**What changed**
- D1: `tools/realtime-policy/render-thread-exports.json`, 33 entries: name without the
  `miso_engine_web_v1_` prefix, `callers` (worklet `engine` or `feed` and the class method that makes
  the call), `wrapped`, and a `reason` on exactly `dispose` and `render_allocation_count`. `src/table.rs`
  parses it strictly (unknown key, wrong type, duplicate name, empty callers, unknown worklet, wrapped
  with a reason, unwrapped without one are errors); `Policy::workspace()` loads it with `include_str!`.
  `Policy` gains `render_thread_exports` and `unwrapped_exports` (the two named exceptions, a constant of
  `policy.rs`; a table entry that is unwrapped and not named there is a finding). `serde_json = "=1.0.151"`
  is a new dependency of the tool (the version `session-validator` and `host-web` already pin);
  `Cargo.lock` gains one line (`realtime-policy` depends on `serde_json`).
- D2: `src/render_wrap.rs` parses `hosts/host-web/src/ffi.rs` (always parsed, like the capi ffi) and, per
  table entry, requires one top-level `#[unsafe(no_mangle)]` function of that name; for a wrapped entry its
  body is exactly one statement, a call of one closure that takes no parameter, whose callee is
  `render_locked` or the full path `crate::render_lock::render_locked`. A bare callee needs the file to
  import `crate::render_lock::render_locked` under its own name (top level, grouped or not) and to
  introduce no other `render_locked` anywhere (fn, const, static, struct, enum, union, type, trait, mod,
  `macro_rules!`, foreign fn, any pattern binding including parameters, any other `use`). New classes:
  `MissingWebFfi`, `RenderLockedNotImported`, `RenderLockedShadowed`, `ExportMissing`, `ExportDefinedTwice`,
  `UnwrappedNotPermitted`, `RenderLockedBody`, `RenderLockedUnresolved`. `Finding` gains an `export` field
  that the line prints as `: export <name>`, so each failure names the export and the rule.
- D3, `scripts/test-web-audioworklet.mjs` (`testProcessor` only): the stubbed `WebAssembly.Instance` hands
  the worklet a proxy over the fake exports. A processor counts as constructed once its port has posted
  `miso.ready.v1`; from then the proxy records each `miso_engine_web_v1_*` read and call. Installing it in
  the Instance stub (not per case) covers every processor of the suite, `testQualificationBoot`'s six
  included. An unlisted export that the fakes do not define gets a stub that returns 0, so the final check
  names it instead of a bare `TypeError` stopping the run. The final check requires that `render`,
  `command_submit`, `source_submit` and `meter_poll` were seen (the recording is not vacuous) and that
  every recorded name is a table entry with an `engine` caller. Recorded on the real worklet:
  `command_submit dispose eq_target_config_copy meter_lease meter_poll render source_seek source_submit`.
- D3, `sdk/test/browser-pcm-evals.mjs`: `recordFeedCalls` wraps the `exports` of both engine fakes the feed
  worklet drives (the actual-Wasm one and `runPrelude`'s); the last test requires `source_submit` and
  `source_seek` to have been seen and every recorded name to be a table entry with a `feed` caller.
- Tests: `tests/policy/render_wrap.rs` (10 cases, corrected from "11" in attempt 2, on a base fixture whose `ffi.rs` now holds a small
  exports file and a small case table), `src/table.rs`'s two unit tests; `walk.rs`'s root-is-a-file case
  gains `MissingWebFfi` (the file is gone with its root).

**Gates**
- Gate 1: `cargo test --locked -p realtime-policy` 196 integration + 2 unit pass; tool on the tree: the one
  known finding above and nothing else. `bash scripts/test-web-audioworklet.sh` exit 0 and its TMPDIR left
  empty; `bash scripts/check-sdk-headless.sh target/ci/qualification-artifacts` 363 tests, 0 fail.
- Gate 4: `cargo clippy --locked -p realtime-policy --all-targets -- -D warnings`, `cargo fmt --all --
  --check`, `bash scripts/check-workspace-policy.sh` (ok), `python3 scripts/check-ci-path-routing.py`
  (passed; no CI file changed), `scripts/build-web-audioworklet.sh --named-twin ...` (module
  `9784d3509dba...`), `scripts/check-web-audioworklet.sh --without-metadata-regeneration`,
  `check-browser-expected-resources.py --artifacts` all exit 0.

**Mutations (each applied alone, then reverted; the revert is green)**
- Gate 2, on the real `ffi.rs`, tool output after the known finding: (1) `source_seek` body run without
  `render_locked` -> red, `export miso_engine_web_v1_source_seek: hosts/host-web/src/ffi.rs:3961`;
  (2) the import replaced by a local `fn render_locked<R>(f: impl FnOnce() -> R) -> R { f() }` -> red:
  `RenderLockedNotImported`, `RenderLockedShadowed` (`ffi.rs:44`) and 31 `RenderLockedUnresolved`, one per
  wrapped export (34 lines in all with the known finding); (3) `let _before = handle;` before the call in
  `spectrum_select` -> red, names `miso_engine_web_v1_spectrum_select:2968`.
- Gate 3: (a) `this.exports.miso_engine_web_v1_spectrum_selection_epoch()` at the top of the engine
  worklet's `receiveSource` (a mutated copy through `MISO_ENGINE_WEB_WORKLET_TEST_MODULE`) -> `testProcessor`
  red, `actual: [ 'spectrum_selection_epoch' ]`, message lists `miso_engine_web_v1_spectrum_selection_epoch`;
  (b) `source_seek` removed from the table -> `browser-pcm-evals` red, the last test names
  `miso_engine_web_v1_source_seek`; (c) the same call added to the feed worklet's `applySharedSeek` ->
  red, names `miso_engine_web_v1_spectrum_selection_epoch`.
- Harness hooks (not in the spec): ready flag never set in `testProcessor` -> red, "the recording proxy saw
  the engine worklet call render"; `recordFeedCalls` removed from both fakes -> red, "saw the feed worklet
  call source_submit".
- Rule and loader, each against `tests/policy/render_wrap.rs` or `table.rs`'s tests (one red test or more,
  revert green): only the first statement judged (statement-after case red); shadow scan off (3 cases red);
  a bare callee always resolved (4 red); closure parameters allowed (call-shape case red); a missing export
  silent (export case red); every unwrapped entry permitted (exception case red); `#[unsafe(no_mangle)]`
  not required (export case red); `only_keys` accepting any key and an unwrapped entry without a reason
  (loader test red; the first run of this mutation stayed green, because the case it used also removed a
  required key, so three cases that add an extra key were added).

**Test value.** `render_wrap.rs`: a post-construction export that loses its wrap, gains a statement beside
it, or has `render_locked` shadowed by a local function, a macro, a binding or another import turns it red;
the #1488 gate-2 shape was green in every earlier test. The two harness checks: a worklet that starts
calling an export after construction that the table does not list. `table.rs`: a table that loses a wrap
without a word (a misspelled key, an unwrapped entry with no reason).

**Open.** The headers and the policy section still carry the set as prose (`ffi.rs` header, `render_lock.rs`
header, policy "set 1"); D4 forbids a header edit here and the policy file is outside the authorized paths.
Root may point them at the table in a doc pass. The tool run on the whole tree stays red until #1345 D8.

## Attempt 2 record

Base `d8a1f734c` (the spec-only commit that records D2 Amendment 1), `codex/d15-stream-j3`.

**Measured first (Step 2), on the real tree.** Rules (a), (b) and (c) refuse nothing: no `#[cfg]` or
`#[cfg_attr]` on the import (`ffi.rs:43`), on any of the 33 listed exports, on any wrap call or on
`mod ffi;` / `mod render_lock;` (`lib.rs:7443-7444`); no `export_name` under `hosts/host-web/src`; no
`#[path]`. Rule (d) read literally (a `use ...::*` anywhere in the two files) would have refused five
real lines, all `use super::*;` in child `#[cfg(test)]` modules: `ffi.rs:4853`, `4883`, `5351`,
`6443` and `render_lock.rs:141`. I stopped and reported them; root ruled on (d)'s scope (recorded
verbatim in D2 Amendment 1) and the tool reads the two files' top-level items only. Real-tree result
with the finished tool: the release tool prints exactly `crates/effect-contract/src/live.rs:366`
(pre-#1345) and nothing else. The only `cfg` near the exports is on `pub mod native_staging`
(`ffi.rs:4613`), a child module that holds no listed export; no rule touches it.

**What changed**
- `tools/realtime-policy/src/web_host.rs` (new): rule (a) on `lib.rs`'s `mod ffi;` / `mod render_lock;`
  and on the two files' inner attributes; rule (c); rule (d) on the two files' top-level `use` items
  (plain, `pub`, grouped, nested path); rule (b) over every parsed file of `hosts/host-web/src`
  (every attribute, through `unsafe(..)` and `cfg_attr(.., ..)`, any nesting). `lib.rs` now parses
  every file under `hosts/host-web/src`.
- `render_wrap.rs`: rule (a) on the real import's `use` item, on each matching export (also when the
  table's export is defined twice) and on the sole wrap call.
- New classes: `ConditionalWrapPath`, `ExportNameOnWebExport`, `PathOnWebModule`, `GlobInWrapModule`,
  and two fail-closed classes the rules need to mean anything: `WebModuleNotDeclared` (`lib.rs` does not
  declare `mod ffi;` / `mod render_lock;` as out-of-line modules: an inline `mod ffi { .. }` would move
  what ships away from what is read) and `MissingWebSource` (`lib.rs` or `render_lock.rs` missing).
  Two readings that go slightly past root's letter, flagged for root: an inner `#![cfg(..)]` in
  `ffi.rs` / `render_lock.rs` is refused as the same thing as `#[cfg]` on `mod ffi;`, and the two
  classes above. The real tree has none of them.
- A listed export that moves into a child module is `ExportMissing` (the tool already requires a
  top-level item), so root's "any module that holds a listed export" clause needs no code beyond
  the top-level requirement; a case defends it.
- Fixtures: `lib.rs.txt` declares the two modules; `render_lock.rs.txt` is new and holds a
  `#[cfg(test)] mod tests { use super::* }` (the accepted shape). `walk.rs`'s root-is-a-file case gains
  `MissingWebSource`.
- F2 and the NITs: the attempt-1 record sentence corrected (above); "11 cases" corrected to 10;
  `src/lib.rs:2` rewrapped. Integration tests: 196 to 203 (seven new, all in `tests/policy/web_host.rs`).

**Gates.** `cargo test --locked -p realtime-policy`: 203 integration + 2 unit pass. Release tool on the
real tree: only `live.rs:366`. `cargo clippy --locked -p realtime-policy --all-targets -- -D warnings`,
`cargo fmt --all -- --check`, `bash scripts/check-workspace-policy.sh` exit 0. Worklet chain
(`build-web-audioworklet.sh --named-twin`: shipped module `9784d3509dba...`, same as attempt 1 and as
the base, since no browser-compiled code changed; `check-web-audioworklet.sh
--without-metadata-regeneration`; `check-browser-expected-resources.py --artifacts`;
`test-web-audioworklet.sh` under a private TMPDIR, left empty): exit 0. `check-sdk-headless.sh`:
363 pass, 0 fail.

**Cases, test values and mutations** (each mutation applied alone to the finished tool, red, then
reverted, green; the verifier's reproducers are the sources of the first two cases):
- `a_cfg_split_twin_with_an_export_name_is_refused` (rules (a) and (b)). Test value: a `#[cfg]` split
  that keeps the wrapped export for native builds and exports an unwrapped copy to wasm through
  `export_name`, which the attempt-1 tool passed while the shipped module exported the unwrapped body.
  Mutations: ignore `cfg` on a listed export (red, also the next test); stop reading `unsafe(..)` inside
  attributes (red).
- `conditional_compilation_on_a_guarded_item_is_refused` (rule (a), eight sites, plus the
  `cfg(any())` import with a glob pass-through, the verifier's second form, plus an accepted `cfg` on an
  unguarded item). Test value: a build that drops the import, an export, the wrap call or a module
  declaration while the file the rule read stays wrapped. Mutations, each red: the import site not
  checked; the export site; the call site; the `mod` declaration site; the inner-attribute site;
  `cfg_attr` not treated as conditional.
- `an_export_name_with_the_table_prefix_is_refused_and_another_is_not` (rule (b), three spellings in
  three places, plus an accepted `export_name = "memory_probe"`). Test value: an exported name that
  does not come from a listed function's identifier, in any attribute spelling and any file of the
  crate. Mutations, each red: `cfg_attr` arguments not read; `unsafe(..)` not read; only `ffi.rs`
  scanned; every `export_name` refused (the accepted case goes red).
- `a_path_on_a_guarded_module_or_an_inline_module_is_refused` (rule (c)). Test value: `#[path]`
  moves `mod render_lock;` (or `ffi`) to a file the rule never read, as #1442 D8 accepts for its own
  modules; an inline `mod ffi {}` does the same. Mutations, each red: `#[path]` not checked;
  an undeclared or inline module not refused.
- `a_top_level_glob_use_is_refused_and_a_child_module_glob_is_not` (rule (d), four spellings in both
  files, plus the accepted child-module globs, `use super::*;` and `pub use super::*;`). Test value: a
  glob that supplies a pass-through `render_locked` to the import's module, and the over-refusal of
  the five real test-module lines it must not make. Mutations, each red: groups not searched; nested
  paths not searched; `render_lock.rs` not searched; "refuse globs in every module" (red in the
  dedicated test and in every case of the suite, because the base fixture carries the accepted shape).
- `an_export_moved_into_a_child_module_is_missing`. Test value: an export moved into a child module,
  where a glob is allowed and the wrap would be judged from the wrong scope. Mutation: collect exports
  from child modules too (red).
- `a_missing_host_module_file_is_a_finding`. Test value: a deleted `render_lock.rs` treated as a pass.
  Mutation: no `MissingWebSource` finding (red, with `walk.rs`'s root-is-a-file case).

**Open.** An export or attribute that a macro expansion assembles from parts is #1438 limit 4. The
prose copies of the set (headers, policy "set 1") remain, as in attempt 1.
