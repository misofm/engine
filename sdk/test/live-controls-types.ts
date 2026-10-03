/** Issue #322 compile-time red probes for the catalog-derived live controls. */

import { LiveControlEdits } from "../src/core/live-controls.ts";
import type { RouteEdits, SubmixEdits, TrackEdits } from "../src/core/live-controls.ts";
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

const edits = new LiveControlEdits({
  tracks: ["t"],
  sources: [],
  metersAttached: true,
  submixes: [],
  routes: [],
});
const track = edits.track("t");

track.faderDb(-6, { channel: "left", smoothingSamples: 32 });
// @ts-expect-error no untyped option bag is accepted
track.faderDb(-6, { lane: "left" });

const gate = track.effect("console", 0, "miso.gate-expander");
// @ts-expect-error lookahead is absent from the causal launch gate
gate.parameter("lookahead", 1);

const compressor = track.effect("console", 0, "miso.compressor");
compressor.parameter("threshold", -18, { channel: "both" });
compressor.parameter({ key: "threshold", value: -18, channel: "both", smoothingSamples: 64 });
compressor.observe("Gain Reduction", true, 4);
// @ts-expect-error lookahead is absent from the causal launch compressor
compressor.parameter("lookahead", 1);
// @ts-expect-error a delay parameter is not a compressor parameter
compressor.parameter("delay time", 20);
// @ts-expect-error tap names are descriptor-specific
compressor.observe("Output Level", true);
// @ts-expect-error object edits keep key/value pairs discriminated
compressor.parameter({ key: "threshold", value: true });
// @ts-expect-error a prepared-only parameter is absent from live object edits
compressor.parameter({ key: "lookahead", value: 1 });
// @ts-expect-error object edits reject guessed units instead of converting them
compressor.parameter({ key: "threshold", value: -18, unit: "db" });
// @ts-expect-error unknown parameter keys are not accepted
compressor.parameter({ key: "missing", value: 0 });

const multiband = track.effect("inserts", 0, "miso.multiband-compressor");
multiband.parameter("low_threshold", -18, { channel: "both" });
// @ts-expect-error lookahead is absent from the causal launch multiband compressor
multiband.parameter("lookahead", 1);

const delay = track.effect("inserts", 0, "miso.delay");
delay.parameter("cross feedback", 0.5, { channel: "both" });
// @ts-expect-error cross feedback is shared and cannot address one lane
delay.parameter("cross feedback", 0.5, { channel: "left" });
// @ts-expect-error cross feedback is shared and cannot address one lane in object form
delay.parameter({ key: "cross feedback", value: 0.5, channel: "left" });
// @ts-expect-error delay declares no observation tap
delay.observe("Gain Reduction", true);

const eq = track.effect("console", 1, "miso.parametric-eq");
// @ts-expect-error the retired rack tokens are not live-control racks
track.effect("simd1", 0, "miso.parametric-eq");
// @ts-expect-error the retired rack tokens are not live-control racks
track.effect("dynamic", 0, "miso.parametric-eq");
// Stable IDs: a console slot by its slot ID, an insert by its ID or index.
track.console("eq", "miso.parametric-eq").bypass(true);
track.insert("delay", "miso.delay").bypass(true);
track.insert(0, "miso.delay").parameter("cross feedback", 0.5);
// @ts-expect-error a console slot is addressed by its stable ID, not an index
track.console(0, "miso.parametric-eq");
// @ts-expect-error the enumeration is prepared-only, not a live-control parameter
eq.parameter("band-1-kind", "bell");
eq.parameter("hpf-enabled", true);
eq.parameter("lpf-enabled", false);

// Issue #1214 gate 5: a bus's live edits are a track's minus solo, and submix() says so in its type.
type Exact<A, B> = [A] extends [B] ? ([B] extends [A] ? true : false) : false;
type _SubmixReturn = Assert<Exact<ReturnType<LiveControlEdits["submix"]>, SubmixEdits>>;
type _SubmixHasNoSolo = Assert<"solo" extends keyof SubmixEdits ? false : true>;
type _TrackKeepsSolo = Assert<"solo" extends keyof TrackEdits ? true : false>;
type _SameStripSurface = Assert<Exact<Exclude<keyof TrackEdits, "solo">, keyof SubmixEdits>>;
const busEdits = new LiveControlEdits({
  tracks: ["t"],
  sources: [],
  metersAttached: false,
  submixes: ["bus"],
  routes: ["t-bus"],
});
const bus = busEdits.submix("bus");
bus.faderDb(-6, { channel: "left", smoothingSamples: 32 });
bus.effect("inserts", 0, "miso.compressor").parameter("threshold", -18);
// @ts-expect-error a bus is solo-safe, so its live edits have no solo
bus.solo(true);
// @ts-expect-error a bus's edits are not a track's (they lack solo)
const notTrack: TrackEdits = bus;
void notTrack;
// #1214 MINOR-1: strip() resolves either kind and offers only what every strip shares.
// @ts-expect-error strip() may name a bus, so it never offers solo
busEdits.strip("t").solo(true);
busEdits.strip("bus").effect("inserts", 0, "miso.compressor").observe("Gain Reduction", true, 1);

// Issue #1223 D4/D5: a send's live edits are exactly its gain, mute and matrix. A send has no lane
// (its record's channel is not applicable), no effects and no solo, and route() says so in its type.
type _RouteReturn = Assert<Exact<ReturnType<LiveControlEdits["route"]>, RouteEdits>>;
type _RouteSurface = Assert<Exact<keyof RouteEdits, "gainDb" | "mute" | "matrix">>;
type _SessionMapRoutes = Assert<Exact<MisoSessionMap["routes"], readonly string[]>>;
const send = busEdits.route("t-bus");
send.gainDb(-12, { smoothingSamples: 480 });
send.mute(true);
send.matrix({ ll: 1, lr: 0.25, rl: 0, rr: 0.5 }, { smoothingSamples: 0 });
// @ts-expect-error a send has no lane: its edits move both of its lanes together
send.gainDb(-12, { channel: "left" });
// @ts-expect-error a send's matrix names all four coefficients
send.matrix({ ll: 1, rr: 1 });
// @ts-expect-error a send is not a strip: it has no fader
send.faderDb(-6);
