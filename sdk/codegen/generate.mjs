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

const SESSION_KEYS_ASSET = "miso-engine-v1-session-keys.json";
const RESPONSE_FRAME_ASSET = "miso-engine-v1-response-frame.json";

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
  // Issue #2111: the stem identities' WebAssembly SIMD BLAKE3, which
  // `browser/stems/incremental-blake3.js` loads from `assets/`.
  "miso-engine-v1-blake3.simd128.wasm",
  // Issue #1332 S7: the control Worker of `worker` mode, resolved beside the worklet module.
  "miso-engine-v1-control-worker.js",
  "miso-engine-v1-parameter-metadata.json",
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
      + `/** The lattice declaration every parameter row carries (issue #242). */\n`
      + `export type StepDeclaration = EffectParameter<EffectId>["step"];\n`
      + `export type StepSizeName = keyof StepDeclaration["ladder"];\n`;
  }
  return `export type AbiLayout = typeof ${symbol};\n`
    + `export type AbiStructureName = keyof AbiLayout["structures"];\n`
    + `export type AbiStructure = AbiLayout["structures"][AbiStructureName];\n`
    + `export type AbiField = AbiStructure["fields"][number];\n`
    + `export type AbiConstantName = keyof AbiLayout["constants"];\n`
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
    + `  | NonNullable<Controls["effects"][EffectId]["linkMode"]>\n`
    + `  | NonNullable<Controls["effects"][EffectId]["consoleLinkMode"]>;\n`
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
    + renderAddress(catalog.controls.address);
}

/**
 * The structural facts of an address that the vocabulary does not carry: the extra field of an
 * owner or section of each kind, and the table each pair lists. A kind the core gains that is
 * missing here throws, so a new kind is a decision in this file and not a silent hole in the types.
 */
const OWNER_FIELDS = { session: null, strip: "id", route: "id", vca: "id" };
const SECTION_FIELDS = { own: null, builtins: null, console: "slot", inserts: "effect" };

/** The rows an owner and section kind list, as a type over `Controls`; absent if it has none. */
const ROWS_OF = {
  "session/own": `Controls["sessionOwn"][number]`,
  "strip/own": `Controls["stripOwn"][number]`,
  "strip/builtins": `Controls["builtins"][number]`,
  "route/own": `Controls["routeOwn"][number]`,
  "vca/own": `Controls["vcaOwn"][number]`,
  "strip/console": `EffectRow<E, "console">`,
  "strip/inserts": `EffectRow<E, "inserts">`,
};

function kindObject(kind, field) {
  return field === null
    ? `{ readonly kind: "${kind}" }`
    : `{ readonly kind: "${kind}"; readonly ${field}: string }`;
}

function renderAddress(address) {
  const owners = address.ownerKinds.map((entry) => entry.name);
  const sections = address.sectionKinds.map((entry) => entry.name);
  const lanes = address.lanes.map((entry) => entry.name);
  for (const [what, found, known] of [
    ["owner kind", owners, Object.keys(OWNER_FIELDS)],
    ["section kind", sections, Object.keys(SECTION_FIELDS)],
  ]) {
    const missing = found.filter((name) => !known.includes(name));
    const gone = known.filter((name) => !found.includes(name));
    if (missing.length > 0 || gone.length > 0) {
      throw new Error(
        `the address vocabulary and sdk/codegen/generate.mjs disagree on the ${what}s: `
        + `new ${JSON.stringify(missing)}, gone ${JSON.stringify(gone)}`,
      );
    }
  }
  for (const lane of ["left", "right", "both"]) {
    if (!lanes.includes(lane)) {
      throw new Error(`the address vocabulary has no lane \`${lane}\``);
    }
  }
  const pairs = [];
  for (const owner of owners) {
    for (const section of sections) {
      if (ROWS_OF[`${owner}/${section}`] !== undefined) pairs.push([owner, section]);
    }
  }
  return `\n`
    + `export type ControlOwnerKind = Controls["address"]["ownerKinds"][number]["name"];\n`
    + `export type ControlSectionKind = Controls["address"]["sectionKinds"][number]["name"];\n`
    + `/** A per-lane control is addressed \`left\` or \`right\`; \`both\` writes both lanes at once. */\n`
    + `export type ControlLane = Controls["address"]["lanes"][number]["name"];\n`
    + `/** The owner of a control: the session, or a strip, route or VCA by its stable ID. */\n`
    + `export type ControlOwner =\n`
    + owners.map((kind) => `  | ${kindObject(kind, OWNER_FIELDS[kind])}`).join("\n") + `;\n`
    + `/** Where in an owner a control lives: its own rows, builtins, a console slot or an insert. */\n`
    + `export type ControlSection =\n`
    + sections.map((kind) => `  | ${kindObject(kind, SECTION_FIELDS[kind])}`).join("\n") + `;\n\n`
    + `/** The rows an effect section lists: a console entry's link-mode row adds the \`slot\` choice. */\n`
    + `type EffectRow<E extends EffectId, K extends "console" | "inserts"> =\n`
    + `  | Controls["effects"][E]["bypass"]\n`
    + `  | Controls["effects"][E]["parameters"][number]\n`
    + `  | (K extends "inserts"\n`
    + `    ? NonNullable<Controls["effects"][E]["linkMode"]>\n`
    + `    : NonNullable<Controls["effects"][E]["consoleLinkMode"]>);\n\n`
    + `/**\n`
    + ` * One row as an address: its machine name, and the lanes its \`lanes\` allows. A shared row\n`
    + ` * (one value for its owner) is addressed \`both\` only; a per-lane row \`left\`, \`right\` or\n`
    + ` * \`both\`.\n`
    + ` */\n`
    + `type Addressed<O, S, Row> = Row extends { readonly name: infer N; readonly lanes: infer L }\n`
    + `  ? {\n`
    + `      readonly owner: O;\n`
    + `      readonly section: S;\n`
    + `      readonly control: N;\n`
    + `      readonly lane: L extends "shared" ? Extract<ControlLane, "both"> : ControlLane;\n`
    + `    }\n`
    + `  : never;\n\n`
    + `/**\n`
    + ` * The stable address of one control: owner kind and ID, section kind and slot or effect ID,\n`
    + ` * the control's machine name and the lane. The \`control\` name must be a row of that owner and\n`
    + ` * section in the core's tables and the \`lane\` one that row allows, so an unknown name, a\n`
    + ` * \`left\` lane on a shared row and a link mode on an effect with one mode do not compile.\n`
    + ` */\n`
    + `export type ControlAddress<E extends EffectId = EffectId> =\n`
    + pairs
      .map(([owner, section]) =>
        `  | Addressed<\n`
        + `      ${kindObject(owner, OWNER_FIELDS[owner])},\n`
        + `      ${kindObject(section, SECTION_FIELDS[section])},\n`
        + `      ${ROWS_OF[`${owner}/${section}`]}\n`
        + `    >`)
      .join("\n") + `;\n`;
}


