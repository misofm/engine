import ABI_LAYOUT from "./miso-engine-v1-abi-layout.json" with { type: "json" };

// Private prepared-control transport shared by the browser host and the later headless route.
//
// This module owns semantic snapshots, accepted Rust configuration copies, and the opaque
// companion bytes. It deliberately contains no EQ equations and never exposes target objects to
// callers. The exact byte offsets are the generated host-web ABI layout; capacities are always
// obtained from the live Wasm exports before a buffer is written.

function structure(layout, name) {
  const value = layout[name] ?? layout.structures?.[name];
  if (value === undefined) throw new Error(`missing generated ABI structure ${name}`);
  return value;
}

function field(layout, structureName, fieldName) {
  const row = structure(layout, structureName).fields.find((candidate) => candidate.name === fieldName);
  if (row === undefined) throw new Error(`missing generated ABI field ${structureName}.${fieldName}`);
  return row.offset;
}

function arrayLength(layout, structureName, fieldName) {
  const row = structure(layout, structureName).fields.find((candidate) => candidate.name === fieldName);
  const match = row?.type.match(/\[(\d+)\]$/);
  if (match === null || match === undefined) throw new Error(`missing generated ABI array ${structureName}.${fieldName}`);
  return Number(match[1]);
}

function constant(layout, group, name) {
  const row = layout.constants?.[group]?.find?.((candidate) => candidate.name === name);
  if (row === undefined) throw new Error(`missing generated ABI constant ${group}.${name}`);
  return row.value;
}

function scalarConstant(layout, name) {
  const value = layout.constants?.[name];
  if (!Number.isSafeInteger(value)) throw new Error(`missing generated ABI scalar ${name}`);
  return value;
}

function schemaOf(layout) {
  const commandKind = constant(layout, "wireCommandKinds", "effectParam");
  return Object.freeze({
    abiVersion: layout.abiVersion,
    resultOk: constant(layout, "resultCodes", "ok"),
    resultInvalid: constant(layout, "resultCodes", "invalidArgument"),
    resultWrongState: constant(layout, "resultCodes", "wrongState"),
    resultBackpressure: constant(layout, "resultCodes", "backpressure"),
    resultUnsupported: constant(layout, "resultCodes", "unsupported"),
    resultInternal: constant(layout, "resultCodes", "internal"),
    effectParamKind: commandKind,
    maximumCommandRecords: scalarConstant(layout, "maximumCommandRecords"),
    commandRecordBytes: layout.commandRecord.bytes,
    request: Object.freeze({
      bytes: structure(layout, "eqTargetRequest").bytes,
      structSize: field(layout, "eqTargetRequest", "structSize"),
      abiVersion: field(layout, "eqTargetRequest", "abiVersion"),
      sampleRate: field(layout, "eqTargetRequest", "sampleRateHz"),
      seedCount: field(layout, "eqTargetRequest", "seedCount"),
      editCount: field(layout, "eqTargetRequest", "editCount"),
      reserved: field(layout, "eqTargetRequest", "reserved"),
      values: structure(layout, "eqTargetRequest").bytes,
      editBytes: structure(layout, "eqTargetEdit").bytes,
      editOffset: structure(layout, "eqTargetRequest").bytes + arrayLength(layout, "eqTargetConfig", "values") * 4,
      editParameterId: field(layout, "eqTargetEdit", "parameterId"),
      editChannel: field(layout, "eqTargetEdit", "channel"),
      editValue: field(layout, "eqTargetEdit", "value"),
    }),
    result: Object.freeze({
      headerBytes: structure(layout, "eqTargetResult").bytes,
      structSize: field(layout, "eqTargetResult", "structSize"),
      abiVersion: field(layout, "eqTargetResult", "abiVersion"),
      valueCount: field(layout, "eqTargetResult", "valueCount"),
      targetCount: field(layout, "eqTargetResult", "targetCount"),
      values: structure(layout, "eqTargetResult").bytes,
    }),
    target: Object.freeze({
      bytes: structure(layout, "preparedEffectTarget").bytes,
      slot: field(layout, "preparedEffectTarget", "slot"),
      channel: field(layout, "preparedEffectTarget", "channel"),
      words: field(layout, "preparedEffectTarget", "words"),
      wordCount: arrayLength(layout, "preparedEffectTarget", "words"),
    }),
    config: Object.freeze({
      bytes: structure(layout, "eqTargetConfig").bytes,
      structSize: field(layout, "eqTargetConfig", "structSize"),
      abiVersion: field(layout, "eqTargetConfig", "abiVersion"),
      sampleRate: field(layout, "eqTargetConfig", "sampleRateHz"),
      valueCount: field(layout, "eqTargetConfig", "valueCount"),
      generation: field(layout, "eqTargetConfig", "hostGeneration"),
      revision: field(layout, "eqTargetConfig", "ownerRevision"),
      values: field(layout, "eqTargetConfig", "values"),
      valueCountExpected: arrayLength(layout, "eqTargetConfig", "values"),
    }),
    companion: Object.freeze({
      headerBytes: structure(layout, "preparedEffectCompanionHeader").bytes,
      structSize: field(layout, "preparedEffectCompanionHeader", "structSize"),
      abiVersion: field(layout, "preparedEffectCompanionHeader", "abiVersion"),
      generation: field(layout, "preparedEffectCompanionHeader", "hostGeneration"),
      targetCount: field(layout, "preparedEffectCompanionHeader", "targetCount"),
      reserved: field(layout, "preparedEffectCompanionHeader", "reserved"),
      recordBytes: structure(layout, "preparedEffectCompanionRecord").bytes,
      trackIndex: field(layout, "preparedEffectCompanionRecord", "trackIndex"),
      rack: field(layout, "preparedEffectCompanionRecord", "rack"),
      effectIndex: field(layout, "preparedEffectCompanionRecord", "effectIndex"),
      recordReserved: field(layout, "preparedEffectCompanionRecord", "reserved"),
      revision: field(layout, "preparedEffectCompanionRecord", "baseRevision"),
      slot: field(layout, "preparedEffectCompanionRecord", "slot"),
      channel: field(layout, "preparedEffectCompanionRecord", "channel"),
      words: field(layout, "preparedEffectCompanionRecord", "words"),
    }),
  });
}

