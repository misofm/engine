//! The corpus's own sanity: every case produces events. The pins are checked by gate G5 alone.

use automation::corpus::{CASE_COUNT, run_case};
use sha2::{Digest, Sha256};

fn digest(words: &[u32]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    for word in words {
        hasher.update(word.to_le_bytes());
    }
    hasher.finalize().into()
}

/// Prints the digests in the pin's source form. Run with `--ignored --nocapture` to regenerate.
#[test]
#[ignore = "pin generator"]
fn print_pins() {
    for case in 0..CASE_COUNT {
        let words = run_case(case);
        println!("    // {}", automation::corpus::CASE_NAMES[case]);
        println!("    [");
        for row in digest(&words).chunks(15) {
            let bytes: Vec<String> = row.iter().map(|byte| format!("0x{byte:02X}")).collect();
            println!("        {},", bytes.join(", "));
        }
        println!("    ],");
    }
}

/// Red if a case stopped producing events: a digest of nothing agrees everywhere, so gate G5
/// (`g5_native_digests_match_pins`, the single owner of the pins) would pass vacuously.
#[test]
fn cases_produce_events() {
    for case in 0..CASE_COUNT {
        let words = run_case(case);
        assert!(
            words.len() / 5 > 20,
            "case {case} has {} events",
            words.len() / 5
        );
    }
}
