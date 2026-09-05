/** Issue #322 compile-time red probes for the catalog-derived live console. */

import { ConsoleEdits } from "../src/core/console.ts";
import type {
  MisoCommandAck,
  MisoCommandRequest,
  MisoError,
  MisoAudioWorkletHost,
  MisoObservationRequest,
  MisoObservationAck,
  MisoSessionMap,
  MisoSeekRequest,
  MisoSourceRequest,
  MisoAck,
  MisoStatus,
} from "../src/browser/shipped-host.d.ts";

// @ts-expect-error request IDs belong to the host, never to public request payloads
const oldBrowserRequest: MisoCommandRequest = { requestId: 1, commands: [] };
// @ts-expect-error observation request IDs belong to the host
const oldObservationRequest: MisoObservationRequest = { requestId: 1, subscriptions: [] };
const oldSourceRequest: MisoSourceRequest = {
  // @ts-expect-error source request IDs belong to the host
  requestId: 1, sourceId: "s", generation: 1n, startFrame: 0n, sampleRateHz: 48_000,
  planes: [new Float32Array()], frames: 0, endOfRegion: true,
};
// @ts-expect-error seek request IDs belong to the host
const oldSeekRequest: MisoSeekRequest = { requestId: 1, sourceId: "s", generation: 1n, sourceFrame: 0n };
void [oldBrowserRequest, oldObservationRequest, oldSourceRequest, oldSeekRequest];

declare const commandAck: MisoCommandAck;
declare const acknowledgement: MisoAck;
declare const observationAck: MisoObservationAck;
declare const sessionMap: MisoSessionMap;
declare const engineError: MisoError;
declare const status: MisoStatus;
// @ts-expect-error response request IDs remain readonly
commandAck.requestId = 1;
// @ts-expect-error response request IDs remain readonly
status.requestId = 1;
declare const host: MisoAudioWorkletHost;
// @ts-expect-error meter request IDs belong to the host
host.meters({ requestId: 1, enabled: false, onFrame: null });
// @ts-expect-error telemetry request IDs belong to the host
host.telemetry({ requestId: 1, enabled: false, onFrame: null });
// @ts-expect-error every response request ID remains readonly
acknowledgement.requestId = 1;
// @ts-expect-error every response request ID remains readonly
observationAck.requestId = 1;
// @ts-expect-error every response request ID remains readonly
sessionMap.requestId = 1;
// @ts-expect-error every response request ID remains readonly
engineError.requestId = 1;

type Assert<T extends true> = T;
type NoCallerId<T> = "requestId" extends keyof T ? false : true;
type _CommandPayload = Assert<NoCallerId<Parameters<MisoAudioWorkletHost["command"]>[0]>>;
type _ObservationPayload = Assert<NoCallerId<Parameters<MisoAudioWorkletHost["observe"]>[0]>>;
type _SourcePayload = Assert<NoCallerId<Parameters<MisoAudioWorkletHost["submitSource"]>[0]>>;
type _SeekPayload = Assert<NoCallerId<Parameters<MisoAudioWorkletHost["seekSource"]>[0]>>;
type _MeterPayload = Assert<NoCallerId<Parameters<MisoAudioWorkletHost["meters"]>[0]>>;
type _TelemetryPayload = Assert<NoCallerId<Parameters<MisoAudioWorkletHost["telemetry"]>[0]>>;

const edits = new ConsoleEdits({
  tracks: ["t"],
  sources: [],
  metersAttached: true,
});
const track = edits.track("t");

track.faderDb(-6, { channel: "left", smoothingSamples: 32 });
// @ts-expect-error no untyped option bag is accepted
track.faderDb(-6, { lane: "left" });

const compressor = track.effect("simd1", 0, "miso.compressor");
compressor.parameter("threshold", -18, { channel: "both" });
compressor.observe("Gain Reduction", true, 4);
// @ts-expect-error lookahead is prepared-only and cannot be a live edit
compressor.parameter("lookahead", 1);
// @ts-expect-error a delay parameter is not a compressor parameter
compressor.parameter("delay time", 20);
// @ts-expect-error tap names are descriptor-specific
compressor.observe("Output Level", true);

const delay = track.effect("dynamic", 0, "miso.delay");
delay.parameter("cross feedback", 0.5, { channel: "both" });
// @ts-expect-error cross feedback is shared and cannot address one lane
delay.parameter("cross feedback", 0.5, { channel: "left" });
// @ts-expect-error delay declares no observation tap
delay.observe("Gain Reduction", true);

const eq = track.effect("simd2", 0, "miso.parametric-eq");
// @ts-expect-error the enumeration is prepared-only, not a live console parameter
eq.parameter("band-1-kind", "bell");
