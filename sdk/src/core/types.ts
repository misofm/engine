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
 * engine's own `parameter-metadata` output. A hand-written union would be a sixth copy
 * of a table that has already drifted five times (issue #207's N-13(d)), so there is none.
 */

/**
 * The two places an effect instance lives (owner decision 12).
 *
 * `console` is the session-level console strip: each slot is declared once for the session, in
 * `pre_insert` or `post_insert`, and every track carries one entry per slot with only its own
 * bypass and parameters. `inserts` is the per-track chain between the two console sections, with
 * full effect declarations. These are the session's rack tokens, and the live controls, the
 * observations and the automation targets all name an effect by one of them.
 */
export type Rack = "console" | "inserts";

/**
 * The three racks an automation target may name.
 *
 * `builtins` is not a rack of instances -- it is the strip's own fixed section, admitted as an
 * automation target by issue #178 under #210's D2. It is spelled here rather than in `Rack`
 * because a caller must not be able to place an effect in it.
 */
export type AutomationRack = Rack | "builtins";

export type Channel = "left" | "right" | "both";

/**
 * The seven strip taps a route or a routed sidechain may read from, in signal order. A track and a
 * submix strip carry the same seven.
 *
 * Decision 12 renamed them in place, with wire codes 1-7 unchanged: `post_input` follows the
 * input section, `insert_send` follows `console.pre_insert`, `insert_return` follows the inserts,
 * `pre_fader` follows `console.post_insert`, and `post_pan` follows the pan or matrix.
 */
export type SendTap =
  | "input"
  | "post_input"
  | "insert_send"
  | "insert_return"
  | "pre_fader"
  | "post_fader"
  | "post_pan";

export type Matrix2x2 = Readonly<{ ll: number; lr: number; rl: number; rr: number }>;

/** One value for both lanes, or an explicit left/right pair in either spelling. */
export type PerLane<T> = T | Readonly<{ left: T; right: T }> | readonly [left: T, right: T];

/**
 * The closed source bit-depth token set.
 *
 * Integer `16`, integer `24`, and the *string* `"32f"` -- three tokens, two JSON types. That
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
 * all gone with #241/A2. A source names *content* -- a `blake3:` identity over the canonical PCM
 * preimage -- and declares the shape that content must prove to have. Resolving the identity to
 * bytes and checking the declaration against them is host policy (issue 010), not a document
 * field, so there is nowhere here to write a file path.
 */
export interface SourceSpec {
  readonly channels: 1 | 2;
  readonly bitDepth: BitDepth;
  /** Full canonical content length in frames, beginning at frame zero. Nonzero. */
  readonly frames: number | bigint;
  /** `blake3:` followed by exactly 64 lowercase hex digits. */
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
  /**
   * This track's knobs for every session console slot: exactly one entry per slot, in the
   * session's slot order (`pre_insert`, then `post_insert`). May be omitted only when the session
   * declares no slot; otherwise the builder refuses the track with `console.entry_missing`, as
   * the engine would, rather than inventing entries the caller never wrote.
   */
  readonly console?: readonly ConsoleEntrySpec[];
  /** The per-track insert chain, in signal order: full effect declarations from `effect()`. */
  readonly inserts?: readonly EffectDecl[];
  readonly fader?: FaderSpec;
  readonly pan?: PanSpec;
}

/**
 * A submix strip: a track's strip without its source fields (owner decision 13).
 *
 * Every field follows the track's rules and defaults, and `console` is checked against the session
 * console exactly as a track's is, so a submix with a spec follows `console()`. A submix declared
 * with no spec at all is the transparent strip instead: the identity input section, no inserts, a
 * 0 dB unmuted fader, the identity matrix and every console slot bypassed. Its latency is still paid.
 */
export type SubmixSpec = Omit<TrackSpec, "source">;

/**
 * A VCA group (#1240): a control-only fader with no audio path. `fader` is a per-lane dB offset in
 * `[-144, 24]` that adds to every reachable member's own fader, and its mute mutes every reachable
 * member; it defaults to 0 dB unmuted on both lanes. `members` names already-declared tracks,
 * submixes and VCAs, each once; VCAs nest and overlap, and membership is acyclic.
 */
export interface VcaSpec {
  readonly fader?: FaderSpec;
  readonly members: readonly string[];
}

/**
 * The native effects a console slot may name (owner decision 12, "Eligibility").
 *
 * Every console slot always banks, so it must be an effect whose bank kernel the console can rely
 * on. The list is the engine's `effect_compiler::CONSOLE_ELIGIBLE_EFFECTS`; the SDK's copy is
 * held to it by an eval that boots every catalog effect as a slot.
 */
export type ConsoleEffectId =
  | "miso.parametric-eq"
  | "miso.compressor"
  | "miso.gate-expander"
  | "miso.soft-clip"
  | "miso.transient-shaper"
  | "miso.true-peak-limiter";

/**
 * One session console slot, declared once for every track.
 *
 * A slot carries exactly the fields the session's slot declaration has: its stable `slot` ID,
 * unique across both sections, its native effect, and the quality and link mode every track runs
 * it at. It takes no sidechain -- a keyed effect is an insert -- and no bypass or parameters,
 * which are each track's own, in its `console` entry.
 */
export interface ConsoleSlotSpec<E extends ConsoleEffectId = ConsoleEffectId> {
  readonly slot: string;
  readonly effectId: E;
  /** Launch native descriptors publish only the normal quality row. */
  readonly quality?: "normal";
  readonly linkMode?: "dual_mono" | "maximum" | "average";
}

/** The session console: the slots before the insert point, and the slots after it. */
export interface ConsoleSpec {
  readonly preInsert?: readonly ConsoleSlotSpec[];
  readonly postInsert?: readonly ConsoleSlotSpec[];
}

/**
 * One track's knobs for one console slot.
 *
 * `parameters` are display-unit values by catalog name, exactly as `effect()` takes them, and are
 * checked against the slot's own effect. Pass the slot's effect as `E` to have them typed.
 */
export interface ConsoleEntrySpec<E extends EffectId = EffectId> {
  readonly slot: string;
  readonly bypass?: boolean;
  readonly parameters?: EffectParamValues<E>;
  /** The channel a scalar parameter value addresses. Per-lane pairs override it. */
  readonly channel?: Channel;
}

/**
 * Where a route or a routed sidechain reads: one of a track's or a submix's seven taps.
 *
 * The retired `submix_output` source is refused at runtime; it was the submix's `post_pan` tap.
 */
export type RouteSource =
  | Readonly<{ kind: "track"; trackId: string; tap: SendTap }>
  | Readonly<{ kind: "submix"; submixId: string; tap: SendTap }>;

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
  /**
   * The send's on/off switch; absent is `false`. A muted route stays in the graph, with its edge,
   * its latency compensation and its level kept, and contributes silence (issue #1216).
   */
  readonly mute?: boolean;
  /**
   * Whether the send follows its source strip's lane mutes, as a console's "follow mute": a muted
   * source lane's matrix column contributes nothing (issue #1218). Only a route into a submix may
   * follow. Absent is `true` for a route into a submix and `false` for a route into the output;
   * `true` on a route into the output is refused.
   */
  readonly followsMute?: boolean;
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
  /** The insert's stable ID on its track. Deliberately separate from the native `effectId`. */
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
   * An omitted ID is materialized by `.track()` from the insert's position (`insert-1`, ...), so
   * the emitted document always carries a nonempty ID even though a caller who does not address
   * the insert never has to invent one.
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
  readonly startSample: number | bigint;
  readonly endSample: number | bigint;
  readonly startValue: number;
  readonly endValue: number;
}

/**
 * What a span addresses.
 *
 * `slotId` names an effect by its stable ID, never a positional index, so inserting an effect
 * ahead of an automated one cannot silently re-point the automation. For `rack: "console"` it is
 * the console slot's ID, and for `rack: "inserts"` the insert's ID on this track. For
 * `rack: "builtins"` there is no instance to name and the key may be omitted; the builder writes
 * the schema's fixed `"strip"` literal, and refuses any other spelling.
 */
export interface AutomationTarget {
  readonly trackId: string;
  readonly rack: AutomationRack;
  readonly slotId?: string;
  /** A catalog parameter name for an effect, or a builtin parameter name for `builtins`. */
  readonly parameter: string;
  readonly channel: Channel;
}

export interface AutomationSpec {
  readonly id: string;
  readonly target: AutomationTarget;
  readonly segments: readonly AutomationSegment[];
}
