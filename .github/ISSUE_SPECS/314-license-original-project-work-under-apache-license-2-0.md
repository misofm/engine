# Issue 314: License original project work under Apache License 2.0

**Status: SOL BRIEF APPROVED / IMPLEMENTATION AUTHORIZED.**

GitHub issue: https://github.com/misofm/engine/issues/314

## Objective

Adopt the Apache License, Version 2.0 as the repository-wide default for all original project code,
documentation, SDKs, hosts, tools, sidecars, scripts, fixtures, and generated artifacts that the
project has authority to license. Preserve and clearly identify every third-party work under its
existing license.

## Smallest closable slice

- Add the canonical Apache-2.0 license text at the repository root.
- Add a concise project `NOTICE` and third-party attribution inventory.
- Declare Apache-2.0 consistently in Rust and npm package metadata.
- Add a bounded workspace policy gate that rejects missing or conflicting first-party package
  license metadata and verifies the required license/notice artifacts.
- Document third-party exceptions, including the vendored libm material in `crates/math`.

## Non-goals

- Relicensing third-party works.
- Changing runtime behavior, ABI, DSP, dependencies, or distribution architecture.
- Adding a CLA or changing trademark policy.
- Claiming ownership based only on Git author identity.

## Decision record

Apache-2.0 is selected because the engine is deliberately designed for permissive native, cloud,
mobile, and browser embedding. The explicit patent grant and permissive redistribution terms match
that product contract. Copyleft reciprocity is not an acceptance goal. Third-party notices remain
authoritative for their files.

The project license is a default for original work, not a claim that separately licensed vendored
material has been relicensed. Distribution metadata must keep that boundary visible.

## Objective gates

1. The root `LICENSE` is byte-identical to the canonical Apache License 2.0 text.
2. Every first-party Cargo package reports `license = "Apache-2.0"` through Cargo metadata,
   including the standalone fuzz package.
3. Every first-party npm package manifest declares `Apache-2.0`.
4. `NOTICE` and the third-party inventory identify retained vendored licensing without asserting
   that third-party code was relicensed.
5. The focused workspace-policy tests pass and `cargo metadata --no-deps` succeeds.
6. A path audit finds no contradictory project-level license declaration.

## Realtime and product impact

None. This issue changes legal and package metadata only and must not alter compiled behavior or
artifacts except metadata-bearing package outputs.

## Evidence record

- 2026-09-01: GitHub issue 314 created before implementation. Its number and title match this local
  spec and filename.
- 2026-09-01: Sol approved the smallest closable slice and objective gates above. Attempt 1 is
  authorized.
