/**
 * Shared fixtures for the SDK evals.
 *
 * Every document here is built independently rather than by the SDK's own builder, on
 * purpose: the boot evals are about what the *engine* does with bytes, and a fixture that went
 * through the builder would prove the builder and the engine agree with each other rather than
 * proving either against the schema. The builder gets its own eval.
 */

import { readFile } from "node:fs/promises";
import process from "node:process";

import { ABI_LAYOUT } from "../src/generated/abi.ts";
import { onEndpointIds } from "./control-support.mjs";

const ARTIFACT_DIRECTORY_ENV = "MISO_ENGINE_SDK_ARTIFACTS_HEX";
const WASM_BASENAME = Buffer.from("/miso-engine-v1-audio-worklet.simd128.wasm", "ascii");

/**
 * The release artifact directory.
 *
 * Supplied by `scripts/check-sdk-headless.sh`, which builds it with
 * `scripts/build-web-audioworklet.sh` into a temp directory. There is no checked-in wasm: the
 * module is a build output, and vendoring a two-and-a-half megabyte binary into the source tree
 * would make the SDK's provenance story a copy rather than a derivation.
 */
export function artifactDirBytes() {
  const encoded = process.env[ARTIFACT_DIRECTORY_ENV];
  if (!encoded) {
    throw new Error(
      `${ARTIFACT_DIRECTORY_ENV} must encode a directory built by scripts/build-web-audioworklet.sh`,
    );
  }
  if (!/^(?:[0-9a-f]{2})+$/.test(encoded)) {
    throw new Error(`${ARTIFACT_DIRECTORY_ENV} must be canonical lowercase, even-length hex`);
  }
  return Buffer.from(encoded, "hex");
}

export async function moduleBytes() {
  const modulePath = Buffer.concat([artifactDirBytes(), WASM_BASENAME]);
  return new Uint8Array(await readFile(modulePath));
}

const ZERO_CONTENT = `blake3:${"0".repeat(64)}`;

/**
 * A one-track Session V1 document.
 *
 * JSON keys are always quoted. The helper intentionally emits noncanonical-but-valid whitespace
 * so boot tests continue proving that acceptance belongs to the engine rather than the builder.
 *
 * `effects` places `effectEntry()` records: `preInsert` and `postInsert` become session console
 * slots (identity, quality and link mode on the slot, bypass and params on the track's entry), and
 * `inserts` stays the track's full effect declarations. The retired `simd1`, `dynamic` and
 * `simd2` keys throw, so an unmigrated eval cannot silently lose its effects.
 */
export function sessionDocument(options = {}) {
  const {
    sampleRateHz = 48_000,
    quantumFrames = 128,
    frames = 4_800,
    channels = 2,
    bitDepth = 24,
    content = ZERO_CONTENT,
    sessionId = "sdk-eval",
    sourceExtra = "",
    renderMode = "single_thread",
    trackId = "t",
    effects = {},
    padding = 0,
  } = options;
  for (const retired of ["simd1", "dynamic", "simd2"]) {
    if (retired in effects) throw new Error(`sessionDocument: '${retired}' is retired; use preInsert, inserts or postInsert`);
  }
  const slot = (entry) => ({
    slot: entry.id, identity: entry.identity, quality: entry.quality, link_mode: entry.link_mode,
  });
  const consoleEntry = (entry) => ({ slot: entry.id, bypass: entry.bypass, params: entry.params });
  const preInsert = effects.preInsert ?? [];
  const postInsert = effects.postInsert ?? [];
  const lane = { polarity_invert: false, trim_db: 0.0, hpf_hz: 0.0, lpf_hz: 0.0, delay_samples: 0 };
  const document = {
    schema_version: 1, session_id: sessionId, revision: "1", sample_rate_hz: sampleRateHz,
    quantum_frames: quantumFrames,
    render_profile: { id: "native", mode: renderMode },
    output_profile: { id: "main", channels: 2, sample_format: "f32_planar" },
    sources: [{ id: "s", content, channels, bit_depth: bitDepth, frames: String(frames) }],
    console: { pre_insert: preInsert.map(slot), post_insert: postInsert.map(slot) },
    tracks: [{
      id: trackId, source_id: "s", left_source_channel: 0,
      right_source_channel: channels === 1 ? 0 : 1,
      builtins: { left: { ...lane }, right: { ...lane } },
      console: [...preInsert, ...postInsert].map(consoleEntry),
      inserts: { effects: effects.inserts ?? [] },
      fader: { left_db: 0.0, right_db: 0.0, left_mute: false, right_mute: false },
      pan: { left: -1.0, right: 1.0, smoothing_samples: 0 },
    }],
    submixes: [], vcas: [], control_smoothing: { kind: "default" }, outputs: [{ id: "out" }],
    routes: [{ id: "main", source: { kind: "track", track_id: trackId, tap: "post_pan" },
      destination: { kind: "output_input", output_id: "out" },
      channel_matrix: { ll: 1.0, lr: 0.0, rl: 0.0, rr: 1.0 }, gain_db: 0.0, mute: false, follows_mute: false }],
    automation: [],
  };
  let text = JSON.stringify(document, null, 2);
  if (sourceExtra) text = text.replace(`"frames": "${frames}"`, `"frames": "${frames}"${sourceExtra}`);
  return `${text}${" ".repeat(padding)}\n`;
}

