////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use dharitri_wasm_output;

#[no_mangle]
pub fn init() {
    crowdfunding_dct::endpoints::init(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    crowdfunding_dct::endpoints::callBack(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn claim() {
    crowdfunding_dct::endpoints::claim(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn fund() {
    crowdfunding_dct::endpoints::fund(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getCrowdfundingTokenIdentifier() {
    crowdfunding_dct::endpoints::getCrowdfundingTokenIdentifier(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getCurrentFunds() {
    crowdfunding_dct::endpoints::getCurrentFunds(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getDeadline() {
    crowdfunding_dct::endpoints::getDeadline(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getDeposit() {
    crowdfunding_dct::endpoints::getDeposit(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn getTarget() {
    crowdfunding_dct::endpoints::getTarget(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn status() {
    crowdfunding_dct::endpoints::status(dharitri_wasm_node::vm_api());
}
