import { ABI_LAYOUT } from "../generated/abi.ts";
import type { CommandReasonName } from "../generated/catalog.ts";
import type { CommandReport } from "./boundary.ts";
import {
  CONTROL_TRANSACTION_HEADER_BYTES,
  controlEditBytes,
  controlEditKey,
  validateControlEdit,
} from "./control-frame.ts";
import type { ControlFrameEdit } from "./control-frame.ts";
import type { ControlAppliedReport } from "./control-queue.ts";
import { MisoControlRefusal, MisoEngineError, MisoUsageError } from "./errors.ts";
import { requireU32 } from "../internal/ramp.ts";

/**
 * The live-control writer (issue #243, adopted ruling 5462139867 finding 5).
 *
 * # What the writer is for
 *
 * A hand on a fader produces edits far faster than a render thread drains them. A value edit is a
 * `SET_CONTROL` edit (`ControlFrameEdit`), staged by its address and sent through the engine's one
 * transaction queue (`ControlTransactionQueue`); the core holds every live value in a latest-target
 * cell, which takes any number of writes and refuses none for room, and answers a full control queue
 * with a `backpressure` status. The other live record that rides a bounded queue is an effect's
 * observation subscription (a {@link LaneEdit}, kinds 7 and 8, still a 48-byte record), with one slot
 * per tap the effect declares. In both a submission that does not fit is refused **whole**, with
 * `reason: backpressure`. That refusal is the engine working correctly: it is flow control, not
 * failure. The writer stays correct for any submission the engine may refuse that way, and the bug
 * it guards against is in the caller, and it has two shapes.
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
 * A batch the engine refuses for room must be made smaller. The writer could compute the split if
 * it knew how much room each record needs -- and it is not one: that cost is the engine's
 * admission rule, and a copy of it here would be one more private constant to drift.
 *
 * So the writer discovers the boundary instead: a refusal halves the next attempt's size, and a
 * success grows it back toward the caller's ceiling. The engine's own answer drives the split,
 * which means the writer stays correct if the admission rule ever changes.
 */

/**
 * One observation edit, in the wire's own vocabulary: the subscription or withdrawal of one tap
 * (kinds 7 and 8), the only 48-byte command records left. Every value edit is a
 * {@link ControlFrameEdit} instead.
 */
export interface LaneEdit {
  readonly kind: "observeSubscribe" | "observeUnsubscribe";
  /**
   * The record's rack byte: `1` inserts, `3` console (S1c). The retired `0` and `2` are refused by
   * the engine with `unknownRack`.
   */
  readonly rack: number;
  /** The strip's ID, a track's or a submix's. */
  readonly stripId: string;
  /** The console slot's ID or the insert's ID. */
  readonly effectId: string;
  /** The tap's ID. */
  readonly tapId: number;
  /** The subscription window in blocks, `0` for the default. Any `u32`. */
  readonly windowBlocks: number;
}

