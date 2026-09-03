import { CATALOG } from "../generated/catalog.ts";
import type { BuiltinParameter, EffectDescriptor, EffectId } from "../generated/catalog.ts";
import { MisoUsageError } from "./errors.ts";
import { writeCanonicalSessionDocument } from "../internal/session-json.ts";
import type {
  AutomationSpec,
  AutomationTarget,
  BuiltinsSpec,
  Channel,
  EffectDecl,
  EffectOptions,
  EffectParamValues,
  Matrix2x2,
  RouteDestination,
  RouteSource,
  RouteSpec,
  SessionSampleRateHz,
  SourceSpec,
  TrackSpec,
} from "./types.ts";

/**
 * The Session V1 document builder.
 *
 * # What this file writes
 *
 * `docs/SESSION_SCHEMA_V1.md` is the authority and this is a producer for it, not a second opinion
 * about it. The root keys, their order, the per-record key order, the canonical sort keys and the
 * canonical text layout are all transcribed from the engine's own emit-side walk
 * (`crates/session/src/visit.rs`) and canonical writer
 * (`.../canonical.rs`), so a document this builder emits is byte-identical to what the engine
 * would write back for the same model. There is no JSON parser here and there never will be
 * (ruling 5438024085); reading a document is the engine's job, and `validate()` asks it.
 *
 * # What #241 deleted, and why nothing here remembers it
 *
 * The pre-#241 builder emitted a `limits` root key and a source shaped
 * `{ id, sample_rate_hz, content: { identity, locator }, mapping: { channel_count, region } }`.
 * Queue depth, source-ring size and memory budget are host policy rather than document fields, and
 * a source is now exactly `{ id, content, channels, bit_depth, frames }` with one document-wide
 * rate. Both deletions are structural here: there is no option to set a limit and no field to
 * write a locator into, so a caller cannot author the old shape by accident and then discover at
 * boot that the engine calls it an unknown key.
 *
 * # Immutability
 *
 * Every builder verb returns a new builder over a frozen state, so a partially configured session
 * can be shared, forked and reused without a caller having to defensively copy it. Normalization
 * is memoized per builder: `toJSON()` and `toJson()` are pure functions of the state, and running
 * them twice must cost once.
 *
 * # Declaration order is reference order
 *
 * Cross-entity references are checked at the verb that declares them rather than at `toJSON()`,
 * because an error that fires while the offending call is still on the stack is worth far more
 * than one that fires at the end naming a path. The cost is that a track must follow its source
 * and a route must follow both of its endpoints. That is stated, not silently required: every one
 * of those refusals names what was missing.
 */

/** A JSON-shaped normalized value. Floats keep `-0`; u64 sample times are decimal strings. */
export type ModelValue = string | number | boolean | ModelRecord | readonly ModelValue[];
export interface ModelRecord {
  readonly [key: string]: ModelValue;
}

/**
 * The normalized Session V1 model: the document as data, one key per schema field.
 *
 * This -- not the JSON text -- is what `assertSameSession` compares, because text equality would
 * also be testing the float speller and the indentation, and a plan-equality gate that goes red
 * when a comment moves is a gate people learn to ignore.
 */
export interface SessionModel extends ModelRecord {
  readonly schema_version: 1;
  readonly session_id: string;
  readonly revision: string;
  readonly sample_rate_hz: SessionSampleRateHz;
  readonly quantum_frames: number;
  readonly render_profile: ModelRecord;
  readonly output_profile: ModelRecord;
  readonly sources: readonly ModelRecord[];
  readonly tracks: readonly ModelRecord[];
  readonly submixes: readonly ModelRecord[];
  readonly outputs: readonly ModelRecord[];
  readonly routes: readonly ModelRecord[];
  readonly automation: readonly ModelRecord[];
}

export interface SessionOptions {
  /** Stable Session V1 identity. */
  readonly id: string;
  /** The one rate in the document. V1 has no per-source rate and no implicit conversion. */
  readonly sampleRateHz: SessionSampleRateHz;
  readonly revision?: number | bigint;
  /** Nonzero. Defaults to 128, the quantum every launch host actually renders. */
  readonly quantumFrames?: number;
}

const STABLE_ID = /^[a-z][a-z0-9._-]{0,126}$/;
const CONTENT_IDENTITY = /^sha256:[0-9a-f]{64}$/;
const LAUNCH_RATES: readonly number[] = [44_100, 48_000, 88_200, 96_000];
const U64_MAX = 18_446_744_073_709_551_615n;
const SEND_TAPS: ReadonlySet<string> = new Set([
  "input",
  "post_input_builtins",
  "post_simd1",
  "post_dynamic",
  "post_simd2_pre_fader",
  "post_fader",
  "post_matrix",
]);
const RACKS = ["simd1", "dynamic", "simd2"] as const;
const IDENTITY_MATRIX: Matrix2x2 = Object.freeze({ ll: 1, lr: 0, rl: 0, rr: 1 });

/**
 * The one `effect_id` a `rack = "builtins"` automation target may carry.
 *
 * The strip is a chassis, not a rack of instances, so it has nothing to identify -- but V1 has no
 * optional keys, so the field is written with the schema's fixed literal instead of omitted.
 */
const BUILTIN_STRIP_EFFECT_ID = "strip";

/**
 * Which schema unit token a builtin parameter's automation spans carry.
 *
 * The builtin rows carry a `mapping` rather than the `unitName` the effect rows carry, so the two
 * vocabularies are joined here. Booleans ride the `linear` token, which is what
 * `fixtures/session/v1/builtins-automation.json` writes for `polarity_invert`: the schema's unit
 * set has no boolean member and `linear` is the one that imposes no domain of its own.
 */
const BUILTIN_UNIT_BY_MAPPING: ReadonlyMap<string, string> = new Map([
  ["boolean", "linear"],
  ["decibelAmplitude", "db"],
  ["hertz", "hz"],
  ["linear", "linear"],
]);

function fail(path: string, message: string): never {
  throw new MisoUsageError(`${path}: ${message}`);
}

function freeze<T>(value: T): T {
  if (value !== null && typeof value === "object" && !Object.isFrozen(value)) {
    Object.freeze(value);
    for (const child of Object.values(value as Record<string, unknown>)) freeze(child);
  }
  return value;
}

function asciiCompare(a: string, b: string): number {
  return a < b ? -1 : a > b ? 1 : 0;
}

function byId(a: ModelRecord, b: ModelRecord): number {
  return asciiCompare(String(a.id), String(b.id));
}

