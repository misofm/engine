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
lane.workspace = true
protocol.workspace = true
session.workspace = true
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
# The production boundary checker deliberately permits the conformance dependency only from
# these three exact, cfg(test)-guarded child modules. Keep the hermetic clean fixture shaped like
# the real protocol source so its positive run exercises the same ownership boundary.
for module in controller message_wire session_wire; do
  mkdir -p "$base/crates/protocol/src/$module"
  printf '#[cfg(test)]\nmod tests;\n' >"$base/crates/protocol/src/$module.rs"
  printf 'use conformance::complete_schema_corpus;\n#[test]\nfn extracted_child_fixture_is_test_only() {}\n' \
    >"$base/crates/protocol/src/$module/tests.rs"
done
protocol_fixture_children=(
  crates/protocol/src/controller/tests.rs
  crates/protocol/src/message_wire/tests.rs
  crates/protocol/src/session_wire/tests.rs
)
require_protocol_fixture_children() {
  local fixture_root="$1" child
  for child in "${protocol_fixture_children[@]}"; do
    [[ -r "$fixture_root/$child" ]] || {
      printf 'conformance fixture missing required protocol test child: %s\n' "$child" >&2
      return 1
    }
  done
}
reset() { rm -rf "$work"; cp -R "$base" "$work"; }
pass() { "$@" >/dev/null; }
fail() { local expected=$1; shift; local output; if output=$("$@" 2>&1); then printf 'conformance fixture unexpectedly passed: %s\n' "$expected" >&2; exit 1; fi; [[ "$output" == *"$expected"* ]] || { printf 'wrong conformance diagnostic for %s: %s\n' "$expected" "$output" >&2; exit 1; }; }
require_protocol_fixture_children "$base"
pass bash "$checker" "$base"
(cd "$root" && pass bash scripts/check-conformance-boundaries.sh "$base")
reset; printf '\n[dependencies]\n' >>"$work/crates/dsp-reference/Cargo.toml"; fail 'f64 reference must have zero dependencies' bash "$checker" "$work"
for crate in engine session protocol capi target-smoke effect-contract effect-compiler effect-package lane math; do reset; rm "$work/crates/$crate/Cargo.toml"; fail "missing manifest for $crate" bash "$checker" "$work"; done
for crate in engine session protocol capi target-smoke effect-contract effect-compiler effect-package lane math; do
  reset; rm -rf "$work/crates/$crate/src"
  if [[ "$crate" == protocol ]]; then
    fail 'protocol test-child parent is unreadable' bash "$checker" "$work"
  else
    fail "unreadable source root for $crate" bash "$checker" "$work"
  fi
done
for crate in engine session protocol capi target-smoke effect-contract effect-compiler effect-package lane math; do
  reset; rm "$work/crates/$crate/src/lib.rs"
  if [[ "$crate" == protocol ]]; then
    fail 'protocol default corpus exports scan could not run' bash "$checker" "$work"
  else
    fail "unreadable source root for $crate" bash "$checker" "$work"
  fi
