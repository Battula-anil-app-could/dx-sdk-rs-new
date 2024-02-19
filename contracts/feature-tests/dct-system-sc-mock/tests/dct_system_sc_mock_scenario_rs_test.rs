use dharitri_sc_scenario::*;

fn world() -> ScenarioWorld {
    todo!()
}

#[test]
#[ignore = "builtin SC not implemented"]
fn dct_system_sc_rs() {
    dharitri_sc_scenario::run_rs("scenarios/dct_system_sc.scen.json", world());
}
