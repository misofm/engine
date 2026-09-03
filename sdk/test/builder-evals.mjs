/**
 * Issue #243 eval 5 and adopted-ruling finding 6: the Session V1 builder.
 *
 * The boot evals prove what the engine does with bytes, and deliberately build their fixtures by
 * string concatenation so that they test the engine rather than the SDK. This file is the other
 * half: it tests the *builder*, and it does so by handing the real engine what the builder wrote.
 * A builder eval that only compared the builder's output to the builder's own expectations would
 * prove the two halves of one file agree with each other.
 *
 * Each test names the red mutation that makes it fail, because a probe nobody has seen fail is a
 * probe nobody should trust.
 */

import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { before, describe, test } from "node:test";

import { CATALOG } from "../src/generated/catalog.ts";
import { MisoEngineAsset } from "../src/core/asset.ts";
import { MisoUsageError } from "../src/core/errors.ts";
import {
  assertSameSession,
  effect,
  session,
} from "../src/core/session.ts";
import { writeCanonicalSessionDocument } from "../src/internal/session-json.ts";
import { createOfflineEngine, validate } from "../src/headless/engine.ts";
import { moduleBytes } from "./support.mjs";

const CONTENT_A = `sha256:${"0".repeat(64)}`;
const CONTENT_B = `sha256:2a97516c354b68848cdbd8f54a226a0a55b21ed138e207ad6c5cbb9c00aa5aea`;

let asset;

before(async () => {
  if (process.env.MISO_ENGINE_SDK_SKIP_ASSET === "1") return;
  asset = await MisoEngineAsset.load(await moduleBytes());
});

/** A complete one-track session, parameterised over the things the evals need to vary. */
function oneTrack(options = {}) {
  const {
    id = "builder.eval",
    sampleRateHz = 48_000,
    quantumFrames = 128,
    channels = 2,
    bitDepth = 24,
    frames = 4_800,
    content = CONTENT_A,
    sourceId = "stem",
    trackId = "t",
    builtins = {},
    fader = {},
    pan = { left: -1, right: 1 },
    racks = {},
    gainDb = 0,
  } = options;
  return session({ id, sampleRateHz, quantumFrames, revision: 1 })
    .source(sourceId, { channels, bitDepth, frames, content })
    .track(trackId, { source: sourceId, builtins, fader, pan, ...racks })
    .output("out")
    .route({
      id: "main",
      source: { kind: "track", trackId, tap: "post_matrix" },
      destination: { kind: "output_input", outputId: "out" },
      gainDb,
    });
}

/** A JSON-shaped deep copy, so a test can mutate a model the builder froze. */
function mutableModel(model) {
  return JSON.parse(JSON.stringify(model));
}

/** Deliberately destroy insertion order without changing any normalized value. */
function reverseConstructionOrder(value) {
  if (Array.isArray(value)) return value.map(reverseConstructionOrder);
  if (value === null || typeof value !== "object") return value;
  return Object.fromEntries(
    Object.entries(value).reverse().map(([key, child]) => [key, reverseConstructionOrder(child)]),
  );
}

