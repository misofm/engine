import { ABI_LAYOUT } from "../generated/abi.ts";
import type { LiveControlRack } from "../core/control-builder-support.ts";
import { MisoUsageError } from "../core/errors.ts";

/**
 * Internal rack codes of the numeric observation selection. This module is deliberately absent
 * from every package export map and public barrel.
 */

function rackCode(name: string): number {
  const row = ABI_LAYOUT.constants.racks.find((candidate) => candidate.name === name);
  if (row === undefined) throw new MisoUsageError(`the generated ABI layout has no rack ${name}`);
  return row.value;
}

/**
 * The observation selection's rack word, from the generated layout's `racks` table (S1c): `1`
 * inserts and `3` console. The retired `0` and `2` and the not-applicable `255` are never written.
 */
export const RACKS: Readonly<Record<LiveControlRack, number>> = Object.freeze({
  console: rackCode("console"),
  inserts: rackCode("inserts"),
});
