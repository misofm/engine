PASS

# #1365 attempt 1: verdict

Edit `control_smoothing` with a session transaction, model-only. Stream E2 of decision 15.
Range `bd8a92e9d..b16fb4d14` (`9db950add` is the change, `b16fb4d14` is the Attempt record).
Fresh adversarial verifier. I built a `git archive` export of `b16fb4d14` in `/tmp/claude-1002/v1365` with my own target directories. I did not touch the worktree.

There are no BLOCKER or MAJOR findings.

## What I checked

- **D1, the opcode.** `SetControlSmoothing = 0x0008` is in the root group (`crates/protocol/src/model.rs:37`). It is in `from_raw` (`:139`) and in the opcode map. `SessionEdit::SetControlSmoothing { control_smoothing: Option<ControlSmoothing> }` exists. `0x0006` stays retired: the registry test asserts 47 codes and that `0x0008` round-trips.
  - The registry row in `docs/CONTROL_PROTOCOL_REGISTRY.md` gives field `1`, an optional message, with "absent clears". The nested registry gives `1:mute_ms,2:fader_ms,3:pan_ms`, three required `F32` fields.
- **D2, the wire.**
  - Field 1 is `FieldSpec::msg(1, false, false, ..)`, the same form as the `BACKPRESSURE` precedent. The nested spec has three required `F32` fields (`schema.rs:1184-1203`).
  - The encoder writes field 1 only for `Some` and counts it as 0 or 1 (`session_wire.rs:351-353`, `:376-382`). The decoder reads field 1 with `optional_one` and so keeps `None` apart from any message (`:1268-1273`).
  - A partial message is refused by the schema's required-field check.
  - `parse_control_smoothing` (`:1778`) refuses a non-finite value through `read_f32_exact` and an out-of-range value through the `(0.0..=1000.0)` check. Both refusals are `DecodeError::InvalidTlv`, the code that `read_f32_exact` already uses, and it maps to `MALFORMED_FRAME`.
  - **Bounds against #1054 D1.** `-0.0` passes, because `-0.0 >= 0.0`. The session grammar also accepts `-0.0` and keeps its sign (`session/src/canonical.rs:443`), and the gate-1 re-encode byte check proves the decoder keeps the sign too. `1000` passes and `1000.5` is refused, which matches `parse.rs` `bounded(.., 0.0, 1000.0)` and `validate.rs:76-88`.
  - A Rust caller that skips decode is still refused at prospective-session validation (`validate.rs:83`).
- **D3, apply.** `session.control_smoothing = *control_smoothing` (`model.rs:771-773`). `ControlSmoothing` is `Copy`, so this is correct; the spec's `.clone()` would fail clippy. `record_edit_ramps` lists the edit as a no-op. `ramp_samples()` returns `None` for it.
- **D4, model-only.** The mask is `masked.control_smoothing = current.control_smoothing` (`host-core/src/live_delta.rs:229`), and the doc rule (`:166-176`) has the hazard sentence. `LiveRamps::for_session` has only one reader in the tree, `control-plane/src/control.rs:1351`, and it is evaluated on `next`. No prepared plan reads `control_smoothing`: a grep of every `control_smoothing_samples` and `for_session` user found none.
- **Model-only claim (C ABI).** `model_only_edits_commit_without_a_plan_rebuild` now includes this edit. It asserts:
  - one revision, the same provider epoch and no pending candidate;
  - one `SESSION_COMMITTED`;
  - a canonical snapshot equal to the model with the edit applied;
  - three blocks bit-identical to an uninterrupted reference;
  - `generation == 1`, so no seek, which means no source-ring reset.

  The transaction goes through the real BTLV wire (`rig.transaction`).
- **Same-transaction claim.** `commit_live` builds `LiveRamps::for_session(next)` at `control.rs:1351`. Gate 4's "in the mute's transaction" row shows this end to end: block 0 after the commit equals the control booted muted. Under M4 that row goes red.
- **A ramp already in flight keeps its length (non-goal).**
  - A control_smoothing-only delta has `strips` and `effects` both empty, which the host-core test asserts.
  - `commit_live` writes cells only for `delta.strips` and pushes only `delta.effects` (`control.rs` around `:1574-1600`).
  - Nothing else carries a session-level ramp length into render: every `TrackFaderRecord` carries its own `smoothing_samples`.

  So the edit writes nothing that could retarget a ramp in flight. A later edit's record on the same lane is a new transition with the new length, which is what the spec intends.
