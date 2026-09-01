/**
 * Exact decimal arithmetic for the parameter lattice (issue #242, consumed by #243 S4).
 *
 * # Why `toFixed` is not usable here
 *
 * The engine renders every lattice point with Rust's `format!("{:.*}", precision, value)`, which
 * rounds the value's *exact* binary expansion half-to-even. JavaScript's `Number.prototype.toFixed`
 * rounds half away from zero. The two disagree on every exact tie, and ties are not exotic in this
 * domain: `0.5` at precision 0 renders `0` in Rust and `1` in JavaScript, `0.125` at precision 2
 * renders `0.12` and `0.13`, and a lattice whose step is a negative power of two produces exact
 * ties by construction.
 *
 * A single differing point would put the SDK's whole rank numbering one off the engine's from that
 * point upward -- every `step()` would land somewhere else, and `set()` would refuse legal values.
 * So the rendering below is exact: a finite double is a dyadic rational, `m * 2^e`, so scaling it
 * by `10^p` and rounding is integer arithmetic on `BigInt` with no floating point anywhere in the
 * rounding decision.
 *
 * `tools/parameter-metadata/src/bin/lattice_oracle` is what proves this claim rather
 * than merely stating it: the SDK's points are digested and compared against the engine's own, for
 * every parameter in the shipped catalog.
 */

import { MisoUsageError } from "./errors.ts";

/** Decompose a finite double into `sign * mantissa * 2^exponent` with an integer mantissa. */
function decompose(value: number): { negative: boolean; mantissa: bigint; exponent: number } {
  const view = new DataView(new ArrayBuffer(8));
  view.setFloat64(0, value, true);
  const bits = view.getBigUint64(0, true);
  const negative = (bits >> 63n) === 1n;
  const rawExponent = Number((bits >> 52n) & 0x7ffn);
  const rawMantissa = bits & 0xf_ffff_ffff_ffffn;
  // Subnormals carry no implicit leading bit and share the minimum exponent.
  return rawExponent === 0
    ? { negative, mantissa: rawMantissa, exponent: -1074 }
    : { negative, mantissa: rawMantissa | 0x10_0000_0000_0000n, exponent: rawExponent - 1075 };
}

function pow10(exponent: number): bigint {
  return 10n ** BigInt(exponent);
}

/**
 * Render a double at fixed precision, rounding half-to-even on the exact value.
 *
 * This is Rust's `format!("{:.*}", precision, value)` for a finite `f64`, and it is the only
 * rendering the SDK ever produces for a lattice point.
 */
export function fixedHalfEven(value: number, precision: number): string {
  if (!Number.isFinite(value)) {
    throw new MisoUsageError(`cannot render ${value} at fixed precision`);
  }
  if (!Number.isInteger(precision) || precision < 0 || precision > 8) {
    throw new MisoUsageError(`precision must be an integer in 0..=8, got ${precision}`);
  }
  const { negative, mantissa, exponent } = decompose(value);
  const scale = pow10(precision);

  // value * 10^p  =  mantissa * 2^exponent * 10^p, kept as an exact fraction.
  let numerator = mantissa * scale;
  let denominator = 1n;
  if (exponent >= 0) numerator <<= BigInt(exponent);
  else denominator = 1n << BigInt(-exponent);

  let quotient = numerator / denominator;
  const remainder = numerator - quotient * denominator;
  const twice = remainder * 2n;
  if (twice > denominator || (twice === denominator && (quotient & 1n) === 1n)) {
    quotient += 1n;
  }

  const digits = quotient.toString().padStart(precision + 1, "0");
  const whole = digits.slice(0, digits.length - precision);
  const fraction = precision === 0 ? "" : `.${digits.slice(digits.length - precision)}`;
  // Rust prints `-0` for a negative value that rounds to zero, and so does this: the sign is the
  // value's, not the rendering's.
  return `${negative ? "-" : ""}${whole}${fraction}`;
}

/**
 * Parse a fixed-precision rendering into its scaled integer, or `undefined` if it is not one.
 *
 * Mirrors the engine's `scaled`: the fraction must carry *exactly* `precision` digits, so a
 * rendering that lost or gained a digit is rejected rather than silently rescaled.
 */
