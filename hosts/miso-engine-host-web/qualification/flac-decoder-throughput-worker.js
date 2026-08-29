function hex(bytes) {
  return Array.from(bytes, (byte) => byte.toString(16).padStart(2, "0")).join("");
}

self.onmessage = async ({ data }) => {
  try {
    const loader = await import("/decoder-artifacts/miso-engine-flac-decoder.js");
    const decoder = await loader.loadPinnedFlacDecoder(
      "/decoder-artifacts/miso-engine-flac-decoder.wasm",
    );
    const response = await fetch("/throughput.flac");
    if (!response.ok) throw new Error("throughput fixture fetch failed");
    const encoded = new Uint8Array(await response.arrayBuffer());
    const blocks = [];
    let canonicalBytes = 0;
    const started = performance.now();
    for await (const block of decoder.decodeBlocks(encoded, {
      maximumCanonicalBytes: data.maximumCanonicalBytes,
    })) {
      blocks.push(block.pcm);
      canonicalBytes += block.pcm.byteLength;
    }
    const pcm = new Uint8Array(canonicalBytes);
    let offset = 0;
    for (const block of blocks) {
      pcm.set(block, offset);
      offset += block.byteLength;
    }
    const digest = hex(new Uint8Array(await crypto.subtle.digest("SHA-256", pcm)));
    const elapsedMs = performance.now() - started;
    self.postMessage({
      schema: "miso.flac.throughput.v1",
      canonicalBytes,
      elapsedMs,
      mebibytesPerSecond: (canonicalBytes / (1024 * 1024)) / (elapsedMs / 1000),
      digest,
    });
  } catch (error) {
    self.postMessage({
      schema: "miso.flac.browser.error.v1",
      message: error?.message ?? String(error),
      stack: error?.stack ?? null,
    });
  }
};