function error(result, requestId = 0) {
  return Object.freeze({ tag: "miso.error.v1", requestId, result });
}

function sameBytes(left, right) {
  if (!(left instanceof Uint8Array) || left.byteLength !== right.byteLength) return false;
  for (let index = 0; index < right.byteLength; index += 1) {
    if (left[index] !== right[index]) return false;
  }
  return true;
}

function keyOf(command) {
  return `${command.trackIndex}/${command.rack}/${command.effectIndex}`;
}

function cloneCommands(commands) {
  return commands.map((command) => ({
    ...command,
    values: [...command.values],
  }));
}

function readConfig(bytes, sampleRateHz, schema) {
  if (!(bytes instanceof Uint8Array) || bytes.byteLength !== schema.config.bytes) {
    throw error(schema.resultInternal);
  }
  const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  if (view.getUint32(schema.config.structSize, true) !== schema.config.bytes
      || view.getUint32(schema.config.abiVersion, true) !== schema.abiVersion
      || view.getUint32(schema.config.sampleRate, true) !== sampleRateHz
      || view.getUint32(schema.config.valueCount, true) !== schema.config.valueCountExpected) {
    throw error(schema.resultInternal);
  }
  const generation = view.getBigUint64(schema.config.generation, true);
  const revision = view.getBigUint64(schema.config.revision, true);
  if (generation === 0n || revision === 0xffffffffffffffffn) throw error(schema.resultInternal);
  const values = new Float32Array(schema.config.valueCountExpected);
  for (let index = 0; index < schema.config.valueCountExpected; index += 1) {
    const value = view.getFloat32(schema.config.values + index * 4, true);
    if (!Number.isFinite(value)) throw error(schema.resultInternal);
    values[index] = value;
  }
  return { generation, revision, values };
}

