#[test]
fn call_other_shard_1_go() {
    dharitri_sc_scenario::run_go("scenarios/call_other_shard-1.scen.json");
}

#[test]
fn call_other_shard_2_go() {
    dharitri_sc_scenario::run_go("scenarios/call_other_shard-2.scen.json");
}

#[test]
fn change_board_go() {
    dharitri_sc_scenario::run_go("scenarios/changeBoard.scen.json");
}

#[test]
fn change_quorum_go() {
    dharitri_sc_scenario::run_go("scenarios/changeQuorum.scen.json");
}

#[test]
fn change_quorum_too_big_go() {
    dharitri_sc_scenario::run_go("scenarios/changeQuorum_tooBig.scen.json");
}

#[test]
fn deploy_adder_err_go() {
    dharitri_sc_scenario::run_go("scenarios/deployAdder_err.scen.json");
}

#[test]
fn deploy_adder_then_call_go() {
    dharitri_sc_scenario::run_go("scenarios/deployAdder_then_call.scen.json");
}

#[test]
fn deploy_factorial_go() {
    dharitri_sc_scenario::run_go("scenarios/deployFactorial.scen.json");
}

#[test]
fn deploy_other_multisig_go() {
    dharitri_sc_scenario::run_go("scenarios/deployOtherMultisig.scen.json");
}

#[test]
fn deploy_duplicate_bm_go() {
    dharitri_sc_scenario::run_go("scenarios/deploy_duplicate_bm.scen.json");
}

#[test]
fn remove_everyone_go() {
    dharitri_sc_scenario::run_go("scenarios/remove_everyone.scen.json");
}

// TODO: investigate gas issue
#[test]
#[ignore]
fn send_dct_go() {
    dharitri_sc_scenario::run_go("scenarios/sendDct.scen.json");
}

#[test]
fn upgrade_go() {
    dharitri_sc_scenario::run_go("scenarios/upgrade.scen.json");
}

#[test]
fn upgrade_from_source_go() {
    dharitri_sc_scenario::run_go("scenarios/upgrade_from_source.scen.json");
}
