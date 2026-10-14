import {
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
}

interface Batch {
  readonly edits: Map<string, ControlFrameEdit>;
  readonly promise: Promise<ControlReport>;
  readonly settle: {
    readonly resolve: (report: ControlReport) => void;
    readonly reject: (error: unknown) => void;
  };
}

function addressKey(edit: ControlFrameEdit): string {
  const { owner, section, control, lane } = edit.address;
  return JSON.stringify([
    owner.kind,
    "id" in owner ? owner.id : "",
    section.kind,
    section.kind === "console" ? section.slot : section.kind === "inserts" ? section.effect : "",
    control,
    lane,
  ]);
}

/** The refusal a non-OK reply is: the core's first diagnostic code, else the status name. */
function refusalOf(reply: ControlReply): MisoControlRefusal {
  const first = reply.diagnostics[0];
  return new MisoControlRefusal({
    code: first?.code ?? reply.status,
    status: reply.status,
    revision: reply.revision,
    editIndex: first?.operationIndex,
    diagnostics: reply.diagnostics,
  });
}

function newBatch(): Batch {
  let resolve!: (report: ControlReport) => void;
  let reject!: (error: unknown) => void;
  const promise = new Promise<ControlReport>((done, fail) => {
    resolve = done;
    reject = fail;
  });
  // A staged batch nobody waits on must not become an unhandled rejection; a caller that does wait
  // on `promise` still sees the rejection.
  promise.catch(() => undefined);
  return { edits: new Map(), promise, settle: { resolve, reject } };
}

/**
 * One transaction queue per engine instance (#1563 D4).
 *
 * It holds the committed revision it last saw (from boot, then from every reply frame) and sends
 * each transaction with that exact expected revision. One transaction is in flight at a time:
 * edits staged while one is in flight coalesce by address (the latest value and ramp of an address
 * win; an address staged again moves behind the others, so the order of two overlapping lanes is the
 * order they were last staged) and go in the next transaction. A transaction the core refuses
 * rejects every submit it carried with `MisoControlRefusal` holding the core's code, and is not
 * retried. Every later SDK transaction API uses this queue.
 *
 * The queue decides nothing about a value: the core checks the domain, the address and the revision.
 */
export class ControlTransactionQueue {
  readonly #send: ControlSend;
  #revision: bigint;
  #nextRequestId = 1n;
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
    const revision = options.initialRevision ?? 0n;
    if (typeof revision !== "bigint" || revision < 0n) {
      throw new MisoUsageError("initialRevision must be a non-negative bigint");
    }
    this.#revision = revision;
  }

  /** The last committed revision the queue saw. */
  get revision(): bigint {
    return this.#revision;
  }

  /** The number of addresses staged for the next transaction. */
  get pending(): number {
    return this.#next?.edits.size ?? 0;
  }

  /** Whether a transaction is in flight. */
  get inFlight(): boolean {
    return this.#flying !== undefined;
  }

  /**
   * Stage one edit for the next transaction, superseding any staged edit of the same address.
   * Throws `MisoUsageError` when the edit cannot be encoded, so it never sits staged and fails every
   * later flush.
   */
  stage(edit: ControlFrameEdit): void {
    this.#stageAll([edit]);
  }

  #stageAll(edits: readonly ControlFrameEdit[]): Batch {
    if (this.#closed) throw new MisoUsageError("the control transaction queue is closed");
    for (const edit of edits) validateControlEdit(edit);
    const batch = this.#next ??= newBatch();
    for (const edit of edits) {
      const key = addressKey(edit);
      // Delete before set: the most recently staged address goes last in the transaction.
      batch.edits.delete(key);
      batch.edits.set(key, edit);
    }
    return batch;
  }

  /**
   * Stage `edits` together and send them: they go in one transaction (a pan's two lanes, a matrix's
   * four coefficients), together with anything else staged since the last one. Resolves when that
   * transaction settles; rejects with `MisoControlRefusal` when the core refuses it.
   */
  submit(edits: readonly ControlFrameEdit[]): Promise<ControlReport> {
    if (edits.length === 0) throw new MisoUsageError("a submit needs at least one edit");
    const batch = this.#stageAll(edits);
    this.#pump();
    return batch.promise;
  }

  /**
   * Send everything staged. Resolves when the transaction that carries it settles; with nothing
   * staged it waits for the transaction in flight, or resolves at once with an empty report.
   */
  flush(): Promise<ControlReport> {
    if (this.#next !== undefined) {
      const batch = this.#next;
      this.#pump();
      return batch.promise;
    }
    if (this.#flying !== undefined) return this.#flying.promise;
    return Promise.resolve(Object.freeze({
      ok: true as const,
      revision: this.#revision,
      path: "model_only" as const,
      appliedOperations: 0,
    }));
  }

  /**
   * The session was replaced: the queue starts over from `revision` (a booted session is at `0`).
   * Staged edits are rejected with `MisoUsageError`; a reply still in flight is delivered to its
   * submits but sets no revision. Request IDs keep counting, so none repeats.
   */
  reset(revision = 0n): void {
    if (typeof revision !== "bigint" || revision < 0n) {
      throw new MisoUsageError("revision must be a non-negative bigint");
    }
    this.#epoch += 1;
    this.#revision = revision;
    this.#closed = false;
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
    batch?.settle.reject(error);
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
    const edits = [...batch.edits.values()];
    const requestId = this.#nextRequestId;
    this.#nextRequestId += 1n;
    let outcome: { readonly reply: ControlReply } | { readonly error: unknown };
    try {
      const frame = encodeControlTransaction({
        requestId,
        expectedRevision: this.#revision,
        edits,
      });
      const reply = decodeControlReply(await this.#send(frame));
      if (reply.requestId !== requestId) {
        throw new MisoEngineError(
          `the control reply answers request ${reply.requestId}, not ${requestId}`,
          { phase: "lifecycle", code: "internal" },
        );
      }
      if (epoch === this.#epoch) this.#revision = reply.revision;
      outcome = { reply };
    } catch (error: unknown) {
      outcome = { error };
    }
    this.#flying = undefined;
    if ("error" in outcome) batch.settle.reject(outcome.error);
    else if (outcome.reply.applied !== undefined) {
      batch.settle.resolve(Object.freeze({
        ok: true as const,
        revision: outcome.reply.revision,
        path: outcome.reply.applied.editPath,
        appliedOperations: outcome.reply.applied.appliedOperations,
      }));
    } else batch.settle.reject(refusalOf(outcome.reply));
    this.#pump();
  }
}
