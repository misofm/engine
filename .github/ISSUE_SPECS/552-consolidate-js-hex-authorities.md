# Consolidate lowercase byte-hex encoding at SDK and browser package boundaries

One-line summary: Finish the non-Rust residual of audit #349 CP-20 by replacing three independent SDK and browser lowercase byte-hex encoder bodies with one authority inside each independently shipped module boundary, while preserving hashes, public adapters, browser routes, fixtures, and pins.

## Problem and parent obligation

This is the stateless successor to #543 and audit #349 CP-20. On reviewed source `55eb15e0045464e9fcd5c094e08f54b105e8edb3` against base `86d5b4bd97999a376123f3be3d6c04d27b8733e1`, Sol XHIGH found three live whole-byte-stream, unprefixed lowercase encoders omitted by #543's Rust-only census:

1. `sdk/src/core/asset.ts:159`: exported `sha256Hex` formats the finalized Web Crypto digest.
2. `hosts/host-web/qualification/qualification.js:69`: local `bytesToHex` formats the browser PCM digest.
3. `hosts/host-web/web/stem-store/incremental-sha256.js:110`: public `IncrementalSha256.digestHex` formats the finalized incremental digest.

Each processes every byte with `toString(16).padStart(2, "0")` and concatenates the results. Each is residual CP-20 scope. Preserve exact lowercase values, leading zeroes, hashing ownership and placement, public APIs, fixtures, and pins.

Issue #543 delivers the independently reviewed Rust authority slice. CP-20 remains partial until this complete non-Rust residual is delivered.

## Frozen package-boundary decision

Use exactly two non-Rust authorities because these consumers cross two independently shipped or served module closures.

### SDK authority

Add internal `sdk/src/core/hex.ts` with:

```ts
export function hexLower(bytes: Uint8Array): string
```

`sdk/src/core/asset.ts` imports it. Exported `sha256Hex` retains Web Crypto and all hashing ownership, but delegates only the formatting of the finalized digest bytes. Preserve its export, parameter and return types, error behavior, and the existing `sdk/src/headless/index.ts` export unchanged. Do not expose `hexLower` through the SDK package export map.

The SDK compiler emits only `sdk/src/**/*.ts` under `rootDir: ./src`, and the npm package ships only `dist`. Importing a host-web source would emit a broken external path into the package closure.

### Browser-host authority

Add `hosts/host-web/web/hex-lower.js` with:

```js
export function hexLower(bytes)
```

`hosts/host-web/web/stem-store/incremental-sha256.js` imports `../hex-lower.js`. Retain `IncrementalSha256.digestHex()` and its state/finalization behavior as a thin adapter.

`hosts/host-web/qualification/qualification.js` imports `../web/hex-lower.js`. This resolves to `/web/hex-lower.js` in the browser and to the same filesystem module when imported under Node. Amend `hosts/host-web/qualification/server.mjs` only with an exact allowlisted mapping for `/web/hex-lower.js`; do not expose the `/web/` subtree.

### Why the two authorities are required

Calling the Rust authority directly would require a new allocating C or Wasm ABI, module instantiation, and memory transfer solely to stringify bytes. That would alter the frozen host ABI and add a Rust/Wasm runtime dependency to pure SDK and browser formatting.

A single source shared directly between the SDK and host-web closures is also invalid: TypeScript source is not browser-executable; the SDK build root and npm package closure exclude host-web; and deployed host-web modules must not depend on SDK internal source. Two authorities are the smallest real package-boundary implementation. All three residual independent call-site bodies must disappear. This is a recorded language and packaging boundary decision, not an exclusion of any discovered encoder.

## Implementation and invariants

1. Both helpers return exactly two lowercase ASCII hexadecimal characters per input byte, preserve leading zeroes, return an empty string for empty input, accept non-32-byte inputs, add no prefix or separator, and perform no hashing or I/O.
2. Use a fixed lowercase nibble lookup or an equally bounded direct conversion. Keep the helper names unversioned.
3. Add an independent fixed-literal test for each authority. Each test covers empty input and:

   ```text
   Uint8Array([0x00, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0xff])
   => "000123456789abcdefff"
   ```

   The expected text must be a fixed literal and must not be generated with another formatter.
4. Put the SDK direct literal test in `sdk/test/boot-evals.mjs`, importing the internal helper. Preserve the existing `sha256Hex` test so the public adapter and Web Crypto path remain covered.
5. Put the browser-host direct literal test in `hosts/host-web/tests/stem-store-hash-v1.mjs`, importing `../web/hex-lower.js`. Preserve all existing FIPS SHA-256 vectors and streaming/finalization assertions so `digestHex` remains independently covered.
6. Do not change a digest pin, expected corpus value, fixture, hash algorithm, digest word order, Web Crypto choice, incremental SHA update/finalize behavior, public package export, dependency, lockfile, Rust source, Wasm ABI, or generated artifact.

