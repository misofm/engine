import {
  CONTROL_TRANSACTION_HEADER_BYTES,
  controlEditBytes,
  controlEditKey,
  decodeControlReply,
  encodeControlTransaction,
} from "./control-frame.ts";
import type {
  ControlEditPath,
  ControlFrameEdit,
  ControlLimits,
  ControlReply,
} from "./control-frame.ts";
import { MisoControlRefusal, MisoEngineError, MisoUsageError } from "./errors.ts";

/**
 * Send one command frame to the core and resolve with its reply frame. The headless engine calls the
 * core in process; the browser posts the frame to the Worker that owns the core.
 */
export type ControlSend = (frame: Uint8Array) => Uint8Array | PromiseLike<Uint8Array>;

/** The result of one transaction the core committed. */
export interface ControlAppliedReport {
  readonly ok: true;
  /** The revision the core committed. */
  readonly revision: bigint;
  /** How the core applied it. */
  readonly path: ControlEditPath;
  /** How many edits the transaction held after the queue coalesced the staged ones. */
  readonly appliedOperations: number;
}

/** The result of one transaction the core refused, as `MisoControlRefusal.report` carries it. */
export interface ControlRefusedReport {
  readonly ok: false;
  /** The revision the core reported with the refusal: nothing changed. */
  readonly revision: bigint;
  /** The core's code for the refusal. */
  readonly code: string;
  /** The index of the edit the core's first diagnostic names, in the transaction that was sent. */
  readonly editIndex?: number;
}

export type ControlReport = ControlAppliedReport | ControlRefusedReport;

export interface ControlQueueOptions {
  /**
   * The committed revision the queue starts from: the core's own, as its boot left it. A session
   * does not necessarily boot at `0`, so there is no default.
   */
  readonly initialRevision: bigint;
  /** The request-ID allocator of the endpoint the queue sends through. */
  readonly requestIds?: ControlRequestIds;
  /**
   * What the host's control plane reports as the largest frame and transaction it takes (read from
   * its capabilities). The queue closes a transaction at a submit boundary before either is passed,
   * and refuses a single submit that is larger up front. Absent leaves the queue unbounded, which
   * suits only a scripted core.
   */
  readonly limits?: ControlLimits | undefined;
  /**
   * Read the committed revision from the core. The queue calls it after a call failed without a
   * reply, because the core may have committed the transaction whose reply was lost.
   */
  readonly readRevision?: (() => bigint | PromiseLike<bigint>) | undefined;
}

/** What the queue counted since it was made. */
export interface ControlQueueStats {
  /** Transactions sent, retries included. */
  readonly transactions: number;
  /** Edits dropped from a transaction because a later edit of the same address was staged. */
  readonly coalescedEdits: number;
  /**
   * Submits that resolved `ok` although none of their values reached the core, because every
   * address they held was staged again by a later submit before it was sent. The core committed the
   * later values, and the report these submits resolve with is that transaction's.
   */
  readonly replacedSubmits: number;
}

/** A request-ID allocator. One serves every frame sent through one endpoint, and never repeats. */
export class ControlRequestIds {
  #next = 1n;

  /** The next nonzero request ID. */
  next(): bigint {
    const id = this.#next;
    this.#next += 1n;
    return id;
  }
}

interface Settler<T> {
  readonly resolve: (value: T) => void;
  readonly reject: (error: unknown) => void;
}

/**
 * One call of `submit()` (or one `stage()`): the edits it holds are atomic, so the core applies all
 * of them or the submit is rejected whole, whatever other callers staged beside them.
 */
interface Submission {
  readonly edits: readonly ControlFrameEdit[];
  /** The encoded bytes of each edit, parallel to `edits`. */
  readonly sizes: readonly number[];
  readonly settle: Settler<ControlAppliedReport> | undefined;
}

interface Batch {
  readonly submissions: Submission[];
  /** Settles with the batch's outcome: what `flush()` returns. */
  readonly done: Promise<ControlAppliedReport>;
  readonly settle: Settler<ControlAppliedReport>;
}

function refusalOf(reply: ControlReply, editIndex: number | undefined): MisoControlRefusal {
  const first = reply.diagnostics[0];
  return new MisoControlRefusal({
    code: first?.code ?? reply.status,
    status: reply.status,
    revision: reply.revision,
    editIndex,
    diagnostics: reply.diagnostics,
  });
}

