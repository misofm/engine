import {
  compareExactDecimal,
  fixedHalfEven,
  fractionDigits,
  parseExactDecimal,
  scaledDecimal,
} from "./decimal.ts";
import { MisoUsageError } from "./errors.ts";

/**
 * The finite set of legal persisted values for one parameter (issue #242, consumed by #243 S4).
 *
 * # Why the SDK generates these rather than reading them
 *
 * The shipped catalog carries each parameter's lattice *declaration* -- step, unit, precision, the
 * five ladder multiples -- which is a few dozen bytes. It does not carry the points, because a
 * one-cent lattice from 20 Hz to 20 kHz has about twelve thousand of them and shipping the whole
 * catalog's points would cost megabytes to say something derivable.
 *
 * So this module derives them, and the derivation is held to the engine's own by
 * `tools/miso-engine-parameter-metadata/src/bin/lattice_oracle`, which builds every lattice through
 * `parameter_lattice_points` -- the engine's resolver, the one the descriptor-wire verifier and the
 * compiler both use -- and digests the result. The SDK's eval reproduces that digest for every
 * parameter in the catalog. This file is therefore a *second implementation held to the first*,
 * never an independent opinion about what a lattice is.
 *
 * # The rank law
 *
 * A point's `index` is its RANK in the totally-ordered member set: 0 is the minimum, and the
 * declared bounds and default occupy ranks like any other point rather than sitting off-grid.
 * `step(±1)` is rank ±1 -- the detent IS a stop on the dial. Adopted ruling 5462139867 finding 7,
 * which superseded #242 S3's earlier "k relative to min" spelling. That is why the flanks of an
 * intrinsic member are allowed to be irregular: a round maximum and a meaningful default are worth
 * more than uniform spacing at the two adjacencies that touch them.
 */

/** One legal persisted value and its lossless wire index. */
export interface LatticePoint {
  /** Zero-based rank in ascending numeric order. This is what travels on the wire. */
  readonly index: number;
  /** The one canonical decimal rendering. */
  readonly canonical: string;
  /** True for a descriptor-declared bound, default or enumeration choice. */
  readonly intrinsic: boolean;
}

/** The five named step sizes, smallest first. */
export const STEP_SIZES = Object.freeze(["xs", "sm", "md", "lg", "xl"] as const);
export type StepSizeName = (typeof STEP_SIZES)[number];

/** The declaration a catalog row carries, in the shipped metadata's own shape. */
export interface StepDeclaration {
  readonly unit: string;
  readonly size: string;
  readonly precision: number;
  readonly ladder: Readonly<Record<StepSizeName, number>>;
}

/** Everything the lattice generator needs about a parameter row. */
export interface LatticeDeclaration {
  readonly domainName: string;
  readonly minimum: number | null;
  readonly maximum: number | null;
  readonly default: number;
  readonly enumChoices: readonly { readonly value: number }[] | null;
  readonly step: StepDeclaration;
  /**
   * Whether the declared maximum is itself a member.
   *
   * False only for the rate-keyed builtin shape, where the ceiling is #242 S1's clamp rather than
   * a declared bound: the top point is *generated* there, not admitted. Adopted ruling finding 12.
   */
  readonly maximumIsMember?: boolean;
}

/** The engine's cap, transcribed: a declaration that would exceed it is a declaration error. */
export const MAXIMUM_LATTICE_POINTS = 1_000_000;

/**
 * Narrow a catalog number to the `f32` the descriptor actually declares.
 *
 * Every numeric field in the shipped metadata is an `f32` printed as its shortest round-tripping
 * spelling, and the engine reads it back as that `f32` and widens it to `f64` before it does any
 * arithmetic. JSON has only doubles, so `1.02` parses to the *exact* decimal 1.02 rather than to
 * `1.02f32`, which is 1.0199999809265137 -- and on a geometric lattice that difference compounds
 * with every power. It is not a rounding subtlety: at eight decimals it moves the second point
 * from `1.01999998` to `1.02000000` and every point after it.
 *
 * So every number this module takes from the catalog passes through here first, and the lattice
 * arithmetic below is `f64` arithmetic on `f32` values -- exactly what the engine does.
 */
function declared(value: number): number {
  return Math.fround(value);
}

