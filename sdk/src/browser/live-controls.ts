import type { SessionMap } from "../core/boundary.ts";
import { MisoEngineError, resultName } from "../core/errors.ts";
import { controlCallFailure, decodeControlLimits, encodeRevisionProbe } from "../core/control-frame.ts";
import { ControlRequestIds, ControlTransactionQueue } from "../core/control-queue.ts";
import { readControlStripOrder } from "../core/control-list.ts";
import type { ControlListTransport, ControlStripOrder } from "../core/control-list.ts";
import { EngineLiveControls } from "../core/live-controls.ts";
import type { SessionLike } from "../core/session.ts";
import type { MisoAudioWorkletHost } from "./shipped-host.d.ts";

/**
 * A host rejection as the SDK raises it (#2190): a raw `miso.error.v1` record, which is not an
 * `Error`, becomes `MisoEngineError` with the record's result; anything else passes unchanged.
 * `wrongState` (a disposed or failed host) and `internal` are the host failing, not a refusal of the
 * frame, and the message says so.
 */
export function hostRecordFailure(error: unknown): unknown {
  if (typeof error !== "object" || error === null) return error;
  const { tag, result } = error as { readonly tag?: unknown; readonly result?: unknown };
  if (tag !== "miso.error.v1" || typeof result !== "number" || !Number.isSafeInteger(result) || result < 0) {
    return error;
  }
  const code = resultName(result, "call");
  const message = code === "wrongState" || code === "internal"
    ? `the browser host failed a control call (${code})`
    : "the browser host refused a control frame";
  return new MisoEngineError(message, { phase: "lifecycle", code, result });
}

/**
 * A `control()` rejection as the SDK raises it (#2193). It is {@link hostRecordFailure}'s mapping,
 * with one question asked of the host for `backpressure`: the host refuses a frame with
 * `RESULT_BACKPRESSURE` (6) while 64 frames are in flight, which drains, but a host that failed
 * rejects every later call with its sticky record, and that record can carry result 6 too (a
 * control-event dequeue the engine refused). The two records are the same shape, so the SDK asks
 * `status()`: a failed host refuses it locally, before it posts anything, with its sticky record. A
 * failed host's `backpressure` is raised as `wrongState`, which the writer never retries, so a
 * writer cannot retry a dead host forever. A host without `status()` is outside the shipped
 * declaration; its record keeps the plain mapping.
 *
 * The question waits for no reply. A healthy host answers `status()` behind the frames in flight,
 * on the same port, and one that stopped answering never does; so the SDK waits one task turn, by
 * which a local refusal has settled, and a `status()` still unsettled then is a host that has not
 * failed: the refusal stays `backpressure` and the call returns at once, as before the question.
 */
async function hostControlFailure(host: MisoAudioWorkletHost, error: unknown): Promise<unknown> {
  const mapped = hostRecordFailure(error);
  if (!(mapped instanceof MisoEngineError) || mapped.code !== "backpressure") return mapped;
  if (typeof (host as { readonly status?: unknown }).status !== "function") return mapped;
  let failed = false;
  // Its reply, if one comes, is not wanted, and a late rejection is not left unhandled.
  try {
    host.status().then(() => undefined, () => { failed = true; });
  } catch {
    failed = true;
  }
  await new Promise<void>((resolve) => setTimeout(resolve, 0));
  if (failed) {
    return new MisoEngineError(
      "the browser host failed a control call (its sticky result is 6, backpressure)",
      { phase: "lifecycle", code: "wrongState", result: mapped.result },
    );
  }
  return mapped;
}

/**
 * Post one command frame to the host's `control` message; a call that failed throws. A frame the
 * host refuses before the core rejects as {@link hostControlFailure} maps it (#2193), so a
 * live-control submit and a control-list read never hand the app the host's raw record.
 */
function hostControlSend(host: MisoAudioWorkletHost): (frame: Uint8Array) => Promise<Uint8Array> {
  return async (frame) => {
    let reply: Awaited<ReturnType<MisoAudioWorkletHost["control"]>>;
    try {
      reply = await host.control(frame);
    } catch (error) {
      throw await hostControlFailure(host, error);
    }
    if (reply.result !== 0) throw controlCallFailure(reply.result, reply.response);
    return reply.response;
  };
}

/**
 * How the browser engine reads the control list (#1564 D1): the host's `control` message, with the
 * request IDs of the host's one allocator (the transaction queue's), so no two frames share an ID.
 */
export async function createBrowserControlListTransport(
  host: MisoAudioWorkletHost,
): Promise<ControlListTransport> {
  const queue = await createBrowserControlQueue(host);
  return { send: hostControlSend(host), requestIds: queue.requestIds };
}

/**
 * The strip order of the host's running session, read from the core's control list (#1564 D5).
 * Each call probes the structure epoch and reads the list again only when it differs from the
 * epoch of the order returned last, so a structure change is followed and nothing is read twice.
 */
export function createBrowserStripOrder(host: MisoAudioWorkletHost): () => Promise<ControlStripOrder> {
  return createBrowserStripOrders(host).current;
}

/** The strip orders the page holds for the host's running session (#1562b). */
export interface BrowserStripOrders {
  /** The order of the newest committed list, read again when its structure epoch moved. */
  readonly current: () => Promise<ControlStripOrder>;
  /** The order `current` returned last, without a read; `undefined` before the first read. */
  readonly latest: () => ControlStripOrder | undefined;
  /** The order of the list of `epoch` if the page holds it now. */
  readonly held: (epoch: number | bigint) => ControlStripOrder | undefined;
  /**
   * The order of the list whose structure epoch is `epoch`: a meter frame or an observation map
   * names its strips by the list of the plan that produced it, which is not the newest list while
   * a replacement plan waits for its block. It is a list this page read, or the newest list when
   * that has this epoch. `undefined` when the list of that epoch is gone: the newest list is
   * another one and this page never held it. The caller drops what it cannot name.
   */
  readonly of: (epoch: number | bigint) => Promise<ControlStripOrder | undefined>;
}

