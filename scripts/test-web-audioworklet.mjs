import {
  copyFile,
  mkdtemp,
  readFile,
  readdir,
  rm,
  writeFile,
} from "node:fs/promises";
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { MessageChannel } from "node:worker_threads";
const root = new URL("../", import.meta.url);
const commandArguments = process.argv.slice(2);
const realWasmReceiverRequested = commandArguments.includes("--real-wasm-receiver");
const snapshotModeFlags = [
  ["--snapshot-document", "document"],
  ["--snapshot-nested", "nested"],
  ["--snapshot-single-read", "single-read"],
  ["--snapshot-existing", "existing"],
].filter(([flag]) => commandArguments.includes(flag));
if (snapshotModeFlags.length > 1) throw new TypeError("only one snapshot test mode is allowed");
const snapshotTestMode = snapshotModeFlags[0]?.[1] ?? "all";
const qualificationModuleIndex = commandArguments.indexOf("--qualification-module");
if (qualificationModuleIndex !== -1 && typeof commandArguments[qualificationModuleIndex + 1] !== "string") {
  throw new TypeError("--qualification-module requires a file path");
}

function parseRealWasmReceiverArguments() {
  if (!realWasmReceiverRequested) return null;
  const fakeMutationOverrides = [
    "MISO_ENGINE_WEB_HOST_TEST_MODULE",
    "MISO_ENGINE_WEB_WORKLET_TEST_MODULE",
    "MISO_ENGINE_WEB_HOST_MAX_SAFE_TEST",
  ];
  const activeOverride = fakeMutationOverrides.find((name) => (
    Object.prototype.hasOwnProperty.call(process.env, name)
  ));
  if (activeOverride !== undefined) {
    throw new TypeError(`--real-wasm-receiver rejects fake mutation override ${activeOverride}`);
  }
  if (qualificationModuleIndex !== -1) {
    throw new TypeError("--real-wasm-receiver rejects --qualification-module");
  }
  const artifactsIndex = commandArguments.indexOf("--artifacts");
  if (artifactsIndex === -1 || typeof commandArguments[artifactsIndex + 1] !== "string") {
    throw new TypeError("--real-wasm-receiver requires --artifacts DIR");
  }
  const consumed = new Set([0, artifactsIndex, artifactsIndex + 1]);
  const realIndex = commandArguments.indexOf("--real-wasm-receiver");
  consumed.add(realIndex);
  if (consumed.size !== commandArguments.length
      || commandArguments.filter((argument) => argument === "--real-wasm-receiver").length !== 1
      || commandArguments.filter((argument) => argument === "--artifacts").length !== 1
      || realIndex === artifactsIndex + 1) {
    throw new TypeError("usage: --real-wasm-receiver --artifacts DIR");
  }
  return Object.freeze({ artifactDirectory: resolve(commandArguments[artifactsIndex + 1]) });
}

const realWasmReceiverArguments = parseRealWasmReceiverArguments();
if (realWasmReceiverArguments === null) {
  await import("./test-prepared-control.mjs");
}
const preparedAbiLayout = realWasmReceiverArguments === null
  ? JSON.parse(await readFile(
    new URL("../sdk/assets/miso-engine-v1-abi-layout.json", import.meta.url), "utf8",
  ))
  : undefined;
const unsupportedPreparationModuleBytes = Uint8Array.from([
  0, 97, 115, 109, 1, 0, 0, 0,
  1, 5, 1, 96, 0, 1, 127,
  3, 2, 1, 0,
  5, 3, 1, 0, 1,
  7, 46, 2, 6, 109, 101, 109, 111, 114, 121, 2, 0,
  33, 109, 105, 115, 111, 95, 101, 110, 103, 105, 110, 101, 95, 119, 101, 98,
  95, 118, 49, 95, 101, 113, 95, 116, 97, 114, 103, 101, 116, 95, 111, 112, 101, 110, 0, 0,
  10, 6, 1, 4, 0, 65, 7, 11,
]);
// Issue #151: the host module is overridable for exactly the reason the worklet module already is
// -- so a red mutation of the shipped host runs this same suite and is required to fail it.
const hostUrl = process.env.MISO_ENGINE_WEB_HOST_TEST_MODULE === undefined
  ? new URL("hosts/host-web/web/miso-engine-v1-audio-worklet-host.js", root)
  : pathToFileURL(process.env.MISO_ENGINE_WEB_HOST_TEST_MODULE);
const workletUrl = process.env.MISO_ENGINE_WEB_WORKLET_TEST_MODULE === undefined
  ? new URL("hosts/host-web/web/miso-engine-v1-audio-worklet.js", root)
  : pathToFileURL(process.env.MISO_ENGINE_WEB_WORKLET_TEST_MODULE);
const qualificationUrl = qualificationModuleIndex === -1
  ? new URL("hosts/host-web/qualification/qualification.js", root)
  : pathToFileURL(commandArguments[qualificationModuleIndex + 1]);

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
    idStagingBytes: 1n,
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

async function localErrorResult(promise, result) {
  const error = await errorResult(promise, result);
  assert.equal(error.requestId, 0, "local refusal carries no allocated ID");
  return error;
}

const REAL_STAGE_TIMEOUT_MS = 5000;
const REAL_PROCESSOR_NAME = "miso-engine-v1-audio-worklet";
const REAL_WASM_SHA256 = "b47d05f053dca81687f0065306fb97159f892277e9543326cea7e21544186b61";
const REAL_WASM_FILE = "miso-engine-v1-audio-worklet.simd128.wasm";
const REAL_ARTIFACT_NAMES = Object.freeze([
  "miso-engine-v1-abi-layout.json",
  "miso-engine-v1-audio-worklet-host.d.ts",
  "miso-engine-v1-audio-worklet-host.js",
  "miso-engine-v1-audio-worklet.js",
  REAL_WASM_FILE,
  "miso-engine-v1-parameter-metadata.json",
  "prepared-control.js",
]);
const REAL_ARTIFACT_AUTHORITIES = new Map([
  ["miso-engine-v1-abi-layout.json", new URL("sdk/assets/miso-engine-v1-abi-layout.json", root)],
  ["miso-engine-v1-audio-worklet-host.d.ts", new URL(
    "hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts", root,
  )],
  ["miso-engine-v1-audio-worklet-host.js", new URL(
    "hosts/host-web/web/miso-engine-v1-audio-worklet-host.js", root,
  )],
  ["miso-engine-v1-audio-worklet.js", new URL(
    "hosts/host-web/web/miso-engine-v1-audio-worklet.js", root,
  )],
  ["miso-engine-v1-parameter-metadata.json", new URL(
    "sdk/assets/miso-engine-v1-parameter-metadata.json", root,
  )],
  ["prepared-control.js", new URL("hosts/host-web/web/prepared-control.js", root)],
]);
const REAL_WASM_PIN_URL = new URL(
  "hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256", root,
);
const REAL_GLOBAL_NAMES = Object.freeze([
  "fetch", "AudioWorkletNode", "AudioWorkletProcessor", "registerProcessor", "sampleRate",
  "renderQuantumSize",
]);
const NATIVE_RELEASE_ASSERTION = "ordinary native disposal releases saved native handle";

function realDeadline(label, operation) {
  return new Promise((resolvePromise, rejectPromise) => {
    let settled = false;
    const timer = setTimeout(() => {
      settled = true;
      rejectPromise(new Error(`${label} timed out after ${REAL_STAGE_TIMEOUT_MS}ms`));
    }, REAL_STAGE_TIMEOUT_MS);
    Promise.resolve().then(operation).then(
      (value) => {
        if (settled) return;
        settled = true;
        clearTimeout(timer);
        resolvePromise(value);
      },
      (error) => {
        if (settled) return;
        settled = true;
        clearTimeout(timer);
        rejectPromise(error);
      },
    );
  });
}

function realArtifactPath(directory, name) {
  return join(directory, name);
}

function realPathUrl(directory, name) {
  return pathToFileURL(realArtifactPath(directory, name)).href;
}

async function preflightRealArtifact(directory) {
  const artifactDirectory = resolve(directory);
  const entries = await readdir(artifactDirectory, { withFileTypes: true });
  const names = entries.map((entry) => entry.name).sort();
  const expectedNames = [...REAL_ARTIFACT_NAMES].sort();
  if (names.length !== expectedNames.length
      || names.some((name, index) => name !== expectedNames[index])) {
    throw new Error(
      `real-Wasm artifact preflight: exact seven-file membership mismatch: ${names.join(",")}`,
    );
  }
  if (entries.some((entry) => !entry.isFile())) {
    throw new Error("real-Wasm artifact preflight: every artifact entry must be a regular file");
  }
  const fileEntries = await Promise.all(REAL_ARTIFACT_NAMES.map(async (name) => [
    name,
    await readFile(realArtifactPath(artifactDirectory, name)),
  ]));
  const files = new Map(fileEntries);
  const pin = (await readFile(REAL_WASM_PIN_URL, "utf8")).trim();
  if (pin !== REAL_WASM_SHA256) {
    throw new Error("real-Wasm artifact preflight: repository Wasm SHA-256 pin changed");
  }
  const wasmSha256 = createHash("sha256").update(files.get(REAL_WASM_FILE)).digest("hex");
  if (wasmSha256 !== REAL_WASM_SHA256) {
    throw new Error(`real-Wasm artifact preflight: Wasm SHA-256 mismatch: ${wasmSha256}`);
  }
  for (const [name, authorityUrl] of REAL_ARTIFACT_AUTHORITIES) {
    const authority = await readFile(authorityUrl);
    if (Buffer.compare(files.get(name), authority) !== 0) {
      throw new Error(`real-Wasm artifact preflight: ${name} differs from source authority`);
    }
  }
  return Object.freeze({
    directory: artifactDirectory,
    files,
    wasmPath: realArtifactPath(artifactDirectory, REAL_WASM_FILE),
    wasmUrl: realPathUrl(artifactDirectory, REAL_WASM_FILE),
    abiPath: realArtifactPath(artifactDirectory, "miso-engine-v1-abi-layout.json"),
    abiUrl: realPathUrl(artifactDirectory, "miso-engine-v1-abi-layout.json"),
    hostUrl: realPathUrl(artifactDirectory, "miso-engine-v1-audio-worklet-host.js"),
    workletUrl: realPathUrl(artifactDirectory, "miso-engine-v1-audio-worklet.js"),
    workletSource: files.get("miso-engine-v1-audio-worklet.js").toString("utf8"),
    wasmSha256,
  });
}

function realArrayBuffer(bytes) {
  const copy = new Uint8Array(bytes.byteLength);
  copy.set(bytes);
  return copy.buffer;
}

function defineRealGlobal(name, value) {
  Object.defineProperty(globalThis, name, {
    configurable: true,
    enumerable: true,
    writable: true,
    value,
  });
}

function createRealWebAudioFacade(artifact, allowedWorkletUrl) {
  let active = true;
  let currentProcessorPort = null;
  let registeredProcessor = null;
  let registeredProcessorName = null;
  let constructionCount = 0;
  const allNodes = new Set();
  const pendingModuleLoads = new Set();
  const fetches = [];
  const fetchTargets = new Map([
    [artifact.wasmUrl, artifact.wasmPath],
    [artifact.abiUrl, artifact.abiPath],
  ]);

  function registerProcessor(name, processor) {
    if (!active) throw new Error("real Web Audio facade is torn down");
    if (name !== REAL_PROCESSOR_NAME || typeof processor !== "function") {
      throw new Error(`unexpected AudioWorklet registration: ${name}`);
    }
    if (registeredProcessor !== null) {
      throw new Error("AudioWorklet processor registered more than once");
    }
    registeredProcessorName = name;
    registeredProcessor = processor;
  }

  async function fetchVerifiedFile(input) {
    if (!active) throw new Error("real Web Audio facade is torn down");
    const url = input instanceof URL ? input.href : input;
    if (typeof url !== "string" || !fetchTargets.has(url)) {
      throw new Error(`unverified real-Wasm fetch refused: ${String(url)}`);
    }
    fetches.push(url);
    const bytes = await readFile(fetchTargets.get(url));
    if (!active) throw new Error("real Web Audio facade was torn down during fetch");
    return {
      ok: true,
      arrayBuffer: async () => {
        if (url !== artifact.wasmUrl) throw new Error("non-Wasm arrayBuffer fetch refused");
        return realArrayBuffer(bytes);
      },
      json: async () => {
        if (url !== artifact.abiUrl) throw new Error("non-ABI JSON fetch refused");
        return JSON.parse(bytes.toString("utf8"));
      },
    };
  }

  async function addModule(moduleUrl) {
    if (!active || moduleUrl !== allowedWorkletUrl) {
      throw new Error(`unverified worklet URL refused: ${String(moduleUrl)}`);
    }
    registeredProcessor = null;
    registeredProcessorName = null;
    const loading = (async () => {
      await import(moduleUrl);
      if (!active) throw new Error("real Web Audio facade was torn down during addModule");
      if (registeredProcessorName !== REAL_PROCESSOR_NAME || registeredProcessor === null) {
        throw new Error("registered worklet processor did not attest to the shipped name");
      }
    })();
    pendingModuleLoads.add(loading);
    try {
      await loading;
    } finally {
      pendingModuleLoads.delete(loading);
    }
  }

  class RealAudioWorkletProcessor {
    constructor() {
      if (currentProcessorPort === null) {
        throw new Error("AudioWorkletProcessor was constructed without a native port");
      }
      this.port = currentProcessorPort;
    }
  }

  class RealAudioWorkletNode {
    constructor(context, name, options) {
      if (!active) throw new Error("real Web Audio facade is torn down");
      if (name !== REAL_PROCESSOR_NAME || registeredProcessorName !== REAL_PROCESSOR_NAME
          || registeredProcessor === null) {
        throw new Error(`unattested AudioWorklet processor: ${name}`);
      }
      const processorOptions = options?.processorOptions;
      if (!(processorOptions?.document instanceof Uint8Array)) {
        throw new Error("AudioWorklet construction options omitted the document");
      }
      const channel = new MessageChannel();
      this.port = channel.port1;
      this.processorPort = channel.port2;
      const originalHostPostMessageDescriptor = Object.getOwnPropertyDescriptor(this.port, "postMessage");
      const originalHostPostMessage = this.port.postMessage;
      const node = this;
      const instrumentedHostPostMessage = function (...args) {
        node.hostPostMessageAttempts += 1;
        return Reflect.apply(originalHostPostMessage, this, args);
      };
      Object.defineProperty(this.port, "postMessage", {
        configurable: true,
        enumerable: originalHostPostMessageDescriptor?.enumerable ?? true,
        writable: true,
        value: instrumentedHostPostMessage,
      });
      let hostPostMessageInstrumentationRestored = false;
      this.restoreHostPostMessageInstrumentation = () => {
        if (hostPostMessageInstrumentationRestored) return;
        if (this.port.postMessage !== instrumentedHostPostMessage) {
          throw new Error("host MessagePort postMessage instrumentation was replaced");
        }
        if (originalHostPostMessageDescriptor === undefined) {
          if (!Reflect.deleteProperty(this.port, "postMessage")) {
            throw new Error("host MessagePort postMessage instrumentation could not be removed");
          }
        } else {
          Object.defineProperty(this.port, "postMessage", originalHostPostMessageDescriptor);
        }
        hostPostMessageInstrumentationRestored = true;
      };
      this.responses = [];
      this.requests = [];
      this.messageErrors = [];
      this.hostPostMessageAttempts = 0;
      this.processor = null;
      this.constructionError = null;
      this.constructionTimer = null;
      this.tornDown = false;
      this.closedHostPort = false;
      this.closedProcessorPort = false;
      this.disconnectCount = 0;
      this.onprocessorerror = null;
      this.constructionSnapshot = Object.freeze({
        context,
        name,
        numberOfInputs: options.numberOfInputs,
        numberOfOutputs: options.numberOfOutputs,
        outputChannelCount: Object.freeze([...options.outputChannelCount]),
        processorOptions: Object.freeze({
          module: processorOptions.module,
          document: new Uint8Array(processorOptions.document),
          options: Object.freeze({ ...processorOptions.options }),
        }),
      });
      this.port.on("message", (message) => this.responses.push(message));
      this.processorPort.on("message", (message) => this.requests.push(message));
      this.port.on("messageerror", (error) => this.messageErrors.push(error));
      this.processorPort.on("messageerror", (error) => this.messageErrors.push(error));
      this.port.start();
      this.processorPort.start();
      constructionCount += 1;
      allNodes.add(this);
      this.constructionTimer = setTimeout(() => {
        this.constructionTimer = null;
        if (this.tornDown || !active) return;
        const Processor = registeredProcessor;
        if (Processor === null) {
          this.constructionError = new Error("registered processor disappeared before construction");
          this.onprocessorerror?.(this.constructionError);
          return;
        }
        currentProcessorPort = this.processorPort;
        try {
          this.processor = new Processor(options);
        } catch (error) {
          this.constructionError = error;
          this.onprocessorerror?.(error);
        } finally {
          currentProcessorPort = null;
        }
      }, 0);
    }

    disconnect() {
      this.disconnectCount += 1;
      this.teardown();
    }

    teardown() {
      if (this.tornDown) return;
      this.tornDown = true;
      if (this.constructionTimer !== null) {
        clearTimeout(this.constructionTimer);
        this.constructionTimer = null;
      }
      const errors = [];
      this.port.onmessage = null;
      this.port.onmessageerror = null;
      this.processorPort.onmessage = null;
      try {
        this.port.close();
        this.closedHostPort = true;
      } catch (error) {
        errors.push(error);
      }
      try {
        this.processorPort.close();
        this.closedProcessorPort = true;
      } catch (error) {
        errors.push(error);
      }
      this.port.removeAllListeners();
      this.processorPort.removeAllListeners();
      if (errors.length > 0) throw new AggregateError(errors, "native MessagePort cleanup failed");
    }
  }

  const facade = {
    audioWorklet: Object.freeze({ addModule }),
    fetches,
    install() {
      defineRealGlobal("fetch", fetchVerifiedFile);
      defineRealGlobal("AudioWorkletNode", RealAudioWorkletNode);
      defineRealGlobal("AudioWorkletProcessor", RealAudioWorkletProcessor);
      defineRealGlobal("registerProcessor", registerProcessor);
      defineRealGlobal("sampleRate", 48000);
      defineRealGlobal("renderQuantumSize", 128);
    },
    async teardown() {
      active = false;
      const errors = [];
      for (const node of allNodes) {
        try {
          node.teardown();
        } catch (error) {
          errors.push(error);
        }
        try {
          node.restoreHostPostMessageInstrumentation();
        } catch (error) {
          errors.push(error);
        }
      }
      const moduleResults = await Promise.allSettled([...pendingModuleLoads]);
      for (const result of moduleResults) {
        if (result.status === "rejected") errors.push(result.reason);
      }
      if (errors.length > 0) throw new AggregateError(errors, "real Web Audio facade cleanup failed");
    },
    get constructionCount() {
      return constructionCount;
    },
    get registeredProcessor() {
      return registeredProcessor;
    },
    get nodes() {
      return [...allNodes];
    },
  };
  return facade;
}

