//! Thin command-line boundary for the canonical-PCM stem identity oracle.

fn main() {
    if let Err(error) = miso_engine_stem_hasher::run_cli(std::env::args_os().skip(1)) {
        eprintln!("{error}");
        std::process::exit(2);
    }
}
