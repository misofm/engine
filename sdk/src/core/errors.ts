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
export type MisoErrorCode =
  | ResultCodeName
  | BootResultAliasName
  | SdkTimeoutCode
  | SdkControlCode
  | SdkResponseCode;

/**
 * A code the SDK itself raises when a bounded wait does not settle. It comes from the SDK, not
 * from the engine, so an error that carries it has no engine `result`.
 *
 * - `hostDisposeTimeout`: the worklet host did not answer its dispose request.
 * - `contextCloseTimeout`: `AudioContext.close()` did not settle (it also covers the boot-path
 *   context closes).
 * - `bootConfigureTimeout`: the browser boot's spectrum capability probe (`CAPABILITIES_GET`,
 *   #1596 D2; the taps themselves ride the boot, #2166) did not answer within `requestDeadlineMs`.
 */
export type SdkTimeoutCode =
  | "hostDisposeTimeout"
  | "contextCloseTimeout"
  | "bootConfigureTimeout";

/**
 * The core's own code for a control list or control state read that named a structure that is no
 * longer current (`control.list.stale`). The core sent it as a diagnostic of a refusal; the SDK
 * raises it as the error's `code` after a list read has restarted its allowed number of times, or
 * when a state read names a stale list. The caller lists again.
 */
export type SdkControlCode = "control.list.stale";

/**
 * The core's own diagnostic code for a refused response preview, such as `response.points` or
 * `response.reply_too_large`. The core sent it as the diagnostic of a refusal and the error's
 * `result` names the engine result class the refusal's status maps to. A frame the core could not
 * decode answers `protocol.failure`.
 */
export type SdkResponseCode = `response.${string}` | "protocol.failure";

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
  /** The engine's result code; absent when the SDK itself raised `code` (an `SdkTimeoutCode`). */
  readonly result?: number | undefined;
  readonly diagnostics?: readonly MisoDiagnostic[];
  /** What the SDK mapped into this error, such as the host's raw `miso.error.v1` record (#2201). */
  readonly cause?: unknown;
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
  /** The engine's result code, or `undefined` when the SDK raised `code` and the engine returned none. */
  readonly result: number | undefined;
  readonly diagnostics: readonly MisoDiagnostic[];

  constructor(message: string, init: MisoEngineErrorInit) {
    const diagnostics = init.diagnostics ?? [];
    const detail = diagnostics.length === 0
      ? ""
      : ` (${diagnostics.map((row) => (row.path ? `${row.code} at ${row.path}` : row.code)).join("; ")})`;
    super(`${init.phase}: ${init.code}: ${message}${detail}`, init.cause === undefined ? undefined : { cause: init.cause });
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

/**
 * A programming error in SDK usage -- never an engine refusal.
 *
 * When the SDK refuses, before boot, something the engine would refuse too -- a console entry out
 * of slot order, an ineligible console effect -- it names the engine's own diagnostic code, so a
 * caller sees one vocabulary whichever side answered first. `diagnosticCode` is that code, and it
 * is absent for a refusal the engine has no counterpart for (an API misuse such as a console
 * declared after a track).
 */
export class MisoUsageError extends Error {
  readonly diagnosticCode: string | undefined;

  constructor(message: string, diagnosticCode?: string) {
    super(diagnosticCode === undefined ? message : `${message} (${diagnosticCode})`);
    this.name = "MisoUsageError";
    this.diagnosticCode = diagnosticCode;
  }
}

/**
 * A producer failed after the engine had accepted the session seek (issue #1644 D3): the engine's
 * timeline and every source are at `generation`, the producers before `producerIndex` were
 * repositioned, and that one and the ones after it were not. `cause` is the producer's error.
 * Reposition the rest under `generation`; the seek is not to be sent again.
 */
export class SessionSeekProducerError extends Error {
  readonly generation: bigint;
  readonly producerIndex: number;

  constructor(generation: bigint, producerIndex: number, cause: unknown) {
    super(`the engine applied the session seek at generation ${generation}, but producer ${producerIndex} failed to reposition`, { cause });
    this.name = "SessionSeekProducerError";
    this.generation = generation;
    this.producerIndex = producerIndex;
  }
}

/**
 * A control transaction the core refused (#1563 D4).
 *
 * `code` is the core's own, the first diagnostic's stable dotted code (`control.domain`,
 * `control.address.unknown`, ...), or the reply's status name (`revision_conflict`, `backpressure`,
 * ...) when the core sent no diagnostic. The SDK adds no code of its own. A refused transaction
 * changed nothing and is not retried. `revision` is the revision the core reported with the refusal.
 *
 * One refusal is the SDK's, made before the frame is sent (#2190): a frame larger than the core's
 * reported `maximumFrameBytes` is refused with `status` and `code` `limit_exceeded`, the status the
 * core's codec gives the same frame. Its one diagnostic's `detail` is
 * `frame_bytes=<n>,maximum_frame_bytes=<m>`, and its `revision` is the revision the frame expected.
 */
export class MisoControlRefusal extends Error {
  readonly code: string;
  /** The reply's status name, such as `validation_failed`. */
  readonly status: string;
  readonly revision: bigint;
  /** The index, in the transaction that was sent, of the edit the first diagnostic names. */
  readonly editIndex: number | undefined;
  readonly diagnostics: readonly {
    readonly code: string;
    readonly detail: string | undefined;
    readonly operationIndex: number | undefined;
  }[];
  /** The refusal as a `ControlReport` (`ok: false`). */
  readonly report: {
    readonly ok: false;
    readonly revision: bigint;
    readonly code: string;
    readonly editIndex?: number;
  };

  constructor(init: {
    readonly code: string;
    readonly status: string;
    readonly revision: bigint;
    readonly editIndex: number | undefined;
    readonly diagnostics: MisoControlRefusal["diagnostics"];
    /** The message, when the SDK refused the frame before the core saw it (#2190). */
    readonly message?: string;
  }) {
    super(
      init.message
        ?? `the core refused the control transaction: ${init.code}`
          + (init.editIndex === undefined ? "" : ` at edit ${init.editIndex}`),
    );
    this.name = "MisoControlRefusal";
    this.code = init.code;
    this.status = init.status;
    this.revision = init.revision;
    this.editIndex = init.editIndex;
    this.diagnostics = Object.freeze([...init.diagnostics]);
    this.report = Object.freeze(
      init.editIndex === undefined
        ? { ok: false as const, revision: init.revision, code: init.code }
        : { ok: false as const, revision: init.revision, code: init.code, editIndex: init.editIndex },
    );
  }

  /** The core's code, as `MisoEngineError` and `MisoUsageError` spell it. */
  get diagnosticCode(): string {
    return this.code;
  }
}
