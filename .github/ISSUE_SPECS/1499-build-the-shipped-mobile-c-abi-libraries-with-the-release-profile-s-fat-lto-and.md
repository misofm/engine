# Build the shipped mobile C ABI libraries with the release profile's fat LTO, and gate it

Stream B follow-up of decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-0). Filed
2026-10-09 by root from the #1495 attempt-1 verdict's open item 1
(`docs/handoffs/decision-15-2026-10-05/verdicts/batch-fp/1495-attempt1.md`, "Open items for root",
item 1; original `/home/bl/misofm/submix-verdicts/1495-attempt1.md`; evidence files in
`/tmp/claude-1002/v1495/ev/`, `x86-lto-mxcsr.txt` and `x86-cdylib-helper-calls.txt`).

Root's ruling (2026-10-09), in substance: establish exactly how each shipped mobile artifact (the
iOS staticlib and the Android library) is built; make the release build apply the intended fat LTO
(`lto = "fat"` in `[profile.release]`) to the shipped artifacts; and gate it with a check that
reads the artifact, not source or prose.

**Why stream B.** `crates/capi` is stream B's (STREAMS.md, "Stream B", "Owns"), including
`crates/capi/Cargo.toml`, whose crate-type list is the cause (below), and the C ABI is the mobile
interface (`docs/C_ABI_V1_QUALIFICATION.md:3-4`). No stream owns mobile build or release tooling:
stream H's `docs/RELEASE.md`, `docs/TARGET_MATRIX.md` and workflow ownership is for the browser
artifact (#1334), stream G's #1472 owned only the iOS ratchet section of
`scripts/check-cross-targets.sh`, and stream J owns only the scripts its own issues list. The other
files this slice touches are named exceptions below.

Smallest slice: one checked-in command that builds each shipped mobile artifact with fat LTO, a
gate that is red on today's build and green on that one, and the two dependent statements
corrected. Device linking, packaging and size budgets are successors (see "Successors").

## Problem (verified on `codex/d15-batch-fp` at `abbbc65b0`, rustc 1.97.1)

- **No checked-in build of a shipped mobile artifact.** No script, workflow or release document
  in the repository builds the iOS or Android library an app links. `docs/RELEASE.md` covers only
  the browser AudioWorklet module. `docs/C_ABI_V1_QUALIFICATION.md` names the live C ABI gates
  (`check-capi-abi.sh`, the `capi` tests, `audit capi`), none of which builds for a mobile target.
  The mobile targets are built only as: `cargo check`/clippy rows and two assembly emissions in
  `scripts/check-cross-targets.sh` (`:138-140` iOS and `:161-163` Android, both
  `cargo rustc --release --target <t> -p capi --lib --crate-type staticlib -- --emit asm=...`),
  and the `aarch64-debug`/`aarch64-release` test legs (`scripts/run-aarch64-tests.sh`, tests only).
- **Which Android artifact ships is not written down.** Root's ruling names an Android cdylib;
  `check-cross-targets.sh:156-158` calls the release staticlib "the library an Android app links".
  The repository has no other statement of it.
- **`cargo build` does not apply fat LTO to `capi`.** `crates/capi/Cargo.toml:11` is
  `crate-type = ["rlib", "staticlib", "cdylib"]`; `[profile.release]` (`Cargo.toml:118-122`) is
  `lto = "fat"`, `codegen-units = 1`, `panic = "abort"`, `debug = 1`. Root's re-check
  (`cargo build --locked --release -p capi -v`): `capi`'s rustc line is `--crate-type rlib
  --crate-type staticlib --crate-type cdylib -C opt-level=3 -C panic=abort -C codegen-units=1
  -C debuginfo=1 ... -C target-feature=+avx2,+fma`, with no `-C lto`. With
  `cargo rustc --locked --release -p capi --lib --crate-type staticlib -v` the line is
  `--crate-type staticlib ... -C lto=fat ...`, and the dependencies get `-C linker-plugin-lto`.
  The `rlib` entry is the only difference from the LTO'd builds (#1495 verifier: "likely
  because"; root's re-check confirms the flag difference, not cargo's reason for it).
- **The two archives differ in shape.** On `x86_64-unknown-linux-gnu`: the `cargo build` archive
  `target/release/libcapi.a` has 386 members, one or more per crate (`std`, `lane`, `builtins`,
  the effect crates, ...); the `cargo rustc --crate-type staticlib` archive has 309: one `capi`
  object (the fat-LTO module), 273 `compiler_builtins` objects and 35 C builtins objects. Both
  commands write the same path, `target/release/libcapi.a`, so whichever ran last is the file
  there.
- **The x86_64 cdylib shows the same split** (#1495 verifier, evidence above). `cargo build`'s
  `libcapi.so`: `miso_engine_v1_render_f32_planar` calls `lane::softfma::read_mxcsr` and
  `write_mxcsr` through GOT slots (`0x80897`, `0x808a4`, `0x808f6`, `0x80928`).
  `cargo rustc --release -p capi --lib --crate-type cdylib` (fat LTO): the guard is inline,
  `vstmxcsr 0x10(%rsp)` at `0x72b96`, `vldmxcsr 0x10(%rsp)` at `0x72ba8`, exit
  `vldmxcsr 0xc(%rsp)` at `0x72cdb`, and no helper call. On AArch64 the guard is inline `mrs`/`msr`
  in both forms (`fpenv`'s writers are `#[inline]` `asm!`), so the guard is not an LTO marker
  there.
- **What depends on this.**
  - **#1472's ratchet** (closed; `scripts/check-cross-targets.sh:109-116`,
    `scripts/lib/aarch64-known-defects.py`) counts `memset_pattern16` calls in the post-LTO iOS
    `capi` staticlib assembly, on the premise that this is "the library an iPhone app links". It
    removed the per-crate pre-link scan because "every product crate ships only through `capi`"
    and a call LTO removes "cannot reach a phone". That holds only if the shipped iOS staticlib is
    built with fat LTO. Built by `cargo build --release -p capi --target aarch64-apple-ios`, the
    archive would hold per-crate pre-link code, which is what #1472's removed scan read
    (`host-core` 4, `soft-clip` 1 there, against `builtins` 4, `lane` 1 post-LTO).
  - **#1495's `x86_64` sentence** (`crates/lane/src/fpenv.rs:77-79`, "Realtime properties"):
    "Shipped `x86_64` cdylib: `enter` calls `softfma::read_mxcsr` then `write_mxcsr` out of line,
    and `Drop` calls `write_mxcsr` on each exit path; each helper is a stack-slot
    `STMXCSR`/`LDMXCSR`." It is true for `cargo build`'s non-LTO `libcapi.so` and false for the
    fat-LTO cdylib. Also, `x86_64` is "not a shipped product target"
    (`docs/TARGET_MATRIX.md:8`), so "Shipped" names the artifact `check-capi-abi.sh` links, not one
    an app links.
  - `scripts/check-capi-abi.sh:28`, `:206` build with `cargo build --locked --release -p capi` and
    say the libraries it links "must be the ones a consumer actually gets" (`:222-223`); they are
    the non-LTO form.

