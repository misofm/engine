# Replace source-text and prose scrapes in tests, then lint the pattern

Source: `docs/handoffs/test-value-2026-09-28/TEST-VALUE-AUDIT.md`, verified in `VERIFY-TEST-VALUE.md`. **The Amendments section supersedes the body wherever they conflict.** Owner rulings: `docs/rulings/engine-footprint-2026-09-28.md`. Related cleanup issues: #1017-#1042.

Draft, not a GitHub issue. From the 2026-09-28 test-value audit
([`../TEST-VALUE-AUDIT.md`](../TEST-VALUE-AUDIT.md) §5 item 7 and §9). Base `a9414c0c`. Paths starting
`../` are relative to the audit's handoff folder. No ruling
needed.

## Problem

Thirty tests read source code or documentation text and match on it: `include_str!` of a `.rs`,
`.toml` or `.md` file, or `read_to_string` of a source directory. They are the pattern the 2026-09-04
rubric banned:

- **Brittle.** They fail on a rename, a comment or a `rustfmt` change. For example,
  `crates/graph/src/runtime.rs:7770` pins indentation "as rustfmt lays it out since issue #943".
- **Blind.** They pass on the semantic change they are meant to catch.
- **Still growing.** New ones were added after that audit: `graph/src/runtime.rs:7770` and `:7937`,
  `lane/tests/input_chain_elision.rs:939`, `source/src/lib.rs:2882`.

The full list, with the classifier's note per test, is the rows flagged `scrape` in
[`../data/test-classes.tsv`](../data/test-classes.tsv). Decisions:

| test | decision |
|---|---|
| `crates/graph/src/runtime.rs:7770` `resident_meter_entry_has_one_final_output_dispatch_and_admission_control`; `:7937` `rt9_resident_entry_has_one_guarded_production_caller_and_control` | delete: their recorded mutations (G-2/A-5, 936-4/957-1b) are also red on behavioural meter and render tests |
| `crates/lane/tests/input_chain_elision.rs:939` | delete: `src/kernels/builtins.rs:1749` proves the count behaviourally |
| `crates/math/tests/f1_fast_db_bounds.rs:1168` | delete: it counts the test file's own functions |
| `crates/math/tests/m3_determinism.rs:54`, `:101` (vendored source has no target cfg, `mul_add`, `unsafe`, `force_eval`) | move into `scripts/check-lane-policy.sh`, a policy scan belongs in a policy script. `unsafe` is already `deny` workspace-wide |
| `crates/capi/src/ffi.rs:2516` `ffi_never_forms_a_whole_plan_reference` | **rewrite**: with no Miri leg it is the only guard. Express the rule in the type, or move the scan to `check-realtime-policy.sh` |
| `crates/effect-compiler/tests/migration.rs:1053`; `migration_terminal.rs:1833` | delete: `scripts/check-effect-state-migration-v1.sh` owns the boundary, and Cargo refuses cycles |
| `crates/effect-package/tests/state_vectors.rs:1351` | delete: the selector and registry behaviour is asserted by the vectors |
| `crates/session/tests/json_contract_artifacts.rs:98` (markdown headings, "**25** live … TOMLs") | delete: a prose pin |
| `crates/source/src/native_source.rs:1806`, `:2623`, `:2688`; `crates/source/src/lib.rs:2020`, `:2840` | delete (candidates H40-H42 in `../data/candidates-host-tools.md`) |
| `crates/source/src/lib.rs:2882` `native_commit_publishes_the_decoded_block_without_a_copy` | **rewrite**: assert that the published block's storage is the decoded buffer (pointer identity), or count copies with an instrumented buffer |
| `crates/source/src/lib.rs:2606`, `native_source.rs:4015`, `native_wave.rs:1071` | drop only the scraping assertions; the behavioural halves stay |
| `tools/audit/src/capi.rs:328`, `builtins_graph.rs:845`, `builtins_fixture_check.rs:543` | delete (H47) |
| `tools/audit/src/fixture_builtins.rs:5218`; `tools/bench/src/protocol.rs:1530`, `:1545` | drop only the text greps |
| `tools/parameter-metadata/tests/abi_layout.rs:1379` (scrapes a shell heredoc) | delete: `check-web-audioworklet.sh` owns the frozen export list and checks it against the shipped module. Keeping the list independent of the generated metadata is the point, so it is not derived |
| `tools/session-validator/tests/skill.rs:19` | **rewrite**: run the argv in `SKILL.md`'s fenced blocks through the CLI parser |
| `crates/session/tests/json_contract_artifacts.rs:35`, `:47` | **keep**: they read the shipped JSON schema, a product artifact, not source text |