done
reset; mkdir -p "$work/hosts/engine"; fail 'no crate directory found for engine' bash "$checker" "$work"
reset; printf '\n[dependencies]\nconformance.workspace = true\n' >>"$work/crates/engine/Cargo.toml"; fail 'must not depend on a harness crate' bash "$checker" "$work"
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
reset; make_shim "$scratch/sort-error" sort '/usr/bin/sort "$@"; printf "engine\n"; exit 8'; fail 'dependency extraction failed for crates/engine/Cargo.toml (sort status 8)' env PATH="$scratch/sort-error:$PATH" bash "$checker" "$work"
reset; mkdir -p "$scratch/sort-target"; make_shim "$scratch/sort-target" sort 'input=$(cat); if [[ "$input" == dsp-reference* && "$input" == *session && "$input" != */* ]]; then [[ -z ${SHIM_PARTIAL:-} ]] || { printf "%s\\n" "$input"; printf "lane\\n"; }; exit 8; fi; printf "%s\\n" "$input" | exec /usr/bin/sort "$@"'; for partial in '' 1; do fail 'dependency extraction failed for crates/conformance/Cargo.toml (sort status 8)' env SHIM_PARTIAL="$partial" PATH="$scratch/sort-target:$PATH" bash "$checker" "$work"; done
printf 'counterexample rejected: TOML dependency-sort controls (engine and conformance)\n'
reset; mkdir -p "$scratch/sort-workspace"; make_shim "$scratch/sort-workspace" sort 'input=$(cat); if [[ "$input" == *"crates/engine/Cargo.toml"* && "$input" == *"crates/protocol/Cargo.toml"* ]]; then [[ -z ${SHIM_PARTIAL:-} ]] || { printf "%s\\n" "$input"; printf "workspace-sort-sentinel\\n"; }; exit 8; fi; printf "%s\\n" "$input" | exec /usr/bin/sort "$@"'; for partial in '' 1; do fail 'workspace library manifest discovery sort errored (sort status 8)' env SHIM_PARTIAL="$partial" PATH="$scratch/sort-workspace:$PATH" bash "$checker" "$work"; done
printf 'counterexample rejected: workspace gate_sort_lines manifest sort (two partial-output modes)\n'
reset; make_shim "$scratch/paste-error" paste '[[ -z ${SHIM_PARTIAL:-} ]] || printf "engine|lane\n"; exit 9'; for partial in '' 1; do fail 'join errored (paste status 9)' env SHIM_PARTIAL="$partial" PATH="$scratch/paste-error:$PATH" bash "$checker" "$work"; done
reset; make_shim "$scratch/rg-manifest" rg 'case "$*" in *"engine/src/lib.rs"*) [[ -z ${SHIM_PARTIAL:-} ]] || printf "crates/engine/src/lib.rs:1:mod dsp_reference;\n"; exit 7;; esac; exec /usr/bin/rg "$@"'; for partial in '' 1; do fail 'engine dsp_reference module probe scan errored (rg exit 7)' env SHIM_PARTIAL="$partial" PATH="$scratch/rg-manifest:$PATH" bash "$checker" "$work"; done
reset; make_shim "$scratch/rg-module" rg 'case "$*" in *"mod[[:space:]]"*"engine/src/lib.rs"*) [[ -z ${SHIM_PARTIAL:-} ]] || printf "crates/engine/src/lib.rs:1:mod dsp_reference;\n"; exit 7;; esac; exec /usr/bin/rg "$@"'; for partial in '' 1; do fail 'engine dsp_reference module probe scan errored (rg exit 7)' env SHIM_PARTIAL="$partial" PATH="$scratch/rg-module:$PATH" bash "$checker" "$work"; done
reset; make_shim "$scratch/rg-harness" rg 'case "$*" in *"::"*"engine/src"*) [[ -z ${SHIM_PARTIAL:-} ]] || printf "crates/engine/src/lib.rs:1:use dsp_reference::x;\n"; exit 7;; esac; exec /usr/bin/rg "$@"'; for partial in '' 1; do fail 'engine harness use scan scan errored (rg exit 7)' env SHIM_PARTIAL="$partial" PATH="$scratch/rg-harness:$PATH" bash "$checker" "$work"; done
reset; make_shim "$scratch/rg-hosts" rg 'case "$*" in *"hosts sidecars"*) [[ -z ${SHIM_PARTIAL:-} ]] || printf "hosts/x:1:conformance\n"; exit 7;; esac; exec /usr/bin/rg "$@"'; for partial in '' 1; do fail 'hosts/sidecars harness scan scan errored (rg exit 7)' env SHIM_PARTIAL="$partial" PATH="$scratch/rg-hosts:$PATH" bash "$checker" "$work"; done
reset; printf 'use dsp_reference::x;\n' >>"$work/crates/engine/src/lib.rs"; make_shim "$scratch/rg-filter" rg 'if [[ ${1:-} == -v ]]; then [[ -z ${SHIM_PARTIAL:-} ]] || printf "kept:use dsp_reference::x\n"; exit 7; fi; exec /usr/bin/rg "$@"'; for partial in '' 1; do fail 'harness comment filter filter errored (rg exit 7)' env SHIM_PARTIAL="$partial" PATH="$scratch/rg-filter:$PATH" bash "$checker" "$work"; done
reset; make_shim "$scratch/rg-reference-filter" rg 'case "$*" in *"^(dsp_reference|conformance)$"*) [[ -z ${SHIM_PARTIAL:-} ]] || printf "engine\nlane\n"; exit 7;; esac; exec /usr/bin/rg "$@"'; for partial in '' 1; do fail 'reference library-name filter filter errored (rg exit 7)' env SHIM_PARTIAL="$partial" PATH="$scratch/rg-reference-filter:$PATH" bash "$checker" "$work"; done

