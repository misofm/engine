#!/usr/bin/env bash
set -euo pipefail

if [[ "$#" -ne 4 ]]; then
  printf 'usage: validate-realtime-trace.sh TRACE_DIRECTORY BEGIN_MARKER END_MARKER EXPECTED_INTERVALS\n' >&2
  exit 2
fi

trace_dir=$1
begin_marker=$2
end_marker=$3
expected_intervals=$4
[[ "$expected_intervals" =~ ^[1-9][0-9]*$ ]] || {
  printf 'expected interval count must be a positive integer\n' >&2
  exit 2
}

mapfile -t trace_files < <(find "$trace_dir" -maxdepth 1 -type f -name 'trace.*' | sort)
[[ "${#trace_files[@]}" -gt 0 ]] || {
  printf 'no per-TID trace files found\n' >&2
  exit 1
}

marker_file=
for candidate in "${trace_files[@]}"; do
  if rg -q --fixed-strings "$begin_marker" "$candidate" \
    || rg -q --fixed-strings "$end_marker" "$candidate"; then
    [[ -z "$marker_file" ]] || {
      printf 'markers occur on multiple traced TIDs\n' >&2
      exit 1
    }
    marker_file=$candidate
  fi
done
[[ -n "$marker_file" ]] || {
  printf 'render marker TID is missing\n' >&2
  exit 1
}

interval_file=$(mktemp "${TMPDIR:-/tmp}/miso-engine-trace-intervals.XXXXXX")
trap 'rm -f "$interval_file"' EXIT
awk -v begin="$begin_marker" -v end="$end_marker" '
  index($0, begin) {
    if (inside) { exit 20 }
    if ($1 !~ /^[0-9]+\.[0-9]+$/) { exit 21 }
    start = $1
    inside = 1
    next
  }
  index($0, end) {
    if (!inside) { exit 22 }
    if ($1 !~ /^[0-9]+\.[0-9]+$/ || $1 <= start) { exit 23 }
    print start "\t" $1
    inside = 0
    count += 1
    next
  }
  END {
    if (inside) { exit 24 }
  }
' "$marker_file" >"$interval_file" || {
  printf 'render markers are malformed, nested or unpaired\n' >&2
  exit 1
}

actual_intervals=$(wc -l <"$interval_file")
[[ "$actual_intervals" -eq "$expected_intervals" ]] || {
  printf 'render interval count differs: expected=%s actual=%s\n' \
    "$expected_intervals" "$actual_intervals" >&2
  exit 1
}

for candidate in "${trace_files[@]}"; do
  unexpected=$(awk -v begin="$begin_marker" -v end="$end_marker" '
    FNR == NR {
      starts[++count] = $1
      ends[count] = $2
      next
    }
    index($0, begin) || index($0, end) { next }
    $1 ~ /^[0-9]+\.[0-9]+$/ {
      for (interval = 1; interval <= count; interval += 1) {
        if ($1 > starts[interval] && $1 < ends[interval]) {
          print
          break
        }
      }
    }
  ' "$interval_file" "$candidate")
  [[ -z "$unexpected" ]] || {
    printf 'forbidden syscall overlaps an armed interval in %s:\n%s\n' \
      "$(basename "$candidate")" "$unexpected" >&2
    exit 1
  }
done

printf '{"schema_version":1,"trace_files":%s,"intervals":%s,"violations":0}\n' \
  "${#trace_files[@]}" "$actual_intervals"
