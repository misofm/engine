#!/usr/bin/env bash

# Execute a command into a capture file and preserve its exact nonzero status.
run_and_capture() {
    local capture=$1
    shift
    if "$@" >"$capture" 2>&1; then
        return 0
    else
        local status=$?
        return "$status"
    fi
}

# Same-filesystem hard-link creation is atomic and refuses to replace any existing path.
persist_no_clobber() {
    ln -- "$1" "$2"
}

# Extract the sole MQ1_RESULT marker from captured libtest output. Libtest may
# prefix the marker with `test <name> ...`; the marker remains the authority for
# where the JSON payload starts. The complete payload is validated by the
# caller, so trailing text or malformed JSON cannot be promoted accidentally.
extract_mq1_result() {
    local raw_path=$1
    local result_path=$2

    awk '
        BEGIN { marker = "MQ1_RESULT" }
        {
            rest = $0
            while ((position = index(rest, marker)) != 0) {
                before = substr(rest, 1, position - 1)
                after = substr(rest, position + length(marker), 1)
                count++
                if ((position > 1 && before !~ /[[:space:]]$/) ||
                    (after != "" && after !~ /^[[:space:]]$/)) {
                    malformed = 1
                } else {
                    payload = substr(rest, position + length(marker))
                    sub(/^[[:space:]]+/, "", payload)
                    if (count == 1) print payload
                }
                rest = substr(rest, position + length(marker))
            }
        }
        END {
            if (count != 1 || malformed) exit 1
        }
    ' "$raw_path" >"$result_path" || return 1

    jq -e -s '
        length == 1 and
        (.[0] | type == "object" and
            (keys | sort) == ["bank_ns_per_lane_sample", "fixture_sha256", "scalar_ns_per_lane_sample"] and
            (.fixture_sha256 | type == "string" and test("^[0-9a-f]{64}$")) and
            (.bank_ns_per_lane_sample | type == "array" and length == 2 and
                all(.[]; type == "number" and isfinite and . == . and . > 0)) and
            (.scalar_ns_per_lane_sample | type == "array" and length == 2 and
                all(.[]; type == "number" and isfinite and . == . and . > 0)))
    ' "$result_path" >/dev/null
}

# Validate the effect-source citation used by MQ-1. The supplied source commit
# must be on the selected checkout's history and its tracked render sources
# must still match the checkout. The MB-2 label is accepted only for the
# approved X7/X8 fast-tier shaper source shape.
validate_effect_source_selection() {
    local repo=$1
    local source_commit=$2
    local revision=$3
    local baseline_commit=$4
    local source_file
    local -a source_paths

    [[ "$source_commit" =~ ^[0-9a-f]{40}$ ]] || return 1
    case "$revision" in
        e1)
            [[ "$source_commit" == "$baseline_commit" ]] || return 1
            source_paths=(crates/math/src/lane_math.rs crates/transient-shaper/src)
            ;;
        mb2-fast-db)
            [[ "$source_commit" != "$baseline_commit" ]] || return 1
            source_paths=(crates/math/src/lane_math.rs crates/math/src/fast_db.rs crates/transient-shaper/src)
            ;;
        *)
            return 1
            ;;
    esac
    git -C "$repo" cat-file -e "$source_commit^{commit}" 2>/dev/null || return 1
    git -C "$repo" merge-base --is-ancestor "$source_commit" HEAD || return 1
    git -C "$repo" diff --quiet "$source_commit" -- "${source_paths[@]}" || return 1
    [[ -z "$(git -C "$repo" status --porcelain=v1 --untracked-files=all -- "${source_paths[@]}")" ]] || return 1

    if [[ "$revision" == mb2-fast-db ]]; then
        source_file=$(git -C "$repo" show "$source_commit:crates/transient-shaper/src/lib.rs") || return 1
        [[ "$source_file" == *"FAST-DB-CROSSING X7"* &&
            "$source_file" == *"FAST-DB-CROSSING X8"* &&
            "$source_file" == *"fast_level_db("* &&
            "$source_file" == *"fast_gain_from_db("* &&
            "$source_file" != *"log2_lane("* &&
            "$source_file" != *"exp2_lane("* ]] || return 1
    fi
}
