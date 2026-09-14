import { ABI_LAYOUT } from "../generated/abi.ts";
import { CATALOG } from "../generated/catalog.ts";
import type {
  EffectId,
  EffectDescriptor,
  EffectParameter,
  EffectParameterName,
  TapName,
} from "../generated/catalog.ts";
import type { CommandReport, SessionMap } from "./boundary.ts";
import { MisoUsageError } from "./errors.ts";
import type { LaneEdit } from "./writer.ts";

export type ConsoleRack = "simd1" | "dynamic" | "simd2";
export type ConsoleChannel = "left" | "right" | "both";

export interface SmoothingOptions {
  readonly smoothingSamples?: number;
}

export interface LaneOptions extends SmoothingOptions {
  readonly channel?: ConsoleChannel;
}

/** The two cutoff values carried atomically by one prepared input-filter command. */
export interface InputFilterValues {
  readonly hpfHz: number;
  readonly lpfHz: number;
}

/** Input-filter edits select a lane but use the builtin fixed 64-update policy. */
export interface InputFilterOptions {
  readonly channel?: ConsoleChannel;
}

type LiveParameter<E extends EffectId> = Extract<
  EffectParameter<E>,
  { readonly liveUpdatable: true }
>;

export type LiveEffectParameterName<E extends EffectId> = LiveParameter<E>["name"];

type ParameterRow<
  E extends EffectId,
  N extends LiveEffectParameterName<E>,
> = Extract<LiveParameter<E>, { readonly name: N }>;

type ParameterOfDescriptor<D> = D extends {
  readonly parameters: readonly (infer Parameter)[];
} ? Parameter : never;
type CatalogParameterRow = ParameterOfDescriptor<EffectDescriptor>;
type RuntimeParameterRow = Omit<
  CatalogParameterRow,
  "domainName" | "enumChoices" | "minimum" | "maximum" | "channelPolicyName"
> & {
  readonly domainName: "boolean" | "enumeration" | "continuous";
  readonly enumChoices: readonly { readonly label: string; readonly value: number }[];
  readonly minimum: number | null;
  readonly maximum: number | null;
  readonly channelPolicyName: "shared" | "perLane";
};

type EnumerationLabel<P> = P extends {
  readonly enumChoices: readonly (infer Choice)[];
} ? Choice extends { readonly label: infer Label extends string } ? Label : never : never;

type ParameterValue<P> = P extends { readonly domainName: "boolean" }
  ? boolean
  : P extends { readonly domainName: "enumeration" }
    ? EnumerationLabel<P>
    : number;

type ParameterOptions<P> = P extends { readonly channelPolicyName: "shared" }
  ? SmoothingOptions & { readonly channel?: "both" }
  : LaneOptions;

export type LiveEffectParameterValue<
  E extends EffectId,
  N extends LiveEffectParameterName<E>,
> = ParameterValue<ParameterRow<E, N>>;

export type LiveEffectParameterOptions<
  E extends EffectId,
  N extends LiveEffectParameterName<E>,
> = ParameterOptions<ParameterRow<E, N>>;

/** One catalog-derived object edit for a live effect parameter. */
export type LiveEffectParameterEdit<E extends EffectId> = LiveParameter<E> extends infer P
  ? P extends { readonly name: infer N extends LiveEffectParameterName<E> }
    ? {
      readonly key: N;
      readonly value: ParameterValue<P>;
    } & ParameterOptions<P>
    : never
  : never;

export interface MatrixValues {
  readonly ll: number;
  readonly lr: number;
  readonly rl: number;
  readonly rr: number;
}

export type ConsoleSubmit = (
  edits: readonly LaneEdit[],
) => CommandReport | Promise<CommandReport>;

/** Optional owner hook used to serialize observation edits with managed subscriptions. */
export type ConsoleBeforeSubmit = (
  edits: readonly LaneEdit[],
  managed?: boolean,
) => void | Promise<void>;

const RACKS = Object.freeze({ simd1: 0, dynamic: 1, simd2: 2 } as const);
const CHANNELS = Object.freeze({ left: 0, right: 1, both: 2 } as const);
const NONE = 255;

function u32(value: number, name: string): number {
  if (!Number.isSafeInteger(value) || value < 0 || value > 0xffff_ffff) {
    throw new MisoUsageError(`${name} must be a u32`);
  }
  return value;
}

function smoothing(options: SmoothingOptions): number {
  return u32(options.smoothingSamples ?? 0, "smoothingSamples");
}

type RuntimeParameterOptions = {
  readonly channel?: unknown;
  readonly smoothingSamples?: unknown;
};

function runtimeSmoothing(options: RuntimeParameterOptions): number {
  const sampleCount = options.smoothingSamples ?? 0;
  if (typeof sampleCount !== "number") {
    throw new MisoUsageError("smoothingSamples must be a number");
  }
  return u32(sampleCount, "smoothingSamples");
}

