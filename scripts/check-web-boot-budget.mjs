import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { readFile } from "node:fs/promises";
import process from "node:process";
import { sharedMemoryImports } from "../sdk/src/core/memory.ts";

const ABI_VERSION = 0x00010000;
const BOOT_OPTIONS_BYTES = 64;
// The worst document's length: 1 MiB, the staging ceiling until issue #2191 made the budget the
// document's limit, kept as a representative point of the parse projection.
const DOCUMENT_BYTES = 1 << 20;
const PARSE_TRANSIENT_MULTIPLIER = 20n;
const WASM_PAGE_BYTES = 1 << 16;
// `host_web::DEFAULT_MAXIMUM_MEMORY_BYTES`: the budget a zero `maximum_memory_bytes` word selects.
const DEFAULT_MAXIMUM_MEMORY_BYTES = 512 * 1024 * 1024;

if (process.argv.length !== 3) {
  throw new Error("usage: check-web-boot-budget.mjs MODULE.wasm");
}

const document = execFileSync(
  "cargo",
  ["run", "--locked", "-q", "-p", "host-web", "--example", "worst_boot_document"],
  { cwd: new URL("../", import.meta.url), maxBuffer: DOCUMENT_BYTES + WASM_PAGE_BYTES },
);
assert.equal(document.byteLength, DOCUMENT_BYTES);
// Issue #1295: the browser qualification builds this document in JS (its job has no Rust
// toolchain); the two builders must give the same bytes, the 1 MiB length included.
assert.deepEqual(
  Buffer.from((await import("../hosts/host-web/qualification/worst-replace-document.js")).worstReplaceDocument()),
  document,
  "qualification/worst-replace-document.js differs from the worst_boot_document example",
);
const compiledModule = await WebAssembly.compile(await readFile(process.argv[2]));

function stage(instance, maximumMemoryBytes, admitted = true, bytes = document) {
  const exports = instance.exports;
  assert.equal(exports.miso_engine_web_v1_abi_version(), ABI_VERSION);
  const optionsPointer = exports.miso_engine_web_v1_boot_options_ptr();
  assert.notEqual(optionsPointer, 0);
  const options = new DataView(exports.memory.buffer, optionsPointer, BOOT_OPTIONS_BYTES);
  options.setUint32(0, BOOT_OPTIONS_BYTES, true);
  options.setUint32(4, ABI_VERSION, true);
  options.setBigUint64(24, maximumMemoryBytes, true);
  const before = exports.memory.buffer.byteLength;
  const pointer = exports.miso_engine_web_v1_document_ptr(bytes.byteLength);
  if (!admitted) return { exports, pointer, before };
  assert.notEqual(pointer, 0);
  new Uint8Array(exports.memory.buffer, pointer, bytes.byteLength).set(bytes);
  return { exports, pointer };
}

/// A refused staging's diagnostic, read at `boot_diagnostic_ptr` (#2191).
function stagingRefusal(exports) {
  assert.equal(exports.miso_engine_web_v1_boot_result(), 5);
  const diagnosticBytes = exports.miso_engine_web_v1_boot_diagnostic_bytes();
  const diagnosticPointer = exports.miso_engine_web_v1_boot_diagnostic_ptr();
  assert.ok(diagnosticBytes > 0 && diagnosticPointer !== 0);
  return new TextDecoder().decode(
    new Uint8Array(exports.memory.buffer, diagnosticPointer, diagnosticBytes),
  );
}

// Issue #2191: the boot's pre-allocation projection is the document's parse transient plus the
// control plane's rows for the default control frame. The engine names both in its refusal, and
// the pin below is that projection: `20 * document + rows(frame)`, plus one page.
let pin;
let frameRows;
{
  const instance = await WebAssembly.instantiate(compiledModule, sharedMemoryImports());
  const { exports, pointer } = stage(instance, 1n, false);
  assert.equal(pointer, 0);
  const diagnostic = stagingRefusal(exports);
  const fields = /projected_bytes=(\d+),budget_bytes=1,document_bytes=(\d+),control_frame_bytes=(\d+),control_frame_rows_bytes=(\d+)\]/
    .exec(diagnostic);
  assert.ok(fields, diagnostic);
  const [projected, documentBytes, frame, rows] = fields.slice(1).map(BigInt);
  assert.equal(documentBytes, BigInt(document.byteLength));
  assert.equal(frame, 128n << 10n, "the default control frame");
  assert.ok(rows > 2n * frame, `the frame's rows (${rows}) hold its scratch and replay cache`);
  assert.equal(projected, documentBytes * PARSE_TRANSIENT_MULTIPLIER + rows);
  pin = projected;
  frameRows = rows;
}

{
  const instance = await WebAssembly.instantiate(compiledModule, sharedMemoryImports());
  const initialBytes = instance.exports.memory.buffer.byteLength;
  const { exports, pointer } = stage(instance, 0n);
  const handle = exports.miso_engine_web_v1_boot(document.byteLength);
  const diagnosticBytes = exports.miso_engine_web_v1_boot_diagnostic_bytes();
  const diagnostic = new TextDecoder().decode(
    new Uint8Array(exports.memory.buffer, pointer, diagnosticBytes),
  );
  assert.notEqual(
    handle,
    0,
    `accepted boot result ${exports.miso_engine_web_v1_boot_result()}: ${diagnostic}`,
  );
  const growth = exports.memory.buffer.byteLength - initialBytes;
  assert.ok(
    BigInt(growth) <= pin + BigInt(WASM_PAGE_BYTES),
    `wasm high-water growth ${growth} exceeds ${pin} plus one page`,
  );
  console.log(`web boot budget: growth ${growth} of projection ${pin}`);
  assert.equal(exports.miso_engine_web_v1_dispose(handle), 0);
}

