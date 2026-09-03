/**
 * Issues #278 and #320: the package's entry points are built, and the barrels carry them.
 *
 * # What went wrong, and why a grep would not have caught it
 *
 * `sdk/package.json` exported `./dist/index.js`, `./dist/headless/index.js` and
 * `./dist/browser/index.js`. No script produced any of them -- `tsconfig.json` is `noEmit` and
 * there is no build -- so every one of the three subpaths was a specifier that resolved to
 * nothing. Nothing in the repo noticed, because the SDK's own evals deep-import `../src/**` by
 * relative path and never exercise the package's public specifiers at all. The one consumer-facing
 * example in `sdk/README.md` used the root import and had been broken since it was written.
 *
 * #320 replaces the temporary vendoring contract with a real build. This hermetic suite does not
 * have TypeScript installed and therefore checks the export-map SHAPE against the source barrels;
 * `package-tarball-smoke.mjs` is the independent packed-artifact gate that resolves and imports the
 * emitted files after `npm ci`.
 *
 * # The barrel half
 *
 * `sdk/test/barrel-surface.ts` pins the TYPES the barrels re-export; `tsc` erases before this file
 * runs, so it cannot. What it can check is the other half of the same claim: that the value the
 * barrel exports is the very object the deep import returns -- identity, not a same-shaped
 * re-declaration. `assert.equal` on a function reference is exactly that check.
 */

import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { describe, test } from "node:test";
import { resolve } from "node:path";

import * as rootBarrel from "../src/index.ts";
import * as browserBarrel from "../src/browser/index.ts";
import * as headlessBarrel from "../src/headless/index.ts";

import * as agent from "../src/core/agent.ts";
import * as hostMirror from "../src/browser/host-mirror.ts";
import * as lattice from "../src/core/lattice.ts";
import * as writer from "../src/core/writer.ts";

const SDK_ROOT = resolve(import.meta.dirname, "..");

describe("package entry points", () => {
  test("the internal canonical serializer is absent from every public runtime barrel", () => {
    for (const [name, barrel] of Object.entries({
      root: rootBarrel,
      headless: headlessBarrel,
      browser: browserBarrel,
    })) {
      assert.equal("canonicalSessionJson" in barrel, false, `${name} leaked canonicalSessionJson`);
      assert.equal(
        "writeCanonicalSessionDocument" in barrel,
        false,
        `${name} leaked the internal canonical serializer`,
      );
    }
  });

  test("every declared code subpath names emitted ESM and declarations", async () => {
    const manifest = JSON.parse(await readFile(resolve(SDK_ROOT, "package.json"), "utf8"));
    const subpaths = Object.entries(manifest.exports);
    assert.deepEqual(
      subpaths.map(([subpath]) => subpath),
      [".", "./headless", "./browser", "./assets", "./package.json"],
      "the four code entries plus package metadata are the public package surface",
    );
    for (const [subpath, target] of subpaths) {
      if (subpath === "./package.json") {
        assert.equal(target, "./package.json");
        continue;
      }
      assert.equal(
        typeof target.import,
        "string",
        `${subpath} has an ESM import target`,
      );
      assert.equal(typeof target.types, "string", `${subpath} has a declaration target`);
      assert.match(target.import, /^\.\/dist\/.*\.js$/);
      assert.match(target.types, /^\.\/dist\/.*\.d\.ts$/);
    }
    assert.equal(manifest.private, undefined, "the SDK is publishable");
    assert.deepEqual(manifest.files, ["dist"], "only the prepared package tree is published");
    assert.equal(manifest.dependencies, undefined, "the SDK has no runtime dependencies");
    assert.equal(manifest.peerDependencies, undefined, "the SDK has no peer dependencies");
    assert.equal(manifest.peerDependenciesMeta, undefined, "the SDK has no peer metadata");
    assert.equal(
      manifest.devDependencies?.effect,
      undefined,
      "the SDK does not retain Effect as a development-only dependency",
    );
  });

  test("every import example in the README resolves", async () => {
    // The example that motivated #278 -- `import { catalog, parameter } from "@misofm/engine"` --
    // had been wrong since it was written, and stayed wrong because nothing read it. Reading it
    // here is cheap, and it turns the README's four code blocks into gated claims.
    const readme = await readFile(resolve(SDK_ROOT, "README.md"), "utf8");
    const manifest = JSON.parse(await readFile(resolve(SDK_ROOT, "package.json"), "utf8"));
    const examples = [
      ...readme.matchAll(/^import \{([^}]+)\} from "(@misofm\/engine[^"]*)";$/gm),
    ];
    assert.ok(examples.length >= 4, "the README still carries its import examples");
    for (const [, bindings, specifier] of examples) {
      const subpath = specifier.replace("@misofm/engine", ".").replace("./", "./");
      assert.ok(manifest.exports[subpath === "." ? "." : subpath] !== undefined,
        `README imports an undeclared subpath: ${specifier}`);
      const module = subpath === "." ? rootBarrel
        : subpath === "./headless" ? headlessBarrel
          : subpath === "./browser" ? browserBarrel
            : await import("../src/assets.ts");
      for (const binding of bindings.split(",").map((name) => name.trim())) {
        if (binding.length === 0) continue;
        assert.ok(binding in module, `${specifier} does not export ${binding}`);
      }
    }
  });

  test("the root entry is the module the evals deep-import", async () => {
    assert.equal(rootBarrel.parameter, agent.parameter);
  });
});

