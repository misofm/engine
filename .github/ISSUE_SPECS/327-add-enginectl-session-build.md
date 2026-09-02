# Add `enginectl session build`

## Objective

Ship the smallest publishable `enginectl` vertical slice: accept one bounded, versioned JSON build
request, construct it exclusively through the existing Session V1 SDK builder, validate the
resulting canonical TOML with the package's embedded Wasm engine, and publish the accepted bytes
atomically.

This is a local authoring operation, not live engine control. Nested session structure belongs in
the request document, not a flag mini-language.

## Product contract

`@misofm/engine` exposes the executable as `enginectl` with a package bin target at
`./dist/enginectl.js`. V1 has one substantive command:

```text
enginectl session build --request PATH|- --output PATH|- [--overwrite]
```

The executable also supports `enginectl --help`, `enginectl --version`, `enginectl session --help`,
and `enginectl session build --help` without loading Wasm. No invocation prompts, pages, opens a
browser/editor, consults configuration, reads credentials, loads plugins, emits telemetry, or uses
the network. Node 20+ is the executable runtime.

## Request contract

`--request -` reads stdin. Otherwise its value is a UTF-8 path resolved from the process working
directory. Input is capped at 4 MiB and decoded as fatal UTF-8 before `JSON.parse`.

The JSON document has this V1 shape:

```json
{
  "schemaVersion": 1,
  "session": {
    "id": "vocal-mix",
    "sampleRateHz": 48000,
    "revision": 0,
    "quantumFrames": 128
  },
  "sources": [
    {
      "id": "stem",
      "spec": {
        "channels": 2,
        "bitDepth": "32f",
        "frames": 48000,
        "content": "sha256:0000000000000000000000000000000000000000000000000000000000000000"
      }
    }
  ],
  "tracks": [
    {
      "id": "vocal",
      "spec": {
        "source": "stem",
        "dynamic": [
          {
            "effectId": "miso.compressor",
            "parameters": { "threshold": -18, "ratio": 4 },
            "options": { "slotId": "compressor" }
          }
        ]
      }
    }
  ],
  "outputs": ["main"],
  "routes": [
    {
      "id": "main",
      "source": { "kind": "track", "trackId": "vocal", "tap": "post_matrix" },
      "destination": { "kind": "output_input", "outputId": "main" }
    }
  ]
}
```

- `schemaVersion` and `session` are required.
- `sources`, `tracks`, `submixes`, `outputs`, `routes`, and `automation` are optional arrays that
  default to empty.
- Sources are `{ id, spec: SourceSpec }` and tracks are `{ id, spec }`.
- Each rack contains `{ effectId, parameters?, options? }`, translated through the existing
  `effect()` function.
- Submixes and outputs are arrays of IDs. Routes use the existing camel-case `RouteSpec` shape.
- Automation uses the existing shape except `startSample` and `endSample` are required canonical
  unsigned decimal strings, converted to `bigint`; JSON numbers are refused.
- Unknown keys are refused at each request-owned structural level rather than discarded.
- Builder verbs run in fixed dependency order: session, sources, submixes, outputs, tracks, routes,
  automation. Entity declaration order does not change canonical entity ordering; rack order
  remains signal order.
- SDK defaults remain authoritative. The CLI does not restate effect metadata, parameter domains,
  builtin defaults, canonical TOML formatting, or engine validation rules.
- Duplicate JSON member names are outside V1; the CLI does not add a second JSON parser solely to
  diagnose them.

## Build transaction

One bounded build transaction:

1. Parses argv and reads the complete bounded request.
2. Strictly decodes the request and constructs a `SessionBuilder`.
3. Produces bytes with `builder.toToml()`.
4. Calls the existing Promise-based `validate()` with bundled, digest-verified Wasm.
5. Publishes output only after validation returns `ok: true`.

A JSON, builder, or engine refusal emits no TOML and creates no destination.

## Output contract

For `--output -`, stdout is exactly canonical UTF-8 TOML with its existing final LF. No receipt or
progress shares stdout. Early pipe closure is caller cancellation and emits no stack trace.

For a filesystem output:

- The parent directory must exist.
- Without `--overwrite`, an existing file, directory, or symlink is refused unchanged.
- A uniquely named same-directory temporary file is fully written and closed before an atomic
  no-clobber publication.
- With `--overwrite`, the complete same-directory temporary file atomically replaces the directory
  entry.
- Temporary files are removed on every pre-publication failure.
- Success stdout is one compact JSON document plus LF:

```json
{"schemaVersion":1,"command":"session.build","output":{"path":"session.toml","bytes":1234,"sha256":"..."}}
```

The path is the caller's exact argument. Bytes and SHA-256 describe the published TOML. A receipt
is never emitted before publication. Stderr is empty on success.

