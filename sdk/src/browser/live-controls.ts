import { commandReasonName } from "../core/boundary.ts";
import type { CommandReport, SessionMap } from "../core/boundary.ts";
import { ABI_LAYOUT } from "../generated/abi.ts";
import { controlCallFailure, decodeControlLimits, encodeRevisionProbe } from "../core/control-frame.ts";
import { ControlRequestIds, ControlTransactionQueue } from "../core/control-queue.ts";
import { readControlStripOrder } from "../core/control-list.ts";
import type { ControlListTransport, ControlStripOrder } from "../core/control-list.ts";
import { EngineLiveControls } from "../core/live-controls.ts";
import type { LiveControlBeforeSubmit } from "../core/live-controls.ts";
import { resultName } from "../core/errors.ts";
import type { SessionLike } from "../core/session.ts";
import type { LaneEdit } from "../core/writer.ts";
import { laneEditWord } from "../internal/ramp.ts";
import type {
  MisoAudioWorkletHost,
  MisoCommand,
  MisoCommandKind,
} from "./shipped-host.d.ts";

function kindValue(name: LaneEdit["kind"]): MisoCommandKind {
  const row = ABI_LAYOUT.constants.wireCommandKinds.find((candidate) => candidate.name === name);
  if (row === undefined) throw new Error(`generated ABI has no command kind ${name}`);
  return row.value as MisoCommandKind;
}

function browserCommand(edit: LaneEdit): MisoCommand {
  return {
    kind: kindValue(edit.kind),
    rack: edit.rack,
    channel: edit.channel,
    trackIndex: edit.trackIndex,
    effectIndex: edit.effectIndex ?? 0,
    parameterId: edit.parameterId ?? 0,
    smoothingSamples: laneEditWord(edit),
    values: [...edit.values],
  };
}

/** Post one command frame to the host's `control` message; a call that failed throws. */
function hostControlSend(host: MisoAudioWorkletHost): (frame: Uint8Array) => Promise<Uint8Array> {
  return async (frame) => {
    const reply = await host.control(frame);
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
  let known: ControlStripOrder | undefined;
  return async () => {
    known = await readControlStripOrder(await createBrowserControlListTransport(host), known);
    return known;
  };
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
    const reply = await host.control(encodeRevisionProbe(requestIds.next()));
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
  beforeSubmit?: LiveControlBeforeSubmit,
  session?: SessionLike,
  booted?: Uint8Array,
): Promise<EngineLiveControls> {
  const remoteMap = await host.sessionMap();
  const queue = await createBrowserControlQueue(host);
  // The strip order is the core's control list (#1564 D5), not the worklet's index exports: the
  // map keeps the worklet's sources and meter lease, and takes the tracks and submixes from the
  // owner headers.
  const order = await readControlStripOrder({ send: hostControlSend(host), requestIds: queue.requestIds });
  const map: SessionMap = Object.freeze({
    tracks: order.tracks,
    sources: Object.freeze(remoteMap.sources.map((source) => Object.freeze({ ...source }))),
    metersAttached: remoteMap.metersAttached,
    submixes: order.submixes,
  });
  return new EngineLiveControls(map, {
    queue,
    observe: async (edits): Promise<CommandReport> => {
      const ack = await host.command({ commands: edits.map(browserCommand) });
      return Object.freeze({
        ok: ack.result === 0,
        result: ack.result,
        code: resultName(ack.result, "call"),
        reason: ack.reason,
        reasonName: commandReasonName(ack.reason),
        rejectedIndex: ack.rejectedIndex,
        admitted: ack.admitted,
        appliedAtSample: ack.appliedAtSample,
      });
    },
  }, beforeSubmit, session, booted);
}