function restoreRealGlobals(descriptors) {
  for (const name of REAL_GLOBAL_NAMES) {
    const descriptor = descriptors.get(name);
    if (descriptor === undefined) delete globalThis[name];
    else Object.defineProperty(globalThis, name, descriptor);
  }
}

async function withRealWebAudioFacade(artifact, workletUrl, label, callback) {
  const descriptors = new Map(
    REAL_GLOBAL_NAMES.map((name) => [name, Object.getOwnPropertyDescriptor(globalThis, name)]),
  );
  const facade = createRealWebAudioFacade(artifact, workletUrl);
  let value;
  let operationError = null;
  try {
    facade.install();
    value = await callback(facade);
  } catch (error) {
    operationError = error;
  }
  let cleanupError = null;
  try {
    await realDeadline(`${label} cleanup`, () => facade.teardown());
  } catch (error) {
    cleanupError = error;
  }
  try {
    restoreRealGlobals(descriptors);
    for (const name of REAL_GLOBAL_NAMES) {
      assert.deepEqual(
        Object.getOwnPropertyDescriptor(globalThis, name),
        descriptors.get(name),
        `${label}: global descriptor for ${name} was not restored`,
      );
    }
  } catch (error) {
    cleanupError = cleanupError === null ? error : new AggregateError(
      [cleanupError, error],
      `${label}: global restoration failed after cleanup failure`,
    );
  }
  if (operationError !== null && cleanupError !== null) {
    throw new AggregateError(
      [operationError, cleanupError],
      `${label} failed and cleanup also failed`,
    );
  }
  if (operationError !== null) throw operationError;
  if (cleanupError !== null) throw cleanupError;
  return value;
}

function realBootOptions() {
  return Object.freeze({
    sourceRingFrames: 0,
    maximumMemoryBytes: 67108864n,
    consoleCommandQueueRecords: 0n,
    consoleMeterBlocks: 0n,
    consoleObservationTaps: 0n,
    consoleMasterTrackPlusOne: 0n,
  });
}

function realLifecycleState() {
  return {
    host: null,
    node: null,
    native: null,
    hostDisposeAcknowledged: false,
  };
}

async function ordinaryRealLifecycle(artifact, workletUrl, facade, label, state) {
  const document = new Uint8Array(await realDeadline(
    `${label} session document`,
    () => readFile(new URL("hosts/host-web/tests/browser-v1/session.json", root)),
  ));
  const options = realBootOptions();
  const context = {
    state: "suspended",
    sampleRate: 48000,
    renderQuantumSize: 128,
    audioWorklet: facade.audioWorklet,
  };
  const { createMisoAudioWorkletHost } = await realDeadline(
    `${label} host import`,
    () => import(artifact.hostUrl),
  );
  const host = await realDeadline(`${label} boot`, () => createMisoAudioWorkletHost({
    context,
    document,
    options,
    simd128ModuleUrl: artifact.wasmUrl,
    workletModuleUrl: workletUrl,
  }));
  state.host = host;
  state.node = host.node;
  assert.equal(host.backend, "simd128", `${label}: host backend`);
  assert.equal(facade.constructionCount, 1, `${label}: exactly one native node construction`);
  assert.equal(state.node.constructionError, null, `${label}: no construction error`);
  assert.equal(state.node.processor !== null, true, `${label}: registered processor constructed`);
  assert.equal(
    state.node.processor.constructor,
    facade.registeredProcessor,
    `${label}: node uses the registered processor`,
  );
  const snapshot = state.node.constructionSnapshot;
  assert.equal(snapshot.name, REAL_PROCESSOR_NAME, `${label}: processor name`);
  assert.equal(snapshot.numberOfInputs, 0, `${label}: input count`);
  assert.equal(snapshot.numberOfOutputs, 1, `${label}: output count`);
  assert.deepEqual(snapshot.outputChannelCount, [2], `${label}: output channels`);
  assert.equal(
    snapshot.processorOptions.module instanceof WebAssembly.Module,
    true,
    `${label}: native WebAssembly.Module construction identity`,
  );
  assert.deepEqual([...snapshot.processorOptions.document], [...document], `${label}: document snapshot`);
  assert.deepEqual(snapshot.processorOptions.options, {
    ...options,
    spectrum: null,
    spectrumCollection: null,
  }, `${label}: boot option snapshot`);
  const processor = state.node.processor;
  assert.equal(
    processor.instance instanceof WebAssembly.Instance,
    true,
    `${label}: native WebAssembly.Instance construction identity`,
  );
  assert.equal(
    processor.exports,
    processor.instance.exports,
    `${label}: processor exports preserve native instance identity`,
  );
  const ready = state.node.responses.find((message) => message?.tag === "miso.ready.v1");
  assert.notEqual(ready, undefined, `${label}: actual ready reply`);
  assert.equal(ready.requestId, 0, `${label}: boot correlation`);
  assert.equal(ready.result, 0, `${label}: boot result`);
  assert.equal(ready.backend, "simd128", `${label}: boot backend`);
  assert(Number.isSafeInteger(ready.memoryBytes) && ready.memoryBytes > 0, `${label}: boot memory`);
  assert.deepEqual(facade.fetches, [artifact.wasmUrl, artifact.abiUrl], `${label}: verified fetches`);

  const status = await realDeadline(`${label} status`, () => host.status());
  assert.deepEqual(Object.keys(status).sort(), [
    "backend", "lastResult", "memoryBytes", "nextAbsoluteSample", "quantumFrames", "renderedQuanta",
    "requestId", "result", "sampleRateHz", "state", "tag",
  ], `${label}: status shape`);
  assert.equal(status.tag, "miso.status.v1", `${label}: status tag`);
  assert.equal(status.requestId, 1, `${label}: status correlation`);
  assert.equal(status.result, 0, `${label}: status result`);
  assert.equal(status.lastResult, 0, `${label}: status lastResult`);
  assert.equal(status.state, 2, `${label}: status state`);
  assert.equal(status.backend, 1, `${label}: status backend`);
  assert.equal(status.sampleRateHz, 48000, `${label}: status sample rate`);
  assert.equal(status.quantumFrames, 128, `${label}: status quantum`);
  assert.equal(status.nextAbsoluteSample, 0n, `${label}: status next sample`);
  assert.equal(status.renderedQuanta, 0n, `${label}: status rendered quanta`);
  assert.equal(status.memoryBytes, host.memoryBytes, `${label}: status memory`);
  const statusRequests = state.node.requests.filter((message) => message?.tag === "miso.status.v1");
  assert.deepEqual(statusRequests.map((message) => message.requestId), [1], `${label}: status request`);
  assert(
    state.node.responses.some((message) => message?.tag === "miso.status.v1" && message.requestId === 1),
    `${label}: correlated status reply`,
  );

  const native = Object.freeze({
    instance: processor.instance,
    exports: processor.exports,
    handle: processor.handle,
  });
  state.native = native;
  assert(Number.isSafeInteger(native.handle) && native.handle > 0, `${label}: native handle is live`);
  const beforePointer = native.exports.miso_engine_web_v1_status_ptr(native.handle);
  assert(Number.isInteger(beforePointer) && beforePointer > 0, `${label}: status pointer before disposal`);

  const hostPostMessageAttemptsBeforeDispose = state.node.hostPostMessageAttempts;
  await realDeadline(`${label} dispose`, () => host.dispose());
  state.hostDisposeAcknowledged = true;
  const disposeRequests = state.node.requests.filter((message) => message?.tag === "miso.dispose.v1");
  assert.deepEqual(disposeRequests.map((message) => message.requestId), [2], `${label}: dispose request`);
  const disposeReplies = state.node.responses.filter(
    (message) => message?.tag === "miso.ack.v1" && message.requestId === 2,
  );
  assert.equal(disposeReplies.length, 1, `${label}: one correlated disposal acknowledgement`);
  assert.equal(disposeReplies[0].result, 0, `${label}: disposal acknowledgement result`);
  assert.equal(
    native.exports.miso_engine_web_v1_status_ptr(native.handle),
    0,
    NATIVE_RELEASE_ASSERTION,
  );
  assert.equal(processor.handle, 0, `${label}: processor clears its disposed handle`);
  assert.equal(native.instance.exports, native.exports, `${label}: saved instance remains native`);
  assert.equal(
    state.node.hostPostMessageAttempts,
    hostPostMessageAttemptsBeforeDispose + 1,
    `${label}: first disposal sends one host postMessage attempt`,
  );

  const hostPostMessageAttemptsAfterDispose = state.node.hostPostMessageAttempts;
  await realDeadline(`${label} repeated dispose`, () => host.dispose());
  assert.equal(
    state.node.hostPostMessageAttempts,
    hostPostMessageAttemptsAfterDispose,
    `${label}: repeated disposal sends no additional host postMessage attempt`,
  );
  assert.equal(state.node.closedHostPort, true, `${label}: host port closed`);
  assert.equal(state.node.closedProcessorPort, true, `${label}: processor port closed`);
  assert.equal(state.node.disconnectCount, 1, `${label}: node disconnected once`);
}

function isNativeReleaseAssertion(error) {
  return error?.code === "ERR_ASSERTION"
    && typeof error.message === "string"
    && error.message.startsWith(NATIVE_RELEASE_ASSERTION);
}

function cleanupSavedNativeHandle(state, label, requireLive) {
  if (state.native === null) {
    if (requireLive) throw new Error(`${label}: no independently saved native handle`);
    return;
  }
  const { exports, handle } = state.native;
  const pointerBeforeCleanup = exports.miso_engine_web_v1_status_ptr(handle);
  if (requireLive) {
    assert(Number.isInteger(pointerBeforeCleanup) && pointerBeforeCleanup > 0,
      `${label}: saved handle was not live before fallback cleanup`);
  }
  if (pointerBeforeCleanup === 0) return;
  const result = exports.miso_engine_web_v1_dispose(handle);
  assert.equal(result, 0, `${label}: retained native disposal result`);
  assert.equal(
    exports.miso_engine_web_v1_status_ptr(handle),
    0,
    `${label}: retained native status pointer after fallback cleanup`,
  );
}

async function runPositiveRealLifecycle(artifact) {
  await withRealWebAudioFacade(artifact, artifact.workletUrl, "ordinary lifecycle", async (facade) => {
    const state = realLifecycleState();
    let operationError = null;
    try {
      await ordinaryRealLifecycle(artifact, artifact.workletUrl, facade, "ordinary", state);
    } catch (error) {
      operationError = error;
    }
    let cleanupError = null;
    try {
      await realDeadline("ordinary native failure cleanup", () => (
        cleanupSavedNativeHandle(state, "ordinary native failure cleanup", false)
      ));
    } catch (error) {
      cleanupError = error;
    }
    if (operationError !== null && cleanupError !== null) {
      throw new AggregateError([operationError, cleanupError], "ordinary lifecycle and cleanup failed");
    }
    if (operationError !== null) throw operationError;
    if (cleanupError !== null) throw cleanupError;
  });
  console.log("real-Wasm ordinary lifecycle: boot/status/dispose/repeated-dispose passed; cleanup passed");
}

async function runCorruptedWasmRedControl(artifact) {
  const privateDirectory = await realDeadline(
    "corrupted Wasm private copy",
    () => mkdtemp(join(tmpdir(), "miso-837-corrupt-")),
  );
  try {
    await realDeadline("corrupted Wasm private copy", async () => {
      await Promise.all(REAL_ARTIFACT_NAMES.map((name) => copyFile(
        realArtifactPath(artifact.directory, name),
        realArtifactPath(privateDirectory, name),
      )));
      const corrupted = Uint8Array.from(await readFile(
        realArtifactPath(privateDirectory, REAL_WASM_FILE),
      ));
      corrupted[corrupted.length - 1] ^= 1;
      await writeFile(realArtifactPath(privateDirectory, REAL_WASM_FILE), corrupted);
    });
    let refusal = null;
    try {
      await realDeadline(
        "corrupted Wasm identity preflight",
        () => preflightRealArtifact(privateDirectory),
      );
    } catch (error) {
      refusal = error;
    }
    assert.notEqual(refusal, null, "corrupted Wasm red control must refuse");
    assert.match(refusal.message, /Wasm SHA-256 mismatch/);
    console.log("red control: corrupted private Wasm refused before module/node construction");
  } finally {
    await realDeadline(
      "corrupted Wasm private-copy cleanup",
      () => rm(privateDirectory, { recursive: true, force: true }),
    );
  }
}

async function runDisposalMutantRedControl(artifact) {
  const statement = "const result = this.exports.miso_engine_web_v1_dispose(this.handle);";
  const occurrenceCount = artifact.workletSource.split(statement).length - 1;
  if (occurrenceCount !== 1) {
    throw new Error(
      `disposal mutant refused: expected one ordinary disposal statement, found ${occurrenceCount}`,
    );
  }
  const statementIndex = artifact.workletSource.indexOf(statement);
  const ordinaryBranchIndex = artifact.workletSource.indexOf(
    'if (message.tag === "miso.dispose.v1" && exactFields(message, ["tag", "requestId"]))',
  );
  if (ordinaryBranchIndex < 0 || statementIndex < ordinaryBranchIndex
      || statementIndex > ordinaryBranchIndex + 500) {
    throw new Error("disposal mutant refused: native statement is not in the ordinary branch");
  }
  const replacement = "const result = RESULT_OK;";
  const mutantSource = artifact.workletSource.replace(statement, replacement);
  if (mutantSource === artifact.workletSource
      || mutantSource.split(replacement).length - 1 < 1) {
    throw new Error("disposal mutant refused: source replacement was not unique");
  }
  const mutantWorkletUrl = `data:text/javascript;base64,${Buffer.from(mutantSource, "utf8").toString("base64")}`;
  await withRealWebAudioFacade(artifact, mutantWorkletUrl, "disposal mutant", async (facade) => {
    const state = realLifecycleState();
    let lifecycleError = null;
    try {
      await ordinaryRealLifecycle(artifact, mutantWorkletUrl, facade, "disposal mutant", state);
    } catch (error) {
      lifecycleError = error;
    }
    const expectedFailure = isNativeReleaseAssertion(lifecycleError);
    let cleanupError = null;
    try {
      await realDeadline("disposal mutant fallback native cleanup", () => (
        cleanupSavedNativeHandle(state, "disposal mutant fallback native cleanup", expectedFailure)
      ));
    } catch (error) {
      cleanupError = error;
    }
    if (!expectedFailure) {
      const primary = lifecycleError ?? new Error("disposal mutant did not fail the named release assertion");
      if (cleanupError !== null) {
        throw new AggregateError([primary, cleanupError], "disposal mutant red control and cleanup failed");
      }
      throw primary;
    }
    assert.equal(state.hostDisposeAcknowledged, true, "disposal mutant failure followed an acknowledged disposal");
    assert.equal(state.node.processor.handle, 0, "disposal mutant processor acknowledged and cleared its handle");
    assert.equal(state.node.closedHostPort, true, "disposal mutant host port closed");
    assert.equal(state.node.closedProcessorPort, true, "disposal mutant processor port closed");
    if (cleanupError !== null) throw cleanupError;
    console.log(
      "red control: ordinary disposal mutant failed the named saved-handle release assertion; "
      + "independent native fallback disposal/status-zero cleanup passed",
    );
  });
}

