import { hexLower } from "../web/hex-lower.js";

const SAMPLE_RATE = 48000;
const QUANTUM_FRAMES = 128;
const CORPUS_FRAMES = QUANTUM_FRAMES * 3;
const DEFAULT_RING_FRAMES = 5120;
const STALL_FRAMES = DEFAULT_RING_FRAMES;
const STALL_RENDER_FRAMES = STALL_FRAMES;
const REQUESTED_STALL_MS = 120;
// Issue #137: the live-control rows. 130 blocks is one full 128-block telemetry window plus slack,
// and a two-block meter window makes the decimated cadence observable inside it.
const LIVE_CONTROL_BLOCKS = 130;
const LIVE_CONTROL_FRAMES = LIVE_CONTROL_BLOCKS * QUANTUM_FRAMES;
const LIVE_CONTROL_METER_BLOCKS = 2n;
// Issue #143 E12: the observation row. Sixteen blocks is eight two-block windows -- enough for the
// compressor's 10 ms attack to settle and for `firstSample` monotonicity to be a real sequence
// rather than a single point.
const OBSERVATION_BLOCKS = 16;
const OBSERVATION_FRAMES = OBSERVATION_BLOCKS * QUANTUM_FRAMES;
const OBSERVATION_TAPS = 4n;
const OBSERVATION_TAP_ID = 1;
const OBSERVATION_LEVEL = 0.5;
const MINIMUM_STALL_MS = 100;
// Issue #1333 D3 / Amendment 1: the staging-read rows. One 2,048-frame spectrum window is sixteen
// blocks, the observation session's whole declared source region.
const STAGING_READ_BLOCKS = OBSERVATION_BLOCKS;
const STAGING_READ_FRAMES = STAGING_READ_BLOCKS * QUANTUM_FRAMES;
const SPECTRUM_CAPTURE_BYTES = 1 << 20;
const RESULT_OK = 0;
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

