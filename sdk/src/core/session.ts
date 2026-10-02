import { CATALOG } from "../generated/catalog.ts";
import type { BuiltinParameter, EffectDescriptor, EffectId } from "../generated/catalog.ts";
import { MisoUsageError } from "./errors.ts";
import { writeCanonicalSessionDocument } from "../internal/session-json.ts";
import type {
  AutomationSpec,
  AutomationTarget,
  BuiltinsSpec,
  Channel,
  ConsoleEffectId,
  ConsoleEntrySpec,
  ConsoleSlotSpec,
  ConsoleSpec,
  EffectDecl,
  EffectOptions,
  EffectParamValues,
  Matrix2x2,
  RouteDestination,
  RouteSource,
  RouteSpec,
  SessionSampleRateHz,
  SourceSpec,
  SubmixSpec,
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
 * and the session console, and a route must follow both of its endpoints. That is stated, not
 * silently required: every one of those refusals names what was missing.
 *
 * # The console strip (owner decision 12)
 *
 * The session declares its console once, with `.console()`: the slots before the insert point
 * (`preInsert`) and after it (`postInsert`), each with a stable slot ID, a native effect, and the
 * quality and link mode every track runs it at. Every track then carries exactly one `console`
 * entry per slot, in slot order, with only its own bypass and parameters, and its own `inserts`.
 * The builder refuses what the engine refuses here -- a missing, repeated or misordered entry, an
 * unknown slot, an ineligible effect, a sidechain or any other key a slot does not have -- and it
 * refuses with the engine's own diagnostic code (`MisoUsageError.diagnosticCode`), so an agent
 * meets one vocabulary whether the builder or the engine answered.
 *
 * # Submix strips (owner decision 13)
 *
 * A submix carries a track's strip without the source fields -- builtins, console entries, inserts,
 * fader, and pan or matrix -- under the same rules, so `.submix(id, spec)` follows `.console()` as a
 * track does. `.submix(id)` with no spec is the transparent strip, whose console entries are all
 * bypassed and are written for whatever slots the session declares. A route or a routed sidechain
 * reads any of a track's or a submix's seven taps.
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
  readonly console: ModelRecord;
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
const CONTENT_IDENTITY = /^blake3:[0-9a-f]{64}$/;
const LAUNCH_RATES: readonly number[] = [44_100, 48_000, 88_200, 96_000];
const U64_MAX = 18_446_744_073_709_551_615n;
const SEND_TAPS: ReadonlySet<string> = new Set([
  "input",
  "post_input",
  "insert_send",
  "insert_return",
  "pre_fader",
  "post_fader",
  "post_pan",
]);
const IDENTITY_MATRIX: Matrix2x2 = Object.freeze({ ll: 1, lr: 0, rl: 0, rr: 1 });

/**
 * The one `effect_id` a `rack = "builtins"` automation target may carry.
 *
 * The strip is a chassis, not a rack of instances, so it has nothing to identify -- but V1 has no
 * optional keys, so the field is written with the schema's fixed literal instead of omitted.
 */
const BUILTIN_STRIP_EFFECT_ID = "strip";

/**
 * The native effects a console slot may name (owner decision 12, "Eligibility").
 *
 * This is the engine's `effect_compiler::CONSOLE_ELIGIBLE_EFFECTS`. The parameter metadata does
 * not publish it, so the SDK holds a copy, and `console-evals.mjs` holds the copy to the engine:
 * it boots every catalog effect as a console slot and requires the engine to accept exactly these
 * and refuse every other with `console.slot.ineligible_effect`.
 */
export const CONSOLE_ELIGIBLE_EFFECTS: readonly ConsoleEffectId[] = Object.freeze([
  "miso.parametric-eq",
  "miso.compressor",
  "miso.gate-expander",
  "miso.soft-clip",
  "miso.transient-shaper",
  "miso.true-peak-limiter",
]);

/** The keys a track specification may carry. The retired `simd1`, `dynamic` and `simd2` refuse. */
const TRACK_KEYS: ReadonlySet<string> = new Set([
  "source", "builtins", "console", "inserts", "fader", "pan",
]);
/** A submix strip's keys: a track's without its source. */
const SUBMIX_KEYS: ReadonlySet<string> = new Set(["builtins", "console", "inserts", "fader", "pan"]);
const CONSOLE_KEYS: ReadonlySet<string> = new Set(["preInsert", "postInsert"]);
const CONSOLE_SLOT_KEYS: ReadonlySet<string> = new Set(["slot", "effectId", "quality", "linkMode"]);
const CONSOLE_ENTRY_KEYS: ReadonlySet<string> = new Set(["slot", "bypass", "parameters", "channel"]);
const LINK_MODES: readonly string[] = ["dual_mono", "maximum", "average"];

type LinkMode = NonNullable<EffectOptions["linkMode"]>;

/**
 * The link modes each native effect supports, wherever it sits (a console slot or an insert).
 *
 * This is each engine effect descriptor's `supported_link_modes`. The parameter metadata does not
 * publish it, so the SDK holds a copy, as it holds `CONSOLE_ELIGIBLE_EFFECTS`, and
 * `console-evals.mjs` holds the copy to the engine: it boots every catalog effect at every link
 * mode and requires the engine to accept exactly these and refuse the rest with
 * `effect.link_mode.unsupported`. Keyed by every catalog effect, so a new one does not typecheck
 * until it has a row.
 */
export const EFFECT_LINK_MODES: Readonly<Record<EffectId, readonly LinkMode[]>> = Object.freeze({
  "miso.compressor": Object.freeze(["dual_mono", "maximum", "average"] as const),
  "miso.delay": Object.freeze(["dual_mono"] as const),
  "miso.gate-expander": Object.freeze(["dual_mono", "maximum", "average"] as const),
  "miso.multiband-compressor": Object.freeze(["dual_mono", "maximum", "average"] as const),
  "miso.parametric-eq": Object.freeze(["dual_mono"] as const),
  "miso.soft-clip": Object.freeze(["dual_mono"] as const),
  "miso.transient-shaper": Object.freeze(["dual_mono", "maximum", "average"] as const),
  "miso.true-peak-limiter": Object.freeze(["dual_mono", "maximum"] as const),
});

/** The engine's diagnostic codes for the refusals the builder makes in front of it. */
const CODE = Object.freeze({
  unknownField: "schema.unknown_field",
  invalidEnum: "schema.invalid_enum",
  invalidId: "id.invalid",
  duplicateId: "id.duplicate",
  missingEntity: "reference.missing_entity",
  entryOrder: "console.entry_order",
  entryMissing: "console.entry_missing",
  ineligibleEffect: "console.slot.ineligible_effect",
  wrongType: "schema.wrong_type",
  qualityUnsupported: "effect.quality.unsupported",
  linkModeUnsupported: "effect.link_mode.unsupported",
} as const);

function fail(path: string, message: string, code?: string): never {
  throw new MisoUsageError(`${path}: ${message}`, code);
}

/** Refuse any own key outside `allowed`, as the engine refuses an unknown member. */
function knownKeys(value: object, allowed: ReadonlySet<string>, path: string, what: string): void {
  for (const key of Object.keys(value)) {
    if (!allowed.has(key)) {
      fail(
        `${path}.${key}`,
        `${what} has no '${key}'; its keys are ${nameList([...allowed])}`,
        CODE.unknownField,
      );
    }
  }
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
    fail(path, "a Session V1 stable ID must match [a-z][a-z0-9._-]{0,126}", CODE.invalidId);
  }
  return value;
}