# Keep the three exact child paths and parent guards fail-closed in the hermetic fixture. The
# production checker owns the guard predicate; this fixture assertion owns the child population
# that makes the predicate meaningful and prevents another clean-base integration omission.
reset; rm "$work/crates/protocol/src/controller/tests.rs"; if require_protocol_fixture_children "$work"; then echo 'missing-child fixture escaped focused acceptance' >&2; exit 1; else printf 'counterexample rejected: missing child (status 1)\n'; fi
reset; sed -i '/^#\[cfg(test)\]$/d' "$work/crates/protocol/src/controller.rs"; fail 'crates/protocol/src/controller.rs must contain exactly one literal adjacent' bash "$checker" "$work"
reset; sed -i 's/^#\[cfg(test)\]$/#[cfg(any(test))]/' "$work/crates/protocol/src/message_wire.rs"; fail 'crates/protocol/src/message_wire.rs must contain exactly one literal adjacent' bash "$checker" "$work"
reset; printf 'use conformance::complete_schema_corpus;\n' >>"$work/crates/protocol/src/session_wire.rs"; fail 'protocol production code must not use a harness crate' bash "$checker" "$work"

# The workspace-sort status assertion must reject a scratch-only helper that swallows a failing
# `sort` status. The mutation touches only the copied helper; the candidate checker and fixture
# remain byte-identical, and the same workspace discriminator drives the red assertion.
if [[ -z ${MUTANT_RUN:-} ]]; then
  sort_mutant_root="$scratch/sort-status-mutant/scripts"; mkdir -p "$sort_mutant_root/lib"
  cp "$checker" "$sort_mutant_root/check-conformance-boundaries.sh"; cp "$root/scripts/lib/gate.sh" "$sort_mutant_root/lib/gate.sh"
  sed -i '/^gate_sort_lines()/,/^}/ s/else rc=\$?/else rc=0/' "$sort_mutant_root/lib/gate.sh"
  if MUTANT_RUN=1 CHECKER="$sort_mutant_root/check-conformance-boundaries.sh" bash "$root/scripts/test-conformance-boundaries.sh" >"$scratch/sort-status-mutant.log" 2>&1; then
    echo 'workspace sort fail-open mutant escaped focused acceptance' >&2; exit 1
  else
    sort_mutant_rc=$?
  fi
  rg -q 'wrong conformance diagnostic for workspace library manifest discovery sort errored' "$scratch/sort-status-mutant.log" || {
    echo 'workspace sort mutant failed outside intended assertion' >&2
    cat "$scratch/sort-status-mutant.log" >&2
    exit 1
  }
  printf 'counter-mutant rejected: workspace gate_sort_lines status (status %s)\n' "$sort_mutant_rc"
fi

# The directed module assertion must reject Astra's exact fail-open consumer mutant.
if [[ -z ${MUTANT_RUN:-} ]]; then
  mutant_root="$scratch/module-mutant/scripts"; mkdir -p "$mutant_root/lib"; cp "$checker" "$mutant_root/check-conformance-boundaries.sh"; cp "$root/scripts/lib/gate.sh" "$mutant_root/lib/gate.sh"
  sed -i '/module_probe=.*gate_scan_collect/,/)" || exit \$?/ s/|| exit \$?/|| true/' "$mutant_root/check-conformance-boundaries.sh"
  if MUTANT_RUN=1 CHECKER="$mutant_root/check-conformance-boundaries.sh" bash "$root/scripts/test-conformance-boundaries.sh" >"$scratch/module-mutant.log" 2>&1; then echo 'module fail-open mutant escaped focused acceptance' >&2; exit 1; else mutant_rc=$?; fi
  rg -q 'conformance fixture unexpectedly passed: engine dsp_reference module probe' "$scratch/module-mutant.log" || { echo 'module mutant failed outside intended assertion' >&2; cat "$scratch/module-mutant.log" >&2; exit 1; }
  printf 'counter-mutant rejected: module consumer (status %s)\n' "$mutant_rc"
fi
printf 'conformance boundary fixtures: ok\n'
