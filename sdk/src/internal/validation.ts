import { MisoUsageError } from "../core/errors.ts";

/**
 * Validation primitives the Session V1 builder and its sibling internal helpers share. This module
 * is in no export map or public barrel, like `ramp.ts`: moved out of `core/session.ts` unchanged.
 */

/** The engine's diagnostic codes for the refusals the builder makes in front of it. */
export const CODE = Object.freeze({
  unknownField: "schema.unknown_field",
  invalidEnum: "schema.invalid_enum",
  invalidId: "id.invalid",
  duplicateId: "id.duplicate",
  missingEntity: "reference.missing_entity",
  entryOrder: "console.entry_order",
  entryMissing: "console.entry_missing",
  ineligibleEffect: "console.slot.ineligible_effect",
  wrongType: "schema.wrong_type",
  qualityUnsupported: "effect.quality.unsupported",
  linkModeUnsupported: "effect.link_mode.unsupported",
  outOfRange: "numeric.out_of_schema_range",
} as const);

export function fail(path: string, message: string, code?: string): never {
  throw new MisoUsageError(`${path}: ${message}`, code);
}

/** Refuse any own key outside `allowed`, as the engine refuses an unknown member. */
export function knownKeys(value: object, allowed: ReadonlySet<string>, path: string, what: string): void {
  for (const key of Object.keys(value)) {
    if (!allowed.has(key)) {
      fail(
        `${path}.${key}`,
        `${what} has no '${key}'; its keys are ${nameList([...allowed])}`,
        CODE.unknownField,
      );
    }
  }
}

export function freeze<T>(value: T): T {
  if (value !== null && typeof value === "object" && !Object.isFrozen(value)) {
    Object.freeze(value);
    for (const child of Object.values(value as Record<string, unknown>)) freeze(child);
  }
  return value;
}

/**
 * Round a caller's `f64` to the `f32` the document will actually carry.
 *
 * The rounding happens here rather than at emit time so that `toJSON()` reports the value the
 * engine will see. A gate that compared unrounded inputs would call two sessions equal that render
 * differently, and one that compared the *text* would be comparing the speller.
 */
export function f32(value: unknown, path: string): number {
  if (typeof value !== "number" || !Number.isFinite(value)) fail(path, "expected a finite number");
  const rounded = Math.fround(value);
  if (!Number.isFinite(rounded)) fail(path, "value is outside the finite f32 domain");
  return rounded;
}

/** `'a', 'b' and 'c'` -- the candidate list a refusal quotes back. */
export function nameList(names: readonly string[]): string {
  const quoted = names.map((name) => `'${name}'`);
  if (quoted.length <= 1) return quoted.join("");
  return `${quoted.slice(0, -1).join(", ")} and ${quoted[quoted.length - 1]}`;
}
