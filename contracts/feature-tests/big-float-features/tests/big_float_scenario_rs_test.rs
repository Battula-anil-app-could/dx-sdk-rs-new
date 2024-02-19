use dharitri_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/feature-tests/big-float-features");

    blockchain.register_contract(
        "file:output/big-float-features.wasm",
        big_float_features::ContractBuilder,
    );
    blockchain.register_contract(
        "file:../dct-system-sc-mock/output/dct-system-sc-mock.wasm",
        dct_system_sc_mock::ContractBuilder,
    );

    blockchain
}

#[test]
fn big_float_new_from_big_int_rs() {
    dharitri_sc_scenario::run_rs("scenarios/big_float_new_from_big_int.scen.json", world());
}

#[test]
fn big_float_new_from_big_uint_rs() {
    dharitri_sc_scenario::run_rs("scenarios/big_float_new_from_big_uint.scen.json", world());
}

#[test]
fn big_float_new_from_frac_rs() {
    dharitri_sc_scenario::run_rs("scenarios/big_float_new_from_frac.scen.json", world());
}

#[test]
fn big_float_new_from_int_rs() {
    dharitri_sc_scenario::run_rs("scenarios/big_float_new_from_int.scen.json", world());
}

#[test]
#[ignore]
fn big_float_new_from_managed_buffer_rs() {
    dharitri_sc_scenario::run_rs(
        "scenarios/big_float_new_from_managed_buffer.scen.json",
        world(),
    );
}

#[test]
fn big_float_new_from_parts_rs() {
    dharitri_sc_scenario::run_rs("scenarios/big_float_new_from_parts.scen.json", world());
}

#[test]
fn big_float_new_from_sci_rs() {
    dharitri_sc_scenario::run_rs("scenarios/big_float_new_from_sci.scen.json", world());
}

#[test]
#[ignore]
fn big_float_operator_checks_rs() {
    dharitri_sc_scenario::run_rs("scenarios/big_float_operator_checks.scen.json", world());
}

#[test]
fn big_float_operators_rs() {
    dharitri_sc_scenario::run_rs("scenarios/big_float_operators.scen.json", world());
}