/**
 * The typed builders (decision 17, slice S11b, D4): one method per row of the core's control
 * tables, generated into `control-builders.ts`. Every method builds the row's address and calls the
 * engine's `set` with it, so a builder stages exactly the edit `edit.set` stages for that address
 * and value; none of them carries a row number, a range or a name the tables do not.
 *
 * # One source of truth
 *
 * The addresses come from the core's control tables (`controls`): a row is addressed by its
 * machine name there (`threshold_db`). The catalog's parameter names (`threshold`) are the names the
 * public `parameter()` builder has always taken, so they stay the builder's keys; they are bound to
 * the control rows by the numeric ID the two documents share, and generation throws if a catalog
 * parameter has no control row of that ID, a different display name, or a different value domain.
 * A row the core gains therefore reaches the builders by regenerating, with no code written here.
 *
 * # What this file decides
 *
 * Only the public names #1563 left, which the tables cannot say: a method is the camel case of the
 * row's name unless {@link METHOD_NAMES} names it, a few rows are written together
 * ({@link COMPOSITES}: a pan's two lanes, a matrix's four coefficients, the input filters). Which
 * owner kinds offer a row is not decided here: the tables carry `onSubmix: false` on a strip row a
 * submix does not have (solo: a bus is solo-safe), from the same flag the core refuses a write
 * with, and the track-only builders are derived from it. Each name table is checked against the
 * tables; one that names a row the core no longer has throws.
 */

/** The method of a row whose name is not its camel case: the VCA's offset is its fader. */
const METHOD_NAMES = { "vca/own/offset_db": "faderDb" };

/**
 * Rows written together, by the owner and section kind that lists them. `lanePair` takes one value
 * per lane of a per-lane row; `group` takes an object of values for different rows. `replaces`
 * drops the members' own methods (a matrix is not set one coefficient at a time).
 */
const COMPOSITES = {
  "strip/builtins": [
    { method: "pan", kind: "lanePair", row: "pan", replaces: true },
    {
      method: "matrix",
      kind: "group",
      type: "MatrixValues",
      fields: { ll: "matrix_ll", lr: "matrix_lr", rl: "matrix_rl", rr: "matrix_rr" },
      replaces: true,
    },
    {
      method: "inputFilters",
      kind: "group",
      type: "InputFilterValues",
      fields: { hpfHz: "hpf_hz", lpfHz: "lpf_hz" },
      replaces: false,
    },
  ],
  "route/own": [
    {
      method: "matrix",
      kind: "group",
      type: "MatrixValues",
      fields: { ll: "matrix_ll", lr: "matrix_lr", rl: "matrix_rl", rr: "matrix_rr" },
      replaces: true,
    },
  ],
};

/** The tables a builder class draws on, by owner and section kind, in the order they are listed. */
const BUILDER_TABLES = [
  ["strip", "own", (controls) => controls.stripOwn],
  ["strip", "builtins", (controls) => controls.builtins],
  ["route", "own", (controls) => controls.routeOwn],
  ["vca", "own", (controls) => controls.vcaOwn],
  ["session", "own", (controls) => controls.sessionOwn],
];

function camelCase(name) {
  return name.replace(/_([a-z0-9])/g, (_, character) => character.toUpperCase());
}

