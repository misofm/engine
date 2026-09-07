#!/usr/bin/env bash
set -u
name=$1
shift
repo_root=/tmp/issue470-qual-source-d6f78803
out_root=/tmp/issue470-full-prepin
if [[ $name == qualification-* ]]; then
  cd "$repo_root/hosts/host-web/qualification" || exit 125
else
  cd "$repo_root" || exit 125
fi
utc=$(date -u +%Y-%m-%dT%H:%M:%SZ)
cwd=$(pwd -P)
head=$(git rev-parse HEAD)
status=$(git status --short --branch)
printf -v argv '%q ' "$@"
meta="$out_root/$name.meta"
printf 'pre_command_utc=%s\npre_command_cwd=%s\npre_command_head=%s\npre_command_status=%s\nargv=%s\n' "$utc" "$cwd" "$head" "$status" "$argv" > "$meta"
sha256sum "$repo_root/hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md" "$repo_root/hosts/host-web/qualification/results.json" "$repo_root/hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256" >> "$meta"
sha256sum "$repo_root/hosts/host-web/tests/browser-v1/expected.json" >> "$meta"
sha256sum /tmp/issue470-qualified-artifact/miso-engine-v1-audio-worklet.simd128.wasm >> "$meta"
set +e
"$@" > "$out_root/$name.stdout" 2> "$out_root/$name.stderr"
status_code=$?
set -e
printf 'exit=%s\n' "$status_code" >> "$meta"
printf '%s\n' "[$name] exit=$status_code"
cat "$out_root/$name.stdout"
cat "$out_root/$name.stderr" >&2
exit "$status_code"
