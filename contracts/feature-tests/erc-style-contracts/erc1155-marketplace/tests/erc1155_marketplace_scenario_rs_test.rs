use dharitri_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.register_contract(
        "mxsc:output/erc1155-marketplace.mxsc.json",
        erc1155_marketplace::ContractBuilder,
    );
    blockchain.register_contract(
        "mxsc:../erc1155/output/erc1155.mxsc.json",
        erc1155::ContractBuilder,
    );

    blockchain
}

#[test]
fn auction_batch_rs() {
    world().run("scenarios/auction_batch.scen.json");
}

#[test]
fn auction_single_token_moax_rs() {
    world().run("scenarios/auction_single_token_moax.scen.json");
}

#[test]
fn bid_first_moax_rs() {
    world().run("scenarios/bid_first_moax.scen.json");
}

#[test]
fn bid_second_moax_rs() {
    world().run("scenarios/bid_second_moax.scen.json");
}

#[test]
fn bid_third_moax_rs() {
    world().run("scenarios/bid_third_moax.scen.json");
}

#[test]
fn end_auction_rs() {
    world().run("scenarios/end_auction.scen.json");
}
