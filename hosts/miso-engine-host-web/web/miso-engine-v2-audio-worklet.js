const ABI_VERSION = 0x00010000;
const CONFIG_BYTES = 192;
const RESULT_OK = 0;
const RESULT_INVALID_ARGUMENT = 1;
const RESULT_REPREPARE_REQUIRED = 9;
const RESULT_INTERNAL = 255;
const BUFFER_SESSION_TOML = 1;
const BUFFER_SOURCE_ID = 2;
const BUFFER_SOURCE_PCM = 3;
const BUFFER_OUTPUT_PCM = 5;
const STATE_READY = 2;
const PROCESSOR_NAME = "miso-engine-v2-audio-worklet";

const INIT_FIELDS = [
  "requestId", "module", "backend", "sampleRateHz", "quantumFrames", "sessionToml", "limits",
];
const LIMIT_FIELDS = [
  "sessionTomlBytes", "diagnosticBytes", "sourceIdBytes", "maximumSourceChannels",
  "sourceRingFrames", "maximumAutomationSpansPerBlock", "maximumTracks", "maximumSources",
  "maximumRoutes", "maximumEffects", "maximumGraphSessionPlusPlanBytes",
  "maximumSourceTotalBytes", "maximumSourceOverheadBytes", "maximumEffectStateBytes",
  "maximumEffectScratchBytes", "maximumBuiltinRetainedBytes", "maximumHostRetainedBytes",
  "maximumNamedAllocationBytes", "maximumMeterStreams", "maximumMeterItems", "maximumMeterBytes",
];
const SOURCE_FIELDS = [
  "tag", "requestId", "sourceId", "generation", "startFrame", "sampleRateHz", "planes", "frames",
  "endOfRegion",
];
const SEEK_FIELDS = ["tag", "requestId", "sourceId", "generation", "sourceFrame"];

function exactFields(value, fields) {
  if (value === null || typeof value !== "object") return false;
  const keys = Object.keys(value).sort();
  const expected = [...fields].sort();
  return keys.length === expected.length && keys.every((key, index) => key === expected[index]);
}

function u32(value) {
  return Number.isInteger(value) && value >= 0 && value <= 0xffffffff;
}

function positiveU64(value) {
  return typeof value === "bigint" && value > 0n && value <= 0xffffffffn;
}

class MisoEngineV2AudioWorkletProcessor extends AudioWorkletProcessor {
  constructor(options) {
    super();
    this.ready = false;
    this.disposed = false;
    this.stickyResult = RESULT_OK;
    this.lastRequestId = 0;
    this.handle = 0;
    this.port.onmessage = (event) => this.receive(event.data);
    try {
      this.initialize(options?.processorOptions);
    } catch (_) {
      this.sticky(RESULT_INTERNAL, 0);
    }
  }

