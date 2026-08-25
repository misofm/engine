# Native vectorization certification v1

Issue #144 item 3. This document is the contract the `miso_engine_audit vectorization` subject
implements, and — more importantly — the honest statement of what its evidence does and does not
prove.

The failure this exists to catch has a name in this program's history: **SIMD that isn't.** The
previous engine shipped 79 "bank kernels" of which 42 were scalar loops in bank clothing, and every
test it had stayed green, because nothing it ran ever looked at an instruction. Before this subject,
V2's evidence for "the banks actually vectorize" was manual `objdump` spot checks in a reviewer's
checklist.

## Subjects

| Subject | What it is | What it proves |
| --- | --- | --- |
| Probe object and LLVM IR | `tools/miso-engine-vectorization-probes` built fresh with `--emit=llvm-ir,obj`, per backend | Every kernel family compiles to backend-width vector operations, with no scalar residue, no fast-math licence, and no math-library call |
| Family roster | `crates/miso-engine-lane/src/kernels{,/*}.rs` parsed for its public items | The registry and the engine agree on what the kernel families *are* |
| `libmiso_engine_capi.so` | The real C-ABI release cdylib | The production functions that instantiate the kernels are vectorized in the artifact that ships |
| `libmiso_engine_host_web.so` | The browser artifact's native twin: the same `miso-engine-host-web` crate, native target | The same, for the crate the AudioWorklet module is built from |

## The probes, and why they exist

Every kernel in `miso_engine_lane::kernels` is an `#[inline(always)]` generic body (D10). That is
deliberate — a block kernel only becomes the intended straight-line loop when the optimizer can
instantiate it at the consumer's width and see through the call — but it has one consequence for
evidence: **a lane kernel has no symbol of its own anywhere.** There is nothing named `svf_block` in
any object file this repository produces. It is inlined into whatever called it.

`tools/miso-engine-vectorization-probes` is the smallest construction that gives each family a name
without changing it: one `#[inline(never)]` wrapper per family, itself generic over `Lane`, calling
the production body with production arguments. Three properties are load-bearing:

- **One body per family, not one per width.** The wrappers are generic, so `probe_svf_block` at
  `Simd8` and at `Simd4` are the same source text and cannot drift apart.
- **Block lengths are compile-time constants.** `PROBE_WORDS` is 256, a multiple of both production
  widths, so every kernel's scalar tail is statically dead at every instantiation. Production keeps
  its tail — track counts are not multiples of the width — but a probe body has no reachable tail,
  which is what makes a surviving scalar floating-point instruction *in a probe* a real finding
  rather than the design working as intended.
- **Nothing here is reachable from a render path.** The crate depends only on `miso-engine-lane`, no
  shipped product depends on it, and it selects no backend.

## The rules

Every certified family carries a structural class in `vectorization-allowlist.tsv`, applied to both
the IR and the object:

| Class | Requires |
| --- | --- |
| `vector-arith` | at least one floating-point arithmetic operation at the backend vector width |
| `vector-compare` | at least one floating-point comparison at that width |
| `vector-any` | at least one operation on the backend vector type (a whole-lane move counts) |
| `no-float` | no floating-point arithmetic at all |

Every class additionally forbids, inside the named body:

- **scalar floating-point arithmetic** — including `vfmadd213ss`, the scalar fused multiply-add that
  shares six leading characters with the packed form and that an opcode-prefix scan misses;
- **narrower-than-backend vector arithmetic** — the "half the lanes" regression an eight-lane
  backend would otherwise report as vectorized;
- **fast-math flags** (`fast`, `nnan`, `ninf`, `nsz`, `arcp`, `contract`, `reassoc`, `afn`) — any of
  them would make the rendered bits a property of the optimizer rather than of the frozen operation
  order every fixture in the workspace is pinned to;
- **math-library and contraction intrinsics.** `@llvm.fma` is *permitted*: it is exactly what
  `Lane::fma` lowers to and D3 makes it the one allowed fusion. `@llvm.fmuladd` is forbidden,
  because it is the contractable form the backend may or may not fuse — a rounding the numeric
  contract does not allow to be optional.

## Completeness

`vectorization-families.tsv` is a closed roster of every public item in the three lane kernel
modules. The subject parses those sources and fails if the roster and the registry differ **in
either direction**: a kernel family added to the lane crate is uncertified until it is registered,
and a registered family the lane crate no longer exposes is a stale rule. Each registered family is
then either `certified`, and must carry a rule at *every* backend, or `exempt`, and must carry a
written reason. Today: 27 public kernels, 22 certified at 2 backends, 5 exempt (four mask
constructors and one slice-index helper, none of which performs arithmetic).

The roster scan strips comments and string literals and tracks brace depth, so `pub fn` inside a doc
comment, a block comment, a string, a function body or a nested `mod` is not a kernel. A kernel that
moves into a nested module *leaves* the roster and fails completeness rather than escaping
certification quietly.

## Binding to the shipped artifacts — and its limits

This is the part to read sceptically, because it is the part where a certification programme is most
tempted to overclaim.

