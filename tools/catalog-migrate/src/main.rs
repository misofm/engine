//! CLI boundary for the one-way catalog re-hash oracle.

fn main() {
    if let Err(error) = catalog_migrate::run_cli(std::env::args_os().skip(1)) {
        eprintln!("{error}");
        std::process::exit(2);
    }
}
