import { ABI_LAYOUT } from "../generated/abi.js";
import { CATALOG } from "../generated/catalog.js";
import { MisoCommandError } from "./errors.js";

export interface CommandRecord {
  readonly kind: number;
  readonly rack: number;
  readonly channel: number;
  readonly trackIndex: number;
  readonly effectIndex: number;
  readonly parameterId: number;
  readonly smoothingSamples: number;
  readonly values: readonly [number, number, number, number];
}

export interface EncodedCommandBatch {
  readonly records: Uint8Array;
  readonly count: number;
}

const offsets = Object.freeze(Object.fromEntries(
  ABI_LAYOUT.commandRecord.fields.map((field) => [field.name, field.offset]),
)) as Readonly<Record<string, number>>;
const wireKinds: ReadonlySet<number> = new Set(ABI_LAYOUT.constants.wireCommandKinds.map((row) => row.value));
const recordBytes = ABI_LAYOUT.commandRecord.bytes;
const maximumRecords = ABI_LAYOUT.constants.maximumCommandRecords;

function u32(value: number, path: string): number {
  if (!Number.isSafeInteger(value) || value < 0 || value > 0xffff_ffff) {
    throw new MisoCommandError("expected a u32", path);
  }
  return value;
}

function byte(value: number, path: string): number {
  if (!Number.isSafeInteger(value) || value < 0 || value > 0xff) {
    throw new MisoCommandError("expected a byte", path);
  }
  return value;
}

function f32(value: number, path: string): number {
  const rounded = Math.fround(value);
  if (!Number.isFinite(value) || !Number.isFinite(rounded)) {
    throw new MisoCommandError("command values must be finite f32 values", path);
  }
  return rounded;
}

/** Encode frozen `miso.command.v1` records from generated ABI layout data only. */
export function encodeCommandBatch(commands: readonly CommandRecord[]): EncodedCommandBatch {
  if (commands.length === 0 || commands.length > maximumRecords) {
    throw new MisoCommandError(`expected 1..${maximumRecords} command records`, "$.commands");
  }
  const records = new Uint8Array(commands.length * recordBytes);
  const view = new DataView(records.buffer);
  for (const [index, command] of commands.entries()) {
    const path = `$.commands[${index}]`;
    if (!Array.isArray(command.values) || command.values.length !== 4) {
      throw new MisoCommandError("command values must contain exactly four f32 slots", `${path}.values`);
    }
    if (!wireKinds.has(command.kind)) throw new MisoCommandError("unknown wire command kind", `${path}.kind`);
    view.setUint8(index * recordBytes + offsets.kind, byte(command.kind, `${path}.kind`));
    view.setUint8(index * recordBytes + offsets.rack, byte(command.rack, `${path}.rack`));
    view.setUint8(index * recordBytes + offsets.channel, byte(command.channel, `${path}.channel`));
    view.setUint32(index * recordBytes + offsets.trackIndex, u32(command.trackIndex, `${path}.trackIndex`), true);
    view.setUint32(index * recordBytes + offsets.effectIndex, u32(command.effectIndex, `${path}.effectIndex`), true);
    view.setUint32(index * recordBytes + offsets.parameterId, u32(command.parameterId, `${path}.parameterId`), true);
    view.setUint32(index * recordBytes + offsets.smoothingSamples, u32(command.smoothingSamples, `${path}.smoothingSamples`), true);
    for (const [slot, value] of command.values.entries()) {
      view.setFloat32(
        index * recordBytes + offsets.values + slot * Float32Array.BYTES_PER_ELEMENT,
        f32(value, `${path}.values[${slot}]`),
        true,
      );
    }
  }
  return Object.freeze({ records, count: commands.length });
}

/** Generated metadata is intentionally referenced so the encoder cannot grow a private vocabulary. */
export const COMMAND_REASON_NAMES = Object.freeze(CATALOG.commandReasons.map((reason) => reason.name));
