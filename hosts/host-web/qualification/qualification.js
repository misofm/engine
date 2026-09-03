const SAMPLE_RATE = 48000;
const QUANTUM_FRAMES = 128;
const CORPUS_FRAMES = QUANTUM_FRAMES * 3;
const DEFAULT_RING_FRAMES = 5120;
const STALL_FRAMES = DEFAULT_RING_FRAMES;
const STALL_RENDER_FRAMES = STALL_FRAMES;
const REQUESTED_STALL_MS = 120;
// Issue #137: the console rows. 130 blocks is one full 128-block telemetry window plus slack, and
// a two-block meter window makes the decimated cadence observable inside it.
const CONSOLE_BLOCKS = 130;
const CONSOLE_FRAMES = CONSOLE_BLOCKS * QUANTUM_FRAMES;
const CONSOLE_COMMAND_QUEUE_RECORDS = 64n;
const CONSOLE_METER_BLOCKS = 2n;
const COMMAND_MATRIX = 2;
// Issue #143 E12: the observation row. Sixteen blocks is eight two-block windows -- enough for the
// compressor's 10 ms attack to settle and for `firstSample` monotonicity to be a real sequence
// rather than a single point.
const OBSERVATION_BLOCKS = 16;
const OBSERVATION_FRAMES = OBSERVATION_BLOCKS * QUANTUM_FRAMES;
const OBSERVATION_TAPS = 4n;
const OBSERVATION_MASTER_TRACK_PLUS_ONE = 1n;
const OBSERVATION_TAP_ID = 1;
const OBSERVATION_LEVEL = 0.5;
const MINIMUM_STALL_MS = 100;
const PROCESSOR_NAME = "miso-engine-v1-audio-worklet";
const ARTIFACT_URL = "/artifacts/miso-engine-v1-audio-worklet.simd128.wasm";
const WORKLET_URL = "/artifacts/miso-engine-v1-audio-worklet.js";

// Canonical `() -> v128` module, identical to the preflight probe in the shipped host module.
const SIMD128_PROBE = new Uint8Array([
  0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00,
  0x01, 0x05, 0x01, 0x60, 0x00, 0x01, 0x7b,
  0x03, 0x02, 0x01, 0x00,
  0x0a, 0x08, 0x01, 0x06, 0x00, 0x41, 0x00, 0xfd, 0x0f, 0x0b,
]);

