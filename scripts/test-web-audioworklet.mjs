import assert from "node:assert/strict";
import { pathToFileURL } from "node:url";
const root = new URL("../", import.meta.url);
// Issue #151: the host module is overridable for exactly the reason the worklet module already is
// -- so a red mutation of the shipped host runs this same suite and is required to fail it.
const hostUrl = process.env.MISO_ENGINE_WEB_HOST_TEST_MODULE === undefined
  ? new URL("hosts/host-web/web/miso-engine-v1-audio-worklet-host.js", root)
  : pathToFileURL(process.env.MISO_ENGINE_WEB_HOST_TEST_MODULE);
const workletUrl = process.env.MISO_ENGINE_WEB_WORKLET_TEST_MODULE === undefined
  ? new URL("hosts/host-web/web/miso-engine-v1-audio-worklet.js", root)
  : pathToFileURL(process.env.MISO_ENGINE_WEB_WORKLET_TEST_MODULE);

const limits = Object.freeze({
  sourceRingFrames: 256,
  maximumMemoryBytes: 0n,
  // Issue #137 D1/D2 and #143 D3/D6: the four console words. All zero is "default command-queue
  // depth, no meter observers, no observation capacity, no master designation"; the console tests
  // below override them.
  consoleCommandQueueRecords: 4n,
  consoleMeterBlocks: 2n,
  consoleObservationTaps: 2n,
  consoleMasterTrackPlusOne: 1n,
});

function resourceReport(backend, quantumFrames) {
  return Object.freeze({
    sampleRateHz: 48000,
    quantumFrames,
    backend,
    optionsBytes: 1n,
    statusBytes: 1n,
    sessionDocumentBytes: 1n,
    diagnosticBytes: 1n,
    sourceIdBytes: 1n,
    sourcePcmStagingBytes: 1n,
    outputPcmBytes: 1n,
    bridgeMetadataBytes: 1n,
    bridgeRetainedBytes: 1n,
    largestBridgeAllocationBytes: 1n,
    sourceTotalBytes: 1n,
    sourceOverheadBytes: 1n,
    effectScalarStateBytes: 1n,
    effectScalarScratchBytes: 1n,
    builtinRetainedBytes: 1n,
    graphSessionPlusPlanBytes: 1n,
    graphIncrementalPlanBytes: 1n,
    graphMetadataBytes: 1n,
    graphDelayBytes: 1n,
    largestNamedAllocationBytes: 1n,
    observationRetainedBytes: 1n,
  });
}

function errorResult(promise, result) {
  return promise.then(
    () => assert.fail("expected rejection"),
    (error) => {
      assert.deepEqual(Object.keys(error).sort(), ["requestId", "result", "tag"]);
      assert.equal(error.tag, "miso.error.v1");
      assert.equal(error.result, result);
      return error;
    },
  );
}

