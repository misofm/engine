const SAMPLE_RATE = 48000;
const QUANTUM = 128;
const TOTAL_FRAMES = QUANTUM * 4;

function limits() {
  return {
    sessionTomlBytes: 1 << 20,
    diagnosticBytes: 1 << 14,
    sourceIdBytes: 1 << 10,
    maximumSourceChannels: 8,
    sourceRingFrames: QUANTUM,
    maximumAutomationSpansPerBlock: 256,
    maximumTracks: 1024n,
    maximumSources: 1024n,
    maximumRoutes: 4096n,
    maximumEffects: 8192n,
    maximumGraphSessionPlusPlanBytes: 64n << 20n,
    maximumSourceTotalBytes: 64n << 20n,
    maximumSourceOverheadBytes: 16n << 20n,
    maximumEffectStateBytes: 16n << 20n,
    maximumEffectScratchBytes: 16n << 20n,
    maximumBuiltinRetainedBytes: 64n << 20n,
    maximumHostRetainedBytes: 16n << 20n,
    maximumNamedAllocationBytes: 64n << 20n,
    maximumMeterStreams: 1024n,
    maximumMeterItems: 1n << 20n,
    maximumMeterBytes: 16n << 20n,
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

async function runContext(createHost, backend, source, sessionToml) {
  const context = new OfflineAudioContext(2, TOTAL_FRAMES, SAMPLE_RATE);
  const exposedMainQuantum = Number(context.renderQuantumSize || 0);
  if (exposedMainQuantum !== 0 && exposedMainQuantum !== QUANTUM) {
    throw new Error("main quantum mismatch");
  }
  const scalarUrl = "/artifacts/miso-engine-v2-audio-worklet.scalar.wasm";
  const simdUrl = backend === "scalar"
    ? "/artifacts/forced-scalar-missing.wasm"
    : "/artifacts/miso-engine-v2-audio-worklet.simd128.wasm";
  const host = await createHost({
    context,
    quantumFrames: QUANTUM,
    sessionToml,
    limits: limits(),
    scalarModuleUrl: scalarUrl,
    simd128ModuleUrl: simdUrl,
    workletModuleUrl: "/artifacts/miso-engine-v2-audio-worklet.js",
  });
  if (host.backend !== backend) throw new Error("selected backend mismatch");
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
    backend,
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

export async function runMisoBrowserCorrectness() {
  const { createMisoAudioWorkletHost } = await import(
    "/artifacts/miso-engine-v2-audio-worklet-host.js"
  );
  const [sessionResponse, sourceResponse] = await Promise.all([
    fetch("/fixture/session.toml"),
    fetch("/fixture/source.json"),
  ]);
  if (!sessionResponse.ok || !sourceResponse.ok) throw new Error("fixture fetch failed");
  const sessionToml = new TextEncoder().encode(await sessionResponse.text());
  const source = await sourceResponse.json();
  const runs = [];
  for (const backend of ["scalar", "simd128"]) {
    runs.push(await runContext(createMisoAudioWorkletHost, backend, source, sessionToml));
    runs.push(await runContext(createMisoAudioWorkletHost, backend, source, sessionToml));
  }
  return { schema: "miso.web.browser.result.v1", runs };
}
