import { ABI_LAYOUT } from "../generated/abi.js";
import { CATALOG, type EffectDescriptor, type EffectId } from "../generated/catalog.js";
import { MisoSessionError } from "./errors.js";
import type {
  AutomationSegment,
  AutomationTarget,
  BuiltinsSpec,
  Channel,
  EffectDecl,
  EffectOptions,
  EffectParamValues,
  Matrix2x2,
  Rack,
  RouteDestination,
  RouteSource,
  RouteSpec,
  SourceSpec,
  TrackSpec,
} from "./types.js";

export type SessionSampleRateHz = 44_100 | 48_000 | 88_200 | 96_000;

export interface SessionLimits {
  readonly pcmRingFrames: number;
  readonly controlQueueMessages: number;
  readonly memoryBytes: number;
}

export interface SessionOptions {
  /** Stable Session V1 identity. */
  readonly id: string;
  /** Explicit launch render rate. */
  readonly sampleRateHz: SessionSampleRateHz;
  readonly revision?: number;
  readonly quantumFrames?: number;
  readonly limits?: Partial<SessionLimits>;
}

export interface OutputSpec {
  readonly id: string;
}

export interface AutomationSpec {
  readonly id: string;
  readonly target: AutomationTarget;
  readonly segments: readonly AutomationSegment[];
}

type JsonRecord = Readonly<Record<string, unknown>>;
export interface SessionJson extends JsonRecord {
  readonly schema_version: 1;
  readonly session_id: string;
  readonly revision: number;
  readonly sample_rate_hz: SessionSampleRateHz;
  readonly quantum_frames: number;
  readonly render_profile: JsonRecord;
  readonly output_profile: JsonRecord;
  readonly limits: JsonRecord;
  readonly sources: readonly JsonRecord[];
  readonly tracks: readonly JsonRecord[];
  readonly submixes: readonly JsonRecord[];
  readonly outputs: readonly JsonRecord[];
  readonly routes: readonly JsonRecord[];
  readonly automation: readonly JsonRecord[];
}

export type SessionShape = Readonly<Record<string, readonly EffectDecl[]>>;

export interface PlacedEffect<D extends EffectDecl = EffectDecl> {
  readonly id: string;
  readonly effectId: D["effectId"];
  readonly rack: Rack;
  readonly rackIndex: number;
}

type PlacedEffectTuple<Effects extends readonly EffectDecl[]> = Readonly<{
  [Index in keyof Effects]: Effects[Index] extends EffectDecl ? PlacedEffect<Effects[Index]> : never;
}>;

export type SessionPlanTrack<Tracks extends SessionShape> = {
  [Id in keyof Tracks & string]: Readonly<{ id: Id; effects: PlacedEffectTuple<Tracks[Id]> }>;
}[keyof Tracks & string];

type PrepareConfigField = typeof ABI_LAYOUT.structures.prepareConfig.fields[number];
type PrepareHeaderName = "structSize" | "abiVersion" | "sampleRateHz" | "quantumFrames";
type PrepareLimitField = Exclude<PrepareConfigField, { readonly name: PrepareHeaderName }>;
export type PrepareLimits = {
  readonly [Field in PrepareLimitField as Field["name"]]: Field["type"] extends "u32" ? number : bigint;
};

export interface SessionPlan<Tracks extends SessionShape = SessionShape> {
  readonly json: SessionJson;
  readonly toml: string;
  readonly tracks: readonly SessionPlanTrack<Tracks>[];
  toJson(): SessionJson;
  limits(overrides?: Partial<PrepareLimits>): PrepareLimits;
}

type NormalizedOptions = Readonly<{
  readonly sessionId: string; readonly revision: number; readonly sampleRateHz: SessionSampleRateHz; readonly quantumFrames: number;
  readonly renderProfile: Readonly<{ id: "single"; mode: "single_thread" }>;
  readonly outputProfile: Readonly<{ id: "main"; channels: 2; sampleFormat: "f32_planar" }>;
  readonly limits: SessionLimits;
}>;
type BuilderState = Readonly<{
  readonly options: NormalizedOptions;
  readonly sources: readonly (Readonly<{ id: string; spec: SourceSpec }>)[];
  readonly tracks: readonly (Readonly<{ id: string; spec: TrackSpec }>)[];
  readonly submixes: readonly string[];
  readonly outputs: readonly OutputSpec[];
  readonly routes: readonly RouteSpec[];
  readonly automation: readonly AutomationSpec[];
}>;

type BuilderTracks = SessionShape;
type RackEffects<T> = T extends readonly EffectDecl[] ? T : readonly [];
type TrackEffects<T extends TrackSpec> = readonly [
  ...RackEffects<T["simd1"]>,
  ...RackEffects<T["dynamic"]>,
  ...RackEffects<T["simd2"]>,
];

const DEFAULT_LIMITS: SessionLimits = Object.freeze({
  pcmRingFrames: 1_024,
  controlQueueMessages: 64,
  memoryBytes: 16 * 1024 * 1024,
});
const ID = /^[a-z][a-z0-9._-]{0,126}$/;
const UNITY: Matrix2x2 = Object.freeze({ ll: 1, lr: 0, rl: 0, rr: 1 });
const DEFAULT_BUILTINS = Object.freeze({ polarityInvert: false, trimDb: 0, hpfHz: 0, lpfHz: 0 });
const TOML_I64_MAX = 9_223_372_036_854_775_807n;

function fail(message: string, path: string, descriptor?: Record<string, unknown>): never {
  throw new MisoSessionError(message, path, descriptor);
}

function stableId(value: string, path: string): string {
  if (!ID.test(value)) fail("Session V1 IDs must match [a-z][a-z0-9._-]{0,126}", path);
  return value;
}

function nonemptyString(value: unknown, path: string): string {
  if (typeof value !== "string" || value.length === 0) fail("Expected a nonempty string", path);
  return value;
}

function integer(value: number, path: string, minimum = 0): number {
  if (!Number.isSafeInteger(value) || value < minimum) fail("Expected a non-negative safe integer", path);
  return value;
}

function u32(value: number, path: string, minimum = 0): number {
  integer(value, path, minimum);
  if (value > 0xffff_ffff) fail("Expected a u32", path);
  return value;
}

function f32(value: number, path: string): number {
  if (!Number.isFinite(value)) fail("Expected a finite f32 value", path);
  const rounded = Math.fround(value);
  if (!Number.isFinite(rounded)) fail("Finite f64 value is outside the finite f32 domain", path);
  return rounded;
}

