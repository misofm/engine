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
