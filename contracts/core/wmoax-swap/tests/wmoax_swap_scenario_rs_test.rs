use dharitri_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/core/wmoax-swap");

    blockchain.register_contract(
        "file:output/dharitri-wmoax-swap-sc.wasm",
        dharitri_wmoax_swap_sc::ContractBuilder,
    );
    blockchain
}

#[test]
fn unwrap_moax_rs() {
    dharitri_sc_scenario::run_rs("scenarios/unwrap_moax.scen.json", world());
}

#[test]
fn wrap_moax_rs() {
    dharitri_sc_scenario::run_rs("scenarios/wrap_moax.scen.json", world());
}
