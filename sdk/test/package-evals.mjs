/**
 * Issue #278 gaps 1 and 2: the package's entry points are real, and the barrels carry them.
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
 * The fix is the vendoring contract (`sdk/README.md`, "Consuming this package"): the three entry
 * points name the three real source barrels, and an app vendors `sdk/src/**` and imports through
 * them. This eval is what makes that a contract rather than a paragraph: it resolves every declared
 * subpath through Node's own resolver, which fails on a missing file, so a future `exports` entry
 * that points at an artifact nobody builds cannot pass.
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
import { pathToFileURL } from "node:url";

import * as rootBarrel from "../src/index.ts";
import * as browserBarrel from "../src/browser/index.ts";
import * as headlessBarrel from "../src/headless/index.ts";

import * as agent from "../src/core/agent.ts";
import * as hostMirror from "../src/browser/host-mirror.ts";
import * as lattice from "../src/core/lattice.ts";
import * as writer from "../src/core/writer.ts";

const SDK_ROOT = resolve(import.meta.dirname, "..");

describe("package entry points", () => {
  test("every declared subpath resolves to a file that exists", async () => {
    const manifest = JSON.parse(await readFile(resolve(SDK_ROOT, "package.json"), "utf8"));
    const subpaths = Object.entries(manifest.exports);
    assert.deepEqual(
      subpaths.map(([subpath]) => subpath),
      [".", "./headless", "./browser"],
      "the three entry points are the three barrels",
    );
    for (const [subpath, target] of subpaths) {
      assert.equal(
        typeof target,
        "string",
        `${subpath} is a plain target: there is no build, so there is no condition to branch on`,
      );
      assert.doesNotMatch(
        target,
        /(^|\/)dist(\/|$)/,
        `${subpath} points at ./dist, which no script in this repo produces`,
      );
      // `import()` runs Node's resolver and its loader: a target that does not exist throws
      // ERR_MODULE_NOT_FOUND here, which is the failure the dist map used to hand a consumer.
      const module = await import(pathToFileURL(resolve(SDK_ROOT, target)).href);
      assert.ok(
        Object.keys(module).length > 0,
        `${subpath} resolved but exported nothing`,
      );
    }
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
      const target = manifest.exports[subpath === "." ? "." : subpath];
      assert.ok(target !== undefined, `README imports an undeclared subpath: ${specifier}`);
      const module = await import(pathToFileURL(resolve(SDK_ROOT, target)).href);
      for (const binding of bindings.split(",").map((name) => name.trim())) {
        if (binding.length === 0) continue;
        assert.ok(binding in module, `${specifier} does not export ${binding}`);
      }
    }
  });

  test("the root entry is the module the evals deep-import", async () => {
    const manifest = JSON.parse(await readFile(resolve(SDK_ROOT, "package.json"), "utf8"));
    const root = await import(pathToFileURL(resolve(SDK_ROOT, manifest.exports["."])).href);
    assert.equal(root.parameter, rootBarrel.parameter);
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
