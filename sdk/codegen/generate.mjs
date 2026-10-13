#!/usr/bin/env node
/**
 * Transcribe `sdk/assets/*.json` into TypeScript data modules.
 *
 * This generator deliberately has no engine-specific table of effects, parameters, ABI offsets,
 * result codes or command reasons. The checked-in JSON documents are the only data authority, and
 * they in turn are the engine's own `--print` output (see `codegen/assets.mjs`). `--check`
 * compares the byte-for-byte expected modules with the checked-in copies, so a manual edit to a
 * generated module fails before a consumer can observe the drift.
 *
 * # Why the modules are `as const` values rather than hand-written interfaces
 *
 * Every type the SDK exposes over the catalog and the ABI is *derived* from the emitted value:
 * `EffectId` is the union of the ids actually in the document, `AbiStructureName` is the union of
 * the structures actually emitted. An effect the engine dropped cannot be named in TypeScript, and
 * an ABI field that moved cannot be read at a stale offset, because the offset is read from the
 * same frozen value the type came from. Nothing here restates a fact; it re-shapes one.
 *
 * # The provenance module
 *
 * `provenance.ts` records what the SDK was built against -- ABI version, schema tags, artifact
 * file names -- and deliberately carries **no content hashes**. The release's asset digests belong
 * to the release, not to the source tree: the browser module is a build output, so a checked-in
 * hash would either pin one machine's build or rot on the first rebuild. The SDK verifies the
 * digest a caller supplies from its own release manifest, and pins the two facts that *are*
 * source-derived: the ABI version word and the artifact set. See `src/core/errors.ts` for how the
 * three skew detectors compose.
 */

import { readFile, writeFile } from "node:fs/promises";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import process from "node:process";

const here = dirname(fileURLToPath(import.meta.url));
const sdkRoot = resolve(here, "..");
const assets = resolve(sdkRoot, "assets");
const generated = resolve(sdkRoot, "src", "generated");

const inputs = [
  ["catalog", "miso-engine-v1-parameter-metadata.json", "CATALOG"],
  ["abi", "miso-engine-v1-abi-layout.json", "ABI_LAYOUT"],
];

/** The release artifact set, in the order `scripts/check-web-audioworklet.sh` pins it. */
const ARTIFACT_SET = [
  "miso-engine-v1-abi-layout.json",
  "miso-engine-v1-audio-worklet-host.d.ts",
  "miso-engine-v1-audio-worklet-host.js",
  "miso-engine-v1-audio-worklet.js",
  "miso-engine-v1-audio-worklet.simd128.wasm",
  "miso-engine-v1-parameter-metadata.json",
  "prepared-control.js",
];

function deepFreezeSource() {
  return `function deepFreeze<T>(value: T): T {\n`
    + `  if (value !== null && typeof value === "object") {\n`
    + `    Object.freeze(value);\n`
    + `    for (const child of Object.values(value as Record<string, unknown>)) {\n`
    + `      deepFreeze(child);\n`
    + `    }\n`
    + `  }\n`
    + `  return value;\n`
    + `}\n\n`;
}

function moduleTypes(kind, symbol) {
  if (kind === "catalog") {
    return `export type Catalog = typeof ${symbol};\n`
      + `export type BuiltinParameter = Catalog["builtins"]["parameters"][number];\n`
      + `export type BuiltinParameterName = BuiltinParameter["name"];\n`
      + `export type EffectDescriptor = Catalog["effects"][number];\n`
      + `export type EffectId = EffectDescriptor["id"];\n`
      + `export type EffectParameter<E extends EffectId> =\n`
      + `  Extract<EffectDescriptor, { readonly id: E }>["parameters"][number];\n`
      + `export type EffectParameterName<E extends EffectId> = EffectParameter<E>["name"];\n`
      + `/** One declared port row: its id, role, \`required\` flag and lane layout (issue #278). */\n`
      + `export type EffectPort<E extends EffectId> =\n`
      + `  Extract<EffectDescriptor, { readonly id: E }>["ports"][number];\n`
      + `export type PortName<E extends EffectId> = EffectPort<E>["id"];\n`
      + `/**\n`
      + ` * The ports a routed sidechain may name.\n`
      + ` *\n`
      + ` * \`never\` for an effect that declares no sidechain input, which is what makes a routed\n`
      + ` * sidechain on such an effect unconstructible rather than merely refused at runtime.\n`
      + ` */\n`
      + `export type SidechainPortName<E extends EffectId> =\n`
      + `  Extract<EffectPort<E>, { readonly roleName: "sidechainInput" }>["id"];\n`
      + `export type EffectObservation<E extends EffectId> =\n`
      + `  Extract<EffectDescriptor, { readonly id: E }>["observations"][number];\n`
      + `export type TapName<E extends EffectId> = EffectObservation<E>["name"];\n`
      + `export type CommandKindName = Catalog["commandKinds"][number]["name"];\n`
      + `export type CommandReasonName = Catalog["commandReasons"][number]["name"];\n`
      + `/** The lattice declaration every parameter row carries (issue #242). */\n`
      + `export type StepDeclaration = EffectParameter<EffectId>["step"];\n`
      + `export type StepSizeName = keyof StepDeclaration["ladder"];\n`;
  }
  return `export type AbiLayout = typeof ${symbol};\n`
    + `export type AbiStructureName = keyof AbiLayout["structures"];\n`
    + `export type AbiStructure = AbiLayout["structures"][AbiStructureName];\n`
    + `export type AbiField = AbiStructure["fields"][number];\n`
    + `export type AbiConstantName = keyof AbiLayout["constants"];\n`
    + `export type AbiCommandField = AbiLayout["commandRecord"]["fields"][number];\n`
    + `/** Where a typed SDK refusal was raised. Replaces the dead two-phase "compile". */\n`
    + `export type ErrorPhase = AbiLayout["errorPhases"][number];\n`
    + `export type ResultCodeName = AbiLayout["constants"]["resultCodes"][number]["name"];\n`
    + `/** Boot's return reads through this table; every other call reads the base names. */\n`
    + `export type BootResultAliasName =\n`
    + `  AbiLayout["constants"]["bootResultAliases"][number]["name"];\n`
    + `export type BufferKindName = AbiLayout["constants"]["bufferKinds"][number]["name"];\n`
    + `export type BootExportName = AbiLayout["stagingSequence"][number];\n`
    + `/** Every module export, so no SDK call site types a symbol name. */\n`
    + `export type ExportName = AbiLayout["exports"][number];\n`;
}

