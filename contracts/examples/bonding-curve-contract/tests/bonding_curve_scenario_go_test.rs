#[test]
fn deploy_rs() {
    dharitri_sc_scenario::run_go("scenarios/deploy.scen.json");
}

#[test]
fn deposit_rs() {
    dharitri_sc_scenario::run_go("scenarios/deposit.scen.json");
}

#[test]
fn set_bonding_curve_rs() {
    dharitri_sc_scenario::run_go("scenarios/set_bonding_curve.scen.json");
}

#[test]
fn buy_rs() {
    dharitri_sc_scenario::run_go("scenarios/buy.scen.json");
}

#[test]
fn sell_rs() {
    dharitri_sc_scenario::run_go("scenarios/sell.scen.json");
}

#[test]
fn deposit_more_view_rs() {
    dharitri_sc_scenario::run_go("scenarios/deposit_more_view.scen.json");
}

#[test]
fn claim_rs() {
    dharitri_sc_scenario::run_go("scenarios/claim.scen.json");
}
