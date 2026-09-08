#!/usr/bin/env bash
# Mutation tests proving every clause of the lane policy is enforced.
set -euo pipefail

script_directory="$(cd "$(dirname "$0")" && pwd)"
policy_script="$script_directory/check-lane-policy.sh"
scratch_root="$(mktemp -d)"
trap 'rm -rf -- "$scratch_root"' EXIT

create_valid_fixture() {
    local root="$1"
    mkdir -p \
        "$root/crates/lane/src" \
        "$root/crates/lane/tests" \
        "$root/crates/engine/src" \
        "$root/crates/compressor/src" \
        "$root/hosts/host-web/src" \
        "$root/tools/audit/src" \
        "$root/sidecars"

    printf '%s\n' \
        'pub use wide::f32x8 as Simd8;' \
        'pub fn flush(x: f32) -> f32 { x }' \
        >"$root/crates/lane/src/lib.rs"
    printf '%s\n' 'pub fn detect() { let _ = is_x86_feature_detected!("avx2"); }' \
        >"$root/crates/lane/src/backend.rs"
    printf '%s\n' \
        '#![allow(unsafe_code)]' \
        'use core::arch::x86_64::_mm_getcsr;' \
        'pub fn fma_f32_via_f64(a: f32, b: f32, c: f32) -> f32 { a * b + c }' \
        >"$root/crates/lane/src/softfma.rs"
    # Issue #146: the second, and only other, lane file allowed to name a raw architecture
    # intrinsic. AArch64's FPCR has no stable `core::arch` intrinsic, so the canonical render-entry
    # environment reaches it through `core::arch::asm!`.
    printf '%s\n' \
        '#![allow(unsafe_code)]' \
        'use core::arch::asm;' \
        'pub fn canonical() {}' \
        >"$root/crates/lane/src/fpenv.rs"
    printf '%s\n' \
        'impl Lane for f32 {' \
        '    fn fma(self, b: Self, c: Self) -> Self {' \
        '        // LANE-OP-OK(mul_add): the oracle rounds once.' \
        '        f32::mul_add(self, b, c)' \
        '    }' \
        '}' \
        >"$root/crates/lane/src/scalar.rs"
    printf '%s\n' \
        'fn oracle(a: f32, b: f32, c: f32) -> f32 { f32::mul_add(a, b, c) }' \
        >"$root/crates/lane/tests/g3_softfma.rs"
    printf 'pub fn version() {}\n' >"$root/crates/engine/src/lib.rs"
    printf 'pub fn process() {}\n' >"$root/crates/compressor/src/lib.rs"
    printf 'pub fn render() {}\n' >"$root/hosts/host-web/src/lib.rs"
    printf 'fn main() {}\n' >"$root/tools/audit/src/realtime.rs"
    printf '%s\n' '[package]' 'name = "lane"' >"$root/crates/lane/Cargo.toml"
    printf '%s\n' '[package]' 'name = "engine"' >"$root/crates/engine/Cargo.toml"

    printf '%s\n' \
        '[workspace.dependencies]' \
        'wide = { version = "=1.6.1", default-features = false }' \
        >"$root/Cargo.toml"
    printf '%s\n' \
        'version = 4' \
        '' \
        '[[package]]' \
        'name = "bytemuck"' \
        'version = "1.25.2"' \
        '' \
        '[[package]]' \
        'name = "safe_arch"' \
        'version = "1.2.0"' \
        'dependencies = [' \
        ' "bytemuck",' \
        ']' \
        '' \
        '[[package]]' \
        'name = "lane"' \
        'version = "0.1.0"' \
        'dependencies = [' \
        ' "wide",' \
        ' "engine",' \
        ']' \
        '' \
        '[[package]]' \
        'name = "wide"' \
        'version = "1.6.1"' \
        'dependencies = [' \
        ' "bytemuck",' \
        ' "safe_arch",' \
        ']' \
        >"$root/Cargo.lock"
}

# Each mutation is a shell fragment that edits the fixture rooted at `$root`.
expect_failure() {
    local fixture_name="$1"
    local fixture_root="$scratch_root/$fixture_name"
    shift
    create_valid_fixture "$fixture_root"
    local root="$fixture_root"
    eval "$@"

    local output
    if output="$(bash "$policy_script" "$fixture_root" 2>&1)"; then
        printf 'lane policy mutation unexpectedly passed: %s\n' "$fixture_name" >&2
        exit 1
    fi
    printf '%s\n' "$output" | rg -qF 'lane policy failure:' || {
        printf 'lane policy mutation lacked policy diagnostic: %s\n%s\n' "$fixture_name" "$output" >&2
        exit 1
    }
}