function renderModule(kind, sourceName, symbol, value) {
  // `controls` is the catalog document's own key, rendered as its own module (`controls.ts`).
  // Leaving it out here keeps `catalog.ts` the catalog and the control rows in one place.
  if (kind === "catalog") {
    const { controls: _controls, ...rest } = value;
    value = rest;
  }
  const json = JSON.stringify(value, null, 2);
  return `// @generated by sdk/codegen/generate.mjs from sdk/assets/${sourceName}; DO NOT EDIT.\n`
    + `// That JSON is the engine's own \`parameter-metadata\` output; run\n`
    + `// \`npm run assets\` then \`npm run codegen\` in sdk/ to refresh both.\n\n`
    + deepFreezeSource()
    + `export const ${symbol} = deepFreeze(\n${json} as const,\n);\n\n`
    + moduleTypes(kind, symbol);
}

/**
 * The control types (decision 17, slice S2b): the catalog document's `controls` object as a
 * frozen value, and the address and name types derived from it. Nothing here names a control: a
 * row the core's tables gain reaches these types when \`sdk/assets\` is regenerated.
 */
function renderControls(catalog) {
  if (catalog.controls === undefined) {
    throw new Error("the parameter metadata has no `controls` object; regenerate sdk/assets");
  }
  const json = JSON.stringify(catalog.controls, null, 2);
  return `// @generated by sdk/codegen/generate.mjs from sdk/assets/miso-engine-v1-parameter-metadata.json; DO NOT EDIT.\n`
    + `// That JSON is the engine's own \`parameter-metadata\` output (its \`controls\` object, written\n`
    + `// from the \`controls\` crate's row tables); run \`npm run assets\` then \`npm run codegen\` in\n`
    + `// sdk/ to refresh both.\n\n`
    + `import type { EffectId } from "./catalog.ts";\n\n`
    + deepFreezeSource()
    + `export const CONTROLS = deepFreeze(\n${json} as const,\n);\n\n`
    + `export type Controls = typeof CONTROLS;\n`
    + `/** One row of a control table: its machine name, display strings, domain, class and ramp. */\n`
    + `export type ControlRow =\n`
    + `  | Controls["stripOwn"][number]\n`
    + `  | Controls["builtins"][number]\n`
    + `  | Controls["routeOwn"][number]\n`
    + `  | Controls["vcaOwn"][number]\n`
    + `  | Controls["sessionOwn"][number]\n`
    + `  | Controls["effects"][EffectId]["bypass"]\n`
    + `  | Controls["effects"][EffectId]["parameters"][number]\n`
    + `  | NonNullable<Controls["effects"][EffectId]["linkMode"]>;\n`
    + `export type StripOwnControlName = Controls["stripOwn"][number]["name"];\n`
    + `export type BuiltinControlName = Controls["builtins"][number]["name"];\n`
    + `export type RouteControlName = Controls["routeOwn"][number]["name"];\n`
    + `export type VcaControlName = Controls["vcaOwn"][number]["name"];\n`
    + `export type SessionControlName = Controls["sessionOwn"][number]["name"];\n`
    + `/** An effect's bypass, each parameter it declares, and its link mode if it has a choice. */\n`
    + `export type EffectControlName<E extends EffectId> =\n`
    + `  | Controls["effects"][E]["bypass"]["name"]\n`
    + `  | Controls["effects"][E]["parameters"][number]["name"]\n`
    + `  | NonNullable<Controls["effects"][E]["linkMode"]>["name"];\n\n`
    + `export type ControlOwnerKind = "session" | "strip" | "route" | "vca";\n`
    + `export type ControlSectionKind = "own" | "builtins" | "console" | "inserts";\n`
    + `/** A per-lane control is addressed \`left\` or \`right\`; \`both\` writes both lanes at once. */\n`
    + `export type ControlLane = "left" | "right" | "both";\n`
    + `/** Where in an owner a control lives: its own rows, builtins, a console slot or an insert. */\n`
    + `export type ControlSection =\n`
    + `  | { readonly kind: "own" }\n`
    + `  | { readonly kind: "builtins" }\n`
    + `  | { readonly kind: "console"; readonly slot: string }\n`
    + `  | { readonly kind: "inserts"; readonly effect: string };\n\n`
    + `interface Addressed<O, S, C> {\n`
    + `  readonly owner: O;\n`
    + `  readonly section: S;\n`
    + `  readonly control: C;\n`
    + `  readonly lane: ControlLane;\n`
    + `}\n\n`
    + `/**\n`
    + ` * The stable address of one control: owner kind and ID, section kind and slot or effect ID,\n`
    + ` * the control's machine name and the lane. The \`control\` name must be a row of that owner and\n`
    + ` * section in the core's tables, so an unknown name does not compile.\n`
    + ` */\n`
    + `export type ControlAddress<E extends EffectId = EffectId> =\n`
    + `  | Addressed<{ readonly kind: "session" }, { readonly kind: "own" }, SessionControlName>\n`
    + `  | Addressed<{ readonly kind: "strip"; readonly id: string }, { readonly kind: "own" }, StripOwnControlName>\n`
    + `  | Addressed<{ readonly kind: "strip"; readonly id: string }, { readonly kind: "builtins" }, BuiltinControlName>\n`
    + `  | Addressed<\n`
    + `      { readonly kind: "strip"; readonly id: string },\n`
    + `      { readonly kind: "console"; readonly slot: string } | { readonly kind: "inserts"; readonly effect: string },\n`
    + `      EffectControlName<E>\n`
    + `    >\n`
    + `  | Addressed<{ readonly kind: "route"; readonly id: string }, { readonly kind: "own" }, RouteControlName>\n`
    + `  | Addressed<{ readonly kind: "vca"; readonly id: string }, { readonly kind: "own" }, VcaControlName>;\n`;
}

