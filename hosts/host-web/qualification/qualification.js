import { hexLower } from "../web/hex-lower.js";

const SAMPLE_RATE = 48000;
const QUANTUM_FRAMES = 128;
const CORPUS_FRAMES = QUANTUM_FRAMES * 3;
const STALL_RING_FRAMES = 5120;
const STALL_FRAMES = STALL_RING_FRAMES;
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
const OBSERVATION_TAP_ID = 1;
const OBSERVATION_LEVEL = 0.5;
// Issue #1535: the real-context unsubscribe run, over its own session document
// (`unsubscribe-session.json`, a 512-block source: the observation document's sixteen blocks cannot
// hold the armed blocks, a render and 48 frames after the acknowledgement). A one-block meter
// window, so each frame carries one block's peak; the source fed at most 32 blocks (85 ms) ahead of
// the render, below the 40 blocks the default ring holds (D3.6, root ruling R2).
const UNSUBSCRIBE_SOURCE_BLOCKS = 512;
const UNSUBSCRIBE_METER_BLOCKS = 1n;
const UNSUBSCRIBE_LEAD_BLOCKS = 32;
const UNSUBSCRIBE_ARMED_FRAMES = 4;
// D4: more frames after the acknowledgement than the ring holds blocks, so a feed that stopped at
// the acknowledgement cannot supply them from the ring alone.
const UNSUBSCRIBE_SIGNAL_FRAMES = 48;
const RING_BLOCKS = STALL_RING_FRAMES / QUANTUM_FRAMES;
// The fed signal is a +-0.5 square wave. The lowest track peak measured after the acknowledgement
// is 0.0815 in all three browsers (#1535 attempt 1), 1.6 times this floor; a block of zeros (an
// underrun) peaks at 0.
const UNSUBSCRIBE_PEAK_FLOOR = 0.05;
const MINIMUM_STALL_MS = 100;
// Issue #1333 D3: the staging-read row renders the observation session's whole declared source
// region.
const STAGING_READ_BLOCKS = OBSERVATION_BLOCKS;
const STAGING_READ_FRAMES = STAGING_READ_BLOCKS * QUANTUM_FRAMES;
// Issue #1580 D2: the bound on the wait for the host-level track response's `RESPONSE_READY`.
const TRACK_RESPONSE_WAIT_MS = 10_000;
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
function bootOptions(sourceRingFrames, liveControlMeterBlocks = 0n) {
  return {
    sourceRingFrames,
    // Zero is "no caller-imposed memory ceiling", the same word `tests/browser-v1` sends and the
    // same one the direct oracle writes at boot-options offset 24.
    maximumMemoryBytes: 0n,
    // Issue #1565: offset 32 is reserved and must be zero.
    reserved1: 0n,
    liveControlMeterBlocks,
    // Issue #1590 D5: offset 48 was the observation-taps word; it is reserved and must be zero.
    // Every plan binds every declared tap.
    reserved3: 0n,
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
  // The run does not use OfflineAudioContext.suspend (Firefox, no longer a supported browser
  // since #2092, lacked it; the suspend-free form stays engine-neutral). Prefill two bounded fresh
  // contexts and concatenate the oracle's exact [block 0, block 0, block 1] timeline instead of
  // coordinating submissions with a main-realm-only API. The resulting 384 frames must still match
  // the one frozen native digest bit for bit.
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
      options: bootOptions(STALL_RING_FRAMES),
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
/// `N`, the live-control session's mute ramp: its default `mute_ms` of 10 ms (#1324a).
const STRUCTURAL_MUTE_RAMP = Math.round((10 * SAMPLE_RATE) / 1000);
/// The ceiling to a whole quantum.
const ceilQuantum = (samples) => Math.ceil(samples / QUANTUM_FRAMES) * QUANTUM_FRAMES;
/// `S = ceil_q(p + q + N + C)`: the delay edit restarts the track, a duck-swap (#1324a), committed
/// before the first block (`p = 0`) in a session with no compensation line (`C = 0`).
const STRUCTURAL_SWAP_SAMPLE = ceilQuantum(QUANTUM_FRAMES + STRUCTURAL_MUTE_RAMP);
/// The first frame after the successor's fade-in: it fires at the first block at or after `S + D`,
/// with `D` at most the edit's delay, and ramps over `N`.
const STRUCTURAL_FADED_IN = ceilQuantum(STRUCTURAL_SWAP_SAMPLE + STRUCTURAL_DELAY_SAMPLES)
  + STRUCTURAL_MUTE_RAMP;

/// #1562b: a structural apply in a real browser. The live-control session's track gets a
/// `delay_samples` edit, which the core classifies as a rebuild: it prepares a replacement plan and
/// publishes it while the first plan has rendered nothing (the offline context cannot be suspended
/// in every engine). The edit restarts the track, so the rebuild is a duck-swap (#1324a): the first
/// plan ramps the track out over `N` from the first block, and the successor is adopted at
/// `S = ceil_q(q + N)` and fades the track in. The row holds what the swap must keep: the reply is
/// the rebuild path, the applied revision is not early (it moves at `S`, exactly), the duck holds
/// exact `+0.0` from its end until `S`, the output after the fade-in is the submitted PCM delayed
/// by exactly the edit (computed here, not read back), the meter stream is the successor's (the
/// frame carries the structure epoch of the list), and no allocator call ran on the render thread.
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
  let duckedSilent = true;
  for (let channel = 0; channel < 2; channel += 1) {
    for (let frame = STRUCTURAL_MUTE_RAMP; frame < STRUCTURAL_SWAP_SAMPLE; frame += 1) {
      if (!Object.is(actual[channel][frame], 0)) duckedSilent = false;
    }
    for (let frame = STRUCTURAL_FADED_IN; frame < LIVE_CONTROL_FRAMES; frame += 1) {
      if (!Object.is(actual[channel][frame], expected[channel][frame])) exact = false;
    }
  }
  let increasing = true;
  for (let index = 1; index < meterFrames.length; index += 1) {
    if (meterFrames[index].sequence <= meterFrames[index - 1].sequence) increasing = false;
  }
  const fadedIn = (pcm) => pcm.map((channel) => channel.subarray(STRUCTURAL_FADED_IN));
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
    scheduledSample: String(STRUCTURAL_SWAP_SAMPLE),
    meterFrames: meterFrames.length,
    meterFrameWidth: meterFrames.length === 0 ? 0 : meterFrames[0].peaks,
    meterTrackCount: meterFrames.length === 0 ? 0 : meterFrames[0].trackCount,
    structureEpochs: [...new Set(meterFrames.map((frame) => frame.structureEpoch))],
    sequenceIncreasing: increasing,
    duckedSilent,
    exactDelayedOutput: exact,
    expectedDigest: await pcmDigest(fadedIn(expected), LIVE_CONTROL_FRAMES - STRUCTURAL_FADED_IN),
    renderedDigest: await pcmDigest(fadedIn(actual), LIVE_CONTROL_FRAMES - STRUCTURAL_FADED_IN),
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
/// first quantum is the swap block, where the added track first plays. Issue #1288: a track a swap
/// adds during playback fades in from its first played block plus `D` (0 here: no latent node
/// before the fader) over the session mute ramp (`mute_ms` 10 at 48 kHz, 480 frames). So the
/// output is exact zero before the fire frame, a fade over the ramp (each frame the input scaled by
/// a gain in `[0, 1]` that never falls, and not the input itself), and then the submitted PCM,
/// exactly, computed here and not read back; the meter frame grows to the strip, and no
/// render-thread allocator call ran.
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
  // #1288: the fire frame is the first played block (the swap block, 0) plus D (0), and the fade
  // runs over the session mute ramp.
  const fireFrame = 0 * QUANTUM_FRAMES + 0;
  const rampFrames = Math.floor((10 * SAMPLE_RATE) / 1000 + 0.5);
  const settled = fireFrame + rampFrames;
  let exact = true;
  let fadeShaped = true;
  let faded = false;
  for (let channel = 0; channel < 2; channel += 1) {
    let gain = 0;
    for (let at = 0; at < LIVE_CONTROL_FRAMES; at += 1) {
      const out = actual[channel][at];
      const fed = input[channel][at];
      if (at < fireFrame) {
        if (!Object.is(out, 0)) fadeShaped = false;
      } else if (at < settled) {
        if (!Object.is(out, fed)) faded = true;
        if (Math.abs(fed) > 1e-6) {
          const ratio = out / fed;
          if (!(ratio >= -1e-6 && ratio <= 1 + 1e-6) || ratio < gain - 1e-5) fadeShaped = false;
          gain = Math.max(gain, ratio);
        }
      } else if (!Object.is(out, fed)) {
        exact = false;
      }
    }
  }
  const tail = (planes) => planes.map((channel) => channel.subarray(settled));
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
      fadeShaped: fadeShaped && faded,
      fireFrame,
      rampFrames,
      heard: nonzero(actual),
      expectedDigest: await pcmDigest(tail(input), LIVE_CONTROL_FRAMES - settled),
      renderedDigest: await pcmDigest(tail(actual), LIVE_CONTROL_FRAMES - settled),
      meterFrames: grownFrames.length,
      meterTrackCounts: [...new Set(grownFrames.map((entry) => entry.trackCount))],
      meterWidths: [...new Set(grownFrames.map((entry) => entry.peaks))],
      structureEpochs: [...new Set(grownFrames.map((entry) => entry.structureEpoch))],
      renderAllocations: grownAllocations,
    },
  };
}

