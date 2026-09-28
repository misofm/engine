# Delete dead fixtures and uncompiled research sources, and fix the stale target matrix

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

1. **Build:** `cargo check --locked --workspace --all-targets --all-features` passes, and
   `cargo check --locked --manifest-path fuzz/Cargo.toml --bins` passes. Neither references the
   deleted files.
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
