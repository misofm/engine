#!/usr/bin/env node

import { createHash, randomBytes } from "node:crypto";
import { createReadStream } from "node:fs";
import { link, lstat, open, readFile, rename, stat, unlink } from "node:fs/promises";
import { basename, dirname, resolve } from "node:path";
import process from "node:process";

import { MisoEngineError, MisoUsageError } from "./core/errors.ts";

const MAXIMUM_REQUEST_BYTES = 4 * 1024 * 1024;

type ExitCode = 2 | 3 | 4 | 5 | 70;

interface ErrorExtra {
  readonly phase?: string;
  readonly result?: number;
  readonly diagnostics?: readonly unknown[];
  readonly groups?: readonly string[];
}

class CliFailure extends Error {
  readonly exitCode: ExitCode;
  readonly code: string;
  readonly extra: ErrorExtra;
  readonly effect: "not_applied" | "applied";

  constructor(
    exitCode: ExitCode,
    code: string,
    message: string,
    extra: ErrorExtra = {},
    effect: "not_applied" | "applied" = "not_applied",
  ) {
    super(message);
    this.name = "CliFailure";
    this.exitCode = exitCode;
    this.code = code;
    this.extra = extra;
    this.effect = effect;
  }
}

const HELP = `Usage: enginectl <command>

Commands:
  session build   Build and validate a canonical Session V1 document

Options:
  --help          Show help
  --version       Show package version
`;

const SESSION_HELP = `Usage: enginectl session <command>

Commands:
  build           Build and validate a canonical Session V1 document
`;

const BUILD_HELP = `Usage: enginectl session build (--request PATH|- | --stems DIRECTORY) --output PATH|- [options]

Exactly one of --request and --stems is required. --request reads one strict JSON request from PATH
or stdin with '-'. --stems reads one leaf directory of directly owned FLAC files. Stem sessions
default their ID from the leaf directory name and their quantum to 128; override those defaults
with --session-id and --quantum-frames.

--output - writes only raw canonical TOML (with its final LF); successful stderr is empty. A file
output is published atomically before stdout emits one compact JSON receipt plus LF. Existing
destinations are refused unless --overwrite is present. In stems mode, the output cannot physically
reside inside the stems directory, including through a symlink or case alias.

A stems collection is refused with code stems.collection and sorted child-directory names; those
children are not asserted to be valid leaves. Failures leave stdout empty and write one JSON stderr
document: exit 2 is usage, 3 is input/build refusal, 4 is engine refusal, 5 is output refusal, and
70 is internal or packaged-asset failure. The command is non-interactive and offline.
`;

function usage(message: string): never {
  throw new CliFailure(2, "cli.usage", message);
}

interface BuildArguments {
  readonly request?: string;
  readonly stems?: string;
  readonly output: string;
  readonly overwrite: boolean;
  readonly sessionId?: string;
  readonly quantumFrames?: number;
}

