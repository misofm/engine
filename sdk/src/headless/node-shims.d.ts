declare module "node:fs/promises" {
  export function readFile(path: string | URL): Promise<Uint8Array>;
  export function writeFile(path: string | URL, data: Uint8Array): Promise<void>;
}
