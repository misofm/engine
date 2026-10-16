/**
 * #1564 piece A (decision 17, C4/C5): `engine.controls()`, `engine.controlState()` and `edit.set`.
 *
 * The list is the core's. The reference is `hosts/host-web/tests/fixtures/control-list.json`, which the
 * Rust test `control_list_fixture` writes from the native `controls::list` of the #1560 parity
 * session; every eval here boots that session on the shipped module and judges what the SDK returns
 * against the core, never against a table the SDK holds.
 */

import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { before, describe, test } from "node:test";

import { createEngine } from "../src/browser/index.ts";
import { createBrowserControlListTransport, createBrowserLiveControls, createBrowserStripOrder } from "../src/browser/live-controls.ts";
import { MisoEngineAsset } from "../src/core/asset.ts";
import { decodeControlReply, encodeControlTransaction } from "../src/core/control-frame.ts";
import {
  CONTROL_LIST_RESTARTS,
  decodeControlListPage,
  readControlList,
  readControlState,
} from "../src/core/control-list.ts";
import { MisoControlRefusal, MisoEngineError, MisoUsageError } from "../src/core/errors.ts";
import { createOfflineEngine } from "../src/headless/engine.ts";
import { engineHost, moduleBytes, ramp } from "./support.mjs";

const FIXTURE = JSON.parse(readFileSync(
  new URL("../../hosts/host-web/tests/fixtures/control-list.json", import.meta.url),
  "utf8",
));
const SESSION = JSON.parse(FIXTURE.session);
const OPTIONS = { liveControls: { observationTaps: 1 } };

let asset;

before(async () => {
  asset = await MisoEngineAsset.load(await moduleBytes());
});

async function boot() {
  return createOfflineEngine(FIXTURE.session, { asset, ...OPTIONS });
}

function bits(value) {
  const cell = new Float32Array(1);
  cell[0] = value;
  return new Uint32Array(cell.buffer)[0].toString(16).padStart(8, "0");
}

/** What the fixture records of one row, read from the SDK's row. */
function projected(entry) {
  return {
    owner: entry.address.owner,
    section: entry.address.section,
    control: entry.address.control,
    lane: entry.address.lane,
    name: entry.name,
    class: entry.class,
    kind: entry.kind,
    unit: entry.unit,
    scope: entry.scope,
    family: entry.family,
    active: entry.active,
    automatable: entry.automatable,
    minimumBits: bits(entry.minimum),
    maximumBits: bits(entry.maximum),
    defaultBits: bits(entry.default),
    valueBits: bits(entry.value),
    choices: entry.choices.length,
    automated: entry.automated,
  };
}

function owned(entry, kind, id) {
  return entry.address.owner.kind === kind && (id === undefined || entry.address.owner.id === id);
}

function find(list, predicate, what) {
  const entry = list.controls.find(predicate);
  assert.ok(entry !== undefined, `the list has ${what}`);
  return entry;
}

function feed(engine, block) {
  const shape = engine.shape();
  for (const source of shape.sources) {
    engine.submitSource({
      sourceId: source.id,
      generation: 1n,
      startFrame: BigInt(block * shape.quantumFrames),
      planes: Array.from({ length: source.channels }, (_unused, channel) =>
        ramp(shape.quantumFrames, 71 + block * 16 + channel)),
      endOfRegion: false,
    });
  }
}

function renderBlocks(engine, from, count) {
  const out = [];
  for (let block = from; block < from + count; block += 1) {
    feed(engine, block);
    const { left, right } = engine.render();
    out.push({ left: left.slice(), right: right.slice() });
  }
  return out;
}

/** Commit `value` for t3's `matrix_mode`: the descriptors of its matrix and pan rows change. */
function flipMatrixMode(engine, value) {
  const frame = encodeControlTransaction({
    requestId: engine.controlTransactions().requestIds.next(),
    expectedRevision: engine.controlRevision(),
    edits: [{
      address: {
        owner: { kind: "strip", id: "t3" },
        section: { kind: "own" },
        control: 1,
        lane: "both",
      },
      value,
    }],
  });
  const reply = decodeControlReply(engine.controlFrame(frame));
  assert.equal(reply.status, "ok", "the structure-changing edit is committed");
}

