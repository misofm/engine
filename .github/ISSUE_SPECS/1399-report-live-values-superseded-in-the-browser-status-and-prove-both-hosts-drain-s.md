# Report live_values_superseded in the browser status and prove both hosts drain strip cells alike

Stream B of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-2).
Split from *Hold live values in latest-target cells on both hosts* (#1312), which it follows.
Code anchors verified on `main` at `6fb211594`.

## Product outcome

A browser host reads `live_values_superseded` from its status record, with the same value the C
ABI reports through `COUNTERS_GET`. One script of strip edits renders bit-identically through both
hosts, so the two admissions and drains cannot drift apart.

## Context

- `WebStatus` (`hosts/host-web/src/lib.rs:1276-1302`) is 80 bytes; its last field is
  `reserved: [u64; 4]` at offsets 48, 56, 64, 72, required zero. The status is built at
  `:2186-2197` (`reserved: [0; 4]`). Layout asserts: `hosts/host-web/src/tests.rs:286`
  (`size_of::<WebStatus>() == 80`) and `:413` (`offset_of!(WebStatus, reserved) == 48`).
- Mirrors of the layout:
  - `status_fields` in `tools/parameter-metadata/src/abi_layout.rs:318-343` (`"reserved"`,
    `"u64[4]"` at `:342`), which generates the layout asset
    `sdk/assets/miso-engine-v1-abi-layout.json` (its `structures.status` entry, `:147-161`) and
    from it `sdk/src/generated/abi.ts` (`bash scripts/check-sdk-generated.sh` compares both
    arrows byte for byte);
  - the worklet's status reader, `readStatus()`
    (`hosts/host-web/web/miso-engine-v1-audio-worklet.js:944-960`), which throws unless offsets
    48, 56, 64 and 72 are zero (`:946-949`); its result is spread into the `miso.status.v1` reply
    (`:1070`);
  - the main-realm host's status reply check, an exact field list and value validation
    (`hosts/host-web/web/miso-engine-v1-audio-worklet-host.js:1013-1017`, `:1121-1128`);
  - the status type `MisoStatus`, `hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts:846-857`,
    and its shipped copy `sdk/src/browser/shipped-host.d.ts:846-857`;
  - the hermetic harness's fake status reply (`scripts/test-web-audioworklet.mjs:159-164`).
- #1312 adds the cells, the session counter (one `Arc<AtomicU64>`, D2's per-cell unit: a `Both`
  record counts in each channel cell, so 40 `Both` edits before one block add 78), the C ABI's
  `CounterId::LiveValuesSuperseded` (D8) and the browser admission on cells (D7).
- `hosts/host-web/tests/boot_transient_budget.rs` is the model for a native integration test of
  host-web.

## Decisions frozen for this slice

