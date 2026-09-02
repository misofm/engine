# Finalize cwd-independent stems receipts and enginectl machine-I/O documentation

## Objective

Make file-output receipts from `enginectl session build --stems` durable when input or output
arguments are relative, while preserving the caller's original spelling and every established
request-mode byte. Complete the published help and README contract for raw-TOML versus JSON-receipt
stdout, receipt fields and path semantics, the physical in-leaf output prohibition, typed
collection refusal, and `groups` as child-directory names rather than verified leaf directories.

## Dogfood finding

Issue #333's real four-song dogfood passed, but fresh Sol-high review found three low-cost agent-DX
gaps. A relative `input.path` becomes ambiguous after the receipt moves to another working
directory; neither command-local help nor the README exposes the complete stems receipt contract
and output-location safety rule; and the README calls collection `groups` leaf names even though
discovery reports immediate child directories without inspecting them. These are publication
contract gaps, not reasons to reopen the accepted FLAC importer.

## Decision

Extend stems-mode file receipts additively:

```json
{
  "schemaVersion": 1,
  "command": "session.build",
  "output": {
    "path": "../sessions/song.session.toml",
    "resolvedPath": "/work/sessions/song.session.toml",
    "bytes": 7642,
    "sha256": "..."
  },
  "input": {
    "kind": "stems",
    "path": "./song-stems",
    "resolvedPath": "/work/project/song-stems"
  },
  "session": {
    "id": "song-stems",
    "revision": 0,
    "sampleRateHz": 44100,
    "quantumFrames": 128,
    "sources": 8,
    "tracks": 8
  },
  "stems": []
}
```

`path` remains the exact argument string. `resolvedPath` is the host-native, lexically normalized
absolute path produced by Node `path.resolve()` against the invocation working directory. Additive
fields preserve argument provenance, make input and output symmetric, keep request-mode receipts
byte-for-byte unchanged, and prevent consumers from guessing whether `path` is caller spelling or
an effective target. Only stems-mode file receipts gain these fields. Request-mode receipts retain
their exact existing bytes, and `--output -` still emits only canonical TOML.

`resolvedPath` is not a physical or cross-machine canonical identity. Do not call `realpath`,
lowercase paths, normalize Unicode, emit `file:` URLs, or convert separators. Preserve symlink and
case-alias spelling while removing working-directory dependence and lexical `.`/`..` components.
Windows drive and UNC paths remain host-native JSON strings. Node arguments are already Unicode
strings; the receipt makes no arbitrary Unix-pathname-byte claim. Filesystem `(device, inode)`
identity remains a transient safety mechanism for the in-leaf prohibition and is not persisted.

## Help and documentation contract

`enginectl session build --help` must state, without loading assets:

- exactly one of `--request` and `--stems`;
- stems input and default semantics;
- `--output -` writes raw canonical TOML and successful stderr is empty;
- file output publishes atomically before stdout emits one compact JSON receipt plus LF;
- existing destinations require `--overwrite`;
- stems output cannot physically reside inside the stems directory, including through a symlink or
  case alias;
- a collection refuses with `stems.collection` and sorted child-directory names;
- failures use one JSON stderr document and the documented exit classes; and
- the command is non-interactive and offline.

The README must show the exact stems receipt shape and define the added fields, their absence in
request/raw-output modes, `stems` ordering, and path portability limits. Replace "sorted leaf
names" with "sorted child-directory names" and do not claim those children are valid leaves.

## Scope

- this issue specification;
- `sdk/src/enginectl.ts`;
- `sdk/README.md`;
- focused black-box additions in `sdk/test/enginectl-cli.mjs`; and
- extracted-package smoke only if required to prove the emitted executable retains the contract.

No session grammar, builder, decoder, engine asset, DSP, package export, dependency, workflow, or
runtime behavior outside receipt construction and help is changed.

## Objective gates

1. From a nontrivial working directory, relative stems input and relative file output succeed with
   each `path` equal to its exact argument and each `resolvedPath` equal to
   `path.resolve(cwd, argument)`.
2. A consumer under a different working directory can reopen the output and every mapped stem using
   only the two resolved paths and `stems[].filename`.