- **Acked-batch question.** Can an ack ever precede a drop? No. The edit pushes nothing to any queue or cell. The ack means the committed model holds the value (D5). When the edit rides with live edits, `commit_live`'s existing order applies: everything fallible runs before the first write, and cells have no room to want.
- **The re-pin (D6).** `COMPLETE_SCHEMA_HASH` moves once, `5429f052592101ef` -> `8eb4cc8d075a4355`.
  - **Mirrors.** All of these agree: `protocol_corpus.rs:764` and its history line, `scripts/check-protocol-wasm-parity.sh:172-173`, `docs/CONTROL_PROTOCOL_CONFORMANCE.md:3` (hash, 47 edits, re-pin list), `fuzz/corpus/complete-schema-manifest.md:10,34`, and the registry's "exactly 47". The controller test asserts 47 with limit 46, and `model.rs`'s registry test asserts 47. No stale spelling of the old value remains outside history lines and specs.
  - **Whole move.** I deleted only the corpus's `SetControlSmoothing` edit from the new tree. `conformance_corpus` then computed `6064642609704403439` = `0x5429f052592101ef`, the old pin exactly. So that one corpus edit is the whole move. The frame count stays 46, and the guest's `corpus.len() == 46` is unchanged.
- **Implementer's gate-5 deviation: accepted.** `classify_live_delta` takes its ramps as an argument. A host-core test therefore cannot see a caller that builds them from the pre-commit model. The test feeds `LiveRamps::for_session(next)` and proves that the classifier forwards it. The "one transaction late" defect is caught uniquely by gate 4 through the C ABI (M4 below). The Attempt record states this candidly.
- **Naming and roles.** There are no new version suffixes. The diff has no producer, fan or listener role wording and does not rely on decision 16.

## Mutation runs (my own)

I ran each mutation in a separate export and target. Each run covered all of `-p protocol --features test-support`, `-p host-core --features test-support,control-provider`, `-p capi`, `-p control-plane --features test-support` and `-p conformance`. The unmutated baseline passed: 589 tests, 0 failed.

| # | Defect | Red (and nothing else) |
| --- | --- | --- |
| M3 | delete `masked.control_smoothing = current.control_smoothing` | capi `model_only_edits_commit_without_a_plan_rebuild` (new row), `a_control_smoothing_edit_makes_the_next_mute_a_step`, `a_control_smoothing_edit_gives_the_next_unmute_its_ramp`; host-core `a_control_smoothing_edit_is_live_and_its_lengths_govern_the_same_transactions_records`, `model_only_edits_are_live_with_no_records` (new row). 5 failed, 584 passed |
| M4 | `commit_live` uses `LiveRamps::for_session(<pre-commit model>)` | only the two new gate-4 capi tests (block 0 after the commit is not the muted control). 2 failed, 587 passed |
| D1 | `parse_control_smoothing` range check -> `if true` | only `set_control_smoothing_round_trips_and_decode_refuses_a_partial_or_out_of_range_message` (`-0.001 in field 1` accepted). 1 failed |
| D2 | range check -> `value >= 0.0` (upper bound lost) | only the same gate-1 test. 1 failed |

Each catch is unique. In every row, the only red tests are new tests or rows this slice adds; no test that existed before catches any of these defects.

## Test value, one sentence per new test

- **Gate 1**, `set_control_smoothing_round_trips_and_decode_refuses_a_partial_or_out_of_range_message`: red if the decoder accepts a length outside `[0, 1000]` or a partial message and leaves it to validation, or if the encoder and decoder disagree on the optional field or on a length's field ID (D1, D2).
- **Gate 2**, `set_control_smoothing_sets_then_clears_the_key_in_the_committed_snapshot`: red if the apply arm ignores the `None` clear, merges into the old lengths or writes into a neighbouring field, so the committed canonical JSON is not exactly `explicit` then `default`.
- **Gate 3**, the new row in `model_only_edits_commit_without_a_plan_rebuild`: red if the mask misses the key, so the edit rebuilds and resets the source ring (M3).
- **Gate 4**, `a_control_smoothing_edit_makes_the_next_mute_a_step`: red if the commit reads `LiveRamps` from the pre-commit model, so a mute in the same transaction still ramps (M4).
- **Gate 4**, `a_control_smoothing_edit_gives_the_next_unmute_its_ramp`: red if the new length arrives one transaction late, so the unmute steps (M4). It also checks that every ramp block renders with 0 allocations.
- **Gate 5**, `a_control_smoothing_edit_is_live_and_its_lengths_govern_the_same_transactions_records`: red if the classifier treats a set, change or clear of `control_smoothing` as structure (M3). The new `model_only_edits_are_live_with_no_records` row tries only the set.

## Findings

### MINOR

1. **Spec gate 7's host-core command runs zero tests** (`.github/ISSUE_SPECS/1365-...md`, gate 7). `cargo test --locked -p host-core --features test-support --test live_delta` prints `0 passed`, because `crates/host-core/tests/live_delta.rs:7` is `#![cfg(feature = "control-provider")]`.
   - The implementer ran it with `control-provider` (35 passed).
   - In CI, `test-debug-a` turns the feature on through workspace unification: its `live_delta` binary lists 35 tests.
   - So the gate is covered, but the spec's literal command proves nothing. The Attempt record notes the feature without saying that the spec's command is empty.
   - Root should amend the gate command, which is a spec fix. This does not fail the attempt.

