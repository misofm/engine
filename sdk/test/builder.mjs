#!/usr/bin/env node
/** Focused Session V1 builder gate: canonical JSON/TOML and local metadata refusal. */

import assert from "node:assert/strict";
import { mkdtemp, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { resolve } from "node:path";
import { spawnSync } from "node:child_process";
import { pathToFileURL } from "node:url";

const sdkRoot = resolve(import.meta.dirname, "..");
const compiler = resolve(sdkRoot, "node_modules", ".bin", "tsc");

function compile(directory) {
  const result = spawnSync(compiler, [
    "--project", "tsconfig.json", "--noEmit", "false", "--rootDir", "src", "--outDir", directory,
  ], { cwd: sdkRoot, encoding: "utf8" });
  assert.equal(result.status, 0, `SDK builder compilation failed:\n${result.stdout}${result.stderr}`);
}

async function withModule(run) {
  const directory = await mkdtemp(resolve(tmpdir(), "miso-sdk-builder-"));
  try {
    compile(directory);
    await run(await import(pathToFileURL(resolve(directory, "index.js")).href));
  } finally {
    await rm(directory, { recursive: true, force: true });
  }
}

async function check() {
  await withModule(async (sdk) => {
    const compressor = sdk.effect("miso.compressor", { threshold: -18, ratio: { left: 2, right: 4 } }, { slotId: "vca" });
    const start = sdk.session({ id: "builder.session", sampleRateHz: 48_000 });
    const plan = start
      .source("voice", { channels: 2, frames: 48_000, identity: "fixture-voice", locator: "fixture:voice" })
      .track("lead", { source: "voice", dynamic: [compressor] })
      .automate({ id: "vca-threshold", target: { trackId: "lead", rack: "dynamic", slotId: "vca", parameter: "threshold", channel: "both" }, segments: [{ shape: "linear", startSample: 0n, endSample: 480n, startValue: -18, endValue: -12 }] })
      .build();

    assert.equal(start.build().json.tracks.length, 0, "builder methods must be immutable");
    assert.equal(plan.json.schema_version, 1);
    assert.deepEqual(plan.json.outputs, [{ id: "main" }]);
    assert.deepEqual(plan.json.routes, [{
      id: "auto-route-1",
      source: { kind: "track", track_id: "lead", tap: "post_matrix" },
      destination: { kind: "output_input", output_id: "main" },
      channel_matrix: { ll: 1, lr: 0, rl: 0, rr: 1 }, gain_db: 0,
    }]);
    assert.equal(plan.json.tracks[0].dynamic.effects[0].id, "vca");
    assert.deepEqual(plan.json.automation, [{ id: "vca-threshold", target: { entity_id: "lead", rack: "dynamic", effect_id: "vca", parameter_id: 1, channel: "both" }, segments: [{ shape: "linear", start_sample: "0", end_sample: "480", start_value: -18, end_value: -12, unit: "db" }] }]);
    assert.match(plan.toml, /^schema_version = 1\nsession_id = "builder.session"\n/m);
    assert.equal(sdk.SessionPlan.fromJson(JSON.parse(JSON.stringify(plan.json))).toml, plan.toml, "JSON round-trip must preserve canonical TOML bytes");
    assert.match(plan.toml, /start_sample = 0, end_sample = 480/, "JSON-safe u64 strings must emit as TOML integers");

    const explicit = sdk.session({ id: "explicit", sampleRateHz: 48_000 }).source("voice", { channels: 1, frames: 1 })
      .track("mono", { source: "voice" })
      .submix("bus")
      .output("print")
      .route({ id: "mono-bus", source: { kind: "track", trackId: "mono", tap: "post_fader" }, destination: { kind: "submix_input", submixId: "bus" } })
      .route({ id: "bus-print", source: { kind: "submix_output", submixId: "bus" }, destination: { kind: "output_input", outputId: "print" } })
      .build();
    assert.deepEqual(explicit.json.outputs, [{ id: "print" }], "explicit graph declaration suppresses convenience output");
    assert.equal(explicit.json.routes.length, 2);

    const lanes = sdk.effect("miso.compressor", { threshold: [-24, -12] }, { slotId: "lane-vca", channel: "left", sidechain: { source: { kind: "track", trackId: "mono", tap: "post_fader" }, portId: "detector" } });
    const lanePlan = sdk.session({ id: "lane", sampleRateHz: 48_000 }).source("voice", { channels: 1, frames: 1 })
      .track("mono", { source: "voice", dynamic: [lanes] }).build();
    const laneEffect = lanePlan.json.tracks[0].dynamic.effects[0];
    assert.deepEqual(laneEffect.params.map((item) => item.channel), ["left", "right"], "tuple PerLane form must lower in engine channel order");
    assert.equal(laneEffect.sidechain.kind, "routed");

    const fromBits = (bits) => { const bytes = new ArrayBuffer(4); const view = new DataView(bytes); view.setUint32(0, bits, true); return view.getFloat32(0, true); };
    const floatPlan = sdk.session({ id: "float", sampleRateHz: 48_000 }).source("voice", { channels: 1, frames: 1 })
      .track("mono", { source: "voice", pan: { matrix: { ll: 0.1, lr: fromBits(0x15ae43fd), rl: fromBits(0x95ae43fd), rr: 1 } } }).build();
    assert.match(floatPlan.toml, /ll = 0.1, lr = 0.00000000000000000000000007038530691851209, rl = -0.00000000000000000000000007038530691851209, rr = 1.0/);

    const exponentPlan = sdk.session({ id: "negative-exponent", sampleRateHz: 48_000 }).source("voice", { channels: 1, frames: 1 })
      .track("mono", { source: "voice" }).output("print")
      .route({ id: "mono-print", source: { kind: "track", trackId: "mono", tap: "post_matrix" }, destination: { kind: "output_input", outputId: "print" }, gainDb: -1e-7 })
      .build();
    assert.match(exponentPlan.toml, /gain_db = -0\.0000001/, "negative exponent expansion must preserve the exact f32 value");

    const negativeZeroPlan = sdk.session({ id: "negative-zero", sampleRateHz: 48_000 }).source("voice", { channels: 1, frames: 1 })
      .track("mono", { source: "voice", pan: { matrix: { ll: -0, lr: 0, rl: 0, rr: 1 } } }).build();
    assert.equal(sdk.SessionPlan.fromJson(negativeZeroPlan.json).toml, negativeZeroPlan.toml, "fromJson must preserve signed zero and canonical bytes");

    const longestTrack = `t${"a".repeat(126)}`;
    const maximumIdPlan = sdk.session({ id: "max-id", sampleRateHz: 48_000 }).source("voice", { channels: 1, frames: 1 }).track(longestTrack, { source: "voice" }).build();
    assert.equal(maximumIdPlan.json.routes[0].id, "auto-route-1", "convenience route IDs must not derive from maximum-length track IDs");

    const nonPowerOfTwo = sdk.session({ id: "quantum", sampleRateHz: 48_000, quantumFrames: 127 }).build();
    assert.equal(nonPowerOfTwo.json.quantum_frames, 127, "Session V1 requires a nonzero quantum, not a power of two");
  });
}

async function selfTest() {
  await check();
  await withModule(async (sdk) => {
    assert.throws(
      () => sdk.effect("miso.compressor", { ratio: 20.0001 }),
      (error) => error instanceof sdk.MisoSessionError && error.path === "effect.parameters.ratio" && error.descriptor?.name === "ratio",
      "metadata outside-domain parameter must fail locally during build",
    );
    assert.throws(
      () => sdk.session({ id: "red", sampleRateHz: 48_000 }).source("voice", { channels: 1, frames: 1 }).track("lead", { source: "voice", dynamic: [sdk.effect("miso.compressor", { ratio: 20.0001 })] }).build(),
      (error) => error instanceof sdk.MisoSessionError,
      "the deliberate generated-domain red mutation must never produce a Session V1 plan",
    );
    assert.throws(
      () => sdk.effect("miso.compressor", { threshold: Number.MAX_VALUE }),
      (error) => error instanceof sdk.MisoSessionError && error.path === "effect.parameters.threshold",
      "finite f64 values which overflow f32 must be rejected",
    );
    assert.throws(
      () => sdk.session({ id: "source-red", sampleRateHz: 48_000 }).source("left", { channels: 1, frames: 1 }).source("right", { channels: 1, frames: 1 }).track("track", { source: { left: ["left", 0], right: ["right", 0] } }),
      (error) => error instanceof sdk.MisoSessionError && error.path === "track.source",
      "dual lane convenience cannot invent a second Session V1 source_id",
    );
    assert.throws(
      () => sdk.session({ id: "channel-red", sampleRateHz: 48_000 }).source("voice", { channels: 1, frames: 1 }).track("track", { source: { left: ["voice", 1], right: ["voice", 0] } }),
      (error) => error instanceof sdk.MisoSessionError && error.path === "track.source.left[1]",
      "source channel indexes must be in bounds",
    );
    assert.throws(
      () => sdk.session({ id: "auto-red", sampleRateHz: 48_000 }).source("voice", { channels: 1, frames: 1 }).track("track", { source: "voice", dynamic: [sdk.effect("miso.compressor", { threshold: -18 }, { slotId: "vca" })] }).automate({ id: "bad-time", target: { trackId: "track", rack: "dynamic", slotId: "vca", parameter: "threshold", channel: "both" }, segments: [{ shape: "linear", startSample: 0n, endSample: 5n, startValue: -18, endValue: -12 }, { shape: "linear", startSample: 4n, endSample: 6n, startValue: -12, endValue: -10 }] }),
      (error) => error instanceof sdk.MisoSessionError && error.path === "automation.segments[1].startSample",
      "overlapping automation segments must fail locally",
    );
    assert.throws(
      () => sdk.session({ id: "u64-red", sampleRateHz: 48_000 }).source("voice", { channels: 1, frames: 1 }).track("track", { source: "voice", dynamic: [sdk.effect("miso.compressor", { threshold: -18 }, { slotId: "vca" })] }).automate({ id: "too-large", target: { trackId: "track", rack: "dynamic", slotId: "vca", parameter: "threshold", channel: "both" }, segments: [{ shape: "linear", startSample: 0n, endSample: 9_223_372_036_854_775_808n, startValue: -18, endValue: -12 }] }),
      (error) => error instanceof sdk.MisoSessionError && error.path === "automation.segments[0].endSample",
      "automation times above TOML i64 must be rejected",
    );
    assert.throws(
      () => sdk.session({ id: "main-collision", sampleRateHz: 48_000 }).source("voice", { channels: 1, frames: 1 }).track("main", { source: "voice" }).build(),
      (error) => error instanceof sdk.MisoSessionError,
      "the synthesized main output must not collide with a track ID",
    );
    for (const [label, spec] of [
      ["zero source frames", { channels: 1, frames: 0 }],
      ["empty source identity", { channels: 1, frames: 1, identity: "" }],
      ["empty source locator", { channels: 1, frames: 1, locator: "" }],
    ]) {
      assert.throws(
        () => sdk.session({ id: `source-${label.replaceAll(" ", "-")}`, sampleRateHz: 48_000 }).source("voice", spec).build(),
        (error) => error instanceof sdk.MisoSessionError,
        `the deliberate ${label} mutation must fail locally`,
      );
    }
    assert.throws(
      () => sdk.session({ id: "pan-red", sampleRateHz: 48_000 }).source("voice", { channels: 1, frames: 1 }).track("track", { source: "voice", pan: { left: 2, right: 0 } }).build(),
      (error) => error instanceof sdk.MisoSessionError,
      "pan values outside the Session V1 [-1,1] domain must fail locally",
    );
    const missingSidechain = sdk.effect("miso.compressor", { threshold: -18 }, {
      slotId: "vca", sidechain: { source: { kind: "track", trackId: "missing", tap: "post_fader" }, portId: "detector" },
    });
    assert.throws(
      () => sdk.session({ id: "sidechain-red", sampleRateHz: 48_000 }).source("voice", { channels: 1, frames: 1 }).track("track", { source: "voice", dynamic: [missingSidechain] }).build(),
      (error) => error instanceof sdk.MisoSessionError,
      "a routed sidechain must reference a declared graph entity",
    );
  });
}

if (process.argv.length === 2) {
  await check();
  console.log("SDK Session V1 builder check passed");
} else if (process.argv.length === 3 && process.argv[2] === "--self-test") {
  await selfTest();
  console.log("SDK Session V1 builder self-test passed (metadata red mutation caught)");
} else {
  throw new Error("usage: node sdk/test/builder.mjs [--self-test]");
}
