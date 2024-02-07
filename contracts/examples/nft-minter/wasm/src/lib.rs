////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use dharitri_wasm_output;

#[no_mangle]
pub fn init() {
    nft_minter::endpoints::init(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    nft_minter::endpoints::callBack(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn buyNft() {
    nft_minter::endpoints::buyNft(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn createNft() {
    nft_minter::endpoints::createNft(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getNftPrice() {
    nft_minter::endpoints::getNftPrice(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn issueToken() {
    nft_minter::endpoints::issueToken(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn setLocalRoles() {
    nft_minter::endpoints::setLocalRoles(dharitri_wasm_node::vm_api());
}
