# Check realtime regions with a Rust syntax-tree tool: the crate, the walk, unsafe ownership, markers and floors

Stream J follow-up of decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-0, D15-2).
Root rescoped *Require every loop around a realtime drain to drain a different queue on each pass*
(#1418) after its two verdicts. Root's ruling: replace the awk gate with a tool that reads Rust's
syntax tree. Do not add a fourth layer of patterns. The work has nine slices:

- **C1 (#1445):** an operator script that syncs edited specs to their GitHub bodies, with a self-test. It
  has no dependency and may land first.
- **A1 (this issue):** the crate, the binary, the fail-closed walk, unsafe ownership, markers,
  floors and the case harness.
- **A2 (#1439):** the region boundary rule, the forbidden-body predicate and the whole-plan check.
- **B1a (#1440):** the drain-bound rule's mapping, scope resolver, pops, counts, reads and innermost
  bound, with the ported drain cases those rules decide.
- **B1b (#1441):** the drain-bound rule's outer loops, attributes and constructors, the remaining ported
  cases, the synthetic twins and the marker on `drain_controls`.
- **B2a (#1442):** refuse pops in closures, macros and function values; commit the probe corpus.
- **C2 (#1446):** delete the awk gate and its self-test. CI then runs only the tool.
- **The #1448 guard commit** ("H #1448's guard, landed by J after C2"; root ruling 2 on the fourth
  review, option (C)): the two region markers around `poll_meters`, whose code stream H's #1448
  landed unmarked. It is not a slice of its own: it is B2b-1's first commit, and its spec is
  B2b-1's section "#1448 guard" (#1448 closes long before the J batch).
- **B2b-1 (#1443):** every non-test pop in a marked region or on a control-side allowlist; the markers on
  the render-thread drains. It waits for three production issues in streams B and H.
- **B2b-2 (#1444):** the call rule (a call to a popping function in a marked loop or closure). It waits for
  #1345 (Amendment 1) in stream B.

The order is A1 → A2 → B1a → B1b → B2a → C2 → the #1448 guard commit → B2b-1 → B2b-2, with C1
before C2. #1418 (Amendment 1) and #1426 (Amendment 1) come after the whole batch is on `main`, in
the tool. No production code changes in this slice.

## Problem (verified on `origin/main` at `6d28a80ec`)

Stream B batch 1 (#1309, #1343, #1314, #1311, #1348; `codex/d15-stream-b` at `b8392df66`) lands on
`main` before this slice (Dependencies). It changes only the floors and the self-test's pad files,
which this Problem states for both trees.

**Lines.** Lines of `scripts/check-realtime-policy.sh` below are `main`'s (`6d28a80ec`). On
`b8392df66`, which lands before this slice, every line from `:70` on is two lower: the floors are
`:75-76`, the drain pass `:144-432`, `counted()` `:208-228`, the limits `:122-143` (the stated
limits `:133-136`), the whole-plan check `:438-461`, the success line `:463`, and the file has 463
lines. Lines of `scripts/test-realtime-policy.sh` from `:440` on are one lower there.

**The gate is a text scan.** `scripts/check-realtime-policy.sh` (461 lines) uses `rg`, `awk` and
`sort` to scan the `.rs` files under `crates hosts tools`. Its drain pass (`:142-430`) is a
288-line awk lexer with its own brace stack. Its comment says "It is not a Rust parser"
(`:125-126`), and `:120-141` lists the forms it takes at their word or cannot see. Three verdict
rounds show that this approach does not converge:

- **#1302**, three attempts
  (`docs/handoffs/decision-15-2026-10-05/verdicts/stream-j/1302-attempt1.md` to `-attempt3.md`).
  Each attempt closed one layer of rustfmt layouts or literals. The next verdict found another:
  wrapped headers (J1-1), or-patterns and `}`-led header lines (J2-1), and raw C strings and raw
  strings with more than 16 hashes (J3-1).
- **The stream-J batch verdict** (MINOR-1 to MINOR-3, NIT-1, NIT-2; its findings were folded into
  #1302 by `116521b57`, on `main`). A count under `#[cfg(..)]`, a raw-identifier macro, and a
  finite but huge outer loop all passed.
- **#1418**, two attempts (`da878bd49` and `7caf972db`). Both were reverted in `9d955dc66` and
  are evidence only. The verdicts are
  `docs/handoffs/decision-15-2026-10-05/verdicts/stream-j2/1418-attempt{1,2}.md`. All of these are
  on `main` since #1436 merged `codex/d15-stream-j2`. The attempt-2 verdict reproduced these escapes, each one
  compiled: a `cfg`-gated `let` (MAJOR-1); a `.zip(..)` argument (MAJOR-2); a `map_or` closure and
  a macro call that only look like receiver reads (MAJOR-3); a local `macro_rules!` (MINOR-1). gawk
  and mawk also disagree: mawk stops with `eval stack size=1024` at 36 or 55 links, where gawk
  accepts the same input (MINOR-2).

Each of these is a fact of Rust's grammar: attributes, closures, macro token trees, patterns and
literals. A syntax tree gives these facts directly.

**The existing checks.** Each one is a decision row below or in a later slice.

| # | Check | Anchor | Slice |
|---|---|---|---|
| C1 | workspace root argument, default `.` | `:5`, `:9` | A1 |
| C2 | `crates/engine/src/realtime` must exist | `:16-17` | A1 |
| C3 | `unsafe` syntax only in 18 allowlisted files | `:28-33` | A1 |
| C4 | marker discovery; equal BEGIN and END counts in each file | `:45-65` | A1 |
| C5 | a region is the lines strictly between BEGIN and END | `:58-63` | A1 (text), A2 (syntax) |
| C6 | floors: at least 25 marked files and 89 marked regions (26 and 93 after stream B batch 1) | `:67-74` (`:67-76` on `b8392df66`) | A1 |
| C7 | forbidden-body predicate, 37 patterns | `:76-77` | A2 |
| C8 | drain-bound rule | `:79-430` | B1a, B1b, B2a |
| C9 | the C ABI never forms a reference to a whole `Plan` | `:436-459` | A2 |
| C10 | the success line | `:461` | A1 |
| C11 | every scan and child-process status is checked: a missing root, an unreadable file or a tool error fails the gate (`scripts/lib/gate.sh`) | `:28-30`, `:47-50`, `:53-54`, `:62-63`, `:425`, `:452` | A1 |

**The self-test.** `scripts/test-realtime-policy.sh` (1517 lines) builds a synthetic tree that
sits exactly on the floors (`create_fixture`, `:10-454`) and runs:

- 4 valid trees (`:494-526`);
- 39 non-drain expected failures: 9 forbidden-body cases (`:529-541`, `:585-596`), 17 unsafe cases
  (`:542-581`), 4 marker and floor cases (`:1407-1418`), 9 whole-plan cases (`:1422-1433`);
- the drain cases, which go to B1a and B1b: `marked-unbounded-try-pop-drain` (`:599-600`) and
  every name in the loop at `:1387-1402` (56 names);
- 22 tool-failure injections (`:1437-1489`) and 5 counter-mutants (`:1491-1515`).

Measured on the self-test's base tree: the awk gate prints `ok (89 marked regions in 25 files)`.
The base tree sits exactly on both floors. Its `hosts/host-web` fixture holds 1 region,
`pad_1.rs` 13 and `crates/builtins/src/lib.rs` 5. On the batch-1 branch the base tree has 13 pad
files, `pad_1.rs` holds 14 regions, and it sits on 26 files and 93 regions; the self-test's lines
from `:440` on move down by one.

**The real tree.**
- The gate passes it: `realtime policy: ok (89 marked regions in 25 files)` on `main`, and
  `ok (93 marked regions in 26 files)` on the batch-1 branch.
- All 513 tracked `.rs` files under `crates hosts tools` on `6b9067ede` lex and parse with `syn`
  2.0.119 (`full`): 1.2 s in release, 7 s in debug, measured with a scratch program outside the
  repository. `main` merged with the batch-1 branch has 518; the third review's scanner parsed
  them all.
- All marker lines match `^\s*// REALTIME_POLICY_(BEGIN|END)(: .*)?$`: 178 on `main`, 186 on
  `main` merged with the batch-1 branch. No other line in a `.rs` file under the roots contains
  `REALTIME_POLICY_`.

## Decisions

### The crate and the binary

- **D1. Name and output.**
  - Add `tools/realtime-policy`, package `realtime-policy`, with
    `[lib] name = "realtime_policy"` and `[[bin]] name = "realtime_policy"` (AGENTS.md naming
    rule). It is a workspace member. Its manifest inherits `version`, `license`, `edition`,
    `rust-version` and `publish`, and has `[lints] workspace = true`, as
    `tools/session-validator/Cargo.toml` does.
  - The library exposes `check(workspace_root: &Path, policy: &Policy) -> Report`.
    - `Policy` holds the tool's constants: the unsafe allowlist (D5), the two floors (D8), and the
      fields that later slices add (B2b-1's control-side allowlist). `Policy::workspace()` returns
      the real values. A library case builds its own `Policy` (usually `Policy::workspace()` with
      fields replaced), so no case depends on the real tree's files.
    - A `Report` holds the region and file counts and a sorted list of findings. A finding is a
      class (an enum), an optional path, an optional line and an optional source line.
  - **The policy is injectable.** The library also exposes the binary's whole behaviour as
    `run(args: &[OsString], cwd: &Path, policy: &Policy, out: &mut dyn Write, err: &mut dyn Write) -> u8`
    (the exit code). The binary's `main` is one statement: it calls `run` with
    `std::env::args_os()` (without the program name), `std::env::current_dir()`,
    `&Policy::workspace()`, stdout and stderr, and exits with the code. So every success-path case
    runs `run` with the case policy, and no case depends on the real policy's allowlist or floors
    (third review, MAJOR-3). The binary has no other way to take a policy: no argument, file or
    environment variable changes it.
  - `run` takes zero or one argument, the root, and runs `check(root, policy)`. The default is
    `.`, and a relative root is resolved against `cwd`.
  - Each finding goes to stderr as one line:
    `realtime policy failure: <class>[: <path>[:<line>[: <source line>]]]`. A floor, a missing
    module or a scan-root finding has no path or no line. The binary exits 1 on any finding.
  - With no finding, it prints exactly the C10 line to stdout:
    `realtime policy: ok (<regions> marked regions in <files> files)`. It exits 0.
  - Exit 2 is reserved for a bad argument, for example two arguments.
- **D2. Dependencies.** Use the versions that `Cargo.lock` already holds. Do not run
  `cargo update`.
  - `syn = { version = "=2.0.119", default-features = false, features = ["full", "visit", "parsing", "printing"] }`
  - `proc-macro2 = { version = "=1.0.107", default-features = false, features = ["span-locations"] }`
  - No `regex` and no `quote` crate. Every text pattern in A1 and A2 is a literal or a token
    sequence (A2-D2 and A2-D3 say how), so a regex engine is not needed. If the implementer finds
    one is, stop and report; do not add it.

  The `=` pin follows the `serde_json = "=1.0.151"` precedent. Of the three `syn` versions in the
  lock, 2.0.119 already builds with `full`, `visit`, `fold` and `extra-traits` in this graph (via
  `wasmtime` and `zerovec`'s derives). 3.0.3 builds without `visit`. `span-locations` gives the
  line numbers that region mapping needs. The only lockfile change is the new package's entry.
  Gate 6 holds the shipped artifacts unchanged.
- **D3. The walk fails closed (C11).**
  - **Scan roots.** The walk visits `crates`, `hosts` and `tools` under the root. It skips
    exactly the directories named `target` or `node_modules`. Neither holds a compiled `.rs`
    file, and the skip is the tool's only ignore rule. It considers every other regular file whose
    name ends in `.rs`.
  - **Each of these is a finding, never a skip:**
    - a scan root that does not exist or is not a directory;
    - a directory under a root that cannot be listed;
    - a symbolic link under a root that points at a directory, or whose name ends in `.rs`
      (other links are not followed);
    - a non-regular file whose name ends in `.rs`, such as a directory named `x.rs`;
    - a `.rs` file that cannot be read, or is not UTF-8;
    - a `.rs` file that `proc_macro2` cannot lex;
    - a marked file, or `crates/capi/src/ffi.rs`, that `syn::parse_file` cannot parse (B2b-1 widens
      this to every file that holds a `try_pop` token);
    - a `syn` `Verbatim` node in a marked file.
  - An I/O error is a typed `Result` in one process, so there is no child status to lose. These
    findings replace C11.
- **D4 (C1, C2, C10).** As today: the root argument (D1), the finding "missing realtime module",
  and the success line.

### The checks in this slice

- **D5 (C3, unsafe ownership).**
  - Lex each `.rs` file. The token `unsafe`, followed by `impl`, `fn`, `extern` or a `{ }` group,
    is a finding: "unsafe code exists outside the issue-approved ownership/audit files".
  - The allowlist is `:29` as `main` holds it at implementation time. On `6d28a80ec` and on the batch-1 branch it has 18
    paths:
    - `crates/engine/src/realtime/spsc.rs`, `crates/engine/src/realtime/disjoint.rs`;
    - `crates/lane/src/softfma.rs`, `crates/lane/src/fpenv.rs`;
    - `crates/builtins-compiler/tests/allocation_tracker.rs`, `crates/session/tests/allocation_budget.rs`;
    - `crates/soft-clip/tests/allocation.rs`, `crates/transient-shaper/tests/allocation.rs`;
    - `crates/capi/src/ffi.rs`, `crates/capi/tests/resource_lifecycle.rs`, `crates/capi/tests/plan_swap_race.rs`;
    - `crates/true-peak-limiter/tests/allocation.rs`, `crates/multiband-compressor/tests/no_alloc_render.rs`;
    - `hosts/host-web/src/ffi.rs`, `hosts/host-web/tests/boot_transient_budget.rs`;
    - `tools/bench-support/src/alloc.rs`, `tools/audit/src/capi.rs`, `tools/wasm-gate-guest/src/lib.rs`.
  - **Amended (root, 2026-10-08; see "Amendment (root, 2026-10-08)").** `Policy::workspace()`
    does not carry the four stale entries (`crates/soft-clip/tests/allocation.rs`,
    `crates/transient-shaper/tests/allocation.rs`, `crates/true-peak-limiter/tests/allocation.rs`,
    `crates/multiband-compressor/tests/no_alloc_render.rs`) or `tools/wasm-gate-guest/src/lib.rs`,
    and the awk gate's exclusion line drops the same five in this slice's commit. With
    `hosts/host-web/src/render_lock.rs` (#1333, on `main` since this spec was filed) the allowlist is
    14 paths.
  - The scan is at token level, so macro bodies are scanned, as they are today. A comment or a
    string literal that says `unsafe {` is no longer a finding. That is deliberate: neither is
    code. `unsafe trait` and `#[unsafe(..)]` stay outside the rule, as today.
- **D6 (C4, markers).**
  - **A valid marker line** meets all three conditions:
    1. it matches `^\s*// REALTIME_POLICY_(BEGIN|END)(: .*)?$` (checked by hand-written string
       tests, D2);
    2. no `proc_macro2` leaf token (an identifier, punctuation or literal) spans it. (A `Group`'s
       span covers every line inside it, so groups are not leaf tokens here; the 175 real markers
       that sit inside an `impl` pass.)
    3. it is not inside a block comment.
  - **The block-comment test.** Take the text gap between the two leaf tokens around the line. By
    lexer construction, a gap holds only whitespace and comments. A nested `/* */` scan of that gap
    is therefore exact.
  - **Any other line that contains `REALTIME_POLICY_BEGIN` or `REALTIME_POLICY_END` as a
    substring** is a finding: "malformed realtime policy marker". This covers a string literal,
    a block or doc comment, a marker after code, and `REALTIME_POLICY_BEGINNING`. A malformed
    marker must not silently drop a region.
  - **Pairing, per file.** Markers must alternate BEGIN, END, BEGIN, END.
    - Different BEGIN and END counts give "<file> has unmatched realtime policy markers", as
      today.
    - Equal counts in the wrong order give "misordered realtime policy markers". Today the gate
      checks an open tail "as it stands" (`:118`). That allowance existed only because awk could
      not pair markers.
  - **After a pairing finding**, that file has no regions. No other region rule runs on it, but
    D5 still runs. It counts toward neither floor. So a pairing finding also reports the floor
    findings whenever the count drops below a floor, which it does on the base tree (it sits on
    both floors).
- **D7 (C5, region text).** A region is the lines strictly between a BEGIN line and its END line.
  A2 adds the syntax rule (where a marker may sit, and which nodes a region holds).
- **D8 (C6, floors).**
  - The marked-file floor and the region floor are fields of `Policy`. `Policy::workspace()` holds
    the values that `main`'s floor lines hold at implementation time (`:75-76` once stream B batch
    1 is on `main`; see Dependencies).
  - The findings are "expected at least <n> marked realtime files" and "expected at least <n>
    marked realtime regions". Digits replace today's spelled-out numbers.
  - A file with no markers is not marked.

### The cases

- **D9. Case format.**
  - **The base tree** is `create_fixture` (`:10-454`), ported line for line to
    `tools/realtime-policy/tests/fixtures/base/<repo path>.rs.txt`. It sits exactly on the
    floors, with the pad files at `:437-453`. No fixture file ends in `.rs`, so neither gate
    counts its markers or its `unsafe`. `check-workspace-policy.sh`'s source-scrape rule treats
    a path through `fixtures/` as data (`:292-304`).
  - **The case policy** is `Policy::workspace()` with the floors set to the base tree's counts at
    the base tree's port (93 regions and 26 files once stream B batch 1 is on `main`) and every
    later allowlist empty. The unsafe
    allowlist keeps the real paths, because the base tree's unsafe files use them.
  - **A case** is a short list of edits on the base tree, kept in test code or in one
    `tests/fixtures/cases/<name>.txt` per case. There are four edit kinds:
    - replace one exact line (a missing anchor fails the test, as `replace_line` does);
    - append to a file;
    - create a file;
    - delete a file.
  - **The D3 conditions** (symbolic links, non-UTF-8 bytes, a directory named `x.rs`, an
    unlistable directory, a missing root) are built in test code with `std::fs`,
    `std::os::unix::fs::symlink` and `std::os::unix::fs::PermissionsExt`.
    - The unlistable-directory case checks, after `chmod 000`, that the directory really cannot
      be listed. If it can (the tests run as root), the case fails with that message. It never
      passes without having tested the rule.
  - **Materialization.** Each case builds its tree under `CARGO_TARGET_TMPDIR`, which is outside
    the scan roots. A guard removes the tree when the test ends, even when it fails. The repo
    treats a leftover temporary directory as a defect (#1421, #1429).
  - **Assertion.** Each case asserts the exact set of finding classes. The tool reports every
    finding, where the script stopped at the first, so a shape with two defects asserts both.
- **D10. Library cases (`tests/`), ported.**
  - **Valid trees:** `valid`, `empty-bodies`, and `valid` given as a relative root (the `run`
    case in D11).
  - **`open-tail-valid` becomes an expected failure,** {misordered realtime policy markers, the
    file floor, the region floor}. It appends END and BEGIN to `pad_1.rs`, so that file's 13
    regions drop out (D6). Root accepted the class change (2026-10-05). (The case also puts an
    unmarked `oracle_drain` at the top of every marked file; B2b-1 adds a class for it.)
  - **Unsafe (17):** every `unsafe-*` case at `:542-581`.
  - **Markers and floors (4):**
    - `marked-file-count-floor`: {the file floor, the region floor};
    - `no-marked-files-uses-floor`: {the file floor, the region floor};
    - `marked-region-count-floor`: {the region floor};
    - `unmatched-markers-outside-root`: {unmatched realtime policy markers, the file floor, the
      region floor}.
  - **Retired, with root's acceptance (2026-10-05):** the 22 tool-failure injections and the 5
    counter-mutants (`:1437-1515`). Their defect, a child-process status that is lost, cannot
    exist in one Rust process. The D3 cases below replace them.
  - The forbidden-body and whole-plan cases go to A2; the drain cases go to B1a and B1b. The base tree's
    forbidden spellings and drain shapes are ported as they are and pass, because this slice runs
    neither rule.
- **D11. New cases, one each.**
  - **`run`, with the case policy** (D1):
    - the valid tree gives exit code 0 and exactly the C10 line on `out`;
    - a failing tree gives exit code 1 and a `realtime policy failure: <class>` line on `err`;
    - a relative root is resolved against `cwd`;
    - no argument means `cwd`;
    - two arguments give exit code 2.
  - **The binary** (`CARGO_BIN_EXE_realtime_policy`), on paths that do not depend on the policy:
    - a root whose `tools` directory is missing gives exit 1 and a failure line on stderr;
    - two arguments give exit 2.

    The binary's success path on the real policy is gate 1.
  - **D3:**
    - a missing `tools` root;
    - an unlistable directory;
    - a symbolic link to a directory, and a `.rs` symbolic link;
    - a directory named `x.rs`;
    - a non-UTF-8 `.rs` file;
    - a lex error;
    - a parse error in a marked file;
    - a `target` directory and a `node_modules` directory that hold a bad `.rs` file (passes:
      they are skipped).
    - a marked file that holds a `Verbatim` item. If `syn` 2.0.119 parses no input as
      `Verbatim`, record that in the Evidence and drop the case. Keep the refusal.
  - **D5:** an `unsafe {` inside a `macro_rules!` body in a file outside the allowlist.
  - **D6:**
    - a marker in a string literal, in a doc comment, in a `/* */` block comment, and after
      code;
    - `REALTIME_POLICY_BEGINNING`;
    - an END before its BEGIN.

### CI and delivery

- **D12. CI runs the tool's release binary beside the awk gate.**
  - **Job.** In `.github/workflows/qualification.yml`, job `audit-native`:
    - add `-p realtime-policy` to the release build step (`:743-744`, "Build release audit, bench,
      capi, and session-validator binaries"), and add "realtime-policy" to that step's name;
    - after it, add a step "Realtime source policy (tools/realtime-policy)" that runs
      `./target/release/realtime_policy`.

    This job already builds every tool in release once for all its steps (`:739-742`). Putting
    the binary there avoids a debug `cargo run` in `lint`, which `lint`'s comment (`:507-514`)
    records as the pattern the repository removed.
  - **Tests.** The tool's tests run in `test-debug-a`. Its
    `cargo test --locked --workspace --all-targets` (`:626-634`) does not exclude the new package.
  - **The awk lines stay.** `lint`'s `:529-530` stay until slice C2.
  - **No checker changes in this slice.**
    - `scripts/check-ci-path-routing.py` names neither realtime script nor `tools/`. `realtime` is
      not in `SELF_TEST_SUITES` (`:52-63`). `check_qualification_closures` (`:452-464`) and
      `DEDUPLICATED_OWNERS` (`:741-753`) do not touch these steps. (C2 adds the tool's step to
      `DEDUPLICATED_OWNERS`.)
    - The router sends `tools/realtime-policy/**` to the full route (`classify_paths`,
      `:122-133`).
    - `scripts/check-script-reachability.py` is unaffected: no file under `scripts/` changes.
    - The verdict job's expectation table does not change, because no job is added.
- **D13. Delivery (root, 2026-10-05).** The nine-slice plan is accepted, and the whole set lands
  in one J batch: each slice is a local checkpoint commit on the batch branch, with its own
  verdict, and the branch is pushed once, at the batch boundary. The batch also carries the #1448
  guard commit, after C2 and before B2b-1. B2b-1 waits for the three production issues and B2b-2
  for #1345 (Amendment 1), so the batch boundary waits for them on `main`.
  - Inside the batch, both gates run until C2's commit, and the Hazards' two-gates rule applies to
    each commit before it. No CI run sees a commit between A1 and C2, because the batch pushes
    once. The gap is still safe: each slice's local gates run the awk gate, every slice before C2
    lands in the same push as C2, and no slice reaches `main` without it.

## Authorized paths

- `tools/realtime-policy/**` (new)
- `Cargo.toml` (one `members` entry) and `Cargo.lock` (the new package's entry; no version
  changes). `Cargo.lock` is a cross-stream exception: STREAMS gives it to stream C for #1320.
- `scripts/check-realtime-policy.sh`: the `unsafe source exclusions` line only (the Amendment's five
  removals, so the two gates keep one allowlist until C2).
- `.github/workflows/qualification.yml` (D12's two edits in `audit-native`). This is a
  cross-stream exception: H #1334 edits the workflows, and J #1422, J #1429 and J #1435 edit other
  steps of this file.
- This spec

Root names each cross-stream exception in STREAMS.md (D15-0). The rows are in
`docs/handoffs/decision-15-2026-10-05/STREAMS.md`.

## Non-goals

- A2's checks, the drain rule (B1a, B1b, B2a, B2b-1, B2b-2) and deleting the awk gate (C2).
- Any Rust source outside `tools/realtime-policy`.

## Hazards

- **Floors and the allowlist come from `main` at implementation time.** Stream B batch 1 (#1309,
  #1343, #1314, #1311, #1348; #1314's Amendment 1 raises the floors to 26 files and 93 regions)
  must be on `main` first. Copy `:29` and the two floor lines (`:75-76` on `b8392df66`) as they
  are then.
- **Two gates hold one allowlist, one pattern list and one pair of floors until C2.** Any slice
  that lands between A1 and C2 and changes the unsafe allowlist, the forbidden patterns or a floor
  changes both gates. A floor raise in the awk also raises the awk self-test's base tree, which
  sits exactly on the floors: its pad loop, the pad comment and the matching floor message, as
  `98a2d6bfc` authorized for #1314 (STREAMS' standing exception names this). An open spec that adds a marker or raises a floor today names only the
  script: #1321 (`:55`).
- **Re-measure on rebase.** Each slice that changes the marked regions re-measures the floors when
  it lands, whatever landed before it: the tool slices, stream B's cell slices (#1312, #1346,
  #1347, #1345), #1321 and the production issues that mark a function.
- **Every later edit of the floors or the allowlists is in `Policy::workspace()`.** Once the J
  batch is on `main` (it pushes once, with C2 in it), a slice in any stream that adds a marker
  raises the floors there only. Until then `main` has only the awk gate, so a slice outside the
  batch raises the awk floors and its self-test, and a slice inside the batch before C2 raises
  both. After B2b-1, a slice that adds a control-side `try_pop` adds its allowlist row there. STREAMS names this
  as a standing exception for every stream (STREAMS).
- **The awk gate scans the tool's own sources until C2.**
  - The tool's `.rs` files must not contain text that the awk unsafe regex
    (`unsafe[[:space:]]+(impl|fn|extern)|unsafe[[:space:]]*\{`) or the marker words match, even in
    comments, doc comments and strings. Build those strings with `concat!`.
  - Fixtures are `.rs.txt` files (D9), which the awk does not scan.
- **The tool reads Rust source as text. That is its job.** It is a gate, like the script it
  replaces. AGENTS.md's test-value rule refuses *tests* that grep source. The tool's tests read
  only fixtures, and the binary reads the tree it is given. Build its paths at run time
  (`Path::join`).
- **Over-refusal is acceptable, a false pass is not.** Where the tool cannot classify something,
  it refuses (D3, D6).

## Objective gates

1. **The real tree.** `cargo run --locked --release -q -p realtime-policy` prints
   `realtime policy: ok (R marked regions in F files)`. R and F equal the counts that
   `bash scripts/check-realtime-policy.sh` prints on the same tree (93 and 26 once stream B batch 1
   is on `main`; re-measured at implementation time).
2. **The cases.** `cargo test --locked -p realtime-policy` passes, with every D10 and D11 case.
3. **Parity (PR evidence).** A throwaway script outside the repository materializes each committed
   case's tree and runs `scripts/check-realtime-policy.sh` (gawk) and the tool on it. The
   pass/fail verdicts agree, except for these deliberate differences. Name the case that shows
   each one:
   - D3: a missing root (both fail, by different classes); unlistable directories, symbolic
     links, non-UTF-8 files, lex and parse errors, and `Verbatim` are new findings; `target` and
     `node_modules` are skipped where `rg` skips only what `.gitignore` names;
   - D5: `unsafe {` in comments and literals;
   - D6: malformed and misordered markers, including `open-tail-valid`;
   - D8: the message text;
   - the base tree's forbidden spellings and drains, which this slice does not judge.
4. **Mutations (PR evidence).** Apply each alone. Each turns at least the named case red:
   - remove `crates/engine/src/realtime/spsc.rs` from the allowlist: the valid base tree, and
     gate 1 on that file;
   - drop the D6 block-comment test: the block-comment marker case;
   - treat a group's span as a leaf token's (D6 condition 2): gate 1 refuses real markers;
   - treat an unreadable or missing path as empty: the missing-root and unlistable-directory
     cases;
   - accept misordered markers: the END-before-BEGIN case;
   - lower a floor by one: the matching floor case;
   - have `run` return 0 after findings: the failing `run` case;
   - have `main` ignore `run`'s code: the failing binary case.
5. **Run time (PR evidence).** Record the tool's time on the real tree, and on one synthetic
   marked file of 20,000 lines.
6. **Shipped artifacts unchanged.**
   - The `artifact-identity` job reports the worklet module UNCHANGED against the base.
   - PR evidence: build `cargo build --locked --release -p capi` at the parent and at the change,
     in one checkout. The sha256 of `target/release/libcapi.a` and `target/release/libcapi.so` are
     identical. Build the parent twice first to show the digests are reproducible. If they are
     not, record that and use `cargo tree -e normal,features -p capi` equality instead.
7. `cargo fmt --all -- --check`,
   `cargo clippy --locked -p realtime-policy --all-targets -- -D warnings`,
   `bash scripts/check-workspace-policy.sh`, `bash scripts/check-realtime-policy.sh`,
   `python3 -B scripts/check-ci-path-routing.py`, `python3 -B scripts/test-ci-path-routing.py` and
   `python3 -B scripts/check-script-reachability.py` all exit 0.

*Test value.*
- The ported cases keep the claims that their script rows made.
- The `run` cases are red if a finding's exit status is lost or the root is resolved wrongly.
  The two binary cases are red if `main` does not pass `run`'s code to the process. Every CI result
  depends on that, and no `check` case reaches it.
- The D3 cases are red if a missing root, an unlistable directory, a link, an I/O, lex or parse
  failure becomes a skip, which is the fail-open shape C11 exists to stop. They replace the
  retired tool-failure injections.
- The D6 cases are red if a malformed, misordered or commented-out marker is read as a region.
- The D5 macro case is red if the unsafe scan skips macro token trees.

## Evidence

- Gates 1-7 output. Gate 3's table of verdicts and differences. Gate 4's mutation runs.
- The `audit-native` step time.

## Dependencies

- **After (other streams):** stream B batch 1 (#1309, #1343, #1314, #1311, #1348), which raises
  the floors (Hazards).
- **Not ordered against** stream B's cell slices #1312, #1346, #1347 and #1345 (root's ruling,
  2026-10-05). The re-measure rule in Hazards applies.
- **Before:** A2.
- **STREAMS rows:** `docs/handoffs/decision-15-2026-10-05/STREAMS.md` (root edits it).

## Standing rules for the implementer

- Work only from this body. Read the cited lines first; do not survey the workspace.
- A test that greps source or prose is refused. The tool's tests run the library and the binary
  on fixture trees, which are the tool's own input.
- Attempt budget: three attempts, one adversarial verdict each.
- Size: half a day.

## Amendment (root, 2026-10-08)

Filed from #1478's two verdicts, which found four allowlist entries that approve nothing and an
inert `wasm-gate-guest` entry (`/home/bl/misofm/submix-verdicts/1478-attempt1.md`, "Stale entries";
`1478-attempt2.md`, open item 3). This slice has not started: `tools/realtime-policy` does not
exist and no implementation commit names #1438 (only its filing, `c45f48691`, and a merge that
carried it). Root's ruling (3), verbatim:

> (3) (b): amend #1438's D5 now (the slice has not started): drop the four stale allowlist entries
> and remove the wasm-gate-guest entry if it allows nothing reachable (state the evidence). Body
> syncs join the pending list.

**Decision: all five entries go.** D5 is amended in place (its "Amended" bullet).

- **The four stale entries.** `crates/soft-clip/tests/allocation.rs`,
  `crates/transient-shaper/tests/allocation.rs`, `crates/true-peak-limiter/tests/allocation.rs` and
  `crates/multiband-compressor/tests/no_alloc_render.rs` contain no `unsafe` and no
  `allow(unsafe_code)`; they count through `bench_support::alloc`. Their wrappers were removed by
  `568ad4087`, `cb4898437` (#1046), `a6da0cade` and `39c4651b1` (`git log -S'unsafe impl GlobalAlloc'
  --follow`). On `a059cdd03` the awk gate's regex
  (`unsafe[[:space:]]+(impl|fn|extern)|unsafe[[:space:]]*\{`) matches 0 lines in each. Today each
  entry would let a later change add `#![allow(unsafe_code)]` and unsafe code to that file and pass
  the gate unreviewed.
- **`tools/wasm-gate-guest/src/lib.rs`: the entry allows nothing.**
  - The file's only `unsafe` tokens are eleven `#[unsafe(no_mangle)]` attributes (`:57`-`:220`)
    and its `#![allow(unsafe_code)]` (`:15`). The awk regex matches 0 lines in it, so the realtime
    allowlist entry suppresses nothing today. D5's token rule (`unsafe` followed by `impl`, `fn`,
    `extern` or a `{ }` group) also leaves `#[unsafe(..)]` outside the rule, so under the tool the
    entry would suppress nothing either.
  - The binding approval of this file's unsafe boundary is `scripts/check-bench-policy.sh:209-229`:
    the exact three-file `tools/` set allowed to carry `#![allow(unsafe_code)]`, with the reason
    (a `cdylib` export needs `#[unsafe(no_mangle)]` under edition 2024 and has no safe spelling).
    That check is unchanged and still pins the file.
  - Nothing in it is reachable from a render path: it is the wasm gate guest built only by
    `scripts/run-wasm-gates.sh:73`, linked into no shipped artifact.
  - Removing the entry makes a real unsafe block, fn or impl added to the file a realtime-gate
    finding that needs a decision, which is the intent of an exact allowlist.
- **Both gates.** Until C2 the awk gate and the tool hold one allowlist (Hazards), so this slice
  removes the same five from `scripts/check-realtime-policy.sh`'s exclusion line (Authorized
  paths). Neither self-test names these paths.
- **Gate addition.** Gate 4 gains one mutation, applied alone: add `unsafe {}` to
  `crates/soft-clip/tests/allocation.rs` in the base tree (or a case tree); both the tool and the
  awk gate report it.
- The doc's "Unsafe-code ownership" section is #1489's; it lists the allowlist as `main` holds it.

