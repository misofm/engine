import fs from 'node:fs';
const bytes = fs.readFileSync(process.argv[2]);
const { instance } = await WebAssembly.instantiate(bytes, {});
const e = instance.exports;
const mem = () => e.memory.buffer;
// 1. in-module exhaustive over 2^32
let t = Date.now();
const mism = e.exhaustive_mismatches();
console.log(`wasm exhaustive sanitize (Simd4 vs wasm scalar oracle): mismatches=${mism} (${Date.now()-t} ms)`);
// 2. JS-oracle check: independent of Rust scalar lowering; all 2^32 patterns via chunks
const N = 1 << 16;
let jsMism = 0, checked = 0;
t = Date.now();
const f32 = new Float32Array(1);
const u32 = new Uint32Array(f32.buffer);
const MINP = 1.1754943508222875e-38;
for (let base = 0; base < 2 ** 32; base += N) {
  const buf = new Uint32Array(mem(), e.buf_ptr(), N);
  for (let i = 0; i < N; i++) buf[i] = base + i;
  e.sanitize4(N);
  const out = new Uint32Array(mem(), e.out_ptr(), N);
  for (let i = 0; i < N; i++) {
    const bits = (base + i) >>> 0;
    const exp = (bits >>> 23) & 0xff;
    // normal_or_zero: finite and not subnormal; zero passes; then abs (clear sign)
    let want;
    if (exp === 0xff) want = 0;               // inf / NaN
    else if (exp === 0 && (bits & 0x7fffff) !== 0) want = 0; // subnormal
    else want = bits & 0x7fffffff;             // normal or zero, abs
    if (out[i] !== want) { if (jsMism < 5) console.log('js mismatch', bits.toString(16), out[i].toString(16), want.toString(16)); jsMism++; }
    checked++;
  }
}
console.log(`wasm sanitize vs JS bit-oracle: checked=${checked} mismatches=${jsMism} (${Date.now()-t} ms)`);
// 3. randomized kernel identity + reassociation in-module
t = Date.now();
const km = e.kernel_mismatches(0x9E3779B97F4A7C15n, 400);
console.log(`wasm kernel identity (carried + seeded-zero merge) mismatches=${km} (${Date.now()-t} ms)`);
