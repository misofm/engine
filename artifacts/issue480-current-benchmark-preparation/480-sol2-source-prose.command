set -euo pipefail
expected=$(printf '%s\n' .github/ISSUE_SPECS/480-current-benchmark-build-identity.md scripts/preflight-builtins-current-benchmark.sh scripts/run-builtins-current-benchmark.sh scripts/test-builtins-current-benchmark.sh)
actual=$(git diff --name-only | sort)
[[ "$actual" == "$expected" ]]
git diff --check
[[ $(git diff --name-only -- '*.rs' 'Cargo.toml' 'Cargo.lock' '.cargo/**' '*.jq' 'fixtures/**' '.github/workflows/**' 'scripts/run-builtins-benchmark.sh' 'scripts/run-builtins-full-chain-benchmark.sh') == '' ]]
! rg -n 'MISO_ENGINE_BENCH_(PANIC|DEBUG|DEBUG_ASSERTIONS|OVERFLOW_CHECKS|INCREMENTAL|STRIP|SPLIT_DEBUG_INFO)=' scripts/run-builtins-current-benchmark.sh
rg -q 'split_debuginfo:"off"' scripts/preflight-builtins-current-benchmark.sh
rg -q 'CARGO_PROFILE_RELEASE_SPLIT_DEBUGINFO=off' scripts/preflight-builtins-current-benchmark.sh
rg -q 'rpath:false' scripts/preflight-builtins-current-benchmark.sh
rg -q '#431 remains unspent and unauthorized' .github/ISSUE_SPECS/480-current-benchmark-build-identity.md
