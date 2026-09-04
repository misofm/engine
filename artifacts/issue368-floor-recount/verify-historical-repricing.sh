#!/usr/bin/env bash
set -euo pipefail
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
src="$root/artifacts/mono3/console-benchmark.accepted.jsonl"
out="$root/artifacts/issue368-floor-recount/historical-mono3-repriced.jsonl"
[[ ! -e "$out" ]] || { printf 'refusing to overwrite %s\n' "$out" >&2; exit 1; }
jq -cs -L "$root/scripts" 'include "console-benchmark-record-lib"; . as $all |
 ([ $all[]|select(.record=="console_session")|{key:(.workload_kind+":"+(.round|tostring)),value:.}]|from_entries) as $by |
 [ $all[]|if .record=="console_session" then (floor_pins[.workload_kind]) as $pin |
 .floor_cycles_per_lane_sample=(if $pin[0]==null then null else $pin[0]*$pin[1]/lane_ops_per_cycle end) |
 .percent_of_floor=(if .floor_cycles_per_lane_sample==null then null else 100*.floor_cycles_per_lane_sample/.cycles_per_lane_sample end) |
 .isolated_percent_of_floor=(if .floor_control_row=="none" then null else
 ($by[.floor_control_row+":"+(.round|tostring)]) as $c | (floor_pins[$c.workload_kind]) as $cp |
 100*(.floor_cycles_per_lane_sample-$cp[0]*$cp[1]/lane_ops_per_cycle)/.isolated_cycles_per_lane_sample end)
 else . end]|.[]' "$src" >"$out"
jq -e -n --slurpfile a "$src" --slurpfile b "$out" 'def strip:
 del(.floor_cycles_per_lane_sample,.percent_of_floor,.isolated_percent_of_floor);
 [$a[]|strip]==[$b[]|strip]' >/dev/null
jq -e -s -L "$root/scripts" 'include "console-benchmark-record-lib";
 all(.[]|select(.record=="console_session");floor_shape)' "$out" >/dev/null
! jq -e -s -L "$root/scripts" 'include "console-benchmark-record-lib";
 all(.[]|select(.record=="console_session");floor_shape)' "$src" >/dev/null
jq -e -s -L "$root/scripts" 'include "console-benchmark-record-lib";
 first(.[]|select(.record=="console_session" and .workload_kind=="nine_track_baseline"))|floor_shape' "$out" >/dev/null
! jq -e -s -L "$root/scripts" 'include "console-benchmark-record-lib";
 map(if .record=="console_session" and .workload_kind=="sixty_four_track_console"
 then .cycles_per_lane_sample=-1 else . end)|
 all(.[]|select(.record=="console_session");floor_shape)' "$out" >/dev/null
printf 'historical repricing: PASS; source_sha256=%s repriced_sha256=%s records=%s\n' \
 "$(sha256sum "$src"|awk '{print $1}')" "$(sha256sum "$out"|awk '{print $1}')" "$(wc -l <"$out")"
