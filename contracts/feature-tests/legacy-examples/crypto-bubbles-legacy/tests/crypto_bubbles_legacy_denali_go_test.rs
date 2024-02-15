#[test]
fn balanceof_go() {
    dharitri_sc_scenario::run_go("scenarios/balanceOf.scen.json");
}

#[test]
fn create_go() {
    dharitri_sc_scenario::run_go("scenarios/create.scen.json");
}

#[test]
fn exceptions_go() {
    dharitri_sc_scenario::run_go("scenarios/exceptions.scen.json");
}

#[test]
fn joingame_go() {
    dharitri_sc_scenario::run_go("scenarios/joinGame.scen.json");
}

#[test]
fn rewardandsendtowallet_go() {
    dharitri_sc_scenario::run_go("scenarios/rewardAndSendToWallet.scen.json");
}

#[test]
fn rewardwinner_go() {
    dharitri_sc_scenario::run_go("scenarios/rewardWinner.scen.json");
}

#[test]
fn rewardwinner_last_go() {
    dharitri_sc_scenario::run_go("scenarios/rewardWinner_Last.scen.json");
}

#[test]
fn topup_ok_go() {
    dharitri_sc_scenario::run_go("scenarios/topUp_ok.scen.json");
}

#[test]
fn topup_withdraw_go() {
    dharitri_sc_scenario::run_go("scenarios/topUp_withdraw.scen.json");
}

#[test]
fn withdraw_ok_go() {
    dharitri_sc_scenario::run_go("scenarios/withdraw_Ok.scen.json");
}

#[test]
fn withdraw_toomuch_go() {
    dharitri_sc_scenario::run_go("scenarios/withdraw_TooMuch.scen.json");
}
