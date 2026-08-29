import { ABI_LAYOUT } from "../generated/abi.ts";
import type { AbiStructureName } from "../generated/abi.ts";
import { MisoUsageError } from "./errors.ts";

/**
 * Structure access by field *name*, resolved through the generated layout.
 *
 * There is no numeric offset anywhere in this SDK. Every read and write below looks its field up
 * in `ABI_LAYOUT`, which came from Rust `offset_of!`, so a field that moves moves every access
 * with it and a field that is renamed or deleted fails loudly at the lookup instead of quietly at
 * the wrong address. That is the whole lesson of issue #207's N-13(d): the fifth hand-written copy
 * of the configuration table wrote a 192-byte struct's offsets into a 64-byte buffer, and nothing
 * noticed because a wrong offset is still a valid address.
 */

type Field = { readonly name: string; readonly offset: number; readonly type: string };

function fields(structure: AbiStructureName): readonly Field[] {
  return ABI_LAYOUT.structures[structure].fields;
}

function field(structure: AbiStructureName, name: string): Field {
  const found = fields(structure).find((row) => row.name === name);
  if (found === undefined) {
    throw new MisoUsageError(
      `the generated ABI layout has no ${structure}.${name}; the engine's structure changed`,
    );
  }
  return found;
}

/** Byte size of a structure, from the generated layout. */
export function structBytes(structure: AbiStructureName): number {
  return ABI_LAYOUT.structures[structure].bytes;
}

/** Every field name of a structure, in layout order. */
export function structFieldNames(structure: AbiStructureName): readonly string[] {
  return fields(structure).map((row) => row.name);
}

function expect(found: Field, structure: AbiStructureName, name: string, type: string): Field {
  if (found.type !== type) {
    throw new MisoUsageError(
      `${structure}.${name} is ${found.type} in the generated layout, read as ${type}`,
    );
  }
  return found;
}

/** A reader over one engine structure at one address. */
export class StructView {
  readonly #view: DataView;
  readonly #structure: AbiStructureName;

  constructor(memory: WebAssembly.Memory, structure: AbiStructureName, pointer: number) {
    if (pointer === 0) {
      throw new MisoUsageError(`the engine returned a null ${structure} pointer`);
    }
    this.#view = new DataView(memory.buffer, pointer, structBytes(structure));
    this.#structure = structure;
  }

  u32(name: string): number {
    return this.#view.getUint32(expect(field(this.#structure, name), this.#structure, name, "u32").offset, true);
  }

  u64(name: string): bigint {
    return this.#view.getBigUint64(expect(field(this.#structure, name), this.#structure, name, "u64").offset, true);
  }

