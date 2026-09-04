#!/usr/bin/env bash
# Issue #243 eval 6: the SDK typechecks, and its host mirror is pinned to the shipped declaration.
#
# # Why this needs its own CI step, not a hermetic gate row
#
# `tsc` is a dependency, and installing it needs the network. Every other gate run from CI is
# hermetic by construction, so a row that ran `npm ci` would break that property. The same
# reasoning already keeps `hosts/host-web/qualification`'s suite in its own CI step too.
#
# The SDK's *behavioural* evals need no `node_modules` at all -- they run under Node's native type
# stripping, hermetically, from `check-sdk-headless.sh`. What this adds is the static half: the
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