function runtimeChannel(channel: unknown): ConsoleChannel {
  if (channel === undefined || channel === "both") return "both";
  if (channel === "left" || channel === "right") return channel;
  throw new MisoUsageError("channel must be left, right, or both");
}

const LIVE_PARAMETER_EDIT_FIELDS = new Set(["key", "value", "channel", "smoothingSamples"]);

interface NormalizedParameterEdit {
  readonly key: string;
  readonly value: unknown;
  readonly options: RuntimeParameterOptions;
}

function normalizeParameterEdit(
  keyOrEdit: unknown,
  value: unknown,
  options: RuntimeParameterOptions | undefined,
): NormalizedParameterEdit {
  if (typeof keyOrEdit === "string") {
    return { key: keyOrEdit, value, options: options ?? {} };
  }
  if (keyOrEdit === null || typeof keyOrEdit !== "object" || Array.isArray(keyOrEdit)) {
    throw new MisoUsageError("a live parameter edit must be an object");
  }
  for (const field of Reflect.ownKeys(keyOrEdit)) {
    if (typeof field !== "string" || !LIVE_PARAMETER_EDIT_FIELDS.has(field)) {
      throw new MisoUsageError(`live parameter edit has unknown field '${String(field)}'`);
    }
  }
  const edit = keyOrEdit as Record<string, unknown>;
  if (!Object.prototype.hasOwnProperty.call(edit, "key")) {
    throw new MisoUsageError("live parameter edit requires a key");
  }
  if (!Object.prototype.hasOwnProperty.call(edit, "value")) {
    throw new MisoUsageError("live parameter edit requires a value");
  }
  if (typeof edit.key !== "string") {
    throw new MisoUsageError("live parameter edit key must be a string");
  }
  return {
    key: edit.key,
    value: edit.value,
    options: {
      channel: edit.channel,
      smoothingSamples: edit.smoothingSamples,
    },
  };
}

function finite(value: number, name: string, minimum?: number, maximum?: number): number {
  if (!Number.isFinite(value)) throw new MisoUsageError(`${name} must be finite`);
  if (minimum !== undefined && value < minimum) {
    throw new MisoUsageError(`${name} must be at least ${minimum}`);
  }
  if (maximum !== undefined && value > maximum) {
    throw new MisoUsageError(`${name} must be at most ${maximum}`);
  }
  return value;
}

function builtinNumber(name: string, value: number): number {
  const row = CATALOG.builtins.parameters.find((candidate) => candidate.name === name);
  if (row === undefined || !row.liveUpdatable) {
    throw new MisoUsageError(`the generated catalog has no live builtin ${name}`);
  }
  return finite(
    value,
    name,
    row.minimum ?? undefined,
    typeof row.maximum === "number" ? row.maximum : undefined,
  );
}

function values(a = 0, b = 0, c = 0, d = 0): readonly [number, number, number, number] {
  return Object.freeze([a, b, c, d]);
}

function lane(options: LaneOptions): number {
  return CHANNELS[options.channel ?? "both"];
}

function trackEdit(
  kind: LaneEdit["kind"],
  trackIndex: number,
  options: {
    readonly rack?: number;
    readonly channel?: number;
    readonly effectIndex?: number;
    readonly parameterId?: number;
    readonly smoothingSamples?: number;
    readonly values?: readonly [number, number, number, number];
  } = {},
): LaneEdit {
  return Object.freeze({
    kind,
    trackIndex,
    rack: options.rack ?? NONE,
    channel: options.channel ?? NONE,
    effectIndex: options.effectIndex ?? 0,
    parameterId: options.parameterId ?? 0,
    smoothingSamples: options.smoothingSamples ?? 0,
    values: options.values ?? values(),
  });
}

/** A semantic edit builder bound to the engine's canonical session map. */
export class ConsoleEdits {
  readonly #tracks: ReadonlyMap<string, number>;

  constructor(map: SessionMap) {
    this.#tracks = new Map(map.tracks.map((id, index) => [id, index] as const));
  }

  track(trackId: string): TrackEdits {
    const index = this.#tracks.get(trackId);
    if (index === undefined) {
      throw new MisoUsageError(
        `the compiled session has no track '${trackId}'; expected one of ${[...this.#tracks.keys()].join(", ")}`,
      );
    }
    return new TrackEdits(index);
  }
}

/** Every strip-level live edit. Methods build data and never mutate the engine. */
export class TrackEdits {
  readonly #trackIndex: number;

  constructor(trackIndex: number) {
    this.#trackIndex = trackIndex;
  }

