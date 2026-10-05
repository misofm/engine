# Make the C ABI checker's header mutation legs reach the compiler

Stream J of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-0).

## Mission

`scripts/check-capi-abi.sh --self-test` claims to prove that the C ABI gate refuses a drifted
header. Its three header legs are vacuous today, so make each of them fail for the reason it names,
and prove first that an unmutated copy, staged the same way, passes.

## The defect (found by the #1206 attempt 1 verdict, MINOR-2)

- The legs `header-constant-drift`, `header-layout-drift` and `header-signature-drift` copy
  `crates/capi/include/miso_engine_v1.h` to `$scratch_root/abi-version.h`, `layout.h` and
  `signature.h`, mutate the copy, and pass its path as `MISO_ENGINE_CAPI_HEADER`.
- The checker adds `-I"$(dirname "$header")"`. Both fixtures
  (`crates/capi/tests/c/abi_smoke.c` and `crates/capi/tests/c/header_smoke.cpp`) say
  `#include "miso_engine_v1.h"`. So every mutated run stops at
  `fatal error: miso_engine_v1.h: No such file or directory`, before any static assertion,
  signature check or runtime ABI-version check runs.
- Proof from the verdict: the **unmodified** header, copied as `layout.h` and run the same way,
  also fails with rc 1 and the same error. `expect_failure` accepts any failure, so the self-test
  prints `C ABI mutation tests: ok` while proving nothing about header drift.