/// Issue #1295 D1: the structural-edit capture, the swap block and 32 quanta after it.
const STRUCTURAL_EDIT_BLOCKS = 33;
const STRUCTURAL_EDIT_FRAMES = STRUCTURAL_EDIT_BLOCKS * QUANTUM_FRAMES;
/// Issue #1295 (#1697 gate 3): the quanta the 1 MiB replacement renders after its commit. It
/// replaces the track's source, so the old track ramps out before the swap: the module under Node
/// adopts it at sample 640 (block 5). Sixteen leaves room and gates nothing on the block.
const STRUCTURAL_REPLACE_LARGE_BLOCKS = 16;
/// Issue #1332 S3: the quanta the same capture renders after those sixteen. The replacement grew
/// the module's shared memory in a message handler, so `memory.buffer` is a new object; render
/// must go on over the views it holds, with no quantum silenced for it.
const STRUCTURAL_REPLACE_GROWN_BLOCKS = 64;
const STRUCTURAL_REPLACE_CAPTURE_BLOCKS = STRUCTURAL_REPLACE_LARGE_BLOCKS + STRUCTURAL_REPLACE_GROWN_BLOCKS;
const STRUCTURAL_REPLACE_CAPTURE_FRAMES = STRUCTURAL_REPLACE_CAPTURE_BLOCKS * QUANTUM_FRAMES;

/// Issue #1332 S3: the 1 MiB document's `fixture-source` blocks, fed after its commit, so the
/// tracks it adopts play a signal (a quantum silenced after the growth is then audible in the
/// digest). The page and `run.mjs`'s Node reference both read them here.
export function largeReplaceSource() {
  return {
    sourceId: "fixture-source",
    blocks: Array.from({ length: STRUCTURAL_REPLACE_CAPTURE_BLOCKS }, (_, block) => ({
      startFrame: block * QUANTUM_FRAMES,
      frames: QUANTUM_FRAMES,
      // No block ends the region: the module refuses an end of region short of the 48,000 frames
      // the source declares.
      final: false,
      leftBase: ((block % 8) - 4) / 1024,
      leftStep: 1 / 65536,
    })),
  };
}