3. Absolute arguments remain absolute and lexically normalize consistently.
4. A symlink-spelled stems directory reports its absolute lexical alias, not the physical target;
   the existing physical in-leaf output prohibition still follows aliases and refuses before
   decoder loading.
5. Unicode, spaces, newlines, leading dashes, and instruction-looking path text remain one valid
   JSON record with no raw control injection.
6. Request-mode file receipts, raw TOML, errors, statuses, and stderr remain byte-for-byte
   compatible with the pre-issue executable.
7. Stems `--output -` remains exactly canonical TOML with final LF and empty successful stderr; no
   receipt fields share stdout.
8. File publication still precedes receipt reporting. Post-publication receipt-channel failure
   reports `effect: "applied"`; every earlier failure reports `not_applied`.
9. Help works with stdin closed and both Wasm assets unavailable and covers framing, exits,
   defaults, overwrite, in-leaf refusal, and collection recovery.
10. README documents the exact stems receipt schema and says child-directory names without claiming
    reported groups are valid leaves.
11. Existing 19 black-box CLI tests, package/tarball smoke, strict types, generated/deletion checks,
    and SDK routing remain green.
12. No new filesystem probe, subprocess, network call, dependency, or asset initialization is
    introduced.

## Mutation requirements

- Emit a raw relative argument in place of either `resolvedPath`.
- Replace lexical `resolve()` with `realpath()` so the symlink-alias expectation fails.
- Add either resolved field to request-mode receipts.
- Emit a receipt with `--output -` or before file publication.
- Weaken the physical in-leaf check or stop JSON-escaping hostile path text.
- Remove stdout-mode, in-leaf, or collection semantics from command-local help.

Each mutation must turn its named gate red, and production must be restored byte-for-byte.
README prose receives adversarial semantic review rather than a prose digest gate.

## Proportional qualification

- Extend the built-executable helper to accept an explicit working directory.
- Build from `./Song Stems` to `../sessions/song.session.toml`; assert the full parsed receipt, raw
  and resolved fields, output digest, and reopening from another working directory.
- Cover hostile path text through argument arrays and a directory symlink where supported.
- Retain the decoder-unavailable in-leaf source/new-output/symlink-alias cases.
- Assert the exact request-mode receipt line against the pre-issue contract and retain raw-output,
  error, and publication-order cases.
- Assert semantic command-local help coverage with both assets unavailable.
- Run the focused built CLI suite, extracted package smoke, strict TypeScript,
  generated/deletion checks, and SDK routing/scope classification.

No benchmark run is warranted: the implementation adds two `path.resolve()` string operations and
documentation only. Any filesystem canonicalization or measured workload change is out of scope.

## Non-goals

Batch or recursive collection building, caching, daemons, resolver/pump/playback behavior, writing
receipt files for the caller, TOML inspection or patching, request scaffolding, physical-path
canonicalization, new flags, manifests, configuration, credentials, network, and telemetry remain
out of scope.

## PASS / HOLD

PASS only when every gate passes at one exact checkpoint, request-mode bytes are unchanged, a
receipt remains consumable after working-directory changes, help and README fully describe the
public protocol, and fresh Sol-high review finds no scope expansion.

HOLD for any relative effective target, physical `realpath` semantics, request-mode drift, altered
stream framing or publication order, weakened in-leaf protection, help that requires an issue spec
to understand machine framing, calling reported groups verified leaves, or introduction of batch,
cache, resolver, playback, or receipt-file behavior.

## Rollout

1. Create matching local and GitHub issue #335 before implementation.
2. Commit this Sol-high-approved brief as a clean checkpoint.
3. Implement one bounded attempt with Sol medium and commit only after focused gates pass.
4. Obtain fresh Sol-high adversarial PASS; permit at most two bounded corrections under the
   repository's three-attempt rule.
5. Deliver in one CI-conscious SDK batch and synchronize the local and remote evidence record.

## Evidence