- Real protection exists elsewhere: the `abi_smoke.c` offset pins turn red under a swapped header
  saved as `<dir>/miso_engine_v1.h` (the verdict's M4). This issue repairs the self-test's own
  claim, not the ABI.
- Every spec that cites "`check-capi-abi.sh --self-test` ok" as header-drift evidence relied on
  this vacuous leg.

## Invariants

- The frozen C ABI does not change: no header, fixture, symbol, size, offset or `ABI_VERSION`
  moves. This is a tooling-only repair.
- The checker's default (non-self-test) path behaves exactly as it does now.
- The self-test keeps its existing positive control (the unmutated checker against the real
  header) and its five non-header legs (`missing-c-compiler`, `symbol-addition`,
  `symbol-removal`, `link-failure`, `static-link-failure`) unchanged.
- A test that greps source or prose is refused. Reading the checker's own stderr from a mutated
  run is not source grepping.

## Deliverables

1. Each header leg writes its mutated header as `miso_engine_v1.h` inside its own scratch
   directory (for example `$scratch_root/abi-version/miso_engine_v1.h`), and passes that path as
   `MISO_ENGINE_CAPI_HEADER`.
2. A header positive control runs before the three header legs: an **unmodified** copy of the
   header, staged exactly as the legs stage theirs (its own directory, named `miso_engine_v1.h`,
   passed through `MISO_ENGINE_CAPI_HEADER`), must pass the checker. If it fails, the self-test
   fails with a message naming the staging control.
3. Each header leg must fail for a reason other than a missing header. The self-test captures
   each leg's stderr and fails if it contains `No such file or directory`. The leg must still be
   red.
4. Each mutation must actually change the staged copy. The self-test fails if a staged mutated
   header is byte-identical to the original (a `sed` pattern that stopped matching after a header
   edit would otherwise make the leg identical to the positive control).

## Authorized paths

- `scripts/check-capi-abi.sh` (the `capi_abi_self_test` function only).
- This spec's own record sections.

## Non-goals

- No new mutation legs, no change to the fixtures or the header, no new script or framework.
- No change to CI wiring: `qualification.yml` already runs `check-capi-abi.sh --self-test`.
- No aarch64 or macOS leg beyond what the script already supports.

## Hazards

- Once the legs reach the compiler, a mutation may turn out **green**: the fixtures might not pin
  what the leg mutates. That is a real ABI-pin gap, not a self-test defect. Do not weaken or
  delete the leg; stop, record the leg and the mutation, and file a successor for the missing pin.
- `header-layout-drift` edits the first `uint64_t reserved[4];` in the header. Confirm which
  struct that is at the head commit, and record it.

## Objective gates

1. `bash scripts/check-capi-abi.sh` exits 0.
2. `bash scripts/check-capi-abi.sh --self-test` exits 0 and prints `C ABI mutation tests: ok`.
3. Red on revert: with only deliverable 1 reverted (the legs staged under their old names), the
   self-test exits nonzero, through deliverable 2 or 3. Record the run.
4. Mutation: replace the `header-layout-drift` `sed` expression with one that matches nothing; the
   self-test exits nonzero through deliverable 4. Record the run.
5. Mutation: delete the `UINT32_C(0x00010000)` edit from `header-constant-drift` (stage an
   unmutated copy); the self-test exits nonzero. Record the run.
6. `bash scripts/check-workspace-policy.sh` and `bash scripts/test-workspace-policy.sh` exit 0.

## Evidence

- The output of every gate command above.
- For each header leg, the first compiler or runtime diagnostic that now makes it red (a static
  assertion, a signature mismatch, or the ABI-version check), quoted from one run.
- Test value, one sentence: the repaired legs turn red if the header's ABI constant, a struct
  layout or a function signature drifts without the fixtures following, which the vacuous legs
  never exercised.

## Standing rules for the implementer

- Work only from this body. Read `scripts/check-capi-abi.sh` first; do not survey the workspace.
- Attempt budget: three attempts, one adversarial verdict each.

## Attempt record

### Attempt 1 (implementer, 2026-10-05)

Change: `capi_abi_self_test` in `scripts/check-capi-abi.sh` only. `stage_header <leg>` copies the
real header to `$scratch_root/<leg>/miso_engine_v1.h`; `expect_header_failure` refuses a staged
copy that is byte-identical to the original (`cmp -s`), requires the checker to fail, captures its
stderr and refuses a failure whose stderr contains `No such file or directory`. A header staging
control (`header-staging-control/miso_engine_v1.h`, unmodified, passed through
`MISO_ENGINE_CAPI_HEADER`) must pass before the three legs run. The baseline control and the
non-header legs are unchanged; the default path is untouched.

Hazard check: at the head commit the first `uint64_t reserved[4];` is the tail of
`miso_engine_v1_engine_config` (header line 160); its size is pinned by
`MISO_ENGINE_V1_ENGINE_CONFIG_SIZE` (40) in both fixtures. No leg turned green, so no pin gap.

Gates (x86_64-unknown-linux-gnu):

1. `bash scripts/check-capi-abi.sh` -> rc 0, `C ABI check: ok (x86_64-unknown-linux-gnu, shared and static linkage)`.
2. `bash scripts/check-capi-abi.sh --self-test` -> rc 0, `C ABI mutation tests: ok`.
3. Red on revert (`stage_header` writing `$scratch_root/<leg>.h` again) -> rc 1,
   `C ABI mutation self-test FAILED: header staging control (unmodified staged header) did not pass`
   (stderr shows `fatal error: miso_engine_v1.h: No such file or directory`). With the staging
   control also removed, deliverable 3 catches it: rc 1,
   `C ABI mutation self-test FAILED: header-constant-drift failed on a missing file, not on drift:`.
4. Layout `sed` replaced by `s/NO_SUCH_PATTERN_1232/x/` -> rc 1,
   `C ABI mutation self-test FAILED: header-layout-drift did not change the staged header`.
5. Constant `sed` deleted -> rc 1,
   `C ABI mutation self-test FAILED: header-constant-drift did not change the staged header`.
6. `bash scripts/check-workspace-policy.sh` rc 0; `bash scripts/test-workspace-policy.sh` rc 0
   (`workspace policy mutation tests: ok`).

First diagnostic per leg, with the staged header named correctly:

- header-constant-drift: `crates/capi/tests/c/header_smoke.cpp:7:42: error: static assertion failed` (`MISO_ENGINE_V1_ABI_VERSION == UINT32_C(0x00010000)`).
- header-layout-drift: `crates/capi/tests/c/header_smoke.cpp:8:52: error: static assertion failed` (`sizeof(miso_engine_v1_engine_config) == MISO_ENGINE_V1_ENGINE_CONFIG_SIZE`).
- header-signature-drift: `crates/capi/tests/c/abi_smoke.c:59:56: error: initialization of 'uint32_t (* const)(void)' ... from incompatible pointer type 'uint32_t (*)(uint32_t)' [-Werror=incompatible-pointer-types]`.

Test value: the repaired legs turn red if the header's ABI constant, a struct layout or a function
signature drifts without the fixtures following, which the vacuous legs never exercised; the
staging control and the identical-copy check turn red if staging or a `sed` pattern silently stops
producing a real mutation.
