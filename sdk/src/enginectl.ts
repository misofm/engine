#!/usr/bin/env node

import { createHash, randomBytes } from "node:crypto";
import { createReadStream } from "node:fs";
import { link, open, readFile, rename, unlink } from "node:fs/promises";
import { basename, dirname, resolve } from "node:path";
import process from "node:process";

import { MisoEngineError, MisoUsageError } from "./core/errors.ts";
import { sessionBuilderFromRequest } from "./cli/session-request.ts";

const MAXIMUM_REQUEST_BYTES = 4 * 1024 * 1024;

type ExitCode = 2 | 3 | 4 | 5 | 70;

interface ErrorExtra {
  readonly phase?: string;
  readonly result?: number;
  readonly diagnostics?: readonly unknown[];
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

const BUILD_HELP = `Usage: enginectl session build --request PATH|- --output PATH|- [--overwrite]

Reads one strict, versioned JSON request and validates the generated canonical TOML with the
embedded engine. PATH is resolved from the current directory; '-' selects stdin or raw stdout.
Existing output is refused unless --overwrite is present. The command is always non-interactive.
`;

function usage(message: string): never {
  throw new CliFailure(2, "cli.usage", message);
}

interface BuildArguments {
  readonly request: string;
  readonly output: string;
  readonly overwrite: boolean;
}

function parseBuildArguments(args: readonly string[]): BuildArguments | "help" {
  if (args.length === 1 && args[0] === "--help") return "help";
  let request: string | undefined;
  let output: string | undefined;
  let overwrite = false;
  for (let index = 0; index < args.length; index += 1) {
    const flag = args[index];
    if (flag === "--overwrite") {
      if (overwrite) usage("--overwrite was specified more than once");
      overwrite = true;
      continue;
    }
    if (flag !== "--request" && flag !== "--output") {
      usage(flag?.startsWith("--") ? `unknown flag '${flag}'` : `unexpected operand '${String(flag)}'`);
    }
    const value = args[index + 1];
    if (value === undefined || value.startsWith("--")) usage(`${flag} requires a value`);
    index += 1;
    if (flag === "--request") {
      if (request !== undefined) usage("--request was specified more than once");
      request = value;
    } else {
      if (output !== undefined) usage("--output was specified more than once");
      output = value;
    }
  }
  if (request === undefined) usage("--request is required");
  if (output === undefined) usage("--output is required");
  if (output === "-" && overwrite) usage("--overwrite cannot be used with --output -");
  return { request, output, overwrite };
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

async function build(args: BuildArguments): Promise<void> {
  const request = decodeRequest(await boundedBytes(args.request));
  let toml: string;
  try {
    toml = sessionBuilderFromRequest(request).toToml();
  } catch (error) {
    if (error instanceof MisoUsageError || error instanceof TypeError) {
      throw new CliFailure(3, "request.shape", error.message);
    }
    throw error;
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
  const receipt = {
    schemaVersion: 1,
    command: "session.build",
    output: {
      path: args.output,
      bytes: bytes.byteLength,
      sha256: createHash("sha256").update(bytes).digest("hex"),
    },
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
