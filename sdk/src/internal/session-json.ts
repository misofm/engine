import type { ModelRecord, ModelValue, SessionModel } from "../core/session.ts";
import { MisoUsageError } from "../core/errors.ts";

/**
 * Internal canonical Session V1 serializer shared by `SessionBuilder.toJson()` and its corpus
 * tests. This module is deliberately absent from every package export map and public barrel:
 * callers author through the validated builder rather than serializing arbitrary object models.
 */
export function writeCanonicalSessionDocument(model: SessionModel): string {
  return `${jsonValue(model, "", 0)}\n`;
}

const ROOT_KEYS = [
  "schema_version",
  "session_id",
  "revision",
  "sample_rate_hz",
  "quantum_frames",
  "render_profile",
  "output_profile",
  "sources",
  "tracks",
  "submixes",
  "outputs",
  "routes",
  "automation",
] as const;

/**
 * Every numeric leaf key in the schema, by JSON schema type.
 *
 * A number in the model does not say whether the schema calls it a `u32` or an `f32`, and the two
 * spell differently: `0` versus `0.0`. Every leaf is therefore transcribed from the Rust field
 * walk, and an unlisted numeric leaf fails loudly.
 */
const INTEGER_KEYS: ReadonlySet<string> = new Set([
  "schema_version",
  "sample_rate_hz",
  "quantum_frames",
  "channels",
  "bit_depth",
  "left_source_channel",
  "right_source_channel",
  "delay_samples",
  "smoothing_samples",
  "parameter_id",
]);
const FLOAT_KEYS: ReadonlySet<string> = new Set([
  "trim_db",
  "hpf_hz",
  "lpf_hz",
  "left_db",
  "right_db",
  "left",
  "right",
  "ll",
  "lr",
  "rl",
  "rr",
  "gain_db",
  "value",
  "start_value",
  "end_value",
]);

const OBJECT_KEY_ORDERS = {
  render_profile: ["id", "mode"],
  output_profile: ["id", "channels", "sample_format"],
  sources: ["id", "content", "channels", "bit_depth", "frames"],
  builtins: ["left", "right"],
  channel_builtins: ["polarity_invert", "trim_db", "hpf_hz", "lpf_hz", "delay_samples"],
  rack: ["effects"],
  effects: ["id", "identity", "quality", "bypass", "link_mode", "params", "sidechain"],
  params: ["parameter_id", "channel", "unit", "value"],
  fader: ["left_db", "right_db", "left_mute", "right_mute"],
  pan: ["left", "right", "smoothing_samples"],
  matrix: ["ll", "lr", "rl", "rr", "smoothing_samples"],
  submixes: ["id"],
  outputs: ["id"],
  routes: ["id", "source", "destination", "channel_matrix", "gain_db"],
  channel_matrix: ["ll", "lr", "rl", "rr"],
  automation: ["id", "target", "segments"],
  target: ["entity_id", "rack", "effect_id", "parameter_id", "channel"],
  segments: ["shape", "start_sample", "end_sample", "start_value", "end_value", "unit"],
} as const;

function taggedOrder(record: ModelRecord, key: string): readonly string[] {
  const kind = record.kind;
  if (key === "identity") {
    if (kind === "native") return ["kind", "effect_id"];
    if (kind === "cid") return ["kind", "cid"];
  }
  if (key === "sidechain") {
    if (kind === "none") return ["kind"];
    if (kind === "routed") return ["kind", "source", "port_id"];
  }
  if (key === "source") {
    if (kind === "track") return ["kind", "track_id", "tap"];
    if (kind === "submix_output") return ["kind", "submix_id"];
  }
  if (key === "destination") {
    if (kind === "submix_input") return ["kind", "submix_id"];
    if (kind === "output_input") return ["kind", "output_id"];
  }
  throw new MisoUsageError(`the canonical writer has no declared ${key} variant '${String(kind)}'`);
}

function objectOrder(record: ModelRecord, key: string): readonly string[] {
  if (key === "") return ROOT_KEYS;
  if (key === "tracks") {
    return [
      "id", "source_id", "left_source_channel", "right_source_channel", "builtins",
      "simd1", "dynamic", "simd2", "fader", "pan" in record ? "pan" : "matrix",
    ];
  }
  if (key === "left" || key === "right") return OBJECT_KEY_ORDERS.channel_builtins;
  if (key === "simd1" || key === "dynamic" || key === "simd2") return OBJECT_KEY_ORDERS.rack;
  if (["identity", "sidechain", "source", "destination"].includes(key)) {
    return taggedOrder(record, key);
  }
  const order = OBJECT_KEY_ORDERS[key as keyof typeof OBJECT_KEY_ORDERS];
  if (order !== undefined) return order;
  throw new MisoUsageError(`the canonical writer has no declared object shape for '${key}'`);
}

function checkedObjectOrder(record: ModelRecord, key: string): readonly string[] {
  const order = objectOrder(record, key);
  const actual = Object.keys(record).sort();
  const expected = [...order].sort();
  if (actual.length !== expected.length || actual.some((name, index) => name !== expected[index])) {
    throw new MisoUsageError(
      `the canonical writer received the wrong keys for '${key || "session"}': ${actual.join(", ")}`,
    );
  }
  return order;
}

function jsonValue(value: ModelValue, key: string, depth: number): string {
  if (typeof value === "string") return quote(value);
  if (typeof value === "boolean") return value ? "true" : "false";
  if (typeof value === "number") {
    if (INTEGER_KEYS.has(key)) return String(value);
    if (FLOAT_KEYS.has(key)) return canonicalFloat(value);
    throw new MisoUsageError(
      `the canonical writer has no declared JSON type for the numeric key '${key}'`,
    );
  }
  if (Array.isArray(value)) {
    if (value.length === 0) return "[]";
    const indent = "  ".repeat(depth + 1);
    return `[\n${value.map((item) => `${indent}${jsonValue(item, key, depth + 1)}`).join(",\n")}\n${"  ".repeat(depth)}]`;
  }
  const record = value as ModelRecord;
  const keys = checkedObjectOrder(record, key);
  if (keys.length === 0) return "{}";
  const indent = "  ".repeat(depth + 1);
  return `{\n${keys.map((name) => `${indent}${quote(name)}: ${jsonValue(record[name]!, name, depth + 1)}`).join(",\n")}\n${"  ".repeat(depth)}}`;
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