function bool(value: unknown, path: string): boolean {
  if (typeof value !== "boolean") fail("Expected a boolean", path);
  return value;
}

function freeze<T>(value: T): T {
  if (value && typeof value === "object" && !Object.isFrozen(value)) {
    Object.freeze(value);
    for (const child of Object.values(value as Record<string, unknown>)) freeze(child);
  }
  return value;
}

function effectDescriptor(effectId: string, path: string): EffectDescriptor {
  const descriptor = CATALOG.effects.find((candidate) => candidate.id === effectId);
  if (!descriptor) fail(`Unknown native effect '${effectId}'`, path);
  return descriptor;
}

function parameterValue(
  descriptor: EffectDescriptor,
  name: string,
  raw: unknown,
  channel: Channel,
  path: string,
): readonly JsonRecord[] {
  const parameter = descriptor.parameters.find((candidate) => candidate.name === name);
  if (!parameter) fail(`Unknown parameter '${name}' for ${descriptor.id}`, path);
  const lanes = lanePair(raw);
  const values = parameter.channelPolicyName === "perLane" && lanes
    ? [["left", lanes[0]], ["right", lanes[1]]] as const
    : [[channel, raw]] as const;
  if (parameter.channelPolicyName === "shared" && channel !== "both") {
    fail("A shared parameter must use channel 'both'", path);
  }
  return values.map(([valueChannel, value]) => {
    let normalized: number;
    if (parameter.domainName === "boolean") {
      if (typeof value !== "boolean") fail("Expected a boolean", path, parameter as unknown as Record<string, unknown>);
      normalized = value ? 1 : 0;
    } else if (parameter.domainName === "enumeration") {
      if (typeof value !== "string") fail("Expected an enumeration label", path, parameter as unknown as Record<string, unknown>);
      const choice = parameter.enumChoices.find((candidate) => candidate.label === value);
      if (!choice) fail(`Unknown enumeration label '${value}'`, path, parameter as unknown as Record<string, unknown>);
      normalized = choice.value;
    } else {
      if (typeof value !== "number") fail("Expected a numeric parameter value", path);
      normalized = f32(value, path);
      if (normalized < f32(parameter.minimum, `${path}.minimum`) || normalized > f32(parameter.maximum, `${path}.maximum`)) {
        fail(`Parameter '${name}' is outside its local metadata domain`, path, parameter as unknown as Record<string, unknown>);
      }
    }
    return freeze({ parameter_id: parameter.id, channel: valueChannel, unit: parameter.unitName, value: normalized });
  });
}

function isLaneValue(value: unknown): value is Readonly<{ left: unknown; right: unknown }> {
  return !!value && typeof value === "object" && !Array.isArray(value) && "left" in value && "right" in value;
}

function lanePair(value: unknown): readonly [unknown, unknown] | undefined {
  if (Array.isArray(value) && value.length === 2) return [value[0], value[1]];
  return isLaneValue(value) ? [value.left, value.right] : undefined;
}

/** Creates immutable metadata-validated native effect input. Rack placement resolves an omitted slot ID. */
export function effect<E extends EffectId>(
  effectId: E,
  parameters: EffectParamValues<E> = {},
  options: EffectOptions = {},
): EffectDecl<E> {
  const descriptor = effectDescriptor(effectId, "effectId");
  if (options.slotId !== undefined) stableId(options.slotId, "options.slotId");
  if (options.bypass !== undefined) bool(options.bypass, "options.bypass");
  if (options.quality !== undefined && options.quality !== "normal") fail("Only normal quality is published", "options.quality");
  if (options.linkMode !== undefined && !["dual_mono", "maximum", "average"].includes(options.linkMode)) {
    fail("Unknown detector link mode", "options.linkMode");
  }
  if (options.channel !== undefined && !["left", "right", "both"].includes(options.channel)) fail("Unknown channel", "options.channel");
  if (options.sidechain) {
    routeSource(options.sidechain.source, "options.sidechain.source");
    stableId(options.sidechain.portId, "options.sidechain.portId");
  }
  for (const [name, value] of Object.entries(parameters)) {
    parameterValue(descriptor, name, value, options.channel ?? "both", `effect.parameters.${name}`);
  }
  return freeze({
    effectId,
    ...(options.slotId === undefined ? {} : { slotId: options.slotId }),
    parameters: freeze({ ...parameters }),
    options: freeze({ bypass: options.bypass ?? false, quality: "normal", linkMode: options.linkMode ?? "dual_mono", channel: options.channel ?? "both", ...(options.sidechain ? { sidechain: options.sidechain } : {}) }),
  }) as EffectDecl<E>;
}

export class SessionBuilder<Tracks extends BuilderTracks = {}> {
  constructor(private readonly state: BuilderState) {}

  source(id: string, spec: SourceSpec): SessionBuilder<Tracks> {
    stableId(id, "source.id");
    if (this.state.sources.some((source) => source.id === id)) fail("Duplicate source ID", "source.id");
    if (spec.channels !== 1 && spec.channels !== 2) fail("Source must be mono or dual-mono", "source.channels");
    integer(spec.frames, "source.frames", 1);
    if (spec.sampleRateHz !== undefined) rate(spec.sampleRateHz, "source.sampleRateHz");
    if (spec.identity !== undefined) nonemptyString(spec.identity, "source.identity");
    if (spec.locator !== undefined) nonemptyString(spec.locator, "source.locator");
    return this.next({ sources: [...this.state.sources, freeze({ id, spec: freeze({ ...spec }) })] });
  }

  track<Id extends string, Spec extends TrackSpec>(id: Id, spec: Spec): SessionBuilder<Tracks & Readonly<Record<Id, TrackEffects<Spec>>> > {
    stableId(id, "track.id");
    if (this.graphIds().has(id)) fail("Track ID collides with a graph entity", "track.id");
    validateTrackSpec(spec, "track");
    validateTrackSource(spec, this.state.sources, "track.source");
    return this.next<Tracks & Readonly<Record<Id, TrackEffects<Spec>>>>({ tracks: [...this.state.tracks, freeze({ id, spec: freeze({ ...spec }) })] });
  }

  submix(id: string): SessionBuilder<Tracks> {
    stableId(id, "submix.id");
    if (this.graphIds().has(id)) fail("Submix ID collides with a graph entity", "submix.id");
    return this.next({ submixes: [...this.state.submixes, id] });
  }

  output(id: string): SessionBuilder<Tracks> {
    stableId(id, "output.id");
    if (this.graphIds().has(id)) fail("Output ID collides with a graph entity", "output.id");
    return this.next({ outputs: [...this.state.outputs, freeze({ id })] });
  }

