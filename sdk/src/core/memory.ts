import { ABI_LAYOUT } from "../generated/abi.ts";

/**
 * The imports one instance of the engine module needs: a fresh shared linear memory as
 * `env.memory` (issue #1380 D3).
 *
 * The module is built with atomics and imports its memory; it defines none of its own. The page
 * counts are the ABI layout document's `memory` rows, the limits the module's import declares, so
 * the memory this creates links and can grow to the module's whole maximum (512 MiB). Each call
 * makes a new memory: one instance runs on it, and nothing shares it. A shared `WebAssembly.Memory`
 * can be created on a page that is not cross-origin isolated (#1331a recipe rule 4), so this works
 * in every realm the SDK runs in.
 *
 * This is the SDK and Node realm's one instantiation helper; the shipped worklet has its own.
 */
export function sharedMemoryImports(): { readonly env: { readonly memory: WebAssembly.Memory } } {
  const memory = new WebAssembly.Memory({
    initial: ABI_LAYOUT.memory.initialPages,
    maximum: ABI_LAYOUT.memory.maximumPages,
    shared: true,
  });
  return { env: { memory } };
}
