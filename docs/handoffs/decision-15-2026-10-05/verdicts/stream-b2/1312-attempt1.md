PASS

# #1312 attempt 1 -- adversarial verdict

Reviewed `git diff 9fddf7560 3120682d5` (code b472a7421..0976694b4, Attempt record 3120682d5) on an
export at /tmp/claude-1002/v1312/tree, built with its own target dir. Spec with Amendments 1-3 and
D7 "Where the stamp is taken"; root rulings r1312-d7-final and r1432-n3-final; AGENTS.md;
decision 15. No BLOCKER or MAJOR finding.

## What was checked and holds

- **D7, C ABI live / model-only commit** (crates/control-plane/src/control.rs:1532-1624). Order:
  checks 1-4, readback lookup, step 5 `check_prepared_structural` (#1503 bound), the test fault
  hook, then `revision_gate.stamp(revision)` as the last fallible step. A refused stamp drops the
  owners and token and returns `Internal` with nothing written or pushed and no revision used.
  After it: cell writes (infallible `write`), effect pushes and target publications (`unreachable!`
  / `expect` on checked room and preflights), `prepared.commit(...).expect`, owner commits, then
  `revision_gate.publish(revision)` from the local. Debug assert that the stamped revision is the
  committed one. The stamp is taken even when the delta writes no cell, and is always published.
- **D7, rebuild** (control.rs:1236-1275). The candidate's gate is stamped with `r` after
  `reserve_replacement`, the report table and the pending check, before the protocol commit; a
  refusal drops reports, reservation, candidate provider (gate handle and cells) and token and
  returns `Internal`. `r` is published on the candidate's handle before `reservation.commit()`,
  nothing stamped between. A commit failure after the stamp drops the local candidate. No candidate
  is kept or reused on any path.
- **D7, browser** (hosts/host-web/src/lib.rs:4679-4736, 5662-5689). The matrix and fader bands
  skip the room pass and take no `in_flight` credit; `write_strip_cells` runs after the last room
  check, stamps `last_published + 1` (ceiling compare kept in `stamp`), writes, and publishes on
  every exit, so no stamp outlives the batch. A batch with no strip records stamps nothing.
  `ReadyOwnership::plan_gate` is the one handle on the worklet's plan; render snapshots the same
  gate. No production render path uses `LiveSnapshot::ALL` (grep: only tests).
- **D2 counter.** `CellRead::Applied.superseded` (#1432's `s - p - 1`) summed per cell, one
  `LiveValuesSuperseded::add` per drain (load, saturating add, store; render is the only writer).
  Candidates that are never adopted are never read, so never counted. `Both` counts in each cell
  (gate 1 reads 78). Copied into the provider in the service step (control.rs:1053).
- **D4.** Fader before mute, left before right; a pair is formed only from two `Applied` reads with
  equal words; an `Unchanged` read is `None` and is not paired. Every applied value is applied
  before the first refusal is returned; the fused drain runs the matrix drain even when the fader
  drain refuses.
- **D6 / gate 4.** `apply_pending` on every stage type is `drain(LiveSnapshot::ALL)`, the same code
  as `begin_block`.
- **D12.** Drains moved from `process` to `begin_block` for the fader, matrix and fused banks;
  `BankChain::run` calls `begin_block` on every active slot every block, silent blocks included
  (#1504); the scalar test owners drain at the top of `process` under `block.live_snapshot`.
- **Amendment 1.** No `try_push` shim: `FaderCells` / `MatrixCells` expose only infallible `write`.
- **D9 / header.** Header and C_ABI_V1_QUALIFICATION text match D9; the watermark bound paragraph
  ("until issues 1502, 1503, 1504, 1312 and 1345 have all landed") is unchanged, as root ruled.
- **D8.** `CounterId::LiveValuesSuperseded = 16`, parse arm, registry text, provider slot count 4.
- **Acked-batch question.** No: every fallible check precedes the stamp and the first cell write;
  a write cannot fail and never overwrites the newest published or pinned value (#1432); a value
  above a block's snapshot stays pending. On the browser, a post-stamp invariant break can only
  refuse a batch whose cells are already published (the reverse direction; see m2), never ack a drop.

## Out-of-list edits and re-pins (each judged)

1. Count as a trailing parameter of `prepare_host_runtime_with_live_lanes{,_successor}`: accepted;
   inside prepare.rs's "counter plumbing" authorisation, avoids 22 test struct literals.
2. `test-support` chaining (control-plane -> host-core -> builtins-compiler): accepted. Product
   builds are clean: `check-scalar-oracle-absent.py --native target/release/libcapi.so` and
   `--wasm` on the named twin both pass.
3. `fixtures/graph/v1/direct-route.resources.json` `builtin_bank_bytes` 6599 -> 6663 (+64) and its
   manifest line: arithmetic checks at AVX2 width 8: `Option<FaderReader>` 56 vs 40 (+16 x 8),
   `Option<MatrixReader>` 32 vs 40 (-8 x 8) = +64; the two plan totals move by the same +64; no
   other field moved. Justified.
4. `STRIP_PREPARATION_BYTES` 1080 -> 1088 and `resources.jsonl` + manifest + joined-corpus pin:
   +8 = (56 + 32) - (40 + 40) in `StripControlConsumers`; every processor-payload and largest-
   allocation field moves by exactly +8 x tracks (8, 32, 524296); allocation counts unchanged.
   Justified (but see n1 on the comment).
5. `direct-oracle.mjs` flood on bypass records: forced (matrix no longer overfills);
   `check-browser-expected-resources.py --artifacts` agrees with `expected.json` unchanged.
6. COUNTERS_GET pinned response vector (index 9): a wire-format corpus; +1 `CounterValue`
   (id 16, value 0), length 0x70 -> 0xa0, count 3 -> 4. Justified; mutation M8 shows the pin is
   load-bearing.

## Findings

### MINOR

- **m1. No test covers matrix/pan counting** (crates/builtins-compiler/src/lib.rs `MatrixReader::drain`,
  `self.superseded.add(superseded)`). Mutation M6 (drop that line) survives BC, HC, CP, CAPI and
  WEB. Gate 1 and the drain unit test count fader cells only. The code is correct by inspection,
  but D2's exactness is unasserted for one of the two lanes this slice moves. Add a matrix half to
  `a_drain_applies_the_newest_value_its_snapshot_covers_and_counts_the_rest` (or gate 1) in a
  follow-up or the next attempt of a successor (#1399 is the natural place).
- **m2. Browser steps after the stamp are fallible returns, not `expect`s**
  (hosts/host-web/src/lib.rs:4709-4735 and the push/owner loops at 5690-5860). After
  `write_strip_cells` publishes, a failed push, `publish_candidate_targets`, `commit_owner` or
  `shadow.commit` returns `RESULT_INTERNAL` while the batch's cell values are already published,
  and `admit_commands` rolls back the solo/VCA/route mirrors: a refused-but-applied value on an
  invariant break. Each path is a checked invariant (room pass, EQ preflight, companion
  validation), it is pre-existing style for the queues, and the implementer documents it; D7's text
  ("each an `expect` on a checked invariant") is not met literally. Acceptable before #1382, which
  moves admission to the Worker's control plane.
- **m3. Browser count is per plan, not per session, and uncharged** (crates/host-core/src/prepare.rs:1187-1189).
  Every entry except the two `..._with_live_lanes` ones makes a fresh `Arc<AtomicU64>` per plan,
  created outside the builtins observation window and charged by no resource row (the C ABI charges
  its session count in `capi_resources`). D2 says one count per session passed to every plan. Not
  observable until #1399 exposes it; #1399 must pass a session count from host-web and charge it.

### NIT

- **n1.** tools/audit/src/fixture_builtins.rs re-pin comment: "The largest allocation and the
  allocation counts do not move" is false for `maximum_single_allocation_bytes` (1080 -> 1088,
  4320 -> 4352, 70779960 -> 71304256). The +8/track reason explains it; the sentence should say so.
- **n2.** Rebuild-path stamp is unobservable today: mutation M11 (delete the candidate
  `stamp(revision)` check) survives CP and CAPI. It has no effect until #1277 writes retargets;
  #1277's added gate (refused candidate never reused) is where it becomes defended.
- **n3.** Stale comment in `resource_plan` (builtins-compiler, the per-strip loop): "the only
  live-control-dependent rows left are the two or three bounded rings charged below".
- **n4.** `apply_pending` is inherent and `dead_code`-allowed; #1277 will need it through the
  bank trait object (downcast via `as_any` works, a trait method would be cleaner).
- **n5.** Gate 9's stamp-before-check mutation evidence is blunt: a process-global counter makes 80
  host-web lib tests and `render_locked_source` trip the same `GatePin` assert, so it shows the
  assert fires, not that gate 9 alone catches it. The write-before-check mutation (M9) is the
  discriminating one and is red in gate 9 plus 7 older refusal tests.

## Test value (one sentence per new test)

- Gate 1 `forty_fader_edits_before_one_block_are_all_accepted_and_the_last_is_heard` (capi): red if
  a fader lane is still a bounded queue, the per-cell gap is counted for one cell only (M1: 39 vs
  78), or the service step stops copying the count (M8).
- Gate 2 `a_fault_before_the_live_write_leaves_every_cell_and_the_output_alone` (capi): red if the
  strip cell writes move above step 5 (M7: "no cell took a write"); no other CP/CAPI test went red.
- Gate 4 `apply_pending_equals_the_block_drain` (builtins-compiler): red if `apply_pending` skips a
  stage or reads under another snapshot than the block drain (M4); unique.
- Gate 8 `one_transaction_is_applied_whole_in_the_block_its_revision_is_published_for` (host-core):
  red if the drains apply a value above the block's snapshot (M2: first block applies the
  unpublished transaction); the only other red test is the new drain unit test.
- Gate 9 `a_refused_browser_batch_writes_no_cell_and_leaves_the_gate_alone` (host-web): red if the
  browser writes a cell before its last check (M9: "the refused batch wrote no cell"); the only
  browser test that also checks the gate's published revision and that the refused edit is never
  heard after a later batch publishes.
- `a_drain_applies_the_newest_value_its_snapshot_covers_and_counts_the_rest` (builtins-compiler):
  red if a drain ignores `S` (M2), counts in another unit (M1) or stops pairing equal words (M5).
- `the_drain_applies_the_fader_before_the_mute_whatever_the_write_order` (builtins-compiler): red if
  mute is drained before fader (M3), a fader move cutting a mute's ramp short; unique.
- `composite_drain_applies_the_other_stages_value_when_one_stage_is_refused` (rewritten): red if a
  refused fader value stops the matrix drain (implementer's mutation; not re-run here).

## Mutations re-run (separate export, one at a time, restored with cmp)

| # | Defect | Result |
|---|---|---|
| M1 | fader reader counts cell 0 only | red: BC drain unit test, CAPI gate 1 |
| M2 | fader and matrix drains read `ALL` | red: HC gate 8, BC drain unit test |
| M3 | mute before fader | red: BC order test only |
| M4 | fused `apply_pending` skips matrix | red: BC gate 4 only |
| M5 | equal words never paired | red: 8 BC tests, 1 WEB test |
| M6 | matrix replacements not counted | **survives all five suites** (m1) |
| M7 | stamp + cell writes above step 5 | red: CAPI gate 2 only |
| M8 | count not copied into the provider | red: CAPI gate 1, COUNTERS_GET pin |
| M9 | browser cell writes before room pass | red: gate 9 + 7 older refusal tests |
| M10 | browser stamp before checks (global counter) | GatePin assert; 80 WEB lib tests (n5) |
| M11 | rebuild candidate stamp deleted | **survives CP and CAPI** (n2) |

Baseline (unmutated) BC, HC, CP, CAPI, WEB all green. Logs: /tmp/claude-1002/v1312/mut/logs/.

## Gates run (export of 3120682d5)

- `cargo test --locked --workspace --no-fail-fast`: exit 0, 343 result lines, 0 failed.
- `cargo test --locked` for `-p control-plane --features test-support`, `-p builtins-compiler
  --features test-support`, `-p host-web --features test-support`, `-p host-core --features
  control-provider,test-support`, `-p protocol --features test-support`, `-p capi`: all exit 0.
- `cargo build --locked --release -p audit -p capi && target/release/audit capi`: allocations,
  deallocations, locks, syscalls, file/network I/O, panics 0; `total_violations` 0.
- `check-capi-abi.sh`, `check-realtime-policy.sh`, `test-realtime-policy.sh`,
  `check-workspace-policy.sh`, `check-builtins-policy.sh`, `check-host-core-policy.sh`,
  `check-protocol-control-policy.sh`, `check-graph-policy.sh`: ok.
- `check-rack-policy.sh`: FAIL at crates/rack/src/lib.rs:6191 (`use std::sync::{Arc, Mutex}` in a
  #1504 test module). #1312 does not touch crates/rack; known, being fixed separately; not counted.
- `cargo fmt --all -- --check`; `cargo clippy --locked --workspace --all-targets --all-features
  -- -D warnings`: ok.
- `check-cross-targets.sh`: PASS.
- Worklet chain: `build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh
  --without-metadata-regeneration` (call graph included), `check-browser-expected-resources.py
  --artifacts`, `check-scalar-oracle-absent.py --wasm` and `--native libcapi.so`,
  `test-web-audioworklet.sh`: all pass.
- Not run: the Playwright `browser` job legs (no browsers on this box; the implementer did not run
  them either). The root should run them at the batch boundary.
