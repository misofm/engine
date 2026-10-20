# Close the batch 2 mutation-survivor gaps (TBD-TA5-gap)

## Mission

The batch 2 mutation sample (measured at 81a3074fc, read at ad5c7ae86) found six product or gate behaviours that change under a mutant while no test turns red. It also found one engine contract that has no engine caller. This issue adds one small test per gap and decides the dead contract. Each test has to name the defect it catches, as the "Test value" rule in AGENTS.md requires.

## Scope

One row per item. Lines are at ad5c7ae86.

1. **Section-header names on the control list.** `SectionRecord::name`, crates/host-core/src/control_provider.rs:225. Mutant: the body returns `""`. Unchecked behaviour: every `ControlSectionHeader.name` on the wire is `own`, `builtins` or the console-slot/insert ID. Existing catalog walks match `SectionHeader(_)` and never read `name`. Smallest test: in the existing host-core catalog walk (control_provider.rs, near :2802), assert each section header's `name` against `own`/`builtins`/`address.slot_id`. This is one `assert_eq!` per arm on a session that has one console slot and one insert.
2. **Effect-count cap.** `count_effects`, crates/host-core/src/prepare.rs:6765. Mutant: `Ok(0)`. Unchecked behaviour: `check_counts` refuses a session whose console-plus-insert effect count is above `caps.maximum_effects` with `host.resource.count`. `PreparedReport.effect_count` and `HostShape.effect_count` also report the real count. Every caps test covers tracks, submixes, VCAs or reach pairs only. Smallest test: in crates/host-core/tests (next to submix_caps.rs), one track with two inserts. It prepares at `maximum_effects: 2` and is refused with `host.resource.count\t$\n` at `maximum_effects: 1`.
3. **Effect-order edit refusal.** `apply_session_edit` (`SetTrackEffectOrder`), crates/protocol/src/model.rs:799. Mutant: `||` -> `&&`. Unchecked behaviour: an order list that is a strict subset of the rack's effects is refused with `InvalidEffectOrder`. Under the mutant it succeeds and silently drops the unlisted effect. No test triggers `InvalidEffectOrder` at all. Smallest test: in crates/protocol/tests, a rack `[a, b]` with `SetTrackEffectOrder [b]`. Assert `Err(InvalidEffectOrder)` and that the rack is still `[a, b]`. Add one same-length case with a duplicated ID (`[a, a]`), also refused.
4. **Peak-heap window measures something.** `record_peak_window`, tools/bench-support/src/alloc.rs:159. Mutant: the body is a no-op. Unchecked behaviour: `current_thread_peak()` tracks live and peak bytes. Its two readers (parameter-metadata/tests/control_command_heap.rs, control-plane/tests/observation_bindings.rs) assert only ceilings, which a constant 0 passes, so the #1615 heap gates cannot fail. Smallest test: in bench-support, call `current_thread_peak_reset()`, allocate a `Vec<u8>` of 4096 bytes, and assert `peak_bytes >= 4096`. Then drop it and assert `live_bytes == 0` and `peak_bytes >= 4096`.
5. **Lowercase hex rule in the fixture audit.** `json_object_hex`, tools/audit/src/fixture_builtins.rs:2934. Mutant: `&&` -> `||`. Unchecked behaviour: a JSONL hex field (`start`, `handle`, `reset_generation`, `sequence`) in uppercase is refused. Under the mutant it is accepted. Smallest test: a unit test in fixture_builtins.rs that calls `json_object_hex` on a one-key object. `"00000000000000ab"` gives Ok(0xab), and `"00000000000000AB"` gives the lowercase-hex error.
6. **PCM mismatch index (low value; the implementer may decline it with a reason).** `verify_pcm_words`, tools/audit/src/fixture_builtins.rs:2627. Mutant: `!=` -> `==` in `position`. Unchecked behaviour: the error names the first word that differs, not the first word that is equal. Ok/Err is not affected. Smallest test: write a temp file with words `[1, 2, 3]`, expect `[1, 9, 3]`, and assert the error contains `word=1`.
7. **Dead contract: document-decimal lattice matching.** `ExactDecimal::parse` and `lattice_index_for_decimal`, crates/effect-contract/src/step.rs:731 and :873 (mutants at :774 and :775, `||` -> `&&`, which accept `.5` and non-digit text). No crate, host or C ABI path calls `lattice_index_for_decimal`. Its only non-test caller is the `lattice_oracle` SDK cross-check tool, which passes canonical renderings only. The module comment says persisted values are matched on document text, but no engine path does that. Decision required: either (a) delete `ExactDecimal`, `lattice_index_for_decimal` and their tests, and have `lattice_oracle` use an exact match on `points[i].canonical` instead, or (b) if a product path must match document text, record that as its own issue. Do not add parse tests to code that has no caller.

## Non-goals

- No test for the equivalent or unreachable survivors: `scheduling_barrier` (lane/src/fpenv.rs:504, no instructions, codegen-only), `distinct_planes_mut` (graph/src/runtime.rs:3646, the staged fallback gives the same bits), the `BankStage::begin_block` default (rack/src/lib.rs:502, same body), and `is_strictly_jury_stable` (dsp-reference/src/svf.rs:186, every caller stays at or below 0.45 fs).
- No rewrite of the heap-gate ceilings, the caps model or the fixture corpus.
- No new benchmark or harness.

## Gates

- Each new test is red on its listed mutant and green on the tree. Record each red run in the PR body, not in a committed file.
- Every test has a one-sentence test-value answer naming the defect.
- Item 7: the decision is in the PR body. If (a), the crate and tool still build, and `lattice_oracle` output stays byte-identical to its pre-change output (one-time PR evidence, not a committed test).
- The qualification `lint` job steps pass on the branch.
