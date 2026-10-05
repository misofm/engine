VERDICT: PASS

# #1235 attempt 1 -- adversarial verdict

Commit `3bf212cad` on base `9edd1a6b8` (stream J, decision 15). Reviewed in an export of the commit
(`/tmp/claude-1002/v1235/src`) and of the base (`/tmp/claude-1002/v1235/base`), target dir
`/tmp/claude-1002/v1235/target`. The worktree was not touched.

## Scope and invariants

- Paths: only `crates/builtins-compiler/src/lib.rs` and the spec's own attempt record. Both authorized.
- Deliverable 1: `TestOnlyInitialMatrixTrace`, `EMPTY_INITIAL_MATRIX_TRACE`, the
  `SCALAR_INITIAL_MATRIX_TRACE` thread-local, `record_initial_matrix_state`, `initial_matrix_state`
  and the call site in `strip_bindings` (lib.rs:869, 889, 904, 980, 1003, 2315) are now
  `cfg(all(test, feature = "test-support"))`. That is exactly the gate of the three callers
  (lib.rs:8241 and 8378 in `..._intervening_observer_error_...` and `..._failed_render_...`, 8475;
  tests gated at 8126 and 8293). `record_initial_matrix_state` has no side effect beyond the
  thread-local and cannot panic, so dropping it from no-feature test builds changes nothing anything
  reads.
- Deliverable 2: `BoundaryVariant::{Nonadjacent, NonadjacentOutputConflict}` (lib.rs:6075, 6078) are
  gated like every constructor (tests at 7818, 8001, 8126, all `cfg(all(test, feature =
  "test-support"))`). `NonadjacentTrackA` stays ungated; ungated tests construct it.
- No production change: `strip_bindings` is itself `cfg(any(test, feature = "test-support"))`, and the
  narrowed statement was `cfg(test)`, so in every non-test build (with or without the feature) it is
  absent before and after. `mod tests` compiles in non-test `test-support` builds (lib.rs:5123); there
  the two variants now disappear, but nothing in that configuration constructs them, so no reachable
  behaviour moves. Confirmed by `cargo clippy -p builtins-compiler --lib --features test-support` and
  gate 2 compiling that configuration.
- No `allow`/`expect(dead_code)` added (diff checked).

## The four `matches!` -> `match` rewrites (semantic identity)

Or-pattern alternatives cannot carry `cfg`, so splitting the gated variants into their own
`cfg`-attributed arm is the minimal correct form, and deliverable 2 explicitly allows gating the arms.

| Site | Before | After, feature on | After, feature off |
| --- | --- | --- | --- |
| 6172 `output_track` | `TrackA \| OutputConflict` -> `n-1`, else `0` | `TrackA`->`n-1`, `OutputConflict`->`n-1`, `_`->`0` | `TrackA`->`n-1`, `_`->`0` (OutputConflict absent) |
| 6941 schedule order | `Nonadjacent \| TrackA \| OutputConflict` -> true | same three -> true, `_`->false | `TrackA`->true, `_`->false |
| 6998 `selected_index` | `TrackA \| OutputConflict` -> `0`, else `n-1` | `TrackA`->0, `OutputConflict`->0, `_`->`n-1` | `TrackA`->0, `_`->`n-1` |
| 7019 pair assertions | same as 6941 | same as 6941 | same as 6941 |

Each is the same function of `variant` on every constructible variant in both configurations. The one
real risk of the form -- a `cfg` arm that silently compiles out and lets `_` swallow the variant -- I
checked empirically (mutations below): the gated arms are live with the feature on (M2, M4 red), the
ungated arms are live without it (M6, M8 red), and the narrowed call site is live with the feature on
(M9 red). All four arms carry the identical predicate spelling, so M1/M3's arms are live by the same
token even though the suite cannot see them (see Observations).

## Findings

BLOCKER: none.

MAJOR: none.

MINOR: none.

NIT-1 -- duplicated predicates and the `if match ... {} {` idiom. `crates/builtins-compiler/src/lib.rs:6941-6946`
and `:7019-7024` are the same "is nonadjacent" match; `:6172-6177` and `:6998-7003` share the
"track A is the output/selected track" predicate. Fix (optional): give `BoundaryVariant` two small
methods, e.g. `fn is_nonadjacent(self) -> bool` and `fn track_a_leads(self) -> bool`, each holding one
`cfg`-attributed arm, and call them at the four sites. That halves the `cfg` arms and removes the
`if match` form. The spec's "smallest edit" preference makes the current form acceptable.