/// Issue #1295: a structural edit by `SESSION_REPLACE` in a real browser, on this page (no
/// cross-origin isolation, so the module runs in `single` mode and every control call runs in the
/// worklet's message handler).
///
/// Two boots of the browser fixture's session A. The first gets one `miso.control.v1` with
/// document B (A plus a track on the same source with an EQ insert and its fader at -6 dB, and its
/// route) before the first quantum, which is then the swap block, and renders the swap block and 32
/// quanta after it. The row records the reply (path, revision R), the applied-revision watermark
/// `(R, S, outcome)` as the host reported it, the source underrun counter, the host status's
/// `blockingRebuilds` and `singleModeControlAllocations` before the replace and once the watermark
/// covers R, and the capture's digest, which `run.mjs` holds to the same replace rendered by the
/// module under Node. The second sends the 1 MiB boot-ceiling document (#1697 gate 3) and records
/// the module's memory growth across it, which `run.mjs` reports against the boot's projection
/// `document_bytes * PARSE_TRANSIENT_MULTIPLIER` and does not pin.
async function runStructuralReplaceQualification(createHost, documentA, documentB, source, largeDocument) {
  const { replaceSessionThroughSdk, sourceUnderrunBlocksThroughSdk } = await import("/sdk/sdk-response-client.js");
  const boot = async (frames, options = bootOptions(STRUCTURAL_EDIT_FRAMES)) => {
    const context = new OfflineAudioContext(2, frames, SAMPLE_RATE);
    const host = await createHost({
      context,
      document: documentA,
      options,
      simd128ModuleUrl: ARTIFACT_URL,
      workletModuleUrl: WORKLET_URL,
    });
    host.node.connect(context.destination);
    return { context, host };
  };
  const counters = (status) => ({
    blockingRebuilds: status.blockingRebuilds,
    singleModeControlAllocations: status.singleModeControlAllocations,
    memoryBytes: status.memoryBytes,
    meterFramesSkipped: status.meterFramesSkipped,
    appliedRevision: status.appliedRevision.toString(),
    appliedSample: status.appliedSample.toString(),
    appliedOutcome: status.appliedOutcome.toString(),
    nextAbsoluteSample: status.nextAbsoluteSample.toString(),
  });

  // The exact case.
  const edit = await boot(STRUCTURAL_EDIT_FRAMES);
  for (const description of source.blocks) {
    const acknowledgement = await edit.host.submitSource(corpusRequest(description));
    if (acknowledgement.result !== RESULT_OK) throw new Error("structural-edit prefill rejected");
  }
  const watermarks = [];
  edit.host.onWatermark((watermark) => watermarks.push({
    revision: watermark.revision.toString(),
    sample: watermark.sample.toString(),
    outcome: watermark.outcome.toString(),
  }));
  const before = counters(await edit.host.status());
  const reply = await replaceSessionThroughSdk(edit.host, documentB);
  const pending = counters(await edit.host.status());
  const rendered = await edit.context.startRendering();
  for (let spin = 0; spin < 8 && watermarks.at(-1)?.revision !== reply.revision; spin += 1) {
    await new Promise((resolve) => setTimeout(resolve, 4));
  }
  const after = counters(await edit.host.status());
  const underrunBlocks = await sourceUnderrunBlocksThroughSdk(edit.host);
  const editAllocations = await renderAllocationCount(edit.host);
  await edit.host.dispose();
  const capture = [rendered.getChannelData(0), rendered.getChannelData(1)];

  // The 1 MiB boot-ceiling document. Issue #1332 S3: its commit grows the module's shared memory
  // in the message handler; the capture then renders on for 64 quanta over the source fed after
  // the commit, with the meter stream leased.
  const large = await boot(STRUCTURAL_REPLACE_CAPTURE_FRAMES,
    bootOptions(STRUCTURAL_REPLACE_CAPTURE_FRAMES, LIVE_CONTROL_METER_BLOCKS));
  const largeMeterFrames = [];
  const largeLease = await large.host.meters({
    enabled: true,
    onFrame: (frame) => largeMeterFrames.push({
      trackCount: frame.trackCount,
      endSample: frame.endSample.toString(),
      finite: Array.from(frame.peaks).every(Number.isFinite),
    }),
  });
  const largeBefore = counters(await large.host.status());
  const largeReply = await replaceSessionThroughSdk(large.host, largeDocument);
  const largeCommitted = counters(await large.host.status());
  for (const description of largeReplaceSource().blocks) {
    const acknowledgement = await large.host.submitSource(corpusRequest(description));
    if (acknowledgement.result !== RESULT_OK) throw new Error("structural-replace-large feed rejected");
  }
  const largeRendered = await large.context.startRendering();
  for (let spin = 0; spin < 8 && largeMeterFrames.length === 0; spin += 1) {
    await new Promise((resolve) => setTimeout(resolve, 4));
  }
  const largeAfter = counters(await large.host.status());
  const largeUnderruns = await sourceUnderrunBlocksThroughSdk(large.host);
  const largeAllocations = await renderAllocationCount(large.host);
  await large.host.dispose();
  const largeCapture = [largeRendered.getChannelData(0), largeRendered.getChannelData(1)];
  const largeGrownStart = STRUCTURAL_REPLACE_LARGE_BLOCKS * QUANTUM_FRAMES;

  return {
    edit: {
      replyStatus: reply.status,
      commandPath: reply.path,
      commandApplied: reply.appliedOperations,
      diagnostics: reply.diagnostics,
      expectedRevision: reply.expectedRevision,
      committedRevision: reply.revision,
      frameHex: reply.frameHex,
      sourceRingFrames: STRUCTURAL_EDIT_FRAMES,
      blocks: STRUCTURAL_EDIT_BLOCKS,
      before,
      pending,
      after,
      watermark: watermarks.at(-1) ?? null,
      watermarks: watermarks.length,
      underrunBlocks,
      renderedDigest: await pcmDigest(capture, STRUCTURAL_EDIT_FRAMES),
      renderAllocations: editAllocations,
    },
    large: {
      documentBytes: largeDocument.byteLength,
      replyStatus: largeReply.status,
      commandPath: largeReply.path,
      diagnostics: largeReply.diagnostics,
      committedRevision: largeReply.revision,
      before: largeBefore,
      committed: largeCommitted,
      after: largeAfter,
      renderAllocations: largeAllocations,
      frameHex: largeReply.frameHex,
      sourceRingFrames: STRUCTURAL_REPLACE_CAPTURE_FRAMES,
      blocks: STRUCTURAL_REPLACE_CAPTURE_BLOCKS,
      grownBlocks: STRUCTURAL_REPLACE_GROWN_BLOCKS,
      renderedDigest: await pcmDigest(largeCapture, STRUCTURAL_REPLACE_CAPTURE_FRAMES),
      grownHeard: largeCapture.some((channel) => channel.subarray(largeGrownStart).some((sample) => sample !== 0)),
      underrunBlocks: largeUnderruns,
      meterLeaseResult: largeLease.result,
      meterFrames: largeMeterFrames.length,
      meterTrackCounts: [...new Set(largeMeterFrames.map((frame) => frame.trackCount))],
      meterFramesAfterGrowth: largeMeterFrames
        .filter((frame) => BigInt(frame.endSample) > BigInt(largeGrownStart)).length,
      meterFinite: largeMeterFrames.every((frame) => frame.finite),
    },
  };
}

