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
  decodeControlReply,
  encodeControlTransaction,
} from "../src/core/control-frame.ts";
import { MisoEngineError, MisoUsageError } from "../src/core/errors.ts";

const HEADER_BYTES = 48;

const FIXTURE = JSON.parse(await readFile(
  new URL("../../crates/protocol/tests/fixtures/set-control-frames.json", import.meta.url),
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
    for (const code of [0, 5, 255]) {
      const bad = Uint8Array.from(live);
      bad[at] = code;
      assert.throws(() => decodeControlReply(bad), MisoEngineError, `code ${code}`);
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
