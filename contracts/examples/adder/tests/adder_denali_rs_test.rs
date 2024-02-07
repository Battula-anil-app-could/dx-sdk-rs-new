use dharitri_wasm::*;
use dharitri_wasm_debug::*;

fn contract_map() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/adder");

    blockchain.register_contract(
        "file:output/adder.wasm",
        Box::new(|context| Box::new(adder::contract_obj(context))),
    );
    blockchain
}

#[test]
fn adder_rs() {
    dharitri_wasm_debug::denali_rs("denali/adder.scen.json", contract_map());
}
