//! Release-artifact vectorization certification for the production lane kernels.
//!
//! The probes are deliberately tiny, named, non-inlined wrappers around the same generic kernel
//! bodies the shipped release profile instantiates. The audit disassembles the release binary,
//! isolates only those bodies, and checks the explicit backend allowlist. It reports evidence; it
//! does not select a backend or participate in rendering.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::hint::black_box;
use std::path::{Path, PathBuf};
use std::process::Command;

use lane::Lane;
use lane::kernels::{SvfCoef, SvfState, gain_block, sum2_block, svf_block};
use sha2::{Digest, Sha256};

const DEFAULT_ALLOWLIST: &str = "tools/audit/vectorization-allowlist.tsv";
const PROBE_FRAMES: usize = 32;

#[derive(Clone, Debug)]
struct Rule {
    backend: String,
    family: String,
    symbol: String,
    required: Vec<Vec<String>>,
    forbidden: Vec<String>,
    forbidden_calls: Vec<String>,
}

#[cfg(target_arch = "x86_64")]
const ACTIVE_BACKEND: &str = "x86_64-avx2";
#[cfg(target_arch = "aarch64")]
const ACTIVE_BACKEND: &str = "aarch64-neon";
#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
const ACTIVE_BACKEND: &str = "unsupported";

#[cfg(target_arch = "x86_64")]
const ACTIVE_REGISTRY: &[&str] = &["probe_gain_simd8", "probe_sum2_simd8", "probe_svf_simd8"];
#[cfg(target_arch = "aarch64")]
const ACTIVE_REGISTRY: &[&str] = &["probe_gain_simd4", "probe_sum2_simd4", "probe_svf_simd4"];
#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
const ACTIVE_REGISTRY: &[&str] = &[];

// The gain argument is loaded without `black_box`: an opaque slice makes the wrapper's own
// in-range check emit a `slice_index_fail` trampoline into the captured body, which is wrapper
// plumbing, not kernel code, and would defeat the allowlist's no-call assertion (issue #372).
#[cfg(target_arch = "x86_64")]
#[inline(never)]
fn probe_gain_simd8(io: &mut [f32; PROBE_FRAMES * 8], gain: &[f32; 8]) {
    gain_block::<lane::Simd8>(io, PROBE_FRAMES, lane::Simd8::load(gain));
}

#[cfg(target_arch = "x86_64")]
#[inline(never)]
fn probe_sum2_simd8(
    out: &mut [f32; PROBE_FRAMES * 8],
    a: &[f32; PROBE_FRAMES * 8],
    b: &[f32; PROBE_FRAMES * 8],
) {
    sum2_block::<lane::Simd8>(out, a, b);
}

#[cfg(target_arch = "x86_64")]
#[inline(never)]
fn probe_svf_simd8(
    io: &mut [f32; PROBE_FRAMES * 8],
    coefficients: &SvfCoef<lane::Simd8>,
    state: &mut SvfState<lane::Simd8>,
) {
    svf_block::<lane::Simd8>(io, PROBE_FRAMES, black_box(coefficients), black_box(state));
}

// Same rationale as the x86 gain probe above: no `black_box` on the gain argument (issue #372).
#[cfg(target_arch = "aarch64")]
#[inline(never)]
fn probe_gain_simd4(io: &mut [f32; PROBE_FRAMES * 4], gain: &[f32; 4]) {
    gain_block::<lane::Simd4>(io, PROBE_FRAMES, lane::Simd4::load(gain));
}

#[cfg(target_arch = "aarch64")]
#[inline(never)]
fn probe_sum2_simd4(
    out: &mut [f32; PROBE_FRAMES * 4],
    a: &[f32; PROBE_FRAMES * 4],
    b: &[f32; PROBE_FRAMES * 4],
) {
    sum2_block::<lane::Simd4>(out, a, b);
}

#[cfg(target_arch = "aarch64")]
#[inline(never)]
fn probe_svf_simd4(
    io: &mut [f32; PROBE_FRAMES * 4],
    coefficients: &SvfCoef<lane::Simd4>,
    state: &mut SvfState<lane::Simd4>,
) {
    svf_block::<lane::Simd4>(io, PROBE_FRAMES, black_box(coefficients), black_box(state));
}

