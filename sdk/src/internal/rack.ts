import { ABI_LAYOUT } from "../generated/abi.ts";
import type { LiveControlRack } from "../core/control-builder-support.ts";
import { MisoUsageError } from "../core/errors.ts";

/**
 * Internal rack codes of the observation command record. This module is deliberately absent from
 * every package export map and public barrel.
 */

function rackCode(name: string): number {
  const row = ABI_LAYOUT.constants.racks.find((candidate) => candidate.name === name);
  if (row === undefined) throw new MisoUsageError(`the generated ABI layout has no rack ${name}`);
  return row.value;
}

/**
 * The observation record's rack byte, from the generated layout's `racks` table (S1c): `1` inserts
 * and `3` console. The retired `0` and `2` and the not-applicable `255` are never written.
 */
export const RACKS: Readonly<Record<LiveControlRack, number>> = Object.freeze({
  console: rackCode("console"),
  inserts: rackCode("inserts"),
});

const RACK_CODES: ReadonlySet<number> = new Set(Object.values(RACKS));
const RACK_TEXT = `rack must be ${RACKS.inserts} (inserts) or ${RACKS.console} (console)`;

/**
 * A rack code an observation record may carry, as `DataView.setUint8` needs it: it would wrap
 * `257` to `1` and truncate `1.5`. The browser host refuses the same set (#1542).
 */
export function requireRack(rack: number): number {
  if (!RACK_CODES.has(rack)) throw new MisoUsageError(RACK_TEXT);
  return rack;
}
