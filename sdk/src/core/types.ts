import type {
  CommandReasonName,
  EffectDescriptor,
  EffectId,
  EffectParameter,
  EffectParameterName,
} from "../generated/catalog.js";

export type { CommandReasonName, EffectId } from "../generated/catalog.js";

export type Rack = "simd1" | "dynamic" | "simd2";
export type Channel = "left" | "right" | "both";
export type SendTap =
  | "input"
  | "post_input_builtins"
  | "post_simd1"
  | "post_dynamic"
  | "post_simd2_pre_fader"
  | "post_fader"
  | "post_matrix";
export type Matrix2x2 = Readonly<{ ll: number; lr: number; rl: number; rr: number }>;
export type PerLane<T> = T | Readonly<{ left: T; right: T }> | readonly [left: T, right: T];

type ParameterScalar<P> = P extends { readonly domainName: "boolean" }
  ? boolean
  : P extends { readonly domainName: "enumeration"; readonly enumChoices: readonly (infer C)[] }
    ? C extends { readonly label: infer Label extends string } ? Label : never
    : number;

type ParameterInput<P> = P extends { readonly channelPolicyName: "perLane" }
  ? PerLane<ParameterScalar<P>>
  : ParameterScalar<P>;

/** Values use display units. Per-lane descriptors additionally accept explicit left/right values. */
export type EffectParamValues<E extends EffectId> = Partial<{
  [Name in EffectParameterName<E>]: ParameterInput<Extract<EffectParameter<E>, { readonly name: Name }>>;
}>;

export interface EffectOptions {
  /** Session V1 rack-local identity. It is deliberately separate from native `effectId`. */
  readonly slotId?: string;
  readonly bypass?: boolean;
  /** Launch native descriptors currently publish only the normal quality row. */
  readonly quality?: "normal";
  readonly linkMode?: "dual_mono" | "maximum" | "average";
  readonly channel?: Channel;
  readonly sidechain?: Readonly<{ source: RouteSource; portId: string }>;
}

export interface EffectDecl<E extends EffectId = EffectId> {
  readonly effectId: E;
  /**
   * An omitted factory slot is materialized deterministically by `session().track()`
   * from its rack and declaration order.  The emitted Session V1 value always has
   * a non-empty rack-local ID.
   */
  readonly slotId?: string;
  readonly parameters: EffectParamValues<E>;
  readonly options: Required<Pick<EffectOptions, "bypass" | "quality" | "linkMode" | "channel">>
    & Pick<EffectOptions, "sidechain">;
}

export interface SourceSpec {
  readonly channels: 1 | 2;
  readonly frames: number;
  readonly sampleRateHz?: 44_100 | 48_000 | 88_200 | 96_000;
  readonly locator?: string;
  readonly identity?: string;
}

export interface BuiltinsSpec {
  readonly polarityInvert?: boolean;
  readonly trimDb?: number;
  readonly hpfHz?: number;
  readonly lpfHz?: number;
}

export interface TrackSpec<E extends readonly EffectDecl[] = readonly EffectDecl[]> {
  readonly source: string | Readonly<{ left: readonly [string, number]; right: readonly [string, number] }>;
  readonly builtins?: PerLane<BuiltinsSpec>;
  readonly fader?: Readonly<{ leftDb?: number; rightDb?: number; leftMute?: boolean; rightMute?: boolean }>;
  readonly pan?: Readonly<{ left: number; right: number; smoothingSamples?: number }>
    | Readonly<{ matrix: Matrix2x2; smoothingSamples?: number }>;
  readonly simd1?: E;
  readonly dynamic?: E;
  readonly simd2?: E;
}

export type RouteSource =
  | Readonly<{ kind: "track"; trackId: string; tap: SendTap }>
  | Readonly<{ kind: "submix_output"; submixId: string }>;
export type RouteDestination =
  | Readonly<{ kind: "submix_input"; submixId: string }>
  | Readonly<{ kind: "output_input"; outputId: string }>;
export interface RouteSpec {
  readonly id: string;
  readonly source: RouteSource;
  readonly destination: RouteDestination;
  readonly matrix?: Matrix2x2;
  readonly gainDb?: number;
}

export interface AutomationSegment {
  readonly shape: "step" | "linear" | "exponential";
  readonly startSample: bigint;
  readonly endSample: bigint;
  readonly startValue: number;
  readonly endValue: number;
}

export interface AutomationTarget {
  readonly trackId: string;
  readonly rack: Rack;
  /** Rack-local `EffectOptions.slotId`, never a brittle positional index. */
  readonly slotId: string;
  readonly parameter: string;
  readonly channel: Channel;
}

export interface CommandAck {
  readonly ok: boolean;
  readonly reason: CommandReasonName;
  readonly rejectedIndex: number;
  readonly admitted: number;
  readonly appliedAtSample: bigint;
  readonly explain: string;
}

export type CatalogEffect = EffectDescriptor;

/** Exact tuple positions; broad arrays intentionally have no statically known positions. */
export type Indices<T extends readonly unknown[]> = Exclude<Partial<T>["length"], T["length"]>;

/** A typed rack-local effect handle used by control-facing TrackConsole APIs. */
export interface EffectConsole<D extends EffectDecl = EffectDecl> {
  readonly declaration: D;
  readonly slotId: string;
}

/**
 * Tuple-indexed effect lookup.  `Indices` deliberately rejects the tuple length,
 * preventing an off-by-one index from becoming a runtime command target.
 */
export interface TrackConsole<E extends readonly EffectDecl[] = readonly EffectDecl[]> {
  effect<I extends Indices<E> & keyof E>(index: I): EffectConsole<Extract<E[I], EffectDecl>>;
}
