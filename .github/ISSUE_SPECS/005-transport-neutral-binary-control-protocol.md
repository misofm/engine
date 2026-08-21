# 005 Transport-neutral binary control protocol

## Outcome

Define a granular, low-bandwidth, versioned control protocol independent of WebSocket, IPC, or direct embedding.

## Context

Engine V2 is a greenfield Rust, agent-first mixing/mastering engine. Never inspect, copy, benchmark, or inherit V1/legacy work. The realtime plane exclusively owns a preallocated `PreparedRenderPlan`: graph/schedule/capacities are immutable while its DSP state is mutated only through exclusive render ownership. Render performs no allocation/free, locks, file/network I/O, logging, syscalls, structural plan mutation, or data-dependent unbounded work; displaced plans are retired and freed off-thread. There is no compiled track limit. Audio is planar `f32`; dual-mono L/R state and parameters are independent unless an explicit link mode or smoothed 2x2 matrix declares otherwise. Launch-supported session/render rates are exactly 44,100, 48,000, 88,200, and 96,000 Hz; 176,400, 192,000, 352,800, and 384,000 Hz are extended compatibility evidence only. Source/engine mismatches have no implicit SRC. Output is PCM.

This issue is independently implementable only after its exact dependencies are complete. Its change must follow the Sol-approved brief → Terra attempt 1 with evidence → Sol adversarial review workflow; Sol may make at most two further revisions, then the work must be rescoped/rebriefed rather than weakening gates.

## Scope

Specify binary framing and schemas for capability discovery, revisioned atomic session operations, graph/effect configuration, parameter metadata/state query, absolute-sample-time parameter points and step/linear/exponential segments, transport state, meters, counters, diagnostics, typed backpressure and request/response correlation.

## Required public interfaces/contracts

`ProtocolVersion`, `RequestId`, `ExpectedRevision`, `SampleTime`, `CommandFrame`, `EventFrame`, and `ProtocolCodec` are stable; commands are idempotent or explicitly non-idempotent; PCM payloads are forbidden.

## Deliverables

An evidence-backed format decision comparing at least FlatBuffers and one bounded fixed-layout/TLV design; wire specification, codec, golden frames, compatibility rules, event batching/limits, semantic API mapping, queue-saturation policy, and error code registry.

## Explicit non-goals

Choosing a network server, JSON as the realtime command format, PCM streaming, or calling a renderer directly from a decoder.

## Dependencies by exact issue title

- Versioned TOML schema and transactional session compiler

## Hazards/decisions

Transport is an adapter. WebSocket has TCP framing/client masking overhead and is only a remote boundary: https://www.rfc-editor.org/rfc/rfc6455.html.

## Acceptance gates with objective measurements

Before format freeze, the comparison records encoded size, bounded decode behavior, schema evolution, language interoperability and Wasm cost on the same command corpus. Encode/decode golden frames exactly; malformed lengths and unknown mandatory versions/fields reject deterministically; 10,000 parameter events fit bounded batches with no allocations in the realtime consumer; queue saturation returns typed backpressure and never silently loses commands, while declared telemetry coalescing/drop increments counters; fuzz tests prove PCM cannot be encoded.

## Target matrix

Direct native/FFI, local IPC sidecar, browser message/SAB adapter, optional WebSocket adapter.

## Required evidence

Wire hex fixtures, compatibility matrix, bounded-size calculations, fuzz corpus, and protocol benchmark.

## Evidence and decision record

- Checked-in protocol documentation now records the [BTLV v1 byte contract](../../docs/CONTROL_BTLV_V1.md), [registry](../../docs/CONTROL_PROTOCOL_REGISTRY.md), [controller/queue semantics](../../docs/CONTROL_PROTOCOL_SEMANTICS.md), [bounded resource calculations](../../docs/CONTROL_PROTOCOL_SIZING.md), [provider/adapter boundary](../../docs/CONTROL_PROVIDER_BOUNDARY.md), and [non-benchmark conformance record](../../docs/CONTROL_PROTOCOL_CONFORMANCE.md). These documents summarize the accepted registry and already-recorded evidence; they make no new benchmark or device-runtime claim.

- The deterministic `miso-engine-protocol-audit` executable prepares its corpus, queues, caller
  output, and decode scratch before arming its audit-only forwarding `GlobalAlloc` wrapper. While
  armed it verifies zero allocations for complete typed command, success, non-OK, and event frame
  caller-buffer paths, a 64-edit session transaction, and exactly 10,000 automation records across
  40 bounded batches through encode, decode, enqueue, and dequeue. This is an allocation gate, not
  a timing benchmark.
