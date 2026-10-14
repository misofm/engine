/**
 * A scripted `host.control()` for tests whose browser host is a hand-written fake. Kept apart from
 * `support.mjs`: it reads the frame header by byte offset, which the SDK offset gate
 * (`scripts/check-sdk-deletions.py`) allows only in files that touch no ABI layout.
 */

import { readFileSync } from "node:fs";

const SET_CONTROL_FIXTURE = JSON.parse(readFileSync(
  new URL("../../crates/protocol/tests/fixtures/set-control-frames.json", import.meta.url),
  "utf8",
));

/**
 * It answers a
 * revision probe, and answers each transaction with the Rust codec's `applied_live` reply frame
 * (the fixture of `control-frame-evals.mjs`) patched to the request it received, the revision
 * counting up from `revision`. `frames` records every frame it was sent.
 */
export function scriptedControl({ revision = 1n, frames = [], answer } = {}) {
  let committed = revision;
  let tag = 0;
  const applied = Uint8Array.from(Buffer.from(
    SET_CONTROL_FIXTURE.replies.find((row) => row.name === "applied_live").hex, "hex"));
  return async function control(frame) {
    frames.push(frame.slice());
    tag += 1;
    const view = new DataView(frame.buffer, frame.byteOffset, frame.byteLength);
    const requestId = view.getBigUint64(24, true);
    if (view.getUint16(16, true) !== 0x0003) {
      return { tag: "miso.control.v1", requestId: tag, result: 0, response: new Uint8Array(48), revision: committed };
    }
    if (answer !== undefined) return answer(frame, requestId, tag);
    committed += 1n;
    const reply = applied.slice();
    const out = new DataView(reply.buffer);
    out.setBigUint64(24, requestId, true);
    out.setBigUint64(32, committed, true);
    return { tag: "miso.control.v1", requestId: tag, result: 0, response: reply, revision: committed };
  };
}