fn execute_probes() {
    #[cfg(target_arch = "x86_64")]
    {
        let mut io = [0.25f32; PROBE_FRAMES * 8];
        let a = [0.5f32; PROBE_FRAMES * 8];
        let b = [0.125f32; PROBE_FRAMES * 8];
        let mut state = SvfState {
            ic1: lane::Simd8::splat(0.0),
            ic2: lane::Simd8::splat(0.0),
        };
        let coefficients = SvfCoef {
            c1: lane::Simd8::splat(0.1),
            a2: lane::Simd8::splat(0.1),
            a3: lane::Simd8::splat(0.01),
            m0: lane::Simd8::splat(0.0),
            m1: lane::Simd8::splat(0.0),
            m2: lane::Simd8::splat(1.0),
        };
        probe_gain_simd8(&mut io, &[0.75; 8]);
        probe_sum2_simd8(&mut io, &a, &b);
        probe_svf_simd8(&mut io, &coefficients, &mut state);
        black_box((io, state));
    }
    #[cfg(target_arch = "aarch64")]
    {
        let mut io = [0.25f32; PROBE_FRAMES * 4];
        let a = [0.5f32; PROBE_FRAMES * 4];
        let b = [0.125f32; PROBE_FRAMES * 4];
        let mut state = SvfState {
            ic1: lane::Simd4::splat(0.0),
            ic2: lane::Simd4::splat(0.0),
        };
        let coefficients = SvfCoef {
            c1: lane::Simd4::splat(0.1),
            a2: lane::Simd4::splat(0.1),
            a3: lane::Simd4::splat(0.01),
            m0: lane::Simd4::splat(0.0),
            m1: lane::Simd4::splat(0.0),
            m2: lane::Simd4::splat(1.0),
        };
        probe_gain_simd4(&mut io, &[0.75; 4]);
        probe_sum2_simd4(&mut io, &a, &b);
        probe_svf_simd4(&mut io, &coefficients, &mut state);
        black_box((io, state));
    }
}

fn split_required(value: &str) -> Vec<Vec<String>> {
    value
        .split(',')
        .filter(|part| !part.is_empty())
        .map(|part| part.split('|').map(str::to_owned).collect())
        .collect()
}

fn split_forbidden(value: &str) -> Vec<String> {
    value
        .split(',')
        .filter(|part| !part.is_empty())
        .map(str::to_owned)
        .collect()
}

fn parse_allowlist(text: &str) -> Result<Vec<Rule>, String> {
    let mut rules = Vec::new();
    let mut seen = BTreeSet::new();
    for (line_index, line) in text.lines().enumerate() {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() != 6 {
            return Err(format!(
                "allowlist line {} has {} fields, expected 6",
                line_index + 1,
                fields.len()
            ));
        }
        if !matches!(fields[0], "x86_64-avx2" | "aarch64-neon") {
            return Err(format!(
                "allowlist line {} names unknown backend",
                line_index + 1
            ));
        }
        if !seen.insert((fields[0], fields[2])) {
            return Err(format!(
                "allowlist line {} duplicates a backend/symbol",
                line_index + 1
            ));
        }
        let required = split_required(fields[3]);
        let forbidden = split_forbidden(fields[4]);
        let forbidden_calls = split_forbidden(fields[5]);
        if required.is_empty() || forbidden.is_empty() || forbidden_calls.is_empty() {
            return Err(format!(
                "allowlist line {} has an empty policy",
                line_index + 1
            ));
        }
        rules.push(Rule {
            backend: fields[0].to_owned(),
            family: fields[1].to_owned(),
            symbol: fields[2].to_owned(),
            required,
            forbidden,
            forbidden_calls,
        });
    }
    Ok(rules)
}

fn symbol_bodies(disassembly: &str, symbols: &[&str]) -> BTreeMap<String, String> {
    let mut bodies: BTreeMap<String, String> = BTreeMap::new();
    let mut active: Option<String> = None;
    for line in disassembly.lines() {
        let trimmed = line.trim();
        if trimmed.ends_with(">:") && trimmed.contains('<') {
            active = symbols
                .iter()
                .find(|symbol| trimmed.contains(**symbol))
                .map(|symbol| (*symbol).to_owned());
            if let Some(symbol) = &active {
                bodies.entry(symbol.clone()).or_default();
            }
            continue;
        }
        if let Some(symbol) = &active {
            let normalized = trimmed.split_whitespace().collect::<Vec<_>>().join(" ");
            bodies
                .entry(symbol.clone())
                .or_default()
                .push_str(&normalized);
            bodies.entry(symbol.clone()).or_default().push('\n');
        }
    }
    bodies
}

