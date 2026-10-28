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

function stage(instance, maximumMemoryBytes, admitted = true) {
  const exports = instance.exports;
  assert.equal(exports.miso_engine_web_v1_abi_version(), ABI_VERSION);
  const optionsPointer = exports.miso_engine_web_v1_boot_options_ptr();
  assert.notEqual(optionsPointer, 0);
  const options = new DataView(exports.memory.buffer, optionsPointer, BOOT_OPTIONS_BYTES);
  options.setUint32(0, BOOT_OPTIONS_BYTES, true);
  options.setUint32(4, ABI_VERSION, true);
  options.setBigUint64(24, maximumMemoryBytes, true);
  const before = exports.memory.buffer.byteLength;
  const pointer = exports.miso_engine_web_v1_document_ptr(document.byteLength);
  if (!admitted) return { exports, pointer, before };
  assert.notEqual(pointer, 0);
  new Uint8Array(exports.memory.buffer, pointer, document.byteLength).set(document);
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

console.log("web boot budget high-water gate passed; mismatches: 0");
