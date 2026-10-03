# #1205 *Build submix strips and bus taps in the SDK and teach agents to author them*: Sol verdict, attempt 1

- Reviewed: `git diff 7f767ae0c 7926e48fe` (branch `codex/batch-submix-k1`, worktree
  `/home/bl/misofm/wt-submix-k1`). 14 files, +1004/-123. Every path is on the spec's authorized
  list, and no Rust or generated file changed.
- Binding: `AGENTS.md` and `.github/ISSUE_SPECS/1205-build-submix-strips-and-bus-taps-in-the-sdk-and-teach-agents-to-author-them.md`
  with its Attempt 1 record. I also used #1197 D5 and the MINOR-A amendment, the K1 verdicts
  (#1199-#1204, all PASS) and the app repo at `7effbd6`.
- How I ran it:
  - I did not modify the worktree, the branch or GitHub. `gh issue view 1205` shows it OPEN, and
    its title matches the spec. That is correct in batch mode.
  - I exported `7926e48fe` with `git archive` into `/tmp/claude-1002/v1205/src`, used its own
    `CARGO_TARGET_DIR` and ran `npm ci` there.
  - After every mutation, the mutated file was restored, and I checked it with `cmp`.

## Verdict: PASS

There is no BLOCKER and no MAJOR. There are four MINOR findings: three are test or guidance
coverage gaps and one is a layout issue in the plan. There are five NITs. None of them needs
another attempt, and the MINORs can be fixed in the K1 follow-up commit.

- The builder, the writer, the `enginectl` parser, the skill, the handoff, the migration script
  and the two prose edits all do what D1-D7 and deliverables 1-7 say.
- Every gate I re-ran passes. The counts match Terra's record.
- I checked the engine facts the skill states against the engine. Each one holds.

## Gates (re-run by me on `7926e48fe`, x86-64 AVX2)

| Gate | Command / probe | Result |
|---|---|---|
| shipped module | `bash scripts/build-web-audioworklet.sh <A>` | `6aec3733...5ecc`, named twin `4c0e960a...`. This equals the record, so the artifact is **unchanged** |
| 8 (CI `sdk` job, verbatim order) | `check-sdk-generated.sh <A>`; `check-sdk-deletions.py`; `check-sdk-types.sh`; `check-sdk-headless.sh <A>`; `sdk-package.sh check <A>` | every step exit 0. Headless **333/333**; `enginectl-cli.mjs` **15/15**; tarball gate ok |
| 8 | `check-workspace-policy.sh` (in a scratch `git init` of the export) | ok |
| lint extras | `check-script-reachability.py`, `check-env-vocabulary.sh`, `check-session-policy.sh` | ok (the new `.py` is under exempt `docs/`) |
| 6 | worked-session row in `console-evals.mjs`; `cargo test --locked -p session-validator --test skill` | pass; 1 passed |
| 7 | `git show 0fb85d2c5:.claude/skills/author-session/worked-session.json`, then the script, then `session_validator validate --canonical` | the script reports `transparent strips ['band']; submix_output -> submix post_pan ['band']`. The output passes all 5 stages, is its own canonical form (`cmp`), and is **byte-identical to the checked-in `worked-session.json`** |
| 9 (K1 boundary) | not re-run | the slice changes no Rust and the module hash is unchanged. These gates belong to the root at the K1 head. The record's aarch64 gap (CI `aarch64-debug` at the push) stands |

## Deliverables

- **D1 / D2 / D3, the SDK.**
  - `submix(id, spec?)` validates a spec through `#validateStrip`, the track's path, and through
    `knownKeys`.
  - `normalizeSubmix` writes a spec-less submix as `Submix::unity` (`crates/session/src/model.rs:710`).
  - `RouteSource` is `track | submix`, with a tap. `submix_output` is refused with
    `schema.invalid_enum` and names its replacement, as a route source and as a sidechain source.
  - The writer orders a submix `id, builtins, console, inserts, fader, pan|matrix` and the source
    `kind, submix_id, tap`.
  - Probes against the native validator:
    - the spec-less output with 2 slots is canonical-equal (`validate --canonical`);
    - a submix with `builtins` delay is canonical-equal;
    - a route from each of the seven submix taps boots in the shipped module;
    - a submix insert keyed from its own `input` tap boots.
- **The SDK and the migration script write the same transparent strip.** I took the SDK's spec-less
  document, rewrote it into the pre-K1 shape (a bare `{"id"}` and a `submix_output` route), and ran
  the script on it. The script's output is **byte-identical** to the SDK's `toJson()`. Gate 7 shows
  the script output also equals the engine-migrated worked session, so the SDK, the script and
  `Submix::unity` agree.
- **D4, `enginectl`.**
  - A bare string is a transparent strip, and `{ id, ...strip }` goes through the shared `strip()`.
  - `submix_output` is refused with `request.shape` and diagnostic `schema.invalid_enum`.
  - `sdk/README.md` is updated.
- **D7, the migration script.**
  - It is idempotent: a second run reports `[]` and leaves the bytes the same.
  - `--check` does not write.
  - It rewrites a `submix_output` sidechain source as well as a route source.
  - A partial strip is refused (`submix 'band' carries a partial strip`, exit 1).
  - A layout it cannot reproduce (minified) is refused (exit 1).
- **The skill.** Every claim below holds on this tree:
  - a bare `{"id"}` is refused `schema.missing_field` at typed-model;
  - `source_id` on a submix is refused `schema.unknown_field` at typed-model;
  - a bus cycle passes all 5 validator stages and is refused at boot with `graph.cycle`;
  - a muted bus's `pre_fader` tap is ungated (rendered energy: `pre_fader` 454, `post_fader` 0,
    `post_pan` 0);
  - `miso.compressor` supports `maximum` and `average`;
  - the limiter's latency is 486 samples at 48 kHz (`true-peak-limiter/src/lib.rs:5094`);
  - route-ID summation order (`model.rs:680`).

  The bus-compressor hazard and its guidance match deliverable 3 and DESIGN 2.2b. The skill makes
  no performance claim (D5).