  route(spec: RouteSpec): SessionBuilder<Tracks> {
    stableId(spec.id, "route.id");
    if (this.state.routes.some((route) => route.id === spec.id)) fail("Duplicate route ID", "route.id");
    routeSource(spec.source, "route.source");
    routeDestination(spec.destination, "route.destination");
    if (spec.matrix) routeMatrix(spec.matrix, "route.matrix");
    if (spec.gainDb !== undefined) f32(spec.gainDb, "route.gainDb");
    return this.next({ routes: [...this.state.routes, freeze({ ...spec })] });
  }

  automate(spec: AutomationSpec): SessionBuilder<Tracks> {
    stableId(spec.id, "automation.id");
    if (this.state.automation.some((automation) => automation.id === spec.id)) fail("Duplicate automation ID", "automation.id");
    validateAutomationShape(spec, "automation");
    return this.next({ automation: [...this.state.automation, freeze({ ...spec, segments: freeze([...spec.segments]) })] });
  }

  build(): SessionPlan<Tracks> {
    const json = normalize(this.state);
    validateGraph(json);
    return makePlan(json, summarizeBuilderTracks<Tracks>(this.state));
  }

  private graphIds(): Set<string> {
    return new Set([...this.state.tracks.map((track) => track.id), ...this.state.submixes, ...this.state.outputs.map((output) => output.id)]);
  }

  private next<NextTracks extends BuilderTracks = Tracks>(update: Partial<BuilderState>): SessionBuilder<NextTracks> {
    return new SessionBuilder<NextTracks>(freeze({ ...this.state, ...update }));
  }
}

/** Starts a persistent, immutable Session V1 builder. */
export function session(options: SessionOptions): SessionBuilder<{}> {
  const sessionId = options.id;
  stableId(sessionId, "id");
  const revision = options.revision ?? 0;
  integer(revision, "revision");
  const sampleRateHz = options.sampleRateHz;
  rate(sampleRateHz, "sampleRateHz");
  const quantumFrames = options.quantumFrames ?? 128;
  u32(quantumFrames, "quantumFrames", 1);
  const limits = {
    pcmRingFrames: options.limits?.pcmRingFrames ?? DEFAULT_LIMITS.pcmRingFrames,
    controlQueueMessages: options.limits?.controlQueueMessages ?? DEFAULT_LIMITS.controlQueueMessages,
    memoryBytes: options.limits?.memoryBytes ?? DEFAULT_LIMITS.memoryBytes,
  };
  integer(limits.pcmRingFrames, "limits.pcmRingFrames", 1);
  integer(limits.controlQueueMessages, "limits.controlQueueMessages", 1);
  integer(limits.memoryBytes, "limits.memoryBytes", 1);
  const renderProfile = { id: "single" as const, mode: "single_thread" as const };
  const outputProfile = { id: "main" as const, channels: 2 as const, sampleFormat: "f32_planar" as const };
  return new SessionBuilder(freeze({
    options: freeze({ sessionId, revision, sampleRateHz, quantumFrames, renderProfile: freeze({ ...renderProfile }), outputProfile: freeze({ ...outputProfile }), limits: freeze(limits) }),
    sources: [], tracks: [], submixes: [], outputs: [], routes: [], automation: [],
  }));
}

/** Rebuilds an immutable plan from its JSON-safe normalized form. */
export const SessionPlan = Object.freeze({
  fromJson(json: SessionJson): SessionPlan<SessionShape> {
    const copy = jsonDocument(cloneJson(json) as SessionJson);
    validateJson(copy);
    validateGraph(copy);
    return makePlan(copy, summarizeJsonTracks(copy));
  },
});

const NEGATIVE_ZERO_TAG = "$miso.sdk.f32";

function jsonDocument(json: SessionJson): SessionJson {
  Object.defineProperty(json, "toJSON", { enumerable: false, value: () => encodeJson(json) });
  return freeze(json);
}

function encodeJson(value: unknown): unknown {
  if (typeof value === "number" && Object.is(value, -0)) return { [NEGATIVE_ZERO_TAG]: "-0" };
  if (value === null || typeof value === "string" || typeof value === "boolean" || typeof value === "number") return value;
  if (Array.isArray(value)) return value.map(encodeJson);
  if (value && typeof value === "object") return Object.fromEntries(Object.entries(value).map(([key, child]) => [key, encodeJson(child)]));
  fail("Session JSON contains a non-JSON value", "json");
}

function cloneJson(value: unknown): unknown {
  if (value === null || typeof value === "string" || typeof value === "boolean" || typeof value === "number") return value;
  if (Array.isArray(value)) return value.map(cloneJson);
  if (value && typeof value === "object") {
    const entries = Object.entries(value);
    if (entries.length === 1 && entries[0][0] === NEGATIVE_ZERO_TAG && entries[0][1] === "-0") return -0;
    return Object.fromEntries(entries.map(([key, child]) => [key, cloneJson(child)]));
  }
  fail("Session JSON contains a non-JSON value", "json");
}

function summarizeBuilderTracks<Tracks extends SessionShape>(state: BuilderState): readonly SessionPlanTrack<Tracks>[] {
  const summaries = [...state.tracks].sort((a, b) => asciiCompare(a.id, b.id)).map(({ id, spec }) => {
    const effects = (["simd1", "dynamic", "simd2"] as const).flatMap((rack) => (spec[rack] ?? []).map((declaration, rackIndex) => freeze({
      id: declaration.slotId ?? `${rack}-${rackIndex + 1}`,
      effectId: declaration.effectId,
      rack,
      rackIndex,
    })));
    return freeze({ id, effects: freeze(effects) });
  });
  return freeze(summaries) as unknown as readonly SessionPlanTrack<Tracks>[];
}

function summarizeJsonTracks(json: SessionJson): readonly SessionPlanTrack<SessionShape>[] {
  const summaries = json.tracks.map((track) => {
    const effects = (["simd1", "dynamic", "simd2"] as const).flatMap((rack) => {
      const rackValue = track[rack] as JsonRecord;
      return (rackValue.effects as readonly JsonRecord[]).map((effect, rackIndex) => freeze({
        id: String(effect.id),
        effectId: String((effect.identity as JsonRecord).effect_id) as EffectId,
        rack,
        rackIndex,
      }));
    });
    return freeze({ id: String(track.id), effects: freeze(effects) });
  });
  return freeze(summaries) as unknown as readonly SessionPlanTrack<SessionShape>[];
}

