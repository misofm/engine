const SAMPLE_RATE = 48000;
const QUANTUM = 128;
const TOTAL_FRAMES = QUANTUM * 4;
const PROCESSOR_NAME = "miso-engine-v1-audio-worklet";

function bootOptions() {
  return {
    sourceRingFrames: QUANTUM,
    maximumMemoryBytes: 0n,
    // Issue #137: zero in both console words is the frozen pre-console shape -- no control
    // channel, no meter observers -- which is what this fixture's digest was pinned against.
    consoleCommandQueueRecords: 0n,
    consoleMeterBlocks: 0n,
    // Issue #143 D3/D6: the remaining two policy words. The worklet's exact-field guard means
    // these are not optional -- an `options` object missing them is refused with
    // `RESULT_INVALID_ARGUMENT` before the module is instantiated. Zero in both is the same
    // "no observation capacity, no master designation" every pre-#143 writer already meant, and
    // is what this fixture's digests were pinned against.
    consoleObservationTaps: 0n,
    consoleMasterTrackPlusOne: 0n,
  };
}

function blockPlanes(description) {
  const storage = new ArrayBuffer(QUANTUM * 2 * Float32Array.BYTES_PER_ELEMENT);
  const left = new Float32Array(storage, 0, QUANTUM);
  const right = new Float32Array(storage, QUANTUM * Float32Array.BYTES_PER_ELEMENT, QUANTUM);
  for (let index = 0; index < QUANTUM; index += 1) {
    left[index] = description.leftBase + description.leftStep * index;
    right[index] = 0;
  }
  return [left, right];
}

function sourceRequest(requestId, generation, description) {
  return {
    requestId,
    sourceId: "fixture-source",
    generation: BigInt(generation),
    startFrame: BigInt(description.startFrame),
    sampleRateHz: SAMPLE_RATE,
    planes: blockPlanes(description),
    frames: description.frames,
    endOfRegion: description.final,
  };
}

function returnedOwnership(acknowledgement) {
  return acknowledgement.planes.length === 2
    && acknowledgement.planes[0].buffer === acknowledgement.planes[1].buffer
    && acknowledgement.planes[0].byteOffset === 0
    && acknowledgement.planes[0].length === QUANTUM
    && acknowledgement.planes[1].byteOffset === QUANTUM * Float32Array.BYTES_PER_ELEMENT
    && acknowledgement.planes[1].length === QUANTUM;
}

function plainStatus(status) {
  return {
    ...status,
    nextAbsoluteSample: status.nextAbsoluteSample.toString(),
    renderedQuanta: status.renderedQuanta.toString(),
  };
}

function plainResources(resources) {
  return Object.fromEntries(Object.entries(resources).map(([name, value]) => [
    name,
    typeof value === "bigint" ? value.toString() : value,
  ]));
}

// W4-D1: exactly one artifact ships, so every context loads the same simd128 module. The
// backend row is still reported per run and cross-checked against the raw-Wasm oracle, which is
// what proves the worklet path and the direct path drive the same binary the same way.
const ARTIFACT_URL = "/artifacts/miso-engine-v1-audio-worklet.simd128.wasm";
const BACKEND = "simd128";

