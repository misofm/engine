import { ABI_LAYOUT } from "../generated/abi.ts";
import type { CommandReport } from "./boundary.ts";
import { MisoUsageError } from "./errors.ts";

/**
 * The live-console writer (issue #243, adopted ruling 5462139867 finding 5).
 *
 * # What the writer is for
 *
 * A hand on a fader produces edits far faster than a render thread drains them. The engine's
 * per-track control queue is bounded -- 64 records by default -- and a submission that does not fit
 * is refused **whole**, with `reason: backpressure`. That refusal is the engine working correctly:
 * it is flow control, not failure. The bug it invites is in the caller, and it has two shapes.
 *
 * The first is treating the refusal as an error. A drag that refuses one batch and tears down the
 * session has turned a full queue into a broken mix. So this writer's contract is: **a flow-control
 * refusal is never an error and never terminal**. Nothing here throws on one, no failure callback
 * fires, and the writer is as usable after a refusal as before it.
 *
 * The second is retrying the backlog. A refused batch that is simply resent replays a queue's worth
 * of *stale* intermediate values, so a drag that was refused mid-gesture lands somewhere the hand
 * never was, and the final position arrives last or not at all. So the writer re-stages
 * **latest-wins coalesced**: pending edits are keyed by what they address, and a newer edit
 * replaces an older pending one rather than queueing behind it. After the queue drains, what lands
 * is where the hand actually is.
 *
 * # Why the split is discovered rather than computed
 *
 * A batch too large for the queue must be broken up. The writer could compute the split if it knew
 * how many queue slots each record costs -- and it is not one: a `channel = both` record lowers to
 * *two* lane records, which is why 32 both-lane parameters fill a 64-record queue exactly while 64
 * single-lane ones do. But that ratio is the engine's lowering rule, and a copy of it here would be
 * one more private constant to drift.
 *
 * So the writer discovers the boundary instead: a refusal halves the next attempt's size, and a
 * success grows it back toward the caller's ceiling. The engine's own answer drives the split,
 * which means the writer stays correct if the lowering rule ever changes.
 */

/** One addressed console edit, in the wire's own vocabulary. */
export interface LaneEdit {
  /** A `wireCommandKinds` name, e.g. `effectParam`, `faderDb`, `mute`. */
  readonly kind: string;
  readonly trackIndex: number;
  /** `0` simd1, `1` dynamic, `2` simd2, `255` not applicable. */
  readonly rack: number;
  /** `0` left, `1` right, `2` both, `255` not applicable. */
  readonly channel: number;
  readonly effectIndex?: number;
  readonly parameterId?: number;
  readonly smoothingSamples?: number;
  readonly values: readonly [number, number, number, number];
}

export interface FlushOutcome {
  /** Records the engine admitted. A refusal admits nothing: the batch is one transaction. */
  readonly admitted: number;
  /** True when the engine refused for flow control. Never an error. */
  readonly refused: boolean;
  /** The engine's reason name, present only on a refusal. */
  readonly reason: string | undefined;
  /** Edits still waiting after this flush. */
  readonly pending: number;
  /** The batch size the next flush will attempt. */
  readonly nextBatch: number;
}

export interface WriterStats {
  readonly flushes: number;
  readonly admitted: number;
  /** Flow-control refusals. Expected under load; never an error. */
  readonly refusals: number;
  /**
   * Refusals that were NOT flow control.
   *
   * This is the number that must stay zero. A malformed record or an unknown address is a caller
   * bug and is surfaced by throwing, because unlike backpressure it will never succeed on retry.
   */
  readonly escalations: number;
  /** Edits dropped by coalescing -- a newer value for the same address superseding an older one. */
  readonly coalesced: number;
}

export interface WriterOptions {
  /**
   * Submit one staged batch. Normally `engine.submitCommands` bound to a live engine.
   *
   * May answer asynchronously. In-process the engine answers immediately, but the browser host
   * reaches it over a worklet port, where the answer is a `Promise` by construction; the writer's
   * contract is identical either way, so the signature admits both rather than forcing a caller
   * across that boundary to fake a synchronous report.
   */
  readonly submit: (records: Uint8Array, count: number) => CommandReport | Promise<CommandReport>;
  /**
   * The largest batch to attempt. Defaults to the engine's default queue depth, which is the
   * largest a single-lane batch can be from idle.
   */
  readonly maximumBatch?: number;
}

