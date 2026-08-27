declare module "node:fs/promises" {
  export function readFile(path: string | URL): Promise<Uint8Array>;
}

declare module "node:fs" {
  interface Stats { readonly size: number; isFile(): boolean; isSymbolicLink(): boolean; }
  export function closeSync(fd: number): void;
  export function fstatSync(fd: number): Stats;
  export function openSync(path: string, flags: string): number;
  export function readSync(fd: number, buffer: Uint8Array, offset: number, length: number, position: number): number;
  export function unlinkSync(path: string): void;
  export function writeSync(fd: number, buffer: Uint8Array, offset?: number, length?: number, position?: number | null): number;
}

declare module "node:crypto" {
  interface Hash { update(data: Uint8Array): Hash; digest(encoding: "hex"): string; }
  export function createHash(algorithm: "sha256"): Hash;
}
