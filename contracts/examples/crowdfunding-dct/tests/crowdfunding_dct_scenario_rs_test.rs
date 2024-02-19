use dharitri_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/crowdfunding-dct");

    blockchain.register_contract(
        "file:output/crowdfunding-dct.wasm",
        crowdfunding_dct::ContractBuilder,
    );
    blockchain
}

#[test]
fn generated_fund_rs() {
    dharitri_sc_scenario::run_rs("scenarios/_generated_fund.scen.json", world());
}

#[test]
fn generated_init_rs() {
    dharitri_sc_scenario::run_rs("scenarios/_generated_init.scen.json", world());
}

#[test]
fn generated_query_status_rs() {
    dharitri_sc_scenario::run_rs("scenarios/_generated_query_status.scen.json", world());
}

#[test]
fn generated_sc_err_rs() {
    dharitri_sc_scenario::run_rs("scenarios/_generated_sc_err.scen.json", world());
}

#[test]
fn crowdfunding_claim_failed_rs() {
    dharitri_sc_scenario::run_rs("scenarios/crowdfunding-claim-failed.scen.json", world());
}

#[test]
fn crowdfunding_claim_successful_rs() {
    dharitri_sc_scenario::run_rs("scenarios/crowdfunding-claim-successful.scen.json", world());
}

#[test]
fn crowdfunding_claim_too_early_rs() {
    dharitri_sc_scenario::run_rs("scenarios/crowdfunding-claim-too-early.scen.json", world());
}

#[test]
fn crowdfunding_fund_rs() {
    dharitri_sc_scenario::run_rs("scenarios/crowdfunding-fund.scen.json", world());
}

#[test]
fn crowdfunding_fund_too_late_rs() {
    dharitri_sc_scenario::run_rs("scenarios/crowdfunding-fund-too-late.scen.json", world());
}

#[test]
fn crowdfunding_init_rs() {
    dharitri_sc_scenario::run_rs("scenarios/crowdfunding-init.scen.json", world());
}

#[test]
fn moax_crowdfunding_claim_failed_rs() {
    dharitri_sc_scenario::run_rs(
        "scenarios/moax-crowdfunding-claim-failed.scen.json",
        world(),
    );
}

#[test]
fn moax_crowdfunding_claim_successful_rs() {
    dharitri_sc_scenario::run_rs(
        "scenarios/moax-crowdfunding-claim-successful.scen.json",
        world(),
    );
}

#[test]
fn moax_crowdfunding_claim_too_early_rs() {
    dharitri_sc_scenario::run_rs(
        "scenarios/moax-crowdfunding-claim-too-early.scen.json",
        world(),
    );
}

#[test]
fn moax_crowdfunding_fund_rs() {
    dharitri_sc_scenario::run_rs("scenarios/moax-crowdfunding-fund.scen.json", world());
}

#[test]
fn moax_crowdfunding_fund_too_late_rs() {
    dharitri_sc_scenario::run_rs(
        "scenarios/moax-crowdfunding-fund-too-late.scen.json",
        world(),
    );
}

#[test]
fn moax_crowdfunding_init_rs() {
    dharitri_sc_scenario::run_rs("scenarios/moax-crowdfunding-init.scen.json", world());
}