async function testMainRealm() {
  const original = {
    fetch: globalThis.fetch,
    AudioWorkletNode: globalThis.AudioWorkletNode,
    validate: WebAssembly.validate,
    compile: WebAssembly.compile,
  };
  const events = [];
  let holdSource = false;
  let holdAll = false;
  const heldAll = [];
  let failSource = false;
  let held = null;
  let readyMutation = null;
  let statusMutation = null;
  let planeMutation = null;
  let commandResult = 0;
  let commandMutation = null;

  class FakePort {
    onmessage = null;
    onmessageerror = null;
    closeCount = 0;

    close() {
      this.closeCount += 1;
    }

    postMessage(message, transfer) {
      const received = structuredClone(message, { transfer });
      events.push(["request", received.tag, received.requestId, transfer.length]);
      const respond = () => {
        let response;
        let responseTransfer = [];
        if (received.tag === "miso.source.v1") {
          response = {
            tag: failSource ? "miso.error.v1" : "miso.ack.v1",
            requestId: received.requestId,
            result: failSource ? 1 : 6,
            planes: received.planes,
          };
          responseTransfer = [...new Set(received.planes.map((plane) => plane.buffer))];
        } else if (received.tag === "miso.status.v1") {
          response = {
            tag: "miso.status.v1", requestId: received.requestId, result: 0, state: 2,
            lastResult: 0, backend: 1, sampleRateHz: 48000, quantumFrames: 64,
            nextAbsoluteSample: 64n, renderedQuanta: 1n, memoryBytes: 65536,
          };
          if (statusMutation !== null) response = statusMutation(response);
        } else if (received.tag === "miso.command.v1") {
          response = {
            tag: "miso.ack.v1",
            requestId: received.requestId,
            result: commandResult,
            reason: commandResult === 0 ? 0 : 8,
            rejectedIndex: 0,
            admitted: commandResult === 0 ? received.count : 0,
            appliedAtSample: 512n,
            records: received.records,
          };
          if (commandMutation !== null) response = commandMutation(response);
          responseTransfer = [received.records.buffer];
        } else if (received.tag === "miso.sessionmap.v1") {
          response = {
            tag: "miso.sessionmap.v1",
            requestId: received.requestId,
            result: 0,
            tracks: ["kick", "snare"],
            sources: [
              { id: "bass", channels: 1, frames: 96000n },
              { id: "drums", channels: 2, frames: 2048n },
            ],
            metersAttached: true,
          };
        } else {
          response = { tag: "miso.ack.v1", requestId: received.requestId, result: 0 };
        }
        if (received.tag === "miso.source.v1" && planeMutation !== null) {
          response = planeMutation(response);
        }
        const delivered = structuredClone(response, { transfer: responseTransfer });
        queueMicrotask(() => this.onmessage?.({ data: delivered }));
      };
      if (holdAll) heldAll.push(respond);
      else if (holdSource && received.tag === "miso.source.v1") held = respond;
      else respond();
    }
  }

  class FakeNode {
    static latest;

    constructor(_context, _name, options) {
      this.port = new FakePort();
      this.onprocessorerror = null;
      this.options = options;
      this.disposeMessages = 0;
      this.disconnectCount = 0;
      FakeNode.latest = this;
      queueMicrotask(() => {
        let data = {
          tag: "miso.ready.v1", requestId: 0, result: 0,
          backend: "simd128",
          resources: resourceReport(1, 64),
          memoryBytes: 65536,
        };
        if (readyMutation !== null) data = readyMutation(data);
        this.port.onmessage?.({ data });
      });
    }

    disconnect() {
      this.disconnectCount += 1;
    }
  }

  globalThis.AudioWorkletNode = FakeNode;
  WebAssembly.validate = () => true;
  globalThis.fetch = async (url) => ({
    ok: true,
    arrayBuffer: async () => new TextEncoder().encode(String(url)).buffer,
  });
  WebAssembly.compile = async (bytes) => {
    const url = new TextDecoder().decode(bytes);
    events.push(["compile", url]);
    return Object.freeze({ url });
  };
  const context = {
    state: "suspended",
    sampleRate: 48000,
    renderQuantumSize: 64,
    audioWorklet: { addModule: async (url) => events.push(["addModule", url]) },
  };
  try {
    const { createMisoAudioWorkletHost } = await import(`${hostUrl.href}?main-test`);
    const host = await createMisoAudioWorkletHost({
      context,
      document: new TextEncoder().encode("{\"schema_version\":0}"),
      options: limits,
      simd128ModuleUrl: "simd.wasm",
      workletModuleUrl: "processor.js",
    });
    assert.equal(host.backend, "simd128", "W4-D1 ships exactly one artifact");
    assert.equal(host.memoryBytes, 65536);
    assert.equal(Object.getOwnPropertyDescriptor(host, "memoryBytes").writable, false);
    assert.deepEqual(events.slice(0, 2), [
      ["compile", "simd.wasm"], ["addModule", "processor.js"],
    ]);

    const storage = new ArrayBuffer(32);
    const left = new Float32Array(storage, 0, 2);
    const right = new Float32Array(storage, 16, 2);
    left.set([1, 2]);
    right.set([3, 4]);
    holdSource = true;
    const sourcePromise = host.submitSource({
      requestId: 1, sourceId: "source", generation: 1n, startFrame: 0n,
      sampleRateHz: 48000, planes: [left, right], frames: 2, endOfRegion: false,
    });
    assert.equal(storage.byteLength, 0, "postMessage transfers caller ownership");
    // #106 F3: a held source chunk no longer blocks unrelated requests. Before the bounded
    // pipeline this rejected with RESULT_BACKPRESSURE because the host allowed one request of any
    // kind in flight.
    const heldStatus = host.status();
    held();
    holdSource = false;
    assert.equal((await heldStatus).tag, "miso.status.v1");
    const ack = await sourcePromise;
    assert.deepEqual(Object.keys(ack).sort(), ["planes", "requestId", "result", "tag"]);
    assert.equal(ack.tag, "miso.ack.v1");
    assert.equal(ack.result, 6, "engine backpressure remains a resolved ACK");
    assert.equal(ack.planes[0].byteOffset, 0);
    assert.equal(ack.planes[1].byteOffset, 16);
    assert.equal(ack.planes[0].buffer, ack.planes[1].buffer);
    assert.equal(events.find((event) => event[1] === "miso.source.v1")[3], 1);

    // Request 2 was consumed by the status above, which now settles independently.
    const seek = await host.seekSource({
      requestId: 3, sourceId: "source", generation: 2n, sourceFrame: 10n,
    });
    assert.deepEqual(seek, { tag: "miso.ack.v1", requestId: 3, result: 0 });
    const status = await host.status();
    assert.equal(status.tag, "miso.status.v1");
    assert.equal(status.memoryBytes, host.memoryBytes);
    failSource = true;
    const failedStorage = new ArrayBuffer(16);
    const failedSource = host.submitSource({
      requestId: 5, sourceId: "source", generation: 3n, startFrame: 2n,
      sampleRateHz: 48000, planes: [new Float32Array(failedStorage)], frames: 4,
      endOfRegion: true,
    });
    assert.equal(failedStorage.byteLength, 0);
    const returnedError = await failedSource.then(
      () => assert.fail("expected transferred processor error"),
      (error) => error,
    );
    assert.deepEqual(Object.keys(returnedError).sort(), ["planes", "requestId", "result", "tag"]);
    assert.equal(returnedError.tag, "miso.error.v1");
    assert.equal(returnedError.planes[0].byteLength, 16);
    await host.dispose();
    await host.dispose();
    const disposeEvents = events.filter((event) => event[1] === "miso.dispose.v1");
    assert.equal(disposeEvents.length, 1, "settled disposal is idempotent");
    assert.equal(FakeNode.latest.port.closeCount, 1);
    assert.equal(FakeNode.latest.disconnectCount, 1);

    readyMutation = (ready) => ({
      ...ready,
      resources: { ...ready.resources, backend: 0 },
    });
    await errorResult(createMisoAudioWorkletHost({
      context,
      document: new Uint8Array(),
      options: limits,
      simd128ModuleUrl: "simd.wasm",
      workletModuleUrl: "processor.js",
    }), 255);
    assert.equal(FakeNode.latest.port.closeCount, 1, "creation rejection closes the port");
    assert.equal(FakeNode.latest.disconnectCount, 1, "creation rejection disconnects the node");
    readyMutation = null;

    const schemaHost = await createMisoAudioWorkletHost({
      context,
      document: new Uint8Array(),
      options: limits,
      simd128ModuleUrl: "simd.wasm",
      workletModuleUrl: "processor.js",
    });
    statusMutation = (statusValue) => ({ ...statusValue, memoryBytes: 65537 });
    await errorResult(schemaHost.status(), 255);
    statusMutation = null;
    await schemaHost.dispose();

    const planeHost = await createMisoAudioWorkletHost({
      context,
      document: new Uint8Array(),
      options: limits,
      simd128ModuleUrl: "simd.wasm",
      workletModuleUrl: "processor.js",
    });
    failSource = false;
    planeMutation = (response) => ({
      ...response,
      planes: [new Float32Array(response.planes[0].buffer, 4, 3)],
    });
    const malformedStorage = new ArrayBuffer(16);
    await errorResult(planeHost.submitSource({
      requestId: 1, sourceId: "source", generation: 1n, startFrame: 0n,
      sampleRateHz: 48000, planes: [new Float32Array(malformedStorage)], frames: 4,
      endOfRegion: false,
    }), 255);
    planeMutation = null;
    await planeHost.dispose();

    // #106 F3: the in-flight bound is per source and equals the ring depth in quanta
    // (sourceRingFrames 256 / quantumFrames 64 = 4). Red mutation: return `true` from
    // `#saturated` once one source request is unsettled -> the second request rejects with 6.
    const pipelineHost = await createMisoAudioWorkletHost({
      context,
      document: new Uint8Array(),
      options: limits,
      simd128ModuleUrl: "simd.wasm",
      workletModuleUrl: "processor.js",
    });
    failSource = false;
    holdAll = true;
    const chunk = (requestId, sourceId) => {
      const buffer = new ArrayBuffer(8);
      return {
        request: pipelineHost.submitSource({
          requestId,
          sourceId,
          generation: 1n,
          startFrame: BigInt(requestId),
          sampleRateHz: 48000,
          planes: [new Float32Array(buffer)],
          frames: 2,
          endOfRegion: false,
        }),
        buffer,
      };
    };
    const inFlight = [1, 2, 3, 4].map((requestId) => chunk(requestId, "source"));
    for (const [index, entry] of inFlight.entries()) {
      assert.equal(entry.buffer.byteLength, 0, `chunk ${index} was transferred`);
    }
    const overflow = chunk(5, "source");
    await errorResult(overflow.request, 6);
    assert.equal(
      overflow.buffer.byteLength,
      8,
      "a locally refused chunk keeps its planes: nothing is transferred and the caller can retry",
    );
    // A different source has its own budget and is accepted while the first is saturated.
    const other = chunk(6, "other-source");
    assert.equal(other.buffer.byteLength, 0, "the bound is per source, not per host");
    // One unsettled seek per source: the ring carries a single command slot.
    const firstSeek = pipelineHost.seekSource({
      requestId: 7, sourceId: "source", generation: 2n, sourceFrame: 0n,
    });
    await errorResult(pipelineHost.seekSource({
      requestId: 8, sourceId: "source", generation: 3n, sourceFrame: 0n,
    }), 6);
    holdAll = false;
    for (const respond of heldAll) respond();
    heldAll.length = 0;
    const settled = await Promise.all([
      ...inFlight.map((entry) => entry.request),
      other.request,
      firstSeek,
    ]);
    assert.deepEqual(
      settled.map((message) => message.requestId),
      [1, 2, 3, 4, 6, 7],
      "acknowledgements arrive in request order",
    );
    // With the budget released the source accepts chunks again.
    const again = chunk(9, "source");
    assert.equal(again.buffer.byteLength, 0);
    await again.request;
    await pipelineHost.dispose();

    // W4-D1: a browser that cannot validate simd128 is refused with the typed record, before any
    // network request. Red mutation: delete the `WebAssembly.validate(SIMD128_PROBE)` guard in
    // `createMisoAudioWorkletHost` -> the rejection becomes a generic 255 and `compileCount` grows.
    const compileCount = events.filter((event) => event[0] === "compile").length;
    WebAssembly.validate = () => false;
    const refusal = await createMisoAudioWorkletHost({
      context,
      document: new Uint8Array(),
      options: limits,
      simd128ModuleUrl: "simd.wasm",
      workletModuleUrl: "processor.js",
    }).then(() => assert.fail("expected unsupported-browser refusal"), (error) => error);
    WebAssembly.validate = () => true;
    assert.deepEqual(
      Object.keys(refusal).sort(),
      ["capability", "requestId", "result", "tag"],
    );
    assert.equal(refusal.tag, "miso.unsupported.v1");
    assert.equal(refusal.capability, "simd128");
    assert.equal(refusal.result, 7);
    assert.equal(refusal.requestId, 0);
    assert.equal(Object.isFrozen(refusal), true);
    assert.equal(
      events.filter((event) => event[0] === "compile").length,
      compileCount,
      "the probe refuses before any artifact is fetched",
    );

    // Issue #137 D1/D2/D3: the live console's main-realm half.
    const consoleHost = await createMisoAudioWorkletHost({
      context,
      document: new TextEncoder().encode("{\"schema_version\":0}"),
      options: limits,
      simd128ModuleUrl: "simd.wasm",
      workletModuleUrl: "processor.js",
    });
    const map = await consoleHost.sessionMap();
    assert.deepEqual(map.tracks, ["kick", "snare"], "the canonical track order is the ABI");
    // Issue #241: the source list crosses the port with its `bigint` frame count intact, and the host's
    // acknowledgement validator accepted it -- a malformed row fails the whole host with 255, so
    // reaching this line is itself the assertion that the shape is the declared one.
    assert.deepEqual(map.sources, [
      { id: "bass", channels: 1, frames: 96000n },
      { id: "drums", channels: 2, frames: 2048n },
    ], "the canonical source order and shape are the ABI");
    assert.equal(map.metersAttached, true);

    const pan = {
      kind: 1, rack: 255, channel: 255, trackIndex: 1, effectIndex: 0, parameterId: 0,
      smoothingSamples: 64, values: [-0.5, 0.5, 0, 0],
    };
    const commandAck = await consoleHost.command({ requestId: 200, commands: [pan] });
    assert.equal(commandAck.tag, "miso.ack.v1");
    assert.equal(commandAck.result, 0);
    assert.equal(commandAck.admitted, 1);
    assert.equal(commandAck.appliedAtSample, 512n, "the ack names the exact application sample");
    const staged = events.findLast((event) => event[1] === "miso.command.v1");
    assert.equal(staged[3], 1, "the record block is transferred, never copied");
    assert.equal(commandAck.records.byteLength, 48, "the block comes straight back to the caller");
    const decoded = new DataView(
      commandAck.records.buffer,
      commandAck.records.byteOffset,
      commandAck.records.byteLength,
    );
    assert.equal(decoded.getUint8(0), 1, "kind");
    assert.equal(decoded.getUint8(1), 255, "rack is not applicable to a pan");
    assert.equal(decoded.getUint32(4, true), 1, "track index");
    assert.equal(decoded.getUint32(16, true), 64, "smoothing samples");
    assert.equal(decoded.getFloat32(24, true), -0.5, "value0");
    assert.equal(decoded.getFloat32(28, true), 0.5, "value1");

    // A malformed command never reaches the port.
    const beforeMalformed = events.length;
    await errorResult(
      consoleHost.command({ requestId: 201, commands: [{ ...pan, kind: 99 }] }),
      1,
    );
    await errorResult(
      consoleHost.command({ requestId: 202, commands: [{ ...pan, values: [0, 0, 0, NaN] }] }),
      1,
    );
    await errorResult(consoleHost.command({ requestId: 203, commands: [] }), 1);
    assert.equal(events.length, beforeMalformed, "a malformed batch costs no message");

    // Engine backpressure is a resolved acknowledgement that admits nothing.
    commandResult = 6;
    const refused = await consoleHost.command({ requestId: 204, commands: [pan] });
    assert.equal(refused.result, 6);
    assert.equal(refused.admitted, 0);
    assert.equal(refused.reason, 8);
    commandResult = 0;

    // Local backpressure: the worklet-side queue depth is 4, so a fifth unsettled batch is
    // refused here, before any transfer, and the caller keeps its records.
    holdAll = true;
    const held4 = [205, 206, 207, 208].map((requestId) =>
      consoleHost.command({ requestId, commands: [pan] }));
    await errorResult(consoleHost.command({ requestId: 209, commands: [pan] }), 6);
    holdAll = false;
    for (const respond of heldAll) respond();
    heldAll.length = 0;
    await Promise.all(held4);

    // Leases and their unsolicited frames.
    const meterFrames = [];
    const telemetryFrames = [];
    assert.equal(
      (await consoleHost.meters({ requestId: 210, enabled: true, onFrame: (frame) => meterFrames.push(frame) })).result,
      0,
    );
    assert.equal(
      (await consoleHost.telemetry({ requestId: 211, enabled: true, onFrame: (frame) => telemetryFrames.push(frame) })).result,
      0,
    );
    const node = FakeNode.latest;
    node.port.onmessage({
      data: {
        tag: "miso.meter.v1", sequence: 1, windows: 1, trackCount: 2,
        peaks: new Float32Array([0.125, 0.25, 0.375, 0.5, 0.625, 0.75]),
        trackGrDb: new Float32Array([6.5, 0]), masterGrDb: 6.5,
        firstSample: 512n, endSample: 768n,
      },
    });
    node.port.onmessage({
      data: {
        tag: "miso.telemetry.v1", sequence: 1, blocks: 128, cpuPercent: 4.5, peakBlockMs: 0.2,
        meanBlockMs: 0.1, budgetMs: 1.3, deadlineMisses: 0, resolutionMs: 0.005,
        belowResolution: false,
      },
    });
    assert.equal(meterFrames.length, 1);
    assert.deepEqual([...meterFrames[0].peaks], [0.125, 0.25, 0.375, 0.5, 0.625, 0.75]);
    assert.equal(Object.isFrozen(meterFrames[0]), true);

    // Issue #143 E4: the frame as an **app** reads it.
    //
    // `appGainReduction` is the app's own fold, copied verbatim from `meters.ts`'s
    // `Math.max(0, x ?? 0)` lines. The whole point of the declared `peakMagnitude` fold is that
    // this line is a *no-op*: the engine publishes a non-negative magnitude, so clamping at zero
    // changes nothing. Publish the raw negative decibels instead and every one of these becomes
    // `0` -- the exact dead-meter bug -- which is what the red mutation demonstrates.
    const appGainReduction = (frame) => ({
      tracks: Array.from({ length: frame.trackCount }, (_, index) =>
        Math.max(0, frame.trackGrDb[index] ?? 0)),
      master: Math.max(0, frame.masterGrDb ?? 0),
    });
    const ingested = appGainReduction(meterFrames[0]);
    assert.equal(ingested.tracks.length, meterFrames[0].trackCount);
    assert.equal(ingested.tracks[0], 6.5, "trackGrDb(0) > 0 survives the app's clamp unchanged");
    assert(ingested.tracks[0] > 0);
    assert.equal(ingested.tracks[1], 0, "a track with no observed effect reads exactly zero");
    assert.equal(ingested.master, 6.5);
    assert.equal(meterFrames[0].endSample - meterFrames[0].firstSample, 256n);

    // Every shape rule is a hard failure, not a silent skip. Each entry is one red mutation of
    // one rule in the `miso.meter.v1` branch of `#receive`.
    for (const broken of [
      { trackGrDb: new Float32Array(1) },
      { trackGrDb: [0, 0] },
      { trackGrDb: new Float32Array([-6.5, 0]) },
      { trackGrDb: new Float32Array([Number.NaN, 0]) },
      { masterGrDb: -1 },
      { masterGrDb: "6.5" },
      { firstSample: 512 },
      { endSample: 256n },
    ]) {
      const rejecting = await createMisoAudioWorkletHost({
        context,
        document: new TextEncoder().encode("{\"schema_version\":0}"),
        options: limits,
        simd128ModuleUrl: "simd.wasm",
        workletModuleUrl: "processor.js",
      });
      const rejected = rejecting.status();
      FakeNode.latest.port.onmessage({
        data: {
          tag: "miso.meter.v1", sequence: 1, windows: 1, trackCount: 2,
          peaks: new Float32Array(6), trackGrDb: new Float32Array(2), masterGrDb: null,
          firstSample: 512n, endSample: 768n, ...broken,
        },
      });
      await errorResult(rejected, 255);
    }
    assert.equal(telemetryFrames.length, 1);
    assert.equal(telemetryFrames[0].cpuPercent, 4.5);

    // Issue #143: the `miso.observe.v1` acknowledgement's subscription map.
    //
    // The map is the answer to what `trackGrDb` cannot say -- which tracks have an observed effect
    // at all -- so it is checked as carefully as the frame: the exact wire kinds go out, the map
    // is canonically ordered, `windowBlocks: 0` resolves to the plan default, and an unsubscribe
    // removes exactly one entry.
    const observeAck = await consoleHost.observe({
      requestId: 213,
      subscriptions: [
        { trackIndex: 1, rack: 1, effectIndex: 0, tapId: 1, windowBlocks: 0, armed: true },
        { trackIndex: 0, rack: 1, effectIndex: 0, tapId: 1, windowBlocks: 8, armed: true },
      ],
    });
    assert.equal(observeAck.tag, "miso.observe.v1");
    assert.equal(observeAck.result, 0);
    assert.deepEqual(observeAck.bindings.map((binding) => binding.trackIndex), [0, 1],
      "the map is canonically ordered by (track, rack, effectIndex, tapId)");
    assert.deepEqual(observeAck.bindings[0], {
      trackIndex: 0, rack: 1, effectIndex: 0, tapId: 1, frameSlot: 0, windowBlocks: 8,
    });
    assert.equal(observeAck.bindings[1].windowBlocks, Number(limits.consoleMeterBlocks),
      "`windowBlocks: 0` resolves to the plan default, and the map says which one it got");
    assert.equal(Object.isFrozen(observeAck.bindings), true);
    const unsubscribed = await consoleHost.observe({
      requestId: 214,
      subscriptions: [
        { trackIndex: 1, rack: 1, effectIndex: 0, tapId: 1, windowBlocks: 0, armed: false },
      ],
    });
    assert.equal(unsubscribed.bindings.length, 1, "an unsubscribe removes exactly one entry");
    assert.equal(unsubscribed.bindings[0].trackIndex, 0);
    for (const broken of [
      { trackIndex: -1 }, { rack: 3 }, { tapId: 0 }, { armed: "yes" }, { windowBlocks: -1 },
    ]) {
      await errorResult(consoleHost.observe({
        requestId: 215,
        subscriptions: [{
          trackIndex: 0, rack: 1, effectIndex: 0, tapId: 1, windowBlocks: 0, armed: true,
          ...broken,
        }],
      }), 1);
    }
    await errorResult(consoleHost.observe({ requestId: 216, subscriptions: [] }), 1);

    // Issue #143's two reasons, and the #151 defect they exposed: a refused subscription is a
    // typed *per-request* rejection and costs the host nothing.
    //
    // Reasons 10 (`unknownTap`) and 11 (`observationUnbound`) are the only two the observation
    // path ever returns, and `#receive` used to bound `reason` at the literal `<= 9`. Either one
    // therefore read as a malformed acknowledgement and tripped the sticky 255 that fails every
    // unsettled request and every later one -- so one refused subscription cost the whole
    // session. That is what kept the app's gain-reduction meters dead: the app arms its taps
    // once at startup, and a single refusal took the console with it.
    //
    // Red mutation: restore `validU32(message.reason) && message.reason <= 9` in `#receive` ->
    // every assertion below fails with the sticky signature, starting with `refused.tag` because
    // the promise rejects with `{tag: "miso.error.v1", result: 255}` instead of settling.
    // `scripts/test-web-audioworklet.sh` runs exactly that mutation and requires this file red.
    let refusalRequestId = 240;
    const nextRequestId = () => (refusalRequestId += 10);
    const mapBeforeRefusal = unsubscribed.bindings;
    for (const { reason, result, what } of [
      // The address resolves and the tap id does not. A bad address, like every other unknown, so
      // `RESULT_INVALID_ARGUMENT`.
      { reason: 10, result: 1, what: "an undeclared tap" },
      // The tap is declared and correctly addressed; this preparation bound no observation
      // capacity, which is what `RESULT_UNSUPPORTED` means. Retrying will never help.
      { reason: 11, result: 7, what: "a session that bound no observation capacity" },
    ]) {
      commandResult = result;
      commandMutation = (response) => ({ ...response, reason, rejectedIndex: 0, admitted: 0 });
      const refused = await consoleHost.observe({
        requestId: nextRequestId(),
        subscriptions: [
          { trackIndex: 0, rack: 1, effectIndex: 0, tapId: 9, windowBlocks: 0, armed: true },
        ],
      });
      commandMutation = null;
      commandResult = 0;
      assert.equal(refused.tag, "miso.observe.v1", `${what} settles as a typed observation ack`);
      assert.equal(refused.result, result, `${what} carries its own result code`);
      assert.equal(refused.reason, reason, "the ack names which namespace the caller got wrong");
      assert.equal(Object.isFrozen(refused), true);
      assert.deepEqual(
        refused.bindings,
        mapBeforeRefusal,
        "a batch is all-or-nothing: a refused batch arms nothing and leaves the map untouched",
      );

      // The host is fully healthy afterwards. Each of these is a request the sticky error would
      // have failed with `{tag: "miso.error.v1", result: 255}`.
      assert.equal((await consoleHost.status()).result, 0, `${what}: status still answers`);
      const laterCommand = await consoleHost.command({
        requestId: nextRequestId(), commands: [pan],
      });
      assert.equal(laterCommand.result, 0, `${what}: the command path still admits a batch`);
      assert.equal(laterCommand.admitted, 1);
      assert.deepEqual(
        (await consoleHost.sessionMap()).tracks,
        ["kick", "snare"],
        `${what}: the addressing authority still answers`,
      );
      const framesBefore = meterFrames.length;
      node.port.onmessage({
        data: {
          tag: "miso.meter.v1", sequence: 10 + reason, windows: 1, trackCount: 2,
          peaks: new Float32Array([0.5, 0.5, 0.5, 0.5, 0.5, 0.5]),
          trackGrDb: new Float32Array([3.25, 0]), masterGrDb: 3.25,
          firstSample: 1024n, endSample: 1280n,
        },
      });
      assert.equal(
        meterFrames.length,
        framesBefore + 1,
        `${what}: the meter lease still delivers -- this is the dead-GR-meter regression`,
      );
      assert.equal(meterFrames.at(-1).trackGrDb[0], 3.25);

      // And a *correct* subscription still arms, so nothing about the map machinery was poisoned.
      const recovered = await consoleHost.observe({
        requestId: nextRequestId(),
        subscriptions: [
          { trackIndex: 1, rack: 1, effectIndex: 0, tapId: 1, windowBlocks: 4, armed: true },
        ],
      });
      assert.equal(recovered.result, 0, `${what}: a correct subscription still arms after it`);
      assert.equal(recovered.reason, 0);
      assert.deepEqual(recovered.bindings.map((binding) => binding.trackIndex), [0, 1]);
      const undo = await consoleHost.observe({
        requestId: nextRequestId(),
        subscriptions: [
          { trackIndex: 1, rack: 1, effectIndex: 0, tapId: 1, windowBlocks: 4, armed: false },
        ],
      });
      assert.deepEqual(undo.bindings, mapBeforeRefusal, "the map returns to where it started");
    }

    // A released lease detaches the callback: a late frame is delivered nowhere.
    const framesAtRelease = meterFrames.length;
    await consoleHost.meters({ requestId: nextRequestId(), enabled: false, onFrame: null });
    node.port.onmessage({
      data: {
        tag: "miso.meter.v1", sequence: 2, windows: 1, trackCount: 2,
        peaks: new Float32Array(6), trackGrDb: new Float32Array(2), masterGrDb: null,
        firstSample: 768n, endSample: 1024n,
      },
    });
    assert.equal(meterFrames.length, framesAtRelease, "a released lease receives nothing");

    // A malformed frame is a hard failure, not a silent skip: a console that ignores a broken
    // frame is a console that lies to its user. Red mutation: replace the `#fail` in the
    // `miso.meter.v1` branch of `#receive` with `return` -> this status stays pending forever.
    const doomed = consoleHost.status();
    node.port.onmessage({
      data: {
        tag: "miso.meter.v1", sequence: 3, windows: 1, trackCount: 2,
        peaks: new Float32Array(4), trackGrDb: new Float32Array(2), masterGrDb: null,
        firstSample: 0n, endSample: 0n,
      },
    });
    await errorResult(doomed, 255);
    await consoleHost.dispose();

    // Issue #143 D7 / #151: recompile and re-subscribe, the way the app re-arms after the plan is
    // replaced.
    //
    // A structural session edit produces a replacement plan, and in the browser that is a new host
    // over the new session JSON. Subscriptions belong to the plan they were applied to: the
    // replacement's lanes exist and are unarmed, nothing carries over, and the app re-arms from
    // scratch against the new addressing. This is also the moment the #151 defect was most likely
    // to fire in the field, because a replacement moves effect indices -- a stale tap address
    // comes back as reason 10, and before the fix that killed the freshly prepared host on its
    // first gesture.
    const prepare = (sessionDocument) => createMisoAudioWorkletHost({
      context,
      document: new TextEncoder().encode(sessionDocument),
      options: limits,
      simd128ModuleUrl: "simd.wasm",
      workletModuleUrl: "processor.js",
    });
    const tap = (trackIndex, effectIndex, armed) => ({
      trackIndex, rack: 1, effectIndex, tapId: 1, windowBlocks: 0, armed,
    });

    const beforeEdit = await prepare("{\"schema_version\":0}");
    const armedBefore = await beforeEdit.observe({
      requestId: 1, subscriptions: [tap(0, 0, true), tap(1, 0, true)],
    });
    assert.equal(armedBefore.result, 0);
    assert.deepEqual(armedBefore.bindings.map((binding) => binding.trackIndex), [0, 1]);
    await beforeEdit.dispose();

    const afterEdit = await prepare("format_version = 0 # one effect inserted");
    // The replacement's map starts empty: request ids restart at 1 and nothing carried over.
    // The app's first re-arm uses the *old* effect index, which the replacement no longer has.
    commandResult = 1;
    commandMutation = (response) => ({ ...response, reason: 10, rejectedIndex: 0, admitted: 0 });
    const staleRearm = await afterEdit.observe({
      requestId: 1, subscriptions: [tap(0, 0, true), tap(1, 0, true)],
    });
    commandMutation = null;
    commandResult = 0;
    assert.equal(staleRearm.tag, "miso.observe.v1");
    assert.equal(staleRearm.reason, 10, "a stale tap address is refused, per request");
    assert.deepEqual(staleRearm.bindings, [], "the replacement plan's map is still empty");

    // The app reads the new addressing and re-arms. The host was never sticky, so this is a plain
    // second call rather than a rebuild.
    assert.deepEqual((await afterEdit.sessionMap()).tracks, ["kick", "snare"]);
    const rearmed = await afterEdit.observe({
      requestId: 3, subscriptions: [tap(0, 1, true), tap(1, 1, true)],
    });
    assert.equal(rearmed.result, 0, "the replacement plan re-arms after the refusal");
    assert.deepEqual(rearmed.bindings.map((binding) => binding.effectIndex), [1, 1],
      "the map holds the replacement's addressing, not the retired plan's");
    assert.deepEqual(rearmed.bindings.map((binding) => binding.windowBlocks),
      [Number(limits.consoleMeterBlocks), Number(limits.consoleMeterBlocks)],
      "`windowBlocks: 0` resolves against the replacement's own default");

    // The rest of the console is live on the replacement.
    const rearmedFrames = [];
    assert.equal(
      (await afterEdit.meters({
        requestId: 4, enabled: true, onFrame: (frame) => rearmedFrames.push(frame),
      })).result,
      0,
    );
    FakeNode.latest.port.onmessage({
      data: {
        tag: "miso.meter.v1", sequence: 1, windows: 1, trackCount: 2,
        peaks: new Float32Array(6), trackGrDb: new Float32Array([1.5, 2.5]), masterGrDb: 2.5,
        firstSample: 0n, endSample: 256n,
      },
    });
    assert.equal(rearmedFrames.length, 1, "the replacement's meter sequence restarts at 1");
    assert.deepEqual([...rearmedFrames[0].trackGrDb], [1.5, 2.5]);
    assert.equal((await afterEdit.command({ requestId: 5, commands: [pan] })).result, 0);
    assert.equal((await afterEdit.status()).result, 0);
    await afterEdit.dispose();
  } finally {
    globalThis.fetch = original.fetch;
    globalThis.AudioWorkletNode = original.AudioWorkletNode;
    WebAssembly.validate = original.validate;
    WebAssembly.compile = original.compile;
  }
}