**What is proven.** For each shipped product the subject reads the artifact as built and checks:

1. the declared render entry is a defined, exported symbol, defined exactly once;
2. its direct and GOT-indirect call closure resolves, and the size of that closure is reported;
3. each registered **kernel-host** symbol — a real production function that instantiates lane
   kernels, such as `<miso_engine_parametric_eq::Channel<..., 8>>::process_section` or
   `<miso_engine_builtins::BuiltinInputBankV1>::process` — is defined exactly once, performs at
   least a floor of vector arithmetic instructions at the backend width, is **vector dominated**
   (strictly more vector than scalar arithmetic), and calls no math library.

Rule 3 is the anti-"SIMD that isn't" gate on the shipping bytes: a bank that regressed to a scalar
loop would keep its symbol, keep its name, keep every unit test green, and fail vector dominance.

**What is not proven, and why.**

- **The binding is at the instantiating function, not at the kernel.** Because kernels are
  `#[inline(always)]`, no artifact contains a symbol for `svf_block`. The subject therefore certifies
  the production function the kernel was inlined *into*. That the two are the same code is an
  argument from build inputs — the probe build and the product build consume the same lane sources,
  the same `Cargo.lock`, the same `.cargo/config.toml` ISA pin, and the same toolchain, all of which
  the receipt hashes — not a machine-checked identity. **It is an argument, not a proof.**
- **The call closure stops at `dyn` dispatch.** The shipped render path crosses a
  `dyn PreparedPlanExecutor` boundary, which is a vtable load the disassembly cannot resolve. Direct
  calls and GOT-indirect calls *are* followed (the latter through the relocation table); register-
  indirect calls are counted as unresolved and reported as such. The closure size is evidence about
  the render spine, not a claim that the kernel hosts are reachable from it.
- **The kernel-host registry is curated, not derived.** It names seven production functions per
  product. It is not a proof that no other bank exists; a new effect crate whose bank is never
  registered is simply not covered. That is what the review question "is this bank in the shipped
  registry?" is for.
- **The floors are anti-regression thresholds, not measurements.** They sit well below the observed
  counts so that instruction-selection drift across a compiler bump does not manufacture a failure.
  The observed counts are in the report so drift stays *visible* without gating.
- **This is structural evidence only. It makes no performance claim.** An instruction is not a
  measurement.

## Backend matrix

| Backend | Width | How it is built | Status |
| --- | --- | --- | --- |
| `x86_64-avx2` | `Simd8` (one `__m256`) | Host build; the workspace `.cargo/config.toml` pins `+avx2,+fma` on `x86_64` | Certified in every run |
| `aarch64-neon` | `Simd4` (NEON, baseline on AArch64) | Cross build of the probe **rlib** with `--emit=llvm-ir,obj`; nothing is linked, so no cross linker is needed | Certified when the target standard library is installed; otherwise an explicit skip with a reason |

The AArch64 leg is guarded on `rustc --print target-libdir --target <t>` resolving to a real
directory. When it does not, the backend is reported as `"status": "skipped"` with the reason in the
report — never as a pass. A backend that is neither certified nor explicitly skipped is a failure;
this is the specific hole in the previous slice, where deleting every AArch64 row was green on x86.

The wasm `simd128` backend is *not* in this matrix. It is the same four-lane `Lane` implementation,
but its artifact is a WebAssembly module, and its vectorization evidence is the separate ratchet in
`scripts/check-web-audioworklet.sh` plus the 331-row cross-target digest corpus under wasmtime.

## Codegen configuration, stated rather than hidden

The shipped products are built with the release profile exactly as it ships (`lto = "fat"`,
`codegen-units = 1`, `panic = "abort"`, `debug = 1`) and the ISA pin from `.cargo/config.toml`.

The two **probe** builds differ in exactly one setting: `CARGO_PROFILE_RELEASE_LTO=false`. Under
`lto = "fat"` cargo makes an intermediate rlib emit LLVM bitcode instead of machine code, and there
is nothing to disassemble in bitcode. The kernels are `#[inline(always)]` generics instantiated
inside the probe crate itself, so cross-crate LTO does not participate in their code generation —
but the difference is real, it is recorded in the build manifest the receipt hashes, and it is the
reason the shipped-artifact half of this subject exists rather than being redundant with the probes.

## Receipt

Every run emits one JSON document hashing the complete chain: each lane kernel source, the probe
source, all three registries, `Cargo.lock`, `rust-toolchain.toml`, `.cargo/config.toml`, the build
manifest (compiler, cargo, disassembler and symbol-reader versions and the effective flags), each
emitted `.ll`, each emitted object, each shipped artifact, and the exact disassembly and symbol-table
bytes the rules were evaluated against. The sorted `path  sha256` list is itself hashed into
`receipt.chain_sha256`, so one value identifies the whole chain.

## Status: report, not gate

The CI job is non-blocking and uploads its receipt. The promotion criteria for making it a gate are
in `tools/miso-engine-audit/VECTORIZATION.md`.
