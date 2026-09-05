#!/usr/bin/env bash
set -euo pipefail
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
scratch=$(mktemp -d); trap 'rm -rf -- "$scratch"' EXIT
base="$scratch/base"; work="$scratch/work"; checker="$root/scripts/check-conformance-boundaries.sh"
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
for crate in engine session; do reset; rm "$work/crates/$crate/Cargo.toml"; fail "missing manifest for $crate" bash "$checker" "$work"; done
reset; rm -rf "$work/crates/math/src"; fail 'unreadable source root for math' bash "$checker" "$work"
reset; mkdir -p "$work/hosts/engine"; fail 'no crate directory found for engine' bash "$checker" "$work"
reset; printf 'conformance.workspace = true\n' >>"$work/crates/engine/Cargo.toml"; fail 'must not depend on a harness crate' bash "$checker" "$work"
reset; printf 'use dsp_reference::Oracle;\n' >>"$work/crates/engine/src/lib.rs"; fail 'production code must not use a harness crate' bash "$checker" "$work"
reset; printf 'mod dsp_reference;\nuse dsp_reference::Oracle;\n' >"$work/crates/engine/src/lib.rs"; pass bash "$checker" "$work"
reset; mkdir -p "$work/crates/engine/src/nested"; printf 'mod dsp_reference;\n' >"$work/crates/engine/src/nested/x.rs"; printf 'use dsp_reference::Oracle;\n' >"$work/crates/engine/src/lib.rs"; fail 'production code must not use a harness crate' bash "$checker" "$work"
reset; printf 'conformance\n' >"$work/hosts/use.txt"; fail 'hosts/sidecars must not depend on harness crates' bash "$checker" "$work"
reset; printf 'use engine::Engine;\n' >"$work/crates/dsp-reference/src/lib.rs"; fail 'reference production use scan' bash "$checker" "$work"
reset; rm -rf "$work/sidecars"; fail 'required hosts/sidecars roots missing' bash "$checker" "$work"
reset; sed -i '/sha2=/d' "$work/tools/bench/Cargo.toml"; fail 'consolidated benchmark dependency union changed' bash "$checker" "$work"
make_shim() { local dir=$1 name=$2 body=$3; mkdir -p "$dir"; printf '#!/usr/bin/env bash\n%s\n' "$body" >"$dir/$name"; chmod +x "$dir/$name"; }
reset; make_shim "$scratch/find-error" find 'printf "crates/engine/Cargo.toml\n"; exit 7'; fail 'traversal errored (find status 7)' env PATH="$scratch/find-error:$PATH" bash "$checker" "$work"
reset; make_shim "$scratch/awk-error" awk 'printf "engine\n"; exit 8'; fail 'library name extraction failed' env PATH="$scratch/awk-error:$PATH" bash "$checker" "$work"
reset; make_shim "$scratch/sort-error" sort '/usr/bin/sort "$@"; printf "engine\n"; exit 8'; fail 'sort errored (sort status 8)' env PATH="$scratch/sort-error:$PATH" bash "$checker" "$work"
reset; make_shim "$scratch/paste-error" paste 'printf "engine|lane\n"; exit 9'; fail 'join errored (paste status 9)' env PATH="$scratch/paste-error:$PATH" bash "$checker" "$work"
reset; make_shim "$scratch/rg-error" rg 'printf "valid partial output\n"; exit 7'; fail 'scan errored (rg exit 7)' env PATH="$scratch/rg-error:$PATH" bash "$checker" "$work"
reset; printf 'use dsp_reference::x;\n' >>"$work/crates/engine/src/lib.rs"; make_shim "$scratch/rg-filter" rg 'if [[ ${1:-} == -v ]]; then printf "kept:use dsp_reference::x\n"; exit 7; fi; exec /usr/bin/rg "$@"'; fail 'harness comment filter filter errored (rg exit 7)' env PATH="$scratch/rg-filter:$PATH" bash "$checker" "$work"
printf 'conformance boundary fixtures: ok\n'