function newBatch(): Batch {
  let resolve!: (report: ControlAppliedReport) => void;
  let reject!: (error: unknown) => void;
  const done = new Promise<ControlAppliedReport>((ok, fail) => {
    resolve = ok;
    reject = fail;
  });
  // A staged batch nobody waits on must not become an unhandled rejection; a caller that does wait
  // on `done` still sees the rejection.
  done.catch(() => undefined);
  return { submissions: [], done, settle: { resolve, reject } };
}

function newSubmission(
  edits: readonly ControlFrameEdit[],
  sizes: readonly number[],
  waited: boolean,
): {
  readonly submission: Submission;
  readonly promise: Promise<ControlAppliedReport>;
} {
  if (!waited) {
    return { submission: { edits, sizes, settle: undefined }, promise: Promise.resolve(undefined as never) };
  }
  let resolve!: (report: ControlAppliedReport) => void;
  let reject!: (error: unknown) => void;
  const promise = new Promise<ControlAppliedReport>((ok, fail) => {
    resolve = ok;
    reject = fail;
  });
  promise.catch(() => undefined);
  return { submission: { edits, sizes, settle: { resolve, reject } }, promise };
}

/** One transaction's edits: the latest value of each address, the most recently staged last. */
function coalesce(submissions: readonly Submission[]): {
  readonly edits: ControlFrameEdit[];
  readonly owners: Submission[];
} {
  const latest = new Map<string, { edit: ControlFrameEdit; owner: Submission }>();
  for (const submission of submissions) {
    for (const edit of submission.edits) {
      const key = controlEditKey(edit);
      // Delete before set: the most recently staged address goes last in the transaction.
      latest.delete(key);
      latest.set(key, { edit, owner: submission });
    }
  }
  const rows = [...latest.values()];
  return { edits: rows.map((row) => row.edit), owners: rows.map((row) => row.owner) };
}

/**
 * One transaction queue per engine instance (#1563 D4).
 *
 * It holds the committed revision it last saw (from boot, then from every reply frame) and sends
 * each transaction with that exact expected revision. One transaction is in flight at a time:
 * edits staged while one is in flight coalesce by address (the latest value and ramp of an address
 * win; an address staged again moves behind the others, so the order of two overlapping lanes is the
 * order they were last staged) and go in the next transaction. Every later SDK transaction API uses
 * this queue.
 *
 * # Size
 *
 * A control frame has a ceiling (the host's `maximum_frame_bytes`, 16 KiB in the browser and 4 KiB on
 * the C ABI, and a count of edits), and a frame over it fails differently on each transport. The
 * queue reads the ceiling from the core (`options.limits`) and never builds a frame over it: staged
 * submits are cut into transactions at a submit boundary, so each submit stays atomic, and they go
 * one after another in the order they were staged. A single submit that cannot fit even alone is
 * refused up front with `MisoUsageError`, naming the limit.
 *
 * # Isolation between callers
 *
 * Coalescing mixes callers, and a transaction is atomic, so one caller's out-of-domain edit would
 * refuse the other callers' valid edits with it. The queue keeps each `submit()` apart instead: when
 * the core refuses a transaction and its first diagnostic names an edit, the submit that owns that
 * edit is rejected with `MisoControlRefusal` (whose `editIndex` then indexes that submit's own
 * edits), and the remaining submits are sent again in a new transaction at the revision the
 * refusal reported. Each refusal removes at least one submit, so this ends. A refusal that names
 * no edit (a revision conflict, backpressure, a malformed frame) rejects every submit of that
 * transaction, with no `editIndex`, and is not retried. A submit the core refused is never retried.
 *
 * A call that fails without a reply (the plan is being rebuilt, the control queue is full, a frame
 * the transport would not take, a lost port) does not say which submit it is about. When it carried
 * more than one submit, each of those is sent again alone, once, so one caller's failure rejects
 * that caller and no other. The queue then reads the revision from the core again
 * (`options.readRevision`), because a reply may have been lost after the core committed.
 *
 * The queue decides nothing about a value: the core checks the domain, the address and the revision.
 *
 * # A replaced submit
 *
 * When a later submit holds every address of an earlier one before the earlier one is sent, the
 * earlier values are never sent. That submit resolves `ok` with the report of the transaction that
 * carried the later values, and `stats.replacedSubmits` counts it. The core never saw its value.
 */