function parseBuildArguments(args: readonly string[]): BuildArguments | "help" {
  if (args.length === 1 && args[0] === "--help") return "help";
  let request: string | undefined;
  let stems: string | undefined;
  let output: string | undefined;
  let sessionId: string | undefined;
  let quantumFrames: number | undefined;
  let overwrite = false;
  for (let index = 0; index < args.length; index += 1) {
    const flag = args[index];
    if (flag === "--overwrite") {
      if (overwrite) usage("--overwrite was specified more than once");
      overwrite = true;
      continue;
    }
    if (flag !== "--request" && flag !== "--stems" && flag !== "--output"
      && flag !== "--session-id" && flag !== "--quantum-frames") {
      usage(flag?.startsWith("--") ? `unknown flag '${flag}'` : `unexpected operand '${String(flag)}'`);
    }
    const value = args[index + 1];
    if (value === undefined || value.startsWith("--")) usage(`${flag} requires a value`);
    index += 1;
    if (flag === "--request") {
      if (request !== undefined) usage("--request was specified more than once");
      request = value;
    } else if (flag === "--stems") {
      if (stems !== undefined) usage("--stems was specified more than once");
      stems = value;
    } else if (flag === "--output") {
      if (output !== undefined) usage("--output was specified more than once");
      output = value;
    } else if (flag === "--session-id") {
      if (sessionId !== undefined) usage("--session-id was specified more than once");
      sessionId = value;
    } else {
      if (quantumFrames !== undefined) usage("--quantum-frames was specified more than once");
      if (!/^[1-9][0-9]*$/.test(value)) usage("--quantum-frames requires a positive u32");
      quantumFrames = Number(value);
      if (!Number.isSafeInteger(quantumFrames) || quantumFrames > 0xffff_ffff) {
        usage("--quantum-frames requires a positive u32");
      }
    }
  }
  if ((request === undefined) === (stems === undefined)) {
    usage("exactly one of --request and --stems is required");
  }
  if (output === undefined) usage("--output is required");
  if (output === "-" && overwrite) usage("--overwrite cannot be used with --output -");
  if (request !== undefined && (sessionId !== undefined || quantumFrames !== undefined)) {
    usage("--session-id and --quantum-frames are valid only with --stems");
  }
  if (sessionId !== undefined && !/^[a-z][a-z0-9._-]{0,126}$/.test(sessionId)) {
    usage("--session-id must match [a-z][a-z0-9._-]{0,126}");
  }
  return {
    ...(request === undefined ? {} : { request }),
    ...(stems === undefined ? {} : { stems }),
    output,
    overwrite,
    ...(sessionId === undefined ? {} : { sessionId }),
    ...(quantumFrames === undefined ? {} : { quantumFrames }),
  };
}

async function boundedBytes(path: string): Promise<Uint8Array> {
  const stream = path === "-" ? process.stdin : createReadStream(resolve(path));
  const chunks: Uint8Array[] = [];
  let length = 0;
  try {
    for await (const value of stream) {
      const chunk = value as Uint8Array;
      length += chunk.byteLength;
      if (length > MAXIMUM_REQUEST_BYTES) {
        stream.destroy();
        throw new CliFailure(
          3,
          "request.too_large",
          `request exceeds the ${MAXIMUM_REQUEST_BYTES}-byte limit`,
        );
      }
      chunks.push(chunk);
    }
  } catch (error) {
    if (error instanceof CliFailure) throw error;
    throw new CliFailure(3, "request.read", `could not read request: ${errorMessage(error)}`);
  }
  const bytes = new Uint8Array(length);
  let offset = 0;
  for (const chunk of chunks) {
    bytes.set(chunk, offset);
    offset += chunk.byteLength;
  }
  return bytes;
}

function decodeRequest(bytes: Uint8Array): unknown {
  let text: string;
  try {
    text = new TextDecoder("utf-8", { fatal: true }).decode(bytes);
  } catch (error) {
    throw new CliFailure(3, "request.utf8", `request is not valid UTF-8: ${errorMessage(error)}`);
  }
  try {
    return JSON.parse(text) as unknown;
  } catch (error) {
    throw new CliFailure(3, "request.json", `request is not valid JSON: ${errorMessage(error)}`);
  }
}

function errorMessage(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}

async function writeStream(
  stream: NodeJS.WritableStream,
  value: string | Uint8Array,
): Promise<void> {
  await new Promise<void>((accept, reject) => {
    let settled = false;
    const onError = (error: Error): void => {
      if (settled) return;
      settled = true;
      reject(error);
    };
    stream.once("error", onError);
    try {
      stream.write(value, (error) => {
        if (error !== null && error !== undefined) {
          if (!settled) {
            settled = true;
            reject(error);
          }
          // A Node Writable reports a write failure through its callback and may emit the same
          // error immediately afterward. Keep the one-shot listener through that turn so the
          // second notification is consumed rather than becoming an unhandled traceback.
          setImmediate(() => stream.off("error", onError));
          return;
        }
        stream.off("error", onError);
        if (!settled) {
          settled = true;
          accept();
        }
      });
    } catch (error) {
      stream.off("error", onError);
      settled = true;
      reject(error);
    }
  });
}

function writeStdout(value: string | Uint8Array): Promise<void> {
  return writeStream(process.stdout, value);
}

function isBrokenPipe(error: unknown): boolean {
  return typeof error === "object" && error !== null && "code" in error
    && (error as { code?: unknown }).code === "EPIPE";
}