export interface FlushOutcome {
  /** Records the engine admitted. A refusal admits nothing: the batch is one transaction. */
  readonly admitted: number;
  /** True when the engine refused for flow control. Never an error. */
  readonly refused: boolean;
  /** The engine's reason name, present only on a refusal. */
  readonly reason: CommandReasonName | undefined;
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

/** What a writer needs of the transaction queue: one atomic submit. */
export interface ControlSubmitter {
  submit(edits: readonly ControlFrameEdit[]): Promise<ControlAppliedReport>;
  /** The host's frame ceiling in bytes. A batch is cut by bytes before it passes it. */
  readonly maximumFrameBytes?: number | undefined;
  /** The host's ceiling on the edits of one transaction. */
  readonly maximumTransactionEdits?: number | undefined;
}

interface WriterLimits {
  /**
   * The largest batch to attempt. Defaults to the engine's maximum command-record count
   * (`maximumCommandRecords`), the host's own bound on commands in flight.
   */
  readonly maximumBatch?: number;
}

/** Submit value edits as `SET_CONTROL` transactions through the engine's queue. */
export interface ControlWriterOptions extends WriterLimits {
  readonly queue: ControlSubmitter;
  readonly submit?: never;
  readonly submitEdits?: never;
}

/** Submit observation records (kinds 7 and 8) as one command batch. */
export interface WriterOptions extends WriterLimits {
  readonly queue?: never;
  /**
   * Submit one staged batch. Normally `engine.submitCommands` bound to a live engine.
   *
   * May answer asynchronously. In-process the engine answers immediately, but the browser host
   * reaches it over a worklet port, where the answer is a `Promise` by construction; the writer's
   * contract is identical either way, so the signature admits both rather than forcing a caller
   * across that boundary to fake a synchronous report.
   */
  readonly submit: (records: Uint8Array, count: number) => CommandReport | Promise<CommandReport>;
  readonly submitEdits?: never;
}

/** Submit observation edits directly through a semantic live-control transport. */
export interface SemanticWriterOptions extends WriterLimits {
  readonly queue?: never;
  readonly submit?: never;
  readonly submitEdits: (edits: readonly LaneEdit[]) => CommandReport | Promise<CommandReport>;
}

const BACKPRESSURE = ABI_LAYOUT.constants.commandReasons
  .find((row) => row.name === "backpressure")!.value;
const BACKPRESSURE_NAME: CommandReasonName = "backpressure";
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

function kindValue(name: LaneEdit["kind"]): number {
  const row = ABI_LAYOUT.constants.wireCommandKinds.find((candidate) => candidate.name === name);
  if (row === undefined) {
    throw new MisoUsageError(`the generated ABI layout has no command kind ${name}`);
  }
  return row.value;
}

function isControlEdit(edit: ControlFrameEdit | LaneEdit): edit is ControlFrameEdit {
  return typeof edit === "object" && edit !== null && "address" in edit;
}

/**
 * The address an edit coalesces on.
 *
 * Two edits collapse when they would write the same thing: for a value, the same owner, section,
 * row and lane; for an observation record, the same kind, strip, rack, effect slot and tap. The
 * ramp and the values are deliberately NOT part of the key -- they are what the newer edit replaces.
 */
function coalescingKey(edit: ControlFrameEdit | LaneEdit): string {
  if (isControlEdit(edit)) return controlEditKey(edit);
  // The key is the tap's stable key; the strip and effect IDs are length-prefixed so no pair of
  // IDs can spell another pair.
  return [
    edit.kind, edit.rack, edit.stripId.length, edit.stripId, edit.effectId.length, edit.effectId,
    edit.tapId,
  ].join("/");
}

const ID_ENCODER = new TextEncoder();
const ID_FIELD_BYTES = 128;

/** One record ID's UTF-8 bytes; the field holds 127 of them and a zero after the last. */
function recordId(id: string, what: string): Uint8Array {
  if (typeof id !== "string") throw new MisoUsageError(`${what} must be a string`);
  const bytes = ID_ENCODER.encode(id);
  if (bytes.length >= ID_FIELD_BYTES) {
    throw new MisoUsageError(`${what} is ${bytes.length} bytes; a record holds at most ${ID_FIELD_BYTES - 1}`);
  }
  return bytes;
}

/** Check that an observation edit can be encoded; the core judges what it names. */
function checkLaneEdit(edit: LaneEdit): void {
  recordId(edit.stripId, "an observation record's strip ID");
  recordId(edit.effectId, "an observation record's effect ID");
  requireU32(edit.tapId, "tapId");
  requireU32(edit.windowBlocks, "windowBlocks");
}

export function encodeLaneEdits(edits: readonly LaneEdit[]): Uint8Array {
  const records = new Uint8Array(edits.length * RECORD_BYTES);
  const view = new DataView(records.buffer);
  edits.forEach((edit, index) => {
    const base = index * RECORD_BYTES;
    const strip = recordId(edit.stripId, "an observation record's strip ID");
    const effect = recordId(edit.effectId, "an observation record's effect ID");
    view.setUint8(base + offsetOf("kind"), kindValue(edit.kind));
    view.setUint8(base + offsetOf("rack"), edit.rack);
    view.setUint8(base + offsetOf("stripIdBytes"), strip.length);
    view.setUint8(base + offsetOf("effectIdBytes"), effect.length);
    view.setUint32(base + offsetOf("tapId"), requireU32(edit.tapId, "tapId"), true);
    view.setUint32(base + offsetOf("windowBlocks"), requireU32(edit.windowBlocks, "windowBlocks"), true);
    records.set(strip, base + offsetOf("stripId"));
    records.set(effect, base + offsetOf("effectId"));
  });
  return records;
}

/** What one submit of a batch came to, whichever transport carried it. */
type Attempt =
  | { readonly kind: "admitted"; readonly admitted: number }
  | { readonly kind: "flow"; readonly reason: CommandReasonName };

/**
 * Whether a failed submit leaves its edits worth sending again. A full queue (the core's control
 * queue or the plan's in-flight limit) drains. Anything else the core or the queue answered with
 * is about the edits themselves and never succeeds on resend. An error that is not one of the
 * SDK's (a lost port) says nothing about them.
 */
function isFlowControl(error: unknown): boolean {
  if (error instanceof MisoControlRefusal) return error.status === "backpressure";
  return error instanceof MisoEngineError && error.code === "backpressure";
}

function isTerminal(error: unknown): boolean {
  return !isFlowControl(error)
    && (error instanceof MisoControlRefusal || error instanceof MisoEngineError
      || error instanceof MisoUsageError);
}

export class LiveControlWriter {
  readonly #send: (edits: readonly (ControlFrameEdit | LaneEdit)[]) => Promise<Attempt>;
  readonly #controlEdits: boolean;
  readonly #maximumBatch: number;
  #frameBytes: () => number | undefined = () => undefined;
  #transactionEdits: () => number | undefined = () => undefined;
  /** Insertion-ordered by key, which is what makes coalescing a map update rather than a scan. */
  readonly #pending = new Map<string, ControlFrameEdit | LaneEdit>();
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

