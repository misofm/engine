import {
  POLICY_WORDS,
  ROLE_DEFINED_WORDS,
  constantValue,
  writeBootOptions,
} from "../core/abi.ts";
import type { BootOptions } from "../core/abi.ts";
import { ABI_LAYOUT } from "../generated/abi.ts";
import { MisoEngineError } from "../core/errors.ts";
import type { SourceSpec } from "../core/types.ts";

/**
 * The browser choreography's decidable half (issue #243 S3, #240 S5).
 *
 * Everything in this file is a pure function over values, deliberately: the rest of the browser
 * entry is `AudioContext` construction and `addModule`, which no Node test can exercise, and the
 * decisions that actually matter -- which words the two boots share, when a quantum is refused,
 * which documents a browser may open -- must be provable at a harness rather than only in a
 * browser. #246 owns the browser matrices; this is what #243 can prove.
 */

/**
 * The boot policy a browser session applies to BOTH of its boots.
 *
 * #240 S5 sealed "identical options struct for scratch and worklet boots", and adopted ruling
 * 5462139867 finding 3 corrected it: literally identical is impossible, because the scratch boot
 * writes `require_* = 0` while the worklet writes the physical rate and quantum. The divergence
 * class A-1 actually named was the CONSOLE words -- a scratch instance that budgeted a console
 * differently from the worklet would size its plan differently and answer the wrong question.
 *
 * So the rule is stated over the words it is about: the policy words are shared by construction,
 * because a caller supplies them once and both boots read the same object; the two `require_*`
 * words are role-defined.
 */
export interface BrowserBootPolicy {
  readonly sourceRingFrames?: number;
  readonly maximumMemoryBytes?: bigint;
  readonly console?: BootOptions["console"];
}

/**
 * The scratch boot's options: the shared policy with both `require_*` words at zero.
 *
 * The scratch instance exists to answer one question -- what shape does this document declare? --
 * and it must accept whatever the document says in order to answer it. A scratch boot that
 * required a rate would refuse the very documents it was built to describe.
 */
export function scratchBootOptions(policy: BrowserBootPolicy): BootOptions {
  return withRequiredShape(policy, 0, 0);
}

/**
 * Carry the shared policy words across, then stamp the two role-defined ones.
 *
 * Written out rather than spread because `exactOptionalPropertyTypes` distinguishes "absent" from
 * "present and undefined", and that distinction is load-bearing here: an absent policy word means
 * "the engine's own default", and a word explicitly set to `undefined` would be a different claim
 * that happened to encode the same. Both boots read the same `policy` object, so the shared words
 * are identical by construction rather than by a copy that could drift.
 */
function withRequiredShape(
  policy: BrowserBootPolicy,
  requireSampleRateHz: number,
  requireQuantumFrames: number,
): BootOptions {
  const options: {
    -readonly [Key in keyof BootOptions]?: BootOptions[Key];
  } = { requireSampleRateHz, requireQuantumFrames };
  if (policy.sourceRingFrames !== undefined) options.sourceRingFrames = policy.sourceRingFrames;
  if (policy.maximumMemoryBytes !== undefined) {
    options.maximumMemoryBytes = policy.maximumMemoryBytes;
  }
  if (policy.console !== undefined) options.console = policy.console;
  return options;
}

/**
 * The worklet boot's options: the same shared policy, pinned to the physical shape.
 *
 * The worklet has an `AudioContext` it cannot argue with, so its two `require_*` words carry the
 * physical rate and quantum. A document that does not match is `reprepareRequired` with
 * `host.session.shape` -- the backstop, kept from #240 S5, for the case where the pre-worklet
 * check was somehow passed by a context that then changed under it.
 */
export function workletBootOptions(
  policy: BrowserBootPolicy,
  physical: { readonly sampleRateHz: number; readonly quantumFrames: number },
): BootOptions {
  return withRequiredShape(policy, physical.sampleRateHz, physical.quantumFrames);
}

/**
 * Encode both option blocks and prove they differ in nothing but the two role-defined words.
 *
 * This is finding 3's equality rule, executable. The comparison is over BYTES rather than over the
 * option objects, because bytes are what the engine reads: two objects that agree field by field
 * but encode differently would still be a divergence, and that is precisely the class of bug the
 * five hand-written config tables produced.
 */
