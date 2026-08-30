// Issue #272: re-derive each qualification session document's declared source `content` from the
// PCM the harness actually feeds, and refuse a document that declares anything else.
//
// The #241 migration minted these values from the old locator *names* --
// `sha256("web-browser-console")` and friends -- so three documents claimed canonical-PCM
// identities of audio that was never hashed. Nothing was red, because `content` is grammar-checked on this leg and no browser
// gate reads it. This check closes that: it walks the exported generator behind each document
// (`qualificationSessionSources` in `qualification.js` -- the same function the browser feeds),
// serializes it per `docs/STEM_IDENTITY_V1.md`, and compares.
//
// Deriving rather than pinning is the point. A frozen hex string would have to be edited in step
// with any generator change, and the pre-#272 state is exactly what "edited out of step" looks
// like. Here a changed generator moves the derived identity and the unchanged document goes red.
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFile } from "node:fs/promises";
import path from "node:path";
import process from "node:process";
import { fileURLToPath, pathToFileURL } from "node:url";
import { qualificationConstants, qualificationSessionSources } from "./qualification.js";

const HERE = path.dirname(fileURLToPath(import.meta.url));
const BYTES_PER_SAMPLE = Float32Array.BYTES_PER_ELEMENT;

/// `docs/STEM_IDENTITY_V1.md` "Canonical serialization": samples only -- no header, no length
/// prefix, no shape word -- interleaved frame-major, each `32f` sample its raw four-byte binary32
/// pattern little-endian, preimage length exactly `frames * channels * 4`, digest SHA-256. This is
/// deliberately *not* the planar layout `qualification.js::pcmDigest` uses for render digests: that
/// one is channel-major and answers a different question.
export function deriveSessionIdentity(row) {
  const { quantumFrames, sourceChannels } = qualificationConstants;
  const hash = createHash("sha256");
  const bytes = new ArrayBuffer(quantumFrames * sourceChannels * BYTES_PER_SAMPLE);
  const view = new DataView(bytes);
  const block = new Uint8Array(bytes);
  for (let blockIndex = 0; blockIndex < row.blocks; blockIndex += 1) {
    const planes = row.planes(blockIndex);
    assert.equal(planes.length, sourceChannels, `${row.document}: generator channel count`);
    let offset = 0;
    for (let frame = 0; frame < quantumFrames; frame += 1) {
      for (const plane of planes) {
        view.setFloat32(offset, plane[frame], true);
        offset += BYTES_PER_SAMPLE;
      }
    }
    hash.update(block);
  }
  return {
    document: row.document,
    sourceId: row.sourceId,
    frames: row.blocks * quantumFrames,
    identity: hash.digest("hex"),
  };
}

/// The exact source row the document must carry. Shape and identity travel together because
/// `STEM_IDENTITY_V1` fixes the preimage length at `frames * channels * bytes_per_sample`: the same
/// digest under a different `frames` is not a smaller mistake, it is an impossible pair.
export function expectedSourceRow(derived) {
  const { sourceChannels, sourceBitDepth } = qualificationConstants;
  return `{ id = "${derived.sourceId}", content = "sha256:${derived.identity}", `
    + `channels = ${sourceChannels}, bit_depth = "${sourceBitDepth}", `
    + `frames = ${derived.frames} },`;
}

export async function checkSessionIdentities(directory = HERE) {
  const rows = [];
  for (const source of qualificationSessionSources) {
    const derived = deriveSessionIdentity(source);
    const document = await readFile(path.join(directory, source.document), "utf8");
    const expected = expectedSourceRow(derived);
    if (!document.includes(expected)) {
      throw new Error(
        `session-identity: ${source.document}: declared source row is not the fed PCM's canonical `
        + `identity; expected ${expected}`,
      );
    }
    // One source row per document, so a truthful row cannot sit beside a stale one.
    const declared = document.match(/content = "sha256:[0-9a-f]{64}"/g) ?? [];
    if (declared.length !== 1) {
      throw new Error(
        `session-identity: ${source.document}: expected exactly one source content identity, `
        + `found ${declared.length}`,
      );
    }
    rows.push(derived);
  }
  // Red proof carried with the check: one flipped hex digit in the derived identity must stop
  // matching the document. Without this the check could pass vacuously if the comparison were ever
  // loosened, which is the failure mode that let the name-minted values stand.
  for (const derived of rows) {
    const head = derived.identity[0] === "0" ? "1" : "0";
    const flipped = { ...derived, identity: `${head}${derived.identity.slice(1)}` };
    const document = await readFile(path.join(directory, derived.document), "utf8");
    assert.ok(
      !document.includes(expectedSourceRow(flipped)),
      `session-identity: ${derived.document}: flipped-digit red proof escaped its gate`,
    );
  }
  return rows;
}

async function main() {
  const rows = await checkSessionIdentities();
  for (const row of rows) {
    process.stdout.write(`${row.document}: ${row.frames} frames, sha256:${row.identity}\n`);
  }
  process.stdout.write("session identities: all qualification documents declare their fed PCM\n");
}

if (process.argv[1] !== undefined && import.meta.url === pathToFileURL(process.argv[1]).href) {
  await main();
}
