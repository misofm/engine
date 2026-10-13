PASS

# #1439 attempt 1 verdict (A2: region boundaries, forbidden-body predicate, whole-plan check)

Range: parent `e5fa8319b`, head `f965c5a1c` (`6eb40118d 1ba9241fe 888d979fd 70e8ac837 f965c5a1c`), branch
`codex/d15-stream-j3`. I built and tested from `git archive f965c5a1c` in `/tmp/claude-1002/v1439/tree` with
`CARGO_TARGET_DIR=/tmp/claude-1002/v1439/target`. I did not touch the worktree, except for one read-only
`check-script-reachability.py` run, which needs git. The worktree HEAD was `f965c5a1c` at that time.

There are no BLOCKER and no MAJOR findings. The product claims hold:
- D1 refuses a marker inside a node, inside an attribute, and a region that crosses a macro token tree. It
  accepts the real tree's macro-body region, the attribute-gap region and the two regions that span an `impl`.
- D2 renders the tokens. It catches the turbofish (#948), split and spaced spellings, `async{` and spellings in
  macro arguments, and it ignores comments and literals.
- D3 scans the items after `mod tests`, macro tokens and `cfg(any(test, ..))` modules. It passes a call, an index, a
  path, a field, a `&raw` form and another name.

Every write is inside the authorized paths: `tools/realtime-policy/**`, this spec, and the Evidence section of
the #948 spec. Nothing else changed against the parent.

## Findings

### MINOR-1: the pattern table has no test for each pattern; the `forbidden-await` row tests `async`, not `.await`
- `src/forbidden.rs:17-55` holds the 37 spellings. No test pins each spelling.
  - Mutation M8 changes `".await"` (`:45`) to a dead string. All 77 tests stay green, and gate 1 stays green.
  - Mutation M9 does the same to `"Tcp"` (`:47`), with the same result.
  - Most other rows have no test either. So a deleted or mistyped pattern turns off a forbidden surface, and
    no test or gate fails.
- `tests/policy/forbidden.rs:173` (`forbidden-await`, `async fn probe() { other().await; }`) goes red only
  because of the `async` keyword rule. The row name says `.await`, but `.await` is not what this row tests.
  An `.await` alone is possible: the region can open inside an `async fn` body. My probe `x-await-only` shows
  that the tool refuses that shape today, but no committed test holds it.
- After C2 deletes the awk gate, the tool is the only owner of these spellings. The awk self-test had the same
  gap, and the spec's case list (D4, D5) is met. So this is MINOR, not a gate failure.
- Fix (small): add one table test with one row for each pattern and one for `async`. Each row is a body that
  holds only that spelling. Also make the `forbidden-await` row a region inside an `async fn` body.

### MINOR-2: a parity difference that gate 3 does not name (`#[cfg(test)]` with another attribute on `mod tests`)
- The awk stops the C9 scan only at a `#[cfg(test)]` line that comes directly before `mod tests {`. The tool
  (`src/whole_plan.rs:48-60`) skips a top-level `mod tests` when any of its outer attributes is exactly
  `#[cfg(test)]`. This is what D3 says.
- My probe `x-cfg-test-plus-allow` adds `#[allow(dead_code)]` between the two lines in the base `ffi.rs`. The awk
  fails (it scans the fixture's `&*plan`), and the tool passes.
- The tool is narrower than the awk here, but only for test code. This is not a production false pass.
- Gate 3 asks for a named case for each deliberate difference, and the attempt record does not list this one.
  Its note says that no tree separates the two gates for D3's test-module scope, but this tree does.
- Fix: record this in the attempt record's gate-3 table. If you want to pin it, add a case that passes.

### NIT-1: the "region nodes" type is not built
- A2-D1 defines region nodes, but no A2 gate, test or rule reads them. B1a-D1 is the first consumer.
- If A2 built the type with no reader, it would be dead code, and `clippy -D warnings` would refuse it.
- I accept the deviation. See the spec note for root below.

### NIT-2: the run-time framing in the gate-5 record
- I measured 1.01 s three times at load average 2.0. The awk measured 0.59 s.
- So the extra 0.4 s does not come from the "loaded machine" (load 6.5) that the record mentions. It comes from
  the D1 and D2 passes, as the record itself goes on to say.
- Gate 5 only records the time (A1 gate 5), so this does not fail the attempt. Remove the load framing from
  the record.

## Deviations judged
- **`Policy::always_parsed` removed.** Accepted. A1's spec lists the fields of `Policy` (the allowlists and the
  floors) and does not list this one. D3 fixes the path, so a constant (`whole_plan::CAPI_FFI`) is the honest
  form. This change also removes one public field that only one value ever used. The `walk-ffi-parse-error`
  case still holds.
- **D2 also runs when a marked file does not parse.** Accepted. This makes more findings and fails closed, and
  the parse error is always reported as its own finding.
- **D1 names the outermost statement or item.** Accepted. The spec asks for "a <node kind>" and does not ask for
  the innermost node. The name comes from the child that the marker splits, which is the rule's own unit.
- **Real-tree line numbers.** Confirmed at the head:
  - gate-expander `:1022`/`:1041`;
  - transient-shaper `:613`/`:726`;
  - effect-contract `:231`/`:537`;
  - rack `:628`.
  The mutations confirm the claim "one macro-tree region and two cross-container regions":
  - M5 refuses gate-expander `:1022` and `:1041` only;
  - M18 refuses effect-contract `:231` and transient-shaper `:613` only.
- **Pattern count.** The awk's `:80-82` has 37 alternatives: 36 literals and `async[[:space:]]`. The tool has 37
  literals: the 36 and `.collect::<`. It also has the `async` keyword token. This matches D2.
- **Run time 1.01 s.** Gate 5 only records the time (see NIT-2).

## Test value (one sentence per new test)
- `boundary::a_marker_inside_a_node_is_refused`: red if a marker inside a match, call, struct, enum, parameter
  list, let, if/else, method chain, impl header, struct expression or closure parameter list is accepted (M3).
- `boundary::either_marker_alone_can_split_a_node`: red if only one marker of a region is judged, or if the
  finding sits on the wrong marker line.
- `boundary::a_marker_inside_an_attribute_is_refused`: red if a marker inside an attribute that comes before
  its item is accepted (the implementer's attribute-check mutation; M6 also turns it red).
- `boundary::a_marker_between_attributes_and_their_item_is_a_boundary`: red if the attribute rule is dropped
  (M6) or skips only one attribute (M17; gate 1 also refuses rack `:628`).
- `boundary::a_marker_between_the_children_of_a_container_is_a_boundary`: red if one container kind is missing.
  M19 (no `extern` container) turns only this test red.
- `boundary::a_region_may_span_the_end_of_an_impl`: red if any change of container is refused (M18; gate 1 is
  also red).
- `boundary::a_region_inside_a_macro_body_is_tokens_and_passes`: red if macro groups are not containers (M5;
  gate 1 is also red).
- `boundary::a_region_that_crosses_a_macro_token_tree_is_refused`: red if the macro-boundary check is dropped
  (M4). Only this test goes red.
- `boundary::a_boundary_finding_names_the_marker_line`: red if a split finding names the wrong line or text (M3).
- `forbidden::a_forbidden_spelling_in_a_region_is_a_finding`: the nine ported script rows. Red if the
  predicate misses a marked region anywhere (for example under `tools/`, or outside the realtime root).
- `forbidden::a_spelling_the_line_scan_missed_is_a_finding`: red if macro tokens are skipped (M1), if the
  predicate reads line text (M2), if the turbofish is dropped (M14, #948) or if the `async` rule is dropped
  (M16).
- `forbidden::a_finding_names_the_line_where_the_spelling_starts` and `findings_are_one_per_line`: red if the
  predicate reads line text (M2) or if a finding is put on the wrong line or reported for each match.
- `forbidden::a_forbidden_word_in_a_comment_or_a_literal_passes`: red if literals are rendered as text (M13) or
  if line text is matched (M2).
- `forbidden::a_longer_identifier_still_matches`, `adjacent_words_do_not_join_into_a_spelling` and
  `only_the_tokens_between_the_markers_are_read`: red if word boundaries are added, if words are joined, or if
  the scan reads the whole file or only the first region. These are the implementer's mutations, and I did
  not re-run them.
- `whole_plan::every_form_of_a_whole_plan_borrow_is_a_finding` and `layout_does_not_hide_a_whole_plan_borrow`:
  the ported five forms and the two layout cases.
- `whole_plan::a_missing_file_or_a_render_entry_only_in_tests_is_a_finding`: red if the render-entry check is
  dropped (M12).
- `whole_plan::a_whole_plan_borrow_after_the_test_module_is_a_finding`: red if the scope stops at
  `mod tests` (M7).
- `whole_plan::only_a_cfg_test_module_named_tests_is_skipped`: red if any `cfg` skips the module (M11).
- `whole_plan::a_whole_plan_borrow_inside_a_macro_is_a_finding`: red if the macro-token rule is off (M15) or if
  the scope stops at `mod tests` (M7).
- `whole_plan::borrows_that_are_not_the_whole_plan_pass`: red if a call, index, path, field, `&raw`, longer name
  or double dereference is flagged.

## Gates run
1. Real tree: `realtime_policy` (release) prints `realtime policy: ok (93 marked regions in 26 files)`, and
   `bash scripts/check-realtime-policy.sh` prints the same. Both exit 0.
2. `cargo test --locked -p realtime-policy`: 77 passed.
3. Parity: I did not re-run all 185 trees. The permission system denied my temporary patch to the harness's
   `Tree::drop` (to keep the case trees), so I did not try that route again. Instead I built my own trees from
   `tests/fixtures/base` in my scratchpad (`scratchpad/parity/run.py`), and ran the awk gate and the release
   binary on each:
   - 41 trees that copy the committed cases (D1, D2, D3 and the ported rows). Each outcome matches the attempt
     record's table: the same results, and the same deliberate differences in the same direction.
   - 20 adversarial probes. Two are shared gaps that the tool keeps, as the spec allows: `<Vec<u8>>::new()` and
     `let ref whole = *plan`. One is the undocumented difference in MINOR-2. All the others are refusals that
     the spec intends (a `where` clause, a `const` initializer, the gap between a signature and its body, a
     tuple struct, an inner attribute, a crossing from a macro into a block, a test module nested in another
     module), or passes (a closure block in a call, a let-else block, a region from a then-block to an
     else-block).
4. Mutations: I applied each one alone and then restored the file. I confirmed that the restored files match
   the head.
   - All seven of the spec's mutations go red as the record says: M1, M2, M3, M4, M5 (gate 1 refuses
     gate-expander `:1022` and `:1041`), M6 (gate 1 refuses rack `:628`) and M7.
   - These mutations also go red: M11-M19, except M8 and M9 (see MINOR-1).
5. Run time: 1.01 s (three runs) against 0.59 s for the awk gate. The gate only records the time.
6. `git diff e5fa8319b f965c5a1c` touches nothing outside `tools/realtime-policy` and the two specs.
   `cargo tree -e normal,features -p capi` does not name `realtime-policy`. I did not rebuild the `libcapi`
   digests: no input of `capi` changed, and `Cargo.lock` did not change.
7. These all exit 0 on the export:
   - `cargo fmt --all -- --check`;
   - `cargo clippy --locked -p realtime-policy --all-targets -- -D warnings`;
   - `check-workspace-policy.sh`, `check-bench-policy.sh` and `check-realtime-policy.sh`;
   - `test-realtime-policy.sh` (36 s);
   - `check-ci-path-routing.py` and `test-ci-path-routing.py`;
   - `check-script-reachability.py` (run in the worktree).

#948 (D6): the turbofish case is confirmed. `forbidden-turbofish-collect` fails the tool and passes the awk.
M14 turns it red. Gate 1 passes the workspace. `qualification.yml:766-769` builds and runs `realtime_policy`.
The D6 GitHub steps (3-5) are still to do after the batch push.

## Notes for root (spec)
- B1a-D1 says "A region's nodes are A2-D1's maximal nodes", as if A2 delivered them. A2 did not build them. The
  container layout in `boundary.rs` is the place to build them. Amend B1a so that it says B1a builds the node
  set, and so that a B1a implementer who works only from its body does not look for an existing type.
- Gate 3's list of D3 differences does not include a `#[cfg(test)] mod tests` that has another outer attribute
  (MINOR-2).