## Outcome

- No test reads Rust source or a Cargo manifest as text. The prose pins in the table are gone. The
  one test that still reads a `.md` (`skill.rs`, rewritten) reads it in order to execute its
  commands.
- The allow-list is empty. #1052 landed the lint in `scripts/check-workspace-policy.sh`, with an
  exact allow-list of the scrapes that predate it: one row per file and literal path, each naming
  #1047, #1035 or #1037. Each deletion or rewrite here lowers or deletes its row in the same PR, and
  the lint stays red until the count matches. With no row left, the lint refuses every literal-path
  read of a `.rs` or `.toml` file. #1047 adds no lint and no mutation case.
- The lint cannot see scrapes whose path is built at run time (`read_dir`, joined paths), as in
  `m3_determinism.rs:101`. Those are left to #1052's review question.

## Scope

Authorized paths: the test files in the table, `scripts/check-lane-policy.sh`,
`scripts/check-realtime-policy.sh`, `scripts/check-workspace-policy.sh` (its allow-list rows only),
`scripts/check-web-audioworklet.sh` (the export list only), and this issue's spec.

## Gates

1. **Every rewrite catches its named behaviour.**
   - `source/src/lib.rs:2882`: a scratch mutation adds a `copy_from_slice` into a fresh buffer in
     `commit_block`, and the rewritten test goes red.
   - `capi/src/ffi.rs:2516`: forming `&*plan` in an FFI function goes red.
   - `skill.rs`: renaming a CLI flag goes red.
2. **The moved policy scans still fail on seeded violations.** Add
   `#[cfg(target_feature = "fma")]` to a vendored math file; add `core::arch` to one.
3. **Mutation equivalence.** Every deleted test is in a crate the audit measured, or has a recorded
   mutation that is also red elsewhere. For graph, re-record G-2, A-5, 936-4 and 957-1b from
   `crates/graph/tests/MUTATIONS.md` and show each is still red after the deletions.
4. **Historical bugs.** `../tools/revert.py` for all four bugs; the red set is unchanged.
5. **The allow-list is empty.** No `source_scrape_allowlist` row is left.
   `check-workspace-policy.sh` and `test-workspace-policy.sh` pass, and the latter's planted
   `source-reads` case stays red. The rows #1035 and #1037 own go with their code; any still
   present are deleted here.

## Saving and risk

- **Saving:** ≈ 0 s of runtime. The saving is that no future `rustfmt`, rename or doc edit turns a
  test red, and the common form of the pattern cannot return.
- **Risk:** low. Three rows are rewrites rather than deletions (`source/src/lib.rs:2882`, `capi/src/ffi.rs:2516`, `skill.rs`), because nothing else guards them.

## Amendments (Sol verification, 2026-09-28)

See [`../VERIFY-TEST-VALUE.md`](../VERIFY-TEST-VALUE.md). **These amendments supersede the body wherever they conflict.**

