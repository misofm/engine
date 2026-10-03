import type { EffectId } from "../generated/catalog.ts";
import { MisoUsageError } from "../core/errors.ts";
import { effect, session } from "../core/session.ts";
import type { SessionBuilder } from "../core/session.ts";
import type {
  AutomationSpec,
  ConsoleEntrySpec,
  ConsoleSlotSpec,
  ConsoleSpec,
  EffectOptions,
  RouteDestination,
  RouteSource,
  RouteSpec,
  SourceSpec,
  SubmixSpec,
  TrackSpec,
} from "../core/types.ts";

/**
 * One `enginectl session build` request.
 *
 * `console` is the session console (`{ preInsert, postInsert }` of `{ slot, effectId, quality?,
 * linkMode? }`), and each track spec carries its `console` entries (`{ slot, bypass?, parameters?,
 * channel? }`, one per slot in slot order) and its `inserts` (effect declarations). The retired
 * `simd1`, `dynamic` and `simd2` track keys are refused by name. A `submixes` entry is a bare ID,
 * the transparent strip, or `{ id, builtins?, console?, inserts?, fader?, pan? }`, a submix strip
 * with a track's strip keys. A route or sidechain source is `{ kind: "track", trackId, tap }` or
 * `{ kind: "submix", submixId, tap }`; the retired `submix_output` is refused by name.
 */
export interface SessionBuildRequestV1 {
  readonly schemaVersion: 1;
  readonly session: {
    readonly id: string;
    readonly sampleRateHz: number;
    readonly revision?: number | string;
    readonly quantumFrames?: number;
  };
  readonly sources?: readonly unknown[];
  readonly console?: unknown;
  readonly tracks?: readonly unknown[];
  readonly submixes?: readonly unknown[];
  readonly outputs?: readonly unknown[];
  readonly routes?: readonly unknown[];
  readonly automation?: readonly unknown[];
}

function fail(path: string, message: string): never {
  throw new TypeError(`${path}: ${message}`);
}

function record(value: unknown, path: string): Record<string, unknown> {
  if (value === null || typeof value !== "object" || Array.isArray(value)) {
    fail(path, "expected an object");
  }
  return value as Record<string, unknown>;
}

/** The engine's code for a key its schema does not have, where the request mirrors one of its objects. */
const UNKNOWN_FIELD = "schema.unknown_field";

/**
 * Refuse a request key outside `allowed`, and a missing required one.
 *
 * `unknownFieldCode` is given where the request object mirrors a session object the engine refuses
 * an unknown member of (a console slot, a console entry, a track): the refusal then carries the
 * engine's `schema.unknown_field`, as the builder's does, so `enginectl` and the builder name one
 * code for one defect.
 */
function keys(
  value: Record<string, unknown>,
  allowed: readonly string[],
  required: readonly string[],
  path: string,
  unknownFieldCode?: typeof UNKNOWN_FIELD,
): void {
  const admitted = new Set(allowed);
  for (const key of Object.keys(value)) {
    if (admitted.has(key)) continue;
    if (unknownFieldCode !== undefined) {
      throw new MisoUsageError(`${path}.${key}: unknown key`, unknownFieldCode);
    }
    fail(`${path}.${key}`, "unknown key");
  }
  for (const key of required) {
    if (!Object.hasOwn(value, key)) fail(`${path}.${key}`, "required key is missing");
  }
}

function string(value: unknown, path: string): string {
  if (typeof value !== "string") fail(path, "expected a string");
  return value;
}

function number(value: unknown, path: string): number {
  if (typeof value !== "number") fail(path, "expected a number");
  return value;
}

function array(value: unknown, path: string): readonly unknown[] {
  if (!Array.isArray(value)) fail(path, "expected an array");
  return value;
}

function optionalArray(root: Record<string, unknown>, name: string): readonly unknown[] {
  return root[name] === undefined ? [] : array(root[name], `$.${name}`);
}

function matrix(value: unknown, path: string): { ll: number; lr: number; rl: number; rr: number } {
  const raw = record(value, path);
  keys(raw, ["ll", "lr", "rl", "rr"], ["ll", "lr", "rl", "rr"], path);
  return {
    ll: number(raw.ll, `${path}.ll`),
    lr: number(raw.lr, `${path}.lr`),
    rl: number(raw.rl, `${path}.rl`),
    rr: number(raw.rr, `${path}.rr`),
  };
}

