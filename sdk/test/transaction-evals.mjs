/**
 * Issue #1383 (slices 1383a and 1383b): the SDK's session-transaction builder against the engine.
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
        ],
      });
    const tx = transaction(base);
    refused(() => tx.setTrackRack("missing", []), /setTrackRack\("missing"\)\.id: 'missing' is not a declared track or submix/);
    refused(() => tx.removeTrackEffect("t", "gone"), /removeTrackEffect\("t"\)\.slotId: 'gone' is not an insert/);
    refused(() => tx.putTrackEffect("t", 0, effect("miso.delay")), /putTrackEffect\("t"\)\.effect\.slotId/);
    refused(() => tx.putTrackEffect("t", 2, effect("miso.delay", {}, { slotId: "eq" })), /finalPosition: 2 is past the end of the 1 other/);
    refused(() => tx.putTrackEffect("t", -1, effect("miso.delay", {}, { slotId: "d" })), /finalPosition must be an integer/);
    refused(() => tx.setTrackEffectOrder("t", ["eq"]), /setTrackEffectOrder\("t"\)\.slotIds: expected each of the strip's inserts exactly once/);
    refused(() => tx.setTrackEffectOrder("t", ["eq", "eq"]), /exactly once/);
    refused(() => tx.setTrackEffectOrder("t", ["Bad", "eq"]), /slotIds\[0\]: a Session V1 stable ID/);
    // soft-clip's parameter 1 is drive in dB; the transient shaper's is attack amount, linear.
    refused(() => tx.setEffectIdentity("t", "clip", "miso.transient-shaper"), /setEffectIdentity\("t", "clip"\)\.effectId: .*is in linear, not db/);
    refused(() => tx.setEffectIdentity("t", "clip", "miso.not-an-effect"), /unknown native effect/);
    refused(() => tx.setEffectQuality("t", "clip", "high"), /publish only the 'normal' quality/);
    refused(
      () => tx.setEffectSidechain("t", "clip", { source: { kind: "track", trackId: "t", tap: "input" }, portId: "sidechain-in" }),
      /setEffectSidechain\("t", "clip"\)\.sidechain\.portId: miso\.soft-clip declares no sidechain input/,
    );
    refused(() => tx.setTrackConsole("t", [{ slot: "eq" }]), /setTrackConsole\("t"\): .*change only after the same transaction changes the console/);
    for (const rampSamples of [-1, 1.5, 2 ** 32, "960"]) {
      refused(() => tx.setTrackConsole("t", [{ slot: "eq" }], { rampSamples }), /setTrackConsole\("t"\)\.rampSamples must be an integer/);
    }
    refused(() => tx.build(), /at least one edit/);
  });
});
