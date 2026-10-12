PASS

# #1399 attempt 1: adversarial verdict

Commit under review: `731c8be30` on `codex/d15-stream-b3` (parent `c24dbc321`). I exported the commit with
`git archive` to `/tmp/claude-1002/v1399/tree`. I built it with a private `CARGO_TARGET_DIR`. I never read
or built in the worktree. The spec body at `731c8be30` is the same as the body at its parent. The commit
adds only the Attempt record.

No BLOCKER and no MAJOR. Every frozen decision (D1, D2), every objective gate and every red run that the
Attempt record claims is reproduced. The findings below are spec gaps and bookkeeping that root must
route. None of them is a defect in what the spec asked for.

## Answers to the five questions

1. **One count per session on the browser.** Correct. `compile_ready` makes one `LiveValuesSuperseded`
   (`hosts/host-web/src/lib.rs:6763`). It passes the count to whichever of the three browser entries it
   calls (`:6790`, `:6807`, `:6823`) and keeps it in `ReadyOwnership` (`:7319`). The bridge charges it
   once in `ready_metadata` (`:6840-6845`). host-core charges a count only when it made the count
   (`crates/host-core/src/prepare.rs:1684`, `superseded.is_none() && !control_requests.is_empty()`). The
   browser always allocates and keeps the count, even when no strip reader exists, so an unconditional
   bridge charge is exact. Every production session path now passes a session count: the control plane
   calls `prepare_host_runtime_with_live_lanes*` and host-web calls the three changed entries.
   The standalone `prepare_host_runtime_with_spectrum_collection` (`prepare.rs:1181-1205`) is correct,
   not an inconsistency. It has no session and no live controls (`HostLiveControlRequest::default()`, so
   `control_requests` is empty). It keeps no reader and charges nothing, and it drops the handles. Only
   `crates/host-core/tests/spectrum.rs` calls it. Its inlined body is the same as the old delegation with
   `None`.
2. **Status word at 48, reserved `[u64; 3]`, loaded after every rendered block.** Correct. The render side
   is one `Relaxed` load (`LiveValuesSuperseded::get`) and a plain store into `self.status`
   (`lib.rs:3265`). It is inside `render_next`'s `REALTIME_POLICY` region, after `plan.render` returns
   `Ok`. It has no RMW, no allocation, no lock and no syscall. `check-realtime-policy` and `audit capi`
   (0 allocations, 0 syscalls, 0 locks) are green. I re-did the "move the load before `plan.render`"
   mutation (M2b, with the after-render load removed). Gate 1 goes red. So do the two #1346 tests that
   this commit rewrote to read the status. Gate 2 stays green, because it reads the status only after
   8 more blocks. Gate 1 covers the gap fully: it reads the status after the one block that does the
   counting. A second gate for the same line would add nothing.
3. **Files outside the spec list.**
   - The edits to `crates/host-core/src/prepare.rs` were forced. The Dependencies section (the m3
     deferral) requires host-web to pass a count, and no browser entry took one. The edit is small:
     three trailing `superseded` parameters and the standalone entry inlined.
   - The `protocol` dev-dependency was forced. `control-plane` does not re-export `CommandPayload`,
     `CountersRequest` or the codec (`crates/control-plane/src/lib.rs:60-80`).
   - The `retained_ceilings.rs` rewrite was forced. The assertion it replaces stated the old per-plan
     charge.
   - The three host-core test files change call sites only.
   - Root must ratify the STREAMS rows: see MINOR m3.
