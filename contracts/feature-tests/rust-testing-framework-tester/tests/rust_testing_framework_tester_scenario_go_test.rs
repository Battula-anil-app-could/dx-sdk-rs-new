#[test]
fn test_go() {
    dharitri_sc_scenario::run_go("scenarios/test.scen.json");
}

#[test]
fn test_dct_generation_go() {
    dharitri_sc_scenario::run_go("scenarios/test_dct_generation.scen.json");
}

#[test]
fn test_multiple_sc_go() {
    dharitri_sc_scenario::run_go("scenarios/test_multiple_sc.scen.json");
}

#[test]
#[ignore = "not supported"]
fn trace_deploy_go() {
    dharitri_sc_scenario::run_go("scenarios/trace-deploy.scen.json");
}