  /**
   * Exactly one transport: `queue` for value edits (`stage(ControlFrameEdit)`), or `submit` /
   * `submitEdits` for observation records (`stage(LaneEdit)`).
   */
  constructor(options: ControlWriterOptions | WriterOptions | SemanticWriterOptions) {
    const { queue, submit, submitEdits } = options;
    const given = [queue, submit, submitEdits].filter((value) => value !== undefined).length;
    if (given !== 1) {
      throw new MisoUsageError("LiveControlWriter requires exactly one of queue, submit or submitEdits");
    }
    this.#controlEdits = queue !== undefined;
    if (queue !== undefined) {
      if (queue === null || typeof queue.submit !== "function") {
        throw new MisoUsageError("LiveControlWriter queue must have a submit function");
      }
      this.#frameBytes = () => queue.maximumFrameBytes;
      this.#transactionEdits = () => queue.maximumTransactionEdits;
      this.#send = async (edits) => {
        try {
          await queue.submit(edits as readonly ControlFrameEdit[]);
        } catch (error: unknown) {
          // Flow control is typed and retryable. Anything else the core or the queue refused never
          // succeeds on resend: `#flushOnce` drops the edits it names and surfaces the throw.
          if (isFlowControl(error)) return { kind: "flow", reason: BACKPRESSURE_NAME };
          throw error;
        }
        return { kind: "admitted", admitted: edits.length };
      };
    } else {
      let records: (edits: readonly LaneEdit[]) => CommandReport | Promise<CommandReport>;
      if (typeof submitEdits === "function") records = submitEdits;
      else if (typeof submit === "function") {
        records = (edits) => submit(encodeLaneEdits(edits), edits.length);
      } else {
        throw new MisoUsageError("LiveControlWriter submission callback must be a function");
      }
      this.#send = async (edits) => {
        const report = await records(edits as readonly LaneEdit[]);
        if (report.ok) return { kind: "admitted", admitted: report.admitted };
        if (report.reason !== BACKPRESSURE) {
          throw new MisoUsageError(
            `the live controls refused a batch for ${report.reasonName} at record ${report.rejectedIndex}; `
            + "this is a caller error, not flow control, and will not succeed on retry",
          );
        }
        return { kind: "flow", reason: report.reasonName };
      };
    }
    this.#maximumBatch = options.maximumBatch ?? ABI_LAYOUT.constants.maximumCommandRecords;
    if (!Number.isInteger(this.#maximumBatch) || this.#maximumBatch < 1) {
      throw new MisoUsageError(`maximumBatch must be a positive integer`);
    }
    this.#batch = this.#maximumBatch;
  }

