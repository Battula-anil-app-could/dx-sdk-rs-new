use dharitri_wasm_debug::*;

fn contract_map() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/multisig");

    blockchain.register_contract(
        "file:output/multisig.wasm",
        Box::new(|context| Box::new(multisig::contract_obj(context))),
    );
    blockchain
}

#[test]
fn test_change_board_rs() {
    dharitri_wasm_debug::denali_rs("denali/changeBoard.scen.json", contract_map());
}

#[test]
fn test_change_quorum_rs() {
    dharitri_wasm_debug::denali_rs("denali/changeQuorum.scen.json", contract_map());
}

#[test]
fn test_change_quorum_too_big_rs() {
    dharitri_wasm_debug::denali_rs("denali/changeQuorum_tooBig.scen.json", contract_map());
}
