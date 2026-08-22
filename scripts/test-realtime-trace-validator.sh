#!/usr/bin/env bash
set -euo pipefail

workspace_dir=$(cd "$(dirname "$0")/.." && pwd)
validator="$workspace_dir/scripts/validate-realtime-trace.sh"
scratch=$(mktemp -d "${TMPDIR:-/tmp}/miso-engine-trace-validator.XXXXXX")
trap 'rm -rf "$scratch"' EXIT

write_clean() {
  local root=$1
  mkdir -p "$root"
  printf '%s\n' \
    '1000.000000 write(2, "BEGIN", 5) = 5' \
    '1000.100000 write(2, "END", 3) = 3' \
    >"$root/trace.100"
  printf '%s\n' \
    '999.900000 futex(0x1, FUTEX_WAIT, 0, NULL <unfinished ...>' \
    '1000.200000 <... futex resumed>) = 0' \
    >"$root/trace.101"
}

write_clean "$scratch/clean"
"$validator" "$scratch/clean" BEGIN END 1 >/dev/null

cp -R "$scratch/clean" "$scratch/render-injection"
sed -i '2i 1000.050000 getpid() = 1' "$scratch/render-injection/trace.100"
if "$validator" "$scratch/render-injection" BEGIN END 1 >/dev/null 2>&1; then
  printf 'render-thread syscall injection escaped\n' >&2
  exit 1
fi

cp -R "$scratch/clean" "$scratch/aux-injection"
sed -i '2i 1000.050000 getpid() = 1' "$scratch/aux-injection/trace.101"
if "$validator" "$scratch/aux-injection" BEGIN END 1 >/dev/null 2>&1; then
  printf 'auxiliary-thread syscall injection escaped\n' >&2
  exit 1
fi

printf 'all-TID realtime trace validator mutations: PASS\n'