/** The lists the page remembers: a structural commit and its adoption are the two in flight. */
const HELD_STRIP_ORDERS = 8;

/**
 * The strip orders of one host (#1562b). A rebuild that adds, removes or reorders a strip moves the
 * list's structure epoch at its commit, and the plan that renders keeps naming its strips by the
 * previous list until the block that adopts the replacement. So each order is kept by its epoch and
 * a frame is named by the order of the epoch it carries. Reads are the ones {@link
 * createBrowserStripOrder} makes: a probe, and the whole list only when the epoch moved.
 */
export function createBrowserStripOrders(host: MisoAudioWorkletHost): BrowserStripOrders {
  const known = hostStripOrders.get(host);
  if (known !== undefined) return known;
  const created = newBrowserStripOrders(host);
  hostStripOrders.set(host, created);
  return created;
}

/** One reader per host: the queue's effect-row check, the live controls and the feeds share its cache. */
const hostStripOrders = new WeakMap<object, BrowserStripOrders>();

function newBrowserStripOrders(host: MisoAudioWorkletHost): BrowserStripOrders {
  const orders = new Map<bigint, ControlStripOrder>();
  let newest: ControlStripOrder | undefined;
  const current = async (): Promise<ControlStripOrder> => {
    const read = await readControlStripOrder(await createBrowserControlListTransport(host), newest);
    newest = read;
    orders.delete(read.structureEpoch);
    orders.set(read.structureEpoch, read);
    while (orders.size > HELD_STRIP_ORDERS) {
      const oldest = orders.keys().next();
      if (oldest.done) break;
      orders.delete(oldest.value);
    }
    return read;
  };
  const held = (epoch: number | bigint): ControlStripOrder | undefined => orders.get(BigInt(epoch));
  return Object.freeze({
    current,
    latest: () => newest,
    held,
    of: async (epoch: number | bigint) => {
      const known = held(epoch);
      if (known !== undefined) return known;
      const read = await current();
      return read.structureEpoch === BigInt(epoch) ? read : undefined;
    },
  });
}

const hostQueues = new WeakMap<object, Promise<ControlTransactionQueue>>();

/**
 * The browser's transaction queue for one host (#1563 D7): it posts each `SET_CONTROL` transaction
 * as the worklet's `control` message and decodes the reply with the same code the headless engine
 * uses. It starts from the committed revision the core reports now, read with a revision probe
 * (a `CAPABILITIES_GET`, which takes any revision), because the host exposes the revision only in a
 * reply; the probe's reply also holds the frame limits the queue keeps every transaction under. The
 * probe and the transactions draw their request IDs from the queue's one allocator
 * (`queue.requestIds`), which must also serve every other frame sent through `host.control()`: the
 * engine's replay cache refuses a reused ID.
 *
 * One host has one allocator, so one queue: a second call for the same host returns the first
 * call's queue. A call whose probe failed is not remembered, and the next call probes again.
 */
export function createBrowserControlQueue(
  host: MisoAudioWorkletHost,
): Promise<ControlTransactionQueue> {
  const known = hostQueues.get(host);
  if (known !== undefined) return known;
  const created = openBrowserControlQueue(host);
  hostQueues.set(host, created);
  created.catch(() => {
    if (hostQueues.get(host) === created) hostQueues.delete(host);
  });
  return created;
}

async function openBrowserControlQueue(host: MisoAudioWorkletHost): Promise<ControlTransactionQueue> {
  const requestIds = new ControlRequestIds();
  const probe = async () => {
    let reply: Awaited<ReturnType<MisoAudioWorkletHost["control"]>>;
    try {
      reply = await host.control(encodeRevisionProbe(requestIds.next()));
    } catch (error) {
      throw hostRecordFailure(error);
    }
    if (reply.result !== 0) throw controlCallFailure(reply.result, reply.response);
    return reply;
  };
  const first = await probe();
  return new ControlTransactionQueue(hostControlSend(host), {
    initialRevision: first.revision,
    requestIds,
    limits: decodeControlLimits(first.response),
    readRevision: async () => (await probe()).revision,
  });
}

/**
 * Bind the shared semantic live controls to the shipped MessagePort host.
 *
 * `session`, the builder the host's document came from, lets console slots and inserts be checked
 * and addressed by index. `booted`, the document the host booted, is what `withSession()` holds a
 * later builder to; without it, `withSession()` refuses.
 */
export async function createBrowserLiveControls(
  host: MisoAudioWorkletHost,
  session?: SessionLike,
  booted?: Uint8Array,
): Promise<EngineLiveControls> {
  const remoteMap = await host.sessionMap();
  const queue = await createBrowserControlQueue(host);
  // The strip order is the core's control list (#1564 D5), not the worklet's index exports: the
  // map keeps the worklet's sources and meter lease, and takes the tracks and submixes from the
  // owner headers.
  const orders = createBrowserStripOrders(host);
  const order = await orders.current();
  const map: SessionMap = Object.freeze({
    tracks: order.tracks,
    sources: Object.freeze(remoteMap.sources.map((source) => Object.freeze({ ...source }))),
    metersAttached: remoteMap.metersAttached,
    submixes: order.submixes,
  });
  // The live controls follow the host's newest order (#2189): every read of it -- the browser
  // engine's after a committed rebuild, a meter frame's, a feed's -- moves the strips they check, so
  // live controls an app already holds are never left on the strip set they were made with.
  return new EngineLiveControls(map, { queue, order: () => orders.latest() ?? order }, session, booted);
}
