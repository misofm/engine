import { ABI_LAYOUT } from "../generated/abi.ts";
import { commandReasonName } from "../core/boundary.ts";
import type { CommandReport, SessionMap } from "../core/boundary.ts";
import { EngineConsole } from "../core/console.ts";
import { resultName } from "../core/errors.ts";
import type { LaneEdit } from "../core/writer.ts";
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
    smoothingSamples: edit.smoothingSamples ?? 0,
    values: [...edit.values],
  };
}

/** Bind the shared semantic console to the shipped MessagePort host. */
export async function createBrowserConsole(host: MisoAudioWorkletHost): Promise<EngineConsole> {
  const remoteMap = await host.sessionMap();
  const map: SessionMap = Object.freeze({
    tracks: Object.freeze([...remoteMap.tracks]),
    sources: Object.freeze(remoteMap.sources.map((source) => Object.freeze({ ...source }))),
    metersAttached: remoteMap.metersAttached,
  });
  // `sessionMap()` itself consumes the host's next request ID. Continue from the acknowledgement;
  // restarting at one would be locally well-typed and rejected by the host's monotonic ledger.
  let requestId = remoteMap.requestId;
  return new EngineConsole(map, async (edits): Promise<CommandReport> => {
    requestId += 1;
    const ack = await host.command({
      requestId,
      commands: edits.map(browserCommand),
    });
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
  });
}
