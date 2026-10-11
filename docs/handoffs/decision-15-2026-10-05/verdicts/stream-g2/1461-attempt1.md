PASS

# #1461 attempt 1 -- adversarial verdict

Commit `f3956e63c` (parent `7e8379523`), branch `codex/d15-stream-g2`, worktree
`/home/bl/misofm/wt-d15-g2`. I reviewed an export at `/tmp/claude-1002/v1461/tree`. I did not
build in, edit or check out the worktree.

Summary: the contract change is correct and complete. D1 is met: `PreparedEffect` and
`PreparedEffectBank` carry the metadata, and `metadata()` is gone from both processor traits. D3 is
met for all eight effects and their bank forms. D4 is met: no bit moved. Every gate in the spec
passes when I re-run it. The two MINOR findings do not fail the attempt. There is no BLOCKER and
no MAJOR.

## BLOCKER

None.

## MAJOR

None.

## MINOR

**m1. Three processors keep prepared scalars that no release-build render path reads. D2 asks
for "exactly" the scalars render reads.**

- `crates/parametric-eq/src/lib.rs:2953` and `:2958`: the generic `PreparedParametricEq<L, W>`
  keeps `quantum` and `width`. Only the bank body reads them (`:3989`). The scalar instantiation
  `PreparedParametricEq<f32, 1>` reads neither. Its `process` gets its chunk length from
  `self.rest[0].len()` (`:3795`). The implementer flagged the scalar `width` (point 1) and
  documented it in code. The scalar `quantum` is also unread, and the Attempt record's EQ row does
  not say so.
- `crates/gate-expander/src/lib.rs:353`: `quantum` is read only by
  `debug_assert!(block.frames <= self.quantum)` in the bank body (`:1001`). Release builds never
  read it, and the scalar form never reads it.
- `crates/true-peak-limiter/src/lib.rs:3426`: the same pattern, read only by the bank
  `debug_assert!` at `:4598`. The scalar `LimiterCore<f32>` never reads it.

None of these is control-only data. No control reader uses them, and the record is gone. So root's
standing ruling (render-owned memory carries no control-only data) is not broken. The cost is
nil or a few bytes. But the values are dead weight in render-owned memory, and D2's letter is not
met. Under the no-shortcuts principle, the clean fix is cheap:

- The EQ: derive `BankWidth` from the const `W` (4 or 8), and use the rest-plane length for the
  quantum. The scalar `quantum` field then goes, and the bank `width` field can go too.
- The gate and the limiter: drop the `debug_assert`, or keep the field and the assert under
  `#[cfg(debug_assertions)]`.

Root can also accept the current state as it is. In either case, the Attempt record's EQ row
should state that the scalar `quantum` is unread.

**m2. Two kinds of file outside the authorized paths, edited without a named exception.**

1. `crates/capi/tests/resource_lifecycle.rs` is stream F's file. STREAMS says "F owns
   `crates/capi` tests" (STREAMS.md:286, also :420). This attempt raised the
   `effect_bank_metadata_bytes` ceilings there. Neither the Authorized-paths list nor A2 names
   the file. A2 widens the paths to the files that call `metadata()`, and this file does not call
   it.
2. `crates/host-core/src/control_preparation.rs` (its test module) and `tools/audit`/`tools/bench`
   are not named either. Their edits are mechanical `prepare`/bind caller migrations that D1
   forces. A1 names `tools/console-workload/tests/paired_spans.rs` but not these files.

Each edit is minimal and correct, and the frozen design forces it. The implementer recorded them
in the STREAMS hot-file note (STREAMS.md:147-151) but got no named exception. Root should ratify
them. Stream F's later slices that touch `resource_lifecycle.rs` rebase onto this one.

**Is the re-pin sound (point 3)? Yes, and the new values are ceilings, not loose pins.** I
measured each step myself:

- `size_of::<graph::GraphPreparedEffectBank>()` is 120 at base and 280 at head (+160).
  `PreparedBankMetadata` is 160 bytes and `PreparedEffectMetadata` is 144 bytes. A size probe
  compiled against both trees gave these values.
- The capi row at base measures `effect_bank_metadata_bytes: 805 of 896`. At head it measures
  `1125 of 1280`.
- The row is `n_banks x S + members + lanes + strings`. From the eight-lane base value, the parts
  that do not depend on the bank count sum to 549. #1304's four-lane 921 then solves to exactly
  `124k + 549` with k = 3 banks, so the derivation's three four-lane banks are consistent with the
  measured eight-lane row. Head four-lane = 3 x 284 + 549 = 1,401.
