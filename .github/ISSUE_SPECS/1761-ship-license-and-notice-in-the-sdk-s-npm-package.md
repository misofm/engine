# Ship LICENSE and NOTICE in the SDK's npm package

Found by the #1760 verifier (2026-10-23).

## Problem

`sdk/package.json` publishes `"files": ["dist"]`, and `sdk/` has no LICENSE or NOTICE of its own. npm adds a LICENSE file to the tarball only from the package directory, so a packed `@misofm/engine` carries no license text. Under FSL-1.1-ALv2 (#1760) a redistributed copy must include the terms or a link to them, and the Competing Use limits travel with that text.

## Smallest closable slice

- Put the project's LICENSE and NOTICE into the packed SDK tarball (for example a copy step in the SDK build into `sdk/`, or `files` entries that the build fills), with one source of truth: the repository-root files.
- Gate it where the SDK is already packed or checked: the packed file list contains `LICENSE` and `NOTICE`, and they equal the root files.

## Authorized paths

- `sdk/package.json`, the SDK build script it names, the SDK's existing pack or qualification check
- this spec

## Objective gates

1. `npm pack --dry-run` in `sdk/` lists `LICENSE` and `NOTICE`.
2. The packed files equal the repository-root files byte for byte.
3. The SDK's existing build and tests pass.

## Test value

The pack check turns red when a later `files` or build change drops the license text from the published package, which no current check sees.

## Dependencies

#1760 (the FSL license text).
