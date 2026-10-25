/**
 * #1563A gate 1: the SDK's `SET_CONTROL` transaction encoder writes the protocol codec's bytes, and
 * its reply decoder reads the codec's replies.
 *
 * `crates/protocol/tests/fixtures/set-control-frames.json` is written by the Rust codec
 * (`crates/protocol/tests/set_control_frames.rs` keeps it equal to what the codec writes). It holds
 * every owner kind and section shape in every lane with the ramp absent, `0` and `480`, identifier
 * lengths of every padding residue, unusual float bit patterns, large request IDs and revisions, and
 * the replies the core sends: applied on each edit path, refused with a diagnostic, a revision
 * conflict, and backpressure. This is a wire format, so a byte comparison is the gate.
 */

import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { describe, test } from "node:test";

import {
  commandFrame,
  concat,
  decodeControlReply,
  decodeObservationReadReply,
  field,
  nested,
  encodeControlTransaction,
  encodeObservationRead,
  encodeObserveTransaction,
  u8,
} from "../src/core/control-frame.ts";
import { MisoEngineError, MisoUsageError } from "../src/core/errors.ts";
import {
  decodeSpectrumCaptureReply,
  decodeSpectrumConfiguredReply,
  decodeSpectrumStreamReply,
  encodeSpectrumCommand,
} from "../src/core/spectrum-frame.ts";

const HEADER_BYTES = 48;

/** #1589b: the Rust codec's `Observe` and `OBSERVATION_READ` frames (`crates/protocol/tests/observation_frames.rs`). */
const OBSERVATION_FIXTURE = JSON.parse(await readFile(
  new URL("../../crates/protocol/tests/fixtures/observation-frames.json", import.meta.url),
  "utf8",
));

const FIXTURE = JSON.parse(await readFile(
  new URL("../../crates/protocol/tests/fixtures/set-control-frames.json", import.meta.url),
  "utf8",
));

/** #1596 gate 1: the Rust codec's spectrum frames (`crates/protocol/tests/spectrum_frames.rs`). */
const SPECTRUM_FIXTURE = JSON.parse(await readFile(
  new URL("../../crates/protocol/tests/fixtures/spectrum-frames.json", import.meta.url),
  "utf8",
));

function bytesOf(hex) {
  return Uint8Array.from(Buffer.from(hex, "hex"));
}

function hexOf(bytes) {
  return Buffer.from(bytes).toString("hex");
}

/** A float32 from its bit pattern (canonical NaN only: JS may canonicalise other NaN payloads). */
function floatFromBits(bits) {
  const view = new DataView(new ArrayBuffer(4));
  view.setUint32(0, Number.parseInt(bits, 16));
  return view.getFloat32(0);
}

function editOf(row) {
  return {
    address: {
      owner: row.owner,
      section: row.section,
      control: row.control,
      lane: row.lane,
      ...(row.expectedEffect === null ? {} : { expectedEffect: row.expectedEffect }),
    },
    value: floatFromBits(row.valueBits),
    ...(row.rampSamples === null ? {} : { rampSamples: row.rampSamples }),
  };
}

describe("the transaction encoder equals the protocol codec's bytes", () => {
  for (const command of FIXTURE.commands) {
    test(command.name, () => {
      const bytes = encodeControlTransaction({
        requestId: BigInt(command.requestId),
        expectedRevision: BigInt(command.expectedRevision),
        edits: command.edits.map(editOf),
      });
      assert.equal(hexOf(bytes), command.hex);
    });
  }

  test("the fixture reaches every owner kind, section, lane and ramp form", () => {
    const edits = FIXTURE.commands.flatMap((command) => command.edits);
    const seen = (pick) => new Set(edits.map(pick));
    assert.deepEqual([...seen((row) => row.owner.kind)].sort(), ["route", "session", "strip", "vca"]);
    assert.deepEqual(
      [...seen((row) => row.section.kind)].sort(),
      ["builtins", "console", "inserts", "own"],
    );
    assert.deepEqual([...seen((row) => row.lane)].sort(), ["both", "left", "right"]);
    assert.deepEqual(
      [...seen((row) => row.rampSamples)].sort((a, b) => (a ?? -1) - (b ?? -1)),
      [null, 0, 480, 4294967295],
    );
    // #1678: an insert and a console edit carry the expected effect; the others carry none.
    const withEffect = edits.filter((row) => row.expectedEffect !== null);
    assert.deepEqual([...new Set(withEffect.map((row) => row.section.kind))].sort(), ["console", "inserts"]);
    assert.ok(edits.some((row) => row.expectedEffect === null));
  });
});

