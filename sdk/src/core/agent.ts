import { CATALOG } from "../generated/catalog.ts";
import type { EffectId } from "../generated/catalog.ts";
import { MisoUsageError } from "./errors.ts";
import {
  STEP_SIZES,
  indexForDecimal,
  latticePoints,
  resolveStep,
} from "./lattice.ts";
import type { LatticePoint, NearestLatticeValues, StepSizeName } from "./lattice.ts";

/**
 * The agent operations surface (issue #243 S4).
 *
 * # Agents speak the lattice, not floats
 *
 * A number that reaches an agent as `0.30000001192092896` is a number the agent cannot reason
 * about, cannot round-trip, and cannot compare to the value it asked for. Every value on this
 * surface is therefore a *canonical decimal string*, and every edit travels as an integer rank on
 * the lattice. No `f32` is formatted anywhere in this file, and no value a caller supplies is
 * parsed as a float to decide anything: membership is decided in exact decimal arithmetic on the
 * text (see `indexForDecimal`), which is what makes `set(p, "0.3")` either exactly right or a
 * typed refusal, never a value 0.0000000119 away from what was asked for.
 *
 * # A refusal is an answer
 *
 * Nothing here throws for a value the engine would decline. `set` of an off-lattice decimal returns
 * an ack naming the two points that bracket it, so an agent's next move is obvious rather than
 * exploratory. Only *usage* errors -- naming a parameter that does not exist -- throw, because
 * those are the caller's bug rather than the engine's answer.
 *
 * # What this surface is, and what it is not
 *
 * Ranks and canonical decimals are the persisted-edit vocabulary. Applying an edit to a live
 * session is a separate act with a separate carrier, and against the offline wasm engine that
 * carrier is the 48-byte live-console record, whose value words are continuous `f32` by design
 * (#137 D1: there is no string on the hot path). `decimalToFloat32` below is the single site where
 * a canonical decimal becomes such an `f32`, so an audit of that boundary is one grep.
 */

/** One parameter row, as an agent sees it. */
export interface CatalogParameter {
  readonly effectId: EffectId;
  readonly id: number;
  readonly name: string;
  /** Display unit token, e.g. `db`, `hz`, `ratio`. */
  readonly unit: string;
  readonly domain: string;
  /** Declared bounds as canonical decimals -- the lattice's own endpoints, not raw floats. */
  readonly minimum: string;
  readonly maximum: string;
  readonly default: string;
  /** The lattice declaration: step size as a decimal string, its unit, precision, ladder. */
  readonly step: {
    readonly unit: string;
    readonly size: string;
    readonly precision: number;
    readonly ladder: Readonly<Record<StepSizeName, number>>;
  };
  /** Whether the live-console command path can move this parameter, or only preparation can. */
  readonly liveUpdatable: boolean;
  /** Lane scope: `shared` moves both lanes together, `perLane` addresses one. */
  readonly channelPolicy: string;
  /** How many legal persisted values this parameter has. */
  readonly points: number;
}

function ladderOf(row: { readonly ladder: Readonly<Record<string, number>> }): Readonly<Record<StepSizeName, number>> {
  const ladder: Record<string, number> = {};
  for (const size of STEP_SIZES) {
    const multiple = row.ladder[size];
    if (typeof multiple !== "number") {
      throw new MisoUsageError(`the catalog's ladder has no ${size} multiple`);
    }
    ladder[size] = multiple;
  }
  return Object.freeze(ladder) as Readonly<Record<StepSizeName, number>>;
}

type CatalogRow = (typeof CATALOG)["effects"][number]["parameters"][number];

function declarationOf(row: CatalogRow) {
  return {
    domainName: row.domainName,
    minimum: row.minimum,
    maximum: row.maximum,
    default: row.default,
    enumChoices: row.enumChoices,
    step: { ...row.step, ladder: ladderOf(row.step) },
  };
}

/**
 * A parameter's lattice, its declaration, and a cursor on it.
 *
 * The cursor is what makes `step` a *relative* verb without an intent ledger: the surface holds
 * the rank it last resolved, and a step moves that rank. There is no history and no undo stack --
 * a rank is the whole state, and it is a `u32`.
 */
export class ParameterHandle {
  readonly declaration: CatalogParameter;
  readonly points: readonly LatticePoint[];
  #index: number;

