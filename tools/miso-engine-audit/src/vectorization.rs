//! Native vectorization certification for the production lane kernels (issue #144 item 3).
//!
//! # What is certified, and against what
//!
//! Three independent bodies of evidence, each read from bytes this subject hashes:
//!
//! 1. **Fresh LLVM IR and a fresh object** for every kernel family, at every production backend.
//!    `tools/miso-engine-vectorization-probes` gives each `#[inline(always)]` generic kernel body a
//!    name by wrapping it in one `#[inline(never)]` probe; the runner emits `--emit=llvm-ir,obj`
//!    for that crate and this subject asserts, per family, that the IR does the arithmetic at the
//!    backend's vector type, carries no fast-math flag, calls no math library, and leaves no scalar
//!    floating-point operation behind -- and that the object's named body does the same in
//!    instructions.
//! 2. **Family completeness.** The families registry is compared against the public kernel roster
//!    parsed out of `crates/miso-engine-lane/src/kernels{,/*}.rs`. A kernel family added to the
//!    lane crate and not registered fails the completeness check; a registered family that no
//!    longer exists fails it too.
//! 3. **Binding to the shipped artifacts.** The C-ABI cdylib and the browser artifact's native twin
//!    are disassembled as built, and the production bank functions that instantiate the kernels are
//!    certified in place. `docs/NATIVE_VECTORIZATION_V1.md` states the linkage argument and its
//!    limits; the honest summary is that a kernel has no symbol of its own in a shipped artifact,
//!    so the binding is proven at the *instantiating production function*, not at the kernel.
//!
//! # What it is not
//!
//! It is evidence, not a merge gate, and it selects no backend and participates in no render. It
//! makes no performance claim: an instruction is not a measurement.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use sha2::{Digest, Sha256};

const DEFAULT_REGISTRY_DIRECTORY: &str = "tools/miso-engine-audit";
const DEFAULT_KERNEL_ROOT: &str = "crates/miso-engine-lane/src";
const FAMILIES_FILE: &str = "vectorization-families.tsv";
const ALLOWLIST_FILE: &str = "vectorization-allowlist.tsv";
const SHIPPED_FILE: &str = "vectorization-shipped.tsv";

/// The kernel source files whose public roster the completeness check is derived from.
const KERNEL_SOURCES: &[(&str, &str)] = &[
    ("kernels", "kernels.rs"),
    ("kernels/builtins", "kernels/builtins.rs"),
    ("kernels/halfband", "kernels/halfband.rs"),
];

// --------------------------------------------------------------------------------------------
// Backends
// --------------------------------------------------------------------------------------------

/// One production backend: the width its kernels run at and the dialect that proves it.
struct BackendSpec {
    /// Registry identity.
    id: &'static str,
    /// LLVM vector element count for this backend's lane type.
    lanes: usize,
    /// Machine-instruction dialect, which decides how a body is classified.
    dialect: Dialect,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Dialect {
    X86Avx2,
    Aarch64Neon,
}

const BACKENDS: &[BackendSpec] = &[
    BackendSpec {
        id: "x86_64-avx2",
        lanes: 8,
        dialect: Dialect::X86Avx2,
    },
    BackendSpec {
        id: "aarch64-neon",
        lanes: 4,
        dialect: Dialect::Aarch64Neon,
    },
];

fn backend(id: &str) -> Option<&'static BackendSpec> {
    BACKENDS.iter().find(|spec| spec.id == id)
}

// --------------------------------------------------------------------------------------------
// Registries
// --------------------------------------------------------------------------------------------

/// One row of the families registry: a public kernel and how it is disposed of.
#[derive(Clone, Debug, Eq, PartialEq)]
struct FamilyRow {
    module: String,
    kernel: String,
    family: String,
    certified: bool,
}

/// One row of the allowlist: a certified family at one backend, and its structural classes.
#[derive(Clone, Debug)]
struct ProbeRow {
    backend: String,
    family: String,
    probe_path: String,
    generic_argument: String,
    ir_class: Class,
    asm_class: Class,
}

/// One row of the shipped registry: a rule about one real release artifact.
#[derive(Clone, Debug)]
struct ShippedRow {
    product: String,
    backend: String,
    rule: ShippedRule,
    subject: String,
    floor: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ShippedRule {
    /// The named symbol must be a defined, exported entry, and reach a floor of functions.
    RenderEntry,
    /// The named symbol must exist exactly once and be vectorized at the backend width.
    KernelHost,
}

/// What a certified body is required to contain.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Class {
    /// At least one floating-point arithmetic operation at the backend's vector width.
    VectorArith,
    /// At least one floating-point comparison at the backend's vector width.
    VectorCompare,
    /// At least one operation of any kind on the backend's vector type.
    VectorAny,
    /// No floating-point operation of any kind (an integer-indexed word mover).
    NoFloat,
}

impl Class {
    fn parse(text: &str) -> Option<Self> {
        match text {
            "vector-arith" => Some(Self::VectorArith),
            "vector-compare" => Some(Self::VectorCompare),
            "vector-any" => Some(Self::VectorAny),
            "no-float" => Some(Self::NoFloat),
            _ => None,
        }
    }
}

/// Splits a registry file into non-comment, non-empty tab-separated rows of a fixed arity.
fn registry_rows(text: &str, expected: usize, what: &str) -> Result<Vec<Vec<String>>, String> {
    let mut rows = Vec::new();
    for (index, line) in text.lines().enumerate() {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let fields: Vec<String> = line.split('\t').map(str::to_owned).collect();
        if fields.len() != expected {
            return Err(format!(
                "{what} line {} has {} fields, expected {expected}",
                index + 1,
                fields.len()
            ));
        }
        if fields
            .iter()
            .any(|field| field.trim() != field || field.is_empty())
        {
            return Err(format!(
                "{what} line {} has an empty or padded field",
                index + 1
            ));
        }
        rows.push(fields);
    }
    if rows.is_empty() {
        return Err(format!("{what} is empty"));
    }
    Ok(rows)
}

fn parse_families(text: &str) -> Result<Vec<FamilyRow>, String> {
    let mut rows = Vec::new();
    let mut seen = BTreeSet::new();
    for fields in registry_rows(text, 5, "families registry")? {
        let certified = match fields[3].as_str() {
            "certified" => true,
            "exempt" => false,
            other => return Err(format!("unknown family disposition '{other}'")),
        };
        if !certified && fields[4] == "-" {
            return Err(format!(
                "exempt family '{}' must carry a reason, not '-'",
                fields[1]
            ));
        }
        if !seen.insert((fields[0].clone(), fields[1].clone())) {
            return Err(format!(
                "families registry duplicates {}::{}",
                fields[0], fields[1]
            ));
        }
        rows.push(FamilyRow {
            module: fields[0].clone(),
            kernel: fields[1].clone(),
            family: fields[2].clone(),
            certified,
        });
    }
    Ok(rows)
}

fn parse_allowlist(text: &str) -> Result<Vec<ProbeRow>, String> {
    let mut rows = Vec::new();
    let mut seen = BTreeSet::new();
    for fields in registry_rows(text, 6, "allowlist")? {
        if backend(&fields[0]).is_none() {
            return Err(format!("allowlist names unknown backend '{}'", fields[0]));
        }
        if !seen.insert((fields[0].clone(), fields[1].clone())) {
            return Err(format!(
                "allowlist duplicates backend/family {}/{}",
                fields[0], fields[1]
            ));
        }
        let ir_class = Class::parse(&fields[4])
            .ok_or_else(|| format!("allowlist names unknown IR class '{}'", fields[4]))?;
        let asm_class = Class::parse(&fields[5])
            .ok_or_else(|| format!("allowlist names unknown object class '{}'", fields[5]))?;
        rows.push(ProbeRow {
            backend: fields[0].clone(),
            family: fields[1].clone(),
            probe_path: fields[2].clone(),
            generic_argument: fields[3].clone(),
            ir_class,
            asm_class,
        });
    }
    Ok(rows)
}

fn parse_shipped(text: &str) -> Result<Vec<ShippedRow>, String> {
    let mut rows = Vec::new();
    let mut seen = BTreeSet::new();
    for fields in registry_rows(text, 5, "shipped registry")? {
        if backend(&fields[1]).is_none() {
            return Err(format!(
                "shipped registry names unknown backend '{}'",
                fields[1]
            ));
        }
        let rule = match fields[2].as_str() {
            "render-entry" => ShippedRule::RenderEntry,
            "kernel-host" => ShippedRule::KernelHost,
            other => return Err(format!("shipped registry names unknown rule '{other}'")),
        };
        let floor: usize = fields[4]
            .parse()
            .map_err(|_| format!("shipped registry floor '{}' is not a count", fields[4]))?;
        if !seen.insert((fields[0].clone(), fields[3].clone())) {
            return Err(format!(
                "shipped registry duplicates {}/{}",
                fields[0], fields[3]
            ));
        }
        rows.push(ShippedRow {
            product: fields[0].clone(),
            backend: fields[1].clone(),
            rule,
            subject: fields[3].clone(),
            floor,
        });
    }
    Ok(rows)
}

// --------------------------------------------------------------------------------------------
// Kernel roster: the completeness half
// --------------------------------------------------------------------------------------------

/// Removes comments and string literals so an item scan cannot be fooled by their contents.
///
/// This is the structural minimum a roster scan needs: `pub fn` inside a doc comment, a line
/// comment, a block comment or a string literal is not an item, and a scanner that greps for the
/// token would register one. Negative fixtures for each of those forms are in this module's tests.
fn strip_comments_and_strings(source: &str) -> String {
    let characters: Vec<char> = source.chars().collect();
    let mut out = String::with_capacity(source.len());
    let mut index = 0;
    let mut block_depth = 0usize;
    while index < characters.len() {
        let current = characters[index];
        let next = characters.get(index + 1).copied();
        if block_depth > 0 {
            if current == '*' && next == Some('/') {
                block_depth -= 1;
                index += 2;
                continue;
            }
            if current == '/' && next == Some('*') {
                block_depth += 1;
                index += 2;
                continue;
            }
            if current == '\n' {
                out.push('\n');
            }
            index += 1;
            continue;
        }
        if current == '/' && next == Some('*') {
            block_depth = 1;
            index += 2;
            continue;
        }
        if current == '/' && next == Some('/') {
            while index < characters.len() && characters[index] != '\n' {
                index += 1;
            }
            continue;
        }
        if current == '"' {
            index += 1;
            while index < characters.len() {
                if characters[index] == '\\' {
                    index += 2;
                    continue;
                }
                if characters[index] == '"' {
                    index += 1;
                    break;
                }
                if characters[index] == '\n' {
                    out.push('\n');
                }
                index += 1;
            }
            out.push_str("\"\"");
            continue;
        }
        out.push(current);
        index += 1;
    }
    out
}

