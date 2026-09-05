/** MSB1 shared PCM ring authority, adapted from engine-web-adapter@63b4ee6. */
export const MSB1_MAGIC = 0x4d534231;
export const MSB1_VERSION = 1;
export const MSB1_WRAP = 1 << 30;
export const MSB1_CONTROL_BYTES = 128;
export const MSB1_CONTROL_I64_OFFSET = 112;
export const MSB1_ID_OFFSET = 128;
export const MSB1_ID_CAPACITY = 128;
export const MSB1_HEADER_OFFSET = 256;
export const MSB1_SLOT_HEADER_BYTES = 32;
export const MSB1_FLAG_END_OF_REGION = 1;
export const MSB1_CONTROL = Object.freeze({ MAGIC: 0, VERSION: 1, CAPACITY: 2, CHANNELS: 3, FRAME_CAPACITY: 4, HEADER_OFFSET: 5, PCM_OFFSET: 6, ID_LENGTH: 7, WRITE_INDEX: 8, READ_INDEX: 9, GENERATION_TAG: 10, SEEK_EPOCH: 11, WRITER_STATE: 12, ATTACHED: 13, WROTE: 14, OVERFLOW: 15, SUBMITTED: 16, STALE: 17, REFUSED: 18, LAST_RESULT: 19, UNDERRUNS: 20, DRAIN_BLOCKS: 21, SEEKS_APPLIED: 22, DEPTH: 23, TORN: 24, FINISHED: 25, ERRORS: 26, SUBMITTED_GENERATION_TAG: 27 } as const);
const SLOT = { SEQUENCE: 0, GENERATION_TAG: 1, FRAMES: 2, FLAGS: 3 } as const;
const SLOT_I64 = { GENERATION: 2, START_FRAME: 3 } as const;
export interface Msb1RingLayout { readonly sourceId: string; readonly channels: number; readonly frameCapacity: number; readonly capacity: number }
export interface Msb1RingCounters { readonly wrote:number; readonly overflow:number; readonly submitted:number; readonly stale:number; readonly refused:number; readonly lastResult:number; readonly seeksApplied:number; readonly torn:number; readonly errors:number; readonly occupancy:number; readonly generationTag:number; readonly submittedGenerationTag:number }
export function msb1RingBytes(channels:number, frameCapacity:number, capacity:number):number { return MSB1_HEADER_OFFSET + capacity * MSB1_SLOT_HEADER_BYTES + capacity * channels * frameCapacity * 4; }
export function createMsb1Ring(layout: Msb1RingLayout): SharedArrayBuffer {
  if (!power(layout.capacity)) throw new RangeError("MSB1 capacity must be a power of two");
  if (!positive(layout.channels) || !positive(layout.frameCapacity)) throw new RangeError("MSB1 shape must be positive");
  const id = new TextEncoder().encode(layout.sourceId); if (id.byteLength === 0 || id.byteLength > MSB1_ID_CAPACITY) throw new RangeError("sourceId does not fit MSB1");
  const pcm = MSB1_HEADER_OFFSET + layout.capacity * MSB1_SLOT_HEADER_BYTES; const shared = new SharedArrayBuffer(msb1RingBytes(layout.channels, layout.frameCapacity, layout.capacity)); const c = new Int32Array(shared, 0, 32);
  c[MSB1_CONTROL.VERSION]=1; c[MSB1_CONTROL.CAPACITY]=layout.capacity; c[MSB1_CONTROL.CHANNELS]=layout.channels; c[MSB1_CONTROL.FRAME_CAPACITY]=layout.frameCapacity; c[MSB1_CONTROL.HEADER_OFFSET]=MSB1_HEADER_OFFSET; c[MSB1_CONTROL.PCM_OFFSET]=pcm; c[MSB1_CONTROL.ID_LENGTH]=id.byteLength; c[MSB1_CONTROL.GENERATION_TAG]=1; new Uint8Array(shared, MSB1_ID_OFFSET, id.byteLength).set(id); Atomics.store(c, MSB1_CONTROL.MAGIC, MSB1_MAGIC); return shared;
}
export class Msb1RingWriter {
 readonly capacity:number; readonly channels:number; readonly frameCapacity:number; readonly #v:ReturnType<typeof bind>; #reserved=-1;
 constructor(shared:SharedArrayBuffer){this.#v=bind(shared); this.capacity=this.#v.capacity; this.channels=this.#v.channels; this.frameCapacity=this.#v.frameCapacity;}
 get occupancy(){return occupancy(this.#v.control)}
 engage(generation:bigint){Atomics.store(this.#v.control,MSB1_CONTROL.GENERATION_TAG,Number(BigInt.asIntN(32,generation))); Atomics.store(this.#v.control,MSB1_CONTROL.WRITER_STATE,1)}
 reserve(frames:number):readonly Float32Array[]|null{if(!positive(frames)||frames>this.frameCapacity)throw new RangeError("PCM chunk does not fit MSB1"); if(this.occupancy>=this.capacity){Atomics.add(this.#v.control,MSB1_CONTROL.OVERFLOW,1);return null} const i=Atomics.load(this.#v.control,MSB1_CONTROL.WRITE_INDEX);this.#reserved=i;const p=this.#v.planes[i&(this.capacity-1)]!;for(const x of p)x.fill(0);return p}
 commit(chunk:{readonly generation:bigint;readonly startFrame:bigint;readonly frames:number;readonly endOfRegion:boolean}){if(this.#reserved<0)throw new Error("MSB1 commit without reservation");const i=this.#reserved;this.#reserved=-1;const s=i&(this.capacity-1),w=s*8,w64=s*4;this.#v.headers[w+SLOT.SEQUENCE]=i;this.#v.headers[w+SLOT.GENERATION_TAG]=Number(BigInt.asIntN(32,chunk.generation));this.#v.headers[w+SLOT.FRAMES]=chunk.frames;this.#v.headers[w+SLOT.FLAGS]=chunk.endOfRegion?1:0;this.#v.headersI64[w64+SLOT_I64.GENERATION]=chunk.generation;this.#v.headersI64[w64+SLOT_I64.START_FRAME]=chunk.startFrame;Atomics.add(this.#v.control,MSB1_CONTROL.WROTE,1);Atomics.store(this.#v.control,MSB1_CONTROL.WRITE_INDEX,(i+1)&(MSB1_WRAP-1))}
 seek(generation:bigint,frame:bigint){this.#v.controlI64[0]=generation;this.#v.controlI64[1]=frame;Atomics.store(this.#v.control,MSB1_CONTROL.GENERATION_TAG,Number(BigInt.asIntN(32,generation)));Atomics.add(this.#v.control,MSB1_CONTROL.SEEK_EPOCH,1)}
 release(){Atomics.store(this.#v.control,MSB1_CONTROL.WRITER_STATE,0)}
}
function bind(shared:SharedArrayBuffer){if(!(shared instanceof SharedArrayBuffer))throw new TypeError("MSB1 requires SharedArrayBuffer");const control=new Int32Array(shared,0,32);if(Atomics.load(control,0)!==MSB1_MAGIC||control[1]!==1)throw new TypeError("Shared buffer is not MSB1");const capacity=control[2]!,channels=control[3]!,frameCapacity=control[4]!,headerOffset=control[5]!,pcmOffset=control[6]!;if(!power(capacity)||!positive(channels)||!positive(frameCapacity))throw new TypeError("MSB1 header is invalid");const headers=new Int32Array(shared,headerOffset,capacity*8),planes=Array.from({length:capacity},(_,s)=>Array.from({length:channels},(_,ch)=>new Float32Array(shared,pcmOffset+(s*channels+ch)*frameCapacity*4,frameCapacity)));return {control,controlI64:new BigInt64Array(shared,MSB1_CONTROL_I64_OFFSET,2),headers,headersI64:new BigInt64Array(shared,headerOffset,capacity*4),planes,capacity,channels,frameCapacity}}
 function occupancy(c:Int32Array){return (Atomics.load(c,8)-Atomics.load(c,9))&(MSB1_WRAP-1)} function positive(v:number){return Number.isSafeInteger(v)&&v>0} function power(v:number){return positive(v)&&(v&(v-1))===0}
