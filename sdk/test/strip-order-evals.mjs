/**
 * #1564 piece C (decision 17, C4; D5): the SDK takes its strip order from the core's control list.
 *
 * The strips of a session, tracks then submixes, are the owner header rows of the core's list.
 * (host-web's `live_control_{track,submix}_{count,id}` exports that once carried them are deleted,
 * #1565B.) The reference is `hosts/host-web/tests/fixtures/control-list.json`, the parity session
 * of #1560 (three tracks and one submix).
 */

import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { before, describe, test } from "node:test";

import { createBrowserStripOrder } from "../src/browser/live-controls.ts";
import { MisoEngineAsset } from "../src/core/asset.ts";
import { WasmBoundary } from "../src/core/boundary.ts";
import { decodeControlReply, encodeControlTransaction } from "../src/core/control-frame.ts";
import {
  decodeControlListPage,
  readControlStripOrder,
  stripOrderOf,
  stripSections,
} from "../src/core/control-list.ts";
import { createOfflineEngine } from "../src/headless/engine.ts";
import { scriptedControl } from "./control-support.mjs";
import { moduleBytes, ramp } from "./support.mjs";

const FIXTURE = JSON.parse(readFileSync(
  new URL("../../hosts/host-web/tests/fixtures/control-list.json", import.meta.url),
  "utf8",
));
const SESSION = JSON.parse(FIXTURE.session);
const TRACKS = SESSION.tracks.map((track) => track.id);
const SUBMIXES = SESSION.submixes.map((submix) => submix.id);
// Four taps: the fixture's dynamic EQ declares one per band (#17 S2).
const OPTIONS = { liveControls: { meterBlocks: 1 } };

let asset;

before(async () => {
  asset = await MisoEngineAsset.load(await moduleBytes());
});

function feed(engine, block) {
  const shape = engine.shape();
  for (const source of shape.sources) {
    engine.submitSource({
      sourceId: source.id,
      generation: 1n,
      startFrame: BigInt(block * shape.quantumFrames),
      planes: Array.from({ length: source.channels }, (_unused, channel) =>
        ramp(shape.quantumFrames, 5 + block * 16 + channel)),
      endOfRegion: false,
    });
  }
}

function useStripOrder(engine) {
  // Shape, meters and observation: every consumer of the strip order on the headless engine.
  const shape = engine.shape();
  assert.deepEqual(shape.tracks, TRACKS);
  assert.deepEqual(shape.submixes, SUBMIXES);
  assert.deepEqual(engine.sessionMap().tracks, TRACKS);
  assert.deepEqual(engine.sessionMap().submixes, SUBMIXES);

  assert.equal(engine.meters(true).ok, true);
  let frame;
  for (let block = 0; block < 12 && frame === undefined; block += 1) {
    feed(engine, block);
    engine.render();
    frame = engine.pollMeters();
  }
  assert.ok(frame !== undefined, "a meter frame completes");
  assert.equal(frame.trackCount, TRACKS.length);
  assert.equal(frame.submixCount, SUBMIXES.length);

  const map = engine.observationMap();
  assert.ok(map.bindings.length > 0, "the session has observation taps");
  const strips = new Set([...TRACKS, ...SUBMIXES]);
  for (const binding of map.bindings) {
    assert.ok(strips.has(binding.trackId), `binding ${binding.effectSlotId} names strip ${binding.trackId}`);
  }
  const first = map.bindings[0];
  const rows = engine.readObservations([{
    trackId: first.trackId, rack: first.rack, effectSlotId: first.effectSlotId, tapId: first.tapIds[0], channels: "both",
  }]);
  assert.equal(rows[0].trackId, first.trackId);
}

describe("#1564C gate 5 -- the strip order is the control list's", () => {
  test("the shipped module boots, meters and observes the session in list order", async () => {
    const engine = await createOfflineEngine(FIXTURE.session, { asset, ...OPTIONS });
    try {
      useStripOrder(engine);
      engine.loadSession(FIXTURE.session, OPTIONS);
      useStripOrder(engine);
    } finally {
      engine.dispose();
    }
  });
});

