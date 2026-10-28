// Issue #1295 (the #1697 gate 3 measurement): the 1 MiB worst document (the boot ceiling until
// #2191 made the memory budget the document's limit), built here so the
// browser job, which has no Rust toolchain, can send it. It is the document
// `hosts/host-web/examples/worst_boot_document.rs` prints -- 192 tracks with four parametric-EQ
// inserts each and one route per track, in the session's canonical pretty form, padded with
// spaces to `WORST_DOCUMENT_BYTES` -- byte for byte (compared once against `cargo run -p host-web
// --example worst_boot_document`; the spec's decision record holds that comparison).

export const WORST_DOCUMENT_BYTES = 1 << 20;
const TRACKS = 192;
const EFFECTS_PER_TRACK = 4;

/// A JSON number the canonical form writes with a fraction (`0.0`, `20000.0`).
class Float {
  constructor(value) {
    this.value = value;
  }
}
const float = (value) => new Float(value);

function pretty(value, depth) {
  const inner = "  ".repeat(depth + 1);
  const outer = "  ".repeat(depth);
  if (value instanceof Float) {
    return Number.isInteger(value.value) ? `${value.value}.0` : String(value.value);
  }
  if (Array.isArray(value)) {
    if (value.length === 0) return "[]";
    return `[\n${value.map((item) => inner + pretty(item, depth + 1)).join(",\n")}\n${outer}]`;
  }
  if (value !== null && typeof value === "object") {
    const entries = Object.entries(value);
    if (entries.length === 0) return "{}";
    return `{\n${entries.map(([key, item]) => `${inner}${JSON.stringify(key)}: ${pretty(item, depth + 1)}`)
      .join(",\n")}\n${outer}}`;
  }
  return JSON.stringify(value);
}

function builtin() {
  return {
    polarity_invert: false,
    trim_db: float(0),
    hpf_hz: float(20),
    lpf_hz: float(20000),
    delay_samples: 0,
  };
}

function track(index) {
  const id = `track-${String(index).padStart(3, "0")}`;
  return {
    id,
    source_id: "fixture-source",
    left_source_channel: 0,
    right_source_channel: 1,
    builtins: { left: builtin(), right: builtin() },
    console: [],
    inserts: {
      effects: Array.from({ length: EFFECTS_PER_TRACK }, (_, effect) => ({
        id: `effect-${effect}`,
        identity: { kind: "native", effect_id: "miso.parametric-eq" },
        quality: "normal",
        bypass: false,
        link_mode: "dual_mono",
        params: [],
        sidechain: { kind: "none" },
      })),
    },
    fader: { left_db: float(0), right_db: float(0), left_mute: false, right_mute: false },
    pan: { left: float(1), right: float(1), smoothing_samples: 16 },
  };
}

function route(index) {
  const suffix = String(index).padStart(3, "0");
  return {
    id: `route-${suffix}`,
    source: { kind: "track", track_id: `track-${suffix}`, tap: "post_pan" },
    destination: { kind: "output_input", output_id: "main-out" },
    channel_matrix: { ll: float(1), lr: float(0), rl: float(0), rr: float(1) },
    gain_db: float(0),
    mute: false,
    follows_mute: false,
  };
}

/// The document's bytes, exactly `WORST_DOCUMENT_BYTES` long.
export function worstReplaceDocument() {
  const model = {
    schema_version: 1,
    session_id: "parametric-eq-nine-track",
    revision: "42",
    sample_rate_hz: 48000,
    quantum_frames: 128,
    render_profile: { id: "native", mode: "single_thread" },
    output_profile: { id: "main", channels: 2, sample_format: "f32_planar" },
    sources: [{
      id: "fixture-source",
      content: "blake3:7e945c107a97cd24135e85dc2f407c5ecd39663a8737bf5b92114ccce38f1ab8",
      channels: 2,
      bit_depth: "32f",
      frames: "48000",
    }],
    console: { pre_insert: [], post_insert: [] },
    tracks: Array.from({ length: TRACKS }, (_, index) => track(index)),
    submixes: [],
    vcas: [],
    control_smoothing: { kind: "default" },
    outputs: [{ id: "main-out" }],
    routes: Array.from({ length: TRACKS }, (_, index) => route(index)),
    automation: [],
  };
  const text = new TextEncoder().encode(`${pretty(model, 0)}\n`);
  if (text.byteLength + 2 > WORST_DOCUMENT_BYTES) throw new Error("the worst document outgrew its length");
  const document = new Uint8Array(WORST_DOCUMENT_BYTES).fill(0x20);
  document.set(text, 0);
  return document;
}