async function publish(
  argument: string,
  bytes: Uint8Array,
  overwrite: boolean,
): Promise<void> {
  const destination = resolve(argument);
  const temporary = resolve(
    dirname(destination),
    `.${basename(destination)}.enginectl-${process.pid}-${randomBytes(12).toString("hex")}.tmp`,
  );
  let temporaryExists = false;
  try {
    const handle = await open(temporary, "wx", 0o600);
    temporaryExists = true;
    try {
      await handle.writeFile(bytes);
    } finally {
      await handle.close();
    }
    if (overwrite) {
      await rename(temporary, destination);
      temporaryExists = false;
    } else {
      await link(temporary, destination);
      // Publication is complete once the hard link exists. Failure to remove the now-redundant
      // temporary name must not be misreported as `not_applied` or suppress the receipt.
      await unlink(temporary).catch(() => undefined);
      temporaryExists = false;
    }
  } catch (error) {
    throw new CliFailure(5, "output.publish", `could not publish '${argument}': ${errorMessage(error)}`);
  } finally {
    if (temporaryExists) await unlink(temporary).catch(() => undefined);
  }
}

async function preflightStemOutput(args: BuildArguments): Promise<void> {
  if (args.request !== undefined || args.output === "-") return;
  const destination = resolve(args.output);
  const parentPath = dirname(destination);
  let parent: Awaited<ReturnType<typeof stat>>;
  try {
    parent = await stat(parentPath, { bigint: true });
    if (!parent.isDirectory()) {
      throw new Error("parent is not a directory");
    }
  } catch (error) {
    throw new CliFailure(5, "output.publish", `could not inspect '${args.output}': ${errorMessage(error)}`);
  }
  // Compare physical directory identities, not path spellings. `stat` follows a symlink parent,
  // and `(dev, ino)` also collapses case aliases on a case-insensitive filesystem. This runs
  // before discovery imports or loads the decoder, so an in-leaf destination cannot overwrite a
  // source or make the next invocation reject the session file the previous invocation created.
  try {
    const stems = await stat(resolve(args.stems as string), { bigint: true });
    if (stems.isDirectory() && stems.dev === parent.dev && stems.ino === parent.ino) {
      throw new CliFailure(
        5,
        "output.publish",
        `could not publish '${args.output}': output parent is the stems directory`,
      );
    }
  } catch (error) {
    if (error instanceof CliFailure) throw error;
    // A missing/unreadable stems path is an input refusal, not an output refusal. Discovery below
    // reports it through the established exit-3 `stems.read` contract.
  }
  try {
    const existing = await lstat(destination);
    if (args.overwrite && !existing.isDirectory()) return;
  } catch (error) {
    if (typeof error === "object" && error !== null && "code" in error
      && (error as { readonly code?: unknown }).code === "ENOENT") return;
    throw new CliFailure(5, "output.publish", `could not inspect '${args.output}': ${errorMessage(error)}`);
  }
  throw new CliFailure(
    5,
    "output.publish",
    `could not publish '${args.output}': destination already exists${args.overwrite ? " as a directory" : ""}`,
  );
}

