/** Typed SDK-side refusal details. Runtime engine acknowledgements are added in Phase 2. */
export class MisoSessionError extends Error {
  readonly code = "miso.session.v1" as const;

  constructor(
    message: string,
    readonly path: string,
    readonly descriptor?: Readonly<Record<string, unknown>>,
  ) {
    super(message);
    this.name = "MisoSessionError";
  }
}

/** A malformed command is rejected before any host/engine submission occurs. */
export class MisoCommandError extends Error {
  readonly code = "miso.command.v1" as const;

  constructor(message: string, readonly path: string) {
    super(message);
    this.name = "MisoCommandError";
  }
}
