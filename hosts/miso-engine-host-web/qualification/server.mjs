import { createServer } from "node:http";
import { readFile, readdir, stat } from "node:fs/promises";
import path from "node:path";
import process from "node:process";
import { fileURLToPath } from "node:url";

const HERE = path.dirname(fileURLToPath(import.meta.url));
const HOST_WEB = path.dirname(HERE);
const FIXTURE = path.join(HOST_WEB, "tests", "browser-v1");
const DEMO = path.join(HOST_WEB, "demo");
const ARTIFACT_NAMES = new Set([
  "miso-engine-v2-audio-worklet.simd128.wasm",
  "miso-engine-v2-audio-worklet.js",
  "miso-engine-v2-audio-worklet-host.js",
  "miso-engine-v2-audio-worklet-host.d.ts",
]);
const CONTENT_TYPES = new Map([
  [".d.ts", "text/plain; charset=utf-8"],
  [".html", "text/html; charset=utf-8"],
  [".js", "text/javascript; charset=utf-8"],
  [".json", "application/json; charset=utf-8"],
  [".toml", "text/plain; charset=utf-8"],
  [".wasm", "application/wasm"],
]);

function contentType(file) {
  if (file.endsWith(".d.ts")) return CONTENT_TYPES.get(".d.ts");
  return CONTENT_TYPES.get(path.extname(file)) ?? "application/octet-stream";
}

async function exactArtifacts(directory) {
  const names = await readdir(directory);
  if (names.length !== ARTIFACT_NAMES.size || names.some((name) => !ARTIFACT_NAMES.has(name))) {
    throw new Error("artifact directory must contain the exact shipped four-file set");
  }
  for (const name of names) {
    if (!(await stat(path.join(directory, name))).isFile()) {
      throw new Error(`artifact is not a regular file: ${name}`);
    }
  }
}

function routePath(urlPath, artifactDirectory) {
  const routes = [
    ["/artifacts/", artifactDirectory],
    ["/fixture/", FIXTURE],
    ["/qualification/", HERE],
    ["/demo/", DEMO],
  ];
  if (urlPath === "/") return path.join(DEMO, "index.html");
  for (const [prefix, directory] of routes) {
    if (!urlPath.startsWith(prefix)) continue;
    const name = urlPath.slice(prefix.length);
    if (name.length === 0 || name.includes("/") || name === "." || name === "..") return null;
    if (prefix === "/artifacts/" && !ARTIFACT_NAMES.has(name)) return null;
    return path.join(directory, name);
  }
  return null;
}

export async function startQualificationServer({ artifacts, port = 0 }) {
  const artifactDirectory = path.resolve(artifacts);
  await exactArtifacts(artifactDirectory);
  const server = createServer(async (request, response) => {
    try {
      const url = new URL(request.url, "http://127.0.0.1");
      const file = routePath(decodeURIComponent(url.pathname), artifactDirectory);
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

function argument(name) {
  const index = process.argv.indexOf(name);
  return index === -1 ? null : process.argv[index + 1];
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const artifacts = argument("--artifacts");
  if (artifacts === null) {
    throw new Error("usage: npm run demo -- --artifacts ARTIFACT_DIRECTORY [--port PORT]");
  }
  const portValue = argument("--port");
  const port = portValue === null ? 4174 : Number(portValue);
  const running = await startQualificationServer({ artifacts, port });
  process.stdout.write(`Miso browser demo: ${running.origin}/\n`);
}
