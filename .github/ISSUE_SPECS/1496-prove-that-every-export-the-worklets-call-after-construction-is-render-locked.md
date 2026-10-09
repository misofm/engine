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
- **D4. No product change.** No export, worklet or header changes. If D2 finds an export the table
  lists unwrapped, the slice stops and reports it to root.

## Authorized paths

- `tools/realtime-policy` (D1's table and loader, D2's rule, its unit cases and its CLI output)
- `scripts/test-web-audioworklet.mjs` (stream H's; `testProcessor`'s recording proxy and its final
  check only, by named exception)
- `sdk/test/browser-pcm-evals.mjs` (stream H's; the feed worklet harness's recording proxy and its
  check only, by named exception)
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
