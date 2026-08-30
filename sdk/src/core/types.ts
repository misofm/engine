import type {
  EffectId,
  EffectParameter,
  EffectParameterName,
  SidechainPortName,
} from "../generated/catalog.ts";

/**
 * The author-facing vocabulary of the Session V1 builder.
 *
 * Nothing here restates the engine's parameter tables. Every effect and every parameter name a
 * caller can write is *derived* from `../generated/catalog.ts`, which is itself generated from the
 * engine's own `miso-engine-parameter-metadata` output. A hand-written union would be a sixth copy
 * of a table that has already drifted five times (issue #207's N-13(d)), so there is none.
 */

/** The three effect racks a track carries, in signal order. */
export type Rack = "simd1" | "dynamic" | "simd2";

/**
 * The four racks an automation target may name.
 *
 * `builtins` is not a rack of instances -- it is the strip's own fixed section, admitted as an
 * automation target by issue #178 under #210's D2. It is spelled here rather than in `Rack`
 * because a track has three *effect* racks and a caller must not be able to place an effect in a
 * fourth one that does not exist.
 */
export type AutomationRack = Rack | "builtins";

export type Channel = "left" | "right" | "both";

/** The seven track taps a route or a routed sidechain may read from. */
export type SendTap =
  | "input"
  | "post_input_builtins"
  | "post_simd1"
  | "post_dynamic"
  | "post_simd2_pre_fader"
  | "post_fader"
  | "post_matrix";

export type Matrix2x2 = Readonly<{ ll: number; lr: number; rl: number; rr: number }>;

/** One value for both lanes, or an explicit left/right pair in either spelling. */
export type PerLane<T> = T | Readonly<{ left: T; right: T }> | readonly [left: T, right: T];

/**
 * The closed source bit-depth token set.
 *
 * Integer `16`, integer `24`, and the *string* `"32f"` -- three tokens, two TOML types. That
 * asymmetry is the schema's (`docs/SESSION_SCHEMA_V1.md`: "the canonical writer preserves those
 * spellings"), and adopted-ruling finding 6 binds the SDK's types, builder and
 * `assertSameSession` to carry the whole set rather than the convenient numeric half.
 */
export type BitDepth = 16 | 24 | "32f";

/** The four launch render rates. Anything else refuses at `$.sample_rate_hz`. */
export type SessionSampleRateHz = 44_100 | 48_000 | 88_200 | 96_000;

/**
 * A declared source: exactly the five keys Session V1 has, minus its ID.
 *
 * `locator`, `identity`, `mapping`, `region`, `startFrame` and the per-source `sampleRateHz` are
 * all gone with #241/A2. A source names *content* -- a `sha256:` identity over the canonical PCM
 * preimage -- and declares the shape that content must prove to have. Resolving the identity to
 * bytes and checking the declaration against them is host policy (issue 010), not a document
 * field, so there is nowhere here to write a file path.
 */
export interface SourceSpec {
  readonly channels: 1 | 2;
  readonly bitDepth: BitDepth;
  /** Full canonical content length in frames, beginning at frame zero. Nonzero. */
  readonly frames: number;
  /** `sha256:` followed by exactly 64 lowercase hex digits. */
  readonly content: string;
}

/**
 * One lane of a track's fixed input section. Absent keys take the builtin's catalog default.
 *
 * `delaySamples` is issue #210 phase 2's input-side time alignment, in **samples** -- #147's
 * unit-in-name rule makes the unit part of the key, and a host that thinks in milliseconds
 * converts before it gets here, because the session never does.
 */
export interface BuiltinsSpec {
  readonly polarityInvert?: boolean;
  readonly trimDb?: number;
  readonly hpfHz?: number;
  readonly lpfHz?: number;
  readonly delaySamples?: number;
}

export interface FaderSpec {
  readonly leftDb?: number;
  readonly rightDb?: number;
  readonly leftMute?: boolean;
  readonly rightMute?: boolean;
}

export type PanSpec =
  | Readonly<{ left: number; right: number; smoothingSamples?: number }>
  | Readonly<{ matrix: Matrix2x2; smoothingSamples?: number }>;

/**
 * Which source channels a track's two lanes read.
 *
 * A track has one `source_id` and two channel indices, so the pair-of-tuples spelling the
 * pre-#241 builder accepted -- `{ left: [id, n], right: [id, n] }` -- could express two different
 * sources and then had to refuse it at validation time. This shape cannot express it at all.
 */
export type TrackSourceSpec =
  | string
  | Readonly<{ id: string; left: number; right: number }>;

export interface TrackSpec {
  readonly source: TrackSourceSpec;
  readonly builtins?: PerLane<BuiltinsSpec>;
  readonly fader?: FaderSpec;
  readonly pan?: PanSpec;
  readonly simd1?: readonly EffectDecl[];
  readonly dynamic?: readonly EffectDecl[];
  readonly simd2?: readonly EffectDecl[];
}