function routeSource(value: unknown, path: string): RouteSource {
  const raw = record(value, path);
  const kind = string(raw.kind, `${path}.kind`);
  if (kind === "track") {
    keys(raw, ["kind", "trackId", "tap"], ["kind", "trackId", "tap"], path);
    return {
      kind,
      trackId: string(raw.trackId, `${path}.trackId`),
      tap: string(raw.tap, `${path}.tap`) as Extract<RouteSource, { kind: "track" }>["tap"],
    };
  }
  if (kind === "submix") {
    keys(raw, ["kind", "submixId", "tap"], ["kind", "submixId", "tap"], path);
    return {
      kind,
      submixId: string(raw.submixId, `${path}.submixId`),
      tap: string(raw.tap, `${path}.tap`) as Extract<RouteSource, { kind: "submix" }>["tap"],
    };
  }
  if (kind === "submix_output") {
    throw new MisoUsageError(
      `${path}.kind: 'submix_output' is retired; read a submix as { "kind": "submix", "submixId", `
        + `"tap" }, where tap "post_pan" is the old submix output`,
      "schema.invalid_enum",
    );
  }
  fail(`${path}.kind`, "expected 'track' or 'submix'");
}

function routeDestination(value: unknown, path: string): RouteDestination {
  const raw = record(value, path);
  const kind = string(raw.kind, `${path}.kind`);
  if (kind === "submix_input") {
    keys(raw, ["kind", "submixId"], ["kind", "submixId"], path);
    return { kind, submixId: string(raw.submixId, `${path}.submixId`) };
  }
  if (kind === "output_input") {
    keys(raw, ["kind", "outputId"], ["kind", "outputId"], path);
    return { kind, outputId: string(raw.outputId, `${path}.outputId`) };
  }
  fail(`${path}.kind`, "expected 'submix_input' or 'output_input'");
}

function builtins(value: unknown, path: string): unknown {
  if (Array.isArray(value)) {
    if (value.length !== 2) fail(path, "expected exactly two lane specifications");
    return value.map((lane, index) => builtinLane(lane, `${path}[${index}]`));
  }
  const raw = record(value, path);
  if (Object.hasOwn(raw, "left") || Object.hasOwn(raw, "right")) {
    keys(raw, ["left", "right"], ["left", "right"], path);
    return {
      left: builtinLane(raw.left, `${path}.left`),
      right: builtinLane(raw.right, `${path}.right`),
    };
  }
  return builtinLane(raw, path);
}

function builtinLane(value: unknown, path: string): Record<string, unknown> {
  const raw = record(value, path);
  keys(raw, ["polarityInvert", "trimDb", "hpfHz", "lpfHz", "delaySamples"], [], path);
  return raw;
}

function parameterValue(value: unknown, path: string): unknown {
  if (Array.isArray(value)) {
    if (value.length !== 2) fail(path, "a per-lane parameter array must contain two values");
    return value;
  }
  if (value !== null && typeof value === "object") {
    const raw = record(value, path);
    keys(raw, ["left", "right"], ["left", "right"], path);
  }
  return value;
}

function effectDecl(value: unknown, path: string): ReturnType<typeof effect> {
  const raw = record(value, path);
  keys(raw, ["effectId", "parameters", "options"], ["effectId"], path);
  const effectId = string(raw.effectId, `${path}.effectId`) as EffectId;
  // JSON.parse creates `__proto__` as an own data property. Assigning that member into `{}` would
  // invoke Object.prototype's legacy setter and silently remove the parameter before effect()
  // can issue its normal unknown-parameter refusal. A null-prototype record preserves every JSON
  // member, including all names inherited by ordinary objects.
  const parameters: Record<string, unknown> = Object.create(null) as Record<string, unknown>;
  if (raw.parameters !== undefined) {
    const supplied = record(raw.parameters, `${path}.parameters`);
    for (const [name, value] of Object.entries(supplied)) {
      parameters[name] = parameterValue(value, `${path}.parameters.${name}`);
    }
  }
  let options: EffectOptions = {};
  if (raw.options !== undefined) {
    const supplied = record(raw.options, `${path}.options`);
    keys(
      supplied,
      ["slotId", "bypass", "quality", "linkMode", "channel", "sidechain"],
      [],
      `${path}.options`,
    );
    let sidechain: EffectOptions["sidechain"];
    if (supplied.sidechain !== undefined) {
      const sidechainRaw = record(supplied.sidechain, `${path}.options.sidechain`);
      keys(
        sidechainRaw,
        ["source", "portId"],
        ["source", "portId"],
        `${path}.options.sidechain`,
      );
      sidechain = {
        source: routeSource(sidechainRaw.source, `${path}.options.sidechain.source`),
        portId: string(sidechainRaw.portId, `${path}.options.sidechain.portId`) as never,
      };
    }
    options = {
      ...(supplied.slotId === undefined ? {} : { slotId: string(supplied.slotId, `${path}.options.slotId`) }),
      ...(supplied.bypass === undefined ? {} : { bypass: supplied.bypass as boolean }),
      ...(supplied.quality === undefined ? {} : { quality: supplied.quality as "normal" }),
      ...(supplied.linkMode === undefined ? {} : { linkMode: supplied.linkMode }),
      ...(supplied.channel === undefined ? {} : { channel: supplied.channel }),
      ...(sidechain === undefined ? {} : { sidechain }),
    } as EffectOptions;
  }
  return effect(effectId, parameters as never, options as never);
}

