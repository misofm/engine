import {
  MESSAGE_SESSION_TRANSACTION_APPLY,
  WIRE_BOOL,
  WIRE_F32,
  WIRE_U16,
  WIRE_U32,
  WIRE_U64,
  WIRE_U8,
  WIRE_UTF8,
  commandFrame,
  concat,
  f32,
  field,
  nested,
  requireU32,
  requireU64,
  text,
  u16,
  u32,
  u8,
} from "./control-frame.ts";
import { MisoUsageError } from "./errors.ts";
import {
  SessionBuilder,
  builderContext,
  automationTargetRecord,
  launchRate,
  normalizeAutomation,
  normalizeAutomationSegments,
  normalizeConsoleEntries,
  normalizeConsoleSlots,
  normalizeInsert,
  normalizeInserts,
  normalizeRoute,
  normalizeRouteDestinationSpec,
  normalizeRouteSourceSpec,
  normalizeSidechain,
  normalizeSource,
  normalizeSubmix,
  normalizeTrack,
  normalizeVca,
  quality,
  quantum,
  refuseOutputFollowsMute,
  requireSegments,
  resolveAutomationTarget,
  resolveLanes,
  retargetInsert,
  stableId,
  trackSourceRef,
} from "./session.ts";
import type {
  AutomationScope,
  ConsoleSlotEntry,
  ModelRecord,
  ModelValue,
  RouteGraph,
  SourceEntry,
} from "./session.ts";
import { CONTROLS } from "../generated/controls.ts";
import { CODE, fail, freeze, knownKeys } from "../internal/validation.ts";
import type { EffectId } from "../generated/catalog.ts";
import type {
  AutomationSegment,
  AutomationSpec,
  AutomationTarget,
  ConsoleEntrySpec,
  ConsoleSpec,
  EffectDecl,
  RouteDestination,
  RouteSource,
  RouteSpec,
  SessionSampleRateHz,
  SidechainSpec,
  SourceSpec,
  SubmixSpec,
  TrackSourceSpec,
  TrackSpec,
  VcaSpec,
} from "./types.ts";

/**
 * The SDK's encoder of structural session edits: a typed transaction builder over a base
 * `SessionBuilder`, and the `SESSION_TRANSACTION_APPLY` frame the engine's protocol codec would
 * write for the same edits (issue #1383, decision 15 D15-11).
 *
 * # One validator per field
 *
 * Each method validates its arguments with the session builder's own normalizers
 * (`normalizeSource`, `normalizeTrack`, `resolveLanes`) and encodes the normalized record they
 * return, never the caller's spec. So a track edit carries exactly the values the same track
 * declared on a `SessionBuilder` would write: defaults filled, values rounded to `f32`, parameters
 * in canonical order. The base session supplies what a spec is checked against -- the rate, the
 * sources, the console slots -- and the builder keeps a working copy, so an `upsertTrack` sees an
 * earlier `upsertSource` of the same transaction.
 *
 * The builder checks the shape of each edit, not the references between entities: the engine's
 * store refuses those at the transaction's final validation with a typed missing-entity error, so
 * nothing is lost silently. A sidechain source, a sidechain whose source track a later edit
 * removes, and automation that targets a removed insert are the store's to refuse, as they are for
 * `SessionBuilder`.
 *
 * # What the frame carries, and when
 *
 * `build()` encodes the edits: each one is a `SessionEdit` message `{ opcode: u16, payload }`
 * whose payload follows `crates/protocol/src/schema.rs` (`session::*`) field by field, in the
 * order `crates/protocol/src/session_wire.rs` writes it. Every model field is mandatory there, so
 * every field carries the mandatory flag. The request ID and the expected revision are not part of
 * the transaction: the endpoint stamps both when it sends it (`encodeSessionTransaction`), as the
 * `SET_CONTROL` queue does (#1563 D4). Live values are not edited here: a value is one
 * `SET_CONTROL` (decision 17).
 *
 * # Strip edits
 *
 * The rack and effect edits address a strip -- a track, or a submix of the base session -- and
 * its `inserts`, the only rack a strip edits: the engine refuses `builtins` (no effects) and
 * `console` (session-level slots), so the methods take no rack name and always write `inserts`.
 * An insert is named by its stable ID on the strip (`effect()`'s `slotId`), never by its native
 * effect ID. The builder keeps each strip's working inserts, so a later edit sees an earlier one.
 *
 * `setConsole` replaces the console's slots (#1385). Every strip carries one entry per slot, so a
 * transaction that changes the slots also gives each strip its new entries -- `setTrackConsole`,
 * `upsertTrack` or `upsertSubmix`, each checked against the new slots -- or the store's final
 * validation refuses it. The engine accepts `setTrackConsole` only after an earlier `SetConsole`
 * of the same transaction left the console different from the base session's
 * (`session.edit.value_op_retired`, #1561 D3), and this builder refuses it, after its arguments are
 * validated, on the same rule; a console knob's value is a `SET_CONTROL`. These two are the only
 * methods that take `{ rampSamples }`, written as the optional `ramp_samples` field only when
 * given (#1394 D1).
 *
 * # Session, submix, output and route edits
 *
 * A rate or quantum change is encoded like any other edit, and the engine's control plane refuses
 * it at apply with its typed result (`control.apply.reprepare_required`, #1290a), so the C ABI and
 * the browser see the same transaction. A route's
 * endpoints are checked against the working graph, as `SessionBuilder.route` checks them against
 * its declarations; an endpoint a later edit removes is the store's to refuse.
 *
 * # Automation and VCA edits
 *
 * An automation target resolves through `resolveAutomationTarget` against the working strips,
 * inserts, console slots, routes and VCAs, exactly as `SessionBuilder.automation` resolves it, and
 * its spans are written in the unit of the row it names. Whether that row is automatable is the
 * engine's rule (#1566): the store refuses the transaction, as `validate()` refuses the built
 * session. A VCA's members must be in the working graph and are written in the model's sorted
 * order; a membership cycle, which only a replaced VCA can make, is the store's to refuse.
 *
 * `sdk/test/transaction-evals.mjs` compares these bytes to the codec's re-encoding of them and
 * the store's snapshot after them to a `SessionBuilder` declaring the result
 * (`hosts/host-web/examples/sdk_transaction_oracle.rs`).
 */

// `SessionEditOpcode` (`crates/protocol/src/model.rs`).
const OP_SET_SESSION_ID = 0x0001;
const OP_SET_SAMPLE_RATE_HZ = 0x0002;
const OP_SET_QUANTUM_FRAMES = 0x0003;
const OP_SET_RENDER_PROFILE = 0x0004;
const OP_SET_OUTPUT_PROFILE = 0x0005;
const OP_SET_CONSOLE = 0x0007;
const OP_UPSERT_SOURCE = 0x0100;
const OP_REMOVE_SOURCE = 0x0101;
const OP_SET_SOURCE_CONTENT = 0x0103;
const OP_UPSERT_TRACK = 0x0200;
const OP_REMOVE_TRACK = 0x0201;
const OP_SET_TRACK_SOURCE_ASSIGNMENT = 0x0202;
const OP_SET_TRACK_RACK = 0x0204;
const OP_PUT_TRACK_EFFECT = 0x0205;
const OP_REMOVE_TRACK_EFFECT = 0x0206;
const OP_SET_TRACK_EFFECT_ORDER = 0x0207;
const OP_SET_EFFECT_IDENTITY = 0x0208;
const OP_SET_EFFECT_QUALITY = 0x0209;
const OP_SET_EFFECT_SIDECHAIN = 0x020c;
const OP_SET_TRACK_CONSOLE = 0x0211;
const OP_UPSERT_SUBMIX = 0x0300;
const OP_REMOVE_SUBMIX = 0x0301;
const OP_UPSERT_OUTPUT = 0x0400;
const OP_REMOVE_OUTPUT = 0x0401;
const OP_UPSERT_ROUTE = 0x0500;
const OP_REMOVE_ROUTE = 0x0501;
const OP_SET_ROUTE_SOURCE = 0x0502;
const OP_SET_ROUTE_DESTINATION = 0x0503;
const OP_UPSERT_AUTOMATION = 0x0600;
const OP_REMOVE_AUTOMATION = 0x0601;
const OP_SET_AUTOMATION_TARGET = 0x0602;
const OP_SET_AUTOMATION_SEGMENTS = 0x0603;
const OP_UPSERT_VCA = 0x0700;
const OP_REMOVE_VCA = 0x0701;

