/**
 * Hand-written declaration of the engine's canonical PCM pump (issue #2102), cut from
 * `hosts/host-web/web/stem-store/index.d.ts`. The package stages the engine's `pcm-pump.js`
 * verbatim beside it; only the names below are public.
 */

/** A stem's canonical identity: `blake3:` and 64 lowercase hex digits (docs/STEM_IDENTITY_V1.md). */
export type CanonicalStemIdentity = `blake3:${string}`;

/**
 * The ring a pump writes. The SDK's `Msb1RingWriter` is one; the pump engages it, writes planar
 * chunks, publishes plain seeks and releases it.
 */
export interface CanonicalPcmRing {
  readonly capacity: number;
  readonly channels: number;
  readonly frameCapacity: number;
  readonly occupancy: number;
  engage(generation: bigint): void;
  reserve(frames: number): readonly Float32Array[] | null;
  commit(chunk: {
    readonly generation: bigint;
    readonly startFrame: bigint;
    readonly frames: number;
    readonly endOfRegion: boolean;
  }): void;
  seek(generation: bigint, frame: bigint): void;
  release(): void;
}

/** Where a pump reads each stem's canonical PCM: one Blob per identity, read lazily by window. */
export interface CanonicalPcmLease {
  read(identity: CanonicalStemIdentity): Promise<Blob>;
}

/** One stem the pump feeds: its canonical PCM shape and the ring of its session source. */
export interface CanonicalPcmPumpSource {
  readonly sourceId: string;
  readonly identity: CanonicalStemIdentity;
  readonly channels: number;
  readonly bitDepth: 16 | 24;
  readonly frames: number;
  readonly ring: CanonicalPcmRing;
}

export interface CanonicalPcmPumpOptions {
  readonly lease: CanonicalPcmLease;
  readonly sources: readonly CanonicalPcmPumpSource[];
  /** Read-ahead window per source in frames; at least each ring's frame capacity. Default 4096. */
  readonly windowFrames?: number;
  /** The generation the pump engages its rings under. Default `1n`. */
  readonly generation?: bigint;
  /** Called once with the error that stopped the pump. */
  readonly onError?: (error: unknown) => void;
}

/** A typed pump refusal: `code` is `stem.pump.ring_shape` when a ring's shape is not its source's. */
export class CanonicalPcmPumpError extends Error {
  readonly code: string;
  readonly details: Readonly<Record<string, unknown>>;
}

/**
 * Bounded-window pump from canonical PCM Blobs into MSB1 rings. At most two windows per source are
 * resident (the one being written and the next, read ahead), and at most eight window reads are in
 * flight across all sources. A seek during a read never commits the pre-seek window. The pump never
 * fetches or decodes.
 */
export class CanonicalPcmPump {
  constructor(options: CanonicalPcmPumpOptions);
  /** Every source reached the end of its region. */
  readonly finished: boolean;
  /** Fill every ring until it is full or its source ends. */
  pumpUntilFull(): Promise<{ chunks: number; frames: number; finished: boolean }>;
  /**
   * Reposition every stem to `frame`, clamped to its region, under a new generation: `generation`
   * when given (it must be above the pump's own; pass a session seek's only after the engine
   * accepted it), else the pump's next. It is a `SessionSeekProducer`.
   */
  seek(frame: number, generation?: bigint): Promise<void>;
  /** Release every ring. */
  stop(): void;
}

/**
 * Canonical interleaved little-endian integer PCM to planar `f32`: a 16-bit sample is `n / 32768`,
 * a 24-bit sample is `n / 8388608`. Reads `frames` frames from frame `firstFrame` of `bytes`.
 */
export function deinterleaveCanonicalPcm(
  bytes: Uint8Array,
  firstFrame: number,
  frames: number,
  channels: number,
  bitDepth: 16 | 24,
  planes: readonly Float32Array[],
): void;