function readPreparationResult(exports, memory, schema) {
  const pointer = exports.miso_engine_web_v1_eq_target_result_ptr();
  const bytes = exports.miso_engine_web_v1_eq_target_result_bytes();
  const capacity = exports.miso_engine_web_v1_eq_target_result_capacity();
  if (!Number.isSafeInteger(pointer) || pointer === 0
      || !Number.isSafeInteger(bytes) || bytes < schema.result.headerBytes
      || !Number.isSafeInteger(capacity) || bytes > capacity
      || pointer + bytes > memory.buffer.byteLength) throw error(schema.resultInternal);
  const view = new DataView(memory.buffer, pointer, bytes);
  const valueCount = view.getUint32(schema.result.valueCount, true);
  const targetCount = view.getUint32(schema.result.targetCount, true);
  const targetOffset = schema.result.headerBytes + schema.config.valueCountExpected * 4;
  const maximumTargets = Math.floor((capacity - targetOffset) / schema.target.bytes);
  const expected = targetOffset + targetCount * schema.target.bytes;
  if (view.getUint32(schema.result.structSize, true) !== schema.result.headerBytes
      || view.getUint32(schema.result.abiVersion, true) !== schema.abiVersion
      || valueCount !== schema.config.valueCountExpected || targetCount > maximumTargets
      || expected !== bytes || expected > capacity) throw error(schema.resultInternal);
  const values = new Float32Array(schema.config.valueCountExpected);
  for (let index = 0; index < schema.config.valueCountExpected; index += 1) {
    const value = view.getFloat32(schema.result.values + index * 4, true);
    if (!Number.isFinite(value)) throw error(schema.resultInternal);
    values[index] = value;
  }
  const targets = [];
  for (let index = 0; index < targetCount; index += 1) {
    const offset = targetOffset + index * schema.target.bytes;
    const words = new Uint32Array(schema.target.wordCount);
    for (let word = 0; word < words.length; word += 1) {
      words[word] = view.getUint32(offset + schema.target.words + word * 4, true);
    }
    targets.push({
      slot: view.getUint32(offset + schema.target.slot, true),
      channel: view.getUint32(offset + schema.target.channel, true),
      words,
    });
  }
  return { values, targets };
}

function buildRequest(exports, memory, sampleRateHz, values, edits, schema) {
  const pointer = exports.miso_engine_web_v1_eq_target_request_ptr();
  const capacity = exports.miso_engine_web_v1_eq_target_request_capacity();
  const bytes = schema.request.editOffset + edits.length * schema.request.editBytes;
  if (!Number.isSafeInteger(pointer) || pointer === 0 || !Number.isSafeInteger(capacity)
      || bytes > capacity || pointer + bytes > memory.buffer.byteLength) throw error(schema.resultInternal);
  const view = new DataView(memory.buffer, pointer, bytes);
  view.setUint32(schema.request.structSize, schema.request.bytes, true);
  view.setUint32(schema.request.abiVersion, schema.abiVersion, true);
  view.setUint32(schema.request.sampleRate, sampleRateHz, true);
  view.setUint32(schema.request.seedCount, schema.config.valueCountExpected, true);
  view.setUint32(schema.request.editCount, edits.length, true);
  view.setUint32(schema.request.reserved, 0, true);
  view.setUint32(schema.request.reserved + 4, 0, true);
  view.setUint32(schema.request.reserved + 8, 0, true);
  values.forEach((value, index) => view.setFloat32(schema.request.values + index * 4, value, true));
  edits.forEach((edit, index) => {
    const offset = schema.request.editOffset + index * schema.request.editBytes;
    view.setUint32(offset + schema.request.editParameterId, edit.parameterId, true);
    view.setUint32(offset + schema.request.editChannel, edit.channel, true);
    view.setFloat32(offset + schema.request.editValue, edit.value, true);
  });
  return bytes;
}