export class ControlTransactionQueue {
  readonly #send: ControlSend;
  readonly #ids: ControlRequestIds;
  readonly #limits: ControlLimits | undefined;
  readonly #readRevision: (() => bigint | PromiseLike<bigint>) | undefined;
  #revision: bigint;
  /** The batch being staged, which the next transaction sends. */
  #next: Batch | undefined;
  /** The batch whose transaction is in flight. */
  #flying: Batch | undefined;
  /** Bumped when the session is replaced, so a late reply of the old session sets nothing. */
  #epoch = 0;
  #closed = false;
  #transactions = 0;
  #coalescedEdits = 0;
  #replacedSubmits = 0;

  constructor(send: ControlSend, options: ControlQueueOptions) {
    if (typeof send !== "function") {
      throw new MisoUsageError("a control transaction queue needs a send function");
    }
    this.#send = send;
    this.#ids = options?.requestIds ?? new ControlRequestIds();
    const revision = options?.initialRevision;
    if (typeof revision !== "bigint" || revision < 0n) {
      throw new MisoUsageError("initialRevision must be a non-negative bigint: the core's committed revision");
    }
    this.#revision = revision;
    this.#limits = options.limits;
    this.#readRevision = options.readRevision;
  }

  /**
   * The request-ID allocator of this queue's endpoint. A frame the endpoint carries that is not a
   * transaction (a catalog read, for one) draws its ID here, so no two requests share one.
   */
  get requestIds(): ControlRequestIds {
    return this.#ids;
  }

  /** The last committed revision the queue saw. */
  get revision(): bigint {
    return this.#revision;
  }

  /** The host's frame ceiling in bytes, or `undefined` when the queue was given none. */
  get maximumFrameBytes(): number | undefined {
    return this.#limits?.maximumFrameBytes;
  }

  /** The host's ceiling on the edits of one transaction, or `undefined` when the queue was given none. */
  get maximumTransactionEdits(): number | undefined {
    return this.#limits?.maximumTransactionEdits;
  }

  /** What the queue counted. */
  get stats(): ControlQueueStats {
    return Object.freeze({
      transactions: this.#transactions,
      coalescedEdits: this.#coalescedEdits,
      replacedSubmits: this.#replacedSubmits,
    });
  }

  /** The number of addresses staged for the next transaction. */
  get pending(): number {
    const batch = this.#next;
    return batch === undefined ? 0 : coalesce(batch.submissions).edits.length;
  }

  /** Whether a transaction is in flight. */
  get inFlight(): boolean {
    return this.#flying !== undefined;
  }

  /**
   * Stage one edit for the next transaction, superseding any staged edit of the same address.
   * Throws `MisoUsageError` when the edit cannot be encoded, so it never sits staged and fails every
   * later flush. A core refusal of a staged edit rejects the `flush()` of its batch.
   */
  stage(edit: ControlFrameEdit): void {
    this.#stageAll([edit], false);
  }

