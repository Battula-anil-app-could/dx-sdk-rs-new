////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use dharitri_wasm_output;

#[no_mangle]
pub fn init() {
    erc1155_marketplace::endpoints::init(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    erc1155_marketplace::endpoints::callBack(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn bid() {
    erc1155_marketplace::endpoints::bid(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn claim() {
    erc1155_marketplace::endpoints::claim(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn endAuction() {
    erc1155_marketplace::endpoints::endAuction(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getAuctionStatus() {
    erc1155_marketplace::endpoints::getAuctionStatus(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getCurrentWinner() {
    erc1155_marketplace::endpoints::getCurrentWinner(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getCurrentWinningBid() {
    erc1155_marketplace::endpoints::getCurrentWinningBid(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getPercentageCut() {
    erc1155_marketplace::endpoints::getPercentageCut(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn isUpForAuction() {
    erc1155_marketplace::endpoints::isUpForAuction(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn onERC1155BatchReceived() {
    erc1155_marketplace::endpoints::onERC1155BatchReceived(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn onERC1155Received() {
    erc1155_marketplace::endpoints::onERC1155Received(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn setCutPercentage() {
    erc1155_marketplace::endpoints::setCutPercentage(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn setTokenOwnershipContractAddress() {
    erc1155_marketplace::endpoints::setTokenOwnershipContractAddress(dharitri_wasm_node::vm_api());
}