Sol-high briefing on 2026-09-03 inspected the merged #333 executable, real four-song dogfood,
package help, README, receipt construction, importer path model, and existing black-box tests. It
ruled that additive lexical `resolvedPath` fields plus complete public machine-I/O documentation
are the smallest closable correction. It explicitly excluded collection batching, caches,
resolver/playback work, receipt-file output, physical canonicalization, and benchmarking because
the observed importer already meets its correctness and performance contract.

Attempt 1 implementation evidence on 2026-09-03, based exactly on checkpoint `57e43da3`, adds
`resolvedPath: path.resolve(argument)` only to stems-mode file receipts while leaving the existing
request receipt object and raw-output branch unchanged. The built-executable suite now supplies an
explicit child cwd and discriminates raw versus resolved argument spelling, reopening the output
and mapped stems from another cwd, absolute lexical normalization, preservation of a symlink alias,
hostile Unicode/space/newline/leading-dash/instruction-looking JSON framing, exact request-mode
receipt bytes, stems raw TOML, physical in-leaf protection, help without either Wasm asset, and
publication existing at the instant the receipt write begins. The README publishes the complete
stems receipt shape and corrects collection recovery to sorted child-directory names without
claiming that those children are valid leaves.

Focused qualification results:

- `ENGINECTL=sdk/dist/enginectl.js node --test sdk/test/enginectl-cli.mjs`: PASS, 21/21 after the
  final test hardening.
- `bash scripts/sdk-package.sh check /private/tmp/engine333-artifacts
  /private/tmp/engine-flac-decoder-33666706481`: PASS, 21/21 built-executable tests and extracted
  69-file package/tarball smoke using the existing qualified artifacts.
- `bash scripts/check-sdk-types.sh`: PASS (strict types and shipped-host declaration pin).
- `python3 -B scripts/check-sdk-deletions.py` and the `--self-test` form: PASS; 45 source files
  clean and all 36 mutations caught.
- `python3 -B scripts/check-ci-path-routing.py`, `python3 -B scripts/test-ci-path-routing.py`, and
  `python3 -B scripts/ci-path-router.py --event pull_request` over the issue/spec/SDK paths: PASS;
  the route is `sdk`.
- Generated/assets checks run inside `sdk-package.sh check`: PASS.

A default `npm run build` was also attempted first and stopped before SDK staging because a fresh
local AudioWorklet build produced SHA-256 `1fe4b9ce...` rather than the gate-pinned
`6ddf154d...`. This is the known qualified-artifact reproducibility boundary, not a source/test
failure; the proportional package gate above therefore used the already-qualified artifact
directories as the script explicitly supports. No benchmark was run, and no dependency, flag,
runtime filesystem probe, subprocess, network path, batch/cache/resolver/playback behavior, or
receipt-file behavior was added.

### Attempt 1 — Sol-high adversarial HOLD

Fresh Sol-high review held exact checkpoint `6335ccbf` on one implementation-quality defect. The
observable contract passed, but stems mode first constructed the complete request receipt and its
TOML SHA-256, then reconstructed every common field and recomputed the same SHA-256 in the stems
branch. This contradicts the bounded-performance claim that only two lexical path resolutions were
added and creates an avoidable drift seam across `schemaVersion`, `command`, output path, byte
count, and digest. Attempt 2 must extend the already-computed common receipt/digest exactly once,
preserve request-mode bytes and property order, and take `input.resolvedPath` from the importer's
already-established `stemsBuild.directory` rather than resolving the argument again.

All named adversarial mutations were independently discriminating before HOLD: raw-relative input
and output resolved paths, physical `realpath`, request-mode field leakage, raw-output receipt
contamination, receipt-before-publication, weakened physical in-leaf protection, hostile JSON
framing, and removal of required help semantics each turned a named gate red. The reviewer also
built parent `57e43da3` and candidate `6335ccbf` in isolated copies: request raw TOML was
byte-identical at 1,243 bytes, and request file receipts were byte-identical at 253 bytes with no
`resolvedPath`. Candidate package/tarball qualification passed 21/21 executable tests; strict
types, generated/deletion gates and all 36 deletion mutations, routing gates, SDK classification,
and extracted 69-file package smoke were green. Attempt 1 remains **HOLD** pending the single-pass
common-receipt correction and fresh review.