describe("#1564A gate 1 -- the list is the core's", () => {
  test("headless: engine.controls() yields the rows of the native controls::list, in order", async () => {
    const engine = await boot();
    try {
      const list = await engine.controls();
      assert.equal(list.controls.length, FIXTURE.rows.length, "no row is dropped or added");
      // #1650 D4: the fixture rides `b`'s right fader, so one row is automated and its left
      // neighbour is not; the projection compares the flag on every row.
      assert.deepEqual(
        FIXTURE.rows.filter((row) => row.automated).map((row) => [row.owner.id, row.name, row.lane]),
        [["b", "fader_db", "right"]],
        "the fixture has one automated row",
      );
      assert.deepEqual(list.controls.map(projected), FIXTURE.rows);
      const handles = list.controls.map((entry) => entry.handle);
      assert.ok(handles.every((handle, at) => handle > 0 && (at === 0 || handle > handles[at - 1])),
        "handles are nonzero and ascend");
      assert.equal(typeof list.revision, "bigint");
      assert.equal(typeof list.structureEpoch, "bigint");
      assert.equal(list.revision, engine.controlRevision());
    } finally {
      engine.dispose();
    }
  });

  test("the headers open every owner of the session, in the core's order", async () => {
    const engine = await boot();
    try {
      const list = await engine.controls();
      const owners = list.headers.filter((header) => header.row === "owner");
      const ids = (kind) => owners.filter((header) => header.kind === kind).map((header) => header.address.owner.id);
      // The strips come in `model.strips()` order: the tracks, then the submixes.
      assert.deepEqual(ids("track"), SESSION.tracks.map((track) => track.id));
      assert.deepEqual(ids("submix"), SESSION.submixes.map((submix) => submix.id));
      assert.deepEqual(ids("route"), SESSION.routes.map((route) => route.id).sort());
      assert.deepEqual(ids("vca"), SESSION.vcas.map((vca) => vca.id));
      assert.equal(owners.filter((header) => header.kind === "session").length, 1);
      assert.deepEqual(owners.map((header) => header.kind).filter((kind) => kind === "track" || kind === "submix"),
        ["track", "track", "track", "submix"], "tracks come before submixes");
      const route = owners.find((header) => header.kind === "route" && header.address.owner.id === "t1-b");
      assert.deepEqual(route.route, { source: "t1", destination: "b", tap: "pre_fader" });
      // A section header names its effect; a control row's owner has a header before it.
      const section = list.headers.find((header) => header.row === "section"
        && header.address.section.kind === "inserts" && header.address.section.effect === "comp");
      assert.equal(section.effect, "miso.compressor");
      const seen = new Set();
      for (const header of list.headers) {
        if (header.row === "owner") seen.add(JSON.stringify(header.address.owner));
      }
      for (const entry of list.controls) {
        assert.ok(seen.has(JSON.stringify(entry.address.owner)), "every control belongs to a listed owner");
      }
    } finally {
      engine.dispose();
    }
  });

  test("paging with limit 1 and with 256 gives the same list", async () => {
    const engine = await boot();
    try {
      const whole = await engine.controls({ pageLimit: 256 });
      const single = await engine.controls({ pageLimit: 1 });
      const odd = await engine.controls({ pageLimit: 7 });
      assert.deepEqual(single, whole);
      assert.deepEqual(odd, whole);
      assert.deepEqual(await engine.controls(), whole, "256 is the default");
    } finally {
      engine.dispose();
    }
  });

  test("an owner filter lists that owner's rows only, and the state read covers the same rows", async () => {
    const engine = await boot();
    try {
      const owner = { kind: "strip", id: "t2" };
      const filtered = await engine.controls({ owner });
      const whole = await engine.controls();
      assert.deepEqual(filtered.controls.map(projected),
        whole.controls.filter((entry) => owned(entry, "strip", "t2")).map(projected));
      assert.deepEqual(filtered.controls.map((entry) => entry.handle),
        whole.controls.filter((entry) => owned(entry, "strip", "t2")).map((entry) => entry.handle),
        "a handle is the row's in the whole structure");
      const state = await engine.controlState(filtered);
      assert.deepEqual([...state.values.keys()], filtered.controls.map((entry) => entry.handle));
      const session = await engine.controls({ owner: { kind: "session" } });
      assert.deepEqual(session.controls.map((entry) => entry.name), ["fader_ms", "mute_ms", "pan_ms"]);
    } finally {
      engine.dispose();
    }
  });

  test("a row's descriptor is decoded from the wire: ladder, choices, ramp, reason and reopen", async () => {
    const engine = await boot();
    try {
      const list = await engine.controls();
      const mode = find(list, (e) => owned(e, "strip", "t1") && e.name === "matrix_mode", "t1 matrix_mode");
      assert.equal(mode.kind, "enumeration");
      assert.deepEqual(mode.choices, [{ value: 0, label: "pan" }, { value: 1, label: "matrix" }]);
      assert.equal(mode.rampRow, "pan");
      assert.equal(mode.class, "live");
      assert.equal(mode.reason, undefined);
      const delay = find(list, (e) => owned(e, "strip", "t1") && e.name === "delay_samples", "delay_samples");
      assert.equal(delay.class, "rebuildOnly");
      assert.equal(delay.reason, "newMemory");
      assert.equal(typeof delay.reopen, "string");
      assert.equal(delay.unit, "samples");
      const fader = find(list, (e) => owned(e, "strip", "t1") && e.name === "fader_db", "fader_db");
      assert.equal(fader.unit, "db");
      assert.ok(Array.isArray(fader.ladder));
      assert.equal(fader.rampRow, "fader");
      assert.equal(fader.value, SESSION.tracks[0].fader.left_db);
      const gate = find(list, (e) => owned(e, "strip", "t2") && e.address.section.kind === "inserts"
        && e.address.section.effect === "gate" && e.name === "attack_ms", "the gate's attack_ms");
      assert.equal(gate.class, "rebuildOnly");
    } finally {
      engine.dispose();
    }
  });

  test("the browser transport lists the same rows as the headless one, through a real BrowserEngine", async () => {
    // Two cores of one session: the browser reads one through the host's `control` message, the
    // headless engine reads the other in process.
    const engine = await boot();
    const served = await boot();
    const closed = [];
    const browser = await createEngine({
      document: new Uint8Array([1]),
      scratchBoot: async () => served.shape(),
      createContext: () => ({
        sampleRate: 48000, renderQuantumSize: 128, state: "running",
        audioWorklet: { async addModule() {} }, async close() { closed.push("context"); },
      }),
      createHost: async () => ({ ...engineHost(served), async dispose() { closed.push("host"); } }),
    });
    try {
      const headless = await engine.controls();
      const list = await browser.controls();
      assert.deepEqual(list, headless);
      assert.deepEqual(list.controls.map(projected), FIXTURE.rows);
      const paged = await browser.controls({ pageLimit: 5 });
      assert.deepEqual(paged, headless);
      const state = await browser.controlState(list);
      assert.deepEqual(state, await engine.controlState(headless));
    } finally {
      await browser.close();
      engine.dispose();
      served.dispose();
    }
  });
});