/** `RackName::Inserts`'s wire code (`schema::parameter_rack_wire`). */
const RACK_INSERTS = 2;

// The protocol's enum codes: `SourceBitDepth::wire`, `enum_quality`, `enum_link`, `enum_tap`, and
// `schema::parameter_{channel,unit}_wire`. Each is the token's 1-based declaration order.
const BIT_DEPTH_CODES: ReadonlyMap<ModelValue, number> = new Map<ModelValue, number>([
  [16, 1],
  [24, 2],
  ["32f", 3],
]);
const QUALITY_CODES = codes(["draft", "normal", "high"]);
const LINK_MODE_CODES = codes(["dual_mono", "maximum", "average"]);
/** A console entry's link mode (#1369): `slot` is code 1, then the three link tokens. */
const CONSOLE_LINK_MODE_CODES = codes(["slot", "dual_mono", "maximum", "average"]);
const TAP_CODES = codes([
  "input",
  "post_input",
  "insert_send",
  "insert_return",
  "pre_fader",
  "post_fader",
  "post_pan",
]);
const CHANNEL_CODES = codes(["left", "right", "both"]);
const UNIT_CODES = codes(["db", "hz", "milliseconds", "samples", "linear", "ratio"]);
const ROUTE_SOURCE_TAGS = codes(["track", "submix"]);
const ROUTE_DESTINATION_TAGS = codes(["submix_input", "output_input"]);
/** `RenderMode::SingleThread` and `SampleFormat::F32Planar`, each its enum's only code. */
const RENDER_MODE_CODES = codes(["single_thread"]);
const SAMPLE_FORMAT_CODES = codes(["f32_planar"]);
const SHAPE_CODES = codes(["step", "linear", "exponential"]);
/** The control address codes, from the generated control tables (`CONTROLS.address`). */
const OWNER_KIND_CODES = named(CONTROLS.address.ownerKinds);
const SECTION_KIND_CODES = named(CONTROLS.address.sectionKinds);
const LANE_CODES = named(CONTROLS.address.lanes);
const RENDER_PROFILE_KEYS: ReadonlySet<string> = new Set(["id", "mode"]);
const OUTPUT_PROFILE_KEYS: ReadonlySet<string> = new Set(["id", "channels", "sampleFormat"]);
const SIDECHAIN_TAGS = codes(["none", "routed"]);
const IDENTITY_NATIVE = 1;
const PAN_TAG = 1;
const MATRIX_TAG = 2;

function codes(tokens: readonly string[]): ReadonlyMap<ModelValue, number> {
  return new Map<ModelValue, number>(tokens.map((token, index) => [token, index + 1]));
}

function named(rows: readonly { readonly name: string; readonly code: number }[]): ReadonlyMap<ModelValue, number> {
  return new Map<ModelValue, number>(rows.map((row) => [row.name, row.code]));
}

/** A session's render profile: a stable ID and the only V1 render mode, the default. */
type RenderProfileSpec = { readonly id: string; readonly mode?: "single_thread" };
/** A session's output profile: a stable ID, two channels and planar `f32`, the defaults. */
type OutputProfileSpec = { readonly id: string; readonly channels?: 2; readonly sampleFormat?: "f32_planar" };

/** One structural edit of a session transaction, as the caller gave it. */
export type SessionEditSpec =
  | { readonly kind: "setSessionId"; readonly sessionId: string }
  | { readonly kind: "setSampleRateHz"; readonly sampleRateHz: SessionSampleRateHz }
  | { readonly kind: "setQuantumFrames"; readonly quantumFrames: number }
  | { readonly kind: "setRenderProfile"; readonly renderProfile: RenderProfileSpec }
  | { readonly kind: "setOutputProfile"; readonly outputProfile: OutputProfileSpec }
  | { readonly kind: "setConsole"; readonly console: ConsoleSpec; readonly rampSamples?: number }
  | { readonly kind: "upsertSource"; readonly sourceId: string; readonly source: SourceSpec }
  | { readonly kind: "removeSource"; readonly sourceId: string }
  | { readonly kind: "setSourceContent"; readonly sourceId: string; readonly source: SourceSpec }
  | { readonly kind: "upsertTrack"; readonly trackId: string; readonly track: TrackSpec }
  | { readonly kind: "removeTrack"; readonly trackId: string }
  | {
    readonly kind: "setTrackSourceAssignment";
    readonly trackId: string;
    readonly source: TrackSourceSpec;
  }
  | { readonly kind: "setTrackRack"; readonly trackId: string; readonly inserts: readonly EffectDecl[] }
  | {
    readonly kind: "putTrackEffect";
    readonly trackId: string;
    readonly finalPosition: number;
    readonly effect: EffectDecl;
  }
  | { readonly kind: "removeTrackEffect"; readonly trackId: string; readonly slotId: string }
  | { readonly kind: "setTrackEffectOrder"; readonly trackId: string; readonly slotIds: readonly string[] }
  | {
    readonly kind: "setEffectIdentity";
    readonly trackId: string;
    readonly slotId: string;
    readonly effectId: EffectId;
  }
  | {
    readonly kind: "setEffectQuality";
    readonly trackId: string;
    readonly slotId: string;
    readonly quality: "normal";
  }
  | {
    readonly kind: "setEffectSidechain";
    readonly trackId: string;
    readonly slotId: string;
    readonly sidechain: SidechainSpec | null;
  }
  | {
    readonly kind: "setTrackConsole";
    readonly trackId: string;
    readonly console: readonly ConsoleEntrySpec[];
    readonly rampSamples?: number;
  }
  | { readonly kind: "upsertSubmix"; readonly submixId: string; readonly submix?: SubmixSpec }
  | { readonly kind: "removeSubmix"; readonly submixId: string }
  | { readonly kind: "upsertOutput"; readonly outputId: string }
  | { readonly kind: "removeOutput"; readonly outputId: string }
  | { readonly kind: "upsertRoute"; readonly route: RouteSpec }
  | { readonly kind: "removeRoute"; readonly routeId: string }
  | { readonly kind: "setRouteSource"; readonly routeId: string; readonly source: RouteSource }
  | { readonly kind: "setRouteDestination"; readonly routeId: string; readonly destination: RouteDestination }
  | { readonly kind: "upsertAutomation"; readonly automation: AutomationSpec }
  | { readonly kind: "removeAutomation"; readonly automationId: string }
  | { readonly kind: "setAutomationTarget"; readonly automationId: string; readonly target: AutomationTarget }
  | {
    readonly kind: "setAutomationSegments";
    readonly automationId: string;
    readonly segments: readonly AutomationSegment[];
  }
  | { readonly kind: "upsertVca"; readonly vcaId: string; readonly vca: VcaSpec }
  | { readonly kind: "removeVca"; readonly vcaId: string };


/**
 * A built session transaction: its edits in order and their encoding, without the request ID and
 * expected revision the endpoint stamps when it sends it. Immutable apart from `payload`, which a
 * caller must not change.
 */
export interface SessionTransaction {
  readonly edits: readonly SessionEditSpec[];
  /** The encoded edits: the frame's top-level fields, one `EDIT` per edit. */
  readonly payload: Uint8Array;
  /** The number of top-level fields in `payload`, which is the number of edits. */
  readonly fieldCount: number;
}

/** What a transaction frame needs besides its edits. */
export interface SessionTransactionRequest {
  /** A nonzero `u64` the endpoint correlates the reply by. */
  readonly requestId: bigint;
  /** The committed revision the transaction applies to. */
  readonly expectedRevision: bigint;
}

