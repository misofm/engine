Completed one packaging-only checkpoint at clean head `b5317fe2e641b2a3660dda343e3b33e76b065b10`.

Manifest: [stdout-gzip-manifest.json](/home/bl/misofm/engine-shared-hex-authority/artifacts/issue543-evidence-packaging/stdout-gzip-manifest.json:1)

All fourteen decompressions passed:

| File | Original bytes / SHA-256 | Gzip bytes / SHA-256 |
|---|---:|---:|
| `engine-test.stdout` | 283 / `52b102d0103fc1d4aa42c4cfd4d56f910b43a11ec79a2970fb525bb7d02dfe88` | 159 / `c8d16bc727165a1878b92df2821b56af67c33949edb9e1d9d7c45c4858e6ba0a` |
| `fmt.stdout` | 433 / `7b1e4d6de9df708f0a3a1f95fe46337bc19e225c839f3e88411ae9803688d573` | 267 / `de144c983f270faf69e38cca2058746504f7bce599990aa85220fefb955a712e` |
| `protocol-test.stdout` | 14819 / `02b913d6f1defe97171d1e1de9790189f74b38bd41fc013c339af459d79238b5` | 4010 / `c22cb776fe166a863eb855e7d1b573ab18fd01f3961852d9d1823c564722329f` |
| `03-lock-update-bench-support-digest-tests.stdout` | 458 / `9864adbe5ed1d69813eb5258218f162d29f50255e01e1a5136b5ce8a4ed3ef6d` | 244 / `f892d1ea837d37d48fe2a56bfa4030725d941cc88f089de56fdbfb0696bf68d8` |
| `04-graph-compiler.stdout` | 7520 / `608db4c40018e6e648e34327d5fe567afa31e487d89170287ccfa1d278f74407` | 2287 / `50974147a68fe380feeae9b4fea835777567a020440bbbbe3779727c204503cb` |
| `05-math-lane-identity-determinism.stdout` | 621 / `90764217ec729d1b4c7cbd4fb0f648f14714d032da7322fb21ad6fa79ddb5e48` | 279 / `d3444a21b0b4508b570b45df17be7001414e1002c9366fc6f0cdd1c0d2ceccb4` |
| `06-effect-runtime-determinism.stdout` | 286 / `8350df6407f9f6bf2e2a50b65c00cc63a5dedfeee74ea29b016c21f44dd54447` | 195 / `0beabee2134f9d7bb95411efaf6c448b169674ff044be892cc62dae8e74feaac` |
| `07-native-pcm-runner.stdout` | 1848 / `7f4b1a55b4fce5a892ba310c46f45ee4aaba3ef599e90e29bc14fee3b6a0ecc8` | 678 / `fe1d23caa52ee533710979ac739cfe2b0b17d1e1aa29f7d8a58e547e5c326d87` |
| `08-wasm-gates.stdout` | 1048 / `2fa8e43f3080cc243a3ee12c3700e3ad1c40c3f1717a917e89b9397cd0600dce` | 351 / `6aee2e9630d00cdb490e865841176f6b3d092661aea90b92127f07739b190eab` |
| `09-effect-package.stdout` | 325 / `4cb0fa6dfc9c058f35ac749e84bb923de8677501217eb25521fd108691084dca` | 218 / `dc77cabd8aa9ce03550ecad585873830fa651d815146d8a437e37108b8a59749` |
| `10-effect-compiler.stdout` | 847 / `33151b52af88ec16b7ba3d7dc45cefcf4d132185a4752b3ad82a90c47309761a` | 414 / `f4ab7e3a8abefdfd59edd1f611b5a4d0d74a3e27bc097373aeaf22f8d66a2f19` |
| `11-stem-hasher.stdout` | 1206 / `6329edd353a0a99d8f4dfe29f27ca61b14263f039d7a2ce12ebc58af092a36cb` | 371 / `e4b0c2e145f4db53be2003233b03e398bac406d54ce1120d4bfbd18319b7083a` |
| `12-host-web-native-tests.stdout` | 444 / `cc347419079042033a01a0d6fc4257a44c8c44dc4ad692afcca8be4f04eac2d1` | 195 / `2116b5ebcc7e33420b7174e4e4fa21a33b819a835cf134a4dd0500f414b85e84` |
| `14-parameter-metadata-tests.stdout` | 1381 / `e314c97a7f719662b7be347253664042aed5b1a71f03c3c6c0e6f2f487372b6a` | 422 / `88215835167104f92c319706a812753639b669f17d643dfee71aa6ee36716d9d` |

Path audit: exactly 29 paths—14 plaintext deletions, 14 `.stdout.gz` additions, and 1 manifest addition. Because staging was prohibited, raw `git diff --name-only` lists the 14 tracked deletions; the 15 additions are untracked and were separately verified in the combined 29-path audit.

`git diff --check`: PASS, status 0. No non-packaging paths differ from HEAD. No tests, builds, formatters, policy commands, product commands, commits, pushes, staging, or GitHub mutations were performed.