  constructor(effectId: EffectId, row: CatalogRow) {
    const declaration = declarationOf(row);
    this.points = latticePoints(declaration);
    const start = indexForDecimal(this.points, declaration.step.precision === 0
      ? String(row.default)
      : row.default.toFixed(declaration.step.precision));
    // The default is an intrinsic member by declaration, so it is always found; if a future
    // descriptor made that untrue the cursor would start at the minimum rather than at a guess.
    this.#index = "index" in start ? start.index : 0;
    this.declaration = Object.freeze({
      effectId,
      id: row.id,
      name: row.name,
      unit: row.unitName,
      domain: row.domainName,
      minimum: this.points[0]?.canonical ?? "",
      maximum: this.points[this.points.length - 1]?.canonical ?? "",
      default: this.points[this.#index]?.canonical ?? "",
      step: Object.freeze({
        unit: row.step.unit,
        size: row.step.size,
        precision: row.step.precision,
        ladder: ladderOf(row.step),
      }),
      liveUpdatable: row.liveUpdatable,
      channelPolicy: row.channelPolicyName,
      points: this.points.length,
    });
  }

  /** The current rank. This is what a persisted edit carries on the wire. */
  get index(): number {
    return this.#index;
  }

  /** The current value's canonical decimal. Read-back is always this, never a formatted float. */
  get value(): string {
    return this.points[this.#index]?.canonical ?? "";
  }

  /** Whether the current point is a declared bound, default or enumeration choice. */
  get intrinsic(): boolean {
    return this.points[this.#index]?.intrinsic ?? false;
  }

  /**
   * Set by canonical decimal.
   *
   * Any spelling of the same number is accepted -- `0.3`, `0.30`, `3e-1`, `+0.300` are one decimal
   * -- because the document that carries the value is allowed to keep its author's spelling. What
   * comes back is always the canonical rendering.
   */
  set(decimal: string): SetAck {
    const found = indexForDecimal(this.points, decimal);
    if ("index" in found) {
      this.#index = found.index;
      return Object.freeze({
        ok: true as const,
        index: found.index,
        value: this.value,
      });
    }
    return Object.freeze({
      ok: false as const,
      reason: "offLattice" as const,
      requested: decimal,
      nearest: found.nearest,
      index: this.#index,
      value: this.value,
    });
  }

  /** Set by absolute rank. Out of range is a refusal, not a clamp: an index is not a gesture. */
  setSteps(index: number): SetAck {
    if (!Number.isInteger(index) || index < 0 || index >= this.points.length) {
      return Object.freeze({
        ok: false as const,
        reason: "outOfRange" as const,
        requested: String(index),
        nearest: Object.freeze({
          lower: this.points[0]?.canonical,
          upper: this.points[this.points.length - 1]?.canonical,
        }),
        index: this.#index,
        value: this.value,
      });
    }
    this.#index = index;
    return Object.freeze({ ok: true as const, index, value: this.value });
  }

  /**
   * Move by `count` ladder steps of the named size, clamped at the endpoints.
   *
   * Clamping rather than refusing is deliberate and is the engine's own rule: a nudge past the top
   * of a dial lands on the top. That is why `step` returns `ok` at an endpoint while `setSteps`
   * refuses an out-of-range index -- one is a gesture, the other is an address.
   */
  step(size: StepSizeName, count: number): SetAck {
    const target = resolveStep(this.points, this.#index, size, count, this.declaration.step.ladder);
    if (target === undefined) {
      return Object.freeze({
        ok: false as const,
        reason: "noStep" as const,
        requested: `${size}x${count}`,
        nearest: Object.freeze({ lower: undefined, upper: undefined }),
        index: this.#index,
        value: this.value,
      });
    }
    this.#index = target;
    return Object.freeze({ ok: true as const, index: target, value: this.value });
  }

  /** The rank difference between two states, which is what a diff of two sessions is expressed in. */
  stepsTo(other: ParameterHandle): number {
    if (other.declaration.effectId !== this.declaration.effectId
      || other.declaration.id !== this.declaration.id) {
      throw new MisoUsageError("two parameter handles must name the same parameter to be diffed");
    }
    return other.index - this.index;
  }
}

export type SetAck =
  | { readonly ok: true; readonly index: number; readonly value: string }
  | {
    readonly ok: false;
    readonly reason: "offLattice" | "outOfRange" | "noStep";
    readonly requested: string;
    readonly nearest: NearestLatticeValues;
    readonly index: number;
    readonly value: string;
  };

/** Every effect parameter in the shipped catalog, as agent-facing rows. */
export function catalog(): readonly CatalogParameter[] {
  const rows: CatalogParameter[] = [];
  for (const effect of CATALOG.effects) {
    for (const parameter of effect.parameters) {
      rows.push(new ParameterHandle(effect.id, parameter).declaration);
    }
  }
  return Object.freeze(rows);
}

/** A cursor on one named parameter. */
export function parameter(effectId: EffectId, name: string): ParameterHandle {
  const effect = CATALOG.effects.find((candidate) => candidate.id === effectId);
  if (effect === undefined) {
    throw new MisoUsageError(`the catalog has no effect ${effectId}`);
  }
  const row = effect.parameters.find((candidate) => candidate.name === name);
  if (row === undefined) {
    throw new MisoUsageError(`${effectId} has no parameter named ${name}`);
  }
  return new ParameterHandle(effectId, row);
}

/**
 * The SDK's single canonical-decimal-to-`f32` site.
 *
 * Its Rust counterpart is `miso_engine_effect_contract::decimal_to_f32`, and the two carry the
 * same precondition: the caller must first have proved the text is a descriptor-generated lattice
 * rendering. Keeping the parse here rather than at each call site makes a repo-wide audit of the
 * persisted-value precision boundary one grep, which is the whole reason the Rust side is written
 * that way too.
 *
 * The live command wire carries continuous `f32` by design, so this is the one place a lattice
 * value stops being a decimal. `-0.0` normalizes to `0.0`, matching the engine's signed-zero rule.
 */
export function decimalToFloat32(canonical: string): number | undefined {
  const value = Math.fround(Number(canonical));
  if (!Number.isFinite(value)) return undefined;
  return value === 0 ? 0 : value;
}
