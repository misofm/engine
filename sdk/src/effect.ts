/**
 * Optional Effect integration for ownership and asynchronous SDK boundaries.
 *
 * The engine's render, PCM, wire, and edit-construction paths deliberately do not import Effect.
 * This module is a separate package entry so ordinary Promise consumers retain a zero-dependency
 * runtime while Effect applications get typed failure and guaranteed release.
 */

import { Effect } from "effect";
import type { Scope } from "effect";

import type { CommandReport } from "./core/boundary.ts";
import type { EngineConsole } from "./core/console.ts";
import type { LaneEdit } from "./core/writer.ts";
import {
  createEngine,
  type BrowserEngine,
  type CreateEngineOptions,
} from "./browser/engine.ts";
import {
  createOfflineEngine,
  type OfflineEngine,
  type OfflineEngineOptions,
  type SessionDocument,
} from "./headless/engine.ts";

export type EngineEffectOperation = "openOffline" | "openBrowser" | "submitConsole";

/** An expected Promise rejection promoted into Effect's typed error channel. */
export class EngineEffectError extends Error {
  readonly _tag = "EngineEffectError";
  readonly operation: EngineEffectOperation;
  override readonly cause: unknown;

  constructor(operation: EngineEffectOperation, cause: unknown) {
    const detail = cause instanceof Error ? cause.message : String(cause);
    super(`${operation} failed: ${detail}`);
    this.name = "EngineEffectError";
    this.operation = operation;
    this.cause = cause;
  }
}

function fromPromise<A>(
  operation: EngineEffectOperation,
  run: () => PromiseLike<A>,
): Effect.Effect<A, EngineEffectError> {
  return Effect.tryPromise({
    try: run,
    catch: (cause) => new EngineEffectError(operation, cause),
  });
}

/** Open a caller-owned headless engine. Prefer `scopedOfflineEngine` inside an Effect scope. */
export function openOfflineEngine(
  document: SessionDocument,
  options: OfflineEngineOptions = {},
): Effect.Effect<OfflineEngine, EngineEffectError> {
  return fromPromise("openOffline", () => createOfflineEngine(document, options));
}

/** Open a headless engine whose Wasm instance is disposed when the surrounding scope closes. */
export function scopedOfflineEngine(
  document: SessionDocument,
  options: OfflineEngineOptions = {},
): Effect.Effect<OfflineEngine, EngineEffectError, Scope.Scope> {
  return Effect.acquireRelease(
    openOfflineEngine(document, options),
    (engine) => Effect.sync(() => engine.dispose()),
  );
}

/** Open a caller-owned browser engine. Prefer `scopedBrowserEngine` inside an Effect scope. */
export function openBrowserEngine(
  options: CreateEngineOptions,
): Effect.Effect<BrowserEngine, EngineEffectError> {
  return fromPromise("openBrowser", () => createEngine(options));
}

/** Open a browser engine whose worklet host and AudioContext close with its scope. */
export function scopedBrowserEngine(
  options: CreateEngineOptions,
): Effect.Effect<BrowserEngine, EngineEffectError, Scope.Scope> {
  return Effect.acquireRelease(
    openBrowserEngine(options),
    (engine) => Effect.promise(() => engine.close()),
  );
}

/** Submit one semantic transaction without reclassifying a typed engine refusal as an error. */
export function submitConsole(
  console: EngineConsole,
  ...edits: readonly LaneEdit[]
): Effect.Effect<CommandReport, EngineEffectError> {
  return fromPromise("submitConsole", () => console.submit(...edits));
}
