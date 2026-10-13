# Read workspace manifests and build config as TOML for the realtime tool's claims

Stream J of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`,
D15-0). A successor slice of the realtime-policy tool program (#1438 lists the slices). It carries three
gaps that #1442 attempt 3 (FAIL, 2026-10-12) and #1519 attempt 3 left open, all of one kind: the tool's
claim is "every token of every `.rs` file under `crates`, `hosts` and `tools`, and every file rustc reads as
source from them" (#1438 Hazards, threat model), and a line-based text rule over `Cargo.toml` or
`.cargo/config.toml` does not protect that claim.

## Product outcome

A Cargo manifest and a Cargo build configuration cannot change what the tool judges or what rustc builds
from it, by any direct written form. The rules read the files with a TOML parser, the way Cargo does, so no
spelling (inline table, dotted key, spaced or quoted header, escape) escapes them.

## Problem (verified on `0436b10e7`, the head of `codex/d15-stream-j3`, rustc and cargo 1.97.1, Python 3.12.3)

1. **The dependency-key rule is line-based.** `scripts/check-workspace-policy.sh` (the `sysroot-dependency-scan`
   awk) reads a `[..]` header line and a `key =` line. A dependency key `core`, `std` or `alloc` makes Cargo
   pass `--extern core=..` and redirects the prelude's `core` for the whole crate. Five direct forms defeat
   the awk (#1442 attempt-3 verdict, `/home/bl/misofm/submix-verdicts/1442-attempt3.md` F1; 23 manifests in
   `/home/bl/misofm/submix-verdicts/evidence/1442-attempt3/manifest/`, 13 of them pass the rule; Cargo passes
   `--extern core=<fake>` for each, `manifest/cargo-results.txt`):
   - (i) an inline table: `dependencies = { core = { .. } }` before `[package]`, or under `[target.'cfg(unix)']`;
   - (ii) a dotted key: `dependencies.core = { .. }`, `target.'cfg(unix)'.dependencies.core = ..`;
   - (iii) spaces around the dots of a header: `[ dependencies . core ]`, `[target . 'cfg(unix)' . dependencies]`;
   - (iv) a quoted table name: `["dependencies"]`, `["dependencies".core]`;
   - (v) an escaped quoted key: `"core" = { .. }` written with a TOML escape (`"core"`), `["dependencies"]`.
   The effect is real: a `std` key pointing at a `no_std` facade rebinds `debug_assert`, which B1a-D7 trusts
   (`/home/bl/misofm/submix-verdicts/evidence/1442-attempt3/std-probe/`). The same awk accepts the lookalike
   `test = false` (a `[[bin]]` flag, which `fuzz/Cargo.toml` has six times) and must keep accepting it.
   Prototype (Python `tomllib`, ~25 lines, scratch only): refuses all 23 evidence manifests and none of the
   43 tracked `Cargo.toml` files of the real tree.
2. **Scan-root containment of manifests (measured on the real tree).** The walk reads `.rs` files under
   `SCAN_ROOTS = ["crates", "hosts", "tools"]` (`tools/realtime-policy/src/walk.rs:12`, used by
   `Policy::workspace()` through `rust_files`). Measured by parsing every tracked `Cargo.toml` (43) and by
   `cargo metadata --locked --offline` on the root workspace and on `fuzz/Cargo.toml`:
   - the root workspace has 40 members (`Cargo.toml` `[workspace] members`, no globs, no `exclude`, no
     `default-members`), and `cargo metadata` lists 40 path packages: all 40 lie under the roots
     (29 under `crates/`, 1 under `hosts/`, 10 under `tools/`);
   - path dependencies: 45 `path =` keys in all tracked manifests. 34 are in the root `[workspace.dependencies]`
     (every `crates/*`, `hosts/*` and `tools/*` manifest inherits them with `workspace = true` and has no own
     `path =`); all 34 resolve inside the roots. No `[patch]` or `[replace]` table exists. **No member and no
     path dependency of a scanned crate lies outside the roots.**
   - three tracked manifests lie outside the roots, and none is a member or a dependency of a scanned crate:
     `Cargo.toml` (the virtual workspace root, no `[package]`), `fuzz/Cargo.toml` (`session-fuzz`; own
     `[workspace] members = ["."]`; 2 path dependencies, `../crates/session` and `../crates/protocol`; 6 `[[bin]]`
     targets at `fuzz/fuzz_targets/*.rs`, 7 `.rs` files, built by `fuzz.yml` and `nightly.yml`), and
     `docs/handoffs/control-smoothing-defaults/measure/Cargo.toml` (`control-smoothing-measure`; own empty
     `[workspace]`; 9 path dependencies of the form `../../../../crates/<name>`; 11 `.rs` files). Their
     dependency direction is outward-in (they depend on scanned crates), so the tool's claim about the scanned
     source holds. Their own `.rs` files are not scanned and are no shipped product.
   - Result: **0 containment violations today; the scan roots need no change.** The rule would not refuse the
     tree; it refuses the next member or path dependency that points out of the roots (for example
     `../../vendor/fakestd`, the shape of the 1442 `std-probe`), where rustc would compile source that the
     walk never reads.
3. **Build configuration injection (measured).**
   - Tracked config: one file, `.cargo/config.toml`, one flag line:
     `[target.'cfg(target_arch = "x86_64")'] rustflags = ["-C", "target-feature=+avx2,+fma"]`. No `rustdocflags`,
     no `[env]`, no `include`.
   - `RUSTFLAGS`/`RUSTDOCFLAGS` settings: `.github/workflows` 12 (`fuzz.yml` 106, 111; `nightly.yml` 119;
     `qualification.yml` 475 (`RUSTDOCFLAGS='-D warnings'`), 585, 591, 597, 737, 926, 930, 992, 1019) and
     `scripts` (non-test) 12 (`build-web-audioworklet.sh:113`; `check-cross-targets.sh:180` and five uses of
     `wasm_flags` at 194-217; `check-protocol-wasm-parity.sh:89,115`; `run-protocol-fuzz.sh:28`;
     `run-wasm-gates.sh:72`; `check-browser-expected-resources.py:367`). Their values are `-C target-feature`,
     `-C target-cpu`, `-C link-arg`, `-C strip`, `--remap-path-prefix`, `--cfg loom` and `-D warnings`. No
     `CARGO_ENCODED_RUSTFLAGS`, `CARGO_BUILD_RUSTFLAGS`, `CARGO_TARGET_*_RUSTFLAGS`, `RUSTC_BOOTSTRAP` or
     `RUSTC_WORKSPACE_WRAPPER` appears.
   - **No `crate-attr` (any spelling) appears in `.cargo`, `.github/workflows` or `scripts`: 0 occurrences.**
     `-Zcrate-attr=feature(..)` is the one way a build flag injects a crate attribute (`#![feature(..)]`,
     `#![no_std]`, `#![no_core]`, ...). Stable rustc 1.97.1 refuses `-Z`, so it matters on the nightly jobs
     (`+nightly-2026-08-20` in `fuzz.yml` and `nightly.yml`) and wherever `RUSTC_BOOTSTRAP` is set.
   - The existing `.cargo` rule is a `rg` line match with one exact allowed line. It does not match
     `rustdocflags` (checked: `rustdocflags = ["-Zcrate-attr=feature(x)"]` gives `rg` status 1, so it
     is not flagged), and Cargo's config `include = ["other.toml"]` pulls flags from a file the rule never
     reads (checked on cargo 1.97.1: a `--cfg` in an included file reaches the `rustc` command line).
     Workflows and scripts have no rule at all.

## Decisions

- D1. **Dependency keys, read as TOML.** `scripts/check-workspace-policy.sh` reads every tracked `Cargo.toml`
  (the list it already builds, `tracked_paths`) with Python `tomllib` and refuses, with the manifest, the table
  and the key named:
  - (a) a key, in any of `dependencies`, `dev-dependencies`, `build-dependencies` (and the `dev_dependencies`
    and `build_dependencies` spellings Cargo still reads), of the top level, of every `target.<cfg>` table
    (every `<cfg>` name, recursively), and of `workspace.dependencies`, that equals `core`, `std` or `alloc`
    after `-` is turned into `_` (Cargo does this for the extern name);
  - (b) a `package = "core" | "std" | "alloc"` rename in a dependency value of those tables (over-refusal: the
    extern name is then the key, which is harmless; refused anyway, as an unneeded spelling of the same trap);
  - (c) a manifest that `tomllib` cannot parse (a finding, never a skip), and a dependency table that is not a
    table.
  The line-based awk (`sysroot-dependency-scan`) is deleted in the same change. The retired delivery-codec scan
  (`retired-scan`) is not touched. The name set stays the three of #1442 root Amendment 4; widening it to
  `proc_macro` and `test` (which the package-name rule already bans) is root's ruling, not this slice's.
- D2. **Python is required and the rule fails closed without it.** The reader runs under `python3` and imports
  `tomllib` unconditionally (Python 3.11 or newer; `ubuntu-24.04` has 3.12). A missing interpreter, a missing
  module (Python below 3.11) or any non-violation exit status is an execution failure of the gate (the script's
  `capture`/`execution_failure` pattern: status 3 is a violation, any other non-zero status is a failure of the
  gate itself). No `try: import tomllib except: pass` and no fallback to the awk.
- D3. **One reader.** If #1519 attempt 3 (commit `0436b10e7`, under verification at the time of writing) has
  landed `manifest_target_checker` (the embedded `tomllib` pass for target `path`/`build` keys), this slice
  extends that pass (one parse per manifest, one violation status). If it has not landed, this slice owns the
  reader and #1519's target-path rule then rebases onto it. State which case applies in the attempt record.
- D4. **Containment of members and path dependencies.** In the same reader, with the scan roots read from
  `SCAN_ROOTS` in `tools/realtime-policy/src/walk.rs` (as `SKIPPED_DIRECTORIES` is read today; the read fails
  closed if the constant cannot be found):
  - (a) every entry of the root `Cargo.toml` `[workspace] members` (a glob expanded on disk) must resolve, after
    `..` and symlinks are resolved, to a directory under a scan root;
  - (b) every `path =` of a dependency in any manifest under a scan root (`dependencies`, `dev-dependencies`,
    `build-dependencies`, `target.<cfg>.*`, `workspace.dependencies`, `[patch.*]`, `[replace]`) must resolve, on
    the same terms, under a scan root; a `path =` in the root `Cargo.toml` `[workspace.dependencies]` counts;
  - (c) a manifest outside the roots is not scanned by (b) (`fuzz/Cargo.toml` and the `docs/handoffs/...`
    measurement harness stay accepted: not members, own `[workspace]`, depending inward). (a) still refuses it if it were
    listed as a member. [Replaced by Root Amendment 1 (iii): (b) now reads every tracked manifest, `fuzz/` and
    the `docs/handoffs` harness included; the member rule (a) applies to the root workspace only.]
  The roots do not change (measured in Problem 2). If a later issue needs a member outside them, it adds the
  root to `SCAN_ROOTS` in the tool, not an exception here.
- D5. **Build configuration.** The same script reads (with `tomllib`) every tracked file named `config` or
  `config.toml` in a `.cargo` directory at any depth, and refuses:
  - (a) any string, key or array item in it that, with `_` turned into `-` and lowercased, contains
    `crate-attr` (covers `-Zcrate-attr=..`, `-Z crate-attr=..` as two array items, `--crate-attr`, `[env]` values
    and `rustflags` given as a string or an array) or `rustc-bootstrap`;
  - (b) a `rustflags` or `rustdocflags` item that starts with `@` (rustc reads an argument file the rule does
    not see);
  - (c) an `include` key (Cargo reads another file, possibly outside `.cargo`);
  - (d) a file that does not parse.
  The present ISA-pin rule (exact `rustflags` line, scoped target table, no `[build]` table) is not changed.
- D6. **Workflows and scripts.** The script refuses the words `crate-attr` and `crate_attr` (case-insensitive) and
  `RUSTC_BOOTSTRAP` anywhere in a tracked file under `.github/workflows/` and `scripts/`, other than in the
  policy script and its self-test (which build the words from fragments, as the existing
  `prelaunch_later_generation_pattern` does, so they do not contain them). The refusal is on the word, not on
  the assignment, so a flag assembled in a shell variable and passed later is also refused. Measured: 0 hits
  today (the only repository mention is an unrelated word in a `.github/ISSUE_SPECS` file, outside the path).
- D7. **Limits (written, as #1438 Hazards limit 4 does for macros).** The rules judge written forms. A word
  assembled from parts at run time (`"crate-""attr"` joined by the shell), a flag file generated by a script, a
  `$CARGO_HOME/config.toml` or `RUSTFLAGS` set outside the repository (a developer's shell or the runner image)
  are not read. Whether this is a stated limit of the tool is root's ruling; this slice adds nothing to #1438's
  list.

## Cases (each refused unless marked accepted) and their test value

Self-tests are in `scripts/test-workspace-policy.sh`, one mutation function per row, in the style of
`mutate_sysroot_*`. Each row is red when its check is dropped (Objective gates, 3).

Dependency keys (D1):
- (i) `[package]`-less prefix `dependencies = { core = { package = "spin", version = "0.9" } }` placed before
  `[package]`, and `[target.'cfg(unix)'] dependencies = { std = ".." }`: test value, a reader that looks only at
  `[dependencies]` headers lets an inline table pass (the 1442 attempt-3 false pass).
- (ii) `dependencies.core = ".."` and `target.'cfg(unix)'.dependencies.alloc.package = ".."`: a reader that
  needs a header line for the table misses a dotted key.
- (iii) `[ dependencies . core ]` and `[target . 'cfg(unix)' . dependencies]` with `alloc = ".."`: a regex on the
  header text with no spaces misses spaced dots.
- (iv) `["dependencies"]` with `std = ".."` and `["dependencies".core]`: a regex on the bare word misses a
  quoted table name.
- (v) `"core" = ".."` and `["dependencies"]`: a text reader misses a TOML escape; a parser sees the key.
- `[dev-dependencies]`, `[build-dependencies]`, `[dev_dependencies]`, `[workspace.dependencies]` each with one of
  the three names; a `core` key in a dependency table of a manifest nested deeper than the roots (a
  `crates/a/b/Cargo.toml`): red when that table kind or that directory depth is dropped.
- `x = { package = "core", version = "1" }` (D1 (b)); a manifest that is not valid TOML (D1 (c)).
- Accepted (the lookalikes; red if the reader over-matches): `[[bin]] test = false`, `core_affinity = "0.8"`,
  `standard = "1"`, `[package.metadata.notes] core = "documented"`, a `core` mention in a comment, a `core`
  key in `[features]`, and a `[target.'cfg(unix)'.dependencies]` table with an ordinary name.

Containment (D4):
- a member `"../outside"`; a member glob that expands to a directory outside the roots (a `fuzz` directory
  matched by `*`); a symlinked member whose real path is outside: red when (a) is dropped.
- a path dependency `../../vendor/fakestd` in a scanned crate's `[dependencies]`, in
  `[target.'cfg(unix)'.build-dependencies]`, in `[patch.crates-io]` and in the root `[workspace.dependencies]`:
  red when the table kind or the patch table is dropped from (b).
- Accepted: a path dependency that goes up and back into the roots (`../../crates/sibling`); a path dependency
  between `crates/` and `hosts/`; a manifest outside the roots that has its own `[workspace]` (the `fuzz` shape).
- A scan-root constant that cannot be found makes the gate fail (not pass).

Build configuration (D5, D6):
- `.cargo/config.toml` with `rustflags = ["-Z", "crate-attr=feature(x)"]`, `rustflags = "-Zcrate-attr=no_std"`,
  `[env] RUSTFLAGS = "-Zcrate_attr=.."`, `rustdocflags = ["-Zcrate-attr=.."]` under `[build]`,
  `rustflags = ["@args.txt"]`, `include = ["../elsewhere.toml"]`, a config file that is not valid TOML, and a
  nested `crates/x/.cargo/config.toml` with any of them: red when (a), (b), (c), (d) or the "any depth" walk is
  dropped.
- A workflow step `RUSTFLAGS: -Zcrate-attr=feature(x)`; a script line
  `flags="-Zcrate-attr=.."; RUSTFLAGS="$flags" cargo ..`; an `export RUSTC_BOOTSTRAP=1` in a script: red when D6 is
  dropped, or when it is limited to lines that contain `RUSTFLAGS`.
- Accepted: the present `.cargo/config.toml`; every present `RUSTFLAGS` line of the workflows and scripts; the
  policy script and its self-test (which name the words only in fragments).

Fail closed (D2): the policy script run with a `python3` on `PATH` that lacks `tomllib` (a stub that exits 1 on
`import tomllib`) fails and names the dependency; red when the import is wrapped to continue.

## Authorized paths

- `scripts/check-workspace-policy.sh`: the awk rule it replaces (D1) and the new D4, D5 and D6 rules live here,
  next to the #1442 and #1519 manifest rules; it already builds the tracked-manifest list the reader needs.
- `scripts/test-workspace-policy.sh`: the self-tests of the rows above, and the deletion or rewrite of the four
  `mutate_sysroot_*` cases only where the awk's message text changes.
- This spec.
- `tools/realtime-policy/src/walk.rs`: the doc comment of `SCAN_ROOTS` only (one sentence saying that
  `scripts/check-workspace-policy.sh` reads the constant), as the comment of `SKIPPED_DIRECTORIES` does; no code.
  If #1519 has not landed that sentence for `SKIPPED_DIRECTORIES`, nothing else in the tool changes either.
  [Widened by Root Amendment 1: `tools/realtime-policy/**` is authorized for rule (iv), the doctest rule; see
  "Authorized paths this amendment needs".]
- `.github/ISSUE_SPECS/1438-*.md` Hazards: one sentence that this slice closes the manifest and build-config
  forms listed there, and nothing else (the threat model text is root's).

## Non-goals

- Reading `Cargo.lock`, resolving the registry, or `cargo metadata` as the source of truth (it needs the network
  or a lockfile at gate time; the TOML reader does not).
- Widening the extern-name set beyond `core`, `std`, `alloc`; changing `SCAN_ROOTS`; scanning `fuzz/` or
  `docs/` sources; any change to the realtime-policy checks [amended by Root Amendment 1: rule (iv) adds a
  doctest rule to the realtime-policy tool, and only that].
- The `retired-scan` of delivery-codec identities, the licence and name rules, and the existing ISA-pin rule.
- Any CI workflow edit (`qualification.yml` already runs both scripts at its "Workspace policy" steps).
- A rule for `$CARGO_HOME` or the environment of a developer's shell (D7).

## Hazards

- **Threat model: #1438 Hazards, "root Amendment 2, 2026-10-11".** The tool fails closed over what it judges; over-refusal
  is acceptable and a false pass is not. D1 (b), D5 (b) and D6 over-refuse on purpose. Each rule refuses the
  direct written forms; run-time assembly is D7.
- **A parser that skips instead of failing.** Every `tomllib` error, unreadable file, missing constant and
  non-table value is a finding or a gate failure, never a skipped manifest (D1 (c), D2, D4). A self-test per
  class.
- **A path that normalizes to a lookalike.** Resolve with `..` and symlinks applied (`os.path.realpath`) before
  the comparison with the roots, and compare path components, not string prefixes (`crates-old/` is not
  `crates/`).
- **Untracked files.** [Root Amendment 1 (i) uses a cached-only list for the source-placement rule; the rest
  of this item stands for D1, D4 and D5.] The script already lists tracked and untracked-not-ignored paths (`tracked_paths`); the
  new rules read that list so a new, uncommitted manifest is covered. A fixture tree without git uses the same
  `find` fallback.
- **Two slices edit one script.** #1519 attempt 3 edits the same file. The later slice rebases (the earlier
  assignment of edit order is root's, D3).
- **Self-test fixtures write the forbidden words.** Build `crate-attr` and `RUSTC_BOOTSTRAP` from fragments in
  the policy script and the self-test, or D6 refuses its own gate.

## Objective gates

1. **Real tree.** `bash scripts/check-workspace-policy.sh` prints `workspace policy: ok`. Re-measure and record:
   43 tracked manifests read, 0 dependency-key refusals, 0 containment refusals (40 members, 34 workspace path
   dependencies, all inside the roots), 1 config file read, 0 flag refusals; the `RUSTFLAGS`/`RUSTDOCFLAGS`
   counts of Problem 3 (12 + 12) unchanged or explained.
2. **Self-tests.** `bash scripts/test-workspace-policy.sh` passes with every row of Cases. At least one refused
   self-test per form (i) to (v), per table kind, per containment source (member, dependency, patch), per config
   rule (a) to (d), D6, and the fail-closed Python row; and every accepted lookalike.
3. **Mutations (PR evidence), each alone, with the real tree still passing; each turns red only the rows named
   for it.** For each row of Cases, one mutation drops the check that the row's test-value sentence names:
   the five forms (revert D1 to the old awk: forms (i) to (v) red together and the lookalike green; then
   drop `target.<cfg>` recursion, `dev-dependencies`, `build-dependencies`, the `_` spellings,
   `workspace.dependencies`, the `package =` check, the escape-aware parse by reading the key text instead),
   containment ((a), (b) per table kind, `[patch]`, glob expansion, symlink resolution, the component-wise
   comparison replaced by a string prefix), the config rules ((a) to (d), the depth walk) and D6 (the word
   check limited to `RUSTFLAGS` lines), and D2 (the import wrapped to continue). Record each mutation, the row
   that turned red and that no other row did.
4. **Replay (PR evidence, throwaway, not committed).** Run the new reader on the 23 manifests in
   `/home/bl/misofm/submix-verdicts/evidence/1442-attempt3/manifest/`: all refused; on the real tree's 43
   manifests: none refused.
5. **Test value.** Each new or rewritten test in the PR carries its one-sentence answer to "which plausible
   defect turns this red that no existing test catches" in the attempt record.
6. `cargo run --locked --release -q -p realtime-policy` and `cargo test --locked -p realtime-policy` unchanged (the tool is
   untouched except for one doc comment [amended by Root Amendment 1: the tool gains rule (iv); gate 6 below is
   replaced by "Objective gate changes", gate 6]); `cargo fmt --check` and `cargo clippy -p realtime-policy --all-targets
   -- -D warnings` exit 0.

## Dependencies

After (same stream): #1519 (attempt 3, commit `0436b10e7`, under verification), because both edit
`scripts/check-workspace-policy.sh` and `scripts/test-workspace-policy.sh` and D3 reuses its reader if it
lands. If #1519 spends its budget without PASS, this slice owns the reader and #1519's manifest target-path rule
(root Amendment 2) is split into it or rebased on it, as root rules. Closes the limits that #1442 attempt 3 F1
names for the successor; #1442 root Amendment 4's "dependency key" sentence points here.

## Budget, size and order

Three attempts; about half a working day (one Python reader, three rule groups, about 30 self-test rows, no Rust
code [amended by Root Amendment 1: rules (i) to (iii) are Python and shell; rule (iv) is Rust in the
realtime-policy tool, with its own split clause]). After #1519; independent of C2 (#1446) and B2b-1 (#1443), whose "D8 refuses the rest" sentences do not
depend on it.

## Root Amendment 1 (2026-10-12): Cargo source placement and doctests

Root's ruling, from #1519 attempt 3 spec problems 1 and 2
(`/home/bl/misofm/submix-verdicts/1519-attempt3.md`). It adds two rule groups to this slice, with root's
rulings of 2026-10-12 on the scope readings of the measured draft. Where it conflicts with the text above
(D4 (c), the "Untracked files" hazard, Non-goals "any change to the realtime-policy checks", Objective gate 6,
"no Rust code" in the budget, the authorized paths), this amendment wins; the lines it replaces carry a
bracketed pointer.

### Rule (root's words, verbatim)

Cargo source placement:

(i) refuse a TRACKED path under the roots with a skipped-directory component. Use tracked files only: an
untracked file depends on the checkout, and CI checks out tracked files;

(ii) autodiscovered bin, test, example and bench targets in a skipped directory, and `[[bin]] name` without
`path`, are refused;

(iii) every dependency path ([dependencies], [target.*], [workspace.dependencies], [patch]) and every member
must resolve under the roots, through no skipped directory.

Doctests (iv): in compiled doctest code blocks in doc comments under the roots, refuse `unsafe` and
`include!`, the same as the tool's unsafe-ownership and include rules. A doctest is its own test crate: it
cannot reach a realtime region, but it can hold unsafe code that no ownership rule sees.

Measure 0 refusals on the real tree. Each part gets a case with its test-value sentence. If any real doctest
block holds unsafe or include!, report to root before refusing it.

"The roots" are `SCAN_ROOTS` and "a skipped directory" is `SKIPPED_DIRECTORIES`
(`tools/realtime-policy/src/walk.rs`), both read from that file with a fail-closed read, compared by path
component, ignoring case.

### Scope readings (root's rulings, 2026-10-12)

- (i) Tracked set: a cached-only `git ls-files` (the index only). The existing `tracked_paths` helper also
  lists untracked-not-ignored files (`--others --exclude-standard`); (i) uses a second, cached-only list. A
  git-fixture case pins it (an untracked file in a skipped directory is accepted, a tracked one is refused).
  In a fixture tree with no git, the existing `find` fallback is the set (every file; the fallback already
  prunes `./target`, which is the repository root's own build directory, not a path under the roots).
- (ii) Name without path. A path-less `[[bin]]`, `[[test]]`, `[[example]]` or `[[bench]]` is refused when its
  `name` equals a skipped word (ignoring case): Cargo then looks in `src/bin/<name>/` and the matching
  directories. A path-less target with an ordinary name stays accepted (the real tree has 0 path-less `[[bin]]`,
  3 path-less `[[test]]` and 1 path-less `[[example]]`, none named like a skipped word). An autodiscovered
  target that sits in a skipped directory is a tracked path with a skipped component, so (i) already catches
  it; (ii) has no separate autodiscovery code, and (ii)'s own case and mutation are the name rule.
- (iii) Scope. Dependency paths are checked in every tracked manifest, `fuzz/Cargo.toml` and the
  `docs/handoffs/.../measure/Cargo.toml` harness included. This replaces D4 (c)'s exemption. All 11 of their
  path keys resolve under the roots today. The member rule applies to the root workspace only: the root
  `Cargo.toml` `[workspace] members` (a glob expanded on disk). Not `fuzz/Cargo.toml`: its `members = ["."]`
  is the fuzz directory itself, a workspace of its own. Tables: `dependencies`, `dev-dependencies`,
  `build-dependencies` (and the `_` spellings), every `target.<cfg>` copy of them, `workspace.dependencies`,
  `patch.<registry>`, `replace`. Resolution: join to the manifest's directory, apply `..`, resolve symlinks
  (`os.path.realpath`), then require the first component under the repository root to be a root and no
  component (lexical or real) to be a skipped name; compare components, not string prefixes (`crates-old/` is
  not `crates/`).
- (iv) Doctests. "Compiled doctest code block": a markdown fenced block (3+ backticks or tildes, also behind a
  `>` or a list marker) or an indented (4 spaces) code block, in a `///`, `//!`, `/** */`, `/*! */` or
  `#[doc = ".."]` text, whose rustdoc language string makes it Rust and not `ignore`. Rustdoc's rule (LangString
  parse): tokens split on `,`/space; `rust`, `should_panic`, `no_run`, `test_harness`, `compile_fail`,
  `standalone_crate`, `editionNNNN`, `EnnnN` keep it Rust; `ignore*` makes it not compiled; any other word
  (`text`, `toml`, `sh`, ...) makes it not Rust; an empty string is Rust. `compile_fail` and `no_run` count as
  compiled (rustdoc compiles both, and a `compile_fail` block that wrongly compiles still held the text).
  Hidden lines (`# ` prefix) are part of the block. The scan does not honour `#[cfg]`: a block on an item that
  is cfg-disabled on this host is still compiled on another target.
  - Unsafe. A doctest block follows the same unsafe-ownership rule as the file that holds it. The `unsafe`
    test is the tool's unsafe-ownership rule applied to the block's tokens (`unsafe` followed by `impl`, `fn`,
    `extern` or a brace group), with the same allowlist: a block in a file on `UNSAFE_ALLOWLIST`
    (`tools/realtime-policy/src/policy.rs`) is not refused, because that file may hold the same `unsafe` as
    code. The 4 lane blocks (`crates/lane/src/fpenv.rs:185`, `:256`, `:322` and `crates/lane/src/softfma.rs:132`;
    #1422's plain twins of the `compile_fail` guarantees, each with a SAFETY comment; both files are
    allowlisted) stay accepted.
  - Include. `include!`, `include_str!` and `include_bytes!` in a doctest block are refused outright, with no
    literal-file allowance (rustdoc resolves the path relative to the source file, measured in 1519; the real
    tree has no site, so the stricter form costs nothing). The test refuses the identifiers `include`,
    `include_str` and `include_bytes` anywhere in a block.
  - Doctests in `*.md` files and `#[doc = include_str!("x.md")]` are out of the rule's text; the real tree has
    neither (0 `include_str!` in a doc attribute, 0 doc `.md` includes).