  #stageAll(
    edits: readonly ControlFrameEdit[],
    waited: boolean,
  ): { readonly batch: Batch; readonly promise: Promise<ControlAppliedReport> } {
    if (this.#closed) throw new MisoUsageError("the control transaction queue is closed");
    const sizes = edits.map((edit) => controlEditBytes(edit));
    this.#requireFits(edits, sizes);
    const batch = this.#next ??= newBatch();
    const { submission, promise } = newSubmission(edits, sizes, waited);
    batch.submissions.push(submission);
    return { batch, promise };
  }

  /** Refuse, before anything is staged, a submit no transaction can carry. */
  #requireFits(edits: readonly ControlFrameEdit[], sizes: readonly number[]): void {
    const limits = this.#limits;
    if (limits === undefined) return;
    const rows = new Map<string, number>();
    edits.forEach((edit, at) => {
      rows.set(controlEditKey(edit), sizes[at]!);
    });
    let bytes = CONTROL_TRANSACTION_HEADER_BYTES;
    for (const size of rows.values()) bytes += size;
    if (bytes > limits.maximumFrameBytes) {
      throw new MisoUsageError(
        `a submit of ${rows.size} control edits is ${bytes} bytes, over the host's control frame limit `
          + `of ${limits.maximumFrameBytes} bytes; split it into smaller submits`,
      );
    }
    if (rows.size > limits.maximumTransactionEdits) {
      throw new MisoUsageError(
        `a submit of ${rows.size} control edits is over the host's limit of `
          + `${limits.maximumTransactionEdits} edits in one transaction; split it into smaller submits`,
      );
    }
  }

  /**
   * Stage `edits` together and send them: they go in one transaction (a pan's two lanes, a matrix's
   * four coefficients), together with anything else staged since the last one that fits the host's
   * frame limit. Resolves when that transaction settles; rejects with `MisoControlRefusal` when the
   * core refuses these edits, and with `MisoUsageError` before anything is staged when they are
   * larger than one transaction can be.
   */
  submit(edits: readonly ControlFrameEdit[]): Promise<ControlAppliedReport> {
    if (edits.length === 0) throw new MisoUsageError("a submit needs at least one edit");
    const { promise } = this.#stageAll([...edits], true);
    this.#pump();
    return promise;
  }

  /**
   * Send everything staged. Resolves when the transaction that carries it settles; with nothing
   * staged it waits for the transaction in flight, or resolves at once with an empty report.
   * It rejects when the core refused a `stage()`d edit of the batch, or when the core refused
   * every submit of it.
   */
  flush(): Promise<ControlAppliedReport> {
    if (this.#next !== undefined) {
      const batch = this.#next;
      this.#pump();
      return batch.done;
    }
    if (this.#flying !== undefined) return this.#flying.done;
    return Promise.resolve(Object.freeze({
      ok: true as const,
      revision: this.#revision,
      path: "model_only" as const,
      appliedOperations: 0,
    }));
  }

  /**
   * The session was replaced: the queue starts over from `revision`, the core's committed revision
   * after the new session booted. Staged edits are rejected with `MisoUsageError`; a reply still in
   * flight is delivered to its submits but sets no revision. Request IDs keep counting, so none
   * repeats. A closed queue stays closed: `reset()` never reopens it.
   */
  reset(revision: bigint): void {
    if (typeof revision !== "bigint" || revision < 0n) {
      throw new MisoUsageError("revision must be a non-negative bigint: the core's committed revision");
    }
    this.#epoch += 1;
    this.#revision = revision;
    this.#rejectNext(new MisoUsageError("the session was replaced before the staged edits were sent"));
  }

  /** Reject whatever is staged and refuse new edits. A transaction in flight still settles. */
  close(): void {
    this.#closed = true;
    this.#rejectNext(new MisoUsageError("the control transaction queue was closed"));
  }

  #rejectNext(error: unknown): void {
    const batch = this.#next;
    this.#next = undefined;
    if (batch === undefined) return;
    for (const submission of batch.submissions) submission.settle?.reject(error);
    batch.settle.reject(error);
  }

  #pump(): void {
    if (this.#flying !== undefined || this.#next === undefined) return;
    const batch = this.#next;
    this.#next = undefined;
    this.#flying = batch;
    void this.#transact(batch);
  }

  /**
   * The submits, from the front of `remaining`, one transaction carries: as many as fit the host's
   * frame and edit limits once coalesced, and at least one. `most` caps the count.
   */
  #takeGroup(remaining: readonly Submission[], most: number): Submission[] {
    const limits = this.#limits;
    if (limits === undefined) return remaining.slice(0, most);
    const group: Submission[] = [];
    let rows = new Map<string, number>();
    let bytes = CONTROL_TRANSACTION_HEADER_BYTES;
    for (const submission of remaining) {
      if (group.length >= most) break;
      const trial = new Map(rows);
      let trialBytes = bytes;
      submission.edits.forEach((edit, at) => {
        const key = controlEditKey(edit);
        const size = submission.sizes[at]!;
        trialBytes += size - (trial.get(key) ?? 0);
        trial.set(key, size);
      });
      if (group.length > 0
        && (trialBytes > limits.maximumFrameBytes || trial.size > limits.maximumTransactionEdits)) {
        break;
      }
      rows = trial;
      bytes = trialBytes;
      group.push(submission);
    }
    return group;
  }

  async #refreshRevision(epoch: number): Promise<void> {
    if (this.#readRevision === undefined) return;
    try {
      const revision = await this.#readRevision();
      if (epoch === this.#epoch && typeof revision === "bigint") this.#revision = revision;
    } catch {
      // The core cannot be read either: the next transaction reports what is wrong.
    }
  }

  async #transact(batch: Batch): Promise<void> {
    const epoch = this.#epoch;
    let applied: ControlAppliedReport | undefined;
    let firstRefusal: unknown;
    let stagedRefusal: unknown;
    let remaining: Submission[] = batch.submissions.slice();
    /** How many of the next submits are sent alone, after a call failed with no reply. */
    let alone = 0;
    const fail = (submissions: readonly Submission[], error: unknown): void => {
      for (const submission of submissions) submission.settle?.reject(error);
      firstRefusal ??= error;
      if (submissions.some((submission) => submission.settle === undefined)) stagedRefusal ??= error;
    };
    try {
      while (remaining.length > 0) {
        const group = this.#takeGroup(remaining, alone > 0 ? 1 : Number.POSITIVE_INFINITY);
        if (alone > 0) alone -= 1;
        const { edits, owners } = coalesce(group);
        const requestId = this.#ids.next();
        let reply: ControlReply;
        try {
          const frame = encodeControlTransaction({
            requestId,
            expectedRevision: this.#revision,
            edits,
          });
          this.#transactions += 1;
          reply = decodeControlReply(await this.#send(frame));
          if (reply.requestId !== requestId) {
            throw new MisoEngineError(
              `the control reply answers request ${reply.requestId}, not ${requestId}`,
              { phase: "lifecycle", code: "internal" },
            );
          }
        } catch (error: unknown) {
          await this.#refreshRevision(epoch);
          if (group.length > 1) {
            // It names no submit: each one is sent alone, so only the one at fault is rejected.
            alone = group.length;
            continue;
          }
          fail(group, error);
          remaining = remaining.slice(group.length);
          continue;
        }
        const sameSession = epoch === this.#epoch;
        if (sameSession) this.#revision = reply.revision;
        if (reply.applied !== undefined) {
          const report = Object.freeze({
            ok: true as const,
            revision: reply.revision,
            path: reply.applied.editPath,
            appliedOperations: reply.applied.appliedOperations,
          });
          const sent = new Set(owners);
          for (const submission of group) {
            if (!sent.has(submission)) this.#replacedSubmits += 1;
            submission.settle?.resolve(report);
          }
          this.#coalescedEdits += group.reduce((total, row) => total + row.edits.length, 0) - edits.length;
          applied = report;
          remaining = remaining.slice(group.length);
          continue;
        }
        const index = reply.diagnostics[0]?.operationIndex;
        const culprit = index === undefined ? undefined : owners[index];
        // The named edit, in its own submit's coordinates; `undefined` when it names none of them.
        let own: number | undefined;
        if (culprit !== undefined) {
          const key = controlEditKey(edits[index!]!);
          culprit.edits.forEach((edit, at) => {
            if (controlEditKey(edit) === key) own = at;
          });
        }
        if (culprit === undefined || !sameSession) {
          // No edit to blame, or the session is gone: the refusal is the whole transaction's, and
          // it carries an edit index only when the transaction was one submit.
          fail(group, refusalOf(reply, group.length === 1 ? own : undefined));
          remaining = remaining.slice(group.length);
          continue;
        }
        // Blame the submit that owns the named edit, in its own coordinates, and send the rest again.
        fail([culprit], refusalOf(reply, own));
        remaining = remaining.filter((submission) => submission !== culprit);
      }
    } catch (error: unknown) {
      // A fault in the queue itself: nobody is left waiting on a transaction that will never settle.
      fail(remaining, error);
    } finally {
      this.#flying = undefined;
      if (stagedRefusal !== undefined) batch.settle.reject(stagedRefusal);
      else if (applied !== undefined) batch.settle.resolve(applied);
      else batch.settle.reject(firstRefusal ?? new MisoUsageError("the control transaction sent nothing"));
      this.#pump();
    }
  }
}
