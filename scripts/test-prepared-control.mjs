import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { createPreparedControl } from "../hosts/host-web/web/prepared-control.js";
const abiLayout = JSON.parse(await readFile(new URL("../sdk/assets/miso-engine-v1-abi-layout.json", import.meta.url)));

// A transport fixture, not a second EQ designer. Rust owns the numerical integration gates.
function fixture() {
  const memory = new WebAssembly.Memory({ initial: 1 });
  const log = { seeds: [], payloads: [], copies: 0, closed: 0 };
  let reject = false;
  const instance = { exports: { memory } };
  const methods = {
    open: () => 0, close: () => { log.closed++; return 0; },
    request_ptr: () => 1024, request_capacity: () => 3344,
    result_ptr: () => 8192, result_capacity: () => 944, result_bytes: () => 328,
    rejected_edit_index: () => 0, rejected_reason: () => 6,
    prepare: () => {
      const request = new DataView(memory.buffer, 1024);
      log.seeds.push(request.getFloat32(32, true));
      if (reject) return 1;
      const output = new DataView(memory.buffer, 8192, 328);
      output.setUint32(0, 32, true); output.setUint32(4, abiLayout.abiVersion, true);
      output.setUint32(8, 60, true); output.setUint32(12, 1, true);
      new Uint8Array(memory.buffer, 8192 + 32, 240).set(new Uint8Array(memory.buffer, 1024 + 32, 240));
      output.setFloat32(32, request.getFloat32(280, true), true);
      output.setUint32(272, 0, true); output.setUint32(276, 2, true);
      output.setUint32(280, 0x3f800000, true);
      return 0;
    },
  };
  for (const [name, callback] of Object.entries(methods)) instance.exports[`miso_engine_web_v1_eq_target_${name}`] = callback;
  const configCopy = ({ effectIndex }) => {
    log.copies++;
    if (effectIndex === 1) return { result: 7 };
    const config = new Uint8Array(272), view = new DataView(config.buffer);
    view.setUint32(0, 272, true); view.setUint32(4, abiLayout.abiVersion, true);
    view.setUint32(8, 48000, true); view.setUint32(12, 60, true);
    view.setBigUint64(16, 1n, true);
    return { result: 0, config };
  };
  const ack = (records, count) => ({ result: 0, admitted: count, records });
  const submit = (records, companion, count) => {
    log.payloads.push(companion.slice());
    return ack(structuredClone(records, { transfer: [records.buffer] }), count);
  };
  const refusal = (records, count, reason, rejectedIndex, result) => ({ result, reason, rejectedIndex: rejectedIndex ?? 0, admitted: 0, records });
  const options = { instance, abiLayout, sampleRateHz: 48000, configCopy, configCopySync: configCopy,
    ordinarySubmit: ack, ordinarySubmitSync: ack, preparedSubmit: submit, preparedSubmitSync: submit,
    preparedRefusal: refusal, preparedRefusalSync: refusal };
  return { options, log, reject: () => { reject = true; } };
}
function records(value, effectIndex = 0, leadingOrdinary = false) {
  const bytes = new Uint8Array(48 * (leadingOrdinary ? 2 : 1));
  const offset = leadingOrdinary ? 48 : 0, view = new DataView(bytes.buffer);
  if (leadingOrdinary) view.setUint8(0, 3);
  view.setUint8(offset, 5); view.setUint8(offset + 2, 2);
  view.setUint32(offset + 8, effectIndex, true); view.setUint32(offset + 12, 65, true);
  view.setFloat32(offset + 24, value, true);
  return bytes;
}
const a = fixture(), asyncOwner = createPreparedControl(a.options);
assert.equal((await asyncOwner.submit(records(100), 1)).admitted, 1);
assert.equal((await asyncOwner.submit(records(200), 1)).admitted, 1);
assert.deepEqual(a.log.seeds, [0, 100]);
assert.equal(new DataView(a.log.payloads[1].buffer).getBigUint64(40, true), 1n);
const b = fixture(), syncOwner = createPreparedControl(b.options);
assert.equal(syncOwner.submitSync(records(100), 1).admitted, 1);
assert.deepEqual(a.log.payloads[0], b.log.payloads[0]);
b.reject();
const refused = syncOwner.submitSync(records(NaN, 0, true), 2);
assert.equal(refused.admitted, 0); assert.equal(refused.rejectedIndex, 1); assert.equal(refused.reason, 6);
assert.equal(b.log.payloads.length, 1);

const c = fixture();
let release;
c.options.preparedSubmit = (...args) => new Promise((resolve) => { release = () => resolve(c.options.preparedSubmitSync(...args)); });
const pendingOwner = createPreparedControl(c.options);
await pendingOwner.submit(records(1, 1), 1); // Cache only authoritative ordinary classification.
const pending = pendingOwner.submit(records(100), 1);
while (release === undefined) await Promise.resolve();
const busy = await pendingOwner.submit(records(200), 1);
assert.equal(busy.result, 6); assert.equal(busy.reason, 8); assert.equal(busy.admitted, 0);
assert.equal((await pendingOwner.submit(records(2, 1), 1)).admitted, 1);
release(); await pending;

const d = fixture();
let seedReply;
d.options.configCopy = (address) => new Promise((resolve) => { seedReply = () => resolve(d.options.configCopySync(address)); });
const closing = createPreparedControl(d.options), interrupted = closing.submit(records(100), 1);
closing.close(); seedReply();
await assert.rejects(interrupted, (error) => error.result === 3);
assert.equal(d.log.payloads.length, 0); assert.equal(d.log.closed, 1);

const e = fixture();
e.options.preparedSubmit = (bytes, payload, count) => ({ result: 0, admitted: count, records: new Uint8Array(bytes.length) });
const malformed = createPreparedControl(e.options);
await assert.rejects(malformed.submit(records(100), 1), (error) => error.result === 255);
e.options.preparedSubmit = e.options.preparedSubmitSync;
await malformed.submit(records(100), 1);
assert.equal(e.log.copies, 2);
assert.deepEqual(e.log.seeds, [0, 0]);
console.log("prepared-control sync/async, transfer/ACK, refusal, busy and lifecycle tests passed");
