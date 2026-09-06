set -euo pipefail
expected=$(printf '%s\n' .github/ISSUE_SPECS/480-current-benchmark-build-identity.md scripts/preflight-builtins-current-benchmark.sh scripts/run-builtins-current-benchmark.sh scripts/test-builtins-current-benchmark.sh)
actual=$(git diff --name-only | sort)
[[ "$actual" == "$expected" ]]
git diff --check
[[ $(git diff --name-only -- '*.rs' 'Cargo.toml' 'Cargo.lock' '.cargo/**' '*.jq' 'fixtures/**' '.github/workflows/**' 'scripts/run-builtins-benchmark.sh' 'scripts/run-builtins-full-chain-benchmark.sh') == '' ]]
rg -q 'CARGO_INCREMENTAL=0 RUSTC="\$rustc_executable" RUSTC_WRAPPER= RUSTC_WORKSPACE_WRAPPER=' scripts/preflight-builtins-current-benchmark.sh
rg -Fq "CARGO_ENCODED_RUSTFLAGS='-Ctarget-feature=+avx2,+fma'" scripts/preflight-builtins-current-benchmark.sh
rg -q 'cargo build --locked --release -p bench --target "\$target_triple"' scripts/preflight-builtins-current-benchmark.sh
rg -q '#431 remains.*unspent and unauthorized' .github/ISSUE_SPECS/480-current-benchmark-build-identity.md
