VERDICT: PASS

Issue #1232 attempt 1 -- make the C ABI checker's header mutation legs reach the compiler
(decision-15 stream J). Reviewed commit 3ade8e969 against base b3095ca2a. Export:
/tmp/claude-1002/v1232/src (git archive of 3ade8e969), CARGO_TARGET_DIR=/tmp/claude-1002/v1232/target
(src/target is a symlink to it, because the self-test hard-codes `$workspace_root/target/release`).
The worktree was not touched.

Scope: the diff touches only `capi_abi_self_test` in `scripts/check-capi-abi.sh` (hunk at lines
54-105) and the spec's own attempt record. The default (non-self-test) path, the baseline positive
control and the non-header legs are byte-identical to base. Every deliverable is present:
staging as `<leg>/miso_engine_v1.h` (D1), an unmodified staged control before the legs, with a
named failure message (D2), stderr capture that refuses `No such file or directory` (D3), and a
`cmp -s` refusal of a byte-identical staged copy (D4). Hazard check confirmed: header line 160 is
the `reserved[4]` tail of `miso_engine_v1_engine_config`, and `MISO_ENGINE_V1_ENGINE_CONFIG_SIZE`
(40) is pinned at header_smoke.cpp:8 and abi_smoke.c:25. No leg turned green, so there is no pin gap.

## Findings

### BLOCKER
None.

### MAJOR
None.

### MINOR

MINOR-1 -- The D3 discriminator only works for GCC with English messages. It cannot see a missing
header under clang, so the scripted `*-apple-*` host (Apple clang) or `CC=clang` slips past it.
scripts/check-capi-abi.sh:73 and :78. clang reports a missing include as
`fatal error: 'miso_engine_v1.h' file not found`, which never contains `No such file or directory`.
Reproduced in the export: I staged the three legs under their old names (`$scratch_root/<leg>.h`)
and left the staging control correct. With gcc the self-test is rc 1
(`header-constant-drift failed on a missing file, not on drift`). With
`CC=clang CXX=clang++` it is rc 0 and prints `C ABI mutation tests: ok`, which is the exact
vacuous state this issue removes. The compiler-independent D2 control still catches a regression in
the shared `stage_header`: with clang, stage_header writing `<leg>.h` gives rc 1 through the control.
The hole is therefore limited to a leg whose staging diverges from the control's, for example a
future leg staged by hand on the old pattern, on a run that only uses clang. CI
(qualification.yml:813-816, ubuntu-24.04, default gcc) would still catch it, so this is not MAJOR.
The same applies in principle to a non-English locale: gcc's `%m` text is strerror, and gettext
localizes it. This is unverified here because no gcc/libc translations are installed.
Fix, inside `capi_abi_self_test` only: run the leg with `LC_ALL=C` added to its env and refuse
either phrasing:
`"${common_env[@]}" LC_ALL=C MISO_ENGINE_CAPI_HEADER="$staged" bash "$0" ...` and
`grep -qE "No such file or directory|'miso_engine_v1.h' file not found" "$log"`.
I validated this fix in the export. The clang self-test is rc 0 unmutated, and clang plus old-name
legs is rc 1. The gcc self-test stays rc 0. A stronger, compiler-agnostic alternative would make
each leg prove that `$CC -MM -I"$(dirname "$staged")"` on each fixture lists `$staged`. The
owner's correctness-first rule favours folding the one-line fix into this issue before close rather
than filing a follow-up.

### NIT

NIT-1 -- A missing staged file slips past `expect_header_failure`.
scripts/check-capi-abi.sh:68 and :78. `cmp -s` returns 2 when the staged file is absent, and the
`if` treats that as "differs". The checker then refuses with its own
`C ABI check failure: missing header: ...`, which contains no `No such file or directory`, so the
leg counts as properly red. Reproduced: with the constant leg's `sed` deleted and its staged path
never written, the self-test prints `C ABI mutation tests: ok` (rc 0). This needs two defects at
once. A single staging defect is caught today, because the leg's `sed -i` on a missing file aborts
under `set -e`, and the control catches a failed `cp` (reproduced: rc 1 through the control). Fix:
start `expect_header_failure` with `[[ -f "$staged" ]] || { printf '...%s staged header missing\n' "$name" >&2; return 1; }`.
Alternatively, branch on `cmp`'s status explicitly (0 = identical -> fail, 1 = differs -> continue,
anything else -> fail).

