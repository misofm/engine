/**
 * Issues #1383 (slices 1383a and 1383b) and #1385: the SDK's session-transaction builder against
 * the engine.
 *
 * The oracle (`hosts/host-web/examples/sdk_transaction_oracle.rs`) decodes each SDK frame with the
 * protocol codec, re-encodes the decoded edits, and applies them to the base session with the
 * session store. Nothing here is a committed digest: the bytes are compared to the codec's own
 * encoding of what it decoded, and the snapshot to the session builder's own document.
 */

import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { describe, test } from "node:test";

import { MisoUsageError } from "../src/core/errors.ts";
import { effect, session } from "../src/core/session.ts";
import { encodeSessionTransaction, transaction } from "../src/core/session-transaction.ts";

const REPO_ROOT = resolve(import.meta.dirname, "..", "..");
const CONTENT_A = `blake3:${"a".repeat(64)}`;
const CONTENT_B = `blake3:${"b".repeat(64)}`;
const CONTENT_C = `blake3:${"c".repeat(64)}`;

/** Send `transactions` in order to the oracle over `base`, one revision after another. */
function oracle(base, transactions) {
  const directory = mkdtempSync(join(tmpdir(), "sdk-transaction-oracle-"));
  try {
    const basePath = join(directory, "base.json");
    writeFileSync(basePath, base.toJson());
    const revision = BigInt(base.toJSON().revision);
    const frames = transactions.map((tx, index) => encodeSessionTransaction(tx, {
      requestId: BigInt(index + 1),
      expectedRevision: revision + BigInt(index),
    }));
    const output = execFileSync(
      "cargo",
      ["run", "--locked", "-q", "-p", "host-web", "--example", "sdk_transaction_oracle", "--", basePath],
      { cwd: REPO_ROOT, input: Buffer.concat(frames), encoding: "utf8", maxBuffer: 1 << 26 },
    );
    const answers = output.trim().split("\n").map((line) => JSON.parse(line));
    assert.equal(answers.length, frames.length, "the oracle answers every frame");
    return answers.map((answer, index) => ({ ...answer, sdk: Buffer.from(frames[index]).toString("hex") }));
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
}

/** Every frame applied, and its bytes are the codec's re-encoding of what it decoded. */
function assertCodecBytes(answers) {
  answers.forEach((answer, index) => {
    assert.equal(answer.decode, undefined, `frame ${index} decodes`);
    assert.equal(answer.refused, undefined, `frame ${index} applies`);
    assert.equal(answer.sdk, answer.bytes, `frame ${index} is the codec's bytes`);
  });
}

const mono = { channels: 1, bitDepth: 16, frames: 4800, content: CONTENT_A };
const stereo = { channels: 2, bitDepth: 24, frames: 9600n, content: CONTENT_B };

describe("session transactions encode as the engine's codec does", () => {
  test("every source, track, rack and effect edit, repeated fields at 0, 1 and 3, is the codec's bytes", () => {
    // Red if any field is out of order, at the wrong width, without its mandatory flag, or if a
    // repeated field (inserts, params, console entries) is miscounted: the codec then decodes other
    // edits, which re-encode to other bytes, or refuses the frame.
    const declare = (revision) => session({ id: "tx.bytes", sampleRateHz: 48_000, revision })
      .source("seed", mono);
    const bare = declare(0).track("seed", { source: "seed" });
    const first = transaction(bare)
      .upsertSource("a", mono)
      .upsertSource("b", stereo)
      .upsertTrack("t", { source: "a" })
      .setSourceContent("b", { channels: 2, bitDepth: "32f", frames: 12_000, content: CONTENT_C });
    // The session the engine holds after `first`, which the next transaction is built against.
    const afterFirst = declare(1)
      .source("a", mono)
      .source("b", { channels: 2, bitDepth: "32f", frames: 12_000, content: CONTENT_C })
      .track("seed", { source: "seed" })
      .track("t", { source: "a" });
    const second = transaction(afterFirst)
      .upsertTrack("u", {
        source: { id: "b", left: 1, right: 0 },
        builtins: { left: { trimDb: -3, polarityInvert: true }, right: { hpfHz: 80, delaySamples: 12 } },
        inserts: [effect("miso.soft-clip", { drive: { left: 6, right: 3 } })],
        fader: { leftDb: -6.5, rightMute: true },
        pan: { matrix: { ll: 0.5, lr: 0.25, rl: -0.25, rr: 1 }, smoothingSamples: 64 },
      })
      .upsertTrack("v", {
        source: "b",
        inserts: [
          effect("miso.parametric-eq", { "band-1-enabled": true, "band-1-gain": -2, "band-1-frequency": 900 }),
          effect("miso.compressor", {}, {
            slotId: "comp",
            linkMode: "maximum",
            bypass: true,
            sidechain: { source: { kind: "track", trackId: "seed", tap: "pre_fader" }, portId: "sidechain-in" },
          }),
          effect("miso.gate-expander", { threshold: -50 }, { channel: "left" }),
        ],
        pan: { left: -0.5, right: 0.5 },
      })
      .setTrackSourceAssignment("seed", { id: "b", right: 0 })
      .removeTrack("t")
      .removeSource("a");
    assertCodecBytes(oracle(bare, [first.build(), second.build()]));

    // The rack and effect edits, on a track and on a submix strip; racks and effect orders at 0, 1
    // and 3 inserts.
    const racks = session({ id: "tx.racks", sampleRateHz: 48_000 })
      .source("a", stereo)
      .track("t", {
        source: "a",
        inserts: [
          effect("miso.soft-clip", { mix: 0.5 }, { slotId: "clip" }),
          effect("miso.compressor", { ratio: 4 }, { slotId: "comp" }),
        ],
      })
      .track("key", { source: "a" })
      .submix("bus");
    const rackEdits = transaction(racks)
      .setTrackRack("bus", [])
      .setTrackEffectOrder("bus", [])
      .setTrackRack("bus", [effect("miso.true-peak-limiter", { ceiling: -1 }, { slotId: "limit" })])
      .setTrackEffectOrder("bus", ["limit"])
      .putTrackEffect("t", 1, effect("miso.gate-expander", { threshold: -40 }, { slotId: "gate", linkMode: "average" }))
      .setTrackEffectOrder("t", ["comp", "gate", "clip"])
      .setEffectIdentity("t", "clip", "miso.transient-shaper")
      .setEffectQuality("t", "comp", "normal")
      .setEffectSidechain("t", "comp", { source: { kind: "track", trackId: "key", tap: "post_fader" }, portId: "sidechain-in" })
      .setEffectSidechain("t", "gate", null)
      .removeTrackEffect("t", "gate")
      .setTrackRack("key", [
        effect("miso.parametric-eq", { "band-1-gain": 2 }, { slotId: "eq" }),
        effect("miso.delay", {}, { slotId: "delay" }),
        effect("miso.soft-clip", { drive: { left: 3, right: 1 } }, { slotId: "clip" }),
      ]);
    assertCodecBytes(oracle(racks, [rackEdits.build()]));

    // Console entries at 1 and 3: one per declared slot, so each count is its own base.
    const one = session({ id: "tx.console.one", sampleRateHz: 48_000 })
      .console({ preInsert: [{ slot: "eq", effectId: "miso.parametric-eq" }] })
      .source("a", mono);
    assertCodecBytes(oracle(one, [
      transaction(one).upsertTrack("t", { source: "a", console: [{ slot: "eq", parameters: { "band-1-gain": 3 } }] }).build(),
    ]));
    const three = session({ id: "tx.console.three", sampleRateHz: 96_000 })
      .console({
        preInsert: [{ slot: "eq", effectId: "miso.parametric-eq" }, { slot: "comp", effectId: "miso.compressor" }],
        postInsert: [{ slot: "limit", effectId: "miso.true-peak-limiter" }],
      })
      .source("a", stereo);
    assertCodecBytes(oracle(three, [
      transaction(three).upsertTrack("t", {
        source: "a",
        console: [
          { slot: "eq" },
          { slot: "comp", bypass: true, parameters: { ratio: 3 } },
          { slot: "limit", channel: "right", parameters: { ceiling: -1, release: 100, lookahead: 2 } },
        ],
      }).build(),
    ]));
  });

  test("session, console, submix, output and route edits are the codec's bytes, each ramp omitted, 0 and 960", () => {
    // Red if any field of a session, profile, console, submix, output or route payload is out of
    // order, at the wrong width, without its mandatory flag or miscounted (console slots and
    // entries at 0, 1 and 2): the codec then decodes other edits, which re-encode to other bytes,
    // or refuses the frame. The re-encoding cannot tell an omitted `rampSamples` written as 0 from
    // a given 0, so the three ramp variants must also be three different frames.
    const graph = (revision) => session({ id: "tx.graph", sampleRateHz: 48_000, revision })
      .source("a", stereo)
      .track("t", { source: "a" })
      .submix("bus")
      .output("main")
      .route({ id: "r1", source: { kind: "track", trackId: "t", tap: "post_fader" }, destination: { kind: "submix_input", submixId: "bus" } })
      .route({ id: "r2", source: { kind: "submix", submixId: "bus", tap: "post_pan" }, destination: { kind: "output_input", outputId: "main" } });
    const edits = transaction(graph(0))
      .setSessionId("tx.renamed")
      // The session store applies a rate or quantum change; the control plane refuses it (#1290a).
      .setSampleRateHz(96_000)
      .setQuantumFrames(256)
      .setRenderProfile({ id: "studio" })
      .setOutputProfile({ id: "stereo", channels: 2, sampleFormat: "f32_planar" })
      .upsertSubmix("fx")
      .upsertSubmix("stems", {
        builtins: { left: { trimDb: -2 }, right: { hpfHz: 40 } },
        inserts: [effect("miso.compressor", { ratio: 2 }, { slotId: "glue" })],
        fader: { rightDb: -4, leftMute: true },
        pan: { left: -0.25, right: 0.75, smoothingSamples: 32 },
      })
      .upsertOutput("alt")
      .upsertRoute({
        id: "r3",
        source: { kind: "submix", submixId: "fx", tap: "insert_send" },
        destination: { kind: "output_input", outputId: "alt" },
        matrix: { ll: 0.5, lr: -0.25, rl: 0.125, rr: 1 },
        gainDb: -3.5,
        mute: true,
      })
      .upsertRoute({
        id: "r4",
        source: { kind: "track", trackId: "t", tap: "pre_fader" },
        destination: { kind: "submix_input", submixId: "stems" },
        followsMute: false,
      })
      .setRouteSource("r1", { kind: "track", trackId: "t", tap: "input" })
      .setRouteDestination("r1", { kind: "submix_input", submixId: "fx" })
      .removeRoute("r2")
      .removeSubmix("bus")
      .removeOutput("main");
    assertCodecBytes(oracle(graph(0), [edits.build()]));

    // The console: one slot to none, then none to two, each strip given its entries in the same
    // transaction; `setConsole` and `setTrackConsole` with `rampSamples` omitted, 0 and 960.
    const oneSlot = (revision) => session({ id: "tx.console", sampleRateHz: 48_000, revision })
      .console({ preInsert: [{ slot: "eq", effectId: "miso.parametric-eq" }] })
      .source("a", mono)
      .track("t", { source: "a", console: [{ slot: "eq" }] })
      .submix("bus");
    const noSlot = (revision) => session({ id: "tx.console", sampleRateHz: 48_000, revision })
      .source("a", mono)
      .track("t", { source: "a" })
      .submix("bus");
    const frames = [undefined, 0, 960].map((rampSamples) => {
      const ramp = rampSamples === undefined ? {} : { rampSamples };
      const clear = transaction(oneSlot(0))
        .setConsole({}, ramp)
        .setTrackConsole("t", [], ramp)
        .setTrackConsole("bus", [], ramp);
      const two = transaction(noSlot(1))
        .setConsole({
          preInsert: [{ slot: "eq", effectId: "miso.parametric-eq" }],
          postInsert: [{ slot: "comp", effectId: "miso.compressor", linkMode: "maximum" }],
        }, ramp)
        .setTrackConsole("t", [
          { slot: "eq", parameters: { "band-1-gain": 2 } },
          { slot: "comp", bypass: true, linkMode: "dual_mono", parameters: { ratio: 3 } },
        ], ramp)
        .upsertSubmix("bus");
      const answers = oracle(oneSlot(0), [clear.build(), two.build()]);
      assertCodecBytes(answers);
      return answers.map((answer) => answer.sdk).join("");
    });
    assert.equal(new Set(frames).size, 3, "omitted, 0 and 960 are three different frames");
    // The same with `setConsole`'s ramp fixed: only `setTrackConsole`'s ramp varies.
    const trackFrames = [undefined, 0, 960].map((rampSamples) => {
      const ramp = rampSamples === undefined ? {} : { rampSamples };
      const clear = transaction(oneSlot(0)).setConsole({}).setTrackConsole("t", [], ramp).setTrackConsole("bus", [], ramp);
      const answers = oracle(oneSlot(0), [clear.build()]);
      assertCodecBytes(answers);
      return answers[0].sdk;
    });
    assert.equal(new Set(trackFrames).size, 3, "setTrackConsole's omitted, 0 and 960 are three different frames");
  });

  test("automation and VCA edits are the codec's bytes, members and spans at 0, 1 and 3", () => {
    // Red if any field of an automation, its control-address target (optional ID, slot, control
    // and lane fields), a span, the offset, or a VCA is out of order, at the wrong width, with the
    // wrong mandatory flag or miscounted: the codec then decodes other edits, which re-encode to
    // other bytes, or refuses the frame.
    const target = (lane) => ({ owner: { kind: "strip", id: "t" }, section: { kind: "inserts", effect: "eq" }, control: "band_1_gain_db", lane });
    const ramp = { shape: "linear", startSample: 0, endSample: 4800, startValue: -6, endValue: 3 };
    const base = session({ id: "tx.automation", sampleRateHz: 48_000 })
      .console({ preInsert: [{ slot: "comp", effectId: "miso.compressor" }] })
      .source("a", stereo)
      .track("t", { source: "a", console: [{ slot: "comp" }], inserts: [effect("miso.parametric-eq", {}, { slotId: "eq" })] })
      .track("u", { source: "a", console: [{ slot: "comp" }] })
      .submix("bus")
      .output("main")
      .route({ id: "send", source: { kind: "track", trackId: "t", tap: "post_fader" }, destination: { kind: "submix_input", submixId: "bus" } })
      .vca("old", { members: ["u"] })
      .automation({ id: "a1", target: target("both"), segments: [ramp] });
    const edits = transaction(base)
      .upsertVca("empty", { members: [] })
      .upsertVca("grp", { fader: { leftDb: -3, rightMute: true }, members: ["u", "t", "bus"] })
      .upsertVca("top", { fader: { rightDb: 2.5 }, members: ["grp"] })
      .removeVca("old")
      .upsertAutomation({
        id: "a2",
        target: target("left"),
        segments: [
          { shape: "step", startSample: 0, endSample: 10, startValue: 1, endValue: 1 },
          { shape: "linear", startSample: 10, endSample: 480, startValue: 1, endValue: -2 },
          { shape: "exponential", startSample: 480, endSample: 9_007_199_254_740_991n, startValue: 0.5, endValue: 2 },
        ],
      })
      .upsertAutomation({
        id: "a3",
        target: { owner: { kind: "strip", id: "u" }, section: { kind: "builtins" }, control: "fader_db", lane: "both" },
        segments: [{ shape: "linear", startSample: 0, endSample: 96, startValue: -12, endValue: 0 }],
        offset: { leftDb: -1.5, rightDb: 0.5 },
      })
      .upsertAutomation({
        id: "a4",
        target: { owner: { kind: "strip", id: "t" }, section: { kind: "console", slot: "comp" }, control: "threshold_db", lane: "right" },
        segments: [{ shape: "linear", startSample: 0, endSample: 96, startValue: -20, endValue: -10 }],
      })
      .upsertAutomation({
        id: "a5",
        target: { owner: { kind: "vca", id: "grp" }, section: { kind: "own" }, control: "offset_db", lane: "left" },
        segments: [{ shape: "linear", startSample: 0, endSample: 96, startValue: -6, endValue: 0 }],
      })
      .setAutomationTarget("a1", target("right"))
      .setAutomationSegments("a1", [{ shape: "step", startSample: 100, endSample: 200, startValue: 2, endValue: 2 }])
      .setAutomationSegments("a2", [ramp, { ...ramp, startSample: 4800, endSample: 9600 }, { ...ramp, startSample: 9600, endSample: 9601 }])
      .removeAutomation("a4");
    assertCodecBytes(oracle(base, [edits.build()]));
  });

  test("each snapshot after a transaction is the document a session builder declares directly", () => {
    // Red if a byte-valid encoder maps a spec field to the wrong model field (left and right
    // builtins swapped, the fader's or pan's lanes crossed, a parameter on the wrong channel, a
    // reassigned track's source lanes swapped): the store's
    // snapshot then differs from the builder's document for the same session.
    const declare = (revision) => session({ id: "tx.semantics", sampleRateHz: 48_000, revision })
      .source("drums", stereo);
    const drums = { source: "drums", fader: { leftDb: -1 } };
    const vocal = {
      source: "vox",
      builtins: { left: { trimDb: -4, lpfHz: 12_000 }, right: { polarityInvert: true, delaySamples: 7 } },
      inserts: [effect("miso.parametric-eq", { "band-1-gain": { left: 2, right: -3 } })],
      fader: { leftDb: -2, rightDb: -8, leftMute: true },
      pan: { left: -0.75, right: 0.25 },
    };
    const base = declare(0).track("drums", drums);
    const add = transaction(base).upsertSource("vox", mono).upsertTrack("vocal", vocal);
    const afterAdd = declare(1).source("vox", mono).track("drums", drums).track("vocal", vocal);
    const remove = transaction(afterAdd)
      .removeTrack("drums")
      .setTrackSourceAssignment("vocal", { id: "drums", left: 1, right: 0 });
    const [added, removed] = oracle(base, [add.build(), remove.build()]);
    assert.equal(added.ok, afterAdd.toJson());
    const reassigned = { ...vocal, source: { id: "drums", left: 1, right: 0 } };
    assert.equal(removed.ok, declare(2).source("vox", mono).track("vocal", reassigned).toJson());
  });

  test("console, submix, output and route edits land on the entity and field they name", () => {
    // Red if a byte-valid encoder maps a spec field to the wrong model field (a route's tap, mute
    // and follows_mute, or matrix coefficients crossed, a destination's kind swapped, a submix
    // strip's console entry, fader lane or pan crossed, a console slot in the wrong section): the
    // store's snapshot then differs from the builder's document for the same session.
    const base = session({ id: "tx.graph.semantics", sampleRateHz: 48_000 })
      .source("a", stereo)
      .track("t", { source: "a" })
      .output("main");
    const console = {
      preInsert: [{ slot: "eq", effectId: "miso.parametric-eq" }],
      postInsert: [{ slot: "comp", effectId: "miso.compressor", linkMode: "average" }],
    };
    const trackConsole = [{ slot: "eq", parameters: { "band-1-gain": -3 } }, { slot: "comp", bypass: true }];
    const bus = {
      console: [{ slot: "eq", bypass: true }, { slot: "comp", channel: "left", parameters: { ratio: 6 } }],
      fader: { leftDb: -1.5, rightMute: true },
      pan: { left: 0.25, right: -0.5 },
    };
    const send = {
      id: "send",
      source: { kind: "track", trackId: "t", tap: "pre_fader" },
      destination: { kind: "submix_input", submixId: "bus" },
      matrix: { ll: 0.75, lr: 0.25, rl: -0.5, rr: 1 },
      gainDb: -6,
      mute: false,
      followsMute: true,
    };
    const edits = transaction(base)
      .setConsole(console)
      .setTrackConsole("t", trackConsole)
      .upsertSubmix("bus", bus)
      .upsertOutput("alt")
      .upsertRoute(send)
      .upsertRoute({ id: "out", source: { kind: "track", trackId: "t", tap: "post_pan" }, destination: { kind: "output_input", outputId: "main" } })
      .setRouteSource("out", { kind: "submix", submixId: "bus", tap: "post_fader" })
      .setRouteDestination("out", { kind: "output_input", outputId: "alt" })
      .removeOutput("main");
    const [answer] = oracle(base, [edits.build()]);
    const expected = session({ id: "tx.graph.semantics", sampleRateHz: 48_000, revision: 1 })
      .console(console)
      .source("a", stereo)
      .track("t", { source: "a", console: trackConsole })
      .submix("bus", bus)
      .output("alt")
      .route(send)
      .route({ id: "out", source: { kind: "submix", submixId: "bus", tap: "post_fader" }, destination: { kind: "output_input", outputId: "alt" } });
    assert.equal(answer.ok, expected.toJson());
  });

  test("automation and VCA edits land on the entity and field they name", () => {
    // Red if a byte-valid encoder maps a spec field to the wrong model field (a VCA's members or
    // its fader lanes crossed, an automation target's lane, slot or row, a span's start and end, or
    // the offset's lanes swapped): the store's snapshot then differs from the builder's document.
    const declare = (revision) => session({ id: "tx.automation.semantics", sampleRateHz: 48_000, revision })
      .source("a", stereo)
      .track("t", { source: "a", inserts: [effect("miso.parametric-eq", {}, { slotId: "eq" })] })
      .track("u", { source: "a" });
    const vca = { fader: { leftDb: -4, rightMute: true }, members: ["u", "t"] };
    const gain = {
      id: "gain",
      target: { owner: { kind: "strip", id: "t" }, section: { kind: "inserts", effect: "eq" }, control: "band_1_gain_db", lane: "left" },
      segments: [
        { shape: "linear", startSample: 0, endSample: 4800, startValue: -6, endValue: 3 },
        { shape: "step", startSample: 4800, endSample: 9600, startValue: 1.5, endValue: 1.5 },
      ],
    };
    const ride = {
      id: "ride",
      target: { owner: { kind: "strip", id: "u" }, section: { kind: "builtins" }, control: "fader_db", lane: "right" },
      segments: [{ shape: "linear", startSample: 96, endSample: 960, startValue: -20, endValue: -2 }],
      offset: { rightDb: -1.25 },
    };
    const base = declare(0).automation({ ...ride, segments: [{ shape: "step", startSample: 0, endSample: 1, startValue: 0, endValue: 0 }] });
    const edits = transaction(base)
      .upsertVca("grp", vca)
      .upsertAutomation(gain)
      .setAutomationSegments("ride", ride.segments)
      .setAutomationTarget("ride", ride.target)
      .removeAutomation("ride")
      .upsertAutomation(ride);
    const [answer] = oracle(base, [edits.build()]);
    assert.equal(answer.ok, declare(1).vca("grp", vca).automation(gain).automation(ride).toJson());
  });

  test("rack and effect edits land on the strip, insert and field they name", () => {
    // Red if a byte-valid encoder writes a put at the wrong position, reorders the wrong strip,
    // retargets or reroutes the wrong insert, or crosses a sidechain's tap: the store's snapshot
    // then differs from the builder's document for the same strips.
    const declare = (revision, strips) => strips(session({ id: "tx.effects", sampleRateHz: 48_000, revision })
      .source("a", stereo)
      .track("key", { source: "a" }));
    const base = declare(0, (builder) => builder
      .track("t", {
        source: "a",
        inserts: [
          effect("miso.soft-clip", { mix: 0.25 }, { slotId: "clip" }),
          effect("miso.compressor", { ratio: 3 }, { slotId: "comp" }),
          effect("miso.parametric-eq", { "band-1-gain": -4 }, { slotId: "eq" }),
        ],
      })
      .submix("bus"));
    const edits = transaction(base)
      .putTrackEffect("t", 1, effect("miso.gate-expander", { threshold: -45 }, { slotId: "gate" }))
      .setTrackEffectOrder("t", ["eq", "comp", "gate", "clip"])
      .setEffectIdentity("t", "clip", "miso.transient-shaper")
      .setEffectSidechain("t", "comp", { source: { kind: "track", trackId: "key", tap: "pre_fader" }, portId: "sidechain-in" })
      .removeTrackEffect("t", "eq")
      .setTrackRack("bus", [effect("miso.true-peak-limiter", { ceiling: -2 }, { slotId: "limit" })]);
    const [answer] = oracle(base, [edits.build()]);
    const expected = declare(1, (builder) => builder
      .track("t", {
        source: "a",
        inserts: [
          effect("miso.compressor", { ratio: 3 }, {
            slotId: "comp",
            sidechain: { source: { kind: "track", trackId: "key", tap: "pre_fader" }, portId: "sidechain-in" },
          }),
          effect("miso.gate-expander", { threshold: -45 }, { slotId: "gate" }),
          effect("miso.transient-shaper", { mix: 0.25 }, { slotId: "clip" }),
        ],
      })
      .submix("bus", {
        inserts: [effect("miso.true-peak-limiter", { ceiling: -2 }, { slotId: "limit" })],
        // The transparent strip a spec-less `.submix(id)` declares keeps its identity matrix.
        pan: { matrix: { ll: 1, lr: 0, rl: 0, rr: 1 } },
      }));
    assert.equal(answer.ok, expected.toJson());
  });
});

const refused = (call, pattern) => assert.throws(call, (error) => {
  assert.ok(error instanceof MisoUsageError, `expected a MisoUsageError, got ${error}`);
  assert.match(error.message, pattern);
  return true;
});

describe("the builder refuses what the engine would refuse, before any byte exists", () => {
  test("an empty transaction, an unknown effect, an undeclared source and a fader out of domain", () => {
    // Red if the builder lets through a transaction the engine must refuse: the caller then pays a
    // round trip and gets the refusal without the path of the call that caused it.
    const base = session({ id: "tx.refusals", sampleRateHz: 48_000 }).source("a", mono);
    const tx = transaction(base);
    refused(() => tx.build(), /at least one edit/);
    const unknown = { effectId: "miso.not-an-effect", parameters: {}, options: { bypass: false, quality: "normal", linkMode: "dual_mono", channel: "both" } };
    refused(() => tx.upsertTrack("t", { source: "a", inserts: [unknown] }), /unknown native effect/);
    refused(() => tx.upsertTrack("t", { source: "missing" }), /upsertTrack\("t"\)\.source: 'missing' is not a declared source/);
    refused(() => tx.upsertTrack("t", { source: "a", fader: { leftDb: 40 } }), /upsertTrack\("t"\)\.fader\.leftDb/);
    refused(() => tx.removeSource("missing"), /'missing' is not a declared source/);
    // An ID the codec cannot carry as a stable ID would encode and then fail to decode.
    refused(() => tx.upsertSource("Bad ID", mono), /upsertSource\(\)\.id: a Session V1 stable ID/);
    refused(() => tx.upsertSource("", mono), /upsertSource\(\)\.id: a Session V1 stable ID/);
    refused(() => tx.upsertTrack("Bad ID", { source: "a" }), /upsertTrack\(\)\.id: a Session V1 stable ID/);
    // The refusals left the builder as it was: still no edit.
    refused(() => tx.build(), /at least one edit/);
  });

  test("rack and effect edits the engine would refuse, and setTrackConsole without a console change", () => {
    // Red if a strip edit the engine refuses (a missing strip or insert, a put past the end or with
    // no stable ID, an order that is not a permutation, an identity the insert's parameters do not
    // fit, an unknown port) or a `setTrackConsole` with no earlier console change, or a
    // `rampSamples` that is not a u32, reaches the wire: the caller pays a round trip and loses the
    // path of the call.
    const base = session({ id: "tx.effect.refusals", sampleRateHz: 48_000 })
      .console({ preInsert: [{ slot: "eq", effectId: "miso.parametric-eq" }] })
      .source("a", mono)
      .track("t", {
        source: "a",
        console: [{ slot: "eq" }],
        inserts: [
          effect("miso.soft-clip", { drive: 6 }, { slotId: "clip" }),
          effect("miso.parametric-eq", {}, { slotId: "eq" }),
          effect("miso.compressor", {}, { slotId: "linked", linkMode: "maximum" }),
          effect("miso.compressor", {}, {
            slotId: "keyed",
            sidechain: { source: { kind: "track", trackId: "t", tap: "input" }, portId: "sidechain-in" },
          }),
        ],
      });
    const tx = transaction(base);
    refused(() => tx.setTrackRack("missing", []), /setTrackRack\("missing"\)\.id: 'missing' is not a declared track or submix/);
    refused(() => tx.removeTrackEffect("t", "gone"), /removeTrackEffect\("t"\)\.slotId: 'gone' is not an insert/);
    refused(() => tx.putTrackEffect("t", 0, effect("miso.delay")), /putTrackEffect\("t"\)\.effect\.slotId/);
    refused(() => tx.putTrackEffect("t", 4, effect("miso.delay", {}, { slotId: "eq" })), /finalPosition: 4 is past the end of the 3 other/);
    refused(() => tx.putTrackEffect("t", -1, effect("miso.delay", {}, { slotId: "d" })), /finalPosition must be an integer/);
    refused(() => tx.setTrackEffectOrder("t", ["eq"]), /setTrackEffectOrder\("t"\)\.slotIds: expected each of the strip's inserts exactly once/);
    refused(() => tx.setTrackEffectOrder("t", ["eq", "eq"]), /exactly once/);
    refused(() => tx.setTrackEffectOrder("t", ["Bad", "eq"]), /slotIds\[0\]: a Session V1 stable ID/);
    // soft-clip's parameter 1 is drive in dB; the transient shaper's is attack amount, linear.
    refused(() => tx.setEffectIdentity("t", "clip", "miso.transient-shaper"), /setEffectIdentity\("t", "clip"\)\.effectId: .*is in linear, not db/);
    refused(() => tx.setEffectIdentity("t", "clip", "miso.not-an-effect"), /unknown native effect/);
    refused(() => tx.setEffectIdentity("t", "linked", "miso.parametric-eq"), /supports only the link modes 'dual_mono', not 'maximum'/);
    refused(() => tx.setEffectIdentity("t", "keyed", "miso.soft-clip"), /miso\.soft-clip declares no sidechain input/);
    refused(() => tx.setEffectQuality("t", "clip", "high"), /publish only the 'normal' quality/);
    refused(
      () => tx.setEffectSidechain("t", "clip", { source: { kind: "track", trackId: "t", tap: "input" }, portId: "sidechain-in" }),
      /setEffectSidechain\("t", "clip"\)\.sidechain\.portId: miso\.soft-clip declares no sidechain input/,
    );
    refused(() => tx.setTrackConsole("t", [{ slot: "eq" }]), /setTrackConsole\("t"\): .*change only after the same transaction changes the console/);
    for (const rampSamples of [-1, 1.5, 2 ** 32, "960"]) {
      refused(() => tx.setTrackConsole("t", [{ slot: "eq" }], { rampSamples }), /setTrackConsole\("t"\)\.rampSamples must be an integer/);
    }
    // A `setConsole` that leaves the base session's console as it was opens nothing (#1561 D3).
    refused(
      () => tx.setConsole({ preInsert: [{ slot: "eq", effectId: "miso.parametric-eq" }] }).setTrackConsole("t", [{ slot: "eq" }]),
      /setTrackConsole\("t"\): .*change only after the same transaction changes the console/,
    );
    refused(() => tx.build(), /at least one edit/);
  });

  test("session, console, submix, output and route edits the engine would refuse", () => {
    // Red if a session-level or graph edit the engine refuses (a rate off the launch list, a zero
    // quantum, a mono output, a console slot the console cannot hold, an ID another entity kind
    // holds, a missing submix, output or route, an endpoint outside the working graph, a following
    // route into the output) reaches the wire: the caller pays a round trip and loses the path.
    const base = session({ id: "tx.graph.refusals", sampleRateHz: 48_000 })
      .source("a", mono)
      .track("t", { source: "a" })
      .submix("bus")
      .output("main")
      .route({ id: "send", source: { kind: "track", trackId: "t", tap: "pre_fader" }, destination: { kind: "submix_input", submixId: "bus" } });
    const tx = transaction(base);
    refused(() => tx.setSessionId("Bad ID"), /setSessionId\(\)\.sessionId: a Session V1 stable ID/);
    refused(() => tx.setSampleRateHz(192_000), /setSampleRateHz\(\)\.sampleRateHz: 192000 is not a launch rate/);
    refused(() => tx.setQuantumFrames(0), /setQuantumFrames\(\)\.quantumFrames: expected an integer in 1/);
    refused(() => tx.setRenderProfile({ id: "native", mode: "waves" }), /setRenderProfile\(\)\.mode: single_thread is the only/);
    refused(() => tx.setOutputProfile({ id: "main", channels: 1 }), /setOutputProfile\(\)\.channels: V1 output must contain exactly two/);
    refused(() => tx.setConsole({ preInsert: [{ slot: "d", effectId: "miso.delay" }] }), /setConsole\(\)\.preInsert\[0\]\.effectId: 'miso\.delay' cannot be a console slot/);
    refused(() => tx.upsertSubmix("t"), /upsertSubmix\("t"\)\.id: tracks, submixes, VCAs and outputs share one ID namespace/);
    refused(() => tx.upsertOutput("bus"), /upsertOutput\("bus"\)\.id: tracks, submixes, VCAs and outputs share one ID namespace/);
    refused(() => tx.removeOutput("main").upsertTrack("main", { source: "a" }).upsertSubmix("main"), /upsertSubmix\("main"\)\.id: .*share one ID namespace/);
    refused(() => tx.upsertSubmix("fx", { console: [{ slot: "eq" }] }), /upsertSubmix\("fx"\)\.console\[0\]\.slot: 'eq' is not a console slot/);
    refused(() => tx.removeSubmix("missing"), /removeSubmix\("missing"\)\.id: 'missing' is not a declared submix/);
    refused(() => tx.removeOutput("missing"), /removeOutput\("missing"\)\.id: 'missing' is not a declared output/);
    refused(() => tx.removeRoute("missing"), /removeRoute\("missing"\)\.id: 'missing' is not a declared route/);
    refused(
      () => tx.removeSubmix("bus").upsertRoute({ id: "r", source: { kind: "submix", submixId: "bus", tap: "post_pan" }, destination: { kind: "output_input", outputId: "main" } }),
      /upsertRoute\("r"\)\.source\.submixId: 'bus' is not a declared submix/,
    );
    refused(
      () => tx.upsertRoute({ id: "r", source: { kind: "track", trackId: "t", tap: "input" }, destination: { kind: "output_input", outputId: "main" }, followsMute: true }),
      /upsertRoute\("r"\)\.followsMute: follows_mute applies only to a route into a submix/,
    );
    refused(() => tx.setRouteSource("send", { kind: "track", trackId: "t", tap: "pre_send" }), /setRouteSource\("send"\)\.source\.tap: unknown track tap/);
    refused(() => tx.setRouteDestination("send", { kind: "output_input", outputId: "missing" }), /setRouteDestination\("send"\)\.destination\.outputId: 'missing' is not a declared output/);
    // The base send follows its source's mute by default, so it cannot be pointed at the output.
    refused(() => tx.setRouteDestination("send", { kind: "output_input", outputId: "main" }), /setRouteDestination\("send"\)\.destination: follows_mute applies only/);
    refused(() => tx.build(), /at least one edit/);
  });

  test("an automation target or span the session builder refuses, with the builder's own text", () => {
    // Red if the transaction path skips `resolveAutomationTarget` or the span rules: it then
    // writes automation the store keeps, aimed at no control, or refuses only after a round trip,
    // with a message unlike the one `SessionBuilder.automation` gives for the same spec.
    const base = session({ id: "tx.automation.refusals", sampleRateHz: 48_000 })
      .source("a", mono)
      .track("t", { source: "a", inserts: [effect("miso.parametric-eq", {}, { slotId: "eq" })] })
      .automation({
        id: "ride",
        target: { owner: { kind: "strip", id: "t" }, section: { kind: "builtins" }, control: "fader_db", lane: "both" },
        segments: [{ shape: "linear", startSample: 0, endSample: 10, startValue: -6, endValue: 0 }],
      });
    const tx = transaction(base);
    const span = [{ shape: "linear", startSample: 0, endSample: 10, startValue: 0, endValue: 1 }];
    const targets = [
      { owner: { kind: "strip", id: "t" }, section: { kind: "inserts", effect: "gone" }, control: "band_1_gain_db", lane: "both" },
      { owner: { kind: "strip", id: "t" }, section: { kind: "inserts", effect: "eq" }, control: "band_1_gain", lane: "both" },
      { owner: { kind: "strip", id: "missing" }, section: { kind: "own" }, control: "solo", lane: "both" },
      { owner: { kind: "vca", id: "t" }, section: { kind: "own" }, control: "offset_db", lane: "both" },
      { owner: { kind: "session" }, section: { kind: "own" }, control: "master", lane: "both" },
    ];
    const builderText = (spec) => {
      try {
        base.automation(spec).toJSON();
      } catch (error) {
        assert.ok(error instanceof MisoUsageError);
        return error.message.replace(/^automation\("[^"]*"\)/, "");
      }
      assert.fail("the session builder refuses it");
    };
    const sameText = (call, prefix, text) => assert.throws(call, (error) => {
      assert.ok(error instanceof MisoUsageError, `expected a MisoUsageError, got ${error}`);
      assert.equal(error.message, `${prefix}${text}`);
      return true;
    });
    for (const target of targets) {
      const text = builderText({ id: "x", target, segments: span });
      sameText(() => tx.upsertAutomation({ id: "x", target, segments: span }), 'upsertAutomation("x")', text);
      sameText(() => tx.setAutomationTarget("ride", target), 'setAutomationTarget("ride")', text);
    }
    const rideTarget = { owner: { kind: "strip", id: "t" }, section: { kind: "builtins" }, control: "fader_db", lane: "both" };
    for (const segments of [[], [{ ...span[0], endSample: 0 }], [{ ...span[0], shape: "exponential" }]]) {
      const text = builderText({ id: "x", target: rideTarget, segments });
      sameText(() => tx.setAutomationSegments("ride", segments), 'setAutomationSegments("ride")', text);
      sameText(() => tx.upsertAutomation({ id: "x", target: rideTarget, segments }), 'upsertAutomation("x")', text);
    }
    // A span keeps its unit: a fader ride in dB cannot be re-pointed at a linear pan.
    refused(
      () => tx.setAutomationTarget("ride", { owner: { kind: "strip", id: "t" }, section: { kind: "builtins" }, control: "pan", lane: "both" }),
      /setAutomationTarget\("ride"\)\.target\.control: the row is in linear and this automation's spans are in db/,
    );
    refused(() => tx.removeAutomation("missing"), /removeAutomation\("missing"\)\.id: 'missing' is not a declared automation/);
    refused(() => tx.upsertVca("grp", { members: ["t", "t"] }), /upsertVca\("grp"\)\.members\[1\]: 't' is already a member/);
    refused(() => tx.upsertVca("grp", { members: ["nobody"] }), /upsertVca\("grp"\)\.members\[0\]: 'nobody' is not a declared track, submix or VCA/);
    refused(() => tx.upsertVca("grp", { members: [], fader: { leftDb: 30 } }), /upsertVca\("grp"\)\.fader\.leftDb: a VCA offset is in/);
    refused(() => tx.upsertVca("t", { members: [] }), /upsertVca\("t"\)\.id: tracks, submixes, VCAs and outputs share one ID namespace/);
    refused(() => tx.removeVca("missing"), /removeVca\("missing"\)\.id: 'missing' is not a declared VCA/);
    refused(() => tx.build(), /at least one edit/);
  });
});