/** `ParameterChannel`'s wire order: left, right, both. Canonical params sort by it. */
function channelOrder(channel: string): number {
  return channel === "left" ? 0 : channel === "right" ? 1 : 2;
}

function stableId(value: unknown, path: string): string {
  if (typeof value !== "string" || !STABLE_ID.test(value)) {
    fail(path, "a Session V1 stable ID must match [a-z][a-z0-9._-]{0,126}");
  }
  return value;
}

function integer(value: unknown, path: string, minimum: number, maximum: number): number {
  if (typeof value !== "number" || !Number.isSafeInteger(value) || value < minimum || value > maximum) {
    fail(path, `expected an integer in ${minimum}..=${maximum}`);
  }
  return value;
}

function bool(value: unknown, path: string): boolean {
  if (typeof value !== "boolean") fail(path, "expected a boolean");
  return value;
}

/**
 * Round a caller's `f64` to the `f32` the document will actually carry.
 *
 * The rounding happens here rather than at emit time so that `toJSON()` reports the value the
 * engine will see. A gate that compared unrounded inputs would call two sessions equal that render
 * differently, and one that compared the *text* would be comparing the speller.
 */
function f32(value: unknown, path: string): number {
  if (typeof value !== "number" || !Number.isFinite(value)) fail(path, "expected a finite number");
  const rounded = Math.fround(value);
  if (!Number.isFinite(rounded)) fail(path, "value is outside the finite f32 domain");
  return rounded;
}

function u64(value: unknown, path: string): bigint {
  const normalized = typeof value === "number"
    ? (Number.isSafeInteger(value) && value >= 0 ? BigInt(value) : undefined)
    : typeof value === "bigint" ? value : undefined;
  if (normalized === undefined || normalized < 0n || normalized > U64_MAX) {
    fail(path, "expected a nonnegative safe integer number or bigint through u64::MAX");
  }
  return normalized;
}

// -------------------------------------------------------------------------------------------
// Catalog access. Every domain below is read from the generated catalog rather than restated.
// -------------------------------------------------------------------------------------------

function builtin(name: string): BuiltinParameter {
  const row = CATALOG.builtins.parameters.find((candidate) => candidate.name === name);
  if (row === undefined) {
    throw new MisoUsageError(`the generated catalog has no builtin parameter ${name}`);
  }
  return row;
}

function effectDescriptor(effectId: string, path: string): EffectDescriptor {
  const found = CATALOG.effects.find((candidate) => candidate.id === effectId);
  if (found === undefined) fail(path, `unknown native effect '${effectId}'`);
  return found;
}

/** `'a', 'b' and 'c'` -- the candidate list a refusal quotes back. */
function nameList(names: readonly string[]): string {
  const quoted = names.map((name) => `'${name}'`);
  if (quoted.length <= 1) return quoted.join("");
  return `${quoted.slice(0, -1).join(", ")} and ${quoted[quoted.length - 1]}`;
}

/**
 * Resolve a routed sidechain's port against the effect's own declared port table (issue #278).
 *
 * Until the catalog published `ports`, this was the one session field a builder could not check:
 * a misspelled `portId` parsed, validated, compiled, and only then failed preparation with
 * `effect.sidechain.unknown_port` -- a refusal that names the code but not the line that wrote it.
 * The engine's three refusals are unmoved and remain the authority. What this adds is that the
 * same three become authoring-time refusals that name the legal ports while the offending
 * `effect()` call is still on the stack:
 *
 * - a port no descriptor declares                  -> `effect.sidechain.unknown_port`
 * - a port that exists but is not a sidechain input -> `effect.sidechain.unknown_port`
 * - a routed sidechain on an effect with none      -> `effect.sidechain.unexpected`
 */
function sidechainPort(descriptor: EffectDescriptor, portId: string, path: string): void {
  // Widened deliberately: `portId` arrives as `string` on this path precisely because the caller
  // may not have typechecked, so a narrow literal array could not be asked about it.
  const inputs: readonly string[] = descriptor.ports
    .filter((port) => port.roleName === "sidechainInput")
    .map((port) => port.id);
  if (inputs.length === 0) {
    fail(
      path,
      `${descriptor.id} declares no sidechain input port -- its ports are `
        + `${nameList(descriptor.ports.map((port) => port.id))} -- so it cannot take a routed `
        + `sidechain`,
    );
  }
  if (!inputs.includes(portId)) {
    const declared = descriptor.ports.find((port) => port.id === portId);
    fail(
      path,
      declared === undefined
        ? `${descriptor.id} has no port '${portId}'; its sidechain inputs are ${nameList(inputs)}`
        : `'${portId}' is ${descriptor.id}'s ${declared.roleName} port, not a sidechain input; `
          + `its sidechain inputs are ${nameList(inputs)}`,
    );
  }
}

/** A builtin whose catalog domain is a plain inclusive range: `trim_db`, `fader_db`, `pan`, ... */
function builtinNumber(name: string, value: unknown, path: string): number {
  const row = builtin(name);
  const normalized = f32(value, path);
  if (row.domain === "finiteInclusive" && row.minimum !== null && row.maximum !== null) {
    if (normalized < Math.fround(row.minimum) || normalized > Math.fround(row.maximum)) {
      fail(path, `${name} is outside its catalog domain [${row.minimum}, ${row.maximum}]`);
    }
  }
  return normalized;
}

/**
 * `hpf_hz` / `lpf_hz`: a disabled value, or a rate-keyed hertz range.
 *
 * The schema itself only asks for a finite nonnegative value here and defers the DSP relationship
 * to issue 007. The builder is stricter on purpose: it holds the descriptor's rate-keyed ceiling,
 * so a 96 kHz cutoff pasted into a 44.1 kHz session is refused while the caller can still see
 * which line did it, rather than becoming an unstable biquad the engine happily prepares.
 */
function builtinFilter(name: "hpf_hz" | "lpf_hz", value: unknown, rateHz: number, path: string): number {
  const row = builtin(name);
  const normalized = f32(value, path);
  if (normalized < 0) fail(path, `${name} must be a nonnegative hertz value`);
  const ceilings = row.maximumByRate as Readonly<Record<string, number>> | null;
  const maximum = ceilings?.[String(rateHz)];
  const minimum = row.minimum;
  const disabled = row.disabledValue;
  if (maximum === undefined || minimum === null || disabled === null) {
    throw new MisoUsageError(`the generated catalog's ${name} row cannot bound ${rateHz} Hz`);
  }
  if (normalized !== Math.fround(disabled)
    && (normalized < Math.fround(minimum) || normalized > Math.fround(maximum))) {
    fail(
      path,
      `${name} must be ${disabled} (disabled) or within [${minimum}, ${maximum}] at ${rateHz} Hz`,
    );
  }
  return normalized;
}

