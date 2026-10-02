# #1199 *Declare the submix strip in the session grammar and wire*: Sol verdict, attempt 1

- Reviewed: `git diff 5abde384 27892bcc` (branch `codex/batch-submix-k1`, worktree
  `/home/bl/misofm/wt-submix-k1`). That is 26 files, +979/-116.
- Binding: `AGENTS.md` and `.github/ISSUE_SPECS/1199-declare-the-submix-strip-in-the-session-grammar-and-wire.md`,
  including its "Attempt 1 record".
- How I ran it: the #1200 implementer is editing the worktree, so I touched nothing there, not the
  branch and not GitHub. I exported `27892bcc` with `git archive` to `/tmp/claude-1002/v1199/src` and
  built there with my own `CARGO_TARGET_DIR`.
- Mutations: I applied them only to that scratch copy. I restored every file and confirmed it
  byte-equal to the commit (`cmp` against `git show 27892bcc:<path>`) before running any later gate.
  I removed the scratch tree afterwards.

## Verdict: PASS

There is no BLOCKER and no MAJOR.
- Every deliverable is present, and every objective gate passes when I re-run it.
- The wire amendment adds field IDs and renumbers nothing. Field 3 is left free for `console`.
- The hash re-pin is a legitimate wire-format digest, and the reason for it is written down.
- Each new test fails under a defect it is meant to catch: I ran five mutations to check.

There are two MINOR findings. Both are hand-offs that the root should write into later specs.
Neither needs another attempt.

## Gates (re-run by me on `27892bcc`)