/// Issue #1332 S9: the quanta the growth row's real context renders before the 1 MiB replacement is
/// sent, and the quanta it must render after the commit.
const GROWTH_RENDERED_BEFORE = 8n;
const GROWTH_RENDERED_AFTER = 64n;
/// The label of the growth row's one trusted-click resume (`resumeClickPlan` names it).
const GROWTH_RESUME = "the worklet to render across a growth";

/// Issue #1332 S9: the 1 MiB `SESSION_REPLACE` sent while a real `AudioContext` renders. On the
/// isolated leg the Worker parses the document and grows the shared memory on its own thread while
/// the worklet's `process()` keeps rendering over the views it cut before the growth (S3: a growth
/// is no fault); the worklet re-cuts them only when the Worker's `miso.meter-view.v1` reaches its
/// message handler (R-S7-meters). On the single leg the same edit grows the memory in the worklet's
/// handler, between two quanta. The row records the quanta rendered while the reply was pending,
/// the memory before and after, the status once the context has rendered
/// [`GROWTH_RENDERED_AFTER`] quanta past the commit with the revision applied, and the meter frames
/// that arrived after the reply: the successor's 192 strips must reach the page over the re-cut
/// views. A render is not deterministic on a real context, so no digest is taken here: the offline
/// capture of the same replacement holds the bits (`structural-replace-grown`).
async function runGrowthWhileRenderingRun(createHost, documentA, largeDocument) {
  const { replaceSessionThroughSdk } = await import("/sdk/sdk-response-client.js");
  const context = new AudioContext({ sampleRate: SAMPLE_RATE });
  void context.suspend().catch(() => {});
  await requireSuspended(context, "the growth context before boot");
  const host = await bounded("await", "createHost (growth)", context, () => createHost({
    context,
    document: documentA,
    options: bootOptions(STRUCTURAL_REPLACE_CAPTURE_FRAMES, LIVE_CONTROL_METER_BLOCKS),
    simd128ModuleUrl: ARTIFACT_URL,
    workletModuleUrl: WORKLET_URL,
  }));
  host.node.connect(context.destination);
  const frames = [];
  const lease = await bounded("await", "meters (growth)", context, () => host.meters({
    enabled: true,
    onFrame: (frame) => frames.push({
      trackCount: frame.trackCount,
      finite: Array.from(frame.peaks).every(Number.isFinite),
    }),
  }));
  const status = (step) => bounded("await", step, context, () => host.status());
  const describe = async () => ({
    contextState: context.state,
    currentTime: context.currentTime,
    status: await host.status(),
  });
  const counters = (reply) => ({
    renderedQuanta: reply.renderedQuanta.toString(),
    memoryBytes: reply.memoryBytes,
    appliedRevision: reply.appliedRevision.toString(),
    lastResult: reply.lastResult,
    meterFramesSkipped: reply.meterFramesSkipped,
    blockingRebuilds: reply.blockingRebuilds,
    sampleRateHz: reply.sampleRateHz,
    quantumFrames: reply.quantumFrames,
  });
  await resumeThroughClick(context, GROWTH_RESUME);
  let before;
  await renderUntil(context, async () => {
    before = await status("status (before the growth)");
    return before.renderedQuanta >= GROWTH_RENDERED_BEFORE;
  }, GROWTH_RESUME, describe, performance.now());
  const sentAtFrame = frames.length;
  const sentAt = performance.now();
  const reply = await bounded("await", "the 1 MiB replacement (growth)", context,
    () => replaceSessionThroughSdk(host, largeDocument));
  const replyMs = performance.now() - sentAt;
  const committedAtFrame = frames.length;
  const committed = await status("status (committed)");
  let after;
  await renderUntil(context, async () => {
    after = await status("status (after the growth)");
    return after.renderedQuanta >= committed.renderedQuanta + GROWTH_RENDERED_AFTER
      && after.appliedRevision.toString() === reply.revision
      && frames.slice(committedAtFrame).some((frame) => frame.trackCount === 192);
  }, "the quanta after the growth", describe, performance.now());
  const allocations = await bounded("await", "renderAllocationCount (growth)", context,
    () => renderAllocationCount(host));
  await bounded("await", "dispose (growth)", context, () => host.dispose());
  await bounded("await", "close (growth)", context, () => context.close());
  const afterGrowth = frames.slice(committedAtFrame);
  return {
    meterLeaseResult: lease.result,
    replyStatus: reply.status,
    commandPath: reply.path,
    committedRevision: reply.revision,
    before: counters(before),
    committed: counters(committed),
    after: counters(after),
    replyMs,
    meterBlocks: Number(LIVE_CONTROL_METER_BLOCKS),
    framesDuringReplace: committedAtFrame - sentAtFrame,
    framesAfterGrowth: afterGrowth.length,
    successorFramesAfterGrowth: afterGrowth.filter((frame) => frame.trackCount === 192).length,
    meterFinite: frames.every((frame) => frame.finite),
    renderAllocations: allocations,
  };
}

