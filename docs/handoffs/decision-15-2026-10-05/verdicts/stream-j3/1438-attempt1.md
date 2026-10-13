PASS

# #1438 attempt 1 verdict (A1: realtime-policy crate, walk, unsafe ownership, markers, floors)

Range: parent `2a589d360`, head `7290d6b92` (`1bbc6fef5 b5fe62beb 3ab076114 7290d6b92`), branch
`codex/d15-stream-j3`. Built and tested from `git archive 7290d6b92` in `/tmp/claude-1002/v1438/tree`
with `CARGO_TARGET_DIR=/tmp/claude-1002/v1438/target`. The worktree was not touched, except for one
read-only `check-script-reachability.py` run, which needs git. That run was at worktree HEAD
`08b5f478b`. The three commits after `7290d6b92` change nothing under `scripts/` or `.github/workflows/`.

No BLOCKER and no MAJOR findings. The product claims hold:
- the walk fails closed;
- the token-level unsafe rule works and holds the exact allowlist;
- the three marker conditions work;
- pairing and the floors work;
- `run` and `main` pass the exit codes through.

All authorized-path limits hold. The MINOR items below are small. Root must rule on MINOR-1, because
#1443 depends on it.

## Findings

### MINOR-1: two ported cases lose code that #1443 D8 needs (one deviation is not recorded)
- `tools/realtime-policy/tests/policy/markers.rs:175-182` (`marked-file-count-floor`) deletes
  `crates/builtins/src/lib.rs` and creates it again as `fn builtins() {}`. The script's row deletes only
  the marker lines (`sed -i "/REALTIME_POLICY/d"`), so its code stays. The deleted code includes the
  file's four `try_pop` drains (`builtins/src/lib.rs.txt:14,26,44,56`). `run.rs:317-335` uses the same
  rewrite.
- `markers.rs:139-153` (`open-tail-valid`) leaves out the `oracle_drain` that the script puts at the top
  of every marked file. D10 describes that content ("The case also puts an unmarked `oracle_drain` ...;
  B2b-1 adds a class for it").
- #1443 D8 (`1443-...md:226-227`) says that `open-tail-valid`, `marked-file-count-floor` and
  `no-marked-files-uses-floor` "each gain {unmarked try_pop outside the control-side allowlist}". With
  this port, two of these three cases have no pop, so D8 cannot hold as written.
- The attempt record states the open-tail deferral. It does not state the `marked-file-count-floor`
  rewrite.
- The A1 claims are unchanged, because no rule in this slice reads the code. The defect is the
  hand-off to B2b-1: a B2b-1 implementer who works only from #1443's body will find D8 false.
- Fix: about 10 lines in test code, in the way that `the_floors_fire_when_no_file_is_marked` already
  filters marker lines.

### MINOR-2: a test doc comment states a claim the test does not make
`markers.rs:137-138` says "...and an unmarked drain above them is not part of any region". The case
has no drain. Root's standing ruling: every test-value claim must be true. Correct the comment, or
fix MINOR-1 by adding the drain.

### MINOR-3: one case does not assert the exact class set (D9)
`walk.rs:51-56` (`a_missing_realtime_module_is_a_finding`) asserts `classes().contains(...)`. D9 says:
"Each case asserts the exact set of finding classes." The tree also loses the realtime module's
regions, so the exact set must also include the floor findings.

### NIT-1: an empty statement is reported as Verbatim at line 1
For `;;`, syn 2.0.119 gives `Stmt::Expr(Expr::Verbatim(<empty>), Some(;))` (`syn/src/stmt.rs:162`).
`syntax.rs:284-289` reports it, and the empty stream's span gives line 1. I measured this on a probe
tree: two findings, both `buffer.rs:1: // REALTIME_POLICY_BEGIN`. The spec accepts over-refusal, but
the reported place is wrong. An empty Verbatim holds no tokens, so the tool can skip it safely.
Clippy's `redundant_semicolons` probably keeps such code off `main`.

### NIT-2: the macro case does not cover a macro invocation's token tree
A mutation that skips a group directly after `!` (`m! { ... }`) keeps all 50 tests green.
`unsafe_code_inside_a_macro_body_is_a_finding` covers only the body of a `macro_rules!` definition.
This matches D11's wording. A second row, `m! { unsafe {} }`, would close the gap.

### NIT-3: the non-directory, non-regular `.rs` branch has no test
`walk.rs:160-162` handles a FIFO or a socket that has a `.rs` name. If I delete the branch, all tests
stay green. D11 asks only for a directory named `x.rs`, and that case has a test.

### NIT-4: `main` is not one statement
`main.rs:8-11` adds a `current_dir()` failure branch. It exits 1 even when an absolute root is given,
and its message does not have the finding format. This is fail-closed and harmless, but it differs
from D1's text.

### NIT-5: `unsafe-outside-web-ffi` appends where the script's row overwrites
`unsafe_ownership.rs:63-69` appends. The claim is the same, and the class set is cleaner.

## Spec problems for root
1. **D5 count.** The spec says "14 paths". The real count is 19. #1489 and #1494 added five rows under
   STREAMS' named exception (row 115: "G #1494 ... one unsafe-allowlist row per new unsafe test file"):
   - `crates/lane/tests/fp_env.rs`
   - `crates/lane/tests/g6_ftz_inert.rs`
   - `crates/capi/src/runtime/tests.rs`
   - `crates/host-core/tests/fp_environment.rs`
   - `tools/wasm-gates/tests/g6_full_corpus_ftz.rs`

   The implementation follows D5's rule (copy `:29` as `main` holds it, minus the Amendment's five).
   I measured that the 19 entries are exactly the files the awk regex matches in the tree at the
   commit. The deviation is allowed. Update the number in the spec.