## Error and exit contract

Failure before output leaves stdout empty. Stderr contains one compact JSON document plus LF with
`schemaVersion: 1`, a stable `error.code`, human `message`, `effect`, and applicable engine phase,
result, and ordered diagnostics.

- `0`: success, help, or version.
- `2`: invalid command, flag, or flag combination.
- `3`: request read, size, UTF-8, JSON, shape, or SDK-builder refusal.
- `4`: embedded-engine validation refusal.
- `5`: output preparation or publication refusal.
- `70`: unexpected internal or packaged-asset failure.

Ordinary pre-publication failures report `effect: "not_applied"`. Unknown flags, duplicate flags,
missing flag values, positional operands, and `--overwrite` with `--output -` are refused rather
than guessed. Messages may improve; codes and channel behavior are contract.

## Scope

- `sdk/package.json`: add the zero-dependency `enginectl` bin mapping.
- `sdk/src/enginectl.ts`: shebang entry, dispatch, structured errors, bounded I/O, embedded
  validation, receipt, and atomic publication.
- `sdk/src/cli/session-request.ts`: V1 request types, strict structural decoding, bigint-string
  conversion, and translation through `session()` and `effect()`.
- `sdk/test/enginectl-cli.mjs`: black-box built-executable tests through pipes and real paths.
- `scripts/sdk-package.sh` and `sdk/test/package-tarball-smoke.mjs`: qualify the executable in the
  staged tree and extracted tarball.
- `sdk/README.md`: document request, output modes, overwrite behavior, exits, and the
  non-interactive guarantee.

CLI-only Node imports remain outside SDK core. Established Promise APIs remain unchanged.

## Objective gates

1. A rich request covering sources, tracks, all racks, an effect, submix/output routing, sidechain,
   matrix/pan, and automation produces bytes identical to an equivalent direct builder and is
   accepted by embedded Wasm.
2. Logically identical requests with permuted entity arrays produce identical TOML and SHA-256;
   rack effect order remains unchanged.
3. Stdin plus stdout emits only canonical TOML. File output emits only the JSON receipt whose byte
   count and digest match the published file.
4. Malformed UTF-8/JSON, oversized input, unknown keys, numeric automation samples, invalid
   effects/parameters, and SDK semantic errors return `3`, structured stderr, empty stdout, and no
   destination.
5. A builder-accepted request refused by the engine returns `4` with unchanged engine diagnostics
   and no destination.
6. Existing output is unchanged without `--overwrite`; overwrite replaces it with complete
   accepted bytes. No failure leaves a partial destination or temporary file.
7. A receipt is observed only after the destination can be opened and its digest verified. This
   answers the acked-batch question: validation and publication precede acknowledgement.
8. Unknown, duplicate, and conflicting flags return `2`; output failures return `5`; machine
   failures are JSON without terminal control sequences.
9. Help and version work with stdin closed and Wasm unavailable. Early-closing stdout produces no
   traceback.
10. The tarball retains the existing exports, adds the `enginectl` bin, has zero runtime/peer
    dependencies, and builds a session using only embedded artifacts.
11. Existing SDK headless, strict-type, generated, deletion, package, browser, and release gates
    remain green.

## Explicit non-goals

- Separate `session validate`, `session inspect`, `session patch`, catalog commands, or an edit DSL.
- A flat flag for every nested session field.
- Reading existing TOML, TypeScript TOML parsing, or round-trip editing.
- Live runtime control, command queues, rendering, source resolution, or PCM I/O.
- Prompts, profiles, environment configuration, network, plugins, telemetry, update checks, or
  credentials.
- JSON Schema generation, shell completion, daemon mode, Rust-native CLI, registry publication, or
  package-version changes.

Each is an independently useful successor and cannot hold this slice open.

## Decision record

- Structured JSON is the authoring request because a session is hierarchical and repetitive.
  Flags are transport and publication policy only.
- The request has its own V1 discriminator because it is not the normalized Session V1 model.
- The CLI adapts the public SDK builder and validator; canonical TOML and engine semantics stay in
  their existing authorities.
- Build always performs embedded-Wasm validation, so a separate validate command is unnecessary in
  the first slice.
- Machine output is the default. File output returns a receipt; stdout mode returns raw TOML.
- No-clobber is the default. `--overwrite` is explicit authority for one destination and never
  prompts.
- The request is bounded and fully preflighted before mutation, so the operation is atomic rather
  than best effort.
- The direct Promise SDK remains the sole programming model; the executable adds no dependency or
  lifecycle abstraction.

## Principal risks

- Request decoding can drift from SDK types. Keep it a thin mapper, test rich equivalence against
  direct builder construction, and delegate semantic/default/catalog decisions to the SDK.
