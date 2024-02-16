#[test]
fn approve_siring_go() {
    dharitri_sc_scenario::run_go("scenarios/approve_siring.scen.json");
}

#[test]
fn breed_ok_go() {
    dharitri_sc_scenario::run_go("scenarios/breed_ok.scen.json");
}

#[test]
fn give_birth_go() {
    dharitri_sc_scenario::run_go("scenarios/give_birth.scen.json");
}

#[test]
fn init_go() {
    dharitri_sc_scenario::run_go("scenarios/init.scen.json");
}

#[test]
fn query_go() {
    dharitri_sc_scenario::run_go("scenarios/query.scen.json");
}

#[test]
fn setup_accounts_go() {
    dharitri_sc_scenario::run_go("scenarios/setup_accounts.scen.json");
}