- **The handoff.**
  - `APP-SDK.md` follows the `APP-CONSOLE-SDK.md` model.
  - Its app claim holds: at app `7effbd6`, `src/` authors no submix and no `submix_output`, and
    only tests mention `submixes: []`.
  - The slot IDs `eq` and `compressor` in its example match the app's.
  - `ABI_VERSION` is untouched across K1.
- **`AGENTS.md`.**
  - Exactly the five qualifiers #1197 D5 assigns to #1205 are removed: dual-mono strip (#1200),
    chain (#1200), console slot (#1202), strip inserts (#1200) and seven taps (#1203). All of
    their slices have a PASS verdict.
  - These stay: the route-mute and follow-mute qualifier (#1216/#1218, not landed), the VCA
    qualifier (not landed) and the decision-13 parenthetical with its four authority kinds.
  - No remaining sentence claims a send, live or meter behaviour that has not landed.
  - "strip-locally" is in.
  - The banking paragraph's five "tracks" are now "strips", exactly as MINOR-A asked.
- **`IMPLEMENTATION_PLAN.md`.** Exactly the one sentence changed, with both edits.
- **The `enginectl-cli` cargo dependency is acceptable.**
  - Both callers of `sdk-package.sh` install Rust 1.97.1: CI `sdk` and the npm-publish `qualify`
    leg. Publish mode does not run it.
  - Both callers run `check-sdk-headless.sh` first. Its `console-evals` already runs the identical
    `cargo run --locked -q -p session-validator`, so the CLI test's run is a cache hit and the
    10-minute job budget is not at risk.
  - `npm run build` without a Rust toolchain now fails at the test step. That only matters to a
    developer packaging locally without Rust, who already needs Rust to build the module.

## Findings

### MINOR-1. Gate 2 does not pin three refusals the builder makes, and three mutations of mine survive the whole suite

Each mutation below was applied alone and run against all 333 headless evals and all 15
`enginectl-cli` tests. All three stayed **green**:

| Mutation | What then goes wrong silently |
|---|---|
| A: add `"source"` to `SUBMIX_KEYS` (`session.ts`) | `submix("bus", { source: "s", ... })` is accepted from JS, and the key is **dropped** from the document. The engine would say `schema.unknown_field` |
| D: validate the tap only for `kind: "track"` in `validateRouteSource` | a submix route source with a bad or missing tap (`post_matrix`, `undefined`) gets past the builder. The engine refuses it at validation (`schema.invalid_enum` / missing field). This is exactly what gate 2's test-value sentence says it defends ("lets through a submix the engine would refuse") |
| J: allow `"source"` in the CLI submix object's `keys(...)` (`session-request.ts`) | `enginectl` silently drops an unknown key on a submix object instead of refusing it with `schema.unknown_field` |

The code is right today: an unmutated probe refuses all three with the engine's code. The tests
just do not hold it there.

**Fix:** add three rows to the gate-2 test in `builder-evals.mjs`:
- `submix("bus", { source: "stem" })` refuses with `schema.unknown_field`, with the
  hand-edited-engine pair `source_id` -> `schema.unknown_field`;
- a `{ kind: "submix", submixId, tap: "post_matrix" }` route source refuses with
  `schema.invalid_enum`;
- the same with the tap omitted.

Add one `enginectl-cli.mjs` case: a submix object carrying `source` fails with `request.shape` /
`schema.unknown_field`. Each row's test value: it turns red if the builder or the CLI drops or
passes a submix key or tap that the engine refuses.

### MINOR-2. The SDK cannot author a K1 automation target on a submix, and the skill still says automation lives on "that track's" entry or insert

- #1199 D6 lets an automation target's `entity_id` name a submix (`crates/session/src/validate.rs:757-770`,
  "a declared track or submix"; still inert until #1058).
- The builder refuses it: `automation()` looks only at `tracks` and fails with "is not a declared
  track" (`session.ts:1390-1393`). The `enginectl` target is `trackId` only.
- The product outcome says the SDK is "back in step with the engine grammar". For this one K1
  grammar feature it is not.
- `SKILL.md:102-104` still reads "name a parameter/channel pair already declared on that track's
  entry or insert". The spec's stale-guidance hazard says every sentence must match the merged K1
  grammar.

The impact is nil today, because stored automation renders nothing.

**Fix:**
- In the skill, say "that strip's (a track's or a submix's) entry or insert", and state that a
  target may name a submix.
- Either file a bounded successor for an SDK submix automation target (`{ submixId }` beside
  `trackId`, in the builder and in `enginectl`), or record the gap in the spec and in
  `APP-SDK.md` as a known non-goal of K1.

### MINOR-3. One umbrella now has two handoff folders

- `docs/handoffs/submix-sends-2026-10-02/` holds the design record, the issue drafts and the
  verdicts.
- `docs/handoffs/submix-strips-and-sends/` holds `APP-SDK.md`, the migration script and, later,
  `APP-LIVE.md`.

The implementer followed the spec: D6 freezes the second name "literally", and REVISION-2 row 17
resolved it at design time. #1214, #1223 and the V4 draft already name it. The precedent
(`console-strip-2026-09-29/`) keeps the app docs, the migration script and VERIFY in one dated
folder. Here a reader of the design folder has no pointer to the app folder: `ISSUE-MAP.md` does
not mention it. Only `APP-SDK.md` links the other way.

**Fix (root, outside #1205's authorized paths):** keep the name, because three specs depend on it.
Add one line to `submix-sends-2026-10-02/ISSUE-MAP.md`, or to `DESIGN.md`'s header, naming
`docs/handoffs/submix-strips-and-sends/` as the umbrella's app-facing folder. Alternatively, rule
at the umbrella level to move it under the dated folder and update #1214, #1223 and V4 in the same
commit.

### MINOR-4. The rebuild row's test-value sentence does not name a defect that gate 1 misses

The record says the rebuild row ("the builder rebuilds an engine-written submix strip and bus tap
byte for byte") is red on M8 and on a dropped tap. Gate 1 catches both too (M1, M2 and M8 are
listed against both). What the row does add is the `insert_send` and `insert_return` taps and a
muted (`right_mute: true`) submix fader. Gate 1 and the rich session leave the mute at its default.

**Fix:** reword the sentence. It turns red if a non-default submix fader mute, or an
`insert_send`/`insert_return` tap, is lost between the engine's text and the builder.

### NITs

1. A submix refusal names a track. `submix("bus").console: ... every track carries every slot`
   comes from the shared `normalizeConsoleEntries` message. It should read "every strip carries
   every slot".
2. `AGENTS.md` "Effects and plugins" still says a console slot banks "for every track count". This
   is true but now narrower than the banking paragraph's "strips". The spec said to change nothing
   else, so leave it to the next AGENTS amendment.
3. The migration script reads with universal newlines and writes LF. A CRLF document is migrated
   with LF line endings, which slightly contradicts the "layout preserved byte for byte" claim.
   This is harmless, because canonical is LF. `--check` exits 0 even when a migration is pending.
   That is acceptable under D7's "reports".
4. Add one line to the Attempt 1 record: gate 5 now compares against
   `cargo run -p session-validator` (a deviation from a node-only `enginectl-cli.mjs`), and say why
   it is acceptable (see above).
5. Out of scope, engine side: the boot refusal for a submix insert keyed from its own `post_fader`
   tap reports `graph.cycle` at `$.submixes[id=bus].simd2.effects[id=comp]`. That path uses the
   retired `simd2` rack name and points at the console slot, not at the keyed insert `k`. It
   belongs with the existing 1200/1203 MINOR-1 follow-up (graph.cycle diagnostic path).

## Test value (one sentence per new or rewritten test)

- **`builder-evals` "submix strips: the builder refuses what the engine refuses"** is red when any
  of these gets past the builder: a spec'd submix declared before `console()` (M5), a submix missing
  a console entry (M6), or `submix_output` as a route or sidechain source (M7). It is green on
  mutations A and D (MINOR-1).
- **`console-evals` "a bus session is the engine's canonical JSON ... boots and renders"** is red
  when the writer orders submix keys or spells the bus source differently from the engine (M1, M2),
  or when an SDK bus session fails preparation.
- **`console-evals` "a spec-less submix is the transparent strip"** is red when a spec-less submix
  is not `Submix::unity`: un-bypassed entries (M3), the default pan (M4), or a fader off unity (M9).
  It checks the written document and the bit-exact render.
- **`console-evals` rebuild row** is red when a muted submix fader, or an
  `insert_send`/`insert_return` bus tap, is lost on the engine-text-to-builder path (MINOR-4).
- **`enginectl-cli` gate 5** is red when the CLI drops a submix object's strip (C1, which the parity
  row also catches) or accepts `submix_output`. The second is caught only here (C2).

## Scratch

`/tmp/claude-1002/v1205/` (the export, its target, npm modules and the probes) is deleted after
this verdict.
