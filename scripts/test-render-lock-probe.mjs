// Issue #1529: the positive control of the render-allocation gates, wasm leg.
//
// Usage: node scripts/test-render-lock-probe.mjs PROBE_MODULE
//
// PROBE_MODULE is `miso-engine-v1-render-lock-probe.wasm` from
// `scripts/build-web-audioworklet.sh --render-lock-probe`: the shipped module's sources and cargo
// line plus the default-off `render-lock-probe` feature. On a fresh instance, an allocation and a
// free outside a render-locked window move the count by 0, and inside one by exactly 2. A
// browser module whose `#[global_allocator]` is unregistered or differently cfg'd counts nothing,
// which every zero gate passes and this one does not.
import { readFile } from "node:fs/promises";
import { sharedMemoryImports } from "../sdk/src/core/memory.ts";

if (process.argv.length !== 3) {
  console.error("usage: node scripts/test-render-lock-probe.mjs PROBE_MODULE");
  process.exit(2);
}
const bytes = await readFile(process.argv[2]);
const { instance } = await WebAssembly.instantiate(bytes, sharedMemoryImports());
const probe = instance.exports.miso_engine_web_v1_render_lock_probe;
if (typeof probe !== "function") {
  console.error("the module has no miso_engine_web_v1_render_lock_probe export");
  process.exit(1);
}
const observed = [probe(0), probe(1), probe(0)];
const expected = [0, 2, 2];
if (observed.join() !== expected.join()) {
  console.error(`render-lock probe: expected [${expected}], observed [${observed}]`);
  process.exit(1);
}
console.log(`render-lock probe: [${observed}] (an allocation in a render-locked window is counted)`);