- Atomic publication varies across operating systems. Keep the temporary file beside the target,
  test no-clobber and replacement, and do not claim crash-durable directory persistence.
- Error paths can leak prose or acknowledge too early. Centralize structured rendering, track
  publication, and exercise the built executable through pipes.
- Help/version can trigger eager asset initialization. Keep engine loading on the build path and
  test with Wasm absent.

## Evidence

### Attempt 1 — Sol medium

The implementation adds the `enginectl` package bin, a strict bounded V1 JSON request decoder,
translation exclusively through `session()` and `effect()`, lazy embedded-Wasm validation,
structured machine errors, raw-TOML stdout, and same-directory atomic file publication. The
package gate now exercises the built executable, and the extracted-tarball smoke test proves the
published CLI boots using only its embedded artifacts.

Focused gates passed on 2026-09-02:

- strict TypeScript compilation;
- `scripts/sdk-package.sh build`, including 6/6 black-box `enginectl` tests;
- `scripts/sdk-package.sh check`, including extracted-tarball CLI boot;
- the established headless SDK suite, 111/111; and
- `git diff --check`.

The no-clobber publication uses a same-directory hard link to obtain an atomic, race-free create
on the qualified macOS filesystem. A filesystem that does not support the link operation refuses
safely with exit 5 and leaves the destination absent; portable fallback behavior remains outside
this issue's V1 contract.

Sol/high adversarial verification remains required before PASS and closure.

### Attempt 1 adversarial verdict — Sol high: HOLD

Independent verification kept the strict TypeScript gate, 6/6 CLI suite, 111/111 headless suite,
fresh extracted-tarball smoke test, executable mode, zero-runtime-dependency package shape, rich
builder equivalence, deterministic entity ordering, receipt digest, publication behavior, and
normal help/version paths green. It found three release blockers:

1. stdout `error` events can escape the callback-only writer as an unhandled Node traceback. An
   early-closed help pipe exits 1, and a reporting failure after file publication can leave the
   file applied without a truthful machine outcome;
2. `validate()` returns packaged-asset failures as `ok: false`, so the CLI currently assigns exit
   4 instead of the required exit 70 when embedded Wasm is absent or invalid; and
3. copying effect parameters into an ordinary object allows the magic `__proto__` key to be
   swallowed before `effect()` can refuse it.

Attempt 2 must correct these paths and add direct regression coverage before another Sol/high
adversarial verdict.

### Attempt 2 — Sol medium

The bounded correction adds listener-aware stdout/stderr writes, clean EPIPE cancellation, and
explicit `applied` mutation state for a non-EPIPE receipt failure after publication. It prepares
the receipt before mutation, classifies `ok: false` asset-phase results as packaged-asset failures,
and copies parameter members through a null-prototype record so `__proto__` reaches the existing
effect validator rather than disappearing.

Focused gates passed on 2026-09-02:

- strict TypeScript compilation;
- the expanded black-box `enginectl` suite, 9/9, including early pipe closure, deterministic
  post-publication stdout failure, missing packaged Wasm, and `__proto__` refusal;
- `scripts/sdk-package.sh check`, including the extracted-tarball smoke test;
- the established headless SDK suite, 111/111; and
- `git diff --check`.

A fresh Sol/high adversarial verdict remains required before PASS and closure.

### Attempt 2 adversarial verdict — Sol high: PASS

Sol/high independently reviewed checkpoint `6242d7f6589ad9dd8b2e07a51ca90bb8fb5118b3`
and reported no findings. Verification included:

- strict TypeScript, the 9/9 CLI package/tarball suite, and the 111/111 headless SDK suite;
- real early-closed stdout, callback-plus-error listener races, post-publication `applied`
  reporting, and unavailable-stderr behavior without recursion or tracebacks;
- missing and byte-corrupt Wasm as exit 70 packaged-asset failures, while a graph cycle remained
  exit 4 with ordered engine diagnostics;
- `__proto__` parameter refusal and nineteen nested unknown-key probes;
- preservation of existing directory and symlink targets, explicit overwrite replacing only the
  symlink entry, and absence of leaked temporary files; and
- direct execution of the extracted `enginectl` shebang, executable mode, unchanged exports, zero
  runtime/peer dependencies, and size/SHA-256 agreement for all six embedded artifacts.

The acked-batch question is answered: builder and embedded-engine acceptance precede publication,
publication precedes its receipt, and a reporting failure after publication truthfully reports
`effect: "applied"` rather than claiming the mutation was dropped.

PASS. The documented hard-link portability refusal and lack of crash-durable directory syncing are
accepted V1 limits; live engine control remains a separate product slice.