describe("eval 5 -- the plan-equality gate", () => {
  test("the gate is green across a corpus of built sessions", () => {
    // The corpus deliberately reaches every root key: sources of both channel counts and all
    // three bit depths, a submix, two racks of effects with per-lane and shared parameters, a
    // routed sidechain, a matrix track, and both automation racks.
    const corpus = [
      () => oneTrack(),
      () => oneTrack({ bitDepth: 16, channels: 1, pan: { left: 0, right: 0 } }),
      () => oneTrack({ bitDepth: "32f", sampleRateHz: 96_000, quantumFrames: 127 }),
      () => oneTrack({ builtins: { left: { trimDb: -3, hpfHz: 80 }, right: { trimDb: 1.5, delaySamples: 128 } } }),
      () => oneTrack({ pan: { matrix: { ll: 1, lr: 0, rl: 0, rr: -1 }, smoothingSamples: 64 } }),
      () => oneTrack({ racks: { dynamic: [effect("miso.compressor", { threshold: -18, ratio: 4 })] } }),
      () => richSession(),
    ];
    for (const build of corpus) {
      // Two independent evaluations of the same declaration must normalize identically, which is
      // the weakest thing the gate has to be true for and the one that catches nondeterminism --
      // an iteration order, a `Date`, a counter left in the builder.
      assertSameSession(build(), build());
    }
  });

  test("declaration order does not change the normalized document", () => {
    // Canonical entity sets sort by ID, so two authors who declare the same session in different
    // orders must produce the same document. Red mutation: drop the `.sort(byId)` in `normalize`.
    const forward = session({ id: "order", sampleRateHz: 48_000 })
      .source("a", { channels: 1, bitDepth: 16, frames: 480, content: CONTENT_A })
      .source("b", { channels: 2, bitDepth: "32f", frames: 480, content: CONTENT_B })
      .track("x", { source: "a" })
      .track("y", { source: "b" })
      .output("o1")
      .output("o2")
      .route({
        id: "r2",
        source: { kind: "track", trackId: "y", tap: "post_fader" },
        destination: { kind: "output_input", outputId: "o2" },
      })
      .route({
        id: "r1",
        source: { kind: "track", trackId: "x", tap: "post_matrix" },
        destination: { kind: "output_input", outputId: "o1" },
      });
    const reverse = session({ id: "order", sampleRateHz: 48_000 })
      .source("b", { channels: 2, bitDepth: "32f", frames: 480, content: CONTENT_B })
      .source("a", { channels: 1, bitDepth: 16, frames: 480, content: CONTENT_A })
      .track("y", { source: "b" })
      .track("x", { source: "a" })
      .output("o2")
      .output("o1")
      .route({
        id: "r1",
        source: { kind: "track", trackId: "x", tap: "post_matrix" },
        destination: { kind: "output_input", outputId: "o1" },
      })
      .route({
        id: "r2",
        source: { kind: "track", trackId: "y", tap: "post_fader" },
        destination: { kind: "output_input", outputId: "o2" },
      });
    assertSameSession(forward, reverse);
    assert.equal(forward.toJson(), reverse.toJson());
  });

  test("RED MUTATION: one source's wrong bit depth is caught, and the message names the path", () => {
    // This is the mutation the gate exists for. A builder that wrote a source's depth from the
    // wrong field -- the exact class of bug #241 created when it deleted four source keys and
    // added one -- produces a session that still parses, still boots, and renders the wrong
    // preimage. The gate refuses it and says where.
    //
    // Red mutation of the *gate*: compare only `id`, `channels` and `frames`, as the pre-#241
    // source rows did. This test then goes green while the documents genuinely differ.
    const declared = oneTrack({ bitDepth: 24 });
    const wrong = oneTrack({ bitDepth: 16 });
    assert.throws(
      () => assertSameSession(declared, wrong),
      (error) => {
        assert.ok(error instanceof MisoUsageError, "the gate refuses with a usage error");
        assert.match(error.message, /sources\[0\]\.bit_depth/);
        assert.match(error.message, /24/);
        assert.match(error.message, /16/);
        return true;
      },
    );
  });

  test("the gate sees the bit-depth TOKEN, not just a number", () => {
    // Finding 6's teeth. `16` and `"16"` are the same digits and different tokens; only one of
    // them is in the schema's set. A gate that stringified before comparing would call them equal.
    // Red mutation: compare with `String(left) === String(right)` instead of `Object.is`.
    const model = mutableModel(oneTrack({ bitDepth: 16 }).toJSON());
    model.sources[0].bit_depth = "16";
    assert.throws(
      () => assertSameSession(oneTrack({ bitDepth: 16 }), model),
      (error) => {
        assert.match(error.message, /sources\[0\]\.bit_depth/);
        return true;
      },
    );
  });

  test("the gate reports the FIRST difference, deep inside a track", () => {
    // Red mutation: return the last difference, or report only that the documents differ.
    const left = oneTrack({ builtins: { trimDb: -3 } });
    const right = oneTrack({ builtins: { trimDb: -3.5 } });
    assert.throws(
      () => assertSameSession(left, right),
      (error) => {
        assert.match(error.message, /tracks\[0\]\.builtins\.left\.trim_db/);
        return true;
      },
    );
  });

  test("the gate refuses a differing entity count at the array's own path", () => {
    const one = oneTrack();
    const two = oneTrack().source("extra", {
      channels: 1,
      bitDepth: "32f",
      frames: 480,
      content: CONTENT_B,
    });
    assert.throws(() => assertSameSession(one, two), /sources: an array of 1 !== an array of 2/);
  });
});

describe("finding 6 -- the full bit-depth token set", () => {
  test('16 and 24 emit bare integers; "32f" emits a quoted string', () => {
    // Adopted-ruling finding 6 sentence 1, and the schema's "the canonical writer preserves those
    // spellings". Red mutation: normalize the depth to a number, or quote all three.
    assert.match(oneTrack({ bitDepth: 16 }).toJson(), /"bit_depth": 16,/);
    assert.match(oneTrack({ bitDepth: 24 }).toJson(), /"bit_depth": 24,/);
    assert.match(oneTrack({ bitDepth: "32f" }).toJson(), /"bit_depth": "32f",/);
    assert.doesNotMatch(oneTrack({ bitDepth: 16 }).toJson(), /"bit_depth": "16"/);
  });

  test("toJSON round-trips each token exactly, type included", () => {
    for (const depth of [16, 24, "32f"]) {
      const row = oneTrack({ bitDepth: depth }).toJSON().sources[0];
      assert.equal(row.bit_depth, depth);
      assert.equal(typeof row.bit_depth, typeof depth);
    }
  });

  test("a depth outside the token set is refused at its own path", () => {
    assert.throws(
      () => oneTrack({ bitDepth: 32 }),
      (error) => {
        assert.ok(error instanceof MisoUsageError);
        assert.match(error.message, /source\("stem"\)\.bitDepth/);
        return true;
      },
    );
    assert.throws(() => oneTrack({ bitDepth: "24" }), MisoUsageError);
    assert.throws(() => oneTrack({ bitDepth: "f32" }), MisoUsageError);
  });
});