describe("the reply decoder reads the codec's replies", () => {
  for (const reply of FIXTURE.replies) {
    test(reply.name, () => {
      const decoded = decodeControlReply(bytesOf(reply.hex));
      const expect = reply.expect;
      assert.equal(decoded.requestId, BigInt(expect.requestId));
      assert.equal(decoded.status, expect.status);
      assert.equal(decoded.revision, BigInt(expect.revision));
      assert.equal(decoded.omittedDiagnostics, expect.omittedDiagnostics);
      assert.equal(decoded.backpressure !== undefined, expect.backpressure);
      assert.deepEqual(
        decoded.diagnostics.map((row) => ({
          code: row.code,
          severity: row.severity,
          operationIndex: row.operationIndex ?? null,
        })),
        expect.diagnostics,
      );
      if (expect.status === "ok") {
        assert.deepEqual(
          { ...decoded.applied },
          {
            appliedOperations: expect.appliedOperations,
            editPath: expect.editPath,
            observeArmTags: expect.observeArmTags.map(BigInt),
            removedArmTags: expect.removedArmTags.map(BigInt),
          },
        );
      } else {
        assert.equal(decoded.applied, undefined);
      }
    });
  }

  test("an edit path code of 4 is the monitor path, and an unallocated code is refused", () => {
    const live = bytesOf(FIXTURE.replies.find((row) => row.name === "applied_live").hex);
    // The edit path is the first byte of the second field's value.
    const at = live.length - 8;
    assert.equal(live[at], 1);
    const monitor = Uint8Array.from(live);
    monitor[at] = 4;
    assert.equal(decodeControlReply(monitor).applied.editPath, "monitor");
    // 5 is `unchanged`, a `SESSION_REPLACE` reply's (#1386); the replace evals read it from the core.
    for (const code of [0, 6, 255]) {
      const bad = Uint8Array.from(live);
      bad[at] = code;
      assert.throws(() => decodeControlReply(bad), MisoEngineError, `code ${code}`);
    }
  });

  test("an arm-tag field that is empty or not whole tags is refused", () => {
    const good = bytesOf(FIXTURE.replies.find((row) => row.name === "applied_with_arm_tags").hex);
    // Field 4 (`removed_arm_tags`, one tag) is the frame's last 16 bytes: its 8-byte field header,
    // whose length is at offset 4, then the tag.
    const field = good.length - 16;
    assert.equal(new DataView(good.buffer).getUint16(field, true), 4);
    const empty = Uint8Array.from(good.subarray(0, good.length - 8));
    const view = new DataView(empty.buffer);
    view.setUint32(20, view.getUint32(20, true) - 8, true);
    view.setUint32(field + 4, 0, true);
    // Four bytes of tag and four of padding: the frame keeps its length.
    const ragged = Uint8Array.from(good);
    new DataView(ragged.buffer).setUint32(field + 4, 4, true);
    ragged.fill(0, field + 12);
    for (const [name, bytes] of Object.entries({ empty, ragged })) {
      assert.throws(() => decodeControlReply(bytes), MisoEngineError, name);
    }
  });

  test("a frame that is not a valid reply to this command is refused, never guessed at", () => {
    const good = bytesOf(FIXTURE.replies[0].hex);
    const mutated = (change) => {
      const copy = Uint8Array.from(good);
      change(copy);
      return copy;
    };
    const cases = {
      truncated: good.subarray(0, 47),
      "wrong magic": mutated((bytes) => { bytes[0] = 0; }),
      "a command frame": mutated((bytes) => { bytes[14] = 1; }),
      "another command": mutated((bytes) => { bytes[16] = 1; }),
      "unregistered status": mutated((bytes) => { bytes[18] = 99; }),
      "request id zero": mutated((bytes) => bytes.fill(0, 24, 32)),
      "nonzero reserved": mutated((bytes) => { bytes[44] = 1; }),
      "payload length": good.subarray(0, good.length - 8),
      "nonzero padding": mutated((bytes) => { bytes[HEADER_BYTES + 8 + 4] = 1; }),
    };
    for (const [name, bytes] of Object.entries(cases)) {
      assert.throws(() => decodeControlReply(bytes), MisoEngineError, name);
    }
  });
});