async function build(args: BuildArguments): Promise<void> {
  await preflightStemOutput(args);
  let toml: string;
  let stemsBuild: import("./cli/stems.ts").StemsBuild | undefined;
  if (args.request !== undefined) {
    const request = decodeRequest(await boundedBytes(args.request));
    const { sessionBuilderFromRequest } = await import("./cli/session-request.ts");
    try {
      toml = sessionBuilderFromRequest(request).toToml();
    } catch (error) {
      if (error instanceof MisoUsageError || error instanceof TypeError) {
        throw new CliFailure(3, "request.shape", error.message);
      }
      throw error;
    }
  } else {
    const directory = args.stems as string;
    const { buildFromStems, normalizeSessionId, StemsImportError } = await import("./cli/stems.ts");
    try {
      stemsBuild = await buildFromStems({
        directory,
        sessionId: args.sessionId ?? normalizeSessionId(directory),
        quantumFrames: args.quantumFrames ?? 128,
      });
      toml = stemsBuild.builder.toToml();
    } catch (error) {
      if (error instanceof StemsImportError) {
        throw new CliFailure(error.internal ? 70 : 3, error.code, error.message, error.extra);
      }
      if (error instanceof MisoUsageError || error instanceof TypeError) {
        throw new CliFailure(3, "stems.shape", error.message);
      }
      throw error;
    }
  }
  const bytes = new TextEncoder().encode(toml);
  let result: Awaited<ReturnType<typeof import("./headless/engine.ts")["validate"]>>;
  try {
    const { validate } = await import("./headless/engine.ts");
    result = await validate(bytes);
  } catch (error) {
    if (error instanceof MisoEngineError) {
      throw new CliFailure(70, "internal.packaged_asset", error.message, {
        phase: error.phase,
        result: error.result,
        diagnostics: error.diagnostics,
      });
    }
    throw error;
  }
  if (!result.ok) {
    const packagedAsset = result.phase === "asset";
    throw new CliFailure(
      packagedAsset ? 70 : 4,
      packagedAsset ? "internal.packaged_asset" : "engine.refused",
      packagedAsset
        ? "embedded engine asset could not be loaded"
        : "embedded engine refused the generated session",
      {
        phase: result.phase,
        result: result.result,
        diagnostics: result.diagnostics,
      },
    );
  }
  if (args.output === "-") {
    await writeStdout(bytes);
    return;
  }
  const requestReceipt = {
    schemaVersion: 1,
    command: "session.build",
    output: {
      path: args.output,
      bytes: bytes.byteLength,
      sha256: createHash("sha256").update(bytes).digest("hex"),
    },
  };
  const receipt = stemsBuild === undefined ? requestReceipt : {
    schemaVersion: 1,
    command: "session.build",
    output: {
      path: args.output,
      resolvedPath: resolve(args.output),
      bytes: bytes.byteLength,
      sha256: createHash("sha256").update(bytes).digest("hex"),
    },
    input: {
      kind: "stems",
      path: args.stems,
      resolvedPath: resolve(args.stems as string),
    },
    session: {
      id: stemsBuild.sessionId,
      revision: 0,
      sampleRateHz: stemsBuild.sampleRateHz,
      quantumFrames: stemsBuild.quantumFrames,
      sources: stemsBuild.mappings.length,
      tracks: stemsBuild.mappings.length,
    },
    stems: stemsBuild.mappings,
  };
  const receiptText = `${JSON.stringify(receipt)}\n`;
  await publish(args.output, bytes, args.overwrite);
  try {
    await writeStdout(receiptText);
  } catch (error) {
    if (isBrokenPipe(error)) throw error;
    throw new CliFailure(
      70,
      "output.report",
      `output was published but its receipt could not be written: ${errorMessage(error)}`,
      {},
      "applied",
    );
  }
}

async function version(): Promise<string> {
  const packageJson = JSON.parse(await readFile(new URL("../package.json", import.meta.url), "utf8")) as {
    version?: unknown;
  };
  if (typeof packageJson.version !== "string") throw new Error("package.json has no version");
  return `${packageJson.version}\n`;
}

async function dispatch(args: readonly string[]): Promise<void> {
  if (args.length === 1 && args[0] === "--help") return writeStdout(HELP);
  if (args.length === 1 && args[0] === "--version") return writeStdout(await version());
  if (args[0] !== "session") usage("expected the 'session' command");
  if (args.length === 2 && args[1] === "--help") return writeStdout(SESSION_HELP);
  if (args[1] !== "build") usage("expected the 'session build' command");
  const parsed = parseBuildArguments(args.slice(2));
  if (parsed === "help") return writeStdout(BUILD_HELP);
  await build(parsed);
}

function errorDocument(error: CliFailure): string {
  return `${JSON.stringify({
    schemaVersion: 1,
    error: { code: error.code, message: error.message },
    effect: error.effect,
    ...error.extra,
  })}\n`;
}

try {
  await dispatch(process.argv.slice(2));
} catch (error) {
  if (isBrokenPipe(error)) process.exitCode = 0;
  else {
    const failure = error instanceof CliFailure
      ? error
      : new CliFailure(70, "internal", errorMessage(error));
    process.exitCode = failure.exitCode;
    try {
      await writeStream(process.stderr, errorDocument(failure));
    } catch {
      // Stderr itself is unavailable. There is no second channel to report that failure on, and
      // recursively writing another diagnostic would risk duplicate documents or a traceback.
    }
  }
}