  /** Every `u32`/`u64` field as a plain object, for equality assertions and diagnostics. */
  snapshot(): Readonly<Record<string, number | bigint | readonly number[]>> {
    const out: Record<string, number | bigint | readonly number[]> = {};
    for (const row of fields(this.#structure)) {
      if (row.type === "u32") out[row.name] = this.#view.getUint32(row.offset, true);
      else if (row.type === "u64") out[row.name] = this.#view.getBigUint64(row.offset, true);
      else if (row.type.startsWith("u64[")) {
        const count = Number(row.type.slice(4, -1));
        out[row.name] = Object.freeze(
          Array.from({ length: count }, (_unused, index) =>
            Number(this.#view.getBigUint64(row.offset + index * 8, true))),
        );
      } else if (row.type.startsWith("u32[")) {
        const count = Number(row.type.slice(4, -1));
        out[row.name] = Object.freeze(
          Array.from({ length: count }, (_unused, index) =>
            this.#view.getUint32(row.offset + index * 4, true)),
        );
      }
    }
    return Object.freeze(out);
  }
}

/**
 * The seven boot policy words a caller may set, plus the two handshake words the SDK owns.
 *
 * `PrepareLimits`' 25 public fields are gone with the 192-byte structure they configured. What
 * remains is what boot v2 actually reads, and every one of these is optional: absent means zero,
 * and zero means "the engine's own default", never a number the SDK invented. In particular
 * `maximumMemoryBytes` absent selects `DEFAULT_MAXIMUM_MEMORY_BYTES`, which the SDK names but
 * never restates (issue #243 S2(c)).
 */
export interface BootOptions {
  /** Refuse unless the document's rate is this. Zero/absent accepts the document's own. */
  readonly requireSampleRateHz?: number;
  /** Refuse unless the document's quantum is this. Zero/absent accepts the document's own. */
  readonly requireQuantumFrames?: number;
  /** Per-source ring override. Absent selects the engine's stall-tolerance derivation. */
  readonly sourceRingFrames?: number;
  /** Total boot memory budget. Absent selects the engine's default ceiling. */
  readonly maximumMemoryBytes?: bigint;
  /** The four console words. Absent attaches no console at all. */
  readonly console?: {
    readonly commandQueueRecords?: number;
    readonly meterBlocks?: number;
    readonly observationTaps?: number;
    readonly masterTrackPlusOne?: number;
  };
}

/**
 * The words that must be **identical** between a browser scratch boot and its worklet boot.
 *
 * Adopted ruling 5462139867 finding 3: "identical options struct" was literally impossible, since
 * the scratch boot writes `require_* = 0` while the worklet writes the physical rate and quantum.
 * The divergence class A-1 actually named was the *console* words, so the equality rule is stated
 * over the policy words and the two `require_*` words are role-defined.
 */
export const POLICY_WORDS = Object.freeze([
  "sourceRingFrames",
  "maximumMemoryBytes",
  "consoleCommandQueueRecords",
  "consoleMeterBlocks",
  "consoleObservationTaps",
  "consoleMasterTrackPlusOne",
] as const);

/** The two words whose values are defined by the boot's role rather than shared. */
export const ROLE_DEFINED_WORDS = Object.freeze([
  "requireSampleRateHz",
  "requireQuantumFrames",
] as const);

/**
 * Write the boot options block in place, by field name.
 *
 * The two handshake words are always written explicitly. The ABI accepts an all-zero block as
 * "defaults", but writing the pair is what turns a layout skew into a typed `refusedOptions`
 * instead of an accepted misreading: if either word is nonzero, both must name this exact layout
 * and version. That is the second of the three skew detectors (issue #243 S2(e)).
 */
export function writeBootOptions(
  memory: WebAssembly.Memory,
  pointer: number,
  options: BootOptions,
): Uint8Array {
  const bytes = structBytes("bootOptions");
  if (pointer === 0) {
    throw new MisoUsageError("the engine returned a null boot options pointer");
  }
  const block = new Uint8Array(memory.buffer, pointer, bytes);
  block.fill(0);
  const view = new DataView(memory.buffer, pointer, bytes);
  const u32 = (name: string, value: number): void => {
    view.setUint32(expect(field("bootOptions", name), "bootOptions", name, "u32").offset, value, true);
  };
  const u64 = (name: string, value: bigint): void => {
    view.setBigUint64(expect(field("bootOptions", name), "bootOptions", name, "u64").offset, value, true);
  };

  u32("structSize", bytes);
  u32("abiVersion", ABI_LAYOUT.abiVersion);
  u32("requireSampleRateHz", options.requireSampleRateHz ?? 0);
  u32("requireQuantumFrames", options.requireQuantumFrames ?? 0);
  u32("sourceRingFrames", options.sourceRingFrames ?? 0);
  u64("maximumMemoryBytes", options.maximumMemoryBytes ?? 0n);
  u64("consoleCommandQueueRecords", BigInt(options.console?.commandQueueRecords ?? 0));
  u64("consoleMeterBlocks", BigInt(options.console?.meterBlocks ?? 0));
  u64("consoleObservationTaps", BigInt(options.console?.observationTaps ?? 0));
  u64("consoleMasterTrackPlusOne", BigInt(options.console?.masterTrackPlusOne ?? 0));

  // A detached copy, so a caller can compare two boots' option blocks after the fact without
  // holding a view onto wasm memory that a later `document_ptr` growth could detach.
  return block.slice();
}

/**
 * Derive the per-source ring the engine will pick for a shape, from the *published rule*.
 *
 * The effective ring is not readable back across the ABI -- it is an input word and no export
 * reports it -- so a producer that must size itself applies the rule. Both of its inputs come from
 * the generated document, so this function holds no private copy of `100` or of `2`.
 */
export function defaultSourceRingFrames(sampleRateHz: number, quantumFrames: number): number {
  if (quantumFrames === 0) return 0;
  const { stallToleranceMs, reserveQuanta } = ABI_LAYOUT.constants.sourceRing;
  const stallFrames = Math.floor((sampleRateHz * stallToleranceMs) / 1000);
  const quanta = Math.ceil(stallFrames / quantumFrames) + reserveQuanta;
  return quanta * quantumFrames;
}

/** Numeric value of a named constant group row, e.g. `constantValue("bufferKinds", "command")`. */
export function constantValue(
  group: "resultCodes" | "bootResultAliases" | "states" | "backends" | "bufferKinds"
    | "wireCommandKinds" | "commandReasons",
  name: string,
): number {
  const row = ABI_LAYOUT.constants[group].find((entry) => entry.name === name);
  if (row === undefined) {
    throw new MisoUsageError(`the generated ABI layout has no ${group}.${name}`);
  }
  return row.value;
}
