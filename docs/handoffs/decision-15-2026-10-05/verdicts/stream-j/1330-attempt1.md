VERDICT: PASS

# #1330 attempt 1 -- adversarial verification (decision-15 stream J)

Reviewed `3ade8e969..9edd1a6b8` (a9f09e29a code+test+docs+before record; 9edd1a6b8 after record +
Attempt record), exported with `git archive` to `/tmp/claude-1002/v1330/src`, built and tested only
there (own `CARGO_TARGET_DIR`s). The worktree `/home/bl/misofm/wt-d15-j` was not touched.

The diff stays inside the authorized paths: `validate_prepare_request` and its doc only in
`crates/effect-contract/src/lib.rs`, the new `crates/effect-contract/tests/registry.rs`, one
paragraph under "Stable diagnostics" in `docs/EFFECT_CONTRACT_V1.md`, the two step-record dirs, and
the spec's own Attempt record. D1-D5 are implemented as frozen. No signature changed, and there is no
cache, global or lock. Render is not involved: preparation is control-plane only.

## Findings

No BLOCKER. No MAJOR.

### MINOR

1. **Gate 5 attribution sentence is unsupported and wrong in direction**
   (`.github/ISSUE_SPECS/1330-...md:176-186`, Attempt record, Gate 5). The record handles the load
   honestly: one invocation each, both `uncontrolled`, loadavg stated, no retry. The issue is the
   last sentence: "most of this ~30 % is the load difference ... the profile's 8-9 % attribution is
   the better estimate of the change's share." The two committed records cannot support that, and
   the profile behind the 8-9 % is uncommitted scratch. My own same-load paired cross-check
   contradicts it. I prepared both modules (digests reproduce the records exactly: base `a9a51862...`,
   branch `b723ff9f...`) and ran `rebuild-run` once each, back to back, at loadavg 23.2 then 24.8,
   pinned to CPU 31, with `MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1`. Both validators accepted. The
   records stay uncommitted under `/tmp/claude-1002/v1330/clone-{base,branch}/artifacts/steps/`.

   | document | p50 before -> after (mean of 2 rounds) | min boot before -> after |
   |---|---|---|
   | nine_track_eq | 4.193 -> 3.454 ms (-17.6 %) | 2.550 -> 2.123 ms (-16.7 %) |
   | sixty_four_track_console | 39.078 -> 32.506 ms (-16.8 %) | 23.703 -> 19.994 ms (-15.6 %) |
   | sixty_four_track_app_shape | 38.281 -> 30.804 ms (-19.5 %) | 24.077 -> 19.420 ms (-19.3 %) |
   | sixty_four_track_console_sends | 60.920 -> 53.564 ms (-12.1 %) | 38.417 -> 33.316 ms (-13.3 %) |

   This is still an uncontrolled host and one pair, so it is descriptive too. But it puts the change
   at roughly 12-20 %, not 8-9 %, which is about half of the committed ~30 %, not "most of it is
   load".

   **Fix:** label the committed table's last column as "observed (confounded: loadavg 95 -> 39)".
   Say plainly that the two committed records cannot measure the change. Delete the 8-9 % sentence,
   or replace it with the verifier's same-load pair above, marked uncommitted verifier evidence.

   **Gate 5 as written is met:** both runs are uncontrolled, the records say so, the p50 change is
   stated, and there is no threshold. Only the interpretation needs correcting. Re-measuring for the
   committed record is not required, and AGENTS.md forbids a retry.

### NIT

1. `crates/effect-contract/src/lib.rs:2472-2476`: the new `# Errors` list omits
   `effect.parameter.unknown` and `effect.parameter.initial`. Both are raised through
   `validate_initial_values(d, r.initial_values)?` inside this function. **Fix:** add the two codes.
2. Peak Wasm memory for `sixty_four_track_console` moves 5,701,632 -> 5,767,168 B (one 64 KiB page).
   This is deterministic: it shows in the committed before/after records and in my rerun. The
   removed per-instance `validate_descriptor` calls allocated `BTreeSet`/`BTreeMap` scratch, so the
   allocator's layout changes. It is harmless, and it is not a diagnostic or a bit. **Fix:** note it
   in one line of the Attempt record, since the record otherwise reads as "nothing observable
   moved".

## Focus (a): is the precondition actually upheld in release builds?

Yes, and by construction, not only by documentation.

- **The registry cannot hold an unchecked factory.** `NativeEffectRegistry.factories` is private.
  The only way to populate it is `NativeEffectRegistry::new` (`lib.rs:2285-2313`), which runs
  `validate_descriptor` on every factory. `Default` gives an empty registry.
- **Every production preparation goes through a registry.** Every `prepare_native_session_effects`
  caller passes a `&NativeEffectRegistry`: `host-core/src/prepare.rs:1342-1344`,
  `control_provider.rs:686`, and the graph-compiler tests. The same holds for
  `live_delta.rs:424` and `response.rs:534`. The descriptor handed to `expected_prepared_metadata`
  at `effect-compiler/src/prepare.rs:466` is `factory.descriptor()` of the factory taken from the
  registry (`:354-361`).
- **No host bypasses host-core.** `hosts/host-web` and `crates/capi` reference no
  `NativeEffectFactory`. Fuzz targets prepare no effects.
- **Each production factory checks the descriptor it registered.** Its `descriptor()` returns the
  same `&'static` const that its `prepare`/bank binder passes to `expected_prepared_metadata`
  (delay, compressor, gate-expander, multiband, true-peak-limiter, soft-clip, transient-shaper,
  parametric-eq, including `parametric-eq/src/response.rs:103`).
