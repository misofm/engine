import { constantValue } from "./abi.ts";
import { MisoEngineError, MisoUsageError } from "./errors.ts";
import type { LoudnessFrameOperation, LoudnessFrameReply, LoudnessFrameState } from "./loudness-frame.ts";
import type { ObservationSubscriptionScheduler } from "./observation-subscriptions.ts";
import { spectrumRefusalError } from "./spectrum.ts";

/**
 * One reading of the session's output loudness meter (#2115): ITU-R BS.1770-5 loudness and EBU
 * Tech 3342 loudness range of the rendered master pair, and its BS.1770-5 Annex-2 true peak.
 *
 * A level is absent until its window holds enough audio: `momentary` after 400 ms, `shortTerm`
 * after 3 s, `integrated` and `range` after one gating block passes both gates. Silence reads
 * -120 LUFS (and a zero true peak -120 dBTP), the finite floor. Sample positions are `bigint`, as
 * every SDK sample position is.
 */
export interface LoudnessReading {
  /** Momentary loudness (400 ms window), LUFS. */
  readonly momentary?: number;
  /** Short-term loudness (3 s window), LUFS. */
  readonly shortTerm?: number;
  /** Integrated (gated) loudness since the last start or reset, LUFS. */
  readonly integrated?: number;
  /** Loudness range since the last start or reset (EBU Tech 3342), LU. */
  readonly range?: number;
  /** The largest momentary loudness since the last start or reset, LUFS. */
  readonly maxMomentary?: number;
  /** The largest short-term loudness since the last start or reset, LUFS. */
  readonly maxShortTerm?: number;
  /** The true peak since the last start or reset, `[left, right]`, dBTP. */
  readonly truePeak?: readonly [number, number];
  /** The true peak of the last 10 ms hop the meter delivered, `[left, right]`, dBTP. */
  readonly truePeakLastHop?: readonly [number, number];
  /** The first sample the integrated values count from: the first hop boundary at or after the last start or reset. */
  readonly integratingSince?: bigint;
  /**
   * Hops render dropped because the meter's record queue was full. The count is the engine's,
   * monotone over the session's whole life (a start or reset does not clear it): subtract two
   * readings to learn how many hops were lost between them.
   */
  readonly hopsDropped: bigint;
  /** The end of the last hop the meter delivered, or the sample it was armed from before its first hop. */
  readonly observedSample: bigint;
  /** Whether the meter is measuring. */
  readonly armed: boolean;
}

/** What `onUpdate` receives: a delivered reading that differs from the one before it. */
export interface LoudnessSubscriptionNotification {
  readonly handle: LoudnessSubscription;
  readonly reading: LoudnessReading;
}

/** Optional delivery for `subscribeLoudness`. */
export interface LoudnessSubscriptionRequest {
  /** Called, off render, with each delivered reading that differs from the previous one. */
  readonly onUpdate?: (notification: LoudnessSubscriptionNotification) => void;
  /**
   * Called with the error of a timed read that failed (a deadline, a closed port, a refusal). The
   * handle stays open and the next tick reads again; until a read lands, `readLatest()` keeps the
   * last reading, so its `observedSample` stops advancing. `pump()` rejects with its own failures.
   */
  readonly onError?: (error: unknown) => void;
}

/**
 * A managed handle on the session's one output loudness meter. The subscribe started the meter;
 * the handle reads it about every {@link LOUDNESS_READ_CADENCE_MS} ms (the browser on a timer of
 * the engine's control side, the headless engine at each `pump()`), and stops it at `close()`.
 */
export interface LoudnessSubscription {
  /** The handle itself, as the other managed handles carry it. */
  readonly handle: LoudnessSubscription;
  /**
   * The newest reading a `read`, `reset` or `stop` delivered: `undefined` before the first one
   * lands. After `close()` it keeps the last reading (the stop's, `armed: false`), so a console can
   * still show the final values.
   */
  readLatest(): LoudnessReading | undefined;
  /** Read the meter now (one `LOUDNESS` read) and deliver the reading. Rejects after `close()`. */
  pump(): Promise<LoudnessReading>;
  /**
   * Restart integration at the engine's latest render sample: the integrated loudness, the range,
   * the maxima and the true peak since reset start again from the next hop boundary; the momentary
   * and short-term windows keep running. Rejects with `MisoUsageError` after `close()`.
   */
  reset(): Promise<void>;
  /**
   * Stop the meter. From the call on, the handle sends no read: the timer stops, and a `pump()` or
   * `reset()` not yet sent rejects. The stop goes after the one frame already in flight at most.
   * Idempotent: a second call returns the first call's promise. If the stop fails, the promise
   * rejects and the handle is open again, its timer running, so `close()` may be called again.
   */
  close(): Promise<void>;
}

/** The read cadence of a managed loudness handle, in milliseconds. */
export const LOUDNESS_READ_CADENCE_MS = 100;

/** The one capability flag bit of `LOUDNESS` (`0x0014`). @internal */
export const CAPABILITY_LOUDNESS = 1n << 17n;