  initialize(init) {
    if (!exactFields(init, INIT_FIELDS) || init.requestId !== 0
        || (init.backend !== "scalar" && init.backend !== "simd128")
        || !u32(init.sampleRateHz) || !u32(init.quantumFrames) || init.quantumFrames === 0
        || !(init.sessionToml instanceof Uint8Array) || !exactFields(init.limits, LIMIT_FIELDS)) {
      this.sticky(RESULT_INVALID_ARGUMENT, init?.requestId ?? 0);
      return;
    }
    const exposedQuantum = globalThis.renderQuantumSize;
    if (typeof exposedQuantum === "number" && exposedQuantum !== 0
        && exposedQuantum !== init.quantumFrames) {
      this.sticky(RESULT_REPREPARE_REQUIRED, init.requestId);
      return;
    }
    this.instance = new WebAssembly.Instance(init.module, {});
    this.exports = this.instance.exports;
    if (this.exports.miso_engine_web_v1_abi_version() !== ABI_VERSION
        || this.exports.miso_engine_web_v1_config_bytes() !== CONFIG_BYTES) {
      this.sticky(2, init.requestId);
      return;
    }
    this.handle = this.exports.miso_engine_web_v1_config_new();
    const configPointer = this.exports.miso_engine_web_v1_config_ptr(this.handle);
    if (this.handle === 0 || configPointer === 0) {
      this.sticky(RESULT_INTERNAL, init.requestId);
      return;
    }
    try {
      this.writeConfig(configPointer, init);
    } catch (_) {
      this.sticky(RESULT_INVALID_ARGUMENT, init.requestId);
      return;
    }
    let result = this.exports.miso_engine_web_v1_prepare(this.handle);
    if (result !== RESULT_OK) {
      this.sticky(result, init.requestId);
      return;
    }
    const tomlPointer = this.exports.miso_engine_web_v1_buffer_ptr(this.handle, BUFFER_SESSION_TOML);
    const tomlCapacity = this.exports.miso_engine_web_v1_buffer_capacity(this.handle, BUFFER_SESSION_TOML);
    if (init.sessionToml.byteLength > tomlCapacity) {
      this.sticky(4, init.requestId);
      return;
    }
    new Uint8Array(this.exports.memory.buffer, tomlPointer, init.sessionToml.byteLength).set(init.sessionToml);
    result = this.exports.miso_engine_web_v1_compile(this.handle, init.sessionToml.byteLength);
    if (result !== RESULT_OK) {
      this.sticky(result, init.requestId);
      return;
    }
    this.backend = init.backend;
    this.sampleRateHz = init.sampleRateHz;
    this.quantumFrames = init.quantumFrames;
    this.maximumSourceChannels = init.limits.maximumSourceChannels;
    this.memoryBuffer = this.exports.memory.buffer;
    this.sourceIdPointer = this.exports.miso_engine_web_v1_buffer_ptr(this.handle, BUFFER_SOURCE_ID);
    this.sourceIdCapacity = this.exports.miso_engine_web_v1_buffer_capacity(this.handle, BUFFER_SOURCE_ID);
    this.sourcePcmPointer = this.exports.miso_engine_web_v1_buffer_ptr(this.handle, BUFFER_SOURCE_PCM);
    this.sourcePcm = new Float32Array(
      this.memoryBuffer,
      this.sourcePcmPointer,
      this.exports.miso_engine_web_v1_buffer_capacity(this.handle, BUFFER_SOURCE_PCM) / 4,
    );
    const outputPointer = this.exports.miso_engine_web_v1_buffer_ptr(this.handle, BUFFER_OUTPUT_PCM);
    this.outputLeft = new Float32Array(this.memoryBuffer, outputPointer, this.quantumFrames);
    this.outputRight = new Float32Array(
      this.memoryBuffer,
      outputPointer + this.quantumFrames * 4,
      this.quantumFrames,
    );
    this.statusView = new DataView(
      this.memoryBuffer,
      this.exports.miso_engine_web_v1_status_ptr(this.handle),
      80,
    );
    this.resourceView = new DataView(
      this.memoryBuffer,
      this.exports.miso_engine_web_v1_resource_ptr(this.handle),
      224,
    );
    this.encoder = new TextEncoder();
    this.resources = this.readResources();
    this.memoryBytes = this.memoryBuffer.byteLength;
    this.ready = true;
    this.port.postMessage({
      tag: "miso.ready.v1",
      requestId: init.requestId,
      result: RESULT_OK,
      backend: this.backend,
      resources: this.resources,
      memoryBytes: this.memoryBytes,
    });
  }

  writeConfig(pointer, init) {
    const limits = init.limits;
    const values32 = [
      CONFIG_BYTES, ABI_VERSION, init.sampleRateHz, init.quantumFrames, limits.sessionTomlBytes,
      limits.diagnosticBytes, limits.sourceIdBytes, limits.maximumSourceChannels,
      limits.sourceRingFrames, limits.maximumAutomationSpansPerBlock,
    ];
    if (values32.some((value) => !u32(value))
        || init.sessionToml.byteLength > limits.sessionTomlBytes) {
      throw new RangeError("Invalid u32 preparation limit");
    }
    const values64 = [
      limits.maximumTracks, limits.maximumSources, limits.maximumRoutes, limits.maximumEffects,
      limits.maximumGraphSessionPlusPlanBytes, limits.maximumSourceTotalBytes,
      limits.maximumSourceOverheadBytes, limits.maximumEffectStateBytes,
      limits.maximumEffectScratchBytes, limits.maximumBuiltinRetainedBytes,
      limits.maximumHostRetainedBytes, limits.maximumNamedAllocationBytes,
      limits.maximumMeterStreams, limits.maximumMeterItems, limits.maximumMeterBytes,
    ];
    if (values64.some((value) => !positiveU64(value))) {
      throw new RangeError("Invalid u64 preparation limit");
    }
    const view = new DataView(this.exports.memory.buffer, pointer, CONFIG_BYTES);
    values32.forEach((value, index) => view.setUint32(index * 4, value, true));
    values64.forEach((value, index) => view.setBigUint64(40 + index * 8, value, true));
    for (let index = 0; index < 4; index += 1) view.setBigUint64(160 + index * 8, 0n, true);
  }

