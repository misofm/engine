#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
source "$root/scripts/lib/gate.sh"
scratch=$(mktemp -d); trap 'rm -rf -- "$scratch"' EXIT
before_pwd=$PWD
before_opts=$(set -o)
mkdir -p "$scratch/src"
printf 'clean\n' >"$scratch/src/clean.txt"
GATE_FAILURE_PREFIX='gate test' gate_scan_forbidden clean 'forbidden' '' "$scratch/src"
printf 'forbidden\n' >"$scratch/src/match.txt"
if GATE_FAILURE_PREFIX='gate test' gate_scan_forbidden match 'forbidden' '' "$scratch/src"; then exit 1; fi
rm "$scratch/src/match.txt"
if GATE_FAILURE_PREFIX='gate test' gate_scan_forbidden missing 'forbidden' '' "$scratch/missing" 2>/dev/null; then exit 1; fi
if GATE_FAILURE_PREFIX='gate test' gate_scan_forbidden combined 'forbidden' '' "$scratch/src" "$scratch/missing" 2>/dev/null; then exit 1; fi
mkdir -p "$scratch/bin"
printf '#!/usr/bin/env bash\nexit 2\n' >"$scratch/bin/rg"
chmod +x "$scratch/bin/rg"
if PATH="$scratch/bin:$PATH" GATE_FAILURE_PREFIX='gate test' \
    gate_scan_forbidden execution-error 'forbidden' '' "$scratch/src" 2>/dev/null; then exit 1; fi
[[ "$PWD" == "$before_pwd" ]] || { echo 'gate changed caller cwd' >&2; exit 1; }
[[ "$(set -o)" == "$before_opts" ]] || { echo 'gate changed caller shell options' >&2; exit 1; }
(cd "$scratch" && bash "$root/scripts/check-rack-policy.sh" "$root" >/dev/null)
printf 'gate library tests: ok\n'
