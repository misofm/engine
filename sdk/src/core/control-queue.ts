import {
  controlEditKey,
  decodeControlReply,
  encodeControlTransaction,
  validateControlEdit,
} from "./control-frame.ts";
import type { ControlEditPath, ControlFrameEdit, ControlReply } from "./control-frame.ts";
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
  /** The committed revision the queue starts from. A booted session is at `0`. */
  readonly initialRevision?: bigint;
  /** The request-ID allocator of the endpoint the queue sends through. */
  readonly requestIds?: ControlRequestIds;
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

function newSubmission(edits: readonly ControlFrameEdit[], waited: boolean): {
  readonly submission: Submission;
  readonly promise: Promise<ControlAppliedReport>;
} {
  if (!waited) return { submission: { edits, settle: undefined }, promise: Promise.resolve(undefined as never) };
  let resolve!: (report: ControlAppliedReport) => void;
  let reject!: (error: unknown) => void;
  const promise = new Promise<ControlAppliedReport>((ok, fail) => {
    resolve = ok;
    reject = fail;
  });
  promise.catch(() => undefined);
  return { submission: { edits, settle: { resolve, reject } }, promise };
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
 * # Isolation between callers
 *
 * Coalescing mixes callers, and a transaction is atomic, so one caller's out-of-domain edit would
 * refuse the other callers' valid edits with it. The queue keeps each `submit()` apart instead: when
 * the core refuses a transaction and its first diagnostic names an edit, the submit that owns that
 * edit is rejected with `MisoControlRefusal` (whose `editIndex` then indexes that submit's own
 * edits), and the remaining submits are sent again in a new transaction at the revision the
 * refusal reported. Each refusal removes at least one submit, so this ends. A refusal that names
 * no edit (a revision conflict, backpressure, a malformed frame) rejects every submit it carried
 * and is not retried. A submit the core refused is never retried.
 *
 * The queue decides nothing about a value: the core checks the domain, the address and the revision.
 */
export class ControlTransactionQueue {
  readonly #send: ControlSend;
  readonly #ids: ControlRequestIds;
  #revision: bigint;
  /** The batch being staged, which the next transaction sends. */
  #next: Batch | undefined;
  /** The batch whose transaction is in flight. */
  #flying: Batch | undefined;
  /** Bumped when the session is replaced, so a late reply of the old session sets nothing. */
  #epoch = 0;
  #closed = false;

  constructor(send: ControlSend, options: ControlQueueOptions = {}) {
    if (typeof send !== "function") {
      throw new MisoUsageError("a control transaction queue needs a send function");
    }
    this.#send = send;
    this.#ids = options.requestIds ?? new ControlRequestIds();
    const revision = options.initialRevision ?? 0n;
    if (typeof revision !== "bigint" || revision < 0n) {
      throw new MisoUsageError("initialRevision must be a non-negative bigint");
    }
    this.#revision = revision;
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
    for (const edit of edits) validateControlEdit(edit);
    const batch = this.#next ??= newBatch();
    const { submission, promise } = newSubmission(edits, waited);
    batch.submissions.push(submission);
    return { batch, promise };
  }

  /**
   * Stage `edits` together and send them: they go in one transaction (a pan's two lanes, a matrix's
   * four coefficients), together with anything else staged since the last one. Resolves when that
   * transaction settles; rejects with `MisoControlRefusal` when the core refuses these edits.
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
   * The session was replaced: the queue starts over from `revision` (the core's committed revision
   * after the new session booted; `0` for a booted session on a core that starts every session
   * there). Staged edits are rejected with `MisoUsageError`; a reply still in flight is delivered to
   * its submits but sets no revision. Request IDs keep counting, so none repeats.
   * A closed queue stays closed: `reset()` never reopens it.
   */
  reset(revision = 0n): void {
    if (typeof revision !== "bigint" || revision < 0n) {
      throw new MisoUsageError("revision must be a non-negative bigint");
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

  async #transact(batch: Batch): Promise<void> {
    const epoch = this.#epoch;
    let applied: ControlAppliedReport | undefined;
    let firstRefusal: unknown;
    let stagedRefusal: unknown;
    let remaining = batch.submissions;
    while (remaining.length > 0) {
      const { edits, owners } = coalesce(remaining);
      const requestId = this.#ids.next();
      let reply: ControlReply;
      try {
        const frame = encodeControlTransaction({
          requestId,
          expectedRevision: this.#revision,
          edits,
        });
        reply = decodeControlReply(await this.#send(frame));
        if (reply.requestId !== requestId) {
          throw new MisoEngineError(
            `the control reply answers request ${reply.requestId}, not ${requestId}`,
            { phase: "lifecycle", code: "internal" },
          );
        }
      } catch (error: unknown) {
        for (const submission of remaining) submission.settle?.reject(error);
        firstRefusal ??= error;
        if (remaining.some((submission) => submission.settle === undefined)) stagedRefusal ??= error;
        remaining = [];
        break;
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
        for (const submission of remaining) submission.settle?.resolve(report);
        applied = report;
        remaining = [];
        break;
      }
      const index = reply.diagnostics[0]?.operationIndex;
      const culprit = index === undefined ? undefined : owners[index];
      if (culprit === undefined || edits.length === 0 || !sameSession) {
        // No edit to blame, or the session is gone: the refusal is the whole transaction's.
        const refusal = refusalOf(reply, index);
        for (const submission of remaining) submission.settle?.reject(refusal);
        firstRefusal ??= refusal;
        if (remaining.some((submission) => submission.settle === undefined)) stagedRefusal ??= refusal;
        remaining = [];
        break;
      }
      // Blame the submit that owns the named edit, in its own coordinates, and send the rest again.
      const key = controlEditKey(edits[index!]!);
      let own = -1;
      culprit.edits.forEach((edit, at) => {
        if (controlEditKey(edit) === key) own = at;
      });
      const refusal = refusalOf(reply, own < 0 ? index : own);
      culprit.settle?.reject(refusal);
      firstRefusal ??= refusal;
      if (culprit.settle === undefined) stagedRefusal ??= refusal;
      remaining = remaining.filter((submission) => submission !== culprit);
    }
    this.#flying = undefined;
    if (stagedRefusal !== undefined) batch.settle.reject(stagedRefusal);
    else if (applied !== undefined) batch.settle.resolve(applied);
    else batch.settle.reject(firstRefusal);
    this.#pump();
  }
}
