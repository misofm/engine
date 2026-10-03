# #1228 attempt 1 verdict (Sol): `79d11ea49` + `02feade46` (parent `e7bd95f88`), branch `codex/batch-bench`

**PASS.** No BLOCKER or MAJOR finding. There is one MINOR finding: nothing in the run proves the
sends document's routes are live. There are two NITs.

Summary of what I checked:

- D1-D4 are implemented as frozen, and only authorized paths changed.
- Gate 1 reproduces bit for bit from the committed checkpoint into a fresh, empty WORKDIR.
- Gates 2 and 3 are green.
- Every mutation I planted went red: four validator/suite mutants and four harness/table mutants
  run through the real preflight.
- I confirmed with the shipped module that the sends document boots with **192 live routes and 10
  submixes**.
- Nothing was timed. I never invoked `run`, the harness's `run` mode or a benchmark.
- I did not modify the worktree. It is clean at `02feade46` with no untracked or ignored
  artifacts. Nothing was committed or pushed, and nothing on GitHub was edited.

## Findings

### MINOR-1: no in-run proof that the sends document runs live routes

**The problem.**
- The issue's product outcome is that the sends document is timed on the browser mixer's path,
  where every route into a submix is a live route (192 of 202).
- The harness boots it that way today. I confirmed this with a probe on the prepared module:
  `miso_engine_web_v1_live_control_route_count` returns 192 for the sends document and 0 for the
  other two.
- But nothing asserts or records that count.
- I re-booted the three documents with `liveControlCommandQueueRecords = 0`, which gives static
  routes and the native row's path. The live-route count went to 0, yet the preflight still passed
  and all three digests were identical: `cf5aca93...7002` for sends.
  - Live and static routes are bit-identical by design, so no digest can tell the two paths apart.
  - The record's `console_command_queue_records == 64` comes from the ABI layout's constant, not
    from the engine.
- So if a later edit special-cased documents to boot static, #1229 would time the static path
  under the live-path name, and every gate would stay green.

**Why it is only MINOR.**
- D1 froze "boot unchanged", and the boot is correct today.
- `boot()` is shared with the arms, which need the command queue, so a regression would have to
  special-case documents deliberately.

**Fix.** It is one assertion in the `.mjs`, with no record or validator change:
- in `makeDocument` (or in `boot()` for documents), assert
  `e.miso_engine_web_v1_live_control_route_count(handle)` equals the fixture's count of routes
  with `destination.kind === "submix_input"`;
- for the sends document, also assert that count is greater than 0.

**Test value.** It turns red if a document is booted without live controls, which today goes red
nowhere. Fold it in if another attempt happens; otherwise put it in #1229's brief before anything
is timed.

### NIT-1: two new comment lines are not wrapped

Both are new prose lines that run well past the 100-column wrap of the prose around them:
- `scripts/web-mixing-automation-benchmark.mjs:35` is 127 columns;
- `tools/console-workload/examples/mixing_automation_controls.rs:11` is 129 columns.

`cargo fmt` does not wrap comments, so no gate catches this. It is cosmetic.

### NIT-2: the D4 `.documents |= .[0:2]` case has a second guard

Both `length == 3` and the `.[2]` clause refuse that case (`null | .workload_kind` is not the
sends kind), so the case is not uniquely held by the length rule. The attempt record's mutant shows
this correctly. There is nothing to change; this is noted only so a later reviewer does not
mistake it for a gap.

## Verification

### Diff and scope

- `git diff --name-only e7bd95f88 02feade46` lists exactly the authorized paths: the spec, the
  runner, `test-console-benchmark.sh`, the `.mjs`, the `.jq` library and the example.
- The runner changes only the `documents` count and its header comments.
- The suite changes are all inside the browser part (`:1454-`).
- The two existing documents' `.[0]` and `.[1]` clauses are unchanged.
- No stale "two documents" wording is left. The suite's `'browser rounds on two documents'` label
  predates this change and is about `source_frames`.

### The real path (D1)

- `boot()` sets `liveControlCommandQueueRecords = defaultCommandQueueRecords` (64), with 0 meter
  blocks, 0 observation taps and no master.
- That reaches `host-web::live_control_request` and then `control_queue_depth = Some(64)`. In
  `host-core/src/prepare.rs:1127`, `attach_route_live_controls(depth)` attaches one lane per route
  into a submix, and `graph/src/runtime.rs:4503` builds `NodeKind::LiveRoute` for each.
- The fixture has 192 `submix_input` routes and 10 `output_input` routes.
- The probe confirmed live routes 192, submixes 10 and tracks 64 on the shipped module (see
  MINOR-1).

### Gate 1, reproduced from the committed checkpoint

I made a `git clone --shared` of the repository at `02feade46`; its tree is identical to the
`git archive` export (`diff -r` is empty). Then I ran `prepare` into a fresh `mktemp -d` WORKDIR.

**`prepare` exited 0:**
- `host_web.wasm 30d075d3ce6382f21235675996184c675753acf6d451e11d7d676a3d50aaeff4 at 02feade46...`,
  which is not the release pin `6c952a2c...`;
