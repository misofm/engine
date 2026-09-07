//! One in-process snapshot of the benchmark runner's environment metadata.

use std::collections::BTreeMap;
use std::env::VarError;
use std::ffi::{OsStr, OsString};
use std::sync::OnceLock;

/// The immutable environment seen by a benchmark or audit process.
///
/// Subjects deliberately keep their historical field names, required/optional rules, and record
/// projections. This type only removes repeated calls into the process environment and guarantees
/// that every field in one run comes from the same snapshot.
pub struct Metadata {
    values: BTreeMap<OsString, OsString>,
}

impl Metadata {
    /// Gather the process environment once, on the control plane.
    #[must_use]
    pub fn gather() -> &'static Self {
        static SNAPSHOT: OnceLock<Metadata> = OnceLock::new();
        SNAPSHOT.get_or_init(|| Metadata {
            values: std::env::vars_os().collect(),
        })
    }

    /// Read one value from the frozen snapshot with `std::env::var`-compatible errors.
    pub fn var(&self, name: &str) -> Result<String, VarError> {
        self.values
            .get(OsStr::new(name))
            .cloned()
            .ok_or(VarError::NotPresent)?
            .into_string()
            .map_err(VarError::NotUnicode)
    }

    /// Return a nonempty snapshot value, or "unknown" when it is absent, non-Unicode, or empty.
    #[must_use]
    pub fn nonempty_or_unknown(&self, name: &str) -> String {
        self.var(name)
            .ok()
            .filter(|value| !value.is_empty())
            .unwrap_or_else(|| "unknown".to_owned())
    }

    /// The eleven runner-supplied metadata names, in the order they appear in a record.
    ///
    /// One list, shared: the console benchmark and the #163 phase 2 wasm console arm write records
    /// whose metadata blocks are compared against each other row by row, and two lists that agreed
    /// today could disagree tomorrow without either side noticing.
    pub const RECORD_NAMES: [&'static str; 11] = [
        "MISO_ENGINE_BENCH_CPU_MODEL",
        "MISO_ENGINE_BENCH_GOVERNOR_OR_POWER_MODE",
        "MISO_ENGINE_BENCH_RUST_VERSION",
        "MISO_ENGINE_BENCH_LLVM_VERSION",
        "MISO_ENGINE_BENCH_TARGET_TRIPLE",
        "MISO_ENGINE_BENCH_TARGET_FEATURES",
        "MISO_ENGINE_BENCH_PROFILE",
        "MISO_ENGINE_BENCH_BACKGROUND_LOAD_NOTE",
        "MISO_ENGINE_BENCH_MEASUREMENT_CONTROL",
        "MISO_ENGINE_BENCH_CPU_AFFINITY",
        "MISO_ENGINE_BENCH_CANDIDATE_COMMIT",
    ];

    /// The record-side name of a metadata variable.
    #[must_use]
    pub fn record_key(name: &str) -> &str {
        name.strip_prefix("MISO_ENGINE_BENCH_")
            .expect("every metadata name carries the shared prefix")
    }

    /// The sorted list of metadata names this run could not resolve.
    ///
    /// #104 F2: a runner that forgets to export a name produced records whose every metadata field
    /// was null and which still passed validation. Naming the gaps in the record is what makes a
    /// silent export failure visible instead of invisible.
    #[must_use]
    pub fn missing(&self) -> Vec<String> {
        let mut missing: Vec<String> = Self::RECORD_NAMES
            .iter()
            .filter(|name| self.var(name).is_err())
            .map(|name| Self::record_key(name).to_ascii_lowercase())
            .collect();
        missing.sort();
        missing
    }

    /// The shared metadata block of a benchmark record, as JSON object fields.
    ///
    /// Emitted with a trailing comma, so a caller splices it into the middle of a record. Every
    /// unresolved name is `null` *and* is named in `missing_metadata`, which is what distinguishes
    /// "the runner did not export this" from "this host has no such value".
    #[must_use]
    pub fn record_fields(&self) -> String {
        let field = |name: &str| match self.var(name) {
            Ok(value) => format!("\"{}\"", crate::json::escape(&value)),
            Err(_) => "null".to_string(),
        };
        let missing = self
            .missing()
            .into_iter()
            .map(|name| format!("\"{name}\""))
            .collect::<Vec<_>>()
            .join(",");
        format!(
            concat!(
                "\"cpu_model\":{cpu},\"os\":\"{os}\",\"governor_or_power_mode\":{governor},",
                "\"rust_version\":{rust},\"llvm_version\":{llvm},\"target_triple\":{triple},",
                "\"target_features\":{features},\"profile\":{profile},",
                "\"background_load_note\":{load},\"measurement_control\":{control},",
                "\"cpu_affinity\":{affinity},\"candidate_commit\":{commit},",
                "\"missing_metadata\":[{missing}],",
            ),
            cpu = field("MISO_ENGINE_BENCH_CPU_MODEL"),
            os = std::env::consts::OS,
            governor = field("MISO_ENGINE_BENCH_GOVERNOR_OR_POWER_MODE"),
            rust = field("MISO_ENGINE_BENCH_RUST_VERSION"),
            llvm = field("MISO_ENGINE_BENCH_LLVM_VERSION"),
            triple = field("MISO_ENGINE_BENCH_TARGET_TRIPLE"),
            features = field("MISO_ENGINE_BENCH_TARGET_FEATURES"),
            profile = field("MISO_ENGINE_BENCH_PROFILE"),
            load = field("MISO_ENGINE_BENCH_BACKGROUND_LOAD_NOTE"),
            control = field("MISO_ENGINE_BENCH_MEASUREMENT_CONTROL"),
            affinity = field("MISO_ENGINE_BENCH_CPU_AFFINITY"),
            commit = field("MISO_ENGINE_BENCH_CANDIDATE_COMMIT"),
            missing = missing,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Metadata;
    use std::collections::BTreeMap;
    use std::ffi::OsString;
    #[cfg(unix)]
    use std::os::unix::ffi::OsStringExt;

    #[test]
    fn gather_is_memoized_and_matches_the_process_environment() {
        let first = Metadata::gather();
        let second = Metadata::gather();
        assert!(std::ptr::eq(first, second));
        if let Ok(path) = first.var("PATH") {
            assert!(!path.is_empty());
        }
    }

    #[test]
    fn nonempty_or_unknown_preserves_nonempty_values_and_maps_missing_or_empty() {
        let mut values = BTreeMap::new();
        values.insert(OsString::from("empty"), OsString::new());
        values.insert(OsString::from("whitespace"), OsString::from(" \t\n"));
        values.insert(OsString::from("unicode"), OsString::from("é 音"));
        values.insert(OsString::from("default"), OsString::from("default"));
        values.insert(
            OsString::from("not_measured"),
            OsString::from("not measured"),
        );
        values.insert(OsString::from("ordinary"), OsString::from("value"));
        #[cfg(unix)]
        values.insert(
            OsString::from("non_unicode"),
            OsString::from_vec(vec![0xff]),
        );
        let metadata = Metadata { values };

        assert_eq!(metadata.nonempty_or_unknown("missing"), "unknown");
        assert_eq!(metadata.nonempty_or_unknown("empty"), "unknown");
        assert_eq!(metadata.nonempty_or_unknown("whitespace"), " \t\n");
        assert_eq!(metadata.nonempty_or_unknown("unicode"), "é 音");
        assert_eq!(metadata.nonempty_or_unknown("default"), "default");
        assert_eq!(metadata.nonempty_or_unknown("not_measured"), "not measured");
        assert_eq!(metadata.nonempty_or_unknown("ordinary"), "value");
        #[cfg(unix)]
        assert_eq!(metadata.nonempty_or_unknown("non_unicode"), "unknown");
    }
}