describe("#1564A D2 -- engine.controlState()", () => {
  test("it reads each control row's committed value by handle, and sees a committed edit", async () => {
    const engine = await boot();
    try {
      const list = await engine.controls();
      const before = await engine.controlState(list);
      assert.equal(before.structureEpoch, list.structureEpoch);
      assert.equal(before.values.size, list.controls.length);
      for (const entry of list.controls) {
        assert.equal(bits(before.values.get(entry.handle)), bits(entry.value), entry.name);
      }
      const fader = find(list, (e) => owned(e, "strip", "t1") && e.name === "fader_db" && e.address.lane === "left",
        "t1 left fader");
      const controls = engine.liveControls();
      const report = await controls.submit(controls.edit.set(fader.address, -7.5, { rampSamples: 0 }));
      assert.equal(report.ok, true);
      const after = await engine.controlState(list);
      assert.equal(after.values.get(fader.handle), -7.5);
      assert.ok(after.revision > before.revision, "the committed edit advanced the revision");
      assert.equal(before.values.get(fader.handle), fader.value, "an earlier read is unchanged");
      // The same handle in a fresh list has the new value, and nothing else moved but the derived rows.
      const relisted = await engine.controls();
      assert.equal(relisted.structureEpoch, list.structureEpoch, "a value edit keeps the structure");
      const settled = (rows) => rows.filter((e) => e.handle !== fader.handle && e.class !== "derived").map((e) => e.value);
      assert.deepEqual(settled(relisted.controls), settled(list.controls));
    } finally {
      engine.dispose();
    }
  });
});

