#[test]
fn generated_fund_go() {
    dharitri_sc_scenario::run_go("scenarios/_generated_fund.scen.json");
}

#[test]
fn generated_init_go() {
    dharitri_sc_scenario::run_go("scenarios/_generated_init.scen.json");
}

#[test]
fn generated_query_status_go() {
    dharitri_sc_scenario::run_go("scenarios/_generated_query_status.scen.json");
}

#[test]
fn generated_sc_err_go() {
    dharitri_sc_scenario::run_go("scenarios/_generated_sc_err.scen.json");
}

#[test]
fn crowdfunding_claim_failed_go() {
    dharitri_sc_scenario::run_go("scenarios/crowdfunding-claim-failed.scen.json");
}

#[test]
fn crowdfunding_claim_successful_go() {
    dharitri_sc_scenario::run_go("scenarios/crowdfunding-claim-successful.scen.json");
}

#[test]
fn crowdfunding_claim_too_early_go() {
    dharitri_sc_scenario::run_go("scenarios/crowdfunding-claim-too-early.scen.json");
}

#[test]
fn crowdfunding_fund_go() {
    dharitri_sc_scenario::run_go("scenarios/crowdfunding-fund.scen.json");
}

#[test]
fn crowdfunding_fund_too_late_go() {
    dharitri_sc_scenario::run_go("scenarios/crowdfunding-fund-too-late.scen.json");
}

#[test]
fn crowdfunding_init_go() {
    dharitri_sc_scenario::run_go("scenarios/crowdfunding-init.scen.json");
}

#[test]
fn moax_crowdfunding_claim_failed_go() {
    dharitri_sc_scenario::run_go("scenarios/moax-crowdfunding-claim-failed.scen.json");
}

#[test]
fn moax_crowdfunding_claim_successful_go() {
    dharitri_sc_scenario::run_go("scenarios/moax-crowdfunding-claim-successful.scen.json");
}

#[test]
fn moax_crowdfunding_claim_too_early_go() {
    dharitri_sc_scenario::run_go("scenarios/moax-crowdfunding-claim-too-early.scen.json");
}

#[test]
fn moax_crowdfunding_fund_go() {
    dharitri_sc_scenario::run_go("scenarios/moax-crowdfunding-fund.scen.json");
}

#[test]
fn moax_crowdfunding_fund_too_late_go() {
    dharitri_sc_scenario::run_go("scenarios/moax-crowdfunding-fund-too-late.scen.json");
}

#[test]
fn moax_crowdfunding_init_go() {
    dharitri_sc_scenario::run_go("scenarios/moax-crowdfunding-init.scen.json");
}
