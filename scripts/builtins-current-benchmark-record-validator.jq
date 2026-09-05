include "builtins-benchmark-record-validator";

def current_manifest:
  "b244da45d88d670951205098b7516af20387a141eccb3bf60edb61e8ba57a919";
def current_input:
  if .workload_kind == "full_chain_filters" and .sample_rate_hz == 48000 then "6a1633442678cfdecb2872deacd053e727c47f0bc94039a84b4e950949e195d0"
  elif .workload_kind == "full_chain_filters" and .sample_rate_hz == 96000 then "ac9e825b5051a161ca731b04bd9b9b825bad6484c3a3f911551051e316224fa0"
  elif .workload_kind == "identity_chain" and .sample_rate_hz == 48000 then "15dfc8b6d918d01a5d6e46417e37a10023d31a85391e8fb2371af0cdc055dd95"
  elif .workload_kind == "identity_chain" and .sample_rate_hz == 96000 then "962bc24d4104cb5a30e3a5aa158a5ca1075cae01f08433d2c7cbe8c1271cd99a"
  elif .workload_kind == "matrix_ramp" and .sample_rate_hz == 48000 then "f0d94928bed16804a26befde5eaabd3a8c233afa194a5cdcb259141af78c831b"
  elif .workload_kind == "matrix_ramp" and .sample_rate_hz == 96000 then "ef5bf8c4e954c1e497eea997bffeb85fabad69ac6966f2798bd34ce2fa5ced6f"
  elif .workload_kind == "meter_success_full" and .sample_rate_hz == 48000 then "ded3579ee8ffbf79d920648a33a7e2f35fa9c9b386e98ef469d583830ef992de"
  elif .workload_kind == "meter_success_full" and .sample_rate_hz == 96000 then "aa1c4d8835753ce290d7abcf1cbf3ffdb98b79a58f0ec6cd0cce6614f5befef9"
  elif .workload_kind == "prepare_256_tracks" and .sample_rate_hz == 48000 then "a1dec8525c20505a9b440e6cf93fa6ffa1144896c889fa3abd94f76224f3e210"
  elif .workload_kind == "prepare_256_tracks" and .sample_rate_hz == 96000 then "880faace46cfa2e9f454d625e54206aa752a9947292057a6b58f64224ea13f30"
  else null end;

def current_record_valid:
  (.fixture_manifest_sha256 == current_manifest and .input_fixture_sha256 == current_input) and
  ((.fixture_manifest_sha256 = frozen_manifest_sha256) |
   (.input_fixture_sha256 = frozen_input_sha256) |
   builtins_benchmark_record_valid);