## Decisions

- **D1. Establish each shipped artifact (qualification step, recorded before any build change).**
  The attempt record holds one row per shipped mobile artifact: target triple, crate type, the
  exact command, profile and the `capi` rustc flags read from that command's `-v` output. iOS:
  `aarch64-apple-ios` staticlib. Android: the crate type an app links. If the repository and the
  C ABI documents give no answer, the implementer stops and asks root (an owner question), and
  records the answer; the slice does not guess between cdylib and staticlib. The record also
  states whether the `x86_64` libraries `check-capi-abi.sh` links count as shipped (today's
  `TARGET_MATRIX.md:8`: no).
- **D2. One checked-in build command per shipped artifact, with fat LTO.** A new script
  `scripts/build-capi-release.sh <target> <crate-type> <out-dir>` builds `capi` with
  `cargo rustc --locked --release -p capi --lib --crate-type <crate-type> --target <target>`, so
  `capi`'s rustc line carries `-C lto=fat` under the unchanged `[profile.release]`, and copies the
  artifact to `<out-dir>` (never leaving it only at the shared `target/<t>/release/libcapi.*` path
  another build overwrites). `crates/capi/Cargo.toml`'s crate-type list keeps `rlib`
  (`tools/audit` and `capi`'s own integration tests link `capi` as a library) unless the
  implementer shows a smaller fix; any change to that list also updates
  `scripts/check-release-shape.py`'s pinned set. `scripts/check-capi-abi.sh` builds its `.so` and
  `.a` through the script. A cdylib that needs a target linker the CI runner lacks (the Android
  NDK) is built to its LTO object only (`--emit obj`), and linking it is a successor.
- **D3. #1472's emissions read the shipped compilation.** `check-cross-targets.sh`'s iOS and
  Android emissions use the same `cargo rustc` arguments as D2's script (calling it with an
  `--emit asm` option, or sharing one argument list), so the ratchet and the eight-lane scans
  read the same compilation that ships. Their counts must not change (the iOS log charges
  `builtins` 4, `lane` 1).