function integer(value: unknown, path: string, minimum: number, maximum: number): number {
  if (typeof value !== "number" || !Number.isSafeInteger(value) || value < minimum || value > maximum) {
    fail(path, `expected an integer in ${minimum}..=${maximum}`);
  }
  return value;
}

function bool(value: unknown, path: string, code?: string): boolean {
  if (typeof value !== "boolean") fail(path, "expected a boolean", code);
  return value;
}

/**
 * An effect's quality, wherever it sits: the engine's grammar knows `draft`, `normal` and `high`
 * (anything else is `schema.invalid_enum`), and every launch native descriptor publishes only
 * `normal` (`effect.quality.unsupported` at prepare-effects).
 */
function quality(value: unknown, path: string): "normal" {
  if (value === undefined || value === "normal") return "normal";
  if (value === "draft" || value === "high") {
    fail(path, "launch native descriptors publish only the 'normal' quality row", CODE.qualityUnsupported);
  }
  fail(path, "expected draft, normal or high, and launch effects publish only 'normal'", CODE.invalidEnum);
}

/**
 * An effect's link mode, wherever it sits: one of the three tokens (`schema.invalid_enum`), and
 * one the effect supports (`EFFECT_LINK_MODES`; `effect.link_mode.unsupported` at prepare-effects).
 */
