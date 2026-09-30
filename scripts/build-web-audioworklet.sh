#!/usr/bin/env bash
set -euo pipefail

# Builds the shipped AudioWorklet module and prints `AudioWorklet module <sha256>` on stdout.
#
# Issue #1061 (owner decision 5, docs/rulings/engine-footprint-2026-09-28.md): the module's digest
# is not held to a committed pin on every change. Every PR builds the module here, every artifact
# gate reads those exact bytes, and the `artifact-identity` job reports whether they differ from the
# digest the base commit's own CI run recorded. The committed pin (`hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256`) is
# the release fingerprint: a release change re-pins it, and `npm-publish.yml` publishes only bytes
# equal to it. docs/RELEASE.md is the procedure.
#
# Issue #1109: one cargo build gives two modules. rustc's output is the *named twin*: it keeps the
# wasm `name` section, the function names a devtools stack trace prints and the gates that find
# functions by name read (`check-web-audioworklet-callgraph.py`, `check-web-audioworklet-v8-spill.py`,
# `check-scalar-oracle-absent.py`). The *shipped* module is the named twin minus that one section,
# every other byte in place (`strip-wasm-names.py`): nothing that runs in the browser reads the
# names, and every page load would download them. The shipped module is the one this script names,
# digests, pins and delivers; the named twin is a non-shipped debug artifact, written only where
# `--named-twin` asks, and `strip-wasm-names.py check` proves a pair are twins.
#
# Modes (one at most):
#   (none)         the delivery closure: module, host and worklet JavaScript, declaration,
#                  parameter metadata and ABI layout.
#   --module-only  the module alone (issue #1009), built by exactly the cargo line below, for a gate
#                  that reads only the module (`run-wasm-gates.sh`'s V8 spill gate, the identity
#                  job's twin build). The build has one home, here.
#   --check-pin    the delivery closure, refused (exit 1, nothing written) unless the module's digest
#                  equals the committed pin: the release fingerprint check, runnable locally.
# With any mode, `--named-twin DIR` also writes the named twin,
# `miso-engine-v1-audio-worklet.simd128.named.wasm`, into DIR: an existing, empty, non-symlink
# directory other than the output.
mode=delivery
named_dir=
while (($# > 1)); do
  case $1 in
    --module-only | --check-pin)
      [[ $mode == delivery ]] || break
      [[ $1 == --module-only ]] && mode=module || mode=pinned
      shift
      ;;
    --named-twin)
      [[ -z $named_dir && -n ${2-} ]] || break
      named_dir=$2
      shift 2
      ;;
    *) break ;;
  esac
