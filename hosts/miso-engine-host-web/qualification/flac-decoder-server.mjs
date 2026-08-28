import { createServer } from "node:http";
import { readFile, readdir, stat } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const HERE = path.dirname(fileURLToPath(import.meta.url));
const ROOT = path.resolve(HERE, "../../..");
const FLAC_FIXTURES = path.join(ROOT, "fixtures/flac-delivery/v1/flac");
const PCM_FIXTURES = path.join(ROOT, "fixtures/stem-identity/v1");
const ARTIFACT_NAMES = new Set([
  "decoder-artifact.sha256",
  "miso-engine-flac-decoder.d.ts",
  "miso-engine-flac-decoder.js",
  "miso-engine-flac-decoder.wasm",
]);

async function exactArtifacts(directory) {
  const names = await readdir(directory);
  if (names.length !== ARTIFACT_NAMES.size || names.some((name) => !ARTIFACT_NAMES.has(name))) {
    throw new Error("decoder artifact directory must contain the exact shipped four-file set");
  }
  for (const name of names) {
    if (!(await stat(path.join(directory, name))).isFile()) {
      throw new Error(`decoder artifact is not a regular file: ${name}`);
    }
  }
}

function routePath(urlPath, artifactDirectory, throughputFlac) {
  const routes = [
    ["/decoder-artifacts/", artifactDirectory, ARTIFACT_NAMES],
    ["/flac-fixture/", FLAC_FIXTURES, null],
    ["/pcm-fixture/", PCM_FIXTURES, null],
    ["/qualification/", HERE, null],
  ];
  if (urlPath === "/") return path.join(HERE, "index.html");
  if (urlPath === "/throughput.flac") return throughputFlac;
  for (const [prefix, directory, names] of routes) {
    if (!urlPath.startsWith(prefix)) continue;
    const name = urlPath.slice(prefix.length);
    if (name.length === 0 || name.includes("/") || name === "." || name === "..") return null;
    if (names !== null && !names.has(name)) return null;
    return path.join(directory, name);
  }
  return null;
}

function contentType(file) {
  if (file.endsWith(".js")) return "text/javascript; charset=utf-8";
  if (file.endsWith(".wasm")) return "application/wasm";
  if (file.endsWith(".html")) return "text/html; charset=utf-8";
  return "application/octet-stream";
}

export async function startFlacDecoderServer({ artifacts, throughputFlac = null, port = 0 }) {
  const artifactDirectory = path.resolve(artifacts);
  await exactArtifacts(artifactDirectory);
  const throughputFile = throughputFlac === null ? null : path.resolve(throughputFlac);
  if (throughputFile !== null && !(await stat(throughputFile)).isFile()) {
    throw new Error("throughput FLAC must be a regular file");
  }
  const server = createServer(async (request, response) => {
    try {
      const url = new URL(request.url, "http://127.0.0.1");
      const file = routePath(decodeURIComponent(url.pathname), artifactDirectory, throughputFile);
      if (request.method !== "GET" || file === null) {
        response.writeHead(404).end();
        return;
      }
      const body = await readFile(file);
      response.writeHead(200, {
        "Cache-Control": "no-store",
        "Content-Length": body.byteLength,
        "Content-Type": contentType(file),
        "X-Content-Type-Options": "nosniff",
      });
      response.end(body);
    } catch (_error) {
      response.writeHead(404).end();
    }
  });
  await new Promise((resolve, reject) => {
    server.once("error", reject);
    server.listen(port, "127.0.0.1", resolve);
  });
  const address = server.address();
  return {
    origin: `http://127.0.0.1:${address.port}`,
    close: () => new Promise((resolve, reject) => {
      server.close((error) => error === undefined ? resolve() : reject(error));
    }),
  };
}
