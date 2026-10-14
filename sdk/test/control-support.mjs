/**
 * A scripted `host.control()` for tests whose browser host is a hand-written fake. Kept apart from
 * `support.mjs`: it reads the frame header by byte offset, which the SDK offset gate
 * (`scripts/check-sdk-deletions.py`) allows only in files that touch no ABI layout.
 */

import { readFileSync } from "node:fs";

import { concat, field, text, u32, u8 } from "../src/core/control-frame.ts";

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
export function scriptedControl({ revision = 1n, frames = [], answer, strips = {} } = {}) {
  let committed = revision;
  let tag = 0;
  const applied = Uint8Array.from(Buffer.from(
    SET_CONTROL_FIXTURE.replies.find((row) => row.name === "applied_live").hex, "hex"));
  return async function control(frame) {
    frames.push(frame.slice());
    tag += 1;
    const view = new DataView(frame.buffer, frame.byteOffset, frame.byteLength);
    const requestId = view.getBigUint64(24, true);
    if (view.getUint16(16, true) === 0x000c) {
      // CONTROL_LIST_GET: the strip headers of the scripted session, on one page.
      return { tag: "miso.control.v1", requestId: tag, result: 0, response: stripListReply(requestId, committed, strips), revision: committed };
    }
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

const WIRE_U8 = 1;
const WIRE_U32 = 3;
const WIRE_U64 = 4;
const WIRE_BOOL = 8;
const WIRE_UTF8 = 9;
const WIRE_MESSAGE = 11;
const OWNER_STRIP = 2;
const SECTION_OWN = 6;
const ROW_OWNER_HEADER = 1;
const HEADER_KIND = { track: 1, submix: 2 };

function u64Value(value) {
  const out = new Uint8Array(8);
  new DataView(out.buffer).setBigUint64(0, value, true);
  return out;
}

function message(fields) {
  const header = new Uint8Array(8);
  new DataView(header.buffer).setUint32(0, fields.length, true);
  return concat([header, ...fields]);
}

/**
 * A `CONTROL_LIST_GET` reply that holds only the strip owner headers (tracks, then submixes) of a
 * scripted session: the `ok` page a fake browser host answers with, so the SDK reads its strip order
 * from a list as it does from the core. `strips` is `{ tracks, submixes, structureEpoch }`.
 */
export function stripListReply(requestId, revision, { tracks = ["t"], submixes = [], structureEpoch = 1n } = {}) {
  const rows = [
    ...tracks.map((id) => ["track", id]),
    ...submixes.map((id) => ["submix", id]),
  ].map(([kind, id], index) => field(5, WIRE_MESSAGE, true, message([
    field(1, WIRE_U32, true, u32(index + 1)),
    field(2, WIRE_U8, true, u8(ROW_OWNER_HEADER)),
    field(3, WIRE_MESSAGE, true, message([
      field(1, WIRE_U8, true, u8(OWNER_STRIP)),
      field(2, WIRE_UTF8, false, text(id)),
      field(3, WIRE_U8, true, u8(SECTION_OWN)),
    ])),
    field(4, WIRE_U8, true, u8(HEADER_KIND[kind])),
    field(5, WIRE_UTF8, true, text(id)),
  ])));
  const fields = [
    field(1, WIRE_U64, true, u64Value(revision)),
    field(2, WIRE_U64, true, u64Value(structureEpoch)),
    field(3, WIRE_U32, true, u32(rows.length)),
    field(4, WIRE_BOOL, true, Uint8Array.of(1)),
    ...rows,
  ];
  const payload = concat(fields);
  const out = new Uint8Array(48 + payload.length);
  const view = new DataView(out.buffer);
  out.set([0x4d, 0x49, 0x53, 0x4f, 0x43, 0x54, 0x4c, 0x00], 0);
  view.setUint16(8, 1, true);
  view.setUint16(10, 0, true);
  view.setUint16(12, 48, true);
  out[14] = 2;
  view.setUint16(16, 0x000c, true);
  view.setUint16(18, 0, true);
  view.setUint32(20, payload.length, true);
  view.setBigUint64(24, requestId, true);
  view.setBigUint64(32, revision, true);
  view.setUint32(40, fields.length, true);
  out.set(payload, 48);
  return out;
}

/**
 * Put the frames of a second sender onto an endpoint's one request-ID allocator. A fixture that
 * serves a browser host from a headless engine has two senders on one core (the SDK's browser queue
 * and the engine's own strip-order reads); the core takes request IDs in ascending order and refuses
 * a reused one, so the fixture renumbers the browser's frames onto the engine's allocator and gives
 * each reply its caller's ID back.
 */
export function onEndpointIds(allocator, send) {
  return (frame) => {
    const own = new DataView(frame.buffer, frame.byteOffset, frame.byteLength).getBigUint64(24, true);
    const renumbered = frame.slice();
    new DataView(renumbered.buffer).setBigUint64(24, allocator.next(), true);
    const reply = send(renumbered).slice();
    new DataView(reply.buffer).setBigUint64(24, own, true);
    return reply;
  };
}
