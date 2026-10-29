// Issue #1387 (attempt 2, MAJOR-2): the MSB1 consumer's protocol, one case table for both copies of
// the consumer. `single` mode drains the rings in the worklet prelude
// (`sdk/src/browser-assets/miso-engine-v1-pcm-feed-worklet.js`, driven by `browser-pcm-evals.mjs`)
// and `worker` mode in the control Worker (`hosts/host-web/web/miso-engine-v1-control-worker.js`,
// driven by `scripts/test-web-audioworklet.mjs`). Both runners build an adapter over a fake engine
// and run every case here against it, so a rule one copy keeps and the other drops turns that
// copy's runner red. Generating both copies from one source at build time is a later follow-up.
//
// The adapter (`make()` returns a fresh one per case; quantum 4 frames, at most 2 channels):
//   attach(rings)             the rings, posted on the feed's port (cloned, as a port clones)
//   pass()                    one drain pass, after one rendered quantum
//   seekAt(message)           the engine handler's `miso.seek-at.v1`; its result is `seekAtResult`
//   sessionSeek(generation)   an accepted session seek at `generation` (#1644)
//   prepareSeek(seeks)        the `prepare-seek` op; resolves with its `seek-prepared` reply
//   detachSource(index)       the `detach-source` op; resolves with its `source-detached` reply
//   submitResult, seekResult, seekAtResult   the fake exports' results (settable)
//   seekStale                 when true a refused `source_seek` writes `source.generation.stale`
//   onSeek                    a hook `source_seek` calls with (generation, frame) before it answers
//   onSnapshot                a hook a drain pass calls right after it read the seek publication
//                             (`readSharedSeek`), before it reads any slot (#2185 attempt 2)
//   onApply                   a hook called when prepare-seek applies a captured seek, before the
//                             apply reads anything (the race window between check and apply)
//   submissions               [{ id, generation, start, channels, frames, end, pcm }]
//   seeks                     [[generation, frame]] of every `source_seek` call
//   seekBound                 the consumer's live seek deferral bound in quanta (#2185)
// The prelude submits at most two chunks a pass and the Worker as many as the engine takes, so a
// case passes until the ring is drained (`drain`) wherever counts matter.

import assert from "node:assert/strict";

import {
  MSB1_CONTROL,
  MSB1_CONTROL_BYTES,
  MSB1_CONTROL_I64_OFFSET,
  MSB1_HEADER_OFFSET,
  MSB1_SLOT_HEADER_BYTES,
  createMsb1Ring,
  Msb1RingWriter,
} from "../src/browser/pcm-ring.ts";

const Q = 4;

/** The code the engine writes at the start of the session diagnostic when it refuses a seek at a
 *  generation the source already has (#1644 D2). Both adapters' fake engines write it; the SDK's
 *  real-Wasm test proves the built engine writes exactly it. */
export const STALE_GENERATION_CODE = "source.generation.stale";
const controls = (ring) => new Int32Array(ring, 0, MSB1_CONTROL_BYTES / 4);
const count = (ring, word) => controls(ring)[MSB1_CONTROL[word]];
/** The ring's published seek epoch: what an SDK snapshot of it reads (#2184: 2 steps a seek). */
const epochOf = (ring) => Atomics.load(controls(ring), MSB1_CONTROL.SEEK_EPOCH);
function ring(sourceId, channels = 1, capacity = 8) {
  const shared = createMsb1Ring({ sourceId, channels, frameCapacity: Q, capacity });
  const writer = new Msb1RingWriter(shared);
  writer.engage(1n);
  const write = (generation, startFrame, { frames = Q, end = false, value = 1 } = {}) => {
    const planes = writer.reserve(frames);
    planes.forEach((plane, channel) => plane.fill(value + channel * 10, 0, frames));
    writer.commit({ generation, startFrame, frames, endOfRegion: end });
  };
  return { shared, writer, write };
}
async function drain(consumer, passes = 6) {
  for (let pass = 0; pass < passes; pass += 1) await consumer.pass();
}
const starts = (consumer) => consumer.submissions.map(({ generation, start }) => [generation, start]);

