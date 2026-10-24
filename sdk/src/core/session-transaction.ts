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
  normalizeConsoleEntries,
  normalizeInsert,
  normalizeInserts,
  normalizeSidechain,
  normalizeSource,
  normalizeTrack,
  quality,
  resolveLanes,
  retargetInsert,
  stableId,
  trackSourceRef,
} from "./session.ts";
import type { ConsoleSlotEntry, ModelRecord, ModelValue, SourceEntry } from "./session.ts";
import { CODE, fail, freeze } from "../internal/validation.ts";
import type { EffectId } from "../generated/catalog.ts";
import type {
  ConsoleEntrySpec,
  EffectDecl,
  SidechainSpec,
  SourceSpec,
  TrackSourceSpec,
  TrackSpec,
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
 * `setTrackConsole` gives a strip its console entries after the console's slot set changed. The
 * engine accepts it only after an earlier `SetConsole` of the same transaction changed the console
 * (`session.edit.value_op_retired`, #1561 D3); a console knob's value is a `SET_CONTROL`. This
 * builder has no `setConsole` yet (#1385), so `setTrackConsole` is refused here, after its
 * arguments are validated, until a console-changing edit precedes it. It is the one method that
 * takes `{ rampSamples }`, written as the optional `ramp_samples` field only when given.
 *
 * `sdk/test/transaction-evals.mjs` compares these bytes to the codec's re-encoding of them and
 * the store's snapshot after them to a `SessionBuilder` declaring the result
 * (`hosts/host-web/examples/sdk_transaction_oracle.rs`).
 */

// `SessionEditOpcode` (`crates/protocol/src/model.rs`).
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
const SIDECHAIN_TAGS = codes(["none", "routed"]);
const IDENTITY_NATIVE = 1;
const PAN_TAG = 1;
const MATRIX_TAG = 2;

function codes(tokens: readonly string[]): ReadonlyMap<ModelValue, number> {
  return new Map<ModelValue, number>(tokens.map((token, index) => [token, index + 1]));
}

/** One structural edit of a session transaction, as the caller gave it. */
export type SessionEditSpec =
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
  };


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
  readonly sampleRateHz: number;
  readonly slots: readonly ConsoleSlotEntry[];
  readonly otherGraphIds: ReadonlySet<string>;
  /** The sources after the edits so far, by ID. */
  readonly sources: ReadonlyMap<string, SourceEntry>;
  /** The track IDs after the edits so far. */
  readonly tracks: ReadonlySet<string>;
  /** Each strip's (track's or submix's) insert records after the edits so far, by strip ID. */
  readonly strips: ReadonlyMap<string, readonly ModelRecord[]>;
  /**
   * Whether an earlier edit changed the console's slot set, which opens `setTrackConsole`.
   * Always false until the builder can edit the console (#1385).
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
    if (this.#state.otherGraphIds.has(trackId)) {
      fail(
        `upsertTrack("${trackId}").id`,
        "tracks, submixes, VCAs and outputs share one ID namespace",
        CODE.duplicateId,
      );
    }
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
    if (options === null || typeof options !== "object") fail(`${path}.options`, "expected { rampSamples? }");
    const rampSamples = options.rampSamples === undefined
      ? undefined
      : requireU32(options.rampSamples, `${path}.rampSamples`);
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
        ...(rampSamples === undefined ? [] : [field(3, WIRE_U32, false, u32(rampSamples))]),
      ],
      {},
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

  /** A declared strip's working inserts: a track's, or a base submix's. */
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
    update: Partial<Pick<BuilderState, "sources" | "tracks" | "strips">>,
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
  return new TransactionBuilder(Object.freeze({
    sampleRateHz: context.sampleRateHz,
    slots: context.slots,
    otherGraphIds: context.otherGraphIds,
    sources: new Map(context.sources.map((entry) => [entry.id, entry] as const)),
    tracks: new Set(context.tracks.map((entry) => entry.id)),
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

function utf8(id: number, value: ModelValue | undefined): Uint8Array {
  return field(id, WIRE_UTF8, true, text(String(value)));
}

function byte(id: number, value: number): Uint8Array {
  return field(id, WIRE_U8, true, u8(value));
}

function code(id: number, table: ReadonlyMap<ModelValue, number>, value: ModelValue | undefined): Uint8Array {
  const found = value === undefined ? undefined : table.get(value);
  if (found === undefined) throw new MisoUsageError(`${String(value)} has no wire code`);
  return byte(id, found);
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

function consoleEntryFields(entry: ModelRecord): Uint8Array[] {
  return [
    utf8(1, entry.slot),
    boolean(2, entry.bypass),
    ...records(entry.params).map((param) => message(3, paramFields(param))),
  ];
}