- **Direct factory calls outside a registry only use valid native descriptors.** These are tests,
  `conformance/src/{effect,randomized}.rs`, `tools/audit` and `tools/bench`, which also run in the
  release CI leg `cargo test --release -p audit -p bench -p console-workload`. Every test that builds
  `launch_native_effect_registry()` proves those descriptors valid, and every debug build also
  debug-asserts it. `run_effect_conformance` validates on its own first (`conformance/src/effect.rs:1048`).
- **No test or doc depended on the old prepare-time `effect.descriptor.invalid`.** Nothing outside
  `lib.rs` and the docs named it. `scripts/check-effect-runtime-policy.sh` still finds the code at
  `lib.rs:2293`.
- **Hosts see the same refusal.** They map registry failure to `host.effect.registry`, `LiveRebuild::Structure` or
  `UnknownEffect`, exactly as before, so the D4 sentence is accurate.
- **Release really drops the work.** `[profile.release]` leaves debug-assertions off. The shipped
  module shrinks by 106 B (2,894,202 -> 2,894,096), which shows the call is gone from the delivered
  Wasm.

## Focus (b): test value and mutation

**`registry_refuses_an_invalid_main_descriptor`.** It turns red if someone deletes or weakens
`NativeEffectRegistry::new`'s main-descriptor `validate_descriptor` check. After D1 that check is the
only release-build validation, and no other effect-contract test covers it for a main descriptor:
`response_analysis.rs` covers only the analysis-descriptor branch.

**Reproduced.** `lib.rs:2291` `if validate_descriptor(d).is_err()` -> `if false && validate_descriptor(d).is_err()`;
`cargo test --locked --no-fail-fast -p effect-contract` went RED on exactly this test (panic at
`tests/registry.rs:131`, "a descriptor with contract_major 2 must not enter the registry"), with
every other effect-contract test green, `response_analysis.rs` included. Reverted (byte-identical to
`9edd1a6b8`): GREEN.

The fixture also asserts that its `contract_major: 1` twin validates and registers, so the refusal
is not a fixture defect. No source grep and no digest pin. No test is superseded.

## Focus (c): no bit moved (gate 4)

I built `sdk_render_oracle` in release from `3ade8e969` and from `9edd1a6b8` and confirmed the two
binaries differ. Note that a shared `CARGO_TARGET_DIR` across two `git archive` exports silently
reuses the first build: cargo's unit hashes are workspace-relative and archive mtimes are old. I had
to force the rebuild before comparing. `-- 64` on all four fixtures gives identical digests on base
and branch, equal to the Attempt record's table:

- parametric-eq-nine-track `2938bb78...f1f5c8`
- console-sixty-four-track `24607288...ffce8`
- console-sixty-four-track-app `ac23bd04...8430cf`
- console-sixty-four-track-sends `58c54351...f5883`

## Focus (d): benchmark records

- **Rule compliance holds.** The records comply with AGENTS.md: one invocation each, one warmup
  and two measured rounds, no retry, both `uncontrolled`, the waived ceiling and loadavg stated in
  `measurement_control` and `report.md`, and both validators `accepted`.
- **Gate 5 is met as a descriptive gate.** The interpretation needs the MINOR-1 correction: the
  committed delta is not measurable as the change's effect, and the 8-9 % substitute is
  contradicted by a same-load pair.

## The capi race intermittent (bounded look)

This is not caused by #1330, and not by #1251.

- **#1330 never runs in that test.** `race_session()` (`crates/capi/tests/resource_lifecycle.rs:2372-2386`)
  clears the console pre/post inserts and every track's console and inserts, so no native effect
  is prepared and `validate_prepare_request` never runs. Even where it does run, debug builds still
  execute `validate_descriptor` inside the `debug_assert!`, so the work and timing are unchanged in
  the test profile.
- **#1251 changed no production code.** In `resource_lifecycle.rs` it touched only a doc comment.
  In `capi/src/ffi.rs` and `engine/src/realtime/spsc.rs` its changes sit inside `mod tests`.
- **The failure signature looks like a real defect, not noise.** The right lane is never muted by
  the edit generator (`:2455-2465`). An exact `[0.0, 0.0]` on both lanes against a non-zero fresh
  plan therefore means no source signal reached the output: for example a source-ring/feed handover
  across a `SetSourceContent` structural swap under scheduling stress, or a harness feed race.

  **Recommendation:** the coordinator files a bounded issue with this signature
  ("run 5, raced final block [0,0] vs fresh [-0.1212, 0.0398], loadavg ~98").

## Gates run (all in the export)

| gate | command | result |
|---|---|---|
| 1 | `cargo test --locked -p effect-contract` | pass (incl. `registry_refuses_an_invalid_main_descriptor`) |
| 2a | spec's `test-debug-a` line (`--no-fail-fast`) | pass: 1427 passed, 0 failed, 10 ignored (loadavg ~12-25) |
| 2b | spec's `test-debug-b` line (`--no-fail-fast`) | pass: 812 passed, 0 failed, 23 ignored |
| 3 | `cargo fmt --all -- --check`; `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`; `bash scripts/check-workspace-policy.sh` | all pass |
| 4 | `sdk_render_oracle -- 64` x 4 fixtures, base vs branch | 4/4 equal, matching the record |
| 5 | `prepare` / `rebuild-preflight` / `rebuild-run` once each, base and branch, back to back | modules reproduce the record digests; validators accepted; results under MINOR 1 |
| extra | `cargo test --locked --release -p audit -p bench -p console-workload` (the qualification release leg, where the debug assertion is off) | pass: 113 passed, 0 failed |

During 2a, `capi/tests/resource_lifecycle.rs` passed, including the race test.