function buildCompanion(generation, owners, schema) {
  const targetCount = owners.reduce((total, owner) => total + owner.prepared.targets.length, 0);
  if (targetCount > 2 * schema.maximumCommandRecords) throw error(schema.resultBackpressure);
  const bytes = schema.companion.headerBytes + targetCount * schema.companion.recordBytes;
  const payload = new Uint8Array(bytes);
  const view = new DataView(payload.buffer);
  view.setUint32(schema.companion.structSize, schema.companion.headerBytes, true);
  view.setUint32(schema.companion.abiVersion, schema.abiVersion, true);
  view.setBigUint64(schema.companion.generation, generation, true);
  view.setUint32(schema.companion.targetCount, targetCount, true);
  view.setUint32(schema.companion.reserved, 0, true);
  let index = 0;
  for (const owner of owners) {
    for (const target of owner.prepared.targets) {
      const offset = schema.companion.headerBytes + index * schema.companion.recordBytes;
      view.setUint32(offset + schema.companion.trackIndex, owner.address.trackIndex, true);
      view.setUint32(offset + schema.companion.rack, owner.address.rack, true);
      view.setUint32(offset + schema.companion.effectIndex, owner.address.effectIndex, true);
      view.setUint32(offset + schema.companion.recordReserved, 0, true);
      view.setBigUint64(offset + schema.companion.revision, owner.revision, true);
      view.setUint32(offset + schema.companion.slot, target.slot, true);
      view.setUint32(offset + schema.companion.channel, target.channel, true);
      target.words.forEach((word, wordIndex) =>
        view.setUint32(offset + schema.companion.words + wordIndex * 4, word, true));
      index += 1;
    }
  }
  return payload;
}