interface BuilderState {
  /** The session's rate after the edits so far. */
  readonly sampleRateHz: number;
  /** The base session's console slots, which `consoleChanged` compares the working ones with. */
  readonly baseSlots: readonly ConsoleSlotEntry[];
  /** The console slots after the edits so far. */
  readonly slots: readonly ConsoleSlotEntry[];
  /** The sources after the edits so far, by ID. */
  readonly sources: ReadonlyMap<string, SourceEntry>;
  /** The track, submix, VCA and output IDs after the edits so far: one namespace. */
  readonly tracks: ReadonlySet<string>;
  readonly submixes: ReadonlySet<string>;
  readonly vcas: ReadonlySet<string>;
  readonly outputs: ReadonlySet<string>;
  /** The routes' records after the edits so far, by ID. */
  readonly routes: ReadonlyMap<string, ModelRecord>;
  /** The stored automations' records after the edits so far, by ID. */
  readonly automations: ReadonlyMap<string, ModelRecord>;
  /** Each strip's (track's or submix's) insert records after the edits so far, by strip ID. */
  readonly strips: ReadonlyMap<string, readonly ModelRecord[]>;
  /**
   * Whether the working console differs from the base session's, which opens `setTrackConsole`
   * as the engine's store opens `SetTrackConsole` (#1561 D3).
   */
  readonly consoleChanged: boolean;
  readonly edits: readonly { readonly spec: SessionEditSpec; readonly bytes: Uint8Array }[];
}

/**
 * Typed structural edits of one session, validated as `SessionBuilder` validates the same specs.
 * Every method returns a new builder; a builder can be shared, forked and reused.
 */
export class TransactionBuilder {
  readonly #state: BuilderState;

  /** @internal Use `transaction(base)`. */
  constructor(state: BuilderState) {
    this.#state = state;
  }

  /** Rename the session. */
  setSessionId(sessionId: string): TransactionBuilder {
    const id = stableId(sessionId, "setSessionId().sessionId");
    return this.#next({ kind: "setSessionId", sessionId: id }, OP_SET_SESSION_ID, [utf8(1, id)], {});
  }

