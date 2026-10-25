import { ABI_LAYOUT } from "../generated/abi.ts";
import { constantValue } from "./abi.ts";
import { MisoEngineError, MisoUsageError } from "./errors.ts";
import type { ResponsePreviewGrid } from "./response.ts";
import {
  decodeResponseReady,
  decodeResponseTagReply,
  encodeStripResponseRequest,
  responseRefusalResult,
} from "./response-frame.ts";
import type { ResponseReadyEvent } from "./response-frame.ts";
import type { Channel } from "./types.ts";

/** A bounded grid accepted by a live track-response query. */
export type TrackResponseGrid = ResponsePreviewGrid;

/** Bounds this live endpoint actually enforces. */
export interface TrackResponseLimits {
  readonly maximumResultBytes?: number;
  readonly requestDeadlineMs?: number;
}

/** One on-demand live track response request. */
export interface TrackResponseQuery {
  readonly trackId: string;
  readonly grid: TrackResponseGrid;
  readonly channels?: Channel;
  readonly responseLimits?: TrackResponseLimits;
}

/** One response-capable owner in the selected track's declared signal order. */
export interface TrackResponseMember {
  readonly trackId: string;
  readonly nativeId: string;
  readonly stableId: string;
  /**
   * `input` for the input section's filters, `console` for a session console slot in either
   * section, or `inserts` for one of the track's inserts.
   */
  readonly rack: "input" | "console" | "inserts";
  /**
   * The member's section code in the `RESPONSE_READY` event (decision 17): `4` the input
   * section's filters, `5` a console slot, `2` an insert.
   */
  readonly rackValue: number;
  /** Position in the actual declared response-owner order. */
  readonly slot: number;
  /** `parametricEq`, `inputFilters`, or `unavailable`. */
  readonly kind: "parametricEq" | "inputFilters" | "unavailable";
  readonly kindValue: number;
  readonly bypassed: boolean;
  readonly available: boolean;
  /** A declared owner without a provider remains in place with this reason. */
  readonly excludedReason?: "linearResponseUnavailable";
  readonly enabledLeft: readonly boolean[];
  readonly enabledRight: readonly boolean[];
}

/** The owned result of one live target response query. */
export interface TrackResponseResult {
  readonly trackId: string;
  readonly mode: "target";
  readonly meaning: "eqFilterSubtotal";
  readonly sampleRateHz: number;
  readonly floorDb: number;
  readonly frequenciesHz: Float32Array;
  /** The selected left subtotal, omitted when `channels` excludes left. */
  readonly leftDb?: Float32Array;
  /** The selected right subtotal, omitted when `channels` excludes right. */
  readonly rightDb?: Float32Array;
  readonly members: readonly TrackResponseMember[];
  readonly capturedSample: bigint;
  readonly snapshotToken: bigint;
  readonly excludedMemberCount: number;
  readonly resultBytes: bigint;
}

function generatedMaximum(name: string): number {
  const value = (ABI_LAYOUT.constants as Readonly<Record<string, unknown>>)[name];
  if (typeof value !== "number" || !Number.isSafeInteger(value) || value <= 0) {
    throw new MisoUsageError(`the generated ABI has no valid ${name}`);
  }
  return value;
}

function finiteInteger(value: number, name: string, minimum: number, maximum: number): number {
  if (!Number.isSafeInteger(value) || value < minimum || value > maximum) {
    throw new MisoUsageError(`${name} must be an integer in ${minimum}..=${maximum}`);
  }
  return value;
}

function channelValue(value: Channel | undefined): number {
  if (value === undefined || value === "both") return 3;
  if (value === "left") return 1;
  if (value === "right") return 2;
  throw new MisoUsageError("channels must be left, right, or both");
}

function gridValue(grid: TrackResponseGrid, maximumPoints: number): { readonly kind: number; readonly points: number } {
  if (grid.kind !== "linear" && grid.kind !== "logarithmic") {
    throw new MisoUsageError("grid.kind must be linear or logarithmic");
  }
  const points = finiteInteger(grid.points, "grid.points", 2, maximumPoints);
  if (!Number.isFinite(grid.minimumHz) || !Number.isFinite(grid.maximumHz)) {
    throw new MisoUsageError("grid endpoints must be finite");
  }
  if (grid.kind === "logarithmic" && grid.minimumHz <= 0) {
    throw new MisoUsageError("logarithmic grid minimumHz must be positive");
  }
  return { kind: grid.kind === "linear" ? 1 : 2, points };
}

