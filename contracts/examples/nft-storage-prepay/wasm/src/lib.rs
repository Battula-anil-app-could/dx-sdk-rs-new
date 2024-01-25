////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use dharitri_wasm_output;

#[no_mangle]
pub fn init() {
    nft_storage_prepay::endpoints::init(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    nft_storage_prepay::endpoints::callBack(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn claim() {
    nft_storage_prepay::endpoints::claim(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn depositPaymentForStorage() {
    nft_storage_prepay::endpoints::depositPaymentForStorage(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getCostForSize() {
    nft_storage_prepay::endpoints::getCostForSize(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getCostPerByte() {
    nft_storage_prepay::endpoints::getCostPerByte(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getDepositAmount() {
    nft_storage_prepay::endpoints::getDepositAmount(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn reserveFunds() {
    nft_storage_prepay::endpoints::reserveFunds(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn setCostPerByte() {
    nft_storage_prepay::endpoints::setCostPerByte(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn withdraw() {
    nft_storage_prepay::endpoints::withdraw(dharitri_wasm_node::vm_api());
}