// Issue #137 D1/D2: the two live-control words. Zero in both is the frozen pre-#137 shape -- no
// control channel, no meter observers -- which is what the corpus and attestation gates keep using
// so their digests and resource rows are untouched by the live controls' existence.
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
  liveControlMeterBlocks = 0n,
  liveControlObservationTaps = 0n,
) {
  return {
    sourceRingFrames,
    // Zero is "no caller-imposed memory ceiling", the same word `tests/browser-v1` sends and the
    // same one the direct oracle writes at boot-options offset 24.
    maximumMemoryBytes: 0n,
    // Issue #1565: offset 32 is reserved and must be zero.
    reserved1: 0n,
    liveControlMeterBlocks,
    // Issue #143 D3: zero is the pre-#143 shape, which is what every row but the observation
    // ones keeps using.
    liveControlObservationTaps,
    // Issue #1689: offset 56 was the master boot word; it is reserved and must be zero. The
    // observation row designates its master with the strip's `master` row after boot.
    reserved2: 0n,
  };
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
  return hexLower(new Uint8Array(await crypto.subtle.digest("SHA-256", bytes)));
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

function corpusRequest(description) {
  return {
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
    const acknowledgement = await host.submitSource(corpusRequest(description));
    if (acknowledgement.result !== 0) throw new Error("corpus prefill rejected");
  }
  const rendered = await context.startRendering();
  const renderAllocations = await renderAllocationCount(host);
  await host.dispose();
  return {
    planes: [
      new Float32Array(rendered.getChannelData(0)),
      new Float32Array(rendered.getChannelData(1)),
    ],
    renderAllocations,
  };
}

/// Issue #1333 D3 / Amendment 1 A2: this instance's render-locked allocator count, read before
/// its dispose. Every instance has its own module and so its own counter.
async function renderAllocationCount(host) {
  const reply = await host.renderAllocationCount();
  if (reply.result !== RESULT_OK) throw new Error(`render allocation count refused: ${reply.result}`);
  return reply.count;
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
    pcm[channel].set(first.planes[channel], 0);
    pcm[channel].set(rest.planes[channel], QUANTUM_FRAMES);
  }
  return {
    backend: "simd128",
    pcm,
    renderAllocations: [first.renderAllocations, rest.renderAllocations],
  };
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

/// One matrix retarget of the qualification track as a `SET_CONTROL` transaction (#1563B). The SDK
/// builds and sends it through `host.control()`, and this page does not repeat the SDK's encoder:
/// the runner always builds and serves the SDK bundle for these rows. The session's track is in
/// matrix mode, so the four coefficients are live rows of the strip.
async function applyMatrix(host, matrix) {
  const { applyMatrixThroughSdk } = await import("/sdk/sdk-response-client.js");
  return applyMatrixThroughSdk(host, "track", matrix);
}

/// Designate the qualification track the master with its `master` row (#1689 D6), the one way an
/// app designates it, through the SDK's `strip.master(true)` and `host.control()`.
async function designateMaster(host) {
  const { designateMasterThroughSdk } = await import("/sdk/sdk-response-client.js");
  return designateMasterThroughSdk(host, "track");
}

/// The track IDs of the running session, in the core's order, read by the SDK from the control list.
async function readTracks(host) {
  const { readTracksThroughSdk } = await import("/sdk/sdk-response-client.js");
  return readTracksThroughSdk(host);
}

/// #137 E8: a live-control row -- one parameter change applied and one meter frame received.
///
/// The transaction is awaited before rendering starts, so the whole rendered block is
/// post-transaction: the browser leg proves *that* a change reached the DSP
/// and *what* it did, while the exact-sample transition is proved bit for bit by the native and
/// raw-Wasm command-timeline oracles. The retarget halves the left matrix coefficient, so the
/// expected output is the submitted left plane at half gain and the right plane untouched --
/// computed here, not read back, so live controls that quietly did nothing cannot pass.
async function runLiveControlQualification(createHost, sessionDocument) {
  const context = new OfflineAudioContext(2, LIVE_CONTROL_FRAMES, SAMPLE_RATE);
  const host = await createHost({
    context,
    document: sessionDocument,
    options: bootOptions(
      LIVE_CONTROL_FRAMES,
      LIVE_CONTROL_METER_BLOCKS,
    ),
    simd128ModuleUrl: ARTIFACT_URL,
    workletModuleUrl: WORKLET_URL,
  });
  host.node.connect(context.destination);

  const meterFrames = [];
  const telemetryFrames = [];
  const expected = [new Float32Array(LIVE_CONTROL_FRAMES), new Float32Array(LIVE_CONTROL_FRAMES)];
  let inputPeak = 0;
  for (let block = 0; block < LIVE_CONTROL_BLOCKS; block += 1) {
    const planes = sourcePlanes(block);
    for (let frame = 0; frame < QUANTUM_FRAMES; frame += 1) {
      expected[0][block * QUANTUM_FRAMES + frame] = planes[0][frame] * 0.5;
      expected[1][block * QUANTUM_FRAMES + frame] = planes[1][frame];
      inputPeak = Math.max(inputPeak, Math.abs(planes[0][frame]), Math.abs(planes[1][frame]));
    }
    const acknowledgement = await host.submitSource({
      sourceId: "live-control-source",
      generation: 1n,
      startFrame: BigInt(block * QUANTUM_FRAMES),
      sampleRateHz: SAMPLE_RATE,
      planes,
      frames: QUANTUM_FRAMES,
      endOfRegion: block === LIVE_CONTROL_BLOCKS - 1,
    });
    if (acknowledgement.result !== 0) throw new Error("live-control prefill rejected");
  }

  // Request IDs are strictly monotonic across the whole port, so the leases and the command are
  // taken after the prefill they will be observed over.
  const map = await host.sessionMap();
  const meterLease = await host.meters({
    enabled: true,
    onFrame: (frame) => meterFrames.push(frame),
  });
  const telemetryLease = await host.telemetry({
    enabled: true,
    onFrame: (frame) => telemetryFrames.push(frame),
  });

  // The retarget is a SET_CONTROL transaction, built and sent by the SDK (the served bundle), and
  // committed by the core before the first render: the whole rendered block is post-edit.
  const command = await applyMatrix(host, { ll: 0.5, lr: 0, rl: 0, rr: 1 });
  const tracks = await readTracks(host);

  const rendered = await context.startRendering();
  // Frames posted from the render thread arrive as tasks; yield until the queue is drained.
  for (let spin = 0; spin < 8 && meterFrames.length === 0; spin += 1) {
    await new Promise((resolve) => setTimeout(resolve, 4));
  }
  const renderAllocations = await renderAllocationCount(host);
  await host.dispose();
  const actual = [rendered.getChannelData(0), rendered.getChannelData(1)];

  let exact = true;
  for (let channel = 0; channel < 2; channel += 1) {
    for (let frame = 0; frame < LIVE_CONTROL_FRAMES; frame += 1) {
      if (!Object.is(actual[channel][frame], expected[channel][frame])) exact = false;
    }
  }
  const peaks = meterFrames.map((frame) => Array.from(frame.peaks));
  const masterPeak = peaks.reduce(
    (highest, frame) => Math.max(highest, frame[frame.length - 2], frame[frame.length - 1]),
    0,
  );
  return {
    tracks,
    metersAttached: map.metersAttached,
    commandOk: command.ok,
    commandPath: command.path,
    commandApplied: command.appliedOperations,
    meterLeaseResult: meterLease.result,
    telemetryLeaseResult: telemetryLease.result,
    meterFrames: meterFrames.length,
    telemetryFrames: telemetryFrames.length,
    meterFrameWidth: peaks.length === 0 ? 0 : peaks[0].length,
    masterPeak,
    inputPeak,
    exactRetargetedOutput: exact,
    expectedDigest: await pcmDigest(expected, LIVE_CONTROL_FRAMES),
    renderedDigest: await pcmDigest(actual, LIVE_CONTROL_FRAMES),
    renderAllocations,
  };
}

/// The delay the structural row applies, in samples (#1562b).
const STRUCTURAL_DELAY_SAMPLES = 64;

/// #1562b: a structural apply in a real browser. The live-control session's track gets a
/// `delay_samples` edit, which the core classifies as a rebuild: it prepares a replacement plan and
/// publishes it while the first plan has rendered nothing, so the very first quantum is the swap
/// block (the offline context cannot be suspended in every engine). The row holds what the swap
/// must keep: the reply is the rebuild path, the applied revision is not early, the output is the
/// submitted PCM delayed by exactly the edit (computed here, not read back), the meter stream is the
/// successor's from its first frame (the retired header's views were switched in the swap block,
/// and the frame carries the structure epoch of the list), and no allocator call ran on the
/// render thread.
async function runStructuralApplyQualification(createHost, sessionDocument) {
  const context = new OfflineAudioContext(2, LIVE_CONTROL_FRAMES, SAMPLE_RATE);
  const host = await createHost({
    context,
    document: sessionDocument,
    options: bootOptions(
      LIVE_CONTROL_FRAMES,
      LIVE_CONTROL_METER_BLOCKS,
    ),
    simd128ModuleUrl: ARTIFACT_URL,
    workletModuleUrl: WORKLET_URL,
  });
  host.node.connect(context.destination);

  const meterFrames = [];
  const expected = [new Float32Array(LIVE_CONTROL_FRAMES), new Float32Array(LIVE_CONTROL_FRAMES)];
  const input = [new Float32Array(LIVE_CONTROL_FRAMES), new Float32Array(LIVE_CONTROL_FRAMES)];
  for (let block = 0; block < LIVE_CONTROL_BLOCKS; block += 1) {
    const planes = sourcePlanes(block);
    for (let channel = 0; channel < 2; channel += 1) {
      input[channel].set(planes[channel], block * QUANTUM_FRAMES);
    }
    const acknowledgement = await host.submitSource({
      sourceId: "live-control-source",
      generation: 1n,
      startFrame: BigInt(block * QUANTUM_FRAMES),
      sampleRateHz: SAMPLE_RATE,
      planes,
      frames: QUANTUM_FRAMES,
      endOfRegion: block === LIVE_CONTROL_BLOCKS - 1,
    });
    if (acknowledgement.result !== 0) throw new Error("structural-apply prefill rejected");
  }
  for (let channel = 0; channel < 2; channel += 1) {
    for (let frame = STRUCTURAL_DELAY_SAMPLES; frame < LIVE_CONTROL_FRAMES; frame += 1) {
      expected[channel][frame] = input[channel][frame - STRUCTURAL_DELAY_SAMPLES];
    }
  }
  const meterLease = await host.meters({
    enabled: true,
    onFrame: (frame) => meterFrames.push({
      sequence: frame.sequence,
      trackCount: frame.trackCount,
      structureEpoch: frame.structureEpoch,
      peaks: frame.peaks.length,
      firstSample: frame.firstSample,
      endSample: frame.endSample,
    }),
  });
  const before = await host.status();
  const { applyDelayThroughSdk } = await import("/sdk/sdk-response-client.js");
  const command = await applyDelayThroughSdk(host, "track", STRUCTURAL_DELAY_SAMPLES);
  const pending = await host.status();

  const rendered = await context.startRendering();
  for (let spin = 0; spin < 8 && meterFrames.length === 0; spin += 1) {
    await new Promise((resolve) => setTimeout(resolve, 4));
  }
  const after = await host.status();
  const renderAllocations = await renderAllocationCount(host);
  await host.dispose();
  const actual = [rendered.getChannelData(0), rendered.getChannelData(1)];

  let exact = true;
  for (let channel = 0; channel < 2; channel += 1) {
    for (let frame = 0; frame < LIVE_CONTROL_FRAMES; frame += 1) {
      if (!Object.is(actual[channel][frame], expected[channel][frame])) exact = false;
    }
  }
  let increasing = true;
  for (let index = 1; index < meterFrames.length; index += 1) {
    if (meterFrames[index].sequence <= meterFrames[index - 1].sequence) increasing = false;
  }
  return {
    commandOk: command.ok,
    commandPath: command.path,
    commandApplied: command.appliedOperations,
    committedRevision: command.revision,
    meterLeaseResult: meterLease.result,
    initialRevision: before.appliedRevision.toString(),
    pendingRevision: pending.appliedRevision.toString(),
    finalRevision: after.appliedRevision.toString(),
    finalSample: after.appliedSample.toString(),
    meterFrames: meterFrames.length,
    meterFrameWidth: meterFrames.length === 0 ? 0 : meterFrames[0].peaks,
    meterTrackCount: meterFrames.length === 0 ? 0 : meterFrames[0].trackCount,
    structureEpochs: [...new Set(meterFrames.map((frame) => frame.structureEpoch))],
    sequenceIncreasing: increasing,
    exactDelayedOutput: exact,
    expectedDigest: await pcmDigest(expected, LIVE_CONTROL_FRAMES),
    renderedDigest: await pcmDigest(actual, LIVE_CONTROL_FRAMES),
    renderAllocations,
  };
}

/// #1682: an empty session in a real browser. The live-control session with no source, no track and
/// no route boots and renders silence, and one structural transaction adds the source, the track
/// and the route back, after which the fed PCM comes out of the output unchanged.
///
/// Two renders of the same session. The first only boots it and renders: the output is exact zero,
/// the session map and the control list name no source and no strip, the meter stream (the lease is
/// taken on the empty plan) is the master's two peaks with no track, and no allocator call ran on
/// the render thread. The second applies the transaction (the frame is the protocol codec's, the
/// fixture beside the session), feeds the added source before the first quantum, and renders: the
/// first quantum is the swap block, so the output is the submitted PCM, exactly, computed here and
/// not read back; the meter frame grows to the strip, and no render-thread allocator call ran.
async function runEmptyBootQualification(createHost, emptyDocument, frameFixture) {
  const boot = async () => {
    const context = new OfflineAudioContext(2, LIVE_CONTROL_FRAMES, SAMPLE_RATE);
    const host = await createHost({
      context,
      document: emptyDocument,
      options: bootOptions(LIVE_CONTROL_FRAMES, LIVE_CONTROL_METER_BLOCKS),
      simd128ModuleUrl: ARTIFACT_URL,
      workletModuleUrl: WORKLET_URL,
    });
    host.node.connect(context.destination);
    return { context, host };
  };
  const summarise = (frame) => ({
    sequence: frame.sequence,
    trackCount: frame.trackCount,
    structureEpoch: frame.structureEpoch,
    peaks: frame.peaks.length,
  });
  const nonzero = (pcm) => pcm.some((channel) => channel.some((sample) => !Object.is(sample, 0)));

  // The empty boot alone.
  const quiet = await boot();
  const quietFrames = [];
  const quietMap = await quiet.host.sessionMap();
  const quietTracks = await readTracks(quiet.host);
  const quietLease = await quiet.host.meters({
    enabled: true,
    onFrame: (frame) => quietFrames.push(summarise(frame)),
  });
  const quietRendered = await quiet.context.startRendering();
  for (let spin = 0; spin < 8 && quietFrames.length === 0; spin += 1) {
    await new Promise((resolve) => setTimeout(resolve, 4));
  }
  const quietStatus = await quiet.host.status();
  const quietAllocations = await renderAllocationCount(quiet.host);
  await quiet.host.dispose();
  const quietPcm = [quietRendered.getChannelData(0), quietRendered.getChannelData(1)];

  // The first stem by one structural transaction.
  const grown = await boot();
  const grownFrames = [];
  const input = [new Float32Array(LIVE_CONTROL_FRAMES), new Float32Array(LIVE_CONTROL_FRAMES)];
  const grownLease = await grown.host.meters({
    enabled: true,
    onFrame: (frame) => grownFrames.push(summarise(frame)),
  });
  const { sendStructuralFrameThroughSdk } = await import("/sdk/sdk-response-client.js");
  const reply = await sendStructuralFrameThroughSdk(grown.host, frameFixture.hex, frameFixture.revision);
  const afterMap = await grown.host.sessionMap();
  const afterTracks = await readTracks(grown.host);
  for (let block = 0; block < LIVE_CONTROL_BLOCKS; block += 1) {
    const planes = sourcePlanes(block);
    for (let channel = 0; channel < 2; channel += 1) {
      input[channel].set(planes[channel], block * QUANTUM_FRAMES);
    }
    const acknowledgement = await grown.host.submitSource({
      sourceId: "live-control-source",
      generation: 1n,
      startFrame: BigInt(block * QUANTUM_FRAMES),
      sampleRateHz: SAMPLE_RATE,
      planes,
      frames: QUANTUM_FRAMES,
      endOfRegion: block === LIVE_CONTROL_BLOCKS - 1,
    });
    if (acknowledgement.result !== 0) throw new Error("empty-boot prefill rejected");
  }
  const pending = await grown.host.status();
  const grownRendered = await grown.context.startRendering();
  for (let spin = 0; spin < 8 && grownFrames.length === 0; spin += 1) {
    await new Promise((resolve) => setTimeout(resolve, 4));
  }
  const grownStatus = await grown.host.status();
  const grownAllocations = await renderAllocationCount(grown.host);
  await grown.host.dispose();
  const actual = [grownRendered.getChannelData(0), grownRendered.getChannelData(1)];
  let exact = true;
  for (let channel = 0; channel < 2; channel += 1) {
    for (let at = 0; at < LIVE_CONTROL_FRAMES; at += 1) {
      if (!Object.is(actual[channel][at], input[channel][at])) exact = false;
    }
  }
  return {
    boot: {
      sources: quietMap.sources.length,
      tracks: quietTracks.length,
      metersAttached: quietMap.metersAttached,
      meterLeaseResult: quietLease.result,
      silent: !nonzero(quietPcm),
      renderedQuanta: quietStatus.renderedQuanta.toString(),
      expectedQuanta: LIVE_CONTROL_BLOCKS,
      meterFrames: quietFrames.length,
      meterTrackCounts: [...new Set(quietFrames.map((entry) => entry.trackCount))],
      meterWidths: [...new Set(quietFrames.map((entry) => entry.peaks))],
      renderAllocations: quietAllocations,
    },
    firstStem: {
      replyStatus: reply.status,
      commandPath: reply.path,
      replyBytes: reply.replyBytes,
      sourcesAfter: afterMap.sources.length,
      tracksAfter: afterTracks,
      meterLeaseResult: grownLease.result,
      initialRevision: frameFixture.revision,
      pendingRevision: pending.appliedRevision.toString(),
      committedRevision: reply.revision,
      finalRevision: grownStatus.appliedRevision.toString(),
      finalSample: grownStatus.appliedSample.toString(),
      exactInputOutput: exact,
      heard: nonzero(actual),
      expectedDigest: await pcmDigest(input, LIVE_CONTROL_FRAMES),
      renderedDigest: await pcmDigest(actual, LIVE_CONTROL_FRAMES),
      meterFrames: grownFrames.length,
      meterTrackCounts: [...new Set(grownFrames.map((entry) => entry.trackCount))],
      meterWidths: [...new Set(grownFrames.map((entry) => entry.peaks))],
      structureEpochs: [...new Set(grownFrames.map((entry) => entry.structureEpoch))],
      renderAllocations: grownAllocations,
    },
  };
}

/// Issue #143 E12: subscribe -> nonzero `trackGrDb` in a real browser, and an unarmed tap publishes none.
///
/// Two offline renders of the same sixteen blocks over the same session, differing only in whether
/// the tap is armed when rendering starts. That is E1's leg (c) against leg (d) in a browser: the
/// two renders must produce **identical PCM**, because arming a declared tap may not move a sample.
///
/// `armed` is `true` for the first run and `false` for the second, which never subscribes. The
/// engine admits one observation record per tap per render (#1345 D8, D2 Amendments 1 and 2), and an
/// offline context renders only inside one `startRendering` call (Firefox has no
/// `OfflineAudioContext.suspend`), so a subscribe followed at once by an unsubscribe is refused with
/// typed backpressure here. That an unsubscribe stops a tap's reading is proved by the Rust test
/// `a_withdrawn_tap_leaves_the_first_polled_frame_and_only_its_own_reading` (`hosts/host-web`), not by
/// this page (#1345 Amendment 7).
async function runObservationRun(createHost, sessionDocument, armed) {
  const context = new OfflineAudioContext(2, OBSERVATION_FRAMES, SAMPLE_RATE);
  const host = await createHost({
    context,
    document: sessionDocument,
    options: bootOptions(
      OBSERVATION_FRAMES,
      LIVE_CONTROL_METER_BLOCKS,
      OBSERVATION_TAPS,
    ),
    simd128ModuleUrl: ARTIFACT_URL,
    workletModuleUrl: WORKLET_URL,
  });
  host.node.connect(context.destination);
  const master = await designateMaster(host);
  if (!master.ok || master.path !== "monitor") throw new Error("observation master designation refused");

  const frames = [];
  for (let block = 0; block < OBSERVATION_BLOCKS; block += 1) {
    const acknowledgement = await host.submitSource({
      sourceId: "live-control-source",
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
    stripId: "track",
    rack: 1,
    effectId: "comp",
    tapId: OBSERVATION_TAP_ID,
    windowBlocks: Number(LIVE_CONTROL_METER_BLOCKS),
    armed: true,
  };
  const subscribed = armed ? await host.observe({ subscriptions: [subscription] }) : null;

  const rendered = await context.startRendering();
  for (let spin = 0; spin < 8 && frames.length === 0; spin += 1) {
    await new Promise((resolve) => setTimeout(resolve, 4));
  }
  const renderAllocations = await renderAllocationCount(host);
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
    subscribeResult: subscribed === null ? null : subscribed.result,
    subscribeReason: subscribed === null ? null : subscribed.reason,
    bindings: subscribed === null ? 0 : subscribed.bindings.length,
    stripId: subscribed?.bindings[0]?.stripId ?? null,
    windowBlocks: subscribed?.bindings[0]?.windowBlocks ?? -1,
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
    renderAllocations,
  };
}

/// Every control the page asks the harness to click, in the order the page offers them, as selectors
/// (batch-verdict 2, B2; root 2026-10-12). The page declares this plan and nothing else may resume a
/// real `AudioContext`: `run.mjs` clicks exactly these controls under one deadline and fails by name
/// when the page resumed a context any other way, offered another control, or did not offer one.
/// The two controls belong to `sdk-response-entry.ts`'s collection and hop workloads, which run
/// only when the SDK qualification does; without it the plan is empty.
export function resumeClickPlan(sdkEnabled) {
  if (!sdkEnabled || !WebAssembly.validate(SIMD128_PROBE)) return [];
  return ["#spectrum-collection-resume", "#spectrum-hop-resume"];
}

/// Count every `AudioContext.resume()` of the page, split by whether it ran synchronously inside a
/// trusted click handler (`window.event` is that click) or anywhere else (`direct`). Offline contexts
/// are not counted: they need no activation. The count is part of the result, so a `resume()` that
/// is not inside a click handler (an SDK workload's, or one added later) turns the
/// `resume-clicks` gate red whatever the browser happens to allow.
function countResumes() {
  const record = { inClick: 0, direct: 0 };
  if (typeof AudioContext !== "function") return record;
  const resume = AudioContext.prototype.resume;
  AudioContext.prototype.resume = function countedResume(...args) {
    const event = window.event;
    if (event?.type === "click" && event.isTrusted) record.inClick += 1;
    else record.direct += 1;
    return resume.apply(this, args);
  };
  return record;
}

/// Issue #143 E12: the observation row and its two offline runs.
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
    // #137 E6: the stall runs with live controls attached and their meter lease held, so the
    // 100 ms fault is survived under exactly the command and metering load a mixing console
    // imposes -- and the frozen exact-output requirement is unchanged.
    options: bootOptions(
      DEFAULT_RING_FRAMES,
      LIVE_CONTROL_METER_BLOCKS,
    ),
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
    enabled: true,
    onFrame: (frame) => stallMeterFrames.push(frame),
  });
  // #137 E6: an identity retarget admitted immediately before the fault. It changes no
  // coefficient -- the stall session's pan is already the identity matrix -- so the frozen exact
  // digest still applies, while the control path, its queue and the meter fold are all live
  // across the stall.
  const stallCommand = await applyMatrix(host, { ll: 1, lr: 0, rl: 0, rr: 1 });

  // An exact-length offline context needs no non-portable suspend point: rendering begins before
  // the main-realm fault and consumes all 40 prefilled quanta while that realm is unavailable.
  const rendering = context.startRendering();
  const measuredStallMs = busyWait(REQUESTED_STALL_MS);
  const rendered = await rendering;
  const status = await host.status();
  const renderAllocations = await renderAllocationCount(host);
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
    liveControlCommandOk: stallCommand.ok,
    liveControlMeterLeaseResult: meterLease.result,
    liveControlMeterFrames: stallMeterFrames.length,
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
    renderAllocations,
  };
}