export type RouteSource =
  | Readonly<{ kind: "track"; trackId: string; tap: SendTap }>
  | Readonly<{ kind: "submix_output"; submixId: string }>;

export type RouteDestination =
  | Readonly<{ kind: "submix_input"; submixId: string }>
  | Readonly<{ kind: "output_input"; outputId: string }>;

/**
 * A routed sidechain: the same tagged source a route uses, plus the port it feeds.
 *
 * `portId` is the effect's own declared sidechain-input name, taken from the generated catalog's
 * port table (issue #278). Two consequences follow from the type alone, before `effect()` runs a
 * single check: a misspelling is a compile error naming the legal ports, and an effect that
 * declares no sidechain input gives `never` here, so a routed sidechain on it is unconstructible
 * rather than merely refused. The engine's boot-time refusals -- `effect.sidechain.unknown_port`,
 * `.missing`, `.unexpected` -- are unmoved and remain the authority; this stands in front of them.
 */
export interface SidechainSpec<E extends EffectId = EffectId> {
  readonly source: RouteSource;
  readonly portId: SidechainPortName<E>;
}

export interface RouteSpec {
  readonly id: string;
  readonly source: RouteSource;
  readonly destination: RouteDestination;
  /** Absent is the identity matrix, which is the only sane default for a 2x2 send. */
  readonly matrix?: Matrix2x2;
  readonly gainDb?: number;
}

/**
 * A parameter value in *display* units.
 *
 * `perLane` parameters additionally accept an explicit left/right pair; `shared` parameters do
 * not, because a shared row has one value and a pair would have to be silently collapsed.
 */
type ParameterScalar<P> = P extends { readonly domainName: "boolean" }
  ? boolean
  : P extends { readonly domainName: "enumeration"; readonly enumChoices: readonly (infer C)[] }
    ? C extends { readonly label: infer Label extends string } ? Label : never
    : number;

type ParameterInput<P> = P extends { readonly channelPolicyName: "perLane" }
  ? PerLane<ParameterScalar<P>>
  : ParameterScalar<P>;

export type EffectParamValues<E extends EffectId> = Partial<{
  [Name in EffectParameterName<E>]: ParameterInput<
    Extract<EffectParameter<E>, { readonly name: Name }>
  >;
}>;

export interface EffectOptions<E extends EffectId = EffectId> {
  /** Session V1 rack-local identity. Deliberately separate from the native `effectId`. */
  readonly slotId?: string;
  readonly bypass?: boolean;
  /** Launch native descriptors publish only the normal quality row. */
  readonly quality?: "normal";
  readonly linkMode?: "dual_mono" | "maximum" | "average";
  /** The channel a scalar parameter value addresses. Per-lane pairs override it. */
  readonly channel?: Channel;
  readonly sidechain?: SidechainSpec<E>;
}

export interface EffectDecl<E extends EffectId = EffectId> {
  readonly effectId: E;
  /**
   * An omitted slot is materialized by `.track()` from the rack name and declaration order, so
   * the emitted document always carries a nonempty rack-local ID even though a caller who does
   * not automate the effect never has to invent one.
   */
  readonly slotId?: string;
  readonly parameters: EffectParamValues<E>;
  readonly options:
    Required<Pick<EffectOptions<E>, "bypass" | "quality" | "linkMode" | "channel">>
      & Pick<EffectOptions<E>, "sidechain">;
}

export type AutomationShape = "step" | "linear" | "exponential";

/**
 * One automation span. Sample times are `bigint` because they are u64 on the wire and a session
 * long enough to exceed `Number.MAX_SAFE_INTEGER` must not silently lose its last digits.
 */
export interface AutomationSegment {
  readonly shape: AutomationShape;
  readonly startSample: bigint;
  readonly endSample: bigint;
  readonly startValue: number;
  readonly endValue: number;
}

/**
 * What a span addresses.
 *
 * `slotId` names a rack-local effect, never a positional index, so inserting an effect ahead of an
 * automated one cannot silently re-point the automation. For `rack: "builtins"` there is no
 * instance to name and the key may be omitted; the builder writes the schema's fixed `"strip"`
 * literal, and refuses any other spelling.
 */
export interface AutomationTarget {
  readonly trackId: string;
  readonly rack: AutomationRack;
  readonly slotId?: string;
  /** A catalog parameter name for an effect rack, or a builtin parameter name for `builtins`. */
  readonly parameter: string;
  readonly channel: Channel;
}

export interface AutomationSpec {
  readonly id: string;
  readonly target: AutomationTarget;
  readonly segments: readonly AutomationSegment[];
}