// Issue #137 D1/D2: the two console words. Zero in both is the frozen pre-#137 shape -- no control
// channel, no meter observers -- which is what the corpus and attestation gates keep using so
// their digests and resource rows are untouched by the console's existence.
//
// Issue #281: this is the post-#240 atomic-boot shape. `createMisoAudioWorkletHost` takes exactly
// `{ context, document, options, simd128ModuleUrl, workletModuleUrl }`, and `options` is exactly
// the six boot words below -- both guards are `hasExactFields`, so the pre-#240 spelling
// (`quantumFrames`/`sessionDocument`/`limits`, with the twenty-one capacity ceilings #240 deleted)
// is refused as `miso.error.v1` requestId 0 result 1 before a module is even fetched. The quantum
// now comes from the context's own `renderQuantumSize`, and the ceilings are the engine's, not the
// caller's; `sourceRingFrames` is the one word that survived the change and it keeps its meaning.
function bootOptions(
  sourceRingFrames,
  consoleCommandQueueRecords = 0n,
  consoleMeterBlocks = 0n,
  consoleObservationTaps = 0n,
  consoleMasterTrackPlusOne = 0n,
) {
  return {
    sourceRingFrames,
    // Zero is "no caller-imposed memory ceiling", the same word `tests/browser-v1` sends and the
    // same one the direct oracle writes at boot-options offset 24.
    maximumMemoryBytes: 0n,
    consoleCommandQueueRecords,
    consoleMeterBlocks,
    // Issue #143 D3/D6: the frozen configuration's remaining two reserved words. Zero in both is
    // the pre-#143 shape, which is what every row but the observation one keeps using.
    consoleObservationTaps,
    consoleMasterTrackPlusOne,
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

// The observation row's fed block: a constant level on both channels, high enough above the
// compressor's -30 dBFS threshold that an armed tap has a real reduction to publish. Every block is
// identical, so `blockIndex` is unused -- it is taken anyway to keep the one shape every fed-PCM
// generator in this harness has, which is what `session-identities.mjs` walks.
function observationPlanes(_blockIndex) {
  const storage = new ArrayBuffer(QUANTUM_FRAMES * 2 * Float32Array.BYTES_PER_ELEMENT);
  const left = new Float32Array(storage, 0, QUANTUM_FRAMES);
  const right = new Float32Array(
    storage,
    QUANTUM_FRAMES * Float32Array.BYTES_PER_ELEMENT,
    QUANTUM_FRAMES,
  );
  left.fill(OBSERVATION_LEVEL);
  right.fill(OBSERVATION_LEVEL);
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

async function renderCorpusSegment(createHost, sessionDocument, descriptions) {
  const frames = descriptions.length * QUANTUM_FRAMES;
  const context = new OfflineAudioContext(2, frames, SAMPLE_RATE);
  const host = await createHost({
    context,
    document: sessionDocument,
    options: bootOptions(frames),
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

async function runCorpusQualification(createHost, sessionDocument, source) {
  if (source?.sourceId !== "fixture-source" || source?.sampleRateHz !== SAMPLE_RATE
      || source?.quantumFrames !== QUANTUM_FRAMES || source?.blocks?.length !== 2) {
    throw new Error("corpus fixture shape mismatch");
  }
  // Firefox does not expose OfflineAudioContext.suspend. Prefill two bounded fresh contexts and
  // concatenate the oracle's exact [block 0, block 0, block 1] timeline instead of coordinating
  // submissions with a main-realm-only API. The resulting 384 frames must still match the one
  // frozen native digest bit for bit.
  const first = await renderCorpusSegment(createHost, sessionDocument, [source.blocks[0]]);
  const rest = await renderCorpusSegment(createHost, sessionDocument, source.blocks);
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

async function typedUnsupportedAttestation(createHost, sessionDocument) {
  const context = new OfflineAudioContext(2, QUANTUM_FRAMES, SAMPLE_RATE);
  try {
    await createHost({
      context,
      document: sessionDocument,
      options: bootOptions(DEFAULT_RING_FRAMES),
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

async function diagnoseReady(sessionDocument) {
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
        module,
        document: new Uint8Array(sessionDocument),
        options: bootOptions(QUANTUM_FRAMES),
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

/// #137 E8: a live-console row -- one parameter change applied and one meter frame received.
///
/// The command is awaited before rendering starts, so its acknowledgement names sample `0` and the
/// whole rendered block is post-command: the browser leg proves *that* a change reached the DSP
/// and *what* it did, while the exact-sample transition is proved bit for bit by the native and
/// raw-Wasm command-timeline oracles. The retarget halves the left matrix coefficient, so the
/// expected output is the submitted left plane at half gain and the right plane untouched --
/// computed here, not read back, so a console that quietly did nothing cannot pass.
async function runConsoleQualification(createHost, sessionDocument) {
  const context = new OfflineAudioContext(2, CONSOLE_FRAMES, SAMPLE_RATE);
  const host = await createHost({
    context,
    document: sessionDocument,
    options: bootOptions(CONSOLE_FRAMES, CONSOLE_COMMAND_QUEUE_RECORDS, CONSOLE_METER_BLOCKS),
    simd128ModuleUrl: ARTIFACT_URL,
    workletModuleUrl: WORKLET_URL,
  });
  host.node.connect(context.destination);

  const meterFrames = [];
  const telemetryFrames = [];
  const expected = [new Float32Array(CONSOLE_FRAMES), new Float32Array(CONSOLE_FRAMES)];
  let inputPeak = 0;
  for (let block = 0; block < CONSOLE_BLOCKS; block += 1) {
    const planes = sourcePlanes(block);
    for (let frame = 0; frame < QUANTUM_FRAMES; frame += 1) {
      expected[0][block * QUANTUM_FRAMES + frame] = planes[0][frame] * 0.5;
      expected[1][block * QUANTUM_FRAMES + frame] = planes[1][frame];
      inputPeak = Math.max(inputPeak, Math.abs(planes[0][frame]), Math.abs(planes[1][frame]));
    }
    const acknowledgement = await host.submitSource({
      requestId: block + 1,
      sourceId: "console-source",
      generation: 1n,
      startFrame: BigInt(block * QUANTUM_FRAMES),
      sampleRateHz: SAMPLE_RATE,
      planes,
      frames: QUANTUM_FRAMES,
      endOfRegion: block === CONSOLE_BLOCKS - 1,
    });
    if (acknowledgement.result !== 0) throw new Error("console prefill rejected");
  }

  // Request IDs are strictly monotonic across the whole port, so the leases and the command are
  // taken after the prefill they will be observed over.
  const map = await host.sessionMap();
  const meterLease = await host.meters({
    requestId: 10001,
    enabled: true,
    onFrame: (frame) => meterFrames.push(frame),
  });
  const telemetryLease = await host.telemetry({
    requestId: 10002,
    enabled: true,
    onFrame: (frame) => telemetryFrames.push(frame),
  });

  const command = await host.command({
    requestId: 20001,
    commands: [{
      kind: COMMAND_MATRIX,
      rack: 255,
      channel: 255,
      trackIndex: 0,
      effectIndex: 0,
      parameterId: 0,
      smoothingSamples: 0,
      values: [0.5, 0, 0, 1],
    }],
  });

  const rendered = await context.startRendering();
  // Frames posted from the render thread arrive as tasks; yield until the queue is drained.
  for (let spin = 0; spin < 8 && meterFrames.length === 0; spin += 1) {
    await new Promise((resolve) => setTimeout(resolve, 4));
  }
  await host.dispose();
  const actual = [rendered.getChannelData(0), rendered.getChannelData(1)];

  let exact = true;
  for (let channel = 0; channel < 2; channel += 1) {
    for (let frame = 0; frame < CONSOLE_FRAMES; frame += 1) {
      if (!Object.is(actual[channel][frame], expected[channel][frame])) exact = false;
    }
  }
  const peaks = meterFrames.map((frame) => Array.from(frame.peaks));
  const masterPeak = peaks.reduce(
    (highest, frame) => Math.max(highest, frame[frame.length - 2], frame[frame.length - 1]),
    0,
  );
  return {
    tracks: map.tracks,
    metersAttached: map.metersAttached,
    commandResult: command.result,
    commandReason: command.reason,
    commandAdmitted: command.admitted,
    appliedAtSample: command.appliedAtSample.toString(),
    meterLeaseResult: meterLease.result,
    telemetryLeaseResult: telemetryLease.result,
    meterFrames: meterFrames.length,
    telemetryFrames: telemetryFrames.length,
    meterFrameWidth: peaks.length === 0 ? 0 : peaks[0].length,
    masterPeak,
    inputPeak,
    exactRetargetedOutput: exact,
    expectedDigest: await pcmDigest(expected, CONSOLE_FRAMES),
    renderedDigest: await pcmDigest(actual, CONSOLE_FRAMES),
  };
}

/// Issue #143 E12: subscribe -> nonzero `trackGrDb` -> unsubscribe -> zero, in a real browser.
///
/// Two offline renders of the same sixteen blocks over the same session, differing only in whether
/// the tap is armed when rendering starts. That is the eval's shape and it is also E1's leg (c)
/// against leg (d) in a browser: the two renders must produce **identical PCM**, because arming a
/// declared tap may not move a sample.
///
/// `armed` is `true` for the first run and `false` for the second, which subscribes and then
/// immediately unsubscribes -- so the second run proves that an unsubscribe actually stops the
/// traffic rather than merely being accepted.
async function runObservationRun(createHost, sessionDocument, armed) {
  const context = new OfflineAudioContext(2, OBSERVATION_FRAMES, SAMPLE_RATE);
  const host = await createHost({
    context,
    document: sessionDocument,
    options: bootOptions(
      OBSERVATION_FRAMES,
      CONSOLE_COMMAND_QUEUE_RECORDS,
      CONSOLE_METER_BLOCKS,
      OBSERVATION_TAPS,
      OBSERVATION_MASTER_TRACK_PLUS_ONE,
    ),
    simd128ModuleUrl: ARTIFACT_URL,
    workletModuleUrl: WORKLET_URL,
  });
  host.node.connect(context.destination);

  const frames = [];
  for (let block = 0; block < OBSERVATION_BLOCKS; block += 1) {
    const acknowledgement = await host.submitSource({
      requestId: block + 1,
      sourceId: "console-source",
      generation: 1n,
      startFrame: BigInt(block * QUANTUM_FRAMES),
      sampleRateHz: SAMPLE_RATE,
      planes: observationPlanes(block),
      frames: QUANTUM_FRAMES,
      endOfRegion: block === OBSERVATION_BLOCKS - 1,
    });
    if (acknowledgement.result !== 0) throw new Error("observation prefill rejected");
  }

  const meterLease = await host.meters({
    requestId: 30001,
    enabled: true,
    onFrame: (frame) => frames.push({
      trackGrDb: Array.from(frame.trackGrDb),
      masterGrDb: frame.masterGrDb,
      firstSample: frame.firstSample.toString(),
      endSample: frame.endSample.toString(),
      peaks: frame.peaks.length,
    }),
  });
  const subscription = {
    trackIndex: 0,
    rack: 1,
    effectIndex: 0,
    tapId: OBSERVATION_TAP_ID,
    windowBlocks: Number(CONSOLE_METER_BLOCKS),
    armed: true,
  };
  const subscribed = await host.observe({ requestId: 30002, subscriptions: [subscription] });
  let unsubscribed = null;
  if (!armed) {
    unsubscribed = await host.observe({
      requestId: 30003,
      subscriptions: [{ ...subscription, armed: false }],
    });
  }

  const rendered = await context.startRendering();
  for (let spin = 0; spin < 8 && frames.length === 0; spin += 1) {
    await new Promise((resolve) => setTimeout(resolve, 4));
  }
  await host.dispose();
  const actual = [rendered.getChannelData(0), rendered.getChannelData(1)];
  const values = frames.map((frame) => frame.trackGrDb[0]);
  const samples = frames.map((frame) => BigInt(frame.firstSample));
  let monotonic = true;
  for (let index = 1; index < samples.length; index += 1) {
    if (samples[index] <= samples[index - 1]) monotonic = false;
  }
  let tiles = true;
  for (let index = 1; index < frames.length; index += 1) {
    if (frames[index].firstSample !== frames[index - 1].endSample) tiles = false;
  }
  return {
    meterLeaseResult: meterLease.result,
    subscribeResult: subscribed.result,
    subscribeReason: subscribed.reason,
    bindings: subscribed.bindings.length,
    frameSlot: subscribed.bindings[0]?.frameSlot ?? -1,
    windowBlocks: subscribed.bindings[0]?.windowBlocks ?? -1,
    unsubscribeResult: unsubscribed === null ? null : unsubscribed.result,
    unsubscribeBindings: unsubscribed === null ? null : unsubscribed.bindings.length,
    frames: frames.length,
    trackGrDbWidth: frames.length === 0 ? -1 : frames[0].trackGrDb.length,
    peakWidth: frames.length === 0 ? -1 : frames[0].peaks,
    maximumTrackGrDb: values.reduce((highest, value) => Math.max(highest, value), 0),
    everyValueFinite: values.every((value) => Number.isFinite(value) && value >= 0),
    masterMatchesTrack: frames.every(
      (frame) => frame.masterGrDb === null || frame.masterGrDb === frame.trackGrDb[0],
    ),
    masterPresent: frames.some((frame) => frame.masterGrDb !== null),
    firstSampleMonotonic: monotonic,
    windowsTile: tiles,
    renderedDigest: await pcmDigest(actual, OBSERVATION_FRAMES),
  };
}

/// Issue #143 E12: the observation row, both runs.
async function runObservationQualification(createHost, sessionDocument) {
  const armed = await runObservationRun(createHost, sessionDocument, true);
  const disarmed = await runObservationRun(createHost, sessionDocument, false);
  return { armed, disarmed, identicalAudio: armed.renderedDigest === disarmed.renderedDigest };
}

async function runStallQualification(createHost, sessionDocument) {
  const context = new OfflineAudioContext(2, STALL_RENDER_FRAMES, SAMPLE_RATE);
  const host = await createHost({
    context,
    document: sessionDocument,
    // #137 E6: the stall runs with a live console attached and its meter lease held, so the
    // 100 ms fault is survived under exactly the command and metering load a mixing console
    // imposes -- and the frozen exact-output requirement is unchanged.
    options: bootOptions(DEFAULT_RING_FRAMES, CONSOLE_COMMAND_QUEUE_RECORDS, CONSOLE_METER_BLOCKS),
    simd128ModuleUrl: ARTIFACT_URL,
    workletModuleUrl: WORKLET_URL,
  });
  host.node.connect(context.destination);
  const stallMeterFrames = [];

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

  const meterLease = await host.meters({
    requestId: 30001,
    enabled: true,
    onFrame: (frame) => stallMeterFrames.push(frame),
  });
  // #137 E6: an identity retarget admitted immediately before the fault. It changes no
  // coefficient -- the stall session's pan is already the identity matrix -- so the frozen exact
  // digest still applies, while the control path, its queue and the meter fold are all live
  // across the stall.
  const stallCommand = await host.command({
    requestId: 30002,
    commands: [{
      kind: COMMAND_MATRIX,
      rack: 255,
      channel: 255,
      trackIndex: 0,
      effectIndex: 0,
      parameterId: 0,
      smoothingSamples: 0,
      values: [1, 0, 0, 1],
    }],
  });

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
    consoleCommandResult: stallCommand.result,
    consoleMeterLeaseResult: meterLease.result,
    consoleMeterFrames: stallMeterFrames.length,
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
    consoleResponse, observationResponse,
  ] =
    await Promise.all([
      import("/artifacts/miso-engine-v1-audio-worklet-host.js"),
      fetch("/fixture/expected.json"),
      fetch("/fixture/session.json"),
      fetch("/fixture/source.json"),
      fetch("/qualification/stall-session.json"),
      fetch("/qualification/console-session.json"),
      fetch("/qualification/observation-session.json"),
    ]);
  if (!expectedResponse.ok || !sessionResponse.ok || !sourceResponse.ok || !stallResponse.ok
      || !consoleResponse.ok || !observationResponse.ok) {
    throw new Error("qualification fixture fetch failed");
  }
  const expected = await expectedResponse.json();
  const corpusDocument = new TextEncoder().encode(await sessionResponse.text());
  const source = await sourceResponse.json();
  const stallDocument = new TextEncoder().encode(await stallResponse.text());
  // Issue #137 E8: the console row needs a region long enough for one full 128-block telemetry
  // window, which the 40-block stall region is not.
  const consoleDocument = new TextEncoder().encode(await consoleResponse.text());
  // Issue #143 E12: the console session plus one compressor, so an armed tap has a real reduction.
  const observationDocument = new TextEncoder().encode(await observationResponse.text());
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
          stallDocument,
        ),
      },
      boot: { ready: false, backend: null },
      corpus: null,
      console: null,
      observation: null,
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
        await runCorpusQualification(createMisoAudioWorkletHost, corpusDocument, source),
        await runCorpusQualification(createMisoAudioWorkletHost, corpusDocument, source),
      ],
    };
  } catch (error) {
    const [diagnostic, globals] = await Promise.all([
      diagnoseReady(corpusDocument),
      diagnoseGlobals(),
    ]);
    throw new Error(`corpus qualification failed: ${diagnosticJson({
      error: { name: error?.name, message: error?.message, ...error }, diagnostic, globals,
    })}`);
  }
  const digests = await Promise.all(
    correctness.runs.map((run) => pcmDigest(run.pcm, CORPUS_FRAMES)),
  );
  let live;
  try {
    live = await runConsoleQualification(createMisoAudioWorkletHost, consoleDocument);
  } catch (error) {
    const diagnostic = await diagnoseReady(consoleDocument);
    throw new Error(`console qualification failed: ${diagnosticJson({
      name: error?.name, message: error?.message, ...error, diagnostic,
    })}`);
  }
  let observation;
  try {
    observation = await runObservationQualification(
      createMisoAudioWorkletHost,
      observationDocument,
    );
  } catch (error) {
    const diagnostic = await diagnoseReady(observationDocument);
    throw new Error(`observation qualification failed: ${diagnosticJson({
      name: error?.name, message: error?.message, ...error, diagnostic,
    })}`);
  }
  let stall;
  try {
    stall = await runStallQualification(createMisoAudioWorkletHost, stallDocument);
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
    console: live,
    observation,
    stall,
  };
}

export const qualificationConstants = Object.freeze({
  defaultRingFrames: DEFAULT_RING_FRAMES,
  minimumStallMs: MINIMUM_STALL_MS,
  processorName: PROCESSOR_NAME,
  quantumFrames: QUANTUM_FRAMES,
  sourceChannels: 2,
  sourceBitDepth: "32f",
});

// Issue #272: the fed-PCM generator behind each qualification session document, exported so the
// harness runner can re-derive that document's declared `content` identity from the exact same
// code the browser feeds. Every row states the generator, the block count fed, and the source row
// the document must declare; the identity itself is derived, never written down here, so a
// generator change and a stale document cannot agree. `startFrame` is always `block *
// quantumFrames` and `endOfRegion` lands on the last block, so the fed region is exactly
// `blocks * quantumFrames` frames -- the declared `frames`.
export const qualificationSessionSources = Object.freeze([
  Object.freeze({
    document: "console-session.json",
    sourceId: "console-source",
    blocks: CONSOLE_BLOCKS,
    planes: sourcePlanes,
  }),
  Object.freeze({
    document: "observation-session.json",
    sourceId: "console-source",
    blocks: OBSERVATION_BLOCKS,
    planes: observationPlanes,
  }),
  Object.freeze({
    document: "stall-session.json",
    sourceId: "stall-source",
    blocks: STALL_FRAMES / QUANTUM_FRAMES,
    planes: sourcePlanes,
  }),
]);