expect_failure_with_diagnostic() {
    local name="$1" expected="$2" edit="$3"
    local root="$scratch_root/diagnostic-$name" output
    create_valid_fixture "$root"
    eval "$edit"
    if output="$(bash "$policy_script" "$root" 2>&1)"; then
        printf 'lane diagnostic mutation unexpectedly passed: %s\n' "$name" >&2
        exit 1
    fi
    printf '%s\n' "$output" | rg -qF "lane policy failure: $expected" || {
        printf 'lane diagnostic mutation had wrong output: %s\n%s\n' "$name" "$output" >&2
        exit 1
    }
}

valid_root="$scratch_root/valid root"
create_valid_fixture "$valid_root"
bash "$policy_script" "$valid_root" >/dev/null
(cd "$scratch_root" && bash "$policy_script" "valid root" >/dev/null)

exemption_root="$scratch_root/all-exemptions"
create_valid_fixture "$exemption_root"
mkdir -p "$exemption_root/crates/dsp-reference/src" "$exemption_root/tools/wasm-gates/tests"
printf '%s\n' 'let y = a.mul_add(b, c);' >"$exemption_root/crates/dsp-reference/src/oracle.rs"
printf '%s\n' 'let y = a.mul_add(b, c);' >"$exemption_root/tools/wasm-gates/tests/g5_native_corpus.rs"
bash "$policy_script" "$exemption_root" >/dev/null

expect_loader_failure() {
    local name="$1" expected="$2" edit="$3"
    local root="$scratch_root/loader-$name" policy="$scratch_root/loader-$name.toml" output
    create_valid_fixture "$root"
    cp "$script_directory/policies/lane-source.toml" "$policy"
    eval "$edit"
    if output="$(LANE_POLICY_FILE="$policy" bash "$policy_script" "$root" 2>&1)"; then
        printf 'lane loader mutation unexpectedly passed: %s\n' "$name" >&2
        exit 1
    fi
    printf '%s\n' "$output" | rg -qF 'lane policy failure:' || {
        printf 'lane loader mutation lacked policy diagnostic: %s\n%s\n' "$name" "$output" >&2
        exit 1
    }
    printf '%s\n' "$output" | rg -qF "$expected" || {
        printf 'lane loader mutation had wrong diagnostic: %s\n%s\n' "$name" "$output" >&2
        exit 1
    }
}

expect_loader_failure missing-policy 'lane rule policy invalid' 'rm -f "$policy"'
expect_loader_failure malformed-policy 'lane rule policy invalid' 'printf "%s\n" "version = [" >"$policy"'
expect_loader_failure unknown-field 'unknown or missing top-level fields' 'sed -i "2a unknown = true" "$policy"'
expect_loader_failure missing-required-field 'missing a required field' 'sed -i "/scan_description =/d" "$policy"'
expect_loader_failure empty-rule-population 'exactly four rules' 'sed -i "/^\[\[rules\]\]/,\$c rules = []" "$policy"'
expect_loader_failure duplicate-or-wrong-order 'wrong or duplicate id' \
    'sed -i "0,/id = '\''fusion'\''/s//id = '\''relaxed'\''/" "$policy"'
expect_loader_failure empty-root 'invalid roots' \
    'sed -i "0,/^roots =/s|^roots =.*|roots = [\"\"]|" "$policy"'