export function bootOptionsAgree(
  policy: BrowserBootPolicy,
  physical: { readonly sampleRateHz: number; readonly quantumFrames: number },
): {
  readonly scratch: Uint8Array;
  readonly worklet: Uint8Array;
  readonly maskedEqual: boolean;
  readonly scratchRequireWordsAreZero: boolean;
} {
  // Two scratch addresses in a throwaway page, so the two blocks are encoded by the very function
  // that writes them into a live module rather than by a re-implementation of it.
  const memory = new WebAssembly.Memory({ initial: 1 });
  const bytes = ABI_LAYOUT.structures.bootOptions.bytes;
  const scratch = writeBootOptions(memory, bytes, scratchBootOptions(policy));
  const worklet = writeBootOptions(memory, bytes * 2, workletBootOptions(policy, physical));

  const masked = (block: Uint8Array): Uint8Array => {
    const copy = block.slice();
    // The two `require_*` words are masked by NAME through the generated layout, so a field that
    // moves moves the mask with it and this file holds no offset of its own.
    for (const word of ROLE_DEFINED_WORDS) {
      const offset = fieldOffset(word);
      copy.fill(0, offset, offset + 4);
    }
    return copy;
  };

  const left = masked(scratch);
  const right = masked(worklet);
  let maskedEqual = left.length === right.length;
  if (maskedEqual) {
    for (let index = 0; index < left.length; index += 1) {
      if (left[index] !== right[index]) { maskedEqual = false; break; }
    }
  }

  const view = new DataView(scratch.buffer, scratch.byteOffset, scratch.byteLength);
  const scratchRequireWordsAreZero = ROLE_DEFINED_WORDS
    .every((word) => view.getUint32(fieldOffset(word), true) === 0);

  return { scratch, worklet, maskedEqual, scratchRequireWordsAreZero };
}

/** The offset of one boot-options word, by name. No literal offset appears in this file. */
function fieldOffset(name: string): number {
  const row = ABI_LAYOUT.structures.bootOptions.fields.find((field) => field.name === name);
  if (row === undefined) throw new Error(`the generated layout has no bootOptions.${name}`);
  return row.offset;
}

/** The words shared by both boots, exported so a caller can name them in its own assertions. */
export { POLICY_WORDS, ROLE_DEFINED_WORDS };

/**
 * The pre-worklet quantum check (#240 S5).
 *
 * `renderQuantumSize` is not yet universal, so an absent one reads as the specified default of
 * 128. The check exists because a mismatch discovered *inside* the worklet costs an `addModule`,
 * a node construction and a boot before it can be reported -- and reports it as a sticky failure
 * on a live audio graph rather than as an answer to a question the caller asked.
 */
export function assertQuantumMatch(
  contextRenderQuantumSize: number | undefined,
  documentQuantumFrames: number,
): void {
  const physical = contextRenderQuantumSize ?? 128;
  if (physical !== documentQuantumFrames) {
    throw new MisoEngineError(
      `this AudioContext renders ${physical}-frame quanta, but the document declares `
      + `${documentQuantumFrames}`,
      {
        phase: "boot",
        code: "reprepareRequired",
        result: constantValue("resultCodes", "reprepareRequired"),
        diagnostics: [{ code: "host.session.shape", path: "$.quantum_frames" }],
      },
    );
  }
}

/**
 * The launch bit-depth tokens a WEB session may open.
 *
 * Adopted ruling 5462139867 finding 6 sentence 2: web delivery and store scope stay integer-only
 * at launch, matching the delivered-catalog closure the `32f` ruling itself preserved. A
 * `32f`-declaring document refuses typed at the resolver/ingest boundary rather than arming
 * silently or falling back lossily.
 *
 * The SDK's builder and types carry the full token set (sentence 1) because the SCHEMA carries it
 * and native paths support it; what is scoped here is what a browser may open.
 */
export const WEB_LAUNCH_BIT_DEPTHS: readonly (16 | 24)[] = Object.freeze([16, 24]);

/**
 * Refuse a `32f` source at web session-open.
 *
 * # Where this check lives, and where the rest of it lives
 *
 * This is the SDK's half: a source declared through the SDK's own typed surface carries its
 * `bitDepth`, so a browser open can refuse it before an `AudioContext` is constructed. The other
 * half -- a raw JSON document whose sources the SDK has never seen as values -- belongs to the
 * resolver/ingest boundary, which is #244's, and is where the ruling puts it. The SDK has no session
 * parser and will not grow one (ruling 5438024085), so it cannot and must not try to read a bit
 * depth out of a document's text.
 */
export function assertWebDeliverableSources(
  sources: readonly { readonly id: string; readonly spec: SourceSpec }[],
): void {
  for (const source of sources) {
    if (!WEB_LAUNCH_BIT_DEPTHS.includes(source.spec.bitDepth as 16 | 24)) {
      throw new MisoEngineError(
        `source ${source.id} declares bit depth ${JSON.stringify(source.spec.bitDepth)}, which web `
        + `delivery does not carry at launch`,
        {
          phase: "source",
          code: "unsupported",
          result: constantValue("resultCodes", "unsupported"),
          diagnostics: [{
            code: "stem.depth.unsupported_at_launch",
            path: `$.sources[${source.id}].bit_depth=${String(source.spec.bitDepth)}`,
          }],
        },
      );
    }
  }
}
