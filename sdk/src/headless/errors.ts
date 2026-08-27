/** One engine-emitted dotted diagnostic from the NUL-terminated Wasm buffer. */
export interface SessionDiagnostic {
  readonly code: string;
  readonly path: string;
}

/** A typed SDK failure at the offline asset, preparation, source, render, or output boundary. */
export class MisoOfflineError extends Error {
  readonly code = "miso.offline.v1" as const;

  constructor(
    message: string,
    readonly phase: "asset" | "prepare" | "compile" | "source" | "render" | "output" | "lifecycle",
    readonly result?: number,
    readonly diagnostics: readonly SessionDiagnostic[] = [],
  ) {
    super(message);
    this.name = "MisoOfflineError";
  }
}

/** Exact pre-instantiation asset-attestation refusal. */
export class MisoAssetHashError extends Error {
  readonly code = "miso.asset.hash-mismatch.v1" as const;

  constructor(
    readonly asset: string,
    readonly expectedSha256: string,
    readonly actualSha256: string,
  ) {
    super(`SHA-256 mismatch for ${asset}: expected ${expectedSha256}, got ${actualSha256}`);
    this.name = "MisoAssetHashError";
  }
}

/** A WAV or in-memory source disagrees with the prepared Session V1 declaration. */
export class MisoSourceError extends Error {
  readonly code = "miso.source.v1" as const;

  constructor(message: string, readonly sourceId: string, readonly path: string) {
    super(message);
    this.name = "MisoSourceError";
  }
}