1. **Drop rows in code the filed issues delete:**
   - `source/src/native_source.rs:1806`, `:2623`, `:2688`, `:4015` and `native_wave.rs:1071`
     (#1035);
   - `effect-compiler/tests/migration.rs:1053`, `migration_terminal.rs:1833` and
     `effect-package/tests/state_vectors.rs:1351` (#1037).

   Land after #1035 and #1037, or skip those rows. Each of those issues lowers or deletes its
   allow-list rows when it deletes the code.
2. **The lint pattern must exclude literal paths that contain `/fixtures/`, not files under
   `fixtures/`.** `tools/bench/src/builtins.rs:279` and `:283` legitimately `include_bytes!` fixture
   `.toml` files. A pathspec exclusion still flags them.
3. **`lane/tests/input_chain_elision.rs:939`: state what is lost.**
   - The named replacement (`kernels/builtins.rs:1749`) counts plan selections per channel. It does
     not see a runtime branch inside `mixed_channel_block`'s frame loop, which the scrape does.
   - Either accept that loss in the PR, or leave the "no per-frame branch" claim to a codegen check.
4. **Confirmed.**
   - `graph` G-2, A-5, 936-4 and 957-1b are red on behavioural tests
     (`crates/graph/tests/MUTATIONS.md:458`, `:549`, `:590`, `:594`, `:646`). G-2 is green on the
     scrape itself.
   - No workflow runs Miri, so `capi/src/ffi.rs:2516` is correctly a rewrite.
5. **Size.** If the pruned list still exceeds half a day, split it: (a) the deletions plus the lint;
   (b) the three rewrites.
6. **The lint landed in #1052** (root brief, 2026-09-28). The body above is amended to match:
   #1047 removes the scrapes and deletes the allow-list rows, and adds no lint. Amendment 2's
   `fixtures/` exclusion is in #1052's pattern. In amendment 5's split, slice (a) is the deletions
   and their rows.

## Attempt 1 evidence

Terra, 2026-09-29, branch `codex/1047-replace-source-scrapes` from `main` at `a8955ad4`. Commits:
`67fa22aa` (the change) and this record. Every mutation and seeded violation below was applied to a
scratch copy (`git archive` of the base, with the change's patch where noted), never the checkout.

**Size.** 21 files, +250 / −829 lines in `67fa22aa` (`git diff --numstat a8955ad4 67fa22aa`).
Product code is untouched: every Rust hunk is a `#[cfg(test)]` module or a `tests/` file, plus
`crates/math/VENDORED.md`'s one pointer line. The script hunks are the four gates the spec
authorizes and the self-test suites of the two gates that gained a scan.

**Out of the authorized list, and why.** `scripts/test-lane-policy.sh` and
`scripts/test-realtime-policy.sh`: the scans moved into their gates need fixture files (thirty
vendored sources; a capi `ffi.rs` with a render entry) or every existing case goes red, and the
moved rules get their own red cases there. `crates/math/VENDORED.md:74` named the deleted
`m3_no_target_conditional_source` as "the gate that keeps it that way"; it now names the lane
policy.

### Each scrape, and what replaces it

| spec row | decision taken | replacement or covering catch (re-recorded on the change unless noted) |
|---|---|---|
| `graph/src/runtime.rs` `resident_meter_entry_has_one_final_output_dispatch_and_admission_control` | deleted | #943 G-2, #950 G-1, G-2 and A-5, red on graph-compiler's meter tests (gate 3) |
| `graph/src/runtime.rs` `rt9_resident_entry_has_one_guarded_production_caller_and_control` | deleted | 936-4 / 957-1b, red on three graph dispatch tests (gate 3) |
| `lane/tests/input_chain_elision.rs` `mixed_elision_production_structure_…` | deleted, with its two helpers | wrong shape selected: red on five bit-identity tests. **Loss accepted (amendment 3):** a runtime branch inside `mixed_channel_block`'s frame loop moves no bit and no selection count, so no test sees it; see L-2 below |
| `math/tests/f1_fast_db_bounds.rs` `f1_the_container_pins_exactly_eight_crossings` | deleted | none needed: it counted the test file's own `fn f1_crossing_x` spellings, so no product change could turn it red |
| `math/tests/m3_determinism.rs` `m3_no_target_conditional_source`, `m3_no_unsafe_or_force_eval_in_vendored_source` | moved into `scripts/check-lane-policy.sh` | the same needles, the same comment-line rule, the thirty-file floor; seeded violations red (gate 2) |
| `capi/src/ffi.rs` `ffi_never_forms_a_whole_plan_reference` | **rewritten** as a scan in `scripts/check-realtime-policy.sh` | the same four forms and projection rule over `ffi.rs`'s production region; forming each in the render FFI function is red (gate 1). The type option would change `abi.rs`, which is outside this issue's paths |
| `effect-compiler` `migration.rs:1053`, `migration_terminal.rs:1833`; `effect-package` `state_vectors.rs:1351` | already gone | deleted with their crates and files by #1037 |
| `session/tests/json_contract_artifacts.rs:98` `migration_inventory_…` | deleted | none needed: a prose pin of a ruling's headings and "**25** live … TOMLs"; the module doc no longer names the inventory |
| `source` `native_source.rs:1806`, `:2623`, `:2688`, `:4015`; `native_wave.rs:1071`; `lib.rs:2020` | already gone | deleted by #1035 with the native decode workers |
| `source/src/lib.rs:2882` `native_commit_publishes_the_decoded_block_without_a_copy` | **moot** | #1035 deleted `commit_block` and this test with it; nothing is left to rewrite |
| `source/src/lib.rs:2840` `producer_submission_has_one_stamping_and_copy_body` | deleted | S-1 to S-3 red on 7 to 15 behavioural source tests; S-4 (a second copy) is the loss (gate 3) |
| `source/src/lib.rs:2606` `graph_driver_zero_claims_…` | scraping assertions dropped | the zero-claim recycle and the retained report stay |
| `tools/audit` `capi.rs:328`, `builtins_graph.rs:845`, `builtins_fixture_check.rs:543` | deleted (H47) | `CALLS == 100_000` is the real `audit capi` record's `"calls": 100000` (`qualification.yml:719`); `builtins_graph`'s retirement test (each plan's destruction thread); `issue069_checker_is_read_only_and_rejects_payload_mutation`. The no-timer and no-blocking token lists guarded audit-tool text, not a product claim, and are not replaced |
| `tools/audit/src/fixture_builtins.rs:5218` | text greps dropped, and the two helpers only they used | renamed `issue064_checked_corpus_is_read_only_and_complete`: `--check` through `run`, the tree unchanged, the manifest identity |
| `tools/bench/src/protocol.rs:1530`, `:1545` | text greps dropped | the corpus checksum and the builder/verifier round trip stay |
| `tools/parameter-metadata/tests/abi_layout.rs:1379` | deleted | `check-web-audioworklet.sh` now also holds the **shipped** `miso-engine-v1-abi-layout.json`'s `exports` to its frozen list (`memory` aside), read from the artifact; the list stays independent of the generator. Seeded drift red (gate 1) |
| `tools/session-validator/tests/skill.rs` | **rewritten** as `the_shipped_skill_validator_commands_run_verbatim` | every `session-validator` line in `SKILL.md`'s fenced shell blocks runs through the validator binary on `canonical-minimal.json`; exit 0, and `--canonical` prints the canonical bytes (gate 1) |
| `session/tests/json_contract_artifacts.rs:35`, `:47` | kept | they read the shipped JSON schema |

The skill test is the one test that still reads a `.md`, to execute its commands. It runs only the
session-validator lines: `parameter-metadata -- --print` is another package's binary, which no
`CARGO_BIN_EXE_*` reaches from here, and `scripts/test-web-audioworklet.sh:219` already runs that
exact command in CI.

### The `cargo test -- --list` diff

Base `a8955ad4` against `67fa22aa`, for `-p graph -p lane -p math -p capi -p source -p session -p
audit -p bench -p parameter-metadata -p session-validator --all-targets --features
graph/test-support,math/lane,lane/test-support`: 462 → 449 names.

| binary | removed | added |
|---|---|---|
| `graph` lib | `runtime::tests::resident_meter_entry_has_one_final_output_dispatch_and_admission_control`, `runtime::tests::rt9_resident_entry_has_one_guarded_production_caller_and_control` | — |
| `lane` `input_chain_elision` | `mixed_elision_production_structure_selects_before_frames_and_audits_helpers` | — |
| `math` `f1_fast_db_bounds` | `f1_the_container_pins_exactly_eight_crossings` | — |
| `math` `m3_determinism` | `m3_no_target_conditional_source`, `m3_no_unsafe_or_force_eval_in_vendored_source` | — |
| `capi` lib | `ffi::tests::ffi_never_forms_a_whole_plan_reference` | — |
| `session` `json_contract_artifacts` | `migration_inventory_keeps_all_four_classifications_and_reproducible_audit` | — |
| `source` lib | `tests::producer_submission_has_one_stamping_and_copy_body` | — |
| `audit` bin | `builtins_fixture_check::tests::issue069_author_is_not_reachable_from_audit_mains`, `builtins_graph::tests::issue070_retirement_worker_source_is_limited_to_nonblocking_primitives`, `capi::tests::audit_plan_is_fixed_non_timed_and_calls_the_c_entrypoint`, `fixture_builtins::tests::issue064_checked_corpus_is_read_only_complete_and_has_no_authoring_reachability` | `fixture_builtins::tests::issue064_checked_corpus_is_read_only_and_complete` (the rename) |
| `parameter-metadata` `abi_layout` | `the_published_export_set_is_the_frozen_artifact_set` | — |
| `session-validator` `skill` | `the_shipped_skill_names_the_real_commands` | `the_shipped_skill_validator_commands_run_verbatim` (the rewrite) |

`bench` keeps its names; two of its tests lost their greps.

### Gate 1: every rewrite catches its named behaviour

- **`skill.rs`.** Validator `lib.rs` `"--canonical" if !canonical` → `"--canonicalize" if
  !canonical`: the rewrite is RED (`… validate --canonical draft.json > session.json" does not run
  as written`); the deleted scrape, on the base with the same edit, GREEN. `Some("validate")` →
  `Some("check")`: RED on the rewrite (`… validate path/to/session.json`), GREEN on the scrape. Both
  green again once restored.
- **`capi/src/ffi.rs`.** Each of `&mut *plan`, `&*plan`, `&(*plan)` and `&mut (*plan)`, bound in
  `miso_engine_v1_render_f32_planar` (`ffi.rs:768`), turns `check-realtime-policy.sh` RED: `the C
  ABI forms a reference to a whole Plan …`, naming `crates/capi/src/ffi.rs:768`. The committed file
  is green: `&*plan.cast::<HandleHeader>()`, `&mut *plan_state(plan)`, comment lines and the test
  module are not hits.
- **`source/src/lib.rs:2882`.** Moot: the `commit_block` the gate names was deleted by #1035.
- **The export-set replacement.** A copy of the built artifact directory with
  `miso_engine_web_v1_source_seek` dropped from the layout document, and one with a
  `miso_engine_web_v1_source_rewind` added, each turn `check-web-audioworklet.sh` RED (`the ABI
  layout document's published exports are not the frozen export set`, with the diff). The built
  artifact itself passes the whole script.

### Gate 2: the moved scans still fail on seeded violations

On a scratch copy of the base tree, with the change's `check-lane-policy.sh`:
- `#[cfg(target_feature = "fma")]` and a helper above `vendored/exp2.rs`'s `exp2`: RED,
  `crates/math/src/vendored/exp2.rs:329: target_feature: …`, `the vendored math sources must
  contain no target-conditional, fused or unsafe construct (gate M3)`.
- `use core::arch;` appended to `vendored/log2.rs`: RED, `log2.rs:109: core::arch`. The existing
  architecture rule (`(core|std)::arch::`) does not see this form; the moved scan does.
- Restored: `lane policy: ok`.

`test-lane-policy.sh` carries these as cases (`#[cfg(target_feature)]`, `#[cfg(target_arch)]`,
`use core::arch;`, `std::arch` in a comment, `unsafe`, `force_eval!`, `select_implementation!`,
`read_volatile`, the file floor at 29 files, the missing directory) plus injected `find` and `awk`
failures. `test-realtime-policy.sh` carries the four forms, a double-spaced `&mut  *plan`, a borrow
on its own line, one under a comment line, the missing file, and the render entry present only in
the test module, plus injected `awk` and `rg` failures. Each case asserts its own diagnostic.

### Gate 3: mutation equivalence

Graph, re-recorded from `crates/graph/tests/MUTATIONS.md` on the change, each alone, `cargo test -p
graph -p graph-compiler --lib --features graph/test-support,builtins-compiler/test-support`
(`--no-fail-fast`, dev). The unmutated tree is green.

| row | mutation | red on the change (the source scans are gone) |
|---|---|---|
| #943 G-2 | `Some([*left.get(lane ^ 1)?, *right.get(lane ^ 1)?])` | gc `post_matrix_peak_meters_merge_one_bank_pass_per_cohort_and_publish_the_scalar_frames` |
| #950 G-1 | `meters.lane(lane ^ 1, seeds)` | 9 gc tests, among them `post_matrix_all_meters_…`, `the_full_meter_pass_renders_without_an_audited_event`, `…_stays_off_…`, `an_observer_failing_mid_bank_…` |
| #950 G-2 | `plane::<L>(left, frames, right_seeds)` | the same 8 without `the_full_meter_pass_renders_…` (it was green on the scan, as recorded) |
| #950 A-5 | `bank_meter_seeds`'s result ignored | gc `post_matrix_all_meters_run_one_full_bank_pass_per_cohort_and_publish_the_scalar_frames` |
| 936-4 / 957-1b | `let _ = &active_units; for unit in 0..runtime.units.len() {` | graph lib `a_delayed_claim_stays_dispatched_and_renders_the_base_bits`, `an_observed_source_input_stays_dispatched_and_meters_the_base_values`, `an_unobserved_source_input_is_not_dispatched_and_moves_no_bit` |

Two of the deleted scans' own control rows, run the same way, beyond what the spec asks:
- rt9's admission bypass (`let admitted = true;`): RED on 20, graph lib
  `a_folded_metered_plan_is_the_unfolded_plans_master_and_meters_bit_for_bit` and 19 gc tests.
- the resident scan's `let resident = if true {` (the bank-eligibility guard dropped for the
  resident view): **GREEN** on graph and graph-compiler lib. It is not a recorded mutation, and only
  the deleted scan caught it. Whether a reachable plan binds an ineligible bank with a post-matrix
  meter, where `final_output_lane`'s own shape checks would not already refuse the view, was not
  investigated. Named here for the reviewer.

Lane (`-p lane --lib --tests --features lane/test-support`), on the change and on the base:
- L-1, `[true, false] => mixed_channel_block::<L, false, true>(…)`: RED on the change on
  `elision_is_bit_identical_at_every_width_and_section_pattern`, the three
  `mixed_elision_matches_frozen_bodies_*` and `every_copy_of_the_sanitise_prologue_counts_…`; on the
  base the same five plus the deleted scan.
- L-2, #710's inner-plan mutation (the shape passed into `mixed_channel_block` and read in the frame
  loop as `if shape[0]`, `if !shape[1]`, `else if !shape[0]`): GREEN on the change; on the base RED
  only on the deleted scan. This is amendment 3's loss, accepted: a per-frame branch is a codegen
  property, and no bit or selection count moves.

Source (`-p source --lib --tests`), on the change and on the base:
- S-1, `block.generation = generation;` dropped: RED on 7 (the base: the same 7 and the deleted scan).
- S-2, `block.start_frame = start_frame;` dropped: RED on 14 (15 with the scan).
- S-3, the plane copy dropped: RED on 15 (16 with the scan).
- S-4, the plane copied twice: GREEN on the change; on the base RED only on the deleted scan. A
  second copy into the same block moves no sample; it is a cost, and the loss is accepted.

The audit-tool scans guarded their own tool's text (a timer token, a blocking primitive, a
`--write` spelling), not a product behaviour; their behavioural neighbours are named in the table.
`f1`'s self-count and session's prose pin could not fail on any product change.

### Gate 4: historical bugs

`../tools/revert.py` on scratch copies of the base and the change, each bug alone, `cargo test
--no-fail-fast` (dev) over the packages that hold its recorded reds plus every changed package that
depends on the bug's crate (capi, parameter-metadata, session-validator; graph and source for #970):

| bug | base red | change red |
|---|---|---|
| #966 | 9: the `bank_levels.rs` tests, including the randomized probe | the same 9 |
| #970 | 8: the 7 `collapse_arming.rs` reproducers and the `bank_levels.rs` randomized probe | the same 8 |
| #994 | 9: the knee reproducers and the three `randomized_differential_*` | the same 9 |
| #1015 | 3: the `stationary_subnormal` tests | the same 3 |

No deleted test was red on any of the four bugs. lane, math and session depend on none of the four
crates.

### Gate 5: the allow-list is empty

`source_scrape_allowlist` has no row. `check-workspace-policy.sh`: ok. `test-workspace-policy.sh`:
ok, including the planted `source-reads` case (nine literal-path forms, each reported) and the
ratchet cases. Seeded on a scratch copy of the change, `include_str!("runtime.rs")` in a graph test
is refused: `read as text: crates/graph/src/runtime.rs:…`.

### Other gates

- `cargo fmt --all -- --check`; `cargo check --locked --workspace --all-targets --all-features`;
  `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`: pass, no warning.
- The ten changed packages' tests, dev and release (`--all-targets`, features as above): 430 passed,
  0 failed, 19 ignored in each.
- Console digests (`gain_pan_profile digests`, release): 17 rows, byte-identical on the base and
  the change.
- The AArch64 debug leg's exact package and feature set (`scripts/lib/product-crates.sh`, 25
  crates, plus `dsp-reference`, `conformance`, `target-smoke`), on x86: `cargo test --locked
  --all-targets … --no-run` builds 218 executables; its `-- --list` (1,783 tests) passes
  `aarch64-known-defects.py judge-skips debug`. No known-defect row names a deleted test.
