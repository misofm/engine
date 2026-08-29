/**
 * The browser entry: a scratch boot in a Worker, then an AudioContext, then the worklet.
 *
 * Source plumbing stays bring-your-own here. The store/resolver seam is #244's, and SDK core has
 * no opinions about audio plumbing by ruling #207/5448359546 -- no OPFS, no fetch, no Workers in
 * core -- so this entry takes the Worker boot and the context construction as injected functions
 * rather than reaching for them itself.
 */
export * from "./engine.ts";
export * from "./policy.ts";
