// Issue #1295 D2: the run-time reference of the structural-edit row. The shipped module, under
// Node, through the same exports the worklet calls, single instance: boot document A with the boot
// words the page's host writes, submit the same source blocks, send the same `SESSION_REPLACE`
// frame and document B before block `replaceBlock` (never, when it is `null`), and render
// `blocks` quanta. The page's capture must equal the `replaceBlock` 0 render bit for bit: the
// offline context's first quantum is the swap block. No digest is committed; this runs every time.
//
// The boot and the command staging are `tests/browser-v1/direct-oracle.mjs`'s `boot` and
// `sendControlFrame`, with the two words the worklet sets differently (the ring frames and the
// document staging); that script runs its own `main` on import and exports nothing.

import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFile } from "node:fs/promises";
import { sharedMemoryImports } from "../../../sdk/src/core/memory.ts";

const BOOT_OPTIONS_BYTES = 64;
const SAMPLE_RATE = 48000;
const QUANTUM = 128;
const BUFFER_SOURCE_ID = 2;
const BUFFER_SOURCE_PCM = 3;
const BUFFER_OUTPUT_PCM = 5;

function boot(exports, document, sourceRingFrames) {
  const pointer = exports.miso_engine_web_v1_boot_options_ptr();
  assert.notEqual(pointer, 0);
  const view = new DataView(exports.memory.buffer, pointer, BOOT_OPTIONS_BYTES);
  view.setUint32(0, BOOT_OPTIONS_BYTES, true);
  view.setUint32(4, exports.miso_engine_web_v1_abi_version(), true);
  view.setUint32(8, SAMPLE_RATE, true);
  view.setUint32(12, QUANTUM, true);
  view.setUint32(16, sourceRingFrames, true);
  view.setUint32(20, 0, true);
  for (let offset = 24; offset < BOOT_OPTIONS_BYTES; offset += 8) view.setBigUint64(offset, 0n, true);
  const documentPointer = exports.miso_engine_web_v1_document_ptr(document.byteLength);
  assert.notEqual(documentPointer, 0);
  new Uint8Array(exports.memory.buffer, documentPointer, document.byteLength).set(document);
  const handle = exports.miso_engine_web_v1_boot(document.byteLength);
  assert.notEqual(handle, 0, `boot refused with ${exports.miso_engine_web_v1_boot_result()}`);
  return handle;
}

/// The page's `corpusPlanes`: the left plane a ramp, the right plane zero.
function writeSource(exports, handle, sourceId, block) {
  const id = new TextEncoder().encode(sourceId);
  new Uint8Array(exports.memory.buffer, exports.miso_engine_web_v1_buffer_ptr(handle, BUFFER_SOURCE_ID), id.byteLength)
    .set(id);
  const pcm = new Float32Array(
    exports.memory.buffer,
    exports.miso_engine_web_v1_buffer_ptr(handle, BUFFER_SOURCE_PCM),
    QUANTUM * 2,
  );
  for (let frame = 0; frame < QUANTUM; frame += 1) {
    pcm[frame] = block.leftBase + block.leftStep * frame;
    pcm[QUANTUM + frame] = 0;
  }
  return exports.miso_engine_web_v1_source_submit(
    handle, id.byteLength, 1n, BigInt(block.startFrame), 2, block.frames, block.final ? 1 : 0,
  );
}

/// The worklet's `readControlCommand`: the frame's staging, then the document's, then the command;
/// then both event lanes drained, as the worklet drains them.
function sendReplace(exports, handle, frame, document) {
  const pointer = exports.miso_engine_web_v1_control_request_ptr(handle, frame.byteLength);
  assert.notEqual(pointer, 0);
  const documentPointer = exports.miso_engine_web_v1_replace_document_ptr(handle, document.byteLength);
  assert.notEqual(documentPointer, 0);
  new Uint8Array(exports.memory.buffer, documentPointer, document.byteLength).set(document);
  new Uint8Array(exports.memory.buffer, pointer, frame.byteLength).set(frame);
  assert.equal(exports.miso_engine_web_v1_control_command(handle, frame.byteLength), 0);
  for (const lane of [0, 1]) {
    for (let guard = 0; guard < 16; guard += 1) {
      assert.equal(exports.miso_engine_web_v1_control_event(handle, lane), 0);
      if (exports.miso_engine_web_v1_control_response_bytes(handle) === 0) break;
    }
  }
  return exports.miso_engine_web_v1_control_revision(handle);
}

function digest(planes) {
  const bytes = Buffer.alloc(planes.length * planes[0].length * 4);
  let offset = 0;
  for (const plane of planes) {
    for (const sample of plane) offset = bytes.writeFloatLE(sample, offset);
  }
  return createHash("sha256").update(bytes).digest("hex");
}

/// Render the reference. `frame` is the page's frame, byte for byte (its request ID and revision
/// are the page's: the instance here is fresh at the same revision).
///
/// Issue #1332 S3: `sourceAfterReplace` (optional) is the source the page feeds once the replace is
/// committed, submitted here right after it, before the block's render.
export async function structuralEditReference({
  modulePath, documentA, documentB, source, frame, sourceRingFrames, blocks, replaceBlock,
  sourceAfterReplace = null,
}) {
  const { instance } = await WebAssembly.instantiate(await readFile(modulePath), sharedMemoryImports());
  const exports = instance.exports;
  const handle = boot(exports, documentA, sourceRingFrames);
  for (const block of source.blocks) {
    assert.equal(writeSource(exports, handle, source.sourceId, block), 0, "reference source refused");
  }
  const planes = [new Float32Array(blocks * QUANTUM), new Float32Array(blocks * QUANTUM)];
  let revision = null;
  for (let block = 0; block < blocks; block += 1) {
    if (block === replaceBlock) {
      revision = sendReplace(exports, handle, frame, documentB);
      for (const after of sourceAfterReplace?.blocks ?? []) {
        assert.equal(writeSource(exports, handle, sourceAfterReplace.sourceId, after), 0, "reference source refused");
      }
    }
    assert.equal(exports.miso_engine_web_v1_render(handle, QUANTUM), 0);
    const output = exports.miso_engine_web_v1_buffer_ptr(handle, BUFFER_OUTPUT_PCM);
    planes[0].set(new Float32Array(exports.memory.buffer, output, QUANTUM), block * QUANTUM);
    planes[1].set(new Float32Array(exports.memory.buffer, output + QUANTUM * 4, QUANTUM), block * QUANTUM);
  }
  assert.equal(exports.miso_engine_web_v1_dispose(handle), 0);
  return { digest: digest(planes), revision: revision === null ? null : revision.toString() };
}
