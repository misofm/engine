#!/usr/bin/env bash
# Focused qualification for the builders' caller-owned empty-directory contract.
set -euo pipefail

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)
scratch=$(mktemp -d)
cleanup() {
  rm -rf -- "$scratch"
}
trap cleanup EXIT

mock_bin="$scratch/mock-bin"
mkdir "$mock_bin"
cat >"$mock_bin/cargo" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$*" >>"$(dirname "$0")/cargo.log"
if [[ $1 == build ]]; then
  case " $* " in
    *" -p host-web "*) artifact=host_web ;;
    *" -p flac-decoder "*) artifact=flac_decoder ;;
    *) echo "unexpected cargo build invocation" >&2; exit 1 ;;
  esac
  output="$CARGO_TARGET_DIR/wasm32-unknown-unknown/release/$artifact.wasm"
  mkdir -p "$(dirname "$output")"
  printf '%s fixture\n' "$artifact" >"$output"
  exit 0
fi
if [[ $1 == run && " $* " == *" -p parameter-metadata "* ]]; then
  output=${@: -1}
  printf 'parameter metadata fixture\n' >"$output/parameter-metadata.json"
  exit 0
fi
echo "unexpected cargo invocation" >&2
exit 1
EOF
chmod +x "$mock_bin/cargo"

cat >"$mock_bin/sha256sum" <<EOF
#!/usr/bin/env bash
set -euo pipefail
case "\$1" in
  */host_web.wasm) printf '%s  %s\\n' "$(tr -d '\n' <"$repo_root/hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256")" "\$1" ;;
  */flac_decoder.wasm) printf '%s  %s\\n' "$(tr -d '\n' <"$repo_root/sidecars/flac-decoder/decoder-artifact.sha256")" "\$1" ;;
  *) echo "unexpected sha256sum input" >&2; exit 1 ;;
esac
EOF
chmod +x "$mock_bin/sha256sum"

run_builder() {
  local builder=$1
  local output=$2
  local log=$3
  local mock_log="$mock_bin/cargo.log"
  rm -f "$mock_log" "$log"
  local status=0
  PATH="$mock_bin:$PATH" bash "$repo_root/scripts/$builder" "$output" || status=$?
  if [[ -e $mock_log ]]; then
    cp "$mock_log" "$log"
  fi
  return "$status"
}

assert_refuses_without_build() {
  local builder=$1
  local output=$2
  local expected_sentinel=$3
  local log="$scratch/$builder-refusal.log"
  local status=0
  run_builder "$builder" "$output" "$log" >/dev/null 2>&1 || status=$?
  [[ $status == 2 ]] || {
    echo "$builder accepted an invalid output directory" >&2
    exit 1
  }
  cmp -s "$expected_sentinel" "$output/sentinel" || {
    echo "$builder changed a refused output directory" >&2
    exit 1
  }
  [[ ! -e $log ]] || {
    echo "$builder built after refusing output" >&2
    exit 1
  }
}

assert_refuses_path() {
  local builder=$1
  local output=$2
  local log="$scratch/$builder-path-refusal.log"
  local status=0
  run_builder "$builder" "$output" "$log" >/dev/null 2>&1 || status=$?
  [[ $status == 2 ]] || {
    echo "$builder accepted invalid output path $output" >&2
    exit 1
  }
  [[ ! -e $log ]] || {
    echo "$builder built after refusing output path" >&2
    exit 1
  }
}

check_builder() {
  local builder=$1
  local prefix=$2
  local output="$scratch/$prefix-empty"
  local log="$scratch/$prefix-success.log"
  mkdir "$output"
  run_builder "$builder" "$output" "$log"
  [[ -s $log ]] || {
    echo "$builder did not run its mocked build" >&2
    exit 1
  }
  case "$builder" in
    build-web-audioworklet.sh)
      cmp -s <(printf 'host_web fixture\n') "$output/miso-engine-v1-audio-worklet.simd128.wasm"
      cmp -s "$repo_root/hosts/host-web/web/miso-engine-v1-audio-worklet.js" \
        "$output/miso-engine-v1-audio-worklet.js"
      cmp -s "$repo_root/hosts/host-web/web/miso-engine-v1-audio-worklet-host.js" \
        "$output/miso-engine-v1-audio-worklet-host.js"
      cmp -s "$repo_root/hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts" \
        "$output/miso-engine-v1-audio-worklet-host.d.ts"
      cmp -s <(printf 'parameter metadata fixture\n') "$output/parameter-metadata.json"
      ;;
    build-flac-decoder.sh)
      cmp -s <(printf 'flac_decoder fixture\n') "$output/flac-decoder.wasm"
      cmp -s "$repo_root/sidecars/flac-decoder/flac-decoder.js" "$output/flac-decoder.js"
      cmp -s "$repo_root/sidecars/flac-decoder/flac-decoder.d.ts" "$output/flac-decoder.d.ts"
      cmp -s "$repo_root/sidecars/flac-decoder/decoder-artifact.sha256" \
        "$output/decoder-artifact.sha256"
      ;;
  esac

  local non_empty="$scratch/$prefix-non-empty"
  local expected_sentinel="$scratch/$prefix-non-empty-sentinel"
  mkdir "$non_empty"
  printf 'do not overwrite\n' >"$non_empty/sentinel"
  cp "$non_empty/sentinel" "$expected_sentinel"
  assert_refuses_without_build "$builder" "$non_empty" "$expected_sentinel"

  local symlink_target="$scratch/$prefix-symlink-target"
  local symlink_output="$scratch/$prefix-symlink-output"
  mkdir "$symlink_target"
  printf 'do not overwrite\n' >"$symlink_target/sentinel"
  ln -s "$symlink_target" "$symlink_output"
  local symlink_log="$scratch/$prefix-symlink.log"
  local status=0
  run_builder "$builder" "$symlink_output" "$symlink_log" >/dev/null 2>&1 || status=$?
  [[ $status == 2 ]] || {
    echo "$builder accepted a symlink output directory" >&2
    exit 1
  }
  cmp -s <(printf 'do not overwrite\n') "$symlink_target/sentinel" || {
    echo "$builder changed a symlink target" >&2
    exit 1
  }
  [[ ! -e $symlink_log ]] || {
    echo "$builder built after refusing symlink output" >&2
    exit 1
  }

  assert_refuses_path "$builder" "$scratch/$prefix-missing"
  local regular_file="$scratch/$prefix-regular-file"
  printf 'not a directory\n' >"$regular_file"
  assert_refuses_path "$builder" "$regular_file"
}

check_builder build-web-audioworklet.sh web
check_builder build-flac-decoder.sh flac
echo "SDK artifact builder output-directory contract passed"