- Home of (iv): the realtime-policy tool. The rule is defined as "the same as the tool's unsafe and include
  rules", whose allowlist (`UNSAFE_ALLOWLIST`, a local constant in `policy.rs`), token scan (`unsafe_scan.rs`)
  and include rules (`source_reads.rs`) already live there. The tool's lexer gives every doc form as one
  `#[doc = "..."]` attribute (`///`, `//!`, `/** */` and written `#[doc]`) and does not see a `///` inside a
  string, and the block text is re-lexed with the same code, so `unsafe` in a comment or string inside a block
  is not mistaken for code and an unlexable block fails closed. The "no Rust code" and "tool untouched" lines
  of this slice (Non-goals "any change to the realtime-policy checks", Objective gate 6, the budget sentence)
  are amended to match. If (iv) alone makes #1527 too big for its budget, split (iv) into its own slice before
  implementation, not after a failure.

### Cases

Each is a synthetic tree refused with the named message, plus the lookalikes that must pass. Each case is red
when the named check is dropped, and only its own rows go red.

(i) Skipped component in a tracked path
- Refused: a tracked `crates/foo/node_modules/x/lib.rs`, `hosts/h/Target/y.rs` (case),
  `tools/t/src/target/a.rs` (any depth), `crates/foo/src/bin/target/main.rs` and
  `crates/foo/tests/node_modules/main.rs` (the autodiscovery shapes). Test value: a rule that looks only at
  `.rs` mod paths or at the walk's own skipped list lets rustc build a tracked file the walk never reads (the
  Cargo `path`/autodiscovery route, 1519 problem 1 (a), (c)). Mutation: drop the component test, or test only
  the first component below the root: red.