describe("#1564A gate 2 -- a structure that changes between two pages", () => {
  test("a transaction that advances the structure epoch makes the read restart and return the new list", async () => {
    const engine = await boot();
    try {
      const first = await engine.controls();
      const ids = engine.controlTransactions().requestIds;
      let requests = 0;
      let flipped = false;
      const transport = {
        // The core takes request IDs in ascending order, so the structure changes before the second
        // request draws its ID: between page one and page two.
        requestIds: {
          next() {
            requests += 1;
            if (requests === 2) {
              flipMatrixMode(engine, 0);
              flipped = true;
            }
            return ids.next();
          },
        },
        send: (frame) => engine.controlFrame(frame),
      };
      const list = await readControlList(transport, { pageLimit: 40 });
      assert.ok(flipped, "the structure changed mid-read");
      assert.ok(list.structureEpoch > first.structureEpoch, "the epoch advanced");
      assert.ok(requests > 3, "the read restarted from the first page");
      assert.deepEqual(list, await engine.controls({ pageLimit: 40 }), "the result is the new structure whole");
      assert.deepEqual(
        list.controls.map((e) => e.handle),
        (await engine.controls()).controls.map((e) => e.handle),
        "no handle of the old structure is mixed in",
      );
      const matrix = find(list, (e) => owned(e, "strip", "t3") && e.name === "matrix_ll", "t3 matrix_ll");
      const was = find(first, (e) => owned(e, "strip", "t3") && e.name === "matrix_ll", "t3 matrix_ll");
      assert.equal(was.active, true);
      assert.equal(matrix.active, false, "the descriptors are the new structure's");
    } finally {
      engine.dispose();
    }
  });

  test("a transport that is stale on every continuation rejects after three restarts", async () => {
    const engine = await boot();
    try {
      const ids = engine.controlTransactions().requestIds;
      let firstPages = 0;
      let fresh = true;
      let mode = 0;
      const transport = {
        requestIds: {
          next() {
            if (fresh) {
              firstPages += 1;
            } else {
              mode = 1 - mode;
              flipMatrixMode(engine, mode);
            }
            return ids.next();
          },
        },
        send: (frame) => {
          const reply = engine.controlFrame(frame);
          fresh = !decodeControlListPage(reply).ok;
          return reply;
        },
      };
      await assert.rejects(readControlList(transport, { pageLimit: 40 }), (error) => {
        assert.ok(error instanceof MisoEngineError);
        assert.equal(error.code, "control.list.stale");
        assert.equal(error.diagnosticCode, "control.list.stale");
        return true;
      });
      assert.equal(firstPages, 1 + CONTROL_LIST_RESTARTS, "the first read and three restarts, no more");
    } finally {
      engine.dispose();
    }
  });

  test("a state read of a list from another structure rejects control.list.stale; listing again recovers", async () => {
    const engine = await boot();
    try {
      const list = await engine.controls();
      flipMatrixMode(engine, 0);
      await assert.rejects(engine.controlState(list), (error) => {
        assert.ok(error instanceof MisoEngineError);
        assert.equal(error.code, "control.list.stale");
        return true;
      });
      const again = await engine.controls();
      assert.ok(again.structureEpoch > list.structureEpoch);
      assert.equal((await engine.controlState(again)).values.size, again.controls.length);
    } finally {
      engine.dispose();
    }
  });

  test("a reply that makes no progress ends the read instead of looping", async () => {
    const engine = await boot();
    try {
      // A core that answered every request with the first page would never reach eof.
      let page;
      const stuck = {
        requestIds: engine.controlTransactions().requestIds,
        send: (frame) => {
          if (page === undefined) {
            page = engine.controlFrame(frame);
            return page;
          }
          const reply = page.slice();
          reply.set(frame.subarray(24, 32), 24);
          return reply;
        },
      };
      await assert.rejects(readControlList(stuck, { pageLimit: 3 }), (error) => {
        assert.ok(error instanceof MisoEngineError);
        assert.equal(error.code, "internal");
        return true;
      });
    } finally {
      engine.dispose();
    }
  });
});