function prepareLimits(json: SessionJson, overrides: Partial<PrepareLimits> = {}): PrepareLimits {
  const sessionLimits = json.limits as JsonRecord;
  const effectCount = json.tracks.reduce((total, track) => total + (["simd1", "dynamic", "simd2"] as const).reduce((rackTotal, rack) => rackTotal + ((track[rack] as JsonRecord).effects as readonly JsonRecord[]).length, 0), 0);
  const defaults: PrepareLimits = {
    sessionTomlBytes: 1 << 20,
    diagnosticBytes: 1 << 14,
    sourceIdBytes: 1 << 10,
    maximumSourceChannels: 8,
    sourceRingFrames: Number(sessionLimits.pcm_ring_frames),
    maximumAutomationSpansPerBlock: 256,
    maximumTracks: BigInt(Math.max(1_024, json.tracks.length)),
    maximumSources: BigInt(Math.max(1_024, json.sources.length)),
    maximumRoutes: BigInt(Math.max(4_096, json.routes.length)),
    maximumEffects: BigInt(Math.max(8_192, effectCount)),
    maximumGraphSessionPlusPlanBytes: 64n << 20n,
    maximumSourceTotalBytes: 64n << 20n,
    maximumSourceOverheadBytes: 16n << 20n,
    maximumEffectStateBytes: 16n << 20n,
    maximumEffectScratchBytes: 16n << 20n,
    maximumBuiltinRetainedBytes: 64n << 20n,
    maximumHostRetainedBytes: 16n << 20n,
    maximumNamedAllocationBytes: 64n << 20n,
    maximumMeterStreams: 1_024n,
    maximumMeterItems: 1n << 20n,
    maximumMeterBytes: 16n << 20n,
    consoleCommandQueueRecords: BigInt(Math.min(ABI_LAYOUT.constants.maximumCommandRecords, Number(sessionLimits.control_queue_messages))),
    consoleMeterBlocks: 0n,
    consoleObservationTaps: 0n,
    consoleMasterTrackPlusOne: 0n,
  };
  return freeze({ ...defaults, ...overrides });
}

function makePlan<Tracks extends SessionShape>(json: SessionJson, tracks: readonly SessionPlanTrack<Tracks>[]): SessionPlan<Tracks> {
  return freeze({ json, toml: writeToml(json), tracks, toJson: () => json, limits: (overrides?: Partial<PrepareLimits>) => prepareLimits(json, overrides) });
}

function rate(value: number, path: string): SessionSampleRateHz {
  if (value !== 44_100 && value !== 48_000 && value !== 88_200 && value !== 96_000) fail("Unsupported launch sample rate", path);
  return value;
}

function trackBuiltinMatrix(value: Matrix2x2, path: string): Matrix2x2 {
  return freeze({ ll: builtinNumber("matrix_ll", value.ll, `${path}.ll`), lr: builtinNumber("matrix_lr", value.lr, `${path}.lr`), rl: builtinNumber("matrix_rl", value.rl, `${path}.rl`), rr: builtinNumber("matrix_rr", value.rr, `${path}.rr`) });
}
function routeMatrix(value: Matrix2x2, path: string): Matrix2x2 {
  return freeze({ ll: f32(value.ll, `${path}.ll`), lr: f32(value.lr, `${path}.lr`), rl: f32(value.rl, `${path}.rl`), rr: f32(value.rr, `${path}.rr`) });
}
function panValue(value: number, path: string): number {
  const normalized = f32(value, path);
  if (normalized < -1 || normalized > 1) fail("Pan value is outside the Session V1 [-1,1] domain", path);
  return normalized;
}

function builtinDescriptor(name: string): (typeof CATALOG.builtins.parameters)[number] {
  const descriptor = CATALOG.builtins.parameters.find((candidate) => candidate.name === name);
  if (!descriptor) fail(`Generated builtin descriptor is absent: ${name}`, "catalog.builtins");
  return descriptor;
}
function builtinNumber(name: string, value: number, path: string): number {
  const descriptor = builtinDescriptor(name), normalized = f32(value, path);
  if (descriptor.domain === "finiteInclusive" && (normalized < f32(descriptor.minimum, `${path}.minimum`) || normalized > f32(descriptor.maximum, `${path}.maximum`))) fail("Builtin value is outside its metadata domain", path, descriptor as unknown as Record<string, unknown>);
  return normalized;
}
function builtinBoolean(name: string, value: unknown, path: string): boolean {
  const descriptor = builtinDescriptor(name);
  if (typeof value !== "boolean") fail("Builtin requires a boolean", path, descriptor as unknown as Record<string, unknown>);
  return value;
}
function builtinFilter(name: "hpf_hz" | "lpf_hz", value: number, rateHz: number, path: string): number {
  const descriptor = builtinDescriptor(name), normalized = f32(value, path);
  const rates = descriptor.maximumByRate as Readonly<Record<string, number>> | null;
  const maximum = rates?.[String(rateHz)];
  if (maximum === undefined) fail("Builtin filter descriptor lacks this launch rate", path, descriptor as unknown as Record<string, unknown>);
  const minimum = descriptor.minimum;
  if (minimum === null) fail("Builtin filter descriptor lacks a minimum", path, descriptor as unknown as Record<string, unknown>);
  const disabled = descriptor.disabledValue;
  if (disabled === null) fail("Builtin filter descriptor lacks a disabled value", path, descriptor as unknown as Record<string, unknown>);
  if (normalized !== f32(disabled, `${path}.disabledValue`) && (normalized < f32(minimum, `${path}.minimum`) || normalized > f32(maximum, `${path}.maximum`))) fail("Builtin filter value is outside its metadata domain", path, descriptor as unknown as Record<string, unknown>);
  return normalized;
}

function routeSource(value: RouteSource, path: string): void {
  if (value.kind === "track") { stableId(value.trackId, `${path}.trackId`); if (!SEND_TAPS.has(value.tap)) fail("Unknown send tap", `${path}.tap`); return; }
  if (value.kind === "submix_output") { stableId(value.submixId, `${path}.submixId`); return; }
  fail("Unknown route source", path);
}

function routeDestination(value: RouteDestination, path: string): void {
  if (value.kind === "submix_input") { stableId(value.submixId, `${path}.submixId`); return; }
  if (value.kind === "output_input") { stableId(value.outputId, `${path}.outputId`); return; }
  fail("Unknown route destination", path);
}

const SEND_TAPS = new Set(["input", "post_input_builtins", "post_simd1", "post_dynamic", "post_simd2_pre_fader", "post_fader", "post_matrix"]);

