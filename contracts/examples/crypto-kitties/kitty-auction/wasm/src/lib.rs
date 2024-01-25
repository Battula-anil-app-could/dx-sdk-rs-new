////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use dharitri_wasm_output;

#[no_mangle]
pub fn init() {
    kitty_auction::endpoints::init(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    kitty_auction::endpoints::callBack(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn bid() {
    kitty_auction::endpoints::bid(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn createAndAuctionGenZeroKitty() {
    kitty_auction::endpoints::createAndAuctionGenZeroKitty(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn createSaleAuction() {
    kitty_auction::endpoints::createSaleAuction(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn createSiringAuction() {
    kitty_auction::endpoints::createSiringAuction(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn endAuction() {
    kitty_auction::endpoints::endAuction(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getAuctionStatus() {
    kitty_auction::endpoints::getAuctionStatus(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getCurrentWinningBid() {
    kitty_auction::endpoints::getCurrentWinningBid(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn isUpForAuction() {
    kitty_auction::endpoints::isUpForAuction(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn setKittyOwnershipContractAddress() {
    kitty_auction::endpoints::setKittyOwnershipContractAddress(dharitri_wasm_node::vm_api());
}
