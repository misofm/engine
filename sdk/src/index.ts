/** Zero-runtime-dependency Engine V2 catalog, Session V1, and command-control surface. */
export * from "./generated/catalog.ts";
export * from "./generated/abi.ts";
export * from "./generated/provenance.ts";
export * from "./core/abi.ts";
export * from "./core/agent.ts";
export * from "./core/asset.ts";
export * from "./core/boundary.ts";
export * from "./core/errors.ts";
export * from "./core/session.ts";
export * from "./core/writer.ts";
export type * from "./core/types.ts";

/**
 * `core/lattice.ts`, minus the two names `generated/catalog.ts` already occupies.
 *
 * Both modules spell `StepDeclaration` and `StepSizeName`, and they are not the same type. The
 * generated one is derived from the frozen catalog (`EffectParameter<EffectId>["step"]`), so it is
 * the narrow union of the literal step objects the shipped rows actually carry; the lattice's is
 * the structural shape its generator accepts, which is wider by construction because it must also
 * accept a builtin row and a hand-built declaration. A bare `export *` would collide, and
 * collapsing them would silently widen the catalog's spelling for every existing consumer.
 *
 * So the generated names keep the barrel -- they were reachable through it first, and they are the
 * tighter type -- and the lattice's structural spellings are re-exported beside them under
 * `Lattice*`. The generated file is untouched: this is the only site that knows about the overlap.
 */
export {
  MAXIMUM_LATTICE_POINTS,
  STEP_SIZES,
  indexForDecimal,
  latticePoints,
  resolveStep,
} from "./core/lattice.ts";
export type {
  LatticeDeclaration,
  LatticePoint,
  NearestLatticeValues,
  StepDeclaration as LatticeStepDeclaration,
  StepSizeName as LatticeStepSizeName,
} from "./core/lattice.ts";
