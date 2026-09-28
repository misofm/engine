#!/usr/bin/env bash
# shellcheck shell=bash
#
# The product crates (#1017): every workspace crate in `capi`'s normal-dependency closure, which is
# what an iOS or Android app links through the C ABI. Derived rather than written out, so a crate
# that joins the closure is checked and tested on AArch64 from the day it joins.
#
# Sourced by scripts/check-cross-targets.sh (the AArch64 compile rows) and
# scripts/run-aarch64-tests.sh (the AArch64 test legs). Prints one crate name per line, sorted,
# and fails if the closure lost a crate it cannot lose.

product_crates() {
    local root=$1 crates crate
    crates="$(
        cargo tree --quiet --locked --manifest-path "$root/Cargo.toml" --target all -e normal \
            --prefix none --format '{p}' -p capi |
            grep -F "($root/" | awk '{print $1}' | sort -u
    )" || return 1
    for crate in capi lane engine host-core graph-compiler parametric-eq; do
        grep -qx "$crate" <<<"$crates" || {
            printf "product-crates: capi's workspace closure lost %s\n" "$crate" >&2
            return 1
        }
    done
    (($(wc -l <<<"$crates") >= 20)) || {
        printf "product-crates: capi's workspace closure is only %s crates\n" \
            "$(wc -l <<<"$crates")" >&2
        return 1
    }
    printf '%s\n' "$crates"
}