export function scaledDecimal(text: string, precision: number): bigint | undefined {
  const negative = text.startsWith("-");
  const body = negative ? text.slice(1) : text;
  const point = body.indexOf(".");
  const whole = point < 0 ? body : body.slice(0, point);
  const fraction = point < 0 ? "" : body.slice(point + 1);
  if (whole.length === 0 || !/^[0-9]+$/.test(whole)) return undefined;
  if (fraction.length !== precision) return undefined;
  if (precision > 0 && !/^[0-9]+$/.test(fraction)) return undefined;
  const magnitude = BigInt(whole) * pow10(precision) + (precision === 0 ? 0n : BigInt(fraction));
  return negative ? -magnitude : magnitude;
}

/**
 * The shortest decimal that round-trips through `f32`, which is the value's decimal NAME.
 *
 * Rust's `f32` `Display` is exactly this, and the engine uses its digit count to prove an
 * intrinsic point is spellable at its row's precision. JavaScript has no `f32` formatter, so the
 * shortest round-trip is found by search: the smallest significant-digit count whose parse narrows
 * back to the same `f32`. Nine digits always suffice for `f32`.
 */
export function shortestFloat32(value: number): string {
  const narrowed = Math.fround(value);
  for (let digits = 1; digits <= 9; digits += 1) {
    const candidate = narrowed.toPrecision(digits);
    if (Math.fround(Number(candidate)) === narrowed) {
      // `toPrecision` may return exponent notation; the engine's formatter never does, and a digit
      // count taken from an exponent spelling would be meaningless.
      if (candidate.includes("e") || candidate.includes("E")) {
        return Number(candidate).toString();
      }
      return String(Number(candidate));
    }
  }
  return String(narrowed);
}

/** Digits after the decimal point in a value's shortest `f32` spelling. */
export function fractionDigits(value: number): number | undefined {
  const text = shortestFloat32(value);
  if (text.includes("e") || text.includes("E")) return undefined;
  const point = text.indexOf(".");
  return point < 0 ? 0 : text.length - point - 1;
}

/**
 * A decimal literal normalized to sign, integer digits and fraction digits.
 *
 * Mirrors the engine's `ExactDecimal`. Leading integer zeros and trailing fraction zeros are
 * removed, so `0.3`, `0.30` and `+0.300` are one value and compare equal without any scaling that
 * could overflow -- which is what lets a document's own spelling be preserved while still being
 * matched exactly against a canonical rendering.
 */
export interface ExactDecimal {
  readonly negative: boolean;
  readonly integer: string;
  readonly fraction: string;
}

const DECIMAL = /^[+-]?(?:[0-9]+(?:\.[0-9]*)?|\.[0-9]+)(?:[eE][+-]?[0-9]+)?$/;

export function parseExactDecimal(text: string): ExactDecimal | undefined {
  if (!DECIMAL.test(text)) return undefined;
  let negative = text.startsWith("-");
  let body = text.replace(/^[+-]/, "");

  const exponentAt = body.search(/[eE]/);
  let exponent = 0;
  if (exponentAt >= 0) {
    exponent = Number(body.slice(exponentAt + 1));
    body = body.slice(0, exponentAt);
  }

  const point = body.indexOf(".");
  let integer = point < 0 ? body : body.slice(0, point);
  let fraction = point < 0 ? "" : body.slice(point + 1);

  // Apply the exponent by moving the point, so the result stays exact.
  if (exponent > 0) {
    const moved = fraction.padEnd(exponent, "0");
    integer += moved.slice(0, exponent);
    fraction = moved.slice(exponent);
  } else if (exponent < 0) {
    const shift = -exponent;
    const moved = integer.padStart(shift, "0");
    fraction = moved.slice(moved.length - shift) + fraction;
    integer = moved.slice(0, moved.length - shift);
  }

  integer = integer.replace(/^0+/, "");
  fraction = fraction.replace(/0+$/, "");
  if (integer === "" && fraction === "") negative = false; // one zero, never a negative one
  return { negative, integer, fraction };
}

/** Total order over exact decimals: `-1`, `0` or `1`. */
export function compareExactDecimal(left: ExactDecimal, right: ExactDecimal): number {
  if (left.negative !== right.negative) return left.negative ? -1 : 1;
  const sign = left.negative ? -1 : 1;
  if (left.integer.length !== right.integer.length) {
    return left.integer.length < right.integer.length ? -sign : sign;
  }
  if (left.integer !== right.integer) return left.integer < right.integer ? -sign : sign;
  const width = Math.max(left.fraction.length, right.fraction.length);
  const leftFraction = left.fraction.padEnd(width, "0");
  const rightFraction = right.fraction.padEnd(width, "0");
  if (leftFraction === rightFraction) return 0;
  return leftFraction < rightFraction ? -sign : sign;
}
