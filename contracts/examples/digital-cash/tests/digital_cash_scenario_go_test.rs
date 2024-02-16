#[test]
fn claim_moax_go() {
    dharitri_sc_scenario::run_go("scenarios/claim-moax.scen.json");
}

#[test]
fn claim_dct_go() {
    dharitri_sc_scenario::run_go("scenarios/claim-dct.scen.json");
}

#[test]
fn fund_moax_and_dct_go() {
    dharitri_sc_scenario::run_go("scenarios/fund-moax-and-dct.scen.json");
}

#[test]
fn set_accounts_go() {
    dharitri_sc_scenario::run_go("scenarios/set-accounts.scen.json");
}

#[test]
fn withdraw_moax_go() {
    dharitri_sc_scenario::run_go("scenarios/withdraw-moax.scen.json");
}

#[test]
fn withdraw_dct_go() {
    dharitri_sc_scenario::run_go("scenarios/withdraw-dct.scen.json");
}
