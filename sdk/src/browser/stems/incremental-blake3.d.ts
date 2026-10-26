/**
 * Hand-written declaration of the engine's incremental BLAKE3-256 (issue #2102), cut from
 * `hosts/host-web/web/stem-store/index.d.ts`. The package stages the engine's
 * `incremental-blake3.js` and its provenance record verbatim beside it.
 */

/** Bounded-memory incremental BLAKE3-256 (32-byte default output). One digest per instance. */
export class IncrementalBlake3 {
  update(bytes: ArrayBuffer | ArrayBufferView): this;
  digest(): Uint8Array;
  /** The digest as 64 lowercase hex digits. */
  digestHex(): string;
}

/** Hash a byte stream without retaining it; `hex` is 64 lowercase hex digits. */
export function blake3Stream(
  stream: ReadableStream<Uint8Array>,
  options?: {
    readonly signal?: AbortSignal;
    readonly onChunk?: (bytes: number) => void;
  },
): Promise<{ bytes: number; hex: string }>;
