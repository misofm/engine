import { cp, mkdtemp, readdir, rm } from "node:fs/promises";
import os from "node:os";
import path from "node:path";
import process from "node:process";
import { fileURLToPath, pathToFileURL } from "node:url";

const HERE = path.dirname(fileURLToPath(import.meta.url));

/**
 * The browser bundle of the SDK a run qualifies: the qualification pages' entries, the PCM feed
 * worklet and the response Worker, built against `sdkRoot`'s distribution when it has one, else
 * against the repository's SDK source. Shared by `run.mjs` and `soak.mjs` (#2243).
 */
export async function buildSdkBundle(sdkRoot) {
  const esbuildPath = pathToFileURL(path.join(sdkRoot, "node_modules", "esbuild", "lib", "main.js")).href;
  const { build } = await import(esbuildPath);
  const bundle = await mkdtemp(path.join(os.tmpdir(), "miso-sdk-response-bundle-"));
  // A selected built distribution (including an unpacked npm archive) supplies every SDK runtime
  // import. The existing source-only CI mode remains available before package assembly.
  const sourceRoot = path.resolve(HERE, "../../../sdk/src");
  const distribution = path.join(sdkRoot, "dist");
  const useDistribution = await readdir(distribution).then((entries) => entries.includes("core"), () => false);
  const repositorySdk = sdkRoot === path.dirname(sourceRoot);
  if (!useDistribution && !repositorySdk) {
    throw new Error("selected SDK package has no built distribution");
  }
  // Say which SDK this run qualifies (#1097 verdict L5). A leftover `sdk/dist` -- from
  // `sdk-package.sh check`, say -- silently switches the repository's own SDK from CI's source
  // bundle to whatever that build left behind, so the evidence must name the bundle it used.
  process.stdout.write(!useDistribution
    ? `sdk bundle: the source at ${sourceRoot} (CI's mode)\n`
    : repositorySdk
      ? `sdk bundle: the built distribution at ${distribution}, not CI's source bundle; delete it `
        + "to qualify the source as CI does\n"
      : `sdk bundle: the selected package's built distribution at ${distribution}\n`);
  const shippedSdk = {
    name: "shipped-sdk",
    setup(builder) {
      builder.onResolve({ filter: /\.ts$/ }, (args) => {
        const resolved = path.resolve(args.resolveDir, args.path);
        if (!resolved.startsWith(`${sourceRoot}${path.sep}`)) return undefined;
        return { path: path.join(distribution, path.relative(sourceRoot, resolved).replace(/\.ts$/, ".js")) };
      });
    },
  };
  try {
    await build({
      entryPoints: [path.join(HERE, "sdk-response-entry.ts")],
      plugins: useDistribution ? [shippedSdk] : [],
      outfile: path.join(bundle, "sdk-response-client.js"),
      bundle: true,
      format: "esm",
      platform: "browser",
      target: "es2022",
      legalComments: "inline",
    });
    await build({
      entryPoints: [path.join(HERE, "pcm-feed-suspended-entry.ts")],
      plugins: useDistribution ? [shippedSdk] : [],
      outfile: path.join(bundle, "pcm-feed-suspended-client.js"),
      bundle: true, format: "esm", platform: "browser", target: "es2022",
    });
    // Issue #2243: the on-demand soak's page (`soak.mjs`).
    await build({
      entryPoints: [path.join(HERE, "soak-entry.ts")],
      plugins: useDistribution ? [shippedSdk] : [],
      outfile: path.join(bundle, "soak-client.js"),
      bundle: true, format: "esm", platform: "browser", target: "es2022",
    });
    await cp(useDistribution
      ? path.join(distribution, "assets", "miso-engine-v1-pcm-feed-worklet.js")
      : path.join(sourceRoot, "browser-assets", "miso-engine-v1-pcm-feed-worklet.js"),
    path.join(bundle, "miso-engine-v1-pcm-feed-worklet.js"));
    await build({
      entryPoints: [useDistribution
        ? path.join(distribution, "browser", "response-worker.js")
        : path.join(sourceRoot, "browser", "response-worker.ts")],
      outfile: path.join(bundle, "response-worker.js"),
      bundle: true,
      format: "esm",
      platform: "browser",
      target: "es2022",
      legalComments: "inline",
    });
    return bundle;
  } catch (error) {
    await rm(bundle, { recursive: true, force: true });
    throw error;
  }
}