describe("the encoder refuses only what it cannot encode, before any byte", () => {
  const edit = FIXTURE.commands[0].edits.map(editOf)[0];
  const request = { requestId: 1n, expectedRevision: 0n, edits: [edit] };

  test("a value outside every domain is not the encoder's business", () => {
    for (const value of [30, 2, Number.NaN, Number.POSITIVE_INFINITY, -1e30]) {
      assert.doesNotThrow(() => encodeControlTransaction({ ...request, edits: [{ ...edit, value }] }));
    }
  });

  test("an unknown strip ID is the core's to refuse, not the encoder's", () => {
    const address = { ...edit.address, owner: { kind: "strip", id: "nobody" } };
    assert.doesNotThrow(() => encodeControlTransaction({ ...request, edits: [{ ...edit, address }] }));
  });

  test("an unencodable request is a MisoUsageError", () => {
    const refused = {
      "no edits": { ...request, edits: [] },
      "request id zero": { ...request, requestId: 0n },
      "request id past u64": { ...request, requestId: 1n << 64n },
      "request id a number": { ...request, requestId: 1 },
      "revision negative": { ...request, expectedRevision: -1n },
      "ramp fractional": { ...request, edits: [{ ...edit, rampSamples: 1.5 }] },
      "ramp negative": { ...request, edits: [{ ...edit, rampSamples: -1 }] },
      "ramp past u32": { ...request, edits: [{ ...edit, rampSamples: 2 ** 32 }] },
      "value a string": { ...request, edits: [{ ...edit, value: "1" }] },
      "owner id a number": {
        ...request,
        edits: [{ ...edit, address: { ...edit.address, owner: { kind: "strip", id: 3 } } }],
      },
      "lane unknown": {
        ...request,
        edits: [{ ...edit, address: { ...edit.address, lane: "middle" } }],
      },
      "expected effect on a builtin row": {
        ...request,
        edits: [{ ...edit, address: { ...edit.address, section: { kind: "builtins" }, expectedEffect: "miso.parametric-eq" } }],
      },
      "expected effect a number": {
        ...request,
        edits: [{
          ...edit,
          address: { ...edit.address, section: { kind: "inserts", effect: "eq" }, expectedEffect: 3 },
        }],
      },
      "control row fractional": {
        ...request,
        edits: [{ ...edit, address: { ...edit.address, control: 1.5 } }],
      },
    };
    for (const [name, bad] of Object.entries(refused)) {
      assert.throws(() => encodeControlTransaction(bad), MisoUsageError, name);
    }
  });
});

/** A fixture plane, `offset + step * (index % modulo)` over 1025 bins, as binary32 words. */
function planeOf(rule) {
  if (rule === null) return undefined;
  return Float32Array.from({ length: 1025 }, (_, index) => rule.offset + rule.step * (index % rule.modulo));
}

function spectrumRequestOf(request) {
  if (request.op === "configure") {
    return {
      ...request,
      maximumCaptureBytes: BigInt(request.maximumCaptureBytes),
      hopFrames: request.hopFrames ?? undefined,
    };
  }
  if (request.op === "stream" && request.stream === "select") {
    return { ...request, smoothingMs: request.smoothingMs ?? undefined };
  }
  return request;
}

const SPECTRUM_DECODERS = {
  configure: decodeSpectrumConfiguredReply,
  capture: decodeSpectrumCaptureReply,
  stream: decodeSpectrumStreamReply,
};

/** The decoded value with its planes and bigints as the fixture's `expect` spells them. */
function spectrumValueOf(message, value) {
  const big = (word) => word.toString();
  if (message === "configure") {
    return {
      configuredEpoch: big(value.configuredEpoch),
      adoptedEpoch: big(value.adoptedEpoch),
      rebuild: value.rebuild,
      planEpoch: value.planEpoch === undefined ? null : big(value.planEpoch),
    };
  }
  if (message === "capture") {
    const window = value.window;
    return {
      adoptedEpoch: big(value.adoptedEpoch),
      selectionEpoch: big(value.selectionEpoch),
      status: value.status,
      window: window === undefined ? null : {
        ...window,
        capturedSample: big(window.capturedSample),
        endSample: big(window.endSample),
        snapshotToken: big(window.snapshotToken),
      },
    };
  }
  return {
    ...value,
    target: value.target ?? null,
    channels: value.channels ?? null,
    bins: value.bins ?? null,
    ...Object.fromEntries([
      "adoptedEpoch", "selectionEpoch", "captureEpoch", "sequence", "droppedCaptures", "windows",
      "capturedSample", "endSample", "analysisEpoch", "historyStartSample",
    ].map((key) => [key, big(value[key])])),
  };
}

/** The fixture's `expect` with its plane rules made words. */
function spectrumExpectOf(message, expect) {
  const { message: _message, requestId: _id, revision: _revision, ok: _ok, ...value } = expect;
  const planes = (row) => ({
    ...row,
    frequenciesHz: planeOf(row.frequenciesHz),
    leftDb: planeOf(row.leftDb),
    rightDb: planeOf(row.rightDb),
  });
  if (message === "capture" && value.window !== null) return { ...value, window: planes(value.window) };
  if (message === "stream" && value.bins !== null) return { ...value, bins: planes(value.bins) };
  return value;
}

