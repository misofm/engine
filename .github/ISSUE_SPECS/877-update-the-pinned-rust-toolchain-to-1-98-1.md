# Update the pinned Rust toolchain to 1.99.1

## Product outcome

Move every live toolchain pin from Rust 1.97.1 to the current stable release, Rust 1.99.1 (58cf8f9c3, 2026-10-13; retargeted from 1.98.1 on 2026-10-24, see the decision record), so that the engine, hosts, tools, fuzz builds, npm publication, and browser qualification all build with one current compiler. Keep every DSP, realtime, protocol, session, and ABI contract unchanged: this issue moves no rendered bit by design and changes no product surface.

## Trigger and root evidence

The user requested the bump before an engine optimization pass. The pin has been 1.97.1 since the workspace bootstrap (commit `68ff477e`); there is no prior bump to follow, so this issue records the procedure.

Two facts learned while probing the new toolchain shape the slice:

1. Rust 1.98's clippy adds `chunks_exact_to_as_chunks` (in `clippy::all`, which the workspace denies). It fires 40 times in `crates/lane` alone, every site being `chunks_exact(L::WIDTH)` over a generic lane type. Stable Rust cannot pass a type parameter's associated const as a const-generic argument, so the suggested `as_chunks::<L::WIDTH>()` does not compile; the lint is inapplicable at the hot sites and cosmetic at the cold ones.
2. The content-addressed AudioWorklet artifact is a function of the compiler. Built by `scripts/build-web-audioworklet.sh` with `MISO_ENGINE_WEB_AUDIOWORKLET_REPIN=1` under 1.98.1, `host_web.wasm` digests to `08ae541cf39dd1809840eb213a27d993054b8151538bcfcb73d340b6dba0a0bd`, replacing the accepted 0.4.2 artifact `e18acf9ca97af137a1917e52481c4bf962943d6d755369387969f84c3e381106`. `qualification.yml` rebuilds the artifact and compares it against `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256`, and `run.mjs --check-matrix` requires `results.json` to carry the digest under qualification, so a bump without a repin and a fresh browser record fails the required check on `main`.

## Smallest closable slice

Authorized paths are only:

- this issue spec;
- `rust-toolchain.toml`, `Cargo.toml` (`rust-version` and the workspace clippy lint table);
- `.github/workflows/qualification.yml`, `nightly.yml`, `fuzz.yml`, `npm-publish.yml` (the literal `1.97.1` toolchain installs and `RUSTUP_TOOLCHAIN`; the `nightly-2026-08-20` cargo-fuzz toolchain and the `cargo-fuzz 0.13.2` pin are untouched);
- `docs/TARGET_MATRIX.md` (the reproducible-checks sentence naming the pinned release);
- `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256`, `EXPECTED_WORKLET_SHA256` in `npm-publish.yml`, and the matching `release_pin` literals in `scripts/test-npm-publish-modes.py`;
- `hosts/host-web/qualification/results.json` and `hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md`, regenerated only by `npm run qualify -- --record-matrix` against the repinned artifact.

Allow `clippy::chunks_exact_to_as_chunks` once, in `[workspace.lints.clippy]`, with the rationale above recorded beside it. Do not rewrite any `chunks_exact` site in this issue: a migration of the const-sized cold sites is an ordinary refactor that must prove its own bit-exactness, not a toolchain gate.

Historical evidence keeps its recorded compiler. Issue specs, `artifacts/**`, `docs/evidence/**`, `docs/C_ABI_V1_QUALIFICATION.md`'s frozen preflight, `MUTATIONS.md` files, and the fixture metadata strings in `scripts/test-console-benchmark.sh` and `scripts/test-wasm-console-benchmark.sh` continue to say 1.97.1 because they describe runs that happened under 1.97.1.

Do not change any crate source, DSP, session, protocol, C ABI, SDK, package version, or CI routing. Do not publish npm.

## Objective gates

