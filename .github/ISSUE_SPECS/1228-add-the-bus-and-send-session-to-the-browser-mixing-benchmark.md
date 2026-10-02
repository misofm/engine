# Add the bus-and-send session to the browser mixing benchmark

Successor performance issue BM2, outside *Submix strips and live aux sends* (#1196, filed by *Record the
submix, send and VCA ruling* (#1197) as a standalone issue). It is tooling only: no engine change and no
timing. *Record the bus-and-send baseline and its route-work profile* (#1229, BM3) times what this freezes.

## Product outcome

The V8 mixing benchmark renders the bus-and-send session in the shipped module, booted the way the
producer's mixer boots it: through host-core **with** live controls. After batch K3 its sends are
live routes (`NodeKind::LiveRoute`), which is the path the browser mixer actually runs. BM3 times it
beside the native static row of *Add a bus-and-send row to the native console benchmark* (#1227, BM1).

The document list, its validators, the in-run distinctness checks and the browser half of the
mutation test are frozen here, and the untimed `prepare` and `preflight` steps prove the document
renders before anything is timed.

## Context (verified on `fe8ac679`; BM1 adds the fixture and `sends_console_fixture`)

- **The V8 benchmark** (`scripts/web-mixing-automation-benchmark.mjs`):
  - `loadDocument` (`:137`) asserts one source and no automation, then stretches the source;
  - `DOCUMENT_KINDS` (`:191`) is `["sixty_four_track_console", "sixty_four_track_app_shape"]`, and
    the controls table's documents must match it (`:230`);
  - `boot()` (`:306`) sets `liveControlCommandQueueRecords` to the published
    `defaultCommandQueueRecords` (64, `:91`). The record states it as
    `console_command_queue_records`, pinned `== 64` (`scripts/web-mixing-automation-lib.jq:82`).
    On the host side, `compile_ready` (`hosts/host-web/src/lib.rs:5707`) prepares through host-core,
    so with a queue depth set every route into a submix is a live route after *Produce live send
    records from host-core* (#1221).
  - The in-run distinctness checks compare **only documents 0 and 1**: the preflight
    (`assert.notEqual(digests[DOCUMENT_KINDS[0]], digests[DOCUMENT_KINDS[1]], ...)`, `:484`) and the
    run (`documentDigests[0]` against `[1]`, `:584`).
- **The document list** comes from `DOCUMENTS: [Workload; 2]` in
  `tools/console-workload/examples/mixing_automation_controls.rs:27-30`, which writes each
  `Workload`'s facts into the controls table.
- **The "2" pins.**
  - The runner's `prepare` checks `(.controls | length == 8) and (.documents | length == 2)`
    (`scripts/run-web-mixing-automation-benchmark.sh:84`).
  - `web_documents_valid` (`scripts/web-mixing-automation-lib.jq:35-50`) requires `length == 2`,
    positional `.[0]` and `.[1]` clauses with each document's kind, fixture, strip content, layout and
    bypass facts, and distinct digests.
  - The rounds check `(map(.round) | sort) == [1,2]` (`web_mixing_rounds_valid`,
    `web-mixing-automation-lib.jq:141`) stays. The validator file
    `scripts/web-mixing-automation-validator.jq` is six lines that include the library.
- **The runner's subcommands** (`scripts/run-web-mixing-automation-benchmark.sh:13-36`):
  `prepare WORKDIR` (untimed; builds the module and writes the controls table; refuses a non-empty
  WORKDIR and a dirty tree), `preflight WORKDIR` (untimed; checks the arm premises and that each
  document renders audible bits of its own), and `run WORKDIR --step NAME` (the one timed
  invocation).
- **The browser section of the mutation test** (`scripts/test-console-benchmark.sh:1424-1590`,
  required CI at `.github/workflows/qualification.yml:1036`): the base web record holds exactly two
  `documents` (`:1466-`), `web_document_mutation` cases follow (`:1568-`), and the case
  `'.documents |= . + [.[0]]' 'a third document'` (`:1573`) must be **rejected**. With three documents
  in the base record that case becomes a valid record and turns the script red.
- **After BM1:** `fixtures/session/v1/console-sixty-four-track-sends.json` exists (one source, no
  automation, so `loadDocument` accepts it), `Workload::SixtyFourTrackConsoleSends` states its facts,
  and the record library defines `sends_console_fixture`.

## Decisions frozen for this issue

- **D1. The document.** `DOCUMENTS` becomes `[Workload; 3]`, appending
  `Workload::SixtyFourTrackConsoleSends`; `DOCUMENT_KINDS` appends
  `"sixty_four_track_console_sends"`. The boot is unchanged (queue depth 64, meters, observation and
  master as today), so the document measures the live-controlled producer-mixer shape as it ships.
- **D2. Validation.** The runner's `prepare` check becomes `(.documents | length == 3)`.
  `web_documents_valid` becomes `length == 3`: `.[0]` and `.[1]` unchanged, and a new `.[2]` clause
  stating the sends document's kind, `fixture_id == sends_console_fixture`, its strip content and
  layout (the intended fixture's), `bypass_pattern == "none"` and `bypassed_tracks == 0`. All three
  digests are pairwise distinct.
- **D3. In-run checks.** The preflight (`:484`) and run (`:584`) distinctness assertions become
  pairwise over all three documents.
- **D4. The mutation test's browser section.** The base web record holds three documents; the
  `'a third document'` case becomes `'.documents |= . + [.[0]]' 'a fourth document'`; new cases
  refuse a round without the sends document (`.documents |= .[0:2]`), one whose
  `.[2].workload_kind` is wrong, and one whose `.[2]` digest equals `.[0]`'s.

## Deliverables

- D1 in the example and the `.mjs`.
- D2 in the runner and the library; D3 in the `.mjs`.
- D4 in `scripts/test-console-benchmark.sh`.

## Authorized paths

- `tools/console-workload/examples/mixing_automation_controls.rs`
- `scripts/web-mixing-automation-benchmark.mjs`
- `scripts/run-web-mixing-automation-benchmark.sh` (the `documents` count only)
- `scripts/web-mixing-automation-lib.jq`
- `scripts/test-console-benchmark.sh` (the browser section, `:1424-1590`)
- this spec

## Non-goals

- No timed run: `run` is not invoked here.
- No change to the boot options, to the control arms, or to the two existing documents.
- No per-N browser rows.
- No engine change and no native-row change (BM1).

## Hazards

- **Positional validation.** A clause written against the wrong index passes a malformed record.
  Keep `.[0]` and `.[1]` exactly as they are, and add `.[2]`.
- **The clean-tree rule.** `prepare` refuses a dirty tree and a non-empty WORKDIR, so run gate 1 on a
  committed checkpoint.
- **A pairwise check that is not.** Comparing only `.[2]` with `.[0]` lets a copied `.[1]` digest
  through; D2 and D3 compare every pair.

## Objective gates

1. **The document is prepared and renders.** On a committed checkpoint with an empty `<WORKDIR>`:
   `bash scripts/run-web-mixing-automation-benchmark.sh prepare <WORKDIR>`, then
   `bash scripts/run-web-mixing-automation-benchmark.sh preflight <WORKDIR>`. Both pass; `preflight`
   reports that each of the three documents renders audible bits of its own and that the three
   digests are pairwise distinct. Neither launches the timed harness.
2. **The validator is held.** `bash scripts/test-console-benchmark.sh` passes: the three-document base
   record is accepted, and the D4 cases are refused.
   *Test value: it turns red if a browser round is accepted with the sends document missing,
   mislabelled, duplicated or carrying a copied digest, so the baseline could time the wrong
   session.*
3. **The native side still builds.**
   - `cargo run --locked --release -p console-workload --example mixing_automation_controls` prints
     three documents;
   - `cargo test --locked --release -p audit -p bench -p console-workload`;
   - `cargo fmt --all -- --check`;
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`.

## Evidence

- The `prepare` and `preflight` output.
- The `test-console-benchmark.sh` output with the new cases.

## Dependencies

- *Add a bus-and-send row to the native console benchmark* (#1227, BM1)

## Standing rules for the implementer

- Work only from this body. Read the cited lines first; do not survey the workspace.
- Nothing is timed in this issue. Never invoke `run`.
- Do not quote a projected saving.
- A test that greps source or prose is refused.
- Commit on its own branch from synchronized `main`.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).