/**
 * The largest `RESPONSE_READY` frame a query may ask for, in bytes: the control frame limit
 * (#1579a D6). `responseLimits.maximumResultBytes` is sent as the request's `maximum_reply_bytes`.
 */
export const TRACK_RESPONSE_MAXIMUM_RESULT_BYTES = 1 << 20;

/** The longest strip ID a `RESPONSE_READY` carries, in bytes (#1578 D2). */
const TRACK_RESPONSE_MAXIMUM_ID_BYTES = 127;

/** Clone the endpoint query into the effective bounded form used by managed jobs. */
export function normalizeTrackResponseQuery(request: TrackResponseQuery): TrackResponseQuery {
  if (request === null || typeof request !== "object") {
    throw new MisoUsageError("live track response query must be an object");
  }
  const maximumPoints = generatedMaximum("maximumResponsePoints");
  const maximumCaptureBytes = TRACK_RESPONSE_MAXIMUM_RESULT_BYTES;
  const id = trackIdBytes(request.trackId, TRACK_RESPONSE_MAXIMUM_ID_BYTES);
  const grid = gridValue(request.grid, maximumPoints);
  const channels = channelValue(request.channels) === 1
    ? "left" as const
    : channelValue(request.channels) === 2 ? "right" as const : "both" as const;
  const maximum = maximumResultBytes(request.responseLimits, maximumCaptureBytes);
  const deadline = request.responseLimits?.requestDeadlineMs;
  if (deadline !== undefined && (!Number.isFinite(deadline) || deadline <= 0 || deadline > 2_147_483_647)) {
    throw new MisoUsageError("requestDeadlineMs must be positive and at most 2147483647");
  }
  return Object.freeze({
    trackId: new TextDecoder().decode(id),
    grid: Object.freeze({
      kind: request.grid.kind,
      points: grid.points,
      minimumHz: request.grid.minimumHz,
      maximumHz: request.grid.maximumHz,
    }),
    channels,
    responseLimits: Object.freeze({
      maximumResultBytes: maximum,
      ...(deadline === undefined ? {} : { requestDeadlineMs: deadline }),
    }),
  });
}

function maximumResultBytes(responseLimits: TrackResponseLimits | undefined, maximumCaptureBytes: number): number {
  return finiteInteger(
    responseLimits?.maximumResultBytes ?? maximumCaptureBytes,
    "maximumResultBytes",
    1,
    maximumCaptureBytes,
  );
}

function trackIdBytes(trackId: string, maximumIdBytes: number): Uint8Array<ArrayBuffer> {
  if (typeof trackId !== "string" || trackId.length === 0) {
    throw new MisoUsageError("trackId must be a nonempty string");
  }
  const bytes = new TextEncoder().encode(trackId);
  if (bytes.length === 0 || bytes.length > maximumIdBytes) {
    throw new MisoUsageError("trackId exceeds the live response identity bound");
  }
  return bytes;
}

function invalidPayload(message: string): MisoEngineError {
  return new MisoEngineError(message, { phase: "output", code: "abiMismatch", result: 2 });
}


// ----------------------------------------------------------------------------------------------
// The one request path (#1579a D1): the frames a C ABI host sends and reads.

/** One protocol event as an engine's event lane delivers it. */
export interface TrackResponseControlEvent {
  readonly lane: 0 | 1;
  readonly frame: Uint8Array;
}

/** How a track response query reaches one engine's core. */
export interface TrackResponseTransport {
  /** Send one command frame and resolve with its reply frame. */
  send(frame: Uint8Array): Uint8Array | Promise<Uint8Array>;
  /** Subscribe to the engine's protocol events; returns the unsubscribe function. */
  onControlEvent(listener: (event: TrackResponseControlEvent) => void): () => void;
  /** The engine endpoint's one request-ID allocator. */
  nextRequestId(): bigint;
}

const DEFAULT_REQUEST_DEADLINE_MS = 5_000;

const RACKS: ReadonlyMap<number, TrackResponseMember["rack"]> = new Map([
  [4, "input"],
  [5, "console"],
  [2, "inserts"],
]);
const KINDS: ReadonlyMap<number, TrackResponseMember["kind"]> = new Map([
  [1, "parametricEq"],
  [2, "inputFilters"],
  [3, "unavailable"],
]);

