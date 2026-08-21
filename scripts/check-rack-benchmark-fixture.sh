#!/usr/bin/env bash
# Verify the single frozen Issue-038 workload declaration and its exact manifest.
set -euo pipefail
[[ "$#" -le 1 ]] || { printf 'usage: %s [FIXTURE_ROOT]\n' "$0" >&2; exit 2; }
repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
fixture_root=${1:-"$repo_root/fixtures/rack/issue038-v1"}
manifest="$fixture_root/MANIFEST.tsv"
[[ -f "$manifest" && ! -L "$manifest" ]] || { printf 'missing regular Issue-038 manifest\n' >&2; exit 1; }
[[ "$(sha256sum "$manifest" | awk '{print $1}')" == "2d6b8c4b11bb00a17185d7777300194bf53ab30d86cf581a55886f07c5273985" ]] || {
    printf 'Issue-038 manifest identity mismatch\n' >&2
    exit 1
}
mapfile -t actual_paths < <(find "$fixture_root" -mindepth 1 -maxdepth 1 -type f -printf '%f\n' | sort)
[[ "${actual_paths[*]}" == "MANIFEST.tsv workloads.toml" ]] || {
    printf 'Issue-038 fixture has missing, non-regular, or unlisted entries\n' >&2
    exit 1
}
[[ "$(sed -n '1p' "$manifest")" == $'path\tlength\tsha256' ]] || {
    printf 'Issue-038 manifest header mismatch\n' >&2
    exit 1
}
[[ "$(wc -l <"$manifest")" == 2 ]] || { printf 'Issue-038 manifest cardinality mismatch\n' >&2; exit 1; }
IFS=$'\t' read -r path length expected_sha < <(sed -n '2p' "$manifest")
[[ "$path" == "workloads.toml" && "$length" =~ ^[0-9]+$ && "$expected_sha" =~ ^[0-9a-f]{64}$ ]] || {
    printf 'Issue-038 manifest record mismatch\n' >&2
    exit 1
}
payload="$fixture_root/$path"
[[ -f "$payload" && ! -L "$payload" ]] || { printf 'missing regular Issue-038 workload payload\n' >&2; exit 1; }
[[ "$(wc -c <"$payload")" == "$length" ]] || { printf 'Issue-038 workload length mismatch\n' >&2; exit 1; }
[[ "$(sha256sum "$payload" | awk '{print $1}')" == "$expected_sha" ]] || {
    printf 'Issue-038 workload hash mismatch\n' >&2
    exit 1
}
grep -Fqx 'observations_per_round = 1000' "$payload"
grep -Fqx 'sample_rate_hz = 48000' "$payload"
grep -Fqx 'quantum_frames = 128' "$payload"
[[ "$(rg -c '^  "(scalar_eight_tracks|host_selected_eight_track_bank|mixed_twelve_track_graph):' "$payload")" == 3 ]]
