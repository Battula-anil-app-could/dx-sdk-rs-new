////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use dharitri_wasm_output;

#[no_mangle]
pub fn init() {
    crowdfunding_erc20::endpoints::init(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    crowdfunding_erc20::endpoints::callBack(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn claim() {
    crowdfunding_erc20::endpoints::claim(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn fund() {
    crowdfunding_erc20::endpoints::fund(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn get_deadline() {
    crowdfunding_erc20::endpoints::get_deadline(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn get_deposit() {
    crowdfunding_erc20::endpoints::get_deposit(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn get_erc20_contract_address() {
    crowdfunding_erc20::endpoints::get_erc20_contract_address(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn get_target() {
    crowdfunding_erc20::endpoints::get_target(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn get_total_balance() {
    crowdfunding_erc20::endpoints::get_total_balance(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn status() {
    crowdfunding_erc20::endpoints::status(dharitri_wasm_node::vm_api());
}