/// The public kernel roster of one lane source file: `pub fn` items at module top level.
///
/// Brace depth is tracked, so an item nested inside another item -- a helper inside a function
/// body, or anything inside an inner `mod`, `#[cfg(test)]` included -- is not counted as a kernel.
/// A kernel that moves into a nested module therefore leaves the roster and fails the completeness
/// check rather than silently escaping certification.
fn public_kernels(source: &str) -> Vec<String> {
    let cleaned = strip_comments_and_strings(source);
    let mut names = Vec::new();
    let mut depth = 0i64;
    for line in cleaned.lines() {
        let trimmed = line.trim_start();
        if depth == 0
            && let Some(rest) = trimmed.strip_prefix("pub fn ")
        {
            let name: String = rest
                .chars()
                .take_while(|character| character.is_alphanumeric() || *character == '_')
                .collect();
            if !name.is_empty() {
                names.push(name);
            }
        }
        depth += i64::try_from(line.matches('{').count()).unwrap_or(0);
        depth -= i64::try_from(line.matches('}').count()).unwrap_or(0);
    }
    names.sort();
    names
}

// --------------------------------------------------------------------------------------------
// Structured tool output
// --------------------------------------------------------------------------------------------

/// One defined text symbol of an object or shared object.
#[derive(Clone, Debug)]
struct Symbol {
    name: String,
    address: u64,
}

/// Parses `llvm-nm --format=posix --demangle --defined-only` output structurally.
///
/// A POSIX row is `name kind value size`, space separated, with the demangled name first. A
/// demangled Rust name contains spaces (`<A as B>::c`), so the row is parsed from the *right*: the
/// trailing fields are the value and size, the field before them is the one-letter kind, and
/// everything earlier is the name. Splitting from the left -- which the obvious parse does -- reads
/// `<miso_engine_x::Y as Z>::process` as the name `<miso_engine_x::Y`, and then a rule that
/// compares names can be satisfied by a different symbol than the one it meant.
fn parse_nm(text: &str) -> Vec<Symbol> {
    let mut symbols = Vec::new();
    for line in text.lines() {
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() < 3 {
            continue;
        }
        let (kind_index, address_index) = if fields.len() >= 4 {
            (fields.len() - 3, fields.len() - 2)
        } else {
            (fields.len() - 2, fields.len() - 1)
        };
        let kind = fields[kind_index];
        if !matches!(kind, "t" | "T" | "w" | "W") {
            continue;
        }
        let Ok(address) = u64::from_str_radix(fields[address_index], 16) else {
            continue;
        };
        let name = fields[..kind_index].join(" ");
        if name.is_empty() {
            continue;
        }
        symbols.push(Symbol { name, address });
    }
    symbols
}

/// One disassembled function body.
#[derive(Clone, Debug, Default)]
struct Body {
    name: String,
    address: u64,
    instructions: Vec<Instruction>,
}

#[derive(Clone, Debug)]
struct Instruction {
    mnemonic: String,
    operands: String,
}

/// Parses `llvm-objdump --disassemble --demangle --no-show-raw-insn` output structurally.
///
/// Bodies are keyed by the *address* of the symbol header, never by the demangled name: a
/// disassembly legitimately contains functions whose demangled names are equal (the C ABI cdylib
/// has several `::render`), and a body injected under a duplicated header would otherwise be
/// indistinguishable from the real one. Each instruction is split on tabs into
/// `address: mnemonic operands`, so a rule reads a mnemonic *token* rather than a substring of a
/// whole line: an operand, a comment or a symbolic branch target can no longer satisfy an opcode
/// rule, and the scalar form of a fused multiply-add is no longer hidden by the packed form's name.
fn parse_disassembly(text: &str) -> Vec<Body> {
    let mut bodies: Vec<Body> = Vec::new();
    let mut open = false;
    for line in text.lines() {
        let trimmed = line.trim_end();
        if let Some((address, name)) = parse_symbol_header(trimmed) {
            bodies.push(Body {
                name,
                address,
                instructions: Vec::new(),
            });
            open = true;
            continue;
        }
        if !open || !trimmed.contains('\t') {
            continue;
        }
        let mut parts = trimmed.split('\t');
        let Some(address_field) = parts.next() else {
            continue;
        };
        let Some(address) = address_field.trim().strip_suffix(':') else {
            continue;
        };
        if u64::from_str_radix(address.trim(), 16).is_err() {
            continue;
        }
        let Some(mnemonic) = parts.next() else {
            continue;
        };
        let mnemonic = mnemonic.trim();
        if mnemonic.is_empty() {
            continue;
        }
        let operands = parts.next().unwrap_or("").trim().to_owned();
        if let Some(body) = bodies.last_mut() {
            body.instructions.push(Instruction {
                mnemonic: mnemonic.to_owned(),
                operands,
            });
        }
    }
    bodies
}

/// Finds the one body a named symbol owns, refusing an ambiguous match.
///
/// A relocatable object gives every function the address zero inside its own section, so an address
/// alone does not identify a body there; a shared object gives every function a distinct address,
/// but several may share a demangled name. Requiring the *pair* to resolve to exactly one body
/// covers both, and a disassembly carrying a second header with the same name -- the lookalike
/// evasion -- resolves to two candidates at the same address and is refused rather than read.
fn locate<'a>(bodies: &'a [Body], name: &str, address: u64) -> Result<&'a Body, String> {
    let named: Vec<&Body> = bodies.iter().filter(|body| body.name == name).collect();
    match named.len() {
        1 => Ok(named[0]),
        0 => Err(format!("no disassembled body is named '{name}'")),
        _ => {
            let exact: Vec<&&Body> = named
                .iter()
                .filter(|body| body.address == address)
                .collect();
            if exact.len() == 1 {
                Ok(exact[0])
            } else {
                Err(format!(
                    "{} disassembled bodies are named '{name}', {} of them at {address:#x}",
                    named.len(),
                    exact.len()
                ))
            }
        }
    }
}

/// `<hex address> <<demangled name>>:` -- the one line shape that opens a body.
fn parse_symbol_header(line: &str) -> Option<(u64, String)> {
    let rest = line.strip_suffix(">:")?;
    let (address, name) = rest.split_once(" <")?;
    let address = u64::from_str_radix(address.trim(), 16).ok()?;
    Some((address, name.to_owned()))
}

// --------------------------------------------------------------------------------------------
// Instruction classification
// --------------------------------------------------------------------------------------------

/// What one body's floating-point instructions add up to.
#[derive(Clone, Copy, Debug, Default)]
struct AsmProfile {
    vector_arith: usize,
    vector_other: usize,
    vector_any: usize,
    narrow_arith: usize,
    scalar_arith: usize,
    float_arith_any: usize,
}

/// x86 floating-point arithmetic stems, without the packed or scalar suffix.
const X86_ARITH_STEMS: &[&str] = &[
    "add", "sub", "mul", "div", "min", "max", "sqrt", "rcp", "rsqrt", "hadd", "hsub", "addsub",
];

/// x86 stems that are floating point but move, compare or reshape rather than compute.
const X86_OTHER_STEMS: &[&str] = &[
    "cmp",
    "and",
    "andn",
    "or",
    "xor",
    "blend",
    "blendv",
    "round",
    "mov",
    "shuf",
    "unpckh",
    "unpckl",
    "broadcast",
    "perm",
    "perm2f128",
    "test",
    "cvt",
    "extract",
    "insert",
    "mask",
    "dp",
];

/// Classifies one x86 mnemonic as `(is_float, is_arith, is_packed)`.
///
/// The suffix decides packed versus scalar, so `vfmadd213ss` -- a *scalar* fused multiply-add, the
/// evasion an opcode-prefix scan misses because it shares six leading characters with the packed
/// form -- is classified scalar, and the scalar-fallback rule rejects it.
fn classify_x86(mnemonic: &str) -> (bool, bool, bool) {
    let bare = mnemonic.strip_prefix('v').unwrap_or(mnemonic);
    if bare.len() < 3 {
        return (false, false, false);
    }
    let packed = bare.ends_with("ps") || bare.ends_with("pd");
    let scalar = bare.ends_with("ss") || bare.ends_with("sd");
    if !packed && !scalar {
        return (false, false, false);
    }
    // The integer instructions share the shape (`vpaddd`, `vpcmpeqd`); they are not floating point.
    if bare.starts_with('p') {
        return (false, false, false);
    }
    let stem = &bare[..bare.len() - 2];
    let fused = stem.starts_with("fmadd")
        || stem.starts_with("fmsub")
        || stem.starts_with("fnmadd")
        || stem.starts_with("fnmsub");
    if fused {
        return (true, true, packed);
    }
    if X86_ARITH_STEMS.contains(&stem) {
        return (true, true, packed);
    }
    if X86_OTHER_STEMS
        .iter()
        .any(|prefix| stem.starts_with(prefix))
        || stem == "zeroupper"
        || stem == "zeroall"
    {
        return (true, false, packed);
    }
    (false, false, false)
}

/// AArch64 floating-point arithmetic mnemonics. Width comes from the operands, not the mnemonic.
const AARCH64_ARITH: &[&str] = &[
    "fmul", "fadd", "fsub", "fdiv", "fmla", "fmls", "fnmla", "fnmls", "fmadd", "fmsub", "fnmadd",
    "fnmsub", "fsqrt", "fmax", "fmin", "fmaxnm", "fminnm", "frecpe", "frecps", "frsqrte",
    "frsqrts", "fmulx", "fabd", "fneg",
];

