# Build the browser artifact on a pinned nightly toolchain

Stream H(a) of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-10).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

The shipped AudioWorklet module is built by one pinned, dated nightly with `-Zbuild-std`, named in
one place in the repo. Every build path (CI, the release workflow, a developer's machine) uses it,
the release identity names it, and changing it is held to a re-recorded three-browser qualification
and the parity gates. Every other build in the repo stays on stable 1.99.1. This is the toolchain
that #1332's Wasm threads need; this slice ships the toolchain change alone, with today's module
shape (no atomics, unshared memory), so its risk is reviewed apart from threading.

## Context

- `rust-toolchain.toml:1-7`: `[toolchain] channel = "1.97.1"`, profile minimal, clippy, rustfmt,
  target `wasm32-unknown-unknown`. Verified locally with rustup 1.29.1: an extra top-level table
  (`[browser-artifact] channel = "..."`) is ignored by rustup and the active toolchain stays 1.97.1.
- The one home of the shipped build: `scripts/build-web-audioworklet.sh:108-114` (`remap` of
  `CARGO_HOME` and the repo root, then `cargo build --locked --release --target
  wasm32-unknown-unknown -p host-web`). Its `--module-only`, `--check-pin` and `--named-twin` modes
  and `scripts/test-sdk-artifact-builder-output-contract.sh` pin that cargo line's behaviour.
- CI jobs that run the builder: `artifact` (installs 1.97.1 at `qualification.yml:111-114`, emits
  `rustc` as `rustc -vV`'s `release` at `:137`) and `artifact-identity` (`:176-179` install, twin
  build at `:182-186`, `--rustc "$RUSTC"` at `:200`). `sdk` and the browser legs consume the
  uploaded artifact. `run-wasm-gates.sh` builds its G5 guest with stable (`:73`), which is "the one
  wasm build that ships" by its own header.
- `scripts/check-ci-path-routing.py:557` pins the `artifact-identity` install step text and
  `:681-687` its step shape.
- Release identity: `npm-publish.yml:34` `RUSTUP_TOOLCHAIN: "1.97.1"`, step "Install Rust 1.97.1 and
  the Wasm target" (`:141-147`), build at `:159`; `scripts/test-npm-publish-modes.py:56` pins that
  step name. `scripts/web-audioworklet-identity.py` compares `npm-publish.yml`'s `RUSTUP_TOOLCHAIN`
  with the `artifact` job's rustc release (`release_checks`, `:225-255`); `RUSTC_RELEASE` (`:75`)
  and `RECORD` (`:77`) parse the `<sha256> rustc <release>` record.
  `docs/RELEASE.md:14` and `:125-131` state the rule.
- With the nightly, `rustc -vV` reports `release: 1.100.0-nightly` (verified locally for
  `nightly-2026-08-20`), which no longer names one compiler, so the identity must name the dated
  toolchain instead.
- `nightly-2026-08-20` is the fuzz pin (`fuzz.yml:99`, `nightly.yml:89`): `rustc 1.100.0-nightly
  (f7d782a3b 2026-08-19)`, LLVM 23.1.0.
- `docs/TARGET_MATRIX.md:77` names the pinned toolchain.
- Issue *Update the pinned Rust toolchain to 1.98.1* (#877) is open and moves only the stable pin.

## Decisions frozen for this slice

- D1. **One source of truth.** `rust-toolchain.toml` gains:
  ```toml
  # The browser AudioWorklet artifact only (decision 15, D15-10). Read by
  # scripts/build-web-audioworklet.sh; rustup ignores this table.
  [browser-artifact]
  channel = "nightly-2026-08-20"
  ```
  Use the date #1331 recorded as passing; `nightly-2026-08-20` unless that record names another.
  The fuzz pin in `fuzz.yml`/`nightly.yml` stays its own literal and may differ later.
- D2. **Builder.** `scripts/build-web-audioworklet.sh` reads `channel` from `[browser-artifact]`
  (refuses a missing or non-`nightly-YYYY-MM-DD` value), adds `--print-toolchain` (prints the
  channel and exits, builds nothing), and builds with `RUSTUP_TOOLCHAIN="$channel" cargo build
  --locked --release --target wasm32-unknown-unknown -Zbuild-std=std,panic_abort -p host-web`.
  Keep the toolchain as an environment prefix, not `cargo "+$channel"`:
  `scripts/check-artifact-evidence-leak.sh:112-115` finds the shipped invocation only by the
  pattern `(^|[[:space:]])cargo (build|check)([[:space:]]|$)`, so `cargo +nightly-... build` would
  leave no artifact invocation to gate and `lint` fails (`:117-118`). The prefix also overrides
  `npm-publish.yml`'s job-wide `RUSTUP_TOOLCHAIN: "1.97.1"` for this one line. RUSTFLAGS keep
  `+simd128`, the strip flag and both remaps, and add
  `--remap-path-prefix=$(RUSTUP_TOOLCHAIN="$channel" rustc --print sysroot)=/rustc-sysroot`,
  because build-std compiles std from the sysroot's `rust-src` and embeds that absolute path. No
  `+atomics`, no shared or imported memory: that is #1380 (a slice of #1332). The `parameter-metadata` run stays on
  stable.
- D3. **Install everywhere the builder runs.** `artifact`, `artifact-identity`, `sdk` and
  `wasm-guests` in `qualification.yml`, and the qualify step in `npm-publish.yml`, install the
  channel with `channel="$(bash scripts/build-web-audioworklet.sh --print-toolchain)"` then
  `rustup toolchain install "$channel" --profile minimal --component rust-src`. The `sdk` job
  needs it although it builds no module: `sdk-package.sh check` runs
  `test-sdk-artifact-builder-output-contract.sh` first (`scripts/sdk-package.sh:29-30`,
  `qualification.yml:268-279`), which runs the real builder with only `cargo` mocked, so the
  builder's real `rustc --print sysroot` query needs the channel installed. Stable installs stay for every other cargo use. `run-wasm-gates.sh` builds the G5
  guest with the same channel and `-Zbuild-std=std,panic_abort`, so G5 compares the compiler that
  ships.
- D4. **Identity names the toolchain.** The `artifact` job emits `toolchain` (the channel) in place
  of `rustc`; `web-audioworklet-identity.py` takes `--toolchain`, the record line becomes
  `<sha256> rustc <channel>` (a record from before this change still parses and compares as a
  different toolchain), and `RUSTC_RELEASE` also accepts `nightly-YYYY-MM-DD`. `npm-publish.yml`
  gains `BROWSER_ARTIFACT_TOOLCHAIN: "<channel>"`; `release_checks` requires it to equal the
  `artifact` job's channel. Its install step is renamed "Install the browser artifact toolchain and
  Rust 1.97.1" in the workflow and in `test-npm-publish-modes.py`. `RUSTUP_TOOLCHAIN` stays
  `1.97.1` for the SDK gates' cargo use.
- D5. **Bump rule, enforced.** `web-audioworklet-identity.py` treats a change of
  `[browser-artifact] channel` between base and HEAD as requiring `hosts/host-web/qualification/results.json`
  to record a three-browser qualification of the built digest (the same check a release change
  runs). The AArch64 and native parity gates already run on every `full`-route PR, and a
  `rust-toolchain.toml` change routes `full`. `docs/RELEASE.md` states: bump only together with a
  re-recorded three-browser matrix and green AArch64/native parity gates.
- D6. This change is not a release: `PACKAGE_VERSION` and `EXPECTED_WORKLET_SHA256` stay. It does
  re-record `results.json` and `BROWSER_DEPLOYMENT_MATRIX.md` for the new digest (D5 applies to
  this PR itself).

## Deliverables

1. `rust-toolchain.toml` D1; builder D2; workflow installs D3; identity D4/D5 with self-test cases.
2. Updated `scripts/test-sdk-artifact-builder-output-contract.sh`, `check-ci-path-routing.py`,
   `test-ci-path-routing.py` and `test-npm-publish-modes.py` expectations.
3. `docs/RELEASE.md` and `docs/TARGET_MATRIX.md:77` sentences for the second toolchain.
4. Re-recorded `results.json` and `BROWSER_DEPLOYMENT_MATRIX.md`.

## Authorized paths

- `rust-toolchain.toml`
- `.github/workflows/qualification.yml`, `.github/workflows/npm-publish.yml`
- `docs/RELEASE.md`, `docs/TARGET_MATRIX.md`
- `scripts/build-web-audioworklet.sh`, `scripts/run-wasm-gates.sh`,
  `scripts/web-audioworklet-identity.py`, `scripts/test-npm-publish-modes.py`,
  `scripts/test-sdk-artifact-builder-output-contract.sh`, `scripts/check-ci-path-routing.py`,
  `scripts/test-ci-path-routing.py`
- `hosts/host-web/qualification/results.json`, `hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md`
  (regenerated by `npm run qualify -- --record-matrix` only)

## Non-goals

- No crate source change, no `+atomics`, no shared memory, no policy-gate rewrite (#1332).
- No stable bump (#877), no fuzz pin change, no npm publication, no package version change.
- No other crate or host built with the nightly.

## Hazards

- The module's bytes change (new LLVM, std rebuilt with `+simd128`). No render digest may move:
  the browser `native-corpus-digest` gate, `direct-oracle.mjs` and G5 decide that. A moved digest
  is a finding to report, never a re-pin.
- If D2's remap does not make the digest independent of `RUSTUP_HOME`, `npm-publish.yml`'s rebuild
  will refuse the bytes. Gate 4 checks it.
- #877 edits the same workflow lines; whichever lands second rebases. #877 must not touch
  `[browser-artifact]`.

## Objective gates

1. `rustup show active-toolchain` in the repo still reports 1.97.1;
   `bash scripts/build-web-audioworklet.sh --print-toolchain` prints the D1 channel.
2. `rm -rf <out> <twin> && mkdir -p <out> <twin> && bash scripts/build-web-audioworklet.sh
   --named-twin <twin> <out>`, then `bash scripts/check-web-audioworklet.sh <out>
   <twin>/miso-engine-v1-audio-worklet.simd128.named.wasm`, `python3 -B
   scripts/check-browser-expected-resources.py --artifacts <out>`, `bash
   scripts/test-web-audioworklet.sh`, `bash scripts/test-sdk-artifact-builder-output-contract.sh`.
3. `bash scripts/run-wasm-gates.sh` passes (guest on the nightly).
4. Reproducibility, PR evidence: the module digest is identical when built with `RUSTUP_HOME`
   pointing at a second copy of the toolchain and with another `CARGO_HOME` and checkout path.
5. Browser legs: `npm run qualify -- --artifacts <out> --sdk-root <repo>/sdk --browser <b>
   --self-test-mutations` passes for each of chromium, firefox and webkit. The record is one run,
   `npm run qualify -- --artifacts <out> --sdk-root <repo>/sdk --browser all --record-matrix
   --candidate-commit <sha>` (`hosts/host-web/qualification/run.mjs:879-881` refuses
   `--record-matrix` without `--browser all`), then `--check-matrix` accepts the regenerated files.
6. `python3 -B scripts/web-audioworklet-identity.py --self-test` (with new cases), `python3 -B
   scripts/test-npm-publish-modes.py`, `python3 -B scripts/check-ci-path-routing.py` and `python3
   -B scripts/test-ci-path-routing.py`, `python3 -B scripts/check-release-shape.py --self-test`.
7. Parity on the AArch64 runner and native: the `full` route's `aarch64-release`, `test-release`
   and browser jobs pass at the batch push (they build on stable and compare against the same pins
   the nightly module must hit).
8. `cargo fmt --all -- --check`, `cargo clippy --locked --workspace --all-targets --all-features --
   -D warnings`, `bash scripts/check-workspace-policy.sh`.

## Test value

- Identity self-test "npm toolchain differs from the artifact's channel": a release workflow left on
  another toolchain turns it red; today's check compares rustc releases, which every nightly
  shares (`1.100.0-nightly`).
- Identity self-test "channel changed without a three-browser record of the built digest": a bump
  that skips the matrix turns it red; nothing holds a toolchain change today.
- Builder self-test of `--print-toolchain` and the channel refusal (in
  `test-sdk-artifact-builder-output-contract.sh`): a missing table or a floating `nightly` channel
  turns it red, which would otherwise build with whatever nightly is installed.
- Superseded and edited, not deleted: the npm step-name expectation and the `artifact-identity`
  step-shape expectation change with the workflows.

## Dependencies

- *Prove two Wasm instances on one shared memory in three browser engines and on iOS* (#1331a):
  it chooses the dated nightly, records the build flags, and runs the parity gates on it. Root
  ruling 2026-10-24: this slice starts on #1331a-1's desktop record alone and does not wait for the
  iOS leg (#1331b); the module shape it ships (no atomics, unshared memory) is today's.
- Not dependent on *Gate AudioWorklet render against allocation statically and at runtime* (#1333).

## Decision record (attempt 1, 2026-10-25, base `b0bc367af`)

Anchors moved since the brief: the stable pin is 1.99.1 (not 1.97.1) everywhere; D4's step name is
"Install the browser artifact toolchain and Rust 1.99.1".

- **Inputs from #1331a-1 (PR #1962).** Channel `nightly-2026-08-20` passed (rustc `1.100.0-nightly
  (f7d782a3b 2026-08-19)`, LLVM 23.1.0). Its non-atomic flag set adds `+bulk-memory,+mutable-globals`
  to `+simd128`; both are already default target features of `wasm32-unknown-unknown` on this
  nightly (`rustc --print cfg`), so the builder keeps `-C target-feature=+simd128` alone. The record
  answered the RUSTUP_HOME hazard: without a sysroot remap the digest depends on the toolchain path.
- **D2 as built.** The builder remaps the whole sysroot (`$(rustc --print sysroot)=/rustc-sysroot`,
  the brief's form; the record remapped its `lib/rustlib/src/rust` subdirectory, which the whole
  sysroot covers). `-Zbuild-std=std,panic_abort` sits before `--target ... -p host-web`, so the cargo
  line keeps the tail `scripts/test-artifact-evidence-leak.sh` edits. `--render-lock-probe` shares
  the cargo line and so builds on the nightly too.
- **D3 as built.** Install sites: `artifact`, `artifact-identity`, `wasm-guests` (in their toolchain
  step) and `sdk`, `artifact-gates` (a step of its own after the download is verified:
  `check-ci-path-routing.py` refuses a repository script before that verification in a reader job;
  `artifact-gates` needs it for the render-lock probe, which the brief did not list). `npm-publish`'s
  qualify step installs the channel after refusing a `BROWSER_ARTIFACT_TOOLCHAIN` other than the
  builder's. Only `rust-src` is installed: gate 4 built the module from a toolchain copy with no
  wasm32 `rust-std`.
- **D4 as built.** The `artifact` job's output `rustc` is now `toolchain`; `artifact-record`
  accepts only a dated nightly; `--toolchain` likewise (the builder accepts nothing else), and a
  record from before (`<sha256> rustc 1.99.1`) parses and reads as a toolchain change.
  `release_checks` compares `BROWSER_ARTIFACT_TOOLCHAIN` with the built channel in place of
  `RUSTUP_TOOLCHAIN`, which no longer reaches the module's bytes.
- **D5 as built.** A change of the channel between base and HEAD adds a `TOOLCHAIN:` line and runs
  the `results.json` lineage checks (digest and canonical `candidateCommit`), as a release does.

### Evidence

- Digest before (stable 1.99.1, `b0bc367af`, matches main's `audioworklet-sha256` record):
  `95ee32630f43ff1008c85d4f1dae37258c2785fe892c6676826060ab5f06d0e6`, 5,628,337 B. After (nightly,
  build-std): `8643b810a7396683ff2d4f5788ac3b8e52c082d2475efd385cb131ac3498c255`, 5,625,493 B; named
  twin `4f3b48cf...6ce6`. The pin and `EXPECTED_WORKLET_SHA256` stay (D6). Cargo time for the module:
  31 s stable, 51 s nightly with build-std (one run each, a shared machine).
- Gate 1: `rustup show active-toolchain` = 1.99.1; `--print-toolchain` = `nightly-2026-08-20`.
- Gate 2: `check-web-audioworklet.sh <out> <twin>`, `check-browser-expected-resources.py
  --artifacts` (expected.json digests and rows agree), `test-web-audioworklet.sh`,
  `test-sdk-artifact-builder-output-contract.sh`: pass.
- Gate 3: `run-wasm-gates.sh` (native + guest on the nightly + V8 spill): pass, 0 mismatches.
- Gate 4: a build from a second `RUSTUP_HOME` (toolchain copy without the wasm32 std), another
  `CARGO_HOME` and another checkout path gave `8643b810...` again.
- Gate 5: each of chromium 151.0.7922.34, firefox 153.0, webkit 26.5 passed
  `--self-test-mutations` once (native corpus digest included); one `--browser all
  --record-matrix` run re-recorded `results.json` and `BROWSER_DEPLOYMENT_MATRIX.md`, and
  `--check-matrix` accepts them.
- Gate 6: identity `--self-test`, `test-npm-publish-modes.py`, `check-ci-path-routing.py`,
  `test-ci-path-routing.py`, `check-release-shape.py --self-test`: pass. Gate 7 is CI's.
- Gate 8: `cargo fmt --check`, `check-workspace-policy.sh`: pass; no Rust source changed.

- **Sysroot remap form (deliberate).** The builder remaps the whole sysroot
  (`$(rustc --print sysroot)=/rustc-sysroot`, D2's frozen form) rather than the #1331a-1 record's
  `<sysroot>/lib/rustlib/src/rust=/rust-src`: `-Zbuild-std` compiles std from that `rust-src`,
  which lies inside the sysroot, so the wider prefix covers it and any other sysroot path. Gate 4
  evidence: a second `RUSTUP_HOME` (toolchain copy at another path, no wasm32 std), another
  `CARGO_HOME` and another checkout gave the same digest `8643b810...` as the first build.
- **Re-record after merging main (2026-10-25).** Main at the merge (#2038, #2048) moves the module
  to `f8e2e594f8468f9da76d6a5714ca46b03e42c3b202ac90dff04e064e9e39b1c2` (5,697,370 B); one
  `--browser all --record-matrix` run (candidate `f4a4f0da0`, chromium 151.0.7922.34, firefox
  153.0, webkit 26.5, all gates passed) re-recorded `results.json` and
  `BROWSER_DEPLOYMENT_MATRIX.md`, and `--check-matrix` accepts them.

### Stated limits (each with a successor)

- **The sysroot remap has no committed gate.** The `artifact-identity` twin shares the runner's
  `RUSTUP_HOME`, so a dropped remap stays green on a PR; gate 4 above is PR evidence only. It fails
  closed: the module would then embed the toolchain path, and the next bump or release, built from
  another toolchain path (`npm-publish.yml`'s rebuild) or re-recorded against a moved digest, refuses
  the bytes rather than shipping them. Successor: build the twin from a toolchain copy at another
  path, so the remap is held on every PR.
- **`check-browser-expected-resources.py` without `--artifacts` builds its own module on stable.**
  CI always passes `--artifacts`, so CI reads the shipped bytes; a local run without it reads a
  stable build. Successor: have that path call `build-web-audioworklet.sh --module-only`, the one
  home of the shipped cargo line.

### Verifier finding M1 (folded in)

The builder contract test's mock cargo logs `RUSTUP_TOOLCHAIN`, and the test asserts that the
delivery build's cargo line runs on the `[browser-artifact]` channel with
`-Zbuild-std=std,panic_abort`. It is red on each mutant: dropping `RUSTUP_TOOLCHAIN="$channel"`,
dropping `-Zbuild-std`, and dropping both.

### Merge order (root, 2026-10-25)

The recorded `results.json` describes `8643b810...` built on `b0bc367af`. #1334 merges alone after
the current wave of module-moving PRs; just before it merges, the branch merges `origin/main` and
re-records the matrix (one `--browser all --record-matrix`, then `--check-matrix`), with nothing
in between.
