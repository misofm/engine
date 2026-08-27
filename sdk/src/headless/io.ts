import { MisoOfflineError } from "./errors.js";

function copyBytes(bytes: Uint8Array): Uint8Array<ArrayBuffer> {
  const copy = new Uint8Array(bytes.byteLength);
  copy.set(bytes);
  return copy;
}

/** Read a package/file/HTTP asset without adding a runtime dependency. */
export async function readBytes(input: string | URL): Promise<Uint8Array> {
  const url = input instanceof URL ? input : (typeof input === "string" && /^[a-z][a-z0-9+.-]*:/i.test(input) ? new URL(input) : undefined);
  if (url && url.protocol !== "file:") {
    const response = await fetch(url);
    if (!response.ok) throw new MisoOfflineError(`Asset fetch failed with HTTP ${response.status}: ${url.href}`, "asset");
    return new Uint8Array(await response.arrayBuffer());
  }
  try {
    const { readFile } = await import("node:fs/promises");
    return copyBytes(await readFile(url ?? input));
  } catch (error) {
    throw new MisoOfflineError(`Asset read failed: ${String(input)}`, "asset", undefined, []);
  }
}

export async function sha256(bytes: Uint8Array): Promise<string> {
  const subtle = globalThis.crypto?.subtle;
  if (!subtle) throw new MisoOfflineError("Web Crypto SHA-256 is unavailable", "asset");
  const digest = new Uint8Array(await subtle.digest("SHA-256", copyBytes(bytes).buffer));
  return Array.from(digest, (byte) => byte.toString(16).padStart(2, "0")).join("");
}