- Both new ceilings follow the file's own rule, the measured or derived value + 10 %, rounded up
  to 64. The old ceilings follow the same rule: 896 = ceil64(1.1 x 805), 1,024 = ceil64(1.1 x 921).
  The new ones are 1,280 = ceil64(1.1 x 1,125) and 1,600 = ceil64(1.1 x 1,401).
- `PreparedBankMetadata` and `GraphPreparedEffectBank` contain only fixed-size fields, plus
  pointers and `u64`s that have the same size and alignment on aarch64-linux. So the +160 per bank
  holds there too.
- Open item, as the record says: CI's `aarch64-debug` must print the measured four-lane row. A
  wrong derivation can only fail loudly there; it cannot pass silently.

## NIT

- `crates/effect-compiler/src/prepare.rs:1071-1073`: two doc comments are stacked on
  `type Forge`. The first one ("One forgery per field the mismatch check compares...") belongs
  above `const FORGERIES` at `:1076`.
- `crates/conformance/src/effect.rs:155`: the edited doc line is 119 columns, and the crate uses
  `max_width = 100` (rustfmt does not wrap comments). `docs/EFFECT_CONTRACT_V1.md:314` is 112
  columns. That file already had 5 lines over 100.

## Implementer's review points

1. **EQ `width` in the scalar form.** It is dead data, not control-only data. See m1, which adds
   the scalar `quantum` that the record did not flag.
2. **Gate and limiter `quantum` read only by a `debug_assert`.** The field is render-path data in
   debug builds and dead in release builds. See m1.
3. **The capi re-pin.** The spec does not authorize it, and root should ratify it (m2). The
   derivation is sound, and the values are ceilings (see m2).
4. **The deleted tests are truly superseded.**
   - Rack `a_staging_window_larger_than_the_capacity_is_refused_at_bind` with
     `ShrinkingCapacityBank`: the defect needed two `metadata()` reads that disagree, and that is
     now impossible by type. `LiveControlEffectBankStage::new` reads one `PreparedEffectBank`
     value once (`crates/rack/src/lib.rs:1016-1040`). A future mis-sizing of the staging window
     still hits the `check_automation_window` call that remains (`:1075`). That makes every
     live-control bind fail with `RackError::AutomationWindow`, which the remaining live-bank
     tests catch through their `expect` on construction.
   - Conformance `ChangingMetadata`, `ChangingTail`, `metadata.changed` and the reset probe's
     metadata term: the caller holds the metadata value, so it cannot change after preparation.
     The fault classes that remain representable, a prepare result that misstates latency or
     resources, moved into the prepare result. `every_faulty_mock_is_detected` now pins them to
     `metadata.exact`.
   - No defect class lost coverage.
5. **The no-bit-moved spot check.** I wrote a separate throwaway differential (see "Gate 1" below),
   built it against base and head, and got identical results.

## Other observations (no finding)

- `rack::EffectBankStage` gains `bypassed: bool` (`crates/rack/src/lib.rs:756`, set at `:801`).
  Only the response-snapshot query reads it, which the host calls between render calls. `process`
  does not read it.
  - `size_of::<rack::EffectBankStage>()` is 56 at both base and head, so the bool sits in padding.
    This matches #1460's precedent for `ResponseOwnerBinding::prepared_bypass`.
  - The stage does not keep the record.
- `graph::GraphPreparedEffectBank::metadata` is control-side. `RuntimeParts::stage_for` consumes
  it by value when it builds the stage (`crates/graph/src/runtime.rs:4911-4930`), so no render
  structure keeps it.
- The limiter's `self.metadata.bypass` became `self.coefficients.bypass`. Both values come from
  `metadata.bypass` at construction (`crates/true-peak-limiter/src/lib.rs:3501-3511`), and nothing
  changes them afterwards.
- The compressor's `sidechain_connected` is the same `matches!(.., Connected { .. })` test as before.
- The acked-batch question does not apply: this slice adds no queue and changes none.

## Test value (new or rewritten tests)