function validateTrackSpec(spec: TrackSpec, path: string): void {
  if (typeof spec.source === "string") stableId(spec.source, `${path}.source`);
  else { stableId(spec.source.left[0], `${path}.source.left[0]`); stableId(spec.source.right[0], `${path}.source.right[0]`); integer(spec.source.left[1], `${path}.source.left[1]`); integer(spec.source.right[1], `${path}.source.right[1]`); }
  validateBuiltins(spec.builtins, `${path}.builtins`);
  if (spec.fader) {
    if (spec.fader.leftDb !== undefined) builtinNumber("fader_db", spec.fader.leftDb, `${path}.fader.leftDb`);
    if (spec.fader.rightDb !== undefined) builtinNumber("fader_db", spec.fader.rightDb, `${path}.fader.rightDb`);
    if (spec.fader.leftMute !== undefined) builtinBoolean("mute", spec.fader.leftMute, `${path}.fader.leftMute`);
    if (spec.fader.rightMute !== undefined) builtinBoolean("mute", spec.fader.rightMute, `${path}.fader.rightMute`);
  }
  if (spec.pan) {
    if ("matrix" in spec.pan) trackBuiltinMatrix(spec.pan.matrix, `${path}.pan.matrix`);
    else { panValue(spec.pan.left, `${path}.pan.left`); panValue(spec.pan.right, `${path}.pan.right`); }
    if (spec.pan.smoothingSamples !== undefined) u32(spec.pan.smoothingSamples, `${path}.pan.smoothingSamples`);
  }
  for (const rack of ["simd1", "dynamic", "simd2"] as const) validateRack(spec[rack] ?? [], rack, path);
}

/** Session V1 has one source_id; the ergonomic lane form must still name one source. */
function validateTrackSource(spec: TrackSpec, sources: BuilderState["sources"], path: string): void {
  const left = typeof spec.source === "string" ? [spec.source, 0] as const : spec.source.left;
  const right = typeof spec.source === "string"
    ? [spec.source, sources.find((source) => source.id === spec.source)?.spec.channels === 1 ? 0 : 1] as const
    : spec.source.right;
  const leftSource = sources.find((source) => source.id === left[0]);
  const rightSource = sources.find((source) => source.id === right[0]);
  if (!leftSource) fail("Track references an unknown left source", `${path}.left[0]`);
  if (!rightSource) fail("Track references an unknown right source", `${path}.right[0]`);
  if (left[0] !== right[0]) fail("Session V1 track lanes must reference the same source_id", path);
  if (left[1] >= leftSource.spec.channels) fail("Left source channel exceeds declared channel_count", `${path}.left[1]`);
  if (right[1] >= rightSource.spec.channels) fail("Right source channel exceeds declared channel_count", `${path}.right[1]`);
}

function validateBuiltins(raw: import("./types.js").PerLane<BuiltinsSpec> | undefined, path: string): void {
  if (!raw) return;
  const lanes = Array.isArray(raw) ? [raw[0], raw[1]] : isLaneValue(raw) ? [raw.left, raw.right] : [raw];
  for (const lane of lanes) {
    if (lane.polarityInvert !== undefined) builtinBoolean("polarity_invert", lane.polarityInvert, `${path}.polarityInvert`);
    if (lane.trimDb !== undefined) builtinNumber("trim_db", lane.trimDb, `${path}.trimDb`);
    for (const name of ["hpfHz", "lpfHz"] as const) if (lane[name] !== undefined && f32(lane[name], `${path}.${name}`) < 0) fail("Filter frequency must be zero or positive", `${path}.${name}`, builtinDescriptor(name === "hpfHz" ? "hpf_hz" : "lpf_hz") as unknown as Record<string, unknown>);
  }
}

function validateRack(effects: readonly EffectDecl[], rack: Rack, path: string): void {
  const ids = new Set<string>();
  effects.forEach((decl, index) => {
    const slotId = decl.slotId ?? `${rack}-${index + 1}`;
    stableId(slotId, `${path}.${rack}[${index}].slotId`);
    if (ids.has(slotId)) fail("Duplicate rack-local effect slot ID", `${path}.${rack}[${index}].slotId`);
    ids.add(slotId);
    const descriptor = effectDescriptor(decl.effectId, `${path}.${rack}[${index}].effectId`);
    for (const [name, value] of Object.entries(decl.parameters)) parameterValue(descriptor, name, value, decl.options.channel, `${path}.${rack}[${index}].parameters.${name}`);
  });
}

function validateAutomationShape(spec: AutomationSpec, path: string): void {
  stableId(spec.target.trackId, `${path}.target.trackId`);
  stableId(spec.target.slotId, `${path}.target.slotId`);
  if (!["simd1", "dynamic", "simd2"].includes(spec.target.rack)) fail("Unknown rack", `${path}.target.rack`);
  if (!["left", "right", "both"].includes(spec.target.channel)) fail("Unknown channel", `${path}.target.channel`);
  if (spec.segments.length === 0) fail("Automation requires at least one segment", `${path}.segments`);
  let previousStart: bigint | undefined;
  let previousEnd: bigint | undefined;
  for (const [index, segment] of spec.segments.entries()) {
    u64Toml(segment.startSample, `${path}.segments[${index}].startSample`);
    u64Toml(segment.endSample, `${path}.segments[${index}].endSample`);
    if (segment.endSample <= segment.startSample) fail("Automation segment must have a positive range", `${path}.segments[${index}].endSample`);
    if (previousStart !== undefined && segment.startSample < previousStart) fail("Automation segments must be ordered by startSample", `${path}.segments[${index}].startSample`);
    if (previousEnd !== undefined && segment.startSample < previousEnd) fail("Automation segments must not overlap", `${path}.segments[${index}].startSample`);
    f32(segment.startValue, `${path}.segments[${index}].startValue`); f32(segment.endValue, `${path}.segments[${index}].endValue`);
    if (segment.shape === "exponential" && (segment.startValue <= 0 || segment.endValue <= 0)) fail("Exponential automation requires positive values", `${path}.segments[${index}]`);
    previousStart = segment.startSample;
    previousEnd = segment.endSample;
  }
}

function u64Toml(value: unknown, path: string): bigint {
  if (typeof value !== "bigint" || value < 0n || value > TOML_I64_MAX) fail("Sample time must fit Session V1 TOML i64", path);
  return value;
}