## Observations outside this issue (pre-existing; recommend a follow-up issue, not a finding here)

The existing suite cannot see two of the predicates at all, on base as on head:

- M1 (`NonadjacentOutputConflict => 0` instead of `n - 1` at lib.rs:6175) and the base equivalent B1
  (drop `| NonadjacentOutputConflict` at base lib.rs:6172) both stay green. The #916 test
  `actual_scalar_nonadjacent_output_track_takes_the_split_pair_now_the_output_is_dedicated`
  (lib.rs:8003) claims its second fixture makes t01 the output track, but it passes unchanged when t00
  is the output track, so it never verifies its own premise. Fix in a follow-up: assert in that test
  (or in `track_graph_variant`) that the `Output` edge's source is t01's `PostMatrix` for
  `NonadjacentOutputConflict`.
- M3/B3 (`selected_index` for `NonadjacentOutputConflict`) and M5/B5, M7/B7 (`NonadjacentTrackA`'s
  `output_track` / `selected_index`, no features) also survive. For `n == 2` and a stage-major
  schedule, both orientations satisfy the slot assertions at lib.rs:7036-7068 by construction, so the
  orientation is unobservable.

Neither was introduced by this attempt (base mutants B1, B3, B5, B7 survive identically), and the spec
forbids test changes here.

## Gates run (in the export)

| Gate | Base `9edd1a6b8` | Head `3bf212cad` |
| --- | --- | --- |
| 1. `cargo clippy --locked -p graph -p builtins-compiler --all-targets -- -D warnings` | exit 101: `function initial_matrix_state is never used` (lib.rs:1004), `variants Nonadjacent and NonadjacentOutputConflict are never constructed` (lib.rs:6075) | exit 0 (only the two out-of-scope `clippy.toml` fast-dB warnings) |
| 1b. `cargo clippy --locked -p builtins-compiler --all-targets --features test-support -- -D warnings` | -- | exit 0 |
| 1c. `cargo clippy --locked -p builtins-compiler --all-targets -- -D warnings` | -- | exit 0 |
| 1d. `cargo clippy --locked -p builtins-compiler --lib [--features test-support] -- -D warnings` | -- | exit 0 both |
| 2. `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | -- | exit 0 (148 crates checked, builtins-compiler included) |
| 3a. `cargo test --locked -p builtins-compiler --features test-support` | list: 78 names | exit 0; 76 passed, 2 ignored (lib 54); sorted `--list` identical to base |
| 3b. `cargo test --locked -p builtins-compiler` | list: 58 names | exit 0; 57 passed, 1 ignored (lib 47); sorted `--list` identical to base |
| 4. `cargo fmt --all -- --check`; `bash scripts/check-workspace-policy.sh` | -- | exit 0; `workspace policy: ok` |
| `git diff --check 9edd1a6b8..3bf212cad` | -- | clean |

The implementer's recorded gate results match mine.

## Mutations (lib tests, `cargo test -p builtins-compiler --lib`)

| Mutant (head) | Features | Result |
| --- | --- | --- |
| M1 6175 `OutputConflict => n - 1` -> `0` | test-support | green (pre-existing gap, B1 identical) |
| M2 6944 gated `=> true` -> `false` | test-support | RED: 3 nonadjacent tests |
| M3 7001 `OutputConflict => 0` -> `n - 1` | test-support | green (pre-existing gap, B3 identical) |
| M4 7022 gated `=> true` -> `false` | test-support | RED: 3 nonadjacent tests |
| M5 6173 `TrackA => n - 1` -> `0` | none | green (pre-existing gap, B5 identical) |
| M6 6942 `TrackA => true` -> `false` | none | RED: 2 tests |
| M7 6999 `TrackA => 0` -> `n - 1` | none | green (pre-existing gap, B7 identical) |
| M8 7020 `TrackA => true` -> `false` | none | RED: 2 tests |
| M9 2316 call site disabled | test-support | RED: the 2 tests that read `initial_matrix_state` |

Export restored and byte-compared to the commit; post-restore lib run green (54 passed).

## Test value

No test was added or rewritten (the four rewrites are harness expressions whose meaning did not
change), so no test-value sentence is owed. The defect's reproducer is gate 1: red on base, green on
head, reproduced above.
