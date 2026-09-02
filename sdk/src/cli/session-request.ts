import type { EffectId } from "../generated/catalog.ts";
import { effect, session } from "../core/session.ts";
import type { SessionBuilder } from "../core/session.ts";
import type {
  AutomationSpec,
  EffectOptions,
  RouteDestination,
  RouteSource,
  RouteSpec,
  SourceSpec,
  TrackSpec,
} from "../core/types.ts";

export interface SessionBuildRequestV1 {
  readonly schemaVersion: 1;
  readonly session: {
    readonly id: string;
    readonly sampleRateHz: number;
    readonly revision?: number;
    readonly quantumFrames?: number;
  };
  readonly sources?: readonly unknown[];
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

function keys(
  value: Record<string, unknown>,
  allowed: readonly string[],
  required: readonly string[],
  path: string,
): void {
  const admitted = new Set(allowed);
  for (const key of Object.keys(value)) {
    if (!admitted.has(key)) fail(`${path}.${key}`, "unknown key");
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
  if (kind === "submix_output") {
    keys(raw, ["kind", "submixId"], ["kind", "submixId"], path);
    return { kind, submixId: string(raw.submixId, `${path}.submixId`) };
  }
  fail(`${path}.kind`, "expected 'track' or 'submix_output'");
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

function track(value: unknown, path: string): { id: string; spec: TrackSpec } {
  const wrapper = record(value, path);
  keys(wrapper, ["id", "spec"], ["id", "spec"], path);
  const raw = record(wrapper.spec, `${path}.spec`);
  keys(raw, ["source", "builtins", "fader", "pan", "simd1", "dynamic", "simd2"], ["source"], `${path}.spec`);

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

  let fader: TrackSpec["fader"];
  if (raw.fader !== undefined) {
    const supplied = record(raw.fader, `${path}.spec.fader`);
    keys(supplied, ["leftDb", "rightDb", "leftMute", "rightMute"], [], `${path}.spec.fader`);
    fader = supplied as TrackSpec["fader"];
  }

  let pan: TrackSpec["pan"];
  if (raw.pan !== undefined) {
    const supplied = record(raw.pan, `${path}.spec.pan`);
    if (Object.hasOwn(supplied, "matrix")) {
      keys(supplied, ["matrix", "smoothingSamples"], ["matrix"], `${path}.spec.pan`);
      pan = {
        matrix: matrix(supplied.matrix, `${path}.spec.pan.matrix`),
        ...(supplied.smoothingSamples === undefined ? {} : { smoothingSamples: number(supplied.smoothingSamples, `${path}.spec.pan.smoothingSamples`) }),
      };
    } else {
      keys(supplied, ["left", "right", "smoothingSamples"], ["left", "right"], `${path}.spec.pan`);
      pan = {
        left: number(supplied.left, `${path}.spec.pan.left`),
        right: number(supplied.right, `${path}.spec.pan.right`),
        ...(supplied.smoothingSamples === undefined ? {} : { smoothingSamples: number(supplied.smoothingSamples, `${path}.spec.pan.smoothingSamples`) }),
      };
    }
  }

  const rack = (name: "simd1" | "dynamic" | "simd2") => raw[name] === undefined
    ? undefined
    : array(raw[name], `${path}.spec.${name}`).map((entry, index) =>
      effectDecl(entry, `${path}.spec.${name}[${index}]`));
  const simd1 = rack("simd1");
  const dynamic = rack("dynamic");
  const simd2 = rack("simd2");
  return {
    id: string(wrapper.id, `${path}.id`),
    spec: {
      source,
      ...(raw.builtins === undefined ? {} : { builtins: builtins(raw.builtins, `${path}.spec.builtins`) as NonNullable<TrackSpec["builtins"]> }),
      ...(fader === undefined ? {} : { fader }),
      ...(pan === undefined ? {} : { pan }),
      ...(simd1 === undefined ? {} : { simd1 }),
      ...(dynamic === undefined ? {} : { dynamic }),
      ...(simd2 === undefined ? {} : { simd2 }),
    },
  };
}

function route(value: unknown, path: string): RouteSpec {
  const raw = record(value, path);
  keys(raw, ["id", "source", "destination", "matrix", "gainDb"], ["id", "source", "destination"], path);
  return {
    id: string(raw.id, `${path}.id`),
    source: routeSource(raw.source, `${path}.source`),
    destination: routeDestination(raw.destination, `${path}.destination`),
    ...(raw.matrix === undefined ? {} : { matrix: matrix(raw.matrix, `${path}.matrix`) }),
    ...(raw.gainDb === undefined ? {} : { gainDb: number(raw.gainDb, `${path}.gainDb`) }),
  };
}

function decimalSample(value: unknown, path: string): bigint {
  if (typeof value !== "string" || !/^(0|[1-9][0-9]*)$/.test(value)) {
    fail(path, "expected a canonical unsigned decimal string");
  }
  return BigInt(value);
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
      startSample: decimalSample(segment.startSample, `${where}.startSample`),
      endSample: decimalSample(segment.endSample, `${where}.endSample`),
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
  keys(root, ["schemaVersion", "session", "sources", "tracks", "submixes", "outputs", "routes", "automation"], ["schemaVersion", "session"], "$");
  if (root.schemaVersion !== 1) fail("$.schemaVersion", "expected 1");
  const options = record(root.session, "$.session");
  keys(options, ["id", "sampleRateHz", "revision", "quantumFrames"], ["id", "sampleRateHz"], "$.session");

  let builder = session({
    id: string(options.id, "$.session.id"),
    sampleRateHz: number(options.sampleRateHz, "$.session.sampleRateHz") as 48_000,
    ...(options.revision === undefined ? {} : { revision: number(options.revision, "$.session.revision") }),
    ...(options.quantumFrames === undefined ? {} : { quantumFrames: number(options.quantumFrames, "$.session.quantumFrames") }),
  });
  for (const [index, value] of optionalArray(root, "sources").entries()) {
    const path = `$.sources[${index}]`;
    const wrapper = record(value, path);
    keys(wrapper, ["id", "spec"], ["id", "spec"], path);
    const spec = record(wrapper.spec, `${path}.spec`);
    keys(spec, ["channels", "bitDepth", "frames", "content"], ["channels", "bitDepth", "frames", "content"], `${path}.spec`);
    builder = builder.source(string(wrapper.id, `${path}.id`), spec as unknown as SourceSpec);
  }
  for (const [index, value] of optionalArray(root, "submixes").entries()) {
    builder = builder.submix(string(value, `$.submixes[${index}]`));
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
