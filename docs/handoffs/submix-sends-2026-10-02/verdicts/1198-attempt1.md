# #1198 *Iterate session strips, not tracks, wherever strip semantics apply*: Sol verdict, attempt 1

- Reviewed: `git diff 45c1a342 0a769a63` (branch `codex/batch-submix-k0`, worktree
  `/home/bl/misofm/wt-submix-k0`). `45c1a342` is code-equal to `fe8ac679`: `git diff fe8ac679
  45c1a342` touches only `docs/`, `.github/` and `AGENTS.md`.
- Binding: `AGENTS.md` and `.github/ISSUE_SPECS/1198-iterate-session-strips-not-tracks.md`.
- This review changed nothing in the worktree's tracked files, the branch or GitHub. The tests,
  clippy and doc ran against the worktree's own `target/` at `0a769a63`. That build was already
  fresh: zero `Compiling` lines, and `git status` was clean before and after.

## Verdict: PASS

There is no BLOCKER and no MAJOR. Every objective gate reproduces exactly, the change is class A by
construction, and every site the spec names was switched or commented. There are two MINOR
findings. One is a spec inconsistency the implementer handled correctly; the other is an existing
coverage gap. Neither needs another attempt.

## Gates (re-run by me)

**Gate 1, class A.** I exported both commits with `git archive` into
`/tmp/claude-1002/1198-verify/{base,head}`, each with its own target directory.

| Check | base `45c1a342` | head `0a769a63` |
|---|---|---|
| `cargo build --locked --release -p audit -p bench -p capi -p session-validator` | exit 0 | exit 0 |
| `check-graph-determinism.sh` | PASS (100/100) | PASS (100/100) |
| `fresh-process-determinism.json` sha256 | `9a7f1d6a...42f4051` | `9a7f1d6a...42f4051` |
| `diff` of the two JSON files | empty | empty |
| `graph_fixture -- --check` | exit 0 | exit 0 |
| `check-console-fixtures.sh target/release/session_validator` | ok | ok |
| `check-builtins-fixtures.sh . target/release/audit` | ok (50 files) | ok (50 files) |

Logs: `/tmp/claude-1002/1198-verify/gate1-{base,head}.log`.

**Gate 2.**
- The spec's workspace command: exit 0, 1109 passed, 0 failed, 9 ignored, across 96 test binaries.
  This matches the implementer's numbers.
- `cargo test --locked --release -p audit -p bench -p console-workload`: exit 0, 110 passed, 2
  ignored.
- One test changed, and only its argument (`builtins-compiler/src/lib.rs:12187`, now
  `strip_parameters(&model.strips().next().unwrap(), ..)`). No assertion was edited.

**Gate 3.**
- `cargo fmt --all -- --check`, `cargo clippy --locked --workspace --all-targets --all-features --
  -D warnings`, and `cargo doc --workspace --no-deps` with `RUSTDOCFLAGS='-D warnings'`: all exit 0.
- The `check-` and `test-` policy scripts for session, builtins, graph, host-core, realtime and
  workspace all pass. The realtime check reports "ok (54 marked regions in 15 files)".

**Authorized paths.** All ten touched files are in the authorized list. In host-core, `prepare.rs`
changes only `count_effects` and comments, and `shape.rs` gains one comment line. GitHub #1198 is
OPEN with a title that matches the spec's H1. It stays open as batch mode expects, since nothing is
pushed yet.

## Class A by construction (independent of the gate)

The gate-1 graph fixture (`direct-route`) has no effects and no sidechain. On its own it therefore
proves only the four `collection_path()` chain-edge rows. I checked the other spellings directly:
- `path_prefix()` is `format!("{}[id={}]", "$.tracks", id.as_str())`. `StableId`'s `Display` is
  `write_str(&self.0)` (`session/src/id.rs:33-37`), so the result equals the old
  `format!("$.tracks[id={}]", track.id)` byte for byte. That covers `effect_path`, the sidechain
  path, the builtins `parameter_diagnostic` prefix, and the effect-compiler loop and attach paths.
- **Mutation M1.** I appended a marker to `effect_path`'s format and ran graph-compiler, host-core
  and effect-compiler tests. `graph-compiler/tests/track_delay.rs::the_zero_delay_plan_digest_is_the_current_semantic_plan`
  went red: its pinned canonical digest covers a session with nine EQ effect edges. That test is
  green on head, so effect edge paths in the sealed text did not move.
- `strips()` is `self.tracks.iter().map(StripRef::track)`, so order and count equal the old loops.
- `lower_strip` is `lower_track`'s old body over the same borrowed fields.
- Realtime: no render-path code changed. The new allocations are `path_prefix()` and the per-entry
  `strip_path`. Both are made during control-plane preparation. Entries are consumed in
  `graph-compiler/src/ids.rs::into_effects` and never reach the plan, so `strip_path` is dropped off
  the render thread. `EffectPreparedEntry` is not size-accounted anywhere.

## Site audit

**Switched to `strips()`/`StripRef`, matching every Context bullet:**
- graph-compiler: the lowered racks, the declared set, the stage chains, sidechain edges, the four
  `collection_path()` literals, and the response-binding count and bytes in `compile.rs`; the rack
  chains in `banks.rs`; and `effect_path` in `ids.rs`.
