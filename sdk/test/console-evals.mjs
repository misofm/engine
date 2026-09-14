/** Issue #322: one semantic console and one whole-batch contract over both transports. */

import assert from "node:assert/strict";
import { before, describe, test } from "node:test";

import { createBrowserConsole } from "../src/browser/console.ts";
import { EngineConsole } from "../src/core/console.ts";
import { MisoUsageError } from "../src/core/errors.ts";
import { ABI_LAYOUT } from "../src/generated/abi.ts";
import { CATALOG } from "../src/generated/catalog.ts";
import { MisoEngineAsset } from "../src/core/asset.ts";
import { createOfflineEngine } from "../src/headless/engine.ts";
import { effectEntry, moduleBytes, ramp, sessionDocument } from "./support.mjs";

let asset;

before(async () => {
  asset = await MisoEngineAsset.load(await moduleBytes());
});

function compressorDocument() {
  const compressor = CATALOG.effects.find((row) => row.id === "miso.compressor");
  return sessionDocument({
    effects: {
      simd1: [effectEntry(
        "compressor",
        compressor.id,
        compressor.parameters.map((row) => ({
          id: row.id,
          unit: row.unitName,
          value: row.default,
          channel: "both",
        })),
      )],
    },
  });
}

function feed(engine, block) {
  const shape = engine.shape();
  for (const source of shape.sources) {
    engine.submitSource({
      sourceId: source.id,
      generation: 1n,
      startFrame: BigInt(block * shape.quantumFrames),
      planes: Array.from({ length: source.channels }, (_unused, channel) =>
        ramp(shape.quantumFrames, 71 + block * 16 + channel)),
      endOfRegion: false,
    });
  }
}

function encodeBrowserCommands(commands) {
  const records = new Uint8Array(commands.length * ABI_LAYOUT.commandRecord.bytes);
  const view = new DataView(records.buffer);
  const at = (name) => ABI_LAYOUT.commandRecord.fields.find((row) => row.name === name).offset;
  for (const [index, command] of commands.entries()) {
    const base = index * ABI_LAYOUT.commandRecord.bytes;
    view.setUint8(base + at("kind"), command.kind);
    view.setUint8(base + at("rack"), command.rack);
    view.setUint8(base + at("channel"), command.channel);
    view.setUint32(base + at("trackIndex"), command.trackIndex, true);
    view.setUint32(base + at("effectIndex"), command.effectIndex, true);
    view.setUint32(base + at("parameterId"), command.parameterId, true);
    view.setUint32(base + at("smoothingSamples"), command.smoothingSamples, true);
    command.values.forEach((value, valueIndex) => {
      view.setFloat32(base + at("values") + valueIndex * 4, value, true);
    });
  }
  return records;
}