describe("#1564C D5 -- the strip order follows the structure epoch", () => {
  function advanceEpoch(boundary, value) {
    const frame = encodeControlTransaction({
      requestId: boundary.controlRequestIds.next(),
      expectedRevision: boundary.controlRevision(),
      edits: [{
        address: { owner: { kind: "strip", id: "t3" }, section: { kind: "own" }, control: 1, lane: "both" },
        value,
      }],
    });
    assert.equal(decodeControlReply(boundary.controlFrame(frame)).status, "ok");
  }

  test("the list is read once per epoch and again after the epoch moves", async () => {
    const boundary = await WasmBoundary.boot(asset, new TextEncoder().encode(FIXTURE.session), OPTIONS);
    try {
      const first = boundary.stripOrder();
      assert.deepEqual([...first.tracks], TRACKS);
      assert.deepEqual([...first.submixes], SUBMIXES);
      assert.equal(boundary.stripOrder(), first, "an unchanged epoch returns the order read before");
      assert.equal(boundary.shape().tracks, first.tracks);
      // A live matrix_mode switch changes the descriptors of t3, so the epoch advances. The row
      // holds 0 or 1 at boot; the switch to the other value is the one that changes it.
      advanceEpoch(boundary, 0);
      advanceEpoch(boundary, 1);
      const second = boundary.stripOrder();
      assert.notEqual(second, first, "the list was read again");
      assert.ok(second.structureEpoch > first.structureEpoch);
      assert.deepEqual([...second.tracks], TRACKS);
      assert.equal(boundary.stripOrder(), second);
    } finally {
      boundary.dispose();
    }
  });

  test("the browser reader follows a new epoch and keeps the order of an unchanged one", async () => {
    const strips = { tracks: ["t"], submixes: ["bus"], structureEpoch: 1n };
    const host = { control: scriptedControl({ strips }) };
    const read = createBrowserStripOrder(host);
    const first = await read();
    assert.deepEqual([...first.tracks], ["t"]);
    assert.deepEqual([...first.submixes], ["bus"]);
    // The scripted core's list changes under the same epoch: the cached order stands.
    strips.submixes = ["bus", "aux"];
    assert.equal(await read(), first);
    strips.structureEpoch = 2n;
    const second = await read();
    assert.deepEqual([...second.submixes], ["bus", "aux"]);
    assert.equal(second.structureEpoch, 2n);
  });
});

describe("#2206 -- the strip order reads the list's header rows alone", () => {
  test("every page the core serves the read holds no control row, and the order is the full list's", async () => {
    const engine = await createOfflineEngine(FIXTURE.session, { asset, ...OPTIONS });
    try {
      const full = await engine.controls({ pageLimit: 3 });
      const pages = [];
      const recording = {
        requestIds: engine.controlTransactions().requestIds,
        send: (frame) => {
          const reply = engine.controlFrame(frame);
          pages.push(decodeControlListPage(reply));
          return reply;
        },
      };
      const order = await readControlStripOrder(recording);
      assert.ok(pages.length > 1, "a probe, then the header pages");
      for (const reply of pages) {
        assert.equal(reply.ok, true, "the core accepts the SDK's headers-only request");
        assert.equal(reply.page.controls.length, 0, "a headers-only page carries no control row");
      }
      assert.deepEqual(pages.slice(1).flatMap((reply) => reply.page.headers), full.headers,
        "the header rows of the full list, with their handles, in order");
      const expected = stripOrderOf(full);
      assert.deepEqual([order.tracks, order.submixes, order.effects], [expected.tracks, expected.submixes, expected.effects]);
      assert.deepEqual(stripSections(order), stripSections(expected));
    } finally {
      engine.dispose();
    }
  });
});