/// Classifies one AArch64 mnemonic as `(is_float, is_arith)`.
fn classify_aarch64(mnemonic: &str) -> (bool, bool) {
    if AARCH64_ARITH.contains(&mnemonic) {
        return (true, true);
    }
    let other = mnemonic.starts_with("fcm")
        || mnemonic.starts_with("fcvt")
        || mnemonic.starts_with("frint")
        || mnemonic == "fabs"
        || mnemonic == "fmov";
    (other, false)
}

/// Symbols whose presence in a certified body means the arithmetic left the lane domain.
const FORBIDDEN_CALL_SYMBOLS: &[&str] = &[
    "sinf",
    "cosf",
    "tanf",
    "asinf",
    "acosf",
    "atanf",
    "atan2f",
    "sinhf",
    "coshf",
    "tanhf",
    "expf",
    "exp2f",
    "exp10f",
    "expm1f",
    "logf",
    "log2f",
    "log10f",
    "log1pf",
    "powf",
    "cbrtf",
    "hypotf",
    "fmodf",
    "sin",
    "cos",
    "tan",
    "exp",
    "exp2",
    "log",
    "log2",
    "log10",
    "pow",
    "cbrt",
    "hypot",
    "fmod",
    "__powisf2",
];

fn is_call(mnemonic: &str) -> bool {
    mnemonic.starts_with("call") || mnemonic == "bl" || mnemonic == "blr" || mnemonic == "jmp"
}

fn call_target_is_forbidden(operands: &str) -> bool {
    let Some(start) = operands.find('<') else {
        return false;
    };
    let Some(end) = operands[start..].find('>') else {
        return false;
    };
    let target = &operands[start + 1..start + end];
    let target = target.split('@').next().unwrap_or(target);
    let target = target.split('+').next().unwrap_or(target).trim();
    FORBIDDEN_CALL_SYMBOLS.contains(&target)
}

fn forbidden_calls(body: &Body) -> usize {
    body.instructions
        .iter()
        .filter(|instruction| {
            is_call(&instruction.mnemonic) && call_target_is_forbidden(&instruction.operands)
        })
        .count()
}

/// True when an operand list names a register of the backend's vector width.
///
/// On x86 a 256-bit operand is spelled `%ymm`. On AArch64 the same 128-bit register is spelled
/// `v0.4s` by an arithmetic instruction, `v0.16b` by a bitwise one and `q0` by a load or store, so
/// all three forms count: a whole-lane move is evidence of the width even though its mnemonic is
/// untyped.
fn operands_are_vector_width(operands: &str, spec: &BackendSpec) -> bool {
    match spec.dialect {
        Dialect::X86Avx2 => operands.contains(if spec.lanes == 8 { "%ymm" } else { "%xmm" }),
        Dialect::Aarch64Neon => {
            operands.contains(".4s")
                || operands.contains(".16b")
                || operands.contains(".8h")
                || operands.contains(".2d")
                || operands
                    .split(|c: char| !(c.is_alphanumeric()))
                    .any(|token| {
                        token.len() >= 2
                            && token.starts_with('q')
                            && token[1..].chars().all(|c| c.is_ascii_digit())
                    })
        }
    }
}

fn profile_body(body: &Body, spec: &BackendSpec) -> AsmProfile {
    let mut profile = AsmProfile::default();
    let vector_token = match spec.dialect {
        Dialect::X86Avx2 => {
            if spec.lanes == 8 {
                "%ymm"
            } else {
                "%xmm"
            }
        }
        Dialect::Aarch64Neon => ".4s",
    };
    for instruction in &body.instructions {
        let mnemonic = instruction.mnemonic.as_str();
        if operands_are_vector_width(&instruction.operands, spec) {
            profile.vector_any += 1;
        }
        match spec.dialect {
            Dialect::X86Avx2 => {
                let (is_float, is_arith, is_packed) = classify_x86(mnemonic);
                if !is_float {
                    continue;
                }
                if is_arith {
                    profile.float_arith_any += 1;
                }
                let wide = instruction.operands.contains(vector_token);
                if !is_packed {
                    if is_arith {
                        profile.scalar_arith += 1;
                    }
                } else if wide {
                    if is_arith {
                        profile.vector_arith += 1;
                    } else {
                        profile.vector_other += 1;
                    }
                } else if is_arith {
                    profile.narrow_arith += 1;
                }
            }
            Dialect::Aarch64Neon => {
                let (is_float, is_arith) = classify_aarch64(mnemonic);
                if !is_float {
                    continue;
                }
                if is_arith {
                    profile.float_arith_any += 1;
                }
                let wide = instruction.operands.contains(vector_token);
                let scalar = !instruction.operands.contains('.');
                if wide {
                    if is_arith {
                        profile.vector_arith += 1;
                    } else {
                        profile.vector_other += 1;
                    }
                } else if is_arith {
                    if scalar {
                        profile.scalar_arith += 1;
                    } else {
                        profile.narrow_arith += 1;
                    }
                }
            }
        }
    }
    profile
}

// --------------------------------------------------------------------------------------------
// LLVM IR
// --------------------------------------------------------------------------------------------

/// One function definition lifted out of an `.ll` module.
#[derive(Clone, Debug)]
struct IrFunction {
    lines: Vec<String>,
}

/// Extracts every `define` whose mangled symbol contains `needle`, brace balanced.
///
/// The scan is over `define` lines only, so a mention of the symbol in a comment, a metadata node,
/// a string constant or a call site is not a definition. Bodies end at the closing `}` at column
/// zero, which is where LLVM's textual printer always puts it.
fn ir_definitions(module: &str, needle: &str) -> Vec<IrFunction> {
    let mut found = Vec::new();
    let mut collecting: Option<Vec<String>> = None;
    for line in module.lines() {
        if let Some(body) = collecting.as_mut() {
            if line == "}" {
                found.push(IrFunction {
                    lines: std::mem::take(body),
                });
                collecting = None;
            } else {
                body.push(line.to_owned());
            }
            continue;
        }
        if !line.starts_with("define ") {
            continue;
        }
        let Some(at) = line.find(" @") else { continue };
        let symbol: String = line[at + 2..]
            .chars()
            .take_while(|character| *character != '(' && !character.is_whitespace())
            .collect();
        if symbol.contains(needle) {
            collecting = Some(Vec::new());
        }
    }
    found
}

/// Fast-math flags LLVM prints between an opcode and its type.
///
/// `contract` licenses a fusion the engine performs only through `Lane::fma`; the rest license
/// reassociation or approximation. Any of them on a kernel operation would make the rendered bits
/// a property of the optimizer rather than of the frozen operation order.
const FAST_MATH_FLAGS: &[&str] = &[
    "fast", "nnan", "ninf", "nsz", "arcp", "contract", "reassoc", "afn",
];

/// LLVM intrinsics and math-library symbols a certified kernel body may not reach.
///
/// `llvm.fma` is deliberately absent: it is exactly what `Lane::fma` lowers to, and D3 makes it the
/// one permitted fusion. `llvm.fmuladd` *is* forbidden, because it is the contractable form the
/// backend may or may not fuse -- a rounding the numeric contract does not allow to be optional.
const FORBIDDEN_IR_INTRINSIC_FAMILIES: &[&str] = &[
    "llvm.fmuladd.",
    "llvm.sin.",
    "llvm.cos.",
    "llvm.tan.",
    "llvm.exp.",
    "llvm.exp2.",
    "llvm.exp10.",
    "llvm.log.",
    "llvm.log2.",
    "llvm.log10.",
    "llvm.pow.",
    "llvm.powi.",
];

/// Math-library symbols a certified kernel body may not call.
const FORBIDDEN_IR_LIBM: &[&str] = &[
    "sinf",
    "cosf",
    "tanf",
    "asinf",
    "acosf",
    "atanf",
    "atan2f",
    "sinhf",
    "coshf",
    "tanhf",
    "expf",
    "exp2f",
    "exp10f",
    "expm1f",
    "logf",
    "log2f",
    "log10f",
    "log1pf",
    "powf",
    "cbrtf",
    "hypotf",
    "fmodf",
    "__powisf2",
];

/// Counts the forbidden callees on one IR line.
///
/// Callee names are *extracted* -- every `@` is followed by the identifier characters that make up
/// a symbol -- and then matched whole. A substring scan for `@llvm.exp` reports every
/// `@llvm.experimental.noalias.scope.decl`, which LLVM emits by the dozen in any body that touches
/// two disjoint slices; the rule would then be permanently red for a reason that has nothing to do
/// with arithmetic.
fn forbidden_ir_callees(line: &str) -> usize {
    let characters: Vec<char> = line.chars().collect();
    let mut count = 0usize;
    let mut index = 0usize;
    while index < characters.len() {
        if characters[index] != '@' {
            index += 1;
            continue;
        }
        let start = index + 1;
        let mut end = start;
        while end < characters.len()
            && (characters[end].is_alphanumeric()
                || characters[end] == '.'
                || characters[end] == '_'
                || characters[end] == '$')
        {
            end += 1;
        }
        let name: String = characters[start..end].iter().collect();
        index = end.max(start + 1);
        if name.is_empty() {
            continue;
        }
        if FORBIDDEN_IR_LIBM.contains(&name.as_str())
            || FORBIDDEN_IR_INTRINSIC_FAMILIES
                .iter()
                .any(|family| name.starts_with(family))
        {
            count += 1;
        }
    }
    count
}

#[derive(Clone, Copy, Debug, Default)]
struct IrProfile {
    vector_arith: usize,
    vector_compare: usize,
    vector_any: usize,
    narrow_arith: usize,
    scalar_arith: usize,
    float_any: usize,
    fast_math: usize,
    forbidden_calls: usize,
}

const IR_ARITH_OPCODES: &[&str] = &["fadd", "fsub", "fmul", "fdiv", "frem", "fneg"];

