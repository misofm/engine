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
    listed as a member.
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
- `.github/ISSUE_SPECS/1438-*.md` Hazards: one sentence that this slice closes the manifest and build-config
  forms listed there, and nothing else (the threat model text is root's).

## Non-goals

- Reading `Cargo.lock`, resolving the registry, or `cargo metadata` as the source of truth (it needs the network
  or a lockfile at gate time; the TOML reader does not).
- Widening the extern-name set beyond `core`, `std`, `alloc`; changing `SCAN_ROOTS`; scanning `fuzz/` or
  `docs/` sources; any change to the realtime-policy checks.
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
- **Untracked files.** The script already lists tracked and untracked-not-ignored paths (`tracked_paths`); the
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
   untouched except for one doc comment); `cargo fmt --check` and `cargo clippy -p realtime-policy --all-targets
   -- -D warnings` exit 0.

## Dependencies

After (same stream): #1519 (attempt 3, commit `0436b10e7`, under verification), because both edit
`scripts/check-workspace-policy.sh` and `scripts/test-workspace-policy.sh` and D3 reuses its reader if it
lands. If #1519 spends its budget without PASS, this slice owns the reader and #1519's manifest target-path rule
(root Amendment 2) is split into it or rebased on it, as root rules. Closes the limits that #1442 attempt 3 F1
names for the successor; #1442 root Amendment 4's "dependency key" sentence points here.

## Budget, size and order

Three attempts; about half a working day (one Python reader, three rule groups, about 30 self-test rows, no Rust
code). After #1519; independent of C2 (#1446) and B2b-1 (#1443), whose "D8 refuses the rest" sentences do not
depend on it.