- builtins-compiler:
  - `validate_for_session`, `expected_tails`, and the control/meter `known_strips`;
  - the parameter preflight, the preparation loop, the seal's `tracks` vector (it feeds
    `planned_strip_banks` and `processor_seal`) and `resource_plan`;
  - `track_parameters` became `strip_parameters`, and the five path builders take `&StripRef`.
- effect-compiler: the strip loop and its path, the new `strip_path` field, both attach paths, and
  `declared_live_addresses`.
- session `estimate.rs`: the effect count, the parameter count, and the per-strip vectors.
- host-core: `count_effects`.

**Left track-only, each with `// Source semantics: tracks only.`:**
- builtins-compiler:
  - `track_mono_source`;
  - `session_structural_symmetry`, which also carries the mono-collapse-guard line;
  - `SessionPoolClasses::from_session`.
- graph-compiler: `track_delays`, which also carries the `TrackDelay`/#1201 line.
- host-core `prepare.rs`:
  - the track, source and route counts;
  - the source mappings;
  - the live-control track list;
  - the meter requests;
  - the eligibility filter.
- host-core `shape.rs`: `track_count`.

**Remaining `.tracks` iterations** are all source semantics (above), tests, or sites outside this
slice's Context and authorized paths:
- `session/src/compile.rs:152` canonicalization;
- `session/src/validate.rs`;
- `protocol/src/model.rs` edits;
- host-web.

I found no missed strip-semantics site in the authorized files.

**Contract.**
- The names and signatures match the interface block. `StripKind` has no `#[non_exhaustive]`, and
  the only `match` on it, in `collection_path`, has no wildcard arm. The doc comments for
  `StripKind`, `strips()`, `lower_strip`, `path_prefix` and `collection_path` are the spec's text.
- `lower_track` delegates to `lower_strip`.
- `EffectPreparedEntry::strip_path: Box<str>` is set only at the single production literal.

**Deviations.**
- **Private `StripRef::track` constructor, and the `Clone, Copy, Debug, PartialEq` derives:**
  acceptable. All fields are public, as the contract requires. #1199 D4 and #1200 D0 add
  `StripKind::Submix` inside `crates/session`, where a private `StripRef::submix` sits naturally.
  None of #1199-#1209 needs a public constructor.
- **`solo.rs` has no comment:** correct. See MINOR-1.
- **The aggregate `"$.tracks"` diagnostic literals were kept:** these are in `estimate.rs` and in
  the overflow path of builtins `resource_plan`. Deliverable 5 requires this ("change no ...
  path spelling"). See NIT-2 for #1200.

## Findings

**MINOR-1: a spec inconsistency, not an implementation defect.**
- Context lists "solo (`crates/host-core/src/solo.rs`)" as a source-semantics site, and
  deliverable 3 asks for the comment "at each" such site. But `solo.rs` is not an authorized path.
- The implementer followed the authorized list and recorded the gap, which was the right call.
- Fix: root records this exception in the #1198 decision record. The slice that first edits
  `solo.rs` for strips (#1211 or #1213) adds the line there.

**MINOR-2: an existing coverage gap. The sealed sidechain edge path is pinned by nothing.**
- Mutation M2 appended a marker to the sidechain edge path (`compile.rs:387`). All 320 tests in
  graph-compiler, host-core and effect-compiler stayed green.
- No gate-1 fixture has a routed sidechain: `main-sidechain-pdc.csv` is hand-written PCM with no
  paths.
- This slice is still class A for that path, by the string identity shown above.
- No action is required here: AGENTS.md makes a no-bit-moved comparison PR evidence, not a test.
  A later regression of the `.sidechain` suffix would go unseen, though. Optional successor: add a
  routed-sidechain case to the graph fixture corpus, which is the single owner of the sealed text.

**NIT-1.** The new `processor_seal` doc says "keyed by strip ID (`session.strips()` order)", but
the function sorts its output (`values.sort_unstable()`). Say "input in `strips()` order, output
sorted". It matters in #1200, where the concatenation is no longer sorted.

**NIT-2 (for #1200).** Once `strips()` yields submixes, the estimate's aggregate overflow paths
(`"$.tracks"`, `"$.tracks.console"`, `"$.tracks.inserts.effects"`, `"$.tracks.effects.params"`) and
builtins `resource_plan`'s `"$.tracks"` will label a bus's overflow as tracks. #1200 should decide
whether to keep the aggregate label or route these through `collection_path()`.

**NIT-3.** `compile.rs` formats `strip.path_prefix()` again for every effect, and twice more on
diagnostic paths. effect-compiler hoists it once per strip. It is control-plane and negligible.

## Test value

None is needed. Gate 4 says so explicitly, and the slice's claim is "nothing moved", which
AGENTS.md assigns to PR evidence rather than to a committed test.
- While `strips()` equals the track list, no test can tell strip semantics from track semantics.
- The spellings that matter are already defended:
  - builtins diagnostic-path tests pin `$.tracks[id=vocal]...`;
  - the `track_delay.rs` digest pins effect edge paths (M1 above);
  - `direct-route` pins the collection path.
- The one undefended spelling, the `.sidechain` suffix, is MINOR-2. It is a gap that already
  existed, not one this slice opened.

## Scratch evidence

Everything is under `/tmp/claude-1002/1198-verify/`:
- `gate1-base.log` and `gate1-head.log`, with `{base,head}-fresh-process-determinism.json`;
- `gate23-head.log` and `gate2-release.log`;
- `policy-*.log`;
- `mut-M1.log` and `mut-M2.log`.