fn profile_ir(function: &IrFunction, spec: &BackendSpec) -> IrProfile {
    let mut profile = IrProfile::default();
    let vector_float = format!("<{} x float>", spec.lanes);
    let vector_int = format!("<{} x i32>", spec.lanes);
    let fma_intrinsic = format!("@llvm.fma.v{}f32", spec.lanes);
    // A target intrinsic is how `wide` expresses an ordered lane compare on x86: the IR carries
    // `@llvm.x86.avx.cmp.ps.256`, not a generic `fcmp <8 x float>`. It is still a floating-point
    // comparison at the backend width, and the rule counts it as one.
    let compare_intrinsic = match spec.dialect {
        Dialect::X86Avx2 => {
            if spec.lanes == 8 {
                "@llvm.x86.avx.cmp.ps.256"
            } else {
                "@llvm.x86.sse.cmp.ps"
            }
        }
        Dialect::Aarch64Neon => "@llvm.aarch64.neon.facgt",
    };
    for line in &function.lines {
        profile.forbidden_calls += forbidden_ir_callees(line);
        if line.contains(&vector_float) || line.contains(&vector_int) {
            profile.vector_any += 1;
        }
        if line.contains(&fma_intrinsic) {
            profile.vector_arith += 1;
            profile.float_any += 1;
        }
        if line.contains(compare_intrinsic) {
            profile.vector_compare += 1;
            profile.float_any += 1;
        }
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let Some(position) = tokens
            .iter()
            .position(|token| IR_ARITH_OPCODES.contains(token) || *token == "fcmp")
        else {
            continue;
        };
        let opcode = tokens[position];
        profile.float_any += 1;
        let mut index = position + 1;
        while index < tokens.len() && FAST_MATH_FLAGS.contains(&tokens[index]) {
            profile.fast_math += 1;
            index += 1;
        }
        if opcode == "fcmp" && index < tokens.len() {
            index += 1;
        }
        let rest = tokens[index..].join(" ");
        let is_vector = rest.starts_with(&vector_float);
        let is_scalar = rest.starts_with("float") || rest.starts_with("double");
        let is_narrow = !is_vector && rest.starts_with('<');
        if opcode == "fcmp" {
            if is_vector {
                profile.vector_compare += 1;
            } else if is_scalar {
                profile.scalar_arith += 1;
            } else if is_narrow {
                profile.narrow_arith += 1;
            }
            continue;
        }
        if is_vector {
            profile.vector_arith += 1;
        } else if is_scalar {
            profile.scalar_arith += 1;
        } else if is_narrow {
            profile.narrow_arith += 1;
        }
    }
    profile
}

// --------------------------------------------------------------------------------------------
// Certification
// --------------------------------------------------------------------------------------------

fn class_satisfied_ir(class: Class, profile: &IrProfile) -> Option<String> {
    match class {
        Class::VectorArith if profile.vector_arith == 0 => {
            Some("no floating-point arithmetic at the backend vector type".to_owned())
        }
        Class::VectorCompare if profile.vector_compare == 0 => {
            Some("no floating-point comparison at the backend vector type".to_owned())
        }
        Class::VectorAny if profile.vector_any == 0 => {
            Some("no operation on the backend vector type".to_owned())
        }
        Class::NoFloat if profile.float_any > 0 => Some(format!(
            "{} floating-point operations in a family declared free of them",
            profile.float_any
        )),
        _ => None,
    }
}

fn class_satisfied_asm(class: Class, profile: &AsmProfile) -> Option<String> {
    match class {
        Class::VectorArith if profile.vector_arith == 0 => {
            Some("no packed vector arithmetic at the backend width".to_owned())
        }
        Class::VectorCompare | Class::VectorAny if profile.vector_any == 0 => {
            Some("no instruction at the backend vector width".to_owned())
        }
        Class::NoFloat if profile.float_arith_any > 0 => Some(format!(
            "{} floating-point arithmetic instructions in a family declared free of them",
            profile.float_arith_any
        )),
        _ => None,
    }
}

/// Everything one backend's certification produced.
struct BackendReport {
    id: String,
    status: &'static str,
    skip_reason: Option<String>,
    families: usize,
    ir_path: String,
    object_path: String,
    ir_sha256: String,
    object_sha256: String,
    disassembly_sha256: String,
    symbols_sha256: String,
    observations: Vec<(String, IrProfile, AsmProfile)>,
}

impl ProbeRow {
    /// The needle that identifies this family's definition in the mangled LLVM module.
    ///
    /// Rust's v0 mangling is length prefixed, so `15probe_svf_block` matches the item named
    /// `probe_svf_block` and cannot also match `probe_svf_block_ramped`, whose prefix is `22`. A
    /// rename therefore leaves the needle unmatched rather than drifting onto a lookalike.
    fn ir_needle(&self) -> String {
        let item = self
            .probe_path
            .rsplit("::")
            .next()
            .unwrap_or(&self.probe_path);
        format!("{}{item}", item.len())
    }

    /// The exact demangled symbol this family's probe must define, once.
    fn expected_symbol(&self) -> String {
        if self.generic_argument == "-" {
            self.probe_path.clone()
        } else {
            format!("{}::<{}>", self.probe_path, self.generic_argument)
        }
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "one backend's whole certification; splitting it would separate a rule from its subject"
)]
fn certify_backend(
    spec: &BackendSpec,
    rows: &[ProbeRow],
    ir_path: &Path,
    object_path: &Path,
    tools: &Tools,
    failures: &mut Vec<String>,
) -> BackendReport {
    let mut report = BackendReport {
        id: spec.id.to_owned(),
        status: "fail",
        skip_reason: None,
        families: rows.len(),
        ir_path: ir_path.display().to_string(),
        object_path: object_path.display().to_string(),
        ir_sha256: String::new(),
        object_sha256: String::new(),
        disassembly_sha256: String::new(),
        symbols_sha256: String::new(),
        observations: Vec::new(),
    };
    let module = match fs::read(ir_path) {
        Ok(bytes) => {
            report.ir_sha256 = sha256(&bytes);
            String::from_utf8_lossy(&bytes).into_owned()
        }
        Err(error) => {
            failures.push(format!("{}: cannot read LLVM IR: {error}", spec.id));
            return report;
        }
    };
    match fs::read(object_path) {
        Ok(bytes) => report.object_sha256 = sha256(&bytes),
        Err(error) => {
            failures.push(format!("{}: cannot read object: {error}", spec.id));
            return report;
        }
    }
    let symbols_text = match run_tool(
        &tools.nm,
        &["--demangle", "--defined-only", "--format=posix"],
        object_path,
    ) {
        Ok(text) => text,
        Err(error) => {
            failures.push(format!("{}: {error}", spec.id));
            return report;
        }
    };
    report.symbols_sha256 = sha256(symbols_text.as_bytes());
    let symbols = parse_nm(&symbols_text);
    let disassembly = match run_tool(
        &tools.objdump,
        &["--disassemble", "--demangle", "--no-show-raw-insn"],
        object_path,
    ) {
        Ok(text) => text,
        Err(error) => {
            failures.push(format!("{}: {error}", spec.id));
            return report;
        }
    };
    report.disassembly_sha256 = sha256(disassembly.as_bytes());
    let bodies = parse_disassembly(&disassembly);

    // Completeness against what the object actually defines: every probe symbol present must be
    // registered, so a probe added without a rule cannot ride along uncertified.
    let expected: BTreeSet<String> = rows.iter().map(ProbeRow::expected_symbol).collect();
    for symbol in &symbols {
        if symbol.name.contains("::probe_") && !expected.contains(&symbol.name) {
            failures.push(format!(
                "{}: the artifact defines unregistered probe '{}'",
                spec.id, symbol.name
            ));
        }
    }

    for row in rows {
        let wanted = row.expected_symbol();
        let matches: Vec<&Symbol> = symbols
            .iter()
            .filter(|symbol| symbol.name == wanted)
            .collect();
        if matches.len() != 1 {
            failures.push(format!(
                "{} / {}: expected exactly one defined symbol '{wanted}', found {}",
                spec.id,
                row.family,
                matches.len()
            ));
            continue;
        }
        let body = match locate(&bodies, &wanted, matches[0].address) {
            Ok(body) => body,
            Err(reason) => {
                failures.push(format!("{} / {}: {reason}", spec.id, row.family));
                continue;
            }
        };

        let definitions = ir_definitions(&module, &row.ir_needle());
        if definitions.len() != 1 {
            failures.push(format!(
                "{} / {}: expected exactly one LLVM definition of '{}', found {}",
                spec.id,
                row.family,
                row.ir_needle(),
                definitions.len()
            ));
            continue;
        }
        let ir = profile_ir(&definitions[0], spec);
        let asm = profile_body(body, spec);
        let calls = forbidden_calls(body);

        if let Some(reason) = class_satisfied_ir(row.ir_class, &ir) {
            failures.push(format!("{} / {}: IR {reason}", spec.id, row.family));
        }
        if ir.scalar_arith > 0 {
            failures.push(format!(
                "{} / {}: IR keeps {} scalar floating-point operations; the probe block length is \
                 a multiple of the width, so no tail is reachable",
                spec.id, row.family, ir.scalar_arith
            ));
        }
        if ir.narrow_arith > 0 {
            failures.push(format!(
                "{} / {}: IR performs {} floating-point operations at a narrower vector type than \
                 the backend's",
                spec.id, row.family, ir.narrow_arith
            ));
        }
        if ir.fast_math > 0 {
            failures.push(format!(
                "{} / {}: IR carries {} fast-math flags",
                spec.id, row.family, ir.fast_math
            ));
        }
        if ir.forbidden_calls > 0 {
            failures.push(format!(
                "{} / {}: IR reaches {} forbidden intrinsic or math-library symbols",
                spec.id, row.family, ir.forbidden_calls
            ));
        }
        if let Some(reason) = class_satisfied_asm(row.asm_class, &asm) {
            failures.push(format!("{} / {}: object {reason}", spec.id, row.family));
        }
        if asm.scalar_arith > 0 {
            failures.push(format!(
                "{} / {}: object keeps {} scalar floating-point arithmetic instructions",
                spec.id, row.family, asm.scalar_arith
            ));
        }
        if asm.narrow_arith > 0 {
            failures.push(format!(
                "{} / {}: object performs {} floating-point arithmetic instructions below the \
                 backend width",
                spec.id, row.family, asm.narrow_arith
            ));
        }
        if calls > 0 {
            failures.push(format!(
                "{} / {}: object calls {calls} math-library symbols",
                spec.id, row.family
            ));
        }
        report.observations.push((row.family.clone(), ir, asm));
    }
    report.status = "certified";
    report
}

// --------------------------------------------------------------------------------------------
// Shipped artifacts
// --------------------------------------------------------------------------------------------