  /**
   * Stage one edit, superseding any pending edit for the same address.
   *
   * This is where a drag stops being a backlog: a hundred intermediate positions of one fader
   * collapse to one pending edit, and the one that survives is the latest. An edit the frame
   * cannot hold (a ramp that is not a `u32`, an ID that is not a string) throws a `MisoUsageError`,
   * so it never sits in the pending map and fails every later flush. The value's domain is the
   * core's to judge, at flush.
   */
  stage(edit: ControlFrameEdit | LaneEdit): void {
    if (isControlEdit(edit) !== this.#controlEdits) {
      throw new MisoUsageError(
        this.#controlEdits
          ? "this writer sends value edits; an observation edit needs a writer over a command submit"
          : "this writer sends observation records; a value edit needs a writer over a queue",
      );
    }
    if (isControlEdit(edit)) validateControlEdit(edit);
    else checkLaneEdit(edit);
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
   * The next batch: the oldest pending edits, as many as the batch size allows and, for value
   * edits, as many as one frame holds. At least one, so a batch always makes progress.
   */
  #takeStaged(): [string, ControlFrameEdit | LaneEdit][] {
    const rows = [...this.#pending.entries()];
    const most = Math.min(this.#batch, rows.length, this.#transactionEdits() ?? Number.POSITIVE_INFINITY);
    const ceiling = this.#controlEdits ? this.#frameBytes() : undefined;
    if (ceiling === undefined) return rows.slice(0, most);
    const staged: [string, ControlFrameEdit | LaneEdit][] = [];
    let bytes = CONTROL_TRANSACTION_HEADER_BYTES;
    for (const row of rows) {
      if (staged.length >= most) break;
      bytes += controlEditBytes(row[1] as ControlFrameEdit);
      if (staged.length > 0 && bytes > ceiling) break;
      staged.push(row);
    }
    return staged;
  }

  /**
   * A batch the core or the queue refused for good leaves nothing to resend. A refusal that names
   * an edit (`editIndex`, in the batch's own order) drops that edit alone, so the others go out on
   * the next flush; a refusal that names none, or a call that failed as a whole, drops the batch.
   * Edits are cleared by identity, as after an admission: a newer value staged meanwhile stays.
   */
  #dropRefused(staged: readonly [string, ControlFrameEdit | LaneEdit][], error: unknown): void {
    this.#escalations += 1;
    const named = error instanceof MisoControlRefusal ? error.editIndex : undefined;
    const doomed = named !== undefined && named >= 0 && named < staged.length
      ? [staged[named]!]
      : staged;
    for (const [key, edit] of doomed) {
      if (this.#pending.get(key) === edit) this.#pending.delete(key);
    }
  }

  /**
   * Attempt one batch.
   *
   * On a flow-control refusal nothing is admitted, so nothing is removed from the pending map --
   * the edits stay coalescable, and any newer value staged before the retry supersedes the one
   * that was refused. That is what makes the retry land the hand's final position rather than a
   * queue's worth of stale intermediates.
   *
   * A refusal that is *not* flow control throws (a `MisoControlRefusal` carrying the core's code for
   * a value) and removes what it refused from `pending` first. Backpressure, whether the core's
   * (a `backpressure` status) or the plan's (a `MisoEngineError` with code `backpressure`), will
   * succeed on retry once the render thread drains; an out-of-domain value, an unknown address or a
   * plan that is being rebuilt never will, so keeping it staged would be an infinite loop wearing the
   * costume of resilience. The refusal names an edit (`editIndex`) when the core named one, and only
   * that edit is removed; a refusal that names none removes the whole batch, the rule the queue
   * applies to its own callers. An error that is not the SDK's own (a lost port) leaves the batch
   * staged.
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
    // Keyed pairs rather than bare keys: what is cleared after a success is identified by the
    // EDIT, not by its address. See the delete below.
    const staged = this.#takeStaged();
    const take = staged.length;
    const edits = staged.map(([, edit]) => edit);

    this.#flushes += 1;
    let attempt: Attempt;
    try {
      attempt = await this.#send(edits);
    } catch (error: unknown) {
      if (isTerminal(error)) this.#dropRefused(staged, error);
      throw error;
    }

    if (attempt.kind === "admitted") {
      for (const [key, edit] of staged) {
        // Identity, not presence. The hand does not stop moving while a batch is in flight, and
        // over a port that flight is a round trip: a newer value for an address in this batch can
        // be staged between the submit and its report. Deleting by key alone would drop that newer
        // edit -- `pending` would read zero while only the stale value ever reached the engine,
        // which is precisely the stale-landing bug latest-wins exists to prevent. So a key is
        // cleared only while the map still holds the exact edit that was submitted; a superseded
        // one stays staged and goes out on the next flush.
        //
        // The refusal path below needs no such guard: it admits nothing and therefore deletes
        // nothing, so it has no window to leave open.
        if (this.#pending.get(key) === edit) this.#pending.delete(key);
      }
      this.#admitted += attempt.admitted;
      // Grow back toward the caller's ceiling, so a transient full queue does not permanently
      // shrink throughput.
      this.#batch = Math.min(this.#maximumBatch, Math.max(1, this.#batch * 2));
      return Object.freeze({
        admitted: attempt.admitted,
        refused: false,
        reason: undefined,
        pending: this.#pending.size,
        nextBatch: this.#batch,
      });
    }

    this.#refusals += 1;
    // Halve, so the next attempt discovers the boundary rather than assuming one. A batch of one
    // that still refuses simply waits for the render thread; it never shrinks below one.
    this.#batch = Math.max(1, Math.floor(take / 2));
    return Object.freeze({
      admitted: 0,
      refused: true,
      reason: attempt.reason,
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