function validateAutomationParameterValue(parameter: EffectDescriptor["parameters"][number], value: number, path: string): number {
  const normalized = f32(value, path);
  if (parameter.domainName === "boolean" && normalized !== 0 && normalized !== 1) fail("Boolean automation values must be 0 or 1", path, parameter as unknown as Record<string, unknown>);
  if (parameter.domainName === "enumeration" && !parameter.enumChoices.some((choice) => choice.value === normalized)) fail("Enumeration automation value is not a declared choice", path, parameter as unknown as Record<string, unknown>);
  if (parameter.domainName === "continuous" && (normalized < f32(parameter.minimum, `${path}.minimum`) || normalized > f32(parameter.maximum, `${path}.maximum`))) fail("Automation value is outside its local metadata domain", path, parameter as unknown as Record<string, unknown>);
  return normalized;
}

function normalize(state: BuilderState): SessionJson {
  const o = state.options;
  const explicitRoutes = state.routes.length > 0 || state.outputs.length > 0;
  if (!explicitRoutes && state.tracks.some((track) => track.id === "main")) fail("Synthesized output ID 'main' collides with a track ID", "outputs[0].id");
  const sources = state.sources.map(({ id, spec }) => freeze({ id, sample_rate_hz: spec.sampleRateHz ?? o.sampleRateHz, content: freeze({ identity: spec.identity ?? `source:${id}`, locator: spec.locator ?? `host:${id}` }), mapping: freeze({ channel_count: spec.channels, region: freeze({ start_sample: 0, length_samples: spec.frames }) }) })).sort(byId);
  const trackIds = new Set(state.tracks.map((track) => track.id));
  const tracks = state.tracks.map(({ id, spec }) => normalizeTrack(id, spec, state.sources, o.sampleRateHz)).sort(byId);
  const submixes = state.submixes.map((id) => freeze({ id })).sort(byId);
  const outputs = (explicitRoutes ? state.outputs.map((output) => freeze({ id: output.id })) : [freeze({ id: "main" })]).sort(byId);
  const routes = (explicitRoutes ? state.routes.map(normalizeRoute) : [...state.tracks].sort((a, b) => asciiCompare(a.id, b.id)).map(({ id }, index) => freeze({ id: `auto-route-${index + 1}`, source: freeze({ kind: "track", track_id: id, tap: "post_matrix" }), destination: freeze({ kind: "output_input", output_id: "main" }), channel_matrix: UNITY, gain_db: 0 }))).sort(byId);
  const automation = state.automation.map((item) => normalizeAutomation(item, tracks)).sort(byId);
  for (const track of tracks) if (!trackIds.has(track.id as string)) fail("Unknown track", "tracks");
  return jsonDocument({ schema_version: 1, session_id: o.sessionId, revision: o.revision, sample_rate_hz: o.sampleRateHz, quantum_frames: o.quantumFrames, render_profile: freeze({ id: o.renderProfile.id, mode: o.renderProfile.mode }), output_profile: freeze({ id: o.outputProfile.id, channels: o.outputProfile.channels, sample_format: o.outputProfile.sampleFormat }), limits: freeze({ pcm_ring_frames: o.limits.pcmRingFrames, control_queue_messages: o.limits.controlQueueMessages, memory_bytes: o.limits.memoryBytes }), sources, tracks, submixes, outputs, routes, automation } as SessionJson);
}

function normalizeTrack(id: string, spec: TrackSpec, sources: BuilderState["sources"], sampleRateHz: number): JsonRecord {
  validateTrackSource(spec, sources, `track.${id}.source`);
  const source = typeof spec.source === "string" ? sources.find((candidate) => candidate.id === spec.source) : undefined;
  const sourceRef = typeof spec.source === "string" ? { left: [spec.source, 0], right: [spec.source, source!.spec.channels === 1 ? 0 : 1] } : { left: spec.source.left, right: spec.source.right };
  const builtins = normalizeBuiltins(spec.builtins, sampleRateHz);
  const fader = freeze({ left_db: builtinNumber("fader_db", spec.fader?.leftDb ?? 0, `track.${id}.fader.leftDb`), right_db: builtinNumber("fader_db", spec.fader?.rightDb ?? 0, `track.${id}.fader.rightDb`), left_mute: builtinBoolean("mute", spec.fader?.leftMute ?? false, `track.${id}.fader.leftMute`), right_mute: builtinBoolean("mute", spec.fader?.rightMute ?? false, `track.${id}.fader.rightMute`) });
  const matrixOrPan = spec.pan && "matrix" in spec.pan ? { matrix: freeze({ ...trackBuiltinMatrix(spec.pan.matrix, `track.${id}.matrix`), smoothing_samples: u32(spec.pan.smoothingSamples ?? 0, `track.${id}.matrix.smoothingSamples`) }) } : { pan: freeze({ left: panValue(spec.pan && "left" in spec.pan ? spec.pan.left : 1, `track.${id}.pan.left`), right: panValue(spec.pan && "left" in spec.pan ? spec.pan.right : 1, `track.${id}.pan.right`), smoothing_samples: u32(spec.pan?.smoothingSamples ?? 0, `track.${id}.pan.smoothingSamples`) }) };
  return freeze({ id, source_id: sourceRef.left[0], left_source_channel: sourceRef.left[1], right_source_channel: sourceRef.right[1], builtins, simd1: freeze({ effects: normalizeRack(spec.simd1 ?? [], "simd1") }), dynamic: freeze({ effects: normalizeRack(spec.dynamic ?? [], "dynamic") }), simd2: freeze({ effects: normalizeRack(spec.simd2 ?? [], "simd2") }), fader, ...matrixOrPan });
}

function normalizeBuiltins(raw: TrackSpec["builtins"], sampleRateHz: number): JsonRecord {
  const left = raw && Array.isArray(raw) ? raw[0] : raw && isLaneValue(raw) ? raw.left : raw ?? DEFAULT_BUILTINS;
  const right = raw && Array.isArray(raw) ? raw[1] : raw && isLaneValue(raw) ? raw.right : raw ?? DEFAULT_BUILTINS;
  const lane = (value: BuiltinsSpec): JsonRecord => freeze({ polarity_invert: builtinBoolean("polarity_invert", value.polarityInvert ?? false, "builtins.polarityInvert"), trim_db: builtinNumber("trim_db", value.trimDb ?? 0, "builtins.trimDb"), hpf_hz: builtinFilter("hpf_hz", value.hpfHz ?? 0, sampleRateHz, "builtins.hpfHz"), lpf_hz: builtinFilter("lpf_hz", value.lpfHz ?? 0, sampleRateHz, "builtins.lpfHz") });
  return freeze({ left: lane(left), right: lane(right) });
}