4. **The cross-host test.** It drives the real C ABI admission: the control plane's
   `SESSION_TRANSACTION_APPLY`, with `SetTrackFader` lowered to Left and Right `FaderDb` records and
   `SetTrackMatrixOrPan` lowered with the model's smoothing. It drives the real browser admission: a
   `channel = 2` `Both` record and a `COMMAND_PAN` record. It also covers both delivery shapes. The
   browser binds the fused `FaderMatrixBankProcessor` (`BetweenRenderCalls`), and the control plane binds
   the split fader and matrix stages (`Concurrent`). The cell readers (`drain_fader_controls`,
   `drain_matrix_controls`) are shared code. So "both drains" means the same reader behind two
   admissions and two stage compositions, not two reader implementations.
   I tried three one-host defects:
   - M5: the browser `Both` writes the left cell only.
   - M6: the control plane's fader ramp is 64, not 0.
   - M7: the browser pan smoothing is off by one.
   Gate 2 went red on each one. Each one was also caught by that host's own pins
   (`native_command_timeline_digest_pins_the_wasm_parity`, `records_carry_the_ramps_and_the_model_smoothing`,
   `a_live_fader_after_a_carrying_structural_commit_reaches_the_successor`, and others).
   The gate's own catch is a cross-host differential: a change to one host's admission or stage
   composition whose author also re-pins that host's own digest. Every per-host test then stays green,
   and only this test compares the two hosts' PCM bits and counts. That is a plausible defect
   (re-pins are routine) and a sufficient test value for a differential. But the test's doc comment
   names M5 as a defect "which no per-host test sees", and that is false: 10 browser tests catch M5
   (NIT n3).
5. **`suspended-host.mjs`.** It is a follow-up, not in scope. It pins a historic `SOURCE_COMMIT`
   (`35b0ff75...`), a wasm digest and a layout digest (`8cd125f0...`). The parent's layout digest
   (`5e1cf67d...`) already disagreed with it before this slice. No CI job and no `npm run qualify` runs
   it. Its two exact status field lists (`:40`, `:421`) will refuse the new field when a slice revives
   the file, so whoever revives it must add `liveValuesSuperseded` there. Record this on #1349 or on the
   revival issue.

## Findings

### BLOCKER
None.

### MAJOR
None.

### MINOR

- **m1. The SDK's headless status leaves the new word out** (`sdk/src/core/boundary.ts:290-302`, `:437-462`).
  `EngineStatus` is "A direct snapshot of the frozen status structure". `status()` is "The whole status
  structure". `sdk/src/headless/engine.ts:201` exports it to callers. The SDK reads fields by name from
  the generated layout, so nothing breaks. But an agent on the headless SDK cannot read
  `live_values_superseded`, and the doc's "whole" claim is now false. The spec's Context mirror list
  (D1) does not include this file, and the Authorized paths do not allow it, so the implementer could
  not fix it. Root should add it to #1349, which takes the next three status words and will meet the
  same reader, or to a small successor. The browser host (`MisoStatus`) does carry the word, so the
  spec's product outcome holds.
- **m2. The browser fixture's direct oracle still requires word 48 to be zero**
  (`hosts/host-web/tests/browser-v1/direct-oracle.mjs:56`, `for (const offset of [48, 56, 64, 72]) ... 0n`).
  `check-browser-expected-resources.py --artifacts` runs it (artifact-gates). It is green today only
  because the fixture's command timeline never replaces an unread value. If a fixture script does,
  the oracle fails with "word 48 nonzero" when it should report the count. This is a stale mirror of
  the status reader that the spec's Context missed. Route it with m1 (it has the same owner, stream H,
  and #1349 must edit the same line for 56-72).
- **m3. Root must ratify the STREAMS hot-file rows.**
  - `docs/handoffs/decision-15-2026-10-05/STREAMS.md` row for `crates/host-core/src/prepare.rs` (`:75`)
    does not name #1399. Stream A owns the file, and stream B's named exception (`:265`) covers #1344
    only.
  - The `Cargo.toml`, `Cargo.lock` row (`:113`) does not name #1399, which adds two dev-dependency lock
    lines.
  - The `scripts/test-web-audioworklet.mjs` row (`:109`) names only stream H's slices and J #1496. The
    #1399 spec authorizes the file, but the row does not record an order.
  - The `crates/host-core/tests/{prepare,spectrum,strip_meters}.rs` call-site edits are outside every
    row. `spectrum.rs` call sites are H #1492's named exception.

  Each edit was forced and is a few lines. Every later slice that touches these files must rebase over
  them. They need root's ratification (or a row amendment) before the batch merge, not a code change.

### NIT