function createFakeExports(quantum, backend = 1) {
  const memory = { buffer: new ArrayBuffer(65536) };
  const statusPointer = 16384;
  const resourcePointer = 17000;
  const status = new DataView(memory.buffer, statusPointer, 80);
  status.setUint32(0, 80, true);
  status.setUint32(4, 0x00010000, true);
  status.setUint32(8, 2, true);
  status.setUint32(16, backend, true);
  status.setUint32(20, 48000, true);
  status.setUint32(24, quantum, true);
  const resources = new DataView(memory.buffer, resourcePointer, 224);
  resources.setUint32(0, 224, true);
  resources.setUint32(4, 0x00010000, true);
  resources.setUint32(8, 48000, true);
  resources.setUint32(12, quantum, true);
  resources.setUint32(16, backend, true);
  // Twenty-one u64 rows now: issue #143 carved `observationRetainedBytes` from the first of the
  // report's four reserved words, so offset 192 is a real row and 200..216 stay required zero.
  for (let index = 0; index < 21; index += 1) resources.setBigUint64(32 + index * 8, 1n, true);
  const calls = {
    render: [], source: [], sourceIdBytes: [], seek: [], seekIdBytes: [], dispose: 0,
    sourceResult: 0, bootResult: 0,
  };
  // Issue #137: command staging (kind 6), the meter frame (kind 7) and the command report.
  const commandPointer = 24000;
  const meterFramePointer = 40000;
  const reportPointer = 41000;
  const meterHeaderPointer = 41100;
  const trackIds = ["kick", "snare"];
  // Issue #241: the compiled session's sources, in canonical (stable-ID sorted) order. Channel and
  // frame count differ between rows, so a worklet that reads the wrong row/query is visible here.
  const sourceRows = [
    { id: "bass", channels: 1, frames: 96000n },
    { id: "drums", channels: 2, frames: 2048n },
  ];
  // Issue #143 D5: `3T + 3` -- the frozen `2T + 2` peak section, then one gain-reduction magnitude
  // per track and the master's.
  const meterFrameFloats = trackIds.length * 3 + 3;
  const meterHeader = new DataView(memory.buffer, meterHeaderPointer, 64);
  meterHeader.setUint32(0, 64, true);
  meterHeader.setUint32(4, 0x00010000, true);
  meterHeader.setUint32(8, trackIds.length, true);
  meterHeader.setUint32(40, 1, true);
  meterHeader.setUint32(44, 1, true);
  meterHeader.setBigUint64(16, 512n, true);
  meterHeader.setBigUint64(24, 768n, true);
  const report = new DataView(memory.buffer, reportPointer, 48);
  report.setUint32(0, 48, true);
  report.setUint32(4, 0x00010000, true);
  const pointers = {
    2: 4096, 3: 5000, 5: 8192, 6: commandPointer, 7: meterFramePointer,
  };
  const capacities = {
    2: 64, 3: 2 * quantum * 4, 5: 2 * quantum * 4, 6: 256 * 48,
    7: meterFrameFloats * 4,
  };
  calls.commands = [];
  calls.commandResult = 0;
  calls.meterLease = [];
  calls.meterWindows = 0;
  const exports = {
    memory,
    miso_engine_web_v1_abi_version: () => 0x00010000,
    miso_engine_web_v1_boot_options_ptr: () => 512,
    miso_engine_web_v1_document_ptr: () => 2048,
    miso_engine_web_v1_boot: () => {
      const options = new DataView(memory.buffer, 512, 64);
      if (options.getUint32(8, true) !== status.getUint32(20, true)
          || options.getUint32(12, true) !== status.getUint32(24, true)) {
        calls.bootResult = 9;
        return 0;
      }
      calls.bootResult = 0;
      return 1;
    },
    miso_engine_web_v1_boot_result: () => calls.bootResult,
    miso_engine_web_v1_boot_diagnostic_bytes: () => 0,
    miso_engine_web_v1_buffer_ptr: (_handle, kind) => pointers[kind] ?? 0,
    miso_engine_web_v1_buffer_capacity: (_handle, kind) => capacities[kind] ?? 0,
    miso_engine_web_v1_status_ptr: () => statusPointer,
    miso_engine_web_v1_resource_ptr: () => resourcePointer,
    miso_engine_web_v1_render: (_handle, actualFrames) => {
      calls.render.push(actualFrames);
      if (actualFrames !== quantum) return 9;
      new Float32Array(memory.buffer, 8192, quantum).fill(0.25);
      new Float32Array(memory.buffer, 8192 + quantum * 4, quantum).fill(-0.25);
      const quanta = status.getBigUint64(40, true) + 1n;
      status.setBigUint64(40, quanta, true);
      status.setBigUint64(32, quanta * BigInt(quantum), true);
      return 0;
    },
    miso_engine_web_v1_source_submit: (...args) => {
      calls.source.push(args);
      calls.sourceIdBytes.push(new Uint8Array(memory.buffer, pointers[2], args[1]).slice());
      return calls.sourceResult;
    },
    miso_engine_web_v1_source_seek: (...args) => {
      calls.seek.push(args);
      calls.seekIdBytes.push(new Uint8Array(memory.buffer, pointers[2], args[1]).slice());
      return 0;
    },
    miso_engine_web_v1_command_report_ptr: () => reportPointer,
    miso_engine_web_v1_command_submit: (_handle, count) => {
      calls.commands.push(new Uint8Array(memory.buffer, commandPointer, count * 48).slice());
      report.setUint32(8, calls.commandResult, true);
      report.setUint32(12, calls.commandResult === 0 ? 0 : 8, true);
      report.setUint32(16, 0, true);
      report.setUint32(20, calls.commandResult === 0 ? count : 0, true);
      report.setBigUint64(24, status.getBigUint64(32, true), true);
      return calls.commandResult;
    },
    miso_engine_web_v1_meter_lease: (_handle, enabled) => {
      calls.meterLease.push(enabled);
      return 0;
    },
    miso_engine_web_v1_meter_header_ptr: () => meterHeaderPointer,
    miso_engine_web_v1_meter_poll: () => {
      if (calls.meterWindows === 0) return 0;
      calls.meterWindows -= 1;
      // Issue #143: the two sections carry **different** values, so a worklet that read the peak
      // view where the gain-reduction view belongs (or the reverse) is visible here rather than
      // hidden by a uniform fill.
      const frame = new Float32Array(memory.buffer, meterFramePointer, meterFrameFloats);
      frame.fill(0.5, 0, trackIds.length * 2 + 2);
      frame.fill(6.5, trackIds.length * 2 + 2);
      return 1;
    },
    miso_engine_web_v1_console_track_count: () => trackIds.length,
    miso_engine_web_v1_console_track_id: (_handle, index) => {
      const id = trackIds[index];
      const bytes = new Uint8Array(memory.buffer, pointers[2], id.length);
      for (let byte = 0; byte < id.length; byte += 1) bytes[byte] = id.charCodeAt(byte);
      return id.length;
    },
    miso_engine_web_v1_source_count: () => sourceRows.length,
    miso_engine_web_v1_source_id: (_handle, index) => {
      const id = sourceRows[index]?.id;
      if (id === undefined) return 0;
      const bytes = new Uint8Array(memory.buffer, pointers[2], id.length);
      for (let byte = 0; byte < id.length; byte += 1) bytes[byte] = id.charCodeAt(byte);
      return id.length;
    },
    miso_engine_web_v1_source_channels: (_handle, index) => sourceRows[index]?.channels ?? 0,
    miso_engine_web_v1_source_frames: (_handle, index) => sourceRows[index]?.frames ?? 0n,
    miso_engine_web_v1_dispose: () => {
      calls.dispose += 1;
      return 0;
    },
  };
  return { exports, calls, trackIds, sourceRows, meterFrameFloats };
}

