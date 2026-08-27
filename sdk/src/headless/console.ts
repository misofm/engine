import { encodeCommandBatch, type CommandRecord } from "../core/command.js";
import { effect as validateEffect, type PlacedEffect, type SessionPlan, type SessionShape } from "../core/session.js";
import type { CommandAck, EffectDecl, EffectParamValues, Rack } from "../core/types.js";
import { ABI_LAYOUT } from "../generated/abi.js";
import { CATALOG, type CommandReasonName, type EffectDescriptor, type EffectId } from "../generated/catalog.js";
import { MisoCommandError } from "../core/errors.js";
import { MisoOfflineError } from "./errors.js";
import type { WasmBoundary } from "./abi.js";
import type {
  CommandBatch,
  EngineConsole,
  ObservationAddress,
  ObservationHandle,
  OfflineEffectConsole,
  OfflineTrackConsole,
  ParamAddress,
} from "./types.js";

const kind = Object.freeze(Object.fromEntries(ABI_LAYOUT.constants.wireCommandKinds.map((row) => [row.name, row.value]))) as Readonly<Record<string, number>>;
const reasonByValue: ReadonlyMap<number, CommandReasonName> = new Map(ABI_LAYOUT.constants.commandReasons.map((row) => [row.value, row.name]));
const reportOffsets = Object.freeze(Object.fromEntries(ABI_LAYOUT.structures.commandReport.fields.map((field) => [field.name, field.offset]))) as Readonly<Record<string, number>>;
const rackByte: Readonly<Record<Rack, number>> = Object.freeze({ simd1: 0, dynamic: 1, simd2: 2 });
const channelByte = Object.freeze({ left: 0, right: 1, both: 2 });
const NONE = 255;

function localF32(value: number, path: string, minimum: number, maximum: number): number {
  const rounded = Math.fround(value);
  if (!Number.isFinite(value) || !Number.isFinite(rounded) || rounded < minimum || rounded > maximum) {
    throw new MisoCommandError(`Expected a finite f32 in [${minimum}, ${maximum}]`, path);
  }
  return rounded;
}

interface EffectAddress {
  readonly trackIndex: number;
  readonly rack: Rack;
  readonly rackIndex: number;
  readonly effectId: EffectId;
  readonly slotId: string;
}

function descriptor(id: EffectId): EffectDescriptor {
  const found = CATALOG.effects.find((candidate) => candidate.id === id);
  if (!found) throw new MisoCommandError(`Unknown generated effect ${id}`, "effectId");
  return found;
}

function scalarValue(effectDescriptor: EffectDescriptor, name: string, value: unknown): number {
  const parameter = effectDescriptor.parameters.find((candidate) => candidate.name === name);
  if (!parameter) throw new MisoCommandError("Unknown effect parameter", `parameters.${name}`);
  if (parameter.domainName === "boolean") return value === true ? 1 : 0;
  if (parameter.domainName === "enumeration") {
    const choice = parameter.enumChoices.find((candidate) => candidate.label === value);
    if (!choice) throw new MisoCommandError("Unknown effect parameter enumeration label", `parameters.${name}`);
    return choice.value;
  }
  return Number(value);
}

function lanes(value: unknown): readonly Readonly<{ channel: "left" | "right" | "both"; value: unknown }>[] {
  if (Array.isArray(value)) return Object.freeze([{ channel: "left", value: value[0] }, { channel: "right", value: value[1] }]);
  if (value && typeof value === "object") {
    const pair = value as Readonly<{ left: unknown; right: unknown }>;
    return Object.freeze([{ channel: "left", value: pair.left }, { channel: "right", value: pair.right }]);
  }
  return Object.freeze([{ channel: "both", value }]);
}

function record(partial: Partial<CommandRecord> & Pick<CommandRecord, "kind" | "trackIndex">): CommandRecord {
  const values: readonly [number, number, number, number] = partial.values ?? [0, 0, 0, 0];
  return Object.freeze({
    kind: partial.kind,
    rack: partial.rack ?? NONE,
    channel: partial.channel ?? NONE,
    trackIndex: partial.trackIndex,
    effectIndex: partial.effectIndex ?? 0,
    parameterId: partial.parameterId ?? 0,
    smoothingSamples: partial.smoothingSamples ?? 0,
    values,
  });
}

class Batch implements CommandBatch {
  private readonly records: CommandRecord[] = [];
  constructor(private readonly transport: OfflineConsole<SessionShape>) {}
  add(value: CommandRecord): CommandBatch { this.records.push(value); return this; }
  submit(): Promise<CommandAck> { return this.transport.submit(this.records); }
}

class EffectControl<D extends EffectDecl> implements OfflineEffectConsole<D> {
  readonly declaration: D;
  readonly slotId: string;
  constructor(private readonly transport: OfflineConsole<SessionShape>, private readonly address: EffectAddress, declaration: D) {
    this.declaration = declaration;
    this.slotId = address.slotId;
  }

