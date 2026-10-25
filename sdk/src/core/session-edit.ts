import {
  MESSAGE_SESSION_REPLACE,
  WIRE_BYTES,
  WIRE_U32,
  commandFrame,
  concat,
  field,
  requireU32,
  requireU64,
  u32,
} from "./control-frame.ts";
import type { ControlEditPath, ControlReply } from "./control-frame.ts";
import { MisoControlRefusal, MisoUsageError } from "./errors.ts";

/**
 * The shared core of a structural session edit (#1389, root decision R1; #1296 reuses it): the
 * types `apply` and `replaceSession` resolve with, the `SESSION_REPLACE` request encoder, the
 * watermark snapshot, and the ledger that settles `applied(revision)` from it.
 *
 * Both engines send a transaction and a replacement document through the same core entry
 * (`SESSION_TRANSACTION_APPLY`, `SESSION_REPLACE`, decision 17 C13); only the transport differs.
 * The core decides every path, refusal and revision; this file names and correlates them.
 */

/** How the core applied an edit: `live`, `model_only`, `rebuild`, or for a replace, `unchanged`. */
export type EditPath = ControlEditPath;

/** What a committed `apply` or `replaceSession` resolves with. */
export interface EditOutcome {
  /** The committed revision after the edit; for `unchanged`, the revision that stays committed. */
  readonly revision: bigint;
  readonly path: EditPath;
}

/**
 * The applied-revision watermark: `revision` is the highest committed revision in effect together
 * with every revision before it, and `sample` the absolute render sample of the first block in
 * which it was in effect (0 for the initial revision).
 */
export interface WatermarkSnapshot {
  readonly revision: bigint;
  readonly sample: bigint;
  /** The outcome flags of the revisions the last advance covered. */
  readonly outcome: {
    /** The covered revisions completed exactly as committed. */
    readonly exact: boolean;
    /** A warm successor could not adopt, and its revisions completed through the transition. */
    readonly transitionFallback: boolean;
    /** The covered revisions include ones a newer candidate replaced before render adopted them. */
    readonly superseded: boolean;
  };
}

const OUTCOME_EXACT = 1n;
const OUTCOME_TRANSITION_FALLBACK = 2n;
const OUTCOME_SUPERSEDED = 4n;

/** A snapshot from the record's three words. @internal */
export function watermarkSnapshot(revision: bigint, sample: bigint, flags: bigint): WatermarkSnapshot {
  return Object.freeze({
    revision,
    sample,
    outcome: Object.freeze({
      exact: (flags & OUTCOME_EXACT) !== 0n,
      transitionFallback: (flags & OUTCOME_TRANSITION_FALLBACK) !== 0n,
      superseded: (flags & OUTCOME_SUPERSEDED) !== 0n,
    }),
  });
}

/** Bytes of a `SESSION_REPLACE` document digest: a SHA-256. */
const DIGEST_BYTES = 32;

/** What a `SESSION_REPLACE` frame needs: the document is passed beside it, never in it. @internal */
export interface SessionReplaceRequest {
  readonly requestId: bigint;
  readonly expectedRevision: bigint;
  /** The document's exact length, nonzero. */
  readonly documentBytes: number;
  /** The document's SHA-256. */
  readonly documentSha256: Uint8Array;
}

/**
 * Encode a `SESSION_REPLACE` command frame: `document_bytes` (field 1, `u32`) and `document_sha256`
 * (field 2, 32 bytes), both mandatory (`crates/protocol/src/schema.rs`). Throws `MisoUsageError`
 * for a zero request ID or length or a digest that is not 32 bytes.
 * @internal
 */
export function encodeSessionReplaceRequest(request: SessionReplaceRequest): Uint8Array {
  const requestId = requireU64(request.requestId, "requestId");
  if (requestId === 0n) throw new MisoUsageError("requestId must not be zero");
  const expectedRevision = requireU64(request.expectedRevision, "expectedRevision");
  const documentBytes = requireU32(request.documentBytes, "documentBytes");
  if (documentBytes === 0) throw new MisoUsageError("a replacement document must not be empty");
  if (!(request.documentSha256 instanceof Uint8Array) || request.documentSha256.length !== DIGEST_BYTES) {
    throw new MisoUsageError("documentSha256 must be the 32 bytes of a SHA-256");
  }
  const payload = concat([
    field(1, WIRE_U32, true, u32(documentBytes)),
    field(2, WIRE_BYTES, true, request.documentSha256),
  ]);
  return commandFrame(MESSAGE_SESSION_REPLACE, 0, requestId, expectedRevision, 2, payload);
}

/**
 * The SHA-256 of a document through Web Crypto, the one digest both hosts have without an import
 * (root decision R2: no target-specific digest).
 * @internal
 */
export async function documentSha256(document: Uint8Array<ArrayBuffer>): Promise<Uint8Array> {
  return new Uint8Array(await crypto.subtle.digest("SHA-256", document));
}

/**
 * The outcome of a decoded `SESSION_TRANSACTION_APPLY` or `SESSION_REPLACE` reply, or the core's
 * refusal as `MisoControlRefusal` (its status, diagnostics and the revision it observed), as every
 * protocol call reports a refusal. Nothing was committed by a refused edit.
 * @internal
 */
export function editOutcome(reply: ControlReply): EditOutcome {
  if (reply.applied === undefined) {
    throw new MisoControlRefusal({
      code: reply.diagnostics[0]?.code ?? reply.status,
      status: reply.status,
      revision: reply.revision,
      editIndex: reply.diagnostics[0]?.operationIndex,
      diagnostics: reply.diagnostics,
    });
  }
  return Object.freeze({ revision: reply.revision, path: reply.applied.editPath });
}

interface PendingApplied {
  readonly revision: bigint;
  readonly resolve: (snapshot: WatermarkSnapshot) => void;
  readonly reject: (error: unknown) => void;
}

/**
 * The pending `applied(revision)` promises of one engine (D15-17). A promise resolves with the first
 * watermark snapshot that covers its revision, never at the commit: completion counts render
 * samples, so it stays pending while nothing renders. `rejectAll` ends every pending promise (a
 * reboot or a dispose) and forgets the snapshot, since revisions restart.
 * @internal
 */
export class AppliedLedger {
  #latest: WatermarkSnapshot | undefined;
  #pending: PendingApplied[] = [];

  /** The newest snapshot settled, if any. */
  get latest(): WatermarkSnapshot | undefined {
    return this.#latest;
  }

  /** Resolve when a snapshot covers `revision`; at once when the newest one already does. */
  applied(revision: bigint): Promise<WatermarkSnapshot> {
    if (this.#latest !== undefined && this.#latest.revision >= revision) {
      return Promise.resolve(this.#latest);
    }
    return new Promise((resolve, reject) => {
      this.#pending.push({ revision, resolve, reject });
    });
  }

  /** Take a new snapshot and resolve every pending revision it covers. */
  settle(snapshot: WatermarkSnapshot): void {
    this.#latest = snapshot;
    if (this.#pending.length === 0) return;
    const waiting: PendingApplied[] = [];
    for (const entry of this.#pending) {
      if (entry.revision <= snapshot.revision) entry.resolve(snapshot);
      else waiting.push(entry);
    }
    this.#pending = waiting;
  }

  /** Reject every pending revision with `error` and forget the snapshot. */
  rejectAll(error: unknown): void {
    const pending = this.#pending;
    this.#pending = [];
    this.#latest = undefined;
    for (const entry of pending) entry.reject(error);
  }
}