async function testRealWasmReceiver(artifactDirectory) {
  const artifact = await realDeadline(
    "real-Wasm artifact preflight",
    () => preflightRealArtifact(artifactDirectory),
  );
  console.log(`real-Wasm artifact preflight: exact seven files, SHA-256 ${artifact.wasmSha256}`);
  await runPositiveRealLifecycle(artifact);
  await runCorruptedWasmRedControl(artifact);
  await runDisposalMutantRedControl(artifact);
  console.log("real-Wasm receiver lifecycle qualification passed");
}

async function testMainRealm() {
  const original = {
    fetch: globalThis.fetch,
    AudioWorkletNode: globalThis.AudioWorkletNode,
    validate: WebAssembly.validate,
    compile: WebAssembly.compile,
  };
  const events = [];
  const fetches = [];
  let holdSource = false;
  let holdAll = false;
  const heldAll = [];
  let failSource = false;
  let held = null;
  let readyMutation = null;
  let statusMutation = null;
  let planeMutation = null;
  let commandResult = 0;
  let mixedSuccess = false;
  let commandMutation = null;
  let compileGate = null;
  let addModuleGate = null;
  let readyGate = null;
  let abiGate = null;

  const snapshotGate = () => {
    let release;
    let markStarted;
    const promise = new Promise((resolve) => { release = resolve; });
    const started = new Promise((resolve) => { markStarted = resolve; });
    return {
      promise,
      started,
      start: markStarted,
      release,
    };
  };

  const snapshotContext = ({
    state = "suspended",
    sampleRate = 48000,
    quantum = 64,
    omitQuantum = false,
  } = {}) => {
    const value = {
      state,
      sampleRate,
      audioWorklet: {
        addModule: async (url) => {
          events.push(["addModule", url]);
          if (addModuleGate !== null) {
            const gate = addModuleGate;
            addModuleGate = null;
            gate.start();
            await gate.promise;
          }
        },
      },
    };
    if (!omitQuantum) value.renderQuantumSize = quantum;
    return value;
  };

  class FakePort {
    onmessage = null;
    onmessageerror = null;
    closeCount = 0;

    constructor(sampleRateHz, quantumFrames) {
      this.sampleRateHz = sampleRateHz;
      this.quantumFrames = quantumFrames;
    }

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
            result: failSource ? 1 : (mixedSuccess ? 0 : 6),
            planes: received.planes,
          };
          responseTransfer = [...new Set(received.planes.map((plane) => plane.buffer))];
        } else if (received.tag === "miso.status.v1") {
          response = {
            tag: "miso.status.v1", requestId: received.requestId, result: 0, state: 2,
            lastResult: 0, backend: 1, sampleRateHz: this.sampleRateHz,
            quantumFrames: this.quantumFrames,
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
    static count = 0;

    constructor(context, _name, options) {
      FakeNode.count += 1;
      const sampleRateHz = context.sampleRate;
      const quantumFrames = context.renderQuantumSize ?? 128;
      this.port = new FakePort(sampleRateHz, quantumFrames);
      this.onprocessorerror = null;
      this.options = options;
      this.constructionContext = context;
      this.constructionModule = options.processorOptions.module;
      const constructorDocument = options.processorOptions.document;
      this.constructionDocumentShape = {
        byteOffset: constructorDocument.byteOffset,
        byteLength: constructorDocument.byteLength,
        buffer: constructorDocument.buffer,
      };
      // FakeNode retains the real constructor input by reference above. This separate, immediate
      // byte copy is the content oracle: later caller mutation cannot rewrite the evidence being
      // checked. Shape and backing identity are recorded from the actual constructor argument
      // above, not inferred from this copy.
      this.constructionSnapshot = {
        name: _name,
        numberOfInputs: options.numberOfInputs,
        numberOfOutputs: options.numberOfOutputs,
        outputChannelCount: [...options.outputChannelCount],
        processorOptions: {
          module: options.processorOptions.module,
          document: new Uint8Array(options.processorOptions.document),
          options: structuredClone(options.processorOptions.options),
        },
      };
      this.disposeMessages = 0;
      this.disconnectCount = 0;
      FakeNode.latest = this;
      queueMicrotask(async () => {
        if (readyGate !== null) {
          const gate = readyGate;
          readyGate = null;
          gate.start();
          await gate.promise;
        }
        let data = {
          tag: "miso.ready.v1", requestId: 0, result: 0,
          backend: "simd128",
          resources: resourceReport(1, quantumFrames),
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
  const unsupportedPreparationModule = await original.compile(unsupportedPreparationModuleBytes);
  WebAssembly.compile = async (bytes) => {
    const url = new TextDecoder().decode(bytes);
    events.push(["compile", url]);
    if (compileGate !== null) {
      const gate = compileGate;
      compileGate = null;
      gate.start();
      await gate.promise;
    }
    return unsupportedPreparationModule;
  };
  globalThis.fetch = async (url) => {
    fetches.push(String(url));
    if (abiGate !== null && String(url).includes("miso-engine-v1-abi-layout.json")) {
      const gate = abiGate;
      abiGate = null;
      gate.start();
      await gate.promise;
    }
    return {
      ok: true,
      arrayBuffer: async () => new TextEncoder().encode(String(url)).buffer,
      json: async () => preparedAbiLayout,
    };
  };
  const context = snapshotContext();
  try {
    const { createMisoAudioWorkletHost } = await import(`${hostUrl.href}?main-test`);

    const snapshotFactory = (factoryInput = {}) => {
      const {
        context: factoryContext = snapshotContext(),
        document = new Uint8Array([0x7b, 0x22, 0x73, 0x22, 0x7d]),
        options: factoryOptions = { ...limits },
        simd128ModuleUrl = "simd.wasm",
        workletModuleUrl = "processor.js",
        includePreparedModule = true,
        preparedModule,
      } = factoryInput;
      const factory = {
        context: factoryContext,
        document,
        options: factoryOptions,
        simd128ModuleUrl,
        workletModuleUrl,
      };
      if (includePreparedModule) {
        factory.preparedModule = Object.hasOwn(factoryInput, "preparedModule")
          ? preparedModule
          : unsupportedPreparationModule;
      }
      return factory;
    };

    const expectedProcessorOptions = (options) => {
      const copy = structuredClone(options);
      copy.spectrum = copy.spectrum ?? null;
      copy.spectrumCollection = copy.spectrumCollection ?? null;
      return copy;
    };

    const snapshotLocalRefusal = async (label, factory, result = 1) => {
      const eventsBefore = events.length;
      const fetchesBefore = fetches.length;
      const nodesBefore = FakeNode.count;
      const outcome = await createMisoAudioWorkletHost(factory).then(
        (hostValue) => ({ hostValue }),
        (error) => ({ error }),
      );
      if (outcome.hostValue !== undefined) {
        await outcome.hostValue.dispose().catch(() => undefined);
        assert.fail(label);
      }
      const error = outcome.error;
      assert(error !== undefined, label);
      assert.deepEqual(Object.keys(error).sort(), ["requestId", "result", "tag"], label);
      assert.equal(error.tag, "miso.error.v1", label);
      assert.equal(error.requestId, 0, label);
      assert.equal(error.result, result, label);
      assert.equal(events.length, eventsBefore, `${label}: no loading or addModule`);
      assert.equal(fetches.length, fetchesBefore, `${label}: no fetch`);
      assert.equal(FakeNode.count, nodesBefore, `${label}: no node construction`);
      return error;
    };

    const snapshotPreloadingFailure = async (label, factory, check) => {
      const eventsBefore = events.length;
      const fetchesBefore = fetches.length;
      const nodesBefore = FakeNode.count;
      await assert.rejects(async () => {
        const hostValue = await createMisoAudioWorkletHost(factory);
        await hostValue.dispose();
      }, check, label);
      assert.equal(events.length, eventsBefore, `${label}: no loading or addModule`);
      assert.equal(fetches.length, fetchesBefore, `${label}: no fetch`);
      assert.equal(FakeNode.count, nodesBefore, `${label}: no node construction`);
    };

    const snapshotLocalHostRefusal = async (label, promise, result = 1) => {
      const outcome = await promise.then(
        (hostValue) => ({ hostValue }),
        (error) => ({ error }),
      );
      if (outcome.hostValue !== undefined) {
        await outcome.hostValue.dispose().catch(() => undefined);
        assert.fail(label);
      }
      const error = outcome.error;
      assert(error !== undefined, label);
      assert.equal(error.tag, "miso.error.v1", label);
      assert.equal(error.requestId, 0, label);
      assert.equal(error.result, result, label);
      return error;
    };

    const snapshotCommand = () => ({
      kind: 1, rack: 255, channel: 255, trackIndex: 0, effectIndex: 0, parameterId: 0,
      smoothingSamples: 1, values: [0, 0, 0, 0],
    });

    const runSnapshotDocumentControl = async () => {
      // Red control 1: baseline 299c8285 reads the caller document after this pause. Keep this
      // case first so a later snapshot assertion cannot mask the required named failure.
      const gate = snapshotGate();
      compileGate = gate;
      const backing = new ArrayBuffer(16);
      const document = new Uint8Array(backing, 3, 5);
      document.set([11, 22, 33, 44, 55]);
      const originalVisible = [...document];
      const originalWorkletUrl = "captured-document-processor.js";
      const factory = snapshotFactory({
        document,
        includePreparedModule: false,
        workletModuleUrl: originalWorkletUrl,
      });
      const originalAudioWorklet = factory.context.audioWorklet;
      let audioWorkletReads = 0;
      let replacementAddModuleCalls = 0;
      const replacementAudioWorklet = {
        async addModule(url) { replacementAddModuleCalls += 1; await originalAudioWorklet.addModule(url); },
      };
      Object.defineProperty(factory.context, "audioWorklet", {
        configurable: true,
        get() { audioWorkletReads += 1; return originalAudioWorklet; },
      });
      const eventsBefore = events.length;
      const nodesBefore = FakeNode.count;
      let hostValue;
      try {
        const boot = createMisoAudioWorkletHost(factory);
        await gate.started;
        document[1] = 99;
        factory.document = new Uint8Array([201, 202, 203]);
        factory.workletModuleUrl = "mutated-during-compile.js";
        Object.defineProperty(factory.context, "audioWorklet", {
          get() { audioWorkletReads += 1; return replacementAudioWorklet; },
        });
        gate.release();
        hostValue = await boot;
        assert.equal(FakeNode.count, nodesBefore + 1, "snapshot.document.constructed");
        const node = FakeNode.latest;
        const captured = node.constructionSnapshot.processorOptions.document;
        assert.deepEqual([...captured], originalVisible, "snapshot.document.before-construction");
        assert.equal(
          node.constructionDocumentShape.byteOffset,
          0,
          "snapshot.document.actual-constructor-visible-range-offset",
        );
        assert.equal(
          node.constructionDocumentShape.byteLength,
          originalVisible.length,
          "snapshot.document.actual-constructor-visible-range-length",
        );
        assert.notEqual(
          node.constructionDocumentShape.buffer,
          backing,
          "snapshot.document.actual-constructor-private-storage",
        );
        assert.equal(backing.byteLength, 16, "snapshot.document.caller-buffer-attached");
        assert.equal(
          Object.isFrozen(node.options.processorOptions.document),
          false,
          "snapshot.document.nonempty-not-frozen",
        );
        const addModuleEvents = events.slice(eventsBefore)
          .filter(([kind]) => kind === "addModule");
        assert.equal(addModuleEvents.length, 1, "snapshot.document.add-module-once");
        assert.equal(
          addModuleEvents[0][1],
          originalWorkletUrl,
          "snapshot.document.worklet-url-captured-before-compile",
        );
        assert.equal(audioWorkletReads, 1, "snapshot.document.audio-worklet-captured-before-compile");
        assert.equal(replacementAddModuleCalls, 0, "snapshot.document.replacement-audio-worklet-unused");
      } finally {
        gate.release();
        if (compileGate === gate) compileGate = null;
        if (hostValue !== undefined) await hostValue.dispose().catch(() => undefined);
      }
    };

    const runSnapshotNestedCase = async (label, options, mutate) => {
      const gate = snapshotGate();
      addModuleGate = gate;
      const factory = snapshotFactory({ options });
      const expected = expectedProcessorOptions(options);
      const nodesBefore = FakeNode.count;
      let hostValue;
      try {
        const boot = createMisoAudioWorkletHost(factory);
        await gate.started;
        mutate(factory);
        gate.release();
        hostValue = await boot;
        assert.equal(FakeNode.count, nodesBefore + 1, `snapshot.${label}.constructed`);
        assert.deepEqual(
          FakeNode.latest.constructionSnapshot.processorOptions.options,
          expected,
          `snapshot.${label}.before-construction`,
        );
      } finally {
        gate.release();
        if (addModuleGate === gate) addModuleGate = null;
        if (hostValue !== undefined) await hostValue.dispose().catch(() => undefined);
      }
    };

    const runSnapshotNestedControl = async () => {
      // Red control 2: after the corrected factory exists, replacing this collection and mutating
      // its entries must fail a shallow-copy mutant at the uniquely named assertion below.
      const firstEntry = { target: "output", targetId: "first", channels: "both" };
      const thirdEntry = { target: "trackPostMatrix", targetId: "third", channels: "left" };
      const collection = {
        entries: [firstEntry, , thirdEntry],
        maximumCaptureBytes: 4096,
      };
      await runSnapshotNestedCase(
        "nested",
        {
          ...limits,
          spectrum: null,
          spectrumCollection: collection,
        },
        (factory) => {
          firstEntry.targetId = "mutated-first";
          thirdEntry.channels = "right";
          factory.options.spectrumCollection = {
            entries: [{ target: "output", targetId: "replacement", channels: "both" }],
            maximumCaptureBytes: 8192,
          };
        },
      );
      assert.equal(
        Object.hasOwn(FakeNode.latest.constructionSnapshot.processorOptions.options
          .spectrumCollection.entries, 1),
        false,
        "snapshot.nested.sparse-hole",
      );

      let reads = 0;
      const firstEntries = [{ target: "output", targetId: "getter-first", channels: "both" }];
      const laterEntries = [{ target: "output", targetId: "getter-later", channels: "right" }];
      const getterCollection = { maximumCaptureBytes: 4096 };
      Object.defineProperty(getterCollection, "entries", {
        configurable: true,
        enumerable: true,
        get() {
          reads += 1;
          return reads === 1 ? firstEntries : laterEntries;
        },
      });
      const getterOptions = {
        ...limits,
        spectrum: null,
        spectrumCollection: getterCollection,
      };
      const getterGate = snapshotGate();
      addModuleGate = getterGate;
      const getterFactory = snapshotFactory({ options: getterOptions });
      let getterHost;
      try {
        const boot = createMisoAudioWorkletHost(getterFactory);
        await getterGate.started;
        assert.equal(reads, 1, "snapshot.nested-getter.single-synchronous-read");
        Object.defineProperty(getterCollection, "entries", {
          configurable: true,
          enumerable: true,
          get() {
            reads += 1;
            return laterEntries;
          },
        });
        firstEntries[0].targetId = "caller-mutated-after-capture";
        getterGate.release();
        getterHost = await boot;
        const actualCollection = FakeNode.latest.options.processorOptions.options
          .spectrumCollection;
        assert.equal(reads, 1, "snapshot.nested-getter.no-later-read");
        assert.equal(
          actualCollection.entries[0].targetId,
          "getter-first",
          "snapshot.nested-getter.first-value-reaches-construction",
        );
        assert.equal(
          actualCollection.entries[0].channels,
          "both",
          "snapshot.nested-getter.first-entry-reaches-construction",
        );
      } finally {
        getterGate.release();
        if (addModuleGate === getterGate) addModuleGate = null;
        if (getterHost !== undefined) await getterHost.dispose().catch(() => undefined);
      }

      const undefinedMapEntries = [
        { target: "output", targetId: "map-undefined", channels: "both" },
      ];
      Object.defineProperty(undefinedMapEntries, "map", {
        configurable: true, enumerable: false, value: undefined, writable: true,
      });
      await runSnapshotAccepted(
        "snapshot.nested.map-undefined-accepted",
        snapshotFactory({ options: {
          ...limits,
          spectrum: null,
          spectrumCollection: { entries: undefinedMapEntries, maximumCaptureBytes: 4096 },
        } }),
        (snapshot) => {
          assert.equal(
            snapshot.processorOptions.options.spectrumCollection.entries[0].targetId,
            "map-undefined",
            "snapshot.nested.map-undefined-reaches-construction",
          );
        },
      );

      let hostileMapCalls = 0;
      const hostileEntry = { target: "output", targetId: "hostile-captured", channels: "both" };
      const hostileEntries = [hostileEntry];
      Object.defineProperty(hostileEntries, "map", {
        configurable: true,
        enumerable: false,
        value(capture) {
          hostileMapCalls += 1;
          for (let index = 0; index < this.length; index += 1) {
            if (index in this) this[index] = capture(this[index], index);
          }
          return this;
        },
        writable: true,
      });
      const hostileGate = snapshotGate();
      addModuleGate = hostileGate;
      const hostileFactory = snapshotFactory({ options: {
        ...limits,
        spectrum: null,
        spectrumCollection: { entries: hostileEntries, maximumCaptureBytes: 4096 },
      } });
      let hostileHost;
      try {
        const boot = createMisoAudioWorkletHost(hostileFactory);
        await hostileGate.started;
        hostileEntry.targetId = "mutated-after-capture";
        hostileEntries[0] = { target: "output", targetId: "replacement", channels: "right" };
        hostileGate.release();
        hostileHost = await boot;
        const constructorEntries = FakeNode.latest.options.processorOptions.options
          .spectrumCollection.entries;
        assert.equal(hostileMapCalls, 0, "snapshot.nested.hostile-map-never-called");
        assert.notEqual(
          constructorEntries,
          hostileEntries,
          "snapshot.nested.hostile-map-private-array-identity",
        );
        assert.equal(
          constructorEntries[0].targetId,
          "hostile-captured",
          "snapshot.nested.hostile-map-captured-entry-value",
        );
        assert.equal(
          constructorEntries[0].channels,
          "both",
          "snapshot.nested.hostile-map-captured-entry-channel",
        );
      } finally {
        hostileGate.release();
        if (addModuleGate === hostileGate) addModuleGate = null;
        if (hostileHost !== undefined) await hostileHost.dispose().catch(() => undefined);
      }

      for (const shadow of ["false", "undefined"]) {
        const label = `snapshot.nested.every-${shadow}`;
        const reads = { every: 0, calls: 0, entry: 0, maximum: 0 };
        const entry = { targetId: "unread", channels: "both" };
        Object.defineProperty(entry, "target", {
          enumerable: true,
          get() { reads.entry += 1; return "output"; },
        });
        const entries = [entry];
        Object.defineProperty(entries, "every", {
          get() {
            reads.every += 1;
            return shadow === "undefined" ? undefined : function () {
              assert.equal(this, entries, `${label}.receiver`);
              reads.calls += 1;
              return false;
            };
          },
        });
        const spectrumCollection = { entries };
        Object.defineProperty(spectrumCollection, "maximumCaptureBytes", {
          enumerable: true,
          get() { reads.maximum += 1; throw new Error("late collection maximum read"); },
        });
        const factory = snapshotFactory({ options: { ...limits, spectrum: null, spectrumCollection } });
        if (shadow === "false") await snapshotLocalRefusal(label, factory);
        else await snapshotPreloadingFailure(label, factory, (error) => (
          error instanceof TypeError && error.tag === undefined && /every.*not a function/.test(error.message)
        ));
        assert.deepEqual(reads, {
          every: 1, calls: shadow === "false" ? 1 : 0, entry: 0, maximum: 0,
        }, `${label}.short-circuit`);
      }

      for (const throwingField of ["constructor", "species"]) {
        const label = `snapshot.nested.throwing-${throwingField}`;
        const reads = { constructor: 0, species: 0, every: 0, target: 0, targetId: 0, channels: 0 };
        const values = { target: "output", targetId: "entry-first", channels: "both" };
        const entry = {};
        for (const [field, value] of Object.entries(values)) {
          Object.defineProperty(entry, field, {
            enumerable: true,
            get() { reads[field] += 1; return reads[field] === 1 ? value : "reread"; },
          });
        }
        const entries = [entry, ,];
        const constructor = {
          get [Symbol.species]() { reads.species += 1; throw new Error("array species consulted"); },
        };
        Object.defineProperty(entries, "constructor", {
          get() {
            reads.constructor += 1;
            if (throwingField === "constructor") throw new Error("array constructor consulted");
            return constructor;
          },
        });
        Object.defineProperty(entries, "every", {
          value(callback) {
            reads.every += 1;
            assert.equal(this, entries, `${label}.every-receiver`);
            assert.equal(Array.prototype.every.call(this, callback), true, `${label}.valid-entries`);
            return this; // Truthy validation outcome must never become constructor storage.
          },
        });
        await runSnapshotAccepted(label, snapshotFactory({ options: {
          ...limits, spectrum: null, spectrumCollection: { entries, maximumCaptureBytes: 4096 },
        } }), () => {
          const captured = FakeNode.latest.options.processorOptions.options.spectrumCollection.entries;
          assert.notEqual(captured, entries, `${label}.private-array`);
          assert.equal(Object.getPrototypeOf(captured), Array.prototype, `${label}.ordinary-array`);
          assert.equal(captured.length, 2, `${label}.length`);
          assert.equal(Object.hasOwn(captured, 1), false, `${label}.hole`);
          assert.notEqual(captured[0], entry, `${label}.private-entry`);
          assert.deepEqual(captured[0], values, `${label}.captured-fields`);
          assert.deepEqual(reads, {
            constructor: 0, species: 0, every: 1, target: 1, targetId: 1, channels: 1,
          }, `${label}.single-reads`);
        });
      }
    };

    const runSnapshotSingleRead = async () => {
      let reads = 0;
      const options = { ...limits };
      Object.defineProperty(options, "sourceRingFrames", {
        configurable: true,
        enumerable: true,
        get() {
          reads += 1;
          return reads === 1 ? 256 : 512;
        },
      });
      const factory = snapshotFactory({ options });
      const hostValue = await createMisoAudioWorkletHost(factory);
      try {
        assert.equal(reads, 1, "snapshot.single-read");
        assert.equal(
          FakeNode.latest.constructionSnapshot.processorOptions.options.sourceRingFrames,
          256,
          "snapshot.single-read.constructor-value",
        );
      } finally {
        await hostValue.dispose();
      }

      let throwingSpectrumReads = 0;
      const throwingSpectrumOptions = {
        ...limits, sourceRingFrames: -1, spectrumCollection: null,
      };
      Object.defineProperty(throwingSpectrumOptions, "spectrum", {
        configurable: true,
        enumerable: true,
        get() {
          throwingSpectrumReads += 1;
          throw new Error("later spectrum getter must remain unread");
        },
      });
      await snapshotLocalRefusal(
        "snapshot.short-circuit.invalid-source-before-throwing-spectrum",
        snapshotFactory({ options: throwingSpectrumOptions }),
      );
      assert.equal(
        throwingSpectrumReads,
        0,
        "snapshot.short-circuit.throwing-spectrum-zero-reads",
      );

      let mutatingSpectrumReads = 0;
      const mutatingSpectrumOptions = {
        ...limits, sourceRingFrames: -1, spectrumCollection: null,
      };
      Object.defineProperty(mutatingSpectrumOptions, "spectrum", {
        configurable: true,
        enumerable: true,
        get() {
          mutatingSpectrumReads += 1;
          mutatingSpectrumOptions.sourceRingFrames = 256;
          return null;
        },
      });
      await snapshotLocalRefusal(
        "snapshot.short-circuit.invalid-source-before-mutating-spectrum",
        snapshotFactory({ options: mutatingSpectrumOptions }),
      );
      assert.equal(
        mutatingSpectrumReads,
        0,
        "snapshot.short-circuit.mutating-spectrum-zero-reads",
      );
      assert.equal(
        mutatingSpectrumOptions.sourceRingFrames,
        -1,
        "snapshot.short-circuit.invalid-source-not-mutated",
      );
    };

    const runSnapshotAccepted = async (label, factory, check = () => undefined) => {
      const hostValue = await createMisoAudioWorkletHost(factory);
      try {
        check(FakeNode.latest.constructionSnapshot, hostValue);
      } finally {
        await hostValue.dispose();
      }
      assert.ok(label, "snapshot accepted case has a label");
    };

    const runSnapshotFactoryOrderCases = async () => {
      // Adjacent baseline stages must refuse before even reading the later accessor.
      for (const [name, setup] of [
        ["running-context-spectrum-repair", (f) => {
          f.context.state = "running";
          return [f.options, "spectrum", () => { f.context.state = "suspended"; return null; }];
        }],
        ["invalid-module-before-options", (f) => {
          f.preparedModule = null; return [f, "options"];
        }],
        ["invalid-context-before-document", (f) => {
          f.context.state = "running"; return [f, "document"];
        }],
        ["invalid-quantum-before-rate", (f) => {
          f.context.renderQuantumSize = 0; return [f.context, "sampleRate"];
        }],
        ["invalid-rate-before-document", (f) => {
          f.context.sampleRate = 0; return [f, "document"];
        }],
        ["invalid-document-before-spectrum", (f) => {
          f.document = null; return [f.options, "spectrum"];
        }],
        ["invalid-boot-before-simd-url", (f) => {
          f.options.sourceRingFrames = -1; return [f, "simd128ModuleUrl"];
        }],
        ["invalid-simd-url-before-worklet-url", (f) => {
          f.simd128ModuleUrl = 0; return [f, "workletModuleUrl"];
        }],
      ]) {
        const label = `snapshot.factory-order.${name}`;
        const factory = snapshotFactory({ options: { ...limits, spectrum: null, spectrumCollection: null } });
        const [object, field, repair] = setup(factory);
        let reads = 0;
        Object.defineProperty(object, field, {
          enumerable: true,
          get() {
            reads += 1;
            if (repair !== undefined) return repair();
            throw new Error(`${label}: later getter`);
          },
        });
        await snapshotLocalRefusal(label, factory);
        assert.equal(reads, 0, `${label}.later-reads`);
      }

      // The opening quantum read precedes both preparedModule and the exact-shape decision.
      const repairFactory = snapshotFactory({ preparedModule: null });
      let repairedModule = null;
      let quantumReads = 0;
      let moduleReads = 0;
      Object.defineProperty(repairFactory.context, "renderQuantumSize", {
        get() { quantumReads += 1; repairedModule = unsupportedPreparationModule; return 64; },
      });
      Object.defineProperty(repairFactory, "preparedModule", {
        get() { moduleReads += 1; return repairedModule; },
      });
      const eventsBefore = events.length;
      const fetchesBefore = fetches.length;
      const nodesBefore = FakeNode.count;
      const repairBoot = createMisoAudioWorkletHost(repairFactory);
      let repairHost;
      try {
        assert.equal(quantumReads, 1, "snapshot.factory-order.repair.initial-quantum-read");
        assert.equal(moduleReads, 1, "snapshot.factory-order.repair.initial-module-read");
        repairHost = await repairBoot;
        assert.equal(moduleReads, 1, "snapshot.factory-order.repair.cached-module");
        // One initial read, one preconstruction recheck, and FakeNode's own context observation.
        assert.equal(quantumReads, 3, "snapshot.factory-order.repair.quantum-recheck");
        assert.equal(FakeNode.count, nodesBefore + 1, "snapshot.factory-order.repair.constructed");
        assert.equal(FakeNode.latest.constructionModule, unsupportedPreparationModule,
          "snapshot.factory-order.repair.module-identity");
        assert.deepEqual(events.slice(eventsBefore), [["addModule", "processor.js"]],
          "snapshot.factory-order.repair.no-compile");
        assert.equal(fetches.length, fetchesBefore + 1, "snapshot.factory-order.repair.only-abi-fetch");
        assert.match(fetches[fetchesBefore], /miso-engine-v1-abi-layout\.json$/);
      } finally {
        if (repairHost !== undefined) await repairHost.dispose();
      }

      for (const throws of [false, true]) {
        const label = `snapshot.factory-order.invalid-shape-quantum-${throws ? "throws" : "reads"}`;
        const factory = snapshotFactory();
        factory.extra = true;
        const sentinel = new Error(label);
        const reads = { quantum: 0, module: 0 };
        Object.defineProperty(factory.context, "renderQuantumSize", {
          get() { reads.quantum += 1; if (throws) throw sentinel; return 64; },
        });
        Object.defineProperty(factory, "preparedModule", {
          get() { reads.module += 1; return unsupportedPreparationModule; },
        });
        if (throws) await snapshotPreloadingFailure(label, factory, (error) => error === sentinel);
        else await snapshotLocalRefusal(label, factory);
        assert.deepEqual(reads, { quantum: 1, module: throws ? 0 : 1 }, `${label}.reads`);
      }

      const validate = WebAssembly.validate;
      try {
        for (const supported of [false, true]) {
          const label = `snapshot.factory-order.audio-worklet-simd-${supported}`;
          const factory = snapshotFactory({ includePreparedModule: false });
          const reads = { simd: 0, audioWorklet: 0 };
          WebAssembly.validate = () => { reads.simd += 1; return supported; };
          Object.defineProperty(factory.context, "audioWorklet", {
            get() { reads.audioWorklet += 1; throw new Error("audioWorklet getter sentinel"); },
          });
          if (supported) await snapshotLocalRefusal(label, factory, 255);
          else await snapshotPreloadingFailure(label, factory, (error) => {
            assert.deepEqual(error, {
              tag: "miso.unsupported.v1", requestId: 0, result: 7, capability: "simd128",
            }, label);
            return true;
          });
          assert.deepEqual(reads, { simd: 1, audioWorklet: supported ? 1 : 0 }, `${label}.reads`);
        }
      } finally {
        WebAssembly.validate = validate;
      }
    };

    const runSnapshotStorageCases = async () => {
      const sharedBacking = new SharedArrayBuffer(8);
      const sharedDocument = new Uint8Array(sharedBacking);
      sharedDocument.set([1, 2, 3, 4]);
      await snapshotLocalRefusal(
        "snapshot.storage.shared-before-loading",
        snapshotFactory({ document: sharedDocument }),
      );

      const detachedBacking = new ArrayBuffer(8);
      const detachedDocument = new Uint8Array(detachedBacking);
      structuredClone(detachedBacking, { transfer: [detachedBacking] });
      await snapshotLocalRefusal(
        "snapshot.storage.detached-before-loading",
        snapshotFactory({ document: detachedDocument }),
      );

      const gate = snapshotGate();
      addModuleGate = gate;
      const backing = new ArrayBuffer(12);
      const document = new Uint8Array(backing, 2, 6);
      document.set([31, 41, 51, 61, 71, 81]);
      const expected = [...document];
      const factory = snapshotFactory({ document });
      let hostValue;
      try {
        const boot = createMisoAudioWorkletHost(factory);
        await gate.started;
        structuredClone(backing, { transfer: [backing] });
        gate.release();
        hostValue = await boot;
        assert.equal(backing.byteLength, 0, "snapshot.storage.detached-after-capture.caller-detached");
        assert.deepEqual(
          [...FakeNode.latest.constructionSnapshot.processorOptions.document],
          expected,
          "snapshot.storage.detached-after-capture",
        );
      } finally {
        gate.release();
        if (addModuleGate === gate) addModuleGate = null;
        if (hostValue !== undefined) await hostValue.dispose().catch(() => undefined);
      }

      const emptyHost = await createMisoAudioWorkletHost(snapshotFactory({ document: new Uint8Array() }));
      try {
        assert.equal(
          FakeNode.latest.constructionSnapshot.processorOptions.document.byteLength,
          0,
          "snapshot.storage.attached-empty-admissible",
        );
      } finally {
        await emptyHost.dispose();
      }
    };

    const runSnapshotShapeAndDomainCases = async () => {
      const singleSpectrum = {
        target: "trackPostInputBuiltins",
        targetId: `${"é".repeat(63)}a`,
        channels: "both",
        maximumCaptureBytes: 1_048_576,
      };
      const collectionEntry = {
        target: "trackPostMatrix", targetId: "collection-entry", channels: "left",
      };
      const sparseCollection = {
        entries: [collectionEntry, , { target: "output", targetId: "tail", channels: "right" }],
        maximumCaptureBytes: Number.MAX_SAFE_INTEGER,
      };

      await runSnapshotAccepted(
        "snapshot.shape.legacy-six",
        snapshotFactory({ options: { ...limits } }),
        (snapshot) => {
          assert.equal(snapshot.processorOptions.options.spectrum, null,
            "snapshot.shape.legacy-six.spectrum-normalized");
          assert.equal(snapshot.processorOptions.options.spectrumCollection, null,
            "snapshot.shape.legacy-six.collection-normalized");
        },
      );
      await runSnapshotAccepted(
        "snapshot.shape.explicit-undefined-normalization",
        snapshotFactory({ options: { ...limits, spectrum: undefined, spectrumCollection: undefined } }),
        (snapshot) => {
          assert.equal(snapshot.processorOptions.options.spectrum, null,
            "snapshot.shape.undefined-spectrum-normalized");
          assert.equal(snapshot.processorOptions.options.spectrumCollection, null,
            "snapshot.shape.undefined-collection-normalized");
        },
      );
      await runSnapshotAccepted(
        "snapshot.shape.single-eight",
        snapshotFactory({
          options: { ...limits, spectrum: singleSpectrum, spectrumCollection: null },
        }),
        (snapshot) => {
          assert.deepEqual(snapshot.processorOptions.options.spectrum, singleSpectrum,
            "snapshot.shape.single-spectrum-values");
        },
      );
      await runSnapshotAccepted(
        "snapshot.shape.collection-eight",
        snapshotFactory({
          options: { ...limits, spectrum: null, spectrumCollection: sparseCollection },
        }),
        (snapshot) => {
          assert.equal(
            Object.hasOwn(snapshot.processorOptions.options.spectrumCollection.entries, 1),
            false,
            "snapshot.shape.collection-sparse-hole",
          );
          assert.equal(
            snapshot.processorOptions.options.spectrumCollection.maximumCaptureBytes,
            Number.MAX_SAFE_INTEGER,
            "snapshot.shape.collection-positive-safe-integer-domain",
          );
        },
      );
      await runSnapshotAccepted(
        "snapshot.domain.inclusive-boundaries",
        snapshotFactory({
          options: {
            sourceRingFrames: 0xffffffff,
            maximumMemoryBytes: 0xffffffffffffffffn,
            consoleCommandQueueRecords: 256n,
            consoleMeterBlocks: 0xffffffffn,
            consoleObservationTaps: 16n,
            consoleMasterTrackPlusOne: 0xffffffffn,
            spectrum: null,
            spectrumCollection: null,
          },
        }),
        (snapshot) => {
          assert.equal(snapshot.processorOptions.options.sourceRingFrames, 0xffffffff);
          assert.equal(snapshot.processorOptions.options.consoleCommandQueueRecords, 256n);
          assert.equal(snapshot.processorOptions.options.consoleObservationTaps, 16n);
        },
      );

      const nonEnumerablePrepared = snapshotFactory({ includePreparedModule: false });
      Object.defineProperty(nonEnumerablePrepared, "preparedModule", {
        configurable: true, enumerable: false, value: undefined, writable: true,
      });
      await runSnapshotAccepted(
        "snapshot.shape.non-enumerable-prepared-undefined",
        nonEnumerablePrepared,
      );

      const nonEnumerableCollectionEntries = { maximumCaptureBytes: 1 };
      Object.defineProperty(nonEnumerableCollectionEntries, "entries", {
        configurable: true,
        enumerable: false,
        value: [collectionEntry],
        writable: true,
      });
      await snapshotLocalRefusal(
        "snapshot.shape.non-enumerable-collection-entries",
        snapshotFactory({ options: {
          ...limits, spectrum: null, spectrumCollection: nonEnumerableCollectionEntries,
        } }),
      );

      const nonEnumerableSpectrumOptions = { ...limits };
      Object.defineProperty(nonEnumerableSpectrumOptions, "spectrum", {
        configurable: true,
        enumerable: false,
        value: singleSpectrum,
        writable: true,
      });
      await snapshotLocalRefusal(
        "snapshot.shape.six-enumerable-non-enumerable-spectrum",
        snapshotFactory({ options: nonEnumerableSpectrumOptions }),
      );

      const nonEnumerableCollectionOptions = { ...limits };
      const nonEnumerableCollection = {
        entries: [collectionEntry],
        maximumCaptureBytes: 4096,
      };
      Object.defineProperty(nonEnumerableCollectionOptions, "spectrumCollection", {
        configurable: true,
        enumerable: false,
        value: nonEnumerableCollection,
        writable: true,
      });
      await runSnapshotAccepted(
        "snapshot.shape.six-enumerable-non-enumerable-collection",
        snapshotFactory({ options: nonEnumerableCollectionOptions }),
      );
      const actualNonEnumerableCollection = FakeNode.latest.options.processorOptions.options
        .spectrumCollection;
      assert.ok(actualNonEnumerableCollection !== null, "snapshot.shape.non-enumerable-collection.actual");
      assert.deepEqual(
        actualNonEnumerableCollection.entries,
        [collectionEntry],
        "snapshot.shape.non-enumerable-collection.actual-values",
      );

      await snapshotLocalRefusal(
        "snapshot.shape.seven-field-spectrum",
        snapshotFactory({ options: { ...limits, spectrum: singleSpectrum } }),
      );
      await snapshotLocalRefusal(
        "snapshot.shape.seven-field-collection",
        snapshotFactory({ options: { ...limits, spectrumCollection: sparseCollection } }),
      );
      await snapshotLocalRefusal(
        "snapshot.shape.both-spectrum-forms",
        snapshotFactory({ options: {
          ...limits, spectrum: singleSpectrum, spectrumCollection: sparseCollection,
        } }),
      );
      const unknownFactory = snapshotFactory();
      unknownFactory.protectedOptions = {};
      await snapshotLocalRefusal("snapshot.shape.unknown-factory", unknownFactory);
      await snapshotLocalRefusal(
        "snapshot.shape.unknown-boot",
        snapshotFactory({ options: { ...limits, inventedProtectedOption: true } }),
      );
      await snapshotLocalRefusal(
        "snapshot.shape.unknown-spectrum",
        snapshotFactory({ options: {
          ...limits, spectrum: { ...singleSpectrum, invented: true }, spectrumCollection: null,
        } }),
      );
      await snapshotLocalRefusal(
        "snapshot.shape.unknown-collection",
        snapshotFactory({ options: {
          ...limits, spectrum: null, spectrumCollection: { ...sparseCollection, invented: true },
        } }),
      );
      await snapshotLocalRefusal(
        "snapshot.shape.unknown-entry",
        snapshotFactory({ options: {
          ...limits,
          spectrum: null,
          spectrumCollection: {
            entries: [{ ...collectionEntry, invented: true }], maximumCaptureBytes: 1,
          },
        } }),
      );
      await snapshotLocalRefusal(
        "snapshot.shape.enumerable-prepared-undefined",
        snapshotFactory({ includePreparedModule: true, preparedModule: undefined }),
      );
      await snapshotLocalRefusal(
        "snapshot.shape.enumerable-prepared-null",
        snapshotFactory({ includePreparedModule: true, preparedModule: null }),
      );

      for (const [label, options] of [
        ["source-negative", { ...limits, sourceRingFrames: -1 }],
        ["source-fractional", { ...limits, sourceRingFrames: 1.5 }],
        ["source-too-large", { ...limits, sourceRingFrames: 0x1_0000_0000 }],
        ["memory-number", { ...limits, maximumMemoryBytes: 1 }],
        ["command-too-large", { ...limits, consoleCommandQueueRecords: 257n }],
        ["meter-too-large", { ...limits, consoleMeterBlocks: 0x1_0000_0000n }],
        ["observation-too-large", { ...limits, consoleObservationTaps: 17n }],
        ["master-too-large", { ...limits, consoleMasterTrackPlusOne: 0x1_0000_0000n }],
        ["observation-without-queue", {
          ...limits, consoleCommandQueueRecords: 0n, consoleObservationTaps: 1n,
        }],
        ["master-without-observation", {
          ...limits, consoleObservationTaps: 0n, consoleMasterTrackPlusOne: 1n,
        }],
        ["single-capture-too-large", {
          ...limits,
          spectrum: { ...singleSpectrum, maximumCaptureBytes: 1_048_577 },
          spectrumCollection: null,
        }],
        ["single-target-invalid", {
          ...limits,
          spectrum: { ...singleSpectrum, target: "not-a-target" },
          spectrumCollection: null,
        }],
        ["single-channel-invalid", {
          ...limits,
          spectrum: { ...singleSpectrum, channels: "stereo" },
          spectrumCollection: null,
        }],
        ["single-id-empty", {
          ...limits,
          spectrum: { ...singleSpectrum, targetId: "" },
          spectrumCollection: null,
        }],
        ["single-id-too-long", {
          ...limits,
          spectrum: { ...singleSpectrum, targetId: "a".repeat(128) },
          spectrumCollection: null,
        }],
        ["single-capture-zero", {
          ...limits,
          spectrum: { ...singleSpectrum, maximumCaptureBytes: 0 },
          spectrumCollection: null,
        }],
        ["collection-entry-invalid", {
          ...limits,
          spectrum: null,
          spectrumCollection: {
            entries: [{ ...collectionEntry, channels: "stereo" }], maximumCaptureBytes: 1,
          },
        }],
        ["empty-collection", {
          ...limits,
          spectrum: null,
          spectrumCollection: { entries: [], maximumCaptureBytes: 1 },
        }],
      ]) {
        await snapshotLocalRefusal(`snapshot.domain.${label}`, snapshotFactory({ options }));
      }
    };

    const runSnapshotReferencesAndLimits = async () => {
      const moduleA = unsupportedPreparationModule;
      const moduleB = await original.compile(unsupportedPreparationModuleBytes);
      const capturedContext = snapshotContext();
      const replacementContext = snapshotContext({ sampleRate: 44100, quantum: 32 });
      const gate = snapshotGate();
      addModuleGate = gate;
      const factory = snapshotFactory({
        context: capturedContext,
        preparedModule: moduleA,
        simd128ModuleUrl: "captured-simd.wasm",
        workletModuleUrl: "captured-processor.js",
      });
      const compileCountBefore = events.filter((event) => event[0] === "compile").length;
      const fetchCountBefore = fetches.length;
      let hostValue;
      try {
        const boot = createMisoAudioWorkletHost(factory);
        await gate.started;
        factory.context = replacementContext;
        factory.preparedModule = moduleB;
        factory.simd128ModuleUrl = "mutated-simd.wasm";
        factory.workletModuleUrl = "mutated-processor.js";
        gate.release();
        hostValue = await boot;
        assert.equal(FakeNode.latest.constructionContext, capturedContext,
          "snapshot.references.context");
        assert.equal(FakeNode.latest.constructionModule, moduleA,
          "snapshot.references.supplied-module");
        assert.equal(
          events.filter((event) => event[0] === "compile").length,
          compileCountBefore,
          "snapshot.references.supplied-module-no-fetch-compile",
        );
        assert.equal(
          fetches.slice(fetchCountBefore).some((url) => url.includes("captured-simd.wasm")),
          false,
          "snapshot.references.supplied-module-no-wasm-fetch",
        );
        assert.equal(
          events.findLast((event) => event[0] === "addModule")[1],
          "captured-processor.js",
          "snapshot.references.worklet-url",
        );
      } finally {
        gate.release();
        if (addModuleGate === gate) addModuleGate = null;
        if (hostValue !== undefined) await hostValue.dispose().catch(() => undefined);
      }

      const readyGateValue = snapshotGate();
      readyGate = readyGateValue;
      const readyContext = snapshotContext();
      const readyFactory = snapshotFactory({ context: readyContext });
      let readyHost;
      try {
        const boot = createMisoAudioWorkletHost(readyFactory);
        await readyGateValue.started;
        readyContext.sampleRate = 44100;
        readyContext.renderQuantumSize = 32;
        readyFactory.context = snapshotContext({ sampleRate: 44100, quantum: 32, state: "running" });
        readyFactory.options = { ...limits, sourceRingFrames: 1 };
        readyGateValue.release();
        readyHost = await boot;
        assert.equal(readyHost.backend, "simd128", "snapshot.ready.uses-captured-resources");
        assert.equal(readyHost.resources.sampleRateHz, 48000,
          "snapshot.ready.resources-captured-sample-rate");
        assert.equal(readyHost.resources.quantumFrames, 64,
          "snapshot.ready.resources-captured-quantum");
        const readyStatus = await readyHost.status();
        assert.equal(readyStatus.sampleRateHz, 48000,
          "snapshot.ready.returned-host-captured-sample-rate");
        assert.equal(readyStatus.quantumFrames, 64,
          "snapshot.ready.returned-host-captured-quantum");
      } finally {
        readyGateValue.release();
        if (readyGate === readyGateValue) readyGate = null;
        if (readyHost !== undefined) await readyHost.dispose().catch(() => undefined);
      }

      const explicitOptions = {
        ...limits,
        sourceRingFrames: 256,
        consoleCommandQueueRecords: 2n,
        consoleMeterBlocks: 7n,
        consoleObservationTaps: 1n,
        consoleMasterTrackPlusOne: 1n,
      };
      const explicitGate = snapshotGate();
      abiGate = explicitGate;
      const explicitFactory = snapshotFactory({ options: explicitOptions });
      let explicitHost;
      try {
        const boot = createMisoAudioWorkletHost(explicitFactory);
        await explicitGate.started;
        explicitOptions.sourceRingFrames = 64;
        explicitOptions.consoleCommandQueueRecords = 1n;
        explicitOptions.consoleMeterBlocks = 9n;
        explicitGate.release();
        explicitHost = await boot;
        holdAll = true;
        const sourceEventsBefore = events.length;
        const sourceRequests = [];
        for (let index = 0; index < 4; index += 1) {
          const buffer = new ArrayBuffer(8);
          const request = explicitHost.submitSource({
            sourceId: "snapshot-explicit", generation: BigInt(index + 1), startFrame: 0n,
            sampleRateHz: 48000, planes: [new Float32Array(buffer)], frames: 2, endOfRegion: false,
          });
          assert.equal(buffer.byteLength, 0, "snapshot.limits.explicit-source-capacity");
          sourceRequests.push(request);
        }
        const overflowBuffer = new ArrayBuffer(8);
        await snapshotLocalHostRefusal(
          "snapshot.limits.explicit-source-overflow",
          explicitHost.submitSource({
            sourceId: "snapshot-explicit", generation: 5n, startFrame: 0n,
            sampleRateHz: 48000, planes: [new Float32Array(overflowBuffer)], frames: 2,
            endOfRegion: false,
          }),
          6,
        );
        assert.equal(events.length, sourceEventsBefore + 4,
          "snapshot.limits.explicit-source-overflow-no-post");
        assert.equal(overflowBuffer.byteLength, 8,
          "snapshot.limits.explicit-source-overflow-retains-storage");
        holdAll = false;
        for (const respond of heldAll.splice(0)) respond();
        const sourceAcks = await Promise.all(sourceRequests);
        const nextSourceBuffer = new ArrayBuffer(8);
        const nextSource = explicitHost.submitSource({
          sourceId: "snapshot-explicit", generation: 6n, startFrame: 0n,
          sampleRateHz: 48000, planes: [new Float32Array(nextSourceBuffer)], frames: 2,
          endOfRegion: false,
        });
        assert.equal(nextSourceBuffer.byteLength, 0);
        assert.equal((await nextSource).requestId, Math.max(...sourceAcks.map((ack) => ack.requestId)) + 1,
          "snapshot.limits.explicit-source-no-id-burn");

        holdAll = true;
        const commandEventsBefore = events.length;
        const commandRequests = [
          explicitHost.command({ commands: [snapshotCommand()] }),
          explicitHost.command({ commands: [snapshotCommand()] }),
        ];
        assert.equal(events.length, commandEventsBefore + 2,
          "snapshot.limits.explicit-command-capacity");
        await snapshotLocalHostRefusal(
          "snapshot.limits.explicit-command-overflow",
          explicitHost.command({ commands: [snapshotCommand()] }),
          6,
        );
        assert.equal(events.length, commandEventsBefore + 2,
          "snapshot.limits.explicit-command-overflow-no-post");
        holdAll = false;
        for (const respond of heldAll.splice(0)) respond();
        const commandAcks = await Promise.all(commandRequests);
        const commandAfter = await explicitHost.command({ commands: [snapshotCommand()] });
        assert.equal(commandAfter.requestId, Math.max(...commandAcks.map((ack) => ack.requestId)) + 1,
          "snapshot.limits.explicit-command-no-id-burn");

        const observation = await explicitHost.observe({
          subscriptions: [{
            trackIndex: 0, rack: 1, effectIndex: 0, tapId: 1, windowBlocks: 0, armed: true,
          }],
        });
        assert.equal(observation.bindings[0].windowBlocks, 7,
          "snapshot.limits.observe-default-window");
      } finally {
        holdAll = false;
        for (const respond of heldAll.splice(0)) respond();
        explicitGate.release();
        if (abiGate === explicitGate) abiGate = null;
        if (explicitHost !== undefined) await explicitHost.dispose().catch(() => undefined);
      }

      const zeroOptions = {
        ...limits,
        sourceRingFrames: 64,
        consoleCommandQueueRecords: 0n,
        consoleMeterBlocks: 0n,
        consoleObservationTaps: 0n,
        consoleMasterTrackPlusOne: 0n,
      };
      const zeroGate = snapshotGate();
      abiGate = zeroGate;
      const zeroFactory = snapshotFactory({ options: zeroOptions });
      let zeroHost;
      try {
        const boot = createMisoAudioWorkletHost(zeroFactory);
        await zeroGate.started;
        zeroOptions.consoleCommandQueueRecords = 4n;
        zeroOptions.consoleMeterBlocks = 9n;
        zeroGate.release();
        zeroHost = await boot;
        holdAll = true;
        const first = zeroHost.command({ commands: [snapshotCommand()] });
        const commandEventsBefore = events.length;
        await snapshotLocalHostRefusal(
          "snapshot.limits.zero-command-overflow",
          zeroHost.command({ commands: [snapshotCommand()] }),
          6,
        );
        assert.equal(events.length, commandEventsBefore,
          "snapshot.limits.zero-command-overflow-no-post");
        holdAll = false;
        for (const respond of heldAll.splice(0)) respond();
        const firstAck = await first;
        const next = await zeroHost.command({ commands: [snapshotCommand()] });
        assert.equal(next.requestId, firstAck.requestId + 1,
          "snapshot.limits.zero-command-no-id-burn");
      } finally {
        holdAll = false;
        for (const respond of heldAll.splice(0)) respond();
        zeroGate.release();
        if (abiGate === zeroGate) abiGate = null;
        if (zeroHost !== undefined) await zeroHost.dispose().catch(() => undefined);
      }

      const defaultContext = snapshotContext({ omitQuantum: true });
      const defaultFactory = snapshotFactory({
        context: defaultContext,
        options: {
          ...limits,
          sourceRingFrames: 0,
          consoleCommandQueueRecords: 0n,
          consoleMeterBlocks: 0n,
          consoleObservationTaps: 0n,
          consoleMasterTrackPlusOne: 0n,
        },
      });
      const defaultHost = await createMisoAudioWorkletHost(defaultFactory);
      try {
        const defaultStatus = await defaultHost.status();
        assert.equal(defaultStatus.sampleRateHz, 48000, "snapshot.limits.default-sample-rate");
        assert.equal(defaultStatus.quantumFrames, 128, "snapshot.limits.default-quantum");
      } finally {
        await defaultHost.dispose();
      }

      const defaultDepthContext = snapshotContext();
      const defaultDepthOptions = {
        ...limits,
        sourceRingFrames: 0,
        consoleCommandQueueRecords: 0n,
        consoleMeterBlocks: 0n,
        consoleObservationTaps: 0n,
        consoleMasterTrackPlusOne: 0n,
      };
      const defaultDepthGate = snapshotGate();
      abiGate = defaultDepthGate;
      const defaultDepthFactory = snapshotFactory({
        context: defaultDepthContext, options: defaultDepthOptions,
      });
      let defaultDepthHost;
      try {
        const boot = createMisoAudioWorkletHost(defaultDepthFactory);
        await defaultDepthGate.started;
        defaultDepthContext.sampleRate = 96000;
        defaultDepthContext.renderQuantumSize = 128;
        defaultDepthGate.release();
        defaultDepthHost = await boot;
        const status = await defaultDepthHost.status();
        assert.equal(status.sampleRateHz, 48000, "snapshot.limits.default-depth-status-rate");
        assert.equal(status.quantumFrames, 64, "snapshot.limits.default-depth-status-quantum");
        const expectedBlocks = Math.ceil(48000 / 10 / 64) + 2;
        holdAll = true;
        const requests = [];
        for (let index = 0; index < expectedBlocks; index += 1) {
          const buffer = new ArrayBuffer(8);
          const request = defaultDepthHost.submitSource({
            sourceId: "snapshot-default", generation: BigInt(index + 1), startFrame: 0n,
            sampleRateHz: 48000, planes: [new Float32Array(buffer)], frames: 2,
            endOfRegion: false,
          });
          assert.equal(buffer.byteLength, 0, "snapshot.limits.default-depth-capacity");
          requests.push(request);
        }
        const overflowBuffer = new ArrayBuffer(8);
        const eventsBefore = events.length;
        await snapshotLocalHostRefusal(
          "snapshot.limits.default-depth-overflow",
          defaultDepthHost.submitSource({
            sourceId: "snapshot-default", generation: BigInt(expectedBlocks + 1), startFrame: 0n,
            sampleRateHz: 48000, planes: [new Float32Array(overflowBuffer)], frames: 2,
            endOfRegion: false,
          }),
          6,
        );
        assert.equal(events.length, eventsBefore,
          "snapshot.limits.default-depth-overflow-no-post");
        assert.equal(overflowBuffer.byteLength, 8,
          "snapshot.limits.default-depth-overflow-retains-storage");
        holdAll = false;
        for (const respond of heldAll.splice(0)) respond();
        const acknowledgements = await Promise.all(requests);
        const nextBuffer = new ArrayBuffer(8);
        const next = defaultDepthHost.submitSource({
          sourceId: "snapshot-default", generation: BigInt(expectedBlocks + 2), startFrame: 0n,
          sampleRateHz: 48000, planes: [new Float32Array(nextBuffer)], frames: 2,
          endOfRegion: false,
        });
        assert.equal(nextBuffer.byteLength, 0);
        assert.equal((await next).requestId, Math.max(...acknowledgements.map((ack) => ack.requestId)) + 1,
          "snapshot.limits.default-depth-no-id-burn");
      } finally {
        holdAll = false;
        for (const respond of heldAll.splice(0)) respond();
        defaultDepthGate.release();
        if (abiGate === defaultDepthGate) abiGate = null;
        if (defaultDepthHost !== undefined) await defaultDepthHost.dispose().catch(() => undefined);
      }
    };

    const runSnapshotContextDriftCases = async () => {
      for (const [label, mutate] of [
        ["state", (value) => { value.state = "running"; }],
        ["sample-rate", (value) => { value.sampleRate = 44100; }],
        ["quantum", (value) => { value.renderQuantumSize = 32; }],
      ]) {
        const gate = snapshotGate();
        addModuleGate = gate;
        const value = snapshotContext();
        const beforeNodes = FakeNode.count;
        let boot;
        try {
          boot = createMisoAudioWorkletHost(snapshotFactory({ context: value }));
          await gate.started;
          mutate(value);
          gate.release();
          await snapshotLocalHostRefusal(`snapshot.context-drift.${label}`, boot);
          assert.equal(FakeNode.count, beforeNodes, `snapshot.context-drift.${label}.no-node`);
        } finally {
          gate.release();
          if (addModuleGate === gate) addModuleGate = null;
        }
      }
    };

    const runSnapshotMatrix = async () => {
      const ordinaryOptions = { ...limits };
      await runSnapshotNestedCase(
        "nested-ordinary",
        ordinaryOptions,
        (factory) => { factory.options.sourceRingFrames = 128; },
      );
      const singleOptions = {
        ...limits,
        spectrum: {
          target: "output", targetId: "nested-single", channels: "left", maximumCaptureBytes: 2048,
        },
        spectrumCollection: null,
      };
      await runSnapshotNestedCase(
        "nested-single",
        singleOptions,
        (factory) => {
          factory.options.spectrum.targetId = "mutated-single";
          factory.options.spectrum = {
            target: "output", targetId: "replacement-single", channels: "right",
            maximumCaptureBytes: 4096,
          };
        },
      );
      await runSnapshotStorageCases();
      await runSnapshotFactoryOrderCases();
      await runSnapshotShapeAndDomainCases();
      await runSnapshotReferencesAndLimits();
      await runSnapshotContextDriftCases();
    };

    if (snapshotTestMode === "document") {
      await runSnapshotDocumentControl();
      return;
    }
    if (snapshotTestMode === "nested") {
      await runSnapshotNestedControl();
      return;
    }
    if (snapshotTestMode === "single-read") {
      await runSnapshotSingleRead();
      return;
    }
    if (snapshotTestMode !== "existing") {
      await runSnapshotDocumentControl();
      await runSnapshotNestedControl();
      await runSnapshotSingleRead();
      await runSnapshotMatrix();
    }
    // The legacy receiver assertions intentionally inspect the complete event stream (including
    // the MAX_SAFE_INTEGER control), so keep the new boot matrix's hermetic traffic out of it.
    events.length = 0;
    fetches.length = 0;

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
    if (process.env.MISO_ENGINE_WEB_HOST_MAX_SAFE_TEST === "1") {
      const last = await host.status();
      assert.equal(last.requestId, Number.MAX_SAFE_INTEGER);
      await localErrorResult(host.status(), 1);
      await localErrorResult(host.status(), 1);
      assert.deepEqual(
        events.filter((event) => event[0] === "request").map((event) => event[2]),
        [Number.MAX_SAFE_INTEGER],
        "safe-integer exhaustion never posts or wraps",
      );
      await host.dispose().catch(() => undefined);
      return;
    }

    const storage = new ArrayBuffer(32);
    const left = new Float32Array(storage, 0, 2);
    const right = new Float32Array(storage, 16, 2);
    left.set([1, 2]);
    right.set([3, 4]);
    holdSource = true;
    const sourcePromise = host.submitSource({
      sourceId: "source", generation: 1n, startFrame: 0n,
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
      sourceId: "source", generation: 2n, sourceFrame: 10n,
    });
    assert.deepEqual(seek, { tag: "miso.ack.v1", requestId: 3, result: 0 });
    const status = await host.status();
    assert.equal(status.tag, "miso.status.v1");
    assert.equal(status.memoryBytes, host.memoryBytes);
    failSource = true;
    const failedStorage = new ArrayBuffer(16);
    const failedSource = host.submitSource({
      sourceId: "source", generation: 3n, startFrame: 2n,
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
      sourceId: "source", generation: 1n, startFrame: 0n,
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
    const chunk = (sourceId, startFrame) => {
      const buffer = new ArrayBuffer(8);
      return {
        request: pipelineHost.submitSource({
          sourceId,
          generation: 1n,
          startFrame: BigInt(startFrame),
          sampleRateHz: 48000,
          planes: [new Float32Array(buffer)],
          frames: 2,
          endOfRegion: false,
        }),
        buffer,
      };
    };
    const inFlight = [1, 2, 3, 4].map((startFrame) => chunk("source", startFrame));
    for (const [index, entry] of inFlight.entries()) {
      assert.equal(entry.buffer.byteLength, 0, `chunk ${index} was transferred`);
    }
    const beforeSourceOverflow = events.length;
    const overflow = chunk("source", 5);
    await localErrorResult(overflow.request, 6);
    assert.equal(events.length, beforeSourceOverflow, "source saturation posts no refusal");
    assert.equal(
      overflow.buffer.byteLength,
      8,
      "a locally refused chunk keeps its planes: nothing is transferred and the caller can retry",
    );
    // A different source has its own budget and is accepted while the first is saturated.
    const other = chunk("other-source", 6);
    assert.equal(other.buffer.byteLength, 0, "the bound is per source, not per host");
    // One unsettled seek per source: the ring carries a single command slot.
    const firstSeek = pipelineHost.seekSource({
      sourceId: "source", generation: 2n, sourceFrame: 0n,
    });
    const beforeSeekOverflow = events.length;
    await localErrorResult(pipelineHost.seekSource({
      sourceId: "source", generation: 3n, sourceFrame: 0n,
    }), 6);
    assert.equal(events.length, beforeSeekOverflow, "seek saturation posts no refusal");
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
      [1, 2, 3, 4, 5, 6],
      "acknowledgements arrive in request order",
    );
    // With the budget released the source accepts chunks again.
    const again = chunk("source", 9);
    assert.equal(again.buffer.byteLength, 0);
    assert.equal((await again.request).requestId, 7, "source saturation does not burn an ID");
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
    // Issue #393: one real host fixture owns every request ID across interleaved request classes.
    // Twenty-five rounds cover 250 successful calls, including both lease orderings around a
    // command and direct source/seek traffic. No caller-side ID is supplied anywhere in this loop.
    const mixedAcks = [];
    const mixedSendStart = events.length;
    const mixedSubscription = {
      trackIndex: 0, rack: 1, effectIndex: 0, tapId: 1, windowBlocks: 1, armed: true,
    };
    for (let round = 0; round < 25; round += 1) {
      mixedAcks.push((await consoleHost.command({ commands: [pan] })).requestId);
      mixedAcks.push((await consoleHost.observe({ subscriptions: [mixedSubscription] })).requestId);
      mixedAcks.push((await consoleHost.meters({ enabled: true, onFrame: null })).requestId);
      mixedAcks.push((await consoleHost.meters({ enabled: false, onFrame: null })).requestId);
      mixedAcks.push((await consoleHost.telemetry({ enabled: true, onFrame: null })).requestId);
      mixedAcks.push((await consoleHost.telemetry({ enabled: false, onFrame: null })).requestId);
      const mixedBuffer = new ArrayBuffer(16);
      const mixedLeft = new Float32Array(mixedBuffer, 0, 2);
      const mixedRight = new Float32Array(mixedBuffer, 8, 2);
      mixedLeft.set([round, round + 1]);
      mixedRight.set([round + 2, round + 3]);
      mixedSuccess = true;
      const mixedSourceAck = await consoleHost.submitSource({
        sourceId: "mixed-source", generation: BigInt(round + 1), startFrame: 0n,
        sampleRateHz: 48000, planes: [mixedLeft, mixedRight], frames: 2, endOfRegion: true,
      });
      assert.equal(mixedSourceAck.result, 0, "mixed source admission succeeds");
      assert.equal(mixedBuffer.byteLength, 0, "successful mixed source transfers its buffer");
      assert.equal(mixedSourceAck.planes.length, 2);
      assert.equal(mixedSourceAck.planes[0].byteOffset, 0);
      assert.equal(mixedSourceAck.planes[1].byteOffset, 8);
      assert.equal(mixedSourceAck.planes[0].buffer, mixedSourceAck.planes[1].buffer);
      assert.equal(mixedSourceAck.planes[0].buffer.byteLength, 16);
      const mixedSourceEvent = events.findLast((event) => event[1] === "miso.source.v1");
      assert.equal(mixedSourceEvent[2], mixedSourceAck.requestId);
      assert.equal(mixedSourceEvent[3], 1, "shared mixed source storage transfers once");
      mixedAcks.push(mixedSourceAck.requestId);
      mixedAcks.push((await consoleHost.seekSource({
        sourceId: "mixed-source", generation: BigInt(round + 1), sourceFrame: 0n,
      })).requestId);
      mixedAcks.push((await consoleHost.status()).requestId);
      mixedAcks.push((await consoleHost.sessionMap()).requestId);
    }
    assert.equal(mixedAcks.length, 250, "mixed host regression covers 250 successful calls");
    const mixedSendIds = events.slice(mixedSendStart).map((event) => event[2]);
    assert.deepEqual(mixedAcks, mixedSendIds, "acknowledgements match actual outbound send order");
    assert.equal(mixedAcks[0], 2, "sessionMap consumes the first host allocation");
    assert.equal(mixedAcks.at(-1), 251, "250 mixed calls occupy IDs 2 through 251");
    assert(mixedAcks.every((id, index) => index === 0 || id === mixedAcks[index - 1] + 1),
      "mixed acknowledgements have adjacent IDs");
    assert(mixedAcks.every((_id, index) => index % 10 !== 1 || mixedAcks[index] === mixedAcks[index - 1] + 1),
      "observe delegates with exactly one allocation");
    assert.equal(new Set(mixedAcks).size, mixedAcks.length, "mixed acknowledgements are unique");
    console.log("Issue393 mixed calls: 250 result-zero, send IDs 2..251, adjacent, observe single allocation");
    // The shipped SDK consumer uses the same real host: sessionMap -> direct meter lease ->
    // semantic console submit. This is the collision reproducer that the host-owned ledger fixes.
    const { createBrowserConsole } = await import(new URL("./sdk/src/browser/console.ts", root));
    const browserConsole = await createBrowserConsole(consoleHost);
    await consoleHost.meters({ enabled: true, onFrame: null });
    const sdkReport = await browserConsole.submit(browserConsole.edit.track("kick").faderDb(-1));
    assert.equal(sdkReport.ok, true, "SDK console submits through the real host after direct meters");
    // Every bounded response class refuses locally with requestId 0 and leaves the next accepted
    // request exactly one ID later. Hold each class independently so the fake port cannot answer.
    const assertBoundedNoBurn = async (label, heldCall, refusedCall, nextCall) => {
      const before = events.length;
      holdAll = true;
      const held = heldCall();
      await localErrorResult(refusedCall(), 6);
      assert.equal(events.length, before + 1, `${label}: refusal posted no message`);
      holdAll = false;
      for (const respond of heldAll.splice(0)) respond();
      const first = await held;
      const next = await nextCall();
      assert.equal(next.requestId, first.requestId + 1, `${label}: refusal did not burn an ID`);
      return next;
    };
    await assertBoundedNoBurn(
      "status", () => consoleHost.status(), () => consoleHost.status(), () => consoleHost.status(),
    );
    await assertBoundedNoBurn(
      "sessionMap", () => consoleHost.sessionMap(), () => consoleHost.sessionMap(),
      () => consoleHost.sessionMap(),
    );
    await assertBoundedNoBurn(
      "meters", () => consoleHost.meters({ enabled: false, onFrame: null }),
      () => consoleHost.meters({ enabled: true, onFrame: null }),
      () => consoleHost.meters({ enabled: false, onFrame: null }),
    );
    await assertBoundedNoBurn(
      "telemetry", () => consoleHost.telemetry({ enabled: false, onFrame: null }),
      () => consoleHost.telemetry({ enabled: true, onFrame: null }),
      () => consoleHost.telemetry({ enabled: false, onFrame: null }),
    );
    const commandAck = await consoleHost.command({ commands: [pan] });
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
      consoleHost.command({ commands: [{ ...pan, kind: 99 }] }),
      1,
    );
    await errorResult(
      consoleHost.command({ commands: [{ ...pan, values: [0, 0, 0, NaN] }] }),
      1,
    );
    await errorResult(consoleHost.command({ commands: [] }), 1);
    await localErrorResult(consoleHost.command({ requestId: 999, commands: [pan] }), 1);
    await localErrorResult(consoleHost.observe({ requestId: 999, subscriptions: [{
      trackIndex: 0, rack: 1, effectIndex: 0, tapId: 1, windowBlocks: 1, armed: true,
    }] }), 1);
    const oldShapeBuffer = new ArrayBuffer(256);
    await localErrorResult(consoleHost.submitSource({
      requestId: 999, sourceId: "extra", generation: 1n, startFrame: 0n,
      sampleRateHz: 48000, planes: [new Float32Array(oldShapeBuffer)], frames: 64, endOfRegion: false,
    }), 1);
    assert.equal(oldShapeBuffer.byteLength, 256, "malformed source keeps caller ownership");
    await localErrorResult(consoleHost.seekSource({
      requestId: 999, sourceId: "extra", generation: 1n, sourceFrame: 0n,
    }), 1);
    await localErrorResult(consoleHost.meters({ requestId: 999, enabled: true, onFrame: null }), 1);
    await localErrorResult(consoleHost.telemetry({ requestId: 999, enabled: true, onFrame: null }), 1);
    assert.equal(events.length, beforeMalformed, "a malformed batch costs no message");

    // Engine backpressure is a resolved acknowledgement that admits nothing.
    commandResult = 6;
    const refused = await consoleHost.command({ commands: [pan] });
    assert.equal(refused.result, 6);
    assert.equal(refused.admitted, 0);
    assert.equal(refused.reason, 8);
    commandResult = 0;

    // Local backpressure: the worklet-side queue depth is 4, so a fifth unsettled batch is
    // refused here, before any transfer, and the caller keeps its records.
    holdAll = true;
    const commandEventsBefore = events.length;
    const held4 = Array.from({ length: 4 }, () => consoleHost.command({ commands: [pan] }));
    await localErrorResult(consoleHost.command({ commands: [pan] }), 6);
    await localErrorResult(consoleHost.observe({ subscriptions: [mixedSubscription] }), 6);
    assert.equal(events.length, commandEventsBefore + 4, "command/observe saturation posts no refusal");
    holdAll = false;
    for (const respond of heldAll) respond();
    heldAll.length = 0;
    const heldCommandAcks = await Promise.all(held4);
    const commandAfterBound = await consoleHost.command({ commands: [pan] });
    assert.equal(
      commandAfterBound.requestId,
      Math.max(...heldCommandAcks.map((ack) => ack.requestId)) + 1,
      "command saturation does not burn an ID",
    );

    // Leases and their unsolicited frames.
    const meterFrames = [];
    const telemetryFrames = [];
    assert.equal(
      (await consoleHost.meters({ enabled: true, onFrame: (frame) => meterFrames.push(frame) })).result,
      0,
    );
    assert.equal(
      (await consoleHost.telemetry({ enabled: true, onFrame: (frame) => telemetryFrames.push(frame) })).result,
      0,
    );
    const node = FakeNode.latest;
    node.port.onmessage({
      data: {
        tag: "miso.meter.v1", sequence: 1, generation: 1n, validity: 0xb, lossCount: 0,
        windows: 1, trackCount: 2,
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
      { generation: 0n },
      { generation: 1 },
      { validity: 0x1 },
      { validity: 0x13 },
      { validity: 0x7, lossCount: 0 },
      { validity: 0x3, lossCount: 1 },
      { lossCount: 0x1_0000_0000 },
      { trackGrDb: new Float32Array(1) },
      { trackGrDb: [0, 0] },
      { trackGrDb: new Float32Array([-6.5, 0]) },
      { trackGrDb: new Float32Array([Number.NaN, 0]) },
      { masterGrDb: -1 },
      { masterGrDb: "6.5" },
      { firstSample: 512 },
      { endSample: 256n },
      { endSample: 512n },
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
          tag: "miso.meter.v1", sequence: 1, generation: 1n, validity: 0xb, lossCount: 0,
          windows: 1, trackCount: 2,
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
        subscriptions: [{
          trackIndex: 0, rack: 1, effectIndex: 0, tapId: 1, windowBlocks: 0, armed: true,
          ...broken,
        }],
      }), 1);
    }
    await errorResult(consoleHost.observe({ subscriptions: [] }), 1);

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
        commands: [pan],
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
          tag: "miso.meter.v1", sequence: 10 + reason, generation: 1n, validity: 0xb,
          lossCount: 0, windows: 1, trackCount: 2,
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
        subscriptions: [
          { trackIndex: 1, rack: 1, effectIndex: 0, tapId: 1, windowBlocks: 4, armed: true },
        ],
      });
      assert.equal(recovered.result, 0, `${what}: a correct subscription still arms after it`);
      assert.equal(recovered.reason, 0);
      assert.deepEqual(recovered.bindings.map((binding) => binding.trackIndex), [0, 1]);
      const undo = await consoleHost.observe({
        subscriptions: [
          { trackIndex: 1, rack: 1, effectIndex: 0, tapId: 1, windowBlocks: 4, armed: false },
        ],
      });
      assert.deepEqual(undo.bindings, mapBeforeRefusal, "the map returns to where it started");
    }

    // A released lease detaches the callback: a late frame is delivered nowhere.
    const framesAtRelease = meterFrames.length;
    await consoleHost.meters({ enabled: false, onFrame: null });
    node.port.onmessage({
      data: {
        tag: "miso.meter.v1", sequence: 2, generation: 1n, validity: 0x3, lossCount: 0,
        windows: 1, trackCount: 2,
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
        tag: "miso.meter.v1", sequence: 3, generation: 1n, validity: 0x3, lossCount: 0,
        windows: 1, trackCount: 2,
        peaks: new Float32Array(4), trackGrDb: new Float32Array(2), masterGrDb: null,
        firstSample: 0n, endSample: 0n,
      },
    });
    await errorResult(doomed, 255);
    await consoleHost.dispose();
    const disposedEvents = events.length;
    await localErrorResult(consoleHost.status(), 3);
    await localErrorResult(consoleHost.command({ commands: [pan] }), 3);
    await localErrorResult(consoleHost.observe({ subscriptions: [mixedSubscription] }), 3);
    await localErrorResult(consoleHost.sessionMap(), 3);
    await localErrorResult(consoleHost.meters({ enabled: false, onFrame: null }), 3);
    await localErrorResult(consoleHost.telemetry({ enabled: false, onFrame: null }), 3);
    await localErrorResult(consoleHost.seekSource({
      sourceId: "disposed", generation: 1n, sourceFrame: 0n,
    }), 3);
    await localErrorResult(consoleHost.submitSource({
      sourceId: "disposed", generation: 1n, startFrame: 0n, sampleRateHz: 48000,
      planes: [new Float32Array(2)], frames: 2, endOfRegion: true,
    }), 3);
    assert.equal(events.length, disposedEvents, "disposed refusals post no messages or consume IDs");

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
      subscriptions: [tap(0, 0, true), tap(1, 0, true)],
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
      subscriptions: [tap(0, 0, true), tap(1, 0, true)],
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
      subscriptions: [tap(0, 1, true), tap(1, 1, true)],
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
        enabled: true, onFrame: (frame) => rearmedFrames.push(frame),
      })).result,
      0,
    );
    FakeNode.latest.port.onmessage({
      data: {
        tag: "miso.meter.v1", sequence: 1, generation: 1n, validity: 0xb, lossCount: 0,
        windows: 1, trackCount: 2,
        peaks: new Float32Array(6), trackGrDb: new Float32Array([1.5, 2.5]), masterGrDb: 2.5,
        firstSample: 0n, endSample: 256n,
      },
    });
    assert.equal(rearmedFrames.length, 1, "the replacement's meter sequence restarts at 1");
    assert.deepEqual([...rearmedFrames[0].trackGrDb], [1.5, 2.5]);
    assert.equal((await afterEdit.command({ commands: [pan] })).result, 0);
    assert.equal((await afterEdit.status()).result, 0);
    await afterEdit.dispose();
  } finally {
    globalThis.fetch = original.fetch;
    globalThis.AudioWorkletNode = original.AudioWorkletNode;
    WebAssembly.validate = original.validate;
    WebAssembly.compile = original.compile;
  }
}

