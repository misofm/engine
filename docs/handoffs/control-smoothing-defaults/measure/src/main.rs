//! Issue #1055: measure the engine's own live-control ramps and render the listening stimuli.
//!
//! ```text
//! control_smoothing_measure measure --out DIR      # sections 1-8: mute, fader, pan
//! control_smoothing_measure live --out DIR         # section 9: the other live rows (decision 15)
//! control_smoothing_measure stimuli --out DIR [--mix-wav PATH --mix-offset-s SECONDS]
//! ```
//!
//! The workspace's `clippy.toml` bans `std` transcendentals so that engine DSP uses the `math`
//! crate. Here they only synthesise test material and analyse output off the engine; every engine
//! sample comes from `builtins`. The CSVs print two decimals, far above any libm difference.
#![allow(clippy::disallowed_methods)]

mod analysis;
mod crossfade;
mod effects;
mod live_rows;
mod material;
mod measure;
mod stimuli;
mod strip;
mod wav;

use std::path::PathBuf;

fn value(args: &[String], flag: &str) -> Option<String> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1).cloned())
}

fn usage() -> ! {
    eprintln!(
        "usage: control_smoothing_measure measure --out DIR [--rates R,...]\n       \
         control_smoothing_measure live --out DIR [--rates R,...]\n       \
         control_smoothing_measure stimuli --out DIR [--mix-wav PATH --mix-offset-s SECONDS]"
    );
    std::process::exit(2);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let out = value(&args, "--out").map(PathBuf::from);
    // `--rates 48000,96000` narrows a quick look; the committed data uses all four.
    let rates: Vec<u32> = value(&args, "--rates").map_or(measure::RATES.to_vec(), |v| {
        v.split(',').map(|r| r.parse().expect("--rates")).collect()
    });
    match (args.get(1).map(String::as_str), out) {
        (Some("measure"), Some(out)) => measure::run(&out, &rates),
        (Some("live"), Some(out)) => live_rows::run(&out, &rates),
        (Some("stimuli"), Some(out)) => {
            let mix = value(&args, "--mix-wav").map(PathBuf::from);
            let offset = value(&args, "--mix-offset-s")
                .map_or(0.0, |v| v.parse::<f64>().expect("--mix-offset-s is seconds"));
            stimuli::run(&out, mix.as_deref(), offset);
        }
        _ => usage(),
    }
}