fn certify(disassembly: &str, rules: &[Rule]) -> Vec<String> {
    let active_rules: Vec<&Rule> = rules
        .iter()
        .filter(|rule| rule.backend == ACTIVE_BACKEND)
        .collect();
    let actual: BTreeSet<&str> = active_rules
        .iter()
        .map(|rule| rule.symbol.as_str())
        .collect();
    let expected: BTreeSet<&str> = ACTIVE_REGISTRY.iter().copied().collect();
    let mut failures = Vec::new();
    if actual != expected {
        failures.push(format!(
            "allowlist registry mismatch for {ACTIVE_BACKEND}: expected {expected:?}, got {actual:?}"
        ));
    }
    let symbols: Vec<&str> = active_rules
        .iter()
        .map(|rule| rule.symbol.as_str())
        .collect();
    let bodies = symbol_bodies(disassembly, &symbols);
    for rule in active_rules {
        let Some(body) = bodies.get(&rule.symbol) else {
            failures.push(format!(
                "{} / {}: probe symbol is absent from the artifact",
                rule.family, rule.symbol
            ));
            continue;
        };
        for alternatives in &rule.required {
            if !alternatives.iter().any(|token| body.contains(token)) {
                failures.push(format!(
                    "{} / {}: missing vector family [{}]",
                    rule.family,
                    rule.symbol,
                    alternatives.join("|")
                ));
            }
        }
        for token in &rule.forbidden {
            if token_hits(body, token) {
                failures.push(format!(
                    "{} / {}: forbidden scalar fallback '{token}' is present",
                    rule.family, rule.symbol
                ));
            }
        }
        for token in &rule.forbidden_calls {
            if token_hits(body, token) {
                failures.push(format!(
                    "{} / {}: forbidden call '{token}' is present",
                    rule.family, rule.symbol
                ));
            }
        }
    }
    failures
}

/// A forbidden token is a `|`-separated set of alternatives: the body is in violation when any
/// alternative occurs. The plain mnemonics of the original rows keep their literal meaning.
fn token_hits(body: &str, token: &str) -> bool {
    token
        .split('|')
        .any(|alternative| body.contains(alternative))
}

