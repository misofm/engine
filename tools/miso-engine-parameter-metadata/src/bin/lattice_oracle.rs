//! Emit the engine's own parameter lattices, for the SDK's agent-ops cross-check (issue #243).
//!
//! # Why this exists rather than a bigger metadata document
//!
//! `miso-engine-v2-parameter-metadata.json` carries each parameter's lattice *declaration* -- the
//! step, its unit, the rendering precision, the five ladder multiples -- which is a handful of
//! bytes. It deliberately does not carry the lattice's *points*, because a one-cent lattice from
//! 20 Hz to 20 kHz has about twelve thousand of them and the shipped catalog would grow by
//! megabytes to say something a consumer can derive.
//!
//! So an agent surface has to generate the points itself, and the question that matters is whether
//! it generates the *same* points the engine does. This binary is how that is answered: it walks
//! the launch registry, builds every lattice through `parameter_lattice_points` -- the engine's own
//! resolver, the one the descriptor-wire verifier and the compiler both use -- and prints a
//! digest of the result. The SDK builds its lattices independently and asserts the digests match,
//! point for point, for the entire shipped catalog.
//!
//! A digest rather than a dump because the answer is an equality, not a document: printing twelve
//! thousand decimals per row would make the comparison a diff of megabytes and the failure message
//! useless. `first`/`last`/`count` accompany the digest so a mismatch says *where* to look.
//!
//! # The step-resolution rows
//!
//! Equality of point sets is necessary but not sufficient for an agent surface: `step(param, ±k)`
//! must land on the same rank the engine's `resolve_parameter_step` lands on, including its
//! clamping at the endpoints. Each row therefore also carries a handful of resolutions, chosen to
//! straddle both endpoints so the clamp is exercised rather than assumed.
//!
//! Output is one tab-separated row per parameter, sorted by `(effect_id, parameter_id)`.

use miso_engine_effect_compiler::launch_native_effect_registry;
use miso_engine_effect_contract::{
    LatticePoint, StepSize, lattice_index_for_decimal, parameter_lattice_points,
    resolve_parameter_step,
};
use sha2::{Digest, Sha256};

/// The digest the SDK reproduces: SHA-256 over `index\tcanonical\tintrinsic\n` per point.
///
/// The rank and the intrinsic flag are inside the digest, not merely the text, because both are
/// load-bearing on the wire: the rank *is* the persisted index (adopted ruling finding 7), and a
/// point that stopped being intrinsic would change how a surface renders it.
fn digest(points: &[LatticePoint]) -> String {
    let mut hasher = Sha256::new();
    for point in points {
        hasher.update(point.index.to_string().as_bytes());
        hasher.update(b"\t");
        hasher.update(point.canonical.as_bytes());
        hasher.update(b"\t");
        hasher.update(if point.intrinsic { b"1" } else { b"0" });
        hasher.update(b"\n");
    }
    hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn main() {
    let registry = launch_native_effect_registry().expect("launch effect registry");

    // `--dump EFFECT_ID PARAMETER_ID` prints one lattice's points, one per line, for the moments
    // when a digest mismatch has to be turned into a located disagreement.
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    if let [flag, effect_id, parameter_id] = arguments.as_slice()
        && flag == "--dump"
    {
        let wanted: u32 = parameter_id.parse().expect("PARAMETER_ID is a u32");
        for descriptor in registry.descriptors() {
            if descriptor.id.as_str() != effect_id {
                continue;
            }
            for parameter in descriptor.parameters {
                if parameter.id.0 != wanted {
                    continue;
                }
                for point in parameter_lattice_points(parameter).expect("lattice") {
                    println!(
                        "{}\t{}\t{}",
                        point.index,
                        point.canonical,
                        u8::from(point.intrinsic)
                    );
                }
                return;
            }
        }
        panic!("no such parameter");
    }
    let mut rows: Vec<String> = Vec::new();

    for descriptor in registry.descriptors() {
        for parameter in descriptor.parameters {
            let points = match parameter_lattice_points(parameter) {
                Ok(points) => points,
                // A parameter whose declaration cannot form a lattice is reported as such rather
                // than skipped: the SDK must refuse exactly the same rows the engine refuses, and
                // a silently absent row would let it refuse a row the engine accepts.
                Err(error) => {
                    rows.push(format!(
                        "{}\t{}\t{}\tERROR\t{error:?}\t\t\t",
                        descriptor.id.as_str(),
                        parameter.id.0,
                        parameter.display_name,
                    ));
                    continue;
                }
            };
            let first = points.first().map_or("", |point| point.canonical.as_str());
            let last = points.last().map_or("", |point| point.canonical.as_str());

            // Resolutions chosen to straddle both endpoints, so the clamp is exercised: from the
            // minimum downward, from the maximum upward, and one large jump from the middle.
            let last_index = u32::try_from(points.len().saturating_sub(1)).unwrap_or(0);
            let middle = last_index / 2;
            let resolutions: Vec<String> = [
                (0_u32, StepSize::Xs, 1_i32),
                (0, StepSize::Xl, -1),
                (last_index, StepSize::Xs, 1),
                (last_index, StepSize::Md, -1),
                (middle, StepSize::Lg, 3),
                (middle, StepSize::Xl, -3),
            ]
            .iter()
            .map(|(current, size, count)| {
                resolve_parameter_step(&points, *current, *size, *count, parameter.lattice.ladder)
                    .map_or_else(|| "none".to_owned(), |index| index.to_string())
            })
            .collect();

            // A round trip through the decimal lookup, at both endpoints and the middle: the rank
            // a canonical rendering maps back to must be the rank it came from.
            let lookups: Vec<String> = [0, middle, last_index]
                .iter()
                .map(|index| {
                    points.get(*index as usize).map_or_else(
                        || "none".to_owned(),
                        |point| {
                            lattice_index_for_decimal(&points, &point.canonical)
                                .map_or_else(|_| "miss".to_owned(), |found| found.to_string())
                        },
                    )
                })
                .collect();

            rows.push(format!(
                "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                descriptor.id.as_str(),
                parameter.id.0,
                parameter.display_name,
                points.len(),
                digest(&points),
                first,
                last,
                resolutions.join(","),
                lookups.join(","),
            ));
        }
    }

    rows.sort_unstable();
    println!("effect_id\tparameter_id\tname\tcount\tdigest\tfirst\tlast\tsteps\tlookups");
    for row in rows {
        println!("{row}");
    }
}