function maskBits(mask: number, count: number): readonly boolean[] {
  return Object.freeze(Array.from({ length: count }, (_unused, bit) => ((mask >>> bit) & 1) === 1));
}

/**
 * The SDK's result of one `RESPONSE_READY` (#1579a D3). A refused outcome raises `MisoEngineError`
 * with the core's diagnostic code.
 */
export function trackResponseResultOf(event: ResponseReadyEvent): TrackResponseResult {
  const capture = event.capture;
  if (capture === undefined) {
    const status = responseRefusalResult(event.status);
    throw new MisoEngineError("the live track response capture was refused", {
      phase: "output",
      code: (event.diagnostic ?? status) as MisoEngineError["code"],
      result: constantValue("resultCodes", status),
      diagnostics: [{ code: event.diagnostic ?? status, path: "" }],
    });
  }
  const members = capture.members.map((member, slot): TrackResponseMember => {
    const rack = RACKS.get(member.section);
    const kind = KINDS.get(member.kind);
    if (rack === undefined || kind === undefined) throw invalidPayload("a response member has an unknown code");
    return Object.freeze({
      trackId: event.stripId,
      nativeId: member.effect,
      stableId: member.slotId ?? "input-filters",
      rack,
      rackValue: member.section,
      slot,
      kind,
      kindValue: member.kind,
      bypassed: member.bypassed,
      available: member.kind !== 3,
      ...(member.kind === 3 ? { excludedReason: "linearResponseUnavailable" as const } : {}),
      enabledLeft: maskBits(member.enabledLeft, member.sectionCount),
      enabledRight: maskBits(member.enabledRight, member.sectionCount),
    });
  });
  return Object.freeze({
    trackId: event.stripId,
    mode: "target",
    meaning: "eqFilterSubtotal",
    sampleRateHz: capture.sampleRateHz,
    floorDb: capture.floorDb,
    frequenciesHz: capture.frequenciesHz,
    ...(capture.leftDb === undefined ? {} : { leftDb: capture.leftDb }),
    ...(capture.rightDb === undefined ? {} : { rightDb: capture.rightDb }),
    members: Object.freeze(members),
    capturedSample: capture.capturedSample,
    snapshotToken: event.requestTag,
    excludedMemberCount: capture.excludedCount,
    resultBytes: BigInt(event.frameBytes),
  });
}

function sameBits(left: Float32Array | undefined, right: Float32Array | undefined): boolean {
  if (left === undefined || right === undefined) return left === right;
  if (left.length !== right.length) return false;
  const a = new Uint32Array(left.buffer, left.byteOffset, left.length);
  const b = new Uint32Array(right.buffer, right.byteOffset, right.length);
  for (let index = 0; index < a.length; index += 1) {
    if (a[index] !== b[index]) return false;
  }
  return true;
}

function sameMask(left: readonly boolean[], right: readonly boolean[]): boolean {
  return left.length === right.length && left.every((enabled, index) => enabled === right[index]);
}

/**
 * Whether two results of one query say the same thing (#1579 D4): every member field and every
 * curve bit are equal. The boundary sample, the tag and the frame length are not compared, so a
 * managed subscription publishes a new result only when the response or its owners changed.
 */
export function sameTrackResponseResult(left: TrackResponseResult, right: TrackResponseResult): boolean {
  return left.trackId === right.trackId
    && left.sampleRateHz === right.sampleRateHz
    && sameBits(Float32Array.of(left.floorDb), Float32Array.of(right.floorDb))
    && left.excludedMemberCount === right.excludedMemberCount
    && sameBits(left.frequenciesHz, right.frequenciesHz)
    && sameBits(left.leftDb, right.leftDb)
    && sameBits(left.rightDb, right.rightDb)
    && left.members.length === right.members.length
    && left.members.every((member, index) => {
      const other = right.members[index]!;
      return member.trackId === other.trackId
        && member.nativeId === other.nativeId
        && member.stableId === other.stableId
        && member.rackValue === other.rackValue
        && member.slot === other.slot
        && member.kindValue === other.kindValue
        && member.bypassed === other.bypassed
        && sameMask(member.enabledLeft, other.enabledLeft)
        && sameMask(member.enabledRight, other.enabledRight);
    });
}