- **n1.** `ReadyOwnership::live_values_superseded`'s doc (`hosts/host-web/src/lib.rs:1534-1536`) says "A
  successor plan is prepared with this same count". The browser has no successor preparation today:
  `compile_ready` (`:2174`) is its only preparation. AGENTS.md wants comments to say what the code does,
  so write it as the rule a successor must follow ("must be prepared with").
- **n2.** `LiveValuesSuperseded::get` is still documented "Control: the writes counted so far"
  (`crates/engine/src/realtime/latest_cell.rs:690-691`), but render now calls it every block. The call is
  harmless: a `Relaxed` load, with the render side its only writer. Correct the doc when stream B next
  edits that file.
- **n3.** Gate 2's doc comment (`hosts/host-web/tests/strip_cells_cross_host.rs`, the test's "Test
  value" and "Red mutation" paragraphs) says that the one-host defects it names are ones "no per-host
  test sees". My M5 is the left-cell-only `Both` mutation that the comment names. Ten host-web tests
  turn red on it. Name the real catch instead: a one-host change re-pinned on that host. The comment's
  second named mutation ("count the control plane's matrix cell as unreplaced") cannot be made on one
  host alone, because the matrix reader is shared code.
- **n4.** The Attempt record's gate 2 red run is "the load removed". Gate 1 and the two rewritten #1346
  tests also catch that mutation, so it does not show gate 2's own catch (see Q4).
- **n5.** The schema self-test fixture `scripts/fixtures/abi-layout-v1-self-test.json:148-163` still has the
  old `u64[4]` status. The checker does not check status fields, so this is harmless. #1349 can
  refresh it.

## Test value (one sentence per new or rewritten test)

- **Gate 1, `the_status_reports_the_sessions_superseded_count`** (`src/tests.rs`): it goes red when the
  status word is never written (M1) or is loaded before `plan.render` (M2b). It is the only test that
  holds a `Both` edit to 2 per cell through the status word and holds 56/64/72 to zero after a counted
  block. The two #1346 tests rewritten in this commit overlap only on M1/M2b, because they defend the
  input and send counts.
- **Moved layout asserts in `frozen_layouts_and_values_are_exact`**: they go red when the word moves off 48
  or `reserved` does not move to 56. `check-sdk-generated.sh` catches the same defect only through the
  regenerated asset.
- **Gate 2, `both_hosts_drain_two_fader_and_two_pan_edits_alike`**: it goes red when one host's
  admission or stage composition drifts from the other's and its author re-pins that host's own
  digest. Every per-host test then stays green. This is a cross-host differential, judged by what it
  reaches (see Q4).
- **Gate 3, `scripts/test-web-audioworklet.mjs` cases**: re-run here, all red:
  - J1: the reader rejects word 48 (boot `miso.ready.v1` assertion).
  - J2: the reader drops the field.
  - J3: the reader skips word 72.
  - J4: the host's field list drops the field (exit 1).
  - J5: the host has no `validU64` check ("expected rejection").
  - J6: the reader reads offset 40.

  Before this slice, no test fed a nonzero word 48 or a reply that carries the field.
- **`the_graph_row_charges_the_plans_gate_and_not_the_sessions_superseded_count`** (rewritten,
  `tests/retained_ceilings.rs`): it goes red when the browser's graph row also charges the session count,
  a double charge (M3 makes `left: 48, right: 24`). The exact-decomposition tests compare only the
  bridge rows, so they cannot see a double charge. The host-core test
  `the_graph_row_counts_the_plans_gate_and_a_count_it_made_and_holds` also catches M3 itself.
- **The two exact-decomposition tests' new term**: they go red when the browser never charges the session
  count (M4: both red). No other test sees the browser's bridge rows.

## Mutation runs (each reverted after its run; host-web `--features test-support` + host-core + control-plane + capi)