/**
 * What a `LoudnessOwner` needs from an engine: one `LOUDNESS` frame through the engine's control
 * entry, the capability flags and, in the browser, a timer. @internal
 */
export interface LoudnessTransport {
  send(operation: LoudnessFrameOperation): Promise<LoudnessFrameReply>;
  /** The `CAPABILITIES_GET` flags word. */
  capabilityFlags(): Promise<bigint>;
  readonly scheduler?: ObservationSubscriptionScheduler;
}

/** The public reading of a reply's state: absent wire fields are absent keys. @internal */
export function loudnessReadingOf(value: LoudnessFrameState): LoudnessReading {
  return Object.freeze({
    ...(value.momentaryLufs === undefined ? {} : { momentary: value.momentaryLufs }),
    ...(value.shortTermLufs === undefined ? {} : { shortTerm: value.shortTermLufs }),
    ...(value.integratedLufs === undefined ? {} : { integrated: value.integratedLufs }),
    ...(value.loudnessRangeLu === undefined ? {} : { range: value.loudnessRangeLu }),
    ...(value.momentaryMaxLufs === undefined ? {} : { maxMomentary: value.momentaryMaxLufs }),
    ...(value.shortTermMaxLufs === undefined ? {} : { maxShortTerm: value.shortTermMaxLufs }),
    ...(value.truePeakDbtp === undefined ? {} : { truePeak: value.truePeakDbtp }),
    ...(value.hopTruePeakDbtp === undefined ? {} : { truePeakLastHop: value.hopTruePeakDbtp }),
    ...(value.integratingSince === undefined ? {} : { integratingSince: value.integratingSince }),
    hopsDropped: value.hopsDropped,
    observedSample: value.observedSample,
    armed: value.armed,
  });
}

function sameReading(a: LoudnessReading | undefined, b: LoudnessReading): boolean {
  if (a === undefined) return false;
  const keys = Object.keys(b) as (keyof LoudnessReading)[];
  if (Object.keys(a).length !== keys.length) return false;
  return keys.every((key) => {
    const left = a[key];
    const right = b[key];
    if (Array.isArray(left) && Array.isArray(right)) return left[0] === right[0] && left[1] === right[1];
    return left === right;
  });
}

function closedError(): MisoUsageError {
  return new MisoUsageError("the loudness subscription is closed");
}

function capabilityRefusal(): MisoEngineError {
  return new MisoEngineError("the engine cannot serve LOUDNESS: capability flag bit 17 is clear", {
    phase: "asset",
    code: "unsupported",
    result: constantValue("resultCodes", "unsupported"),
    diagnostics: [{ code: "sdk.asset.capability", path: "CAPABILITIES_GET.flags.loudness" }],
  });
}

type HandleState = "open" | "closing" | "closed";

class LoudnessHandle implements LoudnessSubscription {
  readonly #owner: LoudnessOwner;
  readonly #callback: ((notification: LoudnessSubscriptionNotification) => void) | undefined;
  readonly #onError: ((error: unknown) => void) | undefined;
  #latest: LoudnessReading | undefined;
  #state: HandleState = "open";
  #closing: Promise<void> | undefined;
  #timer: unknown;
  /** Whether a timed read is in flight: a tick while one is skips, so a stuck host holds one. */
  #timedRead = false;

  constructor(
    owner: LoudnessOwner,
    callback: ((notification: LoudnessSubscriptionNotification) => void) | undefined,
    onError: ((error: unknown) => void) | undefined,
  ) {
    this.#owner = owner;
    this.#callback = callback;
    this.#onError = onError;
  }

  get handle(): LoudnessSubscription { return this; }

