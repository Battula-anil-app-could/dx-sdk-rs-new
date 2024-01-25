use dharitri_wasm_debug::*;

fn contract_map() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.register_contract(
        "file:output/send-tx-repeat.wasm",
        Box::new(|context| Box::new(send_tx_repeat::contract_obj(context))),
    );
    blockchain
}

#[test]
fn test_send_tx_repeat_without_data_denali_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/send_tx_repeat_without_data.scen.json",
        contract_map(),
    );
}

#[test]
fn test_send_tx_repeat_with_data_denali_rs() {
    dharitri_wasm_debug::denali_rs("denali/send_tx_repeat_with_data.scen.json", contract_map());
}
