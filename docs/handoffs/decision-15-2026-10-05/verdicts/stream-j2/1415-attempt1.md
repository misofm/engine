PASS

# #1415 attempt 1 verdict: spell the route gain and matrix domain once, in the session model

Commit under review: `50b6d4482` (parent `ffdbacea2`), branch `codex/d15-stream-j2`, worktree
`/home/bl/misofm/wt-d15-j2`. Judged against the spec as corrected at `07dedd828` (gate 2 runs
`cargo run --locked -p graph-compiler --bin graph_fixture -- --check`). Verifier: opus-xhigh,
2026-10-05. Everything below was built and run from an export
(`git archive 50b6d4482` into `/tmp/claude-1002/v1415/tree`, `CARGO_TARGET_DIR=/tmp/claude-1002/v1415/target`).
The worktree was not edited, built or checked out.

## Requirement check

- **D1 (one definition).** `crates/session/src/model.rs:780-797` adds `ROUTE_GAIN_DB_MINIMUM = -144.0`,
  `ROUTE_GAIN_DB_MAXIMUM = 24.0` and `ROUTE_COEFFICIENT_MAGNITUDE_MAXIMUM = 1.0`, `pub`, just before
  `Route`. Each doc comment names #1237 and owner question Q2 of decision 13. They are re-exported by
  `pub use model::*` (`crates/session/src/lib.rs:28`). Met.
- **D2 (both readers use it).** The private copies in `validate.rs` and `ids.rs` are gone.
  `validate_routes` imports the model constants (`validate.rs:5`, read at `:712-713` and `:737-738`).
  `route_values` reads `session::ROUTE_*` (`ids.rs:325-330`). The statements that lived on the deleted
  constants now sit on `validate_routes` (`validate.rs:689-692`) and `route_values` (`ids.rs:312-315`)
  and say that both paths read the session constants. Met. After the change there is exactly one Rust
  definition of the route domain in the workspace (searched every `.rs` file for the three names and
  for `-144.0`; the other `-144..=24` literals are the fader, trim and VCA domains, which are
  non-goals). The live callers (`host-core` `route_controls.rs:89`, host-web send admission through
  it) reach the domain only through `route_coefficients` -> `route_values`.
- **D3 (test keeps its literals).** `crates/graph-compiler/tests/route_coefficients.rs` is unchanged.
  Met.
- **Authorized paths.** Changed files: the spec, `crates/session/src/model.rs` (three constants and
  their docs only), `crates/session/src/validate.rs` (import, deleted constants, `validate_routes` doc),
  `crates/graph-compiler/src/ids.rs` (deleted constants, `route_values` body and doc). In
  `crates/graph-compiler/src/*` only `ids.rs`'s route constants and their reader moved, as STREAMS.md
  line 67 allows. Met.
- **Hazard (glob-import clash).** No crate glob-imports `session::*`; no other definition of the three
  names exists. The attempt record's claim is correct.
- **No behaviour change.** Confirmed at the byte level: the shipped AudioWorklet module built from the
  parent (`d4cf86ea...`) and from the head (`c4d170dd...`) are the same size and differ in exactly 7
  bytes, every one the line field of a `core::panic::Location` record in the data section (file
  length 32 = `crates/graph-compiler/src/ids.rs`, lines 392/394 -> 389/391; file length 30 =
  `crates/session/src/validate.rs`, five lines +1). No code byte moved. See MINOR-1 for what this
  means for CI's `artifact-identity` report.
- **Realtime, naming, acked-batch.** No render path is touched (`route_values` runs at compile and in
  the control producer). The new names are unversioned internal-domain constants, as the version-suffix
  rule wants. No queue is touched, so an ack cannot precede a drop here.
- **Owner principle.** The change removes the duplicate outright instead of adding a guard; no interim
  shortcut.

## Findings

### BLOCKER

None.

### MAJOR

None.

### MINOR

- **MINOR-1: the worklet chain and the cross-target check were not run or recorded, and the attempt
  record does not say the shipped module changes.** `session` and `graph-compiler` are compiled into
  the browser module (host-web -> host-core -> graph-compiler), and the coordinator's worker rules ask
  for the worklet chain when browser-compiled code changes and `scripts/check-cross-targets.sh` when
  engine code changes. The attempt record (spec, "Attempt record") lists neither. The change moves 7
  bytes of panic-location line numbers in the shipped module, so CI's `artifact-identity` job will
  report ARTIFACT CHANGED for this change (an information line, not a failure; the release pin is
  not touched). I ran both and they are green (see "Gates run"), so this is an evidence gap, not a
  defect. Fold-in: one line in the attempt record that names the module digests and the cause of the
  change, and the two gate results.

### NIT