- The audit exercises the generic borrowed outer-frame decoder and the borrowed automation
  decoder. Owned typed convenience decoders can materialize diagnostics or session edits on the
  control plane and are therefore intentionally outside this allocation-free contract.
- The global allocator exists only in the standalone audit binary and forwards unchanged to
  `System`; the workspace unsafe allowlist and mutation test reject unsafe code anywhere else.
- C3 non-benchmark conformance: `complete_schema_corpus()` deterministically encodes and typed-
  decodes all 11 commands (including a transaction containing every 42 allocated session-edit
  opcode), all 11 success responses, all 18 non-OK statuses including typed backpressure, and all
  six events. The reviewed corpus manifest pins its FNV-1a-64 label-and-byte hash to
  `88a8ee6a6d9e4acc`; the one-million mutation test selects the schema-specific command,
  response, event, or transaction decoder for every generated mutation.
- C3 target evidence: scalar and `simd128` Wasm execute and validate that same complete corpus;
  native workspace test/check/Clippy and Android/iOS compile checks passed. Native libFuzzer
  invocation 4 ran four typed targets (command, response, event, session transaction) for 10,000
  runs each with no crash: 40,000 new executions. The candid cumulative count is 140,000,
  including 80,000 current four-target-schema executions and 60,000 earlier-schema executions.
  No benchmark ran.
- Sol adversarial review recorded **ATTEMPT1 FAIL**. Repeated fields accepted a registered field
  under the wrong wire type; empty/any-revision transactions reached public controller/session
  paths; overlap checking did not span separately queued automation batches; meter flags and
  nonascending staged counter batches could be corrupted or stall; capabilities disagreed with
  dispatch; reliable-event/replay backpressure and diagnostic cursor errors were misclassified;
  and construction validation treated whole paginated provider catalogs as one frame. Short new-
  request output also reported a reservation bound as though it were an exact encoded length.
  The attempted FlatBuffers comparison was only a byte-vector wrapper around BTLV and made
  unsupported scratch/allocation/Wasm timing claims, so it was not a valid identical-logical-
  corpus comparison.
- Sol correction attempt 2 made every repeated decoder enforce its registered wire type; requires
  exact nonempty transactions throughout model/controller/wire ingress; tracks cross-batch
  automation intervals in prepared storage; preserves per-record meter flags; splits counter
  telemetry batches at nonascending IDs; derives capabilities and dispatch from one effective
  feature/limit registry (a zero edit limit removes command `0003`, events `8001/8002`, and flags
  3/12); returns typed queue/replay backpressure and distinguishes unavailable from expired
  diagnostic history. Provider fixtures now validate strict ordering and each independently
  paginatable item. A new `OutputReservationTooSmall` endpoint error distinguishes pre-dispatch
  response reservation from exact codec/materialized-frame `OutputTooSmall`, and every short-
  output test preserves caller bytes and state.
- Cancellation review also found and corrected a state-only `TRANSPORT_SET` path that could enter
  cancellation with zero reliable-event reservations while automation was queued. State-only
  transport changes now retain automation; locate/revision/lifecycle cancellation drains only
  after reserving every reliable event and resets the canceled ordering epoch so an earlier new
  absolute schedule can be admitted. Regression coverage exercises both outcomes.
- The corrected isolated `miso-engine-protocol-bench` tool has a checked 54-frame frozen logical
  corpus: capability query/response, one 64-operation transaction, two 128-item descriptor pages,
  two 128-item state pages, exactly 10,000 automation records in 40 batches, 256 meters,
  counter/diagnostic pages, and success/conflict/validation/backpressure responses. Its FNV-1a-64
  label-and-normalized-semantic-record checksum is `9eee4fcb61be3b9e`; native tests reconstruct it
  independently, and scalar/`simd128` Wasm assert that exact corpus checksum without timing.
- `protocol_benchmark.fbs` now defines a semantic `WireFrame` with typed unsigned, float-bit, and
  length-delimited UTF-8 key/value vectors; it has no `payload:[ubyte]` field and does not wrap
  BTLV. The exact-pinned tool-only Apache-2.0 `flatbuffers = 25.12.19` builder and `root_with_opts`
  verifier build and semantically compare every logical value under one-table, depth-two, 64 KiB
  bounds. The dependency has no engine, protocol, browser-host, or render-reachable target impact.