/** One effect record, with explicit parameter rows: an insert, or a console slot via `effects`. */
export function effectEntry(id, effectId, params = []) {
  return {
    id, identity: { kind: "native", effect_id: effectId }, quality: "normal", bypass: false,
    link_mode: "dual_mono",
    params: params.map((p) => ({ parameter_id: p.id, channel: p.channel ?? "both", unit: p.unit, value: Number(p.value) })),
    sidechain: { kind: "none" },
  };
}

/** Deterministic pseudo-audio: no RNG, so a digest is reproducible across machines. */
export function ramp(frames, seed = 1) {
  const plane = new Float32Array(frames);
  let state = seed >>> 0;
  for (let index = 0; index < frames; index += 1) {
    state = (Math.imul(state, 1_664_525) + 1_013_904_223) >>> 0;
    plane[index] = (state / 0x1_0000_0000) * 2 - 1;
  }
  return plane;
}

/** FNV-1a over the little-endian `f32` bytes of both output planes. */
export function digestPlanes(blocks) {
  let hash = 0xcbf2_9ce4_8422_2325n;
  const prime = 0x1000_0000_01b3n;
  const mask = 0xffff_ffff_ffff_ffffn;
  for (const { left, right } of blocks) {
    for (const plane of [left, right]) {
      const bytes = new Uint8Array(plane.buffer, plane.byteOffset, plane.byteLength);
      for (const byte of bytes) {
        hash = ((hash ^ BigInt(byte)) * prime) & mask;
      }
    }
  }
  return hash.toString(16).padStart(16, "0");
}

/**
 * A `MisoAudioWorkletHost` stand-in whose control plane is a real headless engine's: `control()`
 * runs the frame through the same core entry the worklet's message handler calls and answers as the
 * host does (`{ result, response, revision }`, a failed call answering with the diagnostic text), so
 * the browser transport of the SDK is judged against the real core's reply bytes.
 * `command` answers the observation records through the engine's command path.
 */
export function engineHost(engine, { frames = [] } = {}) {
  let next = 0;
  // The engine reads its own strip order from the core's list, so the host's frames share its
  // endpoint's one request-ID allocator.
  const forward = onEndpointIds(engine.controlTransactions().requestIds, (frame) => engine.controlFrame(frame));
  return {
    onControlEvent(listener) { return engine.onControlEvent(listener); },
    async sessionMap() {
      return { tag: "miso.sessionmap.v1", requestId: (next += 1), result: 0, sources: engine.sessionMap().sources, metersAttached: engine.sessionMap().metersAttached };
    },
    async control(frame) {
      frames.push(frame.slice());
      const requestId = (next += 1);
      let result = 0;
      let response;
      try {
        response = forward(frame);
      } catch (error) {
        if (typeof error?.result !== "number") throw error;
        result = error.result;
        response = new TextEncoder().encode(
          error.diagnostics.map((row) => `${row.code}\t${row.path}\n`).join(""),
        );
      }
      return { tag: "miso.control.v1", requestId, result, response, revision: engine.controlRevision() };
    },
    async command(value) {
      const requestId = (next += 1);
      const bytes = new Uint8Array(value.commands.length * ABI_LAYOUT.commandRecord.bytes);
      const view = new DataView(bytes.buffer);
      const at = (name) => ABI_LAYOUT.commandRecord.fields.find((row) => row.name === name).offset;
      value.commands.forEach((command, index) => {
        const base = index * ABI_LAYOUT.commandRecord.bytes;
        view.setUint8(base + at("kind"), command.kind);
        view.setUint8(base + at("rack"), command.rack);
        const strip = new TextEncoder().encode(command.stripId);
        const effect = new TextEncoder().encode(command.effectId);
        view.setUint8(base + at("stripIdBytes"), strip.length);
        view.setUint8(base + at("effectIdBytes"), effect.length);
        view.setUint32(base + at("tapId"), command.tapId, true);
        view.setUint32(base + at("windowBlocks"), command.windowBlocks, true);
        bytes.set(strip, base + at("stripId"));
        bytes.set(effect, base + at("effectId"));
      });
      const report = engine.submitCommands(bytes, value.commands.length);
      return {
        tag: "miso.ack.v1", requestId, result: report.result, reason: report.reason,
        rejectedIndex: report.rejectedIndex, admitted: report.admitted,
        appliedAtSample: report.appliedAtSample, records: bytes,
      };
    },
  };
}