function builtinDefaultNumber(name: string): number {
  return Math.fround(builtin(name).default);
}

function builtinDefaultBoolean(name: string): boolean {
  return builtin(name).default !== 0;
}

// -------------------------------------------------------------------------------------------
// Effect declarations.
// -------------------------------------------------------------------------------------------

function isLanePair(value: unknown): value is Readonly<{ left: unknown; right: unknown }> {
  return typeof value === "object" && value !== null && !Array.isArray(value)
    && "left" in value && "right" in value;
}

function lanePair(value: unknown): readonly [unknown, unknown] | undefined {
  if (Array.isArray(value) && value.length === 2) return [value[0], value[1]];
  return isLanePair(value) ? [value.left, value.right] : undefined;
}

type EffectParameterRow = EffectDescriptor["parameters"][number];

function effectParameter(
  descriptor: EffectDescriptor,
  name: string,
  path: string,
): EffectParameterRow {
  const row = descriptor.parameters.find((candidate) => candidate.name === name);
  if (row === undefined) fail(path, `${descriptor.id} has no parameter '${name}'`);
  return row;
}

/** Normalize one declared parameter into the one or two `params` rows it becomes. */
function parameterRows(
  descriptor: EffectDescriptor,
  name: string,
  raw: unknown,
  channel: Channel,
  path: string,
): readonly ModelRecord[] {
  const row = effectParameter(descriptor, name, path);
  const pair = row.channelPolicyName === "perLane" ? lanePair(raw) : undefined;
  if (row.channelPolicyName === "shared" && channel !== "both") {
    fail(path, `${name} is a shared parameter and must be addressed as 'both'`);
  }
  const addressed: readonly (readonly [Channel, unknown])[] = pair === undefined
    ? [[channel, raw]]
    : [["left", pair[0]], ["right", pair[1]]];
  return addressed.map(([lane, value]) => freeze({
    parameter_id: row.id,
    channel: lane,
    unit: row.unitName,
    value: parameterScalar(row, value, `${path}${pair === undefined ? "" : `.${lane}`}`),
  }));
}

/** Map a display-unit value onto the `f32` the wire carries, per the row's declared domain. */
function parameterScalar(row: EffectParameterRow, value: unknown, path: string): number {
  if (row.domainName === "boolean") return bool(value, path) ? 1 : 0;
  if (row.domainName === "enumeration") {
    if (typeof value !== "string") fail(path, "expected an enumeration label");
    const choice = row.enumChoices.find((candidate) => candidate.label === value);
    if (choice === undefined) {
      fail(path, `'${value}' is not a declared choice for ${row.name}`);
    }
    return Math.fround(choice.value);
  }
  const normalized = f32(value, path);
  if (normalized < Math.fround(row.minimum) || normalized > Math.fround(row.maximum)) {
    fail(path, `${row.name} is outside its catalog domain [${row.minimum}, ${row.maximum}]`);
  }
  return normalized;
}

/**
 * Declare a native effect instance, validated against its generated descriptor.
 *
 * Parameters are given in display units and by name; the descriptor supplies the ABI id, the unit
 * token and the domain. A caller therefore never writes a parameter number, which is the only way
 * to keep a renumbered parameter from silently becoming a different knob.
 */
export function effect<E extends EffectId>(
  effectId: E,
  parameters: EffectParamValues<E> = {},
  options: EffectOptions<E> = {},
): EffectDecl<E> {
  const descriptor = effectDescriptor(effectId, "effect().effectId");
  const path = `effect("${effectId}")`;
  if (options.slotId !== undefined) stableId(options.slotId, `${path}.slotId`);
  if (options.bypass !== undefined) bool(options.bypass, `${path}.bypass`);
  if (options.quality !== undefined && options.quality !== "normal") {
    fail(`${path}.quality`, "launch native descriptors publish only the 'normal' quality row");
  }
  const linkMode = options.linkMode ?? "dual_mono";
  if (!["dual_mono", "maximum", "average"].includes(linkMode)) {
    fail(`${path}.linkMode`, "expected dual_mono, maximum or average");
  }
  const channel = options.channel ?? "both";
  if (!["left", "right", "both"].includes(channel)) {
    fail(`${path}.channel`, "expected left, right or both");
  }
  if (options.sidechain !== undefined) {
    validateRouteSource(options.sidechain.source, `${path}.sidechain.source`);
    // `portId` is typed to the descriptor's own sidechain inputs, so a caller who typechecks
    // cannot reach the refusals below. They are here for the caller who does not -- plain
    // JavaScript, a JSON round-trip, a value that arrived as `string` -- which is every caller
    // whose port the engine used to be the first thing to look at.
    const portId: unknown = options.sidechain.portId;
    if (typeof portId !== "string" || portId.length === 0) {
      fail(`${path}.sidechain.portId`, "a routed sidechain requires a nonempty port ID");
    }
    sidechainPort(descriptor, portId, `${path}.sidechain.portId`);
  }
  for (const [name, value] of Object.entries(parameters)) {
    parameterRows(descriptor, name, value, channel, `${path}.parameters.${name}`);
  }
  return freeze({
    effectId,
    ...(options.slotId === undefined ? {} : { slotId: options.slotId }),
    parameters: { ...parameters },
    options: {
      bypass: options.bypass ?? false,
      quality: "normal" as const,
      linkMode,
      channel,
      ...(options.sidechain === undefined ? {} : { sidechain: options.sidechain }),
    },
  }) as EffectDecl<E>;
}

// -------------------------------------------------------------------------------------------
// Route and graph shapes.
// -------------------------------------------------------------------------------------------

function validateRouteSource(value: RouteSource, path: string): void {
  if (value === null || typeof value !== "object") fail(path, "expected a tagged route source");
  if (value.kind === "track") {
    stableId(value.trackId, `${path}.trackId`);
    if (!SEND_TAPS.has(value.tap)) fail(`${path}.tap`, `unknown track tap '${value.tap}'`);
    return;
  }
  if (value.kind === "submix_output") {
    stableId(value.submixId, `${path}.submixId`);
    return;
  }
  fail(`${path}.kind`, "expected 'track' or 'submix_output'");
}

