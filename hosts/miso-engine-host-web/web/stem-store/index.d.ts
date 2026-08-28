export type StemIdentity = `sha256:${string}`

export type StemProgress = {
  stage:
    | "loading"
    | "fetched"
    | "decoded"
    | "hashed"
    | "verified"
    | "verified-final"
    | "verified-open"
    | "verify-maintenance"
    | "verify-rebuild"
    | "corrupt"
    | "promoted"
    | "ready"
    | "interactive"
  identity?: StemIdentity
  bytes?: number
  totalBytes?: number
  stems?: number
  sessionId?: string
}

export type ResolvedStem = {
  stream: ReadableStream<Uint8Array>
  canonicalBytes?: number
}

/** Environment-neutral contract; SDK core re-exports this seam after #243. */
export interface StemResolver {
  resolve(
    identity: StemIdentity,
    options?: {
      signal?: AbortSignal
      onProgress?: (progress: StemProgress) => void
    }
  ): Promise<ResolvedStem>
}

export class StemResolverError extends Error {
  readonly code: string
  readonly details: Readonly<Record<string, unknown>>
}

export class MemoryStemResolver implements StemResolver {
  constructor(
    stems: Map<string, Uint8Array> | Record<string, Uint8Array>,
    options?: { chunkBytes?: number }
  )
  readonly requests: string[]
  resolve(
    identity: StemIdentity,
    options?: {
      signal?: AbortSignal
      onProgress?: (progress: StemProgress) => void
    }
  ): Promise<ResolvedStem & { canonicalBytes: number }>
}

export class FetchStemResolver implements StemResolver {
  constructor(options: {
    urlForIdentity: (identity: StemIdentity) => string | URL
    decode: (
      delivered: ReadableStream<Uint8Array>,
      context: {
        identity: StemIdentity
        signal?: AbortSignal
        onProgress?: (progress: StemProgress) => void
      }
    ) =>
      | ReadableStream<Uint8Array>
      | Promise<ReadableStream<Uint8Array>>
    fetcher?: typeof fetch
    readDeadlineMs?: number
    maximumResumeAttempts?: number
  })
  resolve(
    identity: StemIdentity,
    options?: {
      signal?: AbortSignal
      onProgress?: (progress: StemProgress) => void
    }
  ): Promise<ResolvedStem>
}

export type StemRequirement = {
  identity: StemIdentity
  /** channels × frames × bit_depth / 8, checked while full hashing. */
  bytes: number
}

export class StemStoreError extends Error {
  readonly code: string
  readonly details: Readonly<Record<string, unknown>>
}

export class OpfsStemStore {
  constructor(options?: {
    folderName?: string
    storage?: StorageManager
    locks?: LockManager
    now?: () => number
    tabId?: string
    readDeadlineMs?: number
  })
  readonly folderName: string
  open(): Promise<this>
  openSession(options: {
    sessionId: string
    stems: StemRequirement[]
    resolver: StemResolver
    signal?: AbortSignal
    onProgress?: (progress: StemProgress) => void
  }): Promise<StemSessionLease>
  read(identity: StemIdentity): Promise<Blob>
  verify(
    identity: StemIdentity,
    options?: {
      signal?: AbortSignal
      onProgress?: (progress: StemProgress) => void
    }
  ): Promise<boolean>
  setOfflinePin(
    identity: StemIdentity,
    pinId: string,
    pinned: boolean
  ): Promise<void>
}

export class StemSessionLease {
  readonly sessionId: string
  readonly stems: StemRequirement[]
  read(identity: StemIdentity): Promise<Blob>
  close(): Promise<void>
}

export class StemSessionGate {
  constructor(store: OpfsStemStore, resolver: StemResolver)
  readonly state: "idle" | "loading" | "interactive" | "refused"
  open(options: {
    sessionId: string
    stems: StemRequirement[]
    /** Invoked synchronously before open() first yields. */
    resume: () => unknown
    signal?: AbortSignal
    onProgress?: (progress: StemProgress) => void
  }): Promise<StemSessionLease>
  close(): Promise<void>
}

export class IncrementalSha256 {
  update(bytes: ArrayBuffer | ArrayBufferView): this
  digest(): Uint8Array
  digestHex(): string
}

export function sha256Stream(
  stream: ReadableStream<Uint8Array>,
  options?: {
    signal?: AbortSignal
    onChunk?: (bytes: number) => void
  }
): Promise<{ bytes: number; hex: string }>

export class Msb1RingWriter {
  constructor(shared: SharedArrayBuffer)
  readonly capacity: number
  readonly channels: number
  readonly frameCapacity: number
  readonly occupancy: number
  engage(generation: bigint): void
  reserve(frames: number): Float32Array[] | null
  commit(chunk: {
    generation: bigint
    startFrame: bigint
    frames: number
    endOfRegion: boolean
  }): void
  seek(generation: bigint, frame: bigint): void
  release(): void
}

export class CanonicalPcmPump {
  constructor(options: {
    lease: Pick<StemSessionLease, "read">
    sources: Array<{
      sourceId: string
      identity: StemIdentity
      channels: number
      bitDepth: 16 | 24
      frames: number
      ring: Msb1RingWriter
    }>
    windowFrames?: number
    generation?: bigint
    onError?: (error: unknown) => void
  })
  readonly finished: boolean
  pumpUntilFull(): Promise<{
    chunks: number
    frames: number
    finished: boolean
  }>
  seek(frame: number): Promise<void>
  stop(): void
}

export function deinterleaveCanonicalPcm(
  bytes: Uint8Array,
  firstFrame: number,
  frames: number,
  channels: number,
  bitDepth: 16 | 24,
  planes: Float32Array[]
): void

export function detectSharedReadOnlyMode(fileHandle: FileSystemFileHandle): Promise<boolean>

export function createStemPumpWorker(options?: {
  WorkerConstructor?: typeof Worker
  url?: URL
}): Worker

export const DEFAULT_STEM_STORE_FOLDER: "miso-stems-v1"
