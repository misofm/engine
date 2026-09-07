# Failed first implementation tranche

## Failed buildable attempt1 checkpoint — independent review pending

Luna's first production/helper-test tranche compiles and its two private tests pass, but root detected a linked-mode regression before further layers. The combined DualMono/LinkedEqual arm supplies [LL,LL,RR,RR]; linked equal requires [LL,RR,RR,LL]. The private expected-word test derives its expectation from the returned route and repeats this mistake. Root captured the existing independent scalar oracle on the exact candidate: `root-scalar-oracle` exits101, failing Maximum-left open-gate identity at frame5424 (actual995039245 vs expected995049015). DualMono's independent comparison passed before that failure.

This is a candid, compiling recovery checkpoint, **not** source acceptance. Production files are only kernel.rs/lib.rs. All actual format/compiler/test command/status/output/source identities are preserved in artifacts/issue534-luna-attempt1, including the earlier invalid multi-filter and temporary-field compile failures. No public fixture/state/mutation/candidate-lowering work was started. Luna is stopped; one consolidated Astra attempt1 verdict will determine the bounded Sol2 continuation. No DSP pin, algorithm contract, timing or gate was weakened.
