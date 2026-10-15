//! The corpus's own sanity: every case produces events, and the pins are for these cases.

use automation::corpus::{CASE_COUNT, G5_DIGESTS, run_case};
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

/// Red if a case stopped producing events (a digest of nothing agrees everywhere), or its digest
/// moved off its pin.
#[test]
fn cases_produce_events_and_match_their_pins() {
    assert_eq!(G5_DIGESTS.len(), CASE_COUNT);
    for (case, pin) in G5_DIGESTS.iter().enumerate() {
        let words = run_case(case);
        assert!(
            words.len() / 5 > 20,
            "case {case} has {} events",
            words.len() / 5
        );
        assert_eq!(digest(&words), *pin, "case {case}");
    }
}