function createFakeExports(quantum, backend = 1, consoleAttached = true) {
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
  // Assignment 8's opaque prepared companion staging is present in the real worklet even while
  // the default semantic command route remains ordinary. Keep the hermetic processor fixture's
  // shape aligned so construction exercises the same bounded prewarm checks.
  const preparedCompanionPointer = 23000;
  const preparedCompanionCapacity = 24 + 2 * 256 * 80;
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
  meterHeader.setBigUint64(48, 1n, true);
  meterHeader.setBigUint64(56, 0xbn, true);
  const report = new DataView(memory.buffer, reportPointer, 48);
  report.setUint32(0, 48, true);
  report.setUint32(4, 0x00010000, true);
  const pointers = {
    2: 4096, 3: 5000, 5: 8192, 6: consoleAttached ? commandPointer : 0, 7: meterFramePointer,
  };
  const capacities = {
    2: 64, 3: 2 * quantum * 4, 5: 2 * quantum * 4,
    6: consoleAttached ? 256 * 48 : 0,
    7: meterFrameFloats * 4,
  };
  // Issue #143: the additive observation ABI is present even in this hermetic processor fake.
  // This fixture does not model a compiled observation row, but the worklet still prewarms and
  // discovers the fixed staging views before it can publish `miso.ready.v1`.
  const observationIdPointer = 17500;
  const observationSelectionPointer = 18000;
  const observationResultPointer = 19000;
  const observationSelectionCapacity = 4;
  // Issue #779: prewarmed fixed live-response request, identity and raw-snapshot staging. The
  // fake refuses capture because it has no prepared graph, but it must expose the complete ABI so
  // construction tests exercise the same prewarm path as the shipped Worklet.
  const trackResponseRequestPointer = 20000;
  const trackResponseTrackIdPointer = 20100;
  const trackResponseSnapshotPointer = 22000;
  const trackResponseSnapshotCapacity = 30000;
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
    miso_engine_web_v1_prepared_companion_ptr: () => (
      consoleAttached ? preparedCompanionPointer : 0
    ),
    miso_engine_web_v1_prepared_companion_capacity: () => (
      consoleAttached ? preparedCompanionCapacity : 0
    ),
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
    miso_engine_web_v1_observation_count: () => 0,
    miso_engine_web_v1_observation_id_ptr: () => observationIdPointer,
    miso_engine_web_v1_observation_id_capacity: () => 128,
    miso_engine_web_v1_observation_track_index: () => 0,
    miso_engine_web_v1_observation_rack: () => 1,
    miso_engine_web_v1_observation_effect_index: () => 0,
    miso_engine_web_v1_observation_effect_slot_id: () => 0,
    miso_engine_web_v1_observation_native_effect_id: () => 0,
    miso_engine_web_v1_observation_tap_count: () => 0,
    miso_engine_web_v1_observation_tap_id: () => 0,
    miso_engine_web_v1_observation_selection_ptr: () => observationSelectionPointer,
    miso_engine_web_v1_observation_selection_bytes: () => 32,
    miso_engine_web_v1_observation_selection_capacity: () => observationSelectionCapacity,
    miso_engine_web_v1_observation_read: () => 0,
    miso_engine_web_v1_observation_result_ptr: () => observationResultPointer,
    miso_engine_web_v1_observation_result_bytes: (_handle, count = 0) => count * 96,
    miso_engine_web_v1_track_response_request_ptr: () => trackResponseRequestPointer,
    miso_engine_web_v1_track_response_request_bytes: () => 48,
    miso_engine_web_v1_track_response_track_id_ptr: () => trackResponseTrackIdPointer,
    miso_engine_web_v1_track_response_track_id_capacity: () => 127,
    miso_engine_web_v1_track_response_snapshot_ptr: () => trackResponseSnapshotPointer,
    miso_engine_web_v1_track_response_snapshot_capacity: () => trackResponseSnapshotCapacity,
    miso_engine_web_v1_track_response_snapshot_set_bytes: (bytes) => (
      bytes >= 104 && bytes <= trackResponseSnapshotCapacity ? 0 : 1
    ),
    miso_engine_web_v1_track_response_capture: () => 7,
    miso_engine_web_v1_track_response_analysis: () => 7,
    miso_engine_web_v1_track_response_result_ptr: () => trackResponseSnapshotPointer,
    miso_engine_web_v1_track_response_result_bytes: () => 0,
    miso_engine_web_v1_track_response_close: () => 0,
    miso_engine_web_v1_dispose: () => {
      calls.dispose += 1;
      return 0;
    },
  };
  return { exports, calls, trackIds, sourceRows, meterFrameFloats, meterHeader };
}

