# host-web: boot-configured control-frame and document limits

GitHub #2191. The browser host had two compiled byte caps that a growing session reaches: a 16 KiB
control frame (a console bus add fails at 46 tracks or more) and a 1 MiB document (a console
session of about 200 to 256 tracks does not boot). AGENTS.md allows only configured resources to
constrain track counts.

## Decision (root, 2026-10-28)

Option A of the mapping: both limits are boot words with engine defaults, charged to the memory
budget and reported, as the C ABI already lets its host choose them. Not B (a transaction body
beside the frame): that is a protocol change and leaves replies and events at 16 KiB.

- Boot options offset 32 (the retired `reserved1`) is `maximum_control_frame_bytes`; offset 48
  (the retired `reserved3`) is `maximum_document_bytes`. Layout (64 bytes), `ABI_VERSION`
  `0x0001_0000`, the protocol, `CAPABILITIES_GET` and the C ABI do not change.
- Frame: zero selects 128 KiB (`DEFAULT_CONTROL_FRAME_BYTES`, root ruling of attempt 2; it holds
  a bus add over about 385 tracks); 16 KiB to 16 MiB is accepted; anything else is refused
  `web.options.control_frame_bytes`. The replay cache is `max(64 KiB, 2 * frame)`.
- Document: the default limit is `(budget - frame rows) / PARSE_TRANSIENT_MULTIPLIER` (about
  26.8 MB at the 512 MiB default); a nonzero word only tightens it.
- Attempt 2 (verdict on `a1026b706`): the boot projects the control plane's frame rows before it
  allocates, and the controller's and provider's counter tables are capped at the protocol's 256.
- The SDK drops its public `MAXIMUM_DOCUMENT_BYTES` export, with an upgrade note; the next SDK
  release is a minor (0.10.0), not bumped here.

## Contract