/**
 * One engine's track response queries through the core's frames (#1579a D1): each query sends a
 * strip `RESPONSE_GET` and waits for the `RESPONSE_READY` whose tag its reply carried, read from
 * the engine's event lane without consuming any other event. Queries on one engine run one at a
 * time, in call order, because the core captures one at a time; a query made while none is in
 * flight sends its frame before `query` returns, so a `render()` right after the call answers it.
 * The deadline (`responseLimits.requestDeadlineMs`, default 5 s) runs from the start of the
 * query's turn to the event. A query that timed out leaves its capture armed in the core: its tag
 * is kept, and the next query waits (under its own deadline) until that tag's event has been read
 * and dropped.
 *
 * The requester listens on the event lane only while a query runs or a timed-out tag is held.
 */
export class TrackResponseRequester {
  readonly #transport: TrackResponseTransport;
  #tail: Promise<unknown> = Promise.resolve();
  /** Queries called and not yet settled, the running one included. */
  #outstanding = 0;
  /** Set by `close`: the error every pending and later query rejects with. */
  #closed: Error | undefined;
  /** The rejection of the running query's deadline, while one runs. */
  #abort: ((error: Error) => void) | undefined;
  #unsubscribe: (() => void) | undefined;
  /**
   * The tag of a query that timed out, until its event is read; `released` settles then, or
   * rejects when an event that cannot be decoded arrives while the tag is held.
   */
  #orphan: {
    readonly tag: bigint;
    readonly released: Promise<void>;
    readonly release: () => void;
    readonly fail: (error: Error) => void;
  } | undefined;
  /**
   * Set while the reply to a query that timed out during its send is still outstanding: that reply
   * may carry a tag to hold, so the requester keeps listening and the next query waits for it.
   */
  #holdPending: Promise<void> | undefined;
  /** Events of the running query read before its tag was known. */
  #seen = new Map<bigint, ResponseReadyEvent>();
  /** The first `RESPONSE_READY` the running query could not decode. */
  #eventError: Error | undefined;
  #waiting: { readonly tag: bigint; readonly settle: (event: ResponseReadyEvent | Error) => void } | undefined;

  constructor(transport: TrackResponseTransport) {
    this.#transport = transport;
  }

  query(request: TrackResponseQuery): Promise<TrackResponseResult> {
    let normalized: TrackResponseQuery;
    try {
      normalized = normalizeTrackResponseQuery(request);
    } catch (error) {
      return Promise.reject(error);
    }
    const start = (): Promise<TrackResponseResult> => this.#run(normalized);
    const idle = this.#outstanding === 0;
    this.#outstanding += 1;
    // Idle: the frame goes out now, before this call returns. Otherwise the query takes its turn.
    const run = idle ? start() : this.#tail.then(start, start);
    this.#tail = run.catch(() => undefined);
    return run;
  }

  /**
   * End this requester: the query in flight and every queued one reject at once with `error`, and
   * so does every later query. The engine calls it when it is disposed.
   */
  close(error: Error): void {
    if (this.#closed !== undefined) return;
    this.#closed = error;
    this.#abort?.(error);
    this.#orphan?.release();
    this.#orphan = undefined;
    this.#holdPending = undefined;
    this.#unsubscribe?.();
    this.#unsubscribe = undefined;
  }

  #listen(): void {
    if (this.#unsubscribe !== undefined || this.#closed !== undefined) return;
    this.#unsubscribe = this.#transport.onControlEvent((event) => this.#onEvent(event));
  }

  #quiet(): void {
    if (this.#outstanding > 0 || this.#orphan !== undefined || this.#holdPending !== undefined) return;
    this.#unsubscribe?.();
    this.#unsubscribe = undefined;
  }

  #onEvent(event: TrackResponseControlEvent): void {
    if (event.lane !== 0) return;
    let ready: ResponseReadyEvent | undefined;
    try {
      ready = decodeResponseReady(event.frame);
    } catch (error) {
      // Only one capture is ever armed: an undecodable answer ends the held tag too, and the query
      // waiting for it fails with the error.
      const orphan = this.#orphan;
      if (orphan !== undefined) {
        this.#orphan = undefined;
        orphan.fail(error as Error);
      }
      this.#eventError ??= error as Error;
      this.#waiting?.settle(error as Error);
      this.#quiet();
      return;
    }
    if (ready === undefined) return;
    const orphan = this.#orphan;
    if (orphan !== undefined && ready.requestTag === orphan.tag) {
      // The answer to a query that timed out: dropped, and the capture slot is free again.
      this.#orphan = undefined;
      orphan.release();
      this.#quiet();
    } else if (this.#waiting !== undefined && ready.requestTag === this.#waiting.tag) {
      this.#waiting.settle(ready);
    } else {
      this.#seen.set(ready.requestTag, ready);
    }
  }

  #hold(tag: bigint): void {
    let release!: () => void;
    let fail!: (error: Error) => void;
    const released = new Promise<void>((resolve, reject) => {
      release = resolve;
      fail = reject;
    });
    released.catch(() => undefined);
    this.#orphan = { tag, released, release, fail };
  }

  async #run(request: TrackResponseQuery): Promise<TrackResponseResult> {
    let timer: ReturnType<typeof setTimeout> | undefined;
    let tag: bigint | undefined;
    let timedOut = false;
    try {
      if (this.#closed !== undefined) throw this.#closed;
      this.#seen = new Map();
      this.#eventError = undefined;
      this.#waiting = undefined;
      this.#listen();
      const deadlineMs = request.responseLimits?.requestDeadlineMs ?? DEFAULT_REQUEST_DEADLINE_MS;
      // The deadline, or the engine's disposal, whichever comes first.
      const deadline = new Promise<never>((_resolve, reject) => {
        this.#abort = reject;
        timer = setTimeout(() => {
          timedOut = true;
          reject(new MisoUsageError("live track response query exceeded its deadline"));
        }, deadlineMs);
      });
      deadline.catch(() => undefined);
      // A timed-out query's capture may still be armed in the core: wait for its late reply, then
      // for its answer, first.
      if (this.#holdPending !== undefined) await Promise.race([this.#holdPending, deadline]);
      if (this.#orphan !== undefined) await Promise.race([this.#orphan.released, deadline]);
      if (this.#closed !== undefined) throw this.#closed;
      const requestId = this.#transport.nextRequestId();
      const frame = encodeStripResponseRequest({
        requestId,
        stripId: request.trackId,
        grid: request.grid.kind,
        points: request.grid.points,
        minimumHz: request.grid.minimumHz,
        maximumHz: request.grid.maximumHz,
        channels: request.channels ?? "both",
        maximumReplyBytes: request.responseLimits?.maximumResultBytes,
      });
      const sent = Promise.resolve(this.#transport.send(frame));
      const replyBytes = await Promise.race([sent, deadline]).catch((error: unknown) => {
        // A reply that arrives after the deadline may still have armed a capture.
        if (timedOut) {
          const pending: Promise<void> = sent
            .then((bytes) => {
              const late = decodeResponseTagReply(bytes);
              if (late.ok && this.#closed === undefined) this.#hold(late.tag);
            })
            .catch(() => undefined)
            .finally(() => {
              if (this.#holdPending === pending) this.#holdPending = undefined;
              this.#quiet();
            });
          this.#holdPending = pending;
        }
        throw error;
      });
      const reply = decodeResponseTagReply(replyBytes);
      if (reply.requestId !== requestId) {
        throw new MisoEngineError("the response reply answers another request", {
          phase: "output", code: "internal", result: constantValue("resultCodes", "internal"),
        });
      }
      if (!reply.ok) {
        const status = responseRefusalResult(reply.status);
        const diagnostics = reply.diagnostics.map((row) => ({ code: row.code, path: "" }));
        throw new MisoEngineError("the live track response query was refused", {
          phase: "output",
          code: (diagnostics[0]?.code ?? status) as MisoEngineError["code"],
          result: constantValue("resultCodes", status),
          diagnostics,
        });
      }
      tag = reply.tag;
      if (this.#eventError !== undefined) throw this.#eventError;
      const known = tag;
      const event = this.#seen.get(known) ?? await Promise.race([
        new Promise<ResponseReadyEvent>((resolve, reject) => {
          this.#waiting = {
            tag: known,
            settle: (value) => (value instanceof Error ? reject(value) : resolve(value)),
          };
        }),
        deadline,
      ]);
      tag = undefined;
      return trackResponseResultOf(event);
    } finally {
      // A query that ended before its event keeps the tag, so the next query waits for it.
      if (tag !== undefined && timedOut && this.#closed === undefined) this.#hold(tag);
      this.#abort = undefined;
      this.#waiting = undefined;
      this.#seen = new Map();
      if (timer !== undefined) clearTimeout(timer);
      this.#outstanding -= 1;
      this.#quiet();
    }
  }
}
