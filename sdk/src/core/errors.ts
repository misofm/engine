import { ABI_LAYOUT } from "../generated/abi.ts";
import type { BootResultAliasName, ErrorPhase, ResultCodeName } from "../generated/abi.ts";

export type { ErrorPhase, ResultCodeName, BootResultAliasName };

/**
 * The name a refusal carries.
 *
 * Boot's refusals are *aliases* of the general result codes rather than fresh numbers -- value 2
 * is `abiMismatch` when a module's version word is wrong and `refusedOptions` when `boot()`
 * rejected the options block -- so the vocabulary a caller sees is the union of both spellings and
 * the SDK chooses by context. Adopted ruling 5462139867 finding 2.
 */
export type MisoErrorCode = ResultCodeName | BootResultAliasName;

const RESULT_NAMES: ReadonlyMap<number, ResultCodeName> = new Map(
  ABI_LAYOUT.constants.resultCodes.map((row) => [row.value, row.name] as const),
);
const BOOT_ALIASES: ReadonlyMap<number, BootResultAliasName> = new Map(
  ABI_LAYOUT.constants.bootResultAliases.map((row) => [row.value, row.name] as const),
);

/**
 * Name a raw result code.
 *
 * `context` is what makes the alias table usable rather than ambiguous: `"boot"` reads value 1 as
 * `refusedDocument`, and every other call site reads it as `invalidArgument`. There is no third
 * option -- a table that named value 2 `refusedOptions` everywhere would misname every non-boot
 * acknowledgement, and one that named it `abiMismatch` everywhere would leave boot's vocabulary
 * unrepresentable.
 */
export function resultName(value: number, context: "call"): ResultCodeName;
export function resultName(value: number, context: "boot"): MisoErrorCode;
export function resultName(value: number, context: "boot" | "call"): MisoErrorCode {
  if (context === "boot") {
    const alias = BOOT_ALIASES.get(value);
    if (alias !== undefined) return alias;
  }
  return RESULT_NAMES.get(value) ?? "internal";
}

/** One line of the engine's `code\tpath\n` diagnostic buffer. */
export interface MisoDiagnostic {
  /** The stable registry string, e.g. `sample_rate.unsupported_at_launch`. */
  readonly code: string;
  /** The JSON-path-shaped location, e.g. `$.sample_rate_hz`. Empty when the code has no path. */
  readonly path: string;
}

/**
 * Parse the engine's diagnostic buffer.
 *
 * The format is deliberately not JSON: `code\tpath\n` per line, so the wasm side can write it with
 * no allocator and the reader needs no parser. A trailing bracketed detail (the budget refusal's
 * `[projected_bytes=..,budget_bytes=..]`) rides on the path field, where a caller that wants it
 * can read it and a caller that matches on `code` never sees it.
 */
export function parseDiagnostics(text: string): readonly MisoDiagnostic[] {
  const rows: MisoDiagnostic[] = [];
  for (const line of text.split("\n")) {
    if (line.length === 0) continue;
    const tab = line.indexOf("\t");
    rows.push(
      tab < 0
        ? { code: line, path: "" }
        : { code: line.slice(0, tab), path: line.slice(tab + 1) },
    );
  }
  return Object.freeze(rows);
}

export interface MisoEngineErrorInit {
  readonly phase: ErrorPhase;
  readonly code: MisoErrorCode;
  readonly result: number;
  readonly diagnostics?: readonly MisoDiagnostic[];
}

/**
 * A typed refusal.
 *
 * `phase` is the thing a numeric result code structurally cannot say: result 2 means one thing
 * when a module's version word did not match and another when `boot()` rejected the options block.
 * The six phases -- `asset`, `boot`, `source`, `render`, `output`, `lifecycle` -- come from the
 * generated layout document and are anchored to the export surface, replacing the dead two-phase
 * `"compile"` spelling of the pre-boot-v1 ABI (issue #243 S2(b)).
 */
export class MisoEngineError extends Error {
  readonly phase: ErrorPhase;
  readonly code: MisoErrorCode;
  readonly result: number;
  readonly diagnostics: readonly MisoDiagnostic[];

  constructor(message: string, init: MisoEngineErrorInit) {
    const diagnostics = init.diagnostics ?? [];
    const detail = diagnostics.length === 0
      ? ""
      : ` (${diagnostics.map((row) => (row.path ? `${row.code} at ${row.path}` : row.code)).join("; ")})`;
    super(`${init.phase}: ${init.code}: ${message}${detail}`);
    this.name = "MisoEngineError";
    this.phase = init.phase;
    this.code = init.code;
    this.result = init.result;
    this.diagnostics = Object.freeze([...diagnostics]);
  }

  /** The first diagnostic's stable code, which is what an assertion usually wants. */
  get diagnosticCode(): string | undefined {
    return this.diagnostics[0]?.code;
  }

  /** The first diagnostic's path. */
  get diagnosticPath(): string | undefined {
    return this.diagnostics[0]?.path;
  }
}

/** A programming error in SDK usage -- never an engine refusal. */
export class MisoUsageError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "MisoUsageError";
  }
}
