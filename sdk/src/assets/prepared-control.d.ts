export interface PreparedAddress {
  readonly trackIndex: number;
  readonly rack: number;
  readonly effectIndex: number;
}

export interface PreparedConfigReply {
  readonly result: number;
  readonly reason: number;
  readonly config: Uint8Array;
}

export interface PreparedAck {
  readonly result: number;
  readonly reason: number;
  readonly rejectedIndex: number;
  readonly admitted: number;
  readonly appliedAtSample: bigint;
  readonly records: Uint8Array;
  readonly ok?: boolean;
  readonly code?: string;
  readonly reasonName?: string;
}

export interface PreparedControl {
  readonly submitSync: (records: Uint8Array, count: number) => PreparedAck;
  readonly close: () => void;
  readonly invalidate: () => void;
}

export interface PreparedControlOptions {
  readonly instance: unknown;
  readonly abiLayout: unknown;
  readonly sampleRateHz: number;
  readonly configCopySync: (address: PreparedAddress) => PreparedConfigReply;
  readonly ordinarySubmitSync: (records: Uint8Array, count: number) => PreparedAck;
  readonly preparedSubmitSync: (records: Uint8Array, companion: Uint8Array, count: number) => PreparedAck;
  readonly preparedRefusalSync: (
    records: Uint8Array,
    count: number,
    reason: number,
    rejectedIndex: number | undefined,
    result: number,
  ) => PreparedAck;
}

export function createPreparedControl(options: PreparedControlOptions): PreparedControl;
