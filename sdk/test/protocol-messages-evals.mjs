/**
 * #1571a: the SDK's protocol message codec and the headless engine's calls over it.
 *
 * Gate 1: `sdk/test/fixtures/protocol-messages.json` is written by the Rust codec
 * (`crates/protocol/tests/sdk_message_fixture.rs` keeps it equal to what the codec writes). It holds
 * one frame of each request the SDK encodes, one of each reply and event it decodes (every optional
 * field present once and absent once), and every counter ID the codec accepts. The SDK encodes the
 * same requests and compares bytes, and decodes every reply and event and compares values. Gates 2
 * to 5 run the headless engine; their browser rows are tranche b's.
 */

import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { before, describe, test } from "node:test";

import { MisoEngineAsset } from "../src/core/asset.ts";
import { MisoControlRefusal, MisoEngineError, MisoUsageError } from "../src/core/errors.ts";
import {
  COUNTER_IDS,
  ProtocolClient,
  decodeCapabilitiesReply,
  decodeCountersReply,
  decodeDiagnosticsReply,
  decodeProtocolEvent,
  decodeSessionSnapshotReply,
  decodeTelemetryReply,
  decodeTransportReply,
  encodeCapabilitiesRequest,
  encodeCountersRequest,
  encodeDiagnosticsRequest,
  encodeSessionSnapshotRequest,
  encodeTelemetryConfigureRequest,
  encodeTransportGetRequest,
  encodeTransportSetRequest,
} from "../src/core/protocol-messages.ts";
import { createOfflineEngine } from "../src/headless/engine.ts";
import { session } from "../src/core/session.ts";
import { transaction } from "../src/core/session-transaction.ts";
import { moduleBytes, sessionDocument } from "./support.mjs";

const FIXTURE = JSON.parse(await readFile(new URL("./fixtures/protocol-messages.json", import.meta.url), "utf8"));

const BIGINT_KEYS = new Set(["requestId", "expectedRevision", "offset", "afterSequence"]);

function bytesOf(hex) {
  return Uint8Array.from(Buffer.from(hex, "hex"));
}

function hexOf(bytes) {
  return Buffer.from(bytes).toString("hex");
}

/** A fixture request as the SDK takes it: the decimal strings of `u64` fields become bigints. */
function requestOf(row) {
  return Object.fromEntries(Object.entries(row).map(([key, value]) => [key, BIGINT_KEYS.has(key) ? BigInt(value) : value]));
}

/** A decoded value in the fixture's spelling: bigints as decimal strings, bytes as hex. */
function plain(value) {
  return JSON.parse(JSON.stringify(value, (_key, item) => {
    if (typeof item === "bigint") return item.toString();
    if (item instanceof Uint8Array) return hexOf(item);
    return item;
  }));
}

const ENCODERS = {
  capabilities: encodeCapabilitiesRequest,
  sessionSnapshot: encodeSessionSnapshotRequest,
  transportGet: encodeTransportGetRequest,
  transportSet: encodeTransportSetRequest,
  telemetryConfigure: encodeTelemetryConfigureRequest,
  counters: encodeCountersRequest,
  diagnostics: encodeDiagnosticsRequest,
};

const DECODERS = {
  capabilities: decodeCapabilitiesReply,
  sessionSnapshot: decodeSessionSnapshotReply,
  transportGet: (bytes) => decodeTransportReply(bytes, "get"),
  transportSet: (bytes) => decodeTransportReply(bytes, "set"),
  telemetryConfigure: decodeTelemetryReply,
  counters: decodeCountersReply,
  diagnostics: decodeDiagnosticsReply,
};

describe("gate 1: the protocol message codec is the protocol codec's", () => {
  test("the counter names are the codec's counter IDs", () => {
    // Red if a counter the core added or retired is missing from, or still in, the SDK's table.
    assert.deepEqual(COUNTER_IDS.map((row) => ({ name: row.name, code: row.code })), FIXTURE.counterIds);
  });

  for (const row of FIXTURE.commands) {
    test(`request ${row.name} encodes to the codec's bytes`, () => {
      assert.equal(hexOf(ENCODERS[row.message](requestOf(row.request))), row.hex);
    });
  }

  for (const row of FIXTURE.replies) {
    test(`reply ${row.name} decodes to the codec's values`, () => {
      assert.deepEqual(plain(DECODERS[row.decoder](bytesOf(row.hex))), row.expect);
    });
  }

  for (const row of FIXTURE.events) {
    test(`event ${row.name} decodes to the codec's values`, () => {
      assert.deepEqual(plain(decodeProtocolEvent(bytesOf(row.hex))), row.expect);
    });
  }

  test("bytes that are not a valid frame raise internal, and a name with no wire code is a usage error", () => {
    const event = bytesOf(FIXTURE.events.find((row) => row.name === "session_committed").hex);
    const truncated = event.slice(0, event.length - 8);
    assert.throws(() => decodeProtocolEvent(truncated), (error) => error instanceof MisoEngineError && error.code === "internal");
    const reply = bytesOf(FIXTURE.replies.find((row) => row.name === "transport_get_stopped").hex);
    // A transport reply read as another command's reply is refused, not misread.
    assert.throws(() => decodeTransportReply(reply, "set"), (error) => error instanceof MisoEngineError && error.code === "internal");
    assert.throws(() => encodeCountersRequest({ requestId: 1n, ids: ["notACounter"] }), MisoUsageError);
  });
});

