function hex(bytes) {
  return Array.from(bytes, (byte) => byte.toString(16).padStart(2, "0")).join("");
}

self.onmessage = async ({ data }) => {
  try {
    const loader = await import("/decoder-artifacts/flac-decoder.js");
    const decoder = await loader.loadPinnedFlacDecoder(
      "/decoder-artifacts/flac-decoder.wasm",
    );
    const rows = [];
    for (const vector of data.vectors) {
      const response = await fetch(`/flac-fixture/${vector.flacFile}`);
      if (!response.ok) throw new Error(`fixture fetch failed: ${vector.flacFile}`);
      const encoded = new Uint8Array(await response.arrayBuffer());
      const blocks = [];
      const blockFrames = [];
      let stream = null;
      for await (const block of decoder.decodeBlocks(encoded, {
        maximumCanonicalBytes: vector.canonicalBytes,
      })) {
        stream = block.stream;
        blocks.push(block.pcm);
        blockFrames.push(block.pcm.byteLength / (stream.channels * (stream.bitDepth / 8)));
      }
      const pcm = new Uint8Array(blocks.reduce((sum, block) => sum + block.byteLength, 0));
      let offset = 0;
      for (const block of blocks) {
        pcm.set(block, offset);
        offset += block.byteLength;
      }
      const digest = hex(new Uint8Array(await crypto.subtle.digest("SHA-256", pcm)));
      const mutated = pcm.slice();
      mutated[0] ^= 1;
      const mutatedDigest = hex(new Uint8Array(await crypto.subtle.digest("SHA-256", mutated)));
      const planes = loader.canonicalPcmBlockToPlanarF32(pcm, stream.channels, stream.bitDepth);
      rows.push({
        vector: vector.vector,
        flacFile: vector.flacFile,
        stream: { ...stream, frames: stream.frames.toString() },
        blockFrames,
        canonicalHex: hex(pcm),
        digest,
        mutatedDigest,
        firstPumpSample: planes[0][0],
      });
    }

    const artifactResponse = await fetch("/decoder-artifacts/flac-decoder.wasm");
    const mutatedArtifact = new Uint8Array(await artifactResponse.arrayBuffer());
    mutatedArtifact[mutatedArtifact.length - 1] ^= 1;
    let provenanceMutation = null;
    try {
      await loader.instantiatePinnedFlacDecoder(mutatedArtifact);
    } catch (error) {
      provenanceMutation = error?.code ?? null;
    }
    self.postMessage({
      schema: "miso.flac.browser.v1",
      worker: true,
      rows,
      provenanceMutation,
    });
  } catch (error) {
    self.postMessage({
      schema: "miso.flac.browser.error.v1",
      message: error?.message ?? String(error),
      stack: error?.stack ?? null,
    });
  }
};
