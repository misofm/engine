# Make SDK artifact builder copies portable across GNU and BSD cp

## Objective

Make the two SDK artifact builders publish their already-qualified files on macOS as well as Linux.
Replace the GNU-only `cp --update=none` invocations while preserving the existing caller-owned output
contract: the caller supplies an existing, non-symlink, empty directory, and any pre-existing content
is refused before the build or copy begins.

## Baseline and cause

On synchronized `main` at `3e41fa0d`, `bash scripts/sdk-package.sh check` successfully compiled the
AudioWorklet on macOS and then stopped at artifact publication with:

```text
cp: illegal option -- -
usage: cp [-R [-H | -L | -P]] [-fi | -n] [-aclpSsvXx] source_file target_file
```

Both `scripts/build-web-audioworklet.sh` and `scripts/build-flac-decoder.sh` use GNU
`cp --update=none`. BSD `cp` does not implement that option. Both scripts already reject a missing,
non-directory, symlink, or non-empty output before they build, so the option provides no additional
no-clobber guarantee inside the accepted contract.

## Smallest closable slice

Use ordinary portable `cp` for the exact named artifact copies after the unchanged empty-directory
preflight. Add one focused shell regression proving the output-directory contract without launching
the expensive real builds, and invoke that regression from the existing SDK package qualification
path before artifact construction.

### Allowed paths

- `.github/ISSUE_SPECS/345-portable-sdk-artifact-builder-copies.md`
- `scripts/build-web-audioworklet.sh`
- `scripts/build-flac-decoder.sh`
- one focused regression file under `scripts/`
- `scripts/sdk-package.sh` only if needed to invoke that focused regression

No other tracked path may change.

### Forbidden scope

- deleting or overwriting caller content, following a symlink output, or weakening the existing
  preflight and exit codes;
- adding platform-specific copy branches, nonstandard copy flags, dependency installation, artifact
  repinning, or package-manifest changes;
- changing build flags, artifact names, digests, contents, SDK APIs, Session V1, or browser behavior;
- general shell-portability cleanup outside these two publication sites; and
- benchmark, DSP, runtime, or app integration changes.

## Objective gates

1. Neither builder contains `cp --update=none` or another GNU-only no-clobber spelling; publication
   uses ordinary `cp` only after the existing empty-directory preflight.
2. The focused regression proves an empty real directory accepts publication semantics and copies
   the expected named fixture bytes exactly.
3. A non-empty output directory refuses before the mocked build or copy step; its sentinel bytes are
   unchanged.
4. A symlink output directory refuses and its target remains unchanged. A missing path and a regular
   file each refuse with exit 2.
5. A real macOS invocation reaches artifact publication without `cp: illegal option -- -`.
6. `bash scripts/sdk-package.sh check` passes with the current pinned AudioWorklet and FLAC artifacts.
7. Existing Linux SDK qualification remains green; artifact hashes, filenames, manifest, and packed
   package contents are unchanged.
8. `bash scripts/check-workspace-policy.sh`, the focused regression, and `git diff --check` pass; the
   exact-path diff contains only this issue's allowed files.

## Review and delivery

Sol approved this smallest slice and its gates before implementation. Terra gets one bounded
implementation attempt, followed by fresh Sol adversarial review. Up to two Sol correction attempts
are allowed; after attempt three fails, stop and rescope rather than weakening the contract.

Create and verify the matching GitHub issue before implementation. Checkpoint the issue brief first,
then checkpoint the coherent implementation and evidence. After Sol PASS and upstream delivery,
synchronize the issue evidence and close it only after the unchanged remote SDK qualification is
green.

## Brief evidence and decision record

Sol reviewed the macOS failure, both scripts' identical output preflight, and the package gate. It
ruled that ordinary `cp` after the already-mandatory empty-directory check is the smallest portable
correction. A broader portability abstraction or package change would not discriminate another
product claim.

## Implementation and Sol review evidence

Checkpoint `5ea05840` replaces only the eight GNU-only copy invocations with ordinary `cp`, adds a
mocked output-contract regression, and runs that regression only from SDK package `check` mode.
The regression proves exact happy-path copies plus refusal-before-build for non-empty, symlink,
missing and regular-file outputs. Shell syntax, the focused regression, workspace policy and diff
checks pass on macOS. Sol's first review found no implementation defect but held on an issue-number
typo in this file; correction checkpoint `9ca8ac20` fixes that one allowed-path digit. Fresh Sol
review returns PASS for the bounded implementation.

A real macOS package check now passes the former `cp` failure and reaches FLAC artifact
qualification. It then reports the pre-existing local FLAC reproducibility mismatch (pinned
`a9fc3301...73b65e`, locally rebuilt `3f4b...df48f8`). This issue does not repin or broaden into
that independent finding. Remote Linux package qualification and final issue closure remain
pending. For local app dogfooding, the package gate may take the current host artifact directory
and the complete already-pinned `a9fc3301...73b65e` FLAC closure as explicit inputs; the resulting
tarball still has to pass the unchanged package smoke and red-mutation checks.