type Entry = { scaled: bigint; canonical: string; intrinsic: boolean };

function insert(values: Entry[], canonical: string, precision: number, intrinsic: boolean): void {
  const value = scaledDecimal(canonical, precision);
  if (value === undefined) {
    throw new MisoUsageError(`the rendering ${canonical} is not exact at precision ${precision}`);
  }
  let low = 0;
  let high = values.length;
  while (low < high) {
    const middle = (low + high) >>> 1;
    const candidate = values[middle]!;
    if (candidate.scaled === value) {
      // A generated point that coincides with a declared one keeps the declaration's flag: the
      // two are one member, and the flag says the member was declared, not how it was reached.
      candidate.intrinsic ||= intrinsic;
      return;
    }
    if (candidate.scaled < value) low = middle + 1;
    else high = middle;
  }
  values.splice(low, 0, { scaled: value, canonical, intrinsic });
}

/**
 * Render a declared value at the row's precision, refusing one that cannot be spelled there.
 *
 * The launch case this catches is a bound carrying more decimals than its row allows: `0.995` at
 * two decimals would render `1.00`, which is outside its own domain. The engine proves this in
 * decimal rather than by round-tripping through `f32`, and so does this.
 */
function intrinsicDecimal(value: number, precision: number): string {
  const digits = fractionDigits(value);
  if (digits === undefined || digits > precision) {
    throw new MisoUsageError(
      `the declared value ${value} needs ${digits ?? "an exponent"} fraction digits, `
      + `but its row is pinned to ${precision}`,
    );
  }
  return fixedHalfEven(value, precision);
}

/** Build every legal persisted value for a parameter, in ascending order. */
export function latticePoints(declaration: LatticeDeclaration): readonly LatticePoint[] {
  const { precision } = declaration.step;
  const maximumIsMember = declaration.maximumIsMember ?? true;
  const values: Entry[] = [];

  if (declaration.domainName === "boolean") {
    insert(values, "0", 0, true);
    insert(values, "1", 0, true);
  } else if (declaration.domainName === "enumeration") {
    // A persisted document spells an enumeration as its CHOICE VALUE, so the choice values are the
    // canonical renderings; the point's rank stays the choice index, which is what persists.
    for (const choice of declaration.enumChoices ?? []) {
      insert(values, intrinsicDecimal(declared(choice.value), precision), precision, true);
    }
  } else {
    if (declaration.minimum === null || declaration.maximum === null) {
      throw new MisoUsageError("a continuous parameter must declare both bounds");
    }
    const minimum = declared(declaration.minimum);
    const maximum = declared(declaration.maximum);
    insert(values, intrinsicDecimal(minimum, precision), precision, true);
    insert(values, intrinsicDecimal(declared(declaration.default), precision), precision, true);
    if (maximumIsMember) insert(values, intrinsicDecimal(maximum, precision), precision, true);

    const minimumScaled = scaledDecimal(fixedHalfEven(minimum, precision), precision)!;
    const maximumScaled = scaledDecimal(fixedHalfEven(maximum, precision), precision)!;
    if (minimumScaled >= maximumScaled) {
      throw new MisoUsageError("a lattice's minimum must be below its maximum at its precision");
    }

    const stepSize = declared(Number(declaration.step.size));
    if (declaration.step.unit === "absolute") {
      const stepScaled = scaledDecimal(fixedHalfEven(stepSize, precision), precision);
      if (stepScaled === undefined || stepScaled <= 0n) {
        throw new MisoUsageError(`the step ${declaration.step.size} is not positive at precision ${precision}`);
      }
      // Integer arithmetic on the scaled decimals, so an arithmetic lattice accumulates no error
      // however many points it has -- which matters: delay time alone has nearly twenty thousand.
      for (
        let value = minimumScaled + stepScaled;
        maximumIsMember ? value < maximumScaled : value <= maximumScaled;
        value += stepScaled
      ) {
        if (values.length >= MAXIMUM_LATTICE_POINTS) {
          throw new MisoUsageError("this declaration exceeds the engine's lattice point cap");
        }
        insert(values, renderScaled(value, precision), precision, false);
      }
    } else if (declaration.step.unit === "cents" || declaration.step.unit === "ratio") {
      if (minimum <= 0) {
        throw new MisoUsageError("a geometric lattice's minimum must be positive");
      }
      const ratio = declaration.step.unit === "cents" ? 2 ** (stepSize / 1200) : stepSize;
      for (let k = 1; ; k += 1) {
        if (values.length >= MAXIMUM_LATTICE_POINTS) {
          throw new MisoUsageError("this declaration exceeds the engine's lattice point cap");
        }
        const value = minimum * ratio ** k;
        if (!Number.isFinite(value)) break;
        if (maximumIsMember ? !(value < maximum) : !(value <= maximum)) break;
        const canonical = fixedHalfEven(value, precision);
        const rendered = scaledDecimal(canonical, precision);
        if (rendered === undefined) {
          throw new MisoUsageError(`the geometric point ${canonical} is not exact`);
        }
        // The rendering, not the double, decides membership: a point that rounds onto the maximum
        // must not be emitted twice under two spellings.
        if (maximumIsMember ? rendered >= maximumScaled : rendered > maximumScaled) break;
        insert(values, canonical, precision, false);
      }
    } else {
      throw new MisoUsageError(`a continuous row cannot carry step unit ${declaration.step.unit}`);
    }
  }

  return Object.freeze(values.map((entry, index) => Object.freeze({
    index,
    canonical: entry.canonical,
    intrinsic: entry.intrinsic,
  })));
}