  /**
   * Change the session's rate to another launch rate. The engine refuses a rate change at apply
   * with its typed result (#1290a); later edits of this builder are checked at the new rate.
   */
  setSampleRateHz(sampleRateHz: SessionSampleRateHz): TransactionBuilder {
    const rate = launchRate(sampleRateHz, "setSampleRateHz().sampleRateHz");
    return this.#next(
      { kind: "setSampleRateHz", sampleRateHz: rate },
      OP_SET_SAMPLE_RATE_HZ,
      [word(1, rate)],
      { sampleRateHz: rate },
    );
  }

  /** Change the render quantum, a nonzero `u32`. The engine refuses a change at apply (#1290a). */
  setQuantumFrames(quantumFrames: number): TransactionBuilder {
    const frames = quantum(quantumFrames, "setQuantumFrames().quantumFrames");
    return this.#next(
      { kind: "setQuantumFrames", quantumFrames: frames },
      OP_SET_QUANTUM_FRAMES,
      [word(1, frames)],
      {},
    );
  }

  /** Replace the render profile: a stable ID, and `single_thread`, the only V1 mode. */
  setRenderProfile(profile: RenderProfileSpec): TransactionBuilder {
    const path = "setRenderProfile()";
    if (profile === null || typeof profile !== "object" || Array.isArray(profile)) {
      fail(path, "expected { id, mode? }");
    }
    knownKeys(profile, RENDER_PROFILE_KEYS, path, "a render profile");
    const id = stableId(profile.id, `${path}.id`);
    const mode = profile.mode ?? "single_thread";
    if (mode !== "single_thread") fail(`${path}.mode`, "single_thread is the only V1 render mode", CODE.invalidEnum);
    return this.#next(
      { kind: "setRenderProfile", renderProfile: { id, mode } },
      OP_SET_RENDER_PROFILE,
      [message(1, [utf8(1, id), code(2, RENDER_MODE_CODES, mode)])],
      {},
    );
  }

  /** Replace the output profile: a stable ID, two channels and `f32_planar`, the only V1 output. */
  setOutputProfile(profile: OutputProfileSpec): TransactionBuilder {
    const path = "setOutputProfile()";
    if (profile === null || typeof profile !== "object" || Array.isArray(profile)) {
      fail(path, "expected { id, channels?, sampleFormat? }");
    }
    knownKeys(profile, OUTPUT_PROFILE_KEYS, path, "an output profile");
    const id = stableId(profile.id, `${path}.id`);
    const channels = profile.channels ?? 2;
    if (channels !== 2) {
      fail(`${path}.channels`, "V1 output must contain exactly two dual-mono channels", CODE.outOfRange);
    }
    const sampleFormat = profile.sampleFormat ?? "f32_planar";
    if (sampleFormat !== "f32_planar") {
      fail(`${path}.sampleFormat`, "f32_planar is the only V1 sample format", CODE.invalidEnum);
    }
    return this.#next(
      { kind: "setOutputProfile", outputProfile: { id, channels, sampleFormat } },
      OP_SET_OUTPUT_PROFILE,
      [message(1, [utf8(1, id), byte(2, channels), code(3, SAMPLE_FORMAT_CODES, sampleFormat)])],
      {},
    );
  }

  /**
   * Replace the session console's slots, validated as `SessionBuilder.console` validates them.
   * Each strip then needs its entries for the new slots in the same transaction: a later
   * `setTrackConsole`, `upsertTrack` or `upsertSubmix` is checked against them. `rampSamples`, when
   * given, is the edit's own ramp length, as `setTrackConsole`'s.
   */
  setConsole(spec: ConsoleSpec, options: { readonly rampSamples?: number } = {}): TransactionBuilder {
    const path = "setConsole()";
    const slots = normalizeConsoleSlots(spec, path);
    const rampSamples = rampOption(options, path);
    return this.#next(
      {
        kind: "setConsole",
        console: {
          preInsert: (spec.preInsert ?? []).map((slot) => ({ ...slot })),
          postInsert: (spec.postInsert ?? []).map((slot) => ({ ...slot })),
        },
        ...(rampSamples === undefined ? {} : { rampSamples }),
      },
      OP_SET_CONSOLE,
      [message(1, consoleFields(slots)), ...rampField(2, rampSamples)],
      { slots, consoleChanged: !sameSlots(slots, this.#state.baseSlots) },
    );
  }

  /** Declare a source, or replace the source with this ID. */
  upsertSource(sourceId: string, spec: SourceSpec): TransactionBuilder {
    stableId(sourceId, "upsertSource().id");
    const record = normalizeSource(sourceId, spec, `upsertSource("${sourceId}")`);
    const sources = new Map(this.#state.sources);
    sources.set(sourceId, freeze({ id: sourceId, spec: { ...spec } }));
    return this.#next(
      { kind: "upsertSource", sourceId, source: { ...spec } },
      OP_UPSERT_SOURCE,
      [message(1, sourceFields(record))],
      { sources },
    );
  }

  /** Remove a declared source. A track that still reads it makes the engine refuse the transaction. */
  removeSource(sourceId: string): TransactionBuilder {
    this.#declaredSource(sourceId, `removeSource("${String(sourceId)}")`);
    const sources = new Map(this.#state.sources);
    sources.delete(sourceId);
    return this.#next(
      { kind: "removeSource", sourceId },
      OP_REMOVE_SOURCE,
      [utf8(1, sourceId)],
      { sources },
    );
  }

  /** Replace a declared source's content, channel count, sample depth and length. */
  setSourceContent(sourceId: string, spec: SourceSpec): TransactionBuilder {
    const path = `setSourceContent("${String(sourceId)}")`;
    this.#declaredSource(sourceId, path);
    const record = normalizeSource(sourceId, spec, path);
    const sources = new Map(this.#state.sources);
    sources.set(sourceId, freeze({ id: sourceId, spec: { ...spec } }));
    // `SetSourceContent` is a source message without its own field numbering: the same five
    // fields, the ID first.
    return this.#next(
      { kind: "setSourceContent", sourceId, source: { ...spec } },
      OP_SET_SOURCE_CONTENT,
      sourceFields(record),
      { sources },
    );
  }

  /** Declare a track, or replace the track with this ID, with the session builder's rules. */
  upsertTrack(trackId: string, spec: TrackSpec): TransactionBuilder {
    stableId(trackId, "upsertTrack().id");
    this.#freeGraphId(trackId, "tracks", `upsertTrack("${trackId}")`);
    const record = normalizeTrack(
      { id: trackId, spec },
      [...this.#state.sources.values()],
      this.#state.slots,
      this.#state.sampleRateHz,
      "upsertTrack",
    );
    const tracks = new Set(this.#state.tracks);
    tracks.add(trackId);
    return this.#next(
      { kind: "upsertTrack", trackId, track: { ...spec } },
      OP_UPSERT_TRACK,
      [message(1, trackFields(record))],
      { tracks, strips: this.#withInserts(trackId, records((record.inserts as ModelRecord).effects)) },
    );
  }

  /** Remove a declared track. */
  removeTrack(trackId: string): TransactionBuilder {
    this.#declaredTrack(trackId, `removeTrack("${String(trackId)}")`);
    const tracks = new Set(this.#state.tracks);
    tracks.delete(trackId);
    const strips = new Map(this.#state.strips);
    strips.delete(trackId);
    return this.#next({ kind: "removeTrack", trackId }, OP_REMOVE_TRACK, [utf8(1, trackId)], { tracks, strips });
  }

  /**
   * Point a declared track at a source and lanes, written as a track's `source` is: a source ID
   * (the whole source) or `{ id, left?, right? }`.
   */
  setTrackSourceAssignment(trackId: string, source: TrackSourceSpec): TransactionBuilder {
    const path = `setTrackSourceAssignment("${String(trackId)}")`;
    this.#declaredTrack(trackId, path);
    const reference = trackSourceRef(source, `${path}.source`);
    const entry = this.#state.sources.get(reference.id);
    if (entry === undefined) {
      fail(`${path}.source`, `'${reference.id}' is not a declared source`, CODE.missingEntity);
    }
    const lanes = resolveLanes(reference, entry.spec.channels, `${path}.source`);
    return this.#next(
      {
        kind: "setTrackSourceAssignment",
        trackId,
        source: typeof source === "string" ? source : { ...source },
      },
      OP_SET_TRACK_SOURCE_ASSIGNMENT,
      [utf8(1, trackId), utf8(2, reference.id), byte(3, lanes.left), byte(4, lanes.right)],
      {},
    );
  }

  /** Replace a strip's whole `inserts` rack, validated as a track's `inserts` are. */
  setTrackRack(trackId: string, inserts: readonly EffectDecl[]): TransactionBuilder {
    const path = `setTrackRack("${String(trackId)}")`;
    this.#strip(trackId, path);
    if (!Array.isArray(inserts)) fail(`${path}.inserts`, "expected an array of effect() declarations");
    const effects = normalizeInserts(inserts, `${path}.inserts`);
    return this.#next(
      { kind: "setTrackRack", trackId, inserts: [...inserts] },
      OP_SET_TRACK_RACK,
      [
        utf8(1, trackId),
        byte(2, RACK_INSERTS),
        message(3, effects.map((effect) => message(1, effectFields(effect)))),
      ],
      { strips: this.#withInserts(trackId, effects) },
    );
  }

  /**
   * Insert an effect into a strip's `inserts` at `finalPosition`, after removing the insert with
   * the same ID if there is one (a move or a replacement). The effect must carry its `slotId`: a
   * position-derived ID could name, and so replace, another insert.
   */
  putTrackEffect(trackId: string, finalPosition: number, effect: EffectDecl): TransactionBuilder {
    const path = `putTrackEffect("${String(trackId)}")`;
    const current = this.#strip(trackId, path);
    if (effect === null || typeof effect !== "object") {
      fail(`${path}.effect`, "expected an effect declaration from effect()");
    }
    if (effect.slotId === undefined) {
      fail(`${path}.effect.slotId`, "a put effect names its insert ID with effect(..., { slotId })");
    }
    const insert = normalizeInsert(effect, effect.slotId, `${path}.effect`);
    const position = requireU32(finalPosition, `${path}.finalPosition`);
    const remaining = current.filter((entry) => entry.id !== insert.id);
    if (position > remaining.length) {
      fail(
        `${path}.finalPosition`,
        `${position} is past the end of the ${remaining.length} other inserts`,
        "session.edit.invalid_final_position",
      );
    }
    const effects = [...remaining.slice(0, position), insert, ...remaining.slice(position)];
    return this.#next(
      { kind: "putTrackEffect", trackId, finalPosition: position, effect },
      OP_PUT_TRACK_EFFECT,
      [utf8(1, trackId), byte(2, RACK_INSERTS), word(3, position), message(4, effectFields(insert))],
      { strips: this.#withInserts(trackId, effects) },
    );
  }

  /** Remove one insert from a strip by its ID. */
  removeTrackEffect(trackId: string, slotId: string): TransactionBuilder {
    const path = `removeTrackEffect("${String(trackId)}")`;
    const current = this.#strip(trackId, path);
    this.#insert(current, slotId, path);
    return this.#next(
      { kind: "removeTrackEffect", trackId, slotId },
      OP_REMOVE_TRACK_EFFECT,
      [utf8(1, trackId), byte(2, RACK_INSERTS), utf8(3, slotId)],
      { strips: this.#withInserts(trackId, current.filter((entry) => entry.id !== slotId)) },
    );
  }

  /** Reorder a strip's inserts: `slotIds` is an exact permutation of its insert IDs. */
  setTrackEffectOrder(trackId: string, slotIds: readonly string[]): TransactionBuilder {
    const path = `setTrackEffectOrder("${String(trackId)}")`;
    const current = this.#strip(trackId, path);
    if (!Array.isArray(slotIds)) fail(`${path}.slotIds`, "expected an array of insert IDs");
    const ids: string[] = slotIds.map((id, index) => stableId(id, `${path}.slotIds[${index}]`));
    const byId = new Map(current.map((entry) => [entry.id, entry] as const));
    if (ids.length !== current.length || new Set(ids).size !== ids.length || ids.some((id) => !byId.has(id))) {
      fail(
        `${path}.slotIds`,
        `expected each of the strip's inserts exactly once: ${current.map((entry) => String(entry.id)).join(", ") || "none"}`,
        "session.edit.invalid_effect_order",
      );
    }
    return this.#next(
      { kind: "setTrackEffectOrder", trackId, slotIds: ids },
      OP_SET_TRACK_EFFECT_ORDER,
      [utf8(1, trackId), byte(2, RACK_INSERTS), ...ids.map((id) => utf8(3, id))],
      { strips: this.#withInserts(trackId, ids.map((id) => byId.get(id) as ModelRecord)) },
    );
  }

  /**
   * Run another native effect in an insert, keeping its ID, bypass, link mode, parameters and
   * sidechain. Refused unless each of those is valid for the new effect.
   */
  setEffectIdentity(trackId: string, slotId: string, effectId: EffectId): TransactionBuilder {
    const path = `setEffectIdentity("${String(trackId)}", "${String(slotId)}")`;
    const current = this.#strip(trackId, path);
    const insert = retargetInsert(this.#insert(current, slotId, path), effectId, `${path}.effectId`);
    return this.#effectEdit(
      { kind: "setEffectIdentity", trackId, slotId, effectId },
      OP_SET_EFFECT_IDENTITY,
      current,
      insert,
      message(4, identityFields(insert)),
    );
  }

  /** Set an insert's quality. Launch native effects publish only `normal`. */
  setEffectQuality(trackId: string, slotId: string, value: "normal"): TransactionBuilder {
    const path = `setEffectQuality("${String(trackId)}", "${String(slotId)}")`;
    const current = this.#strip(trackId, path);
    const found = this.#insert(current, slotId, path);
    const normalized = quality(value, `${path}.quality`);
    const insert = freeze({ ...found, quality: normalized });
    return this.#effectEdit(
      { kind: "setEffectQuality", trackId, slotId, quality: normalized },
      OP_SET_EFFECT_QUALITY,
      current,
      insert,
      code(4, QUALITY_CODES, normalized),
    );
  }

  /** Route an insert's sidechain, as `effect()`'s `sidechain` option, or clear it with `null`. */
  setEffectSidechain(trackId: string, slotId: string, sidechain: SidechainSpec | null): TransactionBuilder {
    const path = `setEffectSidechain("${String(trackId)}", "${String(slotId)}")`;
    const current = this.#strip(trackId, path);
    const found = this.#insert(current, slotId, path);
    const effectId = String(record(found.identity).effect_id);
    const normalized = sidechain === null
      ? freeze({ kind: "none" })
      : normalizeSidechain(effectId, sidechain, `${path}.sidechain`);
    const insert = freeze({ ...found, sidechain: normalized });
    return this.#effectEdit(
      { kind: "setEffectSidechain", trackId, slotId, sidechain: sidechain === null ? null : { ...sidechain } },
      OP_SET_EFFECT_SIDECHAIN,
      current,
      insert,
      message(4, sidechainFields(normalized)),
    );
  }

  /**
   * Give a strip its console entries, one per slot in slot order, after an earlier edit of this
   * transaction changed the console's slot set. Without one the engine refuses it
   * (`session.edit.value_op_retired`), and so does this builder: a console knob's value is a
   * `SET_CONTROL`. `rampSamples`, when given, is the edit's own ramp length: an integer in
   * `[0, 2^32 - 1]`, `0` a step; absent is the session's default ramp. The engine refuses a length
   * above one second at the session's rate.
   */
  setTrackConsole(
    trackId: string,
    console: readonly ConsoleEntrySpec[],
    options: { readonly rampSamples?: number } = {},
  ): TransactionBuilder {
    const path = `setTrackConsole("${String(trackId)}")`;
    this.#strip(trackId, path);
    if (!Array.isArray(console)) fail(`${path}.console`, "expected an array of console entries in slot order");
    const entries = normalizeConsoleEntries(console, this.#state.slots, `${path}.console`);
    const rampSamples = rampOption(options, path);
    if (!this.#state.consoleChanged) {
      fail(
        path,
        "a strip's console entries change only after the same transaction changes the console's "
          + "slots; set a console knob with SET_CONTROL",
        "session.edit.value_op_retired",
      );
    }
    return this.#next(
      {
        kind: "setTrackConsole",
        trackId,
        console: [...console],
        ...(rampSamples === undefined ? {} : { rampSamples }),
      },
      OP_SET_TRACK_CONSOLE,
      [
        utf8(1, trackId),
        ...entries.map((entry) => message(2, consoleEntryFields(entry))),
        ...rampField(3, rampSamples),
      ],
      {},
    );
  }

  /**
   * Declare a submix strip, or replace the submix with this ID, with the session builder's rules:
   * with a `spec` it follows a track's rules, checked against the working console's slots; with
   * none it is the transparent strip, every console slot bypassed.
   */
  upsertSubmix(submixId: string, spec?: SubmixSpec): TransactionBuilder {
    stableId(submixId, "upsertSubmix().id");
    const path = `upsertSubmix("${submixId}")`;
    this.#freeGraphId(submixId, "submixes", path);
    const record = normalizeSubmix(
      { id: submixId, spec },
      this.#state.slots,
      this.#state.sampleRateHz,
      "upsertSubmix",
    );
    const submixes = new Set(this.#state.submixes);
    submixes.add(submixId);
    return this.#next(
      { kind: "upsertSubmix", submixId, ...(spec === undefined ? {} : { submix: { ...spec } }) },
      OP_UPSERT_SUBMIX,
      [message(1, submixFields(record))],
      { submixes, strips: this.#withInserts(submixId, records((record.inserts as ModelRecord).effects)) },
    );
  }

  /** Remove a declared submix. A route or VCA that still names it makes the engine refuse. */
  removeSubmix(submixId: string): TransactionBuilder {
    this.#declared(this.#state.submixes, submixId, "submix", `removeSubmix("${String(submixId)}")`);
    const submixes = new Set(this.#state.submixes);
    submixes.delete(submixId);
    const strips = new Map(this.#state.strips);
    strips.delete(submixId);
    return this.#next(
      { kind: "removeSubmix", submixId },
      OP_REMOVE_SUBMIX,
      [utf8(1, submixId)],
      { submixes, strips },
    );
  }

  /** Declare an output. Its ID shares the graph-entity namespace. */
  upsertOutput(outputId: string): TransactionBuilder {
    stableId(outputId, "upsertOutput().id");
    this.#freeGraphId(outputId, "outputs", `upsertOutput("${outputId}")`);
    const outputs = new Set(this.#state.outputs);
    outputs.add(outputId);
    return this.#next(
      { kind: "upsertOutput", outputId },
      OP_UPSERT_OUTPUT,
      [message(1, [utf8(1, outputId)])],
      { outputs },
    );
  }

  /** Remove a declared output. A route that still reaches it makes the engine refuse. */
  removeOutput(outputId: string): TransactionBuilder {
    this.#declared(this.#state.outputs, outputId, "output", `removeOutput("${String(outputId)}")`);
    const outputs = new Set(this.#state.outputs);
    outputs.delete(outputId);
    return this.#next({ kind: "removeOutput", outputId }, OP_REMOVE_OUTPUT, [utf8(1, outputId)], { outputs });
  }

  /**
   * Declare a route, or replace the route with this ID, with the session builder's rules: both
   * endpoints in the working graph, the matrix and gain in their domains, and `followsMute`
   * defaulting to `true` into a submix and refused into the output.
   */
  upsertRoute(spec: RouteSpec): TransactionBuilder {
    stableId(spec?.id, "upsertRoute().id");
    const record = normalizeRoute(spec, this.#graph(), `upsertRoute("${spec.id}")`);
    const routes = new Map(this.#state.routes);
    routes.set(spec.id, record);
    return this.#next(
      { kind: "upsertRoute", route: { ...spec } },
      OP_UPSERT_ROUTE,
      [message(1, routeFields(record))],
      { routes },
    );
  }

  /** Remove a declared route. */
  removeRoute(routeId: string): TransactionBuilder {
    this.#route(routeId, `removeRoute("${String(routeId)}")`);
    const routes = new Map(this.#state.routes);
    routes.delete(routeId);
    return this.#next({ kind: "removeRoute", routeId }, OP_REMOVE_ROUTE, [utf8(1, routeId)], { routes });
  }

  /** Read a declared route from another tap of a track or submix in the working graph. */
  setRouteSource(routeId: string, source: RouteSource): TransactionBuilder {
    const path = `setRouteSource("${String(routeId)}")`;
    const route = this.#route(routeId, path);
    const normalized = normalizeRouteSourceSpec(source, this.#graph(), `${path}.source`);
    const routes = new Map(this.#state.routes);
    routes.set(routeId, freeze({ ...route, source: normalized }));
    return this.#next(
      { kind: "setRouteSource", routeId, source: { ...source } },
      OP_SET_ROUTE_SOURCE,
      [utf8(1, routeId), message(2, routeSourceFields(normalized))],
      { routes },
    );
  }

  /**
   * Send a declared route to another submix or output in the working graph. A route that follows
   * its source's mute cannot reach the output (#1218 D5): clear `followsMute` with `upsertRoute`.
   */
  setRouteDestination(routeId: string, destination: RouteDestination): TransactionBuilder {
    const path = `setRouteDestination("${String(routeId)}")`;
    const route = this.#route(routeId, path);
    const normalized = normalizeRouteDestinationSpec(destination, this.#graph(), `${path}.destination`);
    if (route.follows_mute === true && normalized.kind !== "submix_input") {
      refuseOutputFollowsMute(`${path}.destination`);
    }
    const routes = new Map(this.#state.routes);
    routes.set(routeId, freeze({ ...route, destination: normalized }));
    return this.#next(
      { kind: "setRouteDestination", routeId, destination: { ...destination } },
      OP_SET_ROUTE_DESTINATION,
      [utf8(1, routeId), message(2, routeDestinationFields(normalized))],
      { routes },
    );
  }

  /** The transaction, in edit order. A transaction with no edit is refused. */
  build(): SessionTransaction {
    const { edits } = this.#state;
    if (edits.length === 0) {
      throw new MisoUsageError("a session transaction needs at least one edit");
    }
    return Object.freeze({
      edits: Object.freeze(edits.map((edit) => edit.spec)),
      payload: concat(edits.map((edit) => edit.bytes)),
      fieldCount: edits.length,
    });
  }

  #declaredSource(sourceId: string, path: string): void {
    if (typeof sourceId !== "string" || !this.#state.sources.has(sourceId)) {
      fail(`${path}.id`, `'${String(sourceId)}' is not a declared source`, CODE.missingEntity);
    }
  }

  #declaredTrack(trackId: string, path: string): void {
    if (typeof trackId !== "string" || !this.#state.tracks.has(trackId)) {
      fail(`${path}.id`, `'${String(trackId)}' is not a declared track`, CODE.missingEntity);
    }
  }

  /**
   * Declare a stored automation, or replace the automation with this ID, with the session
   * builder's rules: the target resolved against the working session, at least one span, the
   * spans written in the target row's unit.
   */
  upsertAutomation(spec: AutomationSpec): TransactionBuilder {
    stableId(spec?.id, "upsertAutomation().id");
    const record = normalizeAutomation(spec, this.#automationScope(), "upsertAutomation");
    const automations = new Map(this.#state.automations);
    automations.set(spec.id, record);
    return this.#next(
      {
        kind: "upsertAutomation",
        automation: {
          ...spec,
          segments: spec.segments.map((segment) => ({ ...segment })),
          ...(spec.offset === undefined ? {} : { offset: { ...spec.offset } }),
        },
      },
      OP_UPSERT_AUTOMATION,
      [message(1, automationFields(record))],
      { automations },
    );
  }

  /** Remove a declared stored automation. */
  removeAutomation(automationId: string): TransactionBuilder {
    this.#automation(automationId, `removeAutomation("${String(automationId)}")`);
    const automations = new Map(this.#state.automations);
    automations.delete(automationId);
    return this.#next(
      { kind: "removeAutomation", automationId },
      OP_REMOVE_AUTOMATION,
      [utf8(1, automationId)],
      { automations },
    );
  }

  /**
   * Point a declared automation at another control, resolved as `upsertAutomation` resolves it.
   * Its spans keep their unit, so the new row must be in the same unit (`control.domain`
   * otherwise, as the engine refuses a span in another unit); replace it with `upsertAutomation`.
   */
  setAutomationTarget(automationId: string, target: AutomationTarget): TransactionBuilder {
    const path = `setAutomationTarget("${String(automationId)}")`;
    const current = this.#automation(automationId, path);
    const resolved = resolveAutomationTarget(target, this.#automationScope(), `${path}.target`);
    const unit = String(records(current.segments)[0]?.unit);
    if (resolved.unit !== unit) {
      fail(
        `${path}.target.control`,
        `the row is in ${resolved.unit} and this automation's spans are in ${unit}; replace it with upsertAutomation`,
        "control.domain",
      );
    }
    const targetRecord = automationTargetRecord(resolved);
    const automations = new Map(this.#state.automations);
    automations.set(automationId, freeze({ ...current, target: targetRecord }));
    return this.#next(
      { kind: "setAutomationTarget", automationId, target: { ...target } as AutomationTarget },
      OP_SET_AUTOMATION_TARGET,
      [utf8(1, automationId), message(2, automationTargetFields(targetRecord))],
      { automations },
    );
  }

  /**
   * Replace a declared automation's spans, validated as `SessionBuilder.automation`'s and written
   * in its target row's unit. An automation keeps at least one span: remove it instead.
   */
  setAutomationSegments(automationId: string, segments: readonly AutomationSegment[]): TransactionBuilder {
    const path = `setAutomationSegments("${String(automationId)}")`;
    const current = this.#automation(automationId, path);
    requireSegments(segments, `${path}.segments`);
    const unit = String(records(current.segments)[0]?.unit);
    const normalized = normalizeAutomationSegments(segments, unit, `${path}.segments`);
    const automations = new Map(this.#state.automations);
    automations.set(automationId, freeze({ ...current, segments: normalized }));
    return this.#next(
      { kind: "setAutomationSegments", automationId, segments: segments.map((segment) => ({ ...segment })) },
      OP_SET_AUTOMATION_SEGMENTS,
      [utf8(1, automationId), ...normalized.map((segment) => message(2, automationSegmentFields(segment)))],
      { automations },
    );
  }

  /**
   * Declare a VCA group, or replace the VCA with this ID, with the session builder's rules: each
   * member a track, submix or other VCA of the working graph, listed once; the offset fader in the
   * `fader_db` domain, 0 dB unmuted by default.
   */
  upsertVca(vcaId: string, spec: VcaSpec): TransactionBuilder {
    stableId(vcaId, "upsertVca().id");
    const path = `upsertVca("${vcaId}")`;
    this.#freeGraphId(vcaId, "vcas", path);
    const candidates = new Set([...this.#state.tracks, ...this.#state.submixes, ...this.#state.vcas]);
    candidates.delete(vcaId);
    const { fader, members } = normalizeVca(spec, candidates, path);
    const vcas = new Set(this.#state.vcas);
    vcas.add(vcaId);
    // The model's member order (`SessionBuilder.toJSON`): stable IDs are ASCII, so code-unit order.
    const sorted = [...members].sort();
    return this.#next(
      { kind: "upsertVca", vcaId, vca: { ...spec, members: [...spec.members] } },
      OP_UPSERT_VCA,
      [message(1, [utf8(1, vcaId), message(2, faderFields(fader)), ...sorted.map((member) => utf8(3, member))])],
      { vcas },
    );
  }

  /** Remove a declared VCA. A VCA that still lists it, or automation on it, makes the engine refuse. */
  removeVca(vcaId: string): TransactionBuilder {
    this.#declared(this.#state.vcas, vcaId, "VCA", `removeVca("${String(vcaId)}")`);
    const vcas = new Set(this.#state.vcas);
    vcas.delete(vcaId);
    return this.#next({ kind: "removeVca", vcaId }, OP_REMOVE_VCA, [utf8(1, vcaId)], { vcas });
  }

  /** Refuse `id` when another kind of graph entity than `own` already holds it. */
  #freeGraphId(id: string, own: "tracks" | "submixes" | "vcas" | "outputs", path: string): void {
    const kinds = ["tracks", "submixes", "vcas", "outputs"] as const;
    if (kinds.some((kind) => kind !== own && this.#state[kind].has(id))) {
      fail(`${path}.id`, "tracks, submixes, VCAs and outputs share one ID namespace", CODE.duplicateId);
    }
  }

  #declared(ids: ReadonlySet<string>, id: string, what: string, path: string): void {
    if (typeof id !== "string" || !ids.has(id)) {
      fail(`${path}.id`, `'${String(id)}' is not a declared ${what}`, CODE.missingEntity);
    }
  }

  #route(routeId: string, path: string): ModelRecord {
    this.#declared(new Set(this.#state.routes.keys()), routeId, "route", path);
    return this.#state.routes.get(routeId) as ModelRecord;
  }

  #automation(automationId: string, path: string): ModelRecord {
    this.#declared(new Set(this.#state.automations.keys()), automationId, "automation", path);
    return this.#state.automations.get(automationId) as ModelRecord;
  }

  /** The working session an automation target resolves against. */
  #automationScope(): AutomationScope {
    return {
      slots: this.#state.slots,
      strips: new Map([...this.#state.strips].map(([id, inserts]) => [
        id,
        inserts.map((insert) => [String(insert.id), String(record(insert.identity).effect_id)] as const),
      ] as const)),
      routes: new Set(this.#state.routes.keys()),
      vcas: this.#state.vcas,
    };
  }

  #graph(): RouteGraph {
    return { tracks: this.#state.tracks, submixes: this.#state.submixes, outputs: this.#state.outputs };
  }

  /** A declared strip's working inserts: a track's or a submix's. */
  #strip(trackId: string, path: string): readonly ModelRecord[] {
    const inserts = typeof trackId === "string" ? this.#state.strips.get(trackId) : undefined;
    if (inserts === undefined) {
      fail(`${path}.id`, `'${String(trackId)}' is not a declared track or submix`, CODE.missingEntity);
    }
    return inserts;
  }

  #insert(inserts: readonly ModelRecord[], slotId: string, path: string): ModelRecord {
    const found = typeof slotId === "string" ? inserts.find((entry) => entry.id === slotId) : undefined;
    if (found === undefined) {
      fail(`${path}.slotId`, `'${String(slotId)}' is not an insert of this strip`, "session.edit.not_found");
    }
    return found;
  }

  #withInserts(trackId: string, inserts: readonly ModelRecord[]): ReadonlyMap<string, readonly ModelRecord[]> {
    const strips = new Map(this.#state.strips);
    strips.set(trackId, Object.freeze([...inserts]));
    return strips;
  }

  /** `SetEffectIdentity`, `SetEffectQuality`, `SetEffectSidechain`: the prefix, then `value`. */
  #effectEdit(
    spec: Extract<SessionEditSpec, { readonly slotId: string }>,
    opcode: number,
    current: readonly ModelRecord[],
    insert: ModelRecord,
    value: Uint8Array,
  ): TransactionBuilder {
    return this.#next(
      spec,
      opcode,
      [utf8(1, spec.trackId), byte(2, RACK_INSERTS), utf8(3, spec.slotId), value],
      { strips: this.#withInserts(spec.trackId, current.map((entry) => (entry.id === insert.id ? insert : entry))) },
    );
  }

  #next(
    spec: SessionEditSpec,
    opcode: number,
    payload: readonly Uint8Array[],
    update: Partial<Omit<BuilderState, "baseSlots" | "edits">>,
  ): TransactionBuilder {
    const bytes = nested(1, true, [field(1, WIRE_U16, true, u16(opcode)), nested(2, true, payload)]);
    return new TransactionBuilder(Object.freeze({
      ...this.#state,
      ...update,
      edits: Object.freeze([...this.#state.edits, Object.freeze({ spec: freeze(spec), bytes })]),
    }));
  }
}

/**
 * Start a transaction of structural edits to the session `base` declares.
 *
 * `base` is what each edit is validated against: its rate, sources, console slots and tracks. It
 * must be the session the engine holds at the revision the transaction is sent against, or the
 * engine judges the edits against another session than the builder did.
 */
export function transaction(base: SessionBuilder): TransactionBuilder {
  if (!(base instanceof SessionBuilder)) {
    throw new MisoUsageError("transaction() takes the SessionBuilder of the session it edits");
  }
  const context = builderContext(base);
  const model = base.toJSON();
  const strips = new Map<string, readonly ModelRecord[]>(
    [...model.tracks, ...model.submixes].map((strip) => [
      String(strip.id),
      records(record(strip.inserts).effects),
    ] as const),
  );
  const ids = (entities: readonly ModelRecord[]): ReadonlySet<string> =>
    new Set(entities.map((entity) => String(entity.id)));
  return new TransactionBuilder(Object.freeze({
    sampleRateHz: context.sampleRateHz,
    baseSlots: context.slots,
    slots: context.slots,
    sources: new Map(context.sources.map((entry) => [entry.id, entry] as const)),
    tracks: new Set(context.tracks.map((entry) => entry.id)),
    submixes: ids(model.submixes),
    vcas: ids(model.vcas),
    outputs: ids(model.outputs),
    routes: new Map(model.routes.map((route) => [String(route.id), route] as const)),
    automations: new Map(model.automation.map((entry) => [String(entry.id), entry] as const)),
    strips,
    consoleChanged: false,
    edits: Object.freeze([]),
  }));
}

/**
 * Encode `tx` as one `SESSION_TRANSACTION_APPLY` command frame with the endpoint's request ID and
 * the revision it applies to. Throws `MisoUsageError` for a zero or out-of-range request ID or
 * revision.
 * @internal
 */
export function encodeSessionTransaction(
  tx: SessionTransaction,
  request: SessionTransactionRequest,
): Uint8Array {
  const requestId = requireU64(request.requestId, "requestId");
  if (requestId === 0n) throw new MisoUsageError("requestId must not be zero");
  const expectedRevision = requireU64(request.expectedRevision, "expectedRevision");
  if (tx.fieldCount === 0 || tx.fieldCount !== tx.edits.length) {
    throw new MisoUsageError("a session transaction comes from TransactionBuilder.build()");
  }
  return commandFrame(MESSAGE_SESSION_TRANSACTION_APPLY, 0, requestId, expectedRevision, tx.fieldCount, tx.payload);
}

// -------------------------------------------------------------------------------------------
// Entity encoders: one per `session_wire.rs` `tx_*` function, from the normalized record.
// -------------------------------------------------------------------------------------------

/** The `{ rampSamples? }` option of `setConsole` and `setTrackConsole`: a `u32`, or absent. */
function rampOption(options: { readonly rampSamples?: number }, path: string): number | undefined {
  if (options === null || typeof options !== "object") fail(`${path}.options`, "expected { rampSamples? }");
  return options.rampSamples === undefined ? undefined : requireU32(options.rampSamples, `${path}.rampSamples`);
}

/** The optional `ramp_samples` field (#1394 D1): written only when a length is given. */
function rampField(id: number, rampSamples: number | undefined): Uint8Array[] {
  return rampSamples === undefined ? [] : [field(id, WIRE_U32, false, u32(rampSamples))];
}

/** Whether two slot lists are the same console, as the engine compares `Console` values. */
function sameSlots(a: readonly ConsoleSlotEntry[], b: readonly ConsoleSlotEntry[]): boolean {
  return a.length === b.length && a.every((slot, index) => {
    const other = b[index] as ConsoleSlotEntry;
    return slot.slot === other.slot && slot.section === other.section && slot.effectId === other.effectId
      && slot.quality === other.quality && slot.linkMode === other.linkMode;
  });
}

function utf8(id: number, value: ModelValue | undefined): Uint8Array {
  return field(id, WIRE_UTF8, true, text(String(value)));
}

function byte(id: number, value: number): Uint8Array {
  return field(id, WIRE_U8, true, u8(value));
}

function code(id: number, table: ReadonlyMap<ModelValue, number>, value: ModelValue | undefined): Uint8Array {
  return byte(id, codeOf(table, value));
}

function codeOf(table: ReadonlyMap<ModelValue, number>, value: ModelValue | undefined): number {
  const found = value === undefined ? undefined : table.get(value);
  if (found === undefined) throw new MisoUsageError(`${String(value)} has no wire code`);
  return found;
}

function boolean(id: number, value: ModelValue | undefined): Uint8Array {
  return field(id, WIRE_BOOL, true, u8(value === true ? 1 : 0));
}

function float(id: number, value: ModelValue | undefined): Uint8Array {
  return field(id, WIRE_F32, true, f32(Number(value)));
}

function word(id: number, value: ModelValue | undefined): Uint8Array {
  return field(id, WIRE_U32, true, u32(Number(value)));
}

function long(id: number, value: ModelValue | undefined): Uint8Array {
  const out = new Uint8Array(8);
  new DataView(out.buffer).setBigUint64(0, BigInt(String(value)), true);
  return field(id, WIRE_U64, true, out);
}

function message(id: number, fields: readonly Uint8Array[]): Uint8Array {
  return nested(id, true, fields);
}

function record(value: ModelValue | undefined): ModelRecord {
  return value as ModelRecord;
}

function records(value: ModelValue | undefined): readonly ModelRecord[] {
  return value as readonly ModelRecord[];
}

/** `tx_source`, and `SetSourceContent`'s payload. */
function sourceFields(source: ModelRecord): Uint8Array[] {
  return [
    utf8(1, source.id),
    utf8(2, source.content),
    byte(3, Number(source.channels)),
    code(4, BIT_DEPTH_CODES, source.bit_depth),
    long(5, source.frames),
  ];
}

/** `tx_track`: fields 6 and 8 are retired (decision 12). */
function trackFields(track: ModelRecord): Uint8Array[] {
  return [
    utf8(1, track.id),
    utf8(2, track.source_id),
    byte(3, Number(track.left_source_channel)),
    byte(4, Number(track.right_source_channel)),
    message(5, builtinsFields(record(track.builtins))),
    message(7, records(record(track.inserts).effects).map((effect) => message(1, effectFields(effect)))),
    message(9, faderFields(record(track.fader))),
    message(10, matrixOrPanFields(track)),
    ...records(track.console).map((entry) => message(11, consoleEntryFields(entry))),
  ];
}

function builtinsFields(builtins: ModelRecord): Uint8Array[] {
  const lane = (value: ModelRecord): Uint8Array[] => [
    boolean(1, value.polarity_invert),
    float(2, value.trim_db),
    float(3, value.hpf_hz),
    float(4, value.lpf_hz),
    word(5, value.delay_samples),
  ];
  return [message(1, lane(record(builtins.left))), message(2, lane(record(builtins.right)))];
}

function effectFields(effect: ModelRecord): Uint8Array[] {
  return [
    utf8(1, effect.id),
    message(2, identityFields(effect)),
    code(3, QUALITY_CODES, effect.quality),
    boolean(4, effect.bypass),
    code(5, LINK_MODE_CODES, effect.link_mode),
    ...records(effect.params).map((param) => message(6, paramFields(param))),
    message(7, sidechainFields(record(effect.sidechain))),
  ];
}

/** `tx_identity`: a launch insert is always a native effect. */
function identityFields(effect: ModelRecord): Uint8Array[] {
  return [byte(1, IDENTITY_NATIVE), utf8(2, record(effect.identity).effect_id)];
}

function paramFields(param: ModelRecord): Uint8Array[] {
  return [
    word(1, param.parameter_id),
    code(2, CHANNEL_CODES, param.channel),
    code(3, UNIT_CODES, param.unit),
    float(4, param.value),
  ];
}

function sidechainFields(sidechain: ModelRecord): Uint8Array[] {
  if (sidechain.kind !== "routed") return [code(1, SIDECHAIN_TAGS, sidechain.kind)];
  return [
    code(1, SIDECHAIN_TAGS, sidechain.kind),
    message(2, routeSourceFields(record(sidechain.source))),
    utf8(3, sidechain.port_id),
  ];
}

function routeSourceFields(source: ModelRecord): Uint8Array[] {
  return [
    code(1, ROUTE_SOURCE_TAGS, source.kind),
    utf8(2, source.kind === "track" ? source.track_id : source.submix_id),
    code(3, TAP_CODES, source.tap),
  ];
}

function faderFields(fader: ModelRecord): Uint8Array[] {
  return [
    float(1, fader.left_db),
    float(2, fader.right_db),
    boolean(3, fader.left_mute),
    boolean(4, fader.right_mute),
  ];
}

/** `tx_matrix_or_pan`: the tagged field 10, `pan` or `matrix`, whichever key the strip carries. */
function matrixOrPanFields(strip: ModelRecord): Uint8Array[] {
  if (strip.matrix !== undefined) {
    const matrix = record(strip.matrix);
    return [
      byte(1, MATRIX_TAG),
      float(2, matrix.ll),
      float(3, matrix.lr),
      float(4, matrix.rl),
      float(5, matrix.rr),
      word(6, matrix.smoothing_samples),
    ];
  }
  const pan = record(strip.pan);
  return [byte(1, PAN_TAG), float(2, pan.left), float(3, pan.right), word(4, pan.smoothing_samples)];
}

/** `tx_console`: the pre-insert slots as field 1, then the post-insert slots as field 2. */
function consoleFields(slots: readonly ConsoleSlotEntry[]): Uint8Array[] {
  const section = (id: number, name: ConsoleSlotEntry["section"]): Uint8Array[] => slots
    .filter((slot) => slot.section === name)
    .map((slot) => message(id, [
      utf8(1, slot.slot),
      message(2, [byte(1, IDENTITY_NATIVE), utf8(2, slot.effectId)]),
      code(3, QUALITY_CODES, slot.quality),
      code(4, LINK_MODE_CODES, slot.linkMode),
    ]));
  return [...section(1, "pre_insert"), ...section(2, "post_insert")];
}

/** `tx_submix`: the track's strip fields without the source, `console` before `inserts`. */
function submixFields(submix: ModelRecord): Uint8Array[] {
  return [
    utf8(1, submix.id),
    message(2, builtinsFields(record(submix.builtins))),
    ...records(submix.console).map((entry) => message(3, consoleEntryFields(entry))),
    message(4, records(record(submix.inserts).effects).map((effect) => message(1, effectFields(effect)))),
    message(5, faderFields(record(submix.fader))),
    message(6, matrixOrPanFields(submix)),
  ];
}

/** `tx_route`. */
function routeFields(route: ModelRecord): Uint8Array[] {
  const matrix = record(route.channel_matrix);
  return [
    utf8(1, route.id),
    message(2, routeSourceFields(record(route.source))),
    message(3, routeDestinationFields(record(route.destination))),
    message(4, [float(1, matrix.ll), float(2, matrix.lr), float(3, matrix.rl), float(4, matrix.rr)]),
    float(5, route.gain_db),
    boolean(6, route.mute),
    boolean(7, route.follows_mute),
  ];
}

/** `tx_automation`: the segments repeat as field 3, and the hand-move offset is field 4. */
function automationFields(automation: ModelRecord): Uint8Array[] {
  const offset = record(automation.offset);
  return [
    utf8(1, automation.id),
    message(2, automationTargetFields(record(automation.target))),
    ...records(automation.segments).map((segment) => message(3, automationSegmentFields(segment))),
    message(4, [float(1, offset.left_db), float(2, offset.right_db)]),
  ];
}

/**
 * `tx_automation_target`: the control list's address message (#1566 D4), its ID, slot, control
 * and lane fields written optional as the list writes them, and no expected effect.
 */
function automationTargetFields(target: ModelRecord): Uint8Array[] {
  return [
    code(1, OWNER_KIND_CODES, target.owner),
    field(2, WIRE_UTF8, false, text(String(target.owner_id))),
    code(3, SECTION_KIND_CODES, target.section),
    ...(target.slot_id === undefined ? [] : [field(4, WIRE_UTF8, false, text(String(target.slot_id)))]),
    field(5, WIRE_U32, false, u32(Number(target.control))),
    field(6, WIRE_U8, false, u8(codeOf(LANE_CODES, target.lane))),
  ];
}

function automationSegmentFields(segment: ModelRecord): Uint8Array[] {
  return [
    code(1, SHAPE_CODES, segment.shape),
    long(2, segment.start_sample),
    long(3, segment.end_sample),
    float(4, segment.start_value),
    float(5, segment.end_value),
    code(6, UNIT_CODES, segment.unit),
  ];
}

function routeDestinationFields(destination: ModelRecord): Uint8Array[] {
  return [
    code(1, ROUTE_DESTINATION_TAGS, destination.kind),
    utf8(2, destination.kind === "submix_input" ? destination.submix_id : destination.output_id),
  ];
}

function consoleEntryFields(entry: ModelRecord): Uint8Array[] {
  return [
    utf8(1, entry.slot),
    boolean(2, entry.bypass),
    ...records(entry.params).map((param) => message(3, paramFields(param))),
    code(4, CONSOLE_LINK_MODE_CODES, entry.link_mode),
  ];
}