2. **D10 "Unsafe (17)".** The 17 include an exact duplicate: `unsafe-outside-exact-allowlist` and
   `unsafe-outside-rack-benchmark-main` have the same file and text (`test-realtime-policy.sh`). Dropping
   one follows the test-value rule. The deviation is allowed. The count is 16.
3. **`open-tail-valid` and `marked-file-count-floor` content versus #1443 D8** (MINOR-1). Root must
   choose one:
   - A1 ports the code (the `oracle_drain` prepend, and marker-only deletion), so #1443 D8 holds as
     written. I recommend this, because it is cheap.
   - #1443 is amended to say that B2b-1 adds that code.
4. **`Lit::Verbatim`.** I agree with the implementer that source text cannot produce it. syn builds it
   only for `(/*ERROR*/)` or for a literal that its parsers reject. I put 16 malformed literals through
   the proc-macro2 1.0.107 lexer (`0b2`, `0o9`, `"\x80"`, `c"\0"`, `'\u{D800}'`, `b"é"`,
   `"\u{110000}"`, `0x`, and others). It rejected each one that rustc rejects. Each one that lexed
   (`1e`, `1.0f`, `1__`) parsed with no `Lit::Verbatim`. No `visit_lit` arm is needed, and no spec
   change is needed.

## Implementer claims checked
| Claim | Result |
|---|---|
| Allowlist is 19 paths | True. It equals the files that match the awk regex at the commit. |
| Floors are 26 files and 93 regions | True. Gate 1: tool and awk both print `ok (93 marked regions in 26 files)`. |
| 562 tracked `.rs` files | True (`git ls-tree` at `7290d6b92`). |
| Base tree is `create_fixture`'s output | True. 36 files, byte-identical to the parent's `create_fixture` output. |
| One unsafe case dropped as a duplicate | True. Allowed (spec problem 2). |
| `open-tail-valid` drain deferred | True. Not allowed as D10 is written, and it conflicts with #1443 D8 (MINOR-1, spec problem 3). |
| No `Lit::Verbatim` arm | Justified (spec problem 4). |
| Parity: 82 trees, 19 differences | Not rerun in full. Spot checks: the base tree passes both gates; the Amendment mutation fails both gates. |
| libcapi identity unchanged | `cargo tree -e normal,features -p capi` is identical at parent and head after path normalisation (D12's allowed fallback). I did not rebuild the sha256 values. |
| About 70 extra mutations | I sampled about 11 of my own. All went red except the two survivors in NIT-2 and NIT-3. |

## Gate 4 mutations, rerun by me (each alone, then restored)
| Mutation | Tests that went red |
|---|---|
| `spsc.rs` removed from the allowlist | 42 red, including `the_base_tree_passes_with_its_counts`. Gate 1 also fails, with 10 `spsc.rs` findings. |
| Block-comment test dropped | `a_marker_line_inside_a_block_comment_is_malformed` |
| Group span treated as a leaf | 45 red. Gate 1 refuses the real markers at `builtins-compiler/src/lib.rs:540` and `:603`. |
| Missing root skipped | `a_missing_scan_root_is_a_finding`, `a_scan_root_that_is_a_file_is_a_finding`, `binary::a_root_with_a_missing_scan_root_exits_one` |
| Unlistable directory skipped | `a_directory_that_cannot_be_listed_is_a_finding` |
| Misordered markers accepted | `an_end_before_its_begin_is_misordered`, `two_begins_in_a_row_are_misordered`, `an_open_tail_is_misordered` |
| File floor lowered by one | 7 red, including `the_file_floor_fires_when_a_marked_file_loses_its_markers` |
| Region floor lowered by one | 5 red, including `the_region_floor_fires_when_a_region_is_dropped_and_the_markers_still_pair` |
| `run` returns 0 after findings | 4 red, including `findings_exit_one_and_print_one_line_each_on_stderr` |
| `main` ignores `run`'s code | both `binary::` cases |
| Amendment: `unsafe {}` in `crates/soft-clip/tests/allocation.rs` | Tool and awk both report it. Also checked for `tools/wasm-gate-guest/src/lib.rs`. |

My extra mutations, each with one red test unless stated:

| Mutation | Tests that went red |
|---|---|
| Leaf-spanning test (condition 2) dropped | the string-literal and doc-comment cases (2 red) |
| Marked files not parsed | `a_marked_file_that_does_not_parse_is_a_finding`, the Verbatim case (2 red) |
| `node_modules` not skipped | `target_and_node_modules_are_skipped` |
| `.rs` symbolic link accepted | `a_symbolic_link_named_rs_is_a_finding` |
| CR not stripped | `markers_with_crlf_line_endings_are_regions` |
| `Type::Verbatim` arm dropped | `syntax_the_parser_keeps_verbatim_is_a_finding` |
| Directory named `x.rs` not reported | `a_directory_named_like_a_source_file_is_a_finding` |
| Groups after `!` skipped | SURVIVED (NIT-2) |
| FIFO branch removed | SURVIVED (NIT-3) |

Extra marker probes on the release binary. The tool's verdict was correct in each:
- a doc comment, a block doc comment, or a `#[doc = "/*"]` directly before a marker;
- a comment that closes before the marker;
- a block comment opened after code;
- markers inside a macro's braces;
- a string that ends on the marker line;
- a tab indent, a double space and a lifetime before a marker.

## Test value (one sentence per new test)
- `valid::the_base_tree_passes_with_its_counts`: red if an allowlisted file or a valid marker is refused, or if the counts are summed wrongly.
- `valid::regions_with_empty_bodies_count`: red if a region with no lines between its markers is not counted.
- `run::a_valid_tree_exits_zero_and_prints_the_success_line`: red if the success path's exit code or the C10 line is wrong.
- `run::findings_exit_one_and_print_one_line_each_on_stderr`: red if findings exit 0, or if the line format or the stream is wrong.
- `run::findings_without_a_place_print_without_one`: red if a finding with no place is printed with stray separators, or exits 0.
- `run::a_relative_root_is_resolved_against_the_working_directory`: red if a relative root is resolved against the process directory and not against `cwd`.
- `run::no_argument_means_the_working_directory`: red if the default root is not `cwd`.
- `run::two_arguments_exit_two`: red if a bad argument list does not give exit 2.
- `run::findings_are_sorted_by_class_then_path`: red if findings come out in walk order and not sorted.
- `binary::a_root_with_a_missing_scan_root_exits_one`: red if `main` drops `run`'s failure code.
- `binary::two_arguments_exit_two`: red if `main` does not pass code 2 through.
- `unsafe_ownership::unsafe_code_outside_the_allowlist_is_a_finding`: red if the allowlist matches by prefix or directory, or if an unlisted file passes (16 ported rows).
- `unsafe_ownership::the_dropped_allowlist_entries_are_findings`: red if any of the Amendment's five entries comes back.
- `unsafe_ownership::a_finding_carries_its_file_line_and_source_line`: red if the reported line or source text is wrong.
- `unsafe_ownership::unsafe_code_inside_a_macro_body_is_a_finding`: red if the scan does not go into `macro_rules!` token trees.
- `unsafe_ownership::mentions_and_the_other_spellings_are_not_unsafe_code`: red if comments, strings, `unsafe trait` or `#[unsafe(..)]` are flagged.
- `markers::a_marker_written_in_a_string_literal_is_malformed`: red if a literal that spans a marker line is not seen (condition 2).
- `markers::a_marker_written_in_a_doc_comment_is_malformed`: red if line or block doc comments can carry markers.
- `markers::a_marker_line_inside_a_block_comment_is_malformed`: red if the block-comment test is gone or does not nest (condition 3).
- `markers::a_line_comment_opening_a_block_comment_text_does_not_open_one`: red if `/*` in a line comment opens a comment, which over-refuses.
- `markers::a_marker_after_code_is_malformed`: red if a trailing marker is accepted, or if its place is wrong.
- `markers::a_longer_word_or_a_bare_colon_is_malformed`: red if the word or the `: ` suffix is matched too loosely.
- `markers::indented_markers_and_markers_with_a_reason_are_regions`: red if an indented marker or a marker with a reason is refused.
- `markers::an_end_before_its_begin_is_misordered`: red if misorder is accepted, or if its line is wrong.
- `markers::two_begins_in_a_row_are_misordered`: red if equal counts in nested order are accepted.
- `markers::an_open_tail_is_misordered`: red if the open-tail allowance comes back, or if a misordered file keeps its regions.
- `markers::unmatched_markers_name_the_file_and_drop_it_from_the_floors`: red if unequal counts pass, or if the file still counts.
- `markers::the_file_floor_fires_when_a_marked_file_loses_its_markers`: red if the file floor is off by one or missing.
- `markers::the_floors_fire_when_no_file_is_marked`: red if zero marked files passes.
- `markers::the_region_floor_fires_when_a_region_is_dropped_and_the_markers_still_pair`: red if the region floor is off by one or missing.
- `markers::a_marker_after_multibyte_text_is_still_a_marker`: red if columns are counted in bytes.
- `markers::markers_with_crlf_line_endings_are_regions`: red if `\r` is not stripped.
- `markers::a_final_marker_without_a_newline_is_a_marker`: red if the last line is lost when the file has no final newline.
- `walk::a_missing_scan_root_is_a_finding` and `a_scan_root_that_is_a_file_is_a_finding`: red if a missing root, or a root that is not a directory, is skipped.
- `walk::a_missing_realtime_module_is_a_finding`: red if the C2 check is removed (but see MINOR-3).
- `walk::a_directory_that_cannot_be_listed_is_a_finding`: red if a `read_dir` error is skipped.
- `walk::a_symbolic_link_to_a_directory_is_a_finding` and `a_symbolic_link_named_rs_is_a_finding`: red if those links are followed or ignored.
- `walk::a_link_to_a_file_with_another_name_is_ignored`: red if harmless links are refused.
- `walk::a_directory_named_like_a_source_file_is_a_finding`: red if a directory with a `.rs` name is not reported.
- `walk::a_source_file_that_is_not_utf8_is_a_finding`: red if the text is decoded lossily.
- `walk::a_source_file_that_cannot_be_read_is_a_finding`: red if a read error becomes a skip.
- `walk::a_source_file_that_does_not_lex_is_a_finding`: red if a lex error is ignored, or if its line is wrong.
- `walk::a_marked_file_that_does_not_parse_is_a_finding`: red if marked files are not parsed.
- `walk::the_c_abi_entry_file_that_does_not_parse_is_a_finding`: red if `ffi.rs` is not always parsed.
- `walk::syntax_the_parser_keeps_verbatim_is_a_finding`: red if any of the eight Verbatim arms or the visitor descents is dropped.
- `walk::target_and_node_modules_are_skipped`: red if either skip is lost.
- `walk::a_directory_with_a_lookalike_name_is_scanned`: red if the skip matches by prefix.
- `walk::nested_files_under_every_root_are_found`: red if the walk does not recurse, or does not cover one of the roots.

## Gates run
1. `cargo run --locked --release -q -p realtime-policy` printed `realtime policy: ok (93 marked regions in 26 files)`, exit 0. `bash scripts/check-realtime-policy.sh` printed the same.
2. `cargo test --locked -p realtime-policy`: 50 passed. No case tree was left behind (each `Tree` removes itself when it is dropped).
3. Parity: spot checks only (see above).
4. Mutations: see above.
5. Real tree: 0.755 s (release).
6. `cargo tree -e normal,features -p capi`: identical at parent and head.
7. All of these exit 0:
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked -p realtime-policy --all-targets -- -D warnings`
   - `check-workspace-policy.sh`
   - `check-realtime-policy.sh`
   - `test-realtime-policy.sh`
   - `check-ci-path-routing.py`
   - `test-ci-path-routing.py`
   - `check-script-reachability.py` (in the worktree; 120 files)
   - `check-bench-policy.sh`
