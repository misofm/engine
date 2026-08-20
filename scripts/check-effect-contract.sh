#!/usr/bin/env bash
set -euo pipefail
cd "${1:-.}"
bash scripts/check-effect-runtime-policy.sh .
bash scripts/test-effect-runtime-policy.sh .
bash scripts/check-effect-runtime-fixtures.sh .
bash scripts/test-effect-runtime-fixtures.sh .
cargo test --locked -p miso-engine-effect-contract -p miso-engine-effect-compiler -p miso-engine-conformance
cargo run --locked --release -q -p miso-engine-effect-contract-bench -- --conformance
printf 'effect runtime contract/conformance: ok\n'