describe("the most important eval -- what the builder writes, the engine boots", () => {
  /** Boot a built session and assert the engine's own answer matches what was declared. */
  async function bootAndCompare(built, expected) {
    const engine = await createOfflineEngine(built, { asset });
    try {
      const shape = engine.shape();
      assert.equal(shape.sampleRateHz, expected.sampleRateHz);
      assert.equal(shape.quantumFrames, expected.quantumFrames);
      assert.deepEqual([...shape.tracks].sort(), [...expected.tracks].sort());
      const bySourceId = new Map(shape.sources.map((row) => [row.id, row]));
      const model = built.toJSON();
      assert.equal(shape.sources.length, model.sources.length);
      for (const source of model.sources) {
        const compiled = bySourceId.get(source.id);
        assert.ok(compiled !== undefined, `the engine compiled no source '${source.id}'`);
        assert.equal(compiled.channels, source.channels);
        assert.equal(compiled.frames, BigInt(source.frames));
      }
      assert.equal(engine.state(), "ready");
      // Rendering is the last thing a merely-parseable document would survive.
      const block = engine.render();
      assert.equal(block.left.length, expected.quantumFrames);
      assert.equal(block.right.length, expected.quantumFrames);
      return shape;
    } finally {
      engine.dispose();
    }
  }

  for (const bitDepth of [16, 24, "32f"]) {
    test(`a ${bitDepth}-bit document boots and introspects as declared`, async () => {
      // Red mutation: emit `bit_depth = 32f` unquoted, or quote `16`. Either is a parse refusal
      // from the real engine, which is exactly the point of booting rather than string-matching.
      const built = oneTrack({ bitDepth, id: `depth.${String(bitDepth).toLowerCase()}` });
      await bootAndCompare(built, { sampleRateHz: 48_000, quantumFrames: 128, tracks: ["t"] });
    });
  }

  test("48000/128 and 96000/127 both boot, and the ring follows the published rule", async () => {
    // 96 kHz at a 127-frame quantum is the shape that structurally could not boot under the
    // pre-boot-v1 SDK's fabricated 1024-frame ring. A builder whose output only ever works at
    // 48 kHz/128 would pass every other test in this file.
    await bootAndCompare(oneTrack({ id: "rate.48k" }), {
      sampleRateHz: 48_000,
      quantumFrames: 128,
      tracks: ["t"],
    });
    const shape = await bootAndCompare(
      oneTrack({ id: "rate.96k", sampleRateHz: 96_000, quantumFrames: 127, frames: 9_600 }),
      { sampleRateHz: 96_000, quantumFrames: 127, tracks: ["t"] },
    );
    assert.equal(shape.sourceRingFrames, 9_906);
    assert.equal(shape.sourceRingFrames % shape.quantumFrames, 0);
  });

  test("every launch rate boots", async () => {
    for (const sampleRateHz of [44_100, 48_000, 88_200, 96_000]) {
      const result = await validate(oneTrack({ id: "rate.sweep", sampleRateHz }), { asset });
      assert.equal(result.ok, true, `${sampleRateHz} Hz must boot`);
      assert.equal(result.shape.sampleRateHz, sampleRateHz);
    }
  });

  test("a rich session -- effects, submix, sidechain, matrix, automation -- boots", async () => {
    // The corpus session in one boot. Red mutation: any per-record key-order or key-name drift
    // from `visit.rs` refuses here as `schema.unknown_field`, with the offending path named.
    const built = richSession();
    const engine = await createOfflineEngine(built, { asset });
    try {
      const shape = engine.shape();
      assert.equal(shape.sampleRateHz, 48_000);
      assert.deepEqual([...shape.tracks].sort(), ["bass", "vocal"]);
      assert.equal(engine.render().left.length, 128);
    } finally {
      engine.dispose();
    }
  });

  test("the engine accepts the builder's document through the toJson() surface too", async () => {
    // `createOfflineEngine` takes a builder directly because a builder is a `{ toJson() }`. The
    // text path must be the same bytes, not a second encoder.
    const built = oneTrack({ id: "surface" });
    const viaObject = await validate(built, { asset });
    const viaText = await validate(built.toJson(), { asset });
    assert.equal(viaObject.ok, true);
    assert.equal(viaText.ok, true);
    assert.deepEqual(viaText.shape.tracks, viaObject.shape.tracks);
  });
});