async function runContext(createHost, source, sessionToml) {
  const context = new OfflineAudioContext(2, TOTAL_FRAMES, SAMPLE_RATE);
  const exposedMainQuantum = Number(context.renderQuantumSize || 0);
  if (exposedMainQuantum !== 0 && exposedMainQuantum !== QUANTUM) {
    throw new Error("main quantum mismatch");
  }
  const host = await createHost({
    context,
    document: sessionToml,
    options: bootOptions(),
    simd128ModuleUrl: ARTIFACT_URL,
    workletModuleUrl: "/artifacts/miso-engine-v1-audio-worklet.js",
  });
  if (host.backend !== BACKEND) throw new Error("selected backend mismatch");
  host.node.connect(context.destination);
  const memoryBytes = host.memoryBytes;
  const resources = plainResources(host.resources);

  const first = await host.submitSource(sourceRequest(1, 1, source.blocks[0]));
  const initialBackpressure = await host.submitSource(sourceRequest(2, 1, source.blocks[1]));
  const pauseOne = context.suspend(QUANTUM / SAMPLE_RATE);
  const rendering = context.startRendering();
  await pauseOne;
  const seek = await host.seekSource({
    requestId: 3, sourceId: source.sourceId, generation: 2n, sourceFrame: 0n,
  });
  const repeat = await host.submitSource(sourceRequest(4, 2, source.blocks[0]));
  const repeatBackpressure = await host.submitSource(sourceRequest(5, 2, source.blocks[1]));
  const pauseTwo = context.suspend((QUANTUM * 2) / SAMPLE_RATE);
  await context.resume();
  await pauseTwo;
  const final = await host.submitSource(sourceRequest(6, 2, source.blocks[1]));
  const pauseThree = context.suspend((QUANTUM * 3) / SAMPLE_RATE);
  await context.resume();
  await pauseThree;
  const status = await host.status();
  const stable = host.memoryBytes === memoryBytes
    && JSON.stringify(plainResources(host.resources)) === JSON.stringify(resources)
    && status.memoryBytes === memoryBytes;
  await host.dispose();
  await host.dispose();
  await context.resume();
  const rendered = await rendering;
  const renderedLeft = rendered.getChannelData(0);
  const renderedRight = rendered.getChannelData(1);
  const positiveZeroSilence = Array.from(renderedLeft.slice(QUANTUM * 3)).every(
    (sample) => Object.is(sample, 0),
  ) && Array.from(renderedRight).every((sample) => Object.is(sample, 0));
  return {
    backend: BACKEND,
    exposedMainQuantum,
    memoryBytes,
    memoryStable: stable,
    positiveZeroSilence,
    resources,
    status: plainStatus(status),
    acknowledgements: {
      first: first.result,
      firstOwnership: returnedOwnership(first),
      initialBackpressure: initialBackpressure.result,
      initialBackpressureOwnership: returnedOwnership(initialBackpressure),
      seek: seek.result,
      repeat: repeat.result,
      repeatOwnership: returnedOwnership(repeat),
      repeatBackpressure: repeatBackpressure.result,
      repeatBackpressureOwnership: returnedOwnership(repeatBackpressure),
      final: final.result,
      finalOwnership: returnedOwnership(final),
    },
    pcm: [
      Array.from(renderedLeft),
      Array.from(renderedRight),
    ],
  };
}

async function runFailureContext(sessionToml) {
  const context = new OfflineAudioContext(2, QUANTUM, 44100);
  const exposedMainQuantum = Number(context.renderQuantumSize || 0);
  if (exposedMainQuantum !== 0 && exposedMainQuantum !== QUANTUM) {
    throw new Error("failure-context main quantum mismatch");
  }
  await context.audioWorklet.addModule("/artifacts/miso-engine-v1-audio-worklet.js");
  const response = await fetch(ARTIFACT_URL);
  if (!response.ok) throw new Error("failure-context artifact fetch failed");
  const module = await WebAssembly.compile(await response.arrayBuffer());
  const node = new AudioWorkletNode(context, PROCESSOR_NAME, {
    numberOfInputs: 0,
    numberOfOutputs: 1,
    outputChannelCount: [2],
    processorOptions: {
      module,
      document: new Uint8Array(sessionToml),
      options: bootOptions(),
    },
  });
  const failure = new Promise((resolve, reject) => {
    node.port.onmessage = (event) => resolve(event.data);
    node.port.onmessageerror = () => reject(new Error("failure-context message error"));
    node.onprocessorerror = () => reject(new Error("failure-context processor error"));
  });
  node.connect(context.destination);
  const [message, rendered] = await Promise.all([failure, context.startRendering()]);
  node.port.onmessage = null;
  node.port.onmessageerror = null;
  node.onprocessorerror = null;
  node.port.close?.();
  node.disconnect();
  const positiveZeroSilence = [0, 1].every((channel) => Array.from(
    rendered.getChannelData(channel),
  ).every((sample) => Object.is(sample, 0)));
  return {
    tag: message.tag,
    requestId: message.requestId,
    result: message.result,
    exposedMainQuantum,
    frames: rendered.length,
    positiveZeroSilence,
  };
}

export async function runMisoBrowserCorrectness() {
  const { createMisoAudioWorkletHost } = await import(
    "/artifacts/miso-engine-v1-audio-worklet-host.js"
  );
  const [sessionResponse, sourceResponse] = await Promise.all([
    fetch("/fixture/session.toml"),
    fetch("/fixture/source.json"),
  ]);
  if (!sessionResponse.ok || !sourceResponse.ok) throw new Error("fixture fetch failed");
  const sessionToml = new TextEncoder().encode(await sessionResponse.text());
  const source = await sourceResponse.json();
  // Two fresh contexts over the one shipped artifact: the pair proves fresh-context determinism,
  // and each run is compared bit-for-bit against the raw-Wasm oracle's hash.
  const runs = [
    await runContext(createMisoAudioWorkletHost, source, sessionToml),
    await runContext(createMisoAudioWorkletHost, source, sessionToml),
  ];
  const failure = await runFailureContext(sessionToml);
  return { schema: "miso.web.browser.result.v1", runs, failure };
}
