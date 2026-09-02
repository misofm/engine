/** Issue #323: stable Effect ownership and error semantics over the production SDK. */

import assert from "node:assert/strict";
import { before, describe, test } from "node:test";

import { Effect, Fiber } from "effect";

import { MisoEngineAsset } from "../src/core/asset.ts";
import { EngineConsole } from "../src/core/console.ts";
import { MisoEngineError } from "../src/core/errors.ts";
import { scratchBootInWorker } from "../src/browser/engine.ts";
import {
  EngineEffectError,
  openBrowserEngine,
  openOfflineEngine,
  scopedBrowserEngine,
  scopedOfflineEngine,
  submitConsole,
} from "../src/effect.ts";
import { moduleBytes, sessionDocument } from "./support.mjs";

let asset;
let bytes;

before(async () => {
  bytes = await moduleBytes();
  asset = await MisoEngineAsset.load(bytes);
});

describe("issue 323 -- optional Effect lifecycle adapter", () => {
  test("a scoped live-Wasm engine releases after success and typed failure", async () => {
    let successful;
    const sampleRate = await Effect.runPromise(Effect.scoped(Effect.gen(function* () {
      successful = yield* scopedOfflineEngine(sessionDocument(), { asset });
      return successful.shape().sampleRateHz;
    })));
    assert.equal(sampleRate, 48_000);
    assert.equal(successful.state(), "disposed");

    let failed;
    const exit = await Effect.runPromiseExit(Effect.scoped(Effect.gen(function* () {
      failed = yield* scopedOfflineEngine(sessionDocument(), { asset });
      return yield* Effect.fail("intentional");
    })));
    assert.equal(exit._tag, "Failure");
    assert.equal(failed.state(), "disposed");
  });

  test("scope interruption releases a live-Wasm engine", async () => {
    let engine;
    let acquired;
    const ready = new Promise((resolve) => { acquired = resolve; });
    const fiber = Effect.runFork(Effect.scoped(Effect.gen(function* () {
      engine = yield* scopedOfflineEngine(sessionDocument(), { asset });
      acquired();
      return yield* Effect.never;
    })));
    await ready;
    await Effect.runPromise(Fiber.interrupt(fiber));
    assert.equal(engine.state(), "disposed");
  });

  test("acquisition rejection retains its typed operation and original engine cause", async () => {
    const failure = await Effect.runPromise(Effect.flip(openOfflineEngine("not Session V1", { asset })));
    assert.ok(failure instanceof EngineEffectError);
    assert.equal(failure.operation, "openOffline");
    assert.ok(failure.cause instanceof MisoEngineError);

    const browserCause = new Error("scratch worker failed");
    const browserFailure = await Effect.runPromise(Effect.flip(openBrowserEngine({
      document: sessionDocument(),
      scratchBoot: async () => { throw browserCause; },
      createContext: () => { throw new Error("must not be reached"); },
      createHost: async () => { throw new Error("must not be reached"); },
    })));
    assert.ok(browserFailure instanceof EngineEffectError);
    assert.equal(browserFailure.operation, "openBrowser");
    assert.equal(browserFailure.cause, browserCause);
  });

  test("an engine refusal remains success, while a transport rejection is typed", async () => {
    const engine = await Effect.runPromise(openOfflineEngine(sessionDocument(), {
      asset,
      console: { commandQueueRecords: 8 },
    }));
    try {
      const console = engine.console();
      const report = await Effect.runPromise(submitConsole(
        console,
        console.edit.track("t").effect("simd1", 99, "miso.compressor").bypass(true),
      ));
      assert.equal(report.ok, false);
      assert.equal(report.admitted, 0);
      assert.equal(report.reasonName, "unknownEffect");
    } finally {
      engine.dispose();
    }

    const cause = new Error("transport never acknowledged");
    let rejectTransport;
    const transport = new Promise((_resolve, reject) => { rejectTransport = reject; });
    const rejecting = new EngineConsole(
      { tracks: ["t"], sources: [], metersAttached: false },
      () => transport,
    );
    let settled = false;
    const pendingFailure = Effect.runPromise(Effect.flip(submitConsole(
      rejecting,
      rejecting.edit.track("t").faderDb(-3),
    ))).then((failure) => {
      settled = true;
      return failure;
    });
    await new Promise((resolve) => setImmediate(resolve));
    assert.equal(settled, false, "the Effect cannot answer before the transport settles");
    rejectTransport(cause);
    const failure = await pendingFailure;
    assert.ok(failure instanceof EngineEffectError);
    assert.equal(failure.operation, "submitConsole");
    assert.equal(failure.cause, cause);
  });

  test("a scoped browser engine closes host then context", async () => {
    const events = [];
    const options = {
      document: sessionDocument(),
      simd128ModuleUrl: "/wasm",
      workletModuleUrl: "/worklet",
      scratchBoot: ({ document, options: boot }) =>
        scratchBootInWorker({ moduleBytes: bytes, document, options: boot }),
      createContext: (request) => ({
        sampleRate: request.sampleRate,
        renderQuantumSize: 128,
        state: "running",
        close: async () => { events.push("context"); },
        audioWorklet: { addModule: async () => {} },
      }),
      createHost: async () => ({
        dispose: async () => { events.push("host"); },
      }),
    };

    await Effect.runPromise(Effect.scoped(Effect.as(scopedBrowserEngine(options), undefined)));
    assert.deepEqual(events, ["host", "context"]);
  });
});
