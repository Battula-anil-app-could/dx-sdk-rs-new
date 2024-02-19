use dharitri_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain
        .set_current_dir_from_workspace("contracts/feature-tests/rust-testing-framework-tester");

    blockchain.register_contract(
        "file:output/rust-testing-framework-tester.wasm",
        rust_testing_framework_tester::ContractBuilder,
    );
    blockchain
}

#[test]
fn test_rs() {
    dharitri_sc_scenario::run_rs("scenarios/test.scen.json", world());
}

#[test]
fn test_dct_generation_rs() {
    dharitri_sc_scenario::run_rs("scenarios/test_dct_generation.scen.json", world());
}

#[test]
fn test_multiple_sc_rs() {
    dharitri_sc_scenario::run_rs("scenarios/test_multiple_sc.scen.json", world());
}

#[test]
#[ignore = "not supported"]
fn trace_deploy_rs() {
    dharitri_sc_scenario::run_rs("scenarios/trace-deploy.scen.json", world());
}