## Allowed paths

- `.github/ISSUE_SPECS/<number>-consolidate-js-hex-authorities.md`;
- `sdk/src/core/hex.ts` (new);
- `sdk/src/core/asset.ts`;
- `sdk/test/boot-evals.mjs`;
- `hosts/host-web/web/hex-lower.js` (new);
- `hosts/host-web/web/stem-store/incremental-sha256.js`;
- `hosts/host-web/qualification/qualification.js`;
- `hosts/host-web/qualification/server.mjs`, limited to the exact helper route;
- `hosts/host-web/tests/stem-store-hash-v1.mjs`;
- issue evidence artifacts.

`sdk/src/headless/index.ts`, package manifests and locks, generated or `dist` files, artifact pins, Cargo files, and Rust source are read-only.

## Finite gates

Run and record exact argv, cwd, status, pre-command Git identity, relevant source hashes, and raw output for:

1. The direct fixed-literal assertions for both helpers, together with the existing `sha256Hex` and incremental SHA-256 fixed vectors.
2. `node scripts/check-stem-store-v1.mjs`.
3. `bash scripts/check-sdk-types.sh`.
4. `bash scripts/check-sdk-headless.sh <existing-exact-six-file-qualification-artifact-directory>`. This runs `sdk/test/*-evals.mjs`, including the boot evals.
5. `bash scripts/sdk-package.sh check <same-artifact-directory>` to prove `hex.ts` emits into `dist` and survives the npm tarball closure. Use already installed pinned dependencies; do not change the lockfile.
6. From `hosts/host-web/qualification`, `npm run session-identities`, proving Node module resolution through `qualification.js`.
7. One existing browser qualification leg against the same artifacts, for example `npm run qualify -- --artifacts <directory> --browser chromium --check-matrix --self-test-mutations`. This proves the exact `/web/hex-lower.js` route and browser import. Required remote `qualification` must supply the existing Chromium, Firefox, and WebKit matrix before merge.
8. `bash scripts/check-workspace-policy.sh`.
9. Full committed original-base and current-base `git diff --check`, plus clean status.

Final static completion gate: run a cross-language semantic census over `crates`, `hosts`, `sdk`, `tools`, and `sidecars`, including `*.rs`, `*.ts`, `*.js`, and `*.mjs`, and excluding `node_modules`, `dist`, and `target`. Search for `toString(16)` with `padStart(2)`, lowercase nibble tables, Rust `:02x`, and `char::from_digit`; inspect every hit. Passing means the three residual call sites delegate, the only raw unprefixed byte encoders are `engine::hex_lower`, SDK `hexLower`, and host-web `hexLower`, and every remaining hit is a concrete parser, decorated diagnostic/repin, fixed-width integer, uppercase, or word-format exclusion already recognized by #543.

Any existing pin or canonical value change is a failure to investigate, not authorization to update expected output.

## Non-goals

- No generic formatting framework or new crate/npm dependency.
- No public SDK export for the internal helper.
- No C ABI, Wasm ABI, module-instantiation, or cross-memory formatting path.
- No broad qualification-server route.
- No parser, decorated diagnostic/repin, fixed-width integer, uppercase, or word-format rewrite.
- No hash replacement, repin, fixture corpus, benchmark, DSP/audio behavior, or generated artifact change.

## Overlap, execution, and closure

Delivered #546 touches `scripts/check-stem-store-v1.mjs` and the operator path-validation script. Rebase onto delivered #546 before implementation and treat those gate paths as read-only. This issue does not overlap #542's protocol test extraction.

Luna HIGH implements attempt 1 under Sol HIGH coordination. Sol XHIGH performs adversarial review under the standing three-attempt rule. Root owns numbering, the matching GitHub issue, worktree, exact-path checkpoints, Git/GitHub synchronization, integration, and delivery.

Close this issue only after Sol review PASS, required CI PASS, merge, and GitHub synchronization. Then update audit #349 CP-20 from `PARTIAL` to delivered, citing both #543's Rust slice and this issue. Issue #543 alone never earns CP-20 closure.


## Numbered activation boundary

This successor is #552, occupying the slot freed by delivered #547. Source implementation waits until #546 is delivered and integrated into this branch. Root owns Git/GitHub; Sol high coordinates Luna high and actual Sol xhigh verifies. The exact existing six-file artifact directory is /tmp/issue537-delivery/artifact; inspect its identity against current pins before gates, without repinning. CP20 remains partial until #543 and this successor are delivered.

## Implementation authorized

#546/PR551 is delivered at a3b4ed763c47658e10fc111e2cfcbd141c77f064 and integrated here. The frozen implementation may now proceed under Sol high coordination using actual Luna high, one focused-green tranche then pause for root checkpoint. Existing operator/path gate is read-only. No cross-language obligation is removed.