describe("#1564A gate 4 -- edit.set reaches any row", () => {
  /**
   * `set` on the row renders as the typed builder does: two engines of the parity session, one
   * edited by the builder and one by `set`, hold the same values and render the same blocks.
   */
  async function sameAsBuilder({ builder, setter, plainEdits = true }) {
    const typed = await boot();
    const generic = await boot();
    try {
      const typedControls = typed.liveControls();
      const genericControls = generic.liveControls();
      const list = await generic.controls();
      const built = builder(typedControls.edit);
      const set = setter(genericControls.edit, list);
      if (plainEdits) assert.deepEqual(plain(set), plain(built), "the edit is the builder's");
      renderBlocks(typed, 0, 3);
      renderBlocks(generic, 0, 3);
      const [typedReport, genericReport] = [await typedControls.submit(built), await genericControls.submit(set)];
      assert.equal(genericReport.ok, true);
      assert.equal(genericReport.path, typedReport.path);
      const a = renderBlocks(typed, 3, 12);
      const b = renderBlocks(generic, 3, 12);
      assert.deepEqual(b, a, "the rendered blocks are bit-identical");
      assert.deepEqual(await generic.controls(), await typed.controls(), "the list holds the same values");
      assert.ok(a.some((block) => block.left.some((sample) => sample !== 0)), "the mix carries signal");
    } finally {
      typed.dispose();
      generic.dispose();
    }
  }

  function plain(set) {
    return set.edits.map((edit) => ({
      owner: edit.address.owner,
      section: edit.address.section,
      control: edit.address.control,
      lane: edit.address.lane,
      value: edit.value,
      ...(edit.rampSamples === undefined ? {} : { rampSamples: edit.rampSamples }),
    }));
  }

  const strip = (id) => ({ kind: "strip", id });
  const BUILTINS = { kind: "builtins" };
  const OWN = { kind: "own" };

  test("a strip builtin row: a fader, by name", () => sameAsBuilder({
    builder: (e) => e.track("t1").faderDb(-6, { smoothingSamples: 0 }),
    setter: (e) => e.set(
      { owner: strip("t1"), section: BUILTINS, control: "fader_db", lane: "both" },
      -6,
      { rampSamples: 0 },
    ),
  }));

  test("a strip builtin row on one lane: a mute", () => sameAsBuilder({
    builder: (e) => e.track("t2").mute(true, { channel: "left", smoothingSamples: 0 }),
    setter: (e) => e.set(
      { owner: strip("t2"), section: BUILTINS, control: "mute", lane: "left" },
      1,
      { rampSamples: 0 },
    ),
  }));

  test("a strip builtin row with a fixed ramp: a high-pass cutoff", () => sameAsBuilder({
    builder: (e) => e.track("t1").hpfHz(120),
    setter: (e) => e.set({ owner: strip("t1"), section: BUILTINS, control: "hpf_hz", lane: "both" }, 120),
  }));

  test("a strip's own row: solo is a monitor edit", async () => {
    const engine = await boot();
    try {
      const controls = engine.liveControls();
      const set = controls.edit.set({ owner: strip("t1"), section: OWN, control: "solo", lane: "both" }, 1);
      assert.deepEqual(plain(set), plain(controls.edit.track("t1").solo(true)));
      const report = await controls.submit(set);
      assert.equal(report.path, "monitor");
    } finally {
      engine.dispose();
    }
  });

  test("a strip's own row that changes the structure: matrix_mode", async () => {
    const engine = await boot();
    try {
      const before = await engine.controls();
      const controls = engine.liveControls();
      const report = await controls.submit(
        controls.edit.set({ owner: strip("t3"), section: OWN, control: "matrix_mode", lane: "both" }, 0),
      );
      assert.equal(report.path, "live");
      const after = await engine.controls();
      assert.ok(after.structureEpoch > before.structureEpoch);
      assert.equal(find(after, (e) => owned(e, "strip", "t3") && e.name === "matrix_mode", "mode").value, 0);
    } finally {
      engine.dispose();
    }
  });

  test("a route's own row: a send's gain", () => sameAsBuilder({
    builder: (e) => e.route("t1-b").gainDb(-12, { smoothingSamples: 0 }),
    setter: (e) => e.set(
      { owner: { kind: "route", id: "t1-b" }, section: OWN, control: "gain_db", lane: "both" },
      -12,
      { rampSamples: 0 },
    ),
  }));

  test("a VCA's own row: its offset on one lane", () => sameAsBuilder({
    builder: (e) => e.vca("v").faderDb(-3, { channel: "right", smoothingSamples: 0 }),
    setter: (e) => e.set(
      { owner: { kind: "vca", id: "v" }, section: OWN, control: "offset_db", lane: "right" },
      -3,
      { rampSamples: 0 },
    ),
  }));

  test("the session's own row: a ramp length is a live setting that the list reads back", async () => {
    const engine = await boot();
    try {
      const before = await engine.controls();
      const row = find(before, (e) => e.address.owner.kind === "session" && e.name === "fader_ms", "fader_ms");
      assert.equal(row.class, "live");
      const controls = engine.liveControls();
      const value = row.value + 5;
      const report = await controls.submit(
        controls.edit.set({ owner: { kind: "session" }, section: OWN, control: "fader_ms", lane: "both" }, value),
      );
      assert.equal(report.ok, true);
      const after = await engine.controls();
      assert.equal(find(after, (e) => e.name === "fader_ms", "fader_ms").value, value);
      assert.ok(after.structureEpoch >= before.structureEpoch);
    } finally {
      engine.dispose();
    }
  });

  test("an insert's parameter, addressed by the numeric row a list entry carries", () => sameAsBuilder({
    builder: (e) => e.track("t1").insert("comp", "miso.compressor")
      .parameter("threshold", -30, { channel: "both" }),
    setter: (e, list) => {
      const row = find(list, (r) => owned(r, "strip", "t1") && r.address.section.kind === "inserts"
        && r.address.section.effect === "comp" && r.name === "threshold_db" && r.address.lane === "left", "threshold");
      return e.set({ ...row.address, lane: "both" }, -30);
    },
  }));

  test("a console slot's parameter, addressed by the numeric row a list entry carries", () => sameAsBuilder({
    builder: (e) => e.track("t2").console("pre-comp", "miso.compressor")
      .parameter("threshold", -24, { channel: "left" }),
    setter: (e, list) => {
      const row = find(list, (r) => owned(r, "strip", "t2") && r.address.section.kind === "console"
        && r.address.section.slot === "pre-comp" && r.name === "threshold_db" && r.address.lane === "left", "slot threshold");
      return e.set(row.address, -24);
    },
  }));

  test("an insert's bypass", () => sameAsBuilder({
    builder: (e) => e.track("t1").insert("eq", "miso.parametric-eq").bypass(true),
    setter: (e, list) => {
      const row = find(list, (r) => owned(r, "strip", "t1") && r.address.section.kind === "inserts"
        && r.address.section.effect === "eq" && r.name === "bypass", "eq bypass");
      return e.set(row.address, 1);
    },
  }));

  test("every live row of the list is accepted as the address of a write to its own value", async () => {
    const engine = await boot();
    try {
      const list = await engine.controls();
      const controls = engine.liveControls();
      let tried = 0;
      for (const row of list.controls) {
        if (row.class !== "live" || !row.active || row.scope !== "model" || row.family === "setting") continue;
        // The route into the output and a monitor row are other tests' business; a row's own value
        // is in its own domain, so the core accepts it as a no-op or a live write.
        const report = await controls.submit(controls.edit.set(row.address, row.value));
        assert.equal(report.ok, true, `${row.name} of ${JSON.stringify(row.address.owner)}`);
        tried += 1;
      }
      assert.ok(tried > 100, `many live rows were written (${tried})`);
    } finally {
      engine.dispose();
    }
  });

  /** The live controls and the list of one endpoint of `engine`: in process, or through the host. */
  async function endpoint(engine, transport) {
    if (transport === "headless") return { controls: engine.liveControls(), list: await engine.controls() };
    const host = engineHost(engine);
    return {
      controls: await createBrowserLiveControls(host),
      list: await readControlList(await createBrowserControlListTransport(host)),
    };
  }

  for (const transport of ["headless", "browser"]) {
    test(`${transport}: set on a derived row rejects control.read_only`, async () => {
      const engine = await boot();
      try {
        const { controls, list } = await endpoint(engine, transport);
        const derived = find(list, (e) => e.class === "derived" && owned(e, "strip", "t1"), "a derived row");
        await assert.rejects(controls.submit(controls.edit.set(derived.address, 0)), (error) => {
          assert.ok(error instanceof MisoControlRefusal, "the core refused the edit");
          assert.equal(error.code, "control.read_only");
          return true;
        });
      } finally {
        engine.dispose();
      }
    });

    test(`${transport}: set on a rebuild-only row is committed on the rebuild path`, async () => {
      const engine = await boot();
      try {
        const { controls, list } = await endpoint(engine, transport);
        const delay = find(list, (e) => owned(e, "strip", "t1") && e.name === "delay_samples" && e.address.lane === "left",
          "t1 delay_samples");
        assert.equal(delay.class, "rebuildOnly");
        const before = engine.controlRevision();
        const report = await controls.submit(controls.edit.set(delay.address, 64));
        assert.equal(report.ok, true);
        assert.equal(report.path, "rebuild", "the core prepares a replacement plan for a rebuild-only row");
        assert.equal(engine.controlRevision(), before + 1n, "the rebuild is committed");
      } finally {
        engine.dispose();
      }
    });
  }

  test("the SDK decides nothing the core decides: an out-of-domain value reaches the core and is refused there", async () => {
    const engine = await boot();
    try {
      const controls = engine.liveControls();
      const set = controls.edit.set({ owner: strip("t1"), section: BUILTINS, control: "fader_db", lane: "both" }, 1e9);
      assert.equal(set.edits[0].value, 1e9, "no SDK-side domain check");
      await assert.rejects(controls.submit(set), (error) => {
        assert.ok(error instanceof MisoControlRefusal);
        assert.equal(error.code, "control.domain");
        return true;
      });
      const unknown = controls.edit.set({ owner: strip("nobody"), section: BUILTINS, control: "fader_db", lane: "both" }, 0);
      await assert.rejects(controls.submit(unknown), (error) => error.code === "control.address.unknown");
    } finally {
      engine.dispose();
    }
  });

  test("set checks only encodability", async () => {
    const engine = await boot();
    try {
      const edit = engine.liveControls().edit;
      const address = { owner: strip("t1"), section: BUILTINS, control: "fader_db", lane: "both" };
      assert.throws(() => edit.set(address, 0, { rampSamples: -1 }), MisoUsageError);
      assert.throws(() => edit.set(address, 0, { rampSamples: 1.5 }), MisoUsageError);
      assert.throws(() => edit.set({ ...address, control: "no_such_row" }, 0), MisoUsageError);
      assert.throws(() => edit.set({ ...address, owner: { kind: "strip", id: 7 } }, 0), MisoUsageError);
      assert.throws(() => edit.set(address, "loud"), MisoUsageError);
      // A named row of an insert depends on the effect it runs. The SDK reads that from the core's
      // section header (#1562b-K), so a document-text boot resolves it; an insert the list does not
      // have cannot be resolved, and the message says to use the numeric row.
      const named = edit.set({
        owner: strip("t1"), section: { kind: "inserts", effect: "comp" }, control: "threshold_db", lane: "both",
      }, -20);
      const compressorRow = find(await engine.controls({ pageLimit: 50 }), (e) => owned(e, "strip", "t1")
        && e.address.section.kind === "inserts" && e.address.section.effect === "comp" && e.name === "threshold_db",
      "the comp insert's threshold_db");
      assert.equal(named.edits[0].address.control, compressorRow.address.control);
      assert.throws(
        () => edit.set({ owner: strip("t1"), section: { kind: "inserts", effect: "ghost" }, control: "threshold_db", lane: "both" }, -20),
        /numeric/,
      );
      assert.equal(edit.set(address, 0, { rampSamples: 0 }).edits[0].rampSamples, 0);
      assert.equal(edit.set(address, 0).edits[0].rampSamples, undefined, "no ramp is sent when none was given");
    } finally {
      engine.dispose();
    }
  });
});

