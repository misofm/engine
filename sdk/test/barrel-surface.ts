/**
 * Issue #278 gap 2: every deep-imported symbol is barrel-reachable, and every deep import still
 * resolves to the same declaration.
 *
 * # Why this is a type file rather than an eval
 *
 * The claim has two halves and only one of them is observable at runtime. "The barrel exports
 * `ConsoleWriter`" is a value check any eval could make; "the barrel's `LatticePoint` is the same
 * type as `core/lattice.ts`'s" is not, because the interface is erased before a test could look at
 * it. A barrel that re-exported a *structurally similar* declaration -- the shape drifting by one
 * optional field -- would pass a runtime check and still break the consumer that trusted it. So
 * this file asserts type identity with `Exact`, which compares declarations rather than
 * assignability, and it is checked by `scripts/check-sdk-types.sh` exactly as
 * `sdk/test/host-mirror.ts` is: it runs nothing, and its job is to fail COMPILATION.
 *
 * # The `StepDeclaration` / `StepSizeName` overlap
 *
 * `generated/catalog.ts` and `core/lattice.ts` both spell those two names, and they are not the
 * same type -- the generated one is the narrow union of the shipped rows' literal step objects,
 * the lattice's is the wider structural shape its generator accepts. `src/index.ts` therefore
 * keeps the generated names on the barrel and re-exports the lattice's as `LatticeStepDeclaration`
 * / `LatticeStepSizeName`. Both arms are pinned below: the generated spelling must still be the
 * one `StepDeclaration` reaches, and the renamed pair must still be the lattice's own. Collapsing
 * them later would silently widen `StepDeclaration` for every existing consumer, and that is the
 * regression this half exists to catch.
 */

import * as barrel from "../src/index.ts";
import * as browserBarrel from "../src/browser/index.ts";
import * as headlessBarrel from "../src/headless/index.ts";

import * as agent from "../src/core/agent.ts";
import * as catalog from "../src/generated/catalog.ts";
import * as hostMirror from "../src/browser/host-mirror.ts";
import * as lattice from "../src/core/lattice.ts";
import * as writer from "../src/core/writer.ts";

/** Declaration identity, not assignability: `Exact<A, B>` is false when either side is wider. */
type Exact<A, B> =
  (<T>() => T extends A ? 1 : 2) extends (<T>() => T extends B ? 1 : 2) ? true : false;
type Assert<T extends true> = T;

// --- core/agent.ts: the whole module is on the root barrel -------------------------------------

type AgentCatalogFn = Assert<Exact<typeof barrel.catalog, typeof agent.catalog>>;
type AgentParameterFn = Assert<Exact<typeof barrel.parameter, typeof agent.parameter>>;
type AgentDecimalFn = Assert<Exact<typeof barrel.decimalToFloat32, typeof agent.decimalToFloat32>>;
type AgentHandle = Assert<Exact<barrel.ParameterHandle, agent.ParameterHandle>>;
type AgentCatalogParameter = Assert<Exact<barrel.CatalogParameter, agent.CatalogParameter>>;
type AgentSetAck = Assert<Exact<barrel.SetAck, agent.SetAck>>;

// --- core/writer.ts: same ----------------------------------------------------------------------

type WriterClass = Assert<Exact<typeof barrel.ConsoleWriter, typeof writer.ConsoleWriter>>;
type WriterInstance = Assert<Exact<barrel.ConsoleWriter, writer.ConsoleWriter>>;
type WriterLaneEdit = Assert<Exact<barrel.LaneEdit, writer.LaneEdit>>;
type WriterFlushOutcome = Assert<Exact<barrel.FlushOutcome, writer.FlushOutcome>>;
type WriterStatsType = Assert<Exact<barrel.WriterStats, writer.WriterStats>>;
type WriterOptionsType = Assert<Exact<barrel.WriterOptions, writer.WriterOptions>>;

// --- core/lattice.ts: everything but the two collided names, which are renamed ------------------

type LatticePointsFn = Assert<Exact<typeof barrel.latticePoints, typeof lattice.latticePoints>>;
type LatticeResolveStepFn = Assert<Exact<typeof barrel.resolveStep, typeof lattice.resolveStep>>;
type LatticeIndexForDecimalFn =
  Assert<Exact<typeof barrel.indexForDecimal, typeof lattice.indexForDecimal>>;
type LatticeStepSizesConst = Assert<Exact<typeof barrel.STEP_SIZES, typeof lattice.STEP_SIZES>>;
type LatticeMaximumConst =
  Assert<Exact<typeof barrel.MAXIMUM_LATTICE_POINTS, typeof lattice.MAXIMUM_LATTICE_POINTS>>;
type LatticePointType = Assert<Exact<barrel.LatticePoint, lattice.LatticePoint>>;
type LatticeDeclarationType = Assert<Exact<barrel.LatticeDeclaration, lattice.LatticeDeclaration>>;
type LatticeNearestType = Assert<Exact<barrel.NearestLatticeValues, lattice.NearestLatticeValues>>;

// The renamed pair is the lattice's, and the unrenamed pair is still the generated one.
type LatticeStepDecl = Assert<Exact<barrel.LatticeStepDeclaration, lattice.StepDeclaration>>;
type LatticeStepSize = Assert<Exact<barrel.LatticeStepSizeName, lattice.StepSizeName>>;
type CatalogStepDecl = Assert<Exact<barrel.StepDeclaration, catalog.StepDeclaration>>;
type CatalogStepSize = Assert<Exact<barrel.StepSizeName, catalog.StepSizeName>>;

// And they are genuinely two types, so the rename is load-bearing rather than cosmetic. If a later
// change makes the generated declaration structural, this line goes red and the rename can go.
type StepDeclarationsDiffer = Assert<
  Exact<barrel.StepDeclaration, lattice.StepDeclaration> extends true ? false : true
>;

// --- browser/host-mirror.ts: on the browser barrel ---------------------------------------------

type HostMirrorFn =
  Assert<Exact<typeof browserBarrel.toWebBootOptions, typeof hostMirror.toWebBootOptions>>;

// --- the barrels the deep imports already had ---------------------------------------------------
//
// The three entry points `sdk/package.json` names must each keep their pre-#278 surface. Naming a
// representative export of each is enough to fail if an entry point stops resolving at all, which
// is the failure the dead `./dist/*` map would have produced for a consumer.

type RootSessionBuilder = Assert<Exact<typeof barrel.SessionBuilder, typeof barrel.SessionBuilder>>;
type HeadlessAsset =
  Assert<Exact<typeof headlessBarrel.MisoEngineAsset, typeof barrel.MisoEngineAsset>>;
type BrowserPolicy =
  Assert<Exact<typeof browserBarrel.scratchBootOptions, typeof browserBarrel.scratchBootOptions>>;

export type BarrelSurfacePins = [
  AgentCatalogFn,
  AgentParameterFn,
  AgentDecimalFn,
  AgentHandle,
  AgentCatalogParameter,
  AgentSetAck,
  WriterClass,
  WriterInstance,
  WriterLaneEdit,
  WriterFlushOutcome,
  WriterStatsType,
  WriterOptionsType,
  LatticePointsFn,
  LatticeResolveStepFn,
  LatticeIndexForDecimalFn,
  LatticeStepSizesConst,
  LatticeMaximumConst,
  LatticePointType,
  LatticeDeclarationType,
  LatticeNearestType,
  LatticeStepDecl,
  LatticeStepSize,
  CatalogStepDecl,
  CatalogStepSize,
  StepDeclarationsDiffer,
  HostMirrorFn,
  RootSessionBuilder,
  HeadlessAsset,
  BrowserPolicy,
];
