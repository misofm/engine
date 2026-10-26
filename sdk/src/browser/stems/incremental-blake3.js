// Source-tree redirect only (issue #2102). `stage-package.mjs` replaces this file in `dist` with a
// verbatim copy of the engine's `hosts/host-web/web/stem-store/incremental-blake3.js`;
// `incremental-blake3.d.ts` beside it is the published declaration.
export { IncrementalBlake3, blake3Stream, prepareBlake3 } from "../../../../hosts/host-web/web/stem-store/incremental-blake3.js";