  /** `open` until `close()` is called, `closing` until its stop lands, then `closed`. */
  get state(): HandleState { return this.#state; }

  readLatest(): LoudnessReading | undefined { return this.#latest; }

  pump(): Promise<LoudnessReading> {
    if (this.#state !== "open") return Promise.reject(closedError());
    return this.#owner.command(this, "read");
  }

  reset(): Promise<void> {
    if (this.#state !== "open") return Promise.reject(closedError());
    return this.#owner.command(this, "reset").then(() => undefined);
  }

  close(): Promise<void> {
    if (this.#closing !== undefined) return this.#closing;
    if (this.#state === "closed") return Promise.resolve();
    this.#state = "closing";
    this.#stopTimer();
    const pending = this.#owner.command(this, "stop").then(() => {
      this.#state = "closed";
      this.#owner.forget(this);
    }, (error: unknown) => {
      // A refused or failed stop leaves the meter as it was: the handle is open again, reads on
      // its timer, and `close()` may be called again. An engine that ended meanwhile closed it.
      this.#closing = undefined;
      if (this.#state === "closing") {
        this.#state = "open";
        this.startTimer();
      }
      throw error;
    });
    this.#closing = pending;
    return pending;
  }

  /** Deliver a reply's reading, and tell the listener when it changed. */
  deliver(reading: LoudnessReading): void {
    const changed = !sameReading(this.#latest, reading);
    this.#latest = reading;
    if (!changed || this.#callback === undefined) return;
    try { this.#callback(Object.freeze({ handle: this, reading })); } catch { /* a listener cannot break the handle */ }
  }

  startTimer(): void {
    const scheduler = this.#owner.scheduler;
    if (scheduler === undefined || this.#timer !== undefined) return;
    this.#timer = scheduler.setInterval(() => {
      if (this.#state !== "open" || this.#timedRead) return;
      this.#timedRead = true;
      this.#owner.command(this, "read").then(() => undefined, (error: unknown) => {
        if (this.#state !== "open") return;
        try { this.#onError?.(error); } catch { /* a listener cannot break the handle */ }
      }).finally(() => { this.#timedRead = false; });
    }, LOUDNESS_READ_CADENCE_MS);
  }

  #stopTimer(): void {
    if (this.#timer !== undefined) this.#owner.scheduler?.clearInterval(this.#timer);
    this.#timer = undefined;
  }

  /** End the handle without a frame: the engine it reads was disposed or rebooted. */
  invalidate(): void {
    this.#stopTimer();
    this.#state = "closed";
  }
}

/**
 * The one loudness owner of an engine lifetime (#2115 S3): it starts the core's meter for a new
 * handle, sends each handle's frames one at a time in call order, and ends every handle when the
 * engine is disposed or reboots. The core, not the SDK, refuses a second start while the meter is
 * armed (`loudness.active`). @internal
 */
export class LoudnessOwner {
  readonly #transport: LoudnessTransport;
  readonly #handles = new Set<LoudnessHandle>();
  #chain: Promise<unknown> = Promise.resolve();
  #ended = false;

  constructor(transport: LoudnessTransport) {
    this.#transport = transport;
  }

  get scheduler(): ObservationSubscriptionScheduler | undefined { return this.#transport.scheduler; }

  /** Start the meter and return its managed handle. */
  subscribe(request: LoudnessSubscriptionRequest | undefined): Promise<LoudnessSubscription> {
    if (request !== undefined && (request === null || typeof request !== "object")) {
      return Promise.reject(new MisoUsageError("a loudness subscription request must be an object"));
    }
    const callback = request?.onUpdate;
    if (callback !== undefined && typeof callback !== "function") {
      return Promise.reject(new MisoUsageError("loudness onUpdate must be a function"));
    }
    const onError = request?.onError;
    if (onError !== undefined && typeof onError !== "function") {
      return Promise.reject(new MisoUsageError("loudness onError must be a function"));
    }
    return this.#enqueue(async () => {
      if (this.#ended) throw new MisoUsageError("the engine was disposed");
      if ((await this.#transport.capabilityFlags() & CAPABILITY_LOUDNESS) === 0n) throw capabilityRefusal();
      let reply: LoudnessFrameReply;
      try {
        reply = await this.#transport.send("start");
      } catch (error) {
        // The start may still reach the core after the sender gave up on it (a deadline, an
        // abort, a lost reply) and arm the meter with no handle, so every later start would be
        // refused `loudness.active`. A quiet stop goes behind it, as the spectrum stream's does.
        void this.#enqueue(() => this.#transport.send("stop")).catch(() => undefined);
        throw error;
      }
      if (!reply.ok) throw spectrumRefusalError("the engine refused the loudness start", reply);
      const handle = new LoudnessHandle(this, callback, onError);
      this.#handles.add(handle);
      handle.startTimer();
      return handle;
    });
  }

  /**
   * Send `operation` for `handle` after every earlier frame, and deliver its reading. A read or a
   * reset whose turn comes once the handle is closing is not sent: it rejects, so a stop waits
   * behind at most the one frame already in flight.
   */
  command(handle: LoudnessHandle, operation: Exclude<LoudnessFrameOperation, "start">): Promise<LoudnessReading> {
    return this.#enqueue(async () => {
      const allowed = operation === "stop" ? handle.state !== "closed" : handle.state === "open";
      if (!allowed || !this.#handles.has(handle)) throw closedError();
      const reply = await this.#transport.send(operation);
      if (!reply.ok) throw spectrumRefusalError(`the engine refused the loudness ${operation}`, reply);
      const reading = loudnessReadingOf(reply.value);
      // The engine may have ended the handle while the frame was in flight.
      if (handle.state !== "closed") handle.deliver(reading);
      return reading;
    });
  }

  forget(handle: LoudnessHandle): void {
    this.#handles.delete(handle);
  }

  /** End every handle without a frame: the engine is disposed (`disposed`) or rebooted. */
  invalidate(disposed = false): void {
    for (const handle of this.#handles) handle.invalidate();
    this.#handles.clear();
    if (disposed) this.#ended = true;
  }

  #enqueue<T>(work: () => Promise<T>): Promise<T> {
    const next = this.#chain.then(work, work);
    this.#chain = next.catch(() => undefined);
    return next;
  }
}
