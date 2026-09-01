#!/usr/bin/env bash
# Issue #243 eval 6: the SDK typechecks, and its host mirror is pinned to the shipped declaration.
#
# # Why this is not a sweep row
#
# `tsc` is a dependency, and installing it needs the network. Every row in `scripts/sweep.sh` is
# hermetic by construction, so a row that ran `npm ci` would break the one property the sweep
# exists to have. The same reasoning already keeps `hosts/host-web/qualification`'s
# suite out of the sweep, and this follows it.
#
# The SDK's *behavioural* evals need no `node_modules` at all -- they run under Node's native type
# stripping and are swept as `check-sdk-headless.sh`. What this adds is the static half: the
# strict-mode typecheck, and with it `sdk/test/host-mirror.ts`, which is checked rather than run.
# That file's whole job is to fail COMPILATION when the shipped `.d.ts` and the SDK's adapter
# disagree -- a `bigint` field that became a `number` is caught there and nowhere else until a
# browser refuses a boot.
set -euo pipefail

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)
cd "$repo_root/sdk"

command -v node >/dev/null || { echo "node is required" >&2; exit 2; }

if [[ ! -x node_modules/.bin/tsc ]]; then
  echo "sdk/node_modules is missing; run 'npm ci' in sdk/ first (this gate needs the network once)" >&2
  exit 2
fi

node_modules/.bin/tsc --noEmit --project tsconfig.json
echo "sdk typecheck passed, including the shipped-host mirror pin"
