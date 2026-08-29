export declare const MISO_ENGINE_FLAC_DECODER_SHA256: string;

export declare class MisoFlacDecoderError extends Error {
  readonly code: string;
  readonly result: number | null;
}

export interface FlacStreamInfo {
  readonly sampleRateHz: number;
  readonly channels: number;
  readonly bitDepth: 16 | 24;
  readonly frames: bigint;
}

export interface DecodedFlacBlock {
  readonly stream: FlacStreamInfo;
  readonly pcm: Uint8Array;
}

export interface PinnedFlacDecoder {
  decodeBlocks(
    flacBytes: Uint8Array | ArrayBuffer,
    options: { readonly maximumCanonicalBytes: number },
  ): AsyncGenerator<DecodedFlacBlock, void, void>;
}

export declare function instantiatePinnedFlacDecoder(
  bytes: Uint8Array | ArrayBuffer,
): Promise<PinnedFlacDecoder>;

export declare function loadPinnedFlacDecoder(url: string): Promise<PinnedFlacDecoder>;

export declare function canonicalPcmBlockToPlanarF32(
  pcmBytes: Uint8Array | ArrayBuffer,
  channels: number,
  bitDepth: 16 | 24,
): Float32Array[];
