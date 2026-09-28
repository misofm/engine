# Replace source-text and prose scrapes in tests, then lint the pattern

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
- `scripts/check-workspace-policy.sh` gains one rule: `git grep` fails on `include_str!`,
  `include_bytes!` or `read_to_string` of a literal `.rs` or `.toml` path under `crates/`, `hosts/` or
  `tools/`, excluding `fixtures/`. It adds no new mutation cases: the rule is one line (AGENTS.md "The
  ceremony boundary").
- The lint cannot see scrapes whose path is built at run time (`read_dir`, joined paths), as in
  `m3_determinism.rs:101`. Those are left to the review question in issue 16.

## Scope

Authorized paths: the test files in the table, `scripts/check-lane-policy.sh`,
`scripts/check-realtime-policy.sh`, `scripts/check-workspace-policy.sh`,
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
5. **The lint works.** It passes on the result, and fails on a scratch branch that adds
   `include_str!("../src/lib.rs")` to any test. Today's tree has 32 such literal-path hits outside
   `fixtures/`, so the lint lands in the same PR as the last deletion.

## Saving and risk

- **Saving:** ≈ 0 s of runtime. The saving is that no future `rustfmt`, rename or doc edit turns a
  test red, and the common form of the pattern cannot return.
- **Risk:** low. Three rows are rewrites rather than deletions (`source/src/lib.rs:2882`, `capi/src/ffi.rs:2516`, `skill.rs`), because nothing else guards them.
