/** Issue #322 compile-time red probes for the catalog-derived live console. */

import { ConsoleEdits } from "../src/core/console.ts";
import type { MisoCommandRequest } from "../src/browser/shipped-host.d.ts";

// @ts-expect-error request IDs belong to the host, never to public request payloads
const oldBrowserRequest: MisoCommandRequest = { requestId: 1, commands: [] };

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