/** The per-track rack keys decision 12 retired: named, so an old request learns what replaced them. */
const RETIRED_TRACK_KEYS = ["simd1", "dynamic", "simd2"] as const;

function consoleSlotRequest(value: unknown, path: string): ConsoleSlotSpec {
  const raw = record(value, path);
  keys(raw, ["slot", "effectId", "quality", "linkMode"], ["slot", "effectId"], path, UNKNOWN_FIELD);
  return {
    slot: string(raw.slot, `${path}.slot`),
    effectId: string(raw.effectId, `${path}.effectId`) as ConsoleSlotSpec["effectId"],
    ...(raw.quality === undefined ? {} : { quality: raw.quality as "normal" }),
    ...(raw.linkMode === undefined ? {} : { linkMode: raw.linkMode as ConsoleSlotSpec["linkMode"] & string }),
  };
}

function consoleRequest(value: unknown): ConsoleSpec {
  const raw = record(value, "$.console");
  keys(raw, ["preInsert", "postInsert"], [], "$.console", UNKNOWN_FIELD);
  const section = (name: "preInsert" | "postInsert") => raw[name] === undefined
    ? []
    : array(raw[name], `$.console.${name}`).map((slot, index) =>
      consoleSlotRequest(slot, `$.console.${name}[${index}]`));
  return { preInsert: section("preInsert"), postInsert: section("postInsert") };
}

function consoleEntryRequest(value: unknown, path: string): ConsoleEntrySpec {
  const raw = record(value, path);
  keys(raw, ["slot", "bypass", "parameters", "channel"], ["slot"], path, UNKNOWN_FIELD);
  // A null-prototype record for the same reason as `effectDecl`'s: `__proto__` is a member name.
  const parameters: Record<string, unknown> = Object.create(null) as Record<string, unknown>;
  if (raw.parameters !== undefined) {
    const supplied = record(raw.parameters, `${path}.parameters`);
    for (const [name, value] of Object.entries(supplied)) {
      parameters[name] = parameterValue(value, `${path}.parameters.${name}`);
    }
  }
  return {
    slot: string(raw.slot, `${path}.slot`),
    ...(raw.bypass === undefined ? {} : { bypass: raw.bypass as boolean }),
    ...(raw.parameters === undefined ? {} : { parameters: parameters as never }),
    ...(raw.channel === undefined ? {} : { channel: raw.channel as ConsoleEntrySpec["channel"] & string }),
  };
}

function track(value: unknown, path: string): { id: string; spec: TrackSpec } {
  const wrapper = record(value, path);
  keys(wrapper, ["id", "spec"], ["id", "spec"], path);
  const raw = record(wrapper.spec, `${path}.spec`);
  for (const retired of RETIRED_TRACK_KEYS) {
    if (Object.hasOwn(raw, retired)) {
      throw new MisoUsageError(
        `${path}.spec.${retired}: the per-track simd1, dynamic and simd2 racks are retired; declare `
          + "session console slots in $.console, each track's knobs for them in spec.console, and "
          + "per-track effects in spec.inserts",
        UNKNOWN_FIELD,
      );
    }
  }
  keys(
    raw,
    ["source", ...STRIP_KEYS],
    ["source"],
    `${path}.spec`,
    UNKNOWN_FIELD,
  );

  let source: TrackSpec["source"];
  if (typeof raw.source === "string") source = raw.source;
  else {
    const sourceRaw = record(raw.source, `${path}.spec.source`);
    keys(sourceRaw, ["id", "left", "right"], ["id", "left", "right"], `${path}.spec.source`);
    source = {
      id: string(sourceRaw.id, `${path}.spec.source.id`),
      left: number(sourceRaw.left, `${path}.spec.source.left`),
      right: number(sourceRaw.right, `${path}.spec.source.right`),
    };
  }
  return { id: string(wrapper.id, `${path}.id`), spec: { source, ...strip(raw, `${path}.spec`) } };
}