expect_loader_failure invalid-regex 'invalid exclude_regex' \
    'sed -i "0,/^exclude_regex =/s|^exclude_regex =.*|exclude_regex = '\''['\''|" "$policy"'

invalid_loader="$scratch_root/invalid-loader"
printf '%s\n' '#!/usr/bin/env python3' 'print("invalid-loader-output")' >"$invalid_loader"
chmod +x "$invalid_loader"
invalid_root="$scratch_root/invalid-loader-root"
create_valid_fixture "$invalid_root"
if output="$(LANE_RULE_LOADER="$invalid_loader" bash "$policy_script" "$invalid_root" 2>&1)"; then
    printf 'lane loader mutation unexpectedly passed: invalid-output\n' >&2
    exit 1
fi
printf '%s\n' "$output" | rg -qF 'lane policy failure:'

expect_loader_status_failure() {
    local name="$1" mode="$2"
    local root="$scratch_root/status-$name" loader="$scratch_root/status-$name-loader" output
    create_valid_fixture "$root"
    case "$mode" in
        malformed)
            printf '%s\n' '#!/usr/bin/env python3' 'print("malformed-loader-output")' >"$loader"
            ;;
        complete)
            printf '%s\n' '#!/usr/bin/env python3' 'import subprocess, sys' \
                "result = subprocess.run([sys.executable, \"$script_directory/lib/gate-rules.py\", sys.argv[1]], capture_output=True, text=True)" \
                'sys.stdout.write(result.stdout)' 'sys.stderr.write(result.stderr)' \
                'raise SystemExit(7)' >"$loader"
            ;;
        partial)
            printf '%s\n' '#!/usr/bin/env python3' 'import subprocess, sys' \
                "result = subprocess.run([sys.executable, \"$script_directory/lib/gate-rules.py\", sys.argv[1]], capture_output=True, text=True)" \
                'sys.stdout.write(result.stdout.splitlines(keepends=True)[0])' \
                'raise SystemExit(7)' >"$loader"
            ;;
        *) printf 'unknown loader status test: %s\n' "$mode" >&2; exit 1 ;;
    esac
    chmod +x "$loader"
    if output="$(LANE_RULE_LOADER="$loader" bash "$policy_script" "$root" 2>&1)"; then
        printf 'lane loader status mutation unexpectedly passed: %s\n' "$name" >&2
        exit 1
    fi
    printf '%s\n' "$output" | rg -qF 'lane policy failure:' || exit 1
    if [[ "$mode" == malformed ]]; then
        printf '%s\n' "$output" | rg -qF 'loader output is invalid' || exit 1
    else
        printf '%s\n' "$output" | rg -qF 'lane source rule loader failed' || exit 1
    fi
}

expect_loader_status_failure status-zero-malformed malformed
expect_loader_status_failure nonzero-complete complete
expect_loader_status_failure nonzero-partial partial

prove_loader_status_mutant() {
    local mutant_dir="$scratch_root/mutant-loader-status" loader="$scratch_root/mutant-loader-status.sh" output
    mkdir -p "$mutant_dir/lib" "$mutant_dir/policies"
    cp "$policy_script" "$mutant_dir/check.sh"
    cp "$script_directory/lib/gate-rules.py" "$mutant_dir/lib/gate-rules.py"
    cp "$script_directory/policies/lane-source.toml" "$mutant_dir/policies/lane-source.toml"
    ln -s "$script_directory/lib/gate.sh" "$mutant_dir/lib/gate.sh"
    sed -i '0,/loader_status=\$?/s//loader_status=0/' "$mutant_dir/check.sh"
    printf '%s\n' '#!/usr/bin/env python3' 'import subprocess, sys' \
        "result = subprocess.run([sys.executable, \"$script_directory/lib/gate-rules.py\", sys.argv[1]], capture_output=True, text=True)" \
        'sys.stdout.write(result.stdout)' 'raise SystemExit(7)' >"$loader"
    chmod +x "$loader"
    set +e
    output="$(if output="$(LANE_RULE_LOADER="$loader" bash "$mutant_dir/check.sh" "$valid_root" 2>&1)"; then
        printf 'lane loader status counter-mutant unexpectedly passed\n'
        exit 1
    else
        printf '%s\n' "$output"
        exit 0
    fi)"
    status=$?
    set -e
    [[ $status == 1 ]] && printf '%s\n' "$output" | rg -qF 'counter-mutant unexpectedly passed' || {
        printf 'lane loader status counter-mutant did not reach its assertion\n%s\n' "$output" >&2
        exit 1
    }
}

prove_loader_status_mutant

four_line_root="$scratch_root/marker-four-lines"
create_valid_fixture "$four_line_root"
sed -i '/LANE-OP-OK(mul_add)/a\        // one\n        // two\n        // three' "$four_line_root/crates/lane/src/scalar.rs"
bash "$policy_script" "$four_line_root" >/dev/null
expect_failure marker-five-lines-before-call \
    'sed -i "/LANE-OP-OK(mul_add)/a\\        // one\\n        // two\\n        // three\\n        // four" "$root/crates/lane/src/scalar.rs"'
expect_failure missing-required-sidecars 'rmdir "$root/sidecars"'
expect_failure empty-required-lane-source 'rm -f "$root/crates/lane/src/"*.rs'
expect_failure empty-workspace-name-aggregate \
    'rm -f "$root/crates/lane/Cargo.toml" "$root/crates/engine/Cargo.toml"'