- **NIT-1: the decided values are still written out in prose.** `crates/graph-compiler/src/ids.rs:310`
  (`route_values`), `:341` (`RouteValueError::Domain`), `:351-352` (`route_coefficients`) and
  `crates/graph/src/lib.rs:784-785` restate `[-144, 24]` dB and `[-1, 1]`, and the SDK comment
  `sdk/src/core/session.ts:324` still calls the bound "the engine's `validate_routes` bound". A
  future domain change still needs a prose sweep. The tests catch a code drift, not a prose one.
  Only `route_values`' doc was in this slice's authority (D2 limits doc edits to the comments that
  named the other copy), so this is a candidate follow-up: link the public docs to the constants.
- **NIT-2: "spelled once, here" (`crates/session/src/model.rs:782`) is true for Rust only.** The SDK
  keeps a TypeScript mirror (`ROUTE_GAIN_DB_DOMAIN`, `ROUTE_COEFFICIENT_DOMAIN`,
  `sdk/src/core/session.ts:325-327`), a declared non-goal that `sdk/test/builder-evals.mjs` holds at
  boot. One clause naming that mirror would tell the next person who changes the domain where the
  second spelling is.

## Test value

No new or rewritten test (the spec says so). The existing test
`a_route_value_is_in_domain_live_exactly_when_the_session_accepts_it` keeps its value. It is red if
the live path and the session validator disagree at a boundary step. Mutation C below proves this:
a one-sided drift of the live gain maximum to `24.5` turns it red at `route_coefficients.rs:397`. It
is also red if the decided domain moves (mutations A and B). The session-side
`crates/session/tests/route_domain.rs` also goes red on mutation A.

## Mutation runs (redone by the verifier)

Command: `cargo test --locked -q -p graph-compiler --test route_coefficients
a_route_value_is_in_domain_live_exactly_when_the_session_accepts_it`.

- Baseline: ok (1 passed).
- A, `session::ROUTE_GAIN_DB_MAXIMUM = 24.5`: red at `route_coefficients.rs:414:5`,
  `left: 33, right: 35` (refused count). Same as the attempt record. Under the same mutation
  `cargo test -p session --test route_domain` is red at `route_domain.rs:102:13`.
- B, `session::ROUTE_COEFFICIENT_MAGNITUDE_MAXIMUM = 0.5`: red at `route_coefficients.rs:347:44`,
  `fixture parses: DiagnosticSet([... NumericOutOfSchemaRange ... channel_matrix ll ...])`. Same as
  the attempt record.
- C (verifier's own, one-sided drift): `route_values`' upper gain bound replaced by the literal
  `24.5_f32`, the session constant unchanged: red at `route_coefficients.rs:397:17`,
  `$.routes[0].gain_db = 24.000002 ... live Ok([15.848936, ...]), session Some("numeric.out_of_schema_range")`.
- After each mutation the file was restored and checked identical to the commit's blob: ok (1 passed).

## Gates run (at `50b6d4482`, from the export)

Gate 1: the mutation runs above.

Gate 2:
- `cargo test --locked -p session` -> 0
- `cargo test --locked -p graph-compiler` -> 0 (164 passed, 0 failed, 2 ignored)
- `cargo test --locked -p host-core --features control-provider,test-support` -> 0
- `cargo test --locked -p host-web --features test-support` -> 0
- `cargo run --locked -p graph-compiler --bin graph_fixture -- --check` -> 0
- `bash scripts/check-graph-determinism.sh` -> 0, "PASS (100/100)". The script hard-codes
  `$workspace/target/debug/graph_fixture` and ignores `CARGO_TARGET_DIR` (an existing limitation, not
  part of this change). The first run exited 127 for that reason. The export's `target` was then
  symlinked to the verifier target dir and the script was run again.

Gate 3:
- `cargo fmt --all -- --check` -> 0
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` -> 0
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` -> 0
- `bash scripts/check-session-policy.sh` -> 0 ("session policy: ok")
- `bash scripts/check-workspace-policy.sh` -> 0 ("workspace policy: ok")

Extra (coordinator worker rules; see MINOR-1):
- `scripts/build-web-audioworklet.sh --module-only` at the parent and at the head -> shipped module
  `d4cf86ea0fe3523a560a4e2bc0eefd738435c5c7f977b0ae4b47d37cc02cb9fa` -> `c4d170ddb3563090d5bec09edf369d380a42b888a926ad1fef34547d7a2a72c5`,
  2,896,157 B each, 7 bytes differ (panic-location line numbers only).
- `scripts/build-web-audioworklet.sh --named-twin` (head) -> 0, reproduces `c4d170dd...`; named twin
  `dbe157afd9341de9cdb6b0a4d0e8cacbf226f0f8746533d9c3c2345e11fd146d`.
- `scripts/check-web-audioworklet.sh --without-metadata-regeneration <artifacts> <named twin>` -> 0
- `python3 -B scripts/check-browser-expected-resources.py --artifacts <artifacts>` -> 0
- `bash scripts/test-web-audioworklet.sh` -> 0
- `bash scripts/check-cross-targets.sh` -> 0 ("cross-target matrix: PASS", the #1018 iOS memset rows
  are its expected failures)
