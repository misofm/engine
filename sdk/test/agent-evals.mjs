/**
 * Issue #243 eval 4 and S4: the agent operations surface.
 *
 * # What "lossless" has to mean here
 *
 * The failure this eval exists to catch is quiet: an agent asks for `0.3`, the value goes through
 * an `f32`, and it reads back `0.30000001192092896`. Nothing errors, the audio is imperceptibly
 * different, and the agent's next comparison against its own request fails for a reason it cannot
 * see. So the assertions below are string equality on canonical decimals, and the fixture set
 * deliberately includes `0.3`-class values -- decimals with no exact binary representation, which
 * is precisely where a float round-trip stops being the identity.
 *
 * # Held to the engine, not to itself
 *
 * The SDK generates lattice points rather than reading them, because shipping the catalog's points
 * would cost megabytes. A second implementation of a rule is only worth having if it is *held to*
 * the first, so the first test below reproduces, for every parameter in the shipped catalog, the
 * digest that `tools/miso-engine-parameter-metadata/src/bin/lattice_oracle` computes from the
 * engine's own `parameter_lattice_points`. Points, ranks, intrinsic flags, step resolutions and
 * decimal lookups are all inside that comparison.
 */

import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import { before, describe, test } from "node:test";
import { resolve } from "node:path";

import { catalog, decimalToFloat32, parameter } from "../src/core/agent.ts";
import { indexForDecimal, latticePoints, resolveStep, STEP_SIZES } from "../src/core/lattice.ts";
import { fixedHalfEven, parseExactDecimal, compareExactDecimal } from "../src/core/decimal.ts";
import { CATALOG } from "../src/generated/catalog.ts";
import { MisoEngineAsset } from "../src/core/asset.ts";
import { createOfflineEngine } from "../src/headless/engine.ts";
import { ABI_LAYOUT } from "../src/generated/abi.ts";
import { effectEntry, moduleBytes, ramp, sessionDocument } from "./support.mjs";

const REPO_ROOT = resolve(import.meta.dirname, "..", "..");

/** The engine's own lattices, one row per parameter. */
function engineLattices() {
  const text = execFileSync(
    "cargo",
    ["run", "--locked", "-q", "-p", "miso-engine-parameter-metadata", "--bin", "lattice_oracle"],
    { cwd: REPO_ROOT, encoding: "utf8", maxBuffer: 1 << 26 },
  );
  return text.trim().split("\n").slice(1).map((line) => {
    const [effectId, id, name, count, digest, first, last, steps, lookups] = line.split("\t");
    return { effectId, id: Number(id), name, count, digest, first, last, steps, lookups };
  });
}

/** The oracle's digest shape: `index\tcanonical\tintrinsic\n` per point. */
function digestPoints(points) {
  const hash = createHash("sha256");
  for (const point of points) {
    hash.update(`${point.index}\t${point.canonical}\t${point.intrinsic ? 1 : 0}\n`);
  }
  return hash.digest("hex");
}

function declarationOf(row) {
  return {
    domainName: row.domainName,
    minimum: row.minimum,
    maximum: row.maximum,
    default: row.default,
    enumChoices: row.enumChoices,
    step: row.step,
  };
}

describe("eval 4 -- the SDK's lattice is the engine's lattice", () => {
  test("every parameter in the shipped catalog reproduces the engine's digest", () => {
    const rows = engineLattices();
    assert.ok(rows.length > 0, "the oracle produced rows");
    let checked = 0;
    for (const row of rows) {
      const effect = CATALOG.effects.find((candidate) => candidate.id === row.effectId);
      assert.ok(effect, `the catalog carries ${row.effectId}`);
      const parameterRow = effect.parameters.find((candidate) => candidate.id === row.id);
      assert.ok(parameterRow, `${row.effectId} carries parameter ${row.id}`);

      const points = latticePoints(declarationOf(parameterRow));
      assert.equal(
        String(points.length),
        row.count,
        `${row.effectId}.${row.name}: point count`,
      );
      assert.equal(
        digestPoints(points),
        row.digest,
        `${row.effectId}.${row.name}: the SDK's points are not the engine's`,
      );

      // Step resolution and decimal lookup, at the same probes the oracle used.
      const last = points.length - 1;
      const middle = last >> 1;
      const steps = [
        [0, "xs", 1], [0, "xl", -1], [last, "xs", 1],
        [last, "md", -1], [middle, "lg", 3], [middle, "xl", -3],
      ].map(([current, size, count]) => {
        const target = resolveStep(points, current, size, count, parameterRow.step.ladder);
        return target === undefined ? "none" : String(target);
      }).join(",");
      assert.equal(steps, row.steps, `${row.effectId}.${row.name}: step resolutions`);

      const lookups = [0, middle, last].map((index) => {
        const found = indexForDecimal(points, points[index].canonical);
        return "index" in found ? String(found.index) : "miss";
      }).join(",");
      assert.equal(lookups, row.lookups, `${row.effectId}.${row.name}: decimal lookups`);
      checked += 1;
    }
    assert.equal(checked, rows.length);
    assert.ok(checked >= 60, `only ${checked} parameters were cross-checked`);
  });
});