| Gate | Command | Result |
|---|---|---|
| 1-3, focused | `cargo test --locked -p session -p protocol -p graph-compiler -p host-core --all-targets --features protocol/test-support,host-core/test-support` | exit 0; 45 binaries, 478 passed, 0 failed. All 7 new tests ran and passed. |
| 4 | the spec's DSP/conformance command (`-p lane … -p conformance --features math/lane,parametric-eq/test-support,builtins/test-support,lane/test-support`) | exit 0; 145 binaries, 787 passed, 0 failed |
| 4 | `bash scripts/check-protocol-wasm-parity.sh`, then again with `--self-test` | `ok (simd128)`. Self-test: the control is green, and all 3 red rebuilds plus the inert-invocation row are refused, so the new search strings match. |
| 5 | `cargo build --locked --release -p audit -p bench -p capi -p session-validator` | exit 0 |
| 5 | `check-graph-determinism.sh` | PASS (100/100) |
| 5 | `cargo run --locked -p graph-compiler --bin graph_fixture -- --check` | exit 0 |
| 5 | `check-console-fixtures.sh target/release/session_validator` | ok |
| 5 | `check-builtins-fixtures.sh . target/release/audit` | ok (50 files) |
| 6 | the test-debug-a workspace command, with the exact excludes and features from `qualification.yml` | exit 0; 97 binaries, 1116 passed, 0 failed, 9 ignored. This matches the record. |
| 6 | `cargo fmt --all -- --check`; `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | both exit 0 |
| 6 | `check-`/`test-` session, protocol-control and workspace policy | all ok/PASS |
| extra | `cargo test -p session-validator`; `RUSTDOCFLAGS='-D warnings' cargo doc --no-deps -p session -p protocol -p conformance`; `check-env-vocabulary.sh` | all exit 0 |
| extra | `session_validator validate --canonical` on `worked-session.json` | all 5 stages PASS; the output is byte-identical to the file |

Other checks:
- **D8.** `live_builtin_delta` does not exist in the tree, so #1053 has not landed. D8 and gate 7
  therefore correctly change nothing.
- **Authorized paths.** All 26 files are in the list.
- **GitHub.** #1199 is OPEN, and its title matches the spec's H1. That is correct for batch mode.

## Checks the brief asked for

**Field and registry IDs.**
- Submix fields:
  - `schema.rs`: `ID` 1, `BUILTINS` 2, `INSERTS` 4, `FADER` 5, `MATRIX_OR_PAN` 6.
  - Each one is built from the same nested spec as the track's field: `builtins::SPEC`,
    `rack::SPEC`, `fader::SPEC`, `matrix_or_pan::KNOWN`.
  - Nothing is renumbered and nothing retired is reused. The submix message never had a retired
    field.
  - Field 3 is unallocated, and both comments say it is reserved for `console` (#1202).
- The codec calls the track's own codecs: `tx_builtins`/`tx_rack`/`tx_fader`/`tx_matrix_or_pan`
  and `parse_builtins`/`parse_rack_message`/`parse_fader`/`parse_matrix_or_pan`. Pan and matrix
  therefore share the one tag, as the hazard requires.
- `visit.rs` keys (`2/4/5/6`, with `pan` and `matrix` both on 6) agree with `schema.rs`.
- The canonical walk writes `id, builtins, inserts, fader, pan|matrix`. That order matches:
  - D2;
  - `SESSION_SCHEMA_V1.md` (lines 42-52);
  - the `submix` `oneOf` in `session-v1.schema.json`;
  - the registry line in `CONTROL_PROTOCOL_REGISTRY.md`.

**The `COMPLETE_SCHEMA_HASH` re-pin is legitimate.**
- It is an FNV-1a-64 hash over the BTLV wire corpus, i.e. a digest of a wire format, which
  AGENTS.md's test-value clause expressly allows.
- It moved for the intended reason. The `UpsertSubmix` row now encodes the four new fields, with
  trim -3, the fixture insert, a muted right lane and a matrix with smoothing 16.
- The reason is recorded in the `protocol_corpus.rs` doc comment ("repinned it from
  `ebf282621550d44a`: … submix fields 2, 4, 5, 6") and in `CONTROL_PROTOCOL_CONFORMANCE.md`.
- No stale `ebf2…` literal remains outside the specs.

**Nothing moved for submix-free sessions.**
- By construction, a submix-free session is unaffected:
  - the walk, the codec and validation change only for `Submix` records;
  - the estimate's `size_of::<Submix>()` is multiplied by a count of 0.
- Gate 5 agrees, and `checked_in_fixtures_are_exact_canonical_bytes` passes.
- Only two committed JSON documents changed:
  - the writer corpus (the `mix` submix row);
  - `worked-session.json` (the `band` strip).
- No other tracked JSON has a non-empty `submixes`. I checked this with a JSON walk over every
  tracked `*.json`.

**Validation parity between tracks and submixes.**
- `validate_strip` is one shared body: builtins, delay maximum, fader finiteness, pan range,
  matrix finiteness and the rack. Track and submix both run it. Only the track's source half and
  console entries stay in `validate_tracks`.
- Diagnostic paths are index paths (`$.submixes[<i>]…`). Uniqueness and the namespace are
  unchanged.
- `parse_submix` reuses the track's sub-parsers. The gate-1 test compares the full diagnostic set
  of each case against the same edit applied to `tracks[0]`, which is stronger than the gate asks.

**The three deviations are acceptable.**
- **The `1e39` case.** JSON cannot spell NaN or infinity. `1e39` is the grammar's non-finite
  `f32`, and it earns the track's `schema.numeric_not_f32_representable`. The model-side case
  uses a real NaN and gets `numeric.non_finite`. Together they cover the gate's intent.
- **A console automation target on a submix is refused.**
  - A submix has no console entries until #1202, so `effect_id` names nothing. This is the same
    code and path as a track whose slot is absent.
  - Automation targets are consumed nowhere downstream. I grepped graph-compiler, host-core,
    builtins-compiler and effect-compiler, so D6's "inert" holds.
  - See MINOR-2 for the hand-off to #1202.
- **The estimate is deferred to #1200.** D4 here and #1200's D0 freeze this explicitly ("inside
  K1 only, bus inserts are uncharged between 02 and 03"). See MINOR-1 for a consequence that the
  record does not state.

**Coherence for the next slice, and the SDK.**
- The tree compiles, and every Rust gate above is green. `strips()` stays track-only, and
  `Submix::unity` exists for #1200 and #1202.
- The graph compiler still lowers a bare summing node (D7).
- The SDK evals are in the required check, so the K1 PR would be red until #1205:
  - the `sdk` job runs `check-sdk-headless.sh`, which runs `node --test 'test/*-evals.mjs'`
    (builder-evals, console-evals);
  - the same job runs `sdk-package.sh check`, which runs `enginectl-cli.mjs`;
  - `verdict` requires `sdk`.
- That is acceptable, for these reasons:
  - #1205 is slice 08 of K1 and the batch is pushed once after it.
  - Feature-branch pushes do not trigger `qualification.yml`.
  - Only `sdk/test/*` and SDK sources author submixes. #1205 already lists every one, plus
    `SKILL.md:66`.

## Findings

### MINOR-1: until #1200, `compile_session` refuses valid sessions with heavy bus inserts

**What happens.**
- `canonical_upper_bound_bytes` is computed as `4096 + 10·strings + 1024·structural_items`.
- `structural_items` includes `effect_count`/`parameter_count`, and those come from `strips()`,
  which is still track-only.
- Submix insert text is therefore written but never budgeted. `with_canonical_bytes` then refuses
  the session *after* the canonical string has been allocated.

**Probe.** I added an uncommitted test to the scratch copy: the `canonical.json` fixture plus one
unity bus, with `miso.parametric-eq` inserts. Results:

| Bus inserts | Result |
|---|---|
| 1×100 params | `capacity.arithmetic_overflow at $.canonical: canonical writer exceeded its conservative preflight bound` |
| 8 effects × 8 params | same refusal |
| 2×32 params | compiles |
| 3×16 params | compiles |
| Same inserts on `tracks[0]` | compiles in every case |

**Why it is only MINOR.**
- It exists only inside K1, which is never pushed before #1200.
- #1200 D0 moves submixes into `strips()`, which restores the bound automatically.
- The brief forbids a direct charge in this slice.

**Fixes.**
- (a) Add one line to the #1199 record's estimate deviation: compile refuses large bus racks with
  `capacity.arithmetic_overflow` at `$.canonical` until #1200. It is not only an under-charge.
- (b) The root should annotate #1200's estimate gate to also require that such a session compiles.
  Eight effects × eight params on a bus is enough to discriminate.

### MINOR-2: the submix console automation arm hard-codes an empty slice, and #1202 does not know

**What happens.**
- `validate_automation` maps `GraphEntity::Submix(submix)` to `(&submix.inserts, [].as_slice())`.
- When #1202 adds `Submix.console`, this still compiles, and console targets on a submix are still
  refused.
- The only forcing function is a comment, plus `automation_target_may_name_a_submix`, which
  *asserts* the refusal.
- #1202's spec never mentions automation.

**Fix.** The root should annotate #1202:
- replace `[].as_slice()` with `submix.console.as_slice()`;
- flip that test's console-target assertion to "valid for a declared slot";
- update `SESSION_SCHEMA_V1.md:158-159` ("a submix carries no console entries yet").

### NIT-1: the random round-trip test's doc comment overclaims

The doc comment says the test turns red if the codec "drops or misnumbers a strip field". A
symmetric renumbering is invisible to a round trip:
- With mutation M5 below (submix `BUILTINS` 2→3 on both sides), the random test stays green.
- Under the same mutation, `COMPLETE_SCHEMA_HASH` goes red.

Say "drops or mis-encodes a field asymmetrically; field numbers are pinned by
`COMPLETE_SCHEMA_HASH`". The same fix applies to the record's test-value line.

### NIT-2: the two docs that carry the hash now disagree on history

- **The fuzz manifest** got the hash only, as authorized. Its history paragraph therefore still
  ends at #1094, and nothing in it traces `ebf2…` → `ca48…`.
- **`CONTROL_PROTOCOL_CONFORMANCE.md`** was authorized for "the hash only" but also gained the
  reason clause. That clause is consistent with its existing parenthetical, and it is harmless.

Either allow a one-sentence history line in the manifest in a later hash-moving slice (#1202
re-pins anyway), or accept the asymmetry.

### NIT-3: track diagnostics are now produced in a different order

`validate_strip` now runs a track's inserts before `validate_console_entries`. Before, it was
console entries first.
- `DiagnosticSet` sorts, so ordinary output is identical.
- Only which diagnostics survive the 64-diagnostic cap can differ, for a pathological track. No
  test or consumer pins that.
- No change is needed. This note is here so nobody rediscovers it.

## Test value: one sentence each, with the mutation evidence

- **`submix_strip_is_refused_with_the_tracks_codes_at_index_paths`.**
  - *Catches:* a submix key validated by a different rule or code than a track's, or reported at
    a track or `[id=..]` path.
  - *Evidence (M1):* re-rooting `validate_submixes` at `$.tracks` turned it red.
- **`submix_strip_validation_reports_index_paths`.**
  - *Catches:* NaN, infinity, negative-HPF and duplicate-insert values on a submix that escape
    the typed entry point's validation or land at a wrong path. JSON cannot express NaN or
    infinity, which makes those cases unique to this test.
  - *Evidence (M1):* red.
- **`submix_strip_round_trips_canonically_in_key_order`.**
  - *Catches:* a submix walk that orders keys other than D2's or drops a field. The **matrix**
    variant is unique here, because the writer corpus's submix uses pan.
  - *Evidence (M3):* swapping inserts and fader in the walk turned it red, together with the
    writer-corpus test.
- **`compile_session_canonicalizes_submix_insert_parameters`.**
  - *Catches:* `compile_session` leaving bus insert parameters in declared order.
  - *Evidence (M2):* dropping the submix chain turned only this test red.
- **`automation_target_may_name_a_submix`.**
  - *Catches:* a submix target being refused, or its insert lookup being skipped. Nothing else
    exercises the D6 arm.
  - *Evidence:* reasoned, not mutated.
- **`unity_submix_is_transparent_and_valid`.**
  - *Catches:* `Submix::unity` drifting from the identity strip, e.g. a muted lane or `rr = 0`.
    That drift would silently gate every migrated bus once #1200 renders it, and no render
    covers it today.
  - *Evidence:* reasoned, not mutated.
- **`random_submix_strips_round_trip_losslessly`.**
  - *Catches:* an asymmetric submix codec defect.
  - *Evidence (M4):* swapping pan left and right in `tx_submix` only turned it red.
    `COMPLETE_SCHEMA_HASH` cannot see that defect, because the corpus row is a matrix.
  - *Limit (M5):* symmetric renumbering stays green and is owned by the hash (NIT-1).

None of these tests greps source or prose. The key-order test inspects the writer's *output*, which
is the subject of the claim. The writer corpus is the Rust-generated single owner of a cross-target
corpus, so it is an allowed byte pin.

## Logs

The scratch build directories (`/tmp/claude-1002/v1199`) were removed after this review. Their
exit codes and totals are reproduced above.