/**
 * A row's class: a route row carries one per target (a send into a submix, a route into the output
 * from before the fader, or from after it).
 */
function rowClasses(row) {
  if (row.class !== undefined) return [row.class];
  if (
    row.intoSubmix !== undefined &&
    row.intoOutputBeforeFader !== undefined &&
    row.intoOutput !== undefined
  ) {
    return [row.intoSubmix.class, row.intoOutputBeforeFader.class, row.intoOutput.class];
  }
  throw new Error(`control row ${row.name} has neither a class nor a class per route target`);
}

function isDerived(row) {
  return rowClasses(row).every((rowClass) => rowClass === "derived");
}

/** How a row's builder takes its value. */
function valueKindOf(row) {
  switch (row.kind) {
    case "toggle":
      return "boolean";
    case "enumeration":
      return "enumeration";
    case "continuous":
      return "number";
    default:
      throw new Error(`control row ${row.name} has a value kind (${row.kind}) the builders do not know`);
  }
}

function choicesOf(row) {
  return row.choices.map((choice) => [choice.label, choice.value]);
}

function choicesLiteral(choices) {
  return `[${choices.map(([label, value]) => `[${JSON.stringify(label)}, ${value}]`).join(", ")}]`;
}

/**
 * The options a row's builder takes, from the row's lanes and ramp: a per-lane row selects a lane,
 * a row with a ramp of its own takes `smoothingSamples`, and a shared row with a fixed ramp takes
 * none.
 */
function optionsOf(row) {
  const lanes = row.lanes === "perLane";
  const ramp = row.ramp !== "fixed";
  if (lanes) return ramp ? "lane" : "channel";
  return ramp ? "smoothing" : "none";
}

const OPTION_TYPES = {
  lane: "LaneOptions",
  channel: "ChannelOptions",
  smoothing: "SmoothingOptions",
  none: null,
};

function docOf(row, extra) {
  const unit = row.displayUnit === "" ? "" : ` (${row.displayUnit})`;
  const notes = [];
  if (rowClasses(row).includes("rebuildOnly")) {
    notes.push("A rebuild-only row: the core says whether a live write is accepted.");
  }
  if (extra !== undefined) notes.push(extra);
  return `/** \`${row.name}\`: ${row.displayName}${unit}.${notes.length > 0 ? ` ${notes.join(" ")}` : ""} */`;
}

/**
 * The builders of one owner and section kind, as entries: `{ method, section, ...shape }`. A
 * composite replaces its members' own methods when it says so.
 */
function entriesOf(ownerKind, sectionKind, rows, controlsDocument) {
  const key = `${ownerKind}/${sectionKind}`;
  const byName = new Map(rows.map((row) => [row.name, row]));
  const composites = COMPOSITES[key] ?? [];
  const consumed = new Set();
  const entries = [];
  for (const composite of composites) {
    const memberNames = composite.kind === "lanePair" ? [composite.row] : Object.values(composite.fields);
    for (const name of memberNames) {
      if (!byName.has(name)) {
        throw new Error(`COMPOSITES ${key} ${composite.method}: the core has no row ${name}`);
      }
    }
    const members = memberNames.map((name) => byName.get(name));
    if (members.some(isDerived)) {
      throw new Error(`COMPOSITES ${key} ${composite.method}: a derived row cannot be written`);
    }
    if (members.some((member) => valueKindOf(member) !== "number")) {
      throw new Error(`COMPOSITES ${key} ${composite.method}: every member must be a number row`);
    }
    const shapes = new Set(members.map((member) => `${member.lanes}/${optionsOf(member)}`));
    if (shapes.size !== 1) {
      throw new Error(`COMPOSITES ${key} ${composite.method}: the members disagree on lanes or ramp`);
    }
    if (composite.replaces) for (const name of memberNames) consumed.add(name);
    entries.push({
      method: composite.method,
      section: sectionKind,
      shape: composite.kind,
      composite,
      members,
      options: composite.kind === "lanePair" ? (optionsOf(members[0]) === "none" ? "none" : "smoothing")
        : optionsOf(members[0]),
      valueKind: "number",
    });
  }
  for (const row of rows) {
    if (isDerived(row) || consumed.has(row.name)) continue;
    const method = METHOD_NAMES[`${key}/${row.name}`] ?? camelCase(row.name);
    entries.push({
      method,
      section: sectionKind,
      shape: "row",
      row,
      options: optionsOf(row),
      valueKind: valueKindOf(row),
    });
  }
  for (const named of Object.keys(METHOD_NAMES)) {
    if (named.startsWith(`${key}/`) && !byName.has(named.slice(key.length + 1))) {
      throw new Error(`METHOD_NAMES ${named}: the core has no such row`);
    }
  }
  void controlsDocument;
  return entries;
}

/** The expression a builder passes as the edit's value, from the argument name it declares. */
function valueExpression(entry, name) {
  switch (entry.valueKind) {
    case "boolean":
      return `flag(${name})`;
    case "enumeration":
      return `enumValue(${choicesLiteral(choicesOf(entry.row))}, ${name}, ${JSON.stringify(entry.method)})`;
    default:
      return name;
  }
}

