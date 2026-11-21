import { ABI_LAYOUT } from "../generated/abi.ts";

const PAGE_BYTES = 65_536n;

/**
 * The pages a budget-capped memory declares for one worker-mode instance reservation (issue #2431),
 * which no budget charges: a second instance's 2 MiB stack with its canary and TLS (33 pages
 * through the allocator) and its leaked record (a 34th), plus one page. The worklet and the
 * worklet host carry the same count.
 */
const RESERVATION_PAGES = 35;

/**
 * The share of the budget a capped memory adds on top of it (issue #2431): one eighth. The budget
 * charges the engine's projected rows, not the allocator's chunk overhead or the fragmentation a
 * replacement leaves, and a live replacement does not add its candidate's control rows to its
 * aggregate peak. No gate yet measures an admitted session's growth near its budget (#2433), so
 * the cap keeps a proportional margin rather than a page of slack. The worklet and the worklet host
 * carry the same share.
 */
const MARGIN_DIVISOR = 8n;

/**
 * The maximum, in pages, of a shared memory whose instance boots with `maximumMemoryBytes`
 * (issue #2431).
 *
 * WebKit reserves a shared memory's whole maximum up front, since a shared buffer cannot move, and
 * iOS has little address space to reserve. So a memory declares only what its budget can use: the
 * initial pages (stack, static data, TLS), the budget rounded up to pages, an eighth of the budget
 * ({@link MARGIN_DIVISOR}) and {@link RESERVATION_PAGES}, never more than the module's own maximum.
 * The budget charges growth past the initial pages. An absent or zero budget selects the engine's
 * default, which is the module's maximum, and so does any value that is not a positive `bigint`:
 * the boot refuses that value itself, typed. A budget above the module's maximum clamps to it, and the boot refuses it with
 * `web.boot.memory_maximum`.
 */
export function sharedMemoryMaximumPages(maximumMemoryBytes?: bigint): number {
  const maximum = ABI_LAYOUT.memory.maximumPages;
  if (typeof maximumMemoryBytes !== "bigint" || maximumMemoryBytes <= 0n) return maximum;
  const marginBytes = MARGIN_DIVISOR * PAGE_BYTES;
  const pages = BigInt(ABI_LAYOUT.memory.initialPages + RESERVATION_PAGES)
    + (maximumMemoryBytes + PAGE_BYTES - 1n) / PAGE_BYTES
    + (maximumMemoryBytes + marginBytes - 1n) / marginBytes;
  return pages < BigInt(maximum) ? Number(pages) : maximum;
}

/**
 * The imports one instance of the engine module needs: a fresh shared linear memory as
 * `env.memory` (issue #1380 D3).
 *
 * The module is built with atomics and imports its memory; it defines none of its own. The initial
 * page count is the ABI layout document's `memory.initialPages`, the module import's minimum. The
 * maximum is {@link sharedMemoryMaximumPages} of the instance's boot budget: the module's whole
 * maximum (512 MiB) without one, less with one, which an imported memory may declare. Each call
 * makes a new memory: one instance runs on it, and nothing shares it. A shared `WebAssembly.Memory`
 * can be created on a page that is not cross-origin isolated (#1331a recipe rule 4), so this works
 * in every realm the SDK runs in.
 *
 * This is the SDK and Node realm's one instantiation helper; the shipped worklet has its own.
 */
export function sharedMemoryImports(
  maximumMemoryBytes?: bigint,
): { readonly env: { readonly memory: WebAssembly.Memory } } {
  const memory = new WebAssembly.Memory({
    initial: ABI_LAYOUT.memory.initialPages,
    maximum: sharedMemoryMaximumPages(maximumMemoryBytes),
    shared: true,
  });
  return { env: { memory } };
}
