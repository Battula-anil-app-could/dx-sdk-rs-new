#[test]
fn auction_batch_go() {
    dharitri_sc_scenario::run_go("scenarios/auction_batch.scen.json");
}

#[test]
fn auction_single_token_moax_go() {
    dharitri_sc_scenario::run_go("scenarios/auction_single_token_moax.scen.json");
}

#[test]
fn bid_first_moax_go() {
    dharitri_sc_scenario::run_go("scenarios/bid_first_moax.scen.json");
}

#[test]
fn bid_second_moax_go() {
    dharitri_sc_scenario::run_go("scenarios/bid_second_moax.scen.json");
}

#[test]
fn bid_third_moax_go() {
    dharitri_sc_scenario::run_go("scenarios/bid_third_moax.scen.json");
}

#[test]
fn end_auction_go() {
    dharitri_sc_scenario::run_go("scenarios/end_auction.scen.json");
}
