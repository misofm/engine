/**
 * Issue #1383 (slice 1383a): the SDK's session-transaction builder against the engine.
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
  test("every source and track edit, repeated fields at 0, 1 and 3, is the codec's bytes", () => {
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
});

describe("the builder refuses what the engine would refuse, before any byte exists", () => {
  test("an empty transaction, an unknown effect, an undeclared source and a fader out of domain", () => {
    // Red if the builder lets through a transaction the engine must refuse: the caller then pays a
    // round trip and gets the refusal without the path of the call that caused it.
    const base = session({ id: "tx.refusals", sampleRateHz: 48_000 }).source("a", mono);
    const tx = transaction(base);
    const refused = (call, pattern) => assert.throws(call, (error) => {
      assert.ok(error instanceof MisoUsageError, `expected a MisoUsageError, got ${error}`);
      assert.match(error.message, pattern);
      return true;
    });
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
});