/** The strip keys a track and a submix share: everything after a track's source. */
const STRIP_KEYS = ["builtins", "console", "inserts", "fader", "pan"] as const;

/** Decode a strip's fields from a request object whose keys the caller has already checked. */
function strip(raw: Record<string, unknown>, path: string): SubmixSpec {
  let fader: SubmixSpec["fader"];
  if (raw.fader !== undefined) {
    const supplied = record(raw.fader, `${path}.fader`);
    keys(supplied, ["leftDb", "rightDb", "leftMute", "rightMute"], [], `${path}.fader`);
    fader = supplied as SubmixSpec["fader"];
  }

  let pan: SubmixSpec["pan"];
  if (raw.pan !== undefined) {
    const supplied = record(raw.pan, `${path}.pan`);
    if (Object.hasOwn(supplied, "matrix")) {
      keys(supplied, ["matrix", "smoothingSamples"], ["matrix"], `${path}.pan`);
      pan = {
        matrix: matrix(supplied.matrix, `${path}.pan.matrix`),
        ...(supplied.smoothingSamples === undefined ? {} : { smoothingSamples: number(supplied.smoothingSamples, `${path}.pan.smoothingSamples`) }),
      };
    } else {
      keys(supplied, ["left", "right", "smoothingSamples"], ["left", "right"], `${path}.pan`);
      pan = {
        left: number(supplied.left, `${path}.pan.left`),
        right: number(supplied.right, `${path}.pan.right`),
        ...(supplied.smoothingSamples === undefined ? {} : { smoothingSamples: number(supplied.smoothingSamples, `${path}.pan.smoothingSamples`) }),
      };
    }
  }

  const consoleEntries = raw.console === undefined
    ? undefined
    : array(raw.console, `${path}.console`).map((entry, index) =>
      consoleEntryRequest(entry, `${path}.console[${index}]`));
  const inserts = raw.inserts === undefined
    ? undefined
    : array(raw.inserts, `${path}.inserts`).map((entry, index) =>
      effectDecl(entry, `${path}.inserts[${index}]`));
  return {
    ...(raw.builtins === undefined ? {} : { builtins: builtins(raw.builtins, `${path}.builtins`) as NonNullable<SubmixSpec["builtins"]> }),
    ...(consoleEntries === undefined ? {} : { console: consoleEntries }),
    ...(inserts === undefined ? {} : { inserts }),
    ...(fader === undefined ? {} : { fader }),
    ...(pan === undefined ? {} : { pan }),
  };
}

/**
 * One `submixes` entry: a bare ID is the transparent strip, and `{ id, ...strip }` a submix strip
 * with a track's strip keys, its console entries checked against `$.console` as a track's are.
 */
function submix(value: unknown, path: string): { id: string; spec: SubmixSpec | undefined } {
  if (typeof value === "string") return { id: value, spec: undefined };
  const raw = record(value, path);
  keys(raw, ["id", ...STRIP_KEYS], ["id"], path, UNKNOWN_FIELD);
  return { id: string(raw.id, `${path}.id`), spec: strip(raw, path) };
}

function route(value: unknown, path: string): RouteSpec {
  const raw = record(value, path);
  keys(raw, ["id", "source", "destination", "matrix", "gainDb", "mute"], ["id", "source", "destination"], path);
  return {
    id: string(raw.id, `${path}.id`),
    source: routeSource(raw.source, `${path}.source`),
    destination: routeDestination(raw.destination, `${path}.destination`),
    ...(raw.matrix === undefined ? {} : { matrix: matrix(raw.matrix, `${path}.matrix`) }),
    ...(raw.gainDb === undefined ? {} : { gainDb: number(raw.gainDb, `${path}.gainDb`) }),
    // The builder refuses a non-boolean `mute`, as it does a non-boolean `bypass`.
    ...(raw.mute === undefined ? {} : { mute: raw.mute as boolean }),
  };
}

const U64_MAX = 18_446_744_073_709_551_615n;

function requestU64(value: unknown, path: string): bigint {
  let parsed: bigint;
  if (typeof value === "number") {
    if (!Number.isSafeInteger(value) || value < 0) {
      fail(path, "expected a nonnegative safe integer number or canonical decimal string");
    }
    parsed = BigInt(value);
  } else if (typeof value === "string" && /^(0|[1-9][0-9]*)$/.test(value)) {
    parsed = BigInt(value);
  } else {
    fail(path, "expected a nonnegative safe integer number or canonical decimal string");
  }
  if (parsed > U64_MAX) fail(path, "value exceeds u64::MAX");
  return parsed;
}