- **D4. A gate that reads the artifact.** A check (in D2's script or beside it, run in CI on every
  route that builds `capi`) refuses an artifact that is not the fat-LTO build, using a marker read
  from the artifact itself: never source text, prose or the build log. Accepted markers: for a
  staticlib, the archive holds exactly one Rust object module besides `compiler_builtins` and the
  C builtins objects (today's `cargo build` archive: per-crate members such as `lane-*`,
  `builtins-*`, `std-*`); for the `x86_64` cdylib, `miso_engine_v1_render_f32_planar` holds the
  guard's `STMXCSR`/`LDMXCSR` inline and calls no `lane::softfma` helper; for an LTO object
  (`--emit obj`), no undefined symbol demangles into a workspace product crate. The implementer may
  choose another marker only if it meets gate 2.
- **D5. Correct the dependent statements in the same commit.** `fpenv.rs:77-79` states the shape
  of the artifact D2 builds (for `x86_64`, the inline guard of the fat-LTO cdylib, re-read from the
  disassembly), or drops the `x86_64` clause if D1 records that no `x86_64` artifact ships; the
  `docs/REALTIME_DEPENDENCY_POLICY.md` citations of `fpenv.rs` are refreshed if lines move.
  `check-capi-abi.sh:222-223` stays true for the libraries it now links.
  `docs/TARGET_MATRIX.md`'s iOS and Android rows and one paragraph of
  `docs/C_ABI_V1_QUALIFICATION.md` name the build command.

## Authorized paths