function valueType(entry) {
  switch (entry.valueKind) {
    case "boolean":
      return "boolean";
    case "enumeration":
      return entry.row.choices.map((choice) => JSON.stringify(choice.label)).join(" | ");
    default:
      return "number";
  }
}

/** One method of a builder class; `call` and `owner` are how the class reaches `set` and its owner. */
function renderEntry(entry, ownerKind, call, owner) {
  const section = `{ kind: "${entry.section}" }`;
  const address = (control, lane) =>
    `{ owner: ${owner}, section: ${section}, control: "${control}", lane${lane === "lane" ? "" : `: ${lane}`} }`;
  const optionType = OPTION_TYPES[entry.options];
  const optionParameter = optionType === null ? "" : `, options: ${optionType} = {}`;
  const laneOption = entry.options === "lane" || entry.options === "channel";
  const rampOption = entry.options === "lane" || entry.options === "smoothing";
  if (entry.shape === "row") {
    const lane = laneOption ? "laneOf(options.channel)" : `"both"`;
    const name = entry.valueKind === "boolean" ? "enabled" : "value";
    const doc = docOf(entry.row);
    const rampArgument = rampOption ? ", rampOptions(options)" : "";
    return `  ${doc}\n`
      + `  ${entry.method}(${name}: ${valueType(entry)}${optionParameter}): ControlEditSet {\n`
      + `    return ${call}(\n`
      + `      ${address(entry.row.name, lane)},\n`
      + `      ${valueExpression(entry, name)}${rampArgument},\n`
      + `    );\n`
      + `  }\n`;
  }
  const rampLine = rampOption ? `    const ramp = rampOptions(options);\n` : "";
  const rampArgument = rampOption ? ", ramp" : "";
  if (entry.shape === "lanePair") {
    const row = entry.members[0];
    return `  /** \`${row.name}\` of each lane: \`left\` on the left lane and \`right\` on the right. */\n`
      + `  ${entry.method}(left: number, right: number${optionParameter}): ControlEditSet {\n`
      + rampLine
      + `    return mergeEdits(\n`
      + `      ${call}(\n        ${address(row.name, `"left"`)},\n        left${rampArgument},\n      ),\n`
      + `      ${call}(\n        ${address(row.name, `"right"`)},\n        right${rampArgument},\n      ),\n`
      + `    );\n`
      + `  }\n`;
  }
  const fieldNames = Object.keys(entry.composite.fields);
  const lane = laneOption ? "lane" : `"both"`;
  const laneLine = laneOption ? `    const lane = laneOf(options.channel);\n` : "";
  const parameter = entry.composite.type === "MatrixValues" ? "matrix" : "filters";
  const cells = fieldNames.map((field) =>
    `      ${call}(\n        ${address(entry.composite.fields[field], lane)},\n        ${parameter}.${field}${rampArgument},\n      ),\n`);
  return `  /** ${fieldNames.map((field) => `\`${entry.composite.fields[field]}\``).join(", ")} in one transaction. */\n`
    + `  ${entry.method}(${parameter}: ${entry.composite.type}${optionParameter}): ControlEditSet {\n`
    + `    requireGroup(${parameter}, ${JSON.stringify(entry.method)}, ${JSON.stringify(fieldNames)});\n`
    + laneLine
    + rampLine
    + `    return mergeEdits(\n`
    + cells.join("")
    + `    );\n`
    + `  }\n`;
}

const GROUP_TYPES = {
  MatrixValues: ["ll", "lr", "rl", "rr"],
  InputFilterValues: ["hpfHz", "lpfHz"],
};

function renderOwnedClass(name, ownerKind, doc, entries) {
  const ownerType = `Extract<ControlOwner, { readonly kind: "${ownerKind}" }>`;
  const body = entries.map((entry) => renderEntry(entry, ownerKind, "this.#set", "this.#owner")).join("\n");
  const constructorParameter = ownerKind === "session" ? "" : `${ownerKind}Id: string, `;
  const ownerValue = ownerKind === "session" ? `{ kind: "session" }` : `{ kind: "${ownerKind}", id: ${ownerKind}Id }`;
  return `${doc}\n`
    + `export class ${name} {\n`
    + `  readonly #owner: ${ownerType};\n`
    + `  readonly #set: ControlSetter;\n\n`
    + `  constructor(${constructorParameter}set: ControlSetter) {\n`
    + `    this.#owner = ${ownerValue};\n`
    + `    this.#set = set;\n`
    + `  }\n\n`
    + body
    + `}\n`;
}

