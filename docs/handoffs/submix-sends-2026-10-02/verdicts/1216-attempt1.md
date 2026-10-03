# #1216 *Mute a route in the session*: Sol verdict, attempt 1

- Reviewed: `git diff 6701cd55e f48fe7c74` (73 files, +1653/-381). Branch `codex/batch-submix-k3`,
  worktree `/home/bl/misofm/wt-submix-k3`.
- Binding: `AGENTS.md`, `.github/ISSUE_SPECS/1216-mute-a-route-in-the-session.md` with its Attempt 1
  record, `/home/bl/misofm/submix-verdicts/1215-attempt1.md` (MINOR-2), and DESIGN P4 and D9 plus
  the signed-zero rules in `docs/handoffs/silence-2026-09-27/`.
- How I ran it:
  - I did not modify the worktree, the branch or GitHub. `gh issue view 1216`: OPEN, and the title
    matches the spec's H1.
  - I exported head `f48fe7c74` and parent `6701cd55e` with `git archive` under
    `/tmp/claude-1002/v1216/`, each with its own `CARGO_TARGET_DIR`. I ran the mutations in a third
    copy, so the gate tree stayed pristine.
  - Mutations were scripted and reverted with `git checkout` in that copy. I deleted all scratch
    afterwards.

## Verdict: PASS

**No BLOCKER and no MAJOR.**

- The field, wire, opcode, SDK, migration, sealed row and fold decline all match D1-D6.
- No unmuted bit moved against the parent.
- I judge all three recorded deviations legitimate.
- The signed-zero question is resolved in the slice's favour: a muted route through `post_pan`
  renders exactly as the same route open with `[+0.0; 4]`.
- #1215 MINOR-2 is now defended: "the runtime ignores the gate" is red under gate 1 and gate 4.

Five NITs and three informational notes follow.

## Gates (head `f48fe7c74`, x86-64-v3, all rc 0)