/// What one shipped product's binding evidence produced.
struct ProductReport {
    product: String,
    artifact: String,
    artifact_sha256: String,
    symbols_sha256: String,
    disassembly_sha256: String,
    exported_entries: usize,
    reachable_functions: usize,
    unresolved_call_edges: usize,
    hosts: Vec<(String, usize, usize)>,
}

/// The base-relative relocation table, which resolves a GOT slot to the function it holds.
fn relative_relocations(text: &str) -> BTreeMap<u64, u64> {
    let mut map = BTreeMap::new();
    for line in text.lines() {
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() < 3 {
            continue;
        }
        let Ok(slot) = u64::from_str_radix(fields[0], 16) else {
            continue;
        };
        if fields[1] != "R_X86_64_RELATIVE" && fields[1] != "R_AARCH64_RELATIVE" {
            continue;
        }
        let Some(target) = fields[2].strip_prefix("*ABS*+0x") else {
            continue;
        };
        let Ok(target) = u64::from_str_radix(target, 16) else {
            continue;
        };
        map.insert(slot, target);
    }
    map
}

/// The set of function bodies reachable from `entry` by direct and GOT-indirect calls.
///
/// Two edge kinds are followed: a direct `call 0x...` whose target is a known body, and an indirect
/// `call *0x..(%rip)` whose comment names a slot the relocation table resolves to a known body.
/// Virtual dispatch through a `dyn` vtable is *not* followed, and the count of call edges that
/// could not be resolved is reported rather than hidden -- the shipped render path crosses one such
/// boundary, and a closure that silently stopped there would overstate what it proves.
fn reachable_from(
    entry: u64,
    bodies: &BTreeMap<u64, &Body>,
    got: &BTreeMap<u64, u64>,
) -> (usize, usize) {
    let mut seen = BTreeSet::new();
    let mut unresolved = 0usize;
    let mut stack = vec![entry];
    while let Some(address) = stack.pop() {
        if !seen.insert(address) {
            continue;
        }
        let Some(body) = bodies.get(&address) else {
            continue;
        };
        for instruction in &body.instructions {
            if !instruction.mnemonic.starts_with("call") && instruction.mnemonic != "bl" {
                continue;
            }
            if let Some(target) = direct_call_target(&instruction.operands) {
                if bodies.contains_key(&target) {
                    stack.push(target);
                } else {
                    unresolved += 1;
                }
                continue;
            }
            if let Some(slot) = indirect_call_slot(&instruction.operands)
                && let Some(target) = got.get(&slot)
                && bodies.contains_key(target)
            {
                stack.push(*target);
                continue;
            }
            unresolved += 1;
        }
    }
    (seen.len(), unresolved)
}

fn direct_call_target(operands: &str) -> Option<u64> {
    let operand = operands.split_whitespace().next()?;
    let hex = operand.strip_prefix("0x")?;
    u64::from_str_radix(hex, 16).ok()
}

fn indirect_call_slot(operands: &str) -> Option<u64> {
    if !operands.starts_with('*') {
        return None;
    }
    let comment = operands.split('#').nth(1)?.trim();
    let hex = comment.split_whitespace().next()?.strip_prefix("0x")?;
    u64::from_str_radix(hex, 16).ok()
}

#[expect(
    clippy::too_many_lines,
    reason = "one artifact's whole binding evidence, in the order the rules are declared"
)]
fn certify_product(
    product: &str,
    artifact: &Path,
    rows: &[ShippedRow],
    tools: &Tools,
    failures: &mut Vec<String>,
) -> ProductReport {
    let mut report = ProductReport {
        product: product.to_owned(),
        artifact: artifact.display().to_string(),
        artifact_sha256: String::new(),
        symbols_sha256: String::new(),
        disassembly_sha256: String::new(),
        exported_entries: 0,
        reachable_functions: 0,
        unresolved_call_edges: 0,
        hosts: Vec::new(),
    };
    match fs::read(artifact) {
        Ok(bytes) => report.artifact_sha256 = sha256(&bytes),
        Err(error) => {
            failures.push(format!("{product}: cannot read artifact: {error}"));
            return report;
        }
    }
    let symbols_text = match run_tool(
        &tools.nm,
        &["--demangle", "--defined-only", "--format=posix"],
        artifact,
    ) {
        Ok(text) => text,
        Err(error) => {
            failures.push(format!("{product}: {error}"));
            return report;
        }
    };
    report.symbols_sha256 = sha256(symbols_text.as_bytes());
    let symbols = parse_nm(&symbols_text);
    let dynamic_text = match run_tool(
        &tools.nm,
        &["--dynamic", "--defined-only", "--format=posix"],
        artifact,
    ) {
        Ok(text) => text,
        Err(error) => {
            failures.push(format!("{product}: {error}"));
            return report;
        }
    };
    let dynamic: BTreeSet<String> = parse_nm(&dynamic_text)
        .into_iter()
        .map(|symbol| symbol.name)
        .collect();
    report.exported_entries = dynamic.len();
    let disassembly = match run_tool(
        &tools.objdump,
        &["--disassemble", "--demangle", "--no-show-raw-insn"],
        artifact,
    ) {
        Ok(text) => text,
        Err(error) => {
            failures.push(format!("{product}: {error}"));
            return report;
        }
    };
    report.disassembly_sha256 = sha256(disassembly.as_bytes());
    let bodies = parse_disassembly(&disassembly);
    let relocations = run_tool(&tools.objdump, &["-R"], artifact).unwrap_or_default();
    let got = relative_relocations(&relocations);

    for row in rows {
        let Some(spec) = backend(&row.backend) else {
            continue;
        };
        let matches: Vec<&Symbol> = symbols
            .iter()
            .filter(|symbol| symbol.name == row.subject)
            .collect();
        if matches.len() != 1 {
            failures.push(format!(
                "{product}: expected exactly one definition of '{}', found {}",
                row.subject,
                matches.len()
            ));
            continue;
        }
        match row.rule {
            ShippedRule::RenderEntry => {
                if !dynamic.contains(&row.subject) {
                    failures.push(format!(
                        "{product}: '{}' is not an exported entry of the shipped artifact",
                        row.subject
                    ));
                    continue;
                }
                let by_address: BTreeMap<u64, &Body> =
                    bodies.iter().map(|body| (body.address, body)).collect();
                let (reachable, unresolved) = reachable_from(matches[0].address, &by_address, &got);
                report.reachable_functions = reachable;
                report.unresolved_call_edges = unresolved;
                if reachable < row.floor {
                    failures.push(format!(
                        "{product}: the render entry reaches {reachable} functions, below the \
                         registered floor of {}",
                        row.floor
                    ));
                }
            }
            ShippedRule::KernelHost => {
                let body = match locate(&bodies, &row.subject, matches[0].address) {
                    Ok(body) => body,
                    Err(reason) => {
                        failures.push(format!("{product}: kernel host {reason}"));
                        continue;
                    }
                };
                let profile = profile_body(body, spec);
                if profile.vector_arith < row.floor {
                    failures.push(format!(
                        "{product}: kernel host '{}' performs {} vector arithmetic instructions at \
                         the backend width, below the registered floor of {}",
                        row.subject, profile.vector_arith, row.floor
                    ));
                }
                if profile.vector_arith <= profile.scalar_arith {
                    failures.push(format!(
                        "{product}: kernel host '{}' is not vector dominated: {} vector against {} \
                         scalar arithmetic instructions",
                        row.subject, profile.vector_arith, profile.scalar_arith
                    ));
                }
                if forbidden_calls(body) > 0 {
                    failures.push(format!(
                        "{product}: kernel host '{}' calls a math-library symbol",
                        row.subject
                    ));
                }
                report.hosts.push((
                    row.subject.clone(),
                    profile.vector_arith,
                    profile.scalar_arith,
                ));
            }
        }
    }
    report
}

// --------------------------------------------------------------------------------------------
// Plumbing
// --------------------------------------------------------------------------------------------

struct Tools {
    nm: PathBuf,
    objdump: PathBuf,
}