/// Issue #1333 D3 / Amendment 1: the worklet's staging reads, each inside a render-locked window.
///
/// The observation session boots with a two-entry spectrum collection -- the track's post-pan
/// boundary and the main output -- and selects the second entry, so each spectrum read resolves a
/// collection's selected entry, the path that once cloned its target on every read. Two instances,
/// because one offline render cannot both complete a one-shot window and stream one: `oneShot`
/// reads a completed one-shot window, then an observation row, then a track response, then a
/// stream read just after its start; `stream` renders with the stream already running and reads a
/// completed stream window. Each reports its own count before its dispose.
async function runStagingReadRun(createHost, sessionDocument, streaming) {
  const context = new OfflineAudioContext(2, STAGING_READ_FRAMES, SAMPLE_RATE);
  const host = await createHost({
    context,
    document: sessionDocument,
    options: {
      ...bootOptions(
        STAGING_READ_FRAMES,
        LIVE_CONTROL_METER_BLOCKS,
        OBSERVATION_TAPS,
      ),
      spectrum: null,
      spectrumCollection: {
        entries: [
          { target: "trackPostPan", targetId: "track", channels: "both" },
          { target: "output", targetId: "main-out", channels: "both" },
        ],
        maximumCaptureBytes: SPECTRUM_CAPTURE_BYTES * 2,
      },
    },
    simd128ModuleUrl: ARTIFACT_URL,
    workletModuleUrl: WORKLET_URL,
  });
  host.node.connect(context.destination);
  for (let block = 0; block < STAGING_READ_BLOCKS; block += 1) {
    const acknowledgement = await host.submitSource({
      sourceId: "live-control-source",
      generation: 1n,
      startFrame: BigInt(block * QUANTUM_FRAMES),
      sampleRateHz: SAMPLE_RATE,
      planes: observationPlanes(block),
      frames: QUANTUM_FRAMES,
      endOfRegion: block === STAGING_READ_BLOCKS - 1,
    });
    if (acknowledgement.result !== RESULT_OK) throw new Error("staging-read prefill rejected");
  }
  const selected = await host.selectSpectrum({
    target: "output", targetId: "main-out", channels: "both",
  });
  const run = { selectResult: selected.result };
  if (streaming) {
    run.streamStartResult = (await host.startSpectrumStream(0)).result;
    await context.startRendering();
    const read = await host.readSpectrumStream(new ArrayBuffer(SPECTRUM_CAPTURE_BYTES));
    run.streamReadResult = read.result;
    run.streamReadBytes = read.byteLength;
  } else {
    const subscribed = await host.observe({
      subscriptions: [{
        stripId: "track",
        rack: 1,
        effectId: "comp",
        tapId: OBSERVATION_TAP_ID,
        windowBlocks: Number(LIVE_CONTROL_METER_BLOCKS),
        armed: true,
      }],
    });
    run.subscribeResult = subscribed.result;
    await context.startRendering();
    const spectrum = await host.readSpectrum({ channels: "both" });
    run.spectrumReadResult = spectrum.result;
    run.spectrumReadBytes = spectrum.snapshot.byteLength;
    const observation = await host.readObservations({
      selections: [{ trackIndex: 0, rack: 1, effectIndex: 0, tapId: OBSERVATION_TAP_ID, channels: 3 }],
    });
    run.observationReadResult = observation.result;
    run.observationRows = observation.rows.length;
    const response = await host.captureTrackResponse({
      trackId: "track",
      grid: 1,
      channels: 3,
      points: 5,
      minimumHz: 20,
      maximumHz: 20_000,
      maximumResultBytes: SPECTRUM_CAPTURE_BYTES,
    });
    run.trackResponseResult = response.result;
    run.trackResponseBytes = response.snapshot.byteLength;
    run.streamStartResult = (await host.startSpectrumStream(0)).result;
    const read = await host.readSpectrumStream(new ArrayBuffer(SPECTRUM_CAPTURE_BYTES));
    run.streamReadResult = read.result;
    run.streamReadBytes = read.byteLength;
  }
  run.renderAllocations = await renderAllocationCount(host);
  await host.dispose();
  return run;
}