describe("validation refusals name the offending path", () => {
  test("an unsupported sample rate is refused before a document exists", () => {
    // Red mutation: accept the rate and let the engine refuse it. The engine's refusal is correct
    // but arrives after a boot; the builder knows the launch set and says so at the call.
    assert.throws(
      () => session({ id: "bad.rate", sampleRateHz: 44_056 }),
      (error) => {
        assert.ok(error instanceof MisoUsageError);
        assert.match(error.message, /session\(\)\.sampleRateHz/);
        assert.match(error.message, /44100, 48000, 88200, 96000/);
        return true;
      },
    );
  });

  test("a duplicate source ID is refused at that source's path", () => {
    assert.throws(
      () =>
        session({ id: "dupe", sampleRateHz: 48_000 })
          .source("stem", { channels: 1, bitDepth: 16, frames: 480, content: CONTENT_A })
          .source("stem", { channels: 2, bitDepth: 24, frames: 480, content: CONTENT_B }),
      (error) => {
        assert.ok(error instanceof MisoUsageError);
        assert.match(error.message, /source\("stem"\)\.id/);
        return true;
      },
    );
  });

  test("a malformed content digest is refused at its own path", () => {
    // The schema is exact: `sha256:` and 64 lowercase hex digits. Red mutation: relax the regex to
    // a prefix check, and every one of these becomes a boot-time refusal instead.
    for (const content of [
      "sha256:beef",
      `sha256:${"0".repeat(63)}`,
      `sha256:${"0".repeat(65)}`,
      `sha256:${"A".repeat(64)}`,
      `sha1:${"0".repeat(64)}`,
      "0".repeat(64),
    ]) {
      assert.throws(
        () => oneTrack({ content }),
        (error) => {
          assert.ok(error instanceof MisoUsageError, `accepted ${content}`);
          assert.match(error.message, /source\("stem"\)\.content/);
          assert.match(error.message, /sha256:\[0-9a-f\]\{64\}/);
          return true;
        },
      );
    }
  });

  test("a track naming an undeclared source is refused at the track's source path", () => {
    assert.throws(
      () =>
        session({ id: "orphan", sampleRateHz: 48_000 })
          .source("stem", { channels: 2, bitDepth: 24, frames: 480, content: CONTENT_A })
          .track("t", { source: "ghost" }),
      (error) => {
        assert.ok(error instanceof MisoUsageError);
        assert.match(error.message, /track\("t"\)\.source/);
        assert.match(error.message, /'ghost' is not a declared source/);
        return true;
      },
    );
  });

  test("a lane index beyond the source's channel count is refused", () => {
    assert.throws(
      () =>
        session({ id: "lane", sampleRateHz: 48_000 })
          .source("mono", { channels: 1, bitDepth: 16, frames: 480, content: CONTENT_A })
          .track("t", { source: { id: "mono", left: 0, right: 1 } }),
      /track\("t"\)\.source\.right/,
    );
  });

  test("a route naming an undeclared endpoint is refused at that endpoint's path", () => {
    const base = session({ id: "route.ref", sampleRateHz: 48_000 })
      .source("stem", { channels: 2, bitDepth: 24, frames: 480, content: CONTENT_A })
      .track("t", { source: "stem" })
      .output("out");
    assert.throws(
      () =>
        base.route({
          id: "r",
          source: { kind: "track", trackId: "nope", tap: "post_matrix" },
          destination: { kind: "output_input", outputId: "out" },
        }),
      /route\("r"\)\.source\.trackId/,
    );
    assert.throws(
      () =>
        base.route({
          id: "r",
          source: { kind: "track", trackId: "t", tap: "post_matrix" },
          destination: { kind: "output_input", outputId: "nope" },
        }),
      /route\("r"\)\.destination\.outputId/,
    );
  });

  test("the ID namespaces are enforced in both directions", () => {
    const base = session({ id: "ns", sampleRateHz: 48_000 })
      .source("stem", { channels: 2, bitDepth: 24, frames: 480, content: CONTENT_A })
      .track("t", { source: "stem" });
    assert.throws(() => base.submix("t"), /submix\("t"\)\.id/);
    assert.throws(() => base.output("t"), /output\("t"\)\.id/);
    // Sources have their own namespace, so a source may legally share a track's ID.
    assert.doesNotThrow(() =>
      base.source("t", { channels: 1, bitDepth: 16, frames: 480, content: CONTENT_B }));
  });

  test("a prepared-only builtin is refused as an automation target", () => {
    // The schema refuses `hpf_hz`, `lpf_hz` and `delay_samples` because a span addressed at a
    // parameter with no post-preparation write path could only ever be inert. Red mutation: admit
    // every builtin row, and the engine refuses the document instead -- later, and less clearly.
    const base = session({ id: "auto", sampleRateHz: 48_000 })
      .source("stem", { channels: 2, bitDepth: 24, frames: 480, content: CONTENT_A })
      .track("t", { source: "stem" });
    for (const parameter of ["hpf_hz", "lpf_hz", "delay_samples"]) {
      assert.throws(
        () =>
          base.automation({
            id: "a",
            target: { trackId: "t", rack: "builtins", parameter, channel: "left" },
            segments: [{ shape: "linear", startSample: 0n, endSample: 480n, startValue: 0, endValue: 1 }],
          }),
        (error) => {
          assert.ok(error instanceof MisoUsageError);
          assert.match(error.message, /automation\("a"\)\.target\.parameter/);
          assert.match(error.message, /prepared-only/);
          return true;
        },
      );
    }
    // A shared matrix coefficient is addressed as `both` and nothing else.
    assert.throws(
      () =>
        base.automation({
          id: "a",
          target: { trackId: "t", rack: "builtins", parameter: "matrix_ll", channel: "left" },
          segments: [{ shape: "linear", startSample: 0n, endSample: 480n, startValue: 0, endValue: 1 }],
        }),
      /automation\("a"\)\.target\.channel/,
    );
  });

  test("an effect parameter outside its catalog domain is refused by name", () => {
    assert.throws(
      () => effect("miso.compressor", { ratio: 100 }),
      (error) => {
        assert.ok(error instanceof MisoUsageError);
        assert.match(error.message, /effect\("miso\.compressor"\)\.parameters\.ratio/);
        return true;
      },
    );
    assert.throws(() => effect("miso.compressor", { nope: 1 }), /has no parameter 'nope'/);
    assert.throws(() => effect("miso.not-an-effect"), /unknown native effect/);
  });
});