describe("eval 4 -- round-trip losslessness", () => {
  test("every lattice endpoint and an interior point round-trip exactly", () => {
    // The whole catalog, not a sample: `set(param, d)` for each endpoint plus an interior point,
    // and the read-back string must be `d` itself.
    let assertions = 0;
    for (const effect of CATALOG.effects) {
      for (const row of effect.parameters) {
        const handle = parameter(effect.id, row.name);
        const points = handle.points;
        const probes = [0, 1, points.length >> 1, points.length - 2, points.length - 1]
          .filter((index) => index >= 0 && index < points.length);
        for (const index of new Set(probes)) {
          const decimal = points[index].canonical;
          const ack = handle.set(decimal);
          assert.equal(ack.ok, true, `${effect.id}.${row.name}: ${decimal} was refused`);
          assert.equal(ack.value, decimal, `${effect.id}.${row.name}: read-back moved`);
          assert.equal(handle.value, decimal);
          assert.equal(handle.index, index, `${effect.id}.${row.name}: rank moved`);
          assertions += 1;
        }
      }
    }
    assert.ok(assertions > 300, `only ${assertions} round trips were made`);
  });

  test("a 0.3-class value survives, where a float round-trip would not", () => {
    // `0.3` has no exact binary representation. Through an `f32` it becomes 0.30000001192092896,
    // and a surface that formatted that back would hand an agent a number it did not ask for.
    //
    // RED MUTATION: replace the read-back in `ParameterHandle.value` with
    // `String(decimalToFloat32(canonical))` -- this test goes red immediately, because
    // `String(Math.fround(0.3))` is "0.30000001192092896".
    const handle = parameter("miso.compressor", "mix");
    assert.equal(handle.declaration.step.precision, 2);
    const ack = handle.set("0.30");
    assert.equal(ack.ok, true);
    assert.equal(ack.value, "0.30", "the canonical decimal is returned verbatim");
    assert.notEqual(String(decimalToFloat32("0.30")), "0.30");
    assert.equal(String(decimalToFloat32("0.30")), "0.30000001192092896");

    // And the equivalent spellings of one decimal are one point.
    for (const spelling of ["0.30", "0.3", "3e-1", "+0.300", "0.300000"]) {
      const found = handle.set(spelling);
      assert.equal(found.ok, true, `${spelling} names a lattice point`);
      assert.equal(found.value, "0.30", `${spelling} canonicalises to 0.30`);
    }
  });

  test("an off-lattice decimal is refused with the two points that bracket it", () => {
    // A refusal is an answer: the agent is told where it may go, not merely that it may not.
    const handle = parameter("miso.compressor", "mix");
    const ack = handle.set("0.305");
    assert.equal(ack.ok, false);
    assert.equal(ack.reason, "offLattice");
    assert.deepEqual({ ...ack.nearest }, { lower: "0.30", upper: "0.31" });
    assert.equal(handle.value, handle.declaration.default, "a refusal moves nothing");
  });

  test("step lands exactly k lattice points away, and clamps at the endpoints", () => {
    const handle = parameter("miso.compressor", "makeup");
    const ladder = handle.declaration.step.ladder;
    handle.setSteps(240);
    for (const size of STEP_SIZES) {
      for (const count of [1, -1, 3, -3]) {
        const before = handle.index;
        const ack = handle.step(size, count);
        assert.equal(ack.ok, true);
        assert.equal(
          handle.index - before,
          ladder[size] * count,
          `${size} x ${count} moved ${handle.index - before} ranks, not ${ladder[size] * count}`,
        );
      }
      handle.setSteps(240);
    }

    // The clamp: a gesture past the end lands on the end rather than refusing.
    handle.setSteps(handle.points.length - 1);
    assert.equal(handle.step("xl", 10).ok, true);
    assert.equal(handle.index, handle.points.length - 1);
    handle.setSteps(0);
    assert.equal(handle.step("xl", -10).ok, true);
    assert.equal(handle.index, 0);
  });

  test("a diff of two states is expressible in integer steps", () => {
    const left = parameter("miso.parametric-eq", "band-1-gain");
    const right = parameter("miso.parametric-eq", "band-1-gain");
    left.setSteps(10);
    right.setSteps(37);
    assert.equal(left.stepsTo(right), 27);
    // And applying that many xs steps closes the gap exactly, because xs is one rank.
    assert.equal(left.declaration.step.ladder.xs, 1);
    left.step("xs", left.stepsTo(right));
    assert.equal(left.index, right.index);
    assert.equal(left.value, right.value);
  });

  test("setSteps refuses an out-of-range rank rather than clamping", () => {
    // An index is an address, not a gesture: clamping one would silently write somewhere else.
    const handle = parameter("miso.compressor", "mix");
    const ack = handle.setSteps(handle.points.length);
    assert.equal(ack.ok, false);
    assert.equal(ack.reason, "outOfRange");
    assert.equal(handle.value, handle.declaration.default);
  });

  test("the generated catalog carries decimals, not floats", () => {
    const rows = catalog();
    assert.ok(rows.length >= 60);
    for (const row of rows) {
      for (const field of [row.minimum, row.maximum, row.default, row.step.size]) {
        assert.equal(typeof field, "string", `${row.effectId}.${row.name}: ${field} is not a string`);
        assert.ok(
          parseExactDecimal(field) !== undefined,
          `${row.effectId}.${row.name}: ${field} is not a decimal literal`,
        );
      }
      assert.ok(row.points > 0);
      assert.equal(typeof row.liveUpdatable, "boolean");
      assert.ok(["shared", "perLane"].includes(row.channelPolicy));
      for (const size of STEP_SIZES) assert.ok(Number.isInteger(row.step.ladder[size]));
      // The declared bounds are themselves lattice members, so min/max are the lattice's own ends.
      assert.equal(compareExactDecimal(parseExactDecimal(row.minimum), parseExactDecimal(row.maximum)), -1);
    }
  });
});