- **D1. Status layout.** `WebStatus` gains `live_values_superseded: u64` at offset 48, and
  `reserved` becomes `[u64; 3]` at offsets 56, 64, 72. The size stays 80. Every mirror in Context
  follows: `status_fields` lists the new field and `u64[3]`; the worklet reader checks only 56, 64
  and 72 and returns `liveValuesSuperseded` (a `bigint`); the host's status field list gains it
  and validates it as a `u64`; both `.d.ts` copies gain `readonly liveValuesSuperseded: bigint`;
  and the harness's fake status reply carries it. *Publish the applied-revision watermark in the browser
  status* (#1349) then takes the three remaining words.
- **D2. Value.** Written at each status refresh from the session counter (a load), so it is never
  behind the last rendered block. No feature bit: an in-place V1 status field (decision 15,
  D15-3).

## Deliverables

1. D1-D2 in `hosts/host-web/src/lib.rs` and the mirrors; the regenerated layout asset and
   `abi.ts`.
2. Gate 1 in `hosts/host-web/src/tests.rs`; gate 3 in `scripts/test-web-audioworklet.mjs`; gate 2
   in a new `hosts/host-web/tests/strip_cells_cross_host.rs`.

## Authorized paths

- `hosts/host-web/src/lib.rs` (`WebStatus` and its refresh only), `hosts/host-web/src/tests.rs`,
  `hosts/host-web/tests/strip_cells_cross_host.rs` (new), `hosts/host-web/Cargo.toml`
  (`control-plane` as a dev-dependency).
- `tools/parameter-metadata/src/abi_layout.rs` (`status_fields` only),
  `sdk/assets/miso-engine-v1-abi-layout.json` and `sdk/src/generated/abi.ts` (both regenerated
  only), `hosts/host-web/web/miso-engine-v1-audio-worklet.js`
  (the status reader only), `hosts/host-web/web/miso-engine-v1-audio-worklet-host.js` (the status
  reply's field list and validation only), `hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts`
  and `sdk/src/browser/shipped-host.d.ts` (`MisoStatus` only).
- `scripts/test-web-audioworklet.mjs` (the fake status reply and gate 3's case only).
- Amendment (root, 2026-10-12), exception m1: `sdk/src/core/boundary.ts` (`EngineStatus` and
  `status()`: the headless status carries `liveValuesSuperseded`, so its doc "the whole status
  structure" is true), with one eval in `sdk/test/capability-evals.mjs`.
- Amendment (root, 2026-10-12), exception m2: `hosts/host-web/tests/browser-v1/direct-oracle.mjs`
  (line 56: word 48 is the superseded count and is no longer required zero; words 56, 64 and 72
  stay required zero).

## Non-goals

- The cells, the counter and the admissions (#1312). The watermark words (#1349).
- Effect, input and route cells (#1345, #1346, #1347): they reach the same counter with no change
  here.

## Objective gates

1. **Status (new host-web test).** 40 `Both` fader edits on one track before one render: the
   status reports `live_values_superseded == 78`, equal to the session counter, and offsets 56,
   64 and 72 read zero. The layout asserts move to the new field (`offset_of!(WebStatus,
   live_values_superseded) == 48`, `offset_of!(WebStatus, reserved) == 56`).
2. **Both hosts agree (new integration test).** One session and one script: two fader edits and
   two pan edits on the same track between two blocks, once through the control plane's
   `SESSION_TRANSACTION_APPLY` and once through host-web's command admission, each followed by 8
   blocks. The PCM is bit-identical and the two counters are equal (3: each fader channel cell and
   the matrix cell replaced once).
3. **The status reply carries the word.** In `scripts/test-web-audioworklet.mjs`, a worklet whose
   fake status block holds a nonzero word at 48 answers a status request with that
   `liveValuesSuperseded`, and the host accepts the reply; a reply without the field is refused.
4. Commands:
   - `cargo test --locked -p host-web --features test-support`
   - the SDK job's `bash scripts/check-sdk-generated.sh target/ci/qualification-artifacts` and
     `bash scripts/check-sdk-types.sh`, after building the artifacts as `qualification.yml` does
   - `bash scripts/check-web-audioworklet.sh`, `bash scripts/test-web-audioworklet.sh`, and the
     browser legs of `qualification.yml`'s `browser` job
   - `bash scripts/check-workspace-policy.sh`, `cargo fmt --all -- --check`,
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`

## Test value

- Gate 1: a status word at the wrong offset or never written.
- Gate 2: the two hosts draining differently (order, `Both` folding), which breaks #1054's
  cross-host bit-identity for two edits in one block.
- Gate 3: a worklet reader that still rejects a nonzero word 48 (every status read would throw
  after the first superseded value), or a host check that refuses or drops the new field.

## Dependencies

- *Hold live values in latest-target cells on both hosts* (#1312).
- **Deferred into this issue by #1312** (recorded in #1312's "Batch follow-ups", m3). The browser
  prepares each plan with its own `live_values_superseded` count (one `Arc<AtomicU64>` per plan,
  charged per plan in host-core's graph row, and only when strip controls hold it), where the C ABI
  passes one count per session. This issue must pass one count from `host-web` to every plan the
  session prepares and charge it once, in place of the per-plan charge; the status field and the
  cross-host agreement test are its other deliverables.

## Attempt record

### Attempt 1 (implementer, on `codex/d15-stream-b3`, the commit that carries this record)

**What landed.**

- `WebStatus` gains `live_values_superseded: u64` at offset 48; `reserved` is `[u64; 3]` at 56, 64, 72;
  the size stays 80 (D1). `render_next` loads the session count into the status after every rendered
  block (`ready.live_values_superseded.get()`, a relaxed atomic load, so it is never behind the last
  block, D2). No feature bit. Mirrors: `status_fields` (`u64` and `u64[3]`), the regenerated
  `sdk/assets/miso-engine-v1-abi-layout.json` and `sdk/src/generated/abi.ts`, the worklet reader
  (checks 56, 64, 72; returns `liveValuesSuperseded`), the host's reply field list and `validU64`
  check, both `.d.ts` copies (`MisoStatus.liveValuesSuperseded: bigint`), and the harness's fake
  status reply.
- The Dependencies deferral from #1312 (m3): the browser makes one `LiveValuesSuperseded` per
  session, at the top of `compile_ready`, and passes it to the preparation entry it calls; it lives in
  `ReadyOwnership` so a successor plan the session prepares later is given the same count. host-core
  charges the count only when it made it (`superseded.is_none()`, unchanged); the browser charges
  `LiveValuesSuperseded::retained_bytes()` once, in `ready_metadata` (so in `bridge_metadata_bytes`
  and `bridge_retained_bytes`), in place of the per-plan charge in the graph row. The test seam
  `#[cfg(test)] ReadyOwnership::live_values_superseded` (#1346) is now an ordinary field; the two
  tests that read it (`many_input_edits_...`, `many_send_edits_...`) read the status instead
  (`host.status().live_values_superseded`); the pre-render "nothing was read yet" line keeps the
  counter's own `get()`, which a status read cannot tell from a never-written status.
- host-core: the three preparation entries the browser calls
  (`prepare_host_runtime_with_selected_meters_between_render_calls`,
  `..._with_live_controls_and_spectrum`, `..._and_spectrum_collection`) take
  `superseded: &LiveValuesSuperseded`, the C ABI's entries' shape (`prepare_host_runtime_with_live_lanes`).
  `prepare_host_runtime_with_spectrum_collection` (no live controls, no session) calls the internal
  entry with no count, as before, rather than the public entry that now requires one.

**Files outside the spec's authorized list, each forced.**

1. `crates/host-core/src/prepare.rs`: the entries above. The Dependencies section requires host-web
   to pass the count; no browser entry took one.
2. `crates/host-core/tests/{prepare,spectrum,strip_meters}.rs`: the call sites of those entries pass
   `&host_core::LiveValuesSuperseded::default()`; no assertion moved.
3. `hosts/host-web/Cargo.toml`: `control-plane` (named) and `protocol` as dev-dependencies. Gate 2
   encodes `SESSION_TRANSACTION_APPLY` and `COUNTERS_GET` frames, which are `protocol` types the
   control-plane crate does not re-export. `Cargo.lock` follows (two lines).
4. `hosts/host-web/tests/retained_ceilings.rs`: the test that asserted the browser's graph row
   carried the plan's own count now asserts the row carries the revision gate and not the count
   (`the_graph_row_charges_the_plans_gate_and_not_the_sessions_superseded_count`); the count's
   one charge is asserted by the exact-decomposition tests below.
5. `hosts/host-web/src/tests.rs` beyond gate 1 and the layout asserts: the two exact-decomposition
   tests (`decoded_command_resource_is_exact_for_live_control_modes_without_effects_or_meters`,
   `effect_control_browser_table_and_payload_reach_exact_budget_gate`) add
   `LiveValuesSuperseded::retained_bytes()` to their expected bridge rows (they sum each retained
   thing independently of the report, so they are the test that the count is charged once, in the
   bridge); four call sites pass a count to the changed entry.

No digest or ceiling was re-pinned. `check-browser-expected-resources.py` is green: the graph rows
fell by the count's 24 bytes and the bridge rows rose by it; every row stays inside its ceiling
(`builtinRetainedBytes` 2999 of 3008, unchanged by this slice).

**Not touched, and why.** `hosts/host-web/qualification/suspended-host.mjs` keeps an exact status
field list and a layout digest (`EXPECTED_LAYOUT_SHA256`, `8cd125f0...`); it is run directly, not by
`npm run qualify`, and its layout pin already disagreed with the tree before this slice
(`sha256sum` of the layout at `HEAD`: `5e1cf67d...`), so it is stale independently of #1399. A
later slice that revives it must add the field to both lists. `qualification.js` reads only
`nextAbsoluteSample` and `renderedQuanta` from the status.

**Pre-existing failure, not this slice's.** `scripts/check-sdk-headless.sh` fails one test,
`a paired builtin filter edit stays whole when only one queue slot remains`
(`sdk/test/writer-evals.mjs:374`): 362 pass, 1 fails, with the artifact built from this tree and
also with the artifact built from `HEAD` (the tree stashed, rebuilt, run, restored). It asserts that
the input band has two queue slots, which #1346 removed. It needs a successor edit (the SDK eval
must express the cell semantics: a filter edit is never refused for room).

**Tests (the plausible defect each defends, and its red run).**

| Test | Defect it turns red | Red run |
|---|---|---|
| gate 1, `the_status_reports_the_sessions_superseded_count` (`src/tests.rs`) | a status word never written, written before the block it counts, or counting a `Both` edit once | the load in `render_next` removed: `left: 0, right: 78`; the load moved before `plan.render`: `left: 0, right: 78` |
| `frozen_layouts_and_values_are_exact`, moved asserts | the word at another offset, `reserved` not moved to 56 | covered by the layout assert and `check-abi-layout-v1.py` (not mutated separately) |
| gate 2, `both_hosts_drain_two_fader_and_two_pan_edits_alike` (`tests/strip_cells_cross_host.rs`) | the two hosts draining or counting differently for two edits in one block, with the drifting host's own pins re-pinned to match | see Follow-ups n4: the load removed (`left: 0, right: 3`) turns gate 1 and the two #1346 status tests red as well, so it does not show gate 2's own catch |
| gate 3, `scripts/test-web-audioworklet.mjs` | a reader that rejects word 48; a reader that drops the field; a reader that stops checking word 72; a host check that drops the field from its list or the `u64` check | reader checks 48: `Expected values to be strictly equal` (fake status block with a nonzero word); reader drops the field: `expected: 1108152157446n`; reader skips word 72: `a nonzero reserved word at 72 fails the boot`; host list without the field: exit 1, `miso.error.v1 result 255`; host without `validU64`: `expected rejection` |
| `the_graph_row_charges_the_plans_gate_and_not_the_sessions_superseded_count` | the graph row charging the session's count per plan (the #1312 m3 defect) | host-core charges although a count was passed: `left: 48, right: 24` |
| the two exact-decomposition tests | the session's count dropped from the bridge rows (never charged) | `ready_metadata` without the count: both fail |

Gate 3 reaches the worklet reader through the fake wasm memory's status block (nonzero word 48
read whole, `0x0102_0304_0506` and `u64::MAX`; nonzero 56, 64 or 72 fails the boot with one
dispose) and the main-realm host through the fake node (the reply with the count, without it, with
a number, a negative, a value past `u64`; a `u64::MAX` count accepted).

**Gates run.**

- `cargo fmt --all -- --check`: clean. `cargo clippy --locked --workspace --all-targets
  --all-features -- -D warnings`: clean.
- `cargo test --locked -p host-web --features test-support`: 192 lib + 2 + 1 + 1 + 2 + 1 integration
  tests pass (2 ignored, as before). `cargo test --locked -p host-core -p control-plane -p capi`: all
  pass.
- `scripts/check-workspace-policy.sh`: ok. `scripts/check-realtime-policy.sh`: ok (102 regions in 27
  files, unchanged), `scripts/test-realtime-policy.sh`: ok. `scripts/check-rack-policy.sh`: PASS.
- `scripts/check-abi-layout-v1.py sdk/assets/miso-engine-v1-abi-layout.json`: satisfies; `--self-test`:
  22 mutations caught. `scripts/check-cross-targets.sh`: PASS. `target/release/audit capi`: exit 0,
  allocations 0, syscalls 0, locks 0, render errors 0.
- Artifacts built as `qualification.yml` does (`build-web-audioworklet.sh --named-twin`): module
  `5f04b994...`. `check-web-audioworklet.sh --without-metadata-regeneration`: passed.
  `check-browser-expected-resources.py --artifacts`: passed (32 red mutations). 
  `check-scalar-oracle-absent.py --wasm`: passed. `test-web-audioworklet.sh` (private TMPDIR, nothing
  left behind): passed. `check-sdk-generated.sh`, `check-sdk-deletions.py`, `check-sdk-types.sh`,
  `sdk-package.sh check`: passed (the stray `sdk/dist` removed). `check-sdk-headless.sh`: the one
  pre-existing failure above.
- Not run: the Playwright browser legs of `qualification.yml`'s `browser` job (no browsers on this
  machine), the V8 spill gate (needs Node 22.23.2 exactly), the AArch64 legs, `check-capi-abi.sh`
  (the C ABI did not change).

### Follow-ups (Sonnet 5.5, on `codex/d15-stream-b3`; the attempt 1 verdict, PASS)

**Amendment (root, 2026-10-12): two exceptions to the authorized list.**

- **m1.** `sdk/src/core/boundary.ts`: `EngineStatus` gains `liveValuesSuperseded: bigint` and
  `status()` reads it from the generated layout by name (`status.u64("liveValuesSuperseded")`), so the
  headless SDK reports the whole status structure and the doc is true.
- **m2.** `hosts/host-web/tests/browser-v1/direct-oracle.mjs`: the status reader no longer requires
  word 48 to be zero (it is the superseded count); words 56, 64 and 72 are still required zero. The
  returned status object is unchanged, so the transcript the artifact gate compares keeps its shape.

**m1 test and mutation run.** `sdk/test/capability-evals.mjs`, "status carries the session's count of
superseded live values". Test value: it goes red when the headless status drops, zeroes or misreads
the field, which no existing headless eval reads. It boots a live-controls session, checks the count
is 0, submits two left-channel `faderDb` edits before any render (the count is still 0: render has
not read either), renders two blocks, and expects `liveValuesSuperseded === 1n` (the left cell was
written twice before the block read it) and `renderedQuanta === 2n`, so a status that read the
rendered-quanta word instead would give 2, not 1. Mutations on `sdk/src/core/boundary.ts`, each alone
then reverted, run as `node --test test/capability-evals.mjs` against a module built from this tree
(11 pass, 1 fail each): the field removed from `status()` (red, expected `1n`, actual `undefined`);
the field read from `renderedQuanta` (red); the field fixed at `0n` (red). Reverted: 12 of 12 pass.

**NITs.**

- **n1.** `ReadyOwnership::live_values_superseded`'s doc (`hosts/host-web/src/lib.rs`) now states the
  rule: a successor plan the session prepares must be given this same count. It no longer says a
  successor "is prepared" with it, which the browser does not do today.
- **n2.** `LiveValuesSuperseded::get`'s doc (`crates/engine/src/realtime/latest_cell.rs`) is no
  longer "Control:" only: control reads it for the counters and render reads it after each block for
  the status.
- **n3.** Gate 2's doc comment no longer says its mutations are ones "no per-host test sees". It
  states the test's own catch: a drift in one host's admission or stage composition whose own pins
  are re-pinned to match, which only the cross-host comparison sees. It names the verdict's M5, M6
  and M7 as mutations that also turn the host's own tests red, and drops the matrix-reader
  mutation, which cannot be made on one host (the reader is shared code).
- **n4.** The attempt 1 gate 2 row no longer cites "the load removed" as gate 2's red run: gate 1
  and the two #1346 status tests catch that mutation too. Gate 2's catch is the re-pinned drift of
  n3, judged as a differential by what it reaches.
- **n5.** `scripts/fixtures/abi-layout-v1-self-test.json` is current: its `status` structure has
  `liveValuesSuperseded` at 48 and `reserved` `u64[3]` at 56 (regenerated by #1345,
  `122bdbbc0`); `check-abi-layout-v1.py --self-test` and `cargo test -p parameter-metadata` pass.

**STREAMS hot-file rows (root ratified).** `docs/handoffs/decision-15-2026-10-05/STREAMS.md` names
#1399 in the rows for `crates/host-core/src/prepare.rs` (with its test call sites in
`crates/host-core/tests/{prepare,spectrum,strip_meters}.rs`), `Cargo.toml`/`Cargo.lock` and
`scripts/test-web-audioworklet.mjs`.

**Not changed.** `hosts/host-web/qualification/suspended-host.mjs` (verdict question 5): whoever
revives it must add `liveValuesSuperseded` to its two status field lists; recorded for #1349 or the
revival issue.

**Gates (the head of the follow-up series: #1447, #1346, #1347 and #1399 follow-ups).**
`cargo fmt --all -- --check`; `cargo clippy --locked --workspace --all-targets --all-features -- -D
warnings`; `cargo test --locked -p source` (debug 38 lib tests, release the same, run for #1447),
`-p engine`, `-p host-web --features test-support` (191 lib tests pass, 2 ignored, and the
integration tests), `-p parameter-metadata` (the ABI layout fixture tests: 5 and 5 pass);
`check-abi-layout-v1.py` on the asset and `--self-test` (22 mutations caught);
`check-workspace-policy.sh`, `check-realtime-policy.sh` (106 regions in 27 files) and
`test-realtime-policy.sh`: ok. Worklet chain on this tree: `build-web-audioworklet.sh --named-twin`
(shipped module `d7be511fbfb5...`), `strip-wasm-names.py` check, `check-web-audioworklet.sh
--without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts` (the direct
oracle accepts word 48; 32 red mutations), `check-scalar-oracle-absent.py --wasm` and
`test-web-audioworklet.sh` (private TMPDIR, nothing left): pass. SDK job (source-bundle mode):
`check-sdk-generated.sh`, `check-sdk-deletions.py`, `check-sdk-types.sh`, `check-sdk-headless.sh`
(363 tests, 363 pass: the 362 of the #1347 verdict plus the new m1 eval) and `sdk-package.sh check`
pass; `sdk/dist` removed afterwards. Not run: the Playwright `browser` legs (no browsers on this
box), AArch64.
