# Delete dead fixtures and uncompiled research sources, and fix the stale target matrix

Issue key for the footprint cleanup: AArch64 CI = #1017, Darwin memset = #1018, LANE-3 = #1019, live-control research = #1020, 00 = #1021, 00b = #1022, 01 = #1023, 03 = #1024, 04a = #1025, 04b = #1026, 04c = #1027, 05 = #1028, 06 = #1029, 07 = #1030, 08 = #1031, R1 = #1032, R2 = #1033, R3 = #1034, R4 = #1035, R5 = #1036, R6 = #1037, R7 = #1038, R9 = #1039, R10 = #1040.

Source: `docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`, verified in `VERIFY-DEAD-CODE.md`. **The Amendments section supersedes the body wherever they conflict.** Owner rulings: `docs/rulings/engine-footprint-2026-09-28.md`.

Scoping study: `docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`, section 3, rows A7 and
A8. No ruling is needed. Nothing reads or compiles any of these files. Git keeps them.

## Context

- **`fixtures/capi-qualification/v1/`: 13 files, 30,007 bytes.**
  - It is a ledger of C-ABI gates. `GATES.tsv` and `AUTHORITIES.sha256` name
    `scripts/check-native-pcm-runner-v1.sh` and
    `scripts/check-native-pcm-runner-portability-v1.sh`, which no longer exist.
  - No code, script or workflow reads the folder.
  - Only `docs/C_ABI_V1_QUALIFICATION.md`, `docs/ENGINE_ENV_VOCABULARY.md`,
    `docs/derivations/243-sdk-boot.md` and inventories mention it.
  - It is dead whatever the C-ABI ruling (`R2-…`) decides.
- **`dsp-research/archive/issue-0{31,42,44,45}/*.rs`: 4 files, 5,134 lines.**
  - These are research sources. No workspace or fuzz manifest includes them.
  - `scripts/check-dsp-research.sh` reads only the Markdown corpus. Confirm by running it.
  - Because they are not Markdown, editing them routes CI to `full`.
- **`docs/TARGET_MATRIX.md`'s table is stale.**
  - It still claims a "native baseline scalar", runtime AVX2 detection, and separate baseline and
    `simd128` wasm artifacts.
  - Master plan #83 D4 (pinned x86-64-v3, no runtime dispatch) and W4-D1 (one `simd128` artifact)
    superseded all three.
  - Its #378 register (native AArch64 "unsupported, no claim") is current and stays.

## Smallest closable slice

1. Delete `fixtures/capi-qualification/`, and remove or re-point the three doc mentions listed
   above.
2. Delete `dsp-research/archive/` whole: four `.rs` files and the `README.md` that indexes them.
   That README says the files "will not build unmodified" and that `check-dsp-research.sh` "does
   not scan this directory". Re-point the comment in `crates/dsp-reference/src/svf.rs` that names
   the archive to the issue specs (#031, #042, #044, #045), which keep the history.
3. Rewrite `docs/TARGET_MATRIX.md`'s table to match today:
   - native x86-64-v3 (AVX2+FMA pinned, `Simd8`) for tooling;
   - one `simd128` AudioWorklet artifact (`Simd4`);
   - everything else unsupported.

   Keep the #378 register.

## Objective gates

1. **Build.** All three pass, and none of them references the deleted files:
   - `cargo check --locked --workspace --all-targets --all-features`;
   - `RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --target wasm32-unknown-unknown -p host-web`;
   - `cargo check --locked --manifest-path fuzz/Cargo.toml --bins`.
2. **Research and docs gates:** `bash scripts/check-dsp-research.sh`,
   `bash scripts/test-dsp-research.sh` and `bash scripts/check-builtins-listening.sh` pass.
3. **Console digests and artifact:** unaffected. The only crate edit is a comment in
   `dsp-reference`, which is a dev-dependency and not in the shipped module's closure. Show
   `git diff --stat` touching only `fixtures/`, `dsp-research/`, `docs/` and that comment.
4. **CI routing:** `check-ci-path-routing.py` and `test-ci-path-routing.py` pass. The change
   routes to `full`, because `fixtures/` and non-Markdown `dsp-research/` files are not evidence
   paths.
5. **No live claim lost:** no test or check reads the deleted files. Show a
   `rg 'capi-qualification|archive/issue-0' crates hosts tools scripts sdk fuzz .github/workflows`
   that finds nothing.

## Dependencies

None.

## Standing rules for the implementer

- Commit on `codex/<issue>-delete-dead-fixtures`.

## Amendments (Sol verification, 2026-09-28)

See `../VERIFY-DEAD-CODE.md`, finding F3.

1. **Step 3's target table contradicts the corrected scope.** "Everything else unsupported" would
   write native AArch64 out of the matrix just as the owner ruled mobile playback (iOS, Android) in
   scope. Rewrite the table as: native x86-64-v3 (AVX2+FMA pinned, `Simd8`) for tooling and tests;
   one `simd128` AudioWorklet artifact (`Simd4`); native AArch64 iOS and Android (`Simd4`, NEON)
   as **product targets pending the R1 ruling**, with the #378 register kept and marked as the list
   of defects to clear. Do not state a support level the owner has not ruled; if the ruling is not
   in yet, leave the two AArch64 rows as they are and change only the stale x86 and wasm rows.
2. **The "When native AArch64 is revived" recipe** (`TARGET_MATRIX.md:52-68`) names
   `-p target-smoke -p host-mobile`. Keep it until R1 decides those crates, and add
   `-p host-core` to it, since `capi` wraps host-core.
3. `fixtures/capi-qualification/v1` stays dead under the corrected scope: it names two scripts that
   no longer exist, and the C ABI's live gates are `scripts/check-capi-abi.sh`, the capi tests and
   `audit capi`.
4. **An open issue cites the fixture.** The open #26 spec (`.github/ISSUE_SPECS/026-…md:88-92`)
   names `fixtures/capi-qualification/v1` as the superseded C-ABI result whose re-run #26 owns.
   The fixture is still unread by any code, so deleting it is safe, but step 1 must re-point #26's
   spec text to a commit permalink in the same change (and #26 is now mobile-relevant, since it
   qualifies the native C ABI and runner target matrix).
