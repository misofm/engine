import { IncrementalBlake3, prepareBlake3 } from "/web/stem-store/incremental-blake3.js";

// Use the supported default layout, without an override that could hide a broken asset URL.
try {
  const backend = await prepareBlake3();
  if (backend !== "simd128") throw new Error(`expected simd128, got ${backend}`);
  const response = await fetch("/stem-identity/VECTORS.tsv");
  if (!response.ok) throw new Error(`corpus fetch failed: ${response.status}`);
  const lines = (await response.text()).trim().split("\n");
  const columns = lines[1].split("\t");
  const rows = [];
  for (const line of lines.slice(2)) {
    const vector = Object.fromEntries(line.split("\t").map((value, index) => [columns[index], value]));
    const pcm = await fetch(`/stem-identity/${vector.pcm_file}`);
    if (!pcm.ok) throw new Error(`PCM fetch failed: ${pcm.status}`);
    const bytes = new Uint8Array(await pcm.arrayBuffer());
    const hash = new IncrementalBlake3();
    // Exercise the incremental boundary in the browser without owning another vector corpus.
    for (let at = 0; at < bytes.length; at += 3) hash.update(bytes.subarray(at, at + 3));
    const identity = `blake3:${hash.digestHex()}`;
    if (identity !== vector.identity) throw new Error(`${vector.name}: ${identity} != ${vector.identity}`);
    rows.push(vector.name);
  }
  if (rows.length === 0) throw new Error("empty identity corpus");
  self.postMessage({ backend, vectors: rows });
} catch (error) {
  self.postMessage({ error: String(error) });
}