### NIT

1. **Unwrapped doc lines after edits.**
   - `crates/capi/src/runtime/live_tests.rs:1984` (139 columns)
   - `crates/host-core/src/live_delta.rs:133` (117 columns)
   - `docs/C_ABI_V1_QUALIFICATION.md:354` (120 columns)

   rustfmt does not wrap comments, so `fmt --check` is green.
2. **Spec Test-value wording for gate 5.** It still claims gate 5 catches "uses the old length in the same transaction". A classifier test cannot catch that defect; gate 4 does (see the accepted deviation above). Root may amend the sentence when the spec closes.

### Observations for root (outside this range, not findings against the attempt)

- **host-web `SessionRamps`** (`hosts/host-web/src/lib.rs:1522-1540`, #1364 on this branch) computes `LiveRamps::for_session` once, from the session the host prepared. The spec's non-goal says that #1309's portable control plane "then serves this edit unchanged". That is true only if #1309 (or #1364) refreshes or drops that cache on a model-only commit. Otherwise a browser `control_smoothing` edit would take effect only at the next rebuild. Worth a line in #1364's or #1309's brief.
- **No SDK encoder for `0x0008`.** The SDK has none yet. This is consistent with the browser non-goal, but the SDK transaction-encoder issues (#1383, #1385) do not list the opcode. Worth naming where it lands.

## Gates run (all green, from the export, own target)

- **The whole `lint` job of `qualification.yml`.**
  - fmt; clippy `--workspace --all-targets --all-features -D warnings`; `cargo doc` with `-D warnings`.
  - Every policy check and its mutation test: workspace, session, env-vocabulary, bench, test-support CI (`check-`/`test-test-support-ci.py`), scalar-oracle self-test, script reachability (green after I `git init` the export, because the script needs `git ls-files`).
  - Console benchmark fixture, builtins fixture mutations, bench preconditions, host-core, protocol-control, realtime (with audit-leak and artifact-evidence-leak), trace validator, lane, unfused seal and self-test.
  - Rack, builtins and graph; effect-runtime policy and fixtures; conformance boundaries; parametric-EQ seal.
  - Release-shape self-test, npm publish modes, stem store V1, stem identity drift.
  - The three x86 sub-v3 and AVX2+FMA probes.
- **The whole `test-release` job.**
  - lane, math and wasm-gates in release; `filter_liveness`, `tail_contract` and `ramp_endpoint` (8 crates); `designer_total --ignored`.
  - The M3 FMA cfg check; the engine SPSC loom leg (13 passed, own target directory); the M1 exhaustive and F1 exhaustive sweeps.
- **The `test-debug-a` and `test-debug-b` legs** with doctests, the builtins-compiler `--no-run` step and `conformance_fixtures --check`.
- **Spec gates.** These were run:
  - `-p protocol --features test-support`, `-p capi`, `-p host-core --test live_delta` (see MINOR 1), `-p conformance`, `conformance_corpus`
  - `check-protocol-wasm-parity.sh` (simd128 golden parity ok)
  - `check-protocol-control-policy.sh`, `check-host-core-policy.sh`, `check-workspace-policy.sh`
  - `run-protocol-allocation-audit.sh` (ok, 64 edits)
  - `audit capi`: `pcm_digest a9738fa800afcab3`, 0 allocations, 0 syscalls, 0 violations, unchanged
  - `check-cross-targets.sh` (PASS, with the known #1018 expected failures), `check-capi-abi.sh` (shared and static)
  - the simd128 `cargo check` of target-smoke, protocol, dsp-reference and conformance
- **The worklet chain.**
  - `build-web-audioworklet.sh --named-twin` produced shipped module `1e2175365b3de67dee196cc73bfdb2e9f037ac67a467505dcffa0af50afc6c93`. The strip-names self-test and check passed.
  - `check-web-audioworklet.sh --without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts` (digests agree; self-test with 32 red mutations) and `check-scalar-oracle-absent.py --wasm` passed.
  - `test-web-audioworklet.sh` passed with a private TMPDIR, which was left empty.
  - I also ran the V8 spill gate (Node 22.23.2 exact), which the implementer could not; it passed.

Not run: AArch64 legs (CI only) and the browser matrix.

Evidence kept: `/tmp/claude-1002/-home-bl-misofm-engine/258223fb-6f51-4ebb-b550-04d18c009589/scratchpad/v1365-evidence/` (logs, mutation diffs, mutation results). The export trees and targets are deleted.