1. `rustup show` in the repository resolves to 1.98.1 and `cargo --version` reports it; `grep -rn '1\.97\.1' rust-toolchain.toml Cargo.toml .github/workflows docs/TARGET_MATRIX.md` is empty.
2. `cargo fmt --all --check` and `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` (the exact `lint` job command) pass under 1.98.1.
3. `cargo test --locked --workspace` passes under 1.98.1.
4. The scalar and simd128 `wasm32-unknown-unknown` release builds of `engine`, `session`, `protocol`, `target-smoke`, and `host-web` succeed.
5. `scripts/build-web-audioworklet.sh` in pin-check mode succeeds against the new `.sha256`, and `python3 scripts/test-npm-publish-modes.py` passes with the new `release_pin`.
6. Chromium, Firefox, and WebKit pass `npm run qualify -- --record-matrix --candidate-commit <sha> --self-test-mutations` against the repinned artifact, and a following `--check-matrix` run accepts the regenerated `results.json`/`BROWSER_DEPLOYMENT_MATRIX.md`.
7. The `qualification` required check is green on the merged `main` commit.
8. Exact authorized-path audit and `git diff --check` pass.

## Delivery

Single feature branch `codex/rust-1-98-1`, checkpoint-committed locally, pushed once for review, merged to `main` once. Record the repin digest, the lint decision, and the browser qualification run identities here; close only after the evidence commit is upstream and the required check is green.

## Starting evidence

- Baseline: `main` at `1dca662a`.
- 1.98.1 host toolchain: `rustc 1.98.1 (48a229cea 2026-09-01)`, LLVM 22.1.8, installed with `clippy`, `rustfmt`, and `wasm32-unknown-unknown`.
- Under 1.98.1 with only the pin edits: `cargo fmt --all --check` clean; simd128 Wasm build clean; clippy fails in `lane` with 36 `chunks_exact_mut` and 4 `chunks_exact` `chunks_exact_to_as_chunks` diagnostics (`crates/lane/src/kernels.rs`, `crates/lane/src/kernels/builtins.rs`).
- REPIN digest under 1.98.1: `08ae541cf39dd1809840eb213a27d993054b8151538bcfcb73d340b6dba0a0bd`.

## Decision record (2026-10-24, attempt 1, branch `codex/batch-b13-877`, base `eb792c949`)

The scope note (`scope/877.md`) wins for paths and decisions; these changes over the body above follow it.

- **Target 1.99.1**, not 1.98.1: 1.98.1 was stale. PR #878 is superseded; root closes it.
- **Lints.** Workspace clippy under 1.99.1 fired only `chunks_exact_to_as_chunks` (allowed once in
  `Cargo.toml` with its rationale) and rustc's `deprecated` on `Atomic*::fetch_update` (renamed
  `try_update`; three sites in `crates/protocol` and `crates/host-core`, the same method). The lint
  groups carry `priority = -1`. No other allow.
- **Pins.** Between releases (`docs/RELEASE.md`, "Between releases") only the pin file moves:
  `EXPECTED_WORKLET_SHA256` and `test-npm-publish-modes.py`'s `release_pin` stay the 0.4.3
  release's, and the release checks do not fire. `npm-publish.yml`'s `RUSTUP_TOOLCHAIN` and step
  name move to 1.99.1; the npm audit normalizes exactly those two literals back to its 1.97.1 hash.
- **Repin.** AudioWorklet module under 1.99.1: `81fa52ee9a58074b6750ff5f038229a8b0c609d10afcece6c2d974f8c9fb5cff`
  (the same tree under 1.97.1 builds `cfd36a3e…`).
- **No bit moved.** The 37 native console bench rows' `output_sha256` and the mixing-automation
  preflight digests are equal under 1.97.1 and 1.99.1 (AVX2 host, PR evidence); release lane,
  math, wasm-gates, console-workload, conformance and dsp-reference tests and the fixture gates pass.
- **AArch64.** iOS memset counts unchanged (builtins 4, lane 1). The arm64 test legs (LANE-3 row)
  run only on CI's arm64 runner; no row was added or deleted.
- **Blocker found.** `scripts/check-web-audioworklet.sh` rule 3 fails on 1.99.1:
  `Channel<f32x4, 4>::decode_track` (state decode, control thread) went from vector 0 / scalar 6
  to vector 1 / scalar 4, because LLVM 22 SLP-vectorizes part of its ramp-path validation. It is
  not a render kernel. Root rules the fix.