expect_failure fusion-outside-lane \
    'printf "%s\n" "let y = a.mul_add(b, c);" >>"$root/crates/compressor/src/lib.rs"'
# sidecars/ is scanned the same as crates/, hosts/ and tools/ (scripts/check-lane-policy.sh:52) --
# fused arithmetic there is the
# same D3 violation as anywhere else.
expect_failure fusion-in-a-sidecar \
    'mkdir -p "$root/sidecars/probe-decoder/src"; printf "%s\n" "let y = a.mul_add(b, c);" >"$root/sidecars/probe-decoder/src/lib.rs"'
expect_failure wide-outside-lane \
    'printf "%s\n" "use wide::f32x4;" >>"$root/hosts/host-web/src/lib.rs"'
expect_failure arch-outside-softfma \
    'printf "%s\n" "use core::arch::x86_64::_mm256_add_ps;" >>"$root/tools/audit/src/realtime.rs"'
expect_failure arch-in-second-lane-file \
    'printf "%s\n" "use core::arch::x86_64::_mm256_add_ps;" >>"$root/crates/lane/src/scalar.rs"'
# The #146 exemption is the file `fpenv.rs`, not the lane crate: a third file does not inherit it.
expect_failure arch-in-a-third-lane-file \
    'printf "%s\n" "use core::arch::asm;" >"$root/crates/lane/src/fpenv_extra.rs"'
# #84 phase A: the legacy `core/arch` exemption is gone entirely, so an intrinsic there -- the
# very file the exemption used to name -- is now a failure like any other.
expect_failure deleted-core-arch-has-no-exemption \
    'mkdir -p "$root/crates/engine/src/arch"; printf "%s\n" "use core::arch::x86_64::_mm256_fmadd_ps;" >"$root/crates/engine/src/arch/x86.rs"'
expect_failure deleted-core-detection-has-no-exemption \
    'printf "%s\n" "pub fn detect() { let _ = is_x86_feature_detected!(\"avx2\"); }" >>"$root/crates/engine/src/lib.rs"'
expect_failure relaxed-simd-anywhere \
    'printf "%s\n" "let y = f32x4_relaxed_madd(a, b, c);" >>"$root/crates/lane/src/lib.rs"'
expect_failure unmarked-wide-max \
    'printf "%s\n" "fn m(a: f32x8, b: f32x8) -> f32x8 { a.max(b) }" >>"$root/crates/lane/src/lib.rs"'
expect_failure unmarked-std-mul-add \
    'printf "%s\n" "fn f(a: f32) -> f32 { f32::mul_add(a, a, a) }" >>"$root/crates/lane/src/lib.rs"'
expect_failure new-runtime-detection \
    'printf "%s\n" "let _ = is_x86_feature_detected!(\"avx2\");" >>"$root/crates/compressor/src/lib.rs"'
expect_failure_with_diagnostic fusion-diagnostic \
    'fused multiply-add and the SIMD vocabulary belong to crates/lane (D3, D4)' \
    'printf "%s\n" "let y = a.mul_add(b, c);" >>"$root/crates/compressor/src/lib.rs"'
expect_failure_with_diagnostic relaxed-diagnostic \
    'relaxed SIMD is forbidden on every target (D3)' \
    'printf "%s\n" "let y = f32x4_relaxed_madd(a, b, c);" >>"$root/crates/compressor/src/lib.rs"'
expect_failure_with_diagnostic architecture-diagnostic \
    'raw architecture intrinsics belong to crates/lane/src/{softfma,fpenv}.rs' \
    'printf "%s\n" "use core::arch::x86_64::_mm256_add_ps;" >>"$root/tools/audit/src/realtime.rs"'
expect_failure_with_diagnostic detection-diagnostic \
    'runtime SIMD detection is forbidden outside the enumerated legacy sites (D4)' \
    'printf "%s\n" "let _ = is_x86_feature_detected!(\"avx2\");" >>"$root/crates/compressor/src/lib.rs"'

competing_root="$scratch_root/diagnostic-competing"
create_valid_fixture "$competing_root"
printf '%s\n' 'let y = a.mul_add(b, c);' >>"$competing_root/crates/compressor/src/lib.rs"
printf '%s\n' 'let y = f32x4_relaxed_madd(a, b, c);' >>"$competing_root/crates/compressor/src/lib.rs"
if output="$(bash "$policy_script" "$competing_root" 2>&1)"; then
    printf 'lane competing-rule mutation unexpectedly passed\n' >&2
    exit 1