const fader = (value) => ({
  address: { owner: { kind: "strip", id: "t" }, section: { kind: "builtins" }, control: 5, lane: "both" },
  value,
});

describe("the headless engine's protocol calls", () => {
  let asset;
  before(async () => {
    asset = await MisoEngineAsset.load(await moduleBytes());
  });

  test("gate 2: capabilities, transport, counters and diagnostics answer at the committed revision", async () => {
    // Red if setTransport sends a revision the queue's commit made stale, or if a call decodes the
    // reply through other code than the frame limits the queue read for itself.
    const engine = await createOfflineEngine(sessionDocument(), { asset });
    try {
      const queue = engine.controlTransactions();
      const capabilities = await engine.capabilities();
      assert.equal(Number(capabilities.maximumFrameBytes), queue.maximumFrameBytes);
      assert.equal(capabilities.maximumTransactionEdits, queue.maximumTransactionEdits);
      for (const command of [0x0001, 0x0002, 0x0007, 0x0008, 0x0009, 0x000a, 0x000b]) {
        assert.ok(capabilities.supportedCommands.includes(command), `command ${command} is served`);
      }
      const edited = await queue.submit([fader(-3)]);
      assert.deepEqual(await engine.transport(), { state: "stopped", effectiveSample: 0n });
      const playing = await engine.setTransport({ state: "playing" });
      assert.equal(playing.state, "playing");
      assert.deepEqual(await engine.transport(), playing);
      assert.equal(engine.controlRevision(), edited.revision, "a transport change commits no revision");
      const counters = await engine.counters(["liveValuesSuperseded"]);
      assert.deepEqual(counters.values.map((row) => row.id), ["liveValuesSuperseded"]);
      // Absent IDs ask for every counter the core serves, which is the core's to choose.
      assert.ok((await engine.counters()).values.some((row) => row.id === "liveValuesSuperseded"));
      assert.deepEqual(await engine.diagnostics(), { lastSequence: 0n, eof: true, diagnostics: [] });
      await assert.rejects(engine.setTransport({ state: "playing", position: 0n }), MisoUsageError);
    } finally {
      engine.dispose();
    }
  });

  test("gate 3: every event reaches the listener, decoded, in engine order", async () => {
    // Red if an event is lost (a block's telemetry replaced before it is read), reordered or
    // misdecoded between the core and the listener.
    const engine = await createOfflineEngine(sessionDocument(), { asset });
    try {
      const events = [];
      const stop = engine.onEvent((event) => events.push(event));
      // The browser engine has no meter delivery until #1588 (#1583 D3): its meter table is empty.
      const applied = await engine.configureTelemetry({
        meters: [],
        meterPeriodBlocks: 0,
        counterIds: ["liveValuesSuperseded"],
        counterPeriodBlocks: 1,
        diagnosticsEnabled: true,
        minimumSeverity: "info",
      });
      assert.deepEqual(applied.counterIds, ["liveValuesSuperseded"]);
      const edited = await engine.controlTransactions().submit([fader(-3)]);
      await engine.setTransport({ state: "playing" });
      const quantum = engine.status().quantumFrames;
      for (let block = 0; block < 4; block += 1) engine.render();
      stop();
      const perBlock = ["diagnostic", "counterSnapshot"];
      assert.deepEqual(events.map((event) => event.kind), [
        "sessionCommitted",
        "transportState",
        ...perBlock, ...perBlock, ...perBlock, ...perBlock,
      ]);
      assert.equal(events[0].revision, edited.revision);
      assert.equal(events[0].previousRevision, edited.revision - 1n);
      assert.equal(events[1].state, "playing");
      for (let block = 0; block < 4; block += 1) {
        const [diagnostic, counters] = events.slice(2 + block * 2, 4 + block * 2);
        const sample = BigInt((block + 1) * quantum);
        assert.equal(diagnostic.diagnostic.sampleTime, sample);
        assert.equal(counters.observedSample, sample);
        assert.deepEqual(counters.values.map((row) => row.id), ["liveValuesSuperseded"]);
      }
      assert.equal(engine.discardedControlEvents, 0);
      assert.equal(engine.controlListenerFailures, 0);
    } finally {
      engine.dispose();
    }
  });

  test("gate 4: the committed document reassembles exactly, past 65,536 bytes", async () => {
    // Red if paging drops, repeats or splits a byte: the reference reads the same document in pages
    // of an odd 977 bytes, directly through the control entry.
    const base = JSON.parse(sessionDocument());
    const [track] = base.tracks;
    const [route] = base.routes;
    base.tracks = [];
    base.routes = [];
    for (let index = 0; index < 120; index += 1) {
      const id = `track-with-a-long-identifier-${String(index).padStart(4, "0")}`;
      base.tracks.push({ ...structuredClone(track), id });
      base.routes.push({ ...structuredClone(route), id: `route-${index}`, source: { ...route.source, track_id: id } });
    }
    const engine = await createOfflineEngine(JSON.stringify(base), { asset });
    try {
      const edited = await engine.controlTransactions().submit([{ ...fader(-3), address: { ...fader(-3).address, owner: { kind: "strip", id: base.tracks[7].id } } }]);
      const { revision, document } = await engine.committedDocument();
      assert.equal(revision, edited.revision);
      const bytes = new TextEncoder().encode(document);
      assert.ok(bytes.length > 65_536, `the document is ${bytes.length} bytes`);
      const ids = engine.controlTransactions().requestIds;
      const chunks = [];
      for (let offset = 0n, eof = false; !eof;) {
        const reply = decodeSessionSnapshotReply(engine.controlFrame(encodeSessionSnapshotRequest({
          requestId: ids.next(), expectedRevision: revision, offset, maximumBytes: 977,
        })));
        assert.equal(reply.ok, true);
        chunks.push(reply.value.chunk);
        offset += BigInt(reply.value.chunk.length);
        eof = reply.value.eof;
      }
      assert.equal(hexOf(bytes), hexOf(Buffer.concat(chunks)));
      const parsed = JSON.parse(document);
      assert.equal(parsed.tracks.length, 120);
      assert.equal(parsed.tracks[7].fader.left_db, -3);
    } finally {
      engine.dispose();
    }
  });

  test("#2190: a frame over the core's frame limit is refused typed and never sent", async () => {
    // Red if ProtocolClient sends a frame over `maximumFrameBytes`: the transport stands in for the
    // browser host, which rejects such a frame before the core with a raw `miso.error.v1` record.
    const source = { channels: 2, bitDepth: 24, frames: 48_000, content: `blake3:${"a".repeat(64)}` };
    const base = session({ id: "frame.limit", sampleRateHz: 48_000, quantumFrames: 128 }).source("a", source).output("out");
    const engine = await createOfflineEngine(base.toJson(), { asset });
    try {
      const limit = engine.controlTransactions().maximumFrameBytes;
      const sizes = [];
      const client = new ProtocolClient({
        send: (frame, document) => {
          sizes.push(frame.length);
          if (frame.length > limit) throw Object.freeze({ tag: "miso.error.v1", requestId: 0, result: 1 });
          return engine.controlFrame(frame, document);
        },
        requestIds: engine.controlTransactions().requestIds,
        committedRevision: () => engine.controlRevision(),
      });
      let builder = transaction(base);
      let tx;
      for (let index = 0; tx === undefined || tx.payload.length <= limit; index += 1) {
        builder = builder.upsertTrack(`track-${String(index).padStart(4, "0")}`, { source: "a" });
        tx = builder.build();
      }
      await assert.rejects(client.applyTransaction(tx), (error) => {
        assert.ok(error instanceof MisoControlRefusal, `${error}`);
        assert.equal(error.status, "limit_exceeded");
        assert.equal(error.code, "limit_exceeded");
        assert.equal(error.diagnostics[0].detail, `frame_bytes=${48 + tx.payload.length},maximum_frame_bytes=${limit}`);
        return true;
      });
      assert.ok(sizes.every((size) => size <= limit), "no frame over the limit was sent");
      assert.equal(engine.controlRevision(), 0n, "nothing was committed");
    } finally {
      engine.dispose();
    }
  });

  test("gate 5: a field the core owns is the core's to refuse", async () => {
    // Red if the SDK pre-checks the diagnostics limit instead of sending it: the refusal must be the
    // core's status and code.
    const engine = await createOfflineEngine(sessionDocument(), { asset });
    try {
      await assert.rejects(engine.diagnostics({ limit: 0 }), (error) => {
        assert.ok(error instanceof MisoControlRefusal);
        assert.equal(error.status, "malformed_frame");
        assert.equal(error.code, "protocol.failure");
        return true;
      });
    } finally {
      engine.dispose();
    }
  });
});