  pan(left: number, right: number, options: SmoothingOptions = {}): LaneEdit {
    return trackEdit("pan", this.#trackIndex, {
      smoothingSamples: smoothing(options),
      values: values(
        builtinNumber("matrix_ll", left),
        builtinNumber("matrix_rr", right),
      ),
    });
  }

  matrix(matrix: MatrixValues, options: SmoothingOptions = {}): LaneEdit {
    return trackEdit("matrix", this.#trackIndex, {
      smoothingSamples: smoothing(options),
      values: values(
        builtinNumber("matrix_ll", matrix.ll),
        builtinNumber("matrix_lr", matrix.lr),
        builtinNumber("matrix_rl", matrix.rl),
        builtinNumber("matrix_rr", matrix.rr),
      ),
    });
  }

  faderDb(db: number, options: LaneOptions = {}): LaneEdit {
    return trackEdit("faderDb", this.#trackIndex, {
      channel: lane(options),
      smoothingSamples: smoothing(options),
      values: values(builtinNumber("fader_db", db)),
    });
  }

  mute(enabled: boolean, options: LaneOptions = {}): LaneEdit {
    return trackEdit("mute", this.#trackIndex, {
      channel: lane(options),
      smoothingSamples: smoothing(options),
      values: values(enabled ? 1 : 0),
    });
  }

  solo(enabled: boolean, options: SmoothingOptions = {}): LaneEdit {
    return trackEdit("solo", this.#trackIndex, {
      smoothingSamples: smoothing(options),
      values: values(enabled ? 1 : 0),
    });
  }

  trimDb(db: number, options: LaneOptions = {}): LaneEdit {
    return trackEdit("trimDb", this.#trackIndex, {
      channel: lane(options),
      smoothingSamples: smoothing(options),
      values: values(builtinNumber("trim_db", db)),
    });
  }

  polarityInvert(enabled: boolean, options: LaneOptions = {}): LaneEdit {
    return trackEdit("polarityInvert", this.#trackIndex, {
      channel: lane(options),
      smoothingSamples: smoothing(options),
      values: values(enabled ? 1 : 0),
    });
  }

  /** Set one lane's HPF cutoff through the prepared input-filter owner. */
  hpfHz(value: number, options: InputFilterOptions = {}): LaneEdit {
    return trackEdit("inputFilters", this.#trackIndex, {
      channel: lane(options),
      parameterId: 3,
      values: values(builtinNumber("hpf_hz", value)),
    });
  }

  /** Set one lane's LPF cutoff through the prepared input-filter owner. */
  lpfHz(value: number, options: InputFilterOptions = {}): LaneEdit {
    return trackEdit("inputFilters", this.#trackIndex, {
      channel: lane(options),
      parameterId: 4,
      values: values(builtinNumber("lpf_hz", value)),
    });
  }

  /** Set both cutoffs as one indivisible wire edit. */
  inputFilters(filters: InputFilterValues, options: InputFilterOptions = {}): LaneEdit {
    if (filters === null || typeof filters !== "object" || Array.isArray(filters)) {
      throw new MisoUsageError("inputFilters requires an object with hpfHz and lpfHz");
    }
    return trackEdit("inputFilters", this.#trackIndex, {
      channel: lane(options),
      values: values(
        builtinNumber("hpf_hz", filters.hpfHz),
        builtinNumber("lpf_hz", filters.lpfHz),
      ),
    });
  }

  effect<E extends EffectId>(
    rack: ConsoleRack,
    effectIndex: number,
    effectId: E,
  ): EffectEdits<E> {
    return new EffectEdits(
      this.#trackIndex,
      RACKS[rack],
      u32(effectIndex, "effectIndex"),
      effectId,
    );
  }
}

/** Catalog-derived edits for one effect instance. */
export class EffectEdits<E extends EffectId> {
  readonly #trackIndex: number;
  readonly #rack: number;
  readonly #effectIndex: number;
  readonly #effectId: E;

  constructor(trackIndex: number, rack: number, effectIndex: number, effectId: E) {
    this.#trackIndex = trackIndex;
    this.#rack = rack;
    this.#effectIndex = effectIndex;
    this.#effectId = effectId;
  }