describe("eval 4 -- losslessness by physics, on the wasm engine", () => {
  let asset;
  before(async () => {
    asset = await MisoEngineAsset.load(await moduleBytes());
  });

  /**
   * The render-digest cross-check.
   *
   * Adopted ruling 5462139867 finding 4 makes this the wasm-side proof of losslessness: a value
   * set through the agent surface and then rendered must produce the same bits as the same value
   * authored into the document. No float formatting is involved in the comparison at all -- the
   * claim is settled by physics rather than by string equality, which is exactly what makes it
   * immune to a formatting bug that happened to be consistent on both sides.
   */
  async function renderBlocks(document, commands, blocks = 8) {
    const engine = await createOfflineEngine(document, {
      asset,
      console: { commandQueueRecords: ABI_LAYOUT.constants.defaultCommandQueueRecords },
    });
    try {
      if (commands) {
        const report = engine.submitCommands(commands.records, commands.count);
        assert.ok(report.ok, `the command batch was refused: ${report.reasonName}`);
      }
      const shape = engine.shape();
      const rendered = [];
      for (let block = 0; block < blocks; block += 1) {
        for (const [index, source] of shape.sources.entries()) {
          const planes = Array.from({ length: source.channels }, (_unused, channel) =>
            ramp(shape.quantumFrames, 5 + index * 16 + channel + block * 1024));
          assert.ok(engine.submitSource({
            sourceId: source.id,
            generation: 1n,
            startFrame: BigInt(block * shape.quantumFrames),
            planes,
            endOfRegion: false,
          }).ok);
        }
        rendered.push(engine.render());
      }
      return rendered;
    } finally {
      engine.dispose();
    }
  }

  function digestFrom(blocks, first) {
    const hash = createHash("sha256");
    for (const { left, right } of blocks.slice(first)) {
      for (const plane of [left, right]) {
        hash.update(Buffer.from(plane.buffer, plane.byteOffset, plane.byteLength));
      }
    }
    return hash.digest("hex");
  }

  /** One 48-byte `effectParam` record, written through the generated field offsets. */
  function effectParamRecord(value) {
    const bytes = ABI_LAYOUT.commandRecord.bytes;
    const records = new Uint8Array(bytes);
    const view = new DataView(records.buffer);
    const at = (name) => ABI_LAYOUT.commandRecord.fields.find((row) => row.name === name).offset;
    const kind = ABI_LAYOUT.constants.wireCommandKinds.find((row) => row.name === "effectParam");
    view.setUint8(at("kind"), kind.value);
    view.setUint8(at("rack"), 0); // simd1
    view.setUint8(at("channel"), 2); // both
    view.setUint32(at("trackIndex"), 0, true);
    view.setUint32(at("effectIndex"), 0, true);
    view.setUint32(at("parameterId"), 4, true); // band-1 gain
    view.setUint32(at("smoothingSamples"), 0, true);
    view.setFloat32(at("values"), value, true);
    return { records, count: 1 };
  }

  test("set -> render -> digest equals the same document authored with that value", async () => {
    const handle = parameter("miso.parametric-eq", "band-1-gain");
    // A decimal with no exact binary representation, chosen for the same reason as the 0.3 probe.
    const ack = handle.set("-6.3");
    assert.equal(ack.ok, true, "-6.3 is a lattice point of a 0.1 dB step at one decimal");
    assert.equal(ack.value, "-6.3");

    const eq = (gain) => ({
      simd1: [effectEntry("eq", "miso.parametric-eq", [
        { id: 1, unit: "linear", value: 1.0 },
        { id: 2, unit: "linear", value: 2.0 },
        { id: 3, unit: "hz", value: 1000.0 },
        { id: 4, unit: "db", value: gain },
      ])],
    });
    const withDefault = sessionDocument({ effects: eq(0.0) });
    // Authored with the very decimal the agent set, spelled as a document would spell it.
    const authored = sessionDocument({ effects: eq("-6.3") });

    const commanded = await renderBlocks(
      withDefault,
      effectParamRecord(decimalToFloat32(handle.value)),
    );
    const declared = await renderBlocks(authored, undefined);

    // The claim is about the value, not about when it arrived.
    //
    // A live retarget and an authored value differ for exactly two blocks, for two reasons that
    // are both about timing rather than about the number: the batch takes effect at
    // `appliedAtSample`, the first sample of the *next* rendered block, so block 0 still carries
    // the old gain; and the band's biquad then carries two samples of state computed under the old
    // coefficients into block 1. From block 2 the two are bit-identical, and stay so -- which is
    // the losslessness claim, settled by physics rather than by string comparison.
    //
    // Adopted ruling 5462139867 finding 4 makes this the wasm-side leg of eval 4. Its other leg,
    // an engine-resolved oracle through the control protocol, has no subject on this tree: see the
    // finding recorded with this change.
    const SETTLED = 2;
    assert.notEqual(
      digestFrom(commanded, 0),
      digestFrom(declared, 0),
      "the retarget must be audible, or the equality below would be vacuous",
    );
    for (let block = 0; block < SETTLED; block += 1) {
      assert.notDeepEqual(
        [...commanded[block].left],
        [...declared[block].left],
        `block ${block} is the retarget transient and must differ`,
      );
    }
    for (let block = SETTLED; block < commanded.length; block += 1) {
      assert.deepEqual(
        [...commanded[block].left],
        [...declared[block].left],
        `block ${block} left plane: set-then-render did not equal authored`,
      );
      assert.deepEqual([...commanded[block].right], [...declared[block].right]);
    }
    assert.equal(
      digestFrom(commanded, SETTLED),
      digestFrom(declared, SETTLED),
      "a value set through the agent surface rendered different bits from the same value authored",
    );

    // The control: a neighbouring lattice point must NOT match, so the equality above is a
    // statement about this value rather than about the digest being insensitive.
    handle.set("-6.2");
    const moved = await renderBlocks(
      withDefault,
      effectParamRecord(decimalToFloat32(handle.value)),
    );
    assert.notEqual(
      digestFrom(moved, SETTLED),
      digestFrom(declared, SETTLED),
      "one lattice point of difference must be audible in the settled window",
    );
  });
});