function normalizeRack(effects: readonly EffectDecl[], rack: Rack): readonly JsonRecord[] {
  return effects.map((decl, index) => {
    const descriptor = effectDescriptor(decl.effectId, `rack.${rack}`);
    const parameters = Object.entries(decl.parameters).flatMap(([name, value]) => parameterValue(descriptor, name, value, decl.options.channel, `rack.${rack}.${name}`)).sort((a, b) => Number(a.parameter_id) - Number(b.parameter_id) || channelOrder(String(a.channel)) - channelOrder(String(b.channel)));
    return freeze({ id: decl.slotId ?? `${rack}-${index + 1}`, identity: freeze({ kind: "native", effect_id: decl.effectId }), quality: "normal", bypass: decl.options.bypass, link_mode: decl.options.linkMode, params: parameters, sidechain: decl.options.sidechain ? freeze({ kind: "routed", source: normalizeRouteSource(decl.options.sidechain.source), port_id: decl.options.sidechain.portId }) : freeze({ kind: "none" }) });
  });
}

function normalizeRoute(spec: RouteSpec): JsonRecord {
  return freeze({ id: spec.id, source: normalizeRouteSource(spec.source), destination: normalizeRouteDestination(spec.destination), channel_matrix: routeMatrix(spec.matrix ?? UNITY, `route.${spec.id}.matrix`), gain_db: f32(spec.gainDb ?? 0, `route.${spec.id}.gainDb`) });
}

function normalizeRouteSource(value: RouteSource): JsonRecord { return value.kind === "track" ? freeze({ kind: "track", track_id: value.trackId, tap: value.tap }) : freeze({ kind: "submix_output", submix_id: value.submixId }); }
function normalizeRouteDestination(value: RouteDestination): JsonRecord { return value.kind === "submix_input" ? freeze({ kind: "submix_input", submix_id: value.submixId }) : freeze({ kind: "output_input", output_id: value.outputId }); }

function normalizeAutomation(spec: AutomationSpec, tracks: readonly JsonRecord[]): JsonRecord {
  const track = tracks.find((candidate) => candidate.id === spec.target.trackId);
  if (!track) fail("Automation targets an unknown track", "automation.target.trackId");
  const rack = track[spec.target.rack] as JsonRecord;
  const effect = (rack.effects as readonly JsonRecord[]).find((candidate) => candidate.id === spec.target.slotId);
  if (!effect) fail("Automation targets an unknown rack-local effect slot", "automation.target.slotId");
  const descriptor = effectDescriptor((effect.identity as JsonRecord).effect_id as string, "automation.target.slotId");
  const parameter = descriptor.parameters.find((candidate) => candidate.name === spec.target.parameter);
  if (!parameter) fail("Automation targets an unknown parameter", "automation.target.parameter");
  if (parameter.channelPolicyName === "shared" && spec.target.channel !== "both") fail("Shared automation targets use channel 'both'", "automation.target.channel");
  if (!(effect.params as readonly JsonRecord[]).some((entry) => entry.parameter_id === parameter.id && entry.channel === spec.target.channel)) fail("Automation parameter/channel is absent from selected effect", "automation.target.parameter");
  const segments = spec.segments.map((segment, index) => freeze({ shape: segment.shape, start_sample: u64Toml(segment.startSample, `automation.segments[${index}].startSample`).toString(), end_sample: u64Toml(segment.endSample, `automation.segments[${index}].endSample`).toString(), start_value: validateAutomationParameterValue(parameter, segment.startValue, `automation.segments[${index}].startValue`), end_value: validateAutomationParameterValue(parameter, segment.endValue, `automation.segments[${index}].endValue`), unit: parameter.unitName }));
  return freeze({ id: spec.id, target: freeze({ entity_id: spec.target.trackId, rack: spec.target.rack, effect_id: spec.target.slotId, parameter_id: parameter.id, channel: spec.target.channel }), segments });
}

function validateGraph(json: SessionJson): void {
  const tracks = new Set(json.tracks.map((track) => String(track.id)));
  const submixes = new Set(json.submixes.map((submix) => String(submix.id)));
  const outputs = new Set(json.outputs.map((output) => String(output.id)));
  const graphIds = new Set<string>();
  for (const [kind, entities] of [["tracks", json.tracks], ["submixes", json.submixes], ["outputs", json.outputs]] as const) {
    for (const [index, entity] of entities.entries()) {
      const id = String(entity.id);
      if (graphIds.has(id)) fail("Track, submix, and output IDs share one namespace", `${kind}[${index}].id`);
      graphIds.add(id);
    }
  }
  const validateSource = (source: JsonRecord, path: string): void => {
    if (source.kind === "track" && !tracks.has(String(source.track_id))) fail("Source references an unknown track", `${path}.track_id`);
    if (source.kind === "submix_output" && !submixes.has(String(source.submix_id))) fail("Source references an unknown submix", `${path}.submix_id`);
  };
  for (const route of json.routes) {
    const source = route.source as JsonRecord; const destination = route.destination as JsonRecord;
    validateSource(source, "routes.source");
    if (destination.kind === "submix_input" && !submixes.has(String(destination.submix_id))) fail("Route references an unknown submix", "routes.destination.submix_id");
    if (destination.kind === "output_input" && !outputs.has(String(destination.output_id))) fail("Route references an unknown output", "routes.destination.output_id");
  }
  for (const [trackIndex, track] of json.tracks.entries()) {
    for (const rackName of ["simd1", "dynamic", "simd2"] as const) {
      const rack = track[rackName] as JsonRecord;
      for (const [effectIndex, effect] of (rack.effects as readonly JsonRecord[]).entries()) {
        const sidechain = effect.sidechain as JsonRecord;
        if (sidechain.kind === "routed") validateSource(sidechain.source as JsonRecord, `tracks[${trackIndex}].${rackName}.effects[${effectIndex}].sidechain.source`);
      }
    }
  }
}