fn sha256(bytes: &[u8]) -> String {
    let mut digest = Sha256::new();
    digest.update(bytes);
    digest
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn json_string(value: &str) -> String {
    format!("\"{}\"", value.replace('\\', "\\\\").replace('\"', "\\\""))
}

struct Args {
    artifact: PathBuf,
    allowlist: PathBuf,
    objdump: PathBuf,
}

fn usage() -> ! {
    eprintln!("usage: audit vectorization [--artifact PATH] [--allowlist PATH] [--objdump PATH]");
    std::process::exit(2);
}

fn args() -> Args {
    let mut artifact = std::env::current_exe().expect("current executable path");
    let mut allowlist = PathBuf::from(DEFAULT_ALLOWLIST);
    let mut objdump = PathBuf::from("llvm-objdump");
    let mut values = std::env::args_os().skip(1);
    while let Some(flag) = values.next() {
        let value = values.next().unwrap_or_else(|| usage());
        match flag.to_str() {
            Some("--artifact") => artifact = value.into(),
            Some("--allowlist") => allowlist = value.into(),
            Some("--objdump") => objdump = value.into(),
            _ => usage(),
        }
    }
    Args {
        artifact,
        allowlist,
        objdump,
    }
}

fn read(path: &Path, what: &str) -> Vec<u8> {
    fs::read(path).unwrap_or_else(|error| {
        eprintln!("cannot read {what} {}: {error}", path.display());
        std::process::exit(2);
    })
}

pub(crate) fn main() {
    execute_probes();
    let args = args();
    let artifact = read(&args.artifact, "artifact");
    let allowlist = read(&args.allowlist, "allowlist");
    let rules = parse_allowlist(std::str::from_utf8(&allowlist).unwrap_or_else(|error| {
        eprintln!("allowlist is not UTF-8: {error}");
        std::process::exit(2);
    }))
    .unwrap_or_else(|error| {
        eprintln!("invalid vectorization allowlist: {error}");
        std::process::exit(2);
    });
    if ACTIVE_BACKEND == "unsupported" {
        eprintln!("native vectorization audit does not support this architecture");
        std::process::exit(2);
    }
    let output = Command::new(&args.objdump)
        .arg("--demangle")
        .arg("--disassemble")
        .arg(&args.artifact)
        .output()
        .unwrap_or_else(|error| {
            eprintln!("failed to run {}: {error}", args.objdump.display());
            std::process::exit(2);
        });
    if !output.status.success() {
        eprintln!(
            "{} failed: {}",
            args.objdump.display(),
            String::from_utf8_lossy(&output.stderr)
        );
        std::process::exit(2);
    }
    let disassembly = String::from_utf8(output.stdout).unwrap_or_else(|error| {
        eprintln!("objdump output is not UTF-8: {error}");
        std::process::exit(2);
    });
    let failures = certify(&disassembly, &rules);
    let active_rule_count = rules
        .iter()
        .filter(|rule| rule.backend == ACTIVE_BACKEND)
        .count();
    let failure_json = failures
        .iter()
        .map(|failure| json_string(failure))
        .collect::<Vec<_>>()
        .join(",");
    println!(
        "{{\"schema_version\":1,\"kind\":\"native_vectorization\",\"subject\":\"release_probe_instantiations_of_production_kernels\",\"status\":\"{}\",\"backend\":\"{}\",\"artifact_sha256\":\"{}\",\"disassembly_sha256\":\"{}\",\"allowlist_sha256\":\"{}\",\"kernel_rules\":{},\"failures\":[{}]}}",
        if failures.is_empty() { "pass" } else { "fail" },
        ACTIVE_BACKEND,
        sha256(&artifact),
        sha256(disassembly.as_bytes()),
        sha256(&allowlist),
        active_rule_count,
        failure_json
    );
    if !failures.is_empty() {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn active_rules() -> Vec<Rule> {
        ACTIVE_REGISTRY
            .iter()
            .map(|symbol| Rule {
                backend: ACTIVE_BACKEND.to_owned(),
                family: (*symbol).to_owned(),
                symbol: (*symbol).to_owned(),
                required: vec![vec!["vector-op".to_owned()]],
                forbidden: vec!["scalar-op".to_owned()],
                forbidden_calls: vec!["call-op".to_owned()],
            })
            .collect()
    }

    fn synthetic_bodies(body: &str) -> String {
        ACTIVE_REGISTRY
            .iter()
            .map(|symbol| format!("0000 <audit::vectorization::{symbol}>:\n  {body}\n"))
            .collect()
    }

    #[test]
    fn missing_vector_family_is_red() {
        let failures = certify(&synthetic_bodies("different-op"), &active_rules());
        assert!(
            failures
                .iter()
                .any(|failure| failure.contains("missing vector family"))
        );
    }

    #[test]
    fn scalar_fallback_is_red() {
        let failures = certify(&synthetic_bodies("vector-op scalar-op"), &active_rules());
        assert!(
            failures
                .iter()
                .any(|failure| failure.contains("forbidden scalar fallback"))
        );
    }

    #[test]
    fn fused_multiply_add_is_red() {
        let mut rules = active_rules();
        rules[0].forbidden = vec!["vfmadd|vfnmadd".to_owned()];
        let failures = certify(
            &synthetic_bodies("vector-op vmulps vaddps vfmadd213ps"),
            &rules,
        );
        assert!(
            failures
                .iter()
                .any(|failure| failure.contains("forbidden scalar fallback")
                    && failure.contains("vfmadd|vfnmadd"))
        );
    }

    #[test]
    fn call_inside_kernel_body_is_red() {
        let mut rules = active_rules();
        rules[0].forbidden_calls = vec!["call".to_owned()];
        let failures = certify(&synthetic_bodies("vector-op vmulps call <memset>"), &rules);
        assert!(
            failures
                .iter()
                .any(|failure| failure.contains("forbidden call") && failure.contains("'call'"))
        );
    }

    #[test]
    fn incomplete_allowlist_is_red() {
        let mut rules = active_rules();
        rules.pop();
        let failures = certify(&synthetic_bodies("vector-op"), &rules);
        assert!(
            failures
                .iter()
                .any(|failure| failure.contains("registry mismatch"))
        );
    }
}