/** What the catalog and the control tables must agree on for the effect builders to join them. */
function effectTables(catalog) {
  const parameters = {};
  const linkModes = {};
  const consoleLinkModes = {};
  const controlEffects = catalog.controls.effects;
  for (const effect of catalog.effects) {
    const table = controlEffects[effect.id];
    if (table === undefined) throw new Error(`the control tables have no effect ${effect.id}`);
    if (table.bypass.name !== "bypass" || table.bypass.control !== 0) {
      throw new Error(`${effect.id}: the bypass row is not control 0 named bypass`);
    }
    const byControl = new Map(table.parameters.map((row) => [row.control, row]));
    const rows = {};
    for (const parameter of effect.parameters) {
      const row = byControl.get(parameter.id);
      if (row === undefined) {
        throw new Error(`${effect.id}: the catalog parameter ${parameter.name} (${parameter.id}) has no control row`);
      }
      if (row.displayName !== parameter.name) {
        throw new Error(
          `${effect.id}: control ${row.name} is displayed ${row.displayName}, but the catalog calls ID ${parameter.id} ${parameter.name}`,
        );
      }
      const expected = { boolean: "toggle", enumeration: "enumeration", continuous: "continuous" }[parameter.domainName];
      if (row.kind !== expected) {
        throw new Error(`${effect.id}.${parameter.name}: the catalog says ${parameter.domainName}, the control row ${row.kind}`);
      }
      const catalogChoices = JSON.stringify(parameter.enumChoices.map((choice) => [choice.label, choice.value]));
      if (row.kind === "enumeration" && JSON.stringify(choicesOf(row)) !== catalogChoices) {
        throw new Error(`${effect.id}.${parameter.name}: the catalog and the control row list different choices`);
      }
      rows[parameter.name] = {
        control: row.name,
        kind: parameter.domainName,
        choices: row.kind === "enumeration" ? choicesOf(row) : [],
      };
    }
    parameters[effect.id] = rows;
    if (table.linkMode !== null && table.linkMode !== undefined) {
      if (table.linkMode.kind !== "enumeration") {
        throw new Error(`${effect.id}: the link mode row is not an enumeration`);
      }
      linkModes[effect.id] = { control: table.linkMode.name, choices: choicesOf(table.linkMode) };
    }
    if (table.consoleLinkMode !== null && table.consoleLinkMode !== undefined) {
      if (table.consoleLinkMode.kind !== "enumeration") {
        throw new Error(`${effect.id}: the console link mode row is not an enumeration`);
      }
      consoleLinkModes[effect.id] = {
        control: table.consoleLinkMode.name,
        choices: choicesOf(table.consoleLinkMode),
      };
    }
  }
  return { parameters, linkModes, consoleLinkModes };
}