  set(params: EffectParamValues<D["effectId"]>): Promise<CommandAck> {
    const validated = validateEffect(this.address.effectId, params as EffectParamValues<EffectId>);
    const effectDescriptor = descriptor(this.address.effectId);
    const records: CommandRecord[] = [];
    for (const [name, input] of Object.entries(validated.parameters)) {
      const parameter = effectDescriptor.parameters.find((candidate) => candidate.name === name);
      if (!parameter) throw new MisoCommandError("Unknown effect parameter", `parameters.${name}`);
      for (const lane of lanes(input)) records.push(record({
        kind: kind.effectParam,
        rack: rackByte[this.address.rack],
        channel: channelByte[parameter.channelPolicyName === "shared" ? "both" : lane.channel],
        trackIndex: this.address.trackIndex,
        effectIndex: this.address.rackIndex,
        parameterId: parameter.id,
        values: [scalarValue(effectDescriptor, name, lane.value), 0, 0, 0],
      }));
    }
    if (records.length === 0) throw new MisoCommandError("Effect set requires at least one parameter", "parameters");
    return this.transport.submit(records);
  }

  bypass(on: boolean): Promise<CommandAck> {
    if (typeof on !== "boolean") throw new MisoCommandError("Effect bypass requires a boolean", "effect.bypass");
    return this.transport.submit([record({ kind: kind.effectBypass, rack: rackByte[this.address.rack], trackIndex: this.address.trackIndex, effectIndex: this.address.rackIndex, values: [on ? 1 : 0, 0, 0, 0] })]);
  }

  observe(tap: string | number, options: Readonly<{ windowBlocks?: number }> = {}): Promise<ObservationHandle> {
    return this.transport.observe({ trackId: this.transport.trackIds[this.address.trackIndex], rack: this.address.rack, effectIndex: this.address.rackIndex, tap, windowBlocks: options.windowBlocks });
  }
}

class TrackControl<E extends readonly EffectDecl[]> implements OfflineTrackConsole<E> {
  constructor(
    private readonly transport: OfflineConsole<SessionShape>,
    private readonly trackIndex: number,
    private readonly placed: readonly PlacedEffect[],
    private readonly declarations: E,
  ) {}

  fader(db: number, options: Readonly<{ channel?: "left" | "right" | "both"; smoothingSamples?: number }> = {}): Promise<CommandAck> {
    return this.transport.submit([record({ kind: kind.faderDb, channel: channelByte[options.channel ?? "both"], trackIndex: this.trackIndex, smoothingSamples: options.smoothingSamples ?? 0, values: [localF32(db, "fader.db", -144, 24), 0, 0, 0] })]);
  }

  mute(on: boolean, options: Readonly<{ channel?: "left" | "right" | "both"; smoothingSamples?: number }> = {}): Promise<CommandAck> {
    if (typeof on !== "boolean") throw new MisoCommandError("Mute requires a boolean", "mute.on");
    return this.transport.submit([record({ kind: kind.mute, channel: channelByte[options.channel ?? "both"], trackIndex: this.trackIndex, smoothingSamples: options.smoothingSamples ?? 0, values: [on ? 1 : 0, 0, 0, 0] })]);
  }

  pan(left: number, right: number, options: Readonly<{ smoothingSamples?: number }> = {}): Promise<CommandAck> {
    return this.transport.submit([record({ kind: kind.pan, trackIndex: this.trackIndex, smoothingSamples: options.smoothingSamples ?? 0, values: [localF32(left, "pan.left", -1, 1), localF32(right, "pan.right", -1, 1), 0, 0] })]);
  }

  effect<I extends import("../core/types.js").Indices<E> & keyof E>(index: I): OfflineEffectConsole<Extract<E[I], EffectDecl>> {
    const placed = this.placed[index as unknown as number];
    const declaration = this.declarations[index] as Extract<E[I], EffectDecl>;
    if (!placed || !declaration) throw new MisoCommandError("Effect index is outside this track tuple", `effects[${String(index)}]`);
    return new EffectControl(this.transport, { trackIndex: this.trackIndex, rack: placed.rack, rackIndex: placed.rackIndex, effectId: placed.effectId, slotId: placed.id }, declaration);
  }
}

export class OfflineConsole<S extends SessionShape> implements EngineConsole<S> {
  readonly trackIds: readonly string[];
  private readonly trackIndexes: ReadonlyMap<string, number>;
  private readonly plan?: SessionPlan<S>;

  constructor(private readonly boundary: WasmBoundary, plan?: SessionPlan<S>) {
    this.plan = plan;
    const ids: string[] = [];
    const idBuffer = boundary.buffer("sourceId");
    const count = boundary.exports.miso_engine_web_v1_console_track_count(boundary.handle);
    for (let index = 0; index < count; index += 1) {
      const length = boundary.exports.miso_engine_web_v1_console_track_id(boundary.handle, index);
      if (length === 0 || length > idBuffer.capacity) throw new MisoOfflineError("Invalid console track map", "prepare", 255);
      ids.push(new TextDecoder("ascii", { fatal: true }).decode(new Uint8Array(boundary.exports.memory.buffer, idBuffer.pointer, length)));
    }
    this.trackIds = Object.freeze(ids);
    this.trackIndexes = new Map(ids.map((id, index) => [id, index]));
  }

