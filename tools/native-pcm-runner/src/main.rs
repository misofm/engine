//! Thin command-line boundary for the native PCM reference runner.

fn main() {
    if let Err(error) = native_pcm_runner::run_cli(std::env::args_os().skip(1)) {
        eprintln!("{error}");
        std::process::exit(2);
    }
}
