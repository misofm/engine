import { constantValue } from "../core/abi.ts";
import { MisoEngineError, MisoUsageError, resultName } from "../core/errors.ts";
import type {
  MisoAudioWorkletHost,
  MisoMeterFrame,
  MisoTelemetryFrame,
} from "./shipped-host.d.ts";

/** One track's two independent peak magnitudes and folded gain reduction. */
export interface TrackMeter {
  readonly peakLeft: number;
  readonly peakRight: number;
  /** Non-negative gain reduction in decibels; zero also means no observed effect. */
  readonly gainReductionDb: number;
}

/** The designated master reading; `null` preserves an unavailable gain-reduction value. */
export interface MasterMeter {
  readonly peakLeft: number;
  readonly peakRight: number;
  readonly gainReductionDb: number | null;
}

/** One copied decimated meter window, addressed by the engine's compiled track IDs. */
export interface MeterUpdate {
  readonly sequence: bigint;
  readonly generation: bigint;
  readonly validity: number;
  readonly lossCount: number;
  readonly windows: number;
  /** This half-open span timestamps the peak window only. */
  readonly firstSample: bigint;
  readonly endSample: bigint;
  readonly tracks: ReadonlyMap<string, TrackMeter>;
  readonly master: MasterMeter;
}

/** One copied render-time telemetry window. */
export interface TelemetryUpdate {
  readonly sequence: bigint;
  readonly blocks: number;
  readonly cpuPercent: number;
  readonly peakBlockMs: number;
  readonly meanBlockMs: number;
  readonly budgetMs: number;
  readonly deadlineMisses: number;
  readonly resolutionMs: number;
  readonly belowResolution: boolean;
}

export type MeterListener = (update: MeterUpdate) => void;
export type TelemetryListener = (update: TelemetryUpdate) => void;

type LeaseResult = { readonly result: number };

type HostFeed<Frame, Update> = {
  readonly subscribe: (listener: (update: Update) => void) => Promise<() => void>;
  readonly close: () => void;
};

/**
 * The small shared lease reconciler is adapted from the adapter's HostFeed at
 * `misofm/engine-web-adapter/src/console.ts` as of source baseline
 * `f833303f146de7cbe1705fe88ae68a6d6e0d4e45`. It lives here so the SDK owns
 * measurement lifetime alongside its console and observation owners.
 */
function createHostFeed<Frame, Update>(options: {
  readonly name: "meters" | "telemetry";
  readonly available: boolean;
  readonly lease: (onFrame: ((frame: Frame) => void) | null) => Promise<LeaseResult>;
  readonly project: (frame: Frame) => Update;
}): HostFeed<Frame, Update> {
  const listeners = new Set<(update: Update) => void>();
  let reconciling: Promise<void> | undefined;
  let armed = false;
  let closed = false;

  const refusal = (result: number): MisoEngineError => new MisoEngineError(
    `the browser host refused the ${options.name} lease`,
    {
      phase: result === constantValue("resultCodes", "wrongState") ? "lifecycle" : "output",
      code: resultName(result, "call"),
      result,
    },
  );

  const reconcile = (): Promise<void> => {
    if (reconciling !== undefined) return reconciling;
    // Do not occupy the transition slot when the desired state already matches the lease.
    if ((!closed && listeners.size > 0) === armed) return Promise.resolve();
    const run = (async () => {
      for (;;) {
        const wanted = !closed && listeners.size > 0;
        if (wanted === armed) return;
        if (wanted) {
          const ack = await options.lease((frame) => {
            let update: Update;
            try {
              update = options.project(frame);
            } catch {
              // Host validation is authoritative. A malformed custom host frame must not make
              // one callback prevent ownership reconciliation or another callback's delivery.
              return;
            }
            for (const listener of [...listeners]) {
              try {
                listener(update);
              } catch {
                // Consumer callback failures are isolated from sibling listeners and the lease.
              }
            }
          });
          if (ack.result !== constantValue("resultCodes", "ok")) {
            throw refusal(ack.result);
          }
          armed = true;
        } else {
          try {
            await options.lease(null);
          } catch {
            // A failed release is terminal for this feed; host disposal remains authoritative.
          } finally {
            armed = false;
          }
        }
      }
    })();
    reconciling = run;
    void run.then(
      () => { if (reconciling === run) reconciling = undefined; },
      () => { if (reconciling === run) reconciling = undefined; },
    );
    return run;
  };

  return {
    async subscribe(listener): Promise<() => void> {
      if (typeof listener !== "function") {
        throw new TypeError(`${options.name} requires a listener function`);
      }
      if (!options.available) {
        throw new MisoUsageError(
          "this engine booted with no console attached; set policy.console and subscribe again",
        );
      }
      if (closed) throw new MisoUsageError("the browser engine is closed");
      listeners.add(listener);
      try {
        await reconcile();
      } catch (error) {
        listeners.delete(listener);
        throw error;
      }
      let live = true;
      return () => {
        if (!live) return;
        live = false;
        listeners.delete(listener);
        void reconcile();
      };
    },
    close(): void {
      closed = true;
      listeners.clear();
      void reconcile();
    },
  };
}

