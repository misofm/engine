#!/usr/bin/env bash
# Negative argument checks only. This script deliberately never invokes the benchmark.
set -euo pipefail

script_directory="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
runner="$script_directory/run-protocol-benchmark.sh"
record_fixture="$script_directory/fixtures/protocol-benchmark-validator-record.json"
validator="$script_directory/protocol-benchmark-validator.jq"
[[ -x "$runner" ]] || {
    printf 'protocol benchmark runner is not executable through its authorized exact path: %s\n' "$runner" >&2
    exit 1
}

for arguments in '' '--rounds 1' '--rounds 3' '--rounds 2 extra'; do
    if "$runner" $arguments >/dev/null 2>&1; then
        printf 'protocol benchmark runner accepted invalid arguments: %s\n' "$arguments" >&2
        exit 1
    fi
done

command -v jq >/dev/null || { printf 'jq is required for validator tests\n' >&2; exit 1; }
record_filter='include "protocol-benchmark-record-validator"; protocol_benchmark_record_valid'
jq -e -L "$script_directory" "$record_filter" "$record_fixture" >/dev/null
if jq '.toolchain = 7' "$record_fixture" \
    | jq -e -L "$script_directory" "$record_filter" >/dev/null 2>&1
then
    printf 'protocol benchmark record validator accepted non-string toolchain metadata\n' >&2
    exit 1
fi
jq -n --slurpfile base "$record_fixture" '
    [range(0; 216) as $index |
        $base[0] + {
            round: (if $index < 108 then 1 else 2 end),
            format: (if ($index < 54 or $index >= 162) then "btlv" else "flatbuffers" end),
            order_index: (if ($index < 54 or ($index >= 108 and $index < 162)) then 0 else 1 end),
            frame_label: ("fixture." + (($index % 54) | tostring))
        }
    ]
' | jq -e -L "$script_directory" -f "$validator" >/dev/null
printf 'protocol benchmark runner/validator negative tests: ok (failed exact shell launches: 1; completed unaccepted workloads: 1; accepted results: 0)\n'
