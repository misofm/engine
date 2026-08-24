const SAMPLE_RATE = 48000;
const QUANTUM_FRAMES = 128;
const CORPUS_FRAMES = QUANTUM_FRAMES * 3;
const DEFAULT_RING_FRAMES = 5120;
const STALL_FRAMES = DEFAULT_RING_FRAMES;
const STALL_RENDER_FRAMES = STALL_FRAMES;
const REQUESTED_STALL_MS = 120;
const MINIMUM_STALL_MS = 100;
const PROCESSOR_NAME = "miso-engine-v2-audio-worklet";
const ARTIFACT_URL = "/artifacts/miso-engine-v2-audio-worklet.simd128.wasm";
const WORKLET_URL = "/artifacts/miso-engine-v2-audio-worklet.js";

// Canonical `() -> v128` module, identical to the preflight probe in the shipped host module.
const SIMD128_PROBE = new Uint8Array([
  0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00,
  0x01, 0x05, 0x01, 0x60, 0x00, 0x01, 0x7b,
  0x03, 0x02, 0x01, 0x00,
  0x0a, 0x08, 0x01, 0x06, 0x00, 0x41, 0x00, 0xfd, 0x0f, 0x0b,
]);

function limits(sourceRingFrames) {
  return {
    sessionTomlBytes: 1 << 20,
    diagnosticBytes: 1 << 14,
    sourceIdBytes: 1 << 10,
    maximumSourceChannels: 8,
    sourceRingFrames,
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

function bytesToHex(bytes) {
  return Array.from(bytes, (byte) => byte.toString(16).padStart(2, "0")).join("");
}

async function pcmDigest(pcm, frames) {
  const bytes = new ArrayBuffer(pcm.length * frames * Float32Array.BYTES_PER_ELEMENT);
  const view = new DataView(bytes);
  let offset = 0;
  for (const channel of pcm) {
    for (let frame = 0; frame < frames; frame += 1) {
      view.setFloat32(offset, channel[frame], true);
      offset += Float32Array.BYTES_PER_ELEMENT;
    }
  }
  return bytesToHex(new Uint8Array(await crypto.subtle.digest("SHA-256", bytes)));
}

function sourcePlanes(blockIndex) {
  const storage = new ArrayBuffer(QUANTUM_FRAMES * 2 * Float32Array.BYTES_PER_ELEMENT);
  const left = new Float32Array(storage, 0, QUANTUM_FRAMES);
  const right = new Float32Array(
    storage,
    QUANTUM_FRAMES * Float32Array.BYTES_PER_ELEMENT,
    QUANTUM_FRAMES,
  );
  const start = blockIndex * QUANTUM_FRAMES;
  for (let frame = 0; frame < QUANTUM_FRAMES; frame += 1) {
    // Binary fractions make the expected identity bit-exact in every engine.
    left[frame] = (start + frame + 1) / 8192;
    right[frame] = -(start + frame + 1) / 16384;
  }
  return [left, right];
}

function corpusPlanes(description) {
  const storage = new ArrayBuffer(QUANTUM_FRAMES * 2 * Float32Array.BYTES_PER_ELEMENT);
  const left = new Float32Array(storage, 0, QUANTUM_FRAMES);
  const right = new Float32Array(
    storage,
    QUANTUM_FRAMES * Float32Array.BYTES_PER_ELEMENT,
    QUANTUM_FRAMES,
  );
  for (let frame = 0; frame < QUANTUM_FRAMES; frame += 1) {
    left[frame] = description.leftBase + description.leftStep * frame;
    right[frame] = 0;
  }
  return [left, right];
}

function corpusRequest(requestId, description) {
  return {
    requestId,
    sourceId: "fixture-source",
    generation: 1n,
    startFrame: BigInt(description.startFrame),
    sampleRateHz: SAMPLE_RATE,
    planes: corpusPlanes(description),
    frames: description.frames,
    endOfRegion: description.final,
  };
}

async function renderCorpusSegment(createHost, sessionToml, descriptions) {
  const frames = descriptions.length * QUANTUM_FRAMES;
  const context = new OfflineAudioContext(2, frames, SAMPLE_RATE);
  const host = await createHost({
    context,
    quantumFrames: QUANTUM_FRAMES,
    sessionToml,
    limits: limits(frames),
    simd128ModuleUrl: ARTIFACT_URL,
    workletModuleUrl: WORKLET_URL,
  });
  if (host.backend !== "simd128") throw new Error("corpus worklet backend mismatch");
  host.node.connect(context.destination);
  for (const [index, description] of descriptions.entries()) {
    const acknowledgement = await host.submitSource(corpusRequest(index + 1, description));
    if (acknowledgement.result !== 0) throw new Error("corpus prefill rejected");
  }
  const rendered = await context.startRendering();
  await host.dispose();
  return [
    new Float32Array(rendered.getChannelData(0)),
    new Float32Array(rendered.getChannelData(1)),
  ];
}

async function runCorpusQualification(createHost, sessionToml, source) {
  if (source?.sourceId !== "fixture-source" || source?.sampleRateHz !== SAMPLE_RATE
      || source?.quantumFrames !== QUANTUM_FRAMES || source?.blocks?.length !== 2) {
    throw new Error("corpus fixture shape mismatch");
  }
  // Firefox does not expose OfflineAudioContext.suspend. Prefill two bounded fresh contexts and
  // concatenate the oracle's exact [block 0, block 0, block 1] timeline instead of coordinating
  // submissions with a main-realm-only API. The resulting 384 frames must still match the one
  // frozen native digest bit for bit.
  const first = await renderCorpusSegment(createHost, sessionToml, [source.blocks[0]]);
  const rest = await renderCorpusSegment(createHost, sessionToml, source.blocks);
  const pcm = [new Float32Array(CORPUS_FRAMES), new Float32Array(CORPUS_FRAMES)];
  for (let channel = 0; channel < 2; channel += 1) {
    pcm[channel].set(first[channel], 0);
    pcm[channel].set(rest[channel], QUANTUM_FRAMES);
  }
  return { backend: "simd128", pcm };
}

function diagnosticJson(value) {
  return JSON.stringify(
    value,
    (_key, item) => typeof item === "bigint" ? item.toString() : item,
  );
}

async function typedUnsupportedAttestation(createHost, sessionToml) {
  const context = new OfflineAudioContext(2, QUANTUM_FRAMES, SAMPLE_RATE);
  try {
    await createHost({
      context,
      quantumFrames: QUANTUM_FRAMES,
      sessionToml,
      limits: limits(DEFAULT_RING_FRAMES),
      simd128ModuleUrl: ARTIFACT_URL,
      workletModuleUrl: WORKLET_URL,
    });
  } catch (error) {
    return error?.tag === "miso.unsupported.v1"
      && error?.requestId === 0
      && error?.result === 7
      && error?.capability === "simd128";
  }
  return false;
}

function busyWait(milliseconds) {
  const started = performance.now();
  while (performance.now() - started < milliseconds) {
    // This is the qualification fault: the main realm cannot service messages or timers.
  }
  return performance.now() - started;
}

async function preflightArtifact() {
  const context = new OfflineAudioContext(2, QUANTUM_FRAMES, SAMPLE_RATE);
  const response = await fetch(ARTIFACT_URL);
  if (!response.ok) throw new Error(`artifact fetch returned ${response.status}`);
  await WebAssembly.compile(await response.arrayBuffer());
  await context.audioWorklet.addModule(WORKLET_URL);
}

async function diagnoseReady(sessionToml) {
  try {
    const context = new OfflineAudioContext(2, QUANTUM_FRAMES, SAMPLE_RATE);
    await context.audioWorklet.addModule(WORKLET_URL);
    const response = await fetch(ARTIFACT_URL);
    const module = await WebAssembly.compile(await response.arrayBuffer());
    const node = new AudioWorkletNode(context, PROCESSOR_NAME, {
      numberOfInputs: 0,
      numberOfOutputs: 1,
      outputChannelCount: [2],
      processorOptions: {
        requestId: 0,
        module,
        backend: "simd128",
        sampleRateHz: SAMPLE_RATE,
        quantumFrames: QUANTUM_FRAMES,
        sessionToml: new Uint8Array(sessionToml),
        limits: limits(QUANTUM_FRAMES),
      },
    });
    return await new Promise((resolve) => {
      node.port.onmessage = (event) => resolve({ kind: "message", message: event.data });
      node.port.onmessageerror = () => resolve({ kind: "messageerror" });
      node.onprocessorerror = () => resolve({ kind: "processorerror" });
    });
  } catch (error) {
    return { kind: "exception", message: error?.message ?? String(error), value: { ...error } };
  }
}

async function diagnoseGlobals() {
  const context = new OfflineAudioContext(2, QUANTUM_FRAMES, SAMPLE_RATE);
  await context.audioWorklet.addModule("/qualification/global-probe.js");
  const node = new AudioWorkletNode(context, "miso-qualification-global-probe");
  return new Promise((resolve) => {
    node.port.onmessage = (event) => resolve(event.data);
  });
}

async function runStallQualification(createHost, sessionToml) {
  const context = new OfflineAudioContext(2, STALL_RENDER_FRAMES, SAMPLE_RATE);
  const host = await createHost({
    context,
    quantumFrames: QUANTUM_FRAMES,
    sessionToml,
    limits: limits(DEFAULT_RING_FRAMES),
    simd128ModuleUrl: ARTIFACT_URL,
    workletModuleUrl: WORKLET_URL,
  });
  host.node.connect(context.destination);

  const blocks = STALL_FRAMES / QUANTUM_FRAMES;
  const expected = [new Float32Array(STALL_FRAMES), new Float32Array(STALL_FRAMES)];
  for (let block = 0; block < blocks; block += 1) {
    const planes = sourcePlanes(block);
    expected[0].set(planes[0], block * QUANTUM_FRAMES);
    expected[1].set(planes[1], block * QUANTUM_FRAMES);
    const acknowledgement = await host.submitSource({
      requestId: block + 1,
      sourceId: "stall-source",
      generation: 1n,
      startFrame: BigInt(block * QUANTUM_FRAMES),
      sampleRateHz: SAMPLE_RATE,
      planes,
      frames: QUANTUM_FRAMES,
      endOfRegion: block === blocks - 1,
    });
    if (acknowledgement.result !== 0) throw new Error("stall prefill rejected");
  }

  // An exact-length offline context needs no non-portable suspend point: rendering begins before
  // the main-realm fault and consumes all 40 prefilled quanta while that realm is unavailable.
  const rendering = context.startRendering();
  const measuredStallMs = busyWait(REQUESTED_STALL_MS);
  const rendered = await rendering;
  const status = await host.status();
  await host.dispose();
  const actual = [rendered.getChannelData(0), rendered.getChannelData(1)];

  let noDropout = true;
  let noDesync = true;
  for (let channel = 0; channel < 2; channel += 1) {
    for (let frame = 0; frame < STALL_FRAMES; frame += 1) {
      if (!Object.is(actual[channel][frame], expected[channel][frame])) noDesync = false;
      if (expected[channel][frame] !== 0 && Object.is(actual[channel][frame], 0)) noDropout = false;
    }
    for (let frame = STALL_FRAMES; frame < STALL_RENDER_FRAMES; frame += 1) {
      if (!Object.is(actual[channel][frame], 0)) noDesync = false;
    }
  }

  return {
    requestedStallMs: REQUESTED_STALL_MS,
    minimumStallMs: MINIMUM_STALL_MS,
    measuredStallMs,
    ringFrames: DEFAULT_RING_FRAMES,
    renderedFrames: STALL_FRAMES,
    nextAbsoluteSample: status.nextAbsoluteSample.toString(),
    renderedQuanta: status.renderedQuanta.toString(),
    noDropout,
    noDesync,
    expectedDigest: await pcmDigest(expected, STALL_FRAMES),
    renderedDigest: await pcmDigest(actual, STALL_FRAMES),
  };
}

export async function runQualification() {
  const [
    { createMisoAudioWorkletHost }, expectedResponse, sessionResponse, sourceResponse, stallResponse,
  ] =
    await Promise.all([
      import("/artifacts/miso-engine-v2-audio-worklet-host.js"),
      fetch("/fixture/expected.json"),
      fetch("/fixture/session.toml"),
      fetch("/fixture/source.json"),
      fetch("/qualification/stall-session.toml"),
    ]);
  if (!expectedResponse.ok || !sessionResponse.ok || !sourceResponse.ok || !stallResponse.ok) {
    throw new Error("qualification fixture fetch failed");
  }
  const expected = await expectedResponse.json();
  const sessionToml = new TextEncoder().encode(await sessionResponse.text());
  const source = await sourceResponse.json();
  const stallSessionToml = new TextEncoder().encode(await stallResponse.text());
  const simd128 = WebAssembly.validate(SIMD128_PROBE);

  if (!simd128) {
    return {
      schema: "miso.web.qualification.result.v1",
      secureContext: window.isSecureContext,
      attestation: {
        probe: false,
        outcome: "miso.unsupported.v1",
        typedRefusal: await typedUnsupportedAttestation(
          createMisoAudioWorkletHost,
          stallSessionToml,
        ),
      },
      boot: { ready: false, backend: null },
      corpus: null,
      stall: null,
    };
  }

  try {
    await preflightArtifact();
  } catch (error) {
    throw new Error(`artifact preflight failed: ${error?.message ?? String(error)}`);
  }
  let correctness;
  try {
    correctness = {
      runs: [
        await runCorpusQualification(createMisoAudioWorkletHost, sessionToml, source),
        await runCorpusQualification(createMisoAudioWorkletHost, sessionToml, source),
      ],
    };
  } catch (error) {
    const [diagnostic, globals] = await Promise.all([
      diagnoseReady(sessionToml),
      diagnoseGlobals(),
    ]);
    throw new Error(`corpus qualification failed: ${diagnosticJson({
      error: { name: error?.name, message: error?.message, ...error }, diagnostic, globals,
    })}`);
  }
  const digests = await Promise.all(
    correctness.runs.map((run) => pcmDigest(run.pcm, CORPUS_FRAMES)),
  );
  let stall;
  try {
    stall = await runStallQualification(createMisoAudioWorkletHost, stallSessionToml);
  } catch (error) {
    throw new Error(`stall qualification failed: ${JSON.stringify({ ...error })}`);
  }
  return {
    schema: "miso.web.qualification.result.v1",
    secureContext: window.isSecureContext,
    attestation: { probe: true, outcome: "simd128", typedRefusal: null },
    boot: {
      ready: correctness.runs.length === 2
        && correctness.runs.every((run) => run.backend === "simd128"),
      backend: "simd128",
    },
    corpus: {
      nativeDigest: expected.directOracle.nativePcmF32leSha256,
      shippedArtifactDigest: expected.directOracle.simd128.pcmF32leSha256,
      browserDigests: digests,
      freshContextIdentity: digests[0] === digests[1],
    },
    stall,
  };
}

export const qualificationConstants = Object.freeze({
  defaultRingFrames: DEFAULT_RING_FRAMES,
  minimumStallMs: MINIMUM_STALL_MS,
  processorName: PROCESSOR_NAME,
});