/// Issue #143 E12: subscribe -> nonzero `trackGrDb` in a real browser, and an unarmed tap publishes none.
///
/// Two offline renders of the same sixteen blocks over the same session, differing only in whether
/// the tap is armed when rendering starts. That is E1's leg (c) against leg (d) in a browser: the
/// two renders must produce **identical PCM**, because arming a declared tap may not move a sample.
///
/// `armed` is `true` for the first run and `false` for the second, which never subscribes. The arm
/// is the core's `Observe` edit through the host's `control` message (#1589c), built with the SDK's
/// codec. The engine admits one observation record per tap per render (#1345 D8, D2 Amendments 1
/// and 2), and an
/// offline context renders only inside one `startRendering` call (the harness does not use
/// `OfflineAudioContext.suspend`), so a subscribe followed at once by an unsubscribe is refused with
/// typed backpressure here. That an unsubscribe stops a tap's reading is proved on a real
/// `AudioContext` by [`runObservationUnsubscribeRun`] (#1535).
async function runObservationRun(createHost, sessionDocument, armed) {
  const context = new OfflineAudioContext(2, OBSERVATION_FRAMES, SAMPLE_RATE);
  const host = await createHost({
    context,
    document: sessionDocument,
    options: bootOptions(
      OBSERVATION_FRAMES,
      LIVE_CONTROL_METER_BLOCKS,
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
    rack: "inserts",
    effectId: "comp",
    tapId: OBSERVATION_TAP_ID,
    windowBlocks: Number(LIVE_CONTROL_METER_BLOCKS),
    armed: true,
  };
  const { observeThroughSdk } = await import("/sdk/sdk-response-client.js");
  const subscribed = armed ? await observeThroughSdk(host, subscription) : null;

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
    subscribeStatus: subscribed?.status ?? null,
    subscribePath: subscribed?.path ?? null,
    subscribeArmTag: subscribed?.armTag ?? null,
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
/// The three SDK controls belong to `sdk-response-entry.ts`'s live track-response, collection and
/// hop workloads, which run first and only when the SDK qualification does; then the growth row's
/// [`resumeThroughClick`] button (#1332 S9); the last is the unsubscribe run's one
/// [`resumeThroughClick`] button (#1535).
export function resumeClickPlan(sdkEnabled) {
  if (!WebAssembly.validate(SIMD128_PROBE)) return [];
  return [
    ...(sdkEnabled ? ["#track-response-resume", "#spectrum-collection-resume", "#spectrum-hop-resume"] : []),
    resumeControlSelector(GROWTH_RESUME),
    resumeControlSelector(UNSUBSCRIBE_RESUME),
  ];
}

/// The label of the one trusted-click resume `runObservationUnsubscribeRun` makes. The click plan
/// names the button by this label, so the run and its plan cannot drift apart.
const UNSUBSCRIBE_RESUME = "the armed tap to render";
const UNSUBSCRIBE_AFTER_WAIT = "the frames after the unsubscribe";

function resumeControlSelector(what) {
  return `button[data-qualification-resume="${what}"]`;
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

function delay(milliseconds) {
  return new Promise((resolve) => setTimeout(resolve, milliseconds));
}

/// The bound on every wait that depends on an audio context or on a host reply in the real-context
/// run, the same 10 s as `sdk-response-entry.ts` uses for its `suspend()` and `resume()` bounds.
const AUDIO_STEP_TIMEOUT_MS = 10_000;

/// Await `start()`, failing with `<kind> timeout at <step>` and the context's state if it has not
/// settled within `AUDIO_STEP_TIMEOUT_MS`. A timeout is a failure of the gate: never a pass, a skip
/// or a retry. WebKit leaves a `resume()` that has no user activation pending; an unbounded wait
/// there hangs the page until CI kills the job.
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

/// Issue #1535 D3.1: the one form a failed host request of the unsubscribe run reaches the page in.
/// `cause` is what the request rejected with or resolved to: a `miso.error.v1` record (named by its
/// tag, request ID and result, never spread raw), an `Error` (its message), or anything else.
/// `inFlight` is the run's own per-kind count of unsettled requests.
export function requestFailure(step, kind, inFlight, cause) {
  const reason = cause instanceof Error
    ? cause.message
    : cause !== null && typeof cause === "object"
      ? diagnosticJson({ tag: cause.tag, requestId: cause.requestId, result: cause.result })
      : String(cause);
  return new Error(
    `request refused at ${step}: kind=${kind} inFlight=${diagnosticJson(inFlight)} cause=${reason}`,
  );
}

/// Fail with `suspend timeout at <step>` unless `context.state` is "suspended" within the bound.
/// The first `suspend()` of a new context is not awaited (an engine may create a later context
/// already running, as Firefox did before #2092, and the `suspend()` is only a request), so this
/// poll is what keeps the run from going on with a context that is still running. Returns the
/// state it saw, which the run records.
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

/// Resume `context` with a trusted click on a button this function adds, the way an app must in
/// Safari: WebKit starts or restarts an `AudioContext` only inside a user activation and otherwise
/// leaves `resume()` pending. `run.mjs` clicks the button [`resumeClickPlan`] names, a trusted event
/// in both supported browsers. The wait for the click is bounded like every other wait here.
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

/// How often `renderUntil` asks the host for its status while it waits, and how long one such
/// request may stay unanswered before the poll counts as unanswered. The host shares one status
/// read between concurrent calls, so a poll that stops waiting is never refused by the next.
const STALL_POLL_INTERVAL_MS = 250;
const STALL_POLL_TIMEOUT_MS = 1_000;

/// The signature of a stall that `renderUntil` records (root ruling, 2026-10-12). It is `full` only
/// if a stalled render is the whole of the evidence: the context says it is "running", its clock
/// (`currentTime`) and the engine's `renderedQuanta` are the same at the start and at the timeout,
/// the processor answered at least two status requests inside the window and its last render result
/// is 0. The status replies come from the processor's own port handler, which runs in the same JS
/// agent as `process()`: an answered reply proves `process()` was not on the stack, so a context
/// that "ran" and rendered nothing is a context that was not pulling its graph. `atResume` and
/// `atTimeout` are `sampleContext` records (or `null` when the sample was not answered);
/// `replyIds` are the request ids of the answered status requests in the window, in order.
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

/// Wait, on a running context, until `condition` holds, for at most `AUDIO_STEP_TIMEOUT_MS` from
/// `started`. There is no recovery and no retry. While it waits it samples `describe()` every
/// `STALL_POLL_INTERVAL_MS`. At the deadline it samples once more and asks `atDeadline(sample)`:
/// `true` returns "deadline" (the caller's own named failure, D4's shortfall); otherwise it throws
/// the named stall failure carrying the [`classifyStallSignature`] of the window.
async function renderUntil(context, condition, what, describe, started, atDeadline = () => false) {
  const atResume = await boundedSample(describe);
  let lastPoll = performance.now();
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
      if (atDeadline(atTimeout)) return "deadline";
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
  return "condition";
}

/// Issue #1535 (successor of #1345 Amendment 6): subscribe -> frames -> unsubscribe -> no reduction
/// on any later frame, on one real `AudioContext` that is resumed once by a trusted click and never
/// suspended again before the final close (root ruling 5, D2).
///
/// The engine admits one observation record per tap per render (#1345 D8), so a render must
/// separate the subscribe from the unsubscribe:
///
/// 1. suspended, the run feeds the source, subscribes, and reads the host status (`R1`);
/// 2. one trusted click resumes the context; the run waits until a status read exceeds `R1` and
///    armed frames have arrived;
/// 3. it unsubscribes. The acknowledgement and the meter frames reach the page in order -- one FIFO
///    port in `single` mode; in `worker` mode the host delivers the Worker's reply only after a
///    fence on the worklet's port (#1332 S9) -- so a frame received after the acknowledgement
///    resolved was polled after the tap was disarmed;
/// 4. under one deadline from the acknowledgement it waits for [`UNSUBSCRIBE_SIGNAL_FRAMES`] frames
///    received after it that peak above [`UNSUBSCRIBE_PEAK_FLOOR`], each frame counted once whatever
///    its block count (D4). At the deadline a render that moved 48 blocks is a `shortfall`; one that
///    did not is a stall (named, with its signature). A frame at or below the floor is reported,
///    never failed (root ruling R1).
///
/// Every host request goes through `request`, which counts it per kind, bounds it, and turns any
/// refusal into [`requestFailure`]'s `Error` (D3.1-2). The feed is driven by the meter frames and
/// the submit acknowledgements: it tops the ring up to [`UNSUBSCRIBE_LEAD_BLOCKS`] blocks ahead of
/// the render, never with more than [`RING_BLOCKS`] submits in flight (D3.4), and never sends a block
/// the render has passed: it records `feedBehind` and stops instead (D3.5).
async function runObservationUnsubscribeRun(createHost, sessionDocument) {
  const context = new AudioContext({ sampleRate: SAMPLE_RATE });
  // The first `suspend()` of a new context is not awaited; the state poll replaces the await.
  void context.suspend().catch(() => {});
  const stateBeforeBoot = await requireSuspended(context, "the new context before boot");
  const inFlight = { source: 0, status: 0, observe: 0, meters: 0, renderAllocations: 0, dispose: 0 };
  let maximumSourcesInFlight = 0;
  const request = async (step, kind, work) => {
    inFlight[kind] += 1;
    if (kind === "source") maximumSourcesInFlight = Math.max(maximumSourcesInFlight, inFlight.source);
    try {
      return await bounded("await", step, context, work);
    } catch (error) {
      throw requestFailure(step, kind, inFlight, error);
    } finally {
      inFlight[kind] -= 1;
    }
  };
  const host = await bounded("await", "createHost", context, () => createHost({
    context,
    document: sessionDocument,
    options: bootOptions(
      STALL_RING_FRAMES,
      UNSUBSCRIBE_METER_BLOCKS,
    ),
    simd128ModuleUrl: ARTIFACT_URL,
    workletModuleUrl: WORKLET_URL,
  }));
  host.node.connect(context.destination);

  // The render position the page knows: the larger of the newest frame's end block and the newest
  // status `renderedQuanta`.
  let renderBlock = 0;
  let fed = 0;
  let feeding = true;
  let feedFailure = null;
  let feedBehind = null;
  // The render block at which the feed would pass the end of the source: the run ends its wait
  // and returns, and the gate names it `shortfall` (the signal after the acknowledgement stopped).
  let sourceExhausted = null;
  const submits = new Set();
  const topUp = () => {
    if (!feeding || feedFailure !== null || feedBehind !== null || sourceExhausted !== null) return;
    const target = renderBlock + UNSUBSCRIBE_LEAD_BLOCKS;
    if (target > UNSUBSCRIBE_SOURCE_BLOCKS) {
      sourceExhausted = { renderBlock, fed };
      return;
    }
    while (fed < target && inFlight.source < RING_BLOCKS) {
      if (fed < renderBlock) {
        feedBehind = { block: fed, renderBlock };
        return;
      }
      const block = fed;
      fed += 1;
      const step = `submitSource block ${block}`;
      const pending = request(step, "source", () => host.submitSource({
        sourceId: "live-control-source",
        generation: 1n,
        startFrame: BigInt(block * QUANTUM_FRAMES),
        sampleRateHz: SAMPLE_RATE,
        planes: unsubscribePlanes(block),
        frames: QUANTUM_FRAMES,
        endOfRegion: block === UNSUBSCRIBE_SOURCE_BLOCKS - 1,
      })).then((acknowledgement) => {
        if (acknowledgement.result !== RESULT_OK) {
          feedFailure ??= requestFailure(step, "source", inFlight, acknowledgement);
        }
        topUp();
      }, (error) => {
        feedFailure ??= error;
      }).finally(() => submits.delete(pending));
      submits.add(pending);
    }
  };
  const requireFeed = () => {
    if (feedFailure !== null) throw feedFailure;
  };
  const seen = (quanta) => {
    renderBlock = Math.max(renderBlock, Number(quanta));
  };
  const frames = [];
  const meterLease = await request("meters", "meters", () => host.meters({
    enabled: true,
    onFrame: (frame) => {
      frames.push({
        trackGrDb: frame.trackGrDb[0],
        peak: Math.max(frame.peaks[0], frame.peaks[1]),
      });
      seen(frame.endSample / BigInt(QUANTUM_FRAMES));
      topUp();
    },
  }));
  const status = async (step) => {
    const reply = await request(step, "status", () => host.status());
    seen(reply.renderedQuanta);
    return reply;
  };
  const describe = async () => ({
    contextState: context.state,
    currentTime: context.currentTime,
    status: await status("status (stall poll)"),
  });

  topUp();
  await bounded("await", "the first source blocks", context, () => Promise.all([...submits]));
  requireFeed();
  // #1589b: the core's `Observe` edit, through the host's `control` message.
  const { observeThroughSdk } = await import("/sdk/sdk-response-client.js");
  const subscription = {
    stripId: "track",
    rack: "inserts",
    effectId: "comp",
    tapId: OBSERVATION_TAP_ID,
    windowBlocks: Number(UNSUBSCRIBE_METER_BLOCKS),
    armed: true,
  };
  const subscribed = await request("observe (subscribe)", "observe",
    () => observeThroughSdk(host, subscription));
  // R1: read after the subscribe resolved. The unsubscribe goes out only after a render beyond it.
  const renderedAfterSubscribe = (await status("status (R1)")).renderedQuanta;

  let renderedBeforeUnsubscribe = renderedAfterSubscribe;
  await resumeThroughClick(context, UNSUBSCRIBE_RESUME);
  await renderUntil(
    context,
    async () => {
      requireFeed();
      if (frames.length < UNSUBSCRIBE_ARMED_FRAMES) return false;
      renderedBeforeUnsubscribe = (await status("status (before unsubscribe)")).renderedQuanta;
      return renderedBeforeUnsubscribe > renderedAfterSubscribe;
    },
    UNSUBSCRIBE_RESUME,
    describe,
    performance.now(),
  );
  const armedList = frames.slice();

  const unsubscribed = await request("observe (unsubscribe)", "observe",
    // #2087: a disarm names its arm by the tag the subscribe's reply gave.
    () => observeThroughSdk(host, { ...subscription, armed: false, armTag: BigInt(subscribed.armTag ?? 0) }));
  // Everything received from here on was polled after the tap was disarmed: the same port, in order
  // (in `worker` mode, after the host's fence on the worklet's port, #1332 S9).
  const acknowledgedAt = performance.now();
  const acknowledgedAtFrame = frames.length;
  const boundary = (await status("status (boundary)")).renderedQuanta;
  const signalFrames = (list) => list.filter((frame) => frame.peak > UNSUBSCRIBE_PEAK_FLOOR).length;
  let signalShortfall = false;
  const ended = await renderUntil(
    context,
    async () => {
      requireFeed();
      return feedBehind !== null || sourceExhausted !== null
        || signalFrames(frames.slice(acknowledgedAtFrame)) >= UNSUBSCRIBE_SIGNAL_FRAMES;
    },
    UNSUBSCRIBE_AFTER_WAIT,
    async () => ({ boundary: String(boundary), ...(await describe()) }),
    acknowledgedAt,
    (sample) => sample?.status !== undefined
      && sample.status.renderedQuanta - boundary >= BigInt(UNSUBSCRIBE_SIGNAL_FRAMES),
  );
  if (ended === "deadline") signalShortfall = true;
  const afterList = frames.slice(acknowledgedAtFrame);
  feeding = false;
  await bounded("await", "the last source blocks", context, () => Promise.all([...submits]));
  requireFeed();

  const allocations = await request("renderAllocationCount", "renderAllocations",
    () => host.renderAllocationCount());
  if (allocations.result !== RESULT_OK) {
    throw requestFailure("renderAllocationCount", "renderAllocations", inFlight, allocations);
  }
  await request("dispose", "dispose", () => host.dispose());
  await bounded("await", "close", context, () => context.close());
  const maximumGrDb = (list) => list.reduce((highest, frame) => Math.max(highest, frame.trackGrDb), 0);
  // The reduction is checked on every frame received after the acknowledgement, up to the dispose;
  // the signal and low-peak counts on the frames of the wait, while the feed ran.
  const everyAfter = frames.slice(acknowledgedAtFrame);
  const low = afterList.filter((frame) => frame.peak <= UNSUBSCRIBE_PEAK_FLOOR);
  return {
    contextStateBeforeBoot: stateBeforeBoot,
    meterLeaseResult: meterLease.result,
    subscribeResult: subscribed.result,
    subscribePath: subscribed.path,
    unsubscribeResult: unsubscribed.result,
    unsubscribePath: unsubscribed.path,
    renderedAfterSubscribe: String(renderedAfterSubscribe),
    renderedBeforeUnsubscribe: String(renderedBeforeUnsubscribe),
    boundary: String(boundary),
    fedBlocks: fed,
    armedFrames: armedList.length,
    armedMaximumTrackGrDb: maximumGrDb(armedList),
    framesAfterAcknowledgement: everyAfter.length,
    signalFramesAfterAcknowledgement: signalFrames(afterList),
    reducingSignalFramesAfterAcknowledgement: everyAfter
      .filter((frame) => frame.trackGrDb > 0 && frame.peak > UNSUBSCRIBE_PEAK_FLOOR).length,
    lowPeakFramesAfterAcknowledgement: low.length,
    minimumPeakAfterAcknowledgement: afterList.reduce((lowest, frame) => Math.min(lowest, frame.peak), Infinity),
    maximumTrackGrDbAfterAcknowledgement: maximumGrDb(everyAfter),
    signalShortfall,
    feedBehind,
    sourceExhausted,
    maximumSourcesInFlight,
    renderAllocations: allocations.count,
  };
}

/// Issue #1535 D5: the `observation-unsubscribe` gate's named failures, empty on a pass. A frame
/// after the acknowledgement at or below the peak floor is not a failure (root ruling R1).
export function unsubscribeFailures(run) {
  if (run === null || typeof run !== "object") return ["result"];
  const wholeNumber = (value) => typeof value === "string" && /^[0-9]+$/.test(value);
  const failures = [];
  if (run.contextStateBeforeBoot !== "suspended") failures.push("context");
  // An `Observe`-only transaction is a monitor edit: an applied one replies `monitor` (#1584 D1).
  if (run.meterLeaseResult !== RESULT_OK || run.subscribeResult !== RESULT_OK || run.subscribePath !== "monitor"
      || !(run.armedFrames >= UNSUBSCRIBE_ARMED_FRAMES) || !(run.armedMaximumTrackGrDb > 0)) {
    failures.push("armed");
  }
  if (!wholeNumber(run.renderedAfterSubscribe) || !wholeNumber(run.renderedBeforeUnsubscribe)
      || BigInt(run.renderedBeforeUnsubscribe) <= BigInt(run.renderedAfterSubscribe)) {
    failures.push("render-between");
  }
  if (run.unsubscribeResult !== RESULT_OK || run.unsubscribePath !== "monitor") failures.push("unsubscribe-ack");
  if (run.maximumTrackGrDbAfterAcknowledgement !== 0 || !(run.framesAfterAcknowledgement >= 1)) {
    failures.push("reduction");
  }
  if (run.signalShortfall !== false || run.sourceExhausted !== null
      || !(run.signalFramesAfterAcknowledgement >= UNSUBSCRIBE_SIGNAL_FRAMES)) {
    failures.push("shortfall");
  }
  if (run.feedBehind !== null || !(run.maximumSourcesInFlight <= RING_BLOCKS)) failures.push("feed");
  return failures;
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
      STALL_RING_FRAMES,
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
    ringFrames: STALL_RING_FRAMES,
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

/// Issue #1333 D3: a rendered session with an observation read (#1590: the core's `OBSERVATION_READ`
/// through the control entry, since the browser's own staging read is gone). The instance reports its count before its dispose. The browser reads
/// spectrum (#1597) and the track response (#1580) through the session's frames, on the control
/// side: this run also sends one strip `RESPONSE_GET` before it renders (#1580 D2), so the render
/// that answers it is inside the counted workload.
async function runStagingReadRun(createHost, sessionDocument) {
  const context = new OfflineAudioContext(2, STAGING_READ_FRAMES, SAMPLE_RATE);
  const host = await createHost({
    context,
    document: sessionDocument,
    options: bootOptions(
      STAGING_READ_FRAMES,
      LIVE_CONTROL_METER_BLOCKS,
    ),
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
  // Issue #1589c: the arm is the core's `Observe` edit through the host's `control` message.
  const {
    observeThroughSdk, readObservationThroughSdk, requestTrackResponseThroughSdk,
  } = await import("/sdk/sdk-response-client.js");
  const subscribed = await observeThroughSdk(host, {
    stripId: "track",
    rack: "inserts",
    effectId: "comp",
    tapId: OBSERVATION_TAP_ID,
    windowBlocks: Number(LIVE_CONTROL_METER_BLOCKS),
    armed: true,
  });
  const run = { subscribeResult: subscribed.result, subscribeStatus: subscribed.status };
  // Issue #1580 D2: the track response through the core's frames, as every host reads it. The
  // capture runs in a render step, so the request goes out before the context renders.
  const response = await requestTrackResponseThroughSdk(host, "track");
  try {
    await context.startRendering();
    // #1590: the row reads through the core's `OBSERVATION_READ`, the browser's one read path.
    const observation = await readObservationThroughSdk(host, {
      stripId: "track", rack: "inserts", effectId: "comp", tapId: OBSERVATION_TAP_ID, channels: "both",
    });
    run.observationReadResult = observation.result;
    run.observationRows = observation.rows;
    const ready = await response.ready(TRACK_RESPONSE_WAIT_MS);
    run.trackResponse = {
      result: response.result,
      replyStatus: response.status,
      events: ready.events,
      tagged: ready.tagged,
      status: ready.status,
    };
  } finally {
    response.stop();
  }
  run.renderAllocations = await renderAllocationCount(host);
  await host.dispose();
  return run;
}

export async function runQualification() {
  const resumeRecord = countResumes();
  const [
    { createMisoAudioWorkletHost: createShippedHost }, expectedResponse, sessionResponse, sourceResponse, stallResponse,
    liveControlResponse, observationResponse, emptyResponse, emptyFrameResponse, unsubscribeResponse,
    structuralEditResponse, { worstReplaceDocument },
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
      fetch("/qualification/unsubscribe-session.json"),
      fetch("/fixture/structural-edit-session.json"),
      import("/qualification/worst-replace-document.js"),
    ]);
  if (!expectedResponse.ok || !sessionResponse.ok || !sourceResponse.ok || !stallResponse.ok
      || !liveControlResponse.ok || !observationResponse.ok || !emptyResponse.ok
      || !emptyFrameResponse.ok || !unsubscribeResponse.ok || !structuralEditResponse.ok) {
    throw new Error("qualification fixture fetch failed");
  }
  // Issue #1332 S9: every host this page boots, in boot order, with the mode it reports. The page
  // never chooses the mode: the host picks `worker` on an isolated page and `single` otherwise.
  const instanceModes = [];
  const createMisoAudioWorkletHost = async (arguments_) => {
    const host = await createShippedHost(arguments_);
    instanceModes.push(host.instanceMode);
    return host;
  };
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
  // #1535: the real-context unsubscribe run's own session, with a 512-block source.
  const unsubscribeDocument = new TextEncoder().encode(await unsubscribeResponse.text());
  // #1295: the browser fixture's session plus one track with an EQ insert, the replacement document.
  const structuralEditDocument = new TextEncoder().encode(await structuralEditResponse.text());
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
      structuralReplace: null,
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
  let structuralReplace;
  try {
    structuralReplace = await runStructuralReplaceQualification(
      createMisoAudioWorkletHost,
      corpusDocument,
      structuralEditDocument,
      source,
      worstReplaceDocument(),
    );
  } catch (error) {
    const diagnostic = await diagnoseReady(corpusDocument);
    throw new Error(`structural-replace qualification failed: ${diagnosticJson({
      name: error?.name, message: error?.message, ...error, diagnostic,
    })}`);
  }
  try {
    structuralReplace.growth = await runGrowthWhileRenderingRun(
      createMisoAudioWorkletHost,
      corpusDocument,
      worstReplaceDocument(),
    );
  } catch (error) {
    const failure = new Error(`growth-while-rendering qualification failed: ${error?.message ?? String(error)}`);
    if (error?.stallSignature !== undefined) failure.stallSignature = error.stallSignature;
    throw failure;
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
  try {
    observation.unsubscribe = await runObservationUnsubscribeRun(
      createMisoAudioWorkletHost,
      unsubscribeDocument,
    );
  } catch (error) {
    // #1535 D3.2: the message and the stall signature only. Every failure of the run is an `Error`
    // (a refused request is `requestFailure`'s), so no raw record reaches the page.
    const failure = new Error(`observation-unsubscribe qualification failed: ${error?.message ?? String(error)}`);
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
    stagingReads = await runStagingReadRun(
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
    crossOriginIsolated: globalThis.crossOriginIsolated === true,
    instanceModes,
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
    structuralReplace,
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
      { workload: "structural-edit", count: structuralReplace.edit.renderAllocations },
      { workload: "structural-replace-large", count: structuralReplace.large.renderAllocations },
      { workload: "growth-while-rendering", count: structuralReplace.growth.renderAllocations },
      { workload: "observation-armed", count: observation.armed.renderAllocations },
      { workload: "observation-disarmed", count: observation.disarmed.renderAllocations },
      { workload: "observation-unsubscribe", count: observation.unsubscribe.renderAllocations },
      { workload: "stall", count: stall.renderAllocations },
      { workload: "staging-reads", count: stagingReads.renderAllocations },
    ],
    sdkResponse,
    // Every `AudioContext.resume()` of the page, for the `resume-clicks` gate: `declared` is the click
    // plan's length, which `run.mjs` also holds the harness to.
    resumes: { declared: resumeClickPlan(sdkResponse !== null).length, ...resumeRecord },
  };
}

export const qualificationConstants = Object.freeze({
  stallRingFrames: STALL_RING_FRAMES,
  minimumStallMs: MINIMUM_STALL_MS,
  processorName: PROCESSOR_NAME,
  quantumFrames: QUANTUM_FRAMES,
  sourceChannels: 2,
  sourceBitDepth: "32f",
  ringBlocks: RING_BLOCKS,
  unsubscribeSignalFrames: UNSUBSCRIBE_SIGNAL_FRAMES,
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
