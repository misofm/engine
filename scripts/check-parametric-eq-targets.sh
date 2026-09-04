#!/usr/bin/env bash
# Thin wrapper, kept so any caller of this name keeps working.
#
# The cargo cross-target matrix moved to scripts/check-cross-targets.sh, which deduplicates it
# against scripts/check-builtins-targets.sh's and scripts/check-effect-interchange-targets.sh's
# matrices under one cached target dir per target triple. The hermetic render-contract construct
# bans (issue #87 F1/F3/F4/F5/F10) moved to scripts/check-parametric-eq-render-contract.sh.
# docs/rulings/de-versioning-inventory.md's note that the `KernelBackendV1`/`PreparedDeltaBankKernelV1`
# forbidden-pattern list must keep the deleted spelling now points at that file.
set -euo pipefail
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
bash "$root/scripts/check-cross-targets.sh"
bash "$root/scripts/check-parametric-eq-render-contract.sh"
