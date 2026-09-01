//! CLI boundary for the FLAC publishing pipeline.

fn main() {
    if let Err(error) = stem_publisher::run_cli(std::env::args_os().skip(1)) {
        eprintln!("{error}");
        std::process::exit(2);
    }
}