function renderControlBuilders(catalog) {
  const controls = catalog.controls;
  const entriesByOwner = new Map();
  for (const [ownerKind, sectionKind, select] of BUILDER_TABLES) {
    const list = entriesByOwner.get(ownerKind) ?? [];
    list.push(...entriesOf(ownerKind, sectionKind, select(controls), controls));
    entriesByOwner.set(ownerKind, list);
  }
  for (const [ownerKind, list] of entriesByOwner) {
    const seen = new Set();
    for (const entry of list) {
      if (seen.has(entry.method)) throw new Error(`${ownerKind}: two builders are named ${entry.method}`);
      seen.add(entry.method);
    }
  }
  const stripEntries = entriesByOwner.get("strip");
  const entryRows = (entry) => (entry.shape === "row" ? [entry.row] : entry.members);
  const trackOnly = stripEntries.filter((entry) => {
    const absent = entryRows(entry).map((row) => row.onSubmix === false);
    if (absent.some(Boolean) && !absent.every(Boolean)) {
      throw new Error(`strip ${entry.method}: its rows disagree on whether a submix has them`);
    }
    return absent.every(Boolean);
  });
  const sharedEntries = stripEntries.filter((entry) => !trackOnly.includes(entry));
  const { parameters, linkModes, consoleLinkModes } = effectTables(catalog);

  const manifest = [];
  const addManifest = (entry, owner, only) => {
    manifest.push({
      owner,
      ...(only === undefined ? {} : { only }),
      section: entry.section,
      method: entry.method,
      shape: entry.shape,
      valueKind: entry.valueKind,
      options: entry.options,
      rows: (entry.shape === "row" ? [entry.row] : entry.members).map((row) => row.name),
      ...(entry.shape === "group" ? { fields: Object.keys(entry.composite.fields) } : {}),
      ...(entry.valueKind === "enumeration" ? { choices: choicesOf(entry.row) } : {}),
    });
  };
  for (const entry of sharedEntries) addManifest(entry, "strip");
  for (const entry of trackOnly) addManifest(entry, "strip", "track");
  for (const owner of ["route", "vca", "session"]) {
    for (const entry of entriesByOwner.get(owner)) addManifest(entry, owner);
  }

  const groupTypes = new Set();
  for (const list of entriesByOwner.values()) {
    for (const entry of list) if (entry.shape === "group") groupTypes.add(entry.composite.type);
  }
  for (const type of groupTypes) {
    if (GROUP_TYPES[type] === undefined) throw new Error(`no declaration for the group type ${type}`);
  }

  const strip = (entries) => entries.map((entry) => renderEntry(entry, "strip", "this.setControl", "this.controlOwner()")).join("\n");
  const labelUnion = (rows) => Object.entries(rows)
    .map(([effect, row]) => `    E extends ${JSON.stringify(effect)} ? ${row.choices.map(([label]) => JSON.stringify(label)).join(" | ")} :`)
    .join("\n");
  const labelType = (rows) => (Object.keys(rows).length === 0
    ? `  never;\n\n`
    : `${labelUnion(rows)}\n    never;\n\n`);

  return `// @generated by sdk/codegen/generate.mjs from sdk/assets/miso-engine-v1-parameter-metadata.json; DO NOT EDIT.\n`
    + `// One builder per row of the core's control tables (its \`controls\` object): each builds the row's\n`
    + `// address and calls \`set\`, so it stages the edit \`edit.set\` stages for that address and value.\n`
    + `// Run \`npm run assets\` then \`npm run codegen\` in sdk/ to refresh.\n\n`
    + `import type { ControlLane, ControlOwner } from "./controls.ts";\n`
    + `import type { EffectId } from "./catalog.ts";\n`
    + `import { MisoUsageError } from "../core/errors.ts";\n`
    + `import {\n`
    + `  StripAddressing,\n`
    + `  enumValue,\n`
    + `  flag,\n`
    + `  laneOf,\n`
    + `  mergeEdits,\n`
    + `  normalizeParameterEdit,\n`
    + `  parameterScalar,\n`
    + `  rampOptions,\n`
    + `  requireGroup,\n`
    + `} from "../core/control-builder-support.ts";\n`
    + `import type {\n`
    + `  ChannelOptions,\n`
    + `  ControlEditSet,\n`
    + `  ControlSetter,\n`
    + `  EffectParameterRow,\n`
    + `  LaneOptions,\n`
    + `  LiveControlLayout,\n`
    + `  LiveEffectParameterEdit,\n`
    + `  LiveEffectParameterName,\n`
    + `  LiveEffectParameterOptions,\n`
    + `  LiveEffectParameterValue,\n`
    + `  RuntimeParameterOptions,\n`
    + `  SetControlOptions,\n`
    + `  SmoothingOptions,\n`
    + `  StripContext,\n`
    + `} from "../core/control-builder-support.ts";\n\n`
    + [...groupTypes].map((type) =>
      `export interface ${type} {\n${GROUP_TYPES[type].map((field) => `  readonly ${field}: number;\n`).join("")}}\n`).join("\n")
    + `\n`
    + `/**\n`
    + ` * Every live edit a strip -- a track or a submix -- shares. Methods build data and never mutate\n`
    + ` * the engine: each is \`edit.set\` over the address of one row of the strip's own or builtin table.\n`
    + ` *\n`
    + ` * \`pan\`, \`matrix\`, \`faderDb\`, \`mute\`, \`trimDb\` and \`polarityInvert\` take an optional\n`
    + ` * \`smoothingSamples\`, sent as the edit's \`ramp_samples\`; absent, the core ramps over the session's\n`
    + ` * own length for the row. The input filters (\`hpfHz\`, \`lpfHz\`, \`inputFilters\`) have a fixed ramp\n`
    + ` * and take none.\n`
    + ` *\n`
    + ` * \`pan\` writes the strip's two pan lanes and \`matrix\` its four coefficients. A strip in the other\n`
    + ` * shape is refused by the core with \`control.inactive\`.\n`
    + ` */\n`
    + `export abstract class StripEdits extends StripAddressing {\n`
    + strip(sharedEntries)
    + `}\n\n`
    + `/** A track strip's live edits: every strip edit, plus \`solo\`. */\n`
    + `export class TrackEdits extends StripEdits {\n`
    + `  constructor(\n`
    + `    trackId: string,\n`
    + `    layout: LiveControlLayout | undefined,\n`
    + `    context: StripContext,\n`
    + `  ) {\n`
    + `    super("track", trackId, layout, context);\n`
    + `  }\n\n`
    + strip(trackOnly)
    + `}\n\n`
    + `/**\n`
    + ` * A submix strip's live edits: every strip edit and no \`solo\` (a bus is solo-safe; the core refuses\n`
    + ` * the solo of a submix).\n`
    + ` */\n`
    + `export class SubmixEdits extends StripEdits {\n`
    + `  constructor(\n`
    + `    submixId: string,\n`
    + `    layout: LiveControlLayout | undefined,\n`
    + `    context: StripContext,\n`
    + `  ) {\n`
    + `    super("submix", submixId, layout, context);\n`
    + `  }\n`
    + `}\n\n`
    + renderOwnedClass("RouteEdits", "route",
      `/**\n * A live send's edits: its gain, its mute, its follow-mute flag and its 2x2 matrix, each over an\n`
      + ` * optional \`smoothingSamples\` (omitted, the core's ramp for the row). A route is addressed by its\n`
      + ` * stable ID. Only routes into submixes are live: the core refuses an output route.\n */`,
      entriesByOwner.get("route")) + `\n`
    + renderOwnedClass("VcaEdits", "vca",
      `/**\n * A VCA's live edits: its per-lane fader offset and mute, built as a strip's \`faderDb\` and \`mute\`\n`
      + ` * are, over an optional lane and \`smoothingSamples\`. A VCA has no audio path: the engine adds the\n`
      + ` * offset to every member's own fader, and its mute mutes every member.\n */`,
      entriesByOwner.get("vca")) + `\n`
    + renderOwnedClass("SessionEdits", "session",
      `/**\n * The session's own live settings: the lengths of the default fader, mute and pan ramps.\n */`,
      entriesByOwner.get("session")) + `\n`
    + `/** One effect parameter's row in the control tables, by the catalog's name for it. */\n`
    + `export const EFFECT_PARAMETER_ROWS: Readonly<Record<string, Readonly<Record<string, EffectParameterRow>>>> =\n`
    + `  ${JSON.stringify(parameters, null, 2).replace(/\n/g, "\n  ")};\n\n`
    + `/** The link mode row of an insert of each effect that has a choice of it. */\n`
    + `export const EFFECT_LINK_MODE_ROWS: Readonly<Record<string, {\n`
    + `  readonly control: string;\n`
    + `  readonly choices: readonly (readonly [string, number])[];\n`
    + `}>> =\n`
    + `  ${JSON.stringify(linkModes, null, 2).replace(/\n/g, "\n  ")};\n\n`
    + `/**\n`
    + ` * The link mode row of a console entry of each effect that has a choice of it: \`slot\` (the\n`
    + ` * slot's declared mode) and then the insert row's choices.\n`
    + ` */\n`
    + `export const EFFECT_CONSOLE_LINK_MODE_ROWS: Readonly<Record<string, {\n`
    + `  readonly control: string;\n`
    + `  readonly choices: readonly (readonly [string, number])[];\n`
    + `}>> =\n`
    + `  ${JSON.stringify(consoleLinkModes, null, 2).replace(/\n/g, "\n  ")};\n\n`
    + `/** The link mode labels of an insert of an effect; \`never\` for an effect with no choice of it. */\n`
    + `export type EffectLinkModeLabel<E extends EffectId> =\n`
    + labelType(linkModes)
    + `/** The link mode labels of a console entry of an effect; \`never\` for an effect with no choice. */\n`
    + `export type EffectConsoleLinkModeLabel<E extends EffectId> =\n`
    + labelType(consoleLinkModes)
    + `/**\n`
    + ` * The edits of one effect instance: each parameter, its bypass and its link mode, as \`edit.set\`\n`
    + ` * over the row of the instance's own control table. The parameter keys are the catalog's names.\n`
    + ` *\n`
    + ` * \`R\` is the instance's rack when the caller knows it (\`console()\`, \`insert()\`), so an insert's\n`
    + ` * \`linkMode\` refuses \`slot\` at compile time; the union, for an instance addressed by a rack\n`
    + ` * the caller passes (\`effect()\`), takes either rack's labels and leaves the refusal to run time.\n`
    + ` */\n`
    + `export abstract class EffectControlBuilders<\n`
    + `  E extends EffectId,\n`
    + `  R extends "console" | "inserts" = "console" | "inserts",\n`
    + `> {\n`
    + `  protected abstract readonly effectId: E;\n`
    + `  /** Whether this instance is a console entry or an insert: their link-mode rows differ. */\n`
    + `  protected abstract readonly rack: R;\n\n`
    + `  /** \`edit.set\` over this instance's address: the row's machine name, the lane and the ramp. */\n`
    + `  protected abstract setRow(\n`
    + `    control: string,\n`
    + `    value: number,\n`
    + `    lane: ControlLane,\n`
    + `    options?: SetControlOptions,\n`
    + `  ): ControlEditSet;\n\n`
    + `  parameter<N extends LiveEffectParameterName<E>>(\n`
    + `    name: N,\n`
    + `    value: LiveEffectParameterValue<E, N>,\n`
    + `    options?: LiveEffectParameterOptions<E, N>,\n`
    + `  ): ControlEditSet;\n`
    + `  parameter(edit: LiveEffectParameterEdit<E>): ControlEditSet;\n`
    + `  parameter(\n`
    + `    keyOrEdit: string | LiveEffectParameterEdit<E>,\n`
    + `    value?: unknown,\n`
    + `    options: RuntimeParameterOptions = {},\n`
    + `  ): ControlEditSet {\n`
    + `    const normalized = normalizeParameterEdit(keyOrEdit, value, options);\n`
    + `    const row = EFFECT_PARAMETER_ROWS[this.effectId]?.[normalized.key];\n`
    + `    if (row === undefined) {\n`
    + `      throw new MisoUsageError(\`\${this.effectId} has no parameter '\${normalized.key}'\`);\n`
    + `    }\n`
    + `    // The value is made a number the frame can hold; whether it is in the row's domain is the\n`
    + `    // core's to say.\n`
    + `    const scalar = parameterScalar(this.effectId, normalized.key, row, normalized.value);\n`
    + `    return this.setRow(\n`
    + `      row.control,\n`
    + `      scalar,\n`
    + `      laneOf(normalized.options.channel),\n`
    + `      rampOptions(normalized.options),\n`
    + `    );\n`
    + `  }\n\n`
    + `  /**\n`
    + `   * Set or lift this instance's bypass (control \`0\` of the instance). The core crossfades it on\n`
    + `   * both hosts and refuses what it cannot do.\n`
    + `   */\n`
    + `  bypass(enabled: boolean): ControlEditSet {\n`
    + `    return this.setRow("bypass", flag(enabled), "both");\n`
    + `  }\n\n`
    + `  /**\n`
    + `   * The link mode of an instance of an effect that has a choice of it. A console entry also\n`
    + `   * takes \`slot\`, the slot's declared mode; an insert refuses it, at compile time when its rack\n`
    + `   * is known and at run time otherwise.\n`
    + `   */\n`
    + `  linkMode(\n`
    + `    mode: R extends "console" ? EffectConsoleLinkModeLabel<E> : EffectLinkModeLabel<E>,\n`
    + `  ): ControlEditSet {\n`
    + `    const row = (this.rack === "console" ? EFFECT_CONSOLE_LINK_MODE_ROWS : EFFECT_LINK_MODE_ROWS)[this.effectId];\n`
    + `    if (row === undefined) throw new MisoUsageError(\`\${this.effectId} has no link mode\`);\n`
    + `    return this.setRow(row.control, enumValue(row.choices, mode, \`\${this.effectId} link mode\`), "both");\n`
    + `  }\n`
    + `}\n\n`
    + `/** One builder of the manifest: where it lives and which rows of the tables it writes. */\n`
    + `export interface ControlBuilderEntry {\n`
    + `  readonly owner: "strip" | "route" | "vca" | "session";\n`
    + `  /** Present for a builder a track has and a submix does not. */\n`
    + `  readonly only?: "track";\n`
    + `  readonly section: "own" | "builtins";\n`
    + `  readonly method: string;\n`
    + `  readonly shape: "row" | "lanePair" | "group";\n`
    + `  readonly valueKind: "number" | "boolean" | "enumeration";\n`
    + `  readonly options: "lane" | "channel" | "smoothing" | "none";\n`
    + `  /** The machine names of the rows it writes, in the order it writes them. */\n`
    + `  readonly rows: readonly string[];\n`
    + `  /** A group builder's argument fields, one per row. */\n`
    + `  readonly fields?: readonly string[];\n`
    + `  readonly choices?: readonly (readonly [string, number])[];\n`
    + `}\n\n`
    + `/** Every builder above, by the rows it writes, so a gate can hold each to \`set\`. */\n`
    + `export const CONTROL_BUILDERS: readonly ControlBuilderEntry[] =\n`
    + `  ${JSON.stringify(manifest, null, 2).replace(/\n/g, "\n  ")};\n`;
}