| Gate | Result |
|---|---|
| 9: test-debug-a (exact DESIGN 7 command, `--no-fail-fast`) | exit 0; 105 `test result` lines, **1181 passed, 0 failed, 9 ignored**, as recorded |
| 9: test-debug-b, then `conformance_fixtures -- --check` | exit 0, **787 passed, 0 failed, 24 ignored**; then exit 0 |
| 8: `cargo build --locked --release -p audit -p bench -p capi -p session-validator` | ok |
| 8: `check-graph-determinism.sh` | PASS (100/100). `fresh-process-determinism.json` is **`cmp`-identical** to the one I generated at parent `6701cd55e` |
| 8: `graph_fixture -- --check` | exit 0; no graph fixture moved |
| 8: `check-console-fixtures.sh target/release/session_validator` | ok |
| 8: `check-builtins-fixtures.sh . target/release/audit` | ok (50 files) |
| 8: `cargo test --locked --release -p audit -p bench -p console-workload` | 110 passed, 0 failed, 2 ignored |
| 8: `audit capi` and `audit protocol`, head against parent | **`cmp`-identical** |
| 8: `audit graph` and `audit builtins-graph`, head against parent | Differ only in `output_address` (a stack address) and in `accepted_manifest_sha256` (the listed re-pin). `accepted_graph_pcm_sha256` and `accepted_graph_meters_sha256` are unchanged |
| 8: every `output_sha256` | none in the diff |
| 9: `check-protocol-wasm-parity.sh` | ok (simd128) |
| 9: `build-web-audioworklet.sh --named-twin B A`, then `check-sdk-types.sh` | ok |
| 9: `check-sdk-headless.sh A` | 349 pass, 0 fail (the three new #1216 evals ran) |
| 9: `sdk-package.sh check A` | ok (16 enginectl tests) |
| 9: `check-browser-expected-resources.py --artifacts A` and `--self-test` | ok (32 red mutations) |
| 9: `cargo fmt --all -- --check` | clean |
| 9: `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | clean |
| 9: the seven check/test policy pairs | all ok |
| 10: `run-aarch64-tests.sh debug` | **not run**: x86 host. It goes to CI `aarch64-debug` at the K3 push |

## Contract checks

### Field, wire, opcode and canonical order

- **Session.**
  - `Route.mute: bool`.
  - `parse_route` lists the key last and requires it.
  - `key_module!(route ... MUTE="mute":6)`.
  - The walk is `[6]`, with `v.bool(MUTE)` after `v.f32(GAIN_DB)`.
- **Wire.**
  - `route::MUTE = FieldSpec::req(6, Wire::Bool)` is in the route spec.
  - `set_route_mute` has `ROUTE_ID` 1 and `MUTE` 2.
  - `SetRouteMute = 0x0506` is in the enum, `from_raw`, the variant, the opcode map, the apply arm,
    encode and decode.
- **SDK.** The writer's key order ends `gain_db, mute`, and normalisation writes `spec.mute ?? false`.
- **No reuse.**
  - Every historical `schema.rs` route module in this repo's history declares fields 1-5 only.
  - `0x0506` first appears in `f48fe7c74` (`git log -S`).
  - The registry's retired codes are `0006`, `0102` and `0104`, and none is touched.

### The re-pins are legitimate

- **C ABI vector.**
  - Each route gains `,\n      "mute": false`, which is 21 bytes. The nine-track fixture has 9
    routes, so the snapshot grows by 189 bytes: 13,729 + 189 = 13,918 = `0x365e`.
  - The vector's LE length moved from `a135` to `5e36`, and nothing else in it changed.
- **`COMPLETE_SCHEMA_HASH`.** It is spelled identically at its four sites. The conformance tests
  and the Wasm parity script pass, and mutation M1 below shows the pin discriminates.
- **Pin chain.**
  - `sha256(canonical.json)` = `0be977ee…`, which equals both `.toml` rows.
  - The `.toml` digests equal their `MANIFEST.tsv` rows, and `sha256(MANIFEST.tsv)` = `6d466903…`,
    which equals both Rust sites.
  - `session.json` is 1926 bytes, matching `sessionDocumentBytes`, and the self-test row is 1927.

### Migration is complete

- **Every changed JSON document.** A script parsed each one at parent and head, including the routes
  embedded as strings in the writer corpus. There are 21 such documents: 20 plus the writer corpus,
  all excluding the schema document and `expected.json`.
  - Every route object at head ends `…, gain_db, mute` with `mute: false`.
  - Removing that key gives the parent document exactly.
  - Line-level, the only changes are 304 × (`"gain_db": 0.0` → `"gain_db": 0.0,`) and 304 ×
    `+"mute": false`.
- **Sweep of every route-bearing file.** I checked every tracked file that mentions
  `channel_matrix`, `output_input`, `submix_input` or `gain_db`.
  - Each remaining file without `mute` is one of two kinds. Some are SDK builder calls, where
    `mute` is optional and defaults to `false`. The rest are path-only diagnostics tests
    (`diagnostic_parity.rs`, `strict_unknowns.rs`) or unrelated EQ/gate `gain_db` fields.
  - No session JSON lives outside the workspace: `fuzz/` has no route literal, and its seeds carry
    no route.
- **Production readers.** Only `graph-compiler/src/compile.rs` reads a route's gain or matrix. There
  is no incremental or diff path that could miss `mute`.

### A muted route mixes `[+0.0; 4]` and declines the fold

- **One coefficient function.**
  - `node_kind` binds `gated_route_coefficients(&transform, gate)` with the session's gate.
  - `plain_route_gains` returns `None` unless `gate == RouteGate::OPEN`, which is the only change
    in `runtime.rs`.
  - The lowering stores `RouteGate { mute: route.mute, follow_zeroed: [false; 2] }` and
    domain-checks through `route_coefficients` with that gate.
- **No second path.**
  - The only other `PreparedRoute` constructors are in `cfg(test)` or test-support code:
    `graph/src/lib.rs` after `:2759`, `builtins-compiler/src/lib.rs:6194`, and
    `graph-compiler/tests/scale.rs`.
  - Routes are never alias candidates (`program.rs:191`), so an identity route cannot be elided
    into a pass-through.
- **D5.** The `route-mute\t<node>` row is written only when `gate.mute`, right after that route's
  `route-transform`. An unmuted text is byte-identical, which the `graph_fixture --check` and
  determinism `cmp` confirm.

### The signed-zero question (the caller's "is post_pan hiding a defect?")

**No defect is hidden.** A muted route through `post_pan` still mixes `[+0.0; 4]`.

- **The scratch test.** I wrote a differential test in host-core, not committed, using the
  `route_mute.rs` harness. It renders a session with random routes muted against its twin, where
  each muted route is instead **open at 0 dB with `channel_matrix` `[+0.0; 4]`**: the D3
  coefficients through the normal open path.
  - Each seed has 2-7 tracks. Each source lane is about one third `±0.0` and the rest noise.
  - Taps are random across input, post_input, pre_fader, post_fader and post_pan.
  - Track and bus strips get random 2x2 matrices with signed and zero entries. Route gains and
    matrices are random, and so are direct sends.
  - The bus output tap is random, and the bus-to-output route is sometimes muted.
- **Results.**
  - 48 seeds with random taps, 152 muted routes: **bit-identical**. The open twin folded 2 lanes.
  - 48 seeds with **every tap `post_pan`**, 152 muted routes: **bit-identical**. The open twin
    folded 44 lanes, so the fold's class-A equality also holds against an open zero-coefficient
    lane.
- **What D3 means for signed zeros.** Against an *absent* route, a muted route can turn a `-0.0`
  sum into `+0.0`, because `+0.0 + -0.0 = +0.0`. This is D3's frozen behaviour and the spec's
  Hazard. It is also P4's own rule that the first input owns the store (`+0.0` when inactive), and
  the silence handoff's: `-0.0` is the additive identity, `+0.0` is not.
- **Deviation 3 is a test-design choice, not an engine limit.** The implementer saw that a
  gain-path mute stayed green through `post_pan` under their scalar oracle. The cause is that the
  strip's pan maps `(-0.0, +x)` to `+0.0`, which hides the open contributor's `-0.0`. My engine-vs-engine
  differential, with **only** `post_pan` taps, is red on that gain-path mutation (M7 below). So the
  engine is not masking anything. The implementer simply chose input taps to keep a cheap scalar
  oracle discriminating, and that choice is sound. The column-shared-sign matrix construction
  makes the muted word identical on both lanes, so the sign survives the unity `b-main` route.

### Deviations

1. **The C ABI vector re-pin: legitimate.** The arithmetic was checked above. It is the pin the
   spec missed, and the change is just the length bytes and one comment.
2. **Gate-4 topology through the bus's `pre_fader` tap: legitimate, and I re-measured it.**
   - With a `post_pan` bus output: open folds 8, muted folds **1**, which is the output's single
     contributor folding once the bus declines.
   - With `pre_fader`: open 8, muted 0.
   - So the spec's premise ("the bus is the plan's only fold candidate") is false for `post_pan`
     and true for `pre_fader`.
3. **Gate-1 input taps: legitimate.** See above.

## #1215 MINOR-2: the runtime ignores the gate (done, as asked)

| # | Mutation (in `graph/src/runtime.rs`) | Result |
|---|---|---|
| X1a | `node_kind` binds `RouteGate::OPEN` | **RED**: gate 1 (`seed 0, r0 muted, plane 0: sample 0: 0.38474104 != 0.0`). Gates 3 and 4 green |
| X1b | `plain_route_gains` ignores the gate (no decline, open coefficients) | **RED**: gate 4 (`folds == 0` fails). Gates 1 and 3 green |
| X1 | both | **RED**: gates 1 and 4 |

The proxy that #1215's gate 2 left open is now closed by #1216's render gates. The gate-2 extension
still compares against `gated_route_coefficients(&prepared…)` and is not that defence, as #1215
predicted.

## My mutations (each applied, run, and reverted)

| # | Mutation | Result |
|---|---|---|
| M1 | `SetRouteMute` encode `!mute` **and** decode `!parse_bool` (symmetric, so a round trip survives) | **RED**: `conformance_corpus::frozen_corpus_bytes_and_typed_decoders_are_unchanged`. All 129 protocol unit tests stay green, as expected, so the hash pin is the unique catch for a value-inverting codec |
| M2 | Lowering skips the overflow check for muted routes (`if gate.mute { Ok([0.0; 4]) }`) | **RED**: gate 2 at `route_coefficients.rs:226` ("mute true"). A unique catch |
| M3 | Canonical walk writes `mute` before `gain_db` | **RED**: `canonical_writer_corpus_is_rust_generated_and_current` |
| M4 | Wire route field 6 `FieldSpec::opt` | **RED**: 7 protocol tests and 2 conformance tests |
| M5 | Mute stored as `follow_zeroed: [route.mute; 2]` (same bits, wrong gate) | **RED**: gate 2 and gate 5. Render gates green, as expected (same constants) |
| M6 | `enginectl` maps `mute: Boolean(raw.mute)` | **RED**: enginectl test 13 |
| M7 | Mute through the gain (open gate, transform at -1000 dB) | **RED**: gate 1 (`sample 12: -0.0 != 0.0`, as recorded), gate 4 (fold not declined), and my `post_pan`-only differential |

## Test value (one sentence each)

- `route_mute_is_a_required_boolean`: red if `mute` becomes optional or non-boolean. No fixture can
  catch that, because every fixture now spells the key.
- `set_route_mute_switches_the_route_in_the_committed_snapshot`: red if the `0506` apply arm writes
  the wrong route or field, or ignores the value. The codec round trip never applies an edit.
- The gate-2 extension: red if the lowering stores a gate other than the session's (M5), or exempts
  a muted route from the domain check (M2). M2 is a unique catch.
- `a_muted_route_seals_one_route_mute_row_after_its_transform`: red if the gate is missing from, or
  misplaced in, the sealed text (M5, and the recorded no-row mutation).
- `a_muted_route_mixes_zero_coefficients`: red if the runtime binds open coefficients for a muted
  route (X1a, unique), or mutes by gain (M7).
- `muting_the_delayed_route_moves_no_delay_or_latency`: red if a muted delayed route loses its
  compensation delay or its node. Its distinct value is the PDC claim that #1217's activity rule
  must not break. See NIT 3.
- `a_bus_with_a_muted_contributor_folds_no_lane`: red if the fold planner folds a gated route
  (X1b, unique).
- SDK builder, console and enginectl evals: red if the SDK omits, misplaces or mistypes the key
  (M6, plus the recorded mutations).

**PASS.**

## Findings (severity-ranked)

No BLOCKER, MAJOR or MINOR.

### NIT 1: the #1215 MINOR-1 doc amendment has not reached #1216's D2 copy

- **The problem.** `#1216` D2 still says `route_coefficients` "returns `Domain` if `route_transform`
  refuses **or** any folded coefficient is not finite". The implementation and the new test pin the
  *open*-fold reading: a muted 700 dB × 1e10 route is refused (`route_coefficients.rs` "mute true"
  loop).
- **Fix:** amend the D2 bullet to "any coefficient of the **open** fold
  (`gated_route_coefficients(&transform, RouteGate::OPEN)`) is not finite, so domain validity never
  depends on the gate". This is #1215 MINOR-1's text, and it belongs in the root's verdict
  follow-up commit.

### NIT 2: #1215 NIT 1's stale number survives next to an edited hunk

- **The problem.** `crates/graph-compiler/tests/route_coefficients.rs` still says "a unit matrix
  folds to a finite 3.2e34". The value is `1.0e35`.
- **Fix:** change it to "folds to a finite 1.0e35".

### NIT 3: two test-value sentences in the record overclaim

- **Gate 1** says it is red "if ... the codec drops `mute`". The test goes through the JSON
  parser and writer (`canonical_session_json` → `compile_host_session`), not the BTLV codec. The
  codec drop is the protocol round trip's catch, which the record lists separately.
- **Gate 3's** recorded mutation (the lowering skips a muted route) is also red under gates 1 and 5.
- **Fix:** in the record, reword gate 1's sentence to "the session grammar or compiler drops
  `mute`". For gate 3, name its distinct defence: "a muted *delayed* route keeps its PDC line, which
  the activity rule (#1217) must not drop".

### NIT 4: enginectl delegates the boolean check to the builder

- **The problem.** `session-request.ts` passes `raw.mute as boolean`, so a bad value is refused at
  `route("to-bus").mute` and not at `$.routes[0].mute`. This matches the existing `bypass`
  precedent, and the test pins it.
- **Fix:** none needed. If enginectl paths are ever unified, do `bypass` and `mute` together.

### NIT 5: #1215 NIT 2 (double `route_transform` derivation) persists

- **The problem.** `compile.rs` still calls `route_coefficients` (which derives the transform) and
  then `route_transform` again. The result is bit-identical.
- **Fix:** optional, as in #1215.

### Informational

- **For #1217 and P4: delayed muted routes and the silence latch.** `[+0.0; 4]` mixes to `-0.0`
  exactly when both source words are negative. A delayed muted route, which P4 says keeps mixing
  its zero target, therefore writes `-0.0` words. Under #942's rule ("`-0.0` is not silent"), those
  words keep its destination from qualifying as `+0.0`-silent for the S-series latch. This costs
  performance only, never bits. Whoever owns the latch on a bus input should know it.
- **Non-finite sources.** A probe fed a muted route NaN and +inf. The output stayed finite, both
  with the route muted and with it open. Non-finite source words are neutralised before the route,
  so "contributes silence" is not undermined by `0 × NaN`.
- **4-lane coverage.** `route_mute.rs` renders on `Backend::current()`, so on x86 it is 8-lane
  only. CI `aarch64-debug` at the K3 push is its 4-lane run (gate 10).