function automation(value: unknown, path: string): AutomationSpec {
  const raw = record(value, path);
  keys(raw, ["id", "target", "segments"], ["id", "target", "segments"], path);
  const target = record(raw.target, `${path}.target`);
  keys(target, ["trackId", "rack", "slotId", "parameter", "channel"], ["trackId", "rack", "parameter", "channel"], `${path}.target`);
  const segments = array(raw.segments, `${path}.segments`).map((value, index) => {
    const where = `${path}.segments[${index}]`;
    const segment = record(value, where);
    keys(segment, ["shape", "startSample", "endSample", "startValue", "endValue"], ["shape", "startSample", "endSample", "startValue", "endValue"], where);
    return {
      shape: string(segment.shape, `${where}.shape`) as "step" | "linear" | "exponential",
      startSample: requestU64(segment.startSample, `${where}.startSample`),
      endSample: requestU64(segment.endSample, `${where}.endSample`),
      startValue: number(segment.startValue, `${where}.startValue`),
      endValue: number(segment.endValue, `${where}.endValue`),
    };
  });
  return {
    id: string(raw.id, `${path}.id`),
    target: {
      trackId: string(target.trackId, `${path}.target.trackId`),
      rack: string(target.rack, `${path}.target.rack`) as AutomationSpec["target"]["rack"],
      ...(target.slotId === undefined ? {} : { slotId: string(target.slotId, `${path}.target.slotId`) }),
      parameter: string(target.parameter, `${path}.target.parameter`),
      channel: string(target.channel, `${path}.target.channel`) as AutomationSpec["target"]["channel"],
    },
    segments,
  };
}

/** Decode and translate one strict V1 authoring request through the public SDK builder. */
export function sessionBuilderFromRequest(value: unknown): SessionBuilder {
  const root = record(value, "$");
  keys(root, ["schemaVersion", "session", "sources", "console", "tracks", "submixes", "outputs", "routes", "automation"], ["schemaVersion", "session"], "$");
  if (root.schemaVersion !== 1) fail("$.schemaVersion", "expected 1");
  const options = record(root.session, "$.session");
  keys(options, ["id", "sampleRateHz", "revision", "quantumFrames"], ["id", "sampleRateHz"], "$.session");

  let builder = session({
    id: string(options.id, "$.session.id"),
    sampleRateHz: number(options.sampleRateHz, "$.session.sampleRateHz") as 48_000,
    ...(options.revision === undefined ? {} : { revision: requestU64(options.revision, "$.session.revision") }),
    ...(options.quantumFrames === undefined ? {} : { quantumFrames: number(options.quantumFrames, "$.session.quantumFrames") }),
  });
  for (const [index, value] of optionalArray(root, "sources").entries()) {
    const path = `$.sources[${index}]`;
    const wrapper = record(value, path);
    keys(wrapper, ["id", "spec"], ["id", "spec"], path);
    const spec = record(wrapper.spec, `${path}.spec`);
    keys(spec, ["channels", "bitDepth", "frames", "content"], ["channels", "bitDepth", "frames", "content"], `${path}.spec`);
    builder = builder.source(string(wrapper.id, `${path}.id`), {
      channels: number(spec.channels, `${path}.spec.channels`) as 1 | 2,
      bitDepth: spec.bitDepth as SourceSpec["bitDepth"],
      frames: requestU64(spec.frames, `${path}.spec.frames`),
      content: string(spec.content, `${path}.spec.content`),
    });
  }
  // The console precedes every track: each track's console entries are checked against it.
  if (root.console !== undefined) builder = builder.console(consoleRequest(root.console));
  for (const [index, value] of optionalArray(root, "submixes").entries()) {
    const decoded = submix(value, `$.submixes[${index}]`);
    builder = builder.submix(decoded.id, decoded.spec);
  }
  for (const [index, value] of optionalArray(root, "outputs").entries()) {
    builder = builder.output(string(value, `$.outputs[${index}]`));
  }
  for (const [index, value] of optionalArray(root, "tracks").entries()) {
    const decoded = track(value, `$.tracks[${index}]`);
    builder = builder.track(decoded.id, decoded.spec);
  }
  for (const [index, value] of optionalArray(root, "routes").entries()) {
    builder = builder.route(route(value, `$.routes[${index}]`));
  }
  for (const [index, value] of optionalArray(root, "automation").entries()) {
    builder = builder.automation(automation(value, `$.automation[${index}]`));
  }
  return builder;
}