async function testProcessor() {
  const originalProcessor = globalThis.AudioWorkletProcessor;
  const originalRegister = globalThis.registerProcessor;
  const originalInstance = WebAssembly.Instance;
  const originalSampleRate = globalThis.sampleRate;
  const originalRenderQuantumSize = globalThis.renderQuantumSize;
  const originalTextEncoder = globalThis.TextEncoder;
  const processorSessionDocument = new originalTextEncoder().encode("{\"schema_version\":0}");
  const nonAsciiSourceId = "caf\u00e9-\u96ea-\ud83d\ude00";
  const nonAsciiSourceIdUtf8 = new originalTextEncoder().encode(nonAsciiSourceId);
  let registered;
  let nextFake;
  let instanceCount = 0;
  let throwInstance = false;
  let throwReadyPost = false;
  class FakePort {
    onmessage = null;
    posts = [];

    postMessage(message, transfer = []) {
      if (throwReadyPost && message?.tag === "miso.ready.v1") {
        throw new Error("synthetic ready publication failure");
      }
      this.posts.push({ message: structuredClone(message, { transfer }), transferCount: transfer.length });
    }
  }
  class FakeProcessor {
    constructor() {
      this.port = new FakePort();
    }
  }
  globalThis.AudioWorkletProcessor = FakeProcessor;
  globalThis.registerProcessor = (_name, implementation) => { registered = implementation; };
  globalThis.sampleRate = 48000;
  globalThis.renderQuantumSize = 64;
  globalThis.TextEncoder = undefined;
  WebAssembly.Instance = class {
    constructor() {
      instanceCount += 1;
      if (throwInstance) throw new Error("synthetic instantiation failure");
      this.exports = nextFake.exports;
    }
  };
  try {
    await import(`${workletUrl.href}?processor-test`);
    assert.equal(typeof registered, "function");
    const construct = (fake) => {
      nextFake = fake;
      return new registered({
        processorOptions: {
          module: {},
          document: processorSessionDocument,
          options: limits,
        },
      });
    };
    const makeProcessor = () => {
      const fake = createFakeExports(64);
      const processor = construct(fake);
      assert.deepEqual(processor.port.posts[0].message, {
        tag: "miso.ready.v1", requestId: 0, result: 0, backend: "simd128",
        resources: processor.resources, memoryBytes: 65536,
      });
      return { processor, fake };
    };

    {
      const before = instanceCount;
      globalThis.sampleRate = 44100;
      const fake = createFakeExports(64);
      const processor = construct(fake);
      globalThis.sampleRate = 48000;
      assert.equal(instanceCount, before + 1, "physical mismatch is answered by atomic boot");
      assert.equal(fake.calls.dispose, 0);
      assert.equal(processor.port.posts[0].message.result, 9);
      assert.equal(processor.process([], [[new Float32Array(64), new Float32Array(64)]]), false);
    }

    {
      const before = instanceCount;
      globalThis.renderQuantumSize = 128;
      const fake = createFakeExports(64);
      const processor = construct(fake);
      globalThis.renderQuantumSize = 64;
      assert.equal(instanceCount, before + 1, "physical mismatch is answered by atomic boot");
      assert.equal(fake.calls.dispose, 0);
      assert.equal(processor.port.posts[0].message.result, 9);
    }

    {
      throwInstance = true;
      const fake = createFakeExports(64);
      const processor = construct(fake);
      throwInstance = false;
      assert.equal(fake.calls.dispose, 0);
      assert.equal(processor.port.posts[0].message.result, 255);
      assert.equal(processor.disposed, true);
    }

    for (const [result, mutate] of [
      [255, (fake) => { fake.exports.miso_engine_web_v1_boot_options_ptr = () => 0; }],
      [5, (fake) => {
        fake.calls.bootResult = 5;
        fake.exports.miso_engine_web_v1_document_ptr = () => 0;
      }],
      [1, (fake) => {
        fake.exports.miso_engine_web_v1_boot = () => 0;
        fake.exports.miso_engine_web_v1_boot_result = () => 1;
      }],
    ]) {
      const fake = createFakeExports(64);
      mutate(fake);
      const processor = construct(fake);
      assert.equal(fake.calls.dispose, 0, "a refused boot publishes no handle to dispose");
      assert.equal(processor.port.posts[0].message.result, result);
      assert.equal(processor.disposed, true);
    }

    const failureMutations = [
      (fake) => { fake.exports.miso_engine_web_v1_buffer_capacity = () => 0; },
      (fake) => { fake.exports.miso_engine_web_v1_status_ptr = () => 65500; },
      (fake) => { new DataView(fake.exports.memory.buffer, 17000).setUint32(20, 1, true); },
    ];
    for (const mutate of failureMutations) {
      const fake = createFakeExports(64);
      mutate(fake);
      const processor = construct(fake);
      assert.equal(fake.calls.dispose, 1, "post-handle construction failure disposes exactly once");
      assert.equal(processor.disposed, true);
      assert.equal(processor.process([], [[new Float32Array(64), new Float32Array(64)]]), false);
      assert.equal(processor.port.posts.length, 1);
      assert.equal(processor.port.posts[0].message.tag, "miso.error.v1");
    }

    {
      const fake = createFakeExports(64, 0);
      const processor = construct(fake);
      assert.equal(fake.calls.dispose, 1, "swapped backend artifact is transactionally disposed");
      assert.equal(processor.port.posts[0].message.result, 1);
      assert.equal(processor.process([], [[new Float32Array(64), new Float32Array(64)]]), false);
    }

    for (const offset of [16384 + 16, 17000 + 16]) {
      const fake = createFakeExports(64);
      new DataView(fake.exports.memory.buffer).setUint32(offset, 0, true);
      const processor = construct(fake);
      assert.equal(fake.calls.dispose, 1, "each Rust backend row is independently authoritative");
      assert.equal(processor.port.posts[0].message.result, 1);
    }

    {
      const fake = createFakeExports(64);
      throwReadyPost = true;
      const processor = construct(fake);
      throwReadyPost = false;
      assert.equal(fake.calls.dispose, 1, "ready publication failure disposes exactly once");
      assert.equal(processor.disposed, true);
      assert.equal(processor.process([], [[new Float32Array(64), new Float32Array(64)]]), false);
    }

    {
      const { processor, fake } = makeProcessor();
      const left = new Float32Array(64);
      const right = new Float32Array(64);
      assert.equal(processor.process([], [[left, right]]), true);
      assert.deepEqual(fake.calls.render, [64]);
      assert(left.every((sample) => sample === 0.25));
      assert(right.every((sample) => sample === -0.25));
      processor.receive({ tag: "miso.status.v1", requestId: 1 });
      const status = processor.port.posts.at(-1).message;
      assert.equal(status.memoryBytes, 65536);
      assert.equal(status.nextAbsoluteSample, 64n);
    }

    {
      const { processor, fake } = makeProcessor();
      processor.ready = false;
      const left = new Float32Array(64).fill(-0);
      const right = new Float32Array(64).fill(1);
      assert.equal(processor.process([], [[left, right]]), true);
      assert.deepEqual(fake.calls.render, []);
      assert(left.every((sample) => Object.is(sample, 0)), "pre-ready left is positive zero");
      assert(right.every((sample) => Object.is(sample, 0)), "pre-ready right is positive zero");
    }

    {
      const { processor, fake } = makeProcessor();
      const wrong = new Float32Array(32).fill(1);
      assert.equal(processor.process([], [[wrong]]), true);
      assert.deepEqual(fake.calls.render, [0]);
      assert(wrong.every((sample) => sample === 0 && !Object.is(sample, -0)));
      assert.equal(processor.stickyResult, 9);
    }

    {
      const { processor, fake } = makeProcessor();
      fake.exports.memory.buffer = new ArrayBuffer(65536);
      const left = new Float32Array(64).fill(1);
      const right = new Float32Array(64).fill(1);
      assert.equal(processor.process([], [[left, right]]), true);
      assert.deepEqual(fake.calls.render, []);
      assert(left.every((sample) => sample === 0 && !Object.is(sample, -0)));
      assert(right.every((sample) => sample === 0 && !Object.is(sample, -0)));
      assert.equal(processor.stickyResult, 9);
    }

    {
      const { processor, fake } = makeProcessor();
      fake.calls.sourceResult = 6;
      const storage = new ArrayBuffer(32);
      const incoming = structuredClone({
        tag: "miso.source.v1", requestId: 1, sourceId: nonAsciiSourceId, generation: 1n,
        startFrame: 0n, sampleRateHz: 48000,
        planes: [new Float32Array(storage, 0, 2), new Float32Array(storage, 16, 2)],
        frames: 2, endOfRegion: false,
      }, { transfer: [storage] });
      processor.receive(incoming);
      const response = processor.port.posts.at(-1);
      assert.equal(response.transferCount, 1);
      assert.equal(response.message.tag, "miso.ack.v1");
      assert.equal(response.message.result, 6);
      assert.equal(response.message.planes[0].byteOffset, 0);
      assert.equal(response.message.planes[1].byteOffset, 16);
      assert.equal(response.message.planes[0].buffer, response.message.planes[1].buffer);
      assert.deepEqual(
        fake.calls.sourceIdBytes[0],
        nonAsciiSourceIdUtf8,
        "non-ASCII submit IDs are byte-identical to UTF-8 without TextEncoder in the worklet",
      );
      processor.receive({
        tag: "miso.seek.v1", requestId: 2, sourceId: nonAsciiSourceId,
        generation: 2n, sourceFrame: 12n,
      });
      assert.deepEqual(processor.port.posts.at(-1).message, {
        tag: "miso.ack.v1", requestId: 2, result: 0,
      });
      assert.deepEqual(
        fake.calls.seekIdBytes[0],
        nonAsciiSourceIdUtf8,
        "non-ASCII seek IDs are byte-identical to UTF-8 without TextEncoder in the worklet",
      );
      processor.receive({ tag: "miso.dispose.v1", requestId: 3 });
      assert.equal(fake.calls.dispose, 1);
      assert.equal(processor.process([], [[new Float32Array(64), new Float32Array(64)]]), false);
    }

    {
      // #106 F5: `panic = abort` means a Rust panic reaches JavaScript as a throw from the render
      // export. `process()` is the only place that can contain it. Red mutation: remove the `try`
      // around `miso_engine_web_v1_render` in `process()` -> this `process()` call throws.
      const { processor } = makeProcessor();
      processor.exports.miso_engine_web_v1_render = () => {
        throw new Error("synthetic wasm trap from the render export");
      };
      const left = new Float32Array(64).fill(-0);
      const right = new Float32Array(64).fill(1);
      assert.equal(processor.process([], [[left, right]]), true, "a trap keeps the node alive");
      assert(left.every((sample) => Object.is(sample, 0)), "trapped left is positive zero");
      assert(right.every((sample) => Object.is(sample, 0)), "trapped right is positive zero");
      assert.equal(processor.stickyResult, 255);
      assert.equal(processor.ready, false);
      processor.receive({ tag: "miso.status.v1", requestId: 1 });
      const settled = processor.port.posts.at(-1).message;
      assert.equal(settled.tag, "miso.error.v1");
      assert.equal(settled.result, 255, "the trap is sticky for every later request");
    }

    {
      const { processor } = makeProcessor();
      const storage = new ArrayBuffer(16);
      const incoming = structuredClone({
        tag: "miso.source.v1", requestId: 1, sourceId: "source", generation: 1n,
        startFrame: 0n, sampleRateHz: 48000, planes: [new Float32Array(storage)],
        frames: 3, endOfRegion: false,
      }, { transfer: [storage] });
      processor.receive(incoming);
      const response = processor.port.posts.at(-1);
      assert.equal(response.message.tag, "miso.error.v1");
      assert.equal(response.message.result, 1);
      assert.equal(response.transferCount, 1);
      assert.equal(response.message.planes[0].byteLength, 16);
    }

    {
      // Issue #137 D1: the worklet copies a staged batch into Wasm, admits it, and hands the
      // record block straight back with the typed report.
      const { processor, fake } = makeProcessor();
      assert.deepEqual(processor.trackIds, fake.trackIds ?? ["kick", "snare"]);
      const records = new Uint8Array(2 * 48);
      const view = new DataView(records.buffer);
      records[0] = 2; // matrix
      records[1] = 255;
      records[2] = 255;
      view.setUint32(4, 1, true);
      view.setFloat32(24, 0.5, true);
      records[48] = 1; // pan
      records[49] = 255;
      records[50] = 255;
      view.setUint32(52, 0, true);
      view.setFloat32(72, -1, true);
      view.setFloat32(76, 1, true);
      processor.receive(structuredClone(
        { tag: "miso.command.v1", requestId: 1, count: 2, records },
        { transfer: [records.buffer] },
      ));
      const ack = processor.port.posts.at(-1);
      assert.equal(ack.message.tag, "miso.ack.v1");
      assert.equal(ack.message.result, 0);
      assert.equal(ack.message.admitted, 2);
      assert.equal(ack.message.reason, 0);
      assert.equal(ack.transferCount, 1, "the record block goes back to the caller");
      assert.equal(fake.calls.commands.length, 1);
      assert.equal(fake.calls.commands[0].length, 96, "exactly the staged bytes are submitted");
      assert.equal(fake.calls.commands[0][0], 2);
      assert.equal(fake.calls.commands[0][48], 1);

      // A record block whose length disagrees with `count` is sticky-invalid, never truncated.
      const short = new Uint8Array(48);
      processor.receive({ tag: "miso.command.v1", requestId: 2, count: 2, records: short });
      assert.equal(processor.port.posts.at(-1).message.tag, "miso.error.v1");
      assert.equal(processor.port.posts.at(-1).message.result, 1);
    }

    {
      // Issue #137 D2/D3: both leases start released, and a released lease costs the render
      // callback one boolean test and no message.
      const { processor, fake } = makeProcessor();
      const left = new Float32Array(64);
      const right = new Float32Array(64);
      fake.calls.meterWindows = 3;
      const before = processor.port.posts.length;
      assert.equal(processor.process([], [[left, right]]), true);
      assert.equal(processor.port.posts.length, before, "no lease, no frame");
      assert.equal(fake.calls.meterLease.length, 0, "no lease, no engine call");

      processor.receive({ tag: "miso.meters.v1", requestId: 1, enabled: true });
      assert.deepEqual(fake.calls.meterLease, [1]);
      assert.deepEqual(processor.port.posts.at(-1).message, {
        tag: "miso.ack.v1", requestId: 1, result: 0,
      });
      assert.equal(processor.process([], [[left, right]]), true);
      const frame = processor.port.posts.at(-1).message;
      assert.equal(frame.tag, "miso.meter.v1");
      assert.equal(frame.sequence, 1);
      assert.equal(frame.windows, 1);
      assert.equal(frame.trackCount, 2);
      assert.equal(frame.peaks.length, 6);
      assert(frame.peaks.every((value) => value === 0.5));
      // Issue #143: the gain-reduction section is its own view, its own length, and its own value.
      assert.equal(frame.trackGrDb.length, 2);
      assert(frame.trackGrDb.every((value) => value === 6.5));
      assert.equal(frame.masterGrDb, 6.5, "the header says the master reading is present");
      assert.equal(frame.firstSample, 512n);
      assert.equal(frame.endSample, 768n);

      // Releasing the lease stops the frames immediately.
      processor.receive({ tag: "miso.meters.v1", requestId: 2, enabled: false });
      assert.deepEqual(fake.calls.meterLease, [1, 0]);
      const quiet = processor.port.posts.length;
      assert.equal(processor.process([], [[left, right]]), true);
      assert.equal(processor.port.posts.length, quiet, "a released lease posts nothing");
    }

    {
      // Issue #137 D3: a full telemetry window posts exactly one frame, and the frame is honest
      // about the resolution of the clock it actually found.
      const { processor } = makeProcessor();
      processor.receive({ tag: "miso.telemetry.v1", requestId: 1, enabled: true });
      assert.deepEqual(processor.port.posts.at(-1).message, {
        tag: "miso.ack.v1", requestId: 1, result: 0,
      });
      const left = new Float32Array(64);
      const right = new Float32Array(64);
      let frames = 0;
      for (let block = 0; block < 128; block += 1) {
        assert.equal(processor.process([], [[left, right]]), true);
        const last = processor.port.posts.at(-1).message;
        if (last.tag === "miso.telemetry.v1") frames += 1;
      }
      assert.equal(frames, 1, "one frame per 128-block window and no more");
      const telemetry = processor.port.posts.at(-1).message;
      assert.equal(telemetry.blocks, 128);
      assert.equal(telemetry.sequence, 1);
      assert.equal(telemetry.deadlineMisses, 0);
      assert(telemetry.budgetMs > 1.3 && telemetry.budgetMs < 1.4, telemetry.budgetMs);
      assert(telemetry.resolutionMs > 0);
      assert.equal(typeof telemetry.belowResolution, "boolean");
      assert(telemetry.cpuPercent >= 0);
      processor.receive({ tag: "miso.telemetry.v1", requestId: 2, enabled: false });
      const quiet = processor.port.posts.length;
      for (let block = 0; block < 200; block += 1) processor.process([], [[left, right]]);
      assert.equal(processor.port.posts.length, quiet, "a released lease reads no clock");
    }

    {
      // The session map answers from the identities read once at construction.
      const { processor, fake } = makeProcessor();
      processor.receive({ tag: "miso.sessionmap.v1", requestId: 1 });
      const map = processor.port.posts.at(-1).message;
      assert.equal(map.tag, "miso.sessionmap.v1");
      assert.deepEqual(map.tracks, ["kick", "snare"]);
      assert.deepEqual(map.sources, fake.sourceRows, "issue #207: canonical source order and shape");
      assert.equal(map.metersAttached, true);
      // The identities were read once at construction and the reads are not repeated per request:
      // a second map answers from the same numbers, and `process()` never sees any of this.
      processor.receive({ tag: "miso.sessionmap.v1", requestId: 2 });
      assert.deepEqual(processor.port.posts.at(-1).message.sources, fake.sourceRows);
    }

    {
      // Issue #241: every source read is checked against something compilation already guarantees,
      // so a mis-wired export fails initialization instead of reaching a consumer as a plausible
      // number. Each mutation below is a different export lying, and each must be caught.
      for (const [what, mutate] of [
        ["a zero channel count", (e) => { e.miso_engine_web_v1_source_channels = () => 0; }],
        ["a channel count past the configured maximum",
          (e) => { e.miso_engine_web_v1_source_channels = () => 3; }],
        ["a zero frame count", (e) => { e.miso_engine_web_v1_source_frames = () => 0n; }],
        ["an empty source ID", (e) => { e.miso_engine_web_v1_source_id = () => 0; }],
        ["a source ID longer than staging",
          (e) => { e.miso_engine_web_v1_source_id = () => 65; }],
      ]) {
        const fake = createFakeExports(64);
        mutate(fake.exports);
        const processor = construct(fake);
        assert.equal(
          processor.port.posts[0].message.result,
          255,
          `${what} must fail initialization`,
        );
      }
    }
  } finally {
    globalThis.AudioWorkletProcessor = originalProcessor;
    globalThis.registerProcessor = originalRegister;
    globalThis.sampleRate = originalSampleRate;
    if (originalRenderQuantumSize === undefined) delete globalThis.renderQuantumSize;
    else globalThis.renderQuantumSize = originalRenderQuantumSize;
    globalThis.TextEncoder = originalTextEncoder;
    WebAssembly.Instance = originalInstance;
  }
}

await testMainRealm();
await testProcessor();
console.log("web AudioWorklet hermetic tests passed");