- `check-web-audioworklet.sh` on an artifact built by `build-web-audioworklet.sh`: pass.
- Every `check-*.sh` but `check-sdk-types.sh` (needs `npm ci`), and every `check-*.py` with
  `python3 -B`: pass. `check-capi-abi.sh` and `check-graph-determinism.sh` read the checkout's own
  `target/`, so they ran in the scratch copy of the change with its own `target/`: pass. The
  argument-taking Python gates ran in `--self-test` mode and, where they take one, on the built
  artifact: pass. `check-console-fixtures.sh` failed once, with the validator's `expected the
  validate subcommand`, and passed on rerun with no source change. The likely cause, not proven:
  the scratch target directory was shared with the base copy, whose validator had been built with
  gate 1's renamed subcommand, so the uplifted `target/debug/session_validator` was briefly that
  binary.
- `test-lane-policy.sh`: pass on four runs of the final suite. A fifth, earlier run of the same
  suite reported `missing injected diagnostic: vendored-scan-0` although its output held that line;
  it did not recur, and the cause was not found. `test-realtime-policy.sh`, `test-workspace-policy.sh`, `test-web-audioworklet.sh`,
  `check-ci-path-routing.py` and `test-ci-path-routing.py`: pass.

### For the reviewer

- The one found gap is `let resident = if true {` (gate 3): green without the resident scan.
- `crates/builtins/tests/MUTATIONS.md:17` names `issue064_checked_corpus_is_read_only_complete_and_has_no_authoring_reachability`,
  now `issue064_checked_corpus_is_read_only_and_complete`; the record is dated evidence and was
  left as written.

## Sol verdict, attempt 1

**PASS.** Sol, 2026-09-29. The change (`67fa22aa`, evidence `7e7b250d`) was merged into the batch head
`codex/batch-slim-4` at `93105e18` (main plus #1060 and the #1069-#1074 specs) in a scratch detached
worktree. All checks below ran on that merge unless noted. Every mutation was applied to a scratch
copy and reverted.

### The merge

- **No textual conflicts.** #1060 and #1047 change disjoint files.
- **No semantic conflicts.**
  - #1060 added no literal-path source read. `check-workspace-policy.sh` passes with the allow-list
    empty.
  - #1060 did not touch `ffi.rs`, and the capi scan is green on the merged file.
  - capi's resource tests pass in dev and release.
  - `check-browser-expected-resources.py --artifacts` passes on an artifact built from the merge.

### Each replacement discriminates (re-applied by Sol)

- **capi whole-plan borrow.** Sol bound six forms in `miso_engine_v1_render_f32_planar`: `&*plan`,
  `&mut *plan`, `&(*plan)`, `&mut (*plan)`, a double-spaced `&mut  *plan`, and `&*plan` inside a
  call. Each turns `check-realtime-policy.sh` RED, naming `ffi.rs:745`. Restored, it is green.
- **M3.**
  - `#[cfg(target_feature = "fma")]` above a helper in `vendored/exp2.rs`: RED with the M3
    diagnostic.
  - `use core::arch;` in `log2.rs`: RED.
  - `unsafe` and `cfg!(target_arch = …)`: each RED.
  - `mul_add`: RED, caught first by the existing D3 rule.
  - Restored: `lane policy: ok`.
  - The needles are the deleted tests' own: the same six on every line, the same four on code
    lines, the same comment rule and the same 30-file floor.
- **skill.rs.** Validator `"--canonical"` → `"--canon"`: RED. `Some("validate")` →
  `Some("check")`: RED. Restored, it is green.
- **Export set.** Sol added `miso_engine_web_v1_source_rewind` to the generator's `EXPORTS` itself,
  not only to the JSON. Sol then regenerated the layout document with `parameter-metadata --write`
  into a copy of the built artifact. `check-web-audioworklet.sh` is RED at the new check, its first
  output line, with a diff naming the export. The unmutated artifact passes the whole script.
- **Scrape lint.** Two planted scrapes are each refused (`read as text: …`), and the lint is green
  once they are removed:
  - `include_str!("../src/kernels/builtins.rs")` in a lane test;
  - `read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"))` in source's test module.

  `test-workspace-policy.sh` passes, including `source-reads`.
- **Graph.** Sol ran all 18 controls of the two deleted scans on the merge (`-p graph -p
  graph-compiler --lib --tests`, `--features graph/test-support,builtins-compiler/test-support`):
  15 resident-scan controls and rt9's three `lib.rs` controls.
  - 17 are RED on behavioural tests, from 1 to 102 tests each.
  - Among them: `Some(index)`, which drops the final-lane offset; the one-frame view; an inverted
    fold flag; the swallowed resident write; the forced sample-peak and meter passes; ignored meter
    peaks; a withheld meter or peak; unread seeds; `left, left`; swapped L/R peaks; the per-lane
    block borrow; a skipped observer; a continued error; the `unit + 1` observer.
  - The only green one is `let resident = if true` (finding 1).

### Moved scans are structural policy, not relabelled tests

AGENTS.md refuses tests that grep source or prose. The `check-*-policy.sh` gates are the repo's home
for structural source rules, such as unsafe owners, the `arch` modules and `MAX_TRACKS`.

- **M3** is a structural determinism rule: no target-conditional, fused or unsafe construct in
  vendored libm. Its behavioural half, the digests, stays a test.
- **The capi rule** is aliasing soundness: no `&Plan` while render holds `&mut PlanState`.
  - No test can observe it without Miri, and no workflow runs Miri.
  - The spec named this home explicitly.
- **The export check** is not a source scan at all. It reads the shipped artifact.

### Test value (rewritten test)

`the_shipped_skill_validator_commands_run_verbatim` goes red when the validator's `validate`
subcommand or `--canonical` flag is renamed, or when `--canonical` stops printing canonical bytes,
without SKILL.md following. No other test catches that. The old scrape was green on both renames.

### Test list

`cargo test --workspace --all-features -- --list` on the base and on the merge: 2,238 → 2,225.

- Exactly the 15 removed and 2 added names in the evidence table; nothing else moved.
- Each removal has its named replacement or catch above. `f1`'s self-count and session's prose pin
  could fail on no product change.
- The audit-tool scans guarded tool text, as the spec's H47 ruling says.

### Gates on the merge

- **Build and lint.**
  - `cargo check --workspace --all-targets --all-features`: pass.
  - clippy `-D warnings`: pass, 0 warnings.
  - `cargo fmt --check`: pass.
- **The ten changed packages' tests** (`--all-targets`, `graph/test-support,math/lane,lane/test-support`):
  - dev: 433 passed, 0 failed, 19 ignored;
  - release: the same.
- **Console digests** (`gain_pan_profile digests`, release): the 17 rows are byte-identical on
  `93105e18` and the merge. `console-workload`'s release tests: 62 passed.
- **The four gates and their self-tests.** Workspace, realtime, lane and web-audioworklet
  (`test-*.sh`) all pass. `test-lane-policy.sh` passed five times; Terra's one-off
  `vendored-scan-0` flake did not reproduce.
- **Every policy script.**
  - 63 invocations pass. They cover:
    - every `check-*.sh`, with CI's arguments, against release binaries, a wasm-scalar build and
      the built artifact;
    - every `check-*.py` under `python3 -B`, run with CI's arguments or with `--self-test`;
    - `test-ci-path-routing.py` and `test-script-reachability.py`.
  - Among them: `check-cross-targets.sh`, `check-capi-abi.sh` (and its `--self-test`) and
    `check-graph-determinism.sh`.
  - `check-sdk-types.sh` exits 2: `sdk/node_modules` is missing, it needs `npm ci`, and the SDK is
    untouched.
- **AArch64 legs resolved on x86.**
  - debug, 25 product crates plus `dsp-reference`, `conformance` and `target-smoke`, with its
    feature set: `--no-run` builds, and `-- --list` passes `aarch64-known-defects.py judge-skips
    debug`;
  - release, `lane` and `math`: builds and passes `judge-skips release`;
  - `console-workload`, release: `--no-run` builds.

### Findings, by severity

1. **Low: `let resident = if true` is an equivalent mutation. It is not an unguarded claim.**
   - **What the guard does.** `eligible` withholds the resident view from a bank that fails any of
     four conditions:
     - a population within the width;
     - a member count that is a multiple of the population;
     - a trailing-prefix active mask;
     - no armed aux lane.
   - **Why every plan the engine can build passes it:**
     - `arm_aux` has no caller outside rack's own tests (rack's own comment: the epilogue is empty
       on every chain the engine builds);
     - rack-compiler pushes every `Some` member before `None` padding, and `trailing_active_mask`
       derives the graph's masks the same way;
     - `finish_unit` sets `lanes = members / run.len()`;
     - `accumulate_aux` only reads scratch, so even an armed chain's resident words equal the
       scattered plane.
   - **Probe.** A panic on any observed ineligible bank, at both eligibility sites, fired zero
     times over 875 tests in nine packages under `--all-features`: graph, graph-compiler,
     builtins-compiler, host-core, capi, host-web, source, console-workload and conformance.
   - **Consequence.** No behavioural test can see the mutation. A synthetic-shape test would pin a
     defensive guard whose alternative is harmless, so Sol recommends none. An issue that
     introduces aux arming or a non-prefix mask owns a metering test for that shape.
2. **Low: the new layout-export check has no red case in `test-web-audioworklet.sh`.** Its
   discrimination rests on the seeded runs above. `test-web-audioworklet.sh` is outside the spec's
   authorized paths, so a red case is a follow-up candidate, not a defect here.
3. **Low: `skill.rs` runs only fenced lines that name `-p session-validator`.**
   - A single misspelled package line would be skipped silently. `ran > 0` catches only the loss of
     all three.
   - SKILL.md's unfenced `parameter-metadata -- --print` (line 24) runs in
     `test-web-audioworklet.sh:219`.
4. **Info: `VENDORED.md` overclaims, as before this change.** It says `src/vendored/` has no
   `cfg(` and names the lane policy as its keeper. Neither the deleted tests nor the moved scan bans
   a bare `cfg(feature = …)`, and Sol's seeded one passes the gate. The overclaim predates this
   change; #1047 only repointed the sentence.
5. **Info: the out-of-scope edits are necessary.**
   - `test-lane-policy.sh` and `test-realtime-policy.sh` must carry the moved rules' fixtures, or
     every existing case goes red. They also gain the moved rules' red cases.
   - `VENDORED.md:74` named a deleted test.