export const MSB1_CONSUMER_CASES = [
  {
    name: "chunks are submitted in order with the ring's ID, planes a quantum apart and stale tags dropped",
    async run(consumer) {
      const mono = ring("mono", 1);
      const stereo = ring("stéréo", 2);
      await consumer.attach([mono.shared, stereo.shared]);
      mono.write(1n, 0n, { value: 3 });
      mono.write(7n, 4n); // a tag the ring does not carry: stale
      mono.write(1n, 4n, { value: 5 });
      stereo.write(1n, 0n, { value: 2, frames: 3, end: true });
      await drain(consumer);
      const byId = (id) => consumer.submissions.filter((entry) => entry.id === id).map(({ generation, start, channels, frames, end }) => [generation, start, channels, frames, end]);
      assert.deepEqual(byId("mono"), [[1n, 0n, 1, 4, 0], [1n, 4n, 1, 4, 0]]);
      assert.deepEqual(byId("stéréo"), [[1n, 0n, 2, 3, 1]]);
      const stereoPcm = consumer.submissions.find((entry) => entry.id === "stéréo").pcm;
      assert.deepEqual(stereoPcm.slice(0, 3), [2, 2, 2]);
      assert.deepEqual(stereoPcm.slice(Q, Q + 3), [12, 12, 12], "the second plane a quantum after the first");
      assert.deepEqual([count(mono.shared, "STALE"), count(mono.shared, "SUBMITTED")], [1, 2]);
      assert.equal(count(stereo.shared, "FINISHED"), 1, "an end-of-region chunk marks the ring finished");
      assert.equal(count(stereo.shared, "SUBMITTED_GENERATION_TAG"), 1);
    },
  },
  {
    name: "a backpressured chunk stays in the ring and is submitted again; another refusal is counted and consumed",
    async run(consumer) {
      const fed = ring("fed");
      await consumer.attach([fed.shared]);
      consumer.submitResult = 6;
      fed.write(1n, 0n);
      await consumer.pass();
      assert.equal(fed.writer.occupancy, 1, "the chunk stays");
      assert.equal(count(fed.shared, "REFUSED"), 0);
      consumer.submitResult = 0;
      await consumer.pass();
      assert.equal(fed.writer.occupancy, 0);
      assert.deepEqual(starts(consumer).at(-1), [1n, 0n]);
      consumer.submitResult = 4;
      fed.write(1n, 4n);
      await consumer.pass();
      assert.deepEqual([fed.writer.occupancy, count(fed.shared, "REFUSED"), count(fed.shared, "LAST_RESULT")], [0, 1, 4]);
    },
  },
  {
    name: "a plain ring seek reaches source_seek before any slot, retries on backpressure, and a refusal reads no slot",
    async run(consumer) {
      const fed = ring("fed");
      await consumer.attach([fed.shared]);
      fed.write(1n, 0n, { end: false });
      fed.writer.seek(2n, 40n);
      fed.write(2n, 40n);
      consumer.seekResult = 6;
      await consumer.pass();
      assert.deepEqual(consumer.submissions, [], "no slot is read while the seek waits");
      assert.equal(count(fed.shared, "SEEKS_APPLIED"), 0);
      consumer.seekResult = 0;
      await drain(consumer);
      assert.ok(consumer.seeks.length >= 2 && consumer.seeks.every(([g, f]) => g === 2n && f === 40n), "the same seek is asked again");
      assert.deepEqual(starts(consumer), [[2n, 40n]], "the old generation's chunk is dropped");
      assert.deepEqual([count(fed.shared, "SEEKS_APPLIED"), count(fed.shared, "STALE")], [1, 1]);
      fed.writer.seek(3n, 80n);
      fed.write(3n, 80n);
      consumer.seekResult = 1;
      await consumer.pass();
      // A refused seek is asked again each drain (the Worker drains twice a block): counted each time.
      assert.ok(count(fed.shared, "REFUSED") >= 1);
      assert.deepEqual([count(fed.shared, "LAST_RESULT"), count(fed.shared, "SEEKS_APPLIED")], [1, 1]);
      assert.equal(consumer.submissions.length, 1, "a refused seek reads no slot");
    },
  },
  {
    // #2184: the writer publishes the tuple and the tag between two epoch steps (odd = in flight).
    // The g2 chunk stands for the writer's next chunk, which a pass that straddles the publication
    // sees beside the new tag. Red on a reader that uses the tag or the tuple outside the seqlock:
    // the old read order asks this seek at the odd epoch, or (old epoch, new tag) submits g2
    // before its seek, which the engine refuses as stale.
    name: "#2184: a seek publication in flight reads no slot and asks no seek; complete, the seek precedes its chunk",
    async run(consumer) {
      const fed = ring("fed");
      await consumer.attach([fed.shared]);
      fed.write(1n, 0n);
      const c = controls(fed.shared);
      const wide = new BigInt64Array(fed.shared, MSB1_CONTROL_I64_OFFSET, 3);
      Atomics.add(c, MSB1_CONTROL.SEEK_EPOCH, 1);
      Atomics.store(wide, 0, 2n);
      Atomics.store(wide, 1, 40n);
      Atomics.store(wide, 2, -1n);
      Atomics.store(c, MSB1_CONTROL.GENERATION_TAG, 2);
      fed.write(2n, 40n);
      await consumer.pass();
      assert.deepEqual([consumer.seeks, consumer.submissions, fed.writer.occupancy], [[], [], 2],
        "a publication in flight asks no seek and reads no slot");
      Atomics.add(c, MSB1_CONTROL.SEEK_EPOCH, 1);
      await drain(consumer);
      assert.deepEqual(consumer.seeks[0], [2n, 40n]);
      assert.deepEqual(starts(consumer), [[2n, 40n]], "the seek precedes its chunk");
      assert.deepEqual([count(fed.shared, "SEEKS_APPLIED"), count(fed.shared, "STALE"), count(fed.shared, "REFUSED")], [1, 1, 0]);
    },
  },
  {
    // #2185: red on a drain that applies a live plain seek in the pass that frees the stale slots:
    // render then drops the old chunks the engine holds before the writer could publish any new
    // one, a gap; and red on a deferral with no bound.
    name: "#2185: a live plain seek over a full ring of the old generation frees its slots and waits for its first chunk; the bound applies it",
    async run(consumer) {
      const fed = ring("fed");
      await consumer.attach([fed.shared]);
      consumer.submitResult = 6; // the engine's ring is full of the old generation
      for (let chunk = 0; chunk < 8; chunk += 1) fed.write(1n, BigInt(chunk * Q));
      await consumer.pass();
      assert.equal(fed.writer.occupancy, 8, "the shared ring is full");
      fed.writer.seek(2n, 40n);
      await consumer.pass();
      assert.deepEqual([consumer.seeks, count(fed.shared, "STALE"), fed.writer.occupancy], [[], 8, 0],
        "the stale slots are freed and no seek is asked before a chunk of its generation");
      fed.write(2n, 40n);
      consumer.submitResult = 0;
      await consumer.pass();
      assert.deepEqual(consumer.seeks, [[2n, 40n]], "the first new chunk applies the seek");
      assert.deepEqual(starts(consumer).filter(([generation]) => generation === 2n), [[2n, 40n]], "then drains it");
      fed.writer.seek(3n, 80n); // a writer that publishes nothing
      for (let pass = 1; pass < consumer.seekBound; pass += 1) await consumer.pass();
      assert.equal(consumer.seeks.length, 1, "no seek within the bound");
      await drain(consumer, 2);
      assert.deepEqual(consumer.seeks.at(-1), [3n, 80n], "the bound applies the seek");
      assert.deepEqual([count(fed.shared, "SEEKS_APPLIED"), count(fed.shared, "REFUSED")], [2, 0]);
    },
  },
  {
    // #2185 attempt 2: red on a pass that reads the write index or classifies slots outside its
    // seek snapshot: it frees the newer seek's first chunk as stale (in the drain and in a
    // deferral), and the engine then refuses that generation's later chunks as non-contiguous.
    name: "#2185: a newer seek and its chunk published after a pass's snapshot survive to be submitted after their seek",
    async run(consumer) {
      const fed = ring("fed");
      await consumer.attach([fed.shared]);
      const once = (publish) => {
        consumer.onSnapshot = () => {
          consumer.onSnapshot = null;
          publish();
        };
      };
      const after = (generation) => starts(consumer).filter(([g]) => g >= generation);
      // The drain: the pass applies g2 and drains its chunk; g3 lands after its snapshot.
      fed.write(1n, 0n);
      fed.writer.seek(2n, 40n);
      fed.write(2n, 40n);
      once(() => { fed.writer.seek(3n, 80n); fed.write(3n, 80n); });
      await consumer.pass();
      fed.write(3n, 84n);
      await drain(consumer);
      assert.deepEqual(consumer.seeks, [[2n, 40n], [3n, 80n]]);
      assert.deepEqual(after(2n), [[2n, 40n], [3n, 80n], [3n, 84n]], "the newer seek's first chunk survives the drain");
      // A deferral: g4 waits for its chunk; g5 and its chunk land after a deferring pass's snapshot.
      fed.writer.seek(4n, 120n);
      await consumer.pass();
      once(() => { fed.writer.seek(5n, 160n); fed.write(5n, 160n); });
      await consumer.pass();
      fed.write(5n, 164n);
      await drain(consumer);
      assert.deepEqual(consumer.seeks.at(-1), [5n, 160n]);
      assert.ok(consumer.seeks.every(([g]) => g !== 4n), "the replaced seek never reaches the engine");
      assert.deepEqual(after(5n), [[5n, 160n], [5n, 164n]], "the newer seek's first chunk survives a deferral");
      assert.equal(count(fed.shared, "REFUSED"), 0);
    },
  },
  {
    name: "#1644: a seek refused as stale at the session seek's generation is the seek applied; any other refusal is counted",
    async run(consumer) {
      const moved = ring("moved");
      const other = ring("other");
      await consumer.attach([moved.shared, other.shared]);
      await consumer.sessionSeek(2n);
      consumer.seekResult = 1;
      consumer.seekStale = true;
      moved.writer.seek(2n, 8n);
      moved.write(2n, 8n);
      await drain(consumer);
      assert.deepEqual([count(moved.shared, "SEEKS_APPLIED"), count(moved.shared, "REFUSED"), count(moved.shared, "SUBMITTED")], [1, 0, 1]);
      // Stale at another generation than the session seek's: a refusal.
      moved.writer.seek(3n, 16n);
      moved.write(3n, 16n);
      await consumer.pass();
      assert.equal(count(moved.shared, "SEEKS_APPLIED"), 1);
      assert.ok(count(moved.shared, "REFUSED") >= 1, "stale at another generation is a refusal");
      // At the session generation but another refusal than the stale one: a refusal too.
      consumer.seekStale = false;
      other.writer.seek(2n, 0n);
      other.write(2n, 0n);
      await consumer.pass();
      assert.equal(count(other.shared, "SEEKS_APPLIED"), 0);
      assert.ok(count(other.shared, "REFUSED") >= 1, "another refusal at the session generation is counted");
    },
  },
  {
    name: "a seek clears the finished mark; a torn slot stops the pass; a bad frame count is counted and skipped",
    async run(consumer) {
      const fed = ring("fed");
      await consumer.attach([fed.shared]);
      fed.write(1n, 0n, { frames: 2, end: true });
      await drain(consumer);
      assert.equal(count(fed.shared, "FINISHED"), 1);
      fed.writer.seek(2n, 0n);
      fed.write(2n, 0n);
      await consumer.pass();
      assert.equal(count(fed.shared, "FINISHED"), 0, "a seek starts the region again");
      // Bad frame counts: 0 and a quantum plus one, both skipped and counted.
      const headers = new Int32Array(fed.shared, MSB1_HEADER_OFFSET, (8 * MSB1_SLOT_HEADER_BYTES) / 4);
      for (const frames of [0, Q + 1]) {
        const slot = Atomics.load(controls(fed.shared), MSB1_CONTROL.WRITE_INDEX) & 7;
        fed.write(2n, 0n);
        headers[slot * 8 + 2] = frames;
      }
      fed.write(2n, 0n);
      await drain(consumer);
      assert.equal(count(fed.shared, "ERRORS"), 2);
      assert.deepEqual(starts(consumer).at(-1), [2n, 0n], "the good chunk behind them is submitted");
      // A slot whose sequence is not the index (a second writer): the pass stops before it.
      const slot = Atomics.load(controls(fed.shared), MSB1_CONTROL.WRITE_INDEX) & 7;
      fed.write(2n, 4n);
      headers[slot * 8] += 1;
      const before = consumer.submissions.length;
      await consumer.pass();
      assert.ok(count(fed.shared, "TORN") >= 1, "a torn slot is counted each drain that meets it");
      assert.equal(consumer.submissions.length, before, "nothing behind a torn slot");
      assert.equal(fed.writer.occupancy, 1, "the torn slot stays");
    },
  },
  {
    name: "underruns are counted per block that found no chunk, and none once the region ended",
    async run(consumer) {
      const fed = ring("fed");
      await consumer.attach([fed.shared]);
      fed.write(1n, 0n);
      for (let pass = 0; pass < 3; pass += 1) await consumer.pass();
      assert.equal(count(fed.shared, "UNDERRUNS"), 2, "two blocks found the ring dry");
      fed.write(1n, 4n, { frames: 3, end: true });
      for (let pass = 0; pass < 3; pass += 1) await consumer.pass();
      assert.equal(count(fed.shared, "UNDERRUNS"), 2, "none after the region's last chunk");
      assert.equal(count(fed.shared, "DRAIN_BLOCKS"), 6);
    },
  },
  {
    name: "a ring whose writer is not live counts no underrun while the source runs dry",
    async run(consumer) {
      const idle = ring("idle");
      await consumer.attach([idle.shared]);
      idle.write(1n, 0n);
      idle.writer.release();
      await drain(consumer, 4);
      assert.deepEqual([count(idle.shared, "UNDERRUNS"), count(idle.shared, "DRAIN_BLOCKS")], [0, 4]);
    },
  },
  {
    name: "prepare-seek applies the captured tuple when the writer seeks again between check and apply",
    async run(consumer) {
      const fed = ring("fed");
      await consumer.attach([fed.shared]);
      fed.writer.seek(2n, 12n);
      const captured = epochOf(fed.shared);
      let advanced = false;
      consumer.onApply = () => {
        if (advanced) return;
        advanced = true;
        fed.writer.seek(3n, 16n);
      };
      let reply = await consumer.prepareSeek([{ epoch: captured, generation: 2n, frame: 12n }]);
      assert.equal(reply.kind, "superseded");
      // The Worker drains after each message, so it may apply the newer seek at once; either way
      // the preparation applied the captured tuple and no seek mixes the two.
      assert.deepEqual(consumer.seeks[0], [2n, 12n], "the captured tuple, never mixed live epoch/generation words");
      assert.ok(consumer.seeks.every(([g, f]) => (g === 2n && f === 12n) || (g === 3n && f === 16n)));
      consumer.onApply = null;
      reply = await consumer.prepareSeek([{ epoch: epochOf(fed.shared), generation: 3n, frame: 16n }]);
      assert.equal(reply.kind, "confirmed");
      assert.deepEqual(consumer.seeks, [[2n, 12n], [3n, 16n]]);
      assert.equal(count(fed.shared, "DRAIN_BLOCKS"), 0, "preparation renders and counts no block");
    },
  },
  {
    name: "a ring written to the old MSB1 header offset is refused at attach",
    async run(consumer) {
      const old = createMsb1Ring({ sourceId: "old", channels: 1, frameCapacity: Q, capacity: 2 });
      controls(old)[MSB1_CONTROL.HEADER_OFFSET] = 256;
      controls(old)[MSB1_CONTROL.PCM_OFFSET] = 256 + 2 * MSB1_SLOT_HEADER_BYTES;
      const current = createMsb1Ring({ sourceId: "current", channels: 1, frameCapacity: Q, capacity: 2 });
      await consumer.attach([old, current]);
      assert.deepEqual([controls(old)[MSB1_CONTROL.ATTACHED], controls(current)[MSB1_CONTROL.ATTACHED]], [0, 1]);
    },
  },
  {
    name: "an anchored seek waits for the handler's OK for its exact tuple, then is consumed with no source_seek",
    async run(consumer) {
      const fed = ring("añadido");
      await consumer.attach([fed.shared]);
      fed.writer.seek(9n, 40n, 2_048n);
      fed.write(9n, 40n);
      await consumer.pass();
      assert.deepEqual([consumer.submissions.length, count(fed.shared, "SEEKS_APPLIED")], [0, 0], "nothing before the handler");
      // A tuple that does not match the ring's (another frame) leaves it waiting.
      await consumer.seekAt({ sourceId: "añadido", generation: 9n, sourceFrame: 44n, anchorSample: 2_048n });
      await consumer.pass();
      assert.equal(consumer.submissions.length, 0, "a mismatched tuple is not this ring's seek");
      // A refused seek-at leaves the epoch unseen.
      consumer.seekAtResult = 1;
      await consumer.seekAt({ sourceId: "añadido", generation: 9n, sourceFrame: 40n, anchorSample: 2_048n });
      await consumer.pass();
      assert.deepEqual([consumer.submissions.length, count(fed.shared, "SEEKS_APPLIED"), count(fed.shared, "REFUSED")], [0, 0, 0]);
      consumer.seekAtResult = 0;
      await consumer.seekAt({ sourceId: "añadido", generation: 9n, sourceFrame: 40n, anchorSample: 2_048n });
      await consumer.pass();
      assert.deepEqual(starts(consumer), [[9n, 40n]]);
      assert.deepEqual([count(fed.shared, "SEEKS_APPLIED"), consumer.seeks.length], [1, 0], "consumed with no plain seek");
    },
  },
  {
    name: "prepare-seek: confirmed frees the stale slots and keeps current ones; refused and superseded answer so; future slots survive",
    async run(consumer) {
      const fed = ring("fed");
      await consumer.attach([fed.shared]);
      consumer.submitResult = 6; // the engine holds what it has: nothing is drained meanwhile
      for (let chunk = 0; chunk < 3; chunk += 1) fed.write(1n, BigInt(chunk * Q));
      fed.writer.seek(2n, 12n);
      fed.write(2n, 12n);
      let reply = await consumer.prepareSeek([{ epoch: epochOf(fed.shared), generation: 2n, frame: 12n }]);
      assert.equal(reply.kind, "confirmed");
      assert.deepEqual(consumer.seeks.at(-1), [2n, 12n]);
      assert.equal(count(fed.shared, "STALE"), 3, "the old generation's slots are freed");
      assert.equal(fed.writer.occupancy, 1, "the current generation's chunk is kept, not consumed");
      // Refused: the seek's own refusal is the reply's result.
      fed.writer.seek(3n, 16n);
      consumer.seekResult = 1;
      reply = await consumer.prepareSeek([{ epoch: epochOf(fed.shared), generation: 3n, frame: 16n }]);
      assert.deepEqual([reply.kind, reply.result], ["refused", 1]);
      consumer.seekResult = 0;
      // Superseded while it runs: the producer seeks again (with PCM at the newer generation)
      // from inside source_seek; the newer chunk is never discarded.
      fed.writer.seek(4n, 20n);
      const passed = epochOf(fed.shared);
      consumer.onSeek = (generation) => {
        if (generation !== 4n) return;
        fed.writer.seek(5n, 24n);
        fed.write(5n, 24n);
      };
      reply = await consumer.prepareSeek([{ epoch: passed, generation: 4n, frame: 20n }]);
      assert.equal(reply.kind, "superseded");
      consumer.onSeek = null;
      reply = await consumer.prepareSeek([{ epoch: epochOf(fed.shared), generation: 5n, frame: 24n }]);
      assert.equal(reply.kind, "confirmed");
      assert.equal(fed.writer.occupancy, 1, "the superseding generation's chunk survives both");
      // A stale snapshot (an epoch the ring has moved past) is superseded before any seek.
      const asked = consumer.seeks.length;
      reply = await consumer.prepareSeek([{ epoch: passed, generation: 4n, frame: 20n }]);
      assert.equal(reply.kind, "superseded");
      assert.equal(consumer.seeks.length, asked);
      consumer.submitResult = 0;
      await drain(consumer);
      assert.deepEqual(starts(consumer).at(-1), [5n, 24n]);
    },
  },
  {
    name: "detach-source withdraws exactly the ring at that index, and an index naming none withdraws nothing",
    async run(consumer) {
      const rings = ["first", "middle", "last"].map((id) => ring(id));
      await consumer.attach(rings.map(({ shared }) => shared));
      let reply = await consumer.detachSource(0);
      assert.equal(reply.detached, true);
      reply = await consumer.detachSource(0);
      assert.equal(reply.detached, false, "the slot stays empty, never compacted");
      reply = await consumer.detachSource(7);
      assert.equal(reply.detached, false);
      for (const { write } of rings) write(1n, 0n);
      await drain(consumer);
      assert.deepEqual(consumer.submissions.map(({ id }) => id).sort(), ["last", "middle"]);
      assert.deepEqual(rings.map(({ shared }) => controls(shared)[MSB1_CONTROL.ATTACHED]), [0, 1, 1]);
      assert.equal(controls(rings[0].shared)[MSB1_CONTROL.READ_INDEX], 0, "the withdrawn ring is never read");
      reply = await consumer.detachSource(2);
      assert.equal(reply.detached, true, "a later index still names its own ring");
      assert.equal(controls(rings[2].shared)[MSB1_CONTROL.ATTACHED], 0);
    },
  },
];