function renderSessionKeys(value) {
  const json = JSON.stringify(value, null, 2);
  return `// @generated by sdk/codegen/generate.mjs from sdk/assets/${SESSION_KEYS_ASSET}; DO NOT EDIT.\n`
    + `// That JSON is the Rust session field walk's own key tables, made by the engine's\n`
    + `// \`parameter-metadata --print-session-keys\`; run \`npm run assets\` then \`npm run codegen\`\n`
    + `// in sdk/ to refresh both. The canonical session writer reads it: record shapes, key order\n`
    + `// and number spelling are never typed by hand.\n\n`
    + deepFreezeSource()
    + `export const SESSION_KEYS = deepFreeze(\n${json} as const,\n);\n\n`
    + `export type SessionKeys = typeof SESSION_KEYS;\n`
    + `export type SessionKeyKind = SessionKeys["records"][number]["keys"][number][1];\n`;
}

function renderResponseFrame(value) {
  const json = JSON.stringify(value, null, 2);
  return `// @generated by sdk/codegen/generate.mjs from sdk/assets/${RESPONSE_FRAME_ASSET}; DO NOT EDIT.\n`
    + `// That JSON is the protocol crate's \`RESPONSE_GET\` frame table, read from the codec's own\n`
    + `// schema specs and strict code readers by \`parameter-metadata --print-response-frame\`; run\n`
    + `// \`npm run assets\` then \`npm run codegen\` in sdk/ to refresh both. The response codec reads\n`
    + `// it: no field ID, wire type or enum code of the frame is typed by hand.\n\n`
    + deepFreezeSource()
    + `export const RESPONSE_FRAME = deepFreeze(\n${json} as const,\n);\n\n`
    + `export type ResponseFrame = typeof RESPONSE_FRAME;\n`;
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
  const sessionKeysPath = resolve(assets, SESSION_KEYS_ASSET);
  let sessionKeys;
  try {
    sessionKeys = JSON.parse(await readFile(sessionKeysPath, "utf8"));
  } catch (error) {
    throw new Error(`cannot read valid JSON input ${sessionKeysPath}: ${error.message}`);
  }
  modules.push([resolve(generated, "session-keys.ts"), renderSessionKeys(sessionKeys)]);
  const responseFramePath = resolve(assets, RESPONSE_FRAME_ASSET);
  let responseFrame;
  try {
    responseFrame = JSON.parse(await readFile(responseFramePath, "utf8"));
  } catch (error) {
    throw new Error(`cannot read valid JSON input ${responseFramePath}: ${error.message}`);
  }
  modules.push([resolve(generated, "response-frame.ts"), renderResponseFrame(responseFrame)]);
  modules.push([resolve(generated, "controls.ts"), renderControls(parsed.catalog)]);
  modules.push([resolve(generated, "control-builders.ts"), renderControlBuilders(parsed.catalog)]);
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