const BACKPRESSURE = ABI_LAYOUT.constants.commandReasons
  .find((row) => row.name === "backpressure")!.value;
const RECORD_BYTES = ABI_LAYOUT.commandRecord.bytes;
const FIELD: ReadonlyMap<string, number> = new Map(
  ABI_LAYOUT.commandRecord.fields.map((row) => [row.name as string, row.offset] as const),
);

function offsetOf(name: string): number {
  const offset = FIELD.get(name);
  if (offset === undefined) {
    throw new MisoUsageError(`the generated command record has no field ${name}`);
  }
  return offset;
}

function kindValue(name: string): number {
  const row = ABI_LAYOUT.constants.wireCommandKinds.find((candidate) => candidate.name === name);
  if (row === undefined) {
    throw new MisoUsageError(`the generated ABI layout has no command kind ${name}`);
  }
  return row.value;
}

/**
 * The address an edit coalesces on.
 *
 * Two edits collapse when they would write the same thing: same kind, same track, same rack, same
 * effect slot, same parameter, same lane. `smoothingSamples` and the values are deliberately NOT
 * part of the key -- they are what the newer edit replaces.
 */
function coalescingKey(edit: LaneEdit): string {
  return [
    edit.kind, edit.trackIndex, edit.rack, edit.channel,
    edit.effectIndex ?? 0, edit.parameterId ?? 0,
  ].join("/");
}

function encode(edits: readonly LaneEdit[]): Uint8Array {
  const records = new Uint8Array(edits.length * RECORD_BYTES);
  const view = new DataView(records.buffer);
  edits.forEach((edit, index) => {
    const base = index * RECORD_BYTES;
    view.setUint8(base + offsetOf("kind"), kindValue(edit.kind));
    view.setUint8(base + offsetOf("rack"), edit.rack);
    view.setUint8(base + offsetOf("channel"), edit.channel);
    view.setUint32(base + offsetOf("trackIndex"), edit.trackIndex, true);
    view.setUint32(base + offsetOf("effectIndex"), edit.effectIndex ?? 0, true);
    view.setUint32(base + offsetOf("parameterId"), edit.parameterId ?? 0, true);
    view.setUint32(base + offsetOf("smoothingSamples"), edit.smoothingSamples ?? 0, true);
    const values = offsetOf("values");
    edit.values.forEach((value, slot) => {
      view.setFloat32(base + values + slot * 4, value, true);
    });
  });
  return records;
}

export class ConsoleWriter {
  readonly #submit: WriterOptions["submit"];
  readonly #maximumBatch: number;
  /** Insertion-ordered by key, which is what makes coalescing a map update rather than a scan. */
  readonly #pending = new Map<string, LaneEdit>();
  #batch: number;
  /**
   * The tail of the flush chain, which is what serializes flushes.
   *
   * A flush picks its batch out of the pending map and only applies the result once the submit has
   * answered. When the answer is a promise, a second flush entered before the first resolves would
   * pick the SAME keys -- a torn batch: the same edits submitted twice, admitted twice in the
   * stats, and still pending afterwards. So each call appends its work here instead of starting
   * against the writer's state directly, and the batch is chosen after the previous flush applied.
   *
   * The chain orders flushes; it never merges them. One call is still one attempt, so `flushes`
   * counts what a synchronous writer would have counted.
   */
  #tail: Promise<unknown> = Promise.resolve();
  #flushes = 0;
  #admitted = 0;
  #refusals = 0;
  #escalations = 0;
  #coalesced = 0;