fi
printf '%s\n' "$output" | rg -qF 'lane policy failure: fused multiply-add and the SIMD vocabulary belong to crates/lane (D3, D4)'
if printf '%s\n' "$output" | rg -qF 'relaxed SIMD is forbidden'; then
    printf 'lane competing-rule mutation did not stop at first rule\n' >&2
    exit 1
fi
expect_failure unpinned-wide-requirement \
    'sed -i "s/=1.6.1/^1.6/" "$root/Cargo.toml"'
expect_failure unpinned-wide-lock \
    'sed -i "s/version = \"1.6.1\"/version = \"1.7.0\"/" "$root/Cargo.lock"'
expect_failure foreign-lane-dependency \
    'sed -i "s/^ \"wide\",$/ \"wide\",\n \"rayon\",/" "$root/Cargo.lock"'
expect_failure foreign-wide-dependency \
    'sed -i "s/^ \"safe_arch\",$/ \"safe_arch\",\n \"serde\",/" "$root/Cargo.lock"'

# Selective failures target one producer/consumer while every earlier invocation delegates.
expect_tool_error() {
    local name="$1" tool="$2" mode="$3" expected="$4" partial="$5"
    local root="$scratch_root/tool-$name" shim="$scratch_root/shim-$name" output
    create_valid_fixture "$root"; mkdir -p "$shim"
    cat >"$shim/$tool" <<'SHIM'
#!/usr/bin/env bash
set -u
joined="$*"; hit=0
case "$INJECT_MODE:$TOOL_NAME" in
 fusion-scan:rg) [[ "$joined" == *mul_add*crates*hosts*tools*sidecars* ]] && hit=1 ;;
 fusion-filter:rg) [[ "$1" == -v && "$joined" == *crates/lane/tests* ]] && hit=1 ;;
 relaxed-scan:rg) [[ "$joined" == *f32x4_relaxed* ]] && hit=1 ;;
 architecture-scan:rg) [[ "$joined" == *'arch::'*crates* ]] && hit=1 ;;
 architecture-filter:rg) [[ "$1" == -v && "$joined" == *softfma* ]] && hit=1 ;;
 detection-scan:rg) [[ "$joined" == *is_x86_feature_detected*crates* ]] && hit=1 ;;
 detection-filter:rg) [[ "$1" == -v && "$joined" == *backend* ]] && hit=1 ;;
 pin:rg) [[ "$joined" == *'-nF wide = {'* ]] && hit=1 ;;
 membership:rg) [[ "$joined" == *'-nx -- engine'* ]] && hit=1 ;;
 lane-find:find) [[ "$joined" == 'crates/lane/src '* ]] && hit=1 ;;
 manifest-find:find) [[ "$joined" == 'crates hosts tools sidecars '* ]] && hit=1 ;;
 lane-sort:sort) hit=1 ;;
 marker-awk:awk) [[ "$joined" == *fifth*scalar.rs* ]] && hit=1 ;;
 version-wide:awk) [[ "$joined" == *package=wide* && "$joined" != *dependencies* ]] && hit=1 ;;
 version-bytemuck:awk) [[ "$joined" == *package=bytemuck* ]] && hit=1 ;;
 version-safe_arch:awk) [[ "$joined" == *package=safe_arch* ]] && hit=1 ;;
 package-name:awk) [[ "$joined" == *in_package*engine/Cargo.toml* ]] && hit=1 ;;
 deps-lane:awk) [[ "$joined" == *package=lane*dependencies* ]] && hit=1 ;;
 deps-wide:awk) [[ "$joined" == *package=wide*dependencies* ]] && hit=1 ;;
esac
if (( hit )); then
  if [[ "$PARTIAL" == 1 ]]; then "$REAL_TOOL" "$@" || true; fi
  printf 'injected-%s-error\n' "$INJECT_MODE" >&2; exit 2
fi
exec "$REAL_TOOL" "$@"
SHIM
    chmod +x "$shim/$tool"
    if output="$(env PATH="$shim:$PATH" TOOL_NAME="$tool" REAL_TOOL="$(command -v "$tool")" INJECT_MODE="$mode" PARTIAL="$partial" bash "$policy_script" "$root" 2>&1)"; then
        printf 'lane injected failure unexpectedly passed: %s\n' "$name" >&2; exit 1
    fi
    printf '%s\n' "$output" | rg -qF "injected-$mode-error" || { printf 'missing injected diagnostic: %s\n%s\n' "$name" "$output" >&2; exit 1; }
    printf '%s\n' "$output" | rg -qF "$expected" || { printf 'wrong injected failure class: %s\n%s\n' "$name" "$output" >&2; exit 1; }
}

