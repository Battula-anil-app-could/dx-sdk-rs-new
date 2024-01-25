use dharitri_wasm::*;
use dharitri_wasm_debug::*;

fn contract_map() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/crowdfunding-dct");

    blockchain.register_contract(
        "file:output/crowdfunding-dct.wasm",
        Box::new(|context| Box::new(crowdfunding_dct::contract_obj(context))),
    );
    blockchain
}

#[test]
fn crowdfunding_claim_failed_rs() {
    dharitri_wasm_debug::denali_rs("denali/crowdfunding-claim-failed.scen.json", contract_map());
}

#[test]
fn crowdfunding_claim_successful_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/crowdfunding-claim-successful.scen.json",
        contract_map(),
    );
}

#[test]
fn crowdfunding_claim_too_early_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/crowdfunding-claim-too-early.scen.json",
        contract_map(),
    );
}

#[test]
fn crowdfunding_fund_rs() {
    dharitri_wasm_debug::denali_rs("denali/crowdfunding-fund.scen.json", contract_map());
}

#[test]
fn crowdfunding_fund_too_late_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/crowdfunding-fund-too-late.scen.json",
        contract_map(),
    );
}

#[test]
fn crowdfunding_init_rs() {
    dharitri_wasm_debug::denali_rs("denali/crowdfunding-init.scen.json", contract_map());
}

#[test]
fn moax_crowdfunding_claim_failed_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/moax-crowdfunding-claim-failed.scen.json",
        contract_map(),
    );
}

#[test]
fn moax_crowdfunding_claim_successful_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/moax-crowdfunding-claim-successful.scen.json",
        contract_map(),
    );
}

#[test]
fn moax_crowdfunding_claim_too_early_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/moax-crowdfunding-claim-too-early.scen.json",
        contract_map(),
    );
}

#[test]
fn moax_crowdfunding_fund_rs() {
    dharitri_wasm_debug::denali_rs("denali/moax-crowdfunding-fund.scen.json", contract_map());
}

#[test]
fn moax_crowdfunding_fund_too_late_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/moax-crowdfunding-fund-too-late.scen.json",
        contract_map(),
    );
}

#[test]
fn moax_crowdfunding_init_rs() {
    dharitri_wasm_debug::denali_rs("denali/moax-crowdfunding-init.scen.json", contract_map());
}
