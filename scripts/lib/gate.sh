#!/usr/bin/env bash

# Shared checked-search primitives. Callers set GATE_FAILURE_PREFIX before invoking these.

gate_fail() {
    printf '%s: %s\n' "${GATE_FAILURE_PREFIX:-gate policy failure}" "$1" >&2
    return 1
}

gate_scan_forbidden() {
    local description="$1" pattern="$2" glob="$3"
    shift 3
    local roots=("$@") output rc missing root
    if [[ -n "$glob" ]]; then
        if output="$(rg -n "$pattern" --glob "$glob" "${roots[@]}" 2>&1)"; then rc=0; else rc=$?; fi
    else
        if output="$(rg -n "$pattern" "${roots[@]}" 2>&1)"; then rc=0; else rc=$?; fi
    fi
    case "$rc" in
        0) printf '%s\n' "$output" >&2; gate_fail "$description"; return 1 ;;
        1) return 0 ;;
        *)
            missing=()
            for root in "${roots[@]}"; do [[ -e "$root" ]] || missing+=("$root"); done
            printf '%s\n' "$output" >&2
            if ((${#missing[@]})); then
                gate_fail "$description scan could not run (rg exit $rc): missing search path(s): ${missing[*]}"
            else
                gate_fail "$description scan errored (rg exit $rc)"
            fi
            return 1
            ;;
    esac
}

# Capture a search where both match and clean no-match are valid results. Exit 0 for either
# predicate result and nonzero only when the search itself could not run, preserving partial output
# in the diagnostic while keeping callers independent of pipefail.
gate_scan_collect() {
    local description="$1" pattern="$2" glob="$3"
    shift 3
    local roots=("$@") output rc
    if [[ -n "$glob" ]]; then
        if output="$(rg -n "$pattern" --glob "$glob" "${roots[@]}" 2>&1)"; then rc=0; else rc=$?; fi
    else
        if output="$(rg -n "$pattern" "${roots[@]}" 2>&1)"; then rc=0; else rc=$?; fi
    fi
    case "$rc" in
        0|1) printf '%s' "$output"; return 0 ;;
        *)
            printf '%s\n' "$output" >&2
            gate_fail "$description scan errored (rg exit $rc)"
            return "$rc"
            ;;
    esac
}

gate_scan_text_collect() {
    local description="$1" pattern="$2" input="$3" output rc
    if output="$(rg -n "$pattern" <<<"$input" 2>&1)"; then rc=0; else rc=$?; fi
    case "$rc" in
        0|1) printf '%s' "$output"; return 0 ;;
        *)
            printf '%s\n' "$output" >&2
            gate_fail "$description scan errored (rg exit $rc)"
            return "$rc"
            ;;
    esac
}

gate_scan_required() {
    local description="$1" pattern="$2" glob="$3"
    shift 3
    local output rc
    if [[ -n "$glob" ]]; then
        if output="$(rg -n "$pattern" --glob "$glob" "$@" 2>&1)"; then rc=0; else rc=$?; fi
    else
        if output="$(rg -n "$pattern" "$@" 2>&1)"; then rc=0; else rc=$?; fi
    fi
    if [[ "$rc" == 0 ]]; then
        printf '%s\n' "$output"
        return 0
    fi
    printf '%s\n' "$output" >&2
    gate_fail "$description search failed (rg exit $rc)"
    return "$rc"
}

# Capture a filesystem enumeration.  find's zero-row result is valid for callers that have
# optional populations; callers requiring a population must check the captured value themselves.
# Any find failure is fatal, including useful output emitted before the failure.
gate_find_collect() {
    local description="$1" output rc
    shift
    if output="$(find "$@" 2>&1)"; then rc=0; else rc=$?; fi
    if [[ "$rc" == 0 ]]; then
        printf '%s' "$output"
        return 0
    fi
    printf '%s\n' "$output" >&2
    gate_fail "$description traversal errored (find status $rc)"
    return "$rc"
}