for partial in 0 1; do
  expect_tool_error "fusion-scan-$partial" rg fusion-scan 'fusion source scan' "$partial"
  expect_tool_error "fusion-filter-$partial" rg fusion-filter 'fusion source exclusions' "$partial"
  expect_tool_error "relaxed-scan-$partial" rg relaxed-scan 'relaxed SIMD source scan' "$partial"
  expect_tool_error "architecture-scan-$partial" rg architecture-scan 'architecture source scan' "$partial"
  expect_tool_error "architecture-filter-$partial" rg architecture-filter 'architecture source exclusions' "$partial"
  expect_tool_error "detection-scan-$partial" rg detection-scan 'detection source scan' "$partial"
  expect_tool_error "detection-filter-$partial" rg detection-filter 'detection source exclusions' "$partial"
  expect_tool_error "lane-find-$partial" find lane-find 'lane source discovery traversal errored' "$partial"
  expect_tool_error "lane-sort-$partial" sort lane-sort 'lane source discovery sort errored' "$partial"
  expect_tool_error "marker-awk-$partial" awk marker-awk 'lane marker-window extraction failed' "$partial"
  expect_tool_error "pin-$partial" rg pin 'wide manifest pin search failed' "$partial"
  for package in wide bytemuck safe_arch; do expect_tool_error "version-$package-$partial" awk "version-$package" "$package locked version extraction failed" "$partial"; done
  expect_tool_error "manifest-find-$partial" find manifest-find 'workspace manifest discovery traversal errored' "$partial"
  expect_tool_error "package-name-$partial" awk package-name 'workspace package-name extraction failed' "$partial"
  expect_tool_error "deps-lane-$partial" awk deps-lane 'lane locked dependency extraction failed' "$partial"
  expect_tool_error "deps-wide-$partial" awk deps-wide 'wide locked dependency extraction failed' "$partial"
  expect_tool_error "membership-$partial" rg membership 'workspace dependency membership search failed' "$partial"
done

prove_lane_mutant_rejected() {
  local name="$1" edit="$2" tool="$3" mode="$4"
  local mutant_dir="$scratch_root/mutant-$name" output status
  mkdir -p "$mutant_dir/lib" "$mutant_dir/policies"
  cp "$policy_script" "$mutant_dir/check.sh"
  cp "$script_directory/lib/gate-rules.py" "$mutant_dir/lib/gate-rules.py"
  cp "$script_directory/policies/lane-source.toml" "$mutant_dir/policies/lane-source.toml"
  ln -s "$script_directory/lib/gate.sh" "$mutant_dir/lib/gate.sh"
  sed -i "$edit" "$mutant_dir/check.sh"
  set +e
  output="$(policy_script="$mutant_dir/check.sh"; expect_tool_error "mutant-$name" "$tool" "$mode" ignored 1 2>&1)"
  status=$?
  set -e
  [[ $status == 1 ]] && printf '%s\n' "$output" | rg -qF 'unexpectedly passed' || {
    printf 'lane counter-mutant did not reach intended assertion: %s\n%s\n' "$name" "$output" >&2; exit 1;
  }
}
prove_lane_mutant_rejected nonempty-version \
  '/versions=.*locked_version/,/\[\[ -n "$versions"/s/|| {.*}/|| true/' awk version-bytemuck
prove_lane_mutant_rejected failed-lane-find \
  '/lane_sources_raw=.*gate_find_collect/c\lane_sources_raw="$(gate_find_collect '\''lane source discovery'\'' crates/lane/src -name '\''*.rs'\'' -type f)" || lane_sources_raw="$(/usr/bin/find crates/lane/src -name '\''*.rs'\'' -type f)"' \
  find lane-find
prove_lane_mutant_rejected failed-dependency-list \
  '/lane_dependencies=.*locked_dependencies/s/|| {.*}/|| true/' awk deps-lane
prove_lane_mutant_rejected failed-membership \
  '/membership_rc == 1/s/|| .*$/|| continue/' rg membership

printf 'lane policy mutation tests: ok\n'