function validateRouteDestination(value: RouteDestination, path: string): void {
  if (value === null || typeof value !== "object") {
    fail(path, "expected a tagged route destination");
  }
  if (value.kind === "submix_input") {
    stableId(value.submixId, `${path}.submixId`);
    return;
  }
  if (value.kind === "output_input") {
    stableId(value.outputId, `${path}.outputId`);
    return;
  }
  fail(`${path}.kind`, "expected 'submix_input' or 'output_input'");
}

function normalizeRouteSource(value: RouteSource): ModelRecord {
  return value.kind === "track"
    ? freeze({ kind: "track", track_id: value.trackId, tap: value.tap })
    : freeze({ kind: "submix_output", submix_id: value.submixId });
}

function normalizeRouteDestination(value: RouteDestination): ModelRecord {
  return value.kind === "submix_input"
    ? freeze({ kind: "submix_input", submix_id: value.submixId })
    : freeze({ kind: "output_input", output_id: value.outputId });
}

// -------------------------------------------------------------------------------------------
// The builder.
// -------------------------------------------------------------------------------------------

interface NormalizedOptions {
  readonly sessionId: string;
  readonly revision: bigint;
  readonly sampleRateHz: SessionSampleRateHz;
  readonly quantumFrames: number;
}

interface SourceEntry {
  readonly id: string;
  readonly spec: SourceSpec;
}

interface TrackEntry {
  readonly id: string;
  readonly spec: TrackSpec;
}

interface BuilderState {
  readonly options: NormalizedOptions;
  readonly sources: readonly SourceEntry[];
  readonly tracks: readonly TrackEntry[];
  readonly submixes: readonly string[];
  readonly outputs: readonly string[];
  readonly routes: readonly RouteSpec[];
  readonly automation: readonly AutomationSpec[];
}

export class SessionBuilder {
  readonly #state: BuilderState;
  #model: SessionModel | undefined;

  constructor(state: BuilderState) {
    this.#state = state;
  }