gate_sort_lines() {
    local description="$1" input="$2" output rc
    if output="$(printf '%s\n' "$input" | sort)"; then rc=0; else rc=$?; fi
    if [[ "$rc" == 0 ]]; then
        printf '%s' "$output"
        return 0
    fi
    printf '%s\n' "$output" >&2
    gate_fail "$description sort errored (sort status $rc)"
    return "$rc"
}

gate_unique_nonempty_lines() {
    local description="$1" input="$2" output rc
    if output="$(printf '%s\n' "$input" | awk 'NF && !seen[$0]++')"; then rc=0; else rc=$?; fi
    [[ "$rc" == 0 ]] || { gate_fail "$description uniqueness filter errored (awk status $rc)"; return "$rc"; }
    printf '%s' "$output"
}

gate_join_lines() {
    local description="$1" delimiter="$2" input="$3" output rc
    if output="$(printf '%s\n' "$input" | paste -sd "$delimiter" -)"; then rc=0; else rc=$?; fi
    [[ "$rc" == 0 ]] || { gate_fail "$description join errored (paste status $rc)"; return "$rc"; }
    printf '%s' "$output"
}

# Exclude allowlisted rows from already-collected text. An empty result is valid, but an rg
# execution error is not. Keeping this separate from the source scan makes both producer statuses
# observable even when the failing command emitted useful partial output.
gate_filter_exclude() {
    local description="$1" pattern="$2" input="$3" output rc
    [[ -n "$input" ]] || return 0
    if output="$(printf '%s\n' "$input" | rg -v "$pattern" 2>&1)"; then rc=0; else rc=$?; fi
    case "$rc" in
        0) printf '%s' "$output" ;;
        1) return 0 ;;
        *)
            printf '%s\n' "$output" >&2
            gate_fail "$description filter errored (rg exit $rc)"
            return "$rc"
            ;;
    esac
}

gate_count_lines() {
    local description="$1" input="$2" output rc
    [[ -n "$input" ]] || { printf '0'; return 0; }
    if output="$(printf '%s\n' "$input" | wc -l)"; then rc=0; else rc=$?; fi
    if [[ "$rc" == 0 ]]; then
        printf '%s' "${output//[[:space:]]/}"
    else
        printf '%s\n' "$output" >&2
        gate_fail "$description count errored (wc exit $rc)"
        return "$rc"
    fi
}

gate_toml_dependencies() {
    local manifest="$1" mode="${2:-rack}" extracted output rc awk_program
    if [[ "$mode" == graph ]]; then
        awk_program='
            /^\[dependencies\]$/ { in_dependencies = 1; next }
            /^\[/ { in_dependencies = 0 }
            in_dependencies && /^[a-zA-Z0-9_-]+[.]workspace/ { print $1 }
        '
    elif [[ "$mode" == plain || "$mode" == plain-target ]]; then
        awk_program='
            /^\[dependencies\]$/ || ("'"$mode"'" == "plain-target" && /^\[target[.].*[.]dependencies\]$/) { in_dependencies = 1; next }
            /^\[/ { in_dependencies = 0 }
            in_dependencies && /^[A-Za-z0-9_-]+(\.workspace)?[[:space:]]*=/ {
                value = $0; sub(/[[:space:]]*=.*$/, "", value); sub(/\.workspace$/, "", value); print value
            }
        '
    else
        awk_program='
            /^\[dependencies\]$/ { in_dependencies = 1; next }
            /^\[/ { in_dependencies = 0 }
            in_dependencies && /^[A-Za-z0-9_-]+(\.workspace)?[[:space:]]*=/ {
                value = $1; sub(/\.workspace$/, "", value); print value
            }
        '
    fi
    if extracted="$(awk '
        '"$awk_program"'
    ' "$manifest")"; then
        :
    else
        rc=$?
        gate_fail "dependency extraction failed for $manifest (awk status $rc)"
        return "$rc"
    fi

    if output="$(printf '%s\n' "$extracted" | sort)"; then
        printf '%s\n' "$output"
        return 0
    else
        rc=$?
        gate_fail "dependency extraction failed for $manifest (sort status $rc)"
        return "$rc"
    fi
}
