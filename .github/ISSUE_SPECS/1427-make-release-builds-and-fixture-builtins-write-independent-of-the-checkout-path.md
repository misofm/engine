# Make release builds and fixture-builtins --write independent of the checkout path

Stream J of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-0).
Filed 2026-10-05 from the verification of *Keep every trim, fader and matrix ramp inside its
endpoints* (#1408), whose attempt-1 record noted that `audit fixture-builtins --write` output
differs from the committed `reference/filter-response.csv` while `--check` passes. Code anchors
verified on `codex/d15-stream-g` at `098499891` (which contains `main` at `68ef86651`).

## Product outcome

Two release builds of one commit render the same bits, wherever the repository is checked out.
Today the release `audit` binary's `fixture-builtins --write` emits one of two different
`fixtures/builtins/v1/reference/filter-response.csv` files depending only on the checkout path,
because `BuiltinChain::process_dual_mono` compiles to different machine code in the two builds. A
pinned artifact whose bytes depend on where a developer cloned the repository cannot be re-pinned
safely, and a render path whose arithmetic depends on the build path is not deterministic across
CI and developer machines. After this slice the generated file is byte-identical across checkout
paths, and a check keeps it that way.

## Context

- **The measured defect (2026-10-05, #1408 verifier; Evidence below).** Two release builds of
  `6fb211594` from different checkout paths produce two `filter-response.csv` files (sha256
  `ece29117...` committed, `d577dab4...` variant). All 1,630 rows differ only in
  `impulse_dft_magnitude_db`, by at most `3.209e-5` dB (`response-low_pass-48000-1-0-6`), median
  `1.94e-12` dB. Every other column is byte-identical. There are exactly two variants. The
  committed one matches `477dc15ee` and every first-parent `main` commit built in one build
  environment; the #1408 bisect's "culprit" `6fb211594` is the commit at which the verifier's
  checkout path changed, not a source change.
- **Why `--check` is green.** `RESPONSE_IMPULSE_DFT_TOLERANCE_DB = 0.05`
  (`tools/audit/src/fixture_builtins.rs:57`) bounds the committed row and the independent
  recurrence against RBJ and against each other (`:4529-4545`), so a `3.2e-5` dB move passes. That
  tolerance is a DSP-accuracy bound, not a reproducibility check, and stays as it is (D4).
- **What renders the column.** `response_row`'s impulse loop (`fixture_builtins.rs:899-918`)
  drives `BuiltinChain::new` and `BuiltinChain::process_dual_mono`
  (`crates/builtins/src/lib.rs:3336`, `:3362`) over one second at the row's quantum, then takes the
  DFT magnitude at the probe. The verifier found `<builtins::BuiltinChain>::process_dual_mono`
  (a local symbol of the release `audit` binary) compiled to different machine code in the two
  builds, so the 1 s impulse response itself differs, not only its formatting.
- **The release profile.** `[profile.release]` is `lto = "fat"`, `codegen-units = 1`,
  `panic = "abort"`, `debug = 1` (`Cargo.toml:118-122`); every release build uses it. The only
  approved global rustc flags are the x86-64-v3 pin
  `[target.'cfg(target_arch = "x86_64")'] rustflags = ["-C", "target-feature=+avx2,+fma"]`
  (`.cargo/config.toml:7-8`), and `scripts/check-workspace-policy.sh:327-357` refuses any other
  `rustflags`/`target-feature`/`target-cpu` line in `.cargo` and any `[build]` table.
- **Where the checkout path enters the binary (observed on `target/release/audit` in this
  worktree).** `strings` finds the absolute checkout path three times: once in `.rodata`, the
  `env!("CARGO_MANIFEST_DIR")` join in `tools/audit/src/builtins_fixture_check.rs:364` (non-test
  code; `:509` and `fixture_builtins.rs:5857` are in test modules), and twice in the DWARF sections
  that `debug = 1` emits (`.debug_str`, `.debug_line`: the compilation directory). Cargo passes a
  workspace member's sources to rustc as workspace-relative paths, so the workspace crates' own
  panic locations do not carry the checkout path; registry crates' panic locations carry
  `CARGO_HOME`, which is the same for two checkouts on one machine.
- **Prior art in this repository.** `scripts/build-web-audioworklet.sh:97-115` already remaps
  `CARGO_HOME` to `/cargo` and the repository root to `/repo` with `--remap-path-prefix` (passed
  through `RUSTFLAGS`, which is safe there because the wasm target has no `target.rustflags`), so
  the shipped worklet's digest is path-independent. No native release build does the same.
- **`trim-paths` is not available.** On the pinned toolchain (`rust-toolchain.toml`: 1.97.1),
  `--config 'profile.release.trim-paths="all"'` fails with "feature `trim-paths` is required ...
  not stabilized in this version of Cargo" (checked 2026-10-05).
- **`RUSTFLAGS` displaces the ISA pin.** Cargo takes rustflags from the first of
  `CARGO_ENCODED_RUSTFLAGS`, `RUSTFLAGS`, `target.<triple>/<cfg>.rustflags`, `build.rustflags`; a
  native `RUSTFLAGS=--remap-path-prefix=...` silently drops `+avx2,+fma` (and `crates/lane` then
  refuses to compile). `target.<triple>.rustflags` and `target.<cfg>.rustflags` that both match are
  joined.
- **Release builds that write or check a fixture or artifact:** `qualification.yml` `audit-native`
  (`cargo build --locked --release -p audit -p bench -p capi -p session-validator`, `:708`; the
  builtins fixture check `:828`), `test-release` (`cargo test --locked --release -p lane -p math
  -p wasm-gates --features math/lane`, `:667`, which owns the G5 native digest corpus),
  `scripts/run-aarch64-tests.sh:170` (`cargo build --locked --release -p audit`), and the worklet
  build above. The G5 digests and the worklet digest are exact and are green in every worktree and
  in CI today, so the path dependence observed so far is confined to the `audit` binary's
  `filter-response.csv` column; step 1 confirms or widens that.

## Decisions frozen for this slice

- **D0. Root decision (2026-10-05).** The decision-15 root coordinator, under the owner's
  no-shortcuts delegation (`no-shortcuts-correctness-first`), approved this issue as Stream J
  hygiene: release builds, and `audit fixture-builtins --write` in particular, must not depend on
  the checkout path. Rationale: a pinned file that two honest builds of the same source disagree on
  cannot be re-pinned without guessing which build is right, and machine code that depends on the
  build path makes "deterministic across targets and hosts" untrue for the native build. The root
  cause is isolated first; the fix is applied in one place; no tolerance is widened.
- **D1. Isolate the root cause first.** Before any fix, build the same commit's release `audit`
  from two checkout paths of different lengths (`git worktree add` at two paths; separate
  `CARGO_TARGET_DIR`s; same `CARGO_HOME`, toolchain and environment) and reproduce the two
  `filter-response.csv` variants and the two `process_dual_mono` disassemblies. Then find which
  path-carrying input changes the code, one variable at a time, recording each experiment:
  - **H1 (root's hypothesis): path strings change code size and so fat-LTO inlining.** Remap both
    roots (`--remap-path-prefix=<repo>=/repo --remap-path-prefix=<CARGO_HOME>=/cargo`, joined
    with the ISA pin, never displacing it) and rebuild at both paths.
  - **H2: debug information.** Rebuild at both paths with `--config profile.release.debug=0`
    (diagnostic only; `debug = 1` stays).
  - **H3: the `env!("CARGO_MANIFEST_DIR")` constant** in `builtins_fixture_check.rs:364`, whose
    length follows the checkout path. Remapping does not change an `env!` value.
  - **H4: anything else** the `cargo build -v` rustc invocations of the two builds show differing
    (for example `-C metadata` hashes), found by diffing those invocations with the paths
    normalized.
  The spec records which hypothesis holds, with the disassembly evidence, before the fix lands.
- **D2. The fix follows the cause and lives in one place.** Prefer removing the dependency at its
  source (for H3: the shipped `audit` binary stops embedding the checkout path, for example by
  resolving the accepted fixture root from its argument or the executable's location). Where a
  compiler flag is needed (H1, H2), apply `--remap-path-prefix` for every release build that
  writes or checks a fixture or artifact from one definition that every such build reads (for
  example one sourced helper that the workflow steps and scripts call, or one cargo `--config`
  file), not a copy per script. The mechanism must keep the x86-64-v3 pin (never native
  `RUSTFLAGS`), pass `scripts/check-workspace-policy.sh` (amend its allow-list only if the one
  definition must live in `.cargo/config.toml`, with the amendment's reason in this spec), and use
  the worklet's labels (`/repo`, `/cargo`) so that a later adoption by
  `scripts/build-web-audioworklet.sh` leaves the worklet digest unchanged. `trim-paths` is not
  available on 1.97.1 (Context); a later toolchain bump may switch to it. If the cause is none of
  H1-H3 and no single stable mechanism removes it, stop and report to root with the evidence.
- **D3. The gate is two checkout paths.** A committed script,
  `scripts/check-checkout-path-independence.sh <commit>`, builds that commit's release `audit` (with
  the D2 mechanism) from two `git worktree` checkouts whose paths differ in length (at least 16
  characters apart), runs `audit fixture-builtins --write` from each into a scratch root, and
  requires (a) byte-identical `reference/filter-response.csv` (and, since it costs nothing more,
  the whole written tree), and (b) identical disassembly of
  `<builtins::BuiltinChain>::process_dual_mono` in the two binaries, compared with
  `objdump -d --no-show-raw-insn` of that symbol with absolute addresses and symbol offsets
  normalized. It removes both worktrees and target directories on exit. It runs in a new job in
  `.github/workflows/nightly.yml` that joins `failure-notice`'s `needs` list (two fat-LTO builds;
  it guards a build property, not a merge input, so it does not belong in `qualification.yml`).
- **D4. No tolerance widening.** `RESPONSE_*_TOLERANCE_DB` and every other `--check` bound stay as
  they are. Reproducibility is gated by D3, not by a looser check.
- **D5. The committed file after the fix.** If the path-independent build's `--write` output equals
  the committed `filter-response.csv`, nothing is re-pinned. Otherwise this slice re-pins it once,
  from the D2 build, in its own commit whose message names the reason ("checkout-path-independent
  build"), with the evidence that only `impulse_dft_magnitude_db` moved, its maximum and median
  change, and that `--check` passes; `MANIFEST.tsv` and the manifest consumer constant
  (`ACCEPTED_MANIFEST_SHA256` in `tools/audit/src/builtins_graph.rs`, read by
  `scripts/check-builtins-fixtures.sh`) move with it. Any other moved fixture file stops the slice.

## Deliverables

1. The D1 isolation record in this spec (which hypothesis holds, experiments and disassembly
   evidence).
2. The D2 fix, in one place, wired into every release build listed in Context that writes or
   checks a fixture or artifact (`audit-native`, `test-release`, `scripts/run-aarch64-tests.sh`).
3. `scripts/check-checkout-path-independence.sh` and its nightly job (D3).
4. The D5 re-pin, only if needed.

## Authorized paths

- `scripts/check-checkout-path-independence.sh` (new), and one new shared definition for the D2
  flags if the fix needs one (for example `scripts/lib/reproducible-release.sh`)
- `.github/workflows/nightly.yml` (the D3 job), `.github/workflows/qualification.yml` (only the
  release build and test steps of `audit-native` and `test-release` that must read the D2
  definition), `scripts/run-aarch64-tests.sh` (the release build only)
- `tools/audit/src/builtins_fixture_check.rs` (only if H3 holds)
- `.cargo/config.toml` and `scripts/check-workspace-policy.sh` (only if D2's one definition must
  live there, per D2)
- Pin data only, per D5: `fixtures/builtins/v1/reference/filter-response.csv`,
  `fixtures/builtins/v1/MANIFEST.tsv`, `ACCEPTED_MANIFEST_SHA256` in
  `tools/audit/src/builtins_graph.rs`
- This spec

## Non-goals

- `scripts/build-web-audioworklet.sh` (already path-independent; stream H owns it), the
  `release-build.yml`/`npm-publish.yml` artifact pipelines, and `trim-paths` (unstable on 1.97.1).
- Changing `[profile.release]` (`debug = 1`, fat LTO and one codegen unit stay; H2's `debug=0`
  build is a diagnostic only).
- Making codegen identical across `CARGO_HOME` locations or machines: the worklet already remaps
  `CARGO_HOME`, and D2 uses the same labels, but D3 gates the checkout path only.
- Rejected alternatives:
  - Widen or keep relying on the `0.05` dB `--check` bound: it is an accuracy bound and hides a
    reproducibility defect (D4).
  - Round `impulse_dft_magnitude_db` before writing: hides the symptom, and the machine code still
    differs.
  - `RUSTFLAGS=--remap-path-prefix=...` per script: displaces the x86-64-v3 pin and is a copy per
    script.

## Hazards

- **Until this issue lands, nobody commits a `--write` re-pin of
  `fixtures/builtins/v1/reference/filter-response.csv`.** A re-pin from an arbitrary checkout path
  picks one of the two variants by accident; a slice whose own change moves the file (for example
  a builtins filter change) stops and waits for this issue instead.
- A remap flag that reaches rustc through `RUSTFLAGS` on x86-64 drops `+avx2,+fma` (Context);
  `crates/lane` then fails to compile, but a crate that does not depend on `lane` builds without
  the pin. Verify with `cargo rustc --release -p math --lib -- --print cfg` that
  `target_feature="fma"` is still set under the D2 mechanism.
- A changed rustflags set changes cargo's fingerprints: the CI steps that share a target directory
  (`test-release`, `audit-native`) must all read the same D2 definition, or each step rebuilds the
  others' artifacts.
- Hot files: `.github/workflows/*.yml` (STREAMS.md row with #877, H #1334, J #1422 and G's
  #1407 follow-up); `scripts/check-workspace-policy.sh` (only if D2 touches it).

## Objective gates

1. **Isolation recorded** (D1): this spec names the hypothesis that holds, with the two
   `process_dual_mono` disassemblies' diff before the fix and none after.
2. **Two checkout paths, one file** (D3): `bash scripts/check-checkout-path-independence.sh HEAD`
   exits 0 on the final tree; on the parent commit (no D2 fix) it exits non-zero naming
   `filter-response.csv` and `process_dual_mono` (PR evidence, the reproducer red on revert).
3. **The pin is intact:** `cargo build --locked --release -p audit && bash
   scripts/check-builtins-fixtures.sh . target/release/audit` passes; the generated tree equals the
   committed one (D5).
4. **The ISA pin survives:** `cargo rustc --locked --release -p math --lib -- --print cfg | grep -x
   'target_feature="fma"'` under the D2 mechanism.
5. Commands:
   - `bash scripts/check-workspace-policy.sh`, `python3 -B scripts/check-ci-path-routing.py`,
     `python3 -B scripts/test-ci-path-routing.py`
   - `cargo test --locked --release -p lane -p math -p wasm-gates --features math/lane` (G5 pins
     unchanged)
   - `cargo test --locked --release -p audit -p bench -p console-workload`
   - `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`
   - the PR or batch run's `qualification` verdict green, and one manual `nightly.yml` dispatch with
     the D3 job green

## Test value

- Gate 2 (the D3 script): a build input that again carries the checkout path into codegen, or a
  release build step that stops reading the D2 definition, turns it red; nothing checks build-path
  reproducibility of a native release binary today, and `--check`'s 0.05 dB bound cannot see a
  `3.2e-5` dB move. It is red on the parent commit by construction (the measured defect).

## Evidence

From `/tmp/claude-1002/v1408/evidence/` (the #1408 verifier's run, 2026-10-05):

- `filter-response.committed-ece29117.csv` (406,312 bytes, sha256
  `ece29117352529a8f2860412eb2a78000a76dd65deda13e1f4d76758e67f5408`, equals the committed file
  and its `MANIFEST.tsv` row) and `filter-response.variant-d577dab4.csv` (406,312 bytes, sha256
  `d577dab4438ce0c6e5550a9d5a93ab62366fec59a3ec95240ee372f36ad2a5ac`).
- Column diff of the two (1,630 rows each): only `impulse_dft_magnitude_db` differs, on all 1,630
  rows; max `|delta|` `3.209458e-5` dB at `response-low_pass-48000-1-0-6`, median `1.94e-12` dB.
  Example, `response-high_pass-44100-1-0-0`: `-16.02737817066901727` committed,
  `-16.02737817086653038` variant.
- `bisect-loop.log`: every sampled first-parent commit up to `d2fe0555a` (index 404) gives the
  committed variant in one environment; `6fb211594` (index 405) gives the other. Root's reading,
  confirmed by the verifier: the checkout path, not the source, changed there.
- `nm -C target/release/audit` lists `<builtins::BuiltinChain>::process_dual_mono` as a local text
  symbol, so D3(b) can disassemble it by name.

## Dependencies

none

## Attempt record