/** Construct the private browser/headless prepared-control owner. */
export function createPreparedControl(options) {
  const { module, sampleRateHz, configCopy, ordinarySubmit, preparedSubmit } = options;
  const schema = schemaOf(options.abiLayout ?? ABI_LAYOUT);
  const owners = new Map();
  let preparation = null;
  let pending = null;
  let disposed = false;
  let generation = 0n;

  try {
    if (!(module instanceof WebAssembly.Module)) throw error(schema.resultInvalid);
    const instance = new WebAssembly.Instance(module, {});
    const exports = instance.exports;
    const open = exports.miso_engine_web_v1_eq_target_open;
    if (typeof open !== "function") throw error(schema.resultInternal);
    const openResult = open();
    if (openResult === schema.resultUnsupported) {
      preparation = { available: false };
    } else if (openResult !== schema.resultOk) {
      throw error(openResult);
    } else if (!(exports.memory instanceof WebAssembly.Memory)) {
      throw error(schema.resultInternal);
    } else {
      preparation = { available: true, instance, exports, memory: exports.memory };
    }
  } catch (_) {
    preparation = { available: false, failure: error(schema.resultInternal) };
  }

  function invalidate(addresses = undefined) {
    if (addresses === undefined) owners.clear();
    else for (const address of addresses) owners.delete(keyOf(address));
    generation = 0n;
  }

  async function loadOwner(address) {
    const key = keyOf(address);
    const existing = owners.get(key);
    if (existing !== undefined) return existing;
    if (!preparation?.available) {
      if (preparation?.failure !== undefined) throw preparation.failure;
      // Production EQ registration is intentionally deferred until assignment 10. Keep the
      // existing ordinary route alive during that window without caching an address classification.
      return { status: "ordinary", address };
    }
    const reply = await configCopy(address);
    if (reply?.result === schema.resultUnsupported) {
      const ordinary = { status: "ordinary", address };
      owners.set(key, ordinary);
      return ordinary;
    }
    if (reply?.result !== schema.resultOk) {
      throw error(reply?.result ?? schema.resultInternal, reply?.requestId ?? 0);
    }
    const config = readConfig(reply.config, sampleRateHz, schema);
    if (generation !== 0n && generation !== config.generation) {
      invalidate();
      throw error(schema.resultWrongState);
    }
    generation = config.generation;
    const owner = {
      status: "prepared",
      address,
      sampleRateHz,
      generation: config.generation,
      revision: config.revision,
      values: config.values,
    };
    owners.set(key, owner);
    return owner;
  }

  async function prepareOwner(owner, commands) {
    const edits = commands.map((command) => ({
      parameterId: command.parameterId,
      channel: command.channel,
      value: command.values[0],
    }));
    const requestBytes = buildRequest(
      preparation.exports,
      preparation.memory,
      sampleRateHz,
      owner.values,
      edits,
      schema,
    );
    const result = preparation.exports.miso_engine_web_v1_eq_target_prepare(requestBytes);
    if (result !== schema.resultOk) {
      const rejected = preparation.exports.miso_engine_web_v1_eq_target_rejected_edit_index();
      throw Object.assign(error(result), { rejectedEditIndex: rejected });
    }
    // Copy before preparing another owner overwrites Rust's single result backing.
    const prepared = readPreparationResult(
      preparation.exports,
      preparation.memory,
      schema,
    );
    return { ...owner, prepared };
  }

  function close() {
    disposed = true;
    invalidate();
    if (preparation?.available) {
      try { preparation.exports.miso_engine_web_v1_eq_target_close(); } catch (_) { /* terminal */ }
    }
    preparation = null;
  }

  function submit(commands, records) {
    if (disposed) return Promise.reject(error(schema.resultWrongState));
    const snapshot = cloneCommands(commands);
    if (!(records instanceof Uint8Array)
        || records.byteLength !== snapshot.length * schema.commandRecordBytes) {
      return Promise.reject(error(schema.resultInvalid));
    }
    const effectCommands = snapshot.filter((command) => command.kind === schema.effectParamKind);
    if (effectCommands.length === 0) {
      return ordinarySubmit(records, snapshot);
    }
    if (pending !== null) return Promise.reject(error(schema.resultBackpressure));
    pending = { snapshot };
    // Keep an independent copy because `records` is transferred to the Worklet transport.
    const originalRecords = records.slice();
    const affected = new Set();
    const run = async () => {
      const groups = new Map();
      for (const command of effectCommands) {
        const key = keyOf(command);
        let group = groups.get(key);
        if (group === undefined) {
          group = { address: {
            trackIndex: command.trackIndex,
            rack: command.rack,
            effectIndex: command.effectIndex,
          }, commands: [], indexes: [] };
          groups.set(key, group);
        }
        group.commands.push(command);
        group.indexes.push(snapshot.indexOf(command));
      }
      const preparedOwners = [];
      for (const group of groups.values()) {
        const owner = await loadOwner(group.address);
        if (owner.status !== "prepared") continue;
        affected.add(group.address);
        let prepared;
        try {
          prepared = await prepareOwner(owner, group.commands);
        } catch (failure) {
          const local = failure?.rejectedEditIndex;
          if (Number.isSafeInteger(local) && local >= 0 && local < group.indexes.length) {
            throw Object.assign({}, failure, { rejectedEditIndex: group.indexes[local] });
          }
          throw failure;
        }
        preparedOwners.push({ ...prepared, address: owner.address, revision: owner.revision });
      }
      if (preparedOwners.length === 0) return ordinarySubmit(originalRecords, snapshot);
      const nextRevisions = preparedOwners.map((owner) => {
        if (owner.revision === 0xffffffffffffffffn) throw error(schema.resultBackpressure);
        return owner.revision + 1n;
      });
      const companion = buildCompanion(generation, preparedOwners, schema);
      const reply = await preparedSubmit(originalRecords, companion, snapshot);
      if (reply?.result !== schema.resultOk || reply.admitted !== snapshot.length
          || !(reply.records instanceof Uint8Array) || !sameBytes(reply.records, originalRecords)) {
        if (reply?.result === schema.resultOk) invalidate([...affected]);
        for (let index = 0; index < preparedOwners.length; index += 1) {
          const owner = preparedOwners[index];
          const current = owners.get(keyOf(owner.address));
          if (current?.status === "prepared" && reply?.result === schema.resultOk) {
            current.values = owner.prepared.values.slice();
            current.revision = nextRevisions[index];
          }
        }
        return reply;
      }
      for (let index = 0; index < preparedOwners.length; index += 1) {
        const owner = preparedOwners[index];
        const current = owners.get(keyOf(owner.address));
        if (current?.status === "prepared") {
          current.values = owner.prepared.values.slice();
          current.revision = nextRevisions[index];
        }
      }
      return reply;
    };
    return run().catch((failure) => {
      invalidate([...affected]);
      throw failure;
    }).finally(() => {
      pending = null;
    });
  }

  return Object.freeze({
    submit,
    close,
    invalidate,
    get available() { return preparation?.available === true; },
  });
}