| Mutation | Red tests |
|---|---|
| base (no mutation) | none (all pass) |
| M1 drop the status load | gate 1, gate 2, `many_input_edits_...`, `many_send_edits_...` |
| M2b move the load before `plan.render` | gate 1, `many_input_edits_...`, `many_send_edits_...` (gate 2 green, as claimed) |
| M3 host-core charges a passed count | `the_graph_row_charges_the_plans_gate_and_not_...`, host-core `the_graph_row_counts_the_plans_gate_and_a_count_it_made_and_holds` |
| M4 `ready_metadata` without the count | `decoded_command_resource_is_exact_...`, `effect_control_browser_table_and_payload_reach_exact_budget_gate` |
| M5 browser `Both` fader writes left only | gate 2 and 10 host-web tests (incl. gate 1, `native_command_timeline_digest_pins_the_wasm_parity`) |
| M6 control-plane fader ramp 64 | gate 2, host-core `records_carry_the_ramps_and_the_model_smoothing`, capi `a_live_fader_after_a_carrying_structural_commit_reaches_the_successor` |
| M7 browser pan smoothing + 1 | gate 2, `command_ack_names_the_exact_application_sample`, `native_command_timeline_digest_pins_the_wasm_parity` |
| J1-J6 (worklet reader and host reply check) | `scripts/test-web-audioworklet.mjs` red on each; green baseline |

My first M2 run inserted the load before `plan.render` without removing the load after it. That run
was invalid and is not counted. M2b is the correct move.

## Gates run (on the export of `731c8be30`, this machine, no browsers)

- **`lint` job, all 32 steps: PASS.** These include fmt, clippy `-D warnings`, rustdoc `-D warnings`,
  workspace policy and its mutations, session policy, env vocabulary, bench policy, test-support CI
  coverage and its mutations, scalar-oracle self-test, script reachability and its mutations (export
  git-initialized), the console benchmark fixture, the standing builtins fixture manifest mutation
  tests, bench preconditions, host-core policy, protocol-control policy, realtime policy, audit-leak and
  artifact-evidence-leak with their mutations, the trace validator, lane policy, the unfused seal,
  rack, builtins and graph policy with their mutations (check-rack-policy PASS), effect-runtime policy
  and fixtures, conformance boundaries, the EQ render contract, release-shape self-test, npm publish
  modes, stem store, stem identity, and the three sub-v3 and AVX2+FMA probes.
- **`test-release` job, all 9 steps: PASS.** These include lane/math/wasm-gates, filter_liveness,
  tail_contract, ramp_endpoint, designer_total, the FMA cfg, the **loom SPSC leg**, M1 exhaustive and F1
  exhaustive.
- **`cargo test -p host-web --features test-support` and `-p host-core -p control-plane -p capi`: PASS**
  (the base run of the mutation sweep).
- **Worklet chain: PASS.**
  - `build-web-audioworklet.sh --named-twin`: module `5f04b994...`, the same digest as the Attempt
    record.
  - `strip-wasm-names.py --self-test` and `check`.
  - `check-web-audioworklet.sh --without-metadata-regeneration`.
  - `check-browser-expected-resources.py --artifacts` (32 red mutations).
  - `check-scalar-oracle-absent.py --wasm`.
  - `test-web-audioworklet.sh` with a private TMPDIR: leftover 0.
  - `check-web-audioworklet-v8-spill.py --self-test` and the gate (Node v22.23.2).
- **`sdk` job checks (source-bundle mode, no stray `sdk/dist`): PASS** for `check-sdk-generated.sh`,
  `check-sdk-deletions.py`, `check-sdk-types.sh` and `sdk-package.sh check`.
  `check-sdk-headless.sh` fails one test: 362 pass, 1 fail, `a paired builtin filter edit stays whole
  when only one queue slot remains` (`sdk/test/writer-evals.mjs:374`, in "the writer contract --
  paused"). It **fails the same way at `731c8be30^`**: I built the parent's artifact and ran the parent
  tree, and got the same 362/1 with the same test. It is pre-existing and not counted against #1399.
- **`check-abi-layout-v1.py` on the asset: PASS. `--self-test`: PASS** (22 mutations caught).
- **`check-cross-targets.sh`: PASS.**
- **`audit capi`: PASS.** 100000 calls, allocations 0, deallocations 0, locks 0, syscalls 0, render
  errors 0, total violations 0.
- **Not run:**
  - the Playwright `browser` job legs (no browsers on this machine);
  - AArch64 legs;
  - `check-capi-abi.sh` (the C ABI did not change);
  - `test-debug-a`/`-b` beyond the four packages above.
