// Issue #1571 gates 2, 3 and 5: the one call sequence the browser row runs on the browser SDK
// engine (in the page, `sdk-response-entry.ts`) and on the headless SDK engine (in Node, `run.mjs`),
// so the two halves cannot drift apart.

/** The quanta rendered after the telemetry, the fader edit and the transport start. */
export const PROTOCOL_BLOCKS = 4;

/** Meter handle 1, `liveValuesSuperseded` and the diagnostics at `"info"`, each every block. */
export const PROTOCOL_TELEMETRY = Object.freeze({
  meterHandles: [1],
  meterPeriodBlocks: 1,
  counterIds: ["liveValuesSuperseded"],
  counterPeriodBlocks: 1,
  diagnosticsEnabled: true,
  minimumSeverity: "info",
});

/** A protocol value in JSON: bigints as decimal strings, bytes as hex. */
export function protocolPlain(value) {
  return JSON.parse(JSON.stringify(value, (_key, item) => {
    if (typeof item === "bigint") return item.toString();
    if (item instanceof Uint8Array) return Array.from(item, (byte) => byte.toString(16).padStart(2, "0")).join("");
    return item;
  }));
}

/**
 * Run the sequence on `engine` (either SDK engine): subscribe to `onEvent`, read the capabilities,
 * configure the telemetry, make one fader edit (`edit`), read the transport, start it, read it
 * again, read the counter and the retained diagnostics, ask for a refused diagnostics page, render
 * (`render`), and read the transport twice more. The browser reads its events once per message it
 * handles, after the reply: the first of these calls reads the ones the render queued, and the
 * second one's reply arrives after them. Returns the answers and the events,
 * in JSON.
 */
export async function protocolCalls(engine, edit, render) {
  const events = [];
  const stop = engine.onEvent((event) => events.push(event));
  try {
    const capabilities = await engine.capabilities();
    const telemetry = await engine.configureTelemetry(PROTOCOL_TELEMETRY);
    const edited = await edit();
    if (!edited.ok) throw new Error("the protocol row's fader edit was refused");
    const stopped = await engine.transport();
    const playing = await engine.setTransport({ state: "playing" });
    const transport = await engine.transport();
    const counters = await engine.counters(["liveValuesSuperseded"]);
    const diagnostics = await engine.diagnostics();
    let refusal;
    try {
      await engine.diagnostics({ limit: 0 });
    } catch (error) {
      refusal = { name: error?.name, status: error?.status, code: error?.code };
    }
    await render();
    const after = await engine.transport();
    // The worklet posts a message's events after its reply, on the same port: the reply to one more
    // call arrives after the events the call above read.
    await engine.transport();
    return protocolPlain({
      editRevision: edited.revision,
      answers: { capabilities, telemetry, stopped, playing, transport, counters, diagnostics, refusal },
      after,
      events,
    });
  } finally {
    stop();
  }
}
