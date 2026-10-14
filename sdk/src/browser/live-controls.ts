import { commandReasonName } from "../core/boundary.ts";
import type { CommandReport, SessionMap } from "../core/boundary.ts";
import { ABI_LAYOUT } from "../generated/abi.ts";
import { controlCallFailure, encodeRevisionProbe } from "../core/control-frame.ts";
import { ControlRequestIds, ControlTransactionQueue } from "../core/control-queue.ts";
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

/**
 * The browser's transaction queue for one host (#1563 D7): it posts each `SET_CONTROL` transaction
 * as the worklet's `control` message and decodes the reply with the same code the headless engine
 * uses. It starts from the committed revision the core reports now, read with a revision probe
 * (a `CAPABILITIES_GET`, which takes any revision), because the host exposes the revision only in a
 * reply. The probe and the transactions draw their request IDs from the queue's one allocator
 * (`queue.requestIds`), which must also serve every other frame sent through `host.control()`: the
 * engine's replay cache refuses a reused ID.
 */
export async function createBrowserControlQueue(
  host: MisoAudioWorkletHost,
): Promise<ControlTransactionQueue> {
  const requestIds = new ControlRequestIds();
  const probe = await host.control(encodeRevisionProbe(requestIds.next()));
  if (probe.result !== 0) throw controlCallFailure(probe.result, probe.response);
  return new ControlTransactionQueue(async (frame) => {
    const reply = await host.control(frame);
    if (reply.result !== 0) throw controlCallFailure(reply.result, reply.response);
    return reply.response;
  }, { initialRevision: probe.revision, requestIds });
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
  const map: SessionMap = Object.freeze({
    tracks: Object.freeze([...remoteMap.tracks]),
    sources: Object.freeze(remoteMap.sources.map((source) => Object.freeze({ ...source }))),
    metersAttached: remoteMap.metersAttached,
    submixes: Object.freeze([...remoteMap.submixes]),
  });
  const queue = await createBrowserControlQueue(host);
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
