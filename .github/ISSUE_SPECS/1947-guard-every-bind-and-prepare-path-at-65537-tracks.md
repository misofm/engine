# Guard every bind and prepare path at 65,537 tracks with one kitchen-sink nightly session

Filed by root as #967's R1 successor (backlog sprint, 2026-10-24).

## Finding

#967 linearised `scatter_redirects` and reshaped the nightly 65,537-track test (a compiled routes+console phase and a hand-built banks-plus-route-per-track phase). Other per-track structures still have no guard at that scale: sources, observers and meters, sidechains, sends and submixes, and stored automation. A quadratic scan in any of them would pass every committed test.

## Smallest closable slice

One nightly (ignored, release) session at 65,537 tracks that carries sources, a meter per strip, a sidechained insert on some tracks, sends into submixes, and stored automation on faders, with a bound chosen as in #967 (several times the measured linear time, well under a quadratic time measured once on the pre-fix code or estimated by doubling). Assert the exact counts of each structure. Split into two nightly sessions if one exceeds 60 s (#967 R2). Fix any quadratic path the session exposes in its own follow-up.

## Paths

`crates/graph-compiler/tests/scale.rs` or `crates/host-core/tests` (whichever owns prepare), `.github/workflows/nightly.yml` only if a new test name must be listed, this spec. After #967 merges.
