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

const ZERO_CONTENT = `sha256:${"0".repeat(64)}`;

/**
 * A one-track Session V1 document.
 *
 * JSON keys are always quoted. The helper intentionally emits noncanonical-but-valid whitespace
 * so boot tests continue proving that acceptance belongs to the engine rather than the builder.
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
    effects = { simd1: [], dynamic: [], simd2: [] },
    padding = 0,
  } = options;
  const lane = { polarity_invert: false, trim_db: 0.0, hpf_hz: 0.0, lpf_hz: 0.0, delay_samples: 0 };
  const document = {
    schema_version: 1, session_id: sessionId, revision: "1", sample_rate_hz: sampleRateHz,
    quantum_frames: quantumFrames,
    render_profile: { id: "native", mode: renderMode },
    output_profile: { id: "main", channels: 2, sample_format: "f32_planar" },
    sources: [{ id: "s", content, channels, bit_depth: bitDepth, frames: String(frames) }],
    tracks: [{
      id: trackId, source_id: "s", left_source_channel: 0,
      right_source_channel: channels === 1 ? 0 : 1,
      builtins: { left: { ...lane }, right: { ...lane } },
      simd1: { effects: effects.simd1 ?? [] }, dynamic: { effects: effects.dynamic ?? [] },
      simd2: { effects: effects.simd2 ?? [] },
      fader: { left_db: 0.0, right_db: 0.0, left_mute: false, right_mute: false },
      pan: { left: -1.0, right: 1.0, smoothing_samples: 0 },
    }],
    submixes: [], outputs: [{ id: "out" }],
    routes: [{ id: "main", source: { kind: "track", track_id: trackId, tap: "post_matrix" },
      destination: { kind: "output_input", output_id: "out" },
      channel_matrix: { ll: 1.0, lr: 0.0, rl: 0.0, rr: 1.0 }, gain_db: 0.0 }],
    automation: [],
  };
  let text = JSON.stringify(document, null, 2);
  if (sourceExtra) text = text.replace(`"frames": "${frames}"`, `"frames": "${frames}"${sourceExtra}`);
  return `${text}${" ".repeat(padding)}\n`;
}

/** One effect entry for a rack, with explicit parameter rows. */
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