1. The resolved frame sizes the page's control staging, the control plane's frame-scaled rows and
   codec, and the replay cache; the memory budget charges all of them. The control plane names
   its frame rows as one pure function of its limits, `control_plane::frame_scaled_retained_bytes`
   (`FrameRows`, which the compile's charge consumes too), and the boot adds them to the parse
   projection and the retained projection, so a frame the budget cannot hold is refused before
   any memory grows: `host.budget.parse_projection` with `projected_bytes`, `budget_bytes`,
   `document_bytes`, `control_frame_bytes` and `control_frame_rows_bytes`. A counter table holds
   at most `protocol::MAXIMUM_COUNTER_ENTRIES` (256) IDs, the codec's own list bound. A frame over the boot's
   frame is refused `RESULT_INVALID_ARGUMENT` with
   `web.control.frame_bytes\t$[frame_bytes=N,limit_bytes=F]`. `CAPABILITIES_GET` reports it.
2. `miso_engine_web_v1_control_frame_bytes(handle)` reports the resolved frame. The control Worker
   and the single-mode worklet put it in their ready messages as `controlFrameBytes`; the page
   host bounds every frame, reply and event by it and exposes it as `host.controlFrameBytes`.
3. The boot, `document_ptr`, `replace_document_ptr` and `SESSION_REPLACE` share one document
   limit. A refusal names the bytes: `web.document.maximum_bytes` (over the word,
   `RESULT_REFUSED_DOCUMENT`) or `host.budget.parse_projection` (over the budget,
   `RESULT_REFUSED_BUDGET`). A refused `document_ptr` writes its diagnostic, read through the new
   export `miso_engine_web_v1_boot_diagnostic_ptr`.
4. The ABI layout drops `constants.maximumDocumentBytes` and adds `parseTransientMultiplier`,
   `defaultControlFrameBytes`, `minimumControlFrameBytes` and `maximumControlFrameBytes`.
5. The SDK's `BootOptions`, `OfflineEngineOptions` and browser `policy` carry
   `maximumControlFrameBytes` and `maximumDocumentBytes` to both boots.

## Non-goals

- A transaction body beside the frame (option B), any protocol or C ABI change.
- Splitting an oversized transaction (the caller splits it; #2190 types the SDK's refusal).
- The page host's local refusal record keeps its frozen shape (`miso.error.v1`, result 1).

## Gates (one test per claim)

1. `hosts/host-web/src/tests.rs`,
   `configured_frame::a_bus_add_over_sixty_four_tracks_commits_within_the_booted_frame`: at the
   16 KiB floor, a 32 KiB word and the default, `CAPABILITIES_GET` names the frame and a replay
   cache of at least two frames; a 64-track bus add (over 16 KiB) is refused naming its bytes at
   the floor and commits at 32 KiB and 128 KiB; one byte over the frame is refused naming both.
   Test value: red when a compiled 16 KiB survives on the frame's path or the replay cache stays
   at 64 KiB. Supersedes `control_admission.rs`'s 16,384/16,385 staging asserts (deleted).
2. `exact_retained_total_is_checked_as_one_budget_not_independent_caps` (folded): the exact
   budget boots and one byte less refuses at 16 KiB and 128 KiB, and between them the aggregate
   grows by exactly the staging plus the control plane's growth, which equals the projected
   `control_frame_rows` growth. Test value: red when the staging is charged at a compiled frame or
   the control plane charges a frame-scaled row the pre-allocation projection leaves out.
3. `the_retired_boot_words_are_reserved_and_the_frame_word_is_bounded` (renamed fold): the frame
   word at 16 KiB - 1 and 16 MiB + 1 is refused `web.options.control_frame_bytes`. Test value: red
   when an unbounded word boots.
4. `raw_ffi_validates_handle_layout_overflow_and_transactional_failure` and
   `a_replace_document_up_to_the_boot_bound_commits_and_is_freed_after_its_command` (folds):
   `document_ptr` refuses one byte over the budget's limit and over the word, each with a
   diagnostic naming bytes and limit, admits the limit, and stages 2 MiB at the default; a 2 MiB
   document boots and a 2 MiB `SESSION_REPLACE` commits; under a 1 MiB word, 1 MiB commits and
   1 MiB + 1 stages nothing. The `MAXIMUM_DOCUMENT_BYTES == 1 << 20` pin is deleted. Test value:
   red when a compiled 1 MiB survives at the boot, either staging, or the word misses the replace
   staging.
5. `tests/boot_transient_budget.rs`: a dense 1536-track x 4-effect document (4,212,583 bytes,
   four times the retired ceiling) joins the pinned-multiplier cases. Test value: red when the
   parse transient grows faster than 20 bytes per byte past the old ceiling, which the derived
   default now admits.
6. `sdk/test/browser-defaults-evals.mjs`, "the host request carries the policy's frame and document
   words, zero when absent", with the policy-word row in `browser-evals.mjs` (both boots, by
   layout name). Test value: red when the host mirror drops either word.
7. Browser qualification, gate `console-limits` (`qualification.js`
   `runConsoleLimitsQualification`, `run.mjs`), both legs: a 256-track console-shaped session (a
   document over 1 MiB) boots at default options and its bus add over all 256 tracks (one frame
   over 16 KiB) commits on the rebuild path inside the reported 128 KiB frame, with the watermark
   covering it. Mutations `console-limits-refused` and `console-limits-small-document`. Test
   value: red on a JS-side 16384 (host, Worker or worklet) and on the worker-mode transfer path,
   which Rust tests cannot see.
8. `tests/boot_transient_budget.rs`,
   `a_frame_the_budget_cannot_hold_is_refused_before_anything_is_allocated`: a 1 MiB frame under
   a 4 MiB budget (below its 4.25 MB of rows, far above the document's parse transient) is refused
   `host.budget.parse_projection`, naming the frame and its rows, with no allocation past the
   diagnostic. Test value: red when the pre-allocation admission leaves out the frame rows.
   The retained-projection drift test, `compile_resource_caps_are_inclusive_and_one_below_rejects`
   and `scripts/check-web-boot-budget.mjs` run at the default frame again; the last pins the wasm
   boot growth at the engine's own projection, `20 * document + rows(frame)` plus one page, read
   from its refusal diagnostic.

## Paths

`crates/control-plane/src/{compile,lib}.rs`, `crates/protocol/src/{message_wire,lib}.rs`,
`hosts/host-web/src/{lib,ffi,tests}.rs`, `hosts/host-web/tests/{boot_transient_budget,control_admission}.rs`,
`hosts/host-web/examples/worst_boot_document.rs`, `hosts/host-web/web/*` (host, worklet, Worker,
`.d.ts`), `hosts/host-web/qualification/*`, `hosts/host-web/tests/browser-v1/{expected.json,browser-correctness.js}`,
`hosts/host-web/{DEPLOYMENT,BROWSER_DEPLOYMENT_MATRIX,MUTATIONS}.md`,
`tools/parameter-metadata/{src,tests}/abi_layout.rs`, `scripts/{check-abi-layout-v1.py,check-browser-expected-resources.py,check-web-audioworklet.sh,check-web-boot-budget.mjs,test-web-audioworklet.mjs,web-mixing-automation-benchmark.mjs}`,
`scripts/fixtures/abi-layout-v1-self-test.json`, `sdk/{assets,src,test}`, `sdk/README.md`.

## Evidence, attempt 2 (2026-10-28, on attempt 1 `a1026b706`)

- Red: the frame rows taken out of the parse projection: gate 8 fails (refused only at
  `host.budget.retained_projection`, after the parse and compile allocated). The replay cache
  taken out of `FrameRows::retained`: gate 2 fails (control growth 425,984 against projected
  229,376).
- Shipped module, the 1 MiB worst document: a budget of the exact retained size (17,567,895)
  is refused at staging (`projected_bytes=21542912`), memory grown one page; a 1 MiB frame under a
  20 MiB budget is refused at staging (`control_frame_rows_bytes=4241408`), one page. Attempt 1
  admitted the first and grew 1.1 MB past the budget, and refused the second only after 14.8 MB.
  `check-web-boot-budget.mjs` at the default frame: growth 19,726,336 of projection 21,542,912.
- Retained bytes after the counter cap (native, one-track session): frame rows 154,112 at 16
  KiB, 580,096 at 128 KiB, 4,250,112 at 1 MiB; exact retained 639,319 at 16 KiB, 1,179,991 at
  128 KiB (attempt 1: 829,775 and 4,974,927 at 256 KiB). About 4.7 bytes per frame byte with the
  staging. Browser fixture, shipped module: `bridgeRetainedBytes` 963,675, `bridgeMetadataBytes`
  811,879 (ceilings re-pinned to +10 %: 1,060,096 and 893,120), memory peak 2,490,368
  (`memoryBytesCeiling` 2,621,440); exact rows 131,072. Drift bounds re-pinned at the default
  frame: 513, 942 and 2,901 per mille (measured 475, 872, 2,686).
- Parse transient near the derived default (PR evidence, not committed; release, the dense
  shape of gate 5): 9,100 tracks x 4 effects, 24,968,199 bytes, parse + model + compile peak
  308,430,347 bytes = **12.353 bytes per input byte**, under the pinned 20.
- **Does not hold for the whole boot** (stopped here, nothing adjusted): the same dense shape's
  full native `boot` peaks at about 38.6 bytes per input byte at every size (192 tracks: 20.4 MB
  for 527 KB; 1536: 162.6 MB for 4.2 MB; 3072: 325.3 MB for 8.4 MB). At 6,000 tracks (16.5 MB,
  admitted by the derived default) it peaks at 638 MB and is refused only by
  `host.budget.retained_exact` (538,058,071 > 536,870,912); at 9,100 tracks (25 MB) it peaks at
  939 MB before `graph.resource.limit`. The pre-allocation projections (`20 * document`, and the
  retained projection, which #239 allows to undershoot the exact row) admit documents whose
  boot exceeds the budget; on wasm such a boot would exhaust the 512 MiB memory. Root to rule.
- Browser qualification: Chromium single and worker, WebKit single and worker, with
  `--self-test-mutations` (Chromium single with `--check-matrix`): `controlFrameBytes` 131,072,
  bus add frame 85,024 bytes, revision 0 -> 1, path rebuild, watermark 1, render allocations 0.
  One WebKit worker run failed `sdk-protocol-lossy` (a lossy event order); four reruns passed.

## Evidence, attempt 1 (2026-10-28, base `cf22f2a19`, merged with `cf6f9420e` (#2190))

Red, each by a temporary mutation of the implementation, then restored:

- Gate 1: replay cache fixed at 64 KiB: `CAPABILITIES_GET` at the default frame is answered
  backpressure (panic "a capabilities reply"). The core's frame compiled at 16 KiB: capabilities
  report 16384 for the 32 KiB word.
- Gate 2: the page staging charged at 16 KiB: "the staging and the control rows carry the whole
  growth" (left 3,899,392). The replay mutation is gate 1's claim and stays green here.
- Gate 3: the frame-word refusal removed: "a nonzero reserved word must not boot".
- Gate 4: a compiled 1 MiB at the boot (the 2 MiB boot refused), at `replace_document_ptr` (the
  2 MiB replacement not staged), at `document_ptr` (result 1 at 2 MiB); the word missing from the
  replace staging (1 MiB + 1 staged).
- Gate 5: a measurement row: 11.275 bytes per input byte at 4,212,583 bytes, under the pinned 20;
  no mutation was found that turns only this row red.
- Gate 6: the host mirror writing `maximumDocumentBytes: 0n`: `not ok`.
- Gate 7: the page host's frame check restored to a literal 16384 (artifact copy): Chromium
  worker leg fails `console-limits qualification failed {"tag":"miso.error.v1","requestId":0,"result":1}`.

Green: Chromium single and worker, WebKit single and worker (Playwright 1.62.1, local pulseaudio
null sink, `--self-test-mutations`; Chromium single with `--check-matrix`): document 1,075,815
bytes, bus add frame 85,024 bytes, `controlFrameBytes` 262,144, revision 0 -> 1, path rebuild,
watermark 1, render allocations 0.

Measured retained memory (native, one-track identity session): exact retained 829,775 bytes at a
16 KiB frame and 4,974,927 at 256 KiB (control plane row 589,161 -> 4,488,553), about 16.9 bytes
per frame byte. Shipped module, browser fixture: `bridgeRetainedBytes` 4,758,611 (was under
671,616), linear memory peak 6,422,528 bytes (was under 2 MiB); `expected.json` re-pinned with
these reasons. At the default frame the 1 MiB worst document's boot high-water is 22,478,848
bytes, over `check-web-boot-budget.mjs`'s 20x-plus-one-page document pin, so that gate boots at
the 16 KiB floor: the frame rows are a fixed boot cost, not a function of the document. The
retained-projection drift test is pinned at the floor for the same reason (at the default its
gaps are 14.9, 14.6 and 8.8 times the projection).

Corrections to the mapping:

- The frame-scaled rows are about 15 to 17 bytes per frame byte, about 4.1 MB at 256 KiB, not the
  estimated 2.25 MiB: `CounterId` is 4 bytes, so its two tables alone are 4F.
- `session.replace.document_too_large` is not reachable through the browser under the word: the
  replace staging refuses the length first, and the core checks the length against the frame's
  `document-bytes` before the limit. Gate 4 asserts the staging's refusal and the exact-limit commit.
- `document_ptr` had no way to return a diagnostic; `miso_engine_web_v1_boot_diagnostic_ptr` is a
  second new export.
- A document over the default limit is now `refusedBudget` (`host.budget.parse_projection`), not
  `refusedDocument`.
- The JS boot-option key renames and the headless SDK words had to land with slices 1 and 2 so each
  commit stays green; slices 1 and 2 left `check-web-audioworklet.sh`'s frozen export list red
  until slice 3.
- `BROWSER_DEPLOYMENT_MATRIX.md` is generated by `qualification/generate-matrix.mjs`.
- The benchmark record key `boot_reserved1` (committed artifacts' schema) is unchanged.