// One byte under the projection, the staging itself refuses the document, before it allocates
// it, and its diagnostic names the projection and the budget.
{
  const instance = await WebAssembly.instantiate(compiledModule, sharedMemoryImports());
  const { exports, pointer, before } = stage(instance, pin - 1n, false);
  assert.equal(pointer, 0);
  const diagnostic = stagingRefusal(exports);
  assert.ok(
    diagnostic.startsWith(
      `host.budget.parse_projection\t$.maximum_memory_bytes[projected_bytes=${pin},budget_bytes=${pin - 1n},`,
    ),
    diagnostic,
  );
  assert.ok(
    exports.memory.buffer.byteLength <= before + WASM_PAGE_BYTES,
    "typed pre-parse refusal grew wasm memory by more than one page",
  );
}

// Issue #2194 gate D: 512 tracks with four delays each declare about 1.5 GB of effect state in
// about 1.4 MB of JSON. At the default budget the boot refuses it from its retained projection,
// before the control plane prepares one delay, so the linear memory grows by no more than the
// document's parse projection plus one page. The diagnostic keeps the name `effect_state_bytes`
// for the effect row, which since issue #2219 charges no processor for a banked member; no delay
// banks, so here it is every instance's.
{
  const effectDocument = execFileSync(
    "cargo",
    ["run", "--locked", "-q", "-p", "host-web", "--example", "effect_state_document"],
    { cwd: new URL("../", import.meta.url), maxBuffer: 4 << 20 },
  );
  const instance = await WebAssembly.instantiate(compiledModule, sharedMemoryImports());
  const initialBytes = instance.exports.memory.buffer.byteLength;
  const { exports } = stage(instance, 0n, true, effectDocument);
  assert.equal(exports.miso_engine_web_v1_boot(effectDocument.byteLength), 0);
  const diagnostic = stagingRefusal(exports);
  const effect = /^host\.budget\.retained_projection\t\$\.maximum_memory_bytes\[projected_bytes=\d+,budget_bytes=\d+,effect_state_bytes=(\d+),graph_bytes=\d+,builtin_bytes=\d+,control_bytes=\d+\]\n$/
    .exec(diagnostic);
  assert.ok(effect && BigInt(effect[1]) > 512n << 20n, diagnostic);
  const bound = BigInt(effectDocument.byteLength) * PARSE_TRANSIENT_MULTIPLIER + frameRows
    + BigInt(WASM_PAGE_BYTES);
  const growth = exports.memory.buffer.byteLength - initialBytes;
  assert.ok(
    BigInt(growth) <= bound,
    `effect-state refusal grew wasm memory by ${growth}, over ${bound}`,
  );
  console.log(`web boot budget: effect-state refusal growth ${growth} of bound ${bound}`);
}

// Issue #2199: the dense shape of #2191 at 7,000 tracks (issue #2219,
// `session::test_support::DENSE_GATE_TRACKS`), about 19.2 MB of JSON whose exact retained rows
// are 16.9 % over the default 512 MiB budget. The boot's retained projection (the graph, builtin and
// control rows beside the effect row) refuses it before the control plane prepares an effect,
// binds a bank or compiles the plan, so the linear memory grows by at most the budget plus one
// page; before #2199 the boot prepared everything and the module trapped in `memory.grow`.
{
  const denseDocument = execFileSync(
    "cargo",
    ["run", "--locked", "-q", "-p", "host-web", "--example", "dense_document"],
    { cwd: new URL("../", import.meta.url), maxBuffer: 32 << 20 },
  );
  const instance = await WebAssembly.instantiate(compiledModule, sharedMemoryImports());
  const initialBytes = instance.exports.memory.buffer.byteLength;
  const { exports } = stage(instance, 0n, true, denseDocument);
  assert.equal(exports.miso_engine_web_v1_boot(denseDocument.byteLength), 0);
  const diagnostic = stagingRefusal(exports);
  const projected = /^host\.budget\.retained_projection\t\$\.maximum_memory_bytes\[projected_bytes=(\d+),budget_bytes=(\d+),/
    .exec(diagnostic);
  assert.ok(projected && BigInt(projected[1]) > BigInt(projected[2]), diagnostic);
  const budget = BigInt(DEFAULT_MAXIMUM_MEMORY_BYTES);
  assert.equal(BigInt(projected[2]), budget, diagnostic);
  const growth = exports.memory.buffer.byteLength - initialBytes;
  assert.ok(
    BigInt(growth) <= budget + BigInt(WASM_PAGE_BYTES),
    `dense-shape refusal grew wasm memory by ${growth}, over the budget plus one page`,
  );
  console.log(
    `web boot budget: dense-shape refusal growth ${growth} of budget ${budget} (document ${denseDocument.byteLength})`,
  );
}

console.log("web boot budget high-water gate passed; mismatches: 0");
