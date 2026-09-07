# 543: Share one lowercase byte-hex encoding authority across Rust packages

One-line summary: Deliver the smallest independently closable Rust slice of audit #349 CP-20 by establishing dependency-free `engine::hex_lower`, delegating every live equivalent Rust encoder to it or recording its separately delivered retirement, and preserving all strings, canonical values, digest pins, public adapters, and hashing ownership.

## Amended scope and completion boundary

Issue #543 is the Rust-package slice of #349 CP-20. It establishes `engine::hex_lower` as the shared lowercase byte-hex authority for Rust packages, delegates every live equivalent Rust encoder, and records the separately delivered #545 retirement of the obsolete `rack_fixture` consumer. Its Rust implementation, dependency edges, pins, APIs, hashing ownership, and full workspace qualification are independently useful and remain accepted evidence.

Issue #543 does not close CP-20. Sol XHIGH attempt 1 correctly found that the original workspace-wide census searched Rust only and omitted three live equivalent encoders:

- `sdk/src/core/asset.ts`: `sha256Hex`;
- `hosts/host-web/qualification/qualification.js`: `bytesToHex`;
- `hosts/host-web/web/stem-store/incremental-sha256.js`: `digestHex`.

These are transferred obligations owned by the separately numbered JS/TS successor. They are not exclusions and are not evidence that the original finding is complete. The original workspace-wide completion language is superseded by this amended Rust boundary.

Issue #543 may close only as the reviewed and delivered Rust slice. Audit #349 CP-20 must remain `PARTIAL` until the numbered JS/TS successor is reviewed, delivered, merged, and synchronized. CP-20 delivery must cite both #543 and that successor.

## Preserved acceptance

- Exactly one dependency-free Rust implementation remains: `engine::hex_lower`.
- All live equivalent Rust encoders delegate to it or were removed by separately reviewed #545.
- Existing adapters, public APIs, SHA-256 ownership, call placement, canonical strings, fixtures, and pins remain unchanged.
- Existing parser, decorated diagnostic/repin, fixed-width integer, uppercase, and word-oriented formats retain the concrete semantic exclusions already recorded by #543.
- The independent fixed-literal engine test and the recorded focused and full Rust workspace gates remain acceptance evidence.
- No claim of cross-language or whole-CP20 completion is made by #543.

