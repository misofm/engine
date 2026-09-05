#!/usr/bin/env bash
set -euo pipefail
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
scratch=$(mktemp -d); trap 'rm -rf -- "$scratch"' EXIT
base="$scratch/base"; work="$scratch/work"; checker="${CHECKER:-$root/scripts/check-conformance-boundaries.sh}"
mkdir -p "$base"/{crates,hosts,tools,sidecars} "$base/crates/dsp-reference/src" "$base/crates/conformance/src" "$base/tools/bench/src"
printf '[package]\nname = "dsp-reference"\n[lib]\nname = "dsp_reference"\n' >"$base/crates/dsp-reference/Cargo.toml"
printf 'pub fn oracle() {}\n' >"$base/crates/dsp-reference/src/lib.rs"
cat >"$base/crates/conformance/Cargo.toml" <<'EOF'
[package]
name = "conformance"
[lib]
name = "conformance"
[dependencies]
dsp-reference.workspace=true
effect-contract.workspace = true
engine.workspace=true
[target.'cfg(unix)'.dependencies]
lane.workspace = true
[dev-dependencies]
ignored = "1"
EOF
cat >"$base/tools/bench/Cargo.toml" <<'EOF'
[package]
name = "bench"
[dependencies]
bench-support="1"
builtins="1"
builtins-compiler="1"
conformance="1"
console-workload="1"
effect-compiler="1"
effect-contract="1"
effect-package="1"
engine="1"
flatbuffers="1"
graph="1"
graph-compiler="1"
lane="1"
protocol="1"
rack="1"
session="1"
[target.'cfg(unix)'.dependencies]
sha2="1"
EOF
printf 'fn main() {}\n' >"$base/tools/bench/src/main.rs"
for crate in engine session protocol capi target-smoke effect-contract effect-compiler effect-package lane math; do
  mkdir -p "$base/crates/$crate/src"
  ident=${crate//-/_}
  if [[ $crate == target-smoke ]]; then printf '[package]\nname = "%s"\n[[bin]]\nname = "%s"\npath = "src/main.rs"\n' "$crate" "$crate" >"$base/crates/$crate/Cargo.toml"; else printf '[package]\nname = "%s"\n[lib]\nname = "%s"\n' "$crate" "$ident" >"$base/crates/$crate/Cargo.toml"; fi
  printf 'pub fn clean() {}\n' >"$base/crates/$crate/src/lib.rs"
done
reset() { rm -rf "$work"; cp -R "$base" "$work"; }
pass() { "$@" >/dev/null; }
fail() { local expected=$1; shift; local output; if output=$("$@" 2>&1); then printf 'conformance fixture unexpectedly passed: %s\n' "$expected" >&2; exit 1; fi; [[ "$output" == *"$expected"* ]] || { printf 'wrong conformance diagnostic for %s: %s\n' "$expected" "$output" >&2; exit 1; }; }
pass bash "$checker" "$base"
(cd "$root" && pass bash scripts/check-conformance-boundaries.sh "$base")
reset; printf '\n[dependencies]\n' >>"$work/crates/dsp-reference/Cargo.toml"; fail 'f64 reference must have zero dependencies' bash "$checker" "$work"
for crate in engine session protocol capi target-smoke effect-contract effect-compiler effect-package lane math; do reset; rm "$work/crates/$crate/Cargo.toml"; fail "missing manifest for $crate" bash "$checker" "$work"; done
for crate in engine session protocol capi target-smoke effect-contract effect-compiler effect-package lane math; do reset; rm -rf "$work/crates/$crate/src"; fail "unreadable source root for $crate" bash "$checker" "$work"; done
for crate in engine session protocol capi target-smoke effect-contract effect-compiler effect-package lane math; do reset; rm "$work/crates/$crate/src/lib.rs"; fail "unreadable source root for $crate" bash "$checker" "$work"; done
reset; mkdir -p "$work/hosts/engine"; fail 'no crate directory found for engine' bash "$checker" "$work"
reset; printf 'conformance.workspace = true\n' >>"$work/crates/engine/Cargo.toml"; fail 'must not depend on a harness crate' bash "$checker" "$work"
reset; printf 'use dsp_reference::Oracle;\n' >>"$work/crates/engine/src/lib.rs"; fail 'production code must not use a harness crate' bash "$checker" "$work"
reset; printf 'mod dsp_reference;\nuse dsp_reference::Oracle;\n' >"$work/crates/engine/src/lib.rs"; pass bash "$checker" "$work"
reset; mkdir -p "$work/crates/engine/src/nested"; printf 'mod dsp_reference;\n' >"$work/crates/engine/src/nested/x.rs"; printf 'use dsp_reference::Oracle;\n' >"$work/crates/engine/src/lib.rs"; fail 'production code must not use a harness crate' bash "$checker" "$work"
reset; printf 'conformance\n' >"$work/hosts/use.txt"; fail 'hosts/sidecars must not depend on harness crates' bash "$checker" "$work"
reset; printf 'use engine::Engine;\n' >"$work/crates/dsp-reference/src/lib.rs"; fail 'reference production use scan' bash "$checker" "$work"
reset; rm -rf "$work/sidecars"; fail 'required hosts/sidecars roots missing' bash "$checker" "$work"
reset; sed -i '/sha2=/d' "$work/tools/bench/Cargo.toml"; fail 'consolidated benchmark dependency union changed' bash "$checker" "$work"
reset; find "$work" -name Cargo.toml -exec sed -i '/^\[lib\]$/,/^name = /d' {} +; fail 'no workspace library names found' bash "$checker" "$work"
reset; for crate in engine session protocol capi effect-contract effect-compiler effect-package lane math; do sed -i '/^\[lib\]$/,/^name = /d' "$work/crates/$crate/Cargo.toml"; done; fail 'no production library names found' bash "$checker" "$work"
make_shim() { local dir=$1 name=$2 body=$3; mkdir -p "$dir"; printf '#!/usr/bin/env bash\n%s\n' "$body" >"$dir/$name"; chmod +x "$dir/$name"; }
reset; make_shim "$scratch/find-error" find 'case "$*" in "crates hosts tools sidecars -name Cargo.toml -type f") [[ -z ${SHIM_PARTIAL:-} ]] || printf "crates/engine/Cargo.toml\n"; exit 7;; esac; exec /usr/bin/find "$@"'; for partial in '' 1; do fail 'workspace library manifest discovery traversal errored (find status 7)' env SHIM_PARTIAL="$partial" PATH="$scratch/find-error:$PATH" bash "$checker" "$work"; done
reset; make_shim "$scratch/awk-error" awk 'case "$*" in *"in_lib = 1"*) [[ -z ${SHIM_PARTIAL:-} ]] || printf "engine\n"; exit 8;; esac; exec /usr/bin/awk "$@"'; for partial in '' 1; do fail 'library name extraction failed' env SHIM_PARTIAL="$partial" PATH="$scratch/awk-error:$PATH" bash "$checker" "$work"; done
reset; make_shim "$scratch/awk-unique" awk 'case "$*" in *"!seen"*) [[ -z ${SHIM_PARTIAL:-} ]] || printf "engine\nlane\n"; exit 8;; esac; exec /usr/bin/awk "$@"'; for partial in '' 1; do fail 'workspace library name aggregation uniqueness filter errored (awk status 8)' env SHIM_PARTIAL="$partial" PATH="$scratch/awk-unique:$PATH" bash "$checker" "$work"; done
reset; make_shim "$scratch/awk-target" awk 'case "$*" in *"in_lib = 1"*) exec /usr/bin/awk "$@";; *"crates/conformance/Cargo.toml"*) [[ -z ${SHIM_PARTIAL:-} ]] || printf "dsp-reference\neffect-contract\nengine\nlane\n"; exit 8;; esac; exec /usr/bin/awk "$@"'; for partial in '' 1; do fail 'dependency extraction failed for crates/conformance/Cargo.toml (awk status 8)' env SHIM_PARTIAL="$partial" PATH="$scratch/awk-target:$PATH" bash "$checker" "$work"; done
reset; make_shim "$scratch/sort-error" sort '/usr/bin/sort "$@"; printf "engine\n"; exit 8'; fail 'sort errored (sort status 8)' env PATH="$scratch/sort-error:$PATH" bash "$checker" "$work"
reset; mkdir -p "$scratch/sort-target"; make_shim "$scratch/sort-target" sort 'count=$(<"$(dirname "$0")/count"); count=$((count+1)); printf "%s\n" "$count" >"$(dirname "$0")/count"; if [[ $count == 3 ]]; then [[ -z ${SHIM_PARTIAL:-} ]] || { /usr/bin/sort "$@"; printf "lane\n"; }; exit 8; fi; exec /usr/bin/sort "$@"'; for partial in '' 1; do printf '0\n' >"$scratch/sort-target/count"; fail 'dependency extraction failed for crates/conformance/Cargo.toml (sort status 8)' env SHIM_PARTIAL="$partial" PATH="$scratch/sort-target:$PATH" bash "$checker" "$work"; done
reset; make_shim "$scratch/paste-error" paste '[[ -z ${SHIM_PARTIAL:-} ]] || printf "engine|lane\n"; exit 9'; for partial in '' 1; do fail 'join errored (paste status 9)' env SHIM_PARTIAL="$partial" PATH="$scratch/paste-error:$PATH" bash "$checker" "$work"; done
reset; make_shim "$scratch/rg-manifest" rg 'case "$*" in *"engine/Cargo.toml"*) [[ -z ${SHIM_PARTIAL:-} ]] || printf "1:conformance.workspace = true\n"; exit 7;; esac; exec /usr/bin/rg "$@"'; for partial in '' 1; do fail 'engine manifest harness predicate scan errored (rg exit 7)' env SHIM_PARTIAL="$partial" PATH="$scratch/rg-manifest:$PATH" bash "$checker" "$work"; done
reset; make_shim "$scratch/rg-module" rg 'case "$*" in *"mod[[:space:]]"*"engine/src/lib.rs"*) [[ -z ${SHIM_PARTIAL:-} ]] || printf "crates/engine/src/lib.rs:1:mod dsp_reference;\n"; exit 7;; esac; exec /usr/bin/rg "$@"'; for partial in '' 1; do fail 'engine dsp_reference module probe scan errored (rg exit 7)' env SHIM_PARTIAL="$partial" PATH="$scratch/rg-module:$PATH" bash "$checker" "$work"; done
reset; make_shim "$scratch/rg-harness" rg 'case "$*" in *"::"*"engine/src"*) [[ -z ${SHIM_PARTIAL:-} ]] || printf "crates/engine/src/lib.rs:1:use dsp_reference::x;\n"; exit 7;; esac; exec /usr/bin/rg "$@"'; for partial in '' 1; do fail 'engine harness use scan scan errored (rg exit 7)' env SHIM_PARTIAL="$partial" PATH="$scratch/rg-harness:$PATH" bash "$checker" "$work"; done
reset; make_shim "$scratch/rg-hosts" rg 'case "$*" in *"hosts sidecars"*) [[ -z ${SHIM_PARTIAL:-} ]] || printf "hosts/x:1:conformance\n"; exit 7;; esac; exec /usr/bin/rg "$@"'; for partial in '' 1; do fail 'hosts/sidecars harness scan scan errored (rg exit 7)' env SHIM_PARTIAL="$partial" PATH="$scratch/rg-hosts:$PATH" bash "$checker" "$work"; done
reset; printf 'use dsp_reference::x;\n' >>"$work/crates/engine/src/lib.rs"; make_shim "$scratch/rg-filter" rg 'if [[ ${1:-} == -v ]]; then [[ -z ${SHIM_PARTIAL:-} ]] || printf "kept:use dsp_reference::x\n"; exit 7; fi; exec /usr/bin/rg "$@"'; for partial in '' 1; do fail 'harness comment filter filter errored (rg exit 7)' env SHIM_PARTIAL="$partial" PATH="$scratch/rg-filter:$PATH" bash "$checker" "$work"; done
reset; make_shim "$scratch/rg-reference-filter" rg 'case "$*" in *"^(dsp_reference|conformance)$"*) [[ -z ${SHIM_PARTIAL:-} ]] || printf "engine\nlane\n"; exit 7;; esac; exec /usr/bin/rg "$@"'; for partial in '' 1; do fail 'reference library-name filter filter errored (rg exit 7)' env SHIM_PARTIAL="$partial" PATH="$scratch/rg-reference-filter:$PATH" bash "$checker" "$work"; done

# The directed module assertion must reject Astra's exact fail-open consumer mutant.
if [[ -z ${MUTANT_RUN:-} ]]; then
  mutant_root="$scratch/module-mutant/scripts"; mkdir -p "$mutant_root/lib"; cp "$checker" "$mutant_root/check-conformance-boundaries.sh"; cp "$root/scripts/lib/gate.sh" "$mutant_root/lib/gate.sh"
  sed -i '/module_probe=.*gate_scan_collect/,/)" || exit \$?/ s/|| exit \$?/|| true/' "$mutant_root/check-conformance-boundaries.sh"
  if MUTANT_RUN=1 CHECKER="$mutant_root/check-conformance-boundaries.sh" bash "$root/scripts/test-conformance-boundaries.sh" >"$scratch/module-mutant.log" 2>&1; then echo 'module fail-open mutant escaped focused acceptance' >&2; exit 1; else mutant_rc=$?; fi
  rg -q 'conformance fixture unexpectedly passed: engine dsp_reference module probe' "$scratch/module-mutant.log" || { echo 'module mutant failed outside intended assertion' >&2; cat "$scratch/module-mutant.log" >&2; exit 1; }
  printf 'counter-mutant rejected: module consumer (status %s)\n' "$mutant_rc"
fi
printf 'conformance boundary fixtures: ok\n'
