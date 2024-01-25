////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use dharitri_wasm_output;

#[no_mangle]
pub fn init() {
    erc20::endpoints::init(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    erc20::endpoints::callBack(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn allowance() {
    erc20::endpoints::allowance(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn approve() {
    erc20::endpoints::approve(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn balanceOf() {
    erc20::endpoints::balanceOf(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn totalSupply() {
    erc20::endpoints::totalSupply(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn transfer() {
    erc20::endpoints::transfer(dharitri_wasm_node::vm_api());
}

#[no_mangle]
pub fn transferFrom() {
    erc20::endpoints::transferFrom(dharitri_wasm_node::vm_api());
}