NIT-2 -- The legs prove a non-missing-file failure, not the reason each one names. A future `sed`
that leaves the header syntactically invalid would turn a leg red on a parse error and still pass.
The three current mutations do fail for their named reasons (diagnostics below). A cheap
compiler-agnostic guard is to require the mutated header to compile alone
(`$CC -std=c11 -Wall -Wextra -Werror -pedantic -fsyntax-only -x c "$staged"`) before the leg runs.
That way the red has to come from the fixtures' pins. This is optional and beyond the spec's
deliverables.

## Is `No such file or directory` a sound discriminator? (requested judgment)

- False positives (a legitimate drift diagnostic containing the text): none found, and a false
  positive would fail closed. No header, fixture or checker message contains the phrase. Every leg
  stops at a compile error before any runtime or file I/O. The checker's own failures use
  `missing header:`/`missing native library:`, not strerror text.
- False negatives (a staging failure that slips past): the clang wording and possibly a non-English
  locale (MINOR-1), and a missing staged file reported in the checker's own words (NIT-1).
  Combining the D2 control, the D4 byte-difference check and D3 makes the result sound on the CI
  host. On clang, D3 adds nothing and only the control remains.

## Gates (run by me in the export, x86_64-unknown-linux-gnu, gcc 13.3.0, bash 5.2.21)

1. `bash scripts/check-capi-abi.sh` -> rc 0, `C ABI check: ok (x86_64-unknown-linux-gnu, shared and static linkage)`.
2. `bash scripts/check-capi-abi.sh --self-test` -> rc 0, `C ABI mutation tests: ok`. Also rc 0
   with `CC=clang CXX=clang++`.
3. Red on revert of D1:
   - (a) `stage_header` writing `$scratch_root/$1.h` -> rc 1,
     `header staging control (unmodified staged header) did not pass`. The stderr shows
     `fatal error: miso_engine_v1.h: No such file or directory`.
   - (b) only the three legs staged under their old names, control intact -> rc 1,
     `header-constant-drift failed on a missing file, not on drift:`.
   - Under clang, (a) is rc 1 and (b) is rc 0 (MINOR-1).
4. Layout `sed` replaced by `s/NO_SUCH_PATTERN_1232/x/` -> rc 1, `header-layout-drift did not change the staged header`.
5. Constant `sed` deleted -> rc 1, `header-constant-drift did not change the staged header`.
6. `bash scripts/check-workspace-policy.sh` -> rc 0 `workspace policy: ok`, both in the plain export
   and in a git-initialised copy. `bash scripts/test-workspace-policy.sh` -> rc 0
   `workspace policy mutation tests: ok`.

First diagnostic per leg (reproduced, matches the attempt record):
- header-constant-drift: `crates/capi/tests/c/header_smoke.cpp:7:42: error: static assertion failed`
- header-layout-drift: `crates/capi/tests/c/header_smoke.cpp:8:52: error: static assertion failed`
- header-signature-drift: `crates/capi/tests/c/abi_smoke.c:59:56: error: initialization of 'uint32_t (* const)(void)' ... from incompatible pointer type 'uint32_t (*)(uint32_t)' [-Werror=incompatible-pointer-types]`

## Test value (one sentence each, each mutation reproduced red, green on revert)

- Header staging control: turns red if staging stops producing an includable `miso_engine_v1.h`
  (e.g. `stage_header` reverts to `<leg>.h`), a defect that previously made every header leg red
  for the wrong reason while the self-test printed ok. Reproduced (3a).
- Identical-copy refusal: turns red if a leg's `sed` stops matching after a header edit or is
  deleted, which would otherwise turn the leg into a duplicate of the control. Reproduced (gates 4
  and 5).
- Missing-file refusal: turns red if a leg is staged under a name the fixtures cannot include while
  the control stays correct. Reproduced (3b) with gcc only (MINOR-1).
- Repaired header legs: turn the self-test red when the fixtures stop pinning what the leg mutates.
  Base b3095ca2a never caught this.
  - Signature: I defeated the abi_smoke.c:59 pin with a `(uint32_t (*)(void))(uintptr_t)` cast.
    Commit -> rc 1 `C ABI mutation unexpectedly passed: header-signature-drift`. The base script
    with the same fixture -> rc 0 `ok`, i.e. vacuous.
  - Constant: I unpinned the ABI constant fully (both static asserts and the three runtime uses
    replaced by the literal). Commit -> rc 1 `unexpectedly passed: header-constant-drift`.
  - Layout: with both size asserts removed, the leg stays red through the runtime (`abi-smoke`
    exits 2 from `engine_create`), which shows that it reaches the linked runtime.
