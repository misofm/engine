#!/usr/bin/env bash
# The workspace-wide release test build, with `panic = "abort"` overridden back to `unwind`.
#
# Why the override exists (do not remove it without reading docs/REALTIME_DEPENDENCY_POLICY.md,
# "Panic behaviour by profile" -- master plan #83 D12):
#
# D12 sets `panic = "abort"` on `[profile.release]` so a benchmark and a shipped artifact measure
# the same codegen. Cargo ignores that setting when it builds a *test* harness, which is what makes
# `cargo test --release` work at all -- but in a `--workspace --all-targets` invocation it means
# every crate is built in BOTH panic variants in one `cargo` run: the abort variant for the shipped
# lib/bin units, the unwind variant for the test harnesses and their dependencies.
#
# Two variants are normally fine, because Cargo hashes the variant into each output filename. They
# are not fine for a lib unit that also carries a `cdylib` or `staticlib` crate-type: those emit
# UN-hashed filenames (`libfoo.so`, `libfoo.a`), so the two panic variants write to the same paths
# and the second clobbers the first. Three packages in this workspace are in that shape:
#
#   crates/miso-engine-capi            rlib + staticlib + cdylib
#   crates/miso-engine-effect-package  rlib + cdylib
#   hosts/miso-engine-host-web         rlib + cdylib
#
# A downstream unit then links whichever variant happened to land last and gets a metadata
# mismatch. The error face is nondeterministic (it depends on build scheduling) and usually reads
# as a bogus `error[E0463]: can't find crate for ...`; the failure itself is deterministic.
#
# Forcing ONE panic variant for the whole invocation removes the collision: every unit in this run
# is `unwind`, so nothing is built twice and nothing is clobbered.
#
# What this does NOT change:
#
# * Shipped artifacts. Nothing here builds one. `scripts/build-web-audioworklet.sh` and every
#   per-package release invocation still get D12's `panic = "abort"` exactly as before.
# * Per-package release invocations (`cargo test --locked --release -p <pkg>`, the many gate legs
#   in .github/workflows/ci.yml). They select one package's targets, never two panic variants of a
#   clobbering lib unit in the same run, so they are unaffected and are deliberately left alone.
#
# The structural alternative -- move `panic = "abort"` off `[profile.release]` onto a separate
# `dist` profile -- is NOT taken here, because it would change D12's "a benchmark measures the
# shipped codegen" intent (`[profile.bench]` inherits `release`). That is an owner decision and is
# recorded as deferred in docs/REALTIME_DEPENDENCY_POLICY.md.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"

CARGO_PROFILE_RELEASE_PANIC=unwind \
    cargo test --locked --release --workspace --all-targets "$@"