describe("barrel reachability", () => {
  test("core/agent is on the root barrel, by identity", () => {
    assert.equal(rootBarrel.catalog, agent.catalog);
    assert.equal(rootBarrel.parameter, agent.parameter);
    assert.equal(rootBarrel.decimalToFloat32, agent.decimalToFloat32);
    assert.equal(rootBarrel.ParameterHandle, agent.ParameterHandle);
  });

  test("core/writer is on the root barrel, by identity", () => {
    assert.equal(rootBarrel.ConsoleWriter, writer.ConsoleWriter);
  });

  test("core/lattice is on the root barrel, by identity", () => {
    assert.equal(rootBarrel.latticePoints, lattice.latticePoints);
    assert.equal(rootBarrel.resolveStep, lattice.resolveStep);
    assert.equal(rootBarrel.indexForDecimal, lattice.indexForDecimal);
    assert.equal(rootBarrel.STEP_SIZES, lattice.STEP_SIZES);
    assert.equal(rootBarrel.MAXIMUM_LATTICE_POINTS, lattice.MAXIMUM_LATTICE_POINTS);
  });

  test("browser/host-mirror is on the browser barrel, by identity", () => {
    assert.equal(browserBarrel.toWebBootOptions, hostMirror.toWebBootOptions);
  });

  test("the pre-#278 barrel surface is unchanged", () => {
    // `StepDeclaration`/`StepSizeName` are types, so the collision the lattice export had to
    // resolve leaves no runtime trace. What is observable is that adding three modules to the root
    // barrel shadowed nothing: every name the barrel carried before is still the same object.
    assert.equal(typeof rootBarrel.session, "function");
    assert.equal(typeof rootBarrel.effect, "function");
    assert.equal(typeof rootBarrel.SessionBuilder, "function");
    assert.equal(typeof rootBarrel.MisoEngineAsset, "function");
    assert.equal(rootBarrel.CATALOG.effects.length > 0, true);
    assert.equal(typeof rootBarrel.ABI_LAYOUT.constants, "object");
    assert.equal(typeof rootBarrel.PROVENANCE.abiVersion, "number");
    assert.equal(headlessBarrel.MisoEngineAsset, rootBarrel.MisoEngineAsset);
    assert.equal(typeof browserBarrel.scratchBootOptions, "function");
  });
});