function meterProjection(frame: MisoMeterFrame, trackIds: readonly string[]): MeterUpdate {
  if (frame.trackCount !== trackIds.length
      || frame.peaks.length !== trackIds.length * 2 + 2
      || frame.trackGrDb.length !== trackIds.length) {
    throw new MisoEngineError("the browser host returned a meter frame for a different session shape", {
      phase: "output",
      code: "abiMismatch",
      result: constantValue("resultCodes", "abiMismatch"),
      diagnostics: [{ code: "sdk.meter.track_count", path: "trackCount" }],
    });
  }
  const tracks = new Map<string, TrackMeter>();
  for (let index = 0; index < trackIds.length; index += 1) {
    const peakLeft = frame.peaks[index * 2]!;
    const peakRight = frame.peaks[index * 2 + 1]!;
    tracks.set(trackIds[index]!, Object.freeze({
      peakLeft,
      peakRight,
      gainReductionDb: frame.trackGrDb[index]!,
    }));
  }
  const masterOffset = trackIds.length * 2;
  return Object.freeze({
    sequence: BigInt(frame.sequence),
    generation: frame.generation,
    validity: frame.validity,
    lossCount: frame.lossCount,
    windows: frame.windows,
    firstSample: frame.firstSample,
    endSample: frame.endSample,
    tracks: tracks as ReadonlyMap<string, TrackMeter>,
    master: Object.freeze({
      peakLeft: frame.peaks[masterOffset]!,
      peakRight: frame.peaks[masterOffset + 1]!,
      gainReductionDb: frame.masterGrDb,
    }),
  });
}

function telemetryProjection(frame: MisoTelemetryFrame): TelemetryUpdate {
  return Object.freeze({
    sequence: BigInt(frame.sequence),
    blocks: frame.blocks,
    cpuPercent: frame.cpuPercent,
    peakBlockMs: frame.peakBlockMs,
    meanBlockMs: frame.meanBlockMs,
    budgetMs: frame.budgetMs,
    deadlineMisses: frame.deadlineMisses,
    resolutionMs: frame.resolutionMs,
    belowResolution: frame.belowResolution,
  });
}

export interface BrowserMeasurementFeeds {
  readonly meters: (listener: MeterListener) => Promise<() => void>;
  readonly telemetry: (listener: TelemetryListener) => Promise<() => void>;
  readonly close: () => void;
}

export function createMeasurementFeeds(
  host: Pick<MisoAudioWorkletHost, "meters" | "telemetry">,
  trackIds: readonly string[],
  available: boolean,
): BrowserMeasurementFeeds {
  const meters = createHostFeed<MisoMeterFrame, MeterUpdate>({
    name: "meters",
    available,
    lease: (onFrame) => host.meters({ enabled: onFrame !== null, onFrame }),
    project: (frame) => meterProjection(frame, trackIds),
  });
  const telemetry = createHostFeed<MisoTelemetryFrame, TelemetryUpdate>({
    name: "telemetry",
    available,
    lease: (onFrame) => host.telemetry({ enabled: onFrame !== null, onFrame }),
    project: telemetryProjection,
  });
  return Object.freeze({
    meters: (listener: MeterListener) => meters.subscribe(listener),
    telemetry: (listener: TelemetryListener) => telemetry.subscribe(listener),
    close: () => { meters.close(); telemetry.close(); },
  });
}