  async sessionMap(): Promise<Readonly<{ tracks: readonly string[] }>> { return Object.freeze({ tracks: this.trackIds }); }

  track<Id extends keyof S & string>(id: Id): OfflineTrackConsole<S[Id]> {
    const trackIndex = this.trackIndexes.get(id);
    if (trackIndex === undefined) throw new MisoCommandError("Unknown track", `tracks.${id}`);
    const summary = this.plan?.tracks.find((track) => track.id === id);
    const effects = summary?.effects ?? [];
    const declared = effects.map((placed) => placed.declaration) as unknown as S[Id];
    return new TrackControl(this as unknown as OfflineConsole<SessionShape>, trackIndex, effects, declared);
  }

  batch(): CommandBatch { return new Batch(this as unknown as OfflineConsole<SessionShape>); }

  async submit(records: readonly CommandRecord[]): Promise<CommandAck> {
    this.boundary.assertLive("lifecycle");
    const encoded = encodeCommandBatch(records);
    const staging = this.boundary.buffer("command");
    if (staging.pointer !== 0 && encoded.records.byteLength <= staging.capacity) {
      new Uint8Array(this.boundary.exports.memory.buffer, staging.pointer, encoded.records.byteLength).set(encoded.records);
    }
    const result = this.boundary.exports.miso_engine_web_v1_command_submit(this.boundary.handle, encoded.count);
    const pointer = this.boundary.exports.miso_engine_web_v1_command_report_ptr(this.boundary.handle);
    const view = new DataView(this.boundary.exports.memory.buffer, pointer, ABI_LAYOUT.structures.commandReport.bytes);
    if (view.getUint32(reportOffsets.structSize, true) !== ABI_LAYOUT.structures.commandReport.bytes
        || view.getUint32(reportOffsets.abiVersion, true) !== ABI_LAYOUT.abiVersion
        || view.getBigUint64(reportOffsets.reserved, true) !== 0n
        || view.getBigUint64(reportOffsets.reserved + 8, true) !== 0n) throw new MisoOfflineError("Invalid command report", "lifecycle", 255);
    const reason = reasonByValue.get(view.getUint32(reportOffsets.reason, true)) ?? "malformed";
    const raw = Object.freeze({
      result,
      reason: view.getUint32(reportOffsets.reason, true),
      rejectedIndex: view.getUint32(reportOffsets.rejectedIndex, true),
      admitted: view.getUint32(reportOffsets.admitted, true),
      appliedAtSample: view.getBigUint64(reportOffsets.appliedAtSample, true),
    });
    const ack: CommandAck = Object.freeze({
      ok: result === 0 && reason === "none",
      reason,
      rejectedIndex: raw.rejectedIndex,
      admitted: raw.admitted,
      appliedAtSample: raw.appliedAtSample,
      explain: reason === "none" ? "The complete command transaction was admitted for the next render block." : `The engine refused the command transaction: ${reason}.`,
      raw,
    });
    return ack;
  }

  setParam(address: ParamAddress, value: number): Promise<CommandAck> {
    const trackIndex = this.trackIndexes.get(address.trackId) ?? 0xffff_ffff;
    const summary = this.plan?.tracks.find((track) => track.id === address.trackId)?.effects.find((candidate) => candidate.rack === address.rack && candidate.rackIndex === address.effectIndex);
    const effectDescriptor = summary ? descriptor(summary.effectId) : undefined;
    const parameter = typeof address.parameter === "number" ? address.parameter : effectDescriptor?.parameters.find((candidate) => candidate.name === address.parameter)?.id ?? 0;
    return this.submit([record({ kind: kind.effectParam, rack: rackByte[address.rack], channel: channelByte[address.channel ?? "both"], trackIndex, effectIndex: address.effectIndex, parameterId: parameter, smoothingSamples: address.smoothingSamples ?? 0, values: [value, 0, 0, 0] })]);
  }

  async observe(address: ObservationAddress): Promise<ObservationHandle> {
    const trackIndex = this.trackIndexes.get(address.trackId) ?? 0xffff_ffff;
    const summary = this.plan?.tracks.find((track) => track.id === address.trackId)?.effects.find((candidate) => candidate.rack === address.rack && candidate.rackIndex === address.effectIndex);
    const effectDescriptor = summary ? descriptor(summary.effectId) : undefined;
    const tapId = typeof address.tap === "number" ? address.tap : effectDescriptor?.observations.find((candidate) => candidate.name === address.tap)?.id ?? 0;
    const command = (commandKind: number) => record({ kind: commandKind, rack: rackByte[address.rack], trackIndex, effectIndex: address.effectIndex, parameterId: tapId, smoothingSamples: address.windowBlocks ?? 0 });
    const ack = await this.submit([command(kind.observeSubscribe)]);
    return Object.freeze({ ack, close: () => this.submit([command(kind.observeUnsubscribe)]) });
  }
}
