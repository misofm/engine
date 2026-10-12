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
const LIVE_CONTROL_COMMAND_QUEUE_RECORDS = 64n;
const LIVE_CONTROL_METER_BLOCKS = 2n;
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
// #1345 Amendment 6: the real-context unsubscribe run, over its own session document
// (`unsubscribe-session.json`, a 512-block source: the observation document's sixteen blocks cannot
// hold the armed blocks, a render, and sixteen blocks after the acknowledgement). A one-block meter window, so each frame
// carries one block's peak; a source fed twelve blocks ahead of the render (~32 ms, well inside the
// 5,120-frame default ring) and no further; sixteen blocks of frames after the acknowledgement.
const UNSUBSCRIBE_SOURCE_BLOCKS = 512;
const UNSUBSCRIBE_METER_BLOCKS = 1n;
const UNSUBSCRIBE_LEAD_BLOCKS = 12;
const UNSUBSCRIBE_ARMED_FRAMES = 4;
const UNSUBSCRIBE_MINIMUM_BLOCKS = 16;
// The fed signal is a +-0.5 square wave; the compressor's reduction at that level (about 5 dB, in
// the armed frames) leaves a track peak near 0.27, far above this floor, and a block of zeros (an
// underrun) peaks at 0.
const UNSUBSCRIBE_PEAK_FLOOR = 0.05;
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
  liveControlCommandQueueRecords = 0n,
  liveControlMeterBlocks = 0n,
  liveControlObservationTaps = 0n,
  liveControlMasterTrackPlusOne = 0n,
) {
  return {
    sourceRingFrames,
    // Zero is "no caller-imposed memory ceiling", the same word `tests/browser-v1` sends and the
    // same one the direct oracle writes at boot-options offset 24.
    maximumMemoryBytes: 0n,
    liveControlCommandQueueRecords,
    liveControlMeterBlocks,
    // Issue #143 D3/D6: the frozen configuration's remaining two reserved words. Zero in both is
    // the pre-#143 shape, which is what every row but the observation one keeps using.
    liveControlObservationTaps,
    liveControlMasterTrackPlusOne,
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

// The unsubscribe run's fed block: a square wave of +-0.5, sixteen samples per half period, on both
// channels. It is not the constant of `observationPlanes`: the strip's input high-pass removes a
// constant, so a fed DC level decays to nothing within a few blocks and a later block could not tell
// signal from silence. Every block is identical (128 is four whole periods), the values are exact in
// binary32, and `blockIndex` is unused, as in every fed-PCM generator here.
function unsubscribePlanes(_blockIndex) {
  const storage = new ArrayBuffer(QUANTUM_FRAMES * 2 * Float32Array.BYTES_PER_ELEMENT);
  const left = new Float32Array(storage, 0, QUANTUM_FRAMES);
  const right = new Float32Array(
    storage,
    QUANTUM_FRAMES * Float32Array.BYTES_PER_ELEMENT,
    QUANTUM_FRAMES,
  );
  for (let frame = 0; frame < QUANTUM_FRAMES; frame += 1) {
    const level = (frame >> 4) % 2 === 0 ? OBSERVATION_LEVEL : -OBSERVATION_LEVEL;
    left[frame] = level;
    right[frame] = level;
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

/// #137 E8: a live-control row -- one parameter change applied and one meter frame received.
///
/// The command is awaited before rendering starts, so its acknowledgement names sample `0` and the
/// whole rendered block is post-command: the browser leg proves *that* a change reached the DSP
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
      LIVE_CONTROL_COMMAND_QUEUE_RECORDS,
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

  const command = await host.command({
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
    expectedDigest: await pcmDigest(expected, LIVE_CONTROL_FRAMES),
    renderedDigest: await pcmDigest(actual, LIVE_CONTROL_FRAMES),
    renderAllocations,
  };
}

/// Issue #143 E12: subscribe -> nonzero `trackGrDb` -> unsubscribe -> zero, in a real browser.
///
/// Two offline renders of the same sixteen blocks over the same session, differing only in whether
/// the tap is armed when rendering starts. That is E1's leg (c) against leg (d) in a browser: the
/// two renders must produce **identical PCM**, because arming a declared tap may not move a sample.
///
/// `armed` is `true` for the first run and `false` for the second, which never subscribes. That the
/// tap stops publishing after an unsubscribe is [`runObservationUnsubscribeRun`]'s, on a real
/// context: the engine admits one observation record per tap per render (#1345 D8, D2 Amendments
/// 1 and 2), so the unsubscribe needs a render after the subscribe, and an offline context renders
/// only inside one `startRendering` call (Firefox has no `OfflineAudioContext.suspend`).
async function runObservationRun(createHost, sessionDocument, armed) {
  const context = new OfflineAudioContext(2, OBSERVATION_FRAMES, SAMPLE_RATE);
  const host = await createHost({
    context,
    document: sessionDocument,
    options: bootOptions(
      OBSERVATION_FRAMES,
      LIVE_CONTROL_COMMAND_QUEUE_RECORDS,
      LIVE_CONTROL_METER_BLOCKS,
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
    trackIndex: 0,
    rack: 1,
    effectIndex: 0,
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
    frameSlot: subscribed?.bindings[0]?.frameSlot ?? -1,
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

function delay(milliseconds) {
  return new Promise((resolve) => setTimeout(resolve, milliseconds));
}

/// The bound on every wait that depends on an audio context or on a host reply in the real-context
/// run, the same 10 s as `sdk-response-entry.ts` uses for its `suspend()` and `resume()` bounds.
const AUDIO_STEP_TIMEOUT_MS = 10_000;

/// Await `start()`, failing with `<kind> timeout at <step>` and the context's state if it has not
/// settled within `AUDIO_STEP_TIMEOUT_MS`. A timeout is a failure of the gate: never a pass, a skip
/// or a retry. WebKit leaves a `resume()` that has no user activation pending (batch-verdict 2, B2);
/// an unbounded wait there hangs the page until CI kills the job.
async function bounded(kind, step, context, start) {
  let timer;
  const stalled = new Promise((_, reject) => {
    timer = setTimeout(() => reject(new Error(
      `${kind} timeout at ${step} after ${AUDIO_STEP_TIMEOUT_MS} ms: state=${context.state} currentTime=${context.currentTime}`,
    )), AUDIO_STEP_TIMEOUT_MS);
  });
  try {
    return await Promise.race([start(), stalled]);
  } finally {
    clearTimeout(timer);
  }
}

/// Fail with `suspend timeout at <step>` unless `context.state` is "suspended" within the bound.
/// The first `suspend()` of a new context is not awaited (Firefox can create a later context already
/// running, and the `suspend()` is only a request), so this poll is what keeps the
/// gate from going on with a context that is still running: the boot needs a suspended context.
/// Returns the state it saw, which the run records.
async function requireSuspended(context, step) {
  const started = performance.now();
  while (context.state !== "suspended") {
    if (performance.now() - started > AUDIO_STEP_TIMEOUT_MS) {
      throw new Error(
        `suspend timeout at ${step} after ${AUDIO_STEP_TIMEOUT_MS} ms: state=${context.state} currentTime=${context.currentTime}`,
      );
    }
    await delay(4);
  }
  return context.state;
}

/// The label of the one trusted-click resume `runObservationUnsubscribeRun` makes (#1345 Amendment 6).
/// The click plan below names the button by this label, so the gate and its plan cannot drift apart.
const UNSUBSCRIBE_RESUME = "the armed tap to render";
const UNSUBSCRIBE_AFTER_WAIT = "the blocks after the unsubscribe";

/// Every control the page asks the harness to click, in the order the page offers them, as selectors
/// (batch-verdict 2, B2; root 2026-10-12). The page declares this plan and nothing else may resume a
/// real `AudioContext`: `run.mjs` clicks exactly these controls under one deadline and fails by name
/// when the page resumed a context any other way, offered another control, or did not offer one.
/// The two SDK controls belong to `sdk-response-entry.ts`'s collection and hop workloads, which run
/// first; the last is `resumeThroughClick`'s button.
export function resumeClickPlan(sdkEnabled) {
  if (!WebAssembly.validate(SIMD128_PROBE)) return [];
  return [
    ...(sdkEnabled ? ["#spectrum-collection-resume", "#spectrum-hop-resume"] : []),
    resumeControlSelector(UNSUBSCRIBE_RESUME),
  ];
}

function resumeControlSelector(what) {
  return `button[data-qualification-resume="${what}"]`;
}

/// Count every `AudioContext.resume()` of the page, split by whether it ran synchronously inside a
/// trusted click handler (`window.event` is that click) or anywhere else (`direct`). Offline contexts
/// are not counted: they need no activation. The count is part of the result, so a `resume()` that
/// bypasses [`resumeThroughClick`] (the gate's own, or one added to the SDK workloads) turns the
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

/// Resume `context` with a trusted click on a button this function adds, the way an app must in
/// Safari. WebKit starts a new `AudioContext`, or restarts a suspended one, only inside a user
/// activation, and otherwise leaves `resume()` pending (state "suspended", or "interrupted"); it
/// neither rejects nor settles. The harness's `page.evaluate` activation lasts only a short while,
/// so a page that has run longer than that before it resumes (the SDK phase makes it so, much
/// longer on a busy machine) has none left (batch-verdict 2, B2). `run.mjs` clicks the button named
/// by [`resumeClickPlan`], which is a trusted event in all three browsers. The wait for the click is
/// bounded like every other wait here.
async function resumeThroughClick(context, what) {
  const button = document.createElement("button");
  button.setAttribute("data-qualification-resume", what);
  button.textContent = `Resume qualification audio: ${what}`;
  document.body.append(button);
  try {
    await bounded("resume", `${what} (click)`, context, () => new Promise((resolve, reject) => {
      button.addEventListener("click", () => context.resume().then(resolve, reject), { once: true });
    }));
  } finally {
    button.remove();
  }
}

/// How often `resumeUntil` asks the host for its status while it waits, and how long one such
/// request may stay unanswered before the poll counts as unanswered.
const STALL_POLL_INTERVAL_MS = 250;
const STALL_POLL_TIMEOUT_MS = 1_000;

/// The signature of a stall that `resumeUntil` records (root ruling, 2026-10-12, decision 15 stream B
/// batch 3). It is `full` only if a stalled render is the whole of the evidence: the context says it
/// is "running", its clock (`currentTime`) and the engine's `renderedQuanta` are the same at the
/// resume and at the timeout, the processor answered at least two status requests inside the window
/// and its last render result is 0. The status replies come from the processor's own port handler,
/// which runs in the same JS agent as `process()`: an answered reply proves `process()` was not on
/// the stack, so a context that "ran" and rendered nothing is a context that was not pulling its
/// graph. Only a failure with a full signature may be listed as a browser exemption; CI still fails
/// by name and never retries. `atResume` and `atTimeout` are `sampleContext` records (or `null`
/// when the sample was not answered); `replyIds` are the request ids of the answered status
/// requests in the window, in order.
export function classifyStallSignature({ atResume, atTimeout, repliesSent, replyIds }) {
  const running = (sample) => sample?.contextState === "running";
  const lastResult = (sample) => sample?.status?.lastResult;
  // The host status carries `renderedQuanta` as a `bigint`; the signature carries it as a decimal
  // string so it serializes.
  const quanta = (sample) => sample?.status?.renderedQuanta === undefined
    ? null
    : String(sample.status.renderedQuanta);
  const answered = replyIds.length;
  const full = running(atResume) && running(atTimeout)
    && typeof atResume.currentTime === "number" && atResume.currentTime === atTimeout.currentTime
    && quanta(atResume) !== null && quanta(atResume) === quanta(atTimeout)
    && answered >= 2
    && lastResult(atResume) === RESULT_OK && lastResult(atTimeout) === RESULT_OK;
  return {
    state: atTimeout?.contextState ?? null,
    stateAtResume: atResume?.contextState ?? null,
    currentTimeAtResume: atResume?.currentTime ?? null,
    currentTimeAtTimeout: atTimeout?.currentTime ?? null,
    renderedQuantaAtResume: quanta(atResume),
    renderedQuantaAtTimeout: quanta(atTimeout),
    repliesSent,
    repliesAnswered: answered,
    replyIds: answered === 0 ? [] : [replyIds[0], replyIds[answered - 1]],
    lastResult: lastResult(atTimeout) ?? null,
    lastResultAtResume: lastResult(atResume) ?? null,
    full,
  };
}

/// One bounded `describe()`: the sample, or `null` if it threw or stayed unanswered for
/// `STALL_POLL_TIMEOUT_MS`. A reply that arrives after the bound is dropped without an unhandled
/// rejection.
async function boundedSample(describe) {
  const pending = describe();
  pending.catch(() => {});
  let timer;
  const unanswered = new Promise((resolve) => {
    timer = setTimeout(() => resolve(null), STALL_POLL_TIMEOUT_MS);
  });
  try {
    return await Promise.race([pending, unanswered]);
  } catch {
    return null;
  } finally {
    clearTimeout(timer);
  }
}

/// Resume `context` through a trusted click and wait until `condition` holds, for at most
/// `AUDIO_STEP_TIMEOUT_MS`. See [`renderUntil`] for the wait; the click is the only resume.
async function resumeUntil(context, condition, what, describe) {
  await resumeThroughClick(context, what);
  await renderUntil(context, condition, what, describe);
}

/// Wait, on a context that is already running, until `condition` holds, for at most
/// `AUDIO_STEP_TIMEOUT_MS`. There is no recovery: a context that does not pull its graph is a named
/// failure of the gate, never a retry (root, 2026-10-12; a script `suspend()` then `resume()` hung
/// the whole page in 3 of 8 runs of the minimal repro, `/tmp/claude-1002/b2diag/drv5.out`). The
/// failure carries `describe()` sampled twice across the window, at the start and at the
/// timeout, so a stalled clock (`currentTime`) and a stalled render count (`renderedQuanta`) show as
/// equal samples. While it waits it also asks for the host status every `STALL_POLL_INTERVAL_MS`
/// (each request bounded), and the failure carries the `stallSignature` of
/// [`classifyStallSignature`]. The polls change no pass condition and no bound.
async function renderUntil(context, condition, what, describe) {
  const started = performance.now();
  const atResume = await describe();
  let lastPoll = started;
  let repliesSent = 0;
  const replyIds = [];
  const poll = async () => {
    repliesSent += 1;
    const sample = await boundedSample(describe);
    if (sample?.status !== undefined) replyIds.push(sample.status.requestId);
    return sample;
  };
  while (!(await condition())) {
    const now = performance.now();
    if (now - started > AUDIO_STEP_TIMEOUT_MS) {
      const atTimeout = await poll();
      const error = new Error(`stall timeout at ${what} after ${AUDIO_STEP_TIMEOUT_MS} ms: ${diagnosticJson({
        atResume,
        atTimeout,
      })}`);
      error.stallSignature = classifyStallSignature({ atResume, atTimeout, repliesSent, replyIds });
      throw error;
    }
    if (now - lastPoll >= STALL_POLL_INTERVAL_MS) {
      await poll();
      lastPoll = performance.now();
    }
    await delay(4);
  }
}

/// One sample of a real context for a stall record: its state, its clock and the host's status (the
/// engine's own `renderedQuanta` counts the blocks whose render returned).
async function sampleContext(context, step, host) {
  return {
    contextState: context.state,
    currentTime: context.currentTime,
    status: await step("status", () => host.status()),
  };
}

/// Issue #143 E12 and #1345 Amendment 6: subscribe -> frames -> unsubscribe -> no reduction on any
/// later frame, in a real browser, on one real `AudioContext` that is resumed once and never
/// suspended again before the final close.
///
/// A named #1345 exception (root, 2026-10-12, batch-verdict B1; Amendment 6 replaced its suspend).
/// This page calls the raw `host.observe`, and the engine admits one observation record per tap per
/// render (#1345 D8, D2 Amendments 1 and 2): a second record for the tap is refused with typed
/// backpressure until a block has drained the first. The honest contract is the one the SDK keeps
/// (`renderWait`): a render separates two operations on one tap. Admission itself still never
/// coalesces. The run therefore proves a render between the two, and does not suspend to get one:
///
/// 1. suspended, it feeds the source, subscribes, and reads the host status (`R1`, the rendered
///    quanta after the subscribe resolved);
/// 2. one trusted click resumes the context (see [`resumeThroughClick`]); the run keeps the source
///    fed and waits until the rendered quanta exceed `R1` and armed frames have arrived;
/// 3. it unsubscribes. The acknowledgement and the meter frames travel one FIFO port, so a frame
///    the page receives after the acknowledgement resolved was polled after the tap was disarmed;
/// 4. it keeps feeding and waits until the frames received after the acknowledgement cover at least
///    [`UNSUBSCRIBE_MINIMUM_BLOCKS`] blocks.
///
/// The gate then requires `trackGrDb == 0` on **every** frame received after the acknowledgement
/// (the contract text: the gain-reduction section is recomputed from the taps armed when each frame
/// is polled), no exclusion of the first frame or of any block after the boundary. A zero must not
/// come from silence: the meter window is one block, so each frame carries one block's peak, and
/// every frame after the acknowledgement must peak above [`UNSUBSCRIBE_PEAK_FLOOR`]. The page
/// reads no source-underrun counter (the host publishes none), so the per-block peak of the fed
/// signal is the evidence that no block of zeros was rendered.
///
/// The source streams just-in-time for the whole run: after every status read the page tops the ring
/// up to [`UNSUBSCRIBE_LEAD_BLOCKS`] blocks ahead of the rendered quanta, never further. The
/// lead is short enough that a feed which stops at the boundary underruns well inside the
/// [`UNSUBSCRIBE_MINIMUM_BLOCKS`] the gate requires.
///
/// `mutation` is `null` for the gate itself; `"skip-unsubscribe"` and `"feed-ends-at-boundary"` are
/// the page mutations `run.mjs` proves red in every browser (see MUTATIONS.md).
async function runObservationUnsubscribeRun(createHost, sessionDocument, mutation = null) {
  const context = new AudioContext({ sampleRate: SAMPLE_RATE });
  // The first `suspend()` of a new context is not awaited, the way `sdk-response-entry.ts` keeps its
  // contexts suspended across addModule. The state poll below is the check that replaces the await:
  // the run goes on only with a context that reports "suspended", and fails with a named location
  // otherwise.
  void context.suspend().catch(() => {});
  const stateBeforeBoot = await requireSuspended(context, "the new context before boot");
  const step = (name, work) => bounded("await", name, context, work);
  const host = await step("createHost", () => createHost({
    context,
    document: sessionDocument,
    options: bootOptions(
      DEFAULT_RING_FRAMES,
      LIVE_CONTROL_COMMAND_QUEUE_RECORDS,
      UNSUBSCRIBE_METER_BLOCKS,
      OBSERVATION_TAPS,
      OBSERVATION_MASTER_TRACK_PLUS_ONE,
    ),
    simd128ModuleUrl: ARTIFACT_URL,
    workletModuleUrl: WORKLET_URL,
  }));
  host.node.connect(context.destination);

  // The source. `fed` counts the blocks submitted; the content is the level the armed tap needs,
  // block after block, to the end of the document's source, for as long as `feeding` holds.
  //
  // The feed is driven by the meter frames themselves, not by status requests: a frame says which
  // block the render has just closed, so every frame tops the ring up to `UNSUBSCRIBE_LEAD_BLOCKS`
  // ahead of it, with the blocks sent back to back and their acknowledgements collected later. It
  // asks the processor for nothing, so the processor answers only the submits and the run's own
  // requests (the real browsers answer one request per quantum or fewer, and a feed that waits for a
  // status read and then for an acknowledgement per block falls behind a render that consumes a
  // block every 2.7 ms).
  let fed = 0;
  let feeding = true;
  let feedFailure = null;
  const inflight = [];
  const submitBlocks = (target) => {
    if (target > UNSUBSCRIBE_SOURCE_BLOCKS) {
      feedFailure ??= new Error("observation unsubscribe source exhausted");
      return;
    }
    for (; fed < target; fed += 1) {
      const block = fed;
      inflight.push(host.submitSource({
        sourceId: "live-control-source",
        generation: 1n,
        startFrame: BigInt(block * QUANTUM_FRAMES),
        sampleRateHz: SAMPLE_RATE,
        planes: unsubscribePlanes(block),
        frames: QUANTUM_FRAMES,
        endOfRegion: block === UNSUBSCRIBE_SOURCE_BLOCKS - 1,
      }).then((acknowledgement) => {
        if (acknowledgement.result !== 0) {
          feedFailure ??= new Error(`observation unsubscribe feed rejected: ${acknowledgement.result} at block ${block}`);
        }
      }, (error) => {
        feedFailure ??= error;
      }));
    }
  };
  const requireFeed = () => {
    if (feedFailure !== null) throw feedFailure;
  };
  const frames = [];
  const meterLease = await step("meters", () => host.meters({
    enabled: true,
    onFrame: (frame) => {
      const closed = {
        trackGrDb: frame.trackGrDb[0],
        peak: Math.max(frame.peaks[0], frame.peaks[1]),
        firstSample: frame.firstSample,
        endSample: frame.endSample,
      };
      frames.push(closed);
      if (feeding) submitBlocks(Number(closed.endSample / BigInt(QUANTUM_FRAMES)) + UNSUBSCRIBE_LEAD_BLOCKS);
    },
  }));
  const renderedQuanta = async () => (await step("status", () => host.status())).renderedQuanta;
  const describe = () => sampleContext(context, step, host);

  submitBlocks(UNSUBSCRIBE_LEAD_BLOCKS);
  await step("submitSource", () => Promise.all(inflight));
  requireFeed();
  const subscription = {
    trackIndex: 0,
    rack: 1,
    effectIndex: 0,
    tapId: OBSERVATION_TAP_ID,
    windowBlocks: Number(UNSUBSCRIBE_METER_BLOCKS),
    armed: true,
  };
  const subscribed = await step("observe (subscribe)", () => host.observe({ subscriptions: [subscription] }));
  // R1: read after the subscribe resolved. The unsubscribe goes out only after a render beyond it.
  const renderedAfterSubscribe = await renderedQuanta();

  let renderedBeforeUnsubscribe = renderedAfterSubscribe;
  await resumeUntil(
    context,
    async () => {
      requireFeed();
      if (frames.length < UNSUBSCRIBE_ARMED_FRAMES) return false;
      renderedBeforeUnsubscribe = await renderedQuanta();
      return renderedBeforeUnsubscribe > renderedAfterSubscribe;
    },
    UNSUBSCRIBE_RESUME,
    describe,
  );
  const maximumGrDb = (list) => list.reduce((highest, frame) => Math.max(highest, frame.trackGrDb), 0);
  const minimumPeak = (list) => list.reduce((lowest, frame) => Math.min(lowest, frame.peak), Infinity);
  const armedFrames = frames.length;
  const armedList = frames.slice();

  const unsubscribed = mutation === "skip-unsubscribe"
    ? { result: 0, bindings: [] }
    : await step("observe (unsubscribe)", () => host.observe({
      subscriptions: [{ ...subscription, armed: false }],
    }));
  // Everything received from here on was polled after the acknowledgement was sent: the same port,
  // in order.
  const acknowledgedAtFrame = frames.length;
  if (mutation === "feed-ends-at-boundary") feeding = false;
  const boundary = await renderedQuanta();

  const blocksAfter = () => frames.slice(acknowledgedAtFrame).reduce(
    (blocks, frame) => blocks + Number((frame.endSample - frame.firstSample) / BigInt(QUANTUM_FRAMES)),
    0,
  );
  await renderUntil(
    context,
    async () => {
      requireFeed();
      return blocksAfter() >= UNSUBSCRIBE_MINIMUM_BLOCKS;
    },
    UNSUBSCRIBE_AFTER_WAIT,
    async () => ({ boundary: String(boundary), ...(await describe()) }),
  );
  const afterList = frames.slice(acknowledgedAtFrame);
  feeding = false;
  await step("submitSource", () => Promise.all(inflight));
  requireFeed();

  const renderAllocations = await step("renderAllocationCount", () => renderAllocationCount(host));
  await step("dispose", () => host.dispose());
  await step("close", () => context.close());
  return {
    contextStateBeforeBoot: stateBeforeBoot,
    meterLeaseResult: meterLease.result,
    subscribeResult: subscribed.result,
    unsubscribeResult: unsubscribed.result,
    unsubscribeBindings: unsubscribed.bindings.length,
    renderedAfterSubscribe: String(renderedAfterSubscribe),
    renderedBeforeUnsubscribe: String(renderedBeforeUnsubscribe),
    boundary: String(boundary),
    fedBlocks: fed,
    armedFrames,
    armedMaximumTrackGrDb: maximumGrDb(armedList),
    framesAfterAcknowledgement: afterList.length,
    blocksAfterAcknowledgement: blocksAfter(),
    maximumTrackGrDbAfterAcknowledgement: maximumGrDb(afterList),
    minimumPeakAfterAcknowledgement: afterList.length === 0 ? 0 : minimumPeak(afterList),
    renderAllocations,
  };
}

/// Issue #143 E12: the observation row, its two offline runs and the real-context unsubscribe.
async function runObservationQualification(createHost, sessionDocument, unsubscribeDocument) {
  const armed = await runObservationRun(createHost, sessionDocument, true);
  const disarmed = await runObservationRun(createHost, sessionDocument, false);
  const unsubscribe = await runObservationUnsubscribeRun(createHost, unsubscribeDocument);
  return {
    armed,
    disarmed,
    unsubscribe,
    identicalAudio: armed.renderedDigest === disarmed.renderedDigest,
  };
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
      LIVE_CONTROL_COMMAND_QUEUE_RECORDS,
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
  const stallCommand = await host.command({
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
    liveControlCommandResult: stallCommand.result,
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
        LIVE_CONTROL_COMMAND_QUEUE_RECORDS,
        LIVE_CONTROL_METER_BLOCKS,
        OBSERVATION_TAPS,
        OBSERVATION_MASTER_TRACK_PLUS_ONE,
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
        trackIndex: 0,
        rack: 1,
        effectIndex: 0,
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
    liveControlResponse, observationResponse, unsubscribeResponse,
  ] =
    await Promise.all([
      import("/artifacts/miso-engine-v1-audio-worklet-host.js"),
      fetch("/fixture/expected.json"),
      fetch("/fixture/session.json"),
      fetch("/fixture/source.json"),
      fetch("/qualification/stall-session.json"),
      fetch("/qualification/live-control-session.json"),
      fetch("/qualification/observation-session.json"),
      fetch("/qualification/unsubscribe-session.json"),
    ]);
  if (!expectedResponse.ok || !sessionResponse.ok || !sourceResponse.ok || !stallResponse.ok
      || !liveControlResponse.ok || !observationResponse.ok || !unsubscribeResponse.ok) {
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
  // #1345 Amendment 6: the real-context unsubscribe run's own session, with a source long enough.
  const unsubscribeDocument = new TextEncoder().encode(await unsubscribeResponse.text());
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
  let observation;
  try {
    observation = await runObservationQualification(
      createMisoAudioWorkletHost,
      observationDocument,
      unsubscribeDocument,
    );
  } catch (error) {
    const diagnostic = await diagnoseReady(observationDocument);
    const failure = new Error(`observation qualification failed: ${diagnosticJson({
      name: error?.name, message: error?.message, ...error, diagnostic,
    })}`);
    // The structured stall signature rides on the failure so `run.mjs` prints and classifies it.
    if (error?.stallSignature !== undefined) failure.stallSignature = error.stallSignature;
    throw failure;
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
      { workload: "observation-armed", count: observation.armed.renderAllocations },
      { workload: "observation-disarmed", count: observation.disarmed.renderAllocations },
      { workload: "observation-unsubscribe", count: observation.unsubscribe.renderAllocations },
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

/// The `observation-unsubscribe` gate on the run's result (#1345 Amendment 6), shared by the gate
/// in `run.mjs` and by the page-mutation proofs, so both judge by one predicate. `true` only if the
/// armed tap published, the unsubscribe followed a render beyond the subscribe, and every frame
/// received after the unsubscribe acknowledgement (at least sixteen blocks of them, none excluded)
/// carried no reduction while still carrying the fed signal.
export function unsubscribeVerdict(run) {
  if (run === null || typeof run !== "object") return false;
  const wholeNumber = (value) => typeof value === "string" && /^[0-9]+$/.test(value);
  return run.contextStateBeforeBoot === "suspended"
    && run.meterLeaseResult === RESULT_OK
    && run.subscribeResult === RESULT_OK
    && run.unsubscribeResult === RESULT_OK
    && run.unsubscribeBindings === 0
    && wholeNumber(run.renderedAfterSubscribe) && wholeNumber(run.renderedBeforeUnsubscribe)
    && BigInt(run.renderedBeforeUnsubscribe) > BigInt(run.renderedAfterSubscribe)
    && run.armedFrames >= UNSUBSCRIBE_ARMED_FRAMES
    && run.armedMaximumTrackGrDb > 0
    && run.blocksAfterAcknowledgement >= UNSUBSCRIBE_MINIMUM_BLOCKS
    && run.framesAfterAcknowledgement >= 1
    && run.maximumTrackGrDbAfterAcknowledgement === 0
    && run.minimumPeakAfterAcknowledgement > UNSUBSCRIBE_PEAK_FLOOR;
}

/// The page-mutation seam (#1345 Amendment 6): the unsubscribe run alone, with one named defect
/// applied in the page. `run.mjs` runs it in a fresh page of every browser under
/// `--self-test-mutations` and requires [`unsubscribeVerdict`] to be false. `null` runs the gate
/// unmutated. The click plan is the run's own single control.
export const unsubscribePageMutations = Object.freeze(["skip-unsubscribe", "feed-ends-at-boundary"]);
export const unsubscribePageClickPlan = Object.freeze([resumeControlSelector(UNSUBSCRIBE_RESUME)]);
export async function runUnsubscribePageMutation(mutation) {
  if (mutation !== null && !unsubscribePageMutations.includes(mutation)) throw new Error(`unknown page mutation: ${mutation}`);
  const [{ createMisoAudioWorkletHost }, observationResponse] = await Promise.all([
    import("/artifacts/miso-engine-v1-audio-worklet-host.js"),
    fetch("/qualification/unsubscribe-session.json"),
  ]);
  if (!observationResponse.ok) throw new Error("qualification fixture fetch failed");
  const document = new TextEncoder().encode(await observationResponse.text());
  const resumeRecord = countResumes();
  const unsubscribe = await runObservationUnsubscribeRun(createMisoAudioWorkletHost, document, mutation);
  return { unsubscribe, resumes: resumeRecord };
}

// Issue #288: these are the existing qualification call sites, exposed only to the hermetic
// contract test. The browser runner continues to enter through runQualification below; keeping
// the references here direct means the test executes the same boot objects that production uses.
export const qualificationBootContract = Object.freeze({
  renderCorpusSegment,
  typedUnsupportedAttestation,
  runLiveControlQualification,
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
    document: "unsubscribe-session.json",
    sourceId: "live-control-source",
    blocks: UNSUBSCRIBE_SOURCE_BLOCKS,
    planes: unsubscribePlanes,
  }),
  Object.freeze({
    document: "stall-session.json",
    sourceId: "stall-source",
    blocks: STALL_FRAMES / QUANTUM_FRAMES,
    planes: sourcePlanes,
  }),
]);
