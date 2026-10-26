// Source-tree redirect only (issue #2102). `stage-package.mjs` replaces this file in `dist` with a
// verbatim copy of the engine's `hosts/host-web/web/stem-store/pcm-pump.js`; `pcm-pump.d.ts` beside
// it is the published declaration.
export { CanonicalPcmPump, CanonicalPcmPumpError, deinterleaveCanonicalPcm } from "../../../../hosts/host-web/web/stem-store/pcm-pump.js";