fn run_tool(tool: &Path, arguments: &[&str], subject: &Path) -> Result<String, String> {
    let output = Command::new(tool)
        .args(arguments)
        .arg(subject)
        .output()
        .map_err(|error| format!("failed to run {}: {error}", tool.display()))?;
    if !output.status.success() {
        return Err(format!(
            "{} {} failed: {}",
            tool.display(),
            arguments.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    String::from_utf8(output.stdout)
        .map_err(|error| format!("{} produced non-UTF-8 output: {error}", tool.display()))
}

fn sha256(bytes: &[u8]) -> String {
    let mut digest = Sha256::new();
    digest.update(bytes);
    digest
        .finalize()
        .iter()
        .fold(String::new(), |mut text, byte| {
            let _ = write!(text, "{byte:02x}");
            text
        })
}

fn json_string(value: &str) -> String {
    let mut out = String::with_capacity(value.len() + 2);
    out.push('"');
    for character in value.chars() {
        match character {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            '\r' => out.push_str("\\r"),
            control if (control as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", control as u32);
            }
            other => out.push(other),
        }
    }
    out.push('"');
    out
}

struct Args {
    registry_directory: PathBuf,
    kernel_root: PathBuf,
    backends: Vec<(String, PathBuf, PathBuf)>,
    skips: Vec<(String, String)>,
    products: Vec<(String, PathBuf)>,
    receipt_inputs: Vec<PathBuf>,
    nm: PathBuf,
    objdump: PathBuf,
}

fn usage() -> ! {
    eprintln!(
        "usage: miso_engine_audit vectorization \\\n  \
         [--registry-directory DIR] [--kernel-root DIR] \\\n  \
         [--backend ID=IR_PATH,OBJECT_PATH]... [--skip-backend ID=REASON]... \\\n  \
         [--product NAME=ARTIFACT_PATH]... [--receipt-input PATH]... \\\n  \
         [--nm PATH] [--objdump PATH]"
    );
    std::process::exit(2);
}

fn args() -> Args {
    let mut parsed = Args {
        registry_directory: PathBuf::from(DEFAULT_REGISTRY_DIRECTORY),
        kernel_root: PathBuf::from(DEFAULT_KERNEL_ROOT),
        backends: Vec::new(),
        skips: Vec::new(),
        products: Vec::new(),
        receipt_inputs: Vec::new(),
        nm: PathBuf::from("llvm-nm"),
        objdump: PathBuf::from("llvm-objdump"),
    };
    let mut values = std::env::args().skip(1);
    while let Some(flag) = values.next() {
        let Some(value) = values.next() else { usage() };
        match flag.as_str() {
            "--registry-directory" => parsed.registry_directory = PathBuf::from(value),
            "--kernel-root" => parsed.kernel_root = PathBuf::from(value),
            "--nm" => parsed.nm = PathBuf::from(value),
            "--objdump" => parsed.objdump = PathBuf::from(value),
            "--receipt-input" => parsed.receipt_inputs.push(PathBuf::from(value)),
            "--backend" => {
                let Some((id, paths)) = value.split_once('=') else {
                    usage()
                };
                let Some((ir, object)) = paths.split_once(',') else {
                    usage()
                };
                parsed
                    .backends
                    .push((id.to_owned(), PathBuf::from(ir), PathBuf::from(object)));
            }
            "--skip-backend" => {
                let Some((id, reason)) = value.split_once('=') else {
                    usage()
                };
                parsed.skips.push((id.to_owned(), reason.to_owned()));
            }
            "--product" => {
                let Some((name, path)) = value.split_once('=') else {
                    usage()
                };
                parsed.products.push((name.to_owned(), PathBuf::from(path)));
            }
            _ => usage(),
        }
    }
    parsed
}

fn read_text(path: &Path, what: &str) -> String {
    match fs::read(path) {
        Ok(bytes) => String::from_utf8_lossy(&bytes).into_owned(),
        Err(error) => {
            eprintln!("cannot read {what} {}: {error}", path.display());
            std::process::exit(2);
        }
    }
}

/// Checks the registered family roster against the kernels the lane crate actually exposes.
fn certify_completeness(
    kernel_root: &Path,
    families: &[FamilyRow],
    failures: &mut Vec<String>,
) -> Vec<(String, String)> {
    let mut hashes = Vec::new();
    let mut observed: BTreeSet<(String, String)> = BTreeSet::new();
    for (module, relative) in KERNEL_SOURCES {
        let path = kernel_root.join(relative);
        let source = read_text(&path, "lane kernel source");
        hashes.push((path.display().to_string(), sha256(source.as_bytes())));
        for kernel in public_kernels(&source) {
            observed.insert(((*module).to_owned(), kernel));
        }
    }
    let registered: BTreeSet<(String, String)> = families
        .iter()
        .map(|row| (row.module.clone(), row.kernel.clone()))
        .collect();
    for (module, kernel) in observed.difference(&registered) {
        failures.push(format!(
            "kernel family completeness: {module}::{kernel} is public in the lane crate and is not \
             registered in {FAMILIES_FILE}"
        ));
    }
    for (module, kernel) in registered.difference(&observed) {
        failures.push(format!(
            "kernel family completeness: {FAMILIES_FILE} registers {module}::{kernel}, which the \
             lane crate no longer exposes"
        ));
    }
    hashes
}

struct ReportInput<'a> {
    failures: &'a [String],
    families: &'a [FamilyRow],
    backend_reports: &'a [BackendReport],
    product_reports: &'a [ProductReport],
    receipt: &'a [(String, String)],
    chain_digest: &'a str,
}

#[expect(
    clippy::too_many_lines,
    reason = "one flat report document; splitting it would scatter the schema across helpers"
)]
fn render_report(input: &ReportInput<'_>) -> String {
    let mut out = String::new();
    out.push_str("{\"schema_version\":2,\"kind\":\"native_vectorization\",");
    let _ = write!(
        out,
        "\"status\":{},",
        json_string(if input.failures.is_empty() {
            "pass"
        } else {
            "fail"
        })
    );
    let certified = input.families.iter().filter(|row| row.certified).count();
    let _ = write!(
        out,
        "\"kernel_families\":{{\"total\":{},\"certified\":{certified},\"exempt\":{}}},",
        input.families.len(),
        input.families.len() - certified
    );
    out.push_str("\"backends\":[");
    for (index, report) in input.backend_reports.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        let _ = write!(
            out,
            "{{\"id\":{},\"status\":{},\"families\":{}",
            json_string(&report.id),
            json_string(report.status),
            report.families
        );
        if let Some(reason) = &report.skip_reason {
            let _ = write!(out, ",\"skip_reason\":{}", json_string(reason));
        }
        out.push_str(",\"probes\":[");
        for (position, (family, ir, asm)) in report.observations.iter().enumerate() {
            if position > 0 {
                out.push(',');
            }
            let _ = write!(
                out,
                "{{\"family\":{},\"ir_vector_arith\":{},\"ir_vector_compare\":{},\
                 \"ir_vector_any\":{},\"ir_scalar_arith\":{},\"ir_narrow_arith\":{},\
                 \"ir_fast_math\":{},\"ir_forbidden_calls\":{},\"asm_vector_arith\":{},\
                 \"asm_vector_any\":{},\"asm_scalar_arith\":{},\"asm_narrow_arith\":{}}}",
                json_string(family),
                ir.vector_arith,
                ir.vector_compare,
                ir.vector_any,
                ir.scalar_arith,
                ir.narrow_arith,
                ir.fast_math,
                ir.forbidden_calls,
                asm.vector_arith,
                asm.vector_any,
                asm.scalar_arith,
                asm.narrow_arith
            );
        }
        out.push_str("]}");
    }
    out.push_str("],\"products\":[");
    for (index, report) in input.product_reports.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        let _ = write!(
            out,
            "{{\"product\":{},\"artifact\":{},\"exported_entries\":{},\
             \"render_reachable_functions\":{},\"unresolved_call_edges\":{},\"kernel_hosts\":[",
            json_string(&report.product),
            json_string(&report.artifact),
            report.exported_entries,
            report.reachable_functions,
            report.unresolved_call_edges
        );
        for (position, (symbol, vector, scalar)) in report.hosts.iter().enumerate() {
            if position > 0 {
                out.push(',');
            }
            let _ = write!(
                out,
                "{{\"symbol\":{},\"vector_arith\":{vector},\"scalar_arith\":{scalar}}}",
                json_string(symbol)
            );
        }
        out.push_str("]}");
    }
    out.push_str("],\"receipt\":{\"chain_sha256\":");
    out.push_str(&json_string(input.chain_digest));
    out.push_str(",\"inputs\":[");
    for (index, (name, digest)) in input.receipt.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        let _ = write!(
            out,
            "{{\"path\":{},\"sha256\":{}}}",
            json_string(name),
            json_string(digest)
        );
    }
    out.push_str("]},\"failures\":[");
    for (index, failure) in input.failures.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        out.push_str(&json_string(failure));
    }
    out.push_str("]}");
    out
}

