# RT-4 public full-chain qualification

Source candidate `e4bcaa2feae13c9f016bb7b2e1eaff8bd7314547` passed full `cargo test --locked --workspace`, including doctests: 275 result blocks, 1,575 passed, zero failed, 24 ignored. Delivered-main baseline had 274 blocks, 1,569 passed, zero failed and 24 ignored.

Astra accepted the final third source attempt and separately accepted supported-Wasm evidence. The named scalar non-LTO inspection does not claim fat-LTO coverage or an executed source fallback.

Independent REPIN and normal builds produced artifact `10b0581f72d921b520e4066b82dc32cb7bea90b757c20ccca3dfc52cf7b9e098`. Current static, resource (26 red mutations), hermetic and Chromium/Firefox/WebKit qualification passed, followed by each browser's matrix check. The initial browser launch stopped before execution because Playwright dependencies were absent; npm ci resolved that. Initial static/hermetic checks stopped because Cargo was outside PATH; corrected runs passed. Original setup-failure logs are retained.

No timing was performed. #430 owns live compiler/bank integration and #431 owns full-chain measurement. #436 retains the separate uninvoked RT-3 capture. This slice alone does not establish an end-to-end speedup or close audit RT-4.