  /** Declare a source. Sources have their own ID namespace, separate from graph entities. */
  source(id: string, spec: SourceSpec): SessionBuilder {
    stableId(id, "source().id");
    const path = `source("${id}")`;
    if (this.#state.sources.some((entry) => entry.id === id)) {
      fail(`${path}.id`, "a source with this ID is already declared");
    }
    if (spec === null || typeof spec !== "object") fail(path, "expected a source specification");
    if (spec.channels !== 1 && spec.channels !== 2) {
      fail(`${path}.channels`, "a V1 source is mono or dual-mono");
    }
    if (spec.bitDepth !== 16 && spec.bitDepth !== 24 && spec.bitDepth !== "32f") {
      fail(`${path}.bitDepth`, 'expected the token 16, 24 or "32f"');
    }
    if (u64(spec.frames, `${path}.frames`) === 0n) fail(`${path}.frames`, "expected a nonzero frame count");
    if (typeof spec.content !== "string" || !CONTENT_IDENTITY.test(spec.content)) {
      fail(`${path}.content`, "source content must match sha256:[0-9a-f]{64}");
    }
    return this.#next({
      sources: [...this.#state.sources, freeze({ id, spec: { ...spec } })],
    });
  }

  /**
   * Declare a track. Its source must already be declared, and its ID must be free in the
   * graph-entity namespace it shares with submixes and outputs.
   */
  track(id: string, spec: TrackSpec): SessionBuilder {
    stableId(id, "track().id");
    const path = `track("${id}")`;
    if (this.#graphIds().has(id)) {
      fail(`${path}.id`, "tracks, submixes and outputs share one ID namespace");
    }
    this.#validateTrack(spec, path);
    return this.#next({ tracks: [...this.#state.tracks, freeze({ id, spec: { ...spec } })] });
  }

  submix(id: string): SessionBuilder {
    stableId(id, "submix().id");
    if (this.#graphIds().has(id)) {
      fail(`submix("${id}").id`, "tracks, submixes and outputs share one ID namespace");
    }
    return this.#next({ submixes: [...this.#state.submixes, id] });
  }

  output(id: string): SessionBuilder {
    stableId(id, "output().id");
    if (this.#graphIds().has(id)) {
      fail(`output("${id}").id`, "tracks, submixes and outputs share one ID namespace");
    }
    return this.#next({ outputs: [...this.#state.outputs, id] });
  }

  /** Declare a route. Both endpoints must already be declared, with the right role. */
  route(spec: RouteSpec): SessionBuilder {
    stableId(spec?.id, "route().id");
    const path = `route("${spec.id}")`;
    if (this.#state.routes.some((route) => route.id === spec.id)) {
      fail(`${path}.id`, "a route with this ID is already declared");
    }
    validateRouteSource(spec.source, `${path}.source`);
    this.#resolveRouteSource(spec.source, `${path}.source`);
    validateRouteDestination(spec.destination, `${path}.destination`);
    if (spec.destination.kind === "submix_input") {
      if (!this.#state.submixes.includes(spec.destination.submixId)) {
        fail(`${path}.destination.submixId`, `'${spec.destination.submixId}' is not a declared submix`);
      }
    } else if (!this.#state.outputs.includes(spec.destination.outputId)) {
      fail(`${path}.destination.outputId`, `'${spec.destination.outputId}' is not a declared output`);
    }
    if (spec.matrix !== undefined) {
      for (const key of ["ll", "lr", "rl", "rr"] as const) {
        f32(spec.matrix[key], `${path}.matrix.${key}`);
      }
    }
    if (spec.gainDb !== undefined) f32(spec.gainDb, `${path}.gainDb`);
    return this.#next({ routes: [...this.#state.routes, freeze({ ...spec })] });
  }

  /**
   * Declare an automation span set.
   *
   * The whole table is consumed by nothing today -- no lowering reads it, for the strip or for any
   * effect rack -- so what this verb buys is authoring and round-tripping, not motion. The schema
   * says so plainly and so does this comment, because a builder that let a caller believe a fader
   * ride would render would be the more expensive kind of wrong.
   */
  automation(spec: AutomationSpec): SessionBuilder {
    stableId(spec?.id, "automation().id");
    const path = `automation("${spec.id}")`;
    if (this.#state.automation.some((entry) => entry.id === spec.id)) {
      fail(`${path}.id`, "an automation with this ID is already declared");
    }
    this.#validateAutomation(spec, path);
    return this.#next({
      automation: [...this.#state.automation, freeze({
        ...spec,
        segments: spec.segments.map((segment) => ({ ...segment })),
      })],
    });
  }

  /**
   * The normalized model: canonical ordering, `f32`-rounded values, schema key names.
   *
   * Named `toJSON` so `JSON.stringify(builder)` does the useful thing. Two caveats a caller should
   * know rather than discover: `JSON.stringify` renders `-0` as `0`, and u64 sample times are
   * decimal *strings* here precisely so that stringifying does not have to invent a lossy number
   * for them. `assertSameSession` compares the model, where both are exact.
   */
  toJSON(): SessionModel {
    this.#model ??= normalize(this.#state);
    return this.#model;
  }

  /** The canonical Session V1 text: LF endings, exactly one final newline. */
  toJson(): string {
    return writeCanonicalSessionDocument(this.toJSON());
  }

  #graphIds(): ReadonlySet<string> {
    return new Set([
      ...this.#state.tracks.map((entry) => entry.id),
      ...this.#state.submixes,
      ...this.#state.outputs,
    ]);
  }

  #source(id: string): SourceEntry | undefined {
    return this.#state.sources.find((entry) => entry.id === id);
  }

  #resolveRouteSource(source: RouteSource, path: string): void {
    if (source.kind === "track") {
      if (!this.#state.tracks.some((entry) => entry.id === source.trackId)) {
        fail(`${path}.trackId`, `'${source.trackId}' is not a declared track`);
      }
      return;
    }
    if (!this.#state.submixes.includes(source.submixId)) {
      fail(`${path}.submixId`, `'${source.submixId}' is not a declared submix`);
    }
  }

  #validateTrack(spec: TrackSpec, path: string): void {
    if (spec === null || typeof spec !== "object") fail(path, "expected a track specification");
    const reference = trackSourceRef(spec.source, `${path}.source`);
    const source = this.#source(reference.id);
    if (source === undefined) {
      fail(`${path}.source`, `'${reference.id}' is not a declared source`);
    }
    resolveLanes(reference, source.spec.channels, `${path}.source`);
    normalizeBuiltins(spec.builtins, this.#state.options.sampleRateHz, `${path}.builtins`);
    normalizeFader(spec.fader, `${path}.fader`);
    normalizeMatrixOrPan(spec.pan, `${path}.pan`);
    for (const rack of RACKS) normalizeRack(spec[rack] ?? [], rack, `${path}.${rack}`);
  }

  #validateAutomation(spec: AutomationSpec, path: string): void {
    const target = spec.target;
    if (target === null || typeof target !== "object") {
      fail(`${path}.target`, "expected an automation target");
    }
    stableId(target.trackId, `${path}.target.trackId`);
    const track = this.#state.tracks.find((entry) => entry.id === target.trackId);
    if (track === undefined) {
      fail(`${path}.target.trackId`, `'${target.trackId}' is not a declared track`);
    }
    if (!["left", "right", "both"].includes(target.channel)) {
      fail(`${path}.target.channel`, "expected left, right or both");
    }
    resolveAutomationTarget(target, track, `${path}.target`);
    if (spec.segments.length === 0) {
      fail(`${path}.segments`, "automation must declare at least one segment");
    }
  }

  #next(update: Partial<BuilderState>): SessionBuilder {
    return new SessionBuilder(freeze({ ...this.#state, ...update }));
  }
}

/** Start an immutable Session V1 builder. */
export function session(options: SessionOptions): SessionBuilder {
  if (options === null || typeof options !== "object") {
    fail("session()", "expected a session options object");
  }
  const sessionId = stableId(options.id, "session().id");
  if (!LAUNCH_RATES.includes(options.sampleRateHz)) {
    fail(
      "session().sampleRateHz",
      `${String(options.sampleRateHz)} is not a launch rate; expected one of ${LAUNCH_RATES.join(", ")}`,
    );
  }
  const revision = u64(options.revision ?? 0, "session().revision");
  const quantumFrames = integer(
    options.quantumFrames ?? 128,
    "session().quantumFrames",
    1,
    0xffff_ffff,
  );
  return new SessionBuilder(freeze({
    options: { sessionId, revision, sampleRateHz: options.sampleRateHz, quantumFrames },
    sources: [],
    tracks: [],
    submixes: [],
    outputs: [],
    routes: [],
    automation: [],
  }));
}

// -------------------------------------------------------------------------------------------
// Normalization.
// -------------------------------------------------------------------------------------------

/**
 * Read a track's source reference without yet knowing the source's channel count.
 *
 * The bare-string form leaves both lanes open, because "the whole source" means lane 0 for a mono
 * source and lanes 0/1 for a dual-mono one, and only the declaration knows which. `resolveLanes`
 * closes them once the source is in hand.
 */
function trackSourceRef(
  source: TrackSpec["source"],
  path: string,
): { readonly id: string; readonly left: number | undefined; readonly right: number | undefined } {
  if (typeof source === "string") {
    return { id: stableId(source, path), left: undefined, right: undefined };
  }
  if (source === null || typeof source !== "object") {
    fail(path, "expected a source ID or { id, left, right }");
  }
  return { id: stableId(source.id, `${path}.id`), left: source.left, right: source.right };
}

function resolveLanes(
  reference: { readonly left: number | undefined; readonly right: number | undefined },
  channels: number,
  path: string,
): { readonly left: number; readonly right: number } {
  return {
    left: integer(reference.left ?? 0, `${path}.left`, 0, channels - 1),
    right: integer(reference.right ?? (channels === 1 ? 0 : 1), `${path}.right`, 0, channels - 1),
  };
}

function laneSpecs(raw: TrackSpec["builtins"]): readonly [BuiltinsSpec, BuiltinsSpec] {
  if (raw === undefined) return [{}, {}];
  if (Array.isArray(raw)) return [raw[0] as BuiltinsSpec, raw[1] as BuiltinsSpec];
  if (isLanePair(raw)) {
    return [raw.left as BuiltinsSpec, raw.right as BuiltinsSpec];
  }
  return [raw as BuiltinsSpec, raw as BuiltinsSpec];
}

function normalizeBuiltins(
  raw: TrackSpec["builtins"],
  sampleRateHz: number,
  path: string,
): ModelRecord {
  const [left, right] = laneSpecs(raw);
  const lane = (spec: BuiltinsSpec, name: string): ModelRecord => freeze({
    polarity_invert: spec.polarityInvert === undefined
      ? builtinDefaultBoolean("polarity_invert")
      : bool(spec.polarityInvert, `${path}.${name}.polarityInvert`),
    trim_db: spec.trimDb === undefined
      ? builtinDefaultNumber("trim_db")
      : builtinNumber("trim_db", spec.trimDb, `${path}.${name}.trimDb`),
    hpf_hz: spec.hpfHz === undefined
      ? builtinDefaultNumber("hpf_hz")
      : builtinFilter("hpf_hz", spec.hpfHz, sampleRateHz, `${path}.${name}.hpfHz`),
    lpf_hz: spec.lpfHz === undefined
      ? builtinDefaultNumber("lpf_hz")
      : builtinFilter("lpf_hz", spec.lpfHz, sampleRateHz, `${path}.${name}.lpfHz`),
    delay_samples: spec.delaySamples === undefined
      ? builtin("delay_samples").default
      : integer(
        spec.delaySamples,
        `${path}.${name}.delaySamples`,
        builtin("delay_samples").minimum ?? 0,
        builtin("delay_samples").maximum ?? 0,
      ),
  });
  return freeze({ left: lane(left ?? {}, "left"), right: lane(right ?? {}, "right") });
}

function normalizeFader(raw: TrackSpec["fader"], path: string): ModelRecord {
  return freeze({
    left_db: raw?.leftDb === undefined
      ? builtinDefaultNumber("fader_db")
      : builtinNumber("fader_db", raw.leftDb, `${path}.leftDb`),
    right_db: raw?.rightDb === undefined
      ? builtinDefaultNumber("fader_db")
      : builtinNumber("fader_db", raw.rightDb, `${path}.rightDb`),
    left_mute: raw?.leftMute === undefined
      ? builtinDefaultBoolean("mute")
      : bool(raw.leftMute, `${path}.leftMute`),
    right_mute: raw?.rightMute === undefined
      ? builtinDefaultBoolean("mute")
      : bool(raw.rightMute, `${path}.rightMute`),
  });
}

/**
 * The `pan`-or-`matrix` variant, and the key it is written under.
 *
 * Both spellings occupy schema field 10; a track carries one or the other, never both. An absent
 * `pan` takes the `pan` builtin's own catalog default for each lane rather than a number this
 * file invented -- the SDK holds no default the engine does not publish.
 */
function normalizeMatrixOrPan(
  raw: TrackSpec["pan"],
  path: string,
): { readonly key: "pan" | "matrix"; readonly value: ModelRecord } {
  const smoothing = integer(raw?.smoothingSamples ?? 0, `${path}.smoothingSamples`, 0, 0xffff_ffff);
  if (raw !== undefined && "matrix" in raw) {
    const coefficient = (key: "ll" | "lr" | "rl" | "rr"): number =>
      builtinNumber(`matrix_${key}`, raw.matrix[key], `${path}.matrix.${key}`);
    return {
      key: "matrix",
      value: freeze({
        ll: coefficient("ll"),
        lr: coefficient("lr"),
        rl: coefficient("rl"),
        rr: coefficient("rr"),
        smoothing_samples: smoothing,
      }),
    };
  }
  return {
    key: "pan",
    value: freeze({
      left: raw === undefined
        ? builtinDefaultNumber("pan")
        : builtinNumber("pan", raw.left, `${path}.left`),
      right: raw === undefined
        ? builtinDefaultNumber("pan")
        : builtinNumber("pan", raw.right, `${path}.right`),
      smoothing_samples: smoothing,
    }),
  };
}

/** A rack's effects, in declared order. Only the *entities* sort; racks preserve signal order. */
function normalizeRack(
  effects: readonly EffectDecl[],
  rack: (typeof RACKS)[number],
  path: string,
): readonly ModelRecord[] {
  const slots = new Set<string>();
  return effects.map((decl, index) => {
    const where = `${path}[${index}]`;
    if (decl === null || typeof decl !== "object") {
      fail(where, "expected an effect declaration from effect()");
    }
    const slotId = stableId(decl.slotId ?? `${rack}-${index + 1}`, `${where}.slotId`);
    if (slots.has(slotId)) fail(`${where}.slotId`, `'${slotId}' is repeated in the ${rack} rack`);
    slots.add(slotId);
    const descriptor = effectDescriptor(decl.effectId, `${where}.effectId`);
    const params = Object.entries(decl.parameters)
      .flatMap(([name, value]) =>
        parameterRows(descriptor, name, value, decl.options.channel, `${where}.parameters.${name}`))
      .sort((a, b) =>
        Number(a.parameter_id) - Number(b.parameter_id)
        || channelOrder(String(a.channel)) - channelOrder(String(b.channel)));
    for (const [position, row] of params.entries()) {
      const previous = params[position - 1];
      if (previous !== undefined
        && previous.parameter_id === row.parameter_id && previous.channel === row.channel) {
        fail(where, `parameter ${String(row.parameter_id)} is addressed twice on ${String(row.channel)}`);
      }
    }
    const sidechain = decl.options.sidechain;
    return freeze({
      id: slotId,
      identity: { kind: "native", effect_id: decl.effectId },
      quality: decl.options.quality,
      bypass: decl.options.bypass,
      link_mode: decl.options.linkMode,
      params,
      sidechain: sidechain === undefined
        ? { kind: "none" }
        : {
          kind: "routed",
          source: normalizeRouteSource(sidechain.source),
          port_id: sidechain.portId,
        },
    });
  });
}

interface ResolvedAutomationTarget {
  readonly parameterId: number;
  readonly unit: string;
  readonly effectId: string;
  /** Absent for the strip, whose values are not bounded by an effect descriptor row. */
  readonly row: EffectParameterRow | undefined;
}

/**
 * Resolve an automation target to its ABI parameter id and unit token.
 *
 * The two racks answer differently on purpose. An effect target is resolved through the *declared
 * instance*: the slot must exist in that rack and the `(parameter_id, channel)` pair must already
 * appear in that instance's params, which is exactly what the engine checks. A `builtins` target
 * has no instance, so it is resolved against the builtin parameter ABI and restricted to the rows
 * that declare `blockTarget` -- `hpf_hz`, `lpf_hz` and `delay_samples` are prepared-only and a
 * span addressed at one of them could only ever be inert, so the schema refuses it and so does
 * this.
 */
function resolveAutomationTarget(
  target: AutomationTarget,
  track: TrackEntry,
  path: string,
): ResolvedAutomationTarget {
  if (target.rack === "builtins") {
    if (target.slotId !== undefined && target.slotId !== BUILTIN_STRIP_EFFECT_ID) {
      fail(`${path}.slotId`, `builtins automation must name the strip, not '${target.slotId}'`);
    }
    const row = CATALOG.builtins.parameters.find((candidate) => candidate.name === target.parameter);
    if (row === undefined) {
      fail(`${path}.parameter`, `'${target.parameter}' is not a builtin parameter`);
    }
    if (row.updateRate !== "blockTarget") {
      fail(
        `${path}.parameter`,
        `${row.name} is prepared-only, so a span addressed at it could only ever be inert`,
      );
    }
    if (row.scope !== "perLane" && target.channel !== "both") {
      fail(`${path}.channel`, `${row.name} is one shared value and is addressed as 'both'`);
    }
    const unit = BUILTIN_UNIT_BY_MAPPING.get(row.mapping);
    if (unit === undefined) {
      throw new MisoUsageError(`no schema unit token for builtin mapping ${row.mapping}`);
    }
    return { parameterId: row.id, unit, effectId: BUILTIN_STRIP_EFFECT_ID, row: undefined };
  }
  if (!RACKS.includes(target.rack)) {
    fail(`${path}.rack`, "expected simd1, dynamic, simd2 or builtins");
  }
  const slotId = stableId(target.slotId, `${path}.slotId`);
  const declared = track.spec[target.rack] ?? [];
  const index = declared.findIndex((decl, position) =>
    (decl.slotId ?? `${target.rack}-${position + 1}`) === slotId);
  const decl = declared[index];
  if (decl === undefined) {
    fail(`${path}.slotId`, `'${slotId}' is not an effect in ${track.id}'s ${target.rack} rack`);
  }
  const descriptor = effectDescriptor(decl.effectId, `${path}.slotId`);
  const row = effectParameter(descriptor, target.parameter, `${path}.parameter`);
  if (row.channelPolicyName === "shared" && target.channel !== "both") {
    fail(`${path}.channel`, `${row.name} is a shared parameter and is addressed as 'both'`);
  }
  const addressed = parameterRows(
    descriptor,
    target.parameter,
    decl.parameters[target.parameter as keyof typeof decl.parameters],
    decl.options.channel,
    `${path}.parameter`,
  );
  if (decl.parameters[target.parameter as keyof typeof decl.parameters] === undefined
    || !addressed.some((entry) => entry.channel === target.channel)) {
    fail(
      `${path}.parameter`,
      `${row.name} is not declared on '${slotId}' for channel '${target.channel}'`,
    );
  }
  return { parameterId: row.id, unit: row.unitName, effectId: slotId, row };
}

function normalizeAutomation(spec: AutomationSpec, tracks: readonly TrackEntry[]): ModelRecord {
  const path = `automation("${spec.id}")`;
  const track = tracks.find((entry) => entry.id === spec.target.trackId);
  if (track === undefined) {
    fail(`${path}.target.trackId`, `'${spec.target.trackId}' is not a declared track`);
  }
  const resolved = resolveAutomationTarget(spec.target, track, `${path}.target`);
  let previousStart: bigint | undefined;
  let previousEnd: bigint | undefined;
  const segments = spec.segments.map((segment, index) => {
    const where = `${path}.segments[${index}]`;
    if (!["step", "linear", "exponential"].includes(segment.shape)) {
      fail(`${where}.shape`, "expected step, linear or exponential");
    }
    const start = u64(segment.startSample, `${where}.startSample`);
    const end = u64(segment.endSample, `${where}.endSample`);
    if (end <= start) fail(`${where}.endSample`, "endSample must be greater than startSample");
    if (previousStart !== undefined && start < previousStart) {
      fail(`${where}.startSample`, "segments must be declared in nondecreasing start order");
    }
    if (previousEnd !== undefined && start < previousEnd) {
      fail(`${where}.startSample`, "segments must not overlap their predecessor");
    }
    previousStart = start;
    previousEnd = end;
    const value = (raw: number, field: string): number => {
      const normalized = resolved.row === undefined
        ? f32(raw, `${where}.${field}`)
        : automationScalar(resolved.row, raw, `${where}.${field}`);
      if (segment.shape === "exponential" && normalized <= 0) {
        fail(`${where}.${field}`, "exponential segments require positive values");
      }
      return normalized;
    };
    return freeze({
      shape: segment.shape,
      start_sample: start.toString(),
      end_sample: end.toString(),
      start_value: value(segment.startValue, "startValue"),
      end_value: value(segment.endValue, "endValue"),
      unit: resolved.unit,
    });
  });
  return freeze({
    id: spec.id,
    target: {
      entity_id: spec.target.trackId,
      rack: spec.target.rack,
      effect_id: resolved.effectId,
      parameter_id: resolved.parameterId,
      channel: spec.target.channel,
    },
    segments,
  });
}

/** An automation value is already in wire units, so an enum rides its numeric value. */
function automationScalar(row: EffectParameterRow, raw: number, path: string): number {
  const value = f32(raw, path);
  if (row.domainName === "boolean" && value !== 0 && value !== 1) {
    fail(path, `${row.name} is boolean, so a span value must be 0 or 1`);
  }
  if (row.domainName === "enumeration"
    && !row.enumChoices.some((choice) => Math.fround(choice.value) === value)) {
    fail(path, `${value} is not a declared choice for ${row.name}`);
  }
  if (row.domainName === "continuous"
    && (value < Math.fround(row.minimum) || value > Math.fround(row.maximum))) {
    fail(path, `${row.name} is outside its catalog domain [${row.minimum}, ${row.maximum}]`);
  }
  return value;
}

function normalizeTrack(
  entry: TrackEntry,
  sources: readonly SourceEntry[],
  sampleRateHz: number,
): ModelRecord {
  const path = `track("${entry.id}")`;
  const spec = entry.spec;
  const reference = trackSourceRef(spec.source, `${path}.source`);
  const source = sources.find((candidate) => candidate.id === reference.id);
  if (source === undefined) fail(`${path}.source`, `'${reference.id}' is not a declared source`);
  const lanes = resolveLanes(reference, source.spec.channels, `${path}.source`);
  const matrixOrPan = normalizeMatrixOrPan(spec.pan, `${path}.pan`);
  return freeze({
    id: entry.id,
    source_id: reference.id,
    left_source_channel: lanes.left,
    right_source_channel: lanes.right,
    builtins: normalizeBuiltins(spec.builtins, sampleRateHz, `${path}.builtins`),
    simd1: { effects: normalizeRack(spec.simd1 ?? [], "simd1", `${path}.simd1`) },
    dynamic: { effects: normalizeRack(spec.dynamic ?? [], "dynamic", `${path}.dynamic`) },
    simd2: { effects: normalizeRack(spec.simd2 ?? [], "simd2", `${path}.simd2`) },
    fader: normalizeFader(spec.fader, `${path}.fader`),
    [matrixOrPan.key]: matrixOrPan.value,
  });
}

function normalize(state: BuilderState): SessionModel {
  const options = state.options;
  const sources = state.sources
    .map(({ id, spec }) => freeze({
      id,
      content: spec.content,
      channels: spec.channels,
      bit_depth: spec.bitDepth,
      frames: u64(spec.frames, `source("${id}").frames`).toString(),
    }))
    .sort(byId);
  const tracks = state.tracks
    .map((entry) => normalizeTrack(entry, state.sources, options.sampleRateHz))
    .sort(byId);
  const submixes = state.submixes.map((id) => freeze({ id })).sort(byId);
  const outputs = state.outputs.map((id) => freeze({ id })).sort(byId);
  const routes = state.routes
    .map((spec) => freeze({
      id: spec.id,
      source: normalizeRouteSource(spec.source),
      destination: normalizeRouteDestination(spec.destination),
      channel_matrix: matrixRecord(spec.matrix ?? IDENTITY_MATRIX, `route("${spec.id}").matrix`),
      gain_db: f32(spec.gainDb ?? 0, `route("${spec.id}").gainDb`),
    }))
    .sort(byId);
  const automation = state.automation
    .map((spec) => normalizeAutomation(spec, state.tracks))
    .sort(byId);
  return freeze({
    schema_version: 1,
    session_id: options.sessionId,
    revision: options.revision.toString(),
    sample_rate_hz: options.sampleRateHz,
    quantum_frames: options.quantumFrames,
    render_profile: { id: "native", mode: "single_thread" },
    output_profile: { id: "main", channels: 2, sample_format: "f32_planar" },
    sources,
    tracks,
    submixes,
    outputs,
    routes,
    automation,
  }) as SessionModel;
}

function matrixRecord(matrix: Matrix2x2, path: string): ModelRecord {
  return freeze({
    ll: f32(matrix.ll, `${path}.ll`),
    lr: f32(matrix.lr, `${path}.lr`),
    rl: f32(matrix.rl, `${path}.rl`),
    rr: f32(matrix.rr, `${path}.rr`),
  });
}

// -------------------------------------------------------------------------------------------
// The plan-equality gate.
// -------------------------------------------------------------------------------------------

/** Anything that can stand in for a built session: a builder, or a model it produced. */
export type SessionLike = SessionModel | { toJSON(): SessionModel };

function modelOf(value: SessionLike, side: string): SessionModel {
  const candidate: unknown = value;
  if (candidate === null || typeof candidate !== "object") {
    throw new MisoUsageError(`assertSameSession(${side}) is not a built Session V1`);
  }
  const record = candidate as { readonly toJSON?: unknown; readonly schema_version?: unknown };
  if (typeof record.toJSON === "function") return (record.toJSON as () => SessionModel)();
  if (record.schema_version !== 1) {
    throw new MisoUsageError(`assertSameSession(${side}) is not a built Session V1`);
  }
  return candidate as SessionModel;
}

/**
 * The permanent plan-equality gate: two built sessions must normalize to the same document.
 *
 * # Why the model rather than the text
 *
 * Comparing `toJson()` output would make every producer of a session also a hostage to the float
 * speller and the indentation. The model is the thing two producers have to agree on, so that is
 * what is compared -- with `Object.is`, so `-0` and `0` are the different values they are.
 *
 * # What the source rows are, post-#241
 *
 * The gate walks whatever the schema declares, so its per-source rows moved with the schema: the
 * pre-#241 `sampleRateHz` and `startFrame` comparisons are gone with the fields, and `content` and
 * `bit_depth` are compared in their place (issue #243 S1). `bit_depth` in particular is compared
 * by identity across the whole token set, so `16` and `"16"` are a difference and not a match --
 * adopted-ruling finding 6 is only worth anything if the gate can see the token, not just a
 * number.
 *
 * The first difference found wins and its path is in the message, because a gate that reports
 * "these differ" without saying where is a gate that costs an afternoon.
 */
export function assertSameSession(a: SessionLike, b: SessionLike): void {
  const difference = firstDifference(modelOf(a, "a"), modelOf(b, "b"), "");
  if (difference !== undefined) {
    throw new MisoUsageError(
      `sessions differ at ${difference.path}: ${difference.left} !== ${difference.right}`,
    );
  }
}

interface Difference {
  readonly path: string;
  readonly left: string;
  readonly right: string;
}

function describe(value: ModelValue | undefined): string {
  if (value === undefined) return "absent";
  if (typeof value === "string") return JSON.stringify(value);
  if (typeof value === "number") return Object.is(value, -0) ? "-0" : String(value);
  if (typeof value === "boolean") return String(value);
  if (Array.isArray(value)) return `an array of ${value.length}`;
  return "a table";
}

function join(path: string, key: string): string {
  return path === "" ? key : `${path}.${key}`;
}

function firstDifference(
  left: ModelValue | undefined,
  right: ModelValue | undefined,
  path: string,
): Difference | undefined {
  if (Array.isArray(left) || Array.isArray(right)) {
    if (!Array.isArray(left) || !Array.isArray(right) || left.length !== right.length) {
      return { path, left: describe(left), right: describe(right) };
    }
    for (let index = 0; index < left.length; index += 1) {
      const found = firstDifference(left[index], right[index], `${path}[${index}]`);
      if (found !== undefined) return found;
    }
    return undefined;
  }
  const leftIsRecord = typeof left === "object" && left !== null;
  const rightIsRecord = typeof right === "object" && right !== null;
  if (leftIsRecord || rightIsRecord) {
    if (!leftIsRecord || !rightIsRecord) {
      return { path, left: describe(left), right: describe(right) };
    }
    const leftRecord = left as ModelRecord;
    const rightRecord = right as ModelRecord;
    // Key *order* is canonical, so a differing order is a real difference and is reported at the
    // first position where the two disagree rather than smoothed over by a set comparison.
    const keys = [...new Set([...Object.keys(leftRecord), ...Object.keys(rightRecord)])];
    for (const key of keys) {
      const found = firstDifference(leftRecord[key], rightRecord[key], join(path, key));
      if (found !== undefined) return found;
    }
    return undefined;
  }
  return Object.is(left, right) ? undefined : { path, left: describe(left), right: describe(right) };
}