- Prepared BTLV owned wrappers and FlatBuffers scratch are created before allocation measurement.
  Output records measured encode/decode/rejection wall time, encoded bytes, post-preparation
  allocation count/bytes, prepared scratch and linear-memory bytes, toolchain/CPU/features/
  governor, and Wasm scalar/`simd128` artifact sizes. Timing is truthfully labeled
  `native-host-harness`; Wasm is `not-measured-corpus-parity-only`. The runner accepts only
  `--rounds 2`, performs one untimed warmup per format, and emits raw rounds in BTLV→FlatBuffers
  then FlatBuffers→BTLV order with no retry, best-of, tuning, or threshold.
- Sol attempt-2 non-benchmark evidence passed: `cargo fmt --check`; workspace all-target/all-
  feature check and Clippy with warnings denied; workspace tests (protocol 85/85 plus the one-
  million mutation corpus); warning-denied workspace docs; benchmark tool tests (4/4); workspace,
  realtime, and protocol-control policies plus mutation tests; protocol allocation audit; runner
  negative-argument guard; four typed fuzz targets × 10,000; scalar/`simd128` protocol and
  benchmark-corpus Wasm parity; and exact Android/iOS compile checks. **ATTEMPT1 FAIL; SOL ATTEMPT2
  PASS after the post-authorization readiness correction below.**
- The first authorized exact shell launch, `./scripts/run-protocol-benchmark.sh --rounds 2`, failed
  with exit 126 (`Permission denied`) before the script body, warmup, timing, or JSONL generation
  because the runner mode was `0644`. This records one failed shell launch but **zero authorized
  benchmark workload executions and zero results**. Sol corrected the runner mode to executable
  `0755` and made
  the nonbenchmark negative guard require and exercise that exact executable path, closing the
  readiness gap that its earlier `bash "$runner"` form missed. The corrected guard passed. At most
  one replacement benchmark run was then authorized, only as
  `./scripts/run-protocol-benchmark.sh --rounds 2`.
- That replacement completed its one warmup and prescribed two-round, 216-row workload, then
  failed exit 5 in post-run jq validation: the metadata clause changed jq `.` from the record to a
  key string and attempted `.[$key]`, producing `Cannot index string with string "toolchain"`.
  The raw JSONL existed only in a command-substitution shell variable and was unrecoverable after
  exit; read-only inspection found no Issue-005 raw, temporary, or result artifact. This records
  **one failed pre-workload shell launch, one completed but unaccepted benchmark workload, zero
  accepted results**.
- Final Sol attempt 3 retains the record as `$record` while checking metadata keys, moves jq and a
  known-record validator preflight before workload launch, and shares the exact validator between
  the runner and non-timing tests. A checked fixture proves valid string metadata no longer raises
  the prior jq runtime error, rejects a non-string `toolchain`, and expands to a synthetic 216-row
  corpus that passes the complete cardinality/order validator. The runner now writes raw JSONL to
  `target/issue005-protocol-benchmark.raw.jsonl` before validation, preserves it on failure,
  refuses to overwrite prior raw/accepted evidence, and renames it to the accepted artifact only
  after validation. Shell syntax and the runner/validator negative suite passed without executing
  timing work. **SOL ATTEMPT3 PASS.** Exactly one final replacement workload (workload execution
  2 of the allowed maximum 2) is authorized, only as
  `./scripts/run-protocol-benchmark.sh --rounds 2`; no further workload is authorized.
- The final authorized workload execution completed successfully and emitted all 216 prescribed
  newline-delimited records. The runner nevertheless reported failure because its `if !` token was
  separated from the environment-assignment command by a newline: Bash therefore treated the
  status-zero workload as the positive `if` condition and entered the failure branch. Read-only
  `/bin/true`/`/bin/false` control-flow checks established this cause without another benchmark.
  The preserved `target/issue005-protocol-benchmark.raw.jsonl` is 262,292 bytes with SHA-256
  `630fab07d3b28e64431616af3b03fbbce18a76250227753a269c9f9ce67b5b57`; the frozen validator
  passes it. It contains exactly 54 distinct labels, 108 rows per format, 108 rows per round, both
  prescribed format orders, and corpus checksum `9eee4fcb61be3b9e`. This complete validated raw
  artifact is the accepted final descriptive benchmark evidence. It is intentionally not renamed:
  the maximum workload count is exhausted, and no runner edit, artifact promotion, or further
  workload is authorized.