describe("#1564A the request IDs of a list read come from the endpoint's one allocator", () => {
  test("a list read and a transaction on one engine never repeat a request ID", async () => {
    const engine = await boot();
    try {
      const seen = new Set();
      const transport = {
        requestIds: engine.controlTransactions().requestIds,
        send: (frame) => {
          const reply = engine.controlFrame(frame);
          const id = decodeControlListPage(reply).requestId;
          assert.ok(!seen.has(id), `request ${id} is new`);
          seen.add(id);
          return reply;
        },
      };
      await readControlList(transport, { pageLimit: 50 });
      const controls = engine.liveControls();
      const list = await engine.controls({ pageLimit: 50 });
      await controls.submit(controls.edit.track("t1").faderDb(-4));
      await readControlState(transport, list, { pageLimit: 50 }).catch(() => undefined);
      assert.ok(seen.size > 4);
    } finally {
      engine.dispose();
    }
  });

  test("the browser transport shares the transaction queue's allocator", async () => {
    const engine = await boot();
    try {
      const host = engineHost(engine);
      const transport = await createBrowserControlListTransport(host);
      const controls = await createBrowserLiveControls(host);
      assert.equal(transport.requestIds, controls.queue.requestIds);
    } finally {
      engine.dispose();
    }
  });

  test("a BrowserEngine interleaves list reads, strip-order probes and transactions on one allocator", async () => {
    // `engineHost` asserts that the IDs the browser path sends are strictly ascending, so a second
    // allocator on the endpoint (it would restart from 1) turns this red even though the fixture
    // maps the frames onto the served engine's own allocator.
    const served = await boot();
    const closed = [];
    const browser = await createEngine({
      document: new Uint8Array([1]),
      scratchBoot: async () => served.shape(),
      createContext: () => ({
        sampleRate: 48000, renderQuantumSize: 128, state: "running",
        audioWorklet: { async addModule() {} }, async close() { closed.push("context"); },
      }),
      createHost: async () => ({ ...engineHost(served), async dispose() { closed.push("host"); } }),
    });
    try {
      const first = await browser.controls({ pageLimit: 5 });
      const fader = find(first, (e) => owned(e, "strip", "t1") && e.name === "fader_db" && e.address.lane === "left",
        "t1 left fader");
      const controls = await browser.liveControls();
      const strips = createBrowserStripOrder(browser.host);
      await strips();
      // A transaction between two reads, then a read racing a transaction and a probe.
      const one = await controls.submit(controls.edit.set(fader.address, -4, { rampSamples: 0 }));
      assert.equal(one.ok, true);
      const raced = await Promise.all([
        browser.controls({ pageLimit: 7 }),
        controls.submit(controls.edit.set(fader.address, -5, { rampSamples: 0 })),
        strips(),
        browser.controlState(first, { pageLimit: 9 }),
      ]);
      assert.equal(raced[1].ok, true);
      const state = await browser.controlState(await browser.controls());
      assert.equal(state.values.get(fader.handle), -5);
      await strips();
    } finally {
      await browser.close();
      served.dispose();
    }
  });
});