- `effect-compiler` `metadata_mismatch_tests::a_prepare_result_whose_metadata_differs_in_any_compared_field_is_refused`
  (rewritten from #1377's two-field test): the defect is a prepare-time mismatch check that stops
  comparing any one of the 16 fields, or compares the returned metadata with itself. Once
  processors no longer report metadata, this is the only test in the workspace that references
  `effect.metadata.mismatch`.
  - My mutation runs: dropping each of the 16 comparisons in turn turned the test red, each time
    with a panic that names the dropped field ("a forged `<field>` prepared"). Comparing with
    itself turned it red. After revert it is green.
- `conformance` `every_faulty_mock_is_detected` (amended rows): the defect is a harness that stops
  checking the prepare result's program key against the expected one (`metadata.exact`). The only
  other runs of the harness are on production factories, which all pass that check, so they
  cannot catch it.
  - My mutation run: replacing the `metadata.exact` push (`crates/conformance/src/effect.rs:1106`)
    with a no-op turned the test red with "fault LatencyChangingBypass escaped detection". After
    revert it is green.

## Gates run (from the export, `CARGO_TARGET_DIR=/tmp/claude-1002/v1461/target`)

- fmt `--check`: ok.
- `cargo clippy --locked --workspace --all-targets -- -D warnings`, with and without
  `--all-features`: ok.
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`: ok.
- test-debug-a (exact `qualification.yml` command and features): 1,453 passed, 0 failed,
  10 ignored. Doctests: 22 passed. `builtins-compiler --no-run`: ok.
- test-debug-b: 888 passed, 0 failed, 25 ignored. Doctests: 3 passed.
  `conformance_fixtures --check`: ok.
- Release:
  - `audit`/`bench`/`console-workload` tests: 113 passed, 0 failed, 2 ignored.
  - The `ramp_endpoint` release targets: 16 passed.
- `check-effect-contract.sh` ok (8 production factories, 0 failed gates).
  `check-workspace-policy.sh` ok. `check-realtime-policy.sh` ok (89 regions).
- `check-capi-abi.sh` ok (shared and static), and its `--self-test` ok.
  `check-scalar-oracle-absent.py --native` ok.
- **Gate 4.** `audit capi`: 0 allocations, 0 deallocations, 0 syscalls, `pcm_digest`
  `cb10fbface44a3a4`. Base `7e8379523` built and run the same way gives `cb10fbface44a3a4`, so
  there is no change.
- `run-wasm-gates.sh` (native + wasm simd128 + V8 EQ loops, the G5 digests): ok.
- `check-cross-targets.sh`: PASS, with only the expected `ios-asm-memset-pattern16` failures
  (#1018).
- Worklet chain:
  - `build-web-audioworklet.sh --named-twin`: ok.
  - `check-web-audioworklet.sh`: ok.
  - `check-browser-expected-resources.py --artifacts`: digests agree, and the self-test is ok.
  - `check-scalar-oracle-absent.py --wasm`: ok.
  - `test-web-audioworklet.sh`, with a fresh TMPDIR left empty: ok.
- **Gate 1 (spot differential, mine, throwaway, not committed).**
  - Built against base and head through the eight production factories.
  - Configurations: every declared quality at 44.1 and 96 kHz, every supported link mode, bypass
    on and off, and sidechain none/unconnected/connected. That is 92 configurations.
  - Runs: 273 scalar runs, plus 84 bank runs at every width this x86-64-v3 build binds:
    - Simd8 for the compressor, gate, multiband, EQ, soft-clip, shaper and limiter.
    - Simd4 also for the multiband and the limiter.
  - Each run is 24 blocks of 64 frames, including silent and loud blocks, with random Point and
    Linear spans. Some spans are refused by the effects' own rules.
  - Mid-run events: a `DiscontinuityKeepParameters` reset, a `FullToDefaults` reset, and a
    snapshot/restore into a fresh twin (scalar) or into another lane (bank).
  - Hashed: every output word, every report, and every state payload after every block.
  - Result: all 412 digests are identical between base and head.
- **Gate 2.** `size_of_val` of each boxed processor, base -> head, reproduces the implementer's
  table exactly:
  - compressor: scalar 3424->3304, bank8 3680->3392
  - delay: scalar 424->304
  - gate-expander: scalar 1000->864, bank8 2400->2272
  - multiband-compressor: scalar 856->728, bank4 3056->2784, bank8 5792->5504
  - parametric-eq: scalar 2288->1992, bank8 15840->15552
  - soft-clip: scalar 376->240, bank8 768->480
  - transient-shaper: scalar 336->216, bank8 1696->1440
  - true-peak-limiter: scalar 832->696, bank4 1584->1312, bank8 2368->2112
- **Gate 3.** The mutation runs above.

Not verified: the four-lane AArch64 measurement of `effect_bank_metadata_bytes`. There is no arm64
host or qemu here, and the record leaves it to CI's `aarch64-debug`. My check of its derivation is
under m2.

Evidence that I keep: `/tmp/claude-1002/v1461/logs/` (gate logs, both differential outputs,
base/head size probes and base capi budget and audit outputs), `/tmp/claude-1002/v1461/diffsrc/`
and `/tmp/claude-1002/v1461/sizesrc/` (the throwaway probe sources).
