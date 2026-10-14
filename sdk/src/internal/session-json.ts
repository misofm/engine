import type { ModelRecord, ModelValue, SessionModel } from "../core/session.ts";
import { MisoUsageError } from "../core/errors.ts";
import { SESSION_KEYS } from "../generated/session-keys.ts";
import type { SessionKeyKind } from "../generated/session-keys.ts";

/**
 * Internal canonical Session V1 serializer shared by `SessionBuilder.toJson()` and its corpus
 * tests. This module is deliberately absent from every package export map and public barrel:
 * callers author through the validated builder rather than serializing arbitrary object models.
 */
export function writeCanonicalSessionDocument(model: SessionModel): string {
  return `${jsonValue(model, "", 0)}\n`;
}

/**
 * One record shape from the engine's key tables: its keys in emit order and what each key holds.
 *
 * `src/generated/session-keys.ts` is the Rust session field walk, recorded over the canonical
 * writer corpus (`parameter-metadata --print-session-keys`). The writer holds no key list, order
 * or number spelling of its own: a field added in Rust reaches this module by regeneration.
 */
interface RecordShape {
  readonly name: string;
  readonly order: readonly string[];
  /** The sorted key names joined, to compare against a model record's own keys. */
  readonly keySet: string;
  readonly kinds: ReadonlyMap<string, SessionKeyKind>;
}

/** Every record shape that may sit under a parent JSON key (`""` for the root). */
const SHAPES_BY_PARENT: ReadonlyMap<string, readonly RecordShape[]> = (() => {
  const byName = new Map<string, RecordShape>();
  for (const record of SESSION_KEYS.records) {
    const name = record.arm === null ? record.module : `${record.module}/${record.arm}`;
    const order = record.keys.map(([key]) => key as string);
    byName.set(name, {
      name,
      order,
      keySet: [...order].sort().join(","),
      kinds: new Map(record.keys.map(([key, kind]) => [key as string, kind as SessionKeyKind])),
    });
  }
  const byParent = new Map<string, RecordShape[]>();
  for (const parent of SESSION_KEYS.parents) {
    byParent.set(parent.key, parent.records.map((entry) => {
      const name = entry.arm === null ? entry.module : `${entry.module}/${entry.arm}`;
      const shape = byName.get(name);
      if (shape === undefined) throw new Error(`session key tables name no record ${name}`);
      return shape;
    }));
  }
  return byParent;
})();

/**
 * The shape of a record under the parent key: the one candidate whose key set equals the
 * record's exactly. Several candidates may share a key set only when the generator proved they
 * write the same order and kinds, so the first answers for all of them.
 */
function recordShape(record: ModelRecord, parentKey: string): RecordShape {
  const candidates = SHAPES_BY_PARENT.get(parentKey);
  if (candidates === undefined) {
    throw new MisoUsageError(`the canonical writer has no declared object shape for '${parentKey}'`);
  }
  const keySet = Object.keys(record).sort().join(",");
  const shape = candidates.find((candidate) => candidate.keySet === keySet);
  if (shape === undefined) {
    throw new MisoUsageError(
      `the canonical writer received the wrong keys for '${parentKey || "session"}': ${keySet.split(",").join(", ")}`,
    );
  }
  return shape;
}

function jsonValue(
  value: ModelValue,
  key: string,
  depth: number,
  kind: SessionKeyKind | undefined = undefined,
): string {
  if (typeof value === "string") return quote(value);
  if (typeof value === "boolean") return value ? "true" : "false";
  if (typeof value === "number") {
    if (kind === "integer") return String(value);
    if (kind === "float") return canonicalFloat(value);
    throw new MisoUsageError(
      `the canonical writer has no declared JSON type for the numeric key '${key}'`,
    );
  }
  if (Array.isArray(value)) {
    if (value.length === 0) return "[]";
    const indent = "  ".repeat(depth + 1);
    return `[\n${value.map((item) => `${indent}${jsonValue(item, key, depth + 1, kind)}`).join(",\n")}\n${"  ".repeat(depth)}]`;
  }
  const record = value as ModelRecord;
  const shape = recordShape(record, key);
  if (shape.order.length === 0) return "{}";
  const indent = "  ".repeat(depth + 1);
  return `{\n${shape.order.map((name) => `${indent}${quote(name)}: ${jsonValue(record[name]!, name, depth + 1, shape.kinds.get(name))}`).join(",\n")}\n${"  ".repeat(depth)}}`;
}

/** The canonical writer's escape set, from the Rust authority's `write_quoted`. */
function quote(value: string): string {
  let out = '"';
  for (const character of value) {
    const code = character.codePointAt(0) ?? 0;
    if (character === '"') out += '\\"';
    else if (character === "\\") out += "\\\\";
    else if (character === "\b") out += "\\b";
    else if (character === "\t") out += "\\t";
    else if (character === "\n") out += "\\n";
    else if (character === "\f") out += "\\f";
    else if (character === "\r") out += "\\r";
    else if (code <= 0x1f || (code >= 0x7f && code <= 0x9f)) {
      out += code <= 0xffff
        ? `\\u${code.toString(16).padStart(4, "0").toUpperCase()}`
        : character;
    } else out += character;
  }
  return `${out}"`;
}

function f32Bits(value: number): number {
  const view = new DataView(new ArrayBuffer(4));
  view.setFloat32(0, value, true);
  return view.getUint32(0, true);
}

/** The two finite values whose shortest direct-f32 spelling double-rounds through JavaScript. */
const DOUBLE_ROUNDING_SPELLINGS: ReadonlyMap<number, string> = new Map([
  [0x15ae_43fd, "0.00000000000000000000000007038530691851209"],
  [0x95ae_43fd, "-0.00000000000000000000000007038530691851209"],
]);

function canonicalFloat(value: number): string {
  const normalized = Math.fround(value);
  if (!Number.isFinite(normalized)) {
    throw new MisoUsageError("the canonical writer cannot spell a non-finite f32");
  }
  if (Object.is(normalized, -0)) return "-0.0";
  const bits = f32Bits(normalized);
  const exact = DOUBLE_ROUNDING_SPELLINGS.get(bits);
  if (exact !== undefined) return exact;
  let text: string | undefined;
  for (let precision = 1; precision <= 9 && text === undefined; precision += 1) {
    const candidate = normalized.toPrecision(precision);
    if (f32Bits(Number(candidate)) === bits) text = candidate;
  }
  if (text === undefined) {
    throw new MisoUsageError(`no canonical f32 spelling round-trips ${normalized}`);
  }
  const decimal = /[eE]/.test(text) ? expandExponent(text) : text;
  return decimal.includes(".") ? decimal : `${decimal}.0`;
}

/** Rust's float `Display` never uses exponent notation, so neither may the canonical writer. */
function expandExponent(text: string): string {
  const [coefficient = "0", exponentText = "0"] = text.toLowerCase().split("e");
  const exponent = Number(exponentText);
  const negative = coefficient.startsWith("-");
  const unsigned = negative ? coefficient.slice(1) : coefficient;
  const dot = unsigned.indexOf(".");
  const digits = unsigned.replace(".", "");
  const point = (dot < 0 ? unsigned.length : dot) + exponent;
  const body = point <= 0
    ? `0.${"0".repeat(-point)}${digits}`
    : point >= digits.length
      ? `${digits}${"0".repeat(point - digits.length)}`
      : `${digits.slice(0, point)}.${digits.slice(point)}`;
  return negative ? `-${body}` : body;
}