function renderProvenance(catalog, abi) {
  const value = {
    abiVersion: abi.abiVersion,
    schemas: {
      catalog: catalog.schema,
      abiLayout: abi.schema,
    },
    artifacts: ARTIFACT_SET,
    stagingSequence: abi.stagingSequence,
  };
  const json = JSON.stringify(value, null, 2);
  return `// @generated by sdk/codegen/generate.mjs; DO NOT EDIT.\n`
    + `//\n`
    + `// What the SDK was built against. Deliberately carries no content hashes: the browser\n`
    + `// module is a build output, so a checked-in digest would pin one machine's build or rot on\n`
    + `// the first rebuild. A caller supplies its release's digest and the SDK verifies it; what\n`
    + `// is source-derived -- the ABI version word and the artifact set -- is pinned here.\n\n`
    + deepFreezeSource()
    + `export const PROVENANCE = deepFreeze(\n${json} as const,\n);\n\n`
    + `export type Provenance = typeof PROVENANCE;\n`
    + `export type ProvenanceArtifactName = Provenance["artifacts"][number];\n`;
}

async function expectedModules() {
  const parsed = {};
  const modules = [];
  for (const [kind, sourceName, symbol] of inputs) {
    const path = resolve(assets, sourceName);
    let value;
    try {
      value = JSON.parse(await readFile(path, "utf8"));
    } catch (error) {
      throw new Error(`cannot read valid JSON input ${path}: ${error.message}`);
    }
    parsed[kind] = value;
    modules.push([resolve(generated, `${kind}.ts`), renderModule(kind, sourceName, symbol, value)]);
  }
  modules.push([resolve(generated, "controls.ts"), renderControls(parsed.catalog)]);
  modules.push([resolve(generated, "provenance.ts"), renderProvenance(parsed.catalog, parsed.abi)]);
  return modules;
}

async function run(check) {
  const modules = await expectedModules();
  let stale = false;
  for (const [path, expected] of modules) {
    if (!check) {
      await writeFile(path, expected, "utf8");
      console.log(`wrote ${path}`);
      continue;
    }
    let actual;
    try {
      actual = await readFile(path, "utf8");
    } catch (error) {
      console.error(`sdk codegen: missing generated module ${path}: ${error.message}`);
      stale = true;
      continue;
    }
    if (actual !== expected) {
      console.error(`sdk codegen: stale generated module ${path}; run \`npm run codegen\` in sdk/`);
      stale = true;
    }
  }
  if (stale) process.exitCode = 1;
  else if (check) console.log("sdk generated modules are current");
}

const args = process.argv.slice(2);
if (args.length > 1 || (args.length === 1 && args[0] !== "--check")) {
  throw new Error("usage: node sdk/codegen/generate.mjs [--check]");
}
await run(args[0] === "--check");
