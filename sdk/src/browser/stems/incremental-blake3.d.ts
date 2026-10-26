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

/**
 * Load the WebAssembly SIMD BLAKE3 (issue #2111) so that every later `IncrementalBlake3` and
 * `blake3Stream` hashes in it. The default URL is the package's
 * `dist/assets/miso-engine-v1-blake3.simd128.wasm`; pass `url` when a bundler moves it.
 * Idempotent: one load per URL per realm. Resolves `"simd128"` when the module loaded, and
 * `"javascript"` when the fetch, the compile (no WebAssembly SIMD) or the instantiation failed; it
 * never rejects, and hashing then stays in JavaScript with the same digests. `blake3Stream` awaits
 * it itself; call it before `new IncrementalBlake3()` to hash in the module.
 */
export function prepareBlake3(options?: { readonly url?: string | URL }): Promise<"simd128" | "javascript">;