  parameter<N extends LiveEffectParameterName<E>>(
    name: N,
    value: LiveEffectParameterValue<E, N>,
    options?: LiveEffectParameterOptions<E, N>,
  ): LaneEdit;
  parameter(edit: LiveEffectParameterEdit<E>): LaneEdit;
  parameter<N extends LiveEffectParameterName<E>>(
    keyOrEdit: string | LiveEffectParameterEdit<E>,
    value?: unknown,
    options: RuntimeParameterOptions = {},
  ): LaneEdit {
    const normalized = normalizeParameterEdit(keyOrEdit, value, options);
    const name = normalized.key;
    value = normalized.value;
    options = normalized.options;
    const descriptor = CATALOG.effects.find((row) => row.id === this.#effectId);
    const row = descriptor?.parameters.find(
      (candidate) => candidate.name === name,
    ) as RuntimeParameterRow | undefined;
    if (row === undefined || !row.liveUpdatable) {
      throw new MisoUsageError(`${this.#effectId}.${String(name)} is not live-updatable`);
    }
    let scalar: number;
    if (row.domainName === "boolean") {
      if (typeof value !== "boolean") {
        throw new MisoUsageError(`${this.#effectId}.${row.name} must be boolean`);
      }
      scalar = value ? 1 : 0;
    } else if (row.domainName === "enumeration") {
      const choice = row.enumChoices.find(
        (candidate) => candidate.label === (value as unknown as string),
      );
      if (choice === undefined) {
        throw new MisoUsageError(
          `${this.#effectId}.${row.name} must be one of ${row.enumChoices.map((item) => item.label).join(", ")}`,
        );
      }
      scalar = choice.value;
    } else {
      if (typeof value !== "number") {
        throw new MisoUsageError(`${this.#effectId}.${row.name} must be numeric`);
      }
      scalar = finite(
        value,
        `${this.#effectId}.${row.name}`,
        row.minimum ?? undefined,
        row.maximum ?? undefined,
      );
    }
    const requestedChannel = runtimeChannel(options.channel);
    if (row.channelPolicyName === "shared" && requestedChannel !== "both") {
      throw new MisoUsageError(`${this.#effectId}.${row.name} is shared and must address both lanes`);
    }
    return trackEdit("effectParam", this.#trackIndex, {
      rack: this.#rack,
      channel: CHANNELS[requestedChannel],
      effectIndex: this.#effectIndex,
      parameterId: row.id,
      smoothingSamples: runtimeSmoothing(options),
      values: values(scalar),
    });
  }

  bypass(enabled: boolean): LaneEdit {
    return trackEdit("effectBypass", this.#trackIndex, {
      rack: this.#rack,
      effectIndex: this.#effectIndex,
      values: values(enabled ? 1 : 0),
    });
  }

  observe(tap: TapName<E>, armed: boolean, windowBlocks = 0): LaneEdit {
    const descriptor = CATALOG.effects.find((row) => row.id === this.#effectId);
    const observation = descriptor?.observations.find((candidate) => candidate.name === tap);
    if (observation === undefined || !observation.subscribable) {
      throw new MisoUsageError(`${this.#effectId} has no subscribable tap '${String(tap)}'`);
    }
    return trackEdit(armed ? "observeSubscribe" : "observeUnsubscribe", this.#trackIndex, {
      rack: this.#rack,
      effectIndex: this.#effectIndex,
      parameterId: observation.id,
      smoothingSamples: u32(windowBlocks, "windowBlocks"),
    });
  }
}

/** One transaction-oriented console over either SDK transport. */
export class EngineConsole {
  readonly edit: ConsoleEdits;
  readonly #submit: ConsoleSubmit;
  readonly #beforeSubmit: ConsoleBeforeSubmit | undefined;

  constructor(map: SessionMap, submit: ConsoleSubmit, beforeSubmit?: ConsoleBeforeSubmit) {
    this.edit = new ConsoleEdits(map);
    this.#submit = submit;
    this.#beforeSubmit = beforeSubmit;
  }

  async submit(...edits: readonly LaneEdit[]): Promise<CommandReport> {
    return this.#submitChecked(edits, false);
  }

  /** @internal Managed observation owner submission with an owner-specific guard bypass. */
  async submitManaged(...edits: readonly LaneEdit[]): Promise<CommandReport> {
    return this.#submitChecked(edits, true);
  }

  async #submitChecked(edits: readonly LaneEdit[], managed: boolean): Promise<CommandReport> {
    if (edits.length === 0 || edits.length > ABI_LAYOUT.constants.maximumCommandRecords) {
      throw new MisoUsageError(
        `a console transaction must contain 1..${ABI_LAYOUT.constants.maximumCommandRecords} edits`,
      );
    }
    const beforeSubmit = this.#beforeSubmit?.(edits, managed);
    if (beforeSubmit !== undefined) await beforeSubmit;
    const report = await this.#submit(edits);
    const resultConsistent = report.ok === (report.result === 0);
    const wholeBatch = report.ok
      ? report.admitted === edits.length
      : report.admitted === 0;
    if (!resultConsistent || !wholeBatch) {
      throw new MisoUsageError(
        `transport violated whole-batch admission: result=${report.result}, ok=${report.ok}, admitted=${report.admitted}, requested=${edits.length}`,
      );
    }
    return report;
  }
}

/** Type-only proof that the live-name filter is derived from the catalog's own rows. */
export type _AllEffectParameterNames<E extends EffectId> = EffectParameterName<E>;
