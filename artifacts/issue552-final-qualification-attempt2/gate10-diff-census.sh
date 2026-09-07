#!/usr/bin/env bash
set -euo pipefail
cd /home/bl/misofm/engine-js-hex-recovery
original=e684275cb349e49a66e73f7f030da6520dd63607
current=origin/main
printf '%s\n' 'DIFF CHECK original base'; git diff --check "$original...HEAD"
printf '%s\n' 'DIFF CHECK current base'; git diff --check "$current...HEAD"
printf '%s\n' 'STATUS'; git status --short --branch
printf '%s\n' 'ORIGINAL BASE NAME-STATUS'; git diff --name-status "$original...HEAD"
printf '%s\n' 'CURRENT BASE NAME-STATUS'; git diff --name-status "$current...HEAD"
printf '%s\n' 'CURRENT BASE PATH CLASSIFICATION'
allowed='sdk/src/core/hex.ts|sdk/src/core/asset.ts|sdk/test/boot-evals.mjs|hosts/host-web/web/hex-lower.js|hosts/host-web/web/stem-store/incremental-sha256.js|hosts/host-web/qualification/qualification.js|hosts/host-web/qualification/server.mjs|hosts/host-web/tests/stem-store-hash-v1.mjs|hosts/host-web/web/stem-store/incremental-sha256.provenance.json|scripts/check-stem-store-v1.mjs'
bad=0
while IFS= read -r path; do
  if [[ "$path" =~ ^($allowed)$ ]]; then printf 'PRODUCT %s\n' "$path"
  elif [[ "$path" == .github/ISSUE_SPECS/* || "$path" == artifacts/* ]]; then printf 'EVIDENCE %s\n' "$path"
  else printf 'UNEXPECTED %s\n' "$path"; bad=1
  fi
done < <(git diff --name-only "$current...HEAD")
printf '%s\n' 'CURRENT BASE COUNTS'
printf 'product='; git diff --name-only "$current...HEAD" | rg -c "^($allowed)$" || true
printf 'issue_specs='; git diff --name-only "$current...HEAD" | rg -c '^\.github/ISSUE_SPECS/' || true
printf 'evidence='; git diff --name-only "$current...HEAD" | rg -c '^artifacts/' || true
printf '%s\n' 'PROHIBITED CURRENT-BASE PATH SEARCH'
git diff --name-only "$current...HEAD" | rg '(^|/)(Cargo\.lock|Cargo\.toml|.*\.rs$|.*\.abi|.*pcm|.*artifact\.sha256$|dist/|generated/)' && bad=1 || true
if (( bad != 0 )); then printf '%s\n' 'diff/path census: FAIL'; exit 1; fi
printf '%s\n' 'diff/path census: PASS (current-base product paths are exactly ten; other paths are issue specs/evidence)'