- `scripts/build-capi-release.sh` (new; B's)
- `crates/capi/Cargo.toml` (B's; the crate-type list only, only if D2's smaller fix needs it)
- `scripts/check-capi-abi.sh` (no stream lists it; by named exception: its build lines and default
  library paths only)
- `scripts/check-release-shape.py` (by named exception; only if `capi`'s crate-type list changes)
- `scripts/check-cross-targets.sh` (G's #1472 section; by named exception: the iOS and Android
  emission commands only, D3)
- `.github/workflows/qualification.yml` (hot-file row; the steps that run D2's script and D4's
  check only)
- `scripts/check-ci-path-routing.py`, `scripts/test-ci-path-routing.py` (J's; only if the router
  refuses the new script's path: one owner entry and its case)
- `crates/lane/src/fpenv.rs` (G's; the "Realtime properties" comment lines only, D5)
- `docs/REALTIME_DEPENDENCY_POLICY.md` (`fpenv.rs` citations only, only if D5 moves lines)
- `docs/TARGET_MATRIX.md` (H's; the iOS and Android rows only), `docs/C_ABI_V1_QUALIFICATION.md`
  (one paragraph)
- this spec

## Non-goals

- Any change to `[profile.release]` (`lto = "fat"` is already the intent) or to engine source code.
- Linking the Android `.so` with the NDK, an Xcode link of the iOS archive, packaging
  (xcframework, AAR), artifact publishing, size budgets, device runs (successors).
- Changing what `audit capi` measures (it links `capi` into its own release binary).

## Hazards

- `cargo build -p capi` and `cargo rustc -p capi --crate-type staticlib` write the same
  `target/release/libcapi.a` (verified). A CI shard that runs both links whichever came last;
  D2's copy to `<out-dir>` and D4's check on that copy are what make the gate exact.
- `fpenv.rs` is on #1446's comment-lines hot-file row and is edited by G #1494 and G #1495: keep
  their wording; the later slice rebases.
- `scripts/check-ci-path-routing.py:870-872` pins two lines of the iOS scan (the products file and
  the `judge-memset` call; #1472's record): keep both byte for byte, or update the pin under the
  named exception.
- An LTO'd C ABI library may change emitted code but must not change a rendered bit: the browser
  module and `audit capi` are not built through D2's script, so their digests stay the base's.

## Objective gates

1. **D1 record.** The attempt record holds D1's table, each row with the `capi` rustc line showing
   `-C lto=fat`, and the Android answer with its source (a document, or root's ruling).
2. **The marker discriminates (mutation, recorded; PR evidence).** For each artifact D2 builds,
   D4's check is green on the script's output and red on the same target's
   `cargo build --locked --release -p capi` output (the non-LTO form), each run recorded with its
   exit status and message. Where `cargo build` cannot produce the artifact for want of a linker,
   the red case is that target's non-LTO object or archive.
3. **#1472 unchanged.** `bash scripts/check-cross-targets.sh` passes and charges `builtins` 4,
   `lane` 1.
4. **The C ABI check links the LTO build.** `bash scripts/check-capi-abi.sh` and its `--self-test`
   pass, with D4's check green on the libraries it linked.
5. **D5 is true (verifier).** The verifier rebuilds D2's `x86_64` cdylib (if D1 keeps it) and
   reads `miso_engine_v1_render_f32_planar`; every sentence of `fpenv.rs`'s "Realtime properties"
   matches it.
6. **Existing gates.** `cargo fmt --all -- --check`, `bash scripts/check-workspace-policy.sh`,
   `python3 -B scripts/check-release-shape.py` (and `--self-test`), the router and its self-test,
   `cargo clippy --locked --workspace --all-targets -- -D warnings`, the C ABI caller audit
   (`pcm_digest` `cb10fbface44a3a4` or the base's) and the browser module digest (the base's) exit 0
   or match.

*Test value.* D4's check is red when a shipped mobile library is built without fat LTO (for
example, by `cargo build -p capi`, which drops `-C lto` while `rlib` is in the crate-type list);
no existing gate catches that, because #1472's ratchet and the eight-lane scans build their own
LTO'd assembly and never read the shipped artifact.

## Evidence

- D1's table; gate 2's red and green runs; gate 3's log lines; the `x86_64` disassembly excerpt
  for D5.

## Successors (qualification and tooling; not this slice)

- Link the Android library with the NDK in CI and run D4's check on the linked `.so`.
- Link the iOS archive with Xcode (an xcframework or a test app) on a macOS runner.
- Package and publish the mobile artifacts, with a size ceiling.

Root files these when a product need or a release asks for them.

## Dependencies

- After (other streams): G #1495 (its `fpenv.rs:77-79` text is the one D5 rewrites; landed with
  the batch-fp batch). G #1472 is closed.
- After (same stream): none.

## Standing rules for the implementer

- Work only from this body. Read the cited lines, #1472's record and the #1495 verdict first.
- AArch64 is read from cross-compiled objects and archives, never emulated.
- A test that greps source or prose is refused.
- Attempt budget: two attempts, one adversarial verdict each.
- Size: half a day. D1 is a stop point: if the Android answer needs root, D1's record is the
  first checkpoint.