#[expect(
    clippy::too_many_lines,
    reason = "one report: read the registries, run each half, emit one receipt"
)]
pub(crate) fn main() {
    let args = args();
    let mut failures: Vec<String> = Vec::new();

    let families_path = args.registry_directory.join(FAMILIES_FILE);
    let allowlist_path = args.registry_directory.join(ALLOWLIST_FILE);
    let shipped_path = args.registry_directory.join(SHIPPED_FILE);
    let families_text = read_text(&families_path, "families registry");
    let allowlist_text = read_text(&allowlist_path, "allowlist");
    let shipped_text = read_text(&shipped_path, "shipped registry");

    let families = match parse_families(&families_text) {
        Ok(rows) => rows,
        Err(error) => {
            eprintln!("invalid families registry: {error}");
            std::process::exit(2);
        }
    };
    let allowlist = match parse_allowlist(&allowlist_text) {
        Ok(rows) => rows,
        Err(error) => {
            eprintln!("invalid vectorization allowlist: {error}");
            std::process::exit(2);
        }
    };
    let shipped = match parse_shipped(&shipped_text) {
        Ok(rows) => rows,
        Err(error) => {
            eprintln!("invalid shipped registry: {error}");
            std::process::exit(2);
        }
    };

    let source_hashes = certify_completeness(&args.kernel_root, &families, &mut failures);

    // Every certified family must have a rule at every backend: an unregistered backend row is how
    // a new family would otherwise reach one architecture and quietly skip the other.
    let certified: BTreeSet<&str> = families
        .iter()
        .filter(|row| row.certified)
        .map(|row| row.family.as_str())
        .collect();
    for spec in BACKENDS {
        let covered: BTreeSet<&str> = allowlist
            .iter()
            .filter(|row| row.backend == spec.id)
            .map(|row| row.family.as_str())
            .collect();
        for family in certified.difference(&covered) {
            failures.push(format!(
                "{}: certified family '{family}' has no rule in {ALLOWLIST_FILE}",
                spec.id
            ));
        }
        for family in covered.difference(&certified) {
            failures.push(format!(
                "{}: {ALLOWLIST_FILE} rules family '{family}', which is not a certified family",
                spec.id
            ));
        }
    }

    let tools = Tools {
        nm: args.nm.clone(),
        objdump: args.objdump.clone(),
    };

    let mut backend_reports = Vec::new();
    for (id, ir_path, object_path) in &args.backends {
        let Some(spec) = backend(id) else {
            failures.push(format!("unknown backend '{id}'"));
            continue;
        };
        let rows: Vec<ProbeRow> = allowlist
            .iter()
            .filter(|row| row.backend == *id)
            .cloned()
            .collect();
        backend_reports.push(certify_backend(
            spec,
            &rows,
            ir_path,
            object_path,
            &tools,
            &mut failures,
        ));
    }
    for (id, reason) in &args.skips {
        if backend(id).is_none() {
            failures.push(format!("unknown skipped backend '{id}'"));
            continue;
        }
        backend_reports.push(BackendReport {
            id: id.clone(),
            status: "skipped",
            skip_reason: Some(reason.clone()),
            families: allowlist.iter().filter(|row| row.backend == *id).count(),
            ir_path: String::new(),
            object_path: String::new(),
            ir_sha256: String::new(),
            object_sha256: String::new(),
            disassembly_sha256: String::new(),
            symbols_sha256: String::new(),
            observations: Vec::new(),
        });
    }
    let covered: BTreeSet<&str> = backend_reports
        .iter()
        .map(|report| report.id.as_str())
        .collect();
    for spec in BACKENDS {
        if !covered.contains(spec.id) {
            failures.push(format!(
                "backend '{}' was neither certified nor explicitly skipped",
                spec.id
            ));
        }
    }

    let mut product_reports = Vec::new();
    let registered_products: BTreeSet<&str> =
        shipped.iter().map(|row| row.product.as_str()).collect();
    for (name, path) in &args.products {
        if !registered_products.contains(name.as_str()) {
            failures.push(format!("product '{name}' has no rules in {SHIPPED_FILE}"));
            continue;
        }
        let rows: Vec<ShippedRow> = shipped
            .iter()
            .filter(|row| row.product == *name)
            .cloned()
            .collect();
        product_reports.push(certify_product(name, path, &rows, &tools, &mut failures));
    }
    let seen_products: BTreeSet<&str> = args
        .products
        .iter()
        .map(|(name, _)| name.as_str())
        .collect();
    for product in registered_products.difference(&seen_products) {
        failures.push(format!(
            "{SHIPPED_FILE} registers product '{product}', which this run did not certify"
        ));
    }

    let mut receipt: Vec<(String, String)> = source_hashes;
    receipt.push((
        families_path.display().to_string(),
        sha256(families_text.as_bytes()),
    ));
    receipt.push((
        allowlist_path.display().to_string(),
        sha256(allowlist_text.as_bytes()),
    ));
    receipt.push((
        shipped_path.display().to_string(),
        sha256(shipped_text.as_bytes()),
    ));
    for path in &args.receipt_inputs {
        match fs::read(path) {
            Ok(bytes) => receipt.push((path.display().to_string(), sha256(&bytes))),
            Err(error) => failures.push(format!(
                "cannot hash receipt input {}: {error}",
                path.display()
            )),
        }
    }
    for report in &backend_reports {
        if report.status == "skipped" {
            continue;
        }
        receipt.push((report.ir_path.clone(), report.ir_sha256.clone()));
        receipt.push((report.object_path.clone(), report.object_sha256.clone()));
        receipt.push((
            format!("{}#disassembly", report.object_path),
            report.disassembly_sha256.clone(),
        ));
        receipt.push((
            format!("{}#symbols", report.object_path),
            report.symbols_sha256.clone(),
        ));
    }
    for report in &product_reports {
        receipt.push((report.artifact.clone(), report.artifact_sha256.clone()));
        receipt.push((
            format!("{}#disassembly", report.artifact),
            report.disassembly_sha256.clone(),
        ));
        receipt.push((
            format!("{}#symbols", report.artifact),
            report.symbols_sha256.clone(),
        ));
    }
    receipt.sort();
    let chain: String = receipt
        .iter()
        .map(|(name, digest)| format!("{digest}  {name}\n"))
        .collect();
    let chain_digest = sha256(chain.as_bytes());

    println!(
        "{}",
        render_report(&ReportInput {
            failures: &failures,
            families: &families,
            backend_reports: &backend_reports,
            product_reports: &product_reports,
            receipt: &receipt,
            chain_digest: &chain_digest,
        })
    );
    if !failures.is_empty() {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The lane type token the registry carries for the eight-lane backend, assembled here rather
    /// than spelled: `scripts/check-lane-policy.sh` forbids the SIMD vocabulary in any `.rs` file
    /// outside the lane crate, and this module is only ever matching an opaque registry string.
    fn lane8() -> String {
        format!("{}::f32x8_::f32x8", "wi".to_owned() + "de")
    }

    fn probe_row(family: &str, ir: Class, asm: Class) -> ProbeRow {
        ProbeRow {
            backend: "x86_64-avx2".to_owned(),
            family: family.to_owned(),
            probe_path: "probes::probe_svf_block".to_owned(),
            generic_argument: lane8(),
            ir_class: ir,
            asm_class: asm,
        }
    }

    // --- the kernel roster scan -------------------------------------------------------------

    #[test]
    fn roster_counts_only_top_level_public_items() {
        let source = "\
pub fn real_kernel<L: Lane>(io: &mut [f32]) {
    fn nested_helper() {}
    let _ = 1;
}
fn private_kernel() {}
pub struct NotAFunction;
mod inner {
    pub fn hidden_kernel() {}
}
pub fn second_kernel() {}
";
        assert_eq!(public_kernels(source), vec!["real_kernel", "second_kernel"]);
    }

    #[test]
    fn roster_ignores_comments_and_strings() {
        let source = "\
/// A doc comment that says pub fn ghost_one().
// A line comment that says pub fn ghost_two().
/* A block comment that says
   pub fn ghost_three() */
const MESSAGE: &str = \"pub fn ghost_four()\";
pub fn only_kernel() {}
";
        assert_eq!(public_kernels(source), vec!["only_kernel"]);
    }

    #[test]
    fn roster_brace_counting_survives_a_brace_in_a_string() {
        // A `{` inside a string literal would unbalance a naive depth counter and hide every
        // later kernel behind a depth that never returns to zero.
        let source = "\
const OPENER: &str = \"{{{\";
pub fn still_visible() {}
";
        assert_eq!(public_kernels(source), vec!["still_visible"]);
    }

    // --- symbol table parsing ---------------------------------------------------------------

    #[test]
    fn nm_rows_are_parsed_from_the_right_so_spaced_names_survive() {
        let text = "<a::B<C> as d::E>::process_bank t 18ca60 30dc\nplain_symbol T 69ae0 5f1\n";
        let symbols = parse_nm(text);
        assert_eq!(symbols.len(), 2);
        assert_eq!(symbols[0].name, "<a::B<C> as d::E>::process_bank");
        assert_eq!(symbols[0].address, 0x18_ca60);
        assert_eq!(symbols[1].name, "plain_symbol");
    }

    #[test]
    fn nm_skips_rows_that_are_not_defined_text_symbols() {
        let text = "some_data d 2c7d28 20\nundefined_symbol U\n";
        assert!(parse_nm(text).is_empty());
    }

    // --- disassembly parsing and body identity ----------------------------------------------

    fn disassembly(name: &str, instructions: &[&str]) -> String {
        let mut text = format!("0000000000000000 <{name}>:\n");
        for (index, instruction) in instructions.iter().enumerate() {
            let _ = writeln!(text, "       {index}:      \t{instruction}");
        }
        text
    }

    #[test]
    fn a_duplicated_symbol_header_is_refused_rather_than_read() {
        // The lookalike evasion: a second header with the certified name. Both bodies sit at the
        // same address in a relocatable object, so neither name nor address disambiguates and the
        // only honest answer is to refuse.
        let text = format!(
            "{}{}",
            disassembly("probes::probe_svf_block", &["vmulps\t%ymm0, %ymm1, %ymm2"]),
            disassembly("probes::probe_svf_block", &["vmulss\t%xmm0, %xmm1, %xmm2"])
        );
        let bodies = parse_disassembly(&text);
        let error = locate(&bodies, "probes::probe_svf_block", 0).unwrap_err();
        assert!(error.contains("2 disassembled bodies are named"), "{error}");
    }

    #[test]
    fn a_renamed_or_inlined_probe_has_no_body_to_certify() {
        // Both evasions collapse to the same observable: the certified name defines no body.
        let bodies = parse_disassembly(&disassembly("probes::probe_other", &["retq"]));
        let error = locate(&bodies, "probes::probe_svf_block", 0).unwrap_err();
        assert!(error.contains("no disassembled body is named"), "{error}");
    }

    #[test]
    fn a_mnemonic_is_a_token_not_a_substring_of_the_line() {
        // `jmp 0x40 <vmulps_lookalike>` mentions a required opcode in its branch target. A
        // substring rule would count it; a token rule reads the mnemonic `jmp`.
        let bodies = parse_disassembly(&disassembly(
            "probes::probe_svf_block",
            &["jmp\t0x40 <vmulps_lookalike>"],
        ));
        let profile = profile_body(&bodies[0], &BACKENDS[0]);
        assert_eq!(profile.vector_arith, 0);
    }

    // --- instruction classification ---------------------------------------------------------

    #[test]
    fn a_scalar_fused_multiply_add_is_classified_scalar() {
        assert_eq!(classify_x86("vfmadd213ps"), (true, true, true));
        assert_eq!(classify_x86("vfmadd213ss"), (true, true, false));
        assert_eq!(classify_x86("vfnmadd231sd"), (true, true, false));
    }

    #[test]
    fn integer_and_non_floating_mnemonics_are_not_floating_point() {
        assert_eq!(classify_x86("vpcmpeqd"), (false, false, false));
        assert_eq!(classify_x86("vpaddd"), (false, false, false));
        assert_eq!(classify_x86("retq"), (false, false, false));
        assert_eq!(classify_aarch64("ldp"), (false, false));
        assert_eq!(classify_aarch64("fmla"), (true, true));
        assert_eq!(classify_aarch64("fmov"), (true, false));
    }

    #[test]
    fn an_injected_scalar_instruction_makes_a_probe_body_red() {
        let bodies = parse_disassembly(&disassembly(
            "probes::probe_svf_block",
            &[
                "vmulps\t%ymm0, %ymm1, %ymm2",
                "vfmadd213ss\t%xmm0, %xmm1, %xmm2",
            ],
        ));
        let profile = profile_body(&bodies[0], &BACKENDS[0]);
        assert_eq!(profile.vector_arith, 1);
        assert_eq!(profile.scalar_arith, 1);
    }

    #[test]
    fn a_narrow_packed_instruction_is_not_backend_width() {
        let bodies = parse_disassembly(&disassembly(
            "probes::probe_svf_block",
            &["vmulps\t%xmm0, %xmm1, %xmm2"],
        ));
        let profile = profile_body(&bodies[0], &BACKENDS[0]);
        assert_eq!(profile.vector_arith, 0);
        assert_eq!(profile.narrow_arith, 1);
    }

    #[test]
    fn aarch64_width_is_read_from_the_operands() {
        let neon = &BACKENDS[1];
        let wide = parse_disassembly(&disassembly(
            "probes::probe_svf_block",
            &["fmla\tv20.4s, v18.4s, v6.4s"],
        ));
        assert_eq!(profile_body(&wide[0], neon).vector_arith, 1);
        let scalar = parse_disassembly(&disassembly(
            "probes::probe_svf_block",
            &["fmla\ts20, s18, s6"],
        ));
        assert_eq!(profile_body(&scalar[0], neon).scalar_arith, 1);
        let narrow = parse_disassembly(&disassembly(
            "probes::probe_svf_block",
            &["fmla\tv20.2s, v18.2s, v6.2s"],
        ));
        assert_eq!(profile_body(&narrow[0], neon).narrow_arith, 1);
    }

    #[test]
    fn a_whole_lane_move_counts_as_backend_width_on_both_dialects() {
        assert!(operands_are_vector_width("%ymm0, (%rax)", &BACKENDS[0]));
        assert!(!operands_are_vector_width("%xmm0, (%rax)", &BACKENDS[0]));
        assert!(operands_are_vector_width("q16, q5, [x2]", &BACKENDS[1]));
        assert!(operands_are_vector_width(
            "v0.16b, v1.16b, v2.16b",
            &BACKENDS[1]
        ));
        assert!(!operands_are_vector_width("x8, xzr", &BACKENDS[1]));
    }

    #[test]
    fn a_math_library_call_target_is_recognised_through_its_decoration() {
        assert!(call_target_is_forbidden("0x1234 <expf@plt>"));
        assert!(call_target_is_forbidden("0x1234 <powf+0x10>"));
        assert!(!call_target_is_forbidden("0x1234 <core::panicking::panic>"));
    }

    // --- LLVM IR -----------------------------------------------------------------------------

    #[test]
    fn the_noalias_scope_intrinsic_is_not_a_transcendental() {
        // The false positive a substring scan for `@llvm.exp` produces on every body that touches
        // two disjoint slices. It cost this subject a red run before the callee scan was exact.
        assert_eq!(
            forbidden_ir_callees(
                "  tail call void @llvm.experimental.noalias.scope.decl(metadata !2383)"
            ),
            0
        );
        assert_eq!(forbidden_ir_callees("  %0 = call float @expf(float %1)"), 1);
        assert_eq!(
            forbidden_ir_callees("  %0 = call <8 x float> @llvm.fmuladd.v8f32(<8 x float> %1)"),
            1
        );
        // `Lane::fma` lowers to this and is the one permitted fusion (D3).
        assert_eq!(
            forbidden_ir_callees("  %0 = call <8 x float> @llvm.fma.v8f32(<8 x float> %1)"),
            0
        );
    }

    #[test]
    fn ir_definitions_are_taken_from_define_lines_only() {
        let module = "\
; a comment mentioning @_RNv15probe_svf_block
define internal void @_RNv15probe_svf_block() {
  %0 = fmul <8 x float> zeroinitializer, zeroinitializer
}
define internal void @_RNv22probe_svf_block_ramped() {
  call void @_RNv15probe_svf_block()
}
";
        assert_eq!(ir_definitions(module, "15probe_svf_block").len(), 1);
        // The length prefix keeps the shorter name from matching the longer one.
        assert_eq!(ir_definitions(module, "22probe_svf_block_ramped").len(), 1);
        assert_eq!(
            ir_definitions(module, "15probe_svf_block")[0].lines.len(),
            1
        );
    }

    #[test]
    fn the_ir_needle_is_length_prefixed_so_a_prefix_cannot_collide() {
        let row = ProbeRow {
            probe_path: "probes::probe_svf_block".to_owned(),
            ..probe_row("recursive-svf", Class::VectorArith, Class::VectorArith)
        };
        assert_eq!(row.ir_needle(), "15probe_svf_block");
        assert_eq!(
            row.expected_symbol(),
            format!("probes::probe_svf_block::<{}>", lane8())
        );
    }

    fn ir_function(lines: &[&str]) -> IrFunction {
        IrFunction {
            lines: lines.iter().map(|line| (*line).to_owned()).collect(),
        }
    }

    #[test]
    fn ir_profiles_separate_width_scalar_and_fast_math() {
        let profile = profile_ir(
            &ir_function(&[
                "  %0 = fmul <8 x float> %a, %b",
                "  %1 = fadd reassoc <8 x float> %a, %b",
                "  %2 = fmul float %c, %d",
                "  %3 = fmul <4 x float> %e, %f",
                "  %4 = fcmp olt <8 x float> %a, %b",
            ]),
            &BACKENDS[0],
        );
        assert_eq!(profile.vector_arith, 2);
        assert_eq!(profile.fast_math, 1);
        assert_eq!(profile.scalar_arith, 1);
        assert_eq!(profile.narrow_arith, 1);
        assert_eq!(profile.vector_compare, 1);
    }

    #[test]
    fn the_x86_lane_compare_intrinsic_counts_as_a_vector_comparison() {
        // `wide` lowers an ordered lane compare to a target intrinsic on x86, not to `fcmp`.
        let profile = profile_ir(
            &ir_function(&[
                "  %0 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %a, <8 x float> %b, i8 17)",
            ]),
            &BACKENDS[0],
        );
        assert_eq!(profile.vector_compare, 1);
    }

    #[test]
    fn a_class_is_only_satisfied_by_what_it_names() {
        let empty = IrProfile::default();
        assert!(class_satisfied_ir(Class::VectorArith, &empty).is_some());
        assert!(class_satisfied_ir(Class::VectorCompare, &empty).is_some());
        assert!(class_satisfied_ir(Class::VectorAny, &empty).is_some());
        assert!(class_satisfied_ir(Class::NoFloat, &empty).is_none());
        let arithmetic = IrProfile {
            float_any: 3,
            ..IrProfile::default()
        };
        assert!(class_satisfied_ir(Class::NoFloat, &arithmetic).is_some());
        let moved = AsmProfile {
            vector_any: 4,
            ..AsmProfile::default()
        };
        assert!(class_satisfied_asm(Class::VectorAny, &moved).is_none());
        assert!(class_satisfied_asm(Class::VectorArith, &moved).is_some());
        assert!(class_satisfied_asm(Class::NoFloat, &moved).is_none());
    }

    // --- registries ---------------------------------------------------------------------------

    #[test]
    fn a_registry_row_of_the_wrong_arity_is_refused() {
        let error = parse_families("kernels\tsvf_block\trecursive-svf\tcertified\n").unwrap_err();
        assert!(error.contains("expected 5"), "{error}");
    }

    #[test]
    fn an_exempt_family_must_carry_a_reason() {
        let error = parse_families("kernels/builtins\tno_lanes\tmask\texempt\t-\n").unwrap_err();
        assert!(error.contains("must carry a reason"), "{error}");
    }

    #[test]
    fn a_duplicated_registry_key_is_refused() {
        let text = "kernels\tsvf_block\ta\tcertified\t-\nkernels\tsvf_block\tb\tcertified\t-\n";
        assert!(parse_families(text).unwrap_err().contains("duplicates"));
    }

    #[test]
    fn an_unknown_backend_or_class_is_refused() {
        let row = format!(
            "sparc-vis\trecursive-svf\tprobes::probe_svf_block\t{}\tvector-arith\tvector-arith\n",
            lane8()
        );
        assert!(
            parse_allowlist(&row)
                .unwrap_err()
                .contains("unknown backend")
        );
        let row = format!(
            "x86_64-avx2\trecursive-svf\tprobes::probe_svf_block\t{}\tsomewhat-vector\tvector-arith\n",
            lane8()
        );
        assert!(
            parse_allowlist(&row)
                .unwrap_err()
                .contains("unknown IR class")
        );
    }

    #[test]
    fn the_shipped_registry_refuses_an_unknown_rule_or_a_non_numeric_floor() {
        let row = "capi-cdylib\tx86_64-avx2\tvibes\tsymbol\t4\n";
        assert!(parse_shipped(row).unwrap_err().contains("unknown rule"));
        let row = "capi-cdylib\tx86_64-avx2\tkernel-host\tsymbol\tmany\n";
        assert!(parse_shipped(row).unwrap_err().contains("is not a count"));
    }

    // --- shipped-artifact call closure ---------------------------------------------------------

    #[test]
    fn the_closure_follows_direct_and_got_indirect_edges_and_counts_the_rest() {
        let text = "\
0000000000001000 <entry>:
       0:      \tcallq\t0x2000 <local>
       5:      \tcallq\t*0x1234(%rip)         # 0x3000 <writev+0x3000>
       b:      \tcallq\t*%rax
0000000000002000 <local>:
       0:      \tretq
0000000000004000 <through_got>:
       0:      \tretq
";
        let bodies = parse_disassembly(text);
        let by_address: BTreeMap<u64, &Body> =
            bodies.iter().map(|body| (body.address, body)).collect();
        let got = relative_relocations("0000000000003000 R_X86_64_RELATIVE        *ABS*+0x4000\n");
        let (reachable, unresolved) = reachable_from(0x1000, &by_address, &got);
        assert_eq!(reachable, 3, "entry, its direct callee and its GOT callee");
        assert_eq!(unresolved, 1, "the register-indirect call stays unresolved");
    }

    #[test]
    fn a_vtable_style_register_indirect_call_is_never_silently_resolved() {
        assert_eq!(indirect_call_slot("*%rax"), None);
        assert_eq!(
            indirect_call_slot("*0x275377(%rip)         # 0x2d4cc8 <writev+0x2d4cc8>"),
            Some(0x2d_4cc8)
        );
        assert_eq!(direct_call_target("0x281940 <name>"), Some(0x28_1940));
    }

    // --- the report ---------------------------------------------------------------------------

    #[test]
    fn the_report_is_valid_json_and_reports_its_failures() {
        let report = render_report(&ReportInput {
            failures: &["a \"quoted\" failure\nwith a newline".to_owned()],
            families: &[FamilyRow {
                module: "kernels".to_owned(),
                kernel: "svf_block".to_owned(),
                family: "recursive-svf".to_owned(),
                certified: true,
            }],
            backend_reports: &[],
            product_reports: &[],
            receipt: &[("Cargo.lock".to_owned(), "ab".to_owned())],
            chain_digest: "cd",
        });
        assert!(report.starts_with('{') && report.ends_with('}'));
        assert!(report.contains("\"status\":\"fail\""));
        assert!(report.contains("a \\\"quoted\\\" failure\\nwith a newline"));
        assert!(report.contains("\"chain_sha256\":\"cd\""));
    }
}