describe("issue #278 -- the port table is enforced, not documented", () => {
  // Before the catalog carried `ports`, `portId` was the one session field a builder could not
  // check: a misspelling parsed, validated, compiled, and failed at PREPARATION with
  // `effect.sidechain.unknown_port`. The engine's refusal has not moved -- the last test in this
  // block proves it still fires on a document that never went through `effect()` -- and what
  // follows it is the authoring-time half that now stands in front of it.

  const source = { kind: "track", trackId: "bass", tap: "post_fader" };

  test("the declared sidechain port is accepted, and it is the descriptor's own", () => {
    for (const id of ["miso.compressor", "miso.gate-expander"]) {
      const decl = effect(id, {}, { sidechain: { source, portId: "sidechain-in" } });
      assert.equal(decl.options.sidechain.portId, "sidechain-in");
      // The name came from the catalog, not from this file: the row it matches is right there.
      const descriptor = CATALOG.effects.find((candidate) => candidate.id === id);
      assert.deepEqual(
        descriptor.ports.filter((port) => port.roleName === "sidechainInput"),
        [{
          id: "sidechain-in",
          role: 3,
          roleName: "sidechainInput",
          required: false,
          layout: 1,
          layoutName: "dualMonoPlanar",
        }],
      );
    }
  });

  test("a misspelled port is refused at authoring time, naming the candidates", () => {
    // Red mutation: delete the `sidechainPort()` call in `effect()` -> this throws nothing, the
    // misspelling survives to `toJson()`, and the session only fails at boot.
    assert.throws(
      () => effect("miso.compressor", {}, { sidechain: { source, portId: "sidechan-in" } }),
      (error) => {
        assert.ok(error instanceof MisoUsageError);
        assert.match(error.message, /effect\("miso\.compressor"\)\.sidechain\.portId/);
        assert.match(error.message, /has no port 'sidechan-in'/);
        assert.match(error.message, /sidechain inputs are 'sidechain-in'/);
        return true;
      },
    );
  });

  test("a port that exists but is not a sidechain input is refused as what it is", () => {
    assert.throws(
      () => effect("miso.compressor", {}, { sidechain: { source, portId: "main-in" } }),
      (error) => {
        assert.ok(error instanceof MisoUsageError);
        assert.match(error.message, /'main-in' is miso\.compressor's mainInput port/);
        assert.match(error.message, /not a sidechain input/);
        return true;
      },
    );
  });

  test("an effect that declares no sidechain input refuses every port", () => {
    // Six of the eight launch effects are in this class. The type says `never` for `portId`, so a
    // caller who typechecks cannot write this at all; the runtime refusal is for the caller who
    // does not -- plain JS, or a `portId` that arrived as a `string` from JSON.
    const portless = CATALOG.effects
      .filter((entry) => !entry.ports.some((port) => port.roleName === "sidechainInput"))
      .map((entry) => entry.id);
    assert.deepEqual(portless, [
      "miso.delay",
      "miso.multiband-compressor",
      "miso.parametric-eq",
      "miso.soft-clip",
      "miso.transient-shaper",
      "miso.true-peak-limiter",
    ]);
    for (const id of portless) {
      for (const portId of ["sidechain-in", "main-in", "anything"]) {
        assert.throws(
          () => effect(id, {}, { sidechain: { source, portId } }),
          (error) => {
            assert.ok(error instanceof MisoUsageError);
            assert.match(error.message, /declares no sidechain input port/);
            assert.match(error.message, /'main-in' and 'main-out'/);
            return true;
          },
          `${id} accepted a sidechain on '${portId}'`,
        );
      }
    }
  });

  test("every catalog port row is a launch descriptor row, and there are eighteen", () => {
    // The catalog is the SDK's only authority on ports, so the shape it publishes is worth
    // stating once here: eight effects, each with exactly one main input and one main output,
    // and exactly two of them with an optional `sidechain-in`.
    const rows = CATALOG.effects.flatMap((entry) => entry.ports);
    assert.equal(rows.length, 18);
    assert.equal(rows.filter((port) => port.roleName === "mainInput").length, 8);
    assert.equal(rows.filter((port) => port.roleName === "mainOutput").length, 8);
    assert.equal(rows.filter((port) => port.roleName === "sidechainInput").length, 2);
    assert.ok(rows.every((port) => port.layoutName === "dualMonoPlanar"));
    assert.ok(rows.every((port) => port.required === (port.roleName !== "sidechainInput")));
  });

  test("the engine still refuses an unknown port at boot -- this replaced nothing", async () => {
    // The authoring check is added IN FRONT OF the engine's, never instead of it. This document is
    // hand-written precisely so it never passes through `effect()`, which is the only way to prove
    // the boot-time refusal is still there.
    //
    // Red mutation: delete the `effect.sidechain.unknown_port` leg in
    // `crates/effect-compiler/src/prepare.rs` -> this boots and the test fails.
    const document = oneTrack({
      racks: { dynamic: [effect("miso.compressor", { threshold: -18 })] },
    }).toJson().replace(
      '"sidechain": {\n              "kind": "none"\n            }',
      '"sidechain": {"kind":"routed","source":{"kind":"track","track_id":"t",'
        + '"tap":"post_fader"},"port_id":"not-a-port"}',
    );
    assert.match(document, /"port_id":"not-a-port"/);
    const outcome = await validate(document, { asset });
    assert.equal(outcome.ok, false);
    assert.ok(
      outcome.diagnostics.some((row) => row.code === "effect.sidechain.unknown_port"),
      `expected effect.sidechain.unknown_port, got ${JSON.stringify(outcome.diagnostics)}`,
    );
  });
});

describe("canonical float spellings", () => {
  test("integral values gain .0 so they stay schema floats", () => {
    // Red mutation: emit `String(value)`. `"trim_db": 0` loses the canonical float spelling.
    // refuses the document at that leaf, so this is a real refusal and not a cosmetic one.
    const json = oneTrack({
      builtins: { trimDb: 0, hpfHz: 20, lpfHz: 20_000 },
      fader: { leftDb: -6, rightDb: 12 },
      pan: { left: -1, right: 1, smoothingSamples: 16 },
    }).toJson();
    assert.match(json, /"trim_db": 0\.0/);
    assert.match(json, /"hpf_hz": 20\.0/);
    assert.match(json, /"lpf_hz": 20000\.0/);
    assert.match(json, /"left_db": -6\.0/);
    assert.match(json, /"right_db": 12\.0/);
    assert.match(json, /"left": -1\.0,[\s\S]*"right": 1\.0,[\s\S]*"smoothing_samples": 16/);
    // Integers stay integers: the smoothing count above, and these.
    assert.match(json, /"sample_rate_hz": 48000/);
    assert.match(json, /"delay_samples": 0/);
    assert.match(json, /"frames": "4800"/);
  });

  test("negative zero is preserved exactly as -0.0", () => {
    // `String(-0)` is `"0"`, so every shortest-digit search agrees with the wrong answer here.
    // Red mutation: drop the `Object.is(value, -0)` branch in `canonicalFloat`.
    const json = oneTrack({ builtins: { trimDb: -0 }, fader: { leftDb: -0 }, gainDb: -0 }).toJson();
    assert.match(json, /"trim_db": -0\.0/);
    assert.match(json, /"left_db": -0\.0/);
    assert.match(json, /"gain_db": -0\.0/);
    // And the model keeps the sign, which is why the gate can see it.
    const model = oneTrack({ builtins: { trimDb: -0 } }).toJSON();
    assert.ok(Object.is(model.tracks[0].builtins.left.trim_db, -0));
    assert.throws(
      () => assertSameSession(oneTrack({ builtins: { trimDb: -0 } }), oneTrack({ builtins: { trimDb: 0 } })),
      /tracks\[0\]\.builtins\.left\.trim_db: -0 !== 0/,
    );
  });

  test("shortest f32 Display, with no exponent anywhere", () => {
    // Rust's float `Display` never uses exponent notation and the canonical writer inherits that,
    // so a spelling like `1e-10` is not merely unusual -- the schema's JSON subset excludes it.
    // Red mutation: return `toPrecision` output without expanding the exponent.
    const json = oneTrack({
      builtins: { trimDb: 1 / 3, hpfHz: 20.5 },
      fader: { leftDb: -3.5, rightDb: 0.1 },
    }).toJson();
    assert.doesNotMatch(json, /[eE][-+]?\d/);
    assert.match(json, /"trim_db": 0\.33333334/);
    assert.match(json, /"hpf_hz": 20\.5/);
    assert.match(json, /"left_db": -3\.5/);
    assert.match(json, /"right_db": 0\.1/);
  });

  test("a value below the exponent threshold is written out in full", () => {
    const json = oneTrack({
      racks: { simd1: [effect("miso.compressor", { mix: 1e-7 })] },
    }).toJson();
    assert.doesNotMatch(json, /[eE][-+]?\d/);
    assert.match(json, /"value": 0\.0000001/);
  });

  test("every durable u64 normalizes to a decimal string through u64::MAX", () => {
    const maximum = 18_446_744_073_709_551_615n;
    const built = session({ id: "u64.maximum", sampleRateHz: 48_000, revision: maximum })
      .source("stem", { channels: 1, bitDepth: 16, frames: maximum, content: CONTENT_A })
      .track("t", { source: "stem" })
      .output("out")
      .route({
        id: "main",
        source: { kind: "track", trackId: "t", tap: "post_matrix" },
        destination: { kind: "output_input", outputId: "out" },
      })
      .automation({
        id: "ride",
        target: { trackId: "t", rack: "builtins", parameter: "fader_db", channel: "both" },
        segments: [{ shape: "step", startSample: 0, endSample: maximum, startValue: 0, endValue: 0 }],
      });
    const model = built.toJSON();
    assert.equal(model.revision, maximum.toString());
    assert.equal(model.sources[0].frames, maximum.toString());
    assert.equal(model.automation[0].segments[0].start_sample, "0");
    assert.equal(model.automation[0].segments[0].end_sample, maximum.toString());
    assert.match(built.toJson(), /"revision": "18446744073709551615"/);
    assert.throws(() => session({ id: "overflow", sampleRateHz: 48_000, revision: maximum + 1n }), /u64::MAX/);
    assert.throws(() => session({ id: "unsafe", sampleRateHz: 48_000, revision: Number.MAX_SAFE_INTEGER + 1 }), /safe integer/);
  });

  test("every emitted float parses back to the same f32 the model holds", async () => {
    // The property the spelling rule exists for, stated the way the schema states it: a canonical
    // spelling must "preserve exact bits through f64-then-f32 conversion by external readers".
    // Every reader in JavaScript is such a reader -- `Number()` is f64 -- so this is the exact
    // round trip a host performs, checked leaf by leaf against the model's own values.
    //
    // Red mutation: emit `value.toPrecision(6)` unconditionally. `trim_db` goes red first.
    const built = oneTrack({
      builtins: { trimDb: 1 / 3, hpfHz: 20.5, lpfHz: 19_753.125 },
      fader: { leftDb: -3.5, rightDb: 0.1 },
      gainDb: -0.25,
    });
    const json = built.toJson();
    const model = built.toJSON();
    const lane = model.tracks[0].builtins.left;
    const pairs = [
      ["trim_db", lane.trim_db],
      ["hpf_hz", lane.hpf_hz],
      ["lpf_hz", lane.lpf_hz],
      ["left_db", model.tracks[0].fader.left_db],
      ["right_db", model.tracks[0].fader.right_db],
      ["gain_db", model.routes[0].gain_db],
    ];
    for (const [key, value] of pairs) {
      const spelling = json.match(new RegExp(`"${key}": (-?[0-9.]+)`))?.[1];
      assert.ok(spelling !== undefined, `no spelling emitted for ${key}`);
      assert.ok(
        Object.is(Math.fround(Number(spelling)), value),
        `${key} = ${spelling} reads back as ${Math.fround(Number(spelling))}, not ${value}`,
      );
    }
    // And the engine accepts the whole document, which is the only reader whose opinion binds.
    const result = await validate(built, { asset });
    assert.equal(result.ok, true);
  });

  test("the builder reproduces a checked-in canonical fixture byte for byte", async () => {
    // The strongest available statement that the builder's canonical text *is* the engine's:
    // `fixtures/session/v1/builtins-automation.json` was written by the engine's own canonical
    // writer, and this rebuilds it from the SDK. Red mutation: change the indent, the trailing
    // comma, the empty-array layout, the root key order or any per-record key order.
    const expected = await readFile(
      new URL("../../fixtures/session/v1/builtins-automation.json", import.meta.url),
      "utf8",
    );
    assert.equal(builtinsAutomationFixture().toJson(), expected);
  });

  test("schema key order, never object construction order, controls canonical bytes", async () => {
    // Red mutation: replace `checkedObjectOrder()` with `Object.entries(record)` in the writer.
    // Every record below was deliberately constructed backwards, including tagged variants.
    const built = richSession();
    const reordered = reverseConstructionOrder(built.toJSON());
    assert.notDeepEqual(Object.keys(reordered), Object.keys(built.toJSON()));
    assert.equal(writeCanonicalSessionDocument(reordered), built.toJson());
  });

  test("the bounded Rust-authority corpus matches the actual SDK writer", async () => {
    const manifest = JSON.parse(await readFile(
      new URL("../../fixtures/session-canonical/v1/canonical-writer-corpus.json", import.meta.url),
      "utf8",
    ));
    assert.equal(manifest.schema, "miso.session.canonical-writer-corpus.v1");

    for (const entry of manifest.documents) {
      const expected = entry.path === undefined
        ? entry.canonical
        : await readFile(new URL(`../../${entry.path}`, import.meta.url), "utf8");
      assert.equal(writeCanonicalSessionDocument(JSON.parse(expected)), expected, entry.id);
    }

    const full = JSON.parse(await readFile(
      new URL("../../fixtures/session/v1/canonical.json", import.meta.url),
      "utf8",
    ));
    const view = new DataView(new ArrayBuffer(4));
    for (const entry of manifest.f32) {
      view.setUint32(0, Number.parseInt(entry.bits, 16), true);
      const model = structuredClone(full);
      model.routes[0].gain_db = view.getFloat32(0, true);
      const line = writeCanonicalSessionDocument(model).split("\n")
        .find((candidate) => candidate.includes('"gain_db":'));
      assert.equal(line?.trim(), `"gain_db": ${entry.canonical}`, entry.id);
    }

    const minimal = JSON.parse(await readFile(
      new URL("../../fixtures/session/v1/canonical-minimal.json", import.meta.url),
      "utf8",
    ));
    for (const entry of manifest.strings) {
      const model = structuredClone(minimal);
      model.session_id = entry.value;
      const line = writeCanonicalSessionDocument(model).split("\n")
        .find((candidate) => candidate.includes('"session_id":'));
      assert.equal(line?.trim(), `"session_id": ${entry.canonical},`, entry.id);
    }
  });
});

/** The checked-in `builtins-automation.json` fixture, rebuilt through the SDK. */
function builtinsAutomationFixture() {
  const span = (shape, startSample, endSample, startValue, endValue) =>
    [{ shape, startSample, endSample, startValue, endValue }];
  return session({ id: "builtins-automation.session", sampleRateHz: 48_000, revision: 1 })
    .source("voice", { channels: 2, bitDepth: "32f", frames: 48_000, content: CONTENT_B })
    .track("vocal", {
      source: "voice",
      builtins: { polarityInvert: false, trimDb: 0, hpfHz: 20, lpfHz: 20_000, delaySamples: 0 },
      pan: { left: 1, right: 1, smoothingSamples: 16 },
    })
    .output("main-out")
    .route({
      id: "to-main",
      source: { kind: "track", trackId: "vocal", tap: "post_matrix" },
      destination: { kind: "output_input", outputId: "main-out" },
    })
    .automation({
      id: "fader-ride",
      target: { trackId: "vocal", rack: "builtins", parameter: "fader_db", channel: "both" },
      segments: span("linear", 0n, 960n, 0, -3),
    })
    .automation({
      id: "matrix-ll",
      target: { trackId: "vocal", rack: "builtins", parameter: "matrix_ll", channel: "both" },
      segments: span("linear", 0n, 960n, 1, 0.5),
    })
    .automation({
      id: "polarity-flip",
      target: { trackId: "vocal", rack: "builtins", parameter: "polarity_invert", channel: "right" },
      segments: span("step", 480n, 960n, 0, 1),
    })
    .automation({
      id: "trim-ride-left",
      target: { trackId: "vocal", rack: "builtins", parameter: "trim_db", channel: "left" },
      segments: span("linear", 0n, 480n, 0, -6),
    });
}

/** A session that reaches every root key and both automation racks. */
function richSession() {
  const compressor = effect(
    "miso.compressor",
    { threshold: -18, ratio: { left: 4, right: 6 }, makeup: 1.5 },
    {
      slotId: "comp",
      linkMode: "maximum",
      // A routed sidechain reuses the tagged route-source shape. Its REFERENT still cannot be
      // checked here -- `effect()` runs before any track exists -- which is why this rides in a
      // session that must boot. Its PORT now is: issue #278 published the port table, so
      // `effect()` resolves `portId` against `miso.compressor`'s own declared sidechain inputs
      // and the type only admits those names. See "the port table is enforced, not documented"
      // below for the enforcing half; the engine's `effect.sidechain.unknown_port` is unmoved and
      // is still what this session's boot proves.
      sidechain: {
        source: { kind: "track", trackId: "bass", tap: "post_fader" },
        portId: "sidechain-in",
      },
    },
  );
  const limiter = effect("miso.true-peak-limiter", { ceiling: -1 }, { slotId: "limit" });
  const eq = effect(
    "miso.parametric-eq",
    { "band-1-enabled": true, "band-1-gain": -2.5, "band-1-frequency": 120 },
    { slotId: "eq" },
  );
  return session({ id: "rich.session", sampleRateHz: 48_000, quantumFrames: 128, revision: 12 })
    .source("voice", { channels: 2, bitDepth: "32f", frames: 48_000, content: CONTENT_B })
    .source("di", { channels: 1, bitDepth: 24, frames: 48_000, content: CONTENT_A })
    .track("vocal", {
      source: "voice",
      builtins: { left: { trimDb: -1, hpfHz: 80, delaySamples: 24 }, right: { trimDb: -1, hpfHz: 80 } },
      fader: { leftDb: -3, rightDb: -3, leftMute: false, rightMute: false },
      pan: { left: -0.5, right: 0.5, smoothingSamples: 32 },
      simd1: [eq],
      dynamic: [compressor, limiter],
    })
    .track("bass", {
      source: { id: "di", left: 0, right: 0 },
      pan: { matrix: { ll: 1, lr: 0, rl: 0, rr: 1 }, smoothingSamples: 8 },
      simd2: [effect("miso.soft-clip", { drive: 3 })],
    })
    .submix("bus")
    .output("main-out")
    .route({
      id: "bass-bus",
      source: { kind: "track", trackId: "bass", tap: "post_fader" },
      destination: { kind: "submix_input", submixId: "bus" },
      gainDb: -2,
    })
    .route({
      id: "bus-main",
      source: { kind: "submix_output", submixId: "bus" },
      destination: { kind: "output_input", outputId: "main-out" },
    })
    .route({
      id: "vocal-main",
      source: { kind: "track", trackId: "vocal", tap: "post_matrix" },
      destination: { kind: "output_input", outputId: "main-out" },
      matrix: { ll: 1, lr: 0, rl: 0, rr: 1 },
    })
    .automation({
      id: "eq-sweep",
      target: {
        trackId: "vocal",
        rack: "simd1",
        slotId: "eq",
        parameter: "band-1-gain",
        channel: "both",
      },
      segments: [
        { shape: "linear", startSample: 0n, endSample: 480n, startValue: -2.5, endValue: 0 },
        { shape: "step", startSample: 480n, endSample: 960n, startValue: 0, endValue: 3 },
      ],
    })
    .automation({
      id: "vocal-ride",
      target: { trackId: "vocal", rack: "builtins", parameter: "fader_db", channel: "both" },
      segments: [{ shape: "linear", startSample: 0n, endSample: 4_800n, startValue: -3, endValue: 0 }],
    });
}