done
if (($# != 1)) || [[ $1 == -* ]]; then
  echo "usage: $0 [--module-only | --check-pin] [--named-twin EMPTY_DIRECTORY] EMPTY_OUTPUT_DIRECTORY" >&2
  exit 2
fi

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)
output_dir=$1
for dir in "$output_dir" ${named_dir:+"$named_dir"}; do
  if [[ ! -d "$dir" || -L "$dir" ]]; then
    echo "output must be an existing non-symlink directory" >&2
    exit 2
  fi
  if [[ -n "$(find "$dir" -mindepth 1 -maxdepth 1 -print -quit)" ]]; then
    echo "output directory must be empty; refusing overwrite" >&2
    exit 2
  fi
done
if [[ -n $named_dir && "$(cd "$named_dir" && pwd -P)" == "$(cd "$output_dir" && pwd -P)" ]]; then
  echo "the named twin must not be written into the output directory" >&2
  exit 2
fi

simd_target=$(mktemp -d)
cleanup() {
  rm -rf -- "$simd_target"
}
trap cleanup EXIT

# Owner decision W4-D1 (#83, 2026-08-24): the app's browser floor guarantees `simd128`, so exactly
# one artifact ships. The scalar worklet build and the dual-artifact selection in `host.js` are
# gone; `host.js` probes `simd128` at init and fails with a typed `miso.unsupported.v1` error when
# the probe fails -- the browser twin of D4's native boot attestation. No scalar wasm build is
# left at all: `lane` refuses wasm32 without `simd128` at compile time (issue #1062).
#
# The browser artifact is the one place the workspace's `debug = 1` (issue 083 D12) is pure cost.
# It exists so a native profile or core dump names a kernel; a downloaded AudioWorklet module pays
# for the DWARF on every page load and cannot use it in production. Measured on this repository:
# 2,153,061 bytes before D12, 16,661,225 with `debug = 1`, 1,940,863 with the debug
# information stripped -- fat LTO alone makes the module *smaller* than it was, and the whole of
# the growth is DWARF.
#
# Stripped here, in the delivery script, and deliberately not in `[profile.release]`: the native
# artifacts keep their line tables. To build a debuggable browser module, override this with
# `MISO_ENGINE_WEB_STRIP=none`. The `name` section is not stripped here but after the build (see
# the top of this file): the gates read it from the named twin.
strip_flag="-C strip=${MISO_ENGINE_WEB_STRIP:-debuginfo}"

# The artifact is content-addressed, so every path rustc embeds in it must be a
# function of the SOURCE and nothing else. It is not by default: dependency
# sources live under CARGO_HOME, whose absolute path differs between a
# developer's machine and CI (`/root/.cargo` vs `/home/runner/.cargo`), and
# rustc bakes those paths into panic locations. Without remapping, that would
# make the artifact digest a function of WHERE cargo's registry sits.
#
# Remapping both roots to fixed labels makes the digest reproducible anywhere.
# Verified: with these flags the digest is identical under CARGO_HOME=/root/.cargo
# and CARGO_HOME=/home/runner/.cargo, which previously produced two different ones,
# and also under a third repo-path/CARGO_HOME combination.
cargo_home=${CARGO_HOME:-$HOME/.cargo}
remap="--remap-path-prefix=$cargo_home=/cargo --remap-path-prefix=$repo_root=/repo"

(
  cd "$repo_root"
  CARGO_TARGET_DIR="$simd_target" RUSTFLAGS="-C target-feature=+simd128 $strip_flag $remap" \
    cargo build --locked --release --target wasm32-unknown-unknown -p host-web
)

named="$simd_target/wasm32-unknown-unknown/release/host_web.wasm"
artifact="$simd_target/miso-engine-v1-audio-worklet.simd128.wasm"
python3 -B "$repo_root/scripts/strip-wasm-names.py" strip "$named" "$artifact" >&2
observed=$(sha256sum "$artifact" | awk '{print $1}')

if [[ $mode == pinned ]]; then
  pin_file="$repo_root/hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256"
  expected=$(tr -d '\n' <"$pin_file")
  [[ "$observed" == "$expected" ]] || {
    printf 'AudioWorklet artifact pin mismatch: expected=%s observed=%s (docs/RELEASE.md)\n' \
      "$expected" "$observed" >&2
    exit 1
  }
fi

printf 'AudioWorklet module %s\n' "$observed"
if [[ -n $named_dir ]]; then
  cp "$named" "$named_dir/miso-engine-v1-audio-worklet.simd128.named.wasm"
fi
if [[ $mode == module ]]; then
  cp "$artifact" "$output_dir/miso-engine-v1-audio-worklet.simd128.wasm"
  exit 0
fi

cp "$artifact" "$output_dir/miso-engine-v1-audio-worklet.simd128.wasm"
cp "$repo_root/hosts/host-web/web/miso-engine-v1-audio-worklet.js" "$output_dir/"
cp "$repo_root/hosts/host-web/web/miso-engine-v1-audio-worklet-host.js" "$output_dir/"
cp "$repo_root/hosts/host-web/web/prepared-control.js" "$output_dir/"
cp "$repo_root/hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts" "$output_dir/"

# Issue #137 D4: the parameter metadata ships beside the module, so the app never introspects the
# Wasm for names, units, ranges, defaults or enumerations. The effect list is read from
# `launch_native_effect_registry()`, so an effect cannot be in the engine and missing here.
(
  cd "$repo_root"
  cargo run --locked --release -q -p parameter-metadata -- --write "$output_dir"
) >/dev/null
