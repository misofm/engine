import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFile, readdir } from "node:fs/promises";
import path from "node:path";
import { pathToFileURL } from "node:url";

const artifactDirectory = path.resolve(process.argv[2] ?? "");
if (process.argv.length !== 3) {
  throw new Error("usage: node scripts/check-flac-decoder.mjs ARTIFACT_DIRECTORY");
}
assert.deepEqual((await readdir(artifactDirectory)).sort(), [
  "decoder-artifact.sha256",
  "miso-engine-flac-decoder.d.ts",
  "miso-engine-flac-decoder.js",
  "miso-engine-flac-decoder.wasm",
]);

const artifact = new Uint8Array(await readFile(
  path.join(artifactDirectory, "miso-engine-flac-decoder.wasm"),
));
const expected = (await readFile(
  path.join(artifactDirectory, "decoder-artifact.sha256"),
  "utf8",
)).trim();
assert.match(expected, /^[0-9a-f]{64}$/);
assert.equal(createHash("sha256").update(artifact).digest("hex"), expected);
const module = await WebAssembly.compile(artifact);
assert.deepEqual(WebAssembly.Module.imports(module), []);

const loaderUrl = pathToFileURL(
  path.join(artifactDirectory, "miso-engine-flac-decoder.js"),
);
const loader = await import(`${loaderUrl.href}?artifact-check`);
assert.equal(loader.MISO_ENGINE_FLAC_DECODER_SHA256, expected);
const decoder = await loader.instantiatePinnedFlacDecoder(artifact);

const root = path.resolve(import.meta.dirname, "..");
const manifest = await readFile(
  path.join(root, "fixtures/flac-delivery/v1/FLAC_VECTORS.tsv"),
  "utf8",
);
const lines = manifest.trimEnd().split("\n");
assert.equal(lines[0], "schema_version\t1");
assert.equal(
  lines[1],
  "vector\tbit_depth\tchannels\tframes\tconfigured_block_frames\tidentity\tpcm_file\tflac_file\tflac_sha256",
);
for (const line of lines.slice(2)) {
  const [vector, bitDepth, channels, frames, , identity, pcmFile, flacFile, flacSha256] =
    line.split("\t");
  const flac = new Uint8Array(await readFile(
    path.join(root, "fixtures/flac-delivery/v1", flacFile),
  ));
  assert.equal(createHash("sha256").update(flac).digest("hex"), flacSha256, flacFile);
  const expectedPcm = new Uint8Array(await readFile(
    path.join(root, "fixtures/stem-identity/v1", pcmFile),
  ));
  const blocks = [];
  let stream;
  for await (const block of decoder.decodeBlocks(flac, {
    maximumCanonicalBytes: expectedPcm.byteLength,
  })) {
    stream = block.stream;
    blocks.push(block.pcm);
  }
  const actual = new Uint8Array(blocks.reduce((sum, block) => sum + block.byteLength, 0));
  let offset = 0;
  for (const block of blocks) {
    actual.set(block, offset);
    offset += block.byteLength;
  }
  assert.deepEqual(actual, expectedPcm, vector);
  assert.equal(stream.bitDepth, Number(bitDepth));
  assert.equal(stream.channels, Number(channels));
  assert.equal(stream.frames, BigInt(frames));
  assert.equal(`sha256:${createHash("sha256").update(actual).digest("hex")}`, identity);

  const mutated = actual.slice();
  mutated[0] ^= 1;
  assert.notEqual(
    `sha256:${createHash("sha256").update(mutated).digest("hex")}`,
    identity,
    `${vector}: one-LSB mutation escaped`,
  );
}

const mutatedArtifact = artifact.slice();
mutatedArtifact[mutatedArtifact.length - 1] ^= 1;
await assert.rejects(
  loader.instantiatePinnedFlacDecoder(mutatedArtifact),
  (error) => error?.code === "miso.flac.decoder.artifact_mismatch",
  "one-byte decoder artifact mutation escaped provenance gate",
);

const pcm16 = new Uint8Array([0x00, 0x80, 0xff, 0x7f]);
const [pcm16Plane] = loader.canonicalPcmBlockToPlanarF32(pcm16, 1, 16);
assert.deepEqual(Array.from(pcm16Plane), [-1, 32767 / 32768]);
const pcm24 = new Uint8Array([0x00, 0x00, 0x80, 0xff, 0xff, 0x7f]);
const [pcm24Plane] = loader.canonicalPcmBlockToPlanarF32(pcm24, 1, 24);
assert.deepEqual(Array.from(pcm24Plane), [-1, 8388607 / 8388608]);

process.stdout.write("FLAC decoder artifact, provenance, vectors, red mutation, and pump gates passed\n");