function linkMode(effectId: EffectId, value: unknown, path: string): LinkMode {
  const mode = value ?? "dual_mono";
  if (typeof mode !== "string" || !LINK_MODES.includes(mode)) {
    fail(path, "expected dual_mono, maximum or average", CODE.invalidEnum);
  }
  const supported = EFFECT_LINK_MODES[effectId];
  if (!supported.includes(mode as LinkMode)) {
    fail(
      path,
      `${effectId} supports only the link modes ${nameList(supported)}, not '${mode}'`,
      CODE.linkModeUnsupported,
    );
  }
  return mode as LinkMode;
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
export function parameterRows(
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
  if (options.bypass !== undefined) bool(options.bypass, `${path}.bypass`, CODE.wrongType);
  quality(options.quality, `${path}.quality`);
  const mode = linkMode(effectId, options.linkMode, `${path}.linkMode`);
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
      linkMode: mode,
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
  const kind: string = value.kind;
  if (kind === "submix_output") {
    fail(
      `${path}.kind`,
      "'submix_output' is retired; read a submix as { kind: \"submix\", submixId, tap }, where "
        + "tap \"post_pan\" is the old submix output",
      CODE.invalidEnum,
    );
  }
  if (value.kind !== "track" && value.kind !== "submix") {
    fail(`${path}.kind`, "expected 'track' or 'submix'", CODE.invalidEnum);
  }
  if (value.kind === "track") stableId(value.trackId, `${path}.trackId`);
  else stableId(value.submixId, `${path}.submixId`);
  if (!SEND_TAPS.has(value.tap)) {
    fail(
      `${path}.tap`,
      `unknown ${value.kind} tap '${String(value.tap)}'; the taps are ${nameList([...SEND_TAPS])}`,
      CODE.invalidEnum,
    );
  }
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
    : freeze({ kind: "submix", submix_id: value.submixId, tap: value.tap });
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

/** A declared submix: its strip, or `undefined` for the transparent strip `.submix(id)` declares. */
interface SubmixEntry {
  readonly id: string;
  readonly spec: SubmixSpec | undefined;
}

/** One validated console slot, in the session's slot order. */
interface ConsoleSlotEntry {
  readonly slot: string;
  readonly section: "pre_insert" | "post_insert";
  readonly effectId: ConsoleEffectId;
  readonly quality: "normal";
  readonly linkMode: "dual_mono" | "maximum" | "average";
}

interface BuilderState {
  readonly options: NormalizedOptions;
  readonly sources: readonly SourceEntry[];
  /** `undefined` until `.console()`; an undeclared console is two empty sections. */
  readonly console: readonly ConsoleSlotEntry[] | undefined;
  readonly tracks: readonly TrackEntry[];
  readonly submixes: readonly SubmixEntry[];
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
      fail(`${path}.content`, "source content must match blake3:[0-9a-f]{64}");
    }
    return this.#next({
      sources: [...this.#state.sources, freeze({ id, spec: { ...spec } })],
    });
  }

  /**
   * Declare the session console, once, before the first track.
   *
   * `preInsert` slots run between the input section and the track's inserts, and `postInsert`
   * slots between the inserts and the fader. Either may be empty, and an undeclared console is two
   * empty sections. Slot IDs are unique across both sections, because a console address names
   * the slot and not its section.
   *
   * It must precede every track, and every submix declared with a strip, because their `console`
   * entries are checked against it when they are declared -- the reference-order rule every other
   * verb follows. A spec-less `.submix(id)` may precede it: its bypassed entries are written for
   * whatever slots the session ends up declaring.
   */
  console(spec: ConsoleSpec): SessionBuilder {
    const path = "console()";
    if (this.#state.console !== undefined) {
      fail(path, "the session console is declared once");
    }
    if (this.#state.tracks.length > 0 || this.#state.submixes.some((entry) => entry.spec !== undefined)) {
      fail(
        path,
        "declare the console before the first track or submix strip: each strip's console entries "
          + "are checked against its slots",
      );
    }
    if (spec === null || typeof spec !== "object" || Array.isArray(spec)) {
      fail(path, "expected { preInsert, postInsert }");
    }
    knownKeys(spec, CONSOLE_KEYS, path, "the session console");
    const slots: ConsoleSlotEntry[] = [];
    for (const [section, key] of [["pre_insert", "preInsert"], ["post_insert", "postInsert"]] as const) {
      const declared = spec[key] ?? [];
      if (!Array.isArray(declared)) fail(`${path}.${key}`, "expected an array of console slots");
      declared.forEach((raw: ConsoleSlotSpec, index: number) => {
        slots.push(consoleSlot(raw, section, `${path}.${key}[${index}]`, slots));
      });
    }
    return this.#next({ console: freeze(slots) });
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

  /**
   * Declare a submix strip. Its ID shares the graph-entity namespace with tracks and outputs.
   *
   * With a `spec`, the strip follows a track's rules and defaults field by field, and its console
   * entries are checked against the declared console now, as a track's are. With no spec it is the
   * transparent strip (identity input section, no inserts, a 0 dB unmuted fader, the identity
   * matrix, every console slot bypassed), so a bare bus passes its sum through unchanged apart from
   * the latency of the session's console slots, which every strip pays.
   */
  submix(id: string, spec?: SubmixSpec): SessionBuilder {
    stableId(id, "submix().id");
    const path = `submix("${id}")`;
    if (this.#graphIds().has(id)) {
      fail(`${path}.id`, "tracks, submixes and outputs share one ID namespace");
    }
    if (spec !== undefined) {
      if (spec === null || typeof spec !== "object" || Array.isArray(spec)) {
        fail(path, "expected a submix strip specification");
      }
      knownKeys(spec, SUBMIX_KEYS, path, "a submix strip");
      this.#validateStrip(spec, path);
    }
    return this.#next({
      submixes: [...this.#state.submixes, freeze({ id, spec: spec === undefined ? undefined : { ...spec } })],
    });
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
      if (!this.#hasSubmix(spec.destination.submixId)) {
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
      ...this.#state.submixes.map((entry) => entry.id),
      ...this.#state.outputs,
    ]);
  }

  #hasSubmix(id: string): boolean {
    return this.#state.submixes.some((entry) => entry.id === id);
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
    if (!this.#hasSubmix(source.submixId)) {
      fail(`${path}.submixId`, `'${source.submixId}' is not a declared submix`);
    }
  }

  #validateTrack(spec: TrackSpec, path: string): void {
    if (spec === null || typeof spec !== "object") fail(path, "expected a track specification");
    knownKeys(spec, TRACK_KEYS, path, "a track");
    const reference = trackSourceRef(spec.source, `${path}.source`);
    const source = this.#source(reference.id);
    if (source === undefined) {
      fail(`${path}.source`, `'${reference.id}' is not a declared source`);
    }
    resolveLanes(reference, source.spec.channels, `${path}.source`);
    this.#validateStrip(spec, path);
  }

  /** A strip's fields after its source: shared by tracks and submixes. */
  #validateStrip(spec: SubmixSpec, path: string): void {
    normalizeBuiltins(spec.builtins, this.#state.options.sampleRateHz, `${path}.builtins`);
    normalizeConsoleEntries(spec.console, this.#state.console ?? [], `${path}.console`);
    normalizeInserts(spec.inserts, `${path}.inserts`);
    normalizeFader(spec.fader, `${path}.fader`);
    normalizeMatrixOrPan(spec.pan, `${path}.pan`);
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
    resolveAutomationTarget(target, track, this.#state.console ?? [], `${path}.target`);
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
    console: undefined,
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

/**
 * One effect instance's `params` rows: canonical `(parameter_id, channel)` order, each pair once.
 *
 * Shared by inserts and console entries, whose parameters are the same rows with the same rules.
 */
function normalizeParams(
  descriptor: EffectDescriptor,
  parameters: Readonly<Record<string, unknown>>,
  channel: Channel,
  where: string,
): readonly ModelRecord[] {
  const params = Object.entries(parameters)
    .flatMap(([name, value]) =>
      parameterRows(descriptor, name, value, channel, `${where}.parameters.${name}`))
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
  return params;
}

/** The ID an insert carries when its declaration names none: its 1-based position. */
function insertId(decl: EffectDecl, index: number): string {
  return decl.slotId ?? `insert-${index + 1}`;
}

/** A track's inserts, in declared order. Only the *entities* sort; inserts preserve signal order. */
function normalizeInserts(
  effects: readonly EffectDecl[] | undefined,
  path: string,
): readonly ModelRecord[] {
  if (effects === undefined) return [];
  if (!Array.isArray(effects)) fail(path, "expected an array of effect() declarations");
  const ids = new Set<string>();
  return effects.map((decl, index) => {
    const where = `${path}[${index}]`;
    if (decl === null || typeof decl !== "object") {
      fail(where, "expected an effect declaration from effect()");
    }
    const id = stableId(insertId(decl, index), `${where}.slotId`);
    if (ids.has(id)) fail(`${where}.slotId`, `'${id}' is repeated in the inserts`, CODE.duplicateId);
    ids.add(id);
    const descriptor = effectDescriptor(decl.effectId, `${where}.effectId`);
    const params = normalizeParams(descriptor, decl.parameters, decl.options.channel, where);
    const sidechain = decl.options.sidechain;
    return freeze({
      id,
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

/**
 * Validate one console slot declaration against the slots declared before it.
 *
 * The refusals are the engine's: a key a slot does not have (`sidechain`, `bypass`, `params`, ...)
 * is `schema.unknown_field`, a slot ID used twice in either section is `id.duplicate`, and an
 * effect off the eligibility list -- the delay, the multiband, anything unknown -- is
 * `console.slot.ineligible_effect`.
 */
function consoleSlot(
  raw: ConsoleSlotSpec,
  section: ConsoleSlotEntry["section"],
  path: string,
  declared: readonly ConsoleSlotEntry[],
): ConsoleSlotEntry {
  if (raw === null || typeof raw !== "object" || Array.isArray(raw)) {
    fail(path, "expected a console slot { slot, effectId, quality?, linkMode? }");
  }
  knownKeys(raw, CONSOLE_SLOT_KEYS, path, "a console slot");
  const slot = stableId(raw.slot, `${path}.slot`);
  if (declared.some((entry) => entry.slot === slot)) {
    fail(
      `${path}.slot`,
      `console slot '${slot}' is already declared; slot IDs are unique across preInsert and postInsert`,
      CODE.duplicateId,
    );
  }
  const effectId: unknown = raw.effectId;
  if (typeof effectId !== "string" || !CONSOLE_ELIGIBLE_EFFECTS.includes(effectId as ConsoleEffectId)) {
    fail(
      `${path}.effectId`,
      `'${String(effectId)}' cannot be a console slot; a slot is one of `
        + `${nameList(CONSOLE_ELIGIBLE_EFFECTS)}, and any other effect is an insert`,
      CODE.ineligibleEffect,
    );
  }
  quality(raw.quality, `${path}.quality`);
  const mode = linkMode(effectId as ConsoleEffectId, raw.linkMode, `${path}.linkMode`);
  return freeze({
    slot,
    section,
    effectId: effectId as ConsoleEffectId,
    quality: "normal" as const,
    linkMode: mode,
  });
}

/**
 * One track's console entries against the session's slots, in the engine's own order.
 *
 * This walks the entries exactly as the engine's `validate_console_entries` does, so the first
 * refusal is the engine's first diagnostic for the same track: an entry naming no declared slot is
 * `reference.missing_entity`, a repeated slot `id.duplicate`, an entry out of slot order
 * `console.entry_order`, and a slot with no entry `console.entry_missing`. An entry carries only
 * the track's knobs, so any other key (`effectId`, `quality`, `linkMode`, `sidechain`, ...) is
 * `schema.unknown_field`.
 */
function normalizeConsoleEntries(
  entries: readonly ConsoleEntrySpec[] | undefined,
  slots: readonly ConsoleSlotEntry[],
  path: string,
): readonly ModelRecord[] {
  const supplied = entries ?? [];
  if (!Array.isArray(supplied)) fail(path, "expected an array of console entries in slot order");
  const bySlot = new Map(slots.map((slot) => [slot.slot, slot] as const));
  const seen = new Set<string>();
  const normalized = supplied.map((entry: ConsoleEntrySpec, position: number) => {
    const where = `${path}[${position}]`;
    if (entry === null || typeof entry !== "object" || Array.isArray(entry)) {
      fail(where, "expected a console entry { slot, bypass?, parameters?, channel? }");
    }
    knownKeys(entry, CONSOLE_ENTRY_KEYS, where, "a console entry");
    const slotId = stableId(entry.slot, `${where}.slot`);
    const slot = bySlot.get(slotId);
    if (slot === undefined) {
      fail(
        `${where}.slot`,
        slots.length === 0
          ? `'${slotId}' is not a console slot; this session declares none`
          : `'${slotId}' is not a console slot; the slots are ${nameList(slots.map((row) => row.slot))}`,
        CODE.missingEntity,
      );
    }
    if (seen.has(slotId)) {
      fail(`${where}.slot`, `'${slotId}' is repeated; a track carries each slot exactly once`, CODE.duplicateId);
    }
    seen.add(slotId);
    if (slots[position]?.slot !== slotId) {
      fail(
        `${where}.slot`,
        `'${slotId}' is out of slot order; entries follow ${nameList(slots.map((row) => row.slot))}`,
        CODE.entryOrder,
      );
    }
    const bypass = entry.bypass === undefined ? false : bool(entry.bypass, `${where}.bypass`, CODE.wrongType);
    const channel = entry.channel ?? "both";
    if (!["left", "right", "both"].includes(channel)) {
      fail(`${where}.channel`, "expected left, right or both", CODE.invalidEnum);
    }
    const parameters = entry.parameters ?? {};
    if (parameters === null || typeof parameters !== "object" || Array.isArray(parameters)) {
      fail(`${where}.parameters`, "expected parameter values by catalog name");
    }
    const descriptor = effectDescriptor(slot.effectId, `${where}.slot`);
    return freeze({
      slot: slotId,
      bypass,
      params: normalizeParams(descriptor, parameters as Record<string, unknown>, channel, where),
    });
  });
  const missing = slots.find((slot) => !seen.has(slot.slot));
  if (missing !== undefined) {
    fail(
      path,
      `no entry for console slot '${missing.slot}'; every track carries every slot, in the order `
        + nameList(slots.map((row) => row.slot)),
      CODE.entryMissing,
    );
  }
  return normalized;
}

/** The session's root `console`: the two sections' slot declarations, in declared order. */
function normalizeConsole(slots: readonly ConsoleSlotEntry[]): ModelRecord {
  const section = (name: ConsoleSlotEntry["section"]): readonly ModelRecord[] => slots
    .filter((slot) => slot.section === name)
    .map((slot) => freeze({
      slot: slot.slot,
      identity: { kind: "native", effect_id: slot.effectId },
      quality: slot.quality,
      link_mode: slot.linkMode,
    }));
  return freeze({ pre_insert: section("pre_insert"), post_insert: section("post_insert") });
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
 * The racks answer differently on purpose. An effect target is resolved through the *declared
 * instance*: a `console` target names a session slot and reads this track's entry for it, an
 * `inserts` target names one of this track's inserts, and in both the `(parameter_id, channel)`
 * pair must already appear in that instance's params, which is exactly what the engine checks. A
 * `builtins` target has no instance, so it is resolved against the builtin parameter ABI and
 * restricted to the rows that declare `blockTarget`. The prepared input-filter rows are live
 * through the paired command path; `delay_samples` remains prepared-only and is refused here.
 */
function resolveAutomationTarget(
  target: AutomationTarget,
  track: TrackEntry,
  slots: readonly ConsoleSlotEntry[],
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
    return {
      parameterId: row.id,
      unit: row.unitName,
      effectId: BUILTIN_STRIP_EFFECT_ID,
      row: undefined,
    };
  }
  if (target.rack !== "console" && target.rack !== "inserts") {
    fail(`${path}.rack`, "expected console, inserts or builtins", CODE.invalidEnum);
  }
  const slotId = stableId(target.slotId, `${path}.slotId`);
  let effectId: string;
  let declaredValue: unknown;
  let channel: Channel;
  if (target.rack === "console") {
    const slot = slots.find((candidate) => candidate.slot === slotId);
    const entry = (track.spec.console ?? []).find((candidate) => candidate.slot === slotId);
    if (slot === undefined || entry === undefined) {
      fail(`${path}.slotId`, `'${slotId}' is not a console slot of this session`, CODE.missingEntity);
    }
    effectId = slot.effectId;
    declaredValue = (entry.parameters as Readonly<Record<string, unknown>> | undefined)
      ?.[target.parameter];
    channel = entry.channel ?? "both";
  } else {
    const declared = track.spec.inserts ?? [];
    const decl = declared.find((candidate, position) => insertId(candidate, position) === slotId);
    if (decl === undefined) {
      fail(`${path}.slotId`, `'${slotId}' is not one of ${track.id}'s inserts`, CODE.missingEntity);
    }
    effectId = decl.effectId;
    declaredValue = decl.parameters[target.parameter as keyof typeof decl.parameters];
    channel = decl.options.channel;
  }
  const descriptor = effectDescriptor(effectId, `${path}.slotId`);
  const row = effectParameter(descriptor, target.parameter, `${path}.parameter`);
  if (row.channelPolicyName === "shared" && target.channel !== "both") {
    fail(`${path}.channel`, `${row.name} is a shared parameter and is addressed as 'both'`);
  }
  if (declaredValue === undefined
    || !parameterRows(descriptor, target.parameter, declaredValue, channel, `${path}.parameter`)
      .some((entry) => entry.channel === target.channel)) {
    fail(
      `${path}.parameter`,
      `${row.name} is not declared on '${slotId}' for channel '${target.channel}'`,
      CODE.missingEntity,
    );
  }
  return { parameterId: row.id, unit: row.unitName, effectId: slotId, row };
}

function normalizeAutomation(
  spec: AutomationSpec,
  tracks: readonly TrackEntry[],
  slots: readonly ConsoleSlotEntry[],
): ModelRecord {
  const path = `automation("${spec.id}")`;
  const track = tracks.find((entry) => entry.id === spec.target.trackId);
  if (track === undefined) {
    fail(`${path}.target.trackId`, `'${spec.target.trackId}' is not a declared track`);
  }
  const resolved = resolveAutomationTarget(spec.target, track, slots, `${path}.target`);
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
  slots: readonly ConsoleSlotEntry[],
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
    console: normalizeConsoleEntries(spec.console, slots, `${path}.console`),
    inserts: { effects: normalizeInserts(spec.inserts, `${path}.inserts`) },
    fader: normalizeFader(spec.fader, `${path}.fader`),
    [matrixOrPan.key]: matrixOrPan.value,
  });
}

/**
 * A submix strip in the track's key order without the source fields. A spec-less submix is the
 * transparent strip, the engine's `Submix::unity`: every field at its identity, the identity
 * matrix, and one bypassed entry per declared console slot.
 */
function normalizeSubmix(
  entry: SubmixEntry,
  slots: readonly ConsoleSlotEntry[],
  sampleRateHz: number,
): ModelRecord {
  const path = `submix("${entry.id}")`;
  const spec: SubmixSpec = entry.spec ?? {
    console: slots.map((slot) => ({ slot: slot.slot, bypass: true })),
    pan: { matrix: IDENTITY_MATRIX },
  };
  const matrixOrPan = normalizeMatrixOrPan(spec.pan, `${path}.pan`);
  return freeze({
    id: entry.id,
    builtins: normalizeBuiltins(spec.builtins, sampleRateHz, `${path}.builtins`),
    console: normalizeConsoleEntries(spec.console, slots, `${path}.console`),
    inserts: { effects: normalizeInserts(spec.inserts, `${path}.inserts`) },
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
  const slots = state.console ?? [];
  const tracks = state.tracks
    .map((entry) => normalizeTrack(entry, state.sources, slots, options.sampleRateHz))
    .sort(byId);
  const submixes = state.submixes
    .map((entry) => normalizeSubmix(entry, slots, options.sampleRateHz))
    .sort(byId);
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
    .map((spec) => normalizeAutomation(spec, state.tracks, slots))
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
    console: normalizeConsole(slots),
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
