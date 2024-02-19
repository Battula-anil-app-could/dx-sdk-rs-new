use dharitri_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/digital-cash");

    blockchain.register_contract(
        "file:output/digital-cash.wasm",
        digital_cash::ContractBuilder,
    );
    blockchain
}

#[test]
#[ignore] // verify_ed25519 not implemented
fn claim_moax_rs() {
    dharitri_sc_scenario::run_rs("scenarios/claim-moax.scen.json", world());
}

#[test]
#[ignore] // verify_ed25519 not implemented
fn claim_dct_rs() {
    dharitri_sc_scenario::run_rs("scenarios/claim-dct.scen.json", world());
}

#[test]
fn fund_moax_and_dct_rs() {
    dharitri_sc_scenario::run_rs("scenarios/fund-moax-and-dct.scen.json", world());
}

#[test]
fn set_accounts_rs() {
    dharitri_sc_scenario::run_rs("scenarios/set-accounts.scen.json", world());
}

#[test]
fn withdraw_moax_rs() {
    dharitri_sc_scenario::run_rs("scenarios/withdraw-moax.scen.json", world());
}

#[test]
fn withdraw_dct_rs() {
    dharitri_sc_scenario::run_rs("scenarios/withdraw-dct.scen.json", world());
}

#[test]
fn forward_rs() {
    dharitri_sc_scenario::run_rs("scenarios/forward.scen.json", world());
}
