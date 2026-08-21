#!/usr/bin/env bash
# Validator and runner-negative tests only. This script never invokes the benchmark workload.
set -euo pipefail

script_directory="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
runner="$script_directory/run-graph-compiler-benchmark.sh"
record_fixture="$script_directory/fixtures/graph-benchmark-validator-record.json"
record_validator="$script_directory/graph-benchmark-record-validator.jq"
validator="$script_directory/graph-benchmark-validator.jq"
[[ -x "$runner" ]] || {
    printf 'graph benchmark runner is not executable through its authorized exact path: %s\n' "$runner" >&2
    exit 1
}

for arguments in '--rounds 2' '--retry' 'extra'; do
    if "$runner" $arguments >/dev/null 2>&1; then
        printf 'graph benchmark runner accepted invalid arguments: %s\n' "$arguments" >&2
        exit 1
    fi
done
[[ "$(rg -c 'cargo run --locked --release --quiet -p miso-engine-graph-bench' "$runner")" == 1 ]] || {
    printf 'graph benchmark runner must contain exactly one workload launch\n' >&2
    exit 1
}

command -v jq >/dev/null || { printf 'jq is required for graph validator tests\n' >&2; exit 1; }
record_filter='include "graph-benchmark-record-validator"; graph_benchmark_record_valid'
jq -e -L "$script_directory" "$record_filter" "$record_fixture" >/dev/null
for mutation in \
    '.rounds = 3' \
    '.timing_ns.p95 = 0' \
    '.output_graph_sha256 = "bad"' \
    '.fixture_counts.tracks = 255' \
    '.errors = 1' \
    '.descriptive_only = false'
do
    if jq "$mutation" "$record_fixture" \
        | jq -e -L "$script_directory" "$record_filter" >/dev/null 2>&1
    then
        printf 'graph record validator accepted mutation: %s\n' "$mutation" >&2
        exit 1
    fi
done

jq -n --slurpfile base "$record_fixture" '
    [range(0; 6) as $index |
        $base[0] + {
            benchmark_id: ([
                "graph_compile_256t_1024r_32s",
                "graph_validate_65537_tracks",
                "graph_debug_sha_dot_256t_1024r_32s"
            ][$index % 3]),
            round: (($index / 3 | floor) + 1),
            fixture_counts: (if ($index % 3) == 1 then
                {tracks: 65537, routes: 1, submixes: 0, effects: 0, sidechains: 0}
            else $base[0].fixture_counts end),
            warmup_iterations: (if ($index % 3) == 1 then 0 else 1 end),
            measured_iterations: (if ($index % 3) == 1 then 1 else 5 end),
            output_counts: ($base[0].output_counts + {
                effects: (if ($index % 3) == 1 then 0 else 64 end)
            })
        }
    ]
' | jq -e -L "$script_directory" -f "$validator" >/dev/null
printf 'graph benchmark runner/validator readiness: PASS (workload launches: 0; accepted results: 0)\n'
