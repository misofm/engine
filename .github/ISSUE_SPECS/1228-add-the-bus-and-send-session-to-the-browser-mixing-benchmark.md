# Add the bus-and-send session to the browser mixing benchmark

Successor performance issue BM2, outside *Submix strips and live aux sends* (#1196, filed by *Record the
submix, send and VCA ruling* (#1197) as a standalone issue). It is tooling only: no engine change and no
timing. *Record the bus-and-send baseline and its route-work profile* (#1229, BM3) times what this freezes.

## Product outcome

The V8 mixing benchmark renders the bus-and-send session as a third document, in the shipped
module, booted the way every document of that benchmark boots: through host-core **with** live
controls (the SDK's default command queue). With live controls, every route into a submix is a live
route (`NodeKind::LiveRoute`): 192 of the session's 202 routes, the path the browser mixer runs. BM3
times it beside the native static row of *Add a bus-and-send row to the native console benchmark*
(#1227, BM1).

The document list, its validator, the in-run distinctness checks and the browser half of the
mutation test are frozen here, and the untimed `prepare` and `preflight` steps prove the document
boots and renders before anything is timed.

## Context (verified on `main` at `1cb677a76`; BM1 adds the fixture, the row and `sends_console_fixture`)

- **The V8 benchmark** (`scripts/web-mixing-automation-benchmark.mjs`):
  - "two documents" is written in the header comment (`:27-46`), the comment at `:186`, the
    assertion message at `:231` and the comment at `:468-469`;
  - `loadDocument` (`:137-149`) asserts one source and no automation, and stretches the one source
    by replacing its single `"frames": "<n>"` spelling;
  - `DOCUMENT_KINDS` (`:191`) is `["sixty_four_track_console", "sixty_four_track_app_shape"]`; the
    controls table's documents must equal it (`:230-231`), and each document is checked against
    its table facts per **track** (`trackLayout`, `trackEffects`, `bypassCensus`, `:232-245`);
  - `boot()` (`:306-348`) sets `liveControlCommandQueueRecords` to the ABI layout's
    `defaultCommandQueueRecords` (64, `:91`) and `liveControlMeterBlocks`,
    `liveControlObservationTaps` and `liveControlMasterTrackPlusOne` to 0. The record states the
    queue as `console_command_queue_records`, pinned `== 64` (`scripts/web-mixing-automation-lib.jq:82`);
  - the document preflight (`documentPreflight`, `:470-487`) asserts each document audible and
    compares **only** `digests[DOCUMENT_KINDS[0]]` with `[1]` (`:484-485`);
  - the run boots the documents after the arms, times them alternated per observation, asserts each
    audible, and compares **only** `documentDigests[0]` with `[1]` (`:566-584`);
  - the record's `statistical_method` says "then the two documents alternated per observation"
    (`:656`); the validator requires only a non-empty string there.
- **Which routes are live, and the boot's budgets.** `LiveRouteState::live_routes` (`crates/host-core/src/live_route_state.rs:78-83`):
  every route whose destination is a submix, when the plan is prepared with live controls. The
  browser host's caps bound submixes, routes and effects by `u64::MAX`
  (`hosts/host-web/src/lib.rs:6587-6592`); its document-size, parse-projection and memory ceilings
  (`:66`, `:80`, `:115`) hold with wide margins for this 379 KB document (BM1's verifier booted a
  draft of it in `host_web.wasm` with today's options: 192 live routes, about 11 MiB). So the
  document needs no boot option beyond today's.
- **The document list** comes from `const DOCUMENTS: [Workload; 2]`
  (`tools/console-workload/examples/mixing_automation_controls.rs:27-30`), which writes each
  `Workload`'s kind, fixture, track count, strip content and layout, input and bypass census into
  the controls table (`:81-102`). Its module doc (`:1-15`) and the constant's doc
  (`:25-26`) describe two documents.
- **The "2" pins.**
  - The runner's `prepare` checks `(.controls | length == 8) and (.documents | length == 2)`
    (`scripts/run-web-mixing-automation-benchmark.sh:84`); its header (`:10-13`, `:23-24`)
    describes two documents.
  - `web_documents_valid` (`scripts/web-mixing-automation-lib.jq:35-48`) requires `length == 2`,
    positional `.[0]` and `.[1]` clauses, and `.[0].output_sha256 != .[1].output_sha256`; the file
    header (`:1-22`) and the comment above the definition (`:29-33`) describe two documents. `web_mixing_rounds_valid` (`:139-143`) compares the
    documents' facts and digests across the rounds, whatever their number. The library includes
    `console-benchmark-record-lib` (`:25`), so BM1's `sends_console_fixture` is in scope.
- **The runner's subcommands** (`scripts/run-web-mixing-automation-benchmark.sh:15-40`):
  `prepare WORKDIR` (untimed: requires an empty WORKDIR and unmodified tracked files, builds the
  module with `scripts/build-web-audioworklet.sh --module-only` and writes the controls table and
  `provenance.json`), `preflight WORKDIR` (untimed: the arms' premises and each document's), and
  `run WORKDIR --step NAME` (the one timed invocation).
- **The browser part of the mutation test** (`scripts/test-console-benchmark.sh`, from the header
  `# #1011: the browser arm of the mixing-automation row` at `:1424` to the end; required CI at
  `.github/workflows/qualification.yml:1036`). BM1 inserts lines above it, so find these by their
  text, not their line numbers:
  - the base round's `documents` (`:1466-1477`) holds two entries with digests `($a[0:63] + "6")`
    and `($a[0:63] + "7")`;
  - `web_document_mutation` cases (`:1568-1591`); `'.documents |= . + [.[0]]' 'a third document'`
    (`:1573`) stays refused on a three-document base (it makes four), so only its label changes;
  - `'.documents |= reverse'` (`:1572`) and `'.documents |= .[0:1]'` (`:1571`) stay refused;
  - cross-round cases on the documents' digests (`:1619-1620`), using suffixes `8` and `9`;
  - the stub-harness runner cases (`:1623-1745`) print `web_template`, which is derived from the
    base round (`:1631`), so they follow the three-document base with no change.
- **After BM1:** `fixtures/session/v1/console-sixty-four-track-sends.json` exists (one source,
  no automation, so `loadDocument` accepts it; its 64 tracks carry the intended strip and no
  bypass), `Workload::SixtyFourTrackConsoleSends` states those facts, and
  `scripts/console-benchmark-record-lib.jq` defines `sends_console_fixture`.

## Decisions frozen for this issue

- **D1. The document.** `DOCUMENTS` becomes `[Workload; 3]`, appending
  `Workload::SixtyFourTrackConsoleSends`; `DOCUMENT_KINDS` appends
  `"sixty_four_track_console_sends"`. The boot is unchanged: live controls with the default
  command queue (64 records), no meters, no observation taps, no master. Every document of this
  benchmark boots that way, so the sends document differs from the standing console's by the
  buses and the (live) sends alone; meters are the native metered row's subject, not this one's.
- **D2. Validation.** The runner's `prepare` check becomes
  `(.controls | length == 8) and (.documents | length == 3)`.
  `web_documents_valid` becomes `length == 3`: `.[0]` and `.[1]` unchanged, a new `.[2]` clause
  (`workload_kind == "sixty_four_track_console_sends"`, `fixture_id == sends_console_fixture`,
  `strip_content == "eq+compressor+limiter"`, `strip_layout == intended_layout`,
  `bypass_pattern == "none"`, `bypassed_tracks == 0`), and all three digests pairwise distinct
  (`map(.output_sha256) | unique | length == 3`).
- **D3. In-run checks.** The document preflight (`:484-485`) and the run (`:584`) assert all
  three digests pairwise distinct, each failure naming the pair. The `statistical_method` string,
  the assertion message at `:231` and every comment the Context lists that says "two documents"
  (in the `.mjs`, the example, the runner and the library) say three.
- **D4. The mutation test's browser part.**
  - The base round's `documents` gains the third entry, at index 2, with the sends fixture's facts
    and the digest `($a[0:63] + "5")`, a suffix no other base value of the browser part uses.
  - `'.documents |= . + [.[0]]'` is relabelled `'a fourth document'`.
  - New refused cases: `'.documents |= .[0:2]'` (the sends document missing);
    `'.documents[2].workload_kind = "sixty_four_track_console"'`;
    `'.documents[2].fixture_id = "fixtures/session/v1/console-sixty-four-track-intended.json"'`;
    `'.documents[2].bypass_pattern = "index_mod_3_is_2" | .documents[2].bypassed_tracks = 21'`;
    `'.documents[2].strip_content = "eq+compressor"'`;
    `'.documents[2].strip_layout = "pre_insert:eq+compressor"'`;
    `'.documents[2].bypassed_tracks = 1'` (pattern left `none`);
    `'.documents[2].output_sha256 = .documents[0].output_sha256'`;
    `'.documents[2].output_sha256 = .documents[1].output_sha256'`.
  - A new cross-round case: round two's `.documents[2].output_sha256` set to
    `"${digest_a:0:63}0"`.

## Deliverables

1. D1 in `tools/console-workload/examples/mixing_automation_controls.rs` (with its module doc and the constant's doc) and
   in `scripts/web-mixing-automation-benchmark.mjs`.
2. D2 in `scripts/run-web-mixing-automation-benchmark.sh` and `scripts/web-mixing-automation-lib.jq`
   (with their header comments); D3 in the `.mjs`.
3. D4 in `scripts/test-console-benchmark.sh`.

## Authorized paths

- `tools/console-workload/examples/mixing_automation_controls.rs`
- `scripts/web-mixing-automation-benchmark.mjs`
- `scripts/run-web-mixing-automation-benchmark.sh` (the `documents` count and header comment only)
- `scripts/web-mixing-automation-lib.jq`
- `scripts/test-console-benchmark.sh` (the browser part, `:1423-`)
- this spec

## Non-goals

- No re-validation of committed records: the two-document `web-mixing-automation.jsonl` files
  under `artifacts/steps/` were accepted by the validator of their day and will not pass D2's
  three-document rule; nothing re-checks them, and they are not edited.
- No timed run: `run` is never invoked here.
- No change to the boot options, to the control arms or to the two existing documents.
- No per-N browser documents and no meters on the documents.
- No engine change and no native-row change (BM1).

## Hazards

- **Positional validation.** A clause written against the wrong index passes a malformed record.
  Keep `.[0]` and `.[1]` exactly as they are, and add `.[2]`.
- **A pairwise check that is not.** Comparing only `.[2]` with `.[0]` lets a copied `.[1]` digest
  through; D2, D3 and D4 cover every pair.
- **The clean-tree rule.** `prepare` refuses modified tracked files and a non-empty WORKDIR, and
  records the commit it built at. Run gate 1 on a committed checkpoint, with a fresh empty WORKDIR
  outside the repository (for example `mktemp -d`).

## Objective gates

1. **The document boots and renders.** On a committed checkpoint, with a fresh empty `<WORKDIR>`:
   `bash scripts/run-web-mixing-automation-benchmark.sh prepare <WORKDIR>`, then
   `bash scripts/run-web-mixing-automation-benchmark.sh preflight <WORKDIR>`. Both exit 0, and the
   preflight's JSON `documents` object names all three kinds with three distinct digests (each
   document asserted audible in-run). Neither launches `run`.
2. **The validator is held.** `bash scripts/test-console-benchmark.sh` passes: the three-document
   base round is accepted and every D4 case is refused.
   *Test value: it turns red if a browser round is accepted with the sends document missing,
   mislabelled, booted from another fixture, or carrying another document's digest, so the
   baseline could publish the wrong session's cost under the sends document's name.*
3. **The native side still builds.**
   - `cargo run --locked --release -p console-workload --example mixing_automation_controls | jq -e '.documents | length == 3'`;
   - `cargo test --locked --release -p audit -p bench -p console-workload`;
   - `cargo fmt --all -- --check`;
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`;
   - `bash scripts/check-workspace-policy.sh` and `bash scripts/test-workspace-policy.sh`.

## Evidence

- The `prepare` and `preflight` output.
- The `test-console-benchmark.sh` summary line and the new cases.

### Attempt 1 record (Terra, on `e7bd95f88`)

Anchors had moved only by BM1's insertions in `test-console-benchmark.sh` (the browser part now
starts at `:1454`); every one was found by its text. Nothing was timed and `run` was never invoked.

- **D1-D4 as frozen.** `DOCUMENTS: [Workload; 3]` appends `SixtyFourTrackConsoleSends`;
  `DOCUMENT_KINDS` appends `sixty_four_track_console_sends`; boot options unchanged (live controls,
  64-record queue, no meters, taps or master). `prepare` checks `.documents | length == 3`.
  `web_documents_valid`: `length == 3`, `.[0]`/`.[1]` untouched, a new `.[2]` clause, and
  `(map(.output_sha256) | unique | length) == 3`. The `.mjs` checks every pair through one helper
  (`assertDocumentsDistinct`, used by the document preflight and the run), each failure naming
  the pair; the statistical method, the table assertion and the header comments say three.
- **Controls table** (`cargo run ... --example mixing_automation_controls`): `.documents | length
  == 3`; the third entry is `sixty_four_track_console_sends`, the sends fixture, 64 tracks,
  `eq+compressor+limiter`, `pre_insert:eq+compressor,post_insert:limiter`, tone, `none`, 0.
- **Harness preflight on the uncommitted tree** (module from `build-web-audioworklet.sh
  --module-only`, `30d075d3...aeff4`): exit 0, `documents` = console `d913ad96...41b1`, app shape
  `3dd8b2ff...d645`, sends `cf5aca93...7002`, each asserted audible. Gate 1 on the committed
  checkpoint is recorded below.
- **Gate 2.** `test-console-benchmark.sh`: `console benchmark validators: PASS (real
  runner/workload/timing invocations: 0/0/0; browser runner on a stub harness, untimed)`. New
  cases: the 9 D4 refusals, the relabelled `a fourth document`, and the cross-round sends-digest
  case; the base round's third document carries digest suffix `5`.
- **Gate 3.** Example `| jq -e '.documents | length == 3'` true; `cargo test --locked --release -p
  audit -p bench -p console-workload` 113 passed, 0 failed; `cargo fmt --all -- --check`, workspace
  clippy `-D warnings`, `check-workspace-policy.sh` and `test-workspace-policy.sh` pass.
  `check-web-audioworklet.sh` and `test-web-audioworklet.sh` also pass. The Playwright browser
  legs were not run: no browser harness, SDK or session file changed.
- **Test value and mutations run** (each red, then restored):
  - dropping the `.[2]` clause: the six fact refusals (kind, fixture, app bypass, content, layout,
    a bypassed track) go red -- a round that names the wrong session under the sends name;
  - `length >= 2` with no `.[2]` clause and a length-relative digest rule: `.documents |= .[0:2]`
    also goes red -- a round that timed no sends document;
  - `length >= 2` alone: `a fourth document` goes red;
  - digests compared only against `.[0]`: `.documents[2] = .documents[1]`'s digest goes red; pairs
    `(0,1),(1,2)` only: `.documents[2] = .documents[0]`'s goes red -- a pairwise rule that is not;
  - round agreement over `.documents[0:2]` only: the new cross-round case goes red alone -- rounds
    that disagree on the sends document's bits;
  - harness: booting the sends entry from the standing fixture (same layout and census, so only
    the distinctness check can see it) fails the preflight with `the documents
    sixty_four_track_console and sixty_four_track_console_sends rendered the same bits`; with the
    check cut back to the old `(0,1)` pair the same mutant exits 0. The run's use of the helper is
    not exercised (it is timed).

## Dependencies

- *Add a bus-and-send row to the native console benchmark* (#1227, BM1)

## Standing rules for the implementer

- Work only from this body. Read the cited lines first; do not survey the workspace.
- Nothing is timed in this issue. Never invoke `run`.
- Do not quote a projected saving.
- A test that greps source or prose is refused.
- Commit on its own branch from synchronized `main` after BM1 has merged, or on BM1's batch
  branch.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).