- Not refused (lookalikes): `crates/target-smoke/` (the real crate), `crates/foo/node_modules_old/x.rs`,
  `crates/foo/src/target.rs`, `target/` at the repository root outside the roots. Test value: a substring
  match refuses the real `crates/target-smoke`. Mutation: substring instead of component: red.
- Not refused: an UNTRACKED `crates/foo/target/x.rs` in a git fixture (not added). Test value: a rule built on
  the existing tracked-or-untracked list refuses a developer's local build directory and passes in CI, the
  checkout dependence root forbade. Mutation: use `--others` in the list: red.
- Self-test shape: git fixture (`git init; git add`) for the tracked and untracked rows; the non-git `find`
  fallback fixture for the tracked-path rows.

(ii) Name without path
- Refused: `[[bin]] name = "target"` with no `path` (implied `src/bin/target.rs`, which (i) does not see),
  `name = "Node_Modules"` in `[[bin]]`, `[[test]]`, `[[example]]` and `[[bench]]`. Test value: a manifest
  reader that checks only `path =` keys (the #1519 rule) accepts a path-less target whose implied path is
  chosen by Cargo, not written. Mutation: drop the name check, or limit it to `[[bin]]`: the `[[test]]`,
  `[[example]]` and `[[bench]]` rows red.
- Not refused: the real path-less `[[test]]`/`[[example]]` (`name = "m1_exhaustive"`), `[[bin]] name = "tgt"`,
  a `[[bin]]` with `path = "src/main.rs"`. Mutation: refuse every path-less target: the lookalike row red.
- No autodiscovery row: the autodiscovered shapes are the (i) rows above.

(iii) Dependency paths and members resolve under the roots
- Refused: `../../vendor/fakestd` as a path dependency in a scanned crate's `[dependencies]`, in
  `[target.'cfg(unix)'.build-dependencies]`, in `[patch.crates-io]`, in the root `[workspace.dependencies]`,
  and in `fuzz/Cargo.toml`; a path through a skipped directory (`../../crates/foo/target/lib`,
  `../sibling/node_modules/x`); a member `"../outside"`; a member glob that expands to `fuzz`; a symlinked
  member or path whose real path is outside. Test value: a path dependency or member outside the roots (the
  shape of the 1442 `std-probe`) makes rustc compile source the walk never reads, and no earlier rule sees the
  manifest edge. Mutations, each alone: drop the `target.<cfg>` recursion, the `patch` table, the
  `workspace.dependencies` table, the `fuzz` (outside-roots) manifests, glob expansion, `realpath`, or the
  skipped-component test: its own row red.
- Not refused: `../../crates/sibling` (out and back in), `crates` to `hosts` and `tools` paths,
  `fuzz/Cargo.toml` with `members = ["."]` and its two inward paths. (A path to `crates-old/x` is refused:
  component compare, not prefix.) Test value (fuzz row): applying the member rule to every manifest,
  including one outside the roots that declares its own workspace, refuses the accepted fuzz layout.
  Mutation: apply the member rule to `fuzz/Cargo.toml`: red.
- A manifest or a `members` entry of the wrong type, a missing `SCAN_ROOTS` constant: gate failure, not a skip.

(iv) Doctests (tool tests under `tools/realtime-policy/tests/`)
- Refused: a doc comment block ```` ```\nunsafe { f() }\n``` ```` in a file not on the allowlist; the same as
  `compile_fail`, `no_run`, `should_panic`, `edition2021`, and `rust,ignore` is NOT refused; a hidden line
  `# unsafe { f() };`; `include!("x.rs")`, `include_str!("x")`, `include_bytes!("x")`; the block in a `//!`, a
  `/** */` and a `#[doc = "```\nunsafe {}\n```"]` form; a `~~~` fence, a 4-backtick fence, a fence behind `> `
  and behind `- `; an indented (4-space) block. Test value: nothing else in the tool tokenizes doc text, so
  unsafe code or an `include!` written only inside a doctest (rustdoc compiles it, the tool reads it as a
  string) passes every other rule. Mutations, each alone: skip `//!`/block/attribute doc forms, drop the
  hidden-line strip, drop `no_run`/`compile_fail` from "compiled", drop the tilde/blockquote/list/indented
  recognition, drop `include_str`/`include_bytes`: its own row red.
- Not refused: `text`/`toml`/`sh`/`ignore` blocks that hold `unsafe` or `include!`; prose and inline code
  (`` `unsafe` ``) mentioning the words; `#![forbid(unsafe_code)]` inside a block; an `unsafe` block in a
  doctest of an allowlisted file (the four real sites). Test value (allowlist row): refusing every `unsafe`
  doctest refuses the lane twin tests that #1422 added beside each `compile_fail` guarantee. Mutation: ignore
  the allowlist: the allowlisted row red (and, on the real tree, 4 findings).
- A doc text that does not lex as Rust (an unbalanced `"` in a `compile_fail` block) is a finding
  (fail-closed), never a skip; a self-test row.

### Measured on the real tree (worktree `wt-d15-j3` at its HEAD, tracked files, Python 3.12.3, rustdoc 1.97.1)

| Part | Measure | Result |
|---|---|---|
| (i) | tracked paths with a `target` or `node_modules` component, anywhere in the repository | 0 (also 0 untracked-not-ignored) |
| (ii) | tracked autodiscovered bin/test/example/bench files in the 40 workspace packages (`src/bin/*.rs`, `src/bin/*/main.rs`, `tests/*`, `examples/*`, `benches/*` same two shapes) | 264, 0 with a skipped component |
| (ii) | `[[bin]]` without `path` | 0 (the 7 `[[bin]]` tables in the workspace all have `path`; one is beyond `src/main.rs`: `tools/parameter-metadata` `src/bin/lattice_oracle.rs`) |
| (ii) | `[[test]]`/`[[example]]` without `path` (implied name -> file, all tracked) | 4: `crates/math` `m1_exhaustive`, `m2_lane_identity`, `f1_fast_db_bounds` -> `tests/<n>.rs`; `crates/builtins` example `input_bound_budget` -> `examples/<n>.rs`. None names a skipped word |
| (iii) | members of the root workspace | 40, no glob, no `exclude`, no `default-members`; all under the roots (29 `crates`, 1 `hosts`, 10 `tools`); every member has a tracked manifest; every tracked manifest under the roots is a member |
| (iii) | dependency `path` keys, all 43 manifests, all tables | 45: 34 root `[workspace.dependencies]`, 9 in `docs/handoffs/control-smoothing-defaults/measure`, 2 in `fuzz`; 0 in `crates`/`hosts`/`tools` manifests (they inherit with `workspace = true`); 0 outside the roots, 0 through a skipped component |
| (iii) | `[patch]` / `[replace]` tables | 0 |
| (iv) | compiled doctest blocks under the roots (rustdoc rule above; `///` and `//!` only, no `/** */`, no `#[doc = ".."]` text block in any tracked `.rs` under the roots) | 36: 21 `compile_fail`, 15 bare. By file: `graph-compiler/src/lib.rs` 12, `lane/src/fpenv.rs` 11, `host-core` 9 (`prepare.rs` 2, `render_session.rs` 3, `route_controls.rs` 4), `engine/src/realtime/plan.rs` 2, `lane/src/softfma.rs` 2. Not compiled: 26 `text`, 1 `toml`, 0 `ignore`. Indented code blocks: 0 |
| (iv) | cross-check: `cargo test --locked --offline --workspace --doc -- --list` on a `git archive` copy | 30 listed on this x86_64 host (6 `fpenv.rs` blocks are `#[cfg(aarch64)]` or `#[cfg(not(any(x86_64, aarch64)))]` items), 36 with the cfg-disabled ones. Matches |
| (iv) | blocks with `unsafe` | 4: `crates/lane/src/fpenv.rs:185` (x86_64), `:256` (aarch64), `:322` (other targets), `crates/lane/src/softfma.rs:132` (x86_64) |
| (iv) | blocks with `include!`, `include_str!`, `include_bytes!`, `#[path]`, `extern`, `asm!`, `mod x;` | 0 |
| (iv) | `#[doc = include_str!(..)]` / doc-attribute text | 0 (the three `#[doc = ..]` uses are macro_rules docs in `crates/session/src/visit.rs`, no code block) |
| (iv) | `[lib] doctest = false` | none in any manifest |

Method (scratch, not committed): `git ls-files -z`; Python `tomllib` on every manifest; `measure.py` and
`doctests.py` in the session scratchpad. The implementation re-measures every number on its own branch and
records any difference in the Attempt record.

### What this means for "0 refusals"

- (i), (ii), (iii): 0 refusals.
- (iv): the four `unsafe` blocks are real sites, reported to root as the rule asked. Each is the twin of a
  `compile_fail` guarantee beside it (#1422): the same two calls, with an `unsafe` block and a `SAFETY`
  comment, to prove a renamed item turns the pair red. They sit in `crates/lane/src/fpenv.rs` and
  `crates/lane/src/softfma.rs`, both on `UNSAFE_ALLOWLIST` (the files that hold the real `unsafe fn
  write_mxcsr` and `write_fp_control_word`). Root's ruling: they stay accepted, so (iv) gives 0 refusals; a
  rule with no allowlist would refuse 4. No `include!` anywhere.

### Authorized paths this amendment needs

- `scripts/check-workspace-policy.sh`: (i), (ii) and (iii). (iii) extends the one `tomllib` reader of D1/D3;
  (i) and (ii) need a cached-only `git ls-files -z` as a second list beside `tracked_paths`, with the `find`
  fallback.
- `scripts/test-workspace-policy.sh`: the rows of the cases above (git fixtures for (i)).
- `tools/realtime-policy/**`, for (iv): `src/lib.rs` (module), a new `src/doctests.rs`, `src/report.rs`
  (finding classes: unsafe in a doctest, include in a doctest, doctest block that does not lex),
  `src/unsafe_scan.rs` and `src/source_reads.rs` (only to expose the stream scan to the new module),
  `tests/` (the doctest cases), and the tool's doc comments and the #1438 Hazards sentence that list the
  rules. The `src/walk.rs` doc comment of `SCAN_ROOTS` gains the sentence that the script reads it for (iii)
  and (i).

### Objective gate changes

- Gate 1 (real tree): add the measures of the table: 0 tracked skipped-component paths; 0 name-without-path
  refusals; 40 members and 45 path keys all inside; and 36 compiled doctest blocks read, 0 refusals, with the
  4 allowlisted `unsafe` blocks accepted.
- Gate 2/3: one self-test row and one mutation per case line above, each alone turning only its own row red.
- Gate 4 (replay): unchanged. Add a replay of the (iii) rows on `fuzz/Cargo.toml` and the `docs/handoffs`
  manifest: 0 refusals.
- Gate 5 (test value): each new row carries its sentence from the cases.
- Gate 6 (replaces the original): `cargo test --locked -p realtime-policy` passes with the new doctest rows,
  `cargo run --locked --release -q -p realtime-policy` still passes (the awk gate is deleted by #1446)
  on the real tree, and `cargo fmt --check` and `cargo clippy -p realtime-policy --all-targets -- -D warnings`
  exit 0.