  constructor(options: WriterOptions) {
    this.#submit = options.submit;
    this.#maximumBatch = options.maximumBatch ?? ABI_LAYOUT.constants.defaultCommandQueueRecords;
    if (!Number.isInteger(this.#maximumBatch) || this.#maximumBatch < 1) {
      throw new MisoUsageError(`maximumBatch must be a positive integer`);
    }
    this.#batch = this.#maximumBatch;
  }

  /**
   * Stage one edit, superseding any pending edit for the same address.
   *
   * This is where a drag stops being a backlog: a hundred intermediate positions of one fader
   * collapse to one pending record, and the one that survives is the latest.
   */
  stage(edit: LaneEdit): void {
    const key = coalescingKey(edit);
    if (this.#pending.has(key)) this.#coalesced += 1;
    // Delete before set so a superseded edit also moves to the back of the insertion order: the
    // most recently touched control is the one most likely to still be moving.
    this.#pending.delete(key);
    this.#pending.set(key, edit);
  }

  get pending(): number {
    return this.#pending.size;
  }

  get stats(): WriterStats {
    return Object.freeze({
      flushes: this.#flushes,
      admitted: this.#admitted,
      refusals: this.#refusals,
      escalations: this.#escalations,
      coalesced: this.#coalesced,
    });
  }

  /**
   * Attempt one batch.
   *
   * On a flow-control refusal nothing is admitted, so nothing is removed from the pending map --
   * the edits stay coalescable, and any newer value staged before the retry supersedes the one
   * that was refused. That is what makes the retry land the hand's final position rather than a
   * queue's worth of stale intermediates.
   *
   * A refusal that is *not* flow control throws. Backpressure will succeed on retry once the
   * render thread drains; a malformed record or an unknown address never will, so retrying it
   * silently would be an infinite loop wearing the costume of resilience.
   *
   * Flushes are serialized: a call entered while a prior submit is still outstanding waits for it
   * rather than picking its batch out of a map the earlier flush has not yet applied to. See
   * `#tail`.
   */
  flush(): Promise<FlushOutcome> {
    const attempt = this.#tail.then(() => this.#flushOnce());
    // The chain absorbs the rejection an escalation throws to ITS caller, so a later flush is
    // ordered behind that attempt rather than inheriting its failure.
    this.#tail = attempt.catch(() => undefined);
    return attempt;
  }

  async #flushOnce(): Promise<FlushOutcome> {
    if (this.#pending.size === 0) {
      return Object.freeze({
        admitted: 0,
        refused: false,
        reason: undefined,
        pending: 0,
        nextBatch: this.#batch,
      });
    }
    const take = Math.min(this.#batch, this.#pending.size);
    const keys = [...this.#pending.keys()].slice(0, take);
    const edits = keys.map((key) => this.#pending.get(key)!);

    this.#flushes += 1;
    const report = await this.#submit(encode(edits), edits.length);

    if (report.ok) {
      for (const key of keys) this.#pending.delete(key);
      this.#admitted += report.admitted;
      // Grow back toward the caller's ceiling, so a transient full queue does not permanently
      // shrink throughput.
      this.#batch = Math.min(this.#maximumBatch, Math.max(1, this.#batch * 2));
      return Object.freeze({
        admitted: report.admitted,
        refused: false,
        reason: undefined,
        pending: this.#pending.size,
        nextBatch: this.#batch,
      });
    }

    if (report.reason !== BACKPRESSURE) {
      this.#escalations += 1;
      throw new MisoUsageError(
        `the console refused a batch for ${report.reasonName} at record ${report.rejectedIndex}; `
        + "this is a caller error, not flow control, and will not succeed on retry",
      );
    }

    this.#refusals += 1;
    // Halve, so the next attempt discovers the boundary rather than assuming one. A batch of one
    // that still refuses simply waits for the render thread; it never shrinks below one.
    this.#batch = Math.max(1, Math.floor(take / 2));
    return Object.freeze({
      admitted: 0,
      refused: true,
      reason: report.reasonName,
      pending: this.#pending.size,
      nextBatch: this.#batch,
    });
  }

  /**
   * Flush repeatedly until nothing is pending or no progress is made.
   *
   * Bounded by `attempts` rather than looping until success, because "the queue is full and
   * nothing is rendering" is a legitimate steady state -- a paused transport -- and a writer that
   * spun there would be a busy loop rather than a retry.
   */
  async drain(attempts = 8): Promise<FlushOutcome> {
    let outcome = await this.flush();
    for (let attempt = 1; attempt < attempts && this.#pending.size > 0; attempt += 1) {
      const next = await this.flush();
      if (next.admitted === 0 && next.refused) return next;
      outcome = next;
    }
    return outcome;
  }
}