function validateJson(json: SessionJson): void {
  if (json.schema_version !== 1) fail("Expected Session V1 JSON", "schema_version");
  stableId(json.session_id, "session_id"); rate(json.sample_rate_hz, "sample_rate_hz"); integer(json.revision, "revision"); u32(json.quantum_frames, "quantum_frames", 1);
  for (const field of ["sources", "tracks", "submixes", "outputs", "routes", "automation"] as const) if (!Array.isArray(json[field])) fail("Expected an array", field);
  for (const [automationIndex, automation] of json.automation.entries()) {
    const segments = automation.segments;
    if (!Array.isArray(segments)) fail("Expected an automation segment array", `automation[${automationIndex}].segments`);
    let previousStart: bigint | undefined;
    let previousEnd: bigint | undefined;
    for (const [segmentIndex, segment] of segments.entries()) {
      if (!segment || typeof segment !== "object") fail("Expected an automation segment", `automation[${automationIndex}].segments[${segmentIndex}]`);
      const record = segment as JsonRecord;
      const start = tomlI64String(record.start_sample, `automation[${automationIndex}].segments[${segmentIndex}].start_sample`);
      const end = tomlI64String(record.end_sample, `automation[${automationIndex}].segments[${segmentIndex}].end_sample`);
      if (end <= start) fail("Automation segment must have a positive range", `automation[${automationIndex}].segments[${segmentIndex}].end_sample`);
      if (previousStart !== undefined && start < previousStart) fail("Automation segments must be ordered by start_sample", `automation[${automationIndex}].segments[${segmentIndex}].start_sample`);
      if (previousEnd !== undefined && start < previousEnd) fail("Automation segments must not overlap", `automation[${automationIndex}].segments[${segmentIndex}].start_sample`);
      previousStart = start; previousEnd = end;
    }
  }
}

function tomlI64String(value: unknown, path: string): bigint {
  if (typeof value !== "string" || !/^(?:0|[1-9][0-9]*)$/.test(value)) fail("JSON automation sample time must be a decimal string", path);
  const parsed = BigInt(value);
  if (parsed > TOML_I64_MAX) fail("JSON automation sample time exceeds TOML i64", path);
  return parsed;
}

function asciiCompare(a: string, b: string): number { return a < b ? -1 : a > b ? 1 : 0; }
function byId(a: JsonRecord, b: JsonRecord): number { return asciiCompare(String(a.id), String(b.id)); }
function channelOrder(channel: string): number { return channel === "left" ? 0 : channel === "right" ? 1 : channel === "both" ? 2 : 3; }
function writeToml(json: SessionJson): string {
  const root = ["schema_version", "session_id", "revision", "sample_rate_hz", "quantum_frames", "render_profile", "output_profile", "limits", "sources", "tracks", "submixes", "outputs", "routes", "automation"] as const;
  return root.map((key) => `${key} = ${tomlValue(json[key], key, true)}\n`).join("");
}

const INTEGER_FIELDS = new Set(["schema_version", "revision", "sample_rate_hz", "quantum_frames", "pcm_ring_frames", "control_queue_messages", "memory_bytes", "channel_count", "start_sample", "length_samples", "left_source_channel", "right_source_channel", "channels", "smoothing_samples", "parameter_id"]);
function tomlValue(value: unknown, field?: string, rootArray = false): string {
  if (typeof value === "string") {
    if ((field === "start_sample" || field === "end_sample") && /^(?:0|[1-9][0-9]*)$/.test(value) && BigInt(value) <= TOML_I64_MAX) return value;
    return quote(value);
  }
  if (typeof value === "boolean") return value ? "true" : "false";
  if (typeof value === "number") return field && INTEGER_FIELDS.has(field) ? String(value) : number(value);
  if (Array.isArray(value)) {
    if (!rootArray) return `[${value.map((item) => tomlValue(item)).join(", ")}]`;
    return value.length === 0 ? "[\n]" : `[\n${value.map((item) => `  ${tomlValue(item)},`).join("\n")}\n]`;
  }
  if (value && typeof value === "object") return `{ ${Object.entries(value as Record<string, unknown>).map(([key, child]) => `${key} = ${tomlValue(child, key)}`).join(", ")} }`;
  fail("Cannot emit non-TOML Session V1 value", "toml");
}

function quote(value: string): string {
  for (let index = 0; index < value.length; index += 1) {
    const unit = value.charCodeAt(index);
    if (unit >= 0xd800 && unit <= 0xdbff) {
      const low = value.charCodeAt(index + 1);
      if (!(low >= 0xdc00 && low <= 0xdfff)) fail("String contains an unpaired high surrogate", "toml.string");
      index += 1;
    } else if (unit >= 0xdc00 && unit <= 0xdfff) {
      fail("String contains an unpaired low surrogate", "toml.string");
    }
  }
  return `"${value.replace(/[\\"\u0000-\u001f\u007f-\u009f]/g, (char) => ({ "\\": "\\\\", "\"": "\\\"", "\b": "\\b", "\t": "\\t", "\n": "\\n", "\f": "\\f", "\r": "\\r" }[char] ?? `\\u${char.charCodeAt(0).toString(16).padStart(4, "0").toUpperCase()}`))}"`;
}
function number(value: number): string {
  const normalized = Math.fround(value);
  if (Object.is(normalized, -0)) return "-0.0";
  const bits = f32Bits(normalized);
  if (bits === 0x15ae43fd) return "0.00000000000000000000000007038530691851209";
  if (bits === 0x95ae43fd) return "-0.00000000000000000000000007038530691851209";
  let text = shortestF32(normalized, bits);
  if (/[eE]/.test(text)) text = expandExponent(text);
  return text.includes(".") ? text : `${text}.0`;
}

function f32Bits(value: number): number {
  const bytes = new ArrayBuffer(4);
  const view = new DataView(bytes);
  view.setFloat32(0, value, true);
  return view.getUint32(0, true);
}

/** The shortest decimal that rounds through JS f64 back to these exact f32 bits. */
function shortestF32(value: number, bits: number): string {
  for (let precision = 1; precision <= 9; precision += 1) {
    const candidate = value.toPrecision(precision);
    if (f32Bits(Number(candidate)) === bits) return candidate;
  }
  fail("Unable to canonicalize finite f32", "toml.f32");
}
function expandExponent(value: string): string {
  const [coefficient, exponentText] = value.toLowerCase().split("e"); const exponent = Number(exponentText); const negative = coefficient.startsWith("-"); const unsignedCoefficient = negative ? coefficient.slice(1) : coefficient; const digits = unsignedCoefficient.replace(".", ""); const point = (unsignedCoefficient.indexOf(".") < 0 ? unsignedCoefficient.length : unsignedCoefficient.indexOf(".")) + exponent;
  const unsigned = point <= 0 ? `0.${"0".repeat(-point)}${digits}` : point >= digits.length ? `${digits}${"0".repeat(point - digits.length)}` : `${digits.slice(0, point)}.${digits.slice(point)}`;
  return negative ? `-${unsigned}` : unsigned;
}