/** Render a scaled integer back to its fixed-precision decimal. */
function renderScaled(value: bigint, precision: number): string {
  const negative = value < 0n;
  const digits = (negative ? -value : value).toString().padStart(precision + 1, "0");
  const whole = digits.slice(0, digits.length - precision);
  const fraction = precision === 0 ? "" : `.${digits.slice(digits.length - precision)}`;
  return `${negative ? "-" : ""}${whole}${fraction}`;
}

/**
 * Move a rank by a named ladder size, clamped to the lattice's endpoints.
 *
 * Clamping rather than refusing is the engine's choice and the right one for a dial: a nudge past
 * the top lands on the top, which is what a hand on a control expects. `count === 0` is `undefined`
 * because a step of nothing is a caller mistake rather than a no-op worth acknowledging.
 */
export function resolveStep(
  points: readonly LatticePoint[],
  current: number,
  size: StepSizeName,
  count: number,
  ladder: Readonly<Record<StepSizeName, number>>,
): number | undefined {
  if (points.length === 0 || current < 0 || current >= points.length || count === 0) {
    return undefined;
  }
  const delta = ladder[size] * count;
  return Math.min(Math.max(current + delta, 0), points.length - 1);
}

export interface NearestLatticeValues {
  readonly lower: string | undefined;
  readonly upper: string | undefined;
}

/**
 * The rank a decimal text names, or the two points that bracket it.
 *
 * Membership is decided on the *text*, in exact decimal arithmetic, never by comparing `f32` words:
 * two different decimals routinely round to one `f32`, so an `f32` comparison silently admits
 * off-lattice text. That is the `0.3`-with-step-`0.1` class the lattice exists to catch.
 * Equivalent spellings of one number -- `0.3`, `0.30`, `3e-1`, `+0.300` -- are the same decimal and
 * all match.
 */
export function indexForDecimal(
  points: readonly LatticePoint[],
  text: string,
): { readonly index: number } | { readonly nearest: NearestLatticeValues } {
  const value = parseExactDecimal(text);
  if (value === undefined) {
    return { nearest: { lower: undefined, upper: undefined } };
  }
  let low = 0;
  let high = points.length;
  while (low < high) {
    const middle = (low + high) >>> 1;
    const point = points[middle]!;
    const candidate = parseExactDecimal(point.canonical);
    if (candidate === undefined) {
      throw new MisoUsageError(`the canonical rendering ${point.canonical} is not a decimal`);
    }
    const order = compareExactDecimal(candidate, value);
    if (order === 0) return { index: point.index };
    if (order < 0) low = middle + 1;
    else high = middle;
  }
  return {
    nearest: {
      lower: low > 0 ? points[low - 1]!.canonical : undefined,
      upper: low < points.length ? points[low]!.canonical : undefined,
    },
  };
}