async function runStagingReadQualification(createHost, sessionDocument) {
  return {
    oneShot: await runStagingReadRun(createHost, sessionDocument, false),
    stream: await runStagingReadRun(createHost, sessionDocument, true),
  };
}

export async function runQualification() {
  const resumeRecord = countResumes();
  const [
    { createMisoAudioWorkletHost }, expectedResponse, sessionResponse, sourceResponse, stallResponse,
    liveControlResponse, observationResponse, emptyResponse, emptyFrameResponse,
  ] =
    await Promise.all([
      import("/artifacts/miso-engine-v1-audio-worklet-host.js"),
      fetch("/fixture/expected.json"),
      fetch("/fixture/session.json"),
      fetch("/fixture/source.json"),
      fetch("/qualification/stall-session.json"),
      fetch("/qualification/live-control-session.json"),
      fetch("/qualification/observation-session.json"),
      fetch("/qualification/empty-session.json"),
      fetch("/qualification/empty-boot-frame.json"),
    ]);
  if (!expectedResponse.ok || !sessionResponse.ok || !sourceResponse.ok || !stallResponse.ok
      || !liveControlResponse.ok || !observationResponse.ok || !emptyResponse.ok
      || !emptyFrameResponse.ok) {
    throw new Error("qualification fixture fetch failed");
  }
  const expected = await expectedResponse.json();
  const corpusDocument = new TextEncoder().encode(await sessionResponse.text());
  const source = await sourceResponse.json();
  const stallDocument = new TextEncoder().encode(await stallResponse.text());
  // Issue #137 E8: the live-control row needs a region long enough for one full 128-block
  // telemetry window, which the 40-block stall region is not.
  const liveControlDocument = new TextEncoder().encode(await liveControlResponse.text());
  // Issue #143 E12: the live-control session plus one compressor, so an armed tap has a real
  // reduction.
  const observationDocument = new TextEncoder().encode(await observationResponse.text());
  // #1682: the live-control session with no source, track or route, and the frame that adds them.
  const emptyDocument = new TextEncoder().encode(await emptyResponse.text());
  const emptyFrame = await emptyFrameResponse.json();
  const simd128 = WebAssembly.validate(SIMD128_PROBE);
  const sdkResponse = simd128 && new URL(import.meta.url).searchParams.get("sdk") === "1"
    ? await (await import("/sdk/sdk-response-client.js")).runSdkResponseQualification()
    : null;

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
      liveControls: null,
      structuralApply: null,
      emptyBoot: null,
      observation: null,
      stall: null,
      stagingReads: null,
      renderAllocations: null,
      sdkResponse,
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
    live = await runLiveControlQualification(createMisoAudioWorkletHost, liveControlDocument);
  } catch (error) {
    const diagnostic = await diagnoseReady(liveControlDocument);
    throw new Error(`live-control qualification failed: ${diagnosticJson({
      name: error?.name, message: error?.message, ...error, diagnostic,
    })}`);
  }
  let structural;
  try {
    structural = await runStructuralApplyQualification(
      createMisoAudioWorkletHost,
      liveControlDocument,
    );
  } catch (error) {
    const diagnostic = await diagnoseReady(liveControlDocument);
    throw new Error(`structural-apply qualification failed: ${diagnosticJson({
      name: error?.name, message: error?.message, ...error, diagnostic,
    })}`);
  }
  let emptyBoot;
  try {
    emptyBoot = await runEmptyBootQualification(createMisoAudioWorkletHost, emptyDocument, emptyFrame);
  } catch (error) {
    const diagnostic = await diagnoseReady(emptyDocument);
    throw new Error(`empty-boot qualification failed: ${diagnosticJson({
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
  let stagingReads;
  try {
    stagingReads = await runStagingReadQualification(
      createMisoAudioWorkletHost,
      observationDocument,
    );
  } catch (error) {
    throw new Error(`staging-read qualification failed: ${diagnosticJson({
      name: error?.name, message: error?.message, ...error,
    })}`);
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
    liveControls: live,
    structuralApply: structural,
    emptyBoot,
    observation,
    stall,
    stagingReads,
    // Issue #1333 D3: every booted instance's render-locked allocator count, each read before
    // that instance's dispose. Decision 15 requires every one to be zero.
    renderAllocations: [
      ...correctness.runs.flatMap((run, index) => run.renderAllocations.map((count, segment) => ({
        workload: `corpus-${index}-${segment}`, count,
      }))),
      { workload: "live-control", count: live.renderAllocations },
      { workload: "structural-apply", count: structural.renderAllocations },
      { workload: "empty-boot", count: emptyBoot.boot.renderAllocations },
      { workload: "empty-boot-first-stem", count: emptyBoot.firstStem.renderAllocations },
      { workload: "observation-armed", count: observation.armed.renderAllocations },
      { workload: "observation-disarmed", count: observation.disarmed.renderAllocations },
      { workload: "stall", count: stall.renderAllocations },
      { workload: "staging-reads-one-shot", count: stagingReads.oneShot.renderAllocations },
      { workload: "staging-reads-stream", count: stagingReads.stream.renderAllocations },
    ],
    sdkResponse,
    // Every `AudioContext.resume()` of the page, for the `resume-clicks` gate: `declared` is the click
    // plan's length, which `run.mjs` also holds the harness to.
    resumes: { declared: resumeClickPlan(sdkResponse !== null).length, ...resumeRecord },
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

// Issue #288: these are the existing qualification call sites, exposed only to the hermetic
// contract test. The browser runner continues to enter through runQualification below; keeping
// the references here direct means the test executes the same boot objects that production uses.
export const qualificationBootContract = Object.freeze({
  renderCorpusSegment,
  typedUnsupportedAttestation,
  runLiveControlQualification,
  runStructuralApplyQualification,
  runEmptyBootQualification,
  runObservationRun,
  runStallQualification,
  runStagingReadRun,
  diagnoseReady,
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
    document: "live-control-session.json",
    sourceId: "live-control-source",
    blocks: LIVE_CONTROL_BLOCKS,
    planes: sourcePlanes,
  }),
  Object.freeze({
    document: "observation-session.json",
    sourceId: "live-control-source",
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
