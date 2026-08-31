import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { readFile } from "node:fs/promises";
import process from "node:process";

const ABI_VERSION = 0x00020000;
const BOOT_OPTIONS_BYTES = 64;
const MAXIMUM_DOCUMENT_BYTES = 1 << 20;
const PARSE_TRANSIENT_MULTIPLIER = 80n;
const WASM_PAGE_BYTES = 1 << 16;

if (process.argv.length !== 3) {
  throw new Error("usage: check-web-boot-budget.mjs MODULE.wasm");
}

const document = execFileSync(
  "cargo",
  ["run", "--locked", "-q", "-p", "host-web", "--example", "worst_boot_document"],
  { cwd: new URL("../", import.meta.url), maxBuffer: MAXIMUM_DOCUMENT_BYTES + WASM_PAGE_BYTES },
);
assert.equal(document.byteLength, MAXIMUM_DOCUMENT_BYTES);
const compiledModule = await WebAssembly.compile(await readFile(process.argv[2]));

function stage(instance, maximumMemoryBytes) {
  const exports = instance.exports;
  assert.equal(exports.miso_engine_web_v1_abi_version(), ABI_VERSION);
  const optionsPointer = exports.miso_engine_web_v1_boot_options_ptr();
  assert.notEqual(optionsPointer, 0);
  const options = new DataView(exports.memory.buffer, optionsPointer, BOOT_OPTIONS_BYTES);
  options.setUint32(0, BOOT_OPTIONS_BYTES, true);
  options.setUint32(4, ABI_VERSION, true);
  options.setBigUint64(24, maximumMemoryBytes, true);
  const pointer = exports.miso_engine_web_v1_document_ptr(document.byteLength);
  assert.notEqual(pointer, 0);
  new Uint8Array(exports.memory.buffer, pointer, document.byteLength).set(document);
  return { exports, pointer };
}

{
  const instance = await WebAssembly.instantiate(compiledModule, {});
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
  const pin = BigInt(document.byteLength) * PARSE_TRANSIENT_MULTIPLIER;
  assert.ok(
    BigInt(growth) <= pin + BigInt(WASM_PAGE_BYTES),
    `wasm high-water growth ${growth} exceeds ${pin} plus one page`,
  );
  assert.equal(exports.miso_engine_web_v1_dispose(handle), 0);
}

{
  const instance = await WebAssembly.instantiate(compiledModule, {});
  const pin = BigInt(document.byteLength) * PARSE_TRANSIENT_MULTIPLIER;
  const { exports, pointer } = stage(instance, pin - 1n);
  const stagedBytes = exports.memory.buffer.byteLength;
  assert.equal(exports.miso_engine_web_v1_boot(document.byteLength), 0);
  assert.equal(exports.miso_engine_web_v1_boot_result(), 5);
  const diagnosticBytes = exports.miso_engine_web_v1_boot_diagnostic_bytes();
  assert.ok(diagnosticBytes > 0);
  const diagnostic = new TextDecoder().decode(
    new Uint8Array(exports.memory.buffer, pointer, diagnosticBytes),
  );
  assert.ok(diagnostic.startsWith("host.budget.parse_projection\t"));
  assert.ok(
    exports.memory.buffer.byteLength <= stagedBytes + WASM_PAGE_BYTES,
    "typed pre-parse refusal grew wasm memory by more than one page",
  );
}

console.log("web boot budget high-water gate passed; mismatches: 0");
