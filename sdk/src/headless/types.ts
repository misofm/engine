import type { CommandRecord } from "../core/command.js";
import type { CommandAck, EffectDecl, EffectParamValues, Indices, Rack, TapName } from "../core/types.js";
import type { SessionShape } from "../core/session.js";
import type { EngineResources, EngineStatus, SessionDiagnostics, WasmAssetOptions } from "./abi.js";

export type OfflineSource = readonly Float32Array[] | Readonly<{ wav: Uint8Array | string }>;

export interface OfflineEngineOptions<S extends SessionShape> {
  readonly session: import("../core/session.js").SessionPlan<S> | Readonly<{ toml: string }>;
  readonly sources: Readonly<Record<string, OfflineSource>>;
  readonly limits?: Partial<import("../core/session.js").PrepareLimits>;
  readonly wasm?: WasmAssetOptions;
}

export interface RenderedAudio {
  readonly left: Float32Array;
  readonly right: Float32Array;
}

export interface RenderReport {
  readonly path: string;
  readonly format: "f32le-planar" | "wav32f";
  readonly frames: number;
  readonly bytes: number;
  readonly sha256: string;
}

export interface MeterFrame {
  readonly sequence: bigint;
  readonly windows: number;
  readonly trackCount: number;
  readonly peaks: Float32Array;
  readonly trackGrDb: Float32Array;
  readonly masterGrDb: number | null;
  readonly firstSample: bigint;
  readonly endSample: bigint;
}

export interface ParamAddress {
  readonly trackId: string;
  readonly rack: Rack;
  readonly effectIndex: number;
  readonly parameter: string | number;
  readonly channel?: "left" | "right" | "both";
  readonly smoothingSamples?: number;
}

export interface ObservationAddress {
  readonly trackId: string;
  readonly rack: Rack;
  readonly effectIndex: number;
  readonly tap: string | number;
  readonly windowBlocks?: number;
}

export interface ObservationHandle {
  readonly ack: CommandAck;
  close(): Promise<CommandAck>;
}

export interface CommandBatch {
  add(record: CommandRecord): CommandBatch;
  submit(): Promise<CommandAck>;
}

export interface OfflineEffectConsole<D extends EffectDecl = EffectDecl> {
  readonly declaration: D;
  readonly slotId: string;
  set(params: EffectParamValues<D["effectId"]>): Promise<CommandAck>;
  bypass(on: boolean): Promise<CommandAck>;
  observe(tap: TapName<D["effectId"]>, options?: Readonly<{ windowBlocks?: number }>): Promise<ObservationHandle>;
}

export interface OfflineTrackConsole<E extends readonly EffectDecl[] = readonly EffectDecl[]> {
  fader(db: number, options?: Readonly<{ channel?: "left" | "right" | "both"; smoothingSamples?: number }>): Promise<CommandAck>;
  mute(on: boolean, options?: Readonly<{ channel?: "left" | "right" | "both"; smoothingSamples?: number }>): Promise<CommandAck>;
  pan(left: number, right: number, options?: Readonly<{ smoothingSamples?: number }>): Promise<CommandAck>;
  effect<I extends Indices<E> & keyof E>(index: I): OfflineEffectConsole<Extract<E[I], EffectDecl>>;
}

export interface EngineConsole<S extends SessionShape> {
  sessionMap(): Promise<Readonly<{ tracks: readonly string[] }>>;
  track<Id extends keyof S & string>(id: Id): OfflineTrackConsole<S[Id]>;
  batch(): CommandBatch;
  submit(records: readonly CommandRecord[]): Promise<CommandAck>;
  setParam(address: ParamAddress, value: number): Promise<CommandAck>;
  observe(address: ObservationAddress): Promise<ObservationHandle>;
}

export interface OfflineEngine<S extends SessionShape> {
  readonly console: EngineConsole<S>;
  readonly resources: EngineResources;
  render(frames: number): RenderedAudio;
  renderAll(): RenderedAudio;
  renderToFile(path: string, options?: Readonly<{ format?: "f32le-planar" | "wav32f" }>): Promise<RenderReport>;
  pollMeters(): MeterFrame | null;
  status(): EngineStatus;
  dispose(): void;
}

export type { EngineResources, EngineStatus, SessionDiagnostics, WasmAssetOptions };