describe("the spectrum codec equals the protocol codec's frames (#1596 gate 1)", () => {
  for (const command of SPECTRUM_FIXTURE.commands) {
    test(`encode ${command.name}`, () => {
      const bytes = encodeSpectrumCommand(BigInt(command.requestId), spectrumRequestOf(command.request));
      assert.equal(hexOf(bytes), command.hex);
    });
  }

  for (const reply of SPECTRUM_FIXTURE.replies) {
    test(`decode ${reply.name}`, () => {
      const { expect } = reply;
      const decoded = SPECTRUM_DECODERS[expect.message](bytesOf(reply.hex));
      assert.equal(decoded.requestId, BigInt(expect.requestId));
      assert.equal(decoded.revision, BigInt(expect.revision));
      assert.equal(decoded.ok, expect.ok);
      if (expect.ok) {
        assert.deepEqual(spectrumValueOf(expect.message, decoded.value), spectrumExpectOf(expect.message, expect));
      } else {
        assert.equal(decoded.status, expect.status);
        assert.equal(decoded.diagnostics[0]?.code, expect.code);
      }
    });
  }
});

/** A binary32 word as the fixture's `leftBits`/`rightBits` spell it, or `null`. */
function bitsOf(value) {
  if (value === undefined) return null;
  const view = new DataView(new ArrayBuffer(4));
  view.setFloat32(0, value, true);
  return view.getUint32(0, true).toString(16).padStart(8, "0");
}

describe("the observation codec equals the protocol codec's frames (#1589b)", () => {
  for (const command of OBSERVATION_FIXTURE.commands) {
    test(`encode ${command.name}`, () => {
      const { request } = command;
      const bytes = request.op === "apply"
        ? encodeObserveTransaction({
          requestId: BigInt(command.requestId),
          expectedRevision: BigInt(request.expectedRevision),
          edits: request.edits,
        })
        : encodeObservationRead(BigInt(command.requestId), request.selections);
      assert.equal(hexOf(bytes), command.hex);
    });
  }

  for (const reply of OBSERVATION_FIXTURE.replies) {
    test(`decode ${reply.name}`, () => {
      const { expect } = reply;
      const decoded = decodeObservationReadReply(bytesOf(reply.hex));
      assert.equal(decoded.requestId, BigInt(expect.requestId));
      assert.equal(decoded.revision, BigInt(expect.revision));
      assert.equal(decoded.ok, expect.ok);
      if (!expect.ok) {
        assert.equal(decoded.status, expect.status);
        assert.equal(decoded.diagnostics[0]?.code, expect.code);
        return;
      }
      assert.equal(decoded.value.observedSample, BigInt(expect.observedSample));
      assert.deepEqual(decoded.value.rows.map((row) => ({
        status: row.status,
        armed: row.armed,
        armSample: row.armSample.toString(),
        armTag: row.armTag.toString(),
        window: row.window === undefined ? null : {
          firstSample: row.window.firstSample.toString(),
          endSample: row.window.endSample.toString(),
          sequence: row.window.sequence.toString(),
          blocks: row.window.blocks,
        },
        leftBits: bitsOf(row.left),
        rightBits: bitsOf(row.right),
        unit: row.unit,
      })), expect.rows);
    });
  }
});

/** An `OBSERVATION_READ` reply of one unarmed row, its unit `unit`, and `extra` fields after it. */
function corruptPage(unit, extra = []) {
  const u64 = (value) => {
    const out = new Uint8Array(8);
    new DataView(out.buffer).setBigUint64(0, value, true);
    return out;
  };
  const row = nested(2, true, [
    field(1, 1, true, u8(2)),
    field(2, 8, true, u8(0)),
    field(3, 4, true, u64(0n)),
    field(4, 4, true, u64(0n)),
    field(11, 1, true, u8(unit)),
    ...extra,
  ]);
  const fields = [field(1, 4, true, u64(0n)), row];
  const out = commandFrame(0x000f, 0, 7n, 5n, fields.length, concat(fields));
  out[14] = 2;
  return out;
}

describe("the observation row decoder refuses what the Rust codec refuses (#1589b)", () => {
  // Red if a corrupt row is taken as data: the Rust codec knows units 1 to 6 and row fields 1 to 11.
  test("a well-formed row decodes, a unit above 6 and a field past 11 are malformed", () => {
    assert.equal(decodeObservationReadReply(corruptPage(6)).value.rows[0].unit, 6);
    assert.throws(() => decodeObservationReadReply(corruptPage(7)), MisoEngineError);
    assert.throws(() => decodeObservationReadReply(corruptPage(1, [field(12, 1, false, u8(0))])), MisoEngineError);
  });
});
