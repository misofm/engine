fn main() {
    graph::test_only_reset_bank_chain_construction_facts();
    let paired = builtins_compiler::test_only_prepared_pair_graph(false);
    println!("paired={:?}", graph::test_only_bank_chain_construction_facts());
    drop(paired);

    graph::test_only_reset_bank_chain_construction_facts();
    let unpaired = builtins_compiler::test_only_prepared_unpaired_graph();
    println!("unpaired={:?}", graph::test_only_bank_chain_construction_facts());
    drop(unpaired);
}
