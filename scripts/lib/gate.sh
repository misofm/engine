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

gate_toml_dependencies() {
    local manifest="$1" output rc
    if output="$(awk '
        /^\[dependencies\]$/ { in_dependencies = 1; next }
        /^\[/ { in_dependencies = 0 }
        in_dependencies && /^[A-Za-z0-9_-]+(\.workspace)?[[:space:]]*=/ {
            value = $1; sub(/\.workspace$/, "", value); print value
        }
    ' "$manifest" | sort)"; then
        printf '%s\n' "$output"
        return 0
    else
        rc=$?
        gate_fail "dependency extraction failed for $manifest (status $rc)"
        return "$rc"
    fi
}