function createTelemetryClock(elapsedMsByBlock) {
  let reads = 0;
  let timeMs = 0;
  return {
    now() {
      const block = Math.floor(reads / 2);
      if (block >= elapsedMsByBlock.length) throw new Error("telemetry clock read past fixture");
      if (reads % 2 === 1) timeMs += elapsedMsByBlock[block];
      reads += 1;
      return timeMs;
    },
    get reads() {
      return reads;
    },
  };
}

function withTelemetryClock(clock, callback) {
  const originalPerformance = Object.getOwnPropertyDescriptor(globalThis, "performance");
  Object.defineProperty(globalThis, "performance", {
    configurable: true,
    enumerable: originalPerformance?.enumerable ?? true,
    writable: true,
    value: { now: clock.now },
  });
  try {
    return callback();
  } finally {
    if (originalPerformance === undefined) delete globalThis.performance;
    else Object.defineProperty(globalThis, "performance", originalPerformance);
  }
}

// Issue #288: exercise the qualification caller's real boot objects through the same host and
// worklet guards as the main-realm suite. The stop sentinel deliberately ends each caller after
// real create/dispose, before any source/render/stall loop; the diagnostic initializer still runs.
async function testQualificationBoot({ registered, makeFake, setNextFake, setProcessorPortFactory, document }) {
  const originalOfflineAudioContext = globalThis.OfflineAudioContext;
  const originalAudioWorkletNode = globalThis.AudioWorkletNode;
  const originalFetch = globalThis.fetch;
  const originalCompile = WebAssembly.compile;
  const originalValidate = WebAssembly.validate;
  const originalSampleRate = globalThis.sampleRate;
  const originalRenderQuantumSize = globalThis.renderQuantumSize;
  const qualificationUrlWithCacheKey = `${qualificationUrl.href}?boot-contract-test`;
  const {
    createMisoAudioWorkletHost,
  } = await import(`${hostUrl.href}?qualification-boot-host`);
  const { qualificationBootContract: hooks } = await import(qualificationUrlWithCacheKey);
  const observed = [];
  const fetchUrls = [];
  let sentinelStops = 0;
  let realReady = 0;
  let realDisposed = 0;

  class QualificationPort {
    constructor() {
      this.peer = null;
      this.handler = null;
      this.pending = [];
      this.closed = false;
    }

    get onmessage() {
      return this.handler;
    }

    set onmessage(handler) {
      this.handler = handler;
      if (typeof handler !== "function") return;
      for (const event of this.pending.splice(0)) queueMicrotask(() => this.handler?.(event));
    }

    postMessage(message, transfer = []) {
      this.peer?.deliver(message, transfer);
    }

    deliver(message, transfer = []) {
      const event = { data: structuredClone(message, { transfer }) };
      if (typeof this.handler !== "function") this.pending.push(event);
      else queueMicrotask(() => this.handler?.(event));
    }

    close() {
      this.closed = true;
    }
  }

  class QualificationOfflineAudioContext {
    constructor(numberOfChannels, length, sampleRate) {
      this.numberOfChannels = numberOfChannels;
      this.length = length;
      this.sampleRate = sampleRate;
      this.renderQuantumSize = 128;
      this.state = "suspended";
      this.destination = {};
      this.audioWorklet = {
        addModule: async () => undefined,
      };
    }

  }

  class QualificationNode {
    static latest = null;

    constructor(context, _name, options = {}) {
      const hostPort = new QualificationPort();
      const processorPort = new QualificationPort();
      hostPort.peer = processorPort;
      processorPort.peer = hostPort;
      this.port = hostPort;
      this.options = options;
      this.onprocessorerror = null;
      this.disconnectCount = 0;
      const processorOptions = options.processorOptions?.options;
      setNextFake(makeFake(
        context.renderQuantumSize,
        1,
        processorOptions?.consoleCommandQueueRecords !== 0n,
      ));
      setProcessorPortFactory(() => processorPort);
      try {
        this.processor = new registered({ processorOptions: options.processorOptions });
      } finally {
        setProcessorPortFactory(null);
      }
      QualificationNode.latest = this;
    }

    disconnect() {
      this.disconnectCount += 1;
    }
  }

  class QualificationStop extends Error {
    constructor(label) {
      super(`qualification boot probe stopped before ${label} loop`);
      this.label = label;
    }
  }

  globalThis.OfflineAudioContext = QualificationOfflineAudioContext;
  globalThis.AudioWorkletNode = QualificationNode;
  globalThis.sampleRate = 48000;
  globalThis.renderQuantumSize = 128;
  globalThis.fetch = async (url) => {
    fetchUrls.push(String(url));
    return {
      ok: true,
      arrayBuffer: async () => new Uint8Array([0]).buffer,
      json: async () => preparedAbiLayout,
    };
  };
  WebAssembly.compile = async () => Object.freeze({ qualification: true });
  WebAssembly.validate = () => true;

  const bounded = (promise, label) => {
    let timer;
    const timeout = new Promise((_, reject) => {
      timer = setTimeout(() => reject(new Error(`qualification boot contract: ${label} timed out`)), 1000);
    });
    return Promise.race([promise, timeout]).finally(() => clearTimeout(timer));
  };
  const forwardingCreateHost = (label) => async (options) => {
    observed.push({ label, options });
    if (options.document !== undefined) {
      assert(options.document instanceof Uint8Array && options.document.byteLength > 0,
        `qualification boot contract: ${label} document must be nonempty`);
    }
    assert.equal(options.context.sampleRate, 48000,
      `qualification boot contract: ${label} sample rate mismatch`);
    assert.equal(options.context.renderQuantumSize, 128,
      `qualification boot contract: ${label} quantum mismatch`);
    if (label === "renderCorpusSegment" && options.options !== undefined) {
      await assert.rejects(
        () => createMisoAudioWorkletHost({
          ...options,
          options: { ...options.options, qualificationExtra: true },
        }),
        (error) => error?.tag === "miso.error.v1" && error.result === 1,
        "qualification boot contract: real guard must reject an extra option",
      );
    }
    let host;
    try {
      host = await bounded(createMisoAudioWorkletHost(options), `${label} real host`);
    } catch (error) {
      throw new Error(
        `qualification boot contract: ${label} real host guard rejected `
        + `(${error?.tag ?? error?.message ?? String(error)} result=${error?.result ?? "?"})`,
      );
    }
    assert.equal(host.backend, "simd128", `qualification boot contract: ${label} backend`);
    realReady += 1;
    try {
      await bounded(host.dispose(), `${label} real dispose`);
    } catch (error) {
      throw new Error(`qualification boot contract: ${label} real host dispose failed: ${error}`);
    }
    realDisposed += 1;
    if (label === "typedUnsupportedAttestation") {
      // The real guard has already accepted and disposed this boot. The helper's catch is then
      // exercised with the typed refusal it expects, instead of mistaking a guard failure for it.
      throw Object.freeze({
        tag: "miso.unsupported.v1", requestId: 0, result: 7, capability: "simd128",
      });
    }
    throw new QualificationStop(label);
  };

  const expectStop = async (label, operation) => {
    try {
      await operation();
    } catch (error) {
      if (error instanceof QualificationStop && error.label === label) {
        sentinelStops += 1;
        return;
      }
      throw error;
    }
    throw new Error(`qualification boot contract: ${label} did not reach its stop sentinel`);
  };

  try {
    const documentBytes = new Uint8Array(document);
    await expectStop("renderCorpusSegment", () => hooks.renderCorpusSegment(
      forwardingCreateHost("renderCorpusSegment"),
      documentBytes,
      [{ startFrame: 0, leftBase: 0, leftStep: 0, frames: 128, final: true }],
    ));
    assert.equal(
      await hooks.typedUnsupportedAttestation(
        forwardingCreateHost("typedUnsupportedAttestation"), documentBytes,
      ),
      true,
      "qualification boot contract: typed unsupported catch did not receive its typed refusal",
    );
    await expectStop("runConsoleQualification", () => hooks.runConsoleQualification(
      forwardingCreateHost("runConsoleQualification"), documentBytes,
    ));
    await expectStop("runObservationRun", () => hooks.runObservationRun(
      forwardingCreateHost("runObservationRun"), documentBytes, true,
    ));
    await expectStop("runStallQualification", () => hooks.runStallQualification(
      forwardingCreateHost("runStallQualification"), documentBytes,
    ));

    assert.deepEqual(
      observed.map(({ label }) => label),
      [
        "renderCorpusSegment", "typedUnsupportedAttestation", "runConsoleQualification",
        "runObservationRun", "runStallQualification",
      ],
      "qualification boot contract: caller witness set changed",
    );
    assert.equal(realReady, 5, "qualification boot contract: five real ready witnesses required");
    assert.equal(realDisposed, 5, "qualification boot contract: five real dispose witnesses required");
    assert.equal(sentinelStops, 4, "qualification boot contract: every non-attestation path must stop its sentinel");
    const plain = observed.filter(({ label }) =>
      label === "renderCorpusSegment" || label === "typedUnsupportedAttestation");
    assert(plain.every(({ options }) => options.options.consoleCommandQueueRecords === 0n
      && options.options.consoleMeterBlocks === 0n
      && options.options.consoleObservationTaps === 0n
      && options.options.consoleMasterTrackPlusOne === 0n),
    "qualification boot contract: plain option variant changed");
    const consoleOptions = observed.find(({ label }) => label === "runConsoleQualification").options.options;
    assert.equal(consoleOptions.consoleCommandQueueRecords, 64n);
    assert.equal(consoleOptions.consoleMeterBlocks, 2n);
    assert.equal(consoleOptions.consoleObservationTaps, 0n);
    assert.equal(consoleOptions.consoleMasterTrackPlusOne, 0n);
    const observationOptions = observed.find(({ label }) => label === "runObservationRun").options.options;
    assert.equal(observationOptions.consoleCommandQueueRecords, 64n);
    assert.equal(observationOptions.consoleMeterBlocks, 2n);
    assert.equal(observationOptions.consoleObservationTaps, 4n);
    assert.equal(observationOptions.consoleMasterTrackPlusOne, 1n);

    const diagnosis = await bounded(hooks.diagnoseReady(documentBytes), "diagnoseReady");
    assert.equal(diagnosis.kind, "message", "qualification boot contract: diagnoseReady did not reach node message");
    assert.equal(diagnosis.message.tag, "miso.ready.v1",
      `qualification boot contract: diagnoseReady initializer was refused `
      + `(tag=${diagnosis.message.tag} result=${diagnosis.message.result})`);
    assert.equal(diagnosis.message.result, 0,
      `qualification boot contract: diagnoseReady initializer was refused `
      + `(tag=${diagnosis.message.tag} result=${diagnosis.message.result})`);
    assert.equal(diagnosis.message.backend, "simd128");
    const diagnosticNode = QualificationNode.latest;
    assert(diagnosticNode?.processor instanceof registered,
      "qualification boot contract: diagnoseReady did not use the registered real worklet class");
    assert.equal(diagnosticNode.processor.ready, true,
      "qualification boot contract: diagnoseReady initializer did not become ready");
    assert(diagnosticNode.options.processorOptions.document instanceof Uint8Array
      && diagnosticNode.options.processorOptions.document.byteLength > 0);
    assert(fetchUrls.length >= 6 && fetchUrls.every((url) => typeof url === "string"),
      "qualification boot contract: fetch escaped the hermetic fixture");
    console.log(
      `qualification boot contract passed: callers=${observed.length} `
      + `real-ready=${realReady} real-disposed=${realDisposed} diagnose-ready=1`,
    );
  } finally {
    if (originalOfflineAudioContext === undefined) delete globalThis.OfflineAudioContext;
    else globalThis.OfflineAudioContext = originalOfflineAudioContext;
    if (originalAudioWorkletNode === undefined) delete globalThis.AudioWorkletNode;
    else globalThis.AudioWorkletNode = originalAudioWorkletNode;
    if (originalFetch === undefined) delete globalThis.fetch;
    else globalThis.fetch = originalFetch;
    WebAssembly.compile = originalCompile;
    WebAssembly.validate = originalValidate;
    if (originalSampleRate === undefined) delete globalThis.sampleRate;
    else globalThis.sampleRate = originalSampleRate;
    if (originalRenderQuantumSize === undefined) delete globalThis.renderQuantumSize;
    else globalThis.renderQuantumSize = originalRenderQuantumSize;
    setProcessorPortFactory(null);
  }
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
  let processorPortFactory = null;
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
      this.port = processorPortFactory?.() ?? new FakePort();
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
      assert.equal(frame.generation, 1n);
      assert.equal(frame.validity, 0xb);
      assert.equal(frame.lossCount, 0);
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

      // A nonzero poll is still withheld unless both complete peak-validity bits are present.
      // This exercises the real worklet guard against a malformed/future Rust publication.
      fake.meterHeader.setBigUint64(56, 0x1n, true);
      const invalidPosts = processor.port.posts.length;
      assert.equal(processor.process([], [[left, right]]), true);
      assert.equal(processor.port.posts.length, invalidPosts, "invalid peak metadata is not posted");

      // Loss metadata crosses the BigInt header as bounded numeric message fields.
      fake.meterHeader.setBigUint64(56, (2n << 32n) | 0xfn, true);
      assert.equal(processor.process([], [[left, right]]), true);
      const lossyFrame = processor.port.posts.at(-1).message;
      assert.equal(lossyFrame.tag, "miso.meter.v1");
      assert.equal(lossyFrame.sequence, 2);
      assert.equal(lossyFrame.generation, 1n);
      assert.equal(lossyFrame.validity, 0xf);
      assert.equal(lossyFrame.lossCount, 2);

      // Releasing the lease stops the frames immediately.
      processor.receive({ tag: "miso.meters.v1", requestId: 2, enabled: false });
      assert.deepEqual(fake.calls.meterLease, [1, 0]);
      const quiet = processor.port.posts.length;
      assert.equal(processor.process([], [[left, right]]), true);
      assert.equal(processor.port.posts.length, quiet, "a released lease posts nothing");
    }

    {
      // Issue #137 D3: a full telemetry window posts exactly one frame, and the frame is honest
      // about the resolution of the clock it actually found. The real process clock made the
      // zero-miss assertion scheduler-sensitive, so each window uses a local monotonic fixture.
      const belowBudgetMs = 0.5;
      const aboveBudgetMs = 2;
      const noMissWindow = Array.from({ length: 128 }, () => belowBudgetMs);
      const oneMissWindow = noMissWindow.map((duration, block) => block === 64
        ? aboveBudgetMs
        : duration);
      const runTelemetryWindow = (elapsedMsByBlock, expectedDeadlineMisses) => {
        const clock = createTelemetryClock(elapsedMsByBlock);
        return withTelemetryClock(clock, () => {
          const { processor } = makeProcessor();
          processor.receive({ tag: "miso.telemetry.v1", requestId: 1, enabled: true });
          assert.deepEqual(processor.port.posts.at(-1).message, {
            tag: "miso.ack.v1", requestId: 1, result: 0,
          });
          const left = new Float32Array(64);
          const right = new Float32Array(64);
          let frames = 0;
          let telemetry;
          for (let block = 0; block < 128; block += 1) {
            assert.equal(processor.process([], [[left, right]]), true);
            const last = processor.port.posts.at(-1).message;
            if (last.tag === "miso.telemetry.v1") {
              frames += 1;
              telemetry = last;
            }
          }
          assert.equal(frames, 1, "one frame per 128-block window and no more");
          assert.equal(telemetry.blocks, 128);
          assert.equal(telemetry.sequence, 1);
          assert.equal(telemetry.deadlineMisses, expectedDeadlineMisses);
          assert(telemetry.budgetMs > 1.3 && telemetry.budgetMs < 1.4, telemetry.budgetMs);
          assert(telemetry.resolutionMs > 0);
          assert.equal(typeof telemetry.belowResolution, "boolean");
          for (const field of ["cpuPercent", "peakBlockMs", "meanBlockMs"]) {
            assert(Number.isFinite(telemetry[field]) && telemetry[field] >= 0, field);
          }
          assert.equal(clock.reads, 256, "two clock reads per leased rendered block");
          processor.receive({ tag: "miso.telemetry.v1", requestId: 2, enabled: false });
          const quiet = processor.port.posts.length;
          const readsAfterRelease = clock.reads;
          for (let block = 0; block < 200; block += 1) {
            assert.equal(processor.process([], [[left, right]]), true);
          }
          assert.equal(clock.reads, readsAfterRelease, "a released lease reads no clock");
          assert.equal(processor.port.posts.length, quiet, "a released lease posts nothing");
        });
      };

      // The first fresh processor has 128 positive sub-budget blocks; the second has exactly one
      // over-budget block, preserving the behavioral deadline-miss discriminator.
      runTelemetryWindow(noMissWindow, 0);
      runTelemetryWindow(oneMissWindow, 1);
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

    await testQualificationBoot({
      registered,
      makeFake: createFakeExports,
      setNextFake: (fake) => { nextFake = fake; },
      setProcessorPortFactory: (factory) => { processorPortFactory = factory; },
      document: processorSessionDocument,
    });
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

if (realWasmReceiverArguments !== null) {
  await testRealWasmReceiver(realWasmReceiverArguments.artifactDirectory);
} else {
  await testMainRealm();
  await testProcessor();
  console.log("web AudioWorklet hermetic tests passed");
}