describe("issue 322 -- shared semantic console", () => {
  test("all eleven command kinds are built by name and admitted by live Wasm", async () => {
    const engine = await createOfflineEngine(compressorDocument(), {
      asset,
      console: { commandQueueRecords: 64, meterBlocks: 2, observationTaps: 1 },
    });
    try {
      const console = engine.console();
      const track = console.edit.track("t");
      const compressor = track.effect("simd1", 0, "miso.compressor");
      const first = await console.submit(
        track.pan(-0.5, 0.5, { smoothingSamples: 16 }),
        track.matrix({ ll: 1, lr: 0, rl: 0, rr: 1 }),
        track.faderDb(-3, { channel: "both" }),
        track.mute(false, { channel: "left" }),
        track.solo(true),
        track.trimDb(1.5, { channel: "right" }),
        track.polarityInvert(false, { channel: "both" }),
        compressor.parameter("threshold", -24, { channel: "both" }),
        compressor.bypass(false),
        compressor.observe("Gain Reduction", true, 2),
      );
      assert.equal(first.ok, true);
      assert.equal(first.admitted, 10);
      assert.equal(first.reasonName, "none");
      assert.equal(first.appliedAtSample, 0n);

      feed(engine, 0);
      engine.render();
      const unsubscribe = await console.submit(
        compressor.observe("Gain Reduction", false, 2),
      );
      assert.equal(unsubscribe.ok, true);
      assert.equal(unsubscribe.admitted, 1);
      assert.equal(unsubscribe.appliedAtSample, 128n);

      const kindNames = [
        "pan", "matrix", "faderDb", "mute", "effectParam", "effectBypass",
        "observeSubscribe", "observeUnsubscribe", "solo", "trimDb", "polarityInvert",
      ];
      assert.deepEqual(
        [...ABI_LAYOUT.constants.wireCommandKinds.map((row) => row.name)].sort(),
        [...kindNames].sort(),
        "the semantic methods cover the generated command vocabulary exactly",
      );

      feed(engine, 1);
      engine.render();
      const refused = await console.submit(
        console.edit.track("t").effect("simd1", 99, "miso.compressor").bypass(true),
      );
      assert.equal(refused.ok, false);
      assert.equal(refused.admitted, 0);
      assert.equal(refused.reasonName, "unknownEffect");

      const recovery = await console.submit(track.faderDb(-6));
      assert.equal(recovery.ok, true, "a typed refusal is per request, not terminal");
    } finally {
      engine.dispose();
    }
  });

  test("unknown tracks and numeric domains refuse before transport", async () => {
    let calls = 0;
    const console = new EngineConsole(
      { tracks: ["t"], sources: [], metersAttached: false },
      () => {
        calls += 1;
        throw new Error("must not be called");
      },
    );
    assert.throws(() => console.edit.track("missing"), MisoUsageError);
    assert.throws(() => console.edit.track("t").faderDb(Number.NaN), /finite/);
    assert.throws(() => console.edit.track("t").pan(-2, 0), /at least -1/);
    assert.throws(
      () => console.edit.track("t").effect("simd1", 0, "miso.compressor")
        .parameter("threshold", 100),
      /at most/,
    );
    const compressor = console.edit.track("t").effect("simd1", 0, "miso.compressor");
    assert.throws(
      () => compressor.parameter({ key: "threshold", value: -18, unit: "db" }),
      /unknown field 'unit'/,
    );
    assert.throws(
      () => compressor.parameter({ key: "threshold", value: true }),
      /must be numeric/,
    );
    assert.throws(
      () => compressor.parameter({ key: "threshold", value: Number.NaN }),
      /finite/,
    );
    assert.throws(
      () => compressor.parameter({ key: "threshold", value: 100 }),
      /at most/,
    );
    assert.throws(
      () => compressor.parameter({ key: "lookahead", value: 1 }),
      /not live-updatable/,
    );
    assert.throws(
      () => compressor.parameter({ key: "threshold", value: -18, channel: "diagonal" }),
      /channel must be left, right, or both/,
    );
    assert.throws(
      () => compressor.parameter({ key: "threshold", value: -18, smoothingSamples: 1.5 }),
      /smoothingSamples must be a u32/,
    );
    const delay = console.edit.track("t").effect("dynamic", 0, "miso.delay");
    assert.throws(
      () => delay.parameter({ key: "cross feedback", value: 0.5, channel: "left" }),
      /shared and must address both lanes/,
    );
    assert.equal(calls, 0);
  });

  test("object and positional effect edits share records and change rendered PCM", async () => {
    async function renderAfter(edit) {
      const engine = await createOfflineEngine(compressorDocument(), {
        asset,
        console: { commandQueueRecords: 64 },
      });
      try {
        const shape = engine.shape();
        feed(engine, 0);
        engine.render();
        const console = engine.console();
        const parameter = console.edit.track("t").effect("simd1", 0, "miso.compressor");
        const report = edit === undefined ? undefined : await console.submit(edit(parameter));
        const output = [];
        for (let block = 1; block < 4; block += 1) {
          feed(engine, block);
          output.push(engine.render());
        }
        return { report, output, quantumFrames: shape.quantumFrames };
      } finally {
        engine.dispose();
      }
    }

    const baseline = await renderAfter();
    const object = await renderAfter((parameter) => parameter.parameter({
      key: "threshold",
      value: -80,
      channel: "both",
      smoothingSamples: 0,
    }));
    const positional = await renderAfter((parameter) =>
      parameter.parameter("threshold", -80, { channel: "both", smoothingSamples: 0 }));

    assert.equal(object.report?.ok, true);
    assert.equal(object.report?.admitted, 1);
    assert.equal(object.report?.appliedAtSample, BigInt(object.quantumFrames));
    for (const [index, block] of object.output.entries()) {
      assert.deepEqual([...block.left], [...positional.output[index].left], `left block ${index}`);
      assert.deepEqual([...block.right], [...positional.output[index].right], `right block ${index}`);
    }
    assert.notDeepEqual(
      [...object.output[0].left],
      [...baseline.output[0].left],
      "the live object edit must affect the next rendered block",
    );
    assert.notDeepEqual(
      [...object.output.at(-1).left],
      [...baseline.output.at(-1).left],
      "the live object edit must remain audible after the compressor settles",
    );
  });

  test("browser transport maps the same semantic record and acknowledgement", async () => {
    let request;
    const host = {
      async sessionMap() {
        return {
          tag: "miso.sessionmap.v1",
          requestId: 1,
          result: 0,
          tracks: ["t"],
          sources: [{ id: "s", channels: 2, frames: 4_800n }],
          metersAttached: false,
        };
      },
      async command(value) {
        request = value;
        return {
          tag: "miso.ack.v1",
          requestId: 2,
          result: 0,
          reason: 0,
          rejectedIndex: 0,
          admitted: value.commands.length,
          appliedAtSample: 256n,
          records: new Uint8Array(value.commands.length * 48),
        };
      },
    };
    const console = await createBrowserConsole(host);
    const report = await console.submit(
      console.edit.track("t").faderDb(-6, { channel: "left", smoothingSamples: 32 }),
    );
    assert.equal(report.ok, true);
    assert.equal(report.reasonName, "none");
    assert.equal(report.appliedAtSample, 256n);
    assert.deepEqual(request, {
      commands: [{
        kind: 3,
        rack: 255,
        channel: 0,
        trackIndex: 0,
        effectIndex: 0,
        parameterId: 0,
        smoothingSamples: 32,
        values: [-6, 0, 0, 0],
      }],
    });
  });

  test("browser transport carries the object edit record and actual report", async () => {
    const engine = await createOfflineEngine(compressorDocument(), {
      asset,
      console: { commandQueueRecords: 64 },
    });
    let request;
    const host = {
      async sessionMap() {
        return {
          tag: "miso.sessionmap.v1",
          requestId: 1,
          result: 0,
          ...engine.sessionMap(),
        };
      },
      async command(value) {
        request = value;
        const records = encodeBrowserCommands(value.commands);
        const report = engine.submitCommands(records, value.commands.length);
        return {
          tag: "miso.ack.v1",
          requestId: 2,
          result: report.result,
          reason: report.reason,
          rejectedIndex: report.rejectedIndex,
          admitted: report.admitted,
          appliedAtSample: report.appliedAtSample,
          records,
        };
      },
    };
    try {
      const console = await createBrowserConsole(host);
      const parameter = console.edit.track("t").effect("simd1", 0, "miso.compressor");
      const objectEdit = parameter.parameter({
        key: "threshold",
        value: -18,
        channel: "both",
        smoothingSamples: 64,
      });
      const positionalEdit = parameter.parameter("threshold", -18, {
        channel: "both",
        smoothingSamples: 64,
      });
      assert.deepEqual(objectEdit, positionalEdit, "the overloads normalize to one LaneEdit");
      const report = await console.submit(objectEdit);
      assert.equal(report.ok, true);
      assert.equal(report.reasonName, "none");
      assert.equal(report.appliedAtSample, 0n);
      assert.deepEqual(request, {
        commands: [{
          kind: 5,
          rack: 0,
          channel: 2,
          trackIndex: 0,
          effectIndex: 0,
          parameterId: 1,
          smoothingSamples: 64,
          values: [-18, 0, 0, 0],
        }],
      });
    } finally {
      engine.dispose();
    }
  });

  test("a torn acknowledgement is rejected after, never before, transport answers", async () => {
    let answered = false;
    const console = new EngineConsole(
      { tracks: ["t"], sources: [], metersAttached: false },
      async () => {
        answered = true;
        return {
          ok: true,
          result: 0,
          code: "ok",
          reason: 0,
          reasonName: "none",
          rejectedIndex: 0,
          admitted: 0,
          appliedAtSample: 0n,
        };
      },
    );
    await assert.rejects(
      () => console.submit(console.edit.track("t").faderDb(-6)),
      /violated whole-batch admission/,
    );
    assert.equal(answered, true, "the SDK inspected an acknowledgement only after transport settled");
  });
});