  readResources() {
    const view = this.resourceView;
    if (view.getUint32(0, true) !== 224 || view.getUint32(4, true) !== ABI_VERSION) {
      throw new RangeError("Invalid resource report");
    }
    const names = [
      "configBytes", "statusBytes", "sessionTomlBytes", "diagnosticBytes", "sourceIdBytes",
      "sourcePcmStagingBytes", "outputPcmBytes", "bridgeMetadataBytes", "bridgeRetainedBytes",
      "largestBridgeAllocationBytes", "sourceTotalBytes", "sourceOverheadBytes",
      "effectScalarStateBytes", "effectScalarScratchBytes", "builtinRetainedBytes",
      "graphSessionPlusPlanBytes", "graphIncrementalPlanBytes", "graphMetadataBytes",
      "graphDelayBytes", "largestNamedAllocationBytes",
    ];
    const resources = {
      sampleRateHz: view.getUint32(8, true),
      quantumFrames: view.getUint32(12, true),
      backend: view.getUint32(16, true),
    };
    names.forEach((name, index) => { resources[name] = view.getBigUint64(32 + index * 8, true); });
    return Object.freeze(resources);
  }

  readStatus() {
    const view = this.statusView;
    if (view.getUint32(0, true) !== 80 || view.getUint32(4, true) !== ABI_VERSION) {
      throw new RangeError("Invalid status report");
    }
    return Object.freeze({
      state: view.getUint32(8, true),
      lastResult: this.stickyResult || view.getUint32(12, true),
      backend: view.getUint32(16, true),
      sampleRateHz: view.getUint32(20, true),
      quantumFrames: view.getUint32(24, true),
      nextAbsoluteSample: view.getBigUint64(32, true),
      renderedQuanta: view.getBigUint64(40, true),
    });
  }

  transferList(planes) {
    if (!Array.isArray(planes)) return [];
    return [...new Set(planes
      .filter((plane) => plane instanceof Float32Array && plane.buffer instanceof ArrayBuffer)
      .map((plane) => plane.buffer))];
  }

  acknowledge(requestId, result, planes) {
    const message = { tag: "miso.ack.v1", requestId, result };
    if (planes !== undefined) message.planes = planes;
    this.port.postMessage(message, this.transferList(planes));
  }

  sticky(result, requestId, planes) {
    this.ready = false;
    this.stickyResult = result;
    const message = { tag: "miso.error.v1", requestId, result };
    if (planes !== undefined) message.planes = planes;
    this.port.postMessage(message, this.transferList(planes));
  }

  receive(message) {
    if (this.disposed) return;
    const returnedPlanes = Array.isArray(message?.planes) ? message.planes : undefined;
    if (!Number.isSafeInteger(message?.requestId) || message.requestId <= this.lastRequestId) {
      this.sticky(RESULT_INVALID_ARGUMENT, message?.requestId ?? 0, returnedPlanes);
      return;
    }
    this.lastRequestId = message.requestId;
    if (message.tag === "miso.dispose.v1" && exactFields(message, ["tag", "requestId"])) {
      const result = this.exports.miso_engine_web_v1_dispose(this.handle);
      if (result === RESULT_OK) this.disposed = true;
      this.acknowledge(message.requestId, result);
      return;
    }
    if (this.stickyResult !== RESULT_OK) {
      this.sticky(this.stickyResult, message.requestId, returnedPlanes);
      return;
    }
    if (this.exports.memory.buffer !== this.memoryBuffer) {
      this.sticky(RESULT_REPREPARE_REQUIRED, message.requestId, returnedPlanes);
      return;
    }
    if (message?.tag === "miso.source.v1" && exactFields(message, SOURCE_FIELDS)) {
      this.receiveSource(message);
    } else if (message?.tag === "miso.seek.v1" && exactFields(message, SEEK_FIELDS)) {
      this.receiveSeek(message);
    } else if (message?.tag === "miso.status.v1"
        && exactFields(message, ["tag", "requestId"])) {
      this.port.postMessage({
        tag: "miso.status.v1",
        requestId: message.requestId,
        result: RESULT_OK,
        ...this.readStatus(),
        memoryBytes: this.exports.memory.buffer.byteLength,
      });
    } else {
      this.sticky(RESULT_INVALID_ARGUMENT, message?.requestId ?? 0, returnedPlanes);
    }
  }