- `provenance.json` records commit `02feade46526...` and controls
  `54c90a0f096d32692306cdb4c79223bbcaadae3304d3fc06d989af5117616666`.

The module and the controls digests are the same as in the attempt record.

**`preflight` exited 0 in 2 s.** Its `documents` were:

| Document | Digest |
|---|---|
| `sixty_four_track_console` | `d913ad961d2d1d78c7a2bd8b4ee0148a107a105c42bbded7ec4494762ee441b1` |
| `sixty_four_track_app_shape` | `3dd8b2fff4b95dba31c0d184543a2a96f9aa8081ab21d74ae58701825af0d645` |
| `sixty_four_track_console_sends` | `cf5aca937d70efaf0eaffd4b91e5fe82af584e6ca013d6074e14fe59951f7002` |

- These match the attempt record.
- The three digests are distinct, and each document was asserted audible.
- The clone's `git status` stayed clean, and no `artifacts/steps/` entry was created.

**Overwrite refusal is intact.** Re-running `prepare` on the now non-empty WORKDIR gave
`WORKDIR must be empty; refusing overwrite` (exit 2), and the WORKDIR's digests were unchanged. The
suite's stub-runner overwrite, race and provenance cases are unchanged and pass.

### Gate 2

`bash scripts/test-console-benchmark.sh` on the export printed `console benchmark validators: PASS
(real runner/workload/timing invocations: 0/0/0; browser runner on a stub harness, untimed)`.

### My own mutations

Each mutant ran on its own copy of `scripts/`.

**Validator and suite mutants** (all red):

| Mutant | Result |
|---|---|
| M1: `(map(.output_sha256) \| unique \| length) >= 2` | 3 cases red: sends = console bits, sends = app bits, and the existing "one session booted twice" |
| M2: `.[2]` also accepts `pre_insert:eq+compressor` | 1 case red: "claiming the app layout" |
| M3: `.[2]` fixture pinned only by a `console-sixty-four-track` prefix | 1 case red: "booted from the standing fixture" |
| M4: base round's sends digest set to suffix `7`, colliding with the app shape | 5 red: both base accepts and the stub runner's undisturbed run, so the base round's distinctness is load-bearing |

**Harness mutants** (the real preflight on the prepared module):

| Mutant | Result |
|---|---|
| H1: table's sends `fixture_id` set to the intended console fixture (same facts) | exit 1: `the documents sixty_four_track_console and sixty_four_track_console_sends rendered the same bits` |
| H2: table's sends `fixture_id` set to the app fixture | exit 1 on the facts check: `sixty_four_track_console_sends ch00: layout` |
| H3: app entry and sends entry both point at the sends fixture with the sends facts (pair 1,2) | exit 1: `the documents sixty_four_track_app_shape and sixty_four_track_console_sends rendered the same bits` |
| H4: `assertDocumentsDistinct` cut to pairs from index 0 only | H3's table exits 0, H1's still fails, so the full pairwise check is what refuses pair (1,2) |

**The run phase (D3).** The run phase calls the same helper, keyed by `workload_kind`. Its key
order is pinned by the `DOCUMENT_KINDS` deepEqual. I checked it by reading the code, not by running
it, because it is timed.

### Gate 3, on the export

All with `CARGO_TARGET_DIR` under `/tmp/claude-1002/v1228/`:
- the example piped to `jq -e '.documents | length == 3'` prints `true`, and the sends entry carries
  the fixture's facts;
- `cargo test --locked --release -p audit -p bench -p console-workload` passes 113 tests and fails
  none;
- `cargo fmt --all -- --check`, workspace clippy with `-D warnings`, `check-workspace-policy.sh`
  (also re-run in the git clone) and `test-workspace-policy.sh` all pass.

### The skipped Playwright legs are equivalent in CI

- No changed path is under `hosts/host-web/**`, `sdk/**`, `fixtures/**` or `crates/**`.
- `console-workload` is not in `host-web`'s or `parameter-metadata`'s dependency tree, so the
  module and its delivery closure are byte-identical to the parent's.
- Nothing in `hosts/host-web/qualification`, `hosts/host-web/web`, `sdk/src`, `sdk/test`,
  `test-web-audioworklet.sh` or `check-web-audioworklet.sh` references any changed file.
- CI's browser qualification job therefore runs on the same inputs. I did not run it locally.
- The changed `.mjs` and suite route to the `console-benchmark` job, which runs
  `test-console-benchmark.sh`; that run is gate 2 above. `node --check` on the `.mjs` passes.

### Ceremony and test value

- The nine new D4 cases, the relabel and the cross-round case each defend one clause of `.[2]`, or
  one pair, or the length.
- No case greps source or prose, and nothing byte-pins prose.
- The spec's test-value sentence holds: the suite turns red on a round accepted with the sends
  document missing, mislabelled, booted from another fixture, or carrying another document's
  digest.

## Scratch

Everything under `/tmp/claude-1002/v1228/` was deleted after this verdict: the export, the clone,
the WORKDIR, the target directories, the mutants and the probe scripts.