  receiveSource(message) {
    if (!Number.isSafeInteger(message.requestId) || message.requestId <= 0
        || typeof message.sourceId !== "string" || typeof message.generation !== "bigint"
        || message.generation <= 0n || typeof message.startFrame !== "bigint"
        || message.startFrame < 0n || message.sampleRateHz !== this.sampleRateHz
        || !u32(message.frames) || message.frames > this.quantumFrames
        || typeof message.endOfRegion !== "boolean" || !Array.isArray(message.planes)
        || message.planes.length === 0 || message.planes.length > this.maximumSourceChannels
        || message.planes.some((plane) => !(plane instanceof Float32Array)
          || !(plane.buffer instanceof ArrayBuffer)
          || (typeof SharedArrayBuffer !== "undefined" && plane.buffer instanceof SharedArrayBuffer)
          || plane.length !== message.frames)) {
      this.sticky(RESULT_INVALID_ARGUMENT, message.requestId ?? 0, message.planes);
      return;
    }
    const id = this.encoder.encode(message.sourceId);
    if (id.byteLength > this.sourceIdCapacity) {
      this.sticky(4, message.requestId, message.planes);
      return;
    }
    new Uint8Array(this.memoryBuffer, this.sourceIdPointer, id.byteLength).set(id);
    message.planes.forEach((plane, channel) => {
      this.sourcePcm.set(plane, channel * this.quantumFrames);
    });
    const result = this.exports.miso_engine_web_v1_source_submit(
      this.handle, id.byteLength, message.generation, message.startFrame, message.planes.length,
      message.frames, message.endOfRegion ? 1 : 0,
    );
    this.acknowledge(message.requestId, result, message.planes);
  }

  receiveSeek(message) {
    if (!Number.isSafeInteger(message.requestId) || message.requestId <= 0
        || typeof message.sourceId !== "string" || typeof message.generation !== "bigint"
        || message.generation <= 0n || typeof message.sourceFrame !== "bigint"
        || message.sourceFrame < 0n) {
      this.sticky(RESULT_INVALID_ARGUMENT, message.requestId ?? 0);
      return;
    }
    const id = this.encoder.encode(message.sourceId);
    if (id.byteLength > this.sourceIdCapacity) {
      this.sticky(4, message.requestId);
      return;
    }
    new Uint8Array(this.memoryBuffer, this.sourceIdPointer, id.byteLength).set(id);
    this.acknowledge(
      message.requestId,
      this.exports.miso_engine_web_v1_source_seek(
        this.handle, id.byteLength, message.generation, message.sourceFrame,
      ),
    );
  }

  silence(outputs) {
    for (let outputIndex = 0; outputIndex < outputs.length; outputIndex += 1) {
      const output = outputs[outputIndex];
      for (let planeIndex = 0; planeIndex < output.length; planeIndex += 1) {
        output[planeIndex].fill(0);
      }
    }
  }

  // PROCESS_POLICY_BEGIN
  process(_inputs, outputs) {
    if (this.disposed) {
      this.silence(outputs);
      return false;
    }
    const output = outputs[0];
    if (!this.ready || this.stickyResult !== RESULT_OK) {
      this.silence(outputs);
      return true;
    }
    if (this.exports.memory.buffer !== this.memoryBuffer) {
      this.stickyResult = RESULT_REPREPARE_REQUIRED;
      this.ready = false;
      this.silence(outputs);
      return true;
    }
    const actualFrames = output !== undefined && output.length === 2
        && output[0].length === this.quantumFrames && output[1].length === this.quantumFrames
      ? this.quantumFrames
      : 0;
    const result = this.exports.miso_engine_web_v1_render(this.handle, actualFrames);
    if (result !== RESULT_OK) {
      this.stickyResult = result;
      this.ready = false;
      this.silence(outputs);
      return true;
    }
    if (actualFrames === 0) {
      this.stickyResult = RESULT_REPREPARE_REQUIRED;
      this.ready = false;
      this.silence(outputs);
      return true;
    }
    output[0].set(this.outputLeft);
    output[1].set(this.outputRight);
    return true;
  }
  // PROCESS_POLICY_END
}

registerProcessor(PROCESSOR_NAME, MisoEngineV2AudioWorkletProcessor);